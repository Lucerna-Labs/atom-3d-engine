//! Sufficient stored-mesh embedding certificate through a degree-one radial map.
//!
//! Full oriented manifold topology, strict face-plane signs and one exact
//! generic forward ray are all mandatory. Failure is never a partial proof.
//! See docs/RADIAL_EMBEDDING.md; general intersection validators are unchanged.
use crate::{
    exact_geometry::{self, Point},
    mesh_topology,
    meshing::{MAX_MESH_TRIANGLES, MAX_MESH_VERTICES},
    Vec3,
};

pub const MAX_CENTRE_CANDIDATES: usize = 2;
pub const MAX_RAY_CANDIDATES: usize = 4;
const MAX_CENTRE_TERMS: usize = 32;
const DIRECTIONS: [[f64; 3]; MAX_RAY_CANDIDATES] =
    [[1., 5. / 16., 11. / 64.], [13. / 64., 1., 7. / 16.], [9. / 32., 15. / 64., 1.], [1., -13. / 64., 7. / 32.]];
pub const SCOPE:&str="Embedding of this finite stored indexed triangle surface by a degree-one radial projection; not source-field coverage, between-sample deformation, or material/UV transport";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureKind {
    InvalidInput,
    Inconclusive,
    Budget,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CentreKind {
    BoundsMidpoint,
    VertexMean,
}
const CENTRES: [CentreKind; MAX_CENTRE_CANDIDATES] = [CentreKind::BoundsMidpoint, CentreKind::VertexMean];
/// Exact rational centre: coordinate i is sum(numerators[i])/denominator.
#[derive(Clone, Debug, PartialEq)]
pub struct CentreDefinition {
    pub kind: CentreKind,
    pub numerators: [Vec<f64>; 3],
    pub denominator: f64,
}
/// The virtual endpoint is encoded exactly; it is never a rounded C+D point.
#[derive(Clone, Debug, PartialEq)]
pub struct RayDefinition {
    pub direction: [f64; 3],
    pub endpoint_numerators: [Vec<f64>; 3],
    pub denominator: f64,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkReport {
    pub work: usize,
    pub structural_work: usize,
    pub topology_work: usize,
    pub point_work: usize,
    pub centre_work: usize,
    pub plane_work: usize,
    pub ray_work: usize,
    pub centres_attempted: usize,
    pub rays_attempted: usize,
    pub face_plane_tests: usize,
    pub edge_cone_tests: usize,
    pub integer_fallbacks: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub work: WorkReport,
    pub topology: mesh_topology::TopologyReport,
    pub input_vertices: usize,
    pub faces: usize,
    pub edges: usize,
    pub euler_characteristic: isize,
    pub centre: CentreDefinition,
    pub ray: RayDefinition,
    pub degree: i8,
    pub forward_face_hits: usize,
    pub scope: &'static str,
}
/// Only a completed certificate can construct this opaque success value.
#[derive(Debug)]
pub struct Certificate {
    report: Report,
}
impl Certificate {
    pub fn report(&self) -> &Report {
        &self.report
    }
    pub fn into_report(self) -> Report {
        self.report
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    pub kind: FailureKind,
    pub message: String,
    pub work: Box<WorkReport>,
}
impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}
impl std::error::Error for Failure {}
#[derive(Clone, Copy)]
enum Ledger {
    Structural,
    Topology,
    Point,
    Centre,
    Plane,
    Ray,
}
struct Budget {
    limit: usize,
    work: WorkReport,
}
impl Budget {
    fn remaining(&self) -> usize {
        self.limit - self.work.work
    }
    fn failure(&self, kind: FailureKind, message: impl Into<String>) -> Failure {
        Failure { kind, message: message.into(), work: Box::new(self.work.clone()) }
    }
    fn charge(&mut self, n: usize, ledger: Ledger) -> Result<(), Failure> {
        if n > self.remaining() {
            return Err(self.failure(FailureKind::Budget, "radial embedding exhausted work budget"));
        }
        self.work.work += n;
        let slot = match ledger {
            Ledger::Structural => &mut self.work.structural_work,
            Ledger::Topology => &mut self.work.topology_work,
            Ledger::Point => &mut self.work.point_work,
            Ledger::Centre => &mut self.work.centre_work,
            Ledger::Plane => &mut self.work.plane_work,
            Ledger::Ray => &mut self.work.ray_work,
        };
        *slot += n;
        Ok(())
    }
    fn exact_failure(&self, message: String) -> Failure {
        self.failure(
            if message.contains("budget") { FailureKind::Budget } else { FailureKind::Inconclusive },
            format!("radial embedding exact predicate: {message}"),
        )
    }
}
fn make_point(rows: &[Vec<f64>; 3], denominator: f64, ledger: Ledger, budget: &mut Budget) -> Result<Point, Failure> {
    match Point::from_expansions([&rows[0], &rows[1], &rows[2]], &[denominator], budget.remaining()) {
        Ok(result) => {
            budget.charge(result.work, ledger)?;
            Ok(result.point)
        }
        Err(error) => {
            budget.charge(error.work, ledger)?;
            Err(budget.exact_failure(error.message))
        }
    }
}
fn orientation(points: [&Point; 4], ledger: Ledger, budget: &mut Budget) -> Result<i8, Failure> {
    match exact_geometry::orient3(points, budget.remaining()) {
        Ok(result) => {
            budget.charge(result.work, ledger)?;
            budget.work.integer_fallbacks += usize::from(result.integer_fallback);
            Ok(result.sign)
        }
        Err(error) => {
            budget.charge(error.work, ledger)?;
            Err(budget.exact_failure(error.message))
        }
    }
}
fn grow(values: &[f64], mut x: f64, budget: &mut Budget) -> Result<Vec<f64>, Failure> {
    budget.charge(values.len() + 1, Ledger::Centre)?;
    let mut next = Vec::with_capacity(values.len() + 1);
    for &v in values {
        let sum = x + v;
        let vc = sum - x;
        let vv = sum - vc;
        let error = (x - vv) + (v - vc);
        if error != 0. {
            next.push(error);
        }
        x = sum;
    }
    if x != 0. {
        next.push(x);
    }
    // At most 2M finite f32 terms span fewer than 300 significant exponent
    // positions. The extra fixed bound makes any unexpected growth inconclusive.
    if next.len() > MAX_CENTRE_TERMS {
        return Err(budget.failure(FailureKind::Inconclusive, "radial centre expansion exceeded its fixed term cap"));
    }
    Ok(next)
}
fn centre(positions: &[Vec3], kind: CentreKind, budget: &mut Budget) -> Result<CentreDefinition, Failure> {
    let mut numerators: [Vec<f64>; 3] = std::array::from_fn(|_| Vec::new());
    let denominator;
    match kind {
        CentreKind::BoundsMidpoint => {
            denominator = 2.;
            let mut min = [f64::INFINITY; 3];
            let mut max = [f64::NEG_INFINITY; 3];
            for p in positions {
                budget.charge(6, Ledger::Centre)?;
                for (i, v) in [p.x as f64, p.y as f64, p.z as f64].into_iter().enumerate() {
                    min[i] = min[i].min(v);
                    max[i] = max[i].max(v);
                }
            }
            budget.charge(6, Ledger::Centre)?;
            for i in 0..3 {
                numerators[i] = vec![min[i], max[i]];
            }
        }
        CentreKind::VertexMean => {
            denominator = positions.len() as f64;
            for p in positions {
                for (i, v) in [p.x as f64, p.y as f64, p.z as f64].into_iter().enumerate() {
                    numerators[i] = grow(&numerators[i], v, budget)?;
                }
            }
        }
    }
    Ok(CentreDefinition { kind, numerators, denominator })
}
// Rational coordinates are needed only for vertices reached by an exact
// face predicate. Most inconclusive centres fail after a short face prefix;
// retaining those points across centre attempts avoids whole-mesh conversion.
struct PointCache<'a> {
    positions: &'a [Vec3],
    points: Vec<Option<Point>>,
}
impl<'a> PointCache<'a> {
    fn new(positions: &'a [Vec3], budget: &mut Budget) -> Result<Self, Failure> {
        budget.charge(positions.len(), Ledger::Structural)?;
        Ok(Self { positions, points: (0..positions.len()).map(|_| None).collect() })
    }
    fn prepare(&mut self, triangle: [u32; 3], budget: &mut Budget) -> Result<(), Failure> {
        budget.charge(3, Ledger::Structural)?;
        for index in triangle {
            let slot = &mut self.points[index as usize];
            if slot.is_none() {
                let p = self.positions[index as usize];
                let rows = [vec![p.x as f64], vec![p.y as f64], vec![p.z as f64]];
                *slot = Some(make_point(&rows, 1., Ledger::Point, budget)?);
            }
        }
        Ok(())
    }
    fn get(&self, index: u32) -> &Point {
        self.points[index as usize].as_ref().expect("face points prepared before predicates")
    }
}
fn common_plane_sign(
    points: &mut PointCache<'_>,
    triangles: &[[u32; 3]],
    centre: &Point,
    budget: &mut Budget,
) -> Result<Option<i8>, Failure> {
    let mut common = 0;
    for t in triangles {
        budget.charge(1, Ledger::Structural)?;
        points.prepare(*t, budget)?;
        budget.work.face_plane_tests += 1;
        let sign = orientation([points.get(t[0]), points.get(t[1]), points.get(t[2]), centre], Ledger::Plane, budget)?;
        if sign == 0 || common != 0 && sign != common {
            return Ok(None);
        }
        common = sign;
    }
    Ok(Some(common))
}
fn ray_hits(
    points: &PointCache<'_>,
    triangles: &[[u32; 3]],
    centre: &Point,
    endpoint: &Point,
    sign: i8,
    budget: &mut Budget,
) -> Result<Option<usize>, Failure> {
    let mut hits = 0;
    for t in triangles {
        let mut interior = true;
        for i in 0..3 {
            budget.charge(1, Ledger::Structural)?;
            budget.work.edge_cone_tests += 1;
            let edge_sign =
                -orientation([centre, endpoint, points.get(t[i]), points.get(t[(i + 1) % 3])], Ledger::Ray, budget)?;
            if edge_sign == 0 {
                return Ok(None);
            }
            interior &= edge_sign == sign;
        }
        hits += usize::from(interior);
    }
    Ok(Some(hits))
}
/// Try at most two exact centres and four fixed dyadic directions per centre.
/// Any failure retains all spent setup, unsuccessful-candidate and predicate
/// work. Inconclusive is not rejection of embedding: callers may run their
/// unchanged general validator using the remaining shared budget.
///
/// The certificate applies to the supplied arrays at this call, not to future
/// geometry edits. No cached proof or caller-provided topology assertion is used.
pub fn certify(positions: &[Vec3], triangles: &[[u32; 3]], max_work: usize) -> Result<Certificate, Failure> {
    let mut budget = Budget { limit: max_work, work: WorkReport::default() };
    budget.charge(1, Ledger::Structural)?;
    if positions.is_empty()
        || positions.len() > MAX_MESH_VERTICES
        || triangles.is_empty()
        || triangles.len() > MAX_MESH_TRIANGLES
    {
        return Err(budget.failure(FailureKind::InvalidInput, "radial embedding input exceeds fixed geometry bounds"));
    }
    for p in positions {
        budget.charge(3, Ledger::Structural)?;
        if !p.x.is_finite() || !p.y.is_finite() || !p.z.is_finite() {
            return Err(budget.failure(FailureKind::InvalidInput, "radial embedding positions must be finite"));
        }
    }
    for t in triangles {
        budget.charge(3, Ledger::Structural)?;
        if t.iter().any(|&i| i as usize >= positions.len()) || t[0] == t[1] || t[1] == t[2] || t[2] == t[0] {
            return Err(budget.failure(FailureKind::InvalidInput, "radial embedding triangle indices are invalid"));
        }
    }
    let topology = match mesh_topology::validate(triangles, budget.remaining()) {
        Ok(report) => {
            budget.charge(report.work, Ledger::Topology)?;
            report
        }
        Err(error) => {
            budget.charge(error.work, Ledger::Topology)?;
            return Err(budget.failure(
                if error.message.contains("budget") { FailureKind::Budget } else { FailureKind::Inconclusive },
                format!("radial embedding requires a closed oriented manifold: {}", error.message),
            ));
        }
    };
    let euler = topology.vertices as isize - topology.edges as isize + topology.triangles as isize;
    if topology.components != 1 || euler != 2 {
        return Err(budget.failure(
            FailureKind::Inconclusive,
            format!(
                "radial embedding requires one component and Euler characteristic 2; found {} and {euler}",
                topology.components
            ),
        ));
    }
    let mut points = PointCache::new(positions, &mut budget)?;
    for kind in CENTRES {
        budget.charge(1, Ledger::Structural)?;
        budget.work.centres_attempted += 1;
        let definition = centre(positions, kind, &mut budget)?;
        let c = make_point(&definition.numerators, definition.denominator, Ledger::Centre, &mut budget)?;
        let Some(sign) = common_plane_sign(&mut points, triangles, &c, &mut budget)? else { continue };
        for direction in DIRECTIONS {
            budget.charge(1, Ledger::Structural)?;
            budget.work.rays_attempted += 1;
            // With <=2M vertices and these dyadic directions, n*D has fewer
            // than 30 significant bits. Each appended term is exactly f64.
            budget.charge(definition.numerators.iter().map(Vec::len).sum::<usize>() + 3, Ledger::Centre)?;
            let mut endpoint_numerators = definition.numerators.clone();
            for i in 0..3 {
                endpoint_numerators[i].push(definition.denominator * direction[i]);
            }
            let q = make_point(&endpoint_numerators, definition.denominator, Ledger::Centre, &mut budget)?;
            let Some(hits) = ray_hits(&points, triangles, &c, &q, sign, &mut budget)? else { continue };
            if hits != 1 {
                return Err(budget.failure(
                    FailureKind::Inconclusive,
                    format!(
                        "radial embedding exact generic ray has {hits} forward hits (signed degree {}), requiring one",
                        i64::from(sign) * hits as i64
                    ),
                ));
            }
            budget.charge(1, Ledger::Structural)?;
            let faces = topology.triangles;
            let edges = topology.edges;
            return Ok(Certificate {
                report: Report {
                    work: budget.work,
                    topology,
                    input_vertices: positions.len(),
                    faces,
                    edges,
                    euler_characteristic: euler,
                    centre: definition,
                    ray: RayDefinition {
                        direction,
                        endpoint_numerators,
                        denominator: match kind {
                            CentreKind::BoundsMidpoint => 2.,
                            CentreKind::VertexMean => positions.len() as f64,
                        },
                    },
                    degree: sign,
                    forward_face_hits: hits,
                    scope: SCOPE,
                },
            });
        }
    }
    Err(budget.failure(FailureKind::Inconclusive,"radial embedding found no strict common-sign centre and exact generic degree-one ray within its fixed candidate caps"))
}

#[cfg(test)]
#[path = "radial_embedding_tests.rs"]
mod tests;

/// Additive complete radial proof plus an exact immutable snapshot for later
/// counted reuse. The ordinary `certify` API and its work remain unchanged.
#[derive(Debug)]
pub struct CertifiedRadialEmbedding {
    pub report: Report,
    pub certificate: crate::surface_intersections::EmbeddingCertificate,
    pub certificate_work: usize,
    pub work: usize,
}
pub fn certify_snapshot(
    positions: &[Vec3],
    triangles: &[[u32; 3]],
    max_work: usize,
) -> Result<CertifiedRadialEmbedding, Failure> {
    let report = certify(positions, triangles, max_work)?.into_report();
    let proof_work = report.work.work;
    let captured = crate::surface_intersections::capture_completed_embedding(
        positions,
        triangles,
        crate::surface_intersections::EmbeddingProofMethod::Radial,
        proof_work,
        max_work - proof_work,
    )
    .map_err(|error| {
        let mut work = report.work.clone();
        work.work += error.work;
        work.structural_work += error.work;
        Failure {
            kind: if error.message.contains("budget") { FailureKind::Budget } else { FailureKind::Inconclusive },
            message: error.message,
            work: Box::new(work),
        }
    })?;
    Ok(CertifiedRadialEmbedding {
        work: proof_work + captured.work,
        certificate_work: captured.work,
        report,
        certificate: captured.certificate,
    })
}
