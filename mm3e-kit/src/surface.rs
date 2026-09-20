//! Exact distance to a finite triangle surface with physical shell thickness.
//!
//! Open boundaries remain open. The field is `min(distance(point, triangle)) -
//! half_thickness`, the Minkowski sum of the authored surface and a sphere, without
//! voxelization or an inferred solid interior. Triangle indices and vertices are retained.

use crate::vec::Vec3;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

pub const MAX_SURFACE_VERTICES: usize = 65_536;
pub const MAX_SURFACE_TRIANGLES: usize = 131_072;
const LEAF_SIZE: usize = 8;

/// Validated, immutable surface geometry. Clones share the source topology and triangle BVH.
#[derive(Clone, Debug)]
pub struct TriangleSurface(Arc<SurfaceData>);

/// A completed surface query and the exact number of distance tests it executed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceDistance {
    pub distance: f32,
    pub work: usize,
}

/// Conservative AABB-overlap candidates in original triangle order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceCandidates {
    pub triangles: Vec<u32>,
    /// One unit per visited BVH node and per tested leaf triangle AABB.
    pub work: usize,
}

/// Rejected candidate query with every completed node/triangle AABB test
/// retained. No partial candidate list is exposed. Invalid input spends zero.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceCandidateFailure {
    pub work: usize,
    pub message: String,
}
impl std::fmt::Display for SurfaceCandidateFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(formatter)
    }
}
impl std::error::Error for SurfaceCandidateFailure {}

/// A completed nearest-surface query in the coordinates stored by this surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceHit {
    /// Euclidean distance to the authored triangles minus half thickness.
    pub distance: f32,
    /// Nearest point on the authored midsurface, retained at traversal f64 precision.
    pub closest_point: [f64; 3],
    /// Shell normal; on the midsurface, the winning triangle's winding normal.
    pub normal: Vec3,
    /// Index in the original `triangles()` array, including deterministic tie ordering.
    pub triangle: u32,
    /// Finite nonnegative normalized weights in that triangle's original [a, b, c] order.
    pub barycentric: [f64; 3],
    /// BVH/triangle distance tests plus one final winner-attribute evaluation.
    pub work: usize,
}

#[derive(Debug)]
struct SurfaceData {
    vertices: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    half_thickness: f32,
    nodes: Vec<Node>,
    order: Vec<u32>,
    bounds: (Vec3, Vec3),
    bound: (Vec3, f32),
}

#[derive(Debug)]
struct Node {
    lo: Vec3,
    hi: Vec3,
    left: u32,
    right: u32,
    start: u32,
    count: u32,
}

