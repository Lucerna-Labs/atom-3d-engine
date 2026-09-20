//! Bounded projection/refinement against the original native field. Incoming
//! mesh winding supplies the outward seed where the field gradient is singular.
//! This preserves input topology and original in-tolerance vertices. Arbitrary
//! scalar-grid meshes can have unsuitable correspondence or extra components;
//! failure is reported instead of deleting geometry or certifying missing caps.
//! Production surface delivery supplies enclosing convex support polytopes.
use mm3e_kit::{
    meshing::{Mesh, MAX_MESH_TRIANGLES, MAX_MESH_VERTICES},
    Vec3,
};
use mm3e_orchestrator::{sampling::CountedSceneField, Scene};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub struct Options {
    pub max_residual: f32,
    pub normal_step_m: f64,
    pub max_work: usize,
    pub max_vertices: usize,
    pub max_triangles: usize,
    pub max_passes: u32,
}
pub struct RefinedMesh {
    pub mesh: Mesh,
    /// Every child retains the original input face that produced it.
    pub face_source_indices: Vec<usize>,
    pub report: Value,
}
type D = [f64; 3];
fn d(p: Vec3) -> D {
    [p.x, p.y, p.z].map(f64::from)
}
fn add(a: D, b: D) -> D {
    std::array::from_fn(|i| a[i] + b[i])
}
fn sub(a: D, b: D) -> D {
    std::array::from_fn(|i| a[i] - b[i])
}
fn scale(a: D, s: f64) -> D {
    a.map(|x| x * s)
}
fn dot(a: D, b: D) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}
fn norm(a: D) -> f64 {
    a[0].hypot(a[1]).hypot(a[2])
}
fn cross(a: D, b: D) -> D {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn point(a: D) -> Result<Vec3, String> {
    let values = a.map(|x| x as f32);
    if !values.into_iter().all(f32::is_finite) {
        return Err("native surface projection exceeds finite f32 positions".into());
    }
    Ok(Vec3::new(values[0], values[1], values[2]))
}
fn normal(positions: &[Vec3], triangle: [u32; 3]) -> Result<D, String> {
    if triangle.iter().any(|&i| i as usize >= positions.len()) {
        return Err("native refinement triangle index is invalid".into());
    }
    let [a, b, c] = triangle.map(|i| d(positions[i as usize]));
    let n = cross(sub(b, a), sub(c, a));
    if !a.into_iter().chain(b).chain(c).all(f64::is_finite) || !norm(n).is_finite() || norm(n) == 0. {
        return Err("native refinement encountered a nonfinite or collapsed triangle".into());
    }
    Ok(n)
}
#[derive(Default)]
struct Budget {
    used: usize,
    maximum: usize,
    field: usize,
    queries: usize,
}
impl Budget {
    #[track_caller]
    fn charge(&mut self, amount: usize) -> Result<(), String> {
        if amount > self.maximum.saturating_sub(self.used) {
            let location = std::panic::Location::caller();
            return Err(format!(
                "native surface refinement exhausted work budget at {}:{} (used {}, requested {})",
                location.file(),
                location.line(),
                self.used,
                amount
            ));
        }
        self.used += amount;
        Ok(())
    }
}
const RADIAL_PROOF_WORK_LIMIT: usize = 16_000_000;

fn radial_work(work: &mm3e_kit::radial_embedding::WorkReport) -> Value {
    json!({"work":work.work,"structural_work":work.structural_work,"topology_work":work.topology_work,
        "point_work":work.point_work,"centre_work":work.centre_work,"plane_work":work.plane_work,
        "ray_work":work.ray_work,"centres_attempted":work.centres_attempted,"rays_attempted":work.rays_attempted,
        "face_plane_tests":work.face_plane_tests,"edge_cone_tests":work.edge_cone_tests,
        "integer_fallbacks":work.integer_fallbacks})
}

/// A failed sufficient proof spends from the same allowance as the complete
/// pairwise fallback. A successful proof is valid only for these exact arrays.
fn validate_embedding(positions: &[Vec3], triangles: &[[u32; 3]], budget: &mut Budget) -> Result<Value, String> {
    use mm3e_kit::radial_embedding::{self, CentreKind, FailureKind};
    let before = budget.used;
    let attempt =
        radial_embedding::certify(positions, triangles, RADIAL_PROOF_WORK_LIMIT.min(budget.maximum - budget.used));
    let unsuccessful = match attempt {
        Ok(certificate) => {
            let report = certificate.into_report();
            budget.charge(report.work.work)?;
            return Ok(json!({"intersection_free":true,"method":"degree_one_radial_projection",
                "work":report.work.work,"pair_validation_work":0,"radial_attempt_limit":RADIAL_PROOF_WORK_LIMIT,
                "radial_certificate":{"work":radial_work(&report.work),"scope":report.scope,
                    "input_vertices":report.input_vertices,"faces":report.faces,"edges":report.edges,
                    "topology":{"work":report.topology.work,"vertices":report.topology.vertices,
                        "edges":report.topology.edges,"triangles":report.topology.triangles,"components":report.topology.components},
                    "euler_characteristic":report.euler_characteristic,"signed_degree":report.degree,
                    "forward_face_hits":report.forward_face_hits,
                    "centre":{"kind":match report.centre.kind {CentreKind::BoundsMidpoint=>"bounds_midpoint",CentreKind::VertexMean=>"vertex_mean"},
                        "numerators":report.centre.numerators,"denominator":report.centre.denominator},
                    "ray":{"direction":report.ray.direction,"endpoint_numerators":report.ray.endpoint_numerators,
                        "denominator":report.ray.denominator}}}));
        }
        Err(error) => {
            budget.charge(error.work.work)?;
            json!({"kind":match error.kind {FailureKind::InvalidInput=>"invalid_input",FailureKind::Inconclusive=>"inconclusive",FailureKind::Budget=>"budget"},
                "message":error.message,"work":radial_work(&error.work)})
        }
    };
    let checked =
        match mm3e_kit::surface_intersections::validate_counted(positions, triangles, budget.maximum - budget.used) {
            Ok(report) => {
                budget.charge(report.work)?;
                report
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(format!(
                    "native surface embedding: {error}; total embedding work {}; radial attempt {unsuccessful}",
                    budget.used - before
                ));
            }
        };
    Ok(json!({"intersection_free":true,"method":"triangle_pair_validation","work":budget.used-before,
        "pair_validation_work":checked.work,"radial_attempt_limit":RADIAL_PROOF_WORK_LIMIT,"radial_attempt":unsuccessful,
        "candidate_pairs":checked.candidate_pairs,"predicate_tests":checked.predicate_tests,
        "exact_predicates":checked.exact_predicates,"bvh_chunks":checked.bvh_chunks}))
}
struct Projector<'a> {
    scene: &'a Scene,
    sampler: CountedSceneField<'a>,
    options: &'a Options,
    bounds: [D; 2],
    locality_m: f64,
    budget: Budget,
    projected: usize,
    seeded: usize,
    maximum_displacement: f64,
    cache: BTreeMap<[u32; 3], f64>,
    cache_hits: usize,
    frozen_conflicts: usize,
    target_queries: usize,
    target_work: usize,
    rounding_updates: usize,
    outward_guard_rejections: usize,
    outward_split_guard_rejections: usize,
    feature_queries: usize,
    feature_query_work: usize,
    feature_cache_hits: usize,
    orientation_witness_queries: usize,
    orientation_witness_tests: usize,
    orientation_witness_candidates: usize,
}
#[derive(Clone, Copy)]
struct SurfaceRegion {
    local_point: D,
    support: [u32; 3],
    count: usize,
    triangle: u32,
}
impl Projector<'_> {
    fn interior_witness(&mut self, positions: &[Vec3], parent: [u32; 3]) -> Result<Option<Vec3>, String> {
        if !crate::surface_delivery::eligible(self.scene) {
            return Ok(None);
        }
        self.budget.charge(4)?;
        let center = parent.into_iter().fold([0.; 3], |sum, id| add(sum, scale(d(positions[id as usize]), 1. / 3.)));
        let object = &self.scene.objects[0];
        let mm3e_orchestrator::Prim::Surface { id } = object.prim else { unreachable!() };
        let local = self.scene.surface_texture_local_point(0, point(center)?)?;
        let hit =
            self.scene.surfaces[id as usize].closest_hit_bounded(local, self.budget.maximum - self.budget.used)?;
        self.budget.charge(hit.work)?;
        self.budget.field += hit.work;
        self.budget.queries += 1;
        self.orientation_witness_queries += 1;
        let local = scale(hit.closest_point, f64::from(object.xform.scale));
        let world = add(
            d(object.xform.pos),
            (0..3).fold([0.; 3], |sum, axis| add(sum, scale(d(object.xform.rot.cols[axis]), local[axis]))),
        );
        let witness = point(world)?;
        Ok((self.sample(witness)? < 0.).then_some(witness))
    }
    fn surface_region(&mut self, p: Vec3) -> Result<SurfaceRegion, String> {
        self.budget.charge(4)?;
        let mm3e_orchestrator::Prim::Surface { id } = self.scene.objects[0].prim else { unreachable!() };
        let source = &self.scene.surfaces[id as usize];
        let local = self.scene.surface_texture_local_point(0, p)?;
        let hit = source.closest_hit_bounded(local, self.budget.maximum - self.budget.used)?;
        self.budget.charge(hit.work)?;
        self.budget.field += hit.work;
        self.budget.queries += 1;
        self.feature_queries += 1;
        self.feature_query_work += hit.work;
        let triangle = source.triangles()[hit.triangle as usize];
        let mut support = [u32::MAX; 3];
        let mut count = 0;
        for (vertex, weight) in triangle.into_iter().zip(hit.barycentric) {
            if weight > 0. {
                support[count] = vertex;
                count += 1;
            }
        }
        support[..count].sort_unstable();
        Ok(SurfaceRegion { local_point: d(local), support, count, triangle: hit.triangle })
    }
    /// Place a split at a source-feature boundary candidate instead of spending
    /// successive bisections crossing a long, already planar part of the edge.
    /// The returned point still lies on the existing edge; normal projection
    /// and the same residual/orientation/embedding gates remain authoritative.
    fn feature_edge_split(
        &mut self,
        a: u32,
        b: u32,
        positions: &[Vec3],
        regions: &mut BTreeMap<u32, SurfaceRegion>,
        remaining_passes: u32,
    ) -> Result<Option<D>, String> {
        if !crate::surface_delivery::eligible(self.scene) {
            return Ok(None);
        }
        self.budget.charge(1)?;
        let object = &self.scene.objects[0];
        let mm3e_orchestrator::Prim::Surface { id } = object.prim else { unreachable!() };
        let radius = f64::from(self.scene.surfaces[id as usize].half_thickness()) * f64::from(object.xform.scale)
            + f64::from(object.mods.round);
        let error = f64::from(self.options.max_residual);
        let length = norm(sub(d(positions[b as usize]), d(positions[a as usize])));
        // Circle sagitta supplies a feature-local planning scale. If ordinary
        // bisection already resolves that scale within the remaining passes,
        // retain its established placement. This estimate is not acceptance:
        // all native probes and final embedding validation still run.
        let chord_scale = 2. * (error * (2. * radius - error)).sqrt();
        if error >= radius || !chord_scale.is_finite() || length * 2f64.powi(-(remaining_passes as i32)) <= chord_scale
        {
            return Ok(None);
        }
        let mut endpoints = [None; 2];
        for (index, id) in [a, b].into_iter().enumerate() {
            self.budget.charge(1)?;
            let region = if let Some(&region) = regions.get(&id) {
                self.feature_cache_hits += 1;
                region
            } else {
                let region = self.surface_region(positions[id as usize])?;
                if regions.len() < 262_144 {
                    self.budget.charge(1)?;
                    regions.insert(id, region);
                }
                region
            };
            endpoints[index] = Some(region);
        }
        let [first, second] = endpoints.map(Option::unwrap);
        self.budget.charge(16)?;
        let mm3e_orchestrator::Prim::Surface { id } = self.scene.objects[0].prim else { unreachable!() };
        let source = &self.scene.surfaces[id as usize];
        let vertex = |id: u32| d(source.vertices()[id as usize]);
        let mut triangles = vec![first.triangle];
        if second.triangle != first.triangle {
            triangles.push(second.triangle);
        }
        let mut fractions = Vec::new();
        // A projected source edge is a shoulder even when a boundary endpoint
        // has been classified as a vertex or as a different incident edge.
        // Test the actual finite source segments, not their infinite extensions.
        for triangle in triangles {
            let points = source.triangles()[triangle as usize].map(vertex);
            let face_normal = cross(sub(points[1], points[0]), sub(points[2], points[0]));
            for (i, j) in [(0, 1), (1, 2), (2, 0)] {
                self.budget.charge(8)?;
                let anchor = points[i];
                let edge = sub(points[j], anchor);
                let normal = cross(edge, face_normal);
                let fa = dot(normal, sub(first.local_point, anchor));
                let fb = dot(normal, sub(second.local_point, anchor));
                let t = fa / (fa - fb);
                if t.is_finite() && 0. < t && t < 1. {
                    let local = add(first.local_point, scale(sub(second.local_point, first.local_point), t));
                    let along = dot(edge, sub(local, anchor)) / dot(edge, edge);
                    if (0.0..=1.0).contains(&along) {
                        fractions.push(t);
                    }
                }
            }
        }
        let (larger, smaller) = if first.count > second.count { (first, second) } else { (second, first) };
        if larger.count == 2 && smaller.count == 1 && larger.support[..2].contains(&smaller.support[0]) {
            self.budget.charge(8)?;
            let anchor = vertex(smaller.support[0]);
            let normal = sub(vertex(larger.support[1]), vertex(larger.support[0]));
            let fa = dot(normal, sub(first.local_point, anchor));
            let fb = dot(normal, sub(second.local_point, anchor));
            let t = fa / (fa - fb);
            if t.is_finite() && 0. < t && t < 1. {
                fractions.push(t);
            }
        }
        fractions.sort_by(|a, b| (a - 0.5).abs().total_cmp(&(b - 0.5).abs()).then(a.total_cmp(b)));
        let Some(&t) = fractions.first() else {
            return Ok(None);
        };
        let pa = d(positions[a as usize]);
        let pb = d(positions[b as usize]);
        let target = add(pa, scale(sub(pb, pa), t));
        // Reuse the existing internal vertex target as a progress guard, rather
        // than manufacturing another almost coincident edge beside a shoulder.
        let minimum_step = f64::from(self.options.max_residual) * 0.125;
        if norm(sub(target, pa)).min(norm(sub(target, pb))) <= minimum_step {
            return Ok(None);
        }
        Ok(Some(target))
    }
    /// Use the same bounded closest-triangle query as the native field, retaining
    /// its f64 closest point through shell projection. Finite-difference normals
    /// at neighboring f32 vertices need not produce a coherent displacement.
    fn surface_target(&mut self, start: Vec3) -> Result<Option<D>, String> {
        if !crate::surface_delivery::eligible(self.scene) {
            return Ok(None);
        }
        self.budget.charge(4)?;
        let object = &self.scene.objects[0];
        let mm3e_orchestrator::Prim::Surface { id } = object.prim else { unreachable!() };
        let surface = &self.scene.surfaces[id as usize];
        let local = self.scene.surface_texture_local_point(0, start)?;
        let hit = surface.closest_hit_bounded(local, self.budget.maximum - self.budget.used)?;
        self.budget.charge(hit.work)?;
        self.budget.field += hit.work;
        self.budget.queries += 1;
        self.target_queries += 1;
        self.target_work += hit.work;
        let radial = sub(d(local), hit.closest_point);
        let distance = norm(radial);
        if distance == 0. {
            return Ok(None);
        }
        let radius = f64::from(surface.half_thickness()) + f64::from(object.mods.round) / f64::from(object.xform.scale);
        if !radius.is_finite() || radius <= 0. {
            return Ok(None);
        }
        // Close to the medial surface, a tiny nearest-feature change can choose
        // the opposite shell. The established outward-face seed is preferable
        // there; the enclosing producer and small chord errors stay outside
        // this poorly conditioned region.
        if distance < radius * 0.75 {
            return Ok(None);
        }
        let target = scale(add(hit.closest_point, scale(radial, radius / distance)), f64::from(object.xform.scale));
        let world = add(
            d(object.xform.pos),
            (0..3).fold([0.; 3], |sum, axis| add(sum, scale(d(object.xform.rot.cols[axis]), target[axis]))),
        );
        let stored = point(world)?;
        Ok(((0..3).all(|i| d(stored)[i] > self.bounds[0][i] && d(stored)[i] < self.bounds[1][i])).then_some(world))
    }
    fn sample(&mut self, p: Vec3) -> Result<f64, String> {
        self.budget.charge(1)?;
        let key = [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
        if let Some(&value) = self.cache.get(&key) {
            self.cache_hits += 1;
            return Ok(value);
        }
        let value = self.sampler.sample(p, self.budget.maximum - self.budget.used).map_err(|error| {
            format!(
                "{error}; native query {p:?}, charged work {}, field queries {}",
                self.budget.used, self.budget.queries
            )
        })?;
        self.budget.charge(value.work)?;
        self.budget.field += value.work;
        self.budget.queries += 1;
        let result = f64::from(value.field.dist);
        if !result.is_finite() {
            return Err("native surface refinement encountered a nonfinite field".into());
        }
        if self.cache.len() < 262_144 {
            self.budget.charge(1)?;
            self.cache.insert(key, result);
        }
        Ok(result)
    }
    fn gradient(&mut self, p: Vec3) -> Result<D, String> {
        let mut gradient = [0.; 3];
        for axis in 0..3 {
            self.budget.charge(1)?;
            let mut plus = d(p);
            let mut minus = plus;
            plus[axis] += self.options.normal_step_m;
            minus[axis] -= self.options.normal_step_m;
            let plus = point(plus)?;
            let minus = point(minus)?;
            let width = d(plus)[axis] - d(minus)[axis];
            if !width.is_finite() || width <= 0. {
                return Err("native surface gradient probes are unrepresentable".into());
            }
            gradient[axis] = (self.sample(plus)? - self.sample(minus)?) / width;
        }
        Ok(gradient)
    }
    fn project(
        &mut self,
        start: Vec3,
        seed: D,
        preserve_valid: bool,
        initial_value: f64,
        initial_gradient: D,
        iterations: &mut u32,
    ) -> Result<Option<Vec3>, String> {
        self.budget.charge(1)?;
        let mut value = initial_value;
        // Preserve already-accepted affine crease coordinates bit for bit.
        let target = f64::from(self.options.max_residual) * 0.125;
        if value.abs() <= if preserve_valid { f64::from(self.options.max_residual) } else { target } {
            return Ok(Some(start));
        }
        let seed_length = norm(seed);
        if !seed_length.is_finite() || seed_length == 0. {
            return Ok(None);
        }
        let seed = scale(seed, 1. / seed_length);
        // A tangent normal line can meet a rounded cap farther away than the
        // signed-distance residual. Keep it inside the original half-cell
        // neighborhood instead of collapsing the point onto a different face.
        let trust = (2. * (value.abs() + self.options.normal_step_m)).max(self.locality_m);
        let mut current = start;
        for iteration in 0..4 {
            if *iterations == 0 {
                break;
            }
            *iterations -= 1;
            let gradient = if iteration == 0 { initial_gradient } else { self.gradient(current)? };
            let derivative = dot(gradient, seed);
            let mut steps = Vec::with_capacity(2);
            if derivative > 0. && derivative.is_finite() {
                steps.push((scale(seed, -value / derivative), false));
            }
            steps.push((scale(seed, -value), true));
            let mut accepted = None;
            'directions: for (mut step, seeded) in steps {
                self.budget.charge(1)?;
                let magnitude = norm(step);
                if !magnitude.is_finite() || magnitude == 0. {
                    continue;
                }
                if magnitude > trust {
                    step = scale(step, trust / magnitude);
                }
                for reduction in 0..12 {
                    self.budget.charge(1)?;
                    let candidate = point(add(d(current), scale(step, 2.0_f64.powi(-reduction))))?;
                    if candidate == current
                        || norm(sub(d(candidate), d(start))) > trust
                        || (0..3).any(|i| d(candidate)[i] <= self.bounds[0][i] || d(candidate)[i] >= self.bounds[1][i])
                    {
                        continue;
                    }
                    let next = self.sample(candidate)?;
                    if next.abs() < value.abs() {
                        accepted = Some((candidate, next, seeded));
                        break 'directions;
                    }
                }
            }
            let Some((candidate, next, seeded)) = accepted else {
                return Ok((current != start).then_some(current));
            };
            if seeded {
                self.seeded += 1;
            }
            current = candidate;
            value = next;
            if value.abs() <= target {
                return Ok(Some(current));
            }
        }
        Ok((current != start).then_some(current))
    }
    fn outward(&mut self, positions: &[Vec3], triangle: [u32; 3]) -> Result<bool, String> {
        self.budget.charge(1)?;
        let n = normal(positions, triangle)?;
        let n = scale(n, 1. / norm(n));
        let center = triangle.into_iter().fold([0.; 3], |sum, i| add(sum, scale(d(positions[i as usize]), 1. / 3.)));
        let plus = point(add(center, scale(n, self.options.normal_step_m)))?;
        let minus = point(sub(center, scale(n, self.options.normal_step_m)))?;
        if plus == minus {
            return Err("native refinement orientation probes are unrepresentable".into());
        }
        Ok(self.sample(plus)? > self.sample(minus)?)
    }
}

