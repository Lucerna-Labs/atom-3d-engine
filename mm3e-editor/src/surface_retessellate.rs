//! Exact surface-preserving two-face retessellation of stored-f32 geometry.
//!
//! A flip changes neither coordinates nor the covered patch: exact coplanarity,
//! strict convexity and matching winding certify both diagonals triangulate the
//! same quad. An interval proof requires improved minimum triangle quality.
//! Both new faces receive the union of all old ancestor sets. This conservative
//! transitive lineage is not a single-parent attribute interpolation rule and
//! is separate from the simplicial vertex-map correspondence certificate.
use mm3e_kit::{
    exact_geometry::{self, Point},
    mesh_topology,
    meshing::{Mesh, MAX_MESH_TRIANGLES, MAX_MESH_VERTICES},
    surface_intersections, Vec3,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_PASSES: usize = 32;
pub const MAX_CANDIDATES: usize = 1_000_000;
pub const MAX_FACE_ANCESTORS: usize = 128;
pub const MAX_ANCESTOR_VALUES: usize = 4_194_304;
#[derive(Clone, Copy)]
pub struct Options {
    pub max_work: usize,
    pub max_passes: usize,
    pub max_candidates: usize,
}
#[derive(Clone, Debug, Serialize)]
pub struct Flip {
    pub faces: [usize; 2],
    pub old_triangles: [[u32; 3]; 2],
    pub new_triangles: [[u32; 3]; 2],
    pub boundary: [u32; 4],
    pub projection_axes: [usize; 2],
    pub orientation: i8,
    pub old_min_quality_upper: f64,
    pub new_min_quality_lower: f64,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct Report {
    pub work: usize,
    pub passes: usize,
    pub candidates: usize,
    pub initial_embedding_work: usize,
    pub initial_embedding_reused: bool,
    pub initial_certificate_match_work: usize,
    pub final_embedding_work: usize,
    pub output_certificate_work: usize,
    pub topology_work: usize,
    pub predicate_work: usize,
    pub quality_evaluations: usize,
    pub quality_cache_hits: usize,
    pub quality_cache_updates: usize,
    pub quality_cache_rollbacks: usize,
    pub rejected_nonplanar: usize,
    pub rejected_nonconvex: usize,
    pub rejected_existing_diagonal: usize,
    pub rejected_quality: usize,
    pub rejected_ancestry_bound: usize,
    pub flips: Vec<Flip>,
    /// Exact unchanged point set and patch coverage; no metric tolerance.
    pub geometric_displacement_m: f64,
}
pub struct RetessellatedMesh {
    pub mesh: Mesh,
    pub face_ancestors: Vec<Vec<usize>>,
    pub report: Report,
    pub embedding_certificate: Option<surface_intersections::EmbeddingCertificate>,
}
#[derive(Debug, Clone)]
pub struct Failure {
    pub work: usize,
    pub message: String,
}
impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (retessellation work {})", self.message, self.work)
    }
}
impl std::error::Error for Failure {}
struct Budget {
    used: usize,
    maximum: usize,
    quality_evaluations: usize,
}
impl Budget {
    fn charge(&mut self, amount: usize) -> Result<(), String> {
        if amount > self.maximum.saturating_sub(self.used) {
            return Err(format!(
                "surface retessellation exhausted work budget (used {}, requested {amount})",
                self.used
            ));
        }
        self.used += amount;
        Ok(())
    }
    fn remaining(&self) -> usize {
        self.maximum - self.used
    }
}
fn xyz(p: Vec3) -> [f64; 3] {
    [p.x, p.y, p.z].map(f64::from)
}
fn edge(a: u32, b: u32) -> (u32, u32) {
    (a.min(b), a.max(b))
}
fn triangle_edges([a, b, c]: [u32; 3]) -> [(u32, u32); 3] {
    [edge(a, b), edge(b, c), edge(c, a)]
}
#[derive(Clone, Copy)]
struct Interval {
    lo: f64,
    hi: f64,
}
impl Interval {
    fn point(v: f64) -> Self {
        Self { lo: v, hi: v }
    }
    fn add(self, b: Self) -> Self {
        Self { lo: (self.lo + b.lo).next_down(), hi: (self.hi + b.hi).next_up() }
    }
    fn sub(self, b: Self) -> Self {
        Self { lo: (self.lo - b.hi).next_down(), hi: (self.hi - b.lo).next_up() }
    }
    fn mul(self, b: Self) -> Self {
        let p = [self.lo * b.lo, self.lo * b.hi, self.hi * b.lo, self.hi * b.hi];
        Self {
            lo: p.into_iter().fold(f64::INFINITY, f64::min).next_down(),
            hi: p.into_iter().fold(f64::NEG_INFINITY, f64::max).next_up(),
        }
    }
    fn square(self) -> Self {
        let hi = self.lo.abs().max(self.hi.abs());
        let lo = if self.lo <= 0. && self.hi >= 0. { 0. } else { self.lo.abs().min(self.hi.abs()) };
        Self { lo: if lo == 0. { 0. } else { (lo * lo).next_down().max(0.) }, hi: (hi * hi).next_up() }
    }
}
fn quality(positions: &[Vec3], t: [u32; 3], budget: &mut Budget) -> Result<Interval, String> {
    budget.charge(100)?;
    budget.quality_evaluations += 1;
    let [a, b, c] = t.map(|v| xyz(positions[v as usize]).map(Interval::point));
    let sub = |b: [Interval; 3], a: [Interval; 3]| std::array::from_fn::<_, 3, _>(|i| b[i].sub(a[i]));
    let [ab, ac, bc] = [sub(b, a), sub(c, a), sub(c, b)];
    let cross = [
        ab[1].mul(ac[2]).sub(ab[2].mul(ac[1])),
        ab[2].mul(ac[0]).sub(ab[0].mul(ac[2])),
        ab[0].mul(ac[1]).sub(ab[1].mul(ac[0])),
    ];
    let sum_squares = |values: &[Interval]| values.iter().fold(Interval::point(0.), |sum, v| sum.add(v.square()));
    let numerator = sum_squares(&cross);
    let denominator = sum_squares(&ab).add(sum_squares(&ac)).add(sum_squares(&bc)).square();
    if denominator.lo <= 0. || !denominator.hi.is_finite() || !numerator.hi.is_finite() {
        return Ok(Interval { lo: 0., hi: f64::INFINITY });
    }
    Ok(Interval {
        lo: (numerator.lo.max(0.) / denominator.hi).next_down().max(0.),
        hi: (numerator.hi / denominator.lo).next_up(),
    })
}
/// Position coordinates never change during either retessellation operation.
/// A cached interval is keyed by its stable face slot AND actual index triple;
/// a different triple is never reused as if it represented the same face.
#[derive(Clone, Copy)]
struct QualityEntry {
    triangle: [u32; 3],
    value: Interval,
}
struct QualityCache {
    values: Vec<Option<QualityEntry>>,
    hits: usize,
    updates: usize,
    rollbacks: usize,
}
impl QualityCache {
    fn new(faces: usize, budget: &mut Budget) -> Result<Self, String> {
        budget.charge(faces)?;
        Ok(Self { values: vec![None; faces], hits: 0, updates: 0, rollbacks: 0 })
    }
    fn get(
        &mut self,
        face: usize,
        triangle: [u32; 3],
        positions: &[Vec3],
        budget: &mut Budget,
    ) -> Result<Interval, String> {
        budget.charge(4)?;
        if let Some(entry) = self.values[face] {
            if entry.triangle == triangle {
                self.hits += 1;
                return Ok(entry.value);
            }
        }
        let value = quality(positions, triangle, budget)?;
        self.store(face, triangle, value, false, budget)?;
        Ok(value)
    }
    fn store(
        &mut self,
        face: usize,
        triangle: [u32; 3],
        value: Interval,
        rollback: bool,
        budget: &mut Budget,
    ) -> Result<(), String> {
        budget.charge(4)?;
        self.values[face] = Some(QualityEntry { triangle, value });
        self.updates += 1;
        self.rollbacks += usize::from(rollback);
        Ok(())
    }
}
fn exact_point(
    vertex: u32,
    positions: &[Vec3],
    points: &mut [Option<Point>],
    budget: &mut Budget,
    report: &mut Report,
) -> Result<Point, String> {
    budget.charge(1)?;
    if let Some(point) = &points[vertex as usize] {
        return Ok(point.clone());
    }
    let v = xyz(positions[vertex as usize]);
    let result = match Point::from_expansions([&[v[0]], &[v[1]], &[v[2]]], &[1.], budget.remaining()) {
        Ok(value) => {
            budget.charge(value.work)?;
            report.predicate_work += value.work;
            value.point
        }
        Err(error) => {
            budget.charge(error.work)?;
            report.predicate_work += error.work;
            return Err(error.message);
        }
    };
    points[vertex as usize] = Some(result.clone());
    Ok(result)
}
fn orient(points: [&Point; 3], axes: [usize; 2], budget: &mut Budget, report: &mut Report) -> Result<i8, String> {
    match exact_geometry::orient2(points, axes, budget.remaining()) {
        Ok(value) => {
            budget.charge(value.work)?;
            report.predicate_work += value.work;
            Ok(value.sign)
        }
        Err(error) => {
            budget.charge(error.work)?;
            report.predicate_work += error.work;
            Err(error.message)
        }
    }
}
fn coplanar(points: [&Point; 4], budget: &mut Budget, report: &mut Report) -> Result<bool, String> {
    match exact_geometry::orient3(points, budget.remaining()) {
        Ok(value) => {
            budget.charge(value.work)?;
            report.predicate_work += value.work;
            Ok(value.sign == 0)
        }
        Err(error) => {
            budget.charge(error.work)?;
            report.predicate_work += error.work;
            Err(error.message)
        }
    }
}

