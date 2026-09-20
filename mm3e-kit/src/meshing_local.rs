//! Cell-local hard-Boolean extraction with stable global feature identities.
//!
//! The caller may omit a source only after proving its zero-set support disjoint
//! from the complete closed cell. An empty program asserts the entire cell is
//! outside. Each remaining leaf is sampled at all eight global lattice nodes;
//! the existing affine tetrahedral arrangement preserves thin constituent planes.
//! Curved leaves are still approximated. Native-field validation/refinement and
//! independent coarse/fine comparison remain the caller's responsibility.
use crate::{
    meshing::{
        finish_mesh, Mesh, MeshingMetadata, MAX_GRID_CELLS, MAX_GRID_SAMPLES, MAX_MESH_TRIANGLES, MAX_MESH_VERTICES,
    },
    meshing_boolean::{
        self as boolean, BooleanExpr, Context, Output, WorkBudget, MAX_ACTIVE_PLANES_PER_TETRAHEDRON,
        MAX_BOOLEAN_ARRANGEMENT_WORK, MAX_BOOLEAN_CHANNELS,
    },
    Vec3,
};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_LOCAL_SHARED_VALUES: usize = 1_048_576;
pub const MAX_LOCAL_GLOBAL_FEATURES: usize = 1_048_576;
pub const MAX_LOCAL_SUPPORT_VALUES: usize = 8_388_608;