fn audit(positions: &[Vec3], triangles: &[[u32; 3]], budget: &mut Budget) -> Result<(usize, usize), String> {
    let mut edges = BTreeMap::<(u32, u32), (usize, i32)>::new();
    let mut faces = BTreeSet::new();
    let mut links = BTreeMap::<u32, Vec<(u32, u32)>>::new();
    let mut adjacent = BTreeMap::<u32, Vec<u32>>::new();
    for &triangle in triangles {
        budget.charge(11)?;
        normal(positions, triangle)?;
        let mut sorted = triangle;
        sorted.sort_unstable();
        if !faces.insert(sorted) {
            return Err("native refinement contains duplicate faces".into());
        }
        let [a, b, c] = triangle;
        for (a, b) in [(a, b), (b, c), (c, a)] {
            let entry = edges.entry((a.min(b), a.max(b))).or_default();
            entry.0 += 1;
            entry.1 += if a < b { 1 } else { -1 };
            adjacent.entry(a).or_default().push(b);
            adjacent.entry(b).or_default().push(a);
        }
        for (v, edge) in [(a, (b, c)), (b, (c, a)), (c, (a, b))] {
            links.entry(v).or_default().push(edge);
        }
    }
    if edges.values().any(|&(n, w)| n != 2 || w != 0) {
        return Err("native refinement requires closed, consistently oriented edges".into());
    }
    for link in links.values() {
        let mut neighbors = BTreeMap::<u32, Vec<u32>>::new();
        for &(a, b) in link {
            budget.charge(2)?;
            neighbors.entry(a).or_default().push(b);
            neighbors.entry(b).or_default().push(a);
        }
        if neighbors.values().any(|v| v.len() != 2) {
            return Err("native refinement has a nonmanifold vertex link".into());
        }
        let mut pending = vec![*neighbors.keys().next().ok_or("empty native refinement vertex link")?];
        let mut seen = BTreeSet::new();
        while let Some(next) = pending.pop() {
            budget.charge(1)?;
            if seen.insert(next) {
                pending.extend(&neighbors[&next]);
            }
        }
        if seen.len() != neighbors.len() {
            return Err("native refinement has disconnected vertex fans".into());
        }
    }
    let mut seen = BTreeSet::new();
    let mut components = 0;
    for &vertex in adjacent.keys() {
        budget.charge(1)?;
        if seen.contains(&vertex) {
            continue;
        }
        components += 1;
        let mut pending = vec![vertex];
        while let Some(next) = pending.pop() {
            budget.charge(1)?;
            if seen.insert(next) {
                pending.extend(&adjacent[&next]);
            }
        }
    }
    Ok((components, links.len()))
}
fn rounded_target_candidates(target: D) -> Result<Vec<Vec3>, String> {
    let mut axes = [[0f32; 2]; 3];
    for axis in 0..3 {
        let nearest = target[axis] as f32;
        axes[axis] = if f64::from(nearest) < target[axis] {
            [nearest, nearest.next_up()]
        } else if f64::from(nearest) > target[axis] {
            [nearest.next_down(), nearest]
        } else {
            [nearest, nearest]
        };
    }
    let mut result = vec![point(target)?];
    for x in axes[0] {
        for y in axes[1] {
            for z in axes[2] {
                let candidate = Vec3::new(x, y, z);
                if !result.contains(&candidate) && d(candidate).into_iter().all(f64::is_finite) {
                    result.push(candidate);
                }
            }
        }
    }
    Ok(result)
}