impl TriangleSurface {
    /// Build an immutable BVH without altering the source vertex or triangle order.
    ///
    /// Requires finite vertices, positive finite half thickness, nonempty topology, valid
    /// indices, nondegenerate unique faces and at most two incident faces per edge. Boundary
    /// edges, disconnected pieces and either triangle winding are supported. This does not
    /// certify self-intersection-free cloth: that is a separate simulation/authoring check.
    pub fn new(vertices: Vec<Vec3>, triangles: Vec<[u32; 3]>, half_thickness: f32) -> Result<Self, String> {
        if vertices.len() < 3 || vertices.len() > MAX_SURFACE_VERTICES {
            return Err(format!("surface requires 3..={MAX_SURFACE_VERTICES} vertices"));
        }
        if triangles.is_empty() || triangles.len() > MAX_SURFACE_TRIANGLES {
            return Err(format!("surface requires 1..={MAX_SURFACE_TRIANGLES} triangles"));
        }
        if !half_thickness.is_finite() || half_thickness <= 0.0 {
            return Err("surface half thickness must be finite and positive".into());
        }
        if let Some(index) = vertices.iter().position(|v| !finite(*v)) {
            return Err(format!("surface vertex {index} must be finite"));
        }
        let mut faces = BTreeSet::new();
        let mut edges = BTreeMap::<(u32, u32), u32>::new();
        let mut lo = Vec3::splat(f32::INFINITY);
        let mut hi = Vec3::splat(f32::NEG_INFINITY);
        for (index, triangle) in triangles.iter().enumerate() {
            if triangle.iter().any(|&i| i as usize >= vertices.len()) {
                return Err(format!("surface triangle {index} has an out-of-range vertex index"));
            }
            let [a, b, c] = triangle.map(|i| D3::from(vertices[i as usize]));
            if (b - a).cross(c - a).length_sq() == 0.0 {
                return Err(format!("surface triangle {index} is degenerate"));
            }
            let mut sorted = *triangle;
            sorted.sort_unstable();
            if !faces.insert(sorted) {
                return Err(format!("surface triangle {index} duplicates a face"));
            }
            for [a, b] in [[triangle[0], triangle[1]], [triangle[1], triangle[2]], [triangle[2], triangle[0]]] {
                let count = edges.entry((a.min(b), a.max(b))).or_default();
                *count += 1;
                if *count > 2 {
                    return Err(format!("surface edge ({}, {}) is nonmanifold", a.min(b), a.max(b)));
                }
            }
            for &vertex in triangle {
                lo = lo.min(vertices[vertex as usize]);
                hi = hi.max(vertices[vertex as usize]);
            }
        }
        let center = ((D3::from(lo) + D3::from(hi)) * 0.5).to_vec();
        // Enclose every corner even when rounding moves the f32 center off the exact midpoint.
        let span = D3(
            (f64::from(hi.x) - f64::from(center.x)).abs().max((f64::from(lo.x) - f64::from(center.x)).abs()),
            (f64::from(hi.y) - f64::from(center.y)).abs().max((f64::from(lo.y) - f64::from(center.y)).abs()),
            (f64::from(hi.z) - f64::from(center.z)).abs().max((f64::from(lo.z) - f64::from(center.z)).abs()),
        );
        let radius = upper(span.length_sq().sqrt() + f64::from(half_thickness));
        let thick = D3::splat(f64::from(half_thickness));
        let lower_bounds = D3::from(lo) - thick;
        let upper_bounds = D3::from(hi) + thick;
        let bounds = (
            Vec3::new(lower(lower_bounds.0), lower(lower_bounds.1), lower(lower_bounds.2)),
            Vec3::new(upper(upper_bounds.0), upper(upper_bounds.1), upper(upper_bounds.2)),
        );
        if !finite(center) || !radius.is_finite() || !finite(bounds.0) || !finite(bounds.1) {
            return Err("surface thickness/bounds exceed finite f32 geometry range".into());
        }
        let mut nodes = Vec::with_capacity(triangles.len().div_ceil(LEAF_SIZE) * 2);
        let mut order: Vec<_> = (0..triangles.len() as u32).collect();
        build(&vertices, &triangles, &mut order, 0, &mut nodes);
        Ok(Self(Arc::new(SurfaceData {
            vertices,
            triangles,
            half_thickness,
            nodes,
            order,
            bounds,
            bound: (center, radius),
        })))
    }

    pub fn vertices(&self) -> &[Vec3] {
        &self.0.vertices
    }
    pub fn triangles(&self) -> &[[u32; 3]] {
        &self.0.triangles
    }
    pub fn half_thickness(&self) -> f32 {
        self.0.half_thickness
    }

    /// Return original triangle IDs whose AABBs, expanded by `padding` on each
    /// axis, overlap the closed query box. Padding is explicit: the surface's
    /// shell thickness is not added implicitly. Extra conservative candidates
    /// are permitted; an overlapping expanded triangle AABB is never excluded.
    ///
    /// Work is precharged before every node/triangle AABB test. Invalid input or
    /// exhausted work returns an error, never a partial candidate list. Results
    /// are sorted by original triangle ID independently of BVH traversal order.
    pub fn triangle_candidates_bounded(
        &self,
        min: Vec3,
        max: Vec3,
        padding: f32,
        max_work: usize,
    ) -> Result<SurfaceCandidates, String> {
        self.triangle_candidates_counted(min, max, padding, max_work).map_err(|error| error.message)
    }

