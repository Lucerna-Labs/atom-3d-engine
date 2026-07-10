//! World-field acceleration: a BVH tree fold over runs of plain-union objects.
//!
//! The renderer's semantics are a left fold over placed objects. Smooth unions and subtraction are
//! order-sensitive, so this module only reorders maximal runs of plain hard unions, where `min` is
//! associative/commutative. It still preserves material tie-breaking by earliest object index.

use mm3e_kit::sdf::Field;
use mm3e_kit::vec::Vec3;
use mm3e_kit::volume::SdfVolume;

use crate::{Combine, Object};

struct Node {
    center: Vec3,
    radius: f32,
    left: u32,
    right: u32,
    leaf: i32,
}

struct Bvh {
    nodes: Vec<Node>,
    root: u32,
}

fn enclose(c1: Vec3, r1: f32, c2: Vec3, r2: f32) -> (Vec3, f32) {
    let d = (c2 - c1).length();
    if d + r2 <= r1 {
        return (c1, r1);
    }
    if d + r1 <= r2 {
        return (c2, r2);
    }
    let radius = (d + r1 + r2) * 0.5;
    (c1 + (c2 - c1).scale((radius - r1) / d.max(1e-12)), radius)
}

impl Bvh {
    fn build(items: &mut [(usize, Vec3, f32)], nodes: &mut Vec<Node>) -> u32 {
        debug_assert!(!items.is_empty());
        if items.len() == 1 {
            let (i, c, r) = items[0];
            nodes.push(Node { center: c, radius: r, left: 0, right: 0, leaf: i as i32 });
            return (nodes.len() - 1) as u32;
        }
        let mut lo = Vec3::splat(f32::INFINITY);
        let mut hi = Vec3::splat(f32::NEG_INFINITY);
        for &(_, c, _) in items.iter() {
            lo = lo.min(c);
            hi = hi.max(c);
        }
        let ext = hi - lo;
        let axis = if ext.x >= ext.y && ext.x >= ext.z {
            0
        } else if ext.y >= ext.z {
            1
        } else {
            2
        };
        let coord = |c: Vec3| match axis {
            0 => c.x,
            1 => c.y,
            _ => c.z,
        };
        items.sort_unstable_by(|a, b| {
            coord(a.1).partial_cmp(&coord(b.1)).unwrap_or(std::cmp::Ordering::Equal).then(a.0.cmp(&b.0))
        });
        let mid = items.len() / 2;
        let (a, b) = items.split_at_mut(mid);
        let left = Self::build(a, nodes);
        let right = Self::build(b, nodes);
        let (lc, lr) = (nodes[left as usize].center, nodes[left as usize].radius);
        let (rc, rr) = (nodes[right as usize].center, nodes[right as usize].radius);
        let (center, radius) = enclose(lc, lr, rc, rr);
        nodes.push(Node { center, radius, left, right, leaf: -1 });
        (nodes.len() - 1) as u32
    }

    fn query(&self, p: Vec3, objects: &[Object], volumes: &[SdfVolume], slack: f32, best: &mut (f32, i64, u32)) {
        let mut stack = [0u32; 64];
        let mut sp = 0usize;
        stack[sp] = self.root;
        sp += 1;
        while sp > 0 {
            sp -= 1;
            let node = &self.nodes[stack[sp] as usize];
            let lower = (p - node.center).length() - node.radius;
            if lower > best.0 {
                continue;
            }
            if node.leaf >= 0 {
                let i = node.leaf as usize;
                let contrib =
                    if lower > slack { Field::new(lower, objects[i].mat) } else { objects[i].field(p, volumes) };
                if contrib.dist < best.0 || (contrib.dist == best.0 && (i as i64) < best.1) {
                    *best = (contrib.dist, i as i64, contrib.mat);
                }
            } else {
                let (l, r) = (node.left, node.right);
                let ld = (p - self.nodes[l as usize].center).length_sq();
                let rd = (p - self.nodes[r as usize].center).length_sq();
                let (near, far) = if ld <= rd { (l, r) } else { (r, l) };
                stack[sp] = far;
                stack[sp + 1] = near;
                sp += 2;
            }
        }
    }
}