/// A continuous shell projection can round neighboring vertices onto one line.
/// Search only each target coordinate's two enclosing f32 values, testing the
/// complete incident patch whenever its three vertices change together.
fn resolve_surface_rounding(
    proposed: &mut [Vec3],
    targets: &[Option<D>],
    triangles: &[[u32; 3]],
    adjacent: &[Vec<usize>],
    reference: &[Vec3],
    budget: &mut Budget,
) -> Result<usize, String> {
    if !targets.iter().any(Option::is_some) {
        return Ok(0);
    }
    budget.charge(proposed.len() + triangles.len())?;
    let before = proposed.to_vec();
    budget.charge(targets.iter().filter(|target| target.is_some()).count().saturating_mul(8))?;
    let pools: Vec<_> = targets
        .iter()
        .enumerate()
        .map(|(i, target)| target.map_or_else(|| Ok(vec![proposed[i]]), rounded_target_candidates))
        .collect::<Result<_, String>>()?;
    let normals = triangles.iter().map(|&t| normal(reference, t)).collect::<Result<Vec<_>, _>>()?;
    for _ in 0..4 {
        let mut progress = false;
        for (face, &triangle) in triangles.iter().enumerate() {
            budget.charge(1)?;
            if normal(proposed, triangle).is_ok_and(|next| dot(normals[face], next) > 0.) {
                continue;
            }
            let indices = triangle.map(|i| i as usize);
            let old = indices.map(|i| proposed[i]);
            let mut affected = BTreeSet::new();
            for i in indices {
                budget.charge(adjacent[i].len())?;
                affected.extend(adjacent[i].iter().copied());
            }
            let invalid = |points: &[Vec3], budget: &mut Budget| -> Result<usize, String> {
                let mut count = 0;
                for &f in &affected {
                    budget.charge(1)?;
                    if !normal(points, triangles[f]).is_ok_and(|next| dot(normals[f], next) > 0.) {
                        count += 1;
                    }
                }
                Ok(count)
            };
            let mut score = invalid(proposed, budget)?;
            let mut best = old;
            'choices: for &a in &pools[indices[0]] {
                for &b in &pools[indices[1]] {
                    for &c in &pools[indices[2]] {
                        budget.charge(1)?;
                        for (i, p) in indices.into_iter().zip([a, b, c]) {
                            proposed[i] = p;
                        }
                        if !normal(proposed, triangle).is_ok_and(|next| dot(normals[face], next) > 0.) {
                            continue;
                        }
                        let next = invalid(proposed, budget)?;
                        if next < score {
                            score = next;
                            best = [a, b, c];
                            if score == 0 {
                                break 'choices;
                            }
                        }
                    }
                }
            }
            for (i, p) in indices.into_iter().zip(best) {
                proposed[i] = p;
            }
            progress |= best != old;
        }
        if !progress {
            break;
        }
    }
    Ok(proposed.iter().zip(before).filter(|(a, b)| **a != *b).count())
}