    /// Candidate-query variant retaining completed traversal work on rejection.
    /// Callers with a shared budget must charge either the success or failure
    /// count before attempting subsequent work. Geometry, ordering, and work
    /// units are identical to `triangle_candidates_bounded`.
    pub fn triangle_candidates_counted(
        &self,
        min: Vec3,
        max: Vec3,
        padding: f32,
        max_work: usize,
    ) -> Result<SurfaceCandidates, SurfaceCandidateFailure> {
        if !finite(min)
            || !finite(max)
            || !padding.is_finite()
            || padding < 0.0
            || min.x > max.x
            || min.y > max.y
            || min.z > max.z
        {
            return Err(SurfaceCandidateFailure {
                work: 0,
                message: "surface candidate query requires finite ordered bounds and nonnegative finite padding".into(),
            });
        }
        let mut work = 0;
        let mut charge = || {
            if work >= max_work {
                return Err(SurfaceCandidateFailure {
                    work,
                    message: format!("surface candidate work budget exceeded ({max_work})"),
                });
            }
            work += 1;
            Ok(())
        };
        let overlaps = |lo: Vec3, hi: Vec3| {
            [(lo.x, hi.x, min.x, max.x), (lo.y, hi.y, min.y, max.y), (lo.z, hi.z, min.z, max.z)].into_iter().all(
                |(lo, hi, min, max)| {
                    // Compare gaps instead of rounding expanded bounds inward
                    // (or overflowing f32 additions). Padding is exact in f64.
                    // Monotone f64 rounding cannot move a true gap <= padding
                    // above that exactly representable comparison threshold.
                    f64::from(lo) - f64::from(max) <= f64::from(padding)
                        && f64::from(min) - f64::from(hi) <= f64::from(padding)
                },
            )
        };
        let mut pending = vec![0_u32];
        let mut triangles = vec![];
        while let Some(index) = pending.pop() {
            charge()?;
            let node = &self.0.nodes[index as usize];
            if !overlaps(node.lo, node.hi) {
                continue;
            }
            if node.count == 0 {
                pending.push(node.right);
                pending.push(node.left);
            } else {
                for &id in &self.0.order[node.start as usize..(node.start + node.count) as usize] {
                    charge()?;
                    let [a, b, c] = self.0.triangles[id as usize].map(|i| self.0.vertices[i as usize]);
                    if overlaps(a.min(b).min(c), a.max(b).max(c)) {
                        triangles.push(id);
                    }
                }
            }
        }
        triangles.sort_unstable();
        Ok(SurfaceCandidates { triangles, work })
    }
    /// Physical shell AABB, including the round rim at open borders.
    pub fn bounds(&self) -> (Vec3, Vec3) {
        self.0.bounds
    }
    /// Conservative local-space sphere, including thickness.
    pub fn bound(&self) -> (Vec3, f32) {
        self.0.bound
    }

    /// Exact nearest-triangle Euclidean distance minus half thickness (rounded to f32).
    /// A nonfinite query returns positive infinity instead of invalidating the field fold.
    pub fn distance(&self, point: Vec3) -> f32 {
        if !finite(point) {
            return f32::INFINITY;
        }
        let (squared, _, _) = self.nearest(D3::from(point));
        (squared.sqrt() - f64::from(self.0.half_thickness)) as f32
    }

    /// The same exact distance as [`Self::distance`], with a strict work limit.
    ///
    /// One unit is charged before each BVH node AABB distance test and each triangle
    /// distance test. The two child AABB tests used to order traversal each count,
    /// as do their repeated AABB tests when those nodes are later popped for pruning.
    /// Nonfinite points and exhausted budgets return an error without a partial result.
    pub fn distance_bounded(&self, point: Vec3, max_work: usize) -> Result<SurfaceDistance, String> {
        if !finite(point) {
            return Err("surface distance query point must be finite".into());
        }
        let mut work = 0;
        let (squared, _, _) = self.nearest_counted(D3::from(point), || {
            if work >= max_work {
                return Err(format!("surface distance work budget exceeded ({max_work})"));
            }
            work += 1;
            Ok(())
        })?;
        Ok(SurfaceDistance { distance: (squared.sqrt() - f64::from(self.0.half_thickness)) as f32, work })
    }

