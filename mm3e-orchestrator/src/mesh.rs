//! Mesh ingestion: OBJ import plus mesh-to-SDF baking.
//!
//! Triangles are accepted as an interchange format, then converted once into the engine's basis:
//! a sampled signed-distance field. Rendering, CSG, GI, physics, and shadows keep querying fields.

use mm3e_kit::vec::Vec3;
use mm3e_kit::volume::SdfVolume;

/// An indexed triangle mesh in local space.
#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub positions: Vec<Vec3>,
    /// Triangles as indices into `positions`.
    pub triangles: Vec<[u32; 3]>,
}

impl Mesh {
    /// The mesh's axis-aligned bounding box, or `None` for an empty mesh.
    pub fn bounds(&self) -> Option<(Vec3, Vec3)> {
        if self.positions.is_empty() || self.triangles.is_empty() {
            return None;
        }
        let mut lo = Vec3::splat(f32::INFINITY);
        let mut hi = Vec3::splat(f32::NEG_INFINITY);
        for &p in &self.positions {
            lo = lo.min(p);
            hi = hi.max(p);
        }
        Some((lo, hi))
    }

    fn validate(&self) -> Result<(), String> {
        if self.positions.is_empty() {
            return Err("mesh has no vertices".into());
        }
        if self.triangles.is_empty() {
            return Err("mesh has no triangles".into());
        }
        for (i, p) in self.positions.iter().enumerate() {
            if !(p.x.is_finite() && p.y.is_finite() && p.z.is_finite()) {
                return Err(format!("mesh vertex {i} is non-finite"));
            }
        }
        let n = self.positions.len() as u32;
        for (i, tri) in self.triangles.iter().enumerate() {
            if tri.iter().any(|&idx| idx >= n) {
                return Err(format!("mesh triangle {i} references a vertex out of range"));
            }
        }
        Ok(())
    }
}

/// Parse a Wavefront OBJ document: `v` positions and `f` faces. Faces are fan-triangulated; OBJ
/// `v/vt/vn` forms and negative indices are supported. Other directives are ignored.
pub fn parse_obj(src: &str) -> Result<Mesh, String> {
    let mut mesh = Mesh::default();
    for (ln, raw) in src.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut toks = line.split_whitespace();
        match toks.next() {
            Some("v") => {
                let mut f = |what: &str| -> Result<f32, String> {
                    toks.next()
                        .ok_or_else(|| format!("line {}: missing {what}", ln + 1))?
                        .parse::<f32>()
                        .map_err(|_| format!("line {}: bad number for {what}", ln + 1))
                };
                let p = Vec3::new(f("x")?, f("y")?, f("z")?);
                if !(p.x.is_finite() && p.y.is_finite() && p.z.is_finite()) {
                    return Err(format!("line {}: non-finite vertex", ln + 1));
                }
                mesh.positions.push(p);
            }
            Some("f") => {
                let mut idx = Vec::new();
                for tok in toks {
                    let first = tok.split('/').next().unwrap_or("");
                    let raw_i: i64 = first.parse().map_err(|_| format!("line {}: bad face index '{tok}'", ln + 1))?;
                    let n = mesh.positions.len() as i64;
                    let resolved = if raw_i > 0 { raw_i - 1 } else { n + raw_i };
                    if resolved < 0 || resolved >= n {
                        return Err(format!("line {}: face index {raw_i} out of range (have {n} vertices)", ln + 1));
                    }
                    idx.push(resolved as u32);
                }
                if idx.len() < 3 {
                    return Err(format!("line {}: face needs at least 3 vertices", ln + 1));
                }
                for t in 1..idx.len() - 1 {
                    mesh.triangles.push([idx[0], idx[t], idx[t + 1]]);
                }
            }
            _ => {}
        }
    }
    mesh.validate()?;
    Ok(mesh)
}

/// Load and parse an OBJ file from disk.
pub fn load_obj(path: &std::path::Path) -> Result<Mesh, String> {
    let src = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    parse_obj(&src)
}