fn project_vertices(
    positions: &mut [Vec3],
    triangles: &[[u32; 3]],
    origins: &[Vec3],
    original_count: usize,
    iterations: &mut [u32],
    adjacent: &[Vec<usize>],
    projector: &mut Projector<'_>,
) -> Result<bool, String> {
    projector.budget.charge(positions.len().saturating_mul(5))?;
    let reference = positions.to_vec();
    let mut proposed = reference.clone();
    let mut targets = vec![None; positions.len()];
    let mut values = vec![0.; positions.len()];
    for vertex in 0..positions.len() {
        projector.budget.charge(1)?;
        let start = reference[vertex];
        let value = projector.sample(start)?;
        values[vertex] = value.abs();
        let threshold = f64::from(projector.options.max_residual) * if vertex < original_count { 1.0 } else { 0.125 };
        if value.abs() <= threshold || iterations[vertex] == 0 {
            continue;
        }
        if let Some(target) = projector.surface_target(start)? {
            let candidate = point(target)?;
            iterations[vertex] -= 1;
            if projector.sample(candidate)?.abs() < value.abs() {
                targets[vertex] = Some(target);
                proposed[vertex] = candidate;
                continue;
            }
        }
        let mut directions = Vec::new();
        let mut seen_directions = BTreeSet::new();
        let mut sum = [0.; 3];
        for &face in &adjacent[vertex] {
            projector.budget.charge(2)?;
            let triangle = triangles[face];
            let n = normal(&reference, triangle)?;
            let n = scale(n, 1. / norm(n));
            sum = add(sum, n);
            let key = n.map(|v| if v == 0. { 0 } else { v.to_bits() });
            if seen_directions.insert(key) {
                directions.push(n);
            }
        }
        let gradient = projector.gradient(start)?;
        let mut best: Option<(Vec3, f64, f64, usize)> = None;
        for seed in std::iter::once(sum).chain(directions).chain((norm(gradient) > 0.).then_some(gradient)) {
            let Some(candidate) =
                projector.project(start, seed, vertex < original_count, value, gradient, &mut iterations[vertex])?
            else {
                continue;
            };
            let error = projector.sample(candidate)?.abs();
            let displacement = norm(sub(d(candidate), d(start)));
            let mut conflicts = 0;
            for &face in &adjacent[vertex] {
                projector.budget.charge(2)?;
                let triangle = triangles[face];
                let old = normal(&reference, triangle)?;
                let [a, b, c] =
                    triangle.map(|i| d(if i as usize == vertex { candidate } else { reference[i as usize] }));
                let next = cross(sub(b, a), sub(c, a));
                if norm(next) == 0. || dot(old, next) <= 0. {
                    conflicts += 1;
                }
            }
            if error < value.abs()
                && best.is_none_or(|(_, old_error, old_distance, old_conflicts)| {
                    conflicts < old_conflicts
                        || (conflicts == old_conflicts
                            && ((error <= threshold && old_error > threshold)
                                || (error <= threshold && old_error <= threshold && displacement < old_distance)
                                || (old_error > threshold && error < old_error)))
                })
            {
                best = Some((candidate, error, displacement, conflicts));
            }
            if best.is_some_and(|(_, error, _, conflicts)| error <= threshold && conflicts == 0) {
                break;
            }
        }
        if let Some((candidate, _, _, _)) = best {
            proposed[vertex] = candidate;
        }
    }
    if proposed == reference {
        return Ok(false);
    }
    projector.budget.charge(positions.len())?;
    let active: Vec<_> = (0..positions.len()).filter(|&i| proposed[i] != reference[i]).collect();
    let mut affected = BTreeSet::new();
    for &i in &active {
        projector.budget.charge(adjacent[i].len())?;
        affected.extend(adjacent[i].iter().copied());
    }
    let mut old_normals = Vec::new();
    let mut protected_outward = BTreeSet::new();
    for face in affected {
        projector.budget.charge(1)?;
        old_normals.push((face, normal(&reference, triangles[face])?));
        if projector.outward(&reference, triangles[face])? {
            protected_outward.insert(face);
        }
    }
    projector.rounding_updates +=
        resolve_surface_rounding(&mut proposed, &targets, triangles, adjacent, &reference, &mut projector.budget)?;
    for &i in &active {
        projector.budget.charge(1)?;
        if (0..3).any(|axis| {
            d(proposed[i])[axis] <= projector.bounds[0][axis] || d(proposed[i])[axis] >= projector.bounds[1][axis]
        }) {
            proposed[i] = reference[i];
        }
    }
    // Each conflict damps only its participating vertices. Unrelated valid
    // proposals keep their full step, while coupled neighbors move together.
    let mut reductions = vec![0u32; positions.len()];
    let mut candidate = reference.clone();
    let mut last_conflicts = BTreeSet::new();
    for _ in 0..12 {
        for &i in &active {
            projector.budget.charge(1)?;
            candidate[i] = point(add(
                d(reference[i]),
                scale(sub(d(proposed[i]), d(reference[i])), 2.0_f64.powi(-(reductions[i] as i32))),
            ))?;
        }
        let mut conflicts = BTreeSet::new();
        for &(face, old) in &old_normals {
            projector.budget.charge(1)?;
            let triangle = triangles[face];
            let mut invalid = match normal(&candidate, triangle) {
                Ok(next) => dot(old, next) <= 0.,
                Err(_) => true,
            };
            if !invalid && protected_outward.contains(&face) && !projector.outward(&candidate, triangle)? {
                invalid = true;
                projector.outward_guard_rejections += 1;
            }
            if invalid {
                for i in triangle {
                    if proposed[i as usize] != reference[i as usize] {
                        conflicts.insert(i as usize);
                    }
                }
            }
        }
        let mut improved = false;
        for &i in &active {
            projector.budget.charge(1)?;
            let next = candidate[i];
            if next == reference[i] {
                continue;
            }
            let error = projector.sample(next)?.abs();
            if error > values[i] {
                conflicts.insert(i);
            } else {
                improved |= error < values[i];
            }
        }
        if conflicts.is_empty() {
            if !improved {
                return Ok(false);
            }
            for &i in &active {
                projector.budget.charge(1)?;
                let next = candidate[i];
                if next != reference[i] {
                    positions[i] = next;
                    projector.projected += 1;
                    projector.maximum_displacement =
                        projector.maximum_displacement.max(norm(sub(d(next), d(origins[i]))));
                }
            }
            return Ok(true);
        }
        projector.budget.charge(conflicts.len())?;
        last_conflicts = conflicts.clone();
        for i in conflicts {
            projector.budget.charge(1)?;
            reductions[i] += 1;
        }
    }
    // A numerically locked patch must not discard unrelated valid proposals.
    // Freeze its vertices, propagating only when that restoration would invert
    // an incident face. Each newly frozen vertex is processed once.
    let mut frozen = BTreeSet::new();
    let mut pending = Vec::new();
    for i in last_conflicts {
        projector.budget.charge(1)?;
        if frozen.insert(i) {
            candidate[i] = reference[i];
            pending.extend(adjacent[i].iter().copied());
        }
    }
    while let Some(face) = pending.pop() {
        projector.budget.charge(2)?;
        let triangle = triangles[face];
        let old = normal(&reference, triangle)?;
        if normal(&candidate, triangle).is_ok_and(|next| dot(old, next) > 0.)
            && (!protected_outward.contains(&face) || projector.outward(&candidate, triangle)?)
        {
            continue;
        }
        let mut changed = false;
        for i in triangle.map(|i| i as usize) {
            projector.budget.charge(1)?;
            if candidate[i] != reference[i] && frozen.insert(i) {
                candidate[i] = reference[i];
                pending.extend(adjacent[i].iter().copied());
                changed = true;
            }
        }
        if !changed {
            return Err("native projection conflict cannot restore a valid incident face".into());
        }
    }
    projector.frozen_conflicts += frozen.len();
    let mut improved = false;
    for &i in &active {
        projector.budget.charge(1)?;
        if candidate[i] != reference[i] && projector.sample(candidate[i])?.abs() < values[i] {
            improved = true;
        }
    }
    if improved {
        for &i in &active {
            projector.budget.charge(1)?;
            if candidate[i] != reference[i] {
                positions[i] = candidate[i];
                projector.projected += 1;
                projector.maximum_displacement =
                    projector.maximum_displacement.max(norm(sub(d(candidate[i]), d(origins[i]))));
            }
        }
    }
    Ok(improved)
}