    /// Find the nearest authored triangle and its interpolation coordinates.
    ///
    /// Traversal and tie ordering match [`Self::distance_bounded`]. One additional
    /// unit is charged before reconstructing the winner's barycentrics and normal.
    /// Successful distances and normals match [`Self::distance`] and [`Self::normal`]
    /// bit for bit. `closest_point` lies on the midsurface, with shell thickness
    /// accounted for only in `distance` and `normal`. Coordinates are exactly the
    /// local or world coordinate system supplied when constructing this surface.
    ///
    /// Nonfinite queries, exhausted budgets, unrepresentable distances and unsupported
    /// attribute precision return an error without a partial hit. Barycentric roundoff
    /// within 128 f64 epsilons is normalized; reconstruction is checked against the
    /// unchanged f64 traversal point, componentwise using that coordinate's magnitude
    /// plus the longest triangle edge as the relative scale. There is no absolute
    /// world-unit tolerance floor.
    pub fn closest_hit_bounded(&self, point: Vec3, max_work: usize) -> Result<SurfaceHit, String> {
        if !finite(point) {
            return Err("surface hit query point must be finite".into());
        }
        let p = D3::from(point);
        let mut work = 0;
        let mut charge = || {
            if work >= max_work {
                return Err(format!("surface hit work budget exceeded ({max_work})"));
            }
            work += 1;
            Ok(())
        };
        let (squared, nearest, triangle) = self.nearest_counted(p, &mut charge)?;
        charge()?;
        let distance = (squared.sqrt() - f64::from(self.0.half_thickness)) as f32;
        if !distance.is_finite() {
            return Err("surface hit distance exceeds finite f32 range".into());
        }
        let vertices = self.0.triangles[triangle as usize].map(|i| D3::from(self.0.vertices[i as usize]));
        let [a, b, c] = vertices;
        let winding = (b - a).cross(c - a);
        let barycentric = closest_barycentric(p, nearest, vertices, winding)?;
        let normal = if squared > 0.0 {
            ((p - nearest) * (1.0 / squared.sqrt())).to_vec()
        } else {
            (winding * (1.0 / winding.length_sq().sqrt())).to_vec()
        };
        if !finite(normal) {
            return Err("surface hit normal exceeds supported floating-point precision".into());
        }
        Ok(SurfaceHit {
            distance,
            closest_point: [nearest.0, nearest.1, nearest.2],
            normal,
            triangle,
            barycentric,
            work,
        })
    }

    /// Shell normal from the exact closest surface point; on the medial surface, where
    /// the signed-distance gradient is undefined, use the nearest triangle's winding normal.
    pub fn normal(&self, point: Vec3) -> Vec3 {
        if !finite(point) {
            return Vec3::ZERO;
        }
        let p = D3::from(point);
        let (squared, nearest, triangle) = self.nearest(p);
        if squared > 0.0 {
            return ((p - nearest) * (1.0 / squared.sqrt())).to_vec();
        }
        let [a, b, c] = self.0.triangles[triangle as usize].map(|i| D3::from(self.0.vertices[i as usize]));
        let normal = (b - a).cross(c - a);
        (normal * (1.0 / normal.length_sq().sqrt())).to_vec()
    }

    fn nearest(&self, p: D3) -> (f64, D3, u32) {
        match self.nearest_counted(p, || Ok::<(), std::convert::Infallible>(())) {
            Ok(nearest) => nearest,
            Err(never) => match never {},
        }
    }

    fn nearest_counted<E>(&self, p: D3, mut charge: impl FnMut() -> Result<(), E>) -> Result<(f64, D3, u32), E> {
        let mut best = (f64::INFINITY, D3::splat(0.0), u32::MAX);
        // Median splits and <=131072 triangles cap the depth below 18, independent of
        // input order/geometry. DFS needs at most one deferred sibling per level.
        let mut stack = [0u32; 32];
        let mut len = 1;
        while len > 0 {
            len -= 1;
            let node = &self.0.nodes[stack[len] as usize];
            charge()?;
            if box_distance_squared(p, node.lo, node.hi) > best.0 {
                continue;
            }
            if node.count > 0 {
                for &index in &self.0.order[node.start as usize..(node.start + node.count) as usize] {
                    charge()?;
                    let [a, b, c] = self.0.triangles[index as usize].map(|i| D3::from(self.0.vertices[i as usize]));
                    let nearest = closest(p, a, b, c);
                    let squared = (p - nearest).length_sq();
                    if squared < best.0 || (squared == best.0 && index < best.2) {
                        best = (squared, nearest, index);
                    }
                }
            } else {
                let left = &self.0.nodes[node.left as usize];
                let right = &self.0.nodes[node.right as usize];
                charge()?;
                let left_squared = box_distance_squared(p, left.lo, left.hi);
                charge()?;
                let right_squared = box_distance_squared(p, right.lo, right.hi);
                let (near, far) =
                    if left_squared <= right_squared { (node.left, node.right) } else { (node.right, node.left) };
                stack[len] = far;
                stack[len + 1] = near;
                len += 2;
            }
        }
        Ok(best)
    }
}