fn closest_on_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }
    let bp = p - b;
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        return a + ab.scale(d1 / (d1 - d3));
    }
    let cp = p - c;
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        return a + ac.scale(d2 / (d2 - d6));
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        return b + (c - b).scale((d4 - d3) / ((d4 - d3) + (d5 - d6)));
    }
    let denom = 1.0 / (va + vb + vc);
    a + ab.scale(vb * denom) + ac.scale(vc * denom)
}

struct TriNode {
    lo: Vec3,
    hi: Vec3,
    left: u32,
    right: u32,
    leaf: i32,
}

struct TriBvh {
    nodes: Vec<TriNode>,
    root: u32,
}

fn box_dist_sq(p: Vec3, lo: Vec3, hi: Vec3) -> f32 {
    (p - p.clamp_to(lo, hi)).length_sq()
}

impl TriBvh {
    fn build_from(items: &mut [(usize, Vec3, Vec3, Vec3)], nodes: &mut Vec<TriNode>) -> u32 {
        debug_assert!(!items.is_empty());
        let mut lo = Vec3::splat(f32::INFINITY);
        let mut hi = Vec3::splat(f32::NEG_INFINITY);
        for &(_, l, h, _) in items.iter() {
            lo = lo.min(l);
            hi = hi.max(h);
        }
        if items.len() == 1 {
            nodes.push(TriNode { lo, hi, left: 0, right: 0, leaf: items[0].0 as i32 });
            return (nodes.len() - 1) as u32;
        }
        let mut clo = Vec3::splat(f32::INFINITY);
        let mut chi = Vec3::splat(f32::NEG_INFINITY);
        for &(_, _, _, c) in items.iter() {
            clo = clo.min(c);
            chi = chi.max(c);
        }
        let ext = chi - clo;
        let coord: fn(Vec3) -> f32 = if ext.x >= ext.y && ext.x >= ext.z {
            |c| c.x
        } else if ext.y >= ext.z {
            |c| c.y
        } else {
            |c| c.z
        };
        items.sort_unstable_by(|a, b| {
            coord(a.3).partial_cmp(&coord(b.3)).unwrap_or(std::cmp::Ordering::Equal).then(a.0.cmp(&b.0))
        });
        let mid = items.len() / 2;
        let (l_items, r_items) = items.split_at_mut(mid);
        let left = Self::build_from(l_items, nodes);
        let right = Self::build_from(r_items, nodes);
        nodes.push(TriNode { lo, hi, left, right, leaf: -1 });
        (nodes.len() - 1) as u32
    }

    fn build(mesh: &Mesh) -> Option<TriBvh> {
        let mut items: Vec<(usize, Vec3, Vec3, Vec3)> = mesh
            .triangles
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                let (a, b, c) =
                    (mesh.positions[t[0] as usize], mesh.positions[t[1] as usize], mesh.positions[t[2] as usize]);
                if (b - a).cross(c - a).length_sq() < 1e-16 {
                    return None;
                }
                Some((i, a.min(b).min(c), a.max(b).max(c), (a + b + c).scale(1.0 / 3.0)))
            })
            .collect();
        if items.is_empty() {
            return None;
        }
        let mut nodes = Vec::with_capacity(items.len() * 2);
        let root = Self::build_from(&mut items, &mut nodes);
        Some(TriBvh { nodes, root })
    }

    fn nearest_sq(&self, p: Vec3, mesh: &Mesh) -> f32 {
        let mut best = f32::INFINITY;
        let mut stack = [0u32; 64];
        let mut sp = 0usize;
        stack[sp] = self.root;
        sp += 1;
        while sp > 0 {
            sp -= 1;
            let node = &self.nodes[stack[sp] as usize];
            if box_dist_sq(p, node.lo, node.hi) >= best {
                continue;
            }
            if node.leaf >= 0 {
                let t = mesh.triangles[node.leaf as usize];
                let q = closest_on_triangle(
                    p,
                    mesh.positions[t[0] as usize],
                    mesh.positions[t[1] as usize],
                    mesh.positions[t[2] as usize],
                );
                best = best.min((p - q).length_sq());
            } else {
                let (l, r) = (node.left, node.right);
                let ld = box_dist_sq(p, self.nodes[l as usize].lo, self.nodes[l as usize].hi);
                let rd = box_dist_sq(p, self.nodes[r as usize].lo, self.nodes[r as usize].hi);
                let (near, far) = if ld <= rd { (l, r) } else { (r, l) };
                stack[sp] = far;
                stack[sp + 1] = near;
                sp += 2;
            }
        }
        best
    }
}

