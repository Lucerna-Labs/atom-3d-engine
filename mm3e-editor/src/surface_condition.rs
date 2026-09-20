//! Bounded static conditioning of an already validated stored-f32 surface.
//!
//! The reference is the supplied mesh, not an unavailable exact producer mesh.
//! Only contractions to unchanged endpoints are considered. Every accepted
//! contraction preserves the full link and a counted source-embedding proof on
//! singleton enclosures of those exact f32 coordinates. Initial/final complete
//! stored embedding and direct frozen-to-final correspondence remain mandatory.
use mm3e_kit::{
    mesh_correspondence::{self, CorrespondenceReport},
    mesh_topology,
    meshing::{Mesh, MAX_MESH_TRIANGLES, MAX_MESH_VERTICES},
    surface_intersections, Vec3,
};
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Clone, Copy)]
pub struct Options {
    pub max_error_m: f64,
    pub max_work: usize,
}
#[derive(Debug, Clone, Serialize)]
pub struct Contraction {
    /// Indices in the unchanged input position array.
    pub removed_vertex: u32,
    pub retained_vertex: u32,
    pub max_displacement_m: f64,
    pub source_work: usize,
}
#[derive(Debug, Clone, Serialize, Default)]
pub struct Report {
    pub work: usize,
    pub initial_embedding_work: usize,
    pub initial_embedding_reused: bool,
    pub initial_certificate_match_work: usize,
    pub final_embedding_work: usize,
    pub output_certificate_work: usize,
    pub topology_work: usize,
    pub topology_snapshot_work: usize,
    pub topology_snapshots: usize,
    /// Aggregate preparation, successful/rejected proposals, commits and
    /// synchronization of the independent source/topology indexes.
    pub source_work: usize,
    pub source_preparation_work: usize,
    pub source_commit_work: usize,
    pub source_synchronization_work: usize,
    pub correspondence_work: usize,
    pub initial_short_edges: usize,
    pub remaining_short_edges: usize,
    pub rejected_topology: usize,
    pub rejected_source: usize,
    pub rejected_envelope: usize,
    pub max_displacement_m: f64,
    pub contractions: Vec<Contraction>,
}
pub struct ConditionedMesh {
    pub mesh: Mesh,
    /// Bound to the final compact arrays; absent on the original standalone API.
    pub embedding_certificate: Option<surface_intersections::EmbeddingCertificate>,
    /// Each surviving triangle retains its original input face.
    pub face_source_indices: Vec<usize>,
    /// Frozen original vertex to compact final vertex, for auditing composition.
    pub original_to_final: Vec<u32>,
    pub correspondence: CorrespondenceReport,
    pub report: Report,
}
#[derive(Debug, Clone)]
pub struct Failure {
    pub work: usize,
    pub message: String,
}
impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (conditioning work {})", self.message, self.work)
    }
}
impl std::error::Error for Failure {}
struct Budget {
    used: usize,
    maximum: usize,
}
impl Budget {
    fn charge(&mut self, amount: usize) -> Result<(), String> {
        if amount > self.maximum.saturating_sub(self.used) {
            return Err(format!("surface conditioning exhausted work budget (used {}, requested {amount})", self.used));
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
/// Outward Euclidean bound. Endpoint coordinates are exact f32 values promoted
/// to f64; scaling avoids losing subnormal differences while squaring.
fn displacement(a: Vec3, b: Vec3) -> f64 {
    let a = xyz(a);
    let b = xyz(b);
    let parts =
        std::array::from_fn::<_, 3, _>(
            |axis| {
                if a[axis] == b[axis] {
                    0.
                } else {
                    (a[axis] - b[axis]).abs().next_up()
                }
            },
        );
    let scale = parts.into_iter().fold(0., f64::max);
    if scale == 0. {
        return 0.;
    }
    let mut sum = 0.;
    for part in parts {
        if part != 0. {
            let q = (part / scale).next_up();
            sum = (sum + (q * q).next_up()).next_up();
        }
    }
    (scale * sum.sqrt().next_up()).next_up()
}
fn edges(triangles: &[[u32; 3]], budget: &mut Budget) -> Result<BTreeSet<(u32, u32)>, String> {
    budget.charge(triangles.len() * 4)?;
    Ok(triangles
        .iter()
        .flat_map(|&[a, b, c]| [(a.min(b), a.max(b)), (b.min(c), b.max(c)), (c.min(a), c.max(a))])
        .collect())
}
fn short_edges(mesh: &Mesh, maximum: f64, budget: &mut Budget) -> Result<Vec<(f64, u32, u32)>, String> {
    let edges = edges(&mesh.triangles, budget)?;
    budget.charge(edges.len() * 24)?;
    let mut short: Vec<_> = edges
        .into_iter()
        .filter_map(|(a, b)| {
            let distance = displacement(mesh.positions[a as usize], mesh.positions[b as usize]);
            (distance <= maximum).then_some((distance, a, b))
        })
        .collect();
    let levels = usize::BITS - short.len().max(1).leading_zeros();
    budget.charge(short.len() * (levels as usize + 1))?;
    short.sort_by(|a, b| a.0.total_cmp(&b.0).then((a.1, a.2).cmp(&(b.1, b.2))));
    Ok(short)
}

pub fn condition(mesh: Mesh, options: Options) -> Result<ConditionedMesh, Failure> {
    let mut budget = Budget { used: 0, maximum: options.max_work };
    condition_inner(mesh, options, &mut budget, None, false).map_err(|message| Failure { work: budget.used, message })
}
/// Reuse a complete input proof only after exact snapshot comparison, otherwise
/// perform the ordinary full input check. Certify the actual compact output.
pub fn condition_certified(
    mesh: Mesh,
    options: Options,
    input: Option<&surface_intersections::EmbeddingCertificate>,
) -> Result<ConditionedMesh, Failure> {
    let mut budget = Budget { used: 0, maximum: options.max_work };
    condition_inner(mesh, options, &mut budget, input, true).map_err(|message| Failure { work: budget.used, message })
}
fn condition_inner(
    mut mesh: Mesh,
    options: Options,
    budget: &mut Budget,
    input: Option<&surface_intersections::EmbeddingCertificate>,
    certify_output: bool,
) -> Result<ConditionedMesh, String> {
    if mesh.positions.is_empty()
        || mesh.positions.len() > MAX_MESH_VERTICES
        || mesh.triangles.is_empty()
        || mesh.triangles.len() > MAX_MESH_TRIANGLES
        || !options.max_error_m.is_finite()
        || options.max_error_m < 0.
    {
        return Err("surface conditioning requires bounded nonempty geometry and a finite nonnegative allowance".into());
    }
    let mut report = Report::default();
    let mut topology = match mesh_topology::PreparedTopology::new(&mesh.triangles, budget.remaining()) {
        Ok(value) => {
            budget.charge(value.initial_report().work)?;
            report.topology_work += value.initial_report().work;
            value
        }
        Err(error) => {
            budget.charge(error.work)?;
            return Err(error.message);
        }
    };
    let initial_components = topology.initial_report().components;
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
    budget.charge(mesh.positions.len() * 5 + mesh.triangles.len() * 2)?;
    let bounds: Vec<_> = mesh.positions.iter().map(|&p| [xyz(p); 2]).collect();
    let mut source_index =
        match surface_intersections::PreparedSourceContractions::new(&bounds, &mesh.triangles, budget.remaining()) {
            Ok(value) => {
                budget.charge(value.preparation_work())?;
                report.source_preparation_work = value.preparation_work();
                report.source_work += value.preparation_work();
                value
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        };
    let original_triangles = mesh.triangles.clone();
    let mut ancestry: Vec<_> = (0..mesh.triangles.len()).collect();
    let mut mapping: Vec<_> = (0..mesh.positions.len() as u32).collect();
    let mut clusters: Vec<Vec<usize>> = (0..mesh.positions.len()).map(|v| vec![v]).collect();
    // One deterministic pass over input edges. Composed endpoints are checked
    // against the current complex each time. Remaining short edges are reported;
    // their elimination is not falsely implied by a successful static certificate.
    let proposals = short_edges(&mesh, options.max_error_m, budget)?;
    report.initial_short_edges = proposals.len();
    for (_, old_a, old_b) in proposals {
        budget.charge(2)?;
        let a = mapping[old_a as usize];
        let b = mapping[old_b as usize];
        if a == b {
            continue;
        }
        budget.charge((clusters[a as usize].len() + clusters[b as usize].len()) * 48)?;
        let envelope = |retained: u32| {
            clusters[a as usize]
                .iter()
                .chain(&clusters[b as usize])
                .map(|&original| displacement(mesh.positions[original], mesh.positions[retained as usize]))
                .fold(0., f64::max)
        };
        let mut directions = [(envelope(b), a, b), (envelope(a), b, a)];
        directions.sort_by(|a, b| a.0.total_cmp(&b.0).then((a.2, a.1).cmp(&(b.2, b.1))));
        if directions[0].0 > options.max_error_m {
            report.rejected_envelope += 1;
            continue;
        }
        for (bound, removed, retained) in directions {
            if bound > options.max_error_m {
                report.rejected_envelope += 1;
                continue;
            }
            let ticket = match topology.check_collapse(removed, retained, budget.remaining()) {
                Ok(value) => {
                    budget.charge(value.report().work)?;
                    report.topology_work += value.report().work;
                    value
                }
                Err(error) => {
                    budget.charge(error.work)?;
                    report.topology_work += error.work;
                    if error.message.contains("work budget") {
                        return Err(error.message);
                    }
                    report.rejected_topology += 1;
                    // Full-link eligibility is independent of direction and no
                    // state changed, so the reverse direction cannot fix it.
                    break;
                }
            };
            let source_ticket = match source_index.check_contraction(removed, retained, budget.remaining()) {
                Ok(value) => {
                    budget.charge(value.report().work)?;
                    report.source_work += value.report().work;
                    value
                }
                Err(error) => {
                    budget.charge(error.work)?;
                    report.source_work += error.work;
                    if error.message.contains("work budget") {
                        return Err(error.message);
                    }
                    report.rejected_source += 1;
                    continue;
                }
            };
            let source_work = source_ticket.report().work;
            // Both immutable admissions succeeded against matching revisions.
            // A failure during either commit aborts this owned operation; it
            // never retries with one private index ahead of the other.
            let topology_delta = match topology.commit_collapse(ticket, budget.remaining()) {
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
            let source_delta = match source_index.commit_contraction(source_ticket, budget.remaining()) {
                Ok(value) => {
                    budget.charge(value.work)?;
                    report.source_commit_work += value.work;
                    report.source_work += value.work;
                    value
                }
                Err(error) => {
                    budget.charge(error.work)?;
                    return Err(error.message);
                }
            };
            let synchronization_work = topology_delta.removed_faces.len()
                + source_delta.removed_faces.len()
                + (topology_delta.updated_faces.len() + source_delta.updated_faces.len()) * 4
                + 1;
            budget.charge(synchronization_work)?;
            report.source_synchronization_work += synchronization_work;
            report.source_work += synchronization_work;
            if topology_delta.revision != source_delta.revision
                || topology_delta.removed_faces != source_delta.removed_faces
                || topology_delta.updated_faces != source_delta.updated_faces
            {
                return Err("surface conditioning source/topology commit deltas diverged".into());
            }
            budget.charge(clusters[removed as usize].len() * 2 + 1)?;
            let incoming = std::mem::take(&mut clusters[removed as usize]);
            for &original in &incoming {
                mapping[original] = retained;
            }
            clusters[retained as usize].extend(incoming);
            report.contractions.push(Contraction {
                removed_vertex: removed,
                retained_vertex: retained,
                max_displacement_m: bound,
                source_work,
            });
            break;
        }
    }
    // Every later proposal reads the two private updated indexes, plus fixed
    // coordinates and the composed vertex map. Copying their complete current
    // face arrays between proposals supplies no additional validation. Preserve
    // per-commit delta agreement, then materialize the final topology once.
    if !report.contractions.is_empty() {
        let snapshot = match topology.snapshot(budget.remaining()) {
            Ok(value) => {
                budget.charge(value.work)?;
                report.topology_work += value.work;
                report.topology_snapshot_work = value.work;
                report.topology_snapshots = 1;
                value
            }
            Err(error) => {
                budget.charge(error.work)?;
                return Err(error.message);
            }
        };
        mesh.triangles = snapshot.triangles;
        ancestry = snapshot.original_face_indices;
    }
    let source_snapshot = match source_index.snapshot(budget.remaining()) {
        Ok(value) => {
            budget.charge(value.work)?;
            report.source_synchronization_work += value.work;
            report.source_work += value.work;
            value
        }
        Err(error) => {
            budget.charge(error.work)?;
            return Err(error.message);
        }
    };
    let synchronization_work = mesh.triangles.len() * 4 + 1;
    budget.charge(synchronization_work)?;
    report.source_synchronization_work += synchronization_work;
    report.source_work += synchronization_work;
    if source_snapshot.revision != topology.revision()
        || source_snapshot.triangles != mesh.triangles
        || source_snapshot.original_face_indices != ancestry
    {
        return Err("surface conditioning final source/topology snapshots diverged".into());
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
    if final_topology.components != initial_components {
        return Err("surface conditioning changed the number of closed components".into());
    }
    if !certify_output {
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
    }
    let correspondence = match mesh_correspondence::certify_counted(
        &bounds,
        &original_triangles,
        &mapping,
        &mesh.positions,
        &mesh.triangles,
        mesh_correspondence::Options { max_error_m: options.max_error_m, max_work: budget.remaining() },
    ) {
        Ok(value) => {
            budget.charge(value.work)?;
            report.correspondence_work = value.work;
            value
        }
        Err(error) => {
            budget.charge(error.work)?;
            return Err(error.message);
        }
    };
    report.max_displacement_m = correspondence.max_displacement_m;
    report.remaining_short_edges = short_edges(&mesh, options.max_error_m, budget)?.len();
    budget.charge(mesh.positions.len() * 4 + mesh.triangles.len() * 40)?;
    let mut used = vec![false; mesh.positions.len()];
    for triangle in &mesh.triangles {
        for &vertex in triangle {
            used[vertex as usize] = true;
        }
    }
    let mut compact_map = vec![u32::MAX; mesh.positions.len()];
    let mut compact = Vec::new();
    for (vertex, &position) in mesh.positions.iter().enumerate() {
        if used[vertex] {
            compact_map[vertex] = compact.len() as u32;
            compact.push(position);
        }
    }
    for triangle in &mut mesh.triangles {
        *triangle = triangle.map(|v| compact_map[v as usize]);
    }
    for vertex in &mut mapping {
        *vertex = compact_map[*vertex as usize];
    }
    mesh.positions = compact;
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
        None
    };
    mesh.metadata.boundary_edges = 0;
    mesh.metadata.connected_components = final_topology.components;
    let origin =
        std::array::from_fn::<_, 3, _>(|axis| (xyz(mesh.metadata.min)[axis] + xyz(mesh.metadata.max)[axis]) * 0.5);
    let mut area = 0.;
    let mut volume = 0.;
    for triangle in &mesh.triangles {
        let [a, b, c] = triangle
            .map(|v| std::array::from_fn::<_, 3, _>(|axis| xyz(mesh.positions[v as usize])[axis] - origin[axis]));
        let u = std::array::from_fn::<_, 3, _>(|axis| b[axis] - a[axis]);
        let v = std::array::from_fn::<_, 3, _>(|axis| c[axis] - a[axis]);
        let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        area += n[0].hypot(n[1]).hypot(n[2]) * 0.5;
        let bc = [b[1] * c[2] - b[2] * c[1], b[2] * c[0] - b[0] * c[2], b[0] * c[1] - b[1] * c[0]];
        volume += (0..3).map(|axis| a[axis] * bc[axis]).sum::<f64>() / 6.;
    }
    if !area.is_finite() || !volume.is_finite() {
        return Err("surface conditioning final measurements exceed finite bounds".into());
    }
    mesh.metadata.surface_area = area;
    mesh.metadata.signed_volume = volume;
    report.work = budget.used;
    Ok(ConditionedMesh {
        mesh,
        embedding_certificate,
        face_source_indices: ancestry,
        original_to_final: mapping,
        correspondence,
        report,
    })
}