fn build(vertices: &[Vec3], triangles: &[[u32; 3]], order: &mut [u32], start: usize, nodes: &mut Vec<Node>) -> u32 {
    let mut lo = Vec3::splat(f32::INFINITY);
    let mut hi = Vec3::splat(f32::NEG_INFINITY);
    for &index in order.iter() {
        for &vertex in &triangles[index as usize] {
            lo = lo.min(vertices[vertex as usize]);
            hi = hi.max(vertices[vertex as usize]);
        }
    }
    let index = nodes.len() as u32;
    nodes.push(Node { lo, hi, left: 0, right: 0, start: start as u32, count: order.len() as u32 });
    if order.len() > LEAF_SIZE {
        let extent = D3::from(hi) - D3::from(lo);
        let axis = if extent.0 >= extent.1 && extent.0 >= extent.2 {
            0
        } else if extent.1 >= extent.2 {
            1
        } else {
            2
        };
        let centroid = |index: u32| {
            let [a, b, c] = triangles[index as usize].map(|i| D3::from(vertices[i as usize]));
            let sum = a + b + c;
            match axis {
                0 => sum.0,
                1 => sum.1,
                _ => sum.2,
            }
        };
        let mid = order.len() / 2;
        order.select_nth_unstable_by(mid, |a, b| centroid(*a).total_cmp(&centroid(*b)).then(a.cmp(b)));
        let (left, right) = order.split_at_mut(mid);
        let left = build(vertices, triangles, left, start, nodes);
        let right = build(vertices, triangles, right, start + mid, nodes);
        nodes[index as usize].left = left;
        nodes[index as usize].right = right;
        nodes[index as usize].count = 0;
    }
    index
}

/// Interior projection or closest point on an edge. All arithmetic is f64 over f32
/// geometry, avoiding underflow for small cloth triangles and overflow for large scenes.
fn closest(p: D3, a: D3, b: D3, c: D3) -> D3 {
    let n = (b - a).cross(c - a);
    let plane = p - n * ((p - a).dot(n) / n.length_sq());
    if (b - a).cross(plane - a).dot(n) >= 0.0
        && (c - b).cross(plane - b).dot(n) >= 0.0
        && (a - c).cross(plane - c).dot(n) >= 0.0
    {
        return plane;
    }
    let edge = |a: D3, b: D3| {
        let ab = b - a;
        a + ab * ((p - a).dot(ab) / ab.length_sq()).clamp(0.0, 1.0)
    };
    let mut nearest = edge(a, b);
    for candidate in [edge(b, c), edge(c, a)] {
        if (p - candidate).length_sq() < (p - nearest).length_sq() {
            nearest = candidate;
        }
    }
    nearest
}