fn x_crossing(a: Vec3, b: Vec3, c: Vec3, oy: f32, oz: f32) -> Option<f32> {
    let e1 = b - a;
    let e2 = c - a;
    let det = e1.y * (-e2.z) + e1.z * e2.y;
    if det.abs() < 1e-12 {
        return None;
    }
    let inv = 1.0 / det;
    let (sy, sz) = (oy - a.y, oz - a.z);
    let u = (sy * (-e2.z) + sz * e2.y) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let v = (sy * e1.z - sz * e1.y) * inv;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    Some(a.x + u * e1.x + v * e2.x)
}

/// Bake `mesh` into a signed-distance grid.
///
/// `resolution` is the sample count along the longest padded axis. Sign comes from row-parity
/// ray crossings, so closed/watertight meshes produce real signed distances; open meshes still
/// produce exact unsigned distances but may have unreliable signs.
pub fn bake_sdf(mesh: &Mesh, resolution: usize, padding: f32) -> Result<SdfVolume, String> {
    mesh.validate()?;
    let (mlo, mhi) = mesh.bounds().ok_or("cannot bake an empty mesh")?;
    if padding <= 0.0 || !padding.is_finite() {
        return Err("padding must be positive and finite".into());
    }
    let resolution = resolution.clamp(2, 512);
    let lo = mlo - Vec3::splat(padding);
    let hi = mhi + Vec3::splat(padding);
    let size = hi - lo;
    let longest = size.x.max(size.y).max(size.z).max(1e-6);
    let dim = |s: f32| (((s / longest) * (resolution - 1) as f32).round() as usize + 1).max(2);
    let (nx, ny, nz) = (dim(size.x), dim(size.y), dim(size.z));
    let cell = Vec3::new(size.x / (nx - 1) as f32, size.y / (ny - 1) as f32, size.z / (nz - 1) as f32);
    let bvh = TriBvh::build(mesh).ok_or("mesh has no non-degenerate triangles")?;
    let nudge = 1.37e-4 * cell.y.min(cell.z);

    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).max(1);
    let span = nz.div_ceil(threads);
    let slabs: Vec<(usize, Vec<f32>)> = std::thread::scope(|s| {
        let (bvh, mesh) = (&bvh, mesh);
        let handles: Vec<_> = (0..threads)
            .map(|ti| {
                let k0 = ti * span;
                let k1 = ((ti + 1) * span).min(nz);
                s.spawn(move || {
                    let mut out = Vec::with_capacity(k1.saturating_sub(k0) * nx * ny);
                    let mut crossings = Vec::new();
                    for k in k0..k1 {
                        let z = lo.z + k as f32 * cell.z + nudge;
                        for j in 0..ny {
                            let y = lo.y + j as f32 * cell.y + nudge;
                            crossings.clear();
                            for t in &mesh.triangles {
                                let (a, b, c) = (
                                    mesh.positions[t[0] as usize],
                                    mesh.positions[t[1] as usize],
                                    mesh.positions[t[2] as usize],
                                );
                                if let Some(x) = x_crossing(a, b, c, y, z) {
                                    crossings.push(x);
                                }
                            }
                            crossings.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                            for i in 0..nx {
                                let p = Vec3::new(
                                    lo.x + i as f32 * cell.x,
                                    lo.y + j as f32 * cell.y,
                                    lo.z + k as f32 * cell.z,
                                );
                                let unsigned = bvh.nearest_sq(p, mesh).sqrt();
                                let inside = crossings.partition_point(|&x| x < p.x) % 2 == 1;
                                out.push(if inside { -unsigned } else { unsigned });
                            }
                        }
                    }
                    (k0, out)
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    let mut data = vec![0.0f32; nx * ny * nz];
    for (k0, slab) in slabs {
        if slab.is_empty() {
            continue;
        }
        let base = k0 * ny * nx;
        data[base..base + slab.len()].copy_from_slice(&slab);
    }
    SdfVolume::new((nx, ny, nz), lo, cell, data)
}