type Edge = (u32, u32);
fn edge(a: u32, b: u32) -> Edge {
    (a.min(b), a.max(b))
}
fn split_pieces([a, b, c]: [u32; 3], midpoints: [Option<u32>; 3]) -> Vec<[u32; 3]> {
    match midpoints {
        [None, None, None] => vec![[a, b, c]],
        [Some(m), None, None] => vec![[a, m, c], [m, b, c]],
        [None, Some(m), None] => vec![[b, m, a], [m, c, a]],
        [None, None, Some(m)] => vec![[c, m, b], [m, a, b]],
        [Some(x), Some(y), None] => vec![[x, b, y], [a, x, y], [a, y, c]],
        [None, Some(y), Some(z)] => vec![[y, c, z], [b, y, z], [b, z, a]],
        [Some(x), None, Some(z)] => vec![[z, a, x], [c, z, x], [c, x, b]],
        [Some(x), Some(y), Some(z)] => vec![[a, x, z], [x, b, y], [z, y, c], [x, y, z]],
    }
}
trait SplitChecks {
    fn budget(&mut self) -> &mut Budget;
    fn preserves_outward(&mut self, _positions: &[Vec3], _parent: [u32; 3], _child: [u32; 3]) -> Result<bool, String> {
        Ok(true)
    }
    fn projected_candidates(&mut self, _point: Vec3) -> Result<(Vec<Vec3>, u32), String> {
        Ok((Vec::new(), 0))
    }
    fn alternate_orientation(
        &mut self,
        _positions: &[Vec3],
        _parent: [u32; 3],
        _children: &[[u32; 3]],
    ) -> Result<bool, String> {
        Ok(false)
    }
}
impl SplitChecks for Budget {
    fn budget(&mut self) -> &mut Budget {
        self
    }
}
struct NativeSplitChecks<'p, 's> {
    projector: &'p mut Projector<'s>,
    outward_parents: &'p BTreeSet<[u32; 3]>,
    interior_points: BTreeMap<[u32; 3], Option<Vec3>>,
    exact_points: BTreeMap<[u32; 3], mm3e_kit::exact_geometry::Point>,
}
impl NativeSplitChecks<'_, '_> {
    fn exact_failure(&self, message: String) -> String {
        if message == "exact geometry exhausted work budget" {
            format!(
                "native surface refinement exhausted work budget during exact split orientation (used {})",
                self.projector.budget.used
            )
        } else {
            message
        }
    }
    fn exact_point(&mut self, p: Vec3) -> Result<mm3e_kit::exact_geometry::Point, String> {
        self.projector.budget.charge(1)?;
        let key = [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
        if let Some(point) = self.exact_points.get(&key) {
            return Ok(point.clone());
        }
        let [x, y, z] = d(p);
        let result = mm3e_kit::exact_geometry::Point::from_expansions(
            [&[x], &[y], &[z]],
            &[1.],
            self.projector.budget.maximum - self.projector.budget.used,
        );
        let point = match result {
            Ok(result) => {
                self.projector.budget.charge(result.work)?;
                result.point
            }
            Err(error) => {
                self.projector.budget.charge(error.work)?;
                return Err(self.exact_failure(error.message));
            }
        };
        if self.exact_points.len() < 262_144 {
            self.projector.budget.charge(1)?;
            self.exact_points.insert(key, point.clone());
        }
        Ok(point)
    }
    fn positive_about(
        &mut self,
        positions: &[Vec3],
        triangle: [u32; 3],
        witness: &mm3e_kit::exact_geometry::Point,
    ) -> Result<bool, String> {
        let a = self.exact_point(positions[triangle[0] as usize])?;
        let b = self.exact_point(positions[triangle[1] as usize])?;
        let c = self.exact_point(positions[triangle[2] as usize])?;
        let result = mm3e_kit::exact_geometry::orient3(
            [&a, &b, &c, witness],
            self.projector.budget.maximum - self.projector.budget.used,
        );
        self.projector.orientation_witness_tests += 1;
        match result {
            Ok(result) => {
                self.projector.budget.charge(result.work)?;
                Ok(result.sign > 0)
            }
            Err(error) => {
                self.projector.budget.charge(error.work)?;
                Err(self.exact_failure(error.message))
            }
        }
    }
}
impl SplitChecks for NativeSplitChecks<'_, '_> {
    fn budget(&mut self) -> &mut Budget {
        &mut self.projector.budget
    }
    fn preserves_outward(&mut self, positions: &[Vec3], parent: [u32; 3], child: [u32; 3]) -> Result<bool, String> {
        if parent != child && self.outward_parents.contains(&parent) && !self.projector.outward(positions, child)? {
            self.projector.outward_split_guard_rejections += 1;
            Ok(false)
        } else {
            Ok(true)
        }
    }
    fn projected_candidates(&mut self, point: Vec3) -> Result<(Vec<Vec3>, u32), String> {
        let Some(target) = self.projector.surface_target(point)? else {
            return Ok((Vec::new(), 0));
        };
        self.projector.budget.charge(8)?;
        let mut accepted = Vec::new();
        for candidate in rounded_target_candidates(target)? {
            if (0..3)
                .all(|i| d(candidate)[i] > self.projector.bounds[0][i] && d(candidate)[i] < self.projector.bounds[1][i])
                && self.projector.sample(candidate)?.abs() <= f64::from(self.projector.options.max_residual) * 0.125
            {
                accepted.push(candidate);
            }
        }
        Ok((accepted, 1))
    }
    fn alternate_orientation(
        &mut self,
        positions: &[Vec3],
        parent: [u32; 3],
        children: &[[u32; 3]],
    ) -> Result<bool, String> {
        // A fixed strict native-interior witness transports this parent's
        // orientation through curvature. Every child must have the same exact
        // positive plane sign about that witness. This is not an embedding
        // proof; native outward checks and final embedding validation remain.
        if !self.outward_parents.contains(&parent) {
            return Ok(false);
        }
        let witness = if let Some(&witness) = self.interior_points.get(&parent) {
            witness
        } else {
            let witness = self.projector.interior_witness(positions, parent)?;
            self.projector.budget.charge(1)?;
            self.interior_points.insert(parent, witness);
            witness
        };
        let Some(witness) = witness else {
            return Ok(false);
        };
        let witness = self.exact_point(witness)?;
        if !self.positive_about(positions, parent, &witness)? {
            return Ok(false);
        }
        for &child in children {
            if !self.positive_about(positions, child, &witness)? {
                return Ok(false);
            }
        }
        self.projector.orientation_witness_candidates += 1;
        Ok(true)
    }
}
fn split_valid(
    positions: &[Vec3],
    triangle: [u32; 3],
    midpoints: [Option<u32>; 3],
    reference: D,
    budget: &mut impl SplitChecks,
) -> Result<bool, String> {
    let pieces = split_pieces(triangle, midpoints);
    let mut world_orientation = true;
    for &piece in &pieces {
        budget.budget().charge(2)?;
        let Ok(next) = normal(positions, piece) else {
            return Ok(false);
        };
        world_orientation &= dot(reference, next) > 0.;
    }
    if !world_orientation && !budget.alternate_orientation(positions, triangle, &pieces)? {
        return Ok(false);
    }
    for piece in pieces {
        if !budget.preserves_outward(positions, triangle, piece)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn midpoint_candidates(a: Vec3, b: Vec3) -> Vec<Vec3> {
    let a = d(a);
    let b = d(b);
    let axes: [[f32; 2]; 3] = std::array::from_fn(|i| {
        let sum = a[i] + b[i];
        let virtual_b = sum - a[i];
        let error = (a[i] - (sum - virtual_b)) + (b[i] - virtual_b);
        let high = sum * 0.5;
        let low = error * 0.5;
        let nearest = high as f32;
        // The two-sum tail preserves a tiny midpoint offset even when the
        // source coordinates have widely separated exponents. Comparison with
        // the exactly representable f32 value therefore avoids double rounding.
        let order = high.partial_cmp(&f64::from(nearest)).unwrap();
        if order.is_lt() || (order.is_eq() && low < 0.) {
            [nearest.next_down(), nearest]
        } else if order.is_gt() || (order.is_eq() && low > 0.) {
            [nearest, nearest.next_up()]
        } else {
            [nearest, nearest]
        }
    });
    let mut candidates = Vec::with_capacity(8);
    for x in axes[0] {
        for y in axes[1] {
            for z in axes[2] {
                let p = Vec3::new(x, y, z);
                if d(p).into_iter().all(f64::is_finite) && !candidates.contains(&p) {
                    candidates.push(p);
                }
            }
        }
    }
    candidates
}

/// Keep both sides of a narrow strip subdivided whenever enclosing f32
/// midpoint choices allow it. Dropping just one of two rounded-coincident
/// midpoints makes subsequent curved projection bow only one strip boundary.
/// Original vertices and shared edge identities never change here.
struct SplitRounding {
    changed: usize,
    projected: BTreeSet<u32>,
    next_pass_projection_cost: BTreeMap<u32, u32>,
}
fn round_split_midpoints<C: SplitChecks>(
    positions: &mut [Vec3],
    triangles: &[[u32; 3]],
    midpoints: &BTreeMap<Edge, u32>,
    feature_candidates: &BTreeMap<u32, Vec<Vec3>>,
    budget: &mut C,
) -> Result<SplitRounding, String> {
    budget.budget().charge(midpoints.len().saturating_mul(9) + triangles.len().saturating_mul(4))?;
    let pools: BTreeMap<_, _> = midpoints
        .iter()
        .map(|(&(a, b), &id)| {
            let a = positions[a as usize];
            let b = positions[b as usize];
            let candidates = feature_candidates.get(&id).cloned().unwrap_or_else(|| midpoint_candidates(a, b));
            let pool = candidates.into_iter().filter(|p| *p != a && *p != b).collect::<Vec<_>>();
            (id, pool)
        })
        .collect();
    let before: BTreeMap<_, _> = midpoints.values().map(|&id| (id, positions[id as usize])).collect();
    let mut projected_pools = BTreeMap::<u32, Vec<Vec3>>::new();
    let mut next_pass_projection_cost = BTreeMap::new();
    let mut incidence = BTreeMap::<u32, BTreeSet<usize>>::new();
    let mut references = Vec::with_capacity(triangles.len());
    for (face, &[a, b, c]) in triangles.iter().enumerate() {
        references.push(normal(positions, [a, b, c])?);
        for key in [edge(a, b), edge(b, c), edge(c, a)] {
            if let Some(&id) = midpoints.get(&key) {
                incidence.entry(id).or_default().insert(face);
            }
        }
    }
    let valid = |face: usize, positions: &[Vec3], budget: &mut C| {
        let [a, b, c] = triangles[face];
        split_valid(
            positions,
            [a, b, c],
            [edge(a, b), edge(b, c), edge(c, a)].map(|key| midpoints.get(&key).copied()),
            references[face],
            budget,
        )
    };
    for _ in 0..4 {
        let mut progress = false;
        for (face, &[a, b, c]) in triangles.iter().enumerate() {
            if valid(face, positions, budget)? {
                continue;
            }
            let variables: Vec<_> =
                [edge(a, b), edge(b, c), edge(c, a)].iter().filter_map(|key| midpoints.get(key).copied()).collect();
            let mut affected = BTreeSet::new();
            for id in &variables {
                budget.budget().charge(incidence[id].len())?;
                affected.extend(incidence[id].iter().copied());
            }
            let invalid = |positions: &[Vec3], budget: &mut C| -> Result<usize, String> {
                let mut count = 0;
                for &face in &affected {
                    if !valid(face, positions, budget)? {
                        count += 1;
                    }
                }
                Ok(count)
            };
            let original: Vec<_> = variables.iter().map(|&id| positions[id as usize]).collect();
            let mut best = original.clone();
            let mut score = invalid(positions, budget)?;
            let mut masks: Vec<u8> = (0..(1u8 << variables.len())).collect();
            masks.sort_by_key(|mask| mask.count_ones());
            budget.budget().charge(masks.len())?;
            for mask in masks {
                if score == 0 {
                    break;
                }
                let mut available = true;
                for (index, &id) in variables.iter().enumerate() {
                    if mask & (1 << index) != 0 {
                        if let std::collections::btree_map::Entry::Vacant(entry) = projected_pools.entry(id) {
                            let (points, cost) = budget.projected_candidates(before[&id])?;
                            budget.budget().charge(2)?;
                            entry.insert(points);
                            next_pass_projection_cost.insert(id, cost);
                        }
                        available &= !projected_pools[&id].is_empty();
                    }
                }
                if !available {
                    continue;
                }
                let unused = [Vec3::ZERO];
                let choices: [&[Vec3]; 3] = std::array::from_fn(|i| {
                    variables.get(i).map_or(unused.as_slice(), |id| {
                        if mask & (1 << i) != 0 {
                            projected_pools[id].as_slice()
                        } else {
                            pools[id].as_slice()
                        }
                    })
                });
                'choices: for &a in choices[0] {
                    for &b in choices[1] {
                        for &c in choices[2] {
                            budget.budget().charge(1)?;
                            for (&id, p) in variables.iter().zip([a, b, c]) {
                                positions[id as usize] = p;
                            }
                            if !valid(face, positions, budget)? {
                                continue;
                            }
                            let next = invalid(positions, budget)?;
                            if next < score {
                                score = next;
                                best = variables.iter().map(|&id| positions[id as usize]).collect();
                                if score == 0 {
                                    break 'choices;
                                }
                            }
                        }
                    }
                }
            }
            for (&id, &p) in variables.iter().zip(&best) {
                positions[id as usize] = p;
            }
            progress |= best != original;
        }
        if !progress {
            break;
        }
    }
    let changed = before.iter().filter(|(id, p)| positions[**id as usize] != **p).count();
    let projected = projected_pools
        .into_iter()
        .filter_map(|(id, candidates)| {
            let point = positions[id as usize];
            (candidates.contains(&point) && !pools[&id].contains(&point)).then_some(id)
        })
        .collect();
    Ok(SplitRounding { changed, projected, next_pass_projection_cost })
}
/// Remove only subdivision requests; never move or remove an existing vertex.
/// Removing a shared edge request rechecks all incident faces, so the final
/// red/green subdivision remains conforming. The finite set shrinks monotonically.
fn restrict_splits<C: SplitChecks>(
    positions: &[Vec3],
    triangles: &[[u32; 3]],
    midpoints: &mut BTreeMap<Edge, u32>,
    budget: &mut C,
) -> Result<usize, String> {
    let mut incidence = BTreeMap::<Edge, Vec<usize>>::new();
    let mut pending = BTreeSet::new();
    for (face, &[a, b, c]) in triangles.iter().enumerate() {
        budget.budget().charge(3)?;
        for key in [edge(a, b), edge(b, c), edge(c, a)] {
            if midpoints.contains_key(&key) {
                incidence.entry(key).or_default().push(face);
                pending.insert(face);
            }
        }
    }
    let mut removed = 0;
    while let Some(face) = pending.pop_first() {
        budget.budget().charge(4)?;
        let triangle @ [a, b, c] = triangles[face];
        let keys = [edge(a, b), edge(b, c), edge(c, a)];
        let current = keys.map(|key| midpoints.get(&key).copied());
        let reference = normal(positions, triangle)?;
        if split_valid(positions, triangle, current, reference, budget)? {
            continue;
        }
        let available = (0..3).fold(0u8, |mask, i| mask | (u8::from(current[i].is_some()) << i));
        let lengths = keys.map(|(a, b)| {
            let v = sub(d(positions[b as usize]), d(positions[a as usize]));
            dot(v, v)
        });
        let mut best_mask = 0u8;
        let mut best_score = 0.;
        for mask in 1u8..8 {
            budget.budget().charge(1)?;
            if mask & !available != 0 || mask.count_ones() < best_mask.count_ones() {
                continue;
            }
            let score: f64 = (0..3).filter(|i| mask & (1 << i) != 0).map(|i| lengths[i]).sum();
            if mask.count_ones() == best_mask.count_ones() && score <= best_score {
                continue;
            }
            let proposed = std::array::from_fn(|i| if mask & (1 << i) != 0 { current[i] } else { None });
            if split_valid(positions, triangle, proposed, reference, budget)? {
                best_mask = mask;
                best_score = score;
            }
        }
        for (i, key) in keys.into_iter().enumerate() {
            if available & (1 << i) != 0 && best_mask & (1 << i) == 0 {
                budget.budget().charge(1)?;
                if midpoints.remove(&key).is_some() {
                    removed += 1;
                    let neighbors = &incidence[&key];
                    budget.budget().charge(neighbors.len())?;
                    pending.extend(neighbors.iter().copied());
                }
            }
        }
    }
    Ok(removed)
}

// Keep a quarter of the final tolerance in reserve for conformity/projection;
// the final native probes, pass limit and geometry limits remain authoritative.
const REFINEMENT_TRIGGER_FRACTION: f64 = 0.75;

pub fn refine(scene: &Scene, mut mesh: Mesh, options: Options) -> Result<RefinedMesh, String> {
    if !options.max_residual.is_finite()
        || options.max_residual <= 0.
        || !options.normal_step_m.is_finite()
        || options.normal_step_m <= 0.
        || options.max_work == 0
        || options.max_passes > 12
        || options.max_vertices > MAX_MESH_VERTICES
        || options.max_triangles > MAX_MESH_TRIANGLES
        || mesh.positions.is_empty()
        || mesh.triangles.is_empty()
        || mesh.positions.len() > options.max_vertices
        || mesh.triangles.len() > options.max_triangles
    {
        return Err("native refinement options/input exceed finite, nonempty geometry or work/pass limits".into());
    }
    let bounds = [d(mesh.metadata.min), d(mesh.metadata.max)];
    if (0..3).any(|i| !bounds[0][i].is_finite() || !bounds[1][i].is_finite() || bounds[0][i] >= bounds[1][i]) {
        return Err("native refinement requires finite ordered original extraction bounds".into());
    }
    if d(mesh.metadata.spacing).into_iter().any(|s| !s.is_finite() || s <= 0.) {
        return Err("native refinement requires finite positive original grid spacing".into());
    }
    let mut budget = Budget { maximum: options.max_work, ..Budget::default() };
    budget.charge(mesh.positions.len().saturating_add(mesh.triangles.len()).saturating_add(scene.objects.len()))?;
    if mesh.positions.iter().any(|p| !d(*p).into_iter().all(f64::is_finite)) {
        return Err("native refinement input positions must be finite".into());
    }
    let (initial_components, _) = audit(&mesh.positions, &mesh.triangles, &mut budget)?;
    let mut projector = Projector {
        scene,
        sampler: CountedSceneField::new(scene)?,
        options: &options,
        bounds,
        locality_m: norm(d(mesh.metadata.spacing)) * 0.5,
        budget,
        projected: 0,
        seeded: 0,
        maximum_displacement: 0.,
        cache: BTreeMap::new(),
        cache_hits: 0,
        frozen_conflicts: 0,
        target_queries: 0,
        target_work: 0,
        rounding_updates: 0,
        outward_guard_rejections: 0,
        outward_split_guard_rejections: 0,
        feature_queries: 0,
        feature_query_work: 0,
        feature_cache_hits: 0,
        orientation_witness_queries: 0,
        orientation_witness_tests: 0,
        orientation_witness_candidates: 0,
    };
    let input_vertices = mesh.positions.len();
    let input_triangles = mesh.triangles.len();
    let mut face_source_indices: Vec<_> = (0..input_triangles).collect();
    projector.budget.charge(mesh.positions.len().saturating_mul(2))?;
    let mut origins = mesh.positions.clone();
    let mut next_pass_projection_cost = vec![0u32; mesh.positions.len()];
    let mut history = Vec::new();
    let mut skipped_edges = 0usize;
    let mut skipped_invalid_splits = 0usize;
    let mut midpoint_rounding_updates = 0usize;
    let mut feature_directed_splits = 0usize;
    let mut feature_split_proposals = 0usize;
    let mut feature_refined_faces = BTreeSet::new();
    let mut budget_limited_marking_passes = 0usize;
    let mut projected_split_vertices = 0usize;
    for pass in 0..=options.max_passes {
        projector.budget.charge(mesh.positions.len().saturating_mul(2))?;
        let mut iterations: Vec<_> = next_pass_projection_cost.iter().map(|&cost| 24u32 - cost).collect();
        next_pass_projection_cost.fill(0);
        projector.budget.charge(mesh.positions.len())?;
        let mut adjacent = vec![Vec::new(); mesh.positions.len()];
        for (face, triangle) in mesh.triangles.iter().enumerate() {
            projector.budget.charge(3)?;
            for &i in triangle {
                adjacent[i as usize].push(face);
            }
        }
        for _ in 0..24 {
            let progress = project_vertices(
                &mut mesh.positions,
                &mesh.triangles,
                &origins,
                input_vertices,
                &mut iterations,
                &adjacent,
                &mut projector,
            )
            .map_err(|error| {
                format!(
                    "{error}; projection pass {pass}, {} vertices/{} faces; prior passes {history:?}",
                    mesh.positions.len(),
                    mesh.triangles.len()
                )
            })?;
            if !progress {
                break;
            }
        }
        let mut failed = BTreeSet::new();
        let mut refine_faces = BTreeSet::new();
        let mut edge_errors = BTreeMap::<Edge, f64>::new();
        let mut orientation_failures = 0usize;
        let mut orientation_examples = Vec::new();
        let mut outward_parents = BTreeSet::new();
        let mut maximum = 0.0_f64;
        let mut worst = String::new();
        let mut vertex_values = Vec::with_capacity(mesh.positions.len());
        for (index, &p) in mesh.positions.iter().enumerate() {
            projector.budget.charge(1)?;
            let f = projector.sample(p)?.abs();
            if f > maximum {
                maximum = f;
                worst = format!("vertex {index} at {p:?}");
            }
            vertex_values.push(f);
        }
        for (i, &triangle) in mesh.triangles.iter().enumerate() {
            projector.budget.charge(1)?;
            if triangle.iter().any(|&i| vertex_values[i as usize] > f64::from(options.max_residual)) {
                failed.insert(i);
            }
            if triangle
                .iter()
                .any(|&i| vertex_values[i as usize] > f64::from(options.max_residual) * REFINEMENT_TRIGGER_FRACTION)
            {
                refine_faces.insert(i);
            }
            let points = triangle.map(|i| d(mesh.positions[i as usize]));
            for (sample_index, weights) in
                [[0.5, 0.5, 0.], [0., 0.5, 0.5], [0.5, 0., 0.5], [1. / 3.; 3]].into_iter().enumerate()
            {
                projector.budget.charge(1)?;
                let p = point((0..3).fold([0.; 3], |sum, i| add(sum, scale(points[i], weights[i]))))?;
                let error = projector.sample(p)?.abs();
                if sample_index < 3 {
                    projector.budget.charge(1)?;
                    let [a, b, c] = triangle;
                    let key = [edge(a, b), edge(b, c), edge(c, a)][sample_index];
                    edge_errors.entry(key).and_modify(|value| *value = value.max(error)).or_insert(error);
                }
                if error > maximum {
                    maximum = error;
                    worst = format!(
                        "face {i}, input face {}, indices {triangle:?}, query {p:?}, weights {weights:?}",
                        face_source_indices[i]
                    );
                }
                if error > f64::from(options.max_residual) {
                    failed.insert(i);
                }
                if error > f64::from(options.max_residual) * REFINEMENT_TRIGGER_FRACTION {
                    refine_faces.insert(i);
                }
            }
            let outward = projector.outward(&mesh.positions, triangle)?;
            if outward {
                projector.budget.charge(1)?;
                outward_parents.insert(triangle);
            }
            if !outward {
                failed.insert(i);
                refine_faces.insert(i);
                orientation_failures += 1;
                if orientation_examples.len() < 8 {
                    projector.budget.charge(4)?;
                    orientation_examples.push(json!({"face":i,"input_face":face_source_indices[i],
                        "indices":triangle,"positions":points,"normal":normal(&mesh.positions,triangle)?}));
                }
            }
        }
        if failed.is_empty() {
            let (components, links) = audit(&mesh.positions, &mesh.triangles, &mut projector.budget)
                .map_err(|error|format!("{error}; final audit pass {pass}, {} vertices/{} faces, maximum residual {maximum}; history {history:?}",mesh.positions.len(),mesh.triangles.len()))?;
            if components != initial_components {
                return Err("native refinement changed component count".into());
            }
            let embedding = validate_embedding(&mesh.positions, &mesh.triangles, &mut projector.budget)?;
            let origin = scale(add(bounds[0], bounds[1]), 0.5);
            let mut area = 0.;
            let mut volume = 0.;
            for &triangle in &mesh.triangles {
                projector.budget.charge(1)?;
                let [a, b, c] = triangle.map(|i| sub(d(mesh.positions[i as usize]), origin));
                area += 0.5 * norm(cross(sub(b, a), sub(c, a)));
                volume += dot(a, cross(b, c)) / 6.;
            }
            mesh.metadata.surface_area = area;
            mesh.metadata.signed_volume = volume;
            mesh.metadata.connected_components = components;
            mesh.metadata.boundary_edges = 0;
            let mut report = json!({"charged_work":projector.budget.used,"field_work":projector.budget.field,"structural_work":projector.budget.used-projector.budget.field,
                "field_queries":projector.budget.queries,"field_cache_hits":projector.cache_hits,"field_cache_entries":projector.cache.len(),
                "field_cache_entry_limit":262_144,"projection_iterations_per_vertex_per_pass_limit":24,"directional_iterations_per_attempt_limit":4,"line_search_steps_per_direction_limit":12,
                "projected_vertex_updates":projector.projected,"frozen_conflict_vertex_events":projector.frozen_conflicts,"outward_seed_steps":projector.seeded,"max_projection_displacement_m":projector.maximum_displacement,
                "closest_surface_target_queries":projector.target_queries,"closest_surface_target_work":projector.target_work,
                "target_rounding_vertex_proposals":projector.rounding_updates,"target_candidates_per_vertex_limit":8,"target_rounding_sweeps_limit":4,
                "outward_projection_guard_rejections":projector.outward_guard_rejections,
                "outward_split_guard_rejections":projector.outward_split_guard_rejections,
                "passes":pass,"input_vertices":input_vertices,"input_triangles":input_triangles,"added_vertices":mesh.positions.len()-input_vertices,
                "added_triangles":mesh.triangles.len()-input_triangles,"max_sampled_native_residual":maximum,"max_residual":options.max_residual,
                "normal_step_m":options.normal_step_m,"closed_oriented_edges":true,"vertex_links_checked":links,"connected_components":components,
                "skipped_unrepresentable_edges":skipped_edges,"skipped_invalid_split_edges":skipped_invalid_splits,"refinement_history":history,"worst_sample":worst,
                "midpoint_rounding_updates":midpoint_rounding_updates,"midpoint_candidates_per_vertex_limit":8,"midpoint_rounding_sweeps_limit":4,
                "embedding_validation":embedding,
                "scope":"Native field projection with finite vertex/edge/centroid residual and face orientation probes, plus a complete stored-f32 embedding proof by the reported method; no global missing-feature certificate"});
            let feature_report = json!({"refinement_trigger_fraction":REFINEMENT_TRIGGER_FRACTION,"feature_directed_splits":feature_directed_splits,
                "feature_directed_split_proposals":feature_split_proposals,"feature_region_queries":projector.feature_queries,
                "feature_region_query_work":projector.feature_query_work,"feature_region_cache_hits":projector.feature_cache_hits,
                "feature_refined_input_faces":feature_refined_faces.len(),
                "budget_limited_edge_marking_passes":budget_limited_marking_passes,
                "projected_split_vertices":projected_split_vertices,"split_candidate_phases_limit":8,"projected_split_candidates_per_vertex_limit":8,
                "local_orientation_witness_queries":projector.orientation_witness_queries,
                "local_orientation_exact_tests":projector.orientation_witness_tests,
                "local_orientation_candidate_acceptances":projector.orientation_witness_candidates,
                "feature_region_cache_entry_limit_per_pass":262_144});
            report.as_object_mut().unwrap().extend(feature_report.as_object().unwrap().clone());
            return Ok(RefinedMesh { mesh, face_source_indices, report });
        }
        if pass == options.max_passes {
            let diagnostics = json!({"charged_work":projector.budget.used,"field_work":projector.budget.field,
                "field_queries":projector.budget.queries,"orientation_failures":orientation_failures,
                "orientation_examples":orientation_examples,"history":history});
            return Err(format!("native surface refinement exceeds residual/orientation tolerance after {pass} passes; maximum residual {maximum}; {worst}; diagnostics={diagnostics}"));
        }
        let failed_faces = failed.len();
        let mut marked = BTreeSet::new();
        let marked_faces = refine_faces.len();
        let mut uniform_faces = mesh.triangles.len();
        let mut geometry_depth = 0u32;
        while uniform_faces <= options.max_triangles / 4 {
            projector.budget.charge(1)?;
            uniform_faces *= 4;
            geometry_depth += 1;
        }
        let budget_limited_marking = geometry_depth < options.max_passes - pass;
        if budget_limited_marking {
            budget_limited_marking_passes += 1;
        }
        for index in refine_faces {
            projector.budget.charge(3)?;
            let [a, b, c] = mesh.triangles[index];
            let pairs = [(a, b), (b, c), (c, a)];
            let active: Vec<_> = pairs
                .into_iter()
                .filter(|&(a, b)| {
                    edge_errors[&edge(a, b)] > f64::from(options.max_residual) * REFINEMENT_TRIGGER_FRACTION
                        || vertex_values[a as usize] > f64::from(options.max_residual) * REFINEMENT_TRIGGER_FRACTION
                        || vertex_values[b as usize] > f64::from(options.max_residual) * REFINEMENT_TRIGGER_FRACTION
                })
                .collect();
            if (budget_limited_marking || feature_refined_faces.contains(&face_source_indices[index]))
                && outward_parents.contains(&[a, b, c])
                && !active.is_empty()
            {
                for (a, b) in active {
                    marked.insert(edge(a, b));
                }
            } else {
                for (a, b) in pairs {
                    marked.insert(edge(a, b));
                }
            }
        }
        let existing_vertices = mesh.positions.len();
        let mut midpoints = BTreeMap::new();
        let mut feature_candidates = BTreeMap::new();
        let mut pending_origins = BTreeMap::new();
        let mut regions = BTreeMap::new();
        let feature_depth = (options.max_passes - pass).min(geometry_depth);
        for (a, b) in marked {
            projector.budget.charge(1)?;
            let event = if edge_errors[&edge(a, b)] > f64::from(options.max_residual) * REFINEMENT_TRIGGER_FRACTION {
                projector.feature_edge_split(a, b, &mesh.positions, &mut regions, feature_depth)?
            } else {
                None
            };
            let target =
                event.unwrap_or_else(|| scale(add(d(mesh.positions[a as usize]), d(mesh.positions[b as usize])), 0.5));
            let midpoint = point(target)?;
            if midpoint == mesh.positions[a as usize] || midpoint == mesh.positions[b as usize] {
                skipped_edges += 1;
                continue;
            }
            if mesh.positions.len() >= options.max_vertices {
                return Err(format!("native refinement exceeded vertex budget; pass {pass}, {failed_faces} failed/{marked_faces} marked faces, {worst}; history {history:?}"));
            }
            midpoints.insert((a, b), mesh.positions.len() as u32);
            projector.budget.charge(1)?;
            pending_origins.insert(mesh.positions.len() as u32, midpoint);
            if event.is_some() {
                projector.budget.charge(8)?;
                feature_candidates.insert(mesh.positions.len() as u32, rounded_target_candidates(target)?);
                feature_split_proposals += 1;
            }
            mesh.positions.push(midpoint);
        }
        let proposed_splits = midpoints.len();
        let (rounding, filtered_splits) = {
            let mut checks = NativeSplitChecks {
                projector: &mut projector,
                outward_parents: &outward_parents,
                interior_points: BTreeMap::new(),
                exact_points: BTreeMap::new(),
            };
            let rounding = round_split_midpoints(
                &mut mesh.positions,
                &mesh.triangles,
                &midpoints,
                &feature_candidates,
                &mut checks,
            )?;
            let filtered = restrict_splits(&mesh.positions, &mesh.triangles, &mut midpoints, &mut checks)?;
            (rounding, filtered)
        };
        midpoint_rounding_updates += rounding.changed;
        skipped_invalid_splits += filtered_splits;
        feature_directed_splits += midpoints.values().filter(|id| feature_candidates.contains_key(id)).count();
        for (&(a, b), id) in &midpoints {
            if feature_candidates.contains_key(id) {
                projector.budget.charge(adjacent[a as usize].len())?;
                for &face in &adjacent[a as usize] {
                    if mesh.triangles[face].contains(&b) {
                        feature_refined_faces.insert(face_source_indices[face]);
                    }
                }
            }
        }
        projector.budget.charge(midpoints.len())?;
        let selected_positions: Vec<_> = midpoints.values().map(|&i| mesh.positions[i as usize]).collect();
        mesh.positions.truncate(existing_vertices);
        for (index, p) in midpoints.values_mut().zip(selected_positions) {
            projector.budget.charge(3)?;
            let old_id = *index;
            let origin = pending_origins[&old_id];
            next_pass_projection_cost.push(*rounding.next_pass_projection_cost.get(&old_id).unwrap_or(&0));
            if rounding.projected.contains(&old_id) {
                projected_split_vertices += 1;
                projector.projected += 1;
                projector.maximum_displacement = projector.maximum_displacement.max(norm(sub(d(p), d(origin))));
            }
            *index = mesh.positions.len() as u32;
            mesh.positions.push(p);
            origins.push(origin);
        }
        if midpoints.is_empty() {
            return Err(format!("native refinement has no representable edge splits; {worst}; history {history:?}"));
        }
        history.push(
            json!({"pass":pass,"vertices":mesh.positions.len(),"faces":mesh.triangles.len(),"failed_faces":failed_faces,
            "marked_faces":marked_faces,"proposed_split_edges":proposed_splits,"filtered_split_edges":filtered_splits,"split_edges":midpoints.len(),"max_residual":maximum,"worst_sample":worst,
            "orientation_failures":orientation_failures,"orientation_examples":orientation_examples,
            "feature_directed_splits":feature_directed_splits,"feature_refined_input_faces":feature_refined_faces.len(),
            "feature_region_queries":projector.feature_queries,
            "budget_limited_edge_marking":budget_limited_marking,
            "charged_work":projector.budget.used,"field_queries":projector.budget.queries}),
        );
        let mut triangles = vec![];
        let mut sources = vec![];
        for (source, &[a, b, c]) in mesh.triangles.iter().enumerate() {
            projector.budget.charge(1)?;
            let pieces =
                split_pieces([a, b, c], [edge(a, b), edge(b, c), edge(c, a)].map(|key| midpoints.get(&key).copied()));
            let original = normal(&mesh.positions, [a, b, c])?;
            projector.budget.charge(pieces.len().saturating_mul(2))?;
            let mut world_orientation = true;
            for &piece in &pieces {
                if dot(original, normal(&mesh.positions, piece)?) <= 0. {
                    world_orientation = false;
                }
            }
            if !world_orientation {
                let mut checks = NativeSplitChecks {
                    projector: &mut projector,
                    outward_parents: &outward_parents,
                    interior_points: BTreeMap::new(),
                    exact_points: BTreeMap::new(),
                };
                if !checks.alternate_orientation(&mesh.positions, [a, b, c], &pieces)? {
                    return Err(
                        "native refinement inverted a child face without an interior orientation witness".into()
                    );
                }
            }
            for piece in pieces {
                if triangles.len() >= options.max_triangles {
                    return Err("native refinement exceeded triangle budget".into());
                }
                triangles.push(piece);
                sources.push(face_source_indices[source]);
            }
        }
        mesh.triangles = triangles;
        face_source_indices = sources;
        // Shared-edge red/green subdivision preserves the indexed topology.
        // Every child is checked above; the complete graph audit runs again
        // before any final mesh can be returned.
    }
    unreachable!("bounded refinement exits at max_passes")
}

#[cfg(test)]
mod split_tests {
    use super::*;
    #[test]
    fn captured_curved_split_has_a_strict_native_interior_orientation_witness() {
        use mm3e_kit::{surface::TriangleSurface, Material, Transform};
        use mm3e_orchestrator::{Object, Prim};
        let mut scene = Scene::new(8, 8);
        let id = scene
            .surface(
                TriangleSurface::new(
                    vec![Vec3::new(-0.3, -0.25, 0.014), Vec3::new(0.3, -0.25, 0.014), Vec3::new(-0.3, 0.35, 0.014)],
                    vec![[0, 1, 2]],
                    0.001,
                )
                .unwrap(),
            )
            .unwrap();
        let material = scene.material(Material::default());
        scene.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, material));
        let options = Options {
            max_residual: 0.000005,
            normal_step_m: 0.00005,
            max_work: 1_000_000,
            max_vertices: 16,
            max_triangles: 32,
            max_passes: 6,
        };
        let mut projector = Projector {
            scene: &scene,
            sampler: CountedSceneField::new(&scene).unwrap(),
            options: &options,
            bounds: [[-0.4, -0.4, -0.03], [0.4, 0.45, 0.05]],
            locality_m: 0.02,
            budget: Budget { maximum: 1_000_000, ..Budget::default() },
            projected: 0,
            seeded: 0,
            maximum_displacement: 0.,
            cache: BTreeMap::new(),
            cache_hits: 0,
            frozen_conflicts: 0,
            target_queries: 0,
            target_work: 0,
            rounding_updates: 0,
            outward_guard_rejections: 0,
            outward_split_guard_rejections: 0,
            feature_queries: 0,
            feature_query_work: 0,
            feature_cache_hits: 0,
            orientation_witness_queries: 0,
            orientation_witness_tests: 0,
            orientation_witness_candidates: 0,
        };
        let raw = [
            [0.2963382601737976, -0.2509806752204895, 0.014195621013641357],
            [0.29267650842666626, -0.25092387199401855, 0.01438269205391407],
            [0.30000001192092896, -0.2509995996952057, 0.014000771567225456],
            [0.29694855213165283, -0.2509951591491699, 0.01409842073917389],
            [0.2963382601737976, -0.2509807050228119, 0.014195515774190426],
        ];
        let positions = raw.map(|p| point(p).unwrap());
        let parents = [[0, 1, 2], [1, 3, 2]];
        let protected = BTreeSet::from(parents);
        for parent in parents {
            assert!(projector.outward(&positions, parent).unwrap());
        }
        let mut old = Budget { maximum: 100_000, ..Budget::default() };
        assert!(!split_valid(
            &positions,
            parents[0],
            [None, Some(4), None],
            normal(&positions, parents[0]).unwrap(),
            &mut old
        )
        .unwrap());
        let mut checks = NativeSplitChecks {
            projector: &mut projector,
            outward_parents: &protected,
            interior_points: BTreeMap::new(),
            exact_points: BTreeMap::new(),
        };
        assert!(split_valid(
            &positions,
            parents[0],
            [None, Some(4), None],
            normal(&positions, parents[0]).unwrap(),
            &mut checks
        )
        .unwrap());
        assert!(split_valid(
            &positions,
            parents[1],
            [None, None, Some(4)],
            normal(&positions, parents[1]).unwrap(),
            &mut checks
        )
        .unwrap());
        let mut children = split_pieces(parents[0], [None, Some(4), None]);
        children[0].swap(0, 1);
        assert!(!checks.alternate_orientation(&positions, parents[0], &children).unwrap());
        assert!(checks.projector.orientation_witness_queries > 0);
        drop(checks);
        let mut empty = scene.clone();
        empty.objects[0].mods.round = -0.001;
        let mut empty_projector = Projector {
            scene: &empty,
            sampler: CountedSceneField::new(&empty).unwrap(),
            cache: BTreeMap::new(),
            budget: Budget { maximum: 1_000_000, ..Budget::default() },
            ..projector
        };
        assert!(empty_projector.interior_witness(&positions, parents[0]).unwrap().is_none());
    }
    #[test]
    fn sheet_strip_retains_both_long_edge_splits_despite_nearest_midpoint_collision() {
        // Native-projected input face 610 from the original uniform sheet.
        let mut positions = vec![
            Vec3::new(-0.5158114, -0.04743417, -0.5),
            Vec3::new(-0.5353553, -0.035355337, -0.4792893),
            Vec3::new(-0.5353553, -0.035355337, -0.47928932),
        ];
        let original = positions.clone();
        let mut midpoints = BTreeMap::new();
        for (a, b) in [(0, 1), (0, 2)] {
            let midpoint = point(scale(add(d(positions[a]), d(positions[b])), 0.5)).unwrap();
            midpoints.insert((a as u32, b as u32), positions.len() as u32);
            positions.push(midpoint);
        }
        assert_eq!(positions[3], positions[4]);
        let triangles = [[0, 1, 2]];
        let reference = normal(&positions, triangles[0]).unwrap();
        let mut budget = Budget { maximum: 100_000, ..Budget::default() };
        assert!(!split_valid(&positions, triangles[0], [Some(3), None, Some(4)], reference, &mut budget).unwrap());
        assert!(
            round_split_midpoints(&mut positions, &triangles, &midpoints, &BTreeMap::new(), &mut budget)
                .unwrap()
                .changed
                > 0
        );
        assert_ne!(positions[3], positions[4]);
        assert_eq!(&positions[..3], &original);
        assert!(split_valid(&positions, triangles[0], [Some(3), None, Some(4)], reference, &mut budget).unwrap());
        assert_eq!(restrict_splits(&positions, &triangles, &mut midpoints, &mut budget).unwrap(), 0);
        assert_eq!(midpoints.len(), 2);
    }
    #[test]
    fn midpoint_candidates_retain_subnormal_tail_below_f64_midpoint_precision() {
        let tiny = f32::from_bits(1);
        let candidates = midpoint_candidates(Vec3::new(1., 0., 0.), Vec3::new(tiny, 0., 0.));
        assert_eq!(candidates, vec![Vec3::new(0.5, 0., 0.), Vec3::new(0.5_f32.next_up(), 0., 0.)]);
        let negative = midpoint_candidates(Vec3::new(-1., 0., 0.), Vec3::new(-tiny, 0., 0.));
        assert_eq!(negative, vec![Vec3::new((-0.5_f32).next_down(), 0., 0.), Vec3::new(-0.5, 0., 0.)]);
    }
    #[test]
    fn captured_radial_targets_require_coupled_rounding_before_projection() {
        // Original bundle face 190, with targets retained from the independent
        // analytic radial-projection diagnostic. Nearest rounding collapses it.
        let reference = [
            [0.641421377658844, -0.699999988079071, -0.10000000149011612],
            [0.6414214372634888, -0.7000000476837158, -0.10000000149011612],
            [0.641421377658844, -0.7000000476837158, -0.10000000149011612],
        ]
        .map(|p| point(p).unwrap());
        let targets = [
            [0.6281084909557224, -0.6678598460622667, -0.06785984750017156],
            [0.6281085204924965, -0.6678598601688043, -0.06785982115909463],
            [0.6281084832406004, -0.6678598678839296, -0.0678598288742182],
        ];
        let mut stored = targets.map(|p| point(p).unwrap());
        let triangle = [0, 1, 2];
        assert!(normal(&stored, triangle).is_err());
        let mut budget = Budget { maximum: 10_000, ..Budget::default() };
        let changed = resolve_surface_rounding(
            &mut stored,
            &targets.map(Some),
            &[triangle],
            &[vec![0], vec![0], vec![0]],
            &reference,
            &mut budget,
        )
        .unwrap();
        assert!(changed > 0);
        assert!(dot(normal(&reference, triangle).unwrap(), normal(&stored, triangle).unwrap()) > 0.);
        for (p, target) in stored.into_iter().zip(targets) {
            for (coordinate, target) in d(p).into_iter().zip(target) {
                let coordinate = coordinate as f32;
                assert!(f64::from(coordinate.next_down()) <= target);
                assert!(f64::from(coordinate.next_up()) >= target);
            }
        }
        let used = budget.used;
        let mut second = targets.map(|p| point(p).unwrap());
        let mut short = Budget { maximum: used - 1, ..Budget::default() };
        assert!(resolve_surface_rounding(
            &mut second,
            &targets.map(Some),
            &[triangle],
            &[vec![0], vec![0], vec![0]],
            &reference,
            &mut short,
        )
        .is_err());
    }
    #[test]
    fn captured_one_ulp_midpoint_combination_keeps_a_conforming_nondegenerate_subset() {
        let raw = [
            [3206820913, 3207803700, 3182634102],
            [3206820914, 3207803700, 3182634100],
            [3206820913, 3207803699, 3184315597],
        ];
        let mut positions: Vec<_> = raw
            .into_iter()
            .map(|p| {
                let p = p.map(f32::from_bits);
                Vec3::new(p[0], p[1], p[2])
            })
            .collect();
        positions.push(positions[0] + Vec3::new(0.01, 0.02, 0.03));
        let triangles = [[0, 2, 1], [0, 1, 3], [1, 2, 3], [2, 0, 3]];
        let mut midpoints = BTreeMap::new();
        for a in 0u32..4 {
            for b in a + 1..4 {
                let midpoint = point(scale(add(d(positions[a as usize]), d(positions[b as usize])), 0.5)).unwrap();
                midpoints.insert((a, b), positions.len() as u32);
                positions.push(midpoint);
            }
        }
        let mut budget = Budget { maximum: 1_000_000, ..Budget::default() };
        let [a, b, c] = triangles[0];
        let initial = [edge(a, b), edge(b, c), edge(c, a)].map(|key| midpoints.get(&key).copied());
        assert!(!split_valid(
            &positions,
            triangles[0],
            initial,
            normal(&positions, triangles[0]).unwrap(),
            &mut budget
        )
        .unwrap());
        let removed = restrict_splits(&positions, &triangles, &mut midpoints, &mut budget).unwrap();
        assert!(removed > 0 && !midpoints.is_empty());
        let mut output = Vec::new();
        for triangle @ [a, b, c] in triangles {
            let reference = normal(&positions, triangle).unwrap();
            for child in
                split_pieces(triangle, [edge(a, b), edge(b, c), edge(c, a)].map(|key| midpoints.get(&key).copied()))
            {
                assert!(dot(reference, normal(&positions, child).unwrap()) > 0.);
                output.push(child);
            }
        }
        assert!(output.len() > triangles.len());
        assert_eq!(audit(&positions, &output, &mut budget).unwrap().0, 1);
        mm3e_kit::surface_intersections::validate(&positions, &output, 1_000_000).unwrap();
    }