/// Recover the original query's winning feature with exactly the same branch and
/// edge-tie ordering as `closest`. Keep the traversal point authoritative: its
/// absolute-coordinate rounding must not be amplified into spurious negative edge
/// weights. Cross areas avoid the nearly cancelling Gram determinant for interiors.
fn closest_barycentric(query: D3, point: D3, vertices: [D3; 3], normal: D3) -> Result<[f64; 3], String> {
    const TOLERANCE: f64 = 128.0 * f64::EPSILON;
    let [a, b, c] = vertices;
    let area_squared = normal.length_sq();
    let plane = query - normal * ((query - a).dot(normal) / area_squared);
    let mut weights = if (b - a).cross(plane - a).dot(normal) >= 0.0
        && (c - b).cross(plane - b).dot(normal) >= 0.0
        && (a - c).cross(plane - c).dot(normal) >= 0.0
    {
        // The query's normal component cancels in these cross products. Avoid
        // subtracting a rounded absolute plane point from a nearby triangle vertex.
        let ap = query - a;
        let v = ap.cross(c - a).dot(normal) / area_squared;
        let w = (b - a).cross(ap).dot(normal) / area_squared;
        [1.0 - v - w, v, w]
    } else {
        let edge = |i: usize, j: usize| {
            let a = vertices[i];
            let ab = vertices[j] - a;
            let t = ((query - a).dot(ab) / ab.length_sq()).clamp(0.0, 1.0);
            let mut weights = [0.0; 3];
            weights[i] = 1.0 - t;
            weights[j] = t;
            (a + ab * t, weights)
        };
        let mut nearest = edge(0, 1);
        for candidate in [edge(1, 2), edge(2, 0)] {
            if (query - candidate.0).length_sq() < (query - nearest.0).length_sq() {
                nearest = candidate;
            }
        }
        nearest.1
    };
    if weights.iter().any(|value| !value.is_finite() || *value < -TOLERANCE || *value > 1.0 + TOLERANCE) {
        return Err("surface hit barycentrics exceed supported floating-point precision".into());
    }
    for value in &mut weights {
        // Canonicalize negative zero as well as tiny boundary roundoff.
        *value = if *value <= 0.0 { 0.0 } else { value.min(1.0) };
    }
    let sum = weights.iter().sum::<f64>();
    for value in &mut weights {
        *value /= sum;
    }
    let largest = (0..3).max_by(|&i, &j| weights[i].total_cmp(&weights[j])).expect("three weights");
    weights[largest] = 1.0 - (weights[(largest + 1) % 3] + weights[(largest + 2) % 3]);
    if weights.iter().any(|value| !value.is_finite() || *value < 0.0 || *value > 1.0)
        || (weights.iter().sum::<f64>() - 1.0).abs() > TOLERANCE
    {
        return Err("surface hit barycentric normalization exceeds supported floating-point precision".into());
    }
    let reconstructed = a * weights[0] + b * weights[1] + c * weights[2];
    let edge_scale = (b - a).length_sq().max((c - a).length_sq()).max((c - b).length_sq()).sqrt();
    for (original, actual, a, b, c) in [
        (point.0, reconstructed.0, a.0, b.0, c.0),
        (point.1, reconstructed.1, a.1, b.1, c.1),
        (point.2, reconstructed.2, a.2, b.2, c.2),
    ] {
        // The traversal's plane projection can leave a few rounding bits on an
        // otherwise zero coordinate axis. Triangle scale admits those bits without
        // imposing an absolute floor that would erase tiny geometry distinctions.
        let scale = original.abs().max(a.abs()).max(b.abs()).max(c.abs()) + edge_scale;
        if !original.is_finite() || !actual.is_finite() || (actual - original).abs() > TOLERANCE * scale {
            return Err("surface hit barycentrics cannot reconstruct the closest point at supported precision".into());
        }
    }
    Ok(weights)
}

fn box_distance_squared(p: D3, lo: Vec3, hi: Vec3) -> f64 {
    let component = |p: f64, lo: f32, hi: f32| (f64::from(lo) - p).max(p - f64::from(hi)).max(0.0);
    D3(component(p.0, lo.x, hi.x), component(p.1, lo.y, hi.y), component(p.2, lo.z, hi.z)).length_sq()
}
fn finite(v: Vec3) -> bool {
    v.x.is_finite() && v.y.is_finite() && v.z.is_finite()
}
fn lower(value: f64) -> f32 {
    let f = value as f32;
    if f64::from(f) > value {
        f.next_down()
    } else {
        f
    }
}
fn upper(value: f64) -> f32 {
    let f = value as f32;
    if f64::from(f) < value {
        f.next_up()
    } else {
        f
    }
}