#[derive(Clone, Copy, Debug)]
pub struct LocalCell {
    pub index: [u32; 3],
    /// Corner order uses bit0=x, bit1=y, bit2=z, matching the global extractor.
    pub node_ids: [u32; 8],
    pub points: [Vec3; 8],
}
#[derive(Clone, Copy, Debug)]
pub struct LocalFeature {
    /// Stable signed scalar-function identity; u64::MAX is reserved for weld keys.
    pub id: u64,
    pub values: [f32; 8],
}
#[derive(Clone, Debug)]
pub struct LocalCellData {
    pub features: Vec<LocalFeature>,
    /// Leaf indices refer to the supplied feature order; sorting is internal.
    /// None is valid only with no features and a proof that the cell is outside.
    pub expression: Option<BooleanExpr>,
    /// Callback work actually charged before source queries/sampling. A callback
    /// must respect its supplied remaining budget, including when it returns Err.
    pub work: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct LocalExtractionOptions {
    /// Aggregate callback plus structural work. Structural arrangement retains
    /// the existing independent 64M hard cap; this does not increase that cap.
    pub max_work: usize,
    pub max_vertices: usize,
    pub max_triangles: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct ConvexRepresentationPolicy {
    pub max_representation_error_m: f64,
}
#[derive(Clone, Debug)]
pub struct ConvexContraction {
    pub removed_vertex: u32,
    pub retained_vertex: u32,
    pub max_displacement_m: f64,
    pub source_check_work: usize,
    pub affected_triangles: usize,
}
#[derive(Clone, Debug)]
pub struct ConvexRoundingSearch {
    /// Satisfaction of the finite stored face predicate, independently followed
    /// by the exact stored embedding and source correspondence gates.
    pub status: &'static str,
    pub work: usize,
    pub variables: usize,
    pub participants: usize,
    pub constraints: usize,
    pub allowed_tuples: usize,
    pub decisions: usize,
    pub stable_faces: usize,
    pub component_limit: Option<(&'static str, usize)>,
}
#[derive(Clone, Debug)]
pub struct ConvexRepresentationReport {
    pub contractions: Vec<ConvexContraction>,
    pub rejected_proposals: usize,
    pub source_check_work: usize,
    pub max_representation_error_m: f64,
    pub correspondence: crate::mesh_correspondence::CorrespondenceReport,
    /// Complete stored-geometry certificate. If the exhaustive initial contact
    /// inventory is empty, it already certifies the identical final mesh.
    pub embedding: crate::surface_intersections::IntersectionReport,
    /// Exact immutable snapshot of the represented mesh before local cleanup.
    /// Consumers must match their actual arrays; repairs or compaction can
    /// invalidate identity even when the geometry remains equivalent.
    pub embedding_certificate: crate::surface_intersections::EmbeddingCertificate,
    pub embedding_certificate_work: usize,
    /// Counted exact snapshot matching/reindex verification after cleanup.
    pub embedding_certificate_binding_work: usize,
    /// Preparation, inventory, every candidate check, and any final recheck.
    pub embedding_validation_work: usize,
    pub embedding_repair_attempts: usize,
    pub embedding_initial_contacts: usize,
    pub rounding_searches: Vec<ConvexRoundingSearch>,
    pub adjacent_sweeps_certified: bool,
}
impl Default for LocalExtractionOptions {
    fn default() -> Self {
        Self {
            max_work: MAX_BOOLEAN_ARRANGEMENT_WORK,
            max_vertices: MAX_MESH_VERTICES,
            max_triangles: MAX_MESH_TRIANGLES,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct LocalExtractionReport {
    pub cell_queries: usize,
    pub callback_work: usize,
    pub arrangement_work: usize,
    pub feature_node_values: usize,
    pub peak_local_channels: usize,
    pub peak_shared_values: usize,
    pub shared_value_checks: usize,
    pub global_features: usize,
    pub proven_outside_cells: usize,
    pub coincident_vertices_merged: usize,
    pub collinear_faces_split: usize,
    pub exact_support_reductions: usize,
    pub convex_edges_split: usize,
    pub convex_aliases_merged: usize,
    /// Maximum displacement of an inserted existing f32 point from its parent
    /// edge interpolation, after any source-derived shared rounding choices.
    /// Includes the bounded original affine solver residual; no coordinate-
    /// proximity weld or enlarged per-axis rounding interval is used.
    pub convex_max_rounding_edge_deviation_m: f64,
    pub convex_rounded_vertices: usize,
    /// Conservative outward-rounded distance bound to original exact-coordinate
    /// intervals for vertices whose representation changed.
    pub convex_max_source_rounding_displacement_m: f64,
    pub convex_source_collinear_faces: usize,
    pub convex_exact_pool_corrections: usize,
    pub convex_exact_pool_binary_steps: usize,
    pub convex_merged_polygons: usize,
    pub convex_coalesced_interior_vertices: usize,
    pub convex_redundant_boundary_vertices: usize,
    pub representation: Option<ConvexRepresentationReport>,
}
impl LocalExtractionReport {
    pub fn charged_work(&self) -> usize {
        self.callback_work + self.arrangement_work
    }
}
#[derive(Clone, Debug)]
pub struct LocalExtraction {
    pub mesh: Mesh,
    pub report: LocalExtractionReport,
    /// Producing affine feature per emitted face, including repaired subdivisions.
    pub face_feature_ids: Vec<u64>,
    /// Affine support identities per final vertex; these are extraction provenance,
    /// not a claim that curved native features are already exactly zero here.
    pub vertex_support_ids: Vec<Vec<u64>>,
}

pub fn extract_local_boolean_isosurface(
    min: Vec3,
    max: Vec3,
    resolution: [u32; 3],
    options: LocalExtractionOptions,
    sample_cell: impl FnMut(&LocalCell, usize) -> Result<LocalCellData, String>,
) -> Result<LocalExtraction, String> {
    extract_local(min, max, resolution, options, sample_cell, false, None)
}

/// Extract an explicit union of convex signed-halfspace intersections. Every
/// cell uses the same facet-clipping strategy, including cells with few planes.
/// This avoids the general arrangement's 32-total-crossing-plane restriction;
/// each individual convex component still has at most 32 active halfspaces and
/// every cell retains the 128-channel and existing aggregate work limits.
/// At most 128 convex components are accepted, without distributing a union
/// through an intersection. Selective clipping is conformed using original
/// affine support. A shared vertex may use only componentwise floor/ceil f32
/// representations common to its source witnesses; single or atomic paired
/// choices must preserve every affected source-oriented facet and source-cell
/// bound. Native-field residual, orientation and convergence checks remain
/// required. A source that has no certified representation is rejected.
pub fn extract_local_convex_union_isosurface(
    min: Vec3,
    max: Vec3,
    resolution: [u32; 3],
    options: LocalExtractionOptions,
    sample_cell: impl FnMut(&LocalCell, usize) -> Result<LocalCellData, String>,
) -> Result<LocalExtraction, String> {
    extract_local(min, max, resolution, options, sample_cell, true, None)
}

/// Explicit bounded static representation correction. Positive source facets
/// may change only through certified simplicial edge contractions, with direct
/// cumulative source-to-stored error, source swept/endpoint intersection checks
/// and final f32 embedding validation. Native and texture gates remain required.
pub fn extract_local_convex_union_isosurface_with_representation(
    min: Vec3,
    max: Vec3,
    resolution: [u32; 3],
    options: LocalExtractionOptions,
    policy: ConvexRepresentationPolicy,
    sample_cell: impl FnMut(&LocalCell, usize) -> Result<LocalCellData, String>,
) -> Result<LocalExtraction, String> {
    if !policy.max_representation_error_m.is_finite() || policy.max_representation_error_m < 0. {
        return Err("convex representation requires a finite nonnegative error bound".into());
    }
    extract_local(min, max, resolution, options, sample_cell, true, Some(policy))
}

fn extract_local(
    min: Vec3,
    max: Vec3,
    resolution: [u32; 3],
    options: LocalExtractionOptions,
    mut sample_cell: impl FnMut(&LocalCell, usize) -> Result<LocalCellData, String>,
    convex: bool,
    representation: Option<ConvexRepresentationPolicy>,
) -> Result<LocalExtraction, String> {
    if options.max_work == 0 || options.max_vertices < 3 || options.max_triangles == 0 {
        return Err("local Boolean work/geometry budgets must be positive".into());
    }
    let lo = xyz(min);
    let hi = xyz(max);
    if lo.iter().chain(&hi).any(|x| !x.is_finite()) || (0..3).any(|i| lo[i] >= hi[i]) || resolution.contains(&0) {
        return Err("local Boolean bounds/resolution must be finite, increasing and nonzero".into());
    }
    let cells = product(resolution.map(|n| n as usize))?;
    let nodes = resolution.map(|n| (n as usize).saturating_add(1));
    if cells > MAX_GRID_CELLS || product(nodes)? > MAX_GRID_SAMPLES {
        return Err("local Boolean lattice exceeds existing grid limits".into());
    }
    let mut axes: [Vec<f32>; 3] = std::array::from_fn(|_| Vec::new());
    for i in 0..3 {
        axes[i] = (0..nodes[i])
            .map(|j| {
                (f64::from(lo[i]) + (f64::from(hi[i]) - f64::from(lo[i])) * j as f64 / f64::from(resolution[i])) as f32
            })
            .collect();
        if axes[i].windows(2).any(|v| v[0] >= v[1]) {
            return Err("local Boolean grid spacing is not representable as f32".into());
        }
    }
    let spacing_values: [f32; 3] =
        std::array::from_fn(|i| ((f64::from(hi[i]) - f64::from(lo[i])) / f64::from(resolution[i])) as f32);
    if spacing_values.iter().any(|x| !x.is_finite() || *x <= 0.0) {
        return Err("local Boolean spacing must be finite and positive".into());
    }
    let spacing = Vec3::new(spacing_values[0], spacing_values[1], spacing_values[2]);
    let mut work = WorkBudget {
        used: 0,
        structural: 0,
        extra_field_evaluations: 0,
        field_cost_per_callback: 1,
        limit: options.max_work.min(MAX_BOOLEAN_ARRANGEMENT_WORK),
        exact_support_reductions: 0,
    };
    let mut report = LocalExtractionReport::default();
    let mut output = Output::local(options.max_vertices, options.max_triangles);
    let mut shared = BTreeMap::<(u32, u64), u32>::new();
    let mut features = BTreeSet::new();
    let mut field_range = [f32::INFINITY, f32::NEG_INFINITY];
    let mut boundary = f32::INFINITY;
    let mut active_cells = 0;
    const TETS: [[usize; 4]; 6] = [[0, 1, 3, 7], [0, 3, 2, 7], [0, 2, 6, 7], [0, 6, 4, 7], [0, 4, 5, 7], [0, 5, 1, 7]];
    for z in 0..resolution[2] as usize {
        boolean::charge(&mut work, shared.len())?;
        let first = (z * nodes[0] * nodes[1]) as u32;
        shared.retain(|&(node, _), _| node >= first);
        for y in 0..resolution[1] as usize {
            for x in 0..resolution[0] as usize {
                boolean::charge(&mut work, 1)?;
                let node_ids = std::array::from_fn(|i| {
                    (((z + ((i >> 2) & 1)) * nodes[1] + y + ((i >> 1) & 1)) * nodes[0] + x + (i & 1)) as u32
                });
                let points = std::array::from_fn(|i| {
                    Vec3::new(axes[0][x + (i & 1)], axes[1][y + ((i >> 1) & 1)], axes[2][z + ((i >> 2) & 1)])
                });
                let cell = LocalCell { index: [x as u32, y as u32, z as u32], node_ids, points };
                let remaining = options.max_work.saturating_sub(report.callback_work + work.used);
                let data = sample_cell(&cell, remaining)?;
                if data.work > remaining {
                    return Err("local Boolean callback exceeded its supplied work budget".into());
                }
                report.callback_work += data.work;
                report.cell_queries += 1;
                work.limit = MAX_BOOLEAN_ARRANGEMENT_WORK.min(options.max_work - report.callback_work);
                if work.used > work.limit {
                    return Err("local Boolean aggregate work exhausted".into());
                }
                if data.features.len() > MAX_BOOLEAN_CHANNELS {
                    return Err("local Boolean cell exceeds 128 features".into());
                }
                if data.features.is_empty() {
                    if data.expression.is_some() {
                        return Err("empty local Boolean cell must have no expression".into());
                    }
                    report.proven_outside_cells += 1;
                    // Explicit Boolean outside constant, not a fabricated native distance.
                    field_range[0] = field_range[0].min(1.0);
                    field_range[1] = field_range[1].max(1.0);
                    if x == 0
                        || y == 0
                        || z == 0
                        || x + 1 == resolution[0] as usize
                        || y + 1 == resolution[1] as usize
                        || z + 1 == resolution[2] as usize
                    {
                        boundary = boundary.min(1.0);
                    }
                    continue;
                }
                let expr = data.expression.ok_or("nonempty local Boolean cell requires an expression")?;
                expr.validate(data.features.len())?;
                boolean::charge(&mut work, data.features.len() * 9)?;
                let mut ordered: Vec<_> = data.features.into_iter().enumerate().collect();
                ordered.sort_by_key(|(_, feature)| feature.id);
                if ordered.iter().any(|(_, f)| f.id == u64::MAX || f.values.iter().any(|v| !v.is_finite()))
                    || ordered.windows(2).any(|pair| pair[0].1.id == pair[1].1.id)
                {
                    return Err("local Boolean features need unique nonreserved IDs and finite values".into());
                }
                let mut remap = vec![0; ordered.len()];
                for (new, (old, _)) in ordered.iter().enumerate() {
                    remap[*old] = new;
                }
                let expr = remap_expression(&expr, &remap);
                let used = expr.validate(ordered.len())?;
                let convex_program = if convex { Some(boolean::ConvexUnion::parse(&expr, &mut work)?) } else { None };
                let ids: Vec<_> = ordered.iter().map(|(_, f)| f.id).collect();
                let values: Vec<_> = ordered.iter().map(|(_, f)| f.values).collect();
                report.peak_local_channels = report.peak_local_channels.max(ids.len());
                report.feature_node_values += 8 * ids.len();
                for (feature, values) in ids.iter().zip(&values) {
                    features.insert(*feature);
                    if features.len() > MAX_LOCAL_GLOBAL_FEATURES {
                        return Err("local Boolean global feature identity budget exceeded".into());
                    }
                    for (&node, &value) in node_ids.iter().zip(values) {
                        boolean::charge(&mut work, 1)?;
                        report.shared_value_checks += 1;
                        let bits = if value == 0.0 { 0 } else { value.to_bits() };
                        if let Some(old) = shared.get(&(node, *feature)) {
                            if *old != bits {
                                return Err(format!(
                                    "local Boolean feature {feature} changed value at shared node {node}"
                                ));
                            }
                        } else {
                            if shared.len() >= MAX_LOCAL_SHARED_VALUES {
                                return Err("local Boolean shared-node consistency cache exceeded its bound".into());
                            }
                            shared.insert((node, *feature), bits);
                        }
                    }
                }
                report.peak_shared_values = report.peak_shared_values.max(shared.len());
                boolean::charge(&mut work, 8 * expression_nodes(&expr))?;
                for corner in 0..8 {
                    let node_values: Vec<_> = values.iter().map(|v| v[corner]).collect();
                    let scalar = expr.scalar(&node_values);
                    field_range[0] = field_range[0].min(scalar);
                    field_range[1] = field_range[1].max(scalar);
                    let [px, py, pz] = [x + (corner & 1), y + ((corner >> 1) & 1), z + ((corner >> 2) & 1)];
                    if px == 0 || py == 0 || pz == 0 || px + 1 == nodes[0] || py + 1 == nodes[1] || pz + 1 == nodes[2] {
                        boundary = boundary.min(scalar);
                        if scalar <= 0.0 {
                            return Err("local Boolean isosurface touches or clips sampled bounds".into());
                        }
                    }
                }
                let mut active = Vec::new();
                let mut constant_inside = 0u128;
                for (channel, channel_values) in values.iter().enumerate() {
                    if used & (1u128 << channel) == 0 {
                        continue;
                    }
                    let min = channel_values.iter().copied().fold(f32::INFINITY, f32::min);
                    let max = channel_values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                    if max < 0.0 {
                        constant_inside |= 1u128 << channel;
                    } else if min <= 0.0 {
                        active.push(channel);
                    }
                }
                if active.is_empty() {
                    continue;
                }
                active_cells += 1;
                for indices in TETS {
                    boolean::charge(&mut work, active.len())?;
                    let mut planes = vec![[0.0; 4]; ids.len()];
                    let mut crossing = Vec::new();
                    let mut signs = constant_inside;
                    for &channel in &active {
                        let v = indices.map(|i| f64::from(values[channel][i]));
                        planes[channel] = v;
                        let min = v.iter().copied().fold(f64::INFINITY, f64::min);
                        let max = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        if min == 0.0 && max == 0.0 {
                            return Err("local Boolean channel vanishes throughout a tetrahedron".into());
                        }
                        if max < 0.0 {
                            signs |= 1u128 << channel;
                        }
                        if min < 0.0 && max >= 0.0 || min == 0.0 && max > 0.0 {
                            crossing.push(channel);
                        }
                    }
                    if !convex && crossing.len() > MAX_ACTIVE_PLANES_PER_TETRAHEDRON {
                        return Err("local Boolean tetrahedron exceeds 32 active planes".into());
                    }
                    let context = Context::new(
                        indices.map(|i| node_ids[i]),
                        indices.map(|i| d(points[i])),
                        &planes,
                        Some(&ids),
                        convex,
                    );
                    if let Some(program) = &convex_program {
                        boolean::emit_convex_tetra(context, signs, &crossing, program, &mut output, &mut work)?;
                    } else {
                        boolean::emit_tetra(context, signs, &crossing, &expr, &mut output, &mut work)?;
                    }
                }
            }
        }
    }
    output.clear_vertex_cache();
    if convex {
        let conformance = output.conform_convex_edges(representation, &mut work)?;
        report.convex_edges_split = conformance.splits;
        report.convex_aliases_merged = conformance.aliases;
        report.convex_max_rounding_edge_deviation_m = conformance.deviation;
        report.convex_rounded_vertices = conformance.rounded_vertices;
        report.convex_max_source_rounding_displacement_m = conformance.source_rounding_displacement;
        report.convex_source_collinear_faces = conformance.source_collinear_faces;
        report.convex_exact_pool_corrections = conformance.exact_pool_corrections;
        report.convex_exact_pool_binary_steps = conformance.exact_pool_binary_steps;
        report.convex_merged_polygons = conformance.merged_polygons;
        report.convex_coalesced_interior_vertices = conformance.coalesced_interior_vertices;
        report.convex_redundant_boundary_vertices = conformance.redundant_boundary_vertices;
        report.representation = conformance.representation;
    }
    let mut cleaned = clean_output(output, &options, &mut work)?;
    let metadata = MeshingMetadata {
        min,
        max,
        resolution,
        spacing,
        iso_level: 0.0,
        field_evaluations: report.cell_queries,
        active_cells,
        field_range,
        boundary_minimum: boundary,
        boundary_intersection: false,
        clipping_allowed: false,
        boundary_edges: 0,
        connected_components: 0,
        degenerate_triangles_removed: cleaned.collapsed + report.convex_source_collinear_faces,
        coincident_vertices_merged: 0,
        collinear_faces_split: 0,
        boolean_channels: report.peak_local_channels,
        boolean_arrangement_work: work.structural,
        boolean_extra_field_evaluations: 0,
        boolean_projected_vertices: 0,
        boolean_refinement_passes: 0,
        boolean_added_vertices: 0,
        boolean_added_triangles: 0,
        boolean_max_vertex_displacement: 0.0,
        surface_area: 0.0,
        signed_volume: 0.0,
    };
    boolean::charge(&mut work, cleaned.positions.len() + cleaned.triangles.len() * 9)?;
    let mut mesh = finish_mesh(
        std::mem::take(&mut cleaned.positions),
        std::mem::take(&mut cleaned.triangles),
        Vec::new(),
        metadata,
    )?;
    boolean::audit_vertex_links(&mesh, &mut work)?;
    if let Some(representation) = &mut report.representation {
        let matched = match representation.embedding_certificate.matches(
            &mesh.positions,
            &mesh.triangles,
            work.limit - work.used,
        ) {
            Ok(value) => {
                boolean::charge(&mut work, value.work)?;
                representation.embedding_certificate_binding_work += value.work;
                value.matches
            }
            Err(error) => {
                boolean::charge(&mut work, error.work)?;
                return Err(error.message);
            }
        };
        if !matched {
            match representation.embedding_certificate.reindexed(
                &mesh.positions,
                &mesh.triangles,
                &cleaned.input_to_compact,
                work.limit - work.used,
            ) {
                Ok(value) => {
                    boolean::charge(&mut work, value.work)?;
                    representation.embedding_certificate_binding_work += value.work;
                    representation.embedding_certificate = value.certificate;
                }
                Err(error) => {
                    boolean::charge(&mut work, error.work)?;
                    representation.embedding_certificate_binding_work += error.work;
                    if error.message.contains("budget") {
                        return Err(error.message);
                    }
                    // Cleanup did more than bijectively rename indices. Keep
                    // the genuine pre-clean proof; later exact matching must
                    // fail and the caller must perform normal validation.
                }
            }
        }
    }
    mesh.metadata.coincident_vertices_merged = cleaned.merged + report.convex_aliases_merged;
    mesh.metadata.collinear_faces_split = cleaned.split;
    mesh.metadata.boolean_arrangement_work = work.structural;
    report.arrangement_work = work.structural;
    report.global_features = features.len();
    report.coincident_vertices_merged = cleaned.merged + report.convex_aliases_merged;
    report.collinear_faces_split = cleaned.split;
    report.exact_support_reductions = work.exact_support_reductions;
    Ok(LocalExtraction { mesh, report, face_feature_ids: cleaned.faces, vertex_support_ids: cleaned.supports })
}

fn remap_expression(expr: &BooleanExpr, remap: &[usize]) -> BooleanExpr {
    match expr {
        BooleanExpr::Leaf(i) => BooleanExpr::Leaf(remap[*i]),
        BooleanExpr::Union(a, b) => {
            BooleanExpr::Union(Box::new(remap_expression(a, remap)), Box::new(remap_expression(b, remap)))
        }
        BooleanExpr::Intersection(a, b) => {
            BooleanExpr::Intersection(Box::new(remap_expression(a, remap)), Box::new(remap_expression(b, remap)))
        }
        BooleanExpr::Subtract(a, b) => {
            BooleanExpr::Subtract(Box::new(remap_expression(a, remap)), Box::new(remap_expression(b, remap)))
        }
    }
}
fn expression_nodes(expr: &BooleanExpr) -> usize {
    match expr {
        BooleanExpr::Leaf(_) => 1,
        BooleanExpr::Union(a, b) | BooleanExpr::Intersection(a, b) | BooleanExpr::Subtract(a, b) => {
            1 + expression_nodes(a) + expression_nodes(b)
        }
    }
}
fn product(values: [usize; 3]) -> Result<usize, String> {
    values
        .into_iter()
        .try_fold(1usize, |a, b| a.checked_mul(b))
        .ok_or_else(|| "local Boolean lattice size overflow".into())
}
fn xyz(p: Vec3) -> [f32; 3] {
    [p.x, p.y, p.z]
}
fn d(p: Vec3) -> [f64; 3] {
    [f64::from(p.x), f64::from(p.y), f64::from(p.z)]
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] - b[i])
}
fn cross([a, b, c]: [f64; 3], [x, y, z]: [f64; 3]) -> [f64; 3] {
    [b * z - c * y, c * x - a * z, a * y - b * x]
}
fn norm(a: [f64; 3]) -> f64 {
    a[0].hypot(a[1]).hypot(a[2])
}

struct Clean {
    positions: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    faces: Vec<u64>,
    supports: Vec<Vec<u64>>,
    collapsed: usize,
    merged: usize,
    split: usize,
    input_to_compact: Vec<u32>,
}
fn clean_output(mut output: Output, options: &LocalExtractionOptions, work: &mut WorkBudget) -> Result<Clean, String> {
    let positions = std::mem::take(&mut output.positions);
    let mut triangles = std::mem::take(&mut output.triangles);
    let collapsed = std::mem::take(&mut output.collapsed);
    let mut faces = output.face_features.take().ok_or("missing local face provenance")?;
    let mut supports = output.global_supports.take().ok_or("missing local vertex provenance")?;
    if faces.len() != triangles.len() || supports.len() != positions.len() {
        return Err("local Boolean provenance counts differ from geometry".into());
    }
    let mut support_count = supports.iter().map(Vec::len).sum::<usize>();
    if support_count > MAX_LOCAL_SUPPORT_VALUES {
        return Err("local Boolean global support storage budget exceeded".into());
    }
    boolean::charge(work, positions.len() + triangles.len() * 3)?;
    let mut parents: Vec<u32> = (0..positions.len() as u32).collect();
    let mut merged = 0;
    let mut split = 0;
    fn root(parents: &mut [u32], mut i: u32) -> u32 {
        let start = i;
        while parents[i as usize] != i {
            i = parents[i as usize];
        }
        let mut p = start;
        while parents[p as usize] != p {
            let next = parents[p as usize];
            parents[p as usize] = i;
            p = next;
        }
        i
    }
    for &[a, b, c] in &collapsed {
        for (a, b) in [(a, b), (b, c), (c, a)] {
            boolean::charge(work, 1)?;
            if positions[a as usize] == positions[b as usize] {
                let a = root(&mut parents, a);
                let b = root(&mut parents, b);
                if a != b {
                    let (lo, hi) = (a.min(b), a.max(b));
                    parents[hi as usize] = lo;
                    let extra = std::mem::take(&mut supports[hi as usize]);
                    support_count -= extra.len();
                    add_support(&mut supports[lo as usize], &extra, &mut support_count, work)?;
                    merged += 1;
                }
            }
        }
    }
    for triangle in &mut triangles {
        *triangle = triangle.map(|i| root(&mut parents, i));
    }
    let mut pending = Vec::new();
    let mut adjacency = BTreeMap::<(u32, u32), Vec<usize>>::new();
    for triangle in &collapsed {
        let [a, b, c] = triangle.map(|i| root(&mut parents, i));
        if a == b || b == c || c == a {
            continue;
        }
        let distance = |a: u32, b: u32| norm(sub(d(positions[a as usize]), d(positions[b as usize])));
        let (ab, bc, ca) = (distance(a, b), distance(b, c), distance(c, a));
        let (i, middle, j) = if ab >= bc && ab >= ca {
            (a, c, b)
        } else if bc >= ca {
            (b, a, c)
        } else {
            (c, b, a)
        };
        let key = (i.min(j), i.max(j));
        adjacency.entry(key).or_default();
        pending.push((key, middle));
    }
    for (face, &[a, b, c]) in triangles.iter().enumerate() {
        for (a, b) in [(a, b), (b, c), (c, a)] {
            if let Some(next) = adjacency.get_mut(&(a.min(b), a.max(b))) {
                next.push(face);
            }
        }
    }
    while !pending.is_empty() {
        let mut progress = false;
        let mut unresolved = Vec::new();
        for (key, middle) in pending {
            boolean::charge(work, 1)?;
            if adjacency[&key].len() != 1 {
                unresolved.push((key, middle));
                continue;
            }
            let index = adjacency[&key][0];
            let old = triangles[index];
            let edge = (0..3)
                .find(|&i| {
                    let (a, b) = (old[i], old[(i + 1) % 3]);
                    (a.min(b), a.max(b)) == key
                })
                .ok_or("local Boolean repair adjacency mismatch")?;
            let (a, b, c) = (old[edge], old[(edge + 1) % 3], old[(edge + 2) % 3]);
            let replacement = [[a, middle, c], [middle, b, c]];
            if replacement.iter().any(|triangle| {
                let [a, b, c] = triangle.map(|i| d(positions[i as usize]));
                norm(cross(sub(b, a), sub(c, a))) == 0.0
            }) {
                unresolved.push((key, middle));
                continue;
            }
            if triangles.len() >= options.max_triangles.min(MAX_MESH_TRIANGLES) {
                return Err("local Boolean repair exceeds triangle budget".into());
            }
            boolean::charge(work, 9)?;
            for (a, b) in [(old[0], old[1]), (old[1], old[2]), (old[2], old[0])] {
                if let Some(next) = adjacency.get_mut(&(a.min(b), a.max(b))) {
                    next.retain(|&face| face != index);
                }
            }
            triangles[index] = replacement[0];
            let appended = triangles.len();
            triangles.push(replacement[1]);
            faces.push(faces[index]);
            add_support(&mut supports[middle as usize], &[faces[index]], &mut support_count, work)?;
            for (face, [a, b, c]) in [(index, replacement[0]), (appended, replacement[1])] {
                for (a, b) in [(a, b), (b, c), (c, a)] {
                    if let Some(next) = adjacency.get_mut(&(a.min(b), a.max(b))) {
                        next.push(face);
                    }
                }
            }
            split += 1;
            progress = true;
        }
        if !progress {
            break;
        }
        pending = unresolved;
    }
    let repaired = repair_boundary_lines(
        &positions,
        &mut triangles,
        &mut faces,
        &mut supports,
        &mut support_count,
        options,
        work,
    )?;
    merged += repaired.0;
    split += repaired.1;
    let mut used = vec![false; positions.len()];
    for triangle in &triangles {
        for &i in triangle {
            used[i as usize] = true;
        }
    }
    let mut remap = vec![u32::MAX; positions.len()];
    let mut compact = Vec::new();
    let mut compact_supports = Vec::new();
    for (index, position) in positions.into_iter().enumerate() {
        if used[index] {
            remap[index] = compact.len() as u32;
            compact.push(position);
            compact_supports.push(std::mem::take(&mut supports[index]));
        }
    }
    for triangle in &mut triangles {
        *triangle = triangle.map(|i| remap[i as usize]);
    }
    Ok(Clean {
        positions: compact,
        triangles,
        faces,
        supports: compact_supports,
        collapsed: collapsed.len(),
        merged,
        split,
        input_to_compact: remap,
    })
}

type Edges = BTreeMap<(u32, u32), Vec<usize>>;
fn edges(triangles: &[[u32; 3]], work: &mut WorkBudget) -> Result<Edges, String> {
    let mut result = Edges::new();
    for (face, &[a, b, c]) in triangles.iter().enumerate() {
        boolean::charge(work, 3)?;
        for (a, b) in [(a, b), (b, c), (c, a)] {
            let entries = result.entry((a.min(b), a.max(b))).or_default();
            entries.push(face);
            if entries.len() > 2 {
                return Err("local Boolean boundary has a nonmanifold edge".into());
            }
        }
    }
    Ok(result)
}
fn add_support(
    support: &mut Vec<u64>,
    incoming: &[u64],
    total: &mut usize,
    work: &mut WorkBudget,
) -> Result<(), String> {
    boolean::charge(work, incoming.len())?;
    let additional = incoming.iter().filter(|id| !support.contains(id)).count();
    if additional > MAX_LOCAL_SUPPORT_VALUES.saturating_sub(*total) {
        return Err("local Boolean global support storage budget exceeded".into());
    }
    *total += additional;
    support.extend(incoming);
    support.sort_unstable();
    support.dedup();
    if support.len() > MAX_BOOLEAN_CHANNELS {
        return Err("local Boolean vertex exceeds 128 global support IDs".into());
    }
    Ok(())
}
/// Reconcile all existing middle vertices on one connected, exactly collinear
/// boundary at once. This handles multiple constraints whose original long edge
/// disappeared after the first split. No coordinate threshold, movement or cap
/// is introduced, and unrelated coincident components are not welded.
fn repair_boundary_lines(
    positions: &[Vec3],
    triangles: &mut Vec<[u32; 3]>,
    faces: &mut Vec<u64>,
    supports: &mut [Vec<u64>],
    support_count: &mut usize,
    options: &LocalExtractionOptions,
    work: &mut WorkBudget,
) -> Result<(usize, usize), String> {
    let edge_map = edges(triangles, work)?;
    let mut neighbors = BTreeMap::<u32, Vec<u32>>::new();
    for (&(a, b), incident) in &edge_map {
        if incident.len() == 1 {
            neighbors.entry(a).or_default().push(b);
            neighbors.entry(b).or_default().push(a);
        }
    }
    if neighbors.is_empty() {
        return Ok((0, 0));
    }
    let mut seen = BTreeSet::new();
    let mut aliases = BTreeMap::new();
    let mut replacements = BTreeMap::<usize, Vec<[u32; 3]>>::new();
    let mut split = 0;
    for &seed in neighbors.keys() {
        if seen.contains(&seed) {
            continue;
        }
        let mut pending = vec![seed];
        let mut component = Vec::new();
        while let Some(index) = pending.pop() {
            boolean::charge(work, 1)?;
            if seen.insert(index) {
                component.push(index);
                pending.extend(&neighbors[&index]);
            }
        }
        let origin = d(positions[seed as usize]);
        let far = *component
            .iter()
            .max_by(|&&a, &&b| {
                norm(sub(d(positions[a as usize]), origin)).total_cmp(&norm(sub(d(positions[b as usize]), origin)))
            })
            .ok_or("empty local Boolean boundary")?;
        let direction = sub(d(positions[far as usize]), origin);
        if norm(direction) == 0.0
            || component.iter().any(|&i| norm(cross(sub(d(positions[i as usize]), origin), direction)) != 0.0)
        {
            let evidence: Vec<_> = component.iter().take(8).map(|&i| (i, positions[i as usize])).collect();
            return Err(format!("local Boolean has a non-collinear open boundary; no geometry-changing repair is permitted: {evidence:?}"));
        }
        let axis = (0..3).max_by(|&a, &b| direction[a].abs().total_cmp(&direction[b].abs())).unwrap();
        component.sort_by(|&a, &b| {
            d(positions[a as usize])[axis].total_cmp(&d(positions[b as usize])[axis]).then(a.cmp(&b))
        });
        let mut ordered: Vec<u32> = Vec::new();
        for &index in &component {
            if let Some(&last) = ordered.last() {
                if positions[index as usize] == positions[last as usize] {
                    aliases.insert(index, last);
                    let incoming = std::mem::take(&mut supports[index as usize]);
                    *support_count -= incoming.len();
                    add_support(&mut supports[last as usize], &incoming, support_count, work)?;
                    continue;
                }
            }
            ordered.push(index);
        }
        for &i in &component {
            for &j in &neighbors[&i] {
                if i >= j {
                    continue;
                }
                boolean::charge(work, 1)?;
                let incident = &edge_map[&(i, j)];
                let face = incident[0];
                let original = triangles[face];
                let edge = (0..3)
                    .find(|&k| {
                        let (a, b) = (original[k], original[(k + 1) % 3]);
                        (a.min(b), a.max(b)) == (i, j)
                    })
                    .ok_or("missing local Boolean boundary incidence")?;
                let map = |i: u32| *aliases.get(&i).unwrap_or(&i);
                let (a, b, c) = (map(original[edge]), map(original[(edge + 1) % 3]), map(original[(edge + 2) % 3]));
                let (va, vb) = (d(positions[a as usize])[axis], d(positions[b as usize])[axis]);
                let mut chain: Vec<_> = ordered
                    .iter()
                    .copied()
                    .filter(|&i| {
                        let v = d(positions[i as usize])[axis];
                        v >= va.min(vb) && v <= va.max(vb)
                    })
                    .collect();
                if va > vb {
                    chain.reverse();
                }
                if chain.first() != Some(&a) || chain.last() != Some(&b) {
                    return Err("local Boolean boundary repair lost an endpoint".into());
                }
                if chain.len() > 2 {
                    if replacements.contains_key(&face) {
                        return Err("local Boolean boundary repair requires incompatible face subdivisions".into());
                    }
                    boolean::charge(work, chain.len())?;
                    for &vertex in &chain {
                        add_support(&mut supports[vertex as usize], &[faces[face]], support_count, work)?;
                    }
                    let pieces: Vec<_> = chain.windows(2).map(|pair| [pair[0], pair[1], c]).collect();
                    split += pieces.len() - 1;
                    replacements.insert(face, pieces);
                }
            }
        }
    }
    let mut result = Vec::new();
    let mut labels = Vec::new();
    for (face, triangle) in triangles.iter().copied().enumerate() {
        let pieces = replacements.remove(&face).unwrap_or_else(|| vec![triangle]);
        for triangle in pieces {
            boolean::charge(work, 3)?;
            let triangle = triangle.map(|i| *aliases.get(&i).unwrap_or(&i));
            let [a, b, c] = triangle.map(|i| d(positions[i as usize]));
            if norm(cross(sub(b, a), sub(c, a))) == 0.0 {
                return Err("local Boolean boundary reconciliation produced a degenerate face".into());
            }
            if result.len() >= options.max_triangles.min(MAX_MESH_TRIANGLES) {
                return Err("local Boolean boundary repair exceeds triangle budget".into());
            }
            result.push(triangle);
            labels.push(faces[face]);
        }
    }
    if edges(&result, work)?.values().any(|faces| faces.len() != 2) {
        return Err("local Boolean exact collinear repair did not restore closed incidence".into());
    }
    *triangles = result;
    *faces = labels;
    Ok((aliases.len(), split))
}

#[cfg(test)]
mod cleanup_tests {
    use super::*;
    fn work() -> WorkBudget {
        WorkBudget {
            used: 0,
            structural: 0,
            extra_field_evaluations: 0,
            field_cost_per_callback: 1,
            limit: 1_000_000,
            exact_support_reductions: 0,
        }
    }
    fn output(positions: Vec<Vec3>, triangles: Vec<[u32; 3]>, collapsed: Vec<[u32; 3]>) -> Output {
        let mut output = Output::local(100, 100);
        let faces: Vec<_> = (0..triangles.len()).map(|i| 1000 + i as u64).collect();
        let mut supports = vec![Vec::new(); positions.len()];
        for (triangle, &feature) in triangles.iter().zip(&faces) {
            for &vertex in triangle {
                supports[vertex as usize].push(feature);
            }
        }
        output.positions = positions;
        output.triangles = triangles;
        output.collapsed = collapsed;
        output.face_features = Some(faces);
        output.global_supports = Some(supports);
        output
    }
    fn options() -> LocalExtractionOptions {
        LocalExtractionOptions { max_work: 1_000_000, max_vertices: 100, max_triangles: 100 }
    }
    #[test]
    fn exact_alias_cleanup_unions_provenance_and_compacts_by_original_identity() {
        let mut source = output(
            vec![
                Vec3::ZERO,
                Vec3::new(1., 0., 0.),
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 0., 1.),
                Vec3::new(0.5, 0.5, 0.),
                Vec3::new(0.5, 0.5, 0.),
            ],
            vec![[0, 2, 5], [0, 4, 1], [1, 4, 3], [5, 2, 3], [0, 1, 3], [2, 0, 3]],
            vec![[0, 5, 4], [4, 5, 3]],
        );
        source.global_supports.as_mut().unwrap()[4].push(2000);
        source.global_supports.as_mut().unwrap()[5].push(3000);
        let cleaned = clean_output(source, &options(), &mut work()).unwrap();
        assert_eq!(cleaned.positions.len(), 5);
        assert_eq!(cleaned.triangles.len(), 6);
        assert_eq!(cleaned.merged, 1);
        assert_eq!(cleaned.faces, (1000..1006).collect::<Vec<_>>());
        assert!(cleaned.supports[4].contains(&2000) && cleaned.supports[4].contains(&3000));
        for (triangle, feature) in cleaned.triangles.iter().zip(&cleaned.faces) {
            for &vertex in triangle {
                assert!(cleaned.supports[vertex as usize].contains(feature));
            }
        }
    }
    #[test]
    fn multiple_collinear_constraints_preserve_face_labels_and_existing_points() {
        let positions = vec![
            Vec3::ZERO,
            Vec3::new(1., 0., 0.),
            Vec3::new(0.25, 0., 0.),
            Vec3::new(0.75, 0., 0.),
            Vec3::new(0., 1., 0.),
            Vec3::new(0., 0., 1.),
        ];
        let source = output(
            positions.clone(),
            vec![[0, 4, 2], [2, 4, 3], [3, 4, 1], [0, 1, 5], [1, 4, 5], [4, 0, 5]],
            vec![[0, 2, 1], [0, 3, 1]],
        );
        let cleaned = clean_output(source, &options(), &mut work()).unwrap();
        assert_eq!(cleaned.positions, positions);
        assert_eq!(cleaned.triangles.len(), 8);
        assert_eq!(cleaned.faces.iter().filter(|&&id| id == 1003).count(), 3);
        assert_eq!(cleaned.faces.len(), cleaned.triangles.len());
        for (triangle, feature) in cleaned.triangles.iter().zip(&cleaned.faces) {
            for &vertex in triangle {
                assert!(cleaned.supports[vertex as usize].contains(feature));
            }
        }
    }
}