pub fn retessellate(
    mesh: Mesh,
    face_ancestors: Vec<Vec<usize>>,
    options: Options,
) -> Result<RetessellatedMesh, Failure> {
    let mut budget = Budget { used: 0, maximum: options.max_work, quality_evaluations: 0 };
    inner(mesh, face_ancestors, options, &mut budget, None, false)
        .map_err(|message| Failure { work: budget.used, message })
}
/// Reuse only an exactly matching complete input proof; certify final arrays.
pub fn retessellate_certified(
    mesh: Mesh,
    face_ancestors: Vec<Vec<usize>>,
    options: Options,
    input: Option<&surface_intersections::EmbeddingCertificate>,
) -> Result<RetessellatedMesh, Failure> {
    let mut budget = Budget { used: 0, maximum: options.max_work, quality_evaluations: 0 };
    inner(mesh, face_ancestors, options, &mut budget, input, true)
        .map_err(|message| Failure { work: budget.used, message })
}
fn inner(
    mut mesh: Mesh,
    mut ancestors: Vec<Vec<usize>>,
    options: Options,
    budget: &mut Budget,
    input: Option<&surface_intersections::EmbeddingCertificate>,
    certify_output: bool,
) -> Result<RetessellatedMesh, String> {
    if mesh.positions.is_empty()
        || mesh.positions.len() > MAX_MESH_VERTICES
        || mesh.triangles.is_empty()
        || mesh.triangles.len() > MAX_MESH_TRIANGLES
        || ancestors.len() != mesh.triangles.len()
        || options.max_passes == 0
        || options.max_passes > MAX_PASSES
        || options.max_candidates == 0
        || options.max_candidates > MAX_CANDIDATES
    {
        return Err("surface retessellation requires bounded geometry, complete ancestry and positive bounded pass/candidate limits".into());
    }
    budget.charge(ancestors.len())?;
    let mut ancestry_count = 0;
    for ids in &mut ancestors {
        if ids.is_empty() || ids.len() > MAX_FACE_ANCESTORS {
            return Err("surface retessellation requires nonempty bounded ancestor sets".into());
        }
        budget.charge(ids.len() * (usize::BITS as usize - ids.len().leading_zeros() as usize + 2))?;
        ids.sort_unstable();
        ids.dedup();
        if ids.iter().any(|&v| v >= MAX_MESH_TRIANGLES) {
            return Err("surface retessellation ancestor index exceeds input face bound".into());
        }
        ancestry_count += ids.len();
        if ancestry_count > MAX_ANCESTOR_VALUES {
            return Err("surface retessellation total ancestor storage bound exceeded".into());
        }
    }
    let mut report = Report::default();
    let initial = match mesh_topology::validate(&mesh.triangles, budget.remaining()) {
        Ok(value) => {
            budget.charge(value.work)?;
            report.topology_work += value.work;
            value
        }
        Err(error) => {
            budget.charge(error.work)?;
            return Err(error.message);
        }
    };
    if let Some(certificate) = input {
        match certificate.matches(&mesh.positions, &mesh.triangles, budget.remaining()) {
            Ok(value) => {
                budget.charge(value.work)?;
                report.initial_certificate_match_work = value.work;
                report.initial_embedding_work += value.work;
                report.initial_embedding_reused = value.matches;
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        }
    }
    if !report.initial_embedding_reused {
        match surface_intersections::validate_counted(&mesh.positions, &mesh.triangles, budget.remaining()) {
            Ok(value) => {
                budget.charge(value.work)?;
                report.initial_embedding_work += value.work;
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        }
    }
    budget.charge(mesh.triangles.len() * 7 + mesh.positions.len())?;
    let mut incident = BTreeMap::<(u32, u32), BTreeSet<usize>>::new();
    for (i, &t) in mesh.triangles.iter().enumerate() {
        for e in triangle_edges(t) {
            incident.entry(e).or_default().insert(i);
        }
    }
    let mut points = vec![None; mesh.positions.len()];
    let mut quality_cache = QualityCache::new(mesh.triangles.len(), budget)?;
    let mut finished = false;
    for _ in 0..options.max_passes {
        report.passes += 1;
        budget.charge(incident.len() * 3)?;
        let mut candidates = Vec::with_capacity(incident.len());
        for (&key, faces) in &incident {
            let mut faces = faces.iter();
            let i = *faces.next().ok_or("retessellation empty edge incidence")?;
            let j = *faces.next().ok_or("retessellation open edge")?;
            let before = quality_cache
                .get(i, mesh.triangles[i], &mesh.positions, budget)?
                .hi
                .min(quality_cache.get(j, mesh.triangles[j], &mesh.positions, budget)?.hi);
            candidates.push((before, key));
        }
        budget
            .charge(candidates.len() * (usize::BITS as usize - candidates.len().max(1).leading_zeros() as usize + 1))?;
        candidates.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let before_count = report.flips.len();
        for (_, key) in candidates {
            budget.charge(1)?;
            let Some(faces) = incident.get(&key) else { continue };
            if report.candidates == options.max_candidates {
                return Err("surface retessellation exhausted candidate limit before convergence".into());
            }
            report.candidates += 1;
            budget.charge(20)?;
            let mut it = faces.iter();
            let i = *it.next().ok_or("retessellation lost edge incidence")?;
            let j = *it.next().ok_or("retessellation lost paired face")?;
            if it.next().is_some() {
                return Err("retessellation encountered overfull edge".into());
            }
            let old = [mesh.triangles[i], mesh.triangles[j]];
            let k = (0..3)
                .find(|&k| edge(old[0][k], old[0][(k + 1) % 3]) == key)
                .ok_or("retessellation edge/face mismatch")?;
            let [a, b, c] = [old[0][k], old[0][(k + 1) % 3], old[0][(k + 2) % 3]];
            let Some(d) = old[1].iter().copied().find(|&v| v != a && v != b) else {
                return Err("retessellation missing opposite vertex".into());
            };
            if c == d || incident.contains_key(&edge(c, d)) {
                report.rejected_existing_diagonal += 1;
                continue;
            }
            let boundary = [a, d, b, c];
            let next = [[c, a, d], [c, d, b]];
            let p = [
                exact_point(a, &mesh.positions, &mut points, budget, &mut report)?,
                exact_point(d, &mesh.positions, &mut points, budget, &mut report)?,
                exact_point(b, &mesh.positions, &mut points, budget, &mut report)?,
                exact_point(c, &mesh.positions, &mut points, budget, &mut report)?,
            ];
            let raw = boundary.map(|v| xyz(mesh.positions[v as usize]));
            budget.charge(12)?;
            let constant = (0..3).find(|&axis| raw.iter().all(|v| v[axis] == raw[0][axis]));
            if constant.is_none() && !coplanar([&p[0], &p[1], &p[2], &p[3]], budget, &mut report)? {
                report.rejected_nonplanar += 1;
                continue;
            }
            let mut projected = None;
            let axes_to_try = if let Some(axis) = constant {
                vec![[(axis + 1) % 3, (axis + 2) % 3]]
            } else {
                vec![[0, 1], [1, 2], [2, 0]]
            };
            for axes in axes_to_try {
                let sign = orient([&p[0], &p[1], &p[2]], axes, budget, &mut report)?;
                if sign != 0 {
                    projected = Some((axes, sign));
                    break;
                }
            }
            let Some((axes, sign)) = projected else {
                report.rejected_nonconvex += 1;
                continue;
            };
            let mut convex = true;
            for q in 1..4 {
                if orient([&p[q], &p[(q + 1) % 4], &p[(q + 2) % 4]], axes, budget, &mut report)? != sign {
                    convex = false;
                    break;
                }
            }
            if !convex {
                report.rejected_nonconvex += 1;
                continue;
            }
            // Old faces are the [0,2] diagonal, new faces the [1,3]
            // diagonal of this exact strictly convex planar boundary. Both
            // diagonal endpoints separate the remaining two vertices, so each
            // pair covers exactly the entire same oriented quad with no overlap
            // except its diagonal. Check each orientation independently too.
            for t in [[0, 2, 3], [2, 0, 1], [3, 0, 1], [3, 1, 2]] {
                if orient(t.map(|q| &p[q]), axes, budget, &mut report)? != sign {
                    convex = false;
                    break;
                }
            }
            if !convex {
                return Err("retessellation exact quad/triangle winding contract failed".into());
            }
            let old_qualities = [
                quality_cache.get(i, old[0], &mesh.positions, budget)?,
                quality_cache.get(j, old[1], &mesh.positions, budget)?,
            ];
            let old_quality = old_qualities[0].hi.min(old_qualities[1].hi);
            let new_qualities =
                [quality(&mesh.positions, next[0], budget)?, quality(&mesh.positions, next[1], budget)?];
            let new_quality = new_qualities[0].lo.min(new_qualities[1].lo);
            if !new_quality.is_finite() || new_quality <= old_quality {
                report.rejected_quality += 1;
                continue;
            }
            budget.charge(ancestors[i].len() + ancestors[j].len())?;
            let merged: Vec<_> =
                ancestors[i].iter().chain(&ancestors[j]).copied().collect::<BTreeSet<_>>().into_iter().collect();
            let new_count = ancestry_count - ancestors[i].len() - ancestors[j].len() + merged.len() * 2;
            if merged.len() > MAX_FACE_ANCESTORS || new_count > MAX_ANCESTOR_VALUES {
                report.rejected_ancestry_bound += 1;
                continue;
            }
            budget.charge(64 + merged.len() * 2)?;
            for (face, t) in [(i, old[0]), (j, old[1])] {
                for e in triangle_edges(t) {
                    let set = incident.get_mut(&e).ok_or("retessellation lost old adjacency")?;
                    if !set.remove(&face) {
                        return Err("retessellation stale old face incidence".into());
                    }
                    if set.is_empty() {
                        incident.remove(&e);
                    }
                }
            }
            for (face, t) in [(i, next[0]), (j, next[1])] {
                for e in triangle_edges(t) {
                    incident.entry(e).or_default().insert(face);
                }
            }
            quality_cache.store(i, next[0], new_qualities[0], false, budget)?;
            quality_cache.store(j, next[1], new_qualities[1], false, budget)?;
            mesh.triangles[i] = next[0];
            mesh.triangles[j] = next[1];
            ancestors[i] = merged.clone();
            ancestors[j] = merged;
            ancestry_count = new_count;
            report.flips.push(Flip {
                faces: [i, j],
                old_triangles: old,
                new_triangles: next,
                boundary,
                projection_axes: axes,
                orientation: sign,
                old_min_quality_upper: old_quality,
                new_min_quality_lower: new_quality,
            });
        }
        if report.flips.len() == before_count {
            finished = true;
            break;
        }
    }
    if !finished {
        return Err("surface retessellation exhausted pass limit before convergence".into());
    }
    let final_topology = match mesh_topology::validate(&mesh.triangles, budget.remaining()) {
        Ok(value) => {
            budget.charge(value.work)?;
            report.topology_work += value.work;
            value
        }
        Err(error) => {
            budget.charge(error.work)?;
            return Err(error.message);
        }
    };
    if final_topology != (mesh_topology::TopologyReport { work: final_topology.work, ..initial }) {
        return Err("surface retessellation changed the surface complex counts".into());
    }
    let embedding_certificate = if certify_output {
        match surface_intersections::validate_certified(&mesh.positions, &mesh.triangles, budget.remaining()) {
            Ok(value) => {
                budget.charge(value.work)?;
                report.final_embedding_work = value.work;
                report.output_certificate_work = value.certificate_work;
                Some(value.certificate)
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        }
    } else {
        match surface_intersections::validate_counted(&mesh.positions, &mesh.triangles, budget.remaining()) {
            Ok(value) => {
                budget.charge(value.work)?;
                report.final_embedding_work = value.work;
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        }
        None
    };
    report.quality_evaluations = budget.quality_evaluations;
    report.quality_cache_hits = quality_cache.hits;
    report.quality_cache_updates = quality_cache.updates;
    report.quality_cache_rollbacks = quality_cache.rollbacks;
    report.work = budget.used;
    Ok(RetessellatedMesh { mesh, face_ancestors: ancestors, report, embedding_certificate })
}

/// A separate graph-correspondence certificate for noncoplanar patches. All
/// coordinates stay fixed, but the covered surface may change by this bound.
#[derive(Clone, Debug, Serialize)]
pub struct WarpedFlip {
    pub faces: [usize; 2],
    pub old_triangles: [[u32; 3]; 2],
    pub new_triangles: [[u32; 3]; 2],
    pub boundary: [u32; 4],
    /// Exact dyadic f64 parameters on old [0,2] and new [1,3] diagonals.
    pub crossing_parameters: [f64; 2],
    pub projection_direction_enclosure: [[f64; 3]; 2],
    pub orientation: i8,
    pub old_min_quality_upper: f64,
    pub new_min_quality_lower: f64,
    pub local_separation_upper_m: f64,
    pub cumulative_surface_error_m: f64,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct BoundedReport {
    pub work: usize,
    pub passes: usize,
    pub candidates: usize,
    pub initial_embedding_work: usize,
    pub initial_embedding_reused: bool,
    pub initial_certificate_match_work: usize,
    pub certificate_snapshot_work: usize,
    pub embedding_validation_work: usize,
    pub final_embedding_work: usize,
    pub final_embedding_reused: bool,
    pub final_certificate_match_work: usize,
    pub topology_work: usize,
    pub predicate_work: usize,
    pub quality_evaluations: usize,
    pub quality_cache_hits: usize,
    pub quality_cache_updates: usize,
    pub quality_cache_rollbacks: usize,
    pub rejected_coplanar: usize,
    pub rejected_existing_diagonal: usize,
    pub rejected_quality: usize,
    pub rejected_projection: usize,
    pub rejected_error_bound: usize,
    pub rejected_ancestry_bound: usize,
    pub rolled_back_intersections: usize,
    pub max_surface_error_m: f64,
    pub allowed_surface_error_m: f64,
    pub flips: Vec<WarpedFlip>,
}
pub struct BoundedRetessellatedMesh {
    pub mesh: Mesh,
    pub face_ancestors: Vec<Vec<usize>>,
    pub face_error_bounds_m: Vec<f64>,
    pub report: BoundedReport,
    pub embedding_certificate: Option<surface_intersections::EmbeddingCertificate>,
}
#[derive(Clone, Copy)]
struct CrossingWitness {
    parameters: [f64; 2],
    direction: [Interval; 3],
    orientation: i8,
    separation: f64,
}
fn interval_cross(a: [Interval; 3], b: [Interval; 3]) -> [Interval; 3] {
    [a[1].mul(b[2]).sub(a[2].mul(b[1])), a[2].mul(b[0]).sub(a[0].mul(b[2])), a[0].mul(b[1]).sub(a[1].mul(b[0]))]
}
fn interval_sub(a: [Interval; 3], b: [Interval; 3]) -> [Interval; 3] {
    std::array::from_fn(|axis| a[axis].sub(b[axis]))
}
fn interval_dot(a: [Interval; 3], b: [Interval; 3]) -> Interval {
    (0..3).fold(Interval::point(0.), |sum, axis| sum.add(a[axis].mul(b[axis])))
}
fn interval_norm_upper(v: [Interval; 3]) -> Option<f64> {
    let a = v.map(|axis| axis.lo.abs().max(axis.hi.abs()));
    let scale = a.into_iter().fold(0., f64::max);
    if !scale.is_finite() {
        return None;
    }
    if scale == 0. {
        return Some(0.);
    }
    let mut sum = 0.;
    for value in a {
        if value != 0. {
            let scaled = (value / scale).next_up();
            sum = (sum + (scaled * scaled).next_up()).next_up();
        }
    }
    let bound = (scale * sum.sqrt().next_up()).next_up();
    bound.is_finite().then_some(bound)
}
/// The proposal arithmetic is not a proof. Each accepted parameter is an exact
/// dyadic witness, and all following interpolation/orientation/norm arithmetic
/// is enclosed outward. Projection along the exact crossing difference makes
/// the two selected strict-interior diagonal points coincide by construction.
fn crossing_witness(raw: [[f64; 3]; 4], budget: &mut Budget) -> Result<Option<CrossingWitness>, String> {
    budget.charge(150)?;
    let sub = |a: [f64; 3], b: [f64; 3]| std::array::from_fn::<_, 3, _>(|axis| a[axis] - b[axis]);
    let dot = |a: [f64; 3], b: [f64; 3]| (0..3).map(|axis| a[axis] * b[axis]).sum::<f64>();
    let [a, b, c, d] = raw;
    let u = sub(c, a);
    let v = sub(d, b);
    let w = sub(a, b);
    let uu = dot(u, u);
    let vv = dot(v, v);
    let uv = dot(u, v);
    let uw = dot(u, w);
    let vw = dot(v, w);
    let denominator = uu * vv - uv * uv;
    let proposed = if denominator > 0. && denominator.is_finite() {
        [(uv * vw - vv * uw) / denominator, (uu * vw - uv * uw) / denominator]
    } else {
        [0.5, 0.5]
    };
    let proposed = proposed.map(|v| if v.is_finite() { v } else { 0.5 });
    let mut candidates = vec![proposed];
    for power in [8, 16, 24, 32, 40, 48] {
        let margin = 2_f64.powi(-power);
        let candidate = proposed.map(|v| v.clamp(margin, 1. - margin));
        if !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    let p = raw.map(|point| point.map(Interval::point));
    let mut best = None;
    for parameters in candidates {
        budget.charge(300)?;
        if parameters.iter().any(|&t| t <= 0. || t >= 1. || !t.is_finite()) {
            continue;
        }
        let old: [Interval; 3] =
            std::array::from_fn(|axis| p[0][axis].add(p[2][axis].sub(p[0][axis]).mul(Interval::point(parameters[0]))));
        let new: [Interval; 3] =
            std::array::from_fn(|axis| p[1][axis].add(p[3][axis].sub(p[1][axis]).mul(Interval::point(parameters[1]))));
        let direction = interval_sub(old, new);
        if direction.iter().any(|v| !v.lo.is_finite() || !v.hi.is_finite()) {
            continue;
        }
        if !direction.iter().any(|v| v.lo > 0. || v.hi < 0.) {
            continue;
        }
        let mut signs = Vec::with_capacity(8);
        for q in 0..4 {
            signs.push(interval_dot(
                interval_cross(interval_sub(p[(q + 1) % 4], p[q]), interval_sub(p[(q + 2) % 4], p[q])),
                direction,
            ));
        }
        for [a, b, c] in [[0, 2, 3], [2, 0, 1], [3, 0, 1], [3, 1, 2]] {
            signs.push(interval_dot(interval_cross(interval_sub(p[b], p[a]), interval_sub(p[c], p[a])), direction));
        }
        let orientation = if signs.iter().all(|s| s.lo > 0. && s.hi.is_finite()) {
            1
        } else if signs.iter().all(|s| s.hi < 0. && s.lo.is_finite()) {
            -1
        } else {
            continue;
        };
        let Some(separation) = interval_norm_upper(direction) else { continue };
        if best.as_ref().is_none_or(|old: &CrossingWitness| separation < old.separation) {
            best = Some(CrossingWitness { parameters, direction, orientation, separation });
        }
    }
    Ok(best)
}
fn replace_pair(
    mesh: &mut Mesh,
    incident: &mut BTreeMap<(u32, u32), BTreeSet<usize>>,
    faces: [usize; 2],
    replacement: [[u32; 3]; 2],
    budget: &mut Budget,
) -> Result<(), String> {
    budget.charge(64)?;
    for face in faces {
        for e in triangle_edges(mesh.triangles[face]) {
            let entries = incident.get_mut(&e).ok_or("warped retessellation lost old adjacency")?;
            if !entries.remove(&face) {
                return Err("warped retessellation stale face incidence".into());
            }
            if entries.is_empty() {
                incident.remove(&e);
            }
        }
    }
    for (face, triangle) in faces.into_iter().zip(replacement) {
        mesh.triangles[face] = triangle;
        for e in triangle_edges(triangle) {
            incident.entry(e).or_default().insert(face);
        }
    }
    Ok(())
}
struct PendingWarp {
    flip: WarpedFlip,
    old_ancestors: [Vec<usize>; 2],
    old_bounds: [f64; 2],
    old_qualities: [Interval; 2],
}

/// Retessellate only noncoplanar patches under an explicit remaining geometric
/// error allowance. Exact retessellation remains a separate unchanged API.
///
/// Each old/new patch is a graph over the same strictly convex projection.
/// Their boundary agrees and their overlay has four affine regions, so the
/// maximum graph separation occurs at the crossing or a boundary vertex.
/// Boundary separation is zero. Local crossing bounds compose per output face
/// as max(previous pair bounds)+local bound, not a sum over disjoint patches.
pub fn retessellate_bounded(
    mesh: Mesh,
    face_ancestors: Vec<Vec<usize>>,
    options: Options,
    max_surface_error_m: f64,
) -> Result<BoundedRetessellatedMesh, Failure> {
    let mut budget = Budget { used: 0, maximum: options.max_work, quality_evaluations: 0 };
    bounded_inner(mesh, face_ancestors, options, max_surface_error_m, &mut budget, None, false)
        .map_err(|message| Failure { work: budget.used, message })
}
/// Graph-correct retessellation with exact immutable embedding-proof reuse.
pub fn retessellate_bounded_certified(
    mesh: Mesh,
    face_ancestors: Vec<Vec<usize>>,
    options: Options,
    max_surface_error_m: f64,
    input: Option<&surface_intersections::EmbeddingCertificate>,
) -> Result<BoundedRetessellatedMesh, Failure> {
    let mut budget = Budget { used: 0, maximum: options.max_work, quality_evaluations: 0 };
    bounded_inner(mesh, face_ancestors, options, max_surface_error_m, &mut budget, input, true)
        .map_err(|message| Failure { work: budget.used, message })
}
fn bounded_inner(
    mut mesh: Mesh,
    mut ancestors: Vec<Vec<usize>>,
    options: Options,
    maximum: f64,
    budget: &mut Budget,
    input: Option<&surface_intersections::EmbeddingCertificate>,
    certify_output: bool,
) -> Result<BoundedRetessellatedMesh, String> {
    if mesh.positions.is_empty()
        || mesh.positions.len() > MAX_MESH_VERTICES
        || mesh.triangles.is_empty()
        || mesh.triangles.len() > MAX_MESH_TRIANGLES
        || ancestors.len() != mesh.triangles.len()
        || options.max_passes == 0
        || options.max_passes > MAX_PASSES
        || options.max_candidates == 0
        || options.max_candidates > MAX_CANDIDATES
        || !maximum.is_finite()
        || maximum < 0.
    {
        return Err("bounded retessellation requires bounded geometry, complete ancestry, positive bounded limits and a finite nonnegative surface allowance".into());
    }
    budget.charge(ancestors.len())?;
    let mut ancestry_count = 0;
    for ids in &mut ancestors {
        if ids.is_empty() || ids.len() > MAX_FACE_ANCESTORS {
            return Err("bounded retessellation requires nonempty bounded ancestor sets".into());
        }
        budget.charge(ids.len() * (usize::BITS as usize - ids.len().leading_zeros() as usize + 2))?;
        ids.sort_unstable();
        ids.dedup();
        if ids.iter().any(|&id| id >= MAX_MESH_TRIANGLES) {
            return Err("bounded retessellation ancestor index exceeds input face bound".into());
        }
        ancestry_count += ids.len();
        if ancestry_count > MAX_ANCESTOR_VALUES {
            return Err("bounded retessellation total ancestor storage exceeded".into());
        }
    }
    let mut report = BoundedReport { allowed_surface_error_m: maximum, ..BoundedReport::default() };
    let initial_topology = match mesh_topology::validate(&mesh.triangles, budget.remaining()) {
        Ok(value) => {
            budget.charge(value.work)?;
            report.topology_work += value.work;
            value
        }
        Err(error) => {
            budget.charge(error.work)?;
            return Err(error.message);
        }
    };
    if let Some(certificate) = input {
        match certificate.matches(&mesh.positions, &mesh.triangles, budget.remaining()) {
            Ok(value) => {
                budget.charge(value.work)?;
                report.initial_certificate_match_work = value.work;
                report.initial_embedding_work += value.work;
                report.initial_embedding_reused = value.matches;
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        }
    }
    let prepared = if report.initial_embedding_reused {
        let certificate = input.ok_or("bounded retessellation lost its input proof")?;
        match surface_intersections::PreparedIntersections::new_certified(
            &mesh.positions,
            &mesh.triangles,
            certificate,
            budget.remaining(),
        ) {
            Ok(value) => {
                budget.charge(value.preparation_work)?;
                report.initial_embedding_work += value.preparation_work;
                // The trusted constructor repeats this exact complete match
                // against unchanged arrays, with identical successful cost.
                report.initial_certificate_match_work *= 2;
                value
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        }
    } else {
        let prepared = match surface_intersections::PreparedIntersections::new(
            &mesh.positions,
            &mesh.triangles,
            budget.remaining(),
        ) {
            Ok(value) => {
                budget.charge(value.preparation_work)?;
                report.initial_embedding_work += value.preparation_work;
                value
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        };
        match prepared.baseline_contacts(0, budget.remaining()) {
            Ok(value) => {
                budget.charge(value.work)?;
                report.initial_embedding_work += value.work;
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        }
        prepared
    };
    let mut current_certificate = if certify_output {
        if report.initial_embedding_reused {
            budget.charge(1)?;
            report.initial_embedding_work += 1;
            input.cloned()
        } else {
            match prepared.baseline_certificate(budget.remaining()) {
                Ok(value) => {
                    budget.charge(value.work)?;
                    report.initial_embedding_work += value.work;
                    report.certificate_snapshot_work += value.work;
                    Some(value.certificate)
                }
                Err(error) => {
                    budget.charge(error.work)?;
                    return Err(error.message);
                }
            }
        }
    } else {
        None
    };
    budget.charge(mesh.triangles.len() * 8 + mesh.positions.len())?;
    let mut incident = BTreeMap::<(u32, u32), BTreeSet<usize>>::new();
    for (face, &triangle) in mesh.triangles.iter().enumerate() {
        for e in triangle_edges(triangle) {
            incident.entry(e).or_default().insert(face);
        }
    }
    let mut bounds = vec![0.0_f64; mesh.triangles.len()];
    let mut points = vec![None; mesh.positions.len()];
    let mut quality_cache = QualityCache::new(mesh.triangles.len(), budget)?;
    let mut exact_report = Report::default();
    let mut finished = false;
    for _ in 0..options.max_passes {
        report.passes += 1;
        budget.charge(incident.len() * 3 + mesh.triangles.len())?;
        let mut candidates = Vec::with_capacity(incident.len());
        for (&e, faces) in &incident {
            let mut it = faces.iter();
            let i = *it.next().ok_or("bounded retessellation empty incidence")?;
            let j = *it.next().ok_or("bounded retessellation open edge")?;
            let q = quality_cache
                .get(i, mesh.triangles[i], &mesh.positions, budget)?
                .hi
                .min(quality_cache.get(j, mesh.triangles[j], &mesh.positions, budget)?.hi);
            candidates.push((q, e));
        }
        budget
            .charge(candidates.len() * (usize::BITS as usize - candidates.len().max(1).leading_zeros() as usize + 1))?;
        candidates.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut touched = vec![false; mesh.triangles.len()];
        let mut pending = Vec::<PendingWarp>::new();
        for (_, e) in candidates {
            budget.charge(1)?;
            let Some(faces) = incident.get(&e) else { continue };
            if report.candidates == options.max_candidates {
                return Err("bounded retessellation exhausted candidate limit before convergence".into());
            }
            report.candidates += 1;
            budget.charge(20)?;
            let mut it = faces.iter();
            let i = *it.next().ok_or("bounded retessellation lost incidence")?;
            let j = *it.next().ok_or("bounded retessellation open incidence")?;
            if it.next().is_some() {
                return Err("bounded retessellation overfull edge".into());
            }
            if touched[i] || touched[j] {
                continue;
            }
            let old = [mesh.triangles[i], mesh.triangles[j]];
            let k = (0..3)
                .find(|&k| edge(old[0][k], old[0][(k + 1) % 3]) == e)
                .ok_or("bounded retessellation edge/face mismatch")?;
            let [a, b, c] = [old[0][k], old[0][(k + 1) % 3], old[0][(k + 2) % 3]];
            let d =
                *old[1].iter().find(|&&v| v != a && v != b).ok_or("bounded retessellation missing opposite vertex")?;
            if c == d || incident.contains_key(&edge(c, d)) {
                report.rejected_existing_diagonal += 1;
                continue;
            }
            let boundary = [a, d, b, c];
            let next = [[c, a, d], [c, d, b]];
            let old_qualities = [
                quality_cache.get(i, old[0], &mesh.positions, budget)?,
                quality_cache.get(j, old[1], &mesh.positions, budget)?,
            ];
            let old_quality = old_qualities[0].hi.min(old_qualities[1].hi);
            let new_qualities =
                [quality(&mesh.positions, next[0], budget)?, quality(&mesh.positions, next[1], budget)?];
            let new_quality = new_qualities[0].lo.min(new_qualities[1].lo);
            if !new_quality.is_finite() || new_quality <= old_quality {
                report.rejected_quality += 1;
                continue;
            }
            let p = [
                exact_point(a, &mesh.positions, &mut points, budget, &mut exact_report)?,
                exact_point(d, &mesh.positions, &mut points, budget, &mut exact_report)?,
                exact_point(b, &mesh.positions, &mut points, budget, &mut exact_report)?,
                exact_point(c, &mesh.positions, &mut points, budget, &mut exact_report)?,
            ];
            if coplanar([&p[0], &p[1], &p[2], &p[3]], budget, &mut exact_report)? {
                report.rejected_coplanar += 1;
                continue;
            }
            let Some(witness) = crossing_witness(boundary.map(|v| xyz(mesh.positions[v as usize])), budget)? else {
                report.rejected_projection += 1;
                continue;
            };
            let cumulative = (bounds[i].max(bounds[j]) + witness.separation).next_up();
            if !cumulative.is_finite() || cumulative > maximum {
                report.rejected_error_bound += 1;
                continue;
            }
            budget.charge((ancestors[i].len() + ancestors[j].len()) * 2)?;
            let merged: Vec<_> =
                ancestors[i].iter().chain(&ancestors[j]).copied().collect::<BTreeSet<_>>().into_iter().collect();
            let new_count = ancestry_count - ancestors[i].len() - ancestors[j].len() + merged.len() * 2;
            if merged.len() > MAX_FACE_ANCESTORS || new_count > MAX_ANCESTOR_VALUES {
                report.rejected_ancestry_bound += 1;
                continue;
            }
            budget.charge(merged.len() * 2 + ancestors[i].len() + ancestors[j].len() + 16)?;
            let old_ancestors = [ancestors[i].clone(), ancestors[j].clone()];
            let old_bounds = [bounds[i], bounds[j]];
            replace_pair(&mut mesh, &mut incident, [i, j], next, budget)?;
            quality_cache.store(i, next[0], new_qualities[0], false, budget)?;
            quality_cache.store(j, next[1], new_qualities[1], false, budget)?;
            ancestors[i] = merged.clone();
            ancestors[j] = merged;
            bounds[i] = cumulative;
            bounds[j] = cumulative;
            ancestry_count = new_count;
            touched[i] = true;
            touched[j] = true;
            pending.push(PendingWarp {
                flip: WarpedFlip {
                    faces: [i, j],
                    old_triangles: old,
                    new_triangles: next,
                    boundary,
                    crossing_parameters: witness.parameters,
                    projection_direction_enclosure: [witness.direction.map(|v| v.lo), witness.direction.map(|v| v.hi)],
                    orientation: witness.orientation,
                    old_min_quality_upper: old_quality,
                    new_min_quality_lower: new_quality,
                    local_separation_upper_m: witness.separation,
                    cumulative_surface_error_m: cumulative,
                },
                old_ancestors,
                old_bounds,
                old_qualities,
            });
        }
        while !pending.is_empty() {
            let checked = if certify_output {
                prepared
                    .validate_retopologized_certified(&mesh.positions, &mesh.triangles, budget.remaining())
                    .map(|value| (value.work, value.certificate_work, Some(value.certificate)))
            } else {
                prepared
                    .validate_retopologized(&mesh.positions, &mesh.triangles, budget.remaining())
                    .map(|value| (value.work, 0, None))
            };
            match checked {
                Ok((work, certificate_work, certificate)) => {
                    budget.charge(work)?;
                    report.embedding_validation_work += work;
                    report.certificate_snapshot_work += certificate_work;
                    if certify_output {
                        current_certificate = certificate;
                    }
                    break;
                }
                Err(error) => {
                    budget.charge(error.work)?;
                    report.embedding_validation_work += error.work;
                    let Some(pair) = error.triangles else { return Err(error.message) };
                    budget.charge(pending.len() * 2)?;
                    let mut rejected = Vec::new();
                    for (index, change) in pending.iter().enumerate() {
                        if change.flip.faces.iter().any(|f| pair.contains(f)) {
                            rejected.push(index);
                        }
                    }
                    if rejected.is_empty() {
                        return Err(format!(
                            "bounded retessellation contact {pair:?} has no pending patch to roll back"
                        ));
                    }
                    for index in rejected.into_iter().rev() {
                        budget.charge(
                            pending.len()
                                + pending[index].old_ancestors[0].len()
                                + pending[index].old_ancestors[1].len()
                                + 8,
                        )?;
                        let change = pending.remove(index);
                        let [i, j] = change.flip.faces;
                        replace_pair(&mut mesh, &mut incident, [i, j], change.flip.old_triangles, budget)?;
                        quality_cache.store(i, change.flip.old_triangles[0], change.old_qualities[0], true, budget)?;
                        quality_cache.store(j, change.flip.old_triangles[1], change.old_qualities[1], true, budget)?;
                        ancestry_count = ancestry_count - ancestors[i].len() - ancestors[j].len()
                            + change.old_ancestors[0].len()
                            + change.old_ancestors[1].len();
                        let [a, b] = change.old_ancestors;
                        ancestors[i] = a;
                        ancestors[j] = b;
                        bounds[i] = change.old_bounds[0];
                        bounds[j] = change.old_bounds[1];
                        report.rolled_back_intersections += 1;
                    }
                }
            }
        }
        if pending.is_empty() {
            finished = true;
            break;
        }
        budget.charge(pending.len())?;
        report.flips.extend(pending.into_iter().map(|p| p.flip));
    }
    if !finished {
        return Err("bounded retessellation exhausted pass limit before convergence".into());
    }
    let final_topology = match mesh_topology::validate(&mesh.triangles, budget.remaining()) {
        Ok(value) => {
            budget.charge(value.work)?;
            report.topology_work += value.work;
            value
        }
        Err(error) => {
            budget.charge(error.work)?;
            return Err(error.message);
        }
    };
    if final_topology != (mesh_topology::TopologyReport { work: final_topology.work, ..initial_topology }) {
        return Err("bounded retessellation changed surface complex counts".into());
    }
    let embedding_certificate = if certify_output {
        if let Some(certificate) = current_certificate.as_ref() {
            match certificate.matches(&mesh.positions, &mesh.triangles, budget.remaining()) {
                Ok(value) => {
                    budget.charge(value.work)?;
                    report.final_certificate_match_work = value.work;
                    report.final_embedding_work += value.work;
                    report.embedding_validation_work += value.work;
                    report.final_embedding_reused = value.matches;
                }
                Err(error) => {
                    budget.charge(error.work)?;
                    return Err(error.message);
                }
            }
        }
        if report.final_embedding_reused {
            current_certificate
        } else {
            match prepared.validate_retopologized_certified(&mesh.positions, &mesh.triangles, budget.remaining()) {
                Ok(value) => {
                    budget.charge(value.work)?;
                    report.final_embedding_work += value.work;
                    report.embedding_validation_work += value.work;
                    report.certificate_snapshot_work += value.certificate_work;
                    Some(value.certificate)
                }
                Err(error) => {
                    budget.charge(error.work)?;
                    return Err(error.message);
                }
            }
        }
    } else {
        match prepared.validate_retopologized(&mesh.positions, &mesh.triangles, budget.remaining()) {
            Ok(value) => {
                budget.charge(value.work)?;
                report.final_embedding_work = value.work;
                report.embedding_validation_work += value.work;
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        }
        None
    };
    budget.charge(mesh.triangles.len() * 42)?;
    let origin = xyz(mesh.metadata.min).map(|v| v * 0.5);
    let upper = xyz(mesh.metadata.max).map(|v| v * 0.5);
    let origin = std::array::from_fn::<_, 3, _>(|i| origin[i] + upper[i]);
    let mut area = 0.;
    let mut volume = 0.;
    for triangle in &mesh.triangles {
        let [a, b, c] =
            triangle.map(|v| std::array::from_fn::<_, 3, _>(|i| xyz(mesh.positions[v as usize])[i] - origin[i]));
        let u = std::array::from_fn::<_, 3, _>(|i| b[i] - a[i]);
        let v = std::array::from_fn::<_, 3, _>(|i| c[i] - a[i]);
        let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        area += n[0].hypot(n[1]).hypot(n[2]) * 0.5;
        let bc = [b[1] * c[2] - b[2] * c[1], b[2] * c[0] - b[0] * c[2], b[0] * c[1] - b[1] * c[0]];
        volume += (0..3).map(|i| a[i] * bc[i]).sum::<f64>() / 6.;
    }
    if !area.is_finite() || !volume.is_finite() {
        return Err("bounded retessellation final measurements exceed finite bounds".into());
    }
    mesh.metadata.surface_area = area;
    mesh.metadata.signed_volume = volume;
    report.max_surface_error_m = bounds.iter().copied().fold(0., f64::max);
    report.predicate_work = exact_report.predicate_work;
    report.quality_evaluations = budget.quality_evaluations;
    report.quality_cache_hits = quality_cache.hits;
    report.quality_cache_updates = quality_cache.updates;
    report.quality_cache_rollbacks = quality_cache.rollbacks;
    report.work = budget.used;
    Ok(BoundedRetessellatedMesh {
        mesh,
        face_ancestors: ancestors,
        face_error_bounds_m: bounds,
        report,
        embedding_certificate,
    })
}