#[derive(Clone, Copy)]
struct D3(f64, f64, f64);
impl D3 {
    fn splat(v: f64) -> Self {
        Self(v, v, v)
    }
    fn dot(self, b: Self) -> f64 {
        self.0 * b.0 + self.1 * b.1 + self.2 * b.2
    }
    fn cross(self, b: Self) -> Self {
        Self(self.1 * b.2 - self.2 * b.1, self.2 * b.0 - self.0 * b.2, self.0 * b.1 - self.1 * b.0)
    }
    fn length_sq(self) -> f64 {
        self.dot(self)
    }
    fn to_vec(self) -> Vec3 {
        Vec3::new(self.0 as f32, self.1 as f32, self.2 as f32)
    }
}
impl From<Vec3> for D3 {
    fn from(v: Vec3) -> Self {
        Self(f64::from(v.x), f64::from(v.y), f64::from(v.z))
    }
}
impl std::ops::Add for D3 {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self(self.0 + b.0, self.1 + b.1, self.2 + b.2)
    }
}
impl std::ops::Sub for D3 {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Self(self.0 - b.0, self.1 - b.1, self.2 - b.2)
    }
}
impl std::ops::Mul<f64> for D3 {
    type Output = Self;
    fn mul(self, b: f64) -> Self {
        Self(self.0 * b, self.1 * b, self.2 * b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> TriangleSurface {
        TriangleSurface::new(
            vec![
                Vec3::new(-1.0, -1.0, 0.0),
                Vec3::new(1.0, -1.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(-1.0, 1.0, 0.0),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
            0.025,
        )
        .unwrap()
    }

    #[test]
    fn physical_shell_has_two_sides_and_round_open_border_without_inferred_interior() {
        let surface = square();
        for sign in [-1.0, 1.0] {
            assert!((surface.distance(Vec3::new(0.2, -0.3, sign * 0.125)) - 0.1).abs() < 1e-7);
            assert!((surface.distance(Vec3::new(0.2, -0.3, sign * 0.025))).abs() < 1e-7);
            assert!(surface.distance(Vec3::new(0.2, -0.3, sign * 0.01)) < 0.0);
            assert!((surface.normal(Vec3::new(0.2, -0.3, sign * 0.025)) - Vec3::new(0.0, 0.0, sign)).length() < 1e-6);
        }
        assert_eq!(surface.distance(Vec3::ZERO), -0.025);
        assert!((surface.distance(Vec3::new(1.1, 0.0, 0.0)) - 0.075).abs() < 1e-7);
        assert!((surface.distance(Vec3::new(1.3, 1.4, 0.0)) - 0.475).abs() < 1e-6);
        assert!(surface.distance(Vec3::new(0.0, 0.0, -2.0)) > 0.0, "open sheet must not invent a filled interior");
        let (lo, hi) = surface.bounds();
        assert!(lo.x <= -1.025 && lo.y <= -1.025 && lo.z <= -0.025);
        assert!(hi.x >= 1.025 && hi.y >= 1.025 && hi.z >= 0.025);
    }

    #[test]
    fn sharply_bent_shared_edge_has_no_crack_and_preserves_both_open_sides() {
        let vertices = vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)];
        let triangles = vec![[0, 1, 2], [0, 3, 1]];
        let surface = TriangleSurface::new(vertices.clone(), triangles.clone(), 0.01).unwrap();
        for step in 1..99 {
            let x = step as f32 * 0.01;
            assert_eq!(surface.distance(Vec3::new(x, 0.0, 0.0)), -0.01);
            assert!(surface.distance(Vec3::new(x, 0.005, 0.005)) < 0.0);
        }
        assert!(surface.distance(Vec3::new(0.1, 0.3, 0.3)) > 0.28, "space between folded faces remains empty");
        assert_eq!(surface.vertices(), vertices);
        assert_eq!(surface.triangles(), triangles);
        let cloned = surface.clone();
        assert!(Arc::ptr_eq(&surface.0, &cloned.0));
        assert_eq!(cloned.distance(Vec3::new(0.1, 0.3, 0.3)), surface.distance(Vec3::new(0.1, 0.3, 0.3)));
    }

    #[test]
    fn bvh_matches_brute_force_on_bent_mesh_across_faces_edges_and_distant_points() {
        let n = 24;
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        for y in 0..=n {
            for x in 0..=n {
                let (x, y) = (x as f32 * 0.05, y as f32 * 0.05);
                vertices.push(Vec3::new(x, y, 0.15 * (6.0 * x).sin() + 0.1 * (7.0 * y).cos()));
            }
        }
        for y in 0..n {
            for x in 0..n {
                let a = y * (n + 1) + x;
                triangles.extend([[a, a + 1, a + n + 2], [a, a + n + 2, a + n + 1]]);
            }
        }
        let surface = TriangleSurface::new(vertices, triangles, 0.004).unwrap();
        let mut seed = 19823u32;
        let mut sample = || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            (seed >> 8) as f32 / (1u32 << 24) as f32
        };
        for index in 0..500 {
            let point = if index < surface.vertices().len() / 4 {
                surface.vertices()[index * 4]
            } else {
                Vec3::new(sample() * 2.0 - 0.4, sample() * 2.0 - 0.4, sample() * 4.0 - 2.0)
            };
            let p = D3::from(point);
            let squared = surface
                .triangles()
                .iter()
                .map(|triangle| {
                    let [a, b, c] = triangle.map(|i| D3::from(surface.vertices()[i as usize]));
                    (p - closest(p, a, b, c)).length_sq()
                })
                .fold(f64::INFINITY, f64::min);
            let expected = (squared.sqrt() - f64::from(surface.half_thickness())) as f32;
            assert_eq!(surface.distance(point).to_bits(), expected.to_bits(), "BVH differs at {point:?}");
            let (center, radius) = surface.bound();
            let lower = (point - center).length() - radius;
            assert!(lower <= expected + 1e-6, "bounding sphere must conservatively enclose thickened geometry");
        }
    }

    #[test]
    fn rejects_invalid_geometry_and_handles_finite_geometry_at_small_and_large_scales() {
        let valid = vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
        for invalid in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(TriangleSurface::new(valid.clone(), vec![[0, 1, 2]], invalid).is_err());
        }
        for triangles in [
            vec![],
            vec![[0, 1, 3]],
            vec![[0, 0, 1]],
            vec![[0, 1, 2], [2, 1, 0]],
            vec![[0, 1, 2]; MAX_SURFACE_TRIANGLES + 1],
        ] {
            assert!(TriangleSurface::new(valid.clone(), triangles, 0.01).is_err());
        }
        assert!(TriangleSurface::new(vec![], vec![[0, 1, 2]], 0.01).is_err());
        assert!(TriangleSurface::new(vec![Vec3::ZERO; MAX_SURFACE_VERTICES + 1], vec![[0, 1, 2]], 0.01).is_err());
        let mut collinear = valid.clone();
        collinear[2] = Vec3::new(2.0, 0.0, 0.0);
        assert!(TriangleSurface::new(collinear, vec![[0, 1, 2]], 0.01).unwrap_err().contains("degenerate"));
        let mut nonmanifold = valid.clone();
        nonmanifold.extend([Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, -1.0, 0.0)]);
        assert!(TriangleSurface::new(nonmanifold, vec![[0, 1, 2], [0, 3, 1], [0, 1, 4]], 0.01)
            .unwrap_err()
            .contains("nonmanifold"));
        for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut vertices = valid.clone();
            vertices[1].y = invalid;
            assert!(TriangleSurface::new(vertices, vec![[0, 1, 2]], 0.01).is_err());
            assert_eq!(square().distance(Vec3::new(invalid, 0.0, 0.0)), f32::INFINITY);
            assert_eq!(square().normal(Vec3::new(invalid, 0.0, 0.0)), Vec3::ZERO);
        }
        for scale in [1e-15, 1.0, 1e15] {
            let surface =
                TriangleSurface::new(valid.iter().map(|p| p.scale(scale)).collect(), vec![[0, 1, 2]], 0.01 * scale)
                    .unwrap();
            let expected = 0.09 * scale;
            let distance = surface.distance(Vec3::new(0.2, 0.2, 0.1).scale(scale));
            assert!((distance - expected).abs() <= expected * 1e-5, "{scale}: {distance} != {expected}");
            assert!((surface.normal(Vec3::new(0.2, 0.2, 0.1).scale(scale)) - Vec3::new(0.0, 0.0, 1.0)).length() < 1e-5);
        }
    }
}