enum Segment {
    One(usize),
    Run { tree: Option<Bvh>, unbounded: Vec<usize> },
}

pub(crate) struct WorldPlan {
    segments: Vec<Segment>,
    bounds: Vec<Option<(Vec3, f32)>>,
    slack: f32,
}

impl WorldPlan {
    pub(crate) fn build(objects: &[Object], volumes: &[SdfVolume]) -> WorldPlan {
        let bounds: Vec<Option<(Vec3, f32)>> = objects.iter().map(|o| o.world_bound(volumes)).collect();
        let slack = objects
            .iter()
            .map(|o| match o.combine {
                Combine::Smooth(k) => k,
                _ => 0.0,
            })
            .fold(0.0_f32, f32::max)
            + 0.1;

        let mut segments = Vec::new();
        let mut run = Vec::new();
        let close_run = |run: &mut Vec<usize>, segments: &mut Vec<Segment>, bounds: &[Option<(Vec3, f32)>]| {
            if run.is_empty() {
                return;
            }
            let mut items = Vec::new();
            let mut unbounded = Vec::new();
            for &i in run.iter() {
                match bounds[i] {
                    Some((c, r)) => items.push((i, c, r)),
                    None => unbounded.push(i),
                }
            }
            let tree = if items.is_empty() {
                None
            } else {
                let mut nodes = Vec::with_capacity(items.len() * 2);
                let root = Bvh::build(&mut items, &mut nodes);
                Some(Bvh { nodes, root })
            };
            segments.push(Segment::Run { tree, unbounded });
            run.clear();
        };
        for (i, o) in objects.iter().enumerate() {
            match o.combine {
                Combine::Union => run.push(i),
                Combine::Smooth(_) | Combine::Subtract => {
                    close_run(&mut run, &mut segments, &bounds);
                    segments.push(Segment::One(i));
                }
            }
        }
        close_run(&mut run, &mut segments, &bounds);
        WorldPlan { segments, bounds, slack }
    }

    fn contrib(&self, i: usize, p: Vec3, objects: &[Object], volumes: &[SdfVolume]) -> Field {
        match self.bounds[i] {
            Some((c, r)) => {
                let lower = (p - c).length() - r;
                if lower > self.slack {
                    Field::new(lower, objects[i].mat)
                } else {
                    objects[i].field(p, volumes)
                }
            }
            None => objects[i].field(p, volumes),
        }
    }

    pub(crate) fn eval(&self, p: Vec3, objects: &[Object], volumes: &[SdfVolume]) -> Field {
        use mm3e_kit::sdf;
        let mut acc = Field::FAR;
        let mut seeded = false;
        for seg in &self.segments {
            match seg {
                Segment::One(i) => {
                    let f = self.contrib(*i, p, objects, volumes);
                    if !seeded {
                        acc = f;
                        seeded = true;
                        continue;
                    }
                    acc = match objects[*i].combine {
                        Combine::Union => sdf::union(acc, f),
                        Combine::Smooth(k) => sdf::smooth_union(acc, f, k),
                        Combine::Subtract => sdf::subtract(acc, f),
                    };
                }
                Segment::Run { tree, unbounded } => {
                    let mut best = if seeded { (acc.dist, -1i64, acc.mat) } else { (f32::INFINITY, i64::MAX, 0u32) };
                    for &i in unbounded {
                        let f = objects[i].field(p, volumes);
                        if f.dist < best.0 || (f.dist == best.0 && (i as i64) < best.1) {
                            best = (f.dist, i as i64, f.mat);
                        }
                    }
                    if let Some(t) = tree {
                        t.query(p, objects, volumes, self.slack, &mut best);
                    }
                    acc = Field::new(best.0, best.2);
                    seeded = true;
                }
            }
        }
        acc
    }
}
