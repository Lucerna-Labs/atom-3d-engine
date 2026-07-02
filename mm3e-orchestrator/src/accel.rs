//! The world-field acceleration plan — a BVH tree-fold over runs of plain-union objects.
//!
//! This is the roadmap's "bounded fold → tree fold" step, built to a hard constraint: the
//! accelerated field is **bit-identical** to the linear fold it replaces (`tests/engine.rs`
//! proves it point-by-point). That falls out of three rules:
//!
//! 1. **Segmentation preserves CSG order.** The object list folds left-to-right, and
//!    `Smooth`/`Subtract` objects are order-sensitive (an object unioned *after* a subtraction
//!    is not carved by it). So the plan splits the list into segments: each `Smooth`/`Subtract`
//!    object (and a non-`Union` first object, which seeds the accumulator exactly as the linear
//!    fold does) is an ordered singleton, and every maximal run of consecutive plain-`Union`
//!    objects becomes one tree. Union is a `min` — commutative and associative — so a run may be
//!    gathered in any order *provided ties break the way the linear fold breaks them* (rule 3).
//! 2. **Leaves contribute exactly what the linear fold contributes.** Outside an object's
//!    bounding sphere by more than `slack`, that is the cheap lower bound `|p − c| − r` (with the
//!    object's material); inside, the exact SDF. A tree node whose own lower bound exceeds the
//!    current best **strictly** cannot supply a smaller contribution, so the whole subtree is
//!    skipped — the resulting `min` is unchanged, which is why the output is identical rather
//!    than merely close. (On an exact tie the node is descended, not skipped: a tied contribution
//!    from a lower object index must still win, rule 3.)
//! 3. **Ties break toward the earlier object.** `sdf::union(a, b)` keeps `a` on equal distances,
//!    so the linear fold resolves equal contributions to the earliest index — and the running
//!    accumulator from previous segments beats every run member on a tie. The traversal carries
//!    `(dist, index)` and replaces only on strictly-smaller distance or equal-distance +
//!    smaller index (the accumulator holds index −1).
//!
//! Unbounded objects (planes; `repeat`/`bend`-modified domains) cannot live in the tree; a run
//! evaluates them linearly *first*, which doubles as a tight initial prune bound — a floor plane
//! near the sample typically lets the root test discard the entire tree in one sphere check.
//!
//! Mechanism-vs-policy note: this module is pure policy (which objects to evaluate, in what
//! order); every distance it produces still comes from the kit's SDFs.

use mm3e_kit::sdf::Field;
use mm3e_kit::vec::Vec3;
use mm3e_kit::volume::SdfVolume;

use crate::{Combine, Object};

/// One node of the flat BVH. `leaf >= 0` is an object index; otherwise `left`/`right` are node
/// indices. The sphere `(center, radius)` conservatively encloses the subtree's object spheres.
struct Node {
    center: Vec3,
    radius: f32,
    left: u32,
    right: u32,
    leaf: i32,
}

/// A BVH over the bounded members of one union run.
struct Bvh {
    nodes: Vec<Node>,
    root: u32,
}

/// The smallest sphere enclosing two spheres (standard construction; exact when one contains
/// the other).
fn enclose(c1: Vec3, r1: f32, c2: Vec3, r2: f32) -> (Vec3, f32) {
    let d = (c2 - c1).length();
    if d + r2 <= r1 {
        return (c1, r1);
    }
    if d + r1 <= r2 {
        return (c2, r2);
    }
    let radius = (d + r1 + r2) * 0.5;
    // d > 0 here: d == 0 with neither containing the other is impossible (equal centers means
    // the larger radius contains the smaller sphere).
    (c1 + (c2 - c1).scale((radius - r1) / d), radius)
}

impl Bvh {
    /// Median-split build over `items = (object index, sphere center, sphere radius)`. Sorting is
    /// made deterministic (coordinate, then index) so every thread builds the identical tree.
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

    /// Fold every leaf that could beat `best` into it. `best` is `(dist, index, mat)`; the
    /// replace rule is strictly-smaller distance, or equal distance with smaller index — the
    /// linear fold's earliest-wins tie-break, made traversal-order independent.
    fn query(&self, p: Vec3, objects: &[Object], volumes: &[SdfVolume], slack: f32, best: &mut (f32, i64, u32)) {
        // Median splits halve the item count per level, so depth ≤ ceil(log2 n) + 1; 48 covers
        // any realizable scene.
        let mut stack = [0u32; 48];
        let mut sp = 0usize;
        stack[sp] = self.root;
        sp += 1;
        while sp > 0 {
            sp -= 1;
            let node = &self.nodes[stack[sp] as usize];
            let lower = (p - node.center).length() - node.radius;
            if lower > best.0 {
                continue; // strictly worse than best: nothing below can win (ties must descend)
            }
            if node.leaf >= 0 {
                let i = node.leaf as usize;
                // The leaf sphere IS the object's bounding sphere, so `lower` here is exactly
                // the linear fold's per-object lower bound: same substitution, same contribution.
                let contrib =
                    if lower > slack { Field::new(lower, objects[i].mat) } else { objects[i].field(p, volumes) };
                if contrib.dist < best.0 || (contrib.dist == best.0 && (i as i64) < best.1) {
                    *best = (contrib.dist, i as i64, contrib.mat);
                }
            } else {
                // Visit the nearer child first (pop order is LIFO): it tightens `best` sooner,
                // letting the farther child's own test prune it.
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

/// One ordered piece of the fold.
enum Segment {
    /// A single order-sensitive object (`Smooth`/`Subtract`, or a non-`Union` seed).
    One(usize),
    /// A maximal run of consecutive plain-`Union` objects: bounded members in the tree,
    /// unbounded ones evaluated linearly (they also seed the prune bound).
    Run { tree: Option<Bvh>, unbounded: Vec<usize> },
}

/// The compiled evaluation plan for a scene's object list. Build once per `world()` closure;
/// evaluate millions of times.
pub(crate) struct WorldPlan {
    segments: Vec<Segment>,
    /// Per-object bounds for the ordered singletons (same `Object::world_bound` the tree used).
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
        let mut run: Vec<usize> = Vec::new();
        let close_run = |run: &mut Vec<usize>, segments: &mut Vec<Segment>| {
            if run.is_empty() {
                return;
            }
            let mut items: Vec<(usize, Vec3, f32)> = Vec::new();
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
                // The linear fold seeds `acc` with object 0's contribution regardless of its
                // combine mode, so a Smooth/Subtract first object is an ordered singleton here
                // and `eval` seeds from it — the same quirk, reproduced.
                Combine::Smooth(_) | Combine::Subtract => {
                    close_run(&mut run, &mut segments);
                    segments.push(Segment::One(i));
                }
            }
        }
        close_run(&mut run, &mut segments);
        WorldPlan { segments, bounds, slack }
    }

    /// An object's contribution at `p` — the linear fold's substitution rule, verbatim.
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

    /// Evaluate the world field at `p` — bit-identical to the linear left fold over `objects`.
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
                    // The accumulator beats run members on ties (union keeps `a`), hence index −1.
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
