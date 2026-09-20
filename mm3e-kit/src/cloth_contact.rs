//! Bounded self-contact for the XPBD triangle network.
//!
//! Point/triangle and segment/segment barycentric constraints distribute normal
//! and Coulomb tangential corrections using inverse masses. These primitive
//! pairs and their mass weighting follow the cloth contact construction in
//! Bridson, Fedkiw and Anderson (2002), https://www.cs.ubc.ca/~rbridson/docs/cloth2002.pdf.
//! This is not their fail-safe impact-zone algorithm. Conservative advancement
//! uses a conservative bound on the closing speed along the closest features'
//! separating axis to avoid stepping over thickness contact on a linear
//! trajectory. A convergence/work
//! failure rejects the entire fixed step. Constraint iterations remain finite;
//! the final local clearance deficit is reported, including pinned conflicts.

use super::{
    error, finite, ClothError, ClothSeam, ClothSelfContactReport, ClothSettings, DVec, Edge, Vec3, MAX_TRIANGLES,
    MAX_VERTICES,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Primitive {
    Vertex(usize),
    Triangle(usize),
    Edge(usize),
}

#[derive(Clone, Debug)]
pub(super) struct Topology {
    triangles: Vec<[usize; 3]>,
    edges: Vec<[usize; 2]>,
    neighbors: Vec<Vec<usize>>,
}

#[derive(Default)]
pub(super) struct Stats {
    pub vertex_triangle: u64,
    pub edge_edge: u64,
    pub candidates: u64,
    pub work: u64,
}

impl Stats {
    fn spend(&mut self, amount: u64, settings: &ClothSettings) -> Result<(), ClothError> {
        self.work = self.work.checked_add(amount).ok_or_else(|| error("self collision work counter overflow"))?;
        if self.work > settings.self_collision_max_work {
            return Err(error("self collision work budget exhausted; fixed step was not committed"));
        }
        Ok(())
    }
    fn candidate(&mut self, settings: &ClothSettings) -> Result<(), ClothError> {
        self.candidates += 1;
        if self.candidates > u64::from(settings.self_collision_max_candidates) {
            return Err(error("self collision candidate budget exhausted; fixed step was not committed"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Pair {
    VertexTriangle([usize; 4]),
    EdgeEdge([usize; 4]),
}

impl Pair {
    fn indices(self) -> [usize; 4] {
        match self {
            Self::VertexTriangle(indices) | Self::EdgeEdge(indices) => indices,
        }
    }
    fn sample(self, points: [DVec; 4]) -> Contact {
        let (coefficients, fallback) = match self {
            Self::VertexTriangle(_) => {
                let bary = triangle_barycentric(points[0], [points[1], points[2], points[3]]);
                ([1.0, -bary[0], -bary[1], -bary[2]], separating_axis(points[2] - points[1], points[3] - points[1]))
            }
            Self::EdgeEdge(_) => {
                let [s, t] = segment_parameters(points);
                ([1.0 - s, s, -(1.0 - t), -t], separating_axis(points[1] - points[0], points[3] - points[2]))
            }
        };
        let difference = weighted_difference(points, coefficients);
        let distance = difference.length();
        Contact {
            coefficients,
            normal: if distance > 1e-15 { difference * (1.0 / distance) } else { fallback },
            distance,
        }
    }

    fn current_bounds_overlap(self, positions: &[DVec], padding: f64) -> bool {
        let indices = self.indices();
        let (a, b): (&[usize], &[usize]) = match self {
            Self::VertexTriangle(_) => (&indices[..1], &indices[1..]),
            Self::EdgeEdge(_) => (&indices[..2], &indices[2..]),
        };
        (0..3).all(|axis| {
            let a_min = a.iter().map(|&i| positions[i].0[axis]).fold(f64::INFINITY, f64::min) - padding;
            let a_max = a.iter().map(|&i| positions[i].0[axis]).fold(f64::NEG_INFINITY, f64::max) + padding;
            let b_min = b.iter().map(|&i| positions[i].0[axis]).fold(f64::INFINITY, f64::min) - padding;
            let b_max = b.iter().map(|&i| positions[i].0[axis]).fold(f64::NEG_INFINITY, f64::max) + padding;
            a_min <= b_max && b_min <= a_max
        })
    }
    fn separating_plane_advance(self, current: [DVec; 4], end: [DVec; 4], normal: DVec, thickness: f64) -> Option<f64> {
        // The closest features define support planes separating these convex
        // primitives. Every A/B vertex projection gap is affine in time. Find
        // the earliest time ANY such gap can reach thickness on this fixed axis.
        // This remains conservative under triangle/edge rotation and avoids
        // the loose global velocity bound's stall on far, fast moving vertices.
        let (a_vertices, b_vertices): (&[usize], &[usize]) = match self {
            Self::VertexTriangle(_) => (&[0], &[1, 2, 3]),
            Self::EdgeEdge(_) => (&[0, 1], &[2, 3]),
        };
        let mut fraction: Option<f64> = None;
        for &a in a_vertices {
            for &b in b_vertices {
                let gap = (current[a] - current[b]).dot(normal);
                let end_gap = (end[a] - end[b]).dot(normal);
                if end_gap <= thickness {
                    let time = ((gap - thickness) / (gap - end_gap)).clamp(0.0, 1.0);
                    fraction = Some(fraction.map_or(time, |earliest| earliest.min(time)));
                }
            }
        }
        fraction
    }
}

#[derive(Clone, Copy)]
struct Contact {
    coefficients: [f64; 4],
    normal: DVec,
    distance: f64,
}

#[derive(Clone, Copy)]
struct Bounds {
    minimum: [f64; 3],
    maximum: [f64; 3],
    primitive: Primitive,
}

impl Bounds {
    fn overlaps(self, other: Self) -> bool {
        (0..3).all(|axis| self.minimum[axis] <= other.maximum[axis] && other.minimum[axis] <= self.maximum[axis])
    }
}

impl Topology {
    pub fn add_seams(&mut self, seams: &[ClothSeam]) {
        for seam in seams {
            let [a, b] = seam.vertices.map(|i| i as usize);
            self.neighbors[a].push(b);
            self.neighbors[b].push(a);
        }
        for neighbors in &mut self.neighbors {
            neighbors.sort_unstable();
            neighbors.dedup();
        }
    }
    pub fn new(vertex_count: usize, triangles: &[[u32; 3]], edges: &[Edge]) -> Self {
        let edges: Vec<_> = edges.iter().map(|edge| edge.vertices).collect();
        Self::from_edges(vertex_count, triangles, edges)
    }

    pub fn from_triangles(vertex_count: usize, triangles: &[[u32; 3]]) -> Result<Self, ClothError> {
        if !(3..=MAX_VERTICES).contains(&vertex_count) || triangles.is_empty() || triangles.len() > MAX_TRIANGLES {
            return Err(error("cloth requires 3..=100000 vertices and 1..=200000 triangles"));
        }
        // Only topology is validated here: a playback pose can legitimately be
        // degenerate, and its zero-distance contacts still need to be measured.
        // As in solver construction, mesh limits bound this O(T log T) setup.
        let mut incidence = BTreeMap::<(usize, usize), (u8, bool)>::new();
        let mut seen = BTreeSet::new();
        let mut used = vec![false; vertex_count];
        for triangle in triangles {
            let [a, b, c] = triangle.map(|v| v as usize);
            if [a, b, c].iter().any(|v| *v >= vertex_count) || a == b || b == c || a == c {
                return Err(error("triangle index is out of bounds or repeated"));
            }
            let mut key = [a, b, c];
            key.sort_unstable();
            if !seen.insert(key) {
                return Err(error("duplicate triangle"));
            }
            for (i, j) in [(a, b), (b, c), (c, a)] {
                used[i] = true;
                let entry = incidence.entry((i.min(j), i.max(j))).or_insert((0, i < j));
                if entry.0 >= 2 {
                    return Err(error("nonmanifold cloth edge has more than two incident triangles"));
                }
                if entry.0 == 1 && entry.1 == (i < j) {
                    return Err(error("neighboring cloth triangles have inconsistent winding"));
                }
                entry.0 += 1;
            }
        }
        if used.iter().any(|v| !v) {
            return Err(error("isolated cloth vertex is not part of any triangle"));
        }
        Ok(Self::from_edges(vertex_count, triangles, incidence.into_keys().map(|(a, b)| [a, b]).collect()))
    }

    fn from_edges(vertex_count: usize, triangles: &[[u32; 3]], edges: Vec<[usize; 2]>) -> Self {
        let mut neighbors = vec![Vec::new(); vertex_count];
        for [a, b] in &edges {
            neighbors[*a].push(*b);
            neighbors[*b].push(*a);
        }
        for adjacent in &mut neighbors {
            adjacent.sort_unstable();
        }
        Self { triangles: triangles.iter().map(|triangle| triangle.map(|v| v as usize)).collect(), edges, neighbors }
    }

    pub fn measure(&self, positions: &[Vec3], settings: &ClothSettings) -> Result<ClothSelfContactReport, ClothError> {
        ClothSettings { self_collision: true, ..*settings }.validate()?;
        if positions.len() != self.neighbors.len() || positions.iter().any(|p| !finite(*p)) {
            return Err(error("self-contact query requires one finite position per topology vertex"));
        }
        let positions = positions.iter().copied().map(DVec::from).collect::<Vec<_>>();
        let mut stats = Stats::default();
        let maximum = self.penetration(&positions, settings, &mut stats)?;
        if !maximum.is_finite() || maximum > f64::from(f32::MAX) {
            return Err(error("self-contact query exceeded finite representable range"));
        }
        Ok(ClothSelfContactReport { max_penetration: maximum as f32, candidates: stats.candidates, work: stats.work })
    }

    fn adjacent(&self, a: usize, b: usize) -> bool {
        a == b || self.neighbors[a].binary_search(&b).is_ok()
    }

    fn pair(&self, a: Primitive, b: Primitive) -> Option<Pair> {
        match (a, b) {
            (Primitive::Vertex(v), Primitive::Triangle(t)) | (Primitive::Triangle(t), Primitive::Vertex(v)) => {
                let [a, b, c] = self.triangles[t];
                if [a, b, c].iter().any(|other| self.adjacent(v, *other)) {
                    None
                } else {
                    Some(Pair::VertexTriangle([v, a, b, c]))
                }
            }
            (Primitive::Edge(a), Primitive::Edge(b)) => self.edge_pair(a, b),
            _ => None,
        }
    }

    fn edge_pair(&self, a: usize, b: usize) -> Option<Pair> {
        let [a0, a1] = self.edges[a.min(b)];
        let [b0, b1] = self.edges[a.max(b)];
        if [a0, a1].iter().any(|a| [b0, b1].iter().any(|b| self.adjacent(*a, *b))) {
            None
        } else {
            Some(Pair::EdgeEdge([a0, a1, b0, b1]))
        }
    }

    fn candidates(
        &self,
        positions: &[DVec],
        previous: &[DVec],
        settings: &ClothSettings,
        stats: &mut Stats,
    ) -> Result<Vec<Pair>, ClothError> {
        // Swept AABBs contain every linearly interpolated primitive. Allocation
        // is bounded by the validated mesh limits; comparison and pair counts
        // are charged even for rejected overlaps/topological neighbors.
        stats.spend((positions.len() + self.triangles.len() + self.edges.len()) as u64, settings)?;
        let padding = f64::from(settings.self_collision_thickness) * 0.5;
        let mut boxes = Vec::with_capacity(positions.len() + self.triangles.len() + self.edges.len());
        let mut add = |primitive, vertices: &[usize]| {
            let mut minimum = [f64::INFINITY; 3];
            let mut maximum = [f64::NEG_INFINITY; 3];
            for &v in vertices {
                for axis in 0..3 {
                    minimum[axis] = minimum[axis].min(positions[v].0[axis]).min(previous[v].0[axis]);
                    maximum[axis] = maximum[axis].max(positions[v].0[axis]).max(previous[v].0[axis]);
                }
            }
            boxes.push(Bounds {
                minimum: minimum.map(|x| x - padding),
                maximum: maximum.map(|x| x + padding),
                primitive,
            });
        };
        for (i, p) in positions.iter().enumerate() {
            p.to_vec()?;
            previous[i].to_vec()?;
            add(Primitive::Vertex(i), &[i]);
        }
        for (i, triangle) in self.triangles.iter().enumerate() {
            add(Primitive::Triangle(i), triangle);
        }
        for (i, edge) in self.edges.iter().enumerate() {
            add(Primitive::Edge(i), edge);
        }
        // Use the longest coordinate extent, with stable axis tie-breaking.
        let extent = std::array::from_fn::<_, 3, _>(|axis| {
            boxes.iter().map(|b| b.maximum[axis]).fold(f64::NEG_INFINITY, f64::max)
                - boxes.iter().map(|b| b.minimum[axis]).fold(f64::INFINITY, f64::min)
        });
        let axis = (1..3).fold(0, |best, i| if extent[i] > extent[best] { i } else { best });
        boxes.sort_unstable_by(|a, b| a.minimum[axis].total_cmp(&b.minimum[axis]).then(a.primitive.cmp(&b.primitive)));
        let mut result = Vec::new();
        let mut active_vertices = Vec::<Bounds>::new();
        let mut active_triangles = Vec::<Bounds>::new();
        let mut active_edges = Vec::<Bounds>::new();
        for &current in &boxes {
            // Separate interval joins never compare impossible vertex/vertex,
            // vertex/edge, triangle/triangle or triangle/edge contact types.
            // Expired intervals are removed after one charged comparison.
            let opposite = match current.primitive {
                Primitive::Vertex(_) => &mut active_triangles,
                Primitive::Triangle(_) => &mut active_vertices,
                Primitive::Edge(_) => &mut active_edges,
            };
            let mut j = 0;
            while j < opposite.len() {
                stats.spend(1, settings)?;
                let other = opposite[j];
                if other.maximum[axis] < current.minimum[axis] {
                    opposite.swap_remove(j);
                    continue;
                }
                if current.overlaps(other) {
                    if let Some(pair) = self.pair(current.primitive, other.primitive) {
                        stats.candidate(settings)?;
                        result.push(pair);
                    }
                }
                j += 1;
            }
            match current.primitive {
                Primitive::Vertex(_) => active_vertices.push(current),
                Primitive::Triangle(_) => active_triangles.push(current),
                Primitive::Edge(_) => active_edges.push(current),
            }
        }
        // Narrowphase Gauss-Seidel ordering follows topology ids, so the same
        // contact set has the same ordering if the longest broadphase axis flips.
        result.sort_unstable_by_key(|pair| match pair {
            Pair::VertexTriangle(i) => (0, *i),
            Pair::EdgeEdge(i) => (1, *i),
        });
        Ok(result)
    }

    fn current_candidates_from_sweep(
        &self,
        swept: &[Pair],
        positions: &[DVec],
        settings: &ClothSettings,
        stats: &mut Stats,
    ) -> Result<Vec<Pair>, ClothError> {
        let padding = f64::from(settings.self_collision_thickness) * 0.5;
        let mut current = Vec::new();
        for &pair in swept {
            stats.spend(1, settings)?;
            if pair.current_bounds_overlap(positions, padding) {
                stats.candidate(settings)?;
                current.push(pair);
            }
        }
        Ok(current)
    }

    pub fn project(
        &self,
        positions: &mut [DVec],
        previous: &[DVec],
        inverse_masses: &[f32],
        settings: &ClothSettings,
        stats: &mut Stats,
    ) -> Result<(), ClothError> {
        // First preserve the approach side along the sweep, then relinearize
        // clearance at the current closest features. An impact normal can be
        // stale after the same triangle has tilted in another contact response.
        let swept_candidates = self.candidates(positions, previous, settings, stats)?;
        let mut unchanged = true;
        for swept in [true, false] {
            let discrete_candidates;
            let candidates = if swept {
                &swept_candidates
            } else {
                // Current boxes are subsets of swept boxes ONLY if the swept
                // response has not changed any positions. Filter that exact
                // superset instead of rebuilding and sorting all primitives.
                discrete_candidates = if unchanged {
                    self.current_candidates_from_sweep(&swept_candidates, positions, settings, stats)?
                } else {
                    self.candidates(positions, positions, settings, stats)?
                };
                &discrete_candidates
            };
            for &pair in candidates {
                let indices = pair.indices();
                let weights = indices.map(|i| f64::from(inverse_masses[i]));
                if weights.iter().all(|w| *w == 0.0) {
                    continue;
                }
                let from = indices.map(|i| previous[i]);
                let to = indices.map(|i| positions[i]);
                let contact = if swept {
                    let Some(contact) = first_contact(pair, from, to, settings, stats)? else {
                        continue;
                    };
                    contact
                } else {
                    stats.spend(1, settings)?;
                    let contact = pair.sample(to);
                    if contact.distance >= f64::from(settings.self_collision_thickness) {
                        continue;
                    }
                    contact
                };
                let gap = weighted_difference(to, contact.coefficients).dot(contact.normal);
                let deficit = f64::from(settings.self_collision_thickness) - gap;
                if deficit <= 0.0 {
                    continue;
                }
                let denominator: f64 = weights.iter().zip(contact.coefficients).map(|(w, c)| w * c * c).sum();
                if denominator <= 0.0 {
                    continue;
                }
                stats.spend(1, settings)?;
                unchanged = false;
                let lambda = deficit / denominator;
                let relative_motion =
                    weighted_difference(std::array::from_fn(|i| to[i] - from[i]), contact.coefficients);
                let tangent = relative_motion - contact.normal * relative_motion.dot(contact.normal);
                let tangent_length = tangent.length();
                let friction = if tangent_length > 0.0 {
                    tangent
                        * (-(tangent_length / denominator).min(f64::from(settings.friction_coefficient) * lambda)
                            / tangent_length)
                } else {
                    DVec::default()
                };
                let impulse = contact.normal * lambda + friction;
                for j in 0..4 {
                    let i = indices[j];
                    // Zero inverse mass receives exactly zero correction, including
                    // when a moving authored pin supplies the collision impulse.
                    if weights[j] > 0.0 {
                        positions[i] = positions[i] + impulse * (weights[j] * contact.coefficients[j]);
                    }
                }
                match pair {
                    Pair::VertexTriangle(_) => stats.vertex_triangle += 1,
                    Pair::EdgeEdge(_) => stats.edge_edge += 1,
                }
            }
        }
        Ok(())
    }

    pub fn penetration(
        &self,
        positions: &[DVec],
        settings: &ClothSettings,
        stats: &mut Stats,
    ) -> Result<f64, ClothError> {
        let mut maximum = 0.0_f64;
        for pair in self.candidates(positions, positions, settings, stats)? {
            stats.spend(1, settings)?;
            let contact = pair.sample(pair.indices().map(|i| positions[i]));
            maximum = maximum.max(f64::from(settings.self_collision_thickness) - contact.distance);
        }
        Ok(maximum)
    }
}

fn first_contact(
    pair: Pair,
    from: [DVec; 4],
    to: [DVec; 4],
    settings: &ClothSettings,
    stats: &mut Stats,
) -> Result<Option<Contact>, ClothError> {
    let movement: [DVec; 4] = std::array::from_fn(|i| to[i] - from[i]);
    let thickness = f64::from(settings.self_collision_thickness);
    // A small speculative skin avoids asymptotic advancement at grazing
    // contacts. Response still enforces the authored thickness, not the skin.
    let tolerance = thickness * 1e-3 + 1e-12;
    let mut t = 0.0;
    for _ in 0..1024 {
        stats.spend(1, settings)?;
        let current = std::array::from_fn(|i| from[i] + movement[i] * t);
        let contact = pair.sample(current);
        if !contact.distance.is_finite() {
            return Err(error("self collision produced nonfinite geometry; fixed step was not committed"));
        }
        if contact.distance <= thickness + tolerance {
            return Ok(Some(contact));
        }
        let Some(fraction) = pair.separating_plane_advance(current, to, contact.normal, thickness) else {
            return Ok(None);
        };
        let advance = (1.0 - t) * fraction;
        if t + advance <= t {
            return Err(error("self collision sweep could not advance numerically; fixed step was not committed"));
        }
        t += advance;
    }
    let last = pair.sample(std::array::from_fn(|i| from[i] + movement[i] * t));
    Err(ClothError(format!("self collision sweep convergence budget exhausted at t={t:.9}, distance={}, thickness={thickness}, indices={:?}; reduce fixed_dt or increase substeps", last.distance, pair.indices())))
}

fn weighted_difference(points: [DVec; 4], coefficients: [f64; 4]) -> DVec {
    // All contact coefficients sum to zero. Subtracting a common origin avoids
    // subtracting large weighted world positions when cloth is far from zero.
    (1..4).fold(DVec::default(), |sum, i| sum + (points[i] - points[0]) * coefficients[i])
}

fn separating_axis(a: DVec, b: DVec) -> DVec {
    let cross = a.cross(b);
    let cross_length = cross.length();
    if cross_length > 1e-12 * a.length() * b.length() && cross_length > 0.0 {
        return cross * (1.0 / cross_length);
    }
    let edge = if a.length_sq() > b.length_sq() { a } else { b };
    if edge.length_sq() == 0.0 {
        return DVec([1.0, 0.0, 0.0]);
    }
    let axis = (1..3).fold(0, |best, i| if edge.0[i].abs() < edge.0[best].abs() { i } else { best });
    let mut unit = DVec::default();
    unit.0[axis] = 1.0;
    let normal = edge.cross(unit);
    normal * (1.0 / normal.length())
}

fn segment_parameters(p: [DVec; 4]) -> [f64; 2] {
    let d0 = p[1] - p[0];
    let d1 = p[3] - p[2];
    let r = p[0] - p[2];
    let a = d0.length_sq();
    let e = d1.length_sq();
    let f = d1.dot(r);
    if a == 0.0 && e == 0.0 {
        return [0.0, 0.0];
    }
    if a == 0.0 {
        return [0.0, (f / e).clamp(0.0, 1.0)];
    }
    let c = d0.dot(r);
    if e == 0.0 {
        return [(-c / a).clamp(0.0, 1.0), 0.0];
    }
    let b = d0.dot(d1);
    // Cross products retain the small nonparallel component that a*e-b*b
    // loses to cancellation for nearly parallel segments.
    let cross = d0.cross(d1);
    let denominator = cross.length_sq();
    let mut s = if denominator > 0.0 { (d1.cross(r).dot(cross) / denominator).clamp(0.0, 1.0) } else { 0.0 };
    let mut t = (b * s + f) / e;
    if t < 0.0 {
        t = 0.0;
        s = (-c / a).clamp(0.0, 1.0);
    }
    if t > 1.0 {
        t = 1.0;
        s = ((b - c) / a).clamp(0.0, 1.0);
    }
    [s, t]
}

fn triangle_barycentric(point: DVec, p: [DVec; 3]) -> [f64; 3] {
    let ab = p[1] - p[0];
    let ac = p[2] - p[0];
    let normal = ab.cross(ac);
    let area_sq = normal.length_sq();
    if area_sq > 0.0 {
        let ap = point - p[0];
        let v = ap.cross(ac).dot(normal) / area_sq;
        let w = ab.cross(ap).dot(normal) / area_sq;
        if v >= 0.0 && w >= 0.0 && v + w <= 1.0 {
            return [1.0 - v - w, v, w];
        }
    }
    // The closest point outside the projected triangle is on an edge. This
    // also handles exactly collapsed current triangles without inventing area.
    let mut best = (f64::INFINITY, [1.0, 0.0, 0.0]);
    for [a, b] in [[0, 1], [1, 2], [2, 0]] {
        let edge = p[b] - p[a];
        let t =
            if edge.length_sq() > 0.0 { ((point - p[a]).dot(edge) / edge.length_sq()).clamp(0.0, 1.0) } else { 0.0 };
        let distance = (point - p[a] - edge * t).length_sq();
        if distance < best.0 {
            let mut bary = [0.0; 3];
            bary[a] = 1.0 - t;
            bary[b] = t;
            best = (distance, bary);
        }
    }
    best.1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn random_point(seed: &mut u64) -> DVec {
        DVec(std::array::from_fn(|_| {
            *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((*seed >> 32) as u32 as f64 / u32::MAX as f64 - 0.5) * 4.0
        }))
    }

    #[test]
    fn closest_feature_normals_are_supporting_planes_for_both_convex_primitives() {
        let mut seed = 71;
        for pair in [Pair::VertexTriangle([0, 1, 2, 3]), Pair::EdgeEdge([0, 1, 2, 3])] {
            for _ in 0..2000 {
                let p = std::array::from_fn(|_| random_point(&mut seed));
                let contact = pair.sample(p);
                let (a, b): (&[usize], &[usize]) = match pair {
                    Pair::VertexTriangle(_) => (&[0], &[1, 2, 3]),
                    Pair::EdgeEdge(_) => (&[0, 1], &[2, 3]),
                };
                // Independent optimality condition: the closest normal must
                // separate ALL convex combinations, not just the chosen pair.
                let gap = a
                    .iter()
                    .flat_map(|&i| b.iter().map(move |&j| (p[i] - p[j]).dot(contact.normal)))
                    .fold(f64::INFINITY, f64::min);
                assert!((gap - contact.distance).abs() < 1e-10, "gap {gap}, distance {}", contact.distance);
                assert!(contact.coefficients.iter().all(|c| c.is_finite()));
            }
        }
    }

    #[test]
    fn conservative_sweeps_do_not_miss_dense_sampled_deforming_primitive_contacts() {
        let mut seed = 871;
        let settings = ClothSettings { self_collision: true, self_collision_thickness: 0.05, ..Default::default() };
        let mut hit_count = 0;
        for pair in [Pair::VertexTriangle([0, 1, 2, 3]), Pair::EdgeEdge([0, 1, 2, 3])] {
            for _ in 0..200 {
                let from: [DVec; 4] = std::array::from_fn(|_| random_point(&mut seed));
                let to: [DVec; 4] = std::array::from_fn(|_| random_point(&mut seed));
                let detected = first_contact(pair, from, to, &settings, &mut Stats::default()).unwrap();
                let sampled = (0..=100).any(|i| {
                    let t = f64::from(i) / 100.0;
                    pair.sample(std::array::from_fn(|j| from[j] * (1.0 - t) + to[j] * t)).distance
                        <= f64::from(settings.self_collision_thickness)
                });
                if sampled {
                    hit_count += 1;
                    assert!(detected.is_some(), "sweep missed a sampled crossing");
                }
            }
        }
        assert!(hit_count > 40, "the randomized control must actually include collisions");
    }

    #[test]
    fn nearly_parallel_edges_skinny_triangles_and_collapses_keep_valid_closest_features() {
        let edges =
            [DVec([-1.0, 0.0, 0.0]), DVec([1.0, 0.0, 0.0]), DVec([-1.0, 1e-10, 0.001]), DVec([1.0, -1e-10, 0.001])];
        let [s, t] = segment_parameters(edges);
        assert!((s - 0.5).abs() < 1e-12 && (t - 0.5).abs() < 1e-12);
        let skinny = [DVec([0.0, 0.0, 0.0]), DVec([1e8, 0.0, 0.0]), DVec([1e8, 1e-6, 0.0])];
        let bary = triangle_barycentric(DVec([5e7, 2e-7, 0.003]), skinny);
        for (actual, expected) in bary.into_iter().zip([0.5, 0.3, 0.2]) {
            assert!((actual - expected).abs() < 1e-12);
        }
        let collapsed = [DVec::default(); 4];
        for pair in [Pair::VertexTriangle([0, 1, 2, 3]), Pair::EdgeEdge([0, 1, 2, 3])] {
            let contact = pair.sample(collapsed);
            assert_eq!(contact.distance, 0.0);
            assert!((contact.normal.length() - 1.0).abs() < 1e-12);
        }
    }

    #[test]
    fn fast_tangential_near_contact_sweeps_finish_without_creating_a_collision() {
        let settings = ClothSettings { self_collision: true, self_collision_thickness: 0.02, ..Default::default() };
        let from = [
            DVec([-1.0, 0.0201, 0.0]),
            DVec([-100.0, 0.0, -100.0]),
            DVec([100.0, 0.0, -100.0]),
            DVec([0.0, 0.0, 100.0]),
        ];
        let mut to = from;
        to[0].0[0] += 10.0;
        let mut stats = Stats::default();
        assert!(first_contact(Pair::VertexTriangle([0, 1, 2, 3]), from, to, &settings, &mut stats).unwrap().is_none());
        assert_eq!(stats.work, 1, "a fixed separating plane proves this slide clear");
    }

    #[test]
    fn partitioned_interval_join_and_unchanged_pose_reuse_match_exhaustive_primitive_pairs() {
        fn keys(pairs: &[Pair]) -> Vec<(u8, [usize; 4])> {
            pairs
                .iter()
                .map(|p| match *p {
                    Pair::VertexTriangle(i) => (0, i),
                    Pair::EdgeEdge(i) => (1, i),
                })
                .collect()
        }
        fn exhaustive(topology: &Topology, p: &[DVec], previous: &[DVec], padding: f64) -> Vec<(u8, [usize; 4])> {
            let overlaps = |a: &[usize], b: &[usize]| {
                (0..3).all(|axis| {
                    let bounds = |vertices: &[usize]| {
                        let values = vertices.iter().flat_map(|&i| [p[i].0[axis], previous[i].0[axis]]);
                        (
                            values.clone().fold(f64::INFINITY, f64::min) - padding,
                            values.fold(f64::NEG_INFINITY, f64::max) + padding,
                        )
                    };
                    let (lo_a, hi_a) = bounds(a);
                    let (lo_b, hi_b) = bounds(b);
                    lo_a <= hi_b && lo_b <= hi_a
                })
            };
            let mut result = Vec::new();
            for v in 0..p.len() {
                for t in 0..topology.triangles.len() {
                    if let Some(Pair::VertexTriangle(indices)) =
                        topology.pair(Primitive::Vertex(v), Primitive::Triangle(t))
                    {
                        if overlaps(&[v], &topology.triangles[t]) {
                            result.push((0, indices));
                        }
                    }
                }
            }
            for a in 0..topology.edges.len() {
                for b in a + 1..topology.edges.len() {
                    if let Some(Pair::EdgeEdge(indices)) = topology.edge_pair(a, b) {
                        if overlaps(&topology.edges[a], &topology.edges[b]) {
                            result.push((1, indices));
                        }
                    }
                }
            }
            result.sort_unstable();
            result
        }
        let disconnected = (0..6).map(|i| [i * 3, i * 3 + 1, i * 3 + 2]).collect::<Vec<_>>();
        let mut grid = Vec::new();
        for row in 0..3 {
            for column in 0..3 {
                let i = row * 4 + column;
                grid.extend([[i, i + 4, i + 1], [i + 1, i + 4, i + 5]]);
            }
        }
        let settings = ClothSettings { self_collision: true, self_collision_thickness: 0.05, ..Default::default() };
        let mut seed = 9329;
        for (n, triangles) in [(18, disconnected), (16, grid)] {
            let topology = Topology::from_triangles(n, &triangles).unwrap();
            for trial in 0..20 {
                let p = (0..n).map(|_| random_point(&mut seed)).collect::<Vec<_>>();
                let previous =
                    if trial % 2 == 0 { p.clone() } else { (0..n).map(|_| random_point(&mut seed)).collect() };
                let swept = topology.candidates(&p, &previous, &settings, &mut Stats::default()).unwrap();
                assert_eq!(
                    keys(&swept),
                    exhaustive(&topology, &p, &previous, f64::from(settings.self_collision_thickness) * 0.5)
                );
                let reused =
                    topology.current_candidates_from_sweep(&swept, &p, &settings, &mut Stats::default()).unwrap();
                let rebuilt = topology.candidates(&p, &p, &settings, &mut Stats::default()).unwrap();
                assert_eq!(keys(&reused), keys(&rebuilt));
                assert_eq!(
                    keys(&rebuilt),
                    exhaustive(&topology, &p, &p, f64::from(settings.self_collision_thickness) * 0.5)
                );
            }
        }
    }

    #[test]
    fn sewing_adjacency_does_not_expand_through_transitive_stitch_links() {
        let mut topology = Topology::from_triangles(9, &[[0, 1, 2], [3, 4, 5], [6, 7, 8]]).unwrap();
        topology.add_seams(&[
            ClothSeam { vertices: [0, 3], rest_length: 0.0, compliance: 0.0 },
            ClothSeam { vertices: [3, 6], rest_length: 0.0, compliance: 0.0 },
        ]);
        assert!(topology.adjacent(0, 3));
        assert!(topology.adjacent(3, 6));
        assert!(!topology.adjacent(0, 6));
        assert!(topology.pair(Primitive::Vertex(0), Primitive::Triangle(2)).is_some());
    }
}