    #[test]
    fn embedding_fallback_keeps_disjoint_components_and_charges_every_attempt() {
        let tetra = [Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.), Vec3::new(0., 0., 1.)];
        let faces = [[0, 2, 1], [0, 1, 3], [1, 2, 3], [2, 0, 3]];
        let mut positions = tetra.to_vec();
        positions.extend(tetra.map(|p| p + Vec3::splat(3.)));
        let mut triangles = faces.to_vec();
        triangles.extend(faces.map(|t| t.map(|i| i + 4)));
        let mut budget = Budget { maximum: 1_000_000, ..Budget::default() };
        let result = validate_embedding(&positions, &triangles, &mut budget).unwrap();
        assert_eq!(result["method"], "triangle_pair_validation");
        assert_eq!(result["radial_attempt"]["kind"], "inconclusive");
        let spent = result["work"].as_u64().unwrap() as usize;
        let radial = result["radial_attempt"]["work"]["work"].as_u64().unwrap() as usize;
        let pairwise = result["pair_validation_work"].as_u64().unwrap() as usize;
        assert!(radial > 0 && pairwise > 0);
        assert_eq!(spent, radial + pairwise);
        assert_eq!(spent, budget.used);
        let mut exact = Budget { maximum: spent, ..Budget::default() };
        assert_eq!(validate_embedding(&positions, &triangles, &mut exact).unwrap(), result);
        let mut short = Budget { maximum: spent - 1, ..Budget::default() };
        assert!(validate_embedding(&positions, &triangles, &mut short).unwrap_err().contains("budget"));
        assert!(short.used >= radial && short.used < spent);
        positions[4..].copy_from_slice(&tetra);
        let mut overlapping = Budget { maximum: 1_000_000, ..Budget::default() };
        assert!(validate_embedding(&positions, &triangles, &mut overlapping)
            .unwrap_err()
            .contains("intersect or overlap"));
        assert!(overlapping.used > radial);
    }
}
