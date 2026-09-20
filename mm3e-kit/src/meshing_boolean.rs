//! Preserve constituent hard-Boolean boundaries inside sampled tetrahedra.
//!
//! Each leaf is interpolated affinely before Boolean classification. Leaf-zero
//! polygons are partitioned by the other active leaf planes, so a thin Boolean
//! wedge need not contain a negative sample of the already-combined scalar.
//! This is a bounded affine-plane arrangement, not dual contouring or an exact
//! representation of the original curved fields. Sampling/convergence remains
//! necessary. Singular arrangements and nonmanifold output are explicit errors.

use std::collections::HashMap;

use crate::{
    meshing::{
        finish_mesh, Mesh, MeshingMetadata, MAX_GRID_CELLS, MAX_GRID_SAMPLES, MAX_MESH_TRIANGLES, MAX_MESH_VERTICES,
    },
    Vec3,
};

#[path = "meshing_boolean_refine.rs"]
mod refine;
#[path = "meshing_convex_representation.rs"]
mod representation;

pub const MAX_BOOLEAN_CHANNELS: usize = 128;
pub const MAX_BOOLEAN_SAMPLE_VALUES: usize = 33_554_432;
pub const MAX_ACTIVE_PLANES_PER_TETRAHEDRON: usize = 32;
/// Legacy name retained: refined extraction applies this cap to structural
/// work PLUS additional callback evaluations weighted by the caller's cost.
pub const MAX_BOOLEAN_ARRANGEMENT_WORK: usize = 64_000_000;
pub const MAX_BOOLEAN_EXPRESSION_NODES: usize = 4095;

/// Post-grid work includes structural operations plus additional callback calls
/// multiplied by the caller's scalar-program cost. Affine wrappers keep source
/// projection disabled; delivery can explicitly enable the bounded refinement.
#[derive(Clone, Copy, Debug)]
pub struct BooleanExtractionOptions {
    pub max_postgrid_work: usize,
    pub field_cost_per_callback: usize,
    pub project_and_refine: bool,
    pub max_projection_iterations: u32,
    pub max_refinement_passes: u32,
    /// Source-normal verification distance, in world meters; f64 preserves the
    /// exact requested offset until rounding each probe at the native boundary.
    pub normal_probe_distance_m: f64,
    /// Allowed comparison difference in authored scalar units, not meters.
    pub normal_comparison_tolerance: f64,
}
impl Default for BooleanExtractionOptions {
    fn default() -> Self {
        Self {
            max_postgrid_work: MAX_BOOLEAN_ARRANGEMENT_WORK,
            field_cost_per_callback: 1,
            project_and_refine: false,
            max_projection_iterations: 8,
            max_refinement_passes: 3,
            normal_probe_distance_m: 0.001,
            normal_comparison_tolerance: 1e-5,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BooleanExpr {
    Leaf(usize),
    Union(Box<Self>, Box<Self>),
    Intersection(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
}

impl BooleanExpr {
    /// Checked scalar min/max evaluation, independent of extraction.
    pub fn evaluate(&self, channels: &[f32]) -> Result<f32, String> {
        self.validate(channels.len())?;
        if channels.iter().any(|v| !v.is_finite()) {
            return Err("Boolean channels must be finite".into());
        }
        Ok(self.scalar(channels))
    }

    pub(crate) fn validate(&self, count: usize) -> Result<u128, String> {
        fn visit(
            expr: &BooleanExpr,
            count: usize,
            depth: usize,
            nodes: &mut usize,
            used: &mut u128,
        ) -> Result<(), String> {
            *nodes += 1;
            if depth > 64 || *nodes > MAX_BOOLEAN_EXPRESSION_NODES {
                return Err("Boolean expression exceeds depth 64 or 4095 nodes".into());
            }
            match expr {
                BooleanExpr::Leaf(i) if *i < count && *i < MAX_BOOLEAN_CHANNELS => *used |= 1u128 << i,
                BooleanExpr::Leaf(_) => return Err("Boolean expression references an invalid channel".into()),
                BooleanExpr::Union(a, b) | BooleanExpr::Intersection(a, b) | BooleanExpr::Subtract(a, b) => {
                    visit(a, count, depth + 1, nodes, used)?;
                    visit(b, count, depth + 1, nodes, used)?;
                }
            }
            Ok(())
        }
        let mut used = 0;
        visit(self, count, 0, &mut 0, &mut used)?;
        Ok(used)
    }

    pub(crate) fn scalar(&self, channels: &[f32]) -> f32 {
        match self {
            Self::Leaf(i) => channels[*i],
            Self::Union(a, b) => a.scalar(channels).min(b.scalar(channels)),
            Self::Intersection(a, b) => a.scalar(channels).max(b.scalar(channels)),
            Self::Subtract(a, b) => a.scalar(channels).max(-b.scalar(channels)),
        }
    }

    fn scalar_with_work(&self, channels: &[f32], work: &mut WorkBudget) -> Result<f32, String> {
        charge(work, 1)?;
        Ok(match self {
            Self::Leaf(i) => channels[*i],
            Self::Union(a, b) => a.scalar_with_work(channels, work)?.min(b.scalar_with_work(channels, work)?),
            Self::Intersection(a, b) => a.scalar_with_work(channels, work)?.max(b.scalar_with_work(channels, work)?),
            Self::Subtract(a, b) => a.scalar_with_work(channels, work)?.max(-b.scalar_with_work(channels, work)?),
        })
    }

    fn inside(&self, signs: u128, work: &mut WorkBudget) -> Result<bool, String> {
        charge(work, 1)?;
        Ok(match self {
            Self::Leaf(i) => signs & (1u128 << i) != 0,
            Self::Union(a, b) => a.inside(signs, work)? || b.inside(signs, work)?,
            Self::Intersection(a, b) => a.inside(signs, work)? && b.inside(signs, work)?,
            Self::Subtract(a, b) => a.inside(signs, work)? && !b.inside(signs, work)?,
        })
    }
}

/// Sample every channel at every lattice point, then extract the composed zero
/// boundary. The callback must fill every entry with a finite negative-inside
/// scalar. Default clipping rejection and final topology checks are shared with
/// the ordinary scalar extractor. Metadata field_evaluations counts callbacks;
/// scalar storage/evaluation count is this value times channel_count.
pub fn extract_boolean_isosurface(
    channel_count: usize,
    sample: impl Fn(Vec3, &mut [f32]),
    expr: &BooleanExpr,
    min: Vec3,
    max: Vec3,
    resolution: [u32; 3],
) -> Result<Mesh, String> {
    extract_boolean_isosurface_with_work_limit(
        channel_count,
        sample,
        expr,
        min,
        max,
        resolution,
        MAX_BOOLEAN_ARRANGEMENT_WORK,
    )
}

/// As `extract_boolean_isosurface`, with a caller's remaining aggregate budget.
/// The effective arrangement limit never exceeds the module's hard work bound.
pub fn extract_boolean_isosurface_with_work_limit(
    channel_count: usize,
    sample: impl Fn(Vec3, &mut [f32]),
    expr: &BooleanExpr,
    min: Vec3,
    max: Vec3,
    resolution: [u32; 3],
    max_arrangement_work: usize,
) -> Result<Mesh, String> {
    extract_boolean_isosurface_with_options(
        channel_count,
        sample,
        expr,
        min,
        max,
        resolution,
        BooleanExtractionOptions { max_postgrid_work: max_arrangement_work, ..Default::default() },
    )
}

pub fn extract_boolean_isosurface_with_options(
    channel_count: usize,
    sample: impl Fn(Vec3, &mut [f32]),
    expr: &BooleanExpr,
    min: Vec3,
    max: Vec3,
    resolution: [u32; 3],
    options: BooleanExtractionOptions,
) -> Result<Mesh, String> {
    if options.max_postgrid_work == 0 || options.field_cost_per_callback == 0 {
        return Err("Boolean work limit and callback cost must be positive".into());
    }
    if !options.normal_probe_distance_m.is_finite()
        || options.normal_probe_distance_m <= 0.0
        || !options.normal_comparison_tolerance.is_finite()
        || options.normal_comparison_tolerance < 0.0
    {
        return Err(
            "Boolean normal probe distance must be positive/finite and comparison tolerance nonnegative/finite".into(),
        );
    }
    if !(1..=12).contains(&options.max_projection_iterations) || options.max_refinement_passes > 4 {
        return Err("Boolean projection/refinement iteration limits exceed bounded ranges".into());
    }
    if channel_count == 0 || channel_count > MAX_BOOLEAN_CHANNELS {
        return Err(format!("Boolean extraction requires 1..={MAX_BOOLEAN_CHANNELS} channels"));
    }
    let used = expr.validate(channel_count)?;
    let lo = coords(min);
    let hi = coords(max);
    if lo.iter().chain(&hi).any(|v| !v.is_finite()) || (0..3).any(|i| lo[i] >= hi[i]) || resolution.contains(&0) {
        return Err("Boolean bounds must be finite/increasing and cell resolutions nonzero".into());
    }
    let cells = resolution.iter().try_fold(1usize, |a, &b| a.checked_mul(b as usize)).ok_or("Boolean grid overflow")?;
    let nodes = resolution.map(|n| (n as usize).saturating_add(1));
    let samples = nodes.iter().try_fold(1usize, |a, &b| a.checked_mul(b)).ok_or("Boolean grid overflow")?;
    let scalar_count = samples.checked_mul(channel_count).ok_or("Boolean channel-grid size overflow")?;
    if cells > MAX_GRID_CELLS || samples > MAX_GRID_SAMPLES || scalar_count > MAX_BOOLEAN_SAMPLE_VALUES {
        return Err("Boolean grid/channel product exceeds bounded sampling storage/work".into());
    }
    let mut axes: [Vec<f32>; 3] = std::array::from_fn(|_| Vec::new());
    for i in 0..3 {
        axes[i] = (0..nodes[i])
            .map(|j| {
                (f64::from(lo[i]) + (f64::from(hi[i]) - f64::from(lo[i])) * j as f64 / f64::from(resolution[i])) as f32
            })
            .collect();
        if axes[i].windows(2).any(|w| w[0] >= w[1]) {
            return Err("Boolean grid spacing is not representable as f32".into());
        }
    }
    let lattice = Lattice { axes, nodes };
    let spacing = Vec3::new(
        ((f64::from(max.x) - f64::from(min.x)) / f64::from(resolution[0])) as f32,
        ((f64::from(max.y) - f64::from(min.y)) / f64::from(resolution[1])) as f32,
        ((f64::from(max.z) - f64::from(min.z)) / f64::from(resolution[2])) as f32,
    );
    if coords(spacing).iter().any(|v| !v.is_finite() || *v <= 0.0) {
        return Err("Boolean spacing must be finite and positive".into());
    }
    let mut values = vec![f32::NAN; scalar_count];
    let mut range = [f32::INFINITY, f32::NEG_INFINITY];
    let mut boundary = f32::INFINITY;
    for z in 0..nodes[2] {
        for y in 0..nodes[1] {
            for x in 0..nodes[0] {
                let index = lattice.index(x, y, z);
                let p = lattice.point(index);
                let channels = &mut values[index * channel_count..(index + 1) * channel_count];
                sample(p, channels);
                if channels.iter().any(|v| !v.is_finite()) {
                    return Err(format!("Boolean callback left a nonfinite channel at {p:?}"));
                }
                let scalar = expr.scalar(channels);
                range[0] = range[0].min(scalar);
                range[1] = range[1].max(scalar);
                if x == 0 || y == 0 || z == 0 || x + 1 == nodes[0] || y + 1 == nodes[1] || z + 1 == nodes[2] {
                    boundary = boundary.min(scalar);
                }
            }
        }
    }
    if boundary <= 0.0 {
        return Err("Boolean isosurface touches or clips sampled bounds; expand bounds".into());
    }
    let mut output = Output::default();
    let mut work = WorkBudget {
        used: 0,
        structural: 0,
        extra_field_evaluations: 0,
        field_cost_per_callback: options.field_cost_per_callback,
        limit: options.max_postgrid_work.min(MAX_BOOLEAN_ARRANGEMENT_WORK),
        exact_support_reductions: 0,
    };
    let mut active_cells = 0;
    const TETS: [[usize; 4]; 6] = [[0, 1, 3, 7], [0, 3, 2, 7], [0, 2, 6, 7], [0, 6, 4, 7], [0, 4, 5, 7], [0, 5, 1, 7]];
    let channels: Vec<_> = (0..channel_count).filter(|i| used & (1u128 << i) != 0).collect();
    for z in 0..nodes[2] - 1 {
        for y in 0..nodes[1] - 1 {
            for x in 0..nodes[0] - 1 {
                let cube = std::array::from_fn::<_, 8, _>(|i| {
                    lattice.index(x + (i & 1), y + ((i >> 1) & 1), z + (i >> 2)) as u32
                });
                let mut active = Vec::new();
                let mut constant_inside = 0u128;
                for &channel in &channels {
                    let v = cube.map(|id| values[id as usize * channel_count + channel]);
                    let smallest = v.iter().copied().fold(f32::INFINITY, f32::min);
                    let largest = v.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                    if largest < 0.0 {
                        constant_inside |= 1u128 << channel;
                    } else if smallest <= 0.0 {
                        active.push(channel);
                    }
                }
                if active.is_empty() {
                    continue;
                }
                active_cells += 1;
                for indices in TETS {
                    let tetra = cube_index(indices, cube);
                    let mut planes = vec![[0.0; 4]; channel_count];
                    let mut crossing = Vec::new();
                    let mut signs = constant_inside;
                    for &channel in &active {
                        let v = tetra.map(|id| f64::from(values[id as usize * channel_count + channel]));
                        planes[channel] = v;
                        let smallest = v.iter().copied().fold(f64::INFINITY, f64::min);
                        let largest = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        if smallest == 0.0 && largest == 0.0 {
                            return Err("Boolean channel vanishes throughout a tetrahedron; singular volume".into());
                        }
                        if largest < 0.0 {
                            signs |= 1u128 << channel;
                        }
                        if smallest < 0.0 && largest >= 0.0 || smallest == 0.0 && largest > 0.0 {
                            crossing.push(channel);
                        }
                    }
                    if crossing.len() > MAX_ACTIVE_PLANES_PER_TETRAHEDRON {
                        return Err("Boolean tetrahedron exceeds 32 active leaf planes".into());
                    }
                    let context =
                        Context::new(tetra, tetra.map(|id| doubles(lattice.point(id as usize))), &planes, None, false);
                    emit_tetra(context, signs, &crossing, expr, &mut output, &mut work)?;
                }
            }
        }
    }
    drop(std::mem::take(&mut output.vertices));
    let refined = if options.project_and_refine {
        refine::run(
            &mut output,
            &lattice,
            &values,
            channel_count,
            used,
            expr,
            &sample,
            &options,
            &mut work,
            min,
            max,
            spacing,
        )?
    } else {
        refine::Report::default()
    };
    let metadata = MeshingMetadata {
        min,
        max,
        resolution,
        spacing,
        iso_level: 0.0,
        field_evaluations: samples,
        active_cells,
        field_range: range,
        boundary_minimum: boundary,
        boundary_intersection: false,
        clipping_allowed: false,
        boundary_edges: 0,
        connected_components: 0,
        degenerate_triangles_removed: output.collapsed.len(),
        coincident_vertices_merged: 0,
        collinear_faces_split: 0,
        boolean_channels: channel_count,
        boolean_arrangement_work: work.structural,
        boolean_extra_field_evaluations: work.extra_field_evaluations,
        boolean_projected_vertices: refined.projected_vertices,
        boolean_refinement_passes: refined.passes,
        boolean_added_vertices: refined.added_vertices,
        boolean_added_triangles: refined.added_triangles,
        boolean_max_vertex_displacement: refined.maximum_displacement,
        surface_area: 0.0,
        signed_volume: 0.0,
    };
    let mut mesh = finish_mesh(output.positions, output.triangles, output.collapsed, metadata)?;
    audit_vertex_links(&mesh, &mut work)?;
    mesh.metadata.boolean_arrangement_work = work.structural;
    Ok(mesh)
}

/// Shared affine arrangement; channel masks remain local, weld IDs may be global.
pub(crate) fn emit_tetra(
    context: Context<'_>,
    signs: u128,
    crossing: &[usize],
    expr: &BooleanExpr,
    output: &mut Output,
    work: &mut WorkBudget,
) -> Result<(), String> {
    let planes = context.planes;
    for &candidate in crossing {
        let v = planes[candidate];
        if !v.iter().any(|&d| d < 0.0) || !v.iter().any(|&d| d >= 0.0) {
            continue;
        }
        let mut coincident = Vec::new();
        let mut owned = true;
        for &other in crossing {
            if other != candidate {
                if let Some(same_direction) = proportional(v, planes[other]) {
                    if other < candidate {
                        owned = false;
                        break;
                    }
                    coincident.push((other, same_direction));
                }
            }
        }
        if !owned {
            continue;
        }
        let gradient = context.gradient(candidate);
        let mut polygon = Vec::new();
        for [a, b] in [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]] {
            if (v[a] < 0.0) != (v[b] < 0.0) {
                let vertex = context.vertex((!((1 << a) | (1 << b))) & 15, 1u128 << candidate, work)?;
                if !polygon.iter().any(|p: &Vertex| p.key == vertex.key) {
                    polygon.push(vertex);
                }
            }
        }
        if polygon.len() < 3 {
            continue;
        }
        sort_polygon(&mut polygon, gradient);
        let mut pieces = vec![Piece { polygon, inside: signs }];
        for &other in crossing {
            if other == candidate || coincident.iter().any(|&(id, _)| id == other) {
                continue;
            }
            let mut next = Vec::new();
            for piece in pieces {
                let (inside, outside) = context.split(piece.polygon, other, work)?;
                if inside.len() >= 3 {
                    next.push(Piece { polygon: inside, inside: piece.inside | (1u128 << other) });
                }
                if outside.len() >= 3 {
                    next.push(Piece { polygon: outside, inside: piece.inside & !(1u128 << other) });
                }
            }
            pieces = next;
        }
        for piece in pieces {
            let mut minus = piece.inside | (1u128 << candidate);
            let mut plus = piece.inside & !(1u128 << candidate);
            for &(other, same) in &coincident {
                if same {
                    minus |= 1u128 << other;
                    plus &= !(1u128 << other);
                } else {
                    minus &= !(1u128 << other);
                    plus |= 1u128 << other;
                }
            }
            let a = expr.inside(minus, work)?;
            let b = expr.inside(plus, work)?;
            if a != b {
                output.polygon(piece.polygon, a, context.feature_id(candidate), None, work)?;
            }
        }
    }
    Ok(())
}

/// A syntactic DNF, without distributing intersections over unions. `false`
/// selects the positive halfspace, as in Subtract(inside, Leaf(channel)).
pub(crate) struct ConvexUnion {
    components: Vec<Vec<(usize, bool)>>,
}
impl ConvexUnion {
    pub(crate) fn parse(expr: &BooleanExpr, work: &mut WorkBudget) -> Result<Self, String> {
        fn component(expr: &BooleanExpr, out: &mut Vec<(usize, bool)>, work: &mut WorkBudget) -> Result<(), String> {
            charge(work, 1)?;
            match expr {
                BooleanExpr::Leaf(i) => out.push((*i, true)),
                BooleanExpr::Intersection(a, b) => {
                    component(a, out, work)?;
                    component(b, out, work)?;
                }
                BooleanExpr::Subtract(a, b) if matches!(**b, BooleanExpr::Leaf(_)) => {
                    component(a, out, work)?;
                    let BooleanExpr::Leaf(i) = **b else { unreachable!() };
                    out.push((i, false));
                }
                _ => return Err("convex local extraction requires a union of convex signed-leaf intersections".into()),
            }
            Ok(())
        }
        fn union(expr: &BooleanExpr, out: &mut Vec<Vec<(usize, bool)>>, work: &mut WorkBudget) -> Result<(), String> {
            charge(work, 1)?;
            if let BooleanExpr::Union(a, b) = expr {
                union(a, out, work)?;
                union(b, out, work)?;
            } else {
                if out.len() >= MAX_BOOLEAN_CHANNELS {
                    return Err("convex local extraction exceeds 128 components".into());
                }
                let mut planes = Vec::new();
                component(expr, &mut planes, work)?;
                planes.sort_unstable();
                planes.dedup();
                out.push(planes);
            }
            Ok(())
        }
        let mut components = Vec::new();
        union(expr, &mut components, work)?;
        components.sort();
        components.dedup();
        Ok(Self { components })
    }
}

/// Clip each convex facet once by its own component, then subtract the other
/// convex volumes. Outside fragments stop traversing that volume's planes.
/// All intersections use the same original-sample solver and global weld keys
/// as the general arrangement. No curved-field or topology substitution occurs.
pub(crate) fn emit_convex_tetra(
    context: Context<'_>,
    signs: u128,
    crossing: &[usize],
    program: &ConvexUnion,
    output: &mut Output,
    work: &mut WorkBudget,
) -> Result<(), String> {
    let mut representative: Vec<_> = (0..context.planes.len()).map(|i| (i, true)).collect();
    for (position, &channel) in crossing.iter().enumerate() {
        for &previous in &crossing[..position] {
            let relation = context.proportional_relation(channel, previous, work)?;
            if let Some(same) = relation {
                representative[channel] = (previous, same);
                break;
            }
        }
    }
    let crossing_mask = crossing.iter().fold(0u128, |mask, &i| mask | (1u128 << i));
    let mut components = Vec::new();
    for component in &program.components {
        let mut active = Vec::new();
        let mut empty = false;
        for &(channel, negative) in component {
            charge(work, 1)?;
            if crossing_mask & (1u128 << channel) == 0 {
                if (signs & (1u128 << channel) != 0) != negative {
                    empty = true;
                    break;
                }
            } else {
                let (canonical, same) = representative[channel];
                active.push((canonical, if same { negative } else { !negative }));
            }
        }
        if empty {
            continue;
        }
        active.sort_unstable();
        active.dedup();
        if active.windows(2).any(|pair| pair[0].0 == pair[1].0) {
            continue;
        }
        if active.len() > MAX_ACTIVE_PLANES_PER_TETRAHEDRON {
            return Err("convex local component exceeds 32 active halfspaces in one tetrahedron".into());
        }
        if active.is_empty() {
            // This component fills the complete tetrahedron; the union has no
            // internal boundary here. Adjacent tetrahedra own crossing facets.
            return Ok(());
        }
        components.push(active);
    }
    components.sort();
    components.dedup();
    for (owner, component) in components.iter().enumerate() {
        for &(candidate, negative) in component {
            charge(work, 6)?;
            let values = context.planes[candidate];
            let mut polygon = Vec::new();
            for [a, b] in [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]] {
                if (values[a] < 0.0) != (values[b] < 0.0) {
                    let vertex = context.vertex((!((1 << a) | (1 << b))) & 15, 1u128 << candidate, work)?;
                    if !polygon.iter().any(|p: &Vertex| p.key == vertex.key) {
                        polygon.push(vertex);
                    }
                }
            }
            if polygon.len() < 3 {
                continue;
            }
            sort_polygon(&mut polygon, context.gradient(candidate));
            for &(channel, inside_negative) in component {
                if channel == candidate {
                    continue;
                }
                let (inside, outside) = context.split(polygon, channel, work)?;
                polygon = if inside_negative { inside } else { outside };
                if polygon.len() < 3 {
                    break;
                }
            }
            if polygon.len() < 3 {
                continue;
            }
            let mut pieces = vec![polygon];
            for (other_owner, other) in components.iter().enumerate() {
                if other_owner == owner {
                    continue;
                }
                // Coplanar equal-facing boundary belongs to the first component.
                // Opposite-facing contact is inside the union and is removed.
                if other_owner > owner && other.contains(&(candidate, negative)) {
                    continue;
                }
                let mut next = Vec::new();
                for polygon in pieces {
                    // A separating plane proves the entire polygon disjoint.
                    // Avoid imprinting every unrelated component plane onto it.
                    let mut separate = false;
                    for &(channel, inside_negative) in other {
                        if channel == candidate {
                            continue;
                        }
                        charge(work, polygon.len())?;
                        let mut entirely_outside = true;
                        for vertex in &polygon {
                            let value = context.value(vertex, channel, work)?;
                            if !(if inside_negative { value > 0. } else { value < 0. }) {
                                entirely_outside = false;
                                break;
                            }
                        }
                        if entirely_outside {
                            separate = true;
                            break;
                        }
                    }
                    if separate {
                        next.push(polygon);
                        continue;
                    }
                    // Separation may require several halfspaces together. Do
                    // not extend their cuts into a facet when the complete
                    // convex intersection is empty.
                    let mut intersection = polygon.clone();
                    for &(channel, inside_negative) in other {
                        if channel == candidate {
                            continue;
                        }
                        let (minus, plus) = context.split(intersection, channel, work)?;
                        intersection = if inside_negative { minus } else { plus };
                        if intersection.len() < 3 {
                            break;
                        }
                    }
                    if intersection.len() < 3 {
                        next.push(polygon);
                        continue;
                    }
                    let mut remainder = polygon;
                    for &(channel, inside_negative) in other {
                        if channel == candidate {
                            continue;
                        }
                        let (minus, plus) = context.split(remainder, channel, work)?;
                        let (inside, outside) = if inside_negative { (minus, plus) } else { (plus, minus) };
                        if outside.len() >= 3 {
                            next.push(outside);
                        }
                        remainder = inside;
                        if remainder.len() < 3 {
                            break;
                        }
                    }
                    // The remainder lies in every halfspace of the other
                    // convex component, so it is an interior union face.
                }
                pieces = next;
                if pieces.is_empty() {
                    break;
                }
            }
            for polygon in pieces {
                output.polygon(
                    polygon,
                    negative,
                    context.feature_id(candidate),
                    Some(scale(context.gradient(candidate), if negative { 1. } else { -1. })),
                    work,
                )?;
            }
        }
    }
    Ok(())
}

/// Edge incidence alone cannot detect two closed sheets touching at one vertex.
/// A regular closed boundary has exactly one connected cycle in each vertex link.
pub(crate) fn audit_vertex_links(mesh: &Mesh, work: &mut WorkBudget) -> Result<(), String> {
    let mut links = vec![Vec::new(); mesh.positions.len()];
    for &[a, b, c] in &mesh.triangles {
        charge(work, 6)?;
        links[a as usize].push((b, c));
        links[b as usize].push((c, a));
        links[c as usize].push((a, b));
    }
    for link in links {
        let mut neighbors: HashMap<u32, Vec<u32>> = HashMap::new();
        for (a, b) in link {
            neighbors.entry(a).or_default().push(b);
            neighbors.entry(b).or_default().push(a);
        }
        if neighbors.values().any(|v| v.len() != 2) {
            return Err("Boolean boundary has a singular vertex link".into());
        }
        let Some((&start, first)) = neighbors.iter().next() else {
            return Err("Boolean boundary contains an unused vertex".into());
        };
        let mut previous = start;
        let mut current = first[0];
        let mut visited = 1;
        while current != start {
            charge(work, 1)?;
            visited += 1;
            if visited > neighbors.len() {
                return Err("Boolean boundary has an inconsistent vertex link".into());
            }
            let next = &neighbors[&current];
            let following = if next[0] == previous { next[1] } else { next[0] };
            previous = current;
            current = following;
        }
        if visited != neighbors.len() {
            return Err("Boolean boundary has a singular vertex contact between sheets".into());
        }
    }
    Ok(())
}

fn cube_index(indices: [usize; 4], cube: [u32; 8]) -> [u32; 4] {
    indices.map(|i| cube[i])
}

struct Lattice {
    axes: [Vec<f32>; 3],
    nodes: [usize; 3],
}
impl Lattice {
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        (z * self.nodes[1] + y) * self.nodes[0] + x
    }
    fn point(&self, id: usize) -> Vec3 {
        Vec3::new(
            self.axes[0][id % self.nodes[0]],
            self.axes[1][(id / self.nodes[0]) % self.nodes[1]],
            self.axes[2][id / (self.nodes[0] * self.nodes[1])],
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Key {
    nodes: [u32; 4],
    planes: [u64; 3],
}
#[derive(Clone)]
struct Vertex {
    key: Key,
    weights: [f64; 4],
    point: [f64; 3],
    faces: u8,
    planes: u128,
    source: SourcePoint,
    global_planes: Option<Vec<u64>>,
}
#[derive(Clone)]
struct SourcePoint {
    point: [f64; 3],
    nodes: [u32; 4],
    weights: [f64; 4],
    affine: Option<std::sync::Arc<AffineSource>>,
}
struct AffineSource {
    exact_basis: std::sync::OnceLock<ExactBasisProof>,
    tetra: [u32; 4],
    points: [[f64; 3]; 4],
    basis: [u64; 3],
    planes: Vec<(u64, [f64; 4])>,
}
struct Piece {
    polygon: Vec<Vertex>,
    inside: u128,
}
// A full cache affects only reuse: uncached exact predicates remain available.
const MAX_CACHED_PLANE_RELATIONS: usize = 512;

pub(crate) struct Context<'a> {
    tetra: [u32; 4],
    points: [[f64; 3]; 4],
    planes: &'a [[f64; 4]],
    feature_ids: Option<&'a [u64]>,
    retain_affine: bool,
    vertex_cache: std::cell::RefCell<std::collections::BTreeMap<(u8, u128), (Vertex, usize)>>,
    proportional_cache: std::cell::RefCell<std::collections::HashMap<(u8, usize, usize), Option<bool>>>,
}

impl<'a> Context<'a> {
    pub(crate) fn new(
        tetra: [u32; 4],
        points: [[f64; 3]; 4],
        planes: &'a [[f64; 4]],
        feature_ids: Option<&'a [u64]>,
        retain_affine: bool,
    ) -> Self {
        Self {
            tetra,
            points,
            planes,
            feature_ids,
            retain_affine,
            vertex_cache: Default::default(),
            proportional_cache: Default::default(),
        }
    }
    fn proportional_relation(
        &self,
        channel: usize,
        other: usize,
        work: &mut WorkBudget,
    ) -> Result<Option<bool>, String> {
        self.support_relation(0, channel, other, work)
    }

    /// Only original immutable plane rows and the support mask determine this
    /// relation. A negative full-tetra result cannot certify a face/edge result.
    /// Products of two finite f32 samples are exact and finite in f64.
    fn support_relation(
        &self,
        faces: u8,
        channel: usize,
        other: usize,
        work: &mut WorkBudget,
    ) -> Result<Option<bool>, String> {
        // Two channel-order operations and one fixed-size hash lookup. Charge
        // before probing, including on misses and before any cache mutation.
        charge(work, 3)?;
        let key = (faces, channel.min(other), channel.max(other));
        let cached = { self.proportional_cache.borrow().get(&key).copied() };
        if let Some(relation) = cached {
            return Ok(relation);
        }
        // Four support-mask checks plus bounded stack-array construction.
        charge(work, 8)?;
        let mut local = [0; 4];
        let mut count = 0;
        for i in 0..4 {
            if faces & (1 << i) == 0 {
                local[count] = i;
                count += 1;
            }
        }
        let local = &local[..count];
        let a = self.planes[key.1];
        let b = self.planes[key.2];
        // Pivot absolute values/comparisons and the two nonzero checks.
        charge(work, count * 3 + 2)?;
        let pivot = local.iter().copied().max_by(|&i, &j| a[i].abs().total_cmp(&a[j].abs()));
        let relation = if let Some(pivot) = pivot.filter(|&i| a[i] != 0.0 && b[i] != 0.0) {
            // Two products and an equality per coordinate, then a sign test.
            // Precharge the bounded loop even when an early mismatch exits it.
            charge(work, count * 3 + 1)?;
            local
                .iter()
                .all(|&i| a[i] * b[pivot] == b[i] * a[pivot])
                .then_some(a[pivot].is_sign_positive() == b[pivot].is_sign_positive())
        } else {
            None
        };
        charge(work, 1)?;
        let mut cache = self.proportional_cache.borrow_mut();
        if cache.len() < MAX_CACHED_PLANE_RELATIONS {
            // Account for storing the fixed-size key and its optional result.
            charge(work, 2)?;
            cache.insert(key, relation);
        }
        Ok(relation)
    }

    fn feature_id(&self, channel: usize) -> u64 {
        self.feature_ids.map_or(channel as u64, |ids| ids[channel])
    }
    fn gradient(&self, channel: usize) -> [f64; 3] {
        let [a, b, c] = [1, 2, 3].map(|i| sub(self.points[i], self.points[0]));
        let v = self.planes[channel];
        let det = dot(a, cross(b, c));
        add(
            add(scale(cross(b, c), (v[1] - v[0]) / det), scale(cross(c, a), (v[2] - v[0]) / det)),
            scale(cross(a, b), (v[3] - v[0]) / det),
        )
    }

    fn vertex(&self, faces: u8, planes: u128, work: &mut WorkBudget) -> Result<Vertex, String> {
        // Rank-two edge intersections are frequent and have no repeated
        // higher-dimensional solve. Cache only rank-three/four source
        // vertices, where a tetrahedron revisits the same support key while
        // splitting several convex components. The cached result is an exact
        // clone of the complete uncached vertex, including source ancestry.
        if self.retain_affine && faces.count_ones() < 2 {
            let key = (faces, planes);
            let cache_depth = usize::BITS as usize - self.vertex_cache.borrow().len().max(1).leading_zeros() as usize;
            charge(work, cache_depth + 1)?;
            let cached = { self.vertex_cache.borrow().get(&key).cloned() };
            if let Some((value, reductions)) = cached {
                charge(work, 6 + 2 * value.global_planes.as_ref().map_or(0, Vec::len))?;
                work.exact_support_reductions += reductions;
                return Ok(value);
            }
            let before = work.exact_support_reductions;
            let value = self.vertex_uncached(faces, planes, work)?;
            let reductions = work.exact_support_reductions - before;
            let mut cache = self.vertex_cache.borrow_mut();
            if cache.len() < 32 {
                let depth = usize::BITS as usize - cache.len().max(1).leading_zeros() as usize;
                charge(work, depth + 2 + 6 + 2 * value.global_planes.as_ref().map_or(0, Vec::len))?;
                cache.insert(key, (value.clone(), reductions));
            }
            return Ok(value);
        }
        self.vertex_uncached(faces, planes, work)
    }

    fn vertex_uncached(&self, faces: u8, planes: u128, work: &mut WorkBudget) -> Result<Vertex, String> {
        charge(work, 1)?;
        let mut local: Vec<_> = (0..4).filter(|i| faces & (1 << i) == 0).collect();
        local.sort_by_key(|&i| self.tetra[i]);
        let cuts: Vec<_> = (0..self.planes.len()).filter(|i| planes & (1u128 << i) != 0).collect();
        let dimension = local.len();
        if dimension == 0 || cuts.len() + 1 < dimension {
            return Err("Boolean vertex has insufficient support constraints".into());
        }
        // A nonzero affine edge equation with one exact zero endpoint has
        // its unique root there. The generic solve/cofactor path reaches this
        // same rank-one recursion; keep every other plane constraint so that
        // its existing residual checks remain mandatory.
        if dimension == 2 && self.retain_affine && self.feature_ids.is_some() {
            for &cut in &cuts {
                charge(work, 3)?;
                let [a, b] = [self.planes[cut][local[0]], self.planes[cut][local[1]]];
                if a == 0. && b == 0. {
                    continue;
                }
                if a == 0. {
                    return self.vertex(faces | (1 << local[1]), planes, work);
                }
                if b == 0. {
                    return self.vertex(faces | (1 << local[0]), planes, work);
                }
                break;
            }
        }
        let mut basis = Vec::new();
        let mut solution = None;
        let edge_fast = dimension == 2 && self.retain_affine && self.feature_ids.is_some();
        if edge_fast {
            for &cut in &cuts {
                // Fixed storage removes allocation, but this still performs
                // the same two-dimensional Gaussian arithmetic as the solver.
                charge(work, 8)?;
                if let Some(weights) = solve_edge_weights(&local, cut, self.planes) {
                    basis = vec![cut];
                    solution = Some(weights);
                    break;
                }
            }
        }
        for a in 0..cuts.len().max(1) {
            if edge_fast {
                break;
            }
            for b in (a + 1)..=cuts.len().max(a + 1) {
                for c in (b + 1)..=cuts.len().max(b + 1) {
                    let selected = match dimension {
                        1 => vec![],
                        2 => vec![cuts[a]],
                        3 if b < cuts.len() => vec![cuts[a], cuts[b]],
                        4 if c < cuts.len() => vec![cuts[a], cuts[b], cuts[c]],
                        _ => continue,
                    };
                    charge(work, dimension.pow(3))?;
                    if let Some(weights) = solve_weights(&local, &selected, self.planes) {
                        basis = selected;
                        solution = Some(weights);
                        break;
                    }
                    if dimension <= 2 {
                        break;
                    }
                }
                if solution.is_some() || dimension <= 2 {
                    break;
                }
            }
            if solution.is_some() {
                break;
            }
        }
        let mut solved = solution.ok_or_else(|| {
            if self.retain_affine {
                format!("Boolean plane arrangement has singular intersection constraints; tetra {:?}, points {:?}, local {:?}, cuts {:?}",self.tetra,self.points,local,cuts.iter().map(|&i|(self.feature_id(i),self.planes[i])).collect::<Vec<_>>())
            }else{"Boolean plane arrangement has singular intersection constraints".into()}
        })?;
        if solved[..dimension].iter().any(|&v| !(-1e-10..=1.0000000001).contains(&v)) {
            return Err("Boolean plane intersection lies outside its support simplex".into());
        }
        if self.feature_ids.is_some() && solved[..dimension].iter().any(|value| value.abs() <= 1e-10) {
            // Precharge bounded expansion construction before its arithmetic.
            charge(
                work,
                match dimension {
                    1 => 1,
                    2 => 8,
                    3 => 64,
                    _ => 2048,
                },
            )?;
            let exact = exact_weights(&local, &basis, self.planes)?;
            for i in 0..dimension {
                if solved[i].abs() <= 1e-10 && (exact[i] == 0.0 || solved[i] == 0.0) {
                    if exact[i] == 0.0 && solved[i] != 0.0 {
                        work.exact_support_reductions += 1;
                    }
                    solved[i] = exact[i];
                }
            }
        }
        let mut weights = [0.0; 4];
        for (i, &local_index) in local.iter().enumerate() {
            weights[local_index] = solved[i];
        }
        for &cut in &cuts {
            let residual = (0..4).map(|i| weights[i] * self.planes[cut][i]).sum::<f64>().abs();
            let scale = local.iter().map(|&i| self.planes[cut][i].abs()).fold(0.0, f64::max);
            if residual > scale * 1e-10 {
                return Err("Boolean intersection has inconsistent supporting planes".into());
            }
        }
        let reduced_faces = (0..4).filter(|&i| weights[i] == 0.0).fold(0u8, |bits, i| bits | (1 << i));
        if reduced_faces != faces {
            return self.vertex(reduced_faces, planes, work);
        }
        let mut key = Key { nodes: [u32::MAX; 4], planes: [u64::MAX; 3] };
        for (i, &index) in local.iter().enumerate() {
            key.nodes[i] = self.tetra[index];
        }
        for (i, &plane) in basis.iter().enumerate() {
            key.planes[i] = self.feature_id(plane);
        }
        let mut point = [0.0; 3];
        for (i, &weight) in weights.iter().enumerate() {
            point = add(point, scale(self.points[i], weight));
        }
        Ok(Vertex {
            key,
            weights,
            point,
            faces,
            planes,
            source: SourcePoint {
                point,
                nodes: key.nodes,
                weights: solved,
                affine: if self.retain_affine {
                    charge(work, cuts.len() * 4)?;
                    Some(std::sync::Arc::new(AffineSource {
                        exact_basis: Default::default(),
                        tetra: self.tetra,
                        points: self.points,
                        basis: key.planes,
                        planes: cuts.iter().map(|&i| (self.feature_id(i), self.planes[i])).collect(),
                    }))
                } else {
                    None
                },
            },
            global_planes: self.feature_ids.map(|ids| cuts.iter().map(|&i| ids[i]).collect()),
        })
    }

    fn value(&self, vertex: &Vertex, channel: usize, work: &mut WorkBudget) -> Result<f64, String> {
        if vertex.planes & (1u128 << channel) != 0 {
            Ok(0.0)
        } else {
            if self.retain_affine {
                charge(work, 8)?;
                let local: Vec<_> = (0..4).filter(|i| vertex.faces & (1 << i) == 0).collect();
                let target = self.planes[channel];
                charge(work, local.len())?;
                if local.iter().all(|&i| target[i] == 0.) {
                    return Ok(0.);
                }
                for other in 0..self.planes.len() {
                    charge(work, 1)?;
                    if vertex.planes & (1u128 << other) == 0 {
                        continue;
                    }
                    let full = self.proportional_relation(channel, other, work)?;
                    if full.is_some()
                        || vertex.faces != 0 && self.support_relation(vertex.faces, channel, other, work)?.is_some()
                    {
                        return Ok(0.);
                    }
                }
                // Dot products, magnitude scan, and residual-threshold test.
                charge(work, 20)?;
                let approximate = (0..4).map(|i| vertex.weights[i] * target[i]).sum::<f64>();
                let scale = target.iter().map(|v| v.abs()).fold(0., f64::max);
                if approximate.abs() <= scale * 1e-10 {
                    // The residual threshold selects an exact predicate; it
                    // never classifies a nonzero quantity as zero. Cofactors
                    // use original f32 rows and an error-free fourth product.
                    charge(
                        work,
                        match local.len() {
                            1 => 1,
                            2 => 16,
                            3 => 256,
                            _ => 4096,
                        },
                    )?;
                    let mut local = local;
                    local.sort_by_key(|&i| self.tetra[i]);
                    let basis: Vec<_> = vertex
                        .key
                        .planes
                        .iter()
                        .filter(|&&id| id != u64::MAX)
                        .map(|id| {
                            self.feature_ids
                                .unwrap()
                                .iter()
                                .position(|other| id == other)
                                .ok_or("convex sign lost source basis")
                        })
                        .collect::<Result<_, _>>()?;
                    return exact_plane_sign(&local, &basis, self.planes, target).map(f64::from);
                }
                return Ok(approximate);
            }
            Ok((0..4).map(|i| vertex.weights[i] * self.planes[channel][i]).sum())
        }
    }

    fn split(
        &self,
        polygon: Vec<Vertex>,
        channel: usize,
        work: &mut WorkBudget,
    ) -> Result<(Vec<Vertex>, Vec<Vertex>), String> {
        charge(work, polygon.len())?;
        let distances: Vec<_> = polygon.iter().map(|v| self.value(v, channel, work)).collect::<Result<_, _>>()?;
        let mut inside = Vec::new();
        let mut outside = Vec::new();
        for i in 0..polygon.len() {
            let j = (i + 1) % polygon.len();
            let a = &polygon[i];
            let b = &polygon[j];
            let da = distances[i];
            let db = distances[j];
            if da == 0.0 {
                let vertex = self.vertex(a.faces, a.planes | (1u128 << channel), work)?;
                push_unique(&mut inside, vertex.clone());
                push_unique(&mut outside, vertex);
            } else if da < 0.0 {
                push_unique(&mut inside, a.clone());
            } else {
                push_unique(&mut outside, a.clone());
            }
            if da < 0.0 && db > 0.0 || da > 0.0 && db < 0.0 {
                let vertex = self.vertex(a.faces & b.faces, (a.planes & b.planes) | (1u128 << channel), work)?;
                push_unique(&mut inside, vertex.clone());
                push_unique(&mut outside, vertex);
            }
        }
        remove_closing_duplicate(&mut inside);
        remove_closing_duplicate(&mut outside);
        Ok((inside, outside))
    }
}

fn solve_weights(local: &[usize], cuts: &[usize], values: &[[f64; 4]]) -> Option<[f64; 4]> {
    let size = local.len();
    let mut matrix = [[0.0; 5]; 4];
    matrix[0][..size].fill(1.0);
    matrix[0][size] = 1.0;
    for (i, &cut) in cuts.iter().enumerate() {
        let reference = local.iter().map(|&j| values[cut][j].abs()).fold(0.0, f64::max);
        if reference == 0.0 {
            return None;
        }
        for (j, &node) in local.iter().enumerate() {
            matrix[i + 1][j] = values[cut][node] / reference;
        }
    }
    for column in 0..size {
        let pivot = (column..size).max_by(|&a, &b| matrix[a][column].abs().total_cmp(&matrix[b][column].abs()))?;
        if matrix[pivot][column].abs() < 1e-12 {
            return None;
        }
        matrix.swap(column, pivot);
        let divisor = matrix[column][column];
        for value in &mut matrix[column][column..=size] {
            *value /= divisor;
        }
        let pivot_row = matrix[column];
        for (row, values) in matrix.iter_mut().enumerate().take(size) {
            if row != column {
                let factor = values[column];
                for (value, &reference) in values[column..=size].iter_mut().zip(&pivot_row[column..=size]) {
                    *value -= factor * reference;
                }
            }
        }
    }
    Some(std::array::from_fn(|i| if i < size { matrix[i][size] } else { 0.0 }))
}

fn solve_edge_weights(local: &[usize], cut: usize, values: &[[f64; 4]]) -> Option<[f64; 4]> {
    debug_assert_eq!(local.len(), 2);
    let mut matrix = [[0.0; 5]; 4];
    matrix[0][..2].fill(1.0);
    matrix[0][2] = 1.0;
    let reference = local.iter().map(|&j| values[cut][j].abs()).fold(0.0, f64::max);
    if reference == 0.0 {
        return None;
    }
    for (j, &node) in local.iter().enumerate() {
        matrix[1][j] = values[cut][node] / reference;
    }
    for column in 0..2 {
        let pivot = (column..2).max_by(|&a, &b| matrix[a][column].abs().total_cmp(&matrix[b][column].abs()))?;
        if matrix[pivot][column].abs() < 1e-12 {
            return None;
        }
        matrix.swap(column, pivot);
        let divisor = matrix[column][column];
        for value in &mut matrix[column][column..=2] {
            *value /= divisor;
        }
        let pivot_row = matrix[column];
        for (row, values) in matrix.iter_mut().enumerate().take(2) {
            if row != column {
                let factor = values[column];
                for (value, &reference) in values[column..=2].iter_mut().zip(&pivot_row[column..=2]) {
                    *value -= factor * reference;
                }
            }
        }
    }
    Some(std::array::from_fn(|i| if i < 2 { matrix[i][2] } else { 0.0 }))
}

fn proportional(a: [f64; 4], b: [f64; 4]) -> Option<bool> {
    let pivot = (0..4).max_by(|&i, &j| a[i].abs().total_cmp(&a[j].abs()))?;
    if a[pivot] == 0.0 || b[pivot] == 0.0 {
        return None;
    }
    (0..4)
        .all(|i| a[i] * b[pivot] == b[i] * a[pivot])
        .then_some(a[pivot].is_sign_positive() == b[pivot].is_sign_positive())
}

fn sort_polygon(polygon: &mut [Vertex], normal: [f64; 3]) {
    let normal = scale(normal, 1.0 / dot(normal, normal).sqrt());
    let axis = if normal[0].abs() < 0.8 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let u = cross(normal, axis);
    let u = scale(u, 1.0 / dot(u, u).sqrt());
    let v = cross(normal, u);
    let center = polygon.iter().fold([0.0; 3], |sum, p| add(sum, scale(p.point, 1.0 / polygon.len() as f64)));
    polygon.sort_by(|a, b| {
        let angle = |p: &Vertex| {
            let d = sub(p.point, center);
            dot(d, v).atan2(dot(d, u))
        };
        angle(a).total_cmp(&angle(b)).then_with(|| a.key.cmp(&b.key))
    });
}

fn push_unique(polygon: &mut Vec<Vertex>, vertex: Vertex) {
    if polygon.last().is_none_or(|p| p.key != vertex.key) {
        polygon.push(vertex);
    }
}
fn remove_closing_duplicate(polygon: &mut Vec<Vertex>) {
    if polygon.len() > 1 && polygon.first().unwrap().key == polygon.last().unwrap().key {
        polygon.pop();
    }
}

#[derive(Default)]
pub(crate) struct Output {
    pub(crate) positions: Vec<Vec3>,
    pub(crate) triangles: Vec<[u32; 3]>,
    pub(crate) collapsed: Vec<[u32; 3]>,
    pub(crate) face_features: Option<Vec<u64>>,
    pub(crate) global_supports: Option<Vec<Vec<u64>>>,
    face_source_normals: Vec<[f64; 3]>,
    convex_source_collinear: usize,
    convex_polygons: Vec<ConvexPolygon>,
    limits: Option<[usize; 2]>,
    support_values: usize,
    vertices: HashMap<Key, u32>,
    sources: Vec<SourcePoint>,
    constraints: Vec<u128>,
}
struct ConvexPolygon {
    vertices: Vec<u32>,
    feature: u64,
    normal: [f64; 3],
    plane: CellPlane,
}
struct CellPlane {
    cell: [u32; 2],
    bounds: [[f64; 3]; 2],
    coefficients: [Vec<f64>; 4],
    negative: bool,
    boundary_axis: Option<(usize, f64, i8)>,
}
impl CellPlane {
    fn outward_normal(&self) -> [f64; 3] {
        let sign = if self.negative { 1. } else { -1. };
        let normal = std::array::from_fn(|axis| {
            sign * self.coefficients[axis + 1].iter().sum::<f64>() / (self.bounds[1][axis] - self.bounds[0][axis])
        });
        let magnitude = dot(normal, normal).sqrt();
        scale(normal, 1. / magnitude)
    }
    fn new(source: &AffineSource, feature: u64, negative: bool, work: &mut WorkBudget) -> Result<Self, String> {
        charge(work, 64)?;
        let values =
            source.planes.iter().find(|(id, _)| *id == feature).ok_or("convex polygon lost its source plane")?.1;
        let bounds = [
            std::array::from_fn(|axis| source.points.iter().map(|p| p[axis]).fold(f64::INFINITY, f64::min)),
            std::array::from_fn(|axis| source.points.iter().map(|p| p[axis]).fold(f64::NEG_INFINITY, f64::max)),
        ];
        let mut corners = Vec::new();
        for (i, point) in source.points.iter().enumerate() {
            let mut bits = 0u8;
            for axis in 0..3 {
                if point[axis] == bounds[1][axis] {
                    bits |= 1 << axis;
                } else if point[axis] != bounds[0][axis] {
                    return Err("convex plane requires original axis-aligned cell corners".into());
                }
            }
            corners.push((bits, i));
        }
        corners.sort_by_key(|&(bits, _)| bits.count_ones());
        if corners[0].0 != 0 || corners[3].0 != 7 {
            return Err("convex plane lost its complete cell diagonal".into());
        }
        let mut coefficients: [Vec<f64>; 4] = std::array::from_fn(|_| Vec::new());
        coefficients[0] = grow_expansion(&[], values[corners[0].1]);
        for pair in corners.windows(2) {
            let change = pair[0].0 ^ pair[1].0;
            if change.count_ones() != 1 || pair[0].0 & pair[1].0 != pair[0].0 {
                return Err("convex plane requires a Freudenthal cell tetrahedron".into());
            }
            let axis = change.trailing_zeros() as usize;
            // TwoDiff via error-free expansion: a subnormal contribution must
            // survive even beside a unit coefficient.
            coefficients[axis + 1] = grow_expansion(&grow_expansion(&[], values[pair[1].1]), -values[pair[0].1]);
        }
        let mut plane = Self {
            cell: [*source.tetra.iter().min().unwrap(), *source.tetra.iter().max().unwrap()],
            bounds,
            coefficients,
            negative,
            boundary_axis: None,
        };
        plane.boundary_axis = plane.boundary_axis_plane(work)?;
        Ok(plane)
    }
    /// Exact signature for a plane that lies on a global axis-aligned lattice
    /// boundary. Multi-term coefficient expansions are compared by an
    /// error-free sum; approximate division and tolerances never qualify.
    fn boundary_axis_plane(&self, work: &mut WorkBudget) -> Result<Option<(usize, f64, i8)>, String> {
        charge(work, 8 + self.coefficients.iter().map(Vec::len).sum::<usize>())?;
        let mut selected = None;
        for coefficient in 1..4 {
            if !self.coefficients[coefficient].is_empty() {
                if selected.is_some() {
                    return Ok(None);
                }
                selected = Some(coefficient);
            }
        }
        let Some(coefficient) = selected else { return Ok(None) };
        let axis = coefficient - 1;
        let sign_term = *self.coefficients[coefficient].last().unwrap();
        if !sign_term.is_finite() || sign_term == 0. || self.bounds[0][axis] >= self.bounds[1][axis] {
            return Ok(None);
        }
        let boundary = if self.coefficients[0].is_empty() {
            0
        } else if self.coefficients[0].len() == 1 && self.coefficients[coefficient].len() == 1 {
            charge(work, 1)?;
            if self.coefficients[0][0] != -self.coefficients[coefficient][0] {
                return Ok(None);
            }
            1
        } else {
            let count = self.coefficients[0].len() + self.coefficients[coefficient].len();
            charge(work, (count + 1) * (count + 1) * 4)?;
            let mut sum = Vec::new();
            for &term in self.coefficients[0].iter().chain(&self.coefficients[coefficient]) {
                sum = grow_expansion(&sum, term);
            }
            if !sum.is_empty() {
                return Ok(None);
            }
            1
        };
        let coordinate = self.bounds[boundary][axis];
        if !coordinate.is_finite() || f64::from(coordinate as f32) != coordinate {
            return Ok(None);
        }
        let coefficient_sign = if sign_term > 0. { 1 } else { -1 };
        let outward_sign = if self.negative { coefficient_sign } else { -coefficient_sign };
        Ok(Some((axis, coordinate, outward_sign)))
    }
    fn same_oriented_plane(&self, other: &Self, work: &mut WorkBudget) -> Result<bool, String> {
        if self.cell != other.cell || self.bounds != other.bounds {
            charge(work, 4)?;
            return Ok(self.boundary_axis.is_some() && self.boundary_axis == other.boundary_axis);
        }
        let Some(pivot) = (1..4).find(|&i| !self.coefficients[i].is_empty()) else {
            return Err("convex plane has no nonzero normal coefficient".into());
        };
        charge(
            work,
            self.coefficients.iter().map(Vec::len).sum::<usize>()
                + other.coefficients.iter().map(Vec::len).sum::<usize>(),
        )?;
        if self.coefficients == other.coefficients {
            return Ok(self.negative == other.negative);
        }
        if other.coefficients[pivot].is_empty() {
            return Ok(false);
        }
        for i in 0..4 {
            let a = &self.coefficients[i];
            let b = &other.coefficients[pivot];
            let c = &other.coefficients[i];
            let d = &self.coefficients[pivot];
            let terms = 2 * (a.len() * b.len() + c.len() * d.len()) + 1;
            charge(work, terms * terms * 4)?;
            let mut difference = Vec::new();
            for (left, right, sign) in [(a, b, 1.), (c, d, -1.)] {
                for &a in left {
                    for &b in right {
                        let product = a * b;
                        let error = a.mul_add(b, -product);
                        difference = grow_expansion(&difference, sign * error);
                        difference = grow_expansion(&difference, sign * product);
                    }
                }
            }
            if !difference.is_empty() {
                return Ok(false);
            }
        }
        let same_sign = self.coefficients[pivot].last().unwrap().is_sign_negative()
            == other.coefficients[pivot].last().unwrap().is_sign_negative();
        Ok(if same_sign { self.negative == other.negative } else { self.negative != other.negative })
    }
}
/// Merge neighboring regions by canceling shared opposite boundary edges.
/// No boundary vertex used by another active region may disappear. Multiple
/// loops, holes, contacts and overlapping directed edges remain unmerged.
fn coalesce_regions(
    polygons: &[ConvexPolygon],
    perimeters: &mut [Vec<u32>],
    work: &mut WorkBudget,
) -> Result<usize, String> {
    use std::collections::{BTreeMap, BTreeSet, VecDeque};
    let mut edges = BTreeMap::<(u32, u32), BTreeSet<usize>>::new();
    let mut vertices = BTreeMap::<u32, BTreeSet<usize>>::new();
    for (region, perimeter) in perimeters.iter().enumerate() {
        charge(work, perimeter.len() * 4)?;
        for (i, &a) in perimeter.iter().enumerate() {
            let b = perimeter[(i + 1) % perimeter.len()];
            edges.entry((a.min(b), a.max(b))).or_default().insert(region);
            vertices.entry(a).or_default().insert(region);
        }
    }
    let mut queue = VecDeque::new();
    for regions in edges.values() {
        if regions.len() == 2 {
            let mut r = regions.iter();
            queue.push_back((*r.next().unwrap(), *r.next().unwrap()));
        }
    }
    let mut merged = 0;
    while let Some((left, right)) = queue.pop_front() {
        charge(work, 1)?;
        if perimeters[left].is_empty()
            || perimeters[right].is_empty()
            || polygons[left].feature != polygons[right].feature
        {
            continue;
        }
        if !polygons[left].plane.same_oriented_plane(&polygons[right].plane, work)? {
            continue;
        }
        charge(work, (perimeters[left].len() + perimeters[right].len()) * 8)?;
        let mut boundary = BTreeSet::new();
        let mut shared = 0;
        let mut duplicate = false;
        for perimeter in [&perimeters[left], &perimeters[right]] {
            for (i, &a) in perimeter.iter().enumerate() {
                let b = perimeter[(i + 1) % perimeter.len()];
                if boundary.remove(&(b, a)) {
                    shared += 1;
                } else if !boundary.insert((a, b)) {
                    duplicate = true;
                }
            }
        }
        if shared == 0 || duplicate || boundary.len() < 3 {
            continue;
        }
        let mut outgoing = BTreeMap::new();
        let mut incoming = BTreeSet::new();
        let mut singular = false;
        for &(a, b) in &boundary {
            if outgoing.insert(a, b).is_some() || !incoming.insert(b) {
                singular = true;
                break;
            }
        }
        if singular || outgoing.len() != incoming.len() || outgoing.keys().any(|v| !incoming.contains(v)) {
            continue;
        }
        let start = *outgoing.keys().next().unwrap();
        let mut current = start;
        let mut joined = Vec::new();
        loop {
            joined.push(current);
            current = outgoing[&current];
            if current == start || joined.len() >= boundary.len() {
                break;
            }
        }
        if current != start || joined.len() != boundary.len() {
            continue;
        }
        let retained: BTreeSet<_> = joined.iter().copied().collect();
        if perimeters[left]
            .iter()
            .chain(&perimeters[right])
            .any(|v| !retained.contains(v) && vertices[v].iter().any(|&region| region != left && region != right))
        {
            continue;
        }
        for region in [left, right] {
            for (i, &a) in perimeters[region].iter().enumerate() {
                let b = perimeters[region][(i + 1) % perimeters[region].len()];
                edges.get_mut(&(a.min(b), a.max(b))).unwrap().remove(&region);
                vertices.get_mut(&a).unwrap().remove(&region);
            }
        }
        perimeters[left] = joined;
        perimeters[right].clear();
        merged += 1;
        charge(work, perimeters[left].len() * 4)?;
        for (i, &a) in perimeters[left].iter().enumerate() {
            let b = perimeters[left][(i + 1) % perimeters[left].len()];
            vertices.entry(a).or_default().insert(left);
            let regions = edges.entry((a.min(b), a.max(b))).or_default();
            regions.insert(left);
            if regions.len() == 2 {
                let mut r = regions.iter();
                queue.push_back((*r.next().unwrap(), *r.next().unwrap()));
            }
        }
    }
    Ok(merged)
}
#[derive(Default)]
pub(crate) struct ConvexConformance {
    pub(crate) splits: usize,
    pub(crate) aliases: usize,
    pub(crate) deviation: f64,
    pub(crate) rounded_vertices: usize,
    pub(crate) source_rounding_displacement: f64,
    pub(crate) source_collinear_faces: usize,
    pub(crate) exact_pool_corrections: usize,
    pub(crate) exact_pool_binary_steps: usize,
    pub(crate) merged_polygons: usize,
    pub(crate) coalesced_interior_vertices: usize,
    pub(crate) redundant_boundary_vertices: usize,
    pub(crate) representation: Option<crate::meshing_local::ConvexRepresentationReport>,
}

fn convex_triangulation(
    perimeter: &[u32],
    positions: &[Vec3],
    normal: [f64; 3],
    work: &mut WorkBudget,
) -> Result<Option<Vec<[u32; 3]>>, String> {
    convex_triangulation_with(perimeter, |id| doubles(positions[id as usize]), normal, work)
}
fn convex_triangulation_with(
    perimeter: &[u32],
    coordinate: impl Fn(u32) -> [f64; 3],
    normal: [f64; 3],
    work: &mut WorkBudget,
) -> Result<Option<Vec<[u32; 3]>>, String> {
    let axis = (0..3).max_by(|&a, &b| normal[a].abs().total_cmp(&normal[b].abs())).unwrap();
    if normal[axis] == 0. || !normal.iter().all(|v| v.is_finite()) {
        return Err("convex facet has no finite source orientation".into());
    }
    let orient = |a: [f64; 3], b: [f64; 3], c: [f64; 3]| {
        let u = (axis + 1) % 3;
        let v = (axis + 2) % 3;
        (b[u] - a[u]) * (c[v] - a[v]) - (b[v] - a[v]) * (c[u] - a[u])
    };
    let on_segment = |a: [f64; 3], b: [f64; 3], p: [f64; 3]| {
        [(axis + 1) % 3, (axis + 2) % 3].into_iter().all(|i| p[i] >= a[i].min(b[i]) && p[i] <= a[i].max(b[i]))
    };
    for i in 0..perimeter.len() {
        for j in i + 1..perimeter.len() {
            if j == i + 1 || i == 0 && j + 1 == perimeter.len() {
                continue;
            }
            charge(work, 12)?;
            let [a, b, c, d] = [
                perimeter[i],
                perimeter[(i + 1) % perimeter.len()],
                perimeter[j],
                perimeter[(j + 1) % perimeter.len()],
            ]
            .map(&coordinate);
            let [ab_c, ab_d, cd_a, cd_b] = [orient(a, b, c), orient(a, b, d), orient(c, d, a), orient(c, d, b)];
            if ab_c * ab_d < 0. && cd_a * cd_b < 0.
                || ab_c == 0. && on_segment(a, b, c)
                || ab_d == 0. && on_segment(a, b, d)
                || cd_a == 0. && on_segment(c, d, a)
                || cd_b == 0. && on_segment(c, d, b)
            {
                return Ok(None);
            }
        }
    }
    if (5..=MAX_CACHED_EAR_VERTICES).contains(&perimeter.len()) {
        return cached_ear_clipping(perimeter, normal, &coordinate, work);
    }
    charge(work, perimeter.len())?;
    let mut perimeter = perimeter.to_vec();
    let mut pieces = Vec::new();
    while perimeter.len() > 3 {
        let n = perimeter.len();
        let mut chosen = None;
        for i in 0..n {
            charge(work, n * 16)?;
            let ids = [perimeter[(i + n - 1) % n], perimeter[i], perimeter[(i + 1) % n]];
            let [a, b, c] = ids.map(&coordinate);
            let face_cross = cross(sub(b, a), sub(c, a));
            if dot(face_cross, normal) <= 0. {
                continue;
            }
            if perimeter.iter().filter(|id| !ids.contains(id)).any(|&id| {
                let p = coordinate(id);
                [(a, b), (b, c), (c, a)].into_iter().all(|(a, b)| dot(cross(sub(b, a), sub(p, a)), normal) >= 0.)
            }) {
                continue;
            }
            let ab = sub(b, a);
            let ac = sub(c, a);
            let bc = sub(c, b);
            let sum = dot(ab, ab) + dot(ac, ac) + dot(bc, bc);
            let quality = dot(face_cross, face_cross) / (sum * sum);
            if chosen.is_none_or(|(_, best)| quality > best) {
                chosen = Some((i, quality));
            }
        }
        let Some((i, _)) = chosen else {
            return Ok(None);
        };
        pieces.push([perimeter[(i + n - 1) % n], perimeter[i], perimeter[(i + 1) % n]]);
        perimeter.remove(i);
    }
    let triangle: [u32; 3] = perimeter.try_into().map_err(|_| "convex perimeter lost its triangle")?;
    let [a, b, c] = triangle.map(&coordinate);
    if dot(cross(sub(b, a), sub(c, a)), normal) <= 0. {
        return Ok(None);
    }
    pieces.push(triangle);
    Ok(Some(pieces))
}
const MAX_CACHED_EAR_VERTICES: usize = 16;
#[derive(Clone, Copy)]
struct CachedEar {
    quality: Option<f64>,
    blockers: u32,
}
fn evaluate_cached_ear(
    ring: &[(u32, u32)],
    index: usize,
    normal: [f64; 3],
    coordinate: &impl Fn(u32) -> [f64; 3],
    work: &mut WorkBudget,
) -> Result<CachedEar, String> {
    let n = ring.len();
    // Exactly the old charge for every ear evaluation that is still performed.
    charge(work, n * 16)?;
    let ids = [ring[(index + n - 1) % n].0, ring[index].0, ring[(index + 1) % n].0];
    let [a, b, c] = ids.map(coordinate);
    let face_cross = cross(sub(b, a), sub(c, a));
    if dot(face_cross, normal) <= 0. {
        return Ok(CachedEar { quality: None, blockers: 0 });
    }
    let mut blockers = 0;
    for &(id, slot) in ring {
        if ids.contains(&id) {
            continue;
        }
        let p = coordinate(id);
        if [(a, b), (b, c), (c, a)].into_iter().all(|(a, b)| dot(cross(sub(b, a), sub(p, a)), normal) >= 0.) {
            blockers |= 1 << slot;
        }
    }
    // A blocked ear keeps its score for the case where its last blocker is
    // removed later. Its vertex triple/coordinate arithmetic are unchanged.
    let ab = sub(b, a);
    let ac = sub(c, a);
    let bc = sub(c, b);
    let sum = dot(ab, ab) + dot(ac, ac) + dot(bc, bc);
    let quality = dot(face_cross, face_cross) / (sum * sum);
    Ok(CachedEar { quality: Some(quality), blockers })
}
fn cached_ear_clipping(
    perimeter: &[u32],
    normal: [f64; 3],
    coordinate: &impl Fn(u32) -> [f64; 3],
    work: &mut WorkBudget,
) -> Result<Option<Vec<[u32; 3]>>, String> {
    charge(work, perimeter.len() * 4 + 1)?;
    let mut ring: Vec<_> = perimeter.iter().copied().enumerate().map(|(slot, id)| (id, slot as u32)).collect();
    let mut cache = Vec::with_capacity(ring.len());
    let mut active = (1u32 << ring.len()) - 1;
    let mut pieces = Vec::new();
    for i in 0..ring.len() {
        cache.push(evaluate_cached_ear(&ring, i, normal, coordinate, work)?);
    }
    while ring.len() > 3 {
        let n = ring.len();
        let mut chosen = None;
        for (i, ear) in cache.iter().enumerate() {
            charge(work, 2)?;
            let Some(quality) = ear.quality else { continue };
            if ear.blockers & active != 0 {
                continue;
            }
            if chosen.is_none_or(|(_, best)| quality > best) {
                chosen = Some((i, quality));
            }
        }
        let Some((i, _)) = chosen else { return Ok(None) };
        charge(work, n * 2 + 4)?;
        pieces.push([ring[(i + n - 1) % n].0, ring[i].0, ring[(i + 1) % n].0]);
        active &= !(1 << ring[i].1);
        ring.remove(i);
        cache.remove(i);
        if ring.len() > 3 {
            let next = i % ring.len();
            let previous = (next + ring.len() - 1) % ring.len();
            for changed in [previous, next] {
                charge(work, 1)?;
                cache[changed] = evaluate_cached_ear(&ring, changed, normal, coordinate, work)?;
            }
        }
    }
    charge(work, 3)?;
    let triangle = [ring[0].0, ring[1].0, ring[2].0];
    let [a, b, c] = triangle.map(coordinate);
    if dot(cross(sub(b, a), sub(c, a)), normal) <= 0. {
        return Ok(None);
    }
    pieces.push(triangle);
    Ok(Some(pieces))
}

fn d_component(p: Vec3, axis: usize) -> f64 {
    f64::from([p.x, p.y, p.z][axis])
}
fn f32_source_interval(value: f64) -> Vec<f32> {
    let nearest = value as f32;
    let mut values = if f64::from(nearest) == value {
        vec![nearest]
    } else if f64::from(nearest) < value {
        vec![nearest, nearest.next_up()]
    } else {
        vec![nearest.next_down(), nearest]
    };
    for value in &mut values {
        if *value == 0. {
            *value = 0.;
        }
    }
    values
}
#[derive(Clone)]
struct ExactCoordinate {
    numerator: Vec<f64>,
    denominator: Vec<f64>,
    // Proven from equal original lattice-coordinate components across all
    // support nodes, never inferred from a rounded quotient or near equality.
    exact_f32: std::cell::Cell<Option<f32>>,
}
impl ExactCoordinate {
    fn all_from_source(source: &SourcePoint, work: &mut WorkBudget) -> Result<[Self; 3], String> {
        let affine = source.affine.as_ref().ok_or("exact rounding requires original source samples")?;
        let local: Vec<_> = source
            .nodes
            .iter()
            .filter(|&&id| id != u32::MAX)
            .map(|id| affine.tetra.iter().position(|node| node == id).ok_or("exact rounding lost source node"))
            .collect::<Result<_, _>>()?;
        charge(
            work,
            match local.len() {
                1 => 1,
                2 => 16,
                3 => 256,
                _ => 4096,
            },
        )?;
        let values: Vec<_> = affine.planes.iter().map(|(_, v)| *v).collect();
        let basis: Vec<_> = affine
            .basis
            .iter()
            .filter(|&&id| id != u64::MAX)
            .map(|id| affine.planes.iter().position(|(other, _)| id == other).ok_or("exact rounding lost source basis"))
            .collect::<Result<_, _>>()?;
        if basis.len() + 1 != local.len() {
            return Err("exact rounding has inconsistent source rank".into());
        }
        let mut numerators: [Vec<f64>; 3] = std::array::from_fn(|_| Vec::new());
        let mut denominator = Vec::new();
        for (column, &node) in local.iter().enumerate() {
            let coordinates = affine.points[node];
            if coordinates.iter().any(|&coordinate| f64::from(coordinate as f32) != coordinate) {
                return Err("exact rounding requires original f32 lattice coordinates".into());
            }
            let sign = if column % 2 == 0 { 1. } else { -1. };
            for cofactor in exact_cofactor(&local, &basis, column, &values) {
                let cofactor = sign * cofactor;
                denominator = grow_expansion(&denominator, cofactor);
                for (numerator, coordinate) in numerators.iter_mut().zip(coordinates) {
                    let product = cofactor * coordinate;
                    let error = cofactor.mul_add(coordinate, -product);
                    *numerator = grow_expansion(numerator, error);
                    *numerator = grow_expansion(numerator, product);
                }
            }
        }
        let last = *denominator.last().ok_or("exact rounding has a zero source determinant")?;
        if last < 0. {
            numerators.iter_mut().flatten().chain(&mut denominator).for_each(|v| *v = -*v);
        }
        let mut result = numerators.map(|numerator| Self {
            numerator,
            denominator: denominator.clone(),
            exact_f32: std::cell::Cell::new(None),
        });
        charge(work, local.len() * 3 + 3)?;
        for (axis, coordinate) in result.iter_mut().enumerate() {
            let original = affine.points[local[0]][axis];
            if local.iter().all(|&node| affine.points[node][axis] == original) {
                // Preserve the old reported approximation and tight interval
                // bit-for-bit as well as the exact mathematical coordinate.
                charge(work, coordinate.numerator.len() + coordinate.denominator.len() + 1)?;
                if coordinate.approximate() == original {
                    coordinate.exact_f32.set(Some(if original == 0. { 0. } else { original as f32 }));
                }
            }
        }
        Ok(result)
    }
    // A single numerator/denominator needs no growing expansion. TwoProduct
    // gives denominator*candidate = high+low exactly. Any OTHER finite f64
    // numerator lies strictly outside high's rounding cell; when numerator==
    // high the residual alone decides the sign. A rounded product >=2^-900
    // has no exact product bit below2^-1006 for two53-bit factors, so the FMA
    // residual is representable without subnormal underflow.
    fn single_product_compare(&self, candidate: f64, work: &mut WorkBudget) -> Result<Option<i8>, String> {
        if self.numerator.len() != 1 || self.denominator.len() != 1 {
            return Ok(None);
        }
        charge(work, 16)?;
        let numerator = self.numerator[0];
        let denominator = self.denominator[0];
        let high = denominator * candidate;
        if !numerator.is_finite()
            || !candidate.is_finite()
            || !high.is_finite()
            || denominator != 0. && candidate != 0. && high.abs() < f64::from_bits((123u64) << 52)
        {
            return Ok(None);
        }
        let low = denominator.mul_add(candidate, -high);
        Ok(Some(if numerator < high {
            -1
        } else if numerator > high {
            1
        } else if low > 0. {
            -1
        } else if low < 0. {
            1
        } else {
            0
        }))
    }
    fn compare(&self, candidate: f32, work: &mut WorkBudget) -> Result<i8, String> {
        if !candidate.is_finite() {
            return Err("exact rounding comparison requires finite f32 value".into());
        }
        if let Some(exact) = self.exact_f32.get() {
            charge(work, 1)?;
            return Ok(if exact < candidate {
                -1
            } else if exact > candidate {
                1
            } else {
                0
            });
        }
        if let Some(sign) = self.single_product_compare(f64::from(candidate), work)? {
            if sign == 0 {
                self.exact_f32.set(Some(if candidate == 0. { 0. } else { candidate }));
            }
            return Ok(sign);
        }
        let bound = self.numerator.len() + 2 * self.denominator.len() + 1;
        charge(work, bound * bound * 4)?;
        let mut delta = self.numerator.clone();
        let candidate = -f64::from(candidate);
        for &component in &self.denominator {
            let product = component * candidate;
            let error = component.mul_add(candidate, -product);
            delta = grow_expansion(&delta, error);
            delta = grow_expansion(&delta, product);
        }
        let sign = delta.last().map_or(0, |v| if *v < 0. { -1 } else { 1 });
        if sign == 0 {
            // This exact expansion comparison proved the rational coordinate
            // equals this original finite f32 value. Later queries of this
            // immutable coordinate can reuse that fact without multiplying
            // and summing the same denominator again.
            self.exact_f32.set(Some(if candidate == 0. { 0. } else { -candidate as f32 }));
        }
        Ok(sign)
    }
    fn approximate(&self) -> f64 {
        self.numerator.iter().sum::<f64>() / self.denominator.iter().sum::<f64>()
    }
    fn tight_interval(&self, work: &mut WorkBudget) -> Result<Option<[f64; 2]>, String> {
        if self.numerator.is_empty() {
            return Ok(Some([0., 0.]));
        }
        let candidate = self.approximate();
        if let Some(exact) = self.exact_f32.get() {
            if candidate == f64::from(exact) {
                charge(work, self.numerator.len() + self.denominator.len() + 2)?;
                return Ok(Some([candidate; 2]));
            }
        }
        fn least_bit(value: f64) -> i32 {
            let bits = value.to_bits() & 0x7fff_ffff_ffff_ffff;
            let exponent = (bits >> 52) as i32;
            let significand =
                if exponent == 0 { bits & ((1u64 << 52) - 1) } else { (bits & ((1u64 << 52) - 1)) | (1u64 << 52) };
            (if exponent == 0 { -1074 } else { exponent - 1023 - 52 }) + significand.trailing_zeros() as i32
        }
        let safe = candidate.is_finite()
            && self.denominator.iter().all(|&v| {
                v == 0.
                    || candidate == 0.
                    || least_bit(v) + least_bit(candidate) >= -1074 && (v * candidate).is_finite()
            });
        let simple = if safe { self.single_product_compare(candidate, work)? } else { None };
        if simple == Some(0) {
            return Ok(Some([candidate, candidate]));
        }
        if safe && simple.is_none() {
            let bound = self.numerator.len() + self.denominator.len() * 2 + 1;
            charge(work, bound * bound * 4)?;
            let mut delta = self.numerator.clone();
            for &component in &self.denominator {
                let product = -candidate * component;
                let error = (-candidate).mul_add(component, -product);
                delta = grow_expansion(&delta, error);
                delta = grow_expansion(&delta, product);
            }
            if delta.is_empty() {
                return Ok(Some([candidate, candidate]));
            }
        }
        charge(work, (self.numerator.len() + self.denominator.len()) * 4 + 16)?;
        let sum_bounds = |terms: &[f64]| {
            let mut low = 0.;
            let mut high = 0.;
            for &value in terms {
                low = (low + value).next_down();
                high = (high + value).next_up();
            }
            [low, high]
        };
        let [nl, nh] = sum_bounds(&self.numerator);
        let [dl, dh] = sum_bounds(&self.denominator);
        if dl <= 0. || ![nl, nh, dl, dh].into_iter().all(f64::is_finite) {
            return Ok(None);
        }
        let quotients = [nl / dl, nl / dh, nh / dl, nh / dh];
        if !quotients.iter().all(|v| v.is_finite()) {
            return Ok(None);
        }
        Ok(Some([
            quotients.iter().copied().fold(f64::INFINITY, f64::min).next_down(),
            quotients.iter().copied().fold(f64::NEG_INFINITY, f64::max).next_up(),
        ]))
    }
    fn nearest(&self, interval: &[f32], work: &mut WorkBudget) -> Result<f32, String> {
        if interval.len() == 1 {
            return Ok(interval[0]);
        }
        if interval.len() != 2 {
            return Err("exact source rounding has an invalid interval".into());
        }
        charge(work, 1)?;
        if interval[0].next_up() == interval[1] {
            charge(work, 2)?;
            let midpoint = (f64::from(interval[0]) + f64::from(interval[1])) * 0.5;
            if let Some(sign) = self.single_product_compare(midpoint, work)? {
                return Ok(if sign < 0 || sign == 0 && interval[0].to_bits() & 1 == 0 {
                    interval[0]
                } else {
                    interval[1]
                });
            }
        }
        let bound = self.numerator.len() + 4 * self.denominator.len() + 1;
        charge(work, bound * bound * 4)?;
        let mut delta: Vec<_> = self.numerator.iter().map(|v| v * 2.).collect();
        for &value in interval {
            for &component in &self.denominator {
                let value = -f64::from(value);
                let product = component * value;
                let error = component.mul_add(value, -product);
                delta = grow_expansion(&delta, error);
                delta = grow_expansion(&delta, product);
            }
        }
        Ok(match delta.last() {
            Some(v) if *v < 0. => interval[0],
            Some(_) => interval[1],
            None => {
                if interval[0].to_bits() & 1 == 0 {
                    interval[0]
                } else {
                    interval[1]
                }
            }
        })
    }
    fn interval(&self, hint: f64, work: &mut WorkBudget) -> Result<(Vec<f32>, usize), String> {
        if self.numerator.is_empty() {
            return Ok((vec![0.], 0));
        }
        if let Some(exact) = self.exact_f32.get() {
            charge(work, 1)?;
            return Ok((vec![exact], 0));
        }
        let nearest = hint as f32;
        let nearest_sign = if nearest.is_finite() { Some(self.compare(nearest, work)?) } else { None };
        if nearest_sign == Some(0) {
            return Ok((vec![nearest], 0));
        }
        let old = f32_source_interval(hint);
        if old.iter().all(|v| v.is_finite()) {
            let signs = old
                .iter()
                .map(|&v| {
                    if v == nearest {
                        charge(work, 1)?;
                        Ok(nearest_sign.expect("finite old endpoint equals the finite hint"))
                    } else {
                        self.compare(v, work)
                    }
                })
                .collect::<Result<Vec<_>, String>>()?;
            if let Some(i) = signs.iter().position(|&s| s == 0) {
                return Ok((vec![old[i]], 0));
            }
            if signs.len() == 2 && signs[0] > 0 && signs[1] < 0 {
                return Ok((old, 0));
            }
        }
        fn key(value: f32) -> u32 {
            let bits = value.to_bits();
            if bits >> 31 == 0 {
                bits ^ 0x8000_0000
            } else {
                !bits
            }
        }
        fn value(key: u32) -> f32 {
            let value = f32::from_bits(if key >> 31 == 0 { !key } else { key ^ 0x8000_0000 });
            if value == 0. {
                0.
            } else {
                value
            }
        }
        let mut low = key(-f32::MAX);
        let mut high = key(f32::MAX);
        let low_sign = self.compare(value(low), work)?;
        let high_sign = self.compare(value(high), work)?;
        if low_sign == 0 {
            return Ok((vec![value(low)], 0));
        }
        if high_sign == 0 {
            return Ok((vec![value(high)], 0));
        }
        if low_sign < 0 || high_sign > 0 {
            return Err("exact source coordinate exceeds finite f32 range".into());
        }
        let mut steps = 0;
        while high - low > 1 {
            steps += 1;
            if steps > 32 {
                return Err("exact f32 rounding search exceeded 32 comparisons".into());
            }
            let middle = low + (high - low) / 2;
            match self.compare(value(middle), work)? {
                0 => return Ok((vec![value(middle)], steps)),
                1 => low = middle,
                _ => high = middle,
            }
        }
        let lower = value(low);
        let upper = value(high);
        if self.compare(lower, work)? <= 0 || self.compare(upper, work)? >= 0 {
            return Err("exact source rounding failed to bracket its coordinate".into());
        }
        Ok((vec![lower, upper], steps))
    }
}
struct SourceRounding {
    lower: [f32; 3],
    upper: [f32; 3],
    exact: [f64; 3],
    bounds: [[f64; 3]; 2],
}
/// Outward-rounded bound from an original exact-coordinate interval box to a
/// stored f32 point. Constant equal coordinates contribute exactly zero; no
/// coordinate borrows another axis's ULP allowance.
fn source_displacement_bound(source: &SourceRounding, point: Vec3) -> f64 {
    let upward = |value: f64| if value == 0. { 0. } else { value.next_up() };
    let mut distances = [0.; 3];
    for (axis, value) in [point.x, point.y, point.z].into_iter().enumerate() {
        let value = f64::from(value);
        let distance =
            upward((value - source.bounds[0][axis]).abs()).max(upward((value - source.bounds[1][axis]).abs()));
        distances[axis] = distance;
    }
    let maximum = distances.into_iter().fold(0., f64::max);
    if maximum == 0. {
        return 0.;
    }
    let mut squared = 0.;
    for distance in distances {
        if distance != 0. {
            let scaled = (distance / maximum).next_up();
            squared = (squared + (scaled * scaled).next_up()).next_up();
        }
    }
    (maximum * squared.sqrt().next_up()).next_up()
}
fn remove_redundant_region_vertices(
    perimeters: &mut [Vec<u32>],
    sources: &[SourcePoint],
    rounding: &[SourceRounding],
    work: &mut WorkBudget,
) -> Result<usize, String> {
    use std::collections::{BTreeMap, BTreeSet, VecDeque};
    let mut incidents = BTreeMap::<u32, BTreeSet<usize>>::new();
    for (region, perimeter) in perimeters.iter().enumerate() {
        charge(work, perimeter.len())?;
        for &vertex in perimeter {
            incidents.entry(vertex).or_default().insert(region);
        }
    }
    let mut queue: VecDeque<_> = incidents.keys().copied().collect();
    let mut removed = 0;
    while let Some(vertex) = queue.pop_front() {
        charge(work, 1)?;
        let Some(regions) = incidents.get(&vertex) else {
            continue;
        };
        if regions.is_empty() {
            continue;
        }
        let mut neighbors = None;
        let mut valid = true;
        for &region in regions {
            let perimeter = &perimeters[region];
            charge(work, perimeter.len())?;
            if perimeter.len() <= 3 {
                valid = false;
                break;
            }
            let i =
                perimeter.iter().position(|&v| v == vertex).ok_or("coordinated boundary incidence is inconsistent")?;
            let a = perimeter[(i + perimeter.len() - 1) % perimeter.len()];
            let b = perimeter[(i + 1) % perimeter.len()];
            let pair = (a.min(b), a.max(b));
            if neighbors.is_some_and(|old| old != pair) {
                valid = false;
                break;
            }
            neighbors = Some(pair);
        }
        if !valid {
            continue;
        }
        let (a, b) = neighbors.unwrap();
        let pa = &rounding[a as usize];
        let pb = &rounding[b as usize];
        let pm = &rounding[vertex as usize];
        let between = (0..3).any(|axis| {
            pa.upper[axis] < pb.lower[axis] && pm.lower[axis] >= pa.upper[axis] && pm.upper[axis] <= pb.lower[axis]
                || pb.upper[axis] < pa.lower[axis]
                    && pm.lower[axis] >= pb.upper[axis]
                    && pm.upper[axis] <= pa.lower[axis]
        });
        if !between {
            continue;
        }
        let points = [&sources[a as usize], &sources[vertex as usize], &sources[b as usize]];
        let mut collinear = singleton_exact_line(points, work)?;
        for i in 0..3 {
            if collinear {
                break;
            }
            if source_collinear([points[i], points[(i + 1) % 3], points[(i + 2) % 3]], work)? {
                collinear = true;
                break;
            }
        }
        if !collinear {
            continue;
        }
        let regions = incidents.remove(&vertex).unwrap();
        for region in regions {
            perimeters[region].retain(|&v| v != vertex);
        }
        removed += 1;
        queue.push_back(a);
        queue.push_back(b);
    }
    Ok(removed)
}
#[path = "meshing_source_basis.rs"]
mod source_basis;
use source_basis::{source_plane_sign, ExactBasisProof};

fn source_plane_sign_uncached(
    source: &SourcePoint,
    nodes: &[u32],
    query: &[f64],
    work: &mut WorkBudget,
) -> Result<i8, String> {
    let affine = source.affine.as_ref().ok_or("convex source identity lacks original samples")?;
    let local: Vec<_> = source
        .nodes
        .iter()
        .filter(|&&id| id != u32::MAX)
        .map(|id| affine.tetra.iter().position(|node| node == id).ok_or("source identity lost lattice support"))
        .collect::<Result<_, _>>()?;
    charge(
        work,
        match local.len() {
            1 => 1,
            2 => 16,
            3 => 256,
            _ => 4096,
        },
    )?;
    let values: Vec<_> = affine.planes.iter().map(|(_, values)| *values).collect();
    let basis: Vec<_> = affine
        .basis
        .iter()
        .filter(|&&id| id != u64::MAX)
        .map(|id| affine.planes.iter().position(|(other, _)| id == other).ok_or("source identity lost its basis"))
        .collect::<Result<_, _>>()?;
    let mut row = [0.; 4];
    for &i in &local {
        let column =
            nodes.iter().position(|id| *id == affine.tetra[i]).ok_or("source identity queries a different simplex")?;
        row[i] = query[column];
    }
    exact_plane_sign(&local, &basis, &values, row)
}
fn same_source_point(a: &SourcePoint, b: &SourcePoint, work: &mut WorkBudget) -> Result<bool, String> {
    if a.nodes != b.nodes {
        return Ok(false);
    }
    let nodes: Vec<_> = a.nodes.into_iter().filter(|&id| id != u32::MAX).collect();
    if nodes.len() == 1 {
        return Ok(true);
    }
    let affine = a.affine.as_ref().ok_or("source identity requires original affine basis")?;
    // Validate uniqueness of A; the exact sign routine rejects zero determinant.
    source_plane_sign(a, &nodes, &vec![0.; nodes.len()], work)?;
    for id in affine.basis.iter().filter(|&&id| id != u64::MAX) {
        let values = &affine.planes.iter().find(|(other, _)| id == other).ok_or("source identity lost a plane")?.1;
        let query: Vec<_> =
            nodes.iter().map(|id| values[affine.tetra.iter().position(|node| node == id).unwrap()]).collect();
        if source_plane_sign(b, &nodes, &query, work)? != 0 {
            return Ok(false);
        }
    }
    Ok(true)
}
fn source_collinear(points: [&SourcePoint; 3], work: &mut WorkBudget) -> Result<bool, String> {
    use std::collections::BTreeSet;
    charge(work, 12)?;
    let nodes: Vec<_> =
        points.iter().flat_map(|p| p.nodes).filter(|&id| id != u32::MAX).collect::<BTreeSet<_>>().into_iter().collect();
    if nodes.len() <= 2 {
        return Ok(true);
    }
    if nodes.len() > 4 {
        return Ok(false);
    }
    let first = points[0].affine.as_ref().ok_or("convex collinearity lacks source support")?;
    if !nodes.iter().all(|id| first.tetra.contains(id)) {
        return Ok(false);
    }
    let mut rows = Vec::new();
    for (id, values) in &first.planes {
        if !points
            .iter()
            .all(|point| point.affine.as_ref().is_some_and(|a| a.planes.iter().any(|(other, _)| id == other)))
        {
            continue;
        }
        let query: Vec<_> =
            nodes.iter().map(|id| values[first.tetra.iter().position(|node| node == id).unwrap()]).collect();
        if query.iter().all(|&v| v == query[0]) {
            continue;
        }
        let mut valid = true;
        for point in points {
            if source_plane_sign(point, &nodes, &query, work)? != 0 {
                valid = false;
                break;
            }
        }
        if !valid {
            continue;
        }
        if nodes.len() == 3 {
            return Ok(true);
        }
        let row: [f64; 4] = query.try_into().unwrap();
        for &previous in &rows {
            charge(work, 256)?;
            let values = [[1.; 4], previous, row];
            if (0..4).any(|column| !exact_cofactor(&[0, 1, 2, 3], &[0, 1, 2], column, &values).is_empty()) {
                return Ok(true);
            }
        }
        rows.push(row);
    }
    Ok(false)
}

/// Exact collinearity for three singleton source lattice points. The source
/// domain may span neighboring tetrahedra, so the older shared-simplex proof
/// is insufficient. Error-free differences and product residuals are required;
/// a rounded cross product never establishes this predicate.
fn singleton_exact_line(points: [&SourcePoint; 3], work: &mut WorkBudget) -> Result<bool, String> {
    charge(work, 12)?;
    let mut nodes = [0u32; 3];
    for (index, point) in points.iter().enumerate() {
        let mut active = point.nodes.iter().copied().filter(|&node| node != u32::MAX);
        let Some(node) = active.next() else { return Ok(false) };
        if active.next().is_some() {
            return Ok(false);
        }
        nodes[index] = node;
    }
    charge(work, 57)?;
    let mut coordinates = [[0.; 3]; 3];
    for (index, point) in points.iter().enumerate() {
        let Some(affine) = point.affine.as_ref() else { return Ok(false) };
        let Some(local) = affine.tetra.iter().position(|&node| node == nodes[index]) else { return Ok(false) };
        coordinates[index] = affine.points[local];
        if !coordinates[index].iter().all(|&value| value.is_finite() && f64::from(value as f32) == value) {
            return Ok(false);
        }
    }
    charge(work, 24)?;
    for axis in 0..3 {
        let [a, middle, b] = coordinates.map(|point| point[axis]);
        if (a < middle && middle < b || b < middle && middle < a)
            && (0..3).filter(|&other| other != axis).all(|other| {
                coordinates[0][other] == coordinates[1][other] && coordinates[1][other] == coordinates[2][other]
            })
        {
            return Ok(true);
        }
    }
    // Finite f32 coordinates have finite differences in f64. If TwoDiff has a
    // nonzero tail, defer to the shared-domain proof rather than rounding it.
    charge(work, 66)?;
    let difference = |a: f64, b: f64| -> Option<f64> {
        let high = a - b;
        let virtual_b = a - high;
        let virtual_a = high + virtual_b;
        let error = (a - virtual_a) + (virtual_b - b);
        (high.is_finite() && error == 0.).then_some(high)
    };
    let mut first = [0.; 3];
    let mut second = [0.; 3];
    for axis in 0..3 {
        let Some(value) = difference(coordinates[1][axis], coordinates[0][axis]) else { return Ok(false) };
        let Some(value2) = difference(coordinates[2][axis], coordinates[0][axis]) else { return Ok(false) };
        first[axis] = value;
        second[axis] = value2;
    }
    let between = (0..3).any(|axis| {
        let [a, middle, b] = coordinates.map(|point| point[axis]);
        a < middle && middle < b || b < middle && middle < a
    });
    if !between {
        return Ok(false);
    }
    charge(work, 24)?;
    let product = |a: f64, b: f64| {
        let high = a * b;
        (high, a.mul_add(b, -high))
    };
    for axis in 0..3 {
        let i = (axis + 1) % 3;
        let j = (axis + 2) % 3;
        if product(first[i], second[j]) != product(first[j], second[i]) {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Exact necessary-condition index for source-edge stitching. A middle
/// source may use only nodes in the at-most-four-node endpoint domain, so the
/// only possible keys are its at most15 nonempty subsets. Remaining plane,
/// rank, barycentric and residual gates are unchanged after this enumeration.
fn source_domain_candidates(
    plane: u64,
    domain: &[u32],
    index: &std::collections::BTreeMap<(u64, [u32; 4]), Vec<u32>>,
    work: &mut WorkBudget,
) -> Result<Vec<u32>, String> {
    if domain.is_empty() || domain.len() > 4 {
        return Err("source stitching domain exceeds four nodes".into());
    }
    let mut candidates = std::collections::BTreeSet::new();
    for mask in 1usize..(1usize << domain.len()) {
        charge(work, domain.len() + 1)?;
        let mut key = [u32::MAX; 4];
        let mut count = 0;
        for (i, &node) in domain.iter().enumerate() {
            if mask & (1 << i) != 0 {
                key[count] = node;
                count += 1;
            }
        }
        key.sort_unstable();
        if let Some(vertices) = index.get(&(plane, key)) {
            charge(work, vertices.len())?;
            candidates.extend(vertices.iter().copied());
        }
    }
    charge(work, candidates.len())?;
    Ok(candidates.into_iter().collect())
}
impl Output {
    pub(crate) fn local(max_vertices: usize, max_triangles: usize) -> Self {
        Self {
            face_features: Some(Vec::new()),
            global_supports: Some(Vec::new()),
            limits: Some([max_vertices.min(MAX_MESH_VERTICES), max_triangles.min(MAX_MESH_TRIANGLES)]),
            ..Self::default()
        }
    }
    pub(crate) fn clear_vertex_cache(&mut self) {
        self.vertices.clear();
    }

    /// Reconcile selective convex clipping's T-junctions before the shared
    /// cleanup. Only existing vertices with a common original affine line in
    /// the same lattice simplex can split an edge; proximity never qualifies.
    pub(crate) fn conform_convex_edges(
        &mut self,
        representation_policy: Option<crate::meshing_local::ConvexRepresentationPolicy>,
        work: &mut WorkBudget,
    ) -> Result<ConvexConformance, String> {
        use std::collections::{BTreeMap, BTreeSet};
        charge(work, self.positions.len() * 2 + self.triangles.len() * 3 + self.collapsed.len() * 3)?;
        let original_positions = self.positions.clone();
        let mut source_rounding = Vec::with_capacity(self.sources.len());
        let mut pool_corrections = 0;
        let mut pool_binary_steps = 0;
        let coordinate_pool_start = work.used;
        for (i, source) in self.sources.iter().enumerate() {
            let (pool, nearest) = (|| -> Result<_, String> {
                let coordinates = ExactCoordinate::all_from_source(source, work)?;
            let mut pool = SourceRounding { lower: [0.; 3], upper: [0.; 3], exact: [0.; 3], bounds: [[0.; 3]; 2] };
            let mut nearest = [0.; 3];
            for (axis, coordinate) in coordinates.into_iter().enumerate() {
                let hint = source.point[axis];
                let (interval, steps) = coordinate.interval(hint, work)?;
                if interval != f32_source_interval(hint) {
                    pool_corrections += 1;
                }
                pool_binary_steps += steps;
                pool.lower[axis] = interval[0];
                pool.upper[axis] = *interval.last().unwrap();
                pool.exact[axis] = coordinate.approximate();
                let tight = coordinate
                    .tight_interval(work)?
                    .unwrap_or([f64::from(pool.lower[axis]), f64::from(pool.upper[axis])]);
                pool.bounds[0][axis] = tight[0];
                pool.bounds[1][axis] = tight[1];
                nearest[axis] = coordinate.nearest(&interval, work)?;
            }
                Ok((pool, nearest))
            })().map_err(|error| format!("{error}; exact coordinate pools source {i}/{}, phase_start {coordinate_pool_start}, phase_spent {}, raw_vertices {}, raw_triangles {}, polygons {}", self.sources.len(), work.used-coordinate_pool_start, self.positions.len(), self.triangles.len(), self.convex_polygons.len()))?;
            self.positions[i] = Vec3::new(nearest[0], nearest[1], nearest[2]);
            source_rounding.push(pool);
        }
        let coordinate_pool_end = work.used;
        fn root(parent: &mut [usize], mut i: usize) -> usize {
            while parent[i] != i {
                parent[i] = parent[parent[i]];
                i = parent[i];
            }
            i
        }
        let mut parent: Vec<_> = (0..self.positions.len()).collect();
        let mut buckets = BTreeMap::<([u32; 4], [u32; 3]), Vec<usize>>::new();
        for i in 0..self.positions.len() {
            charge(work, 1)?;
            let p = self.positions[i];
            let bits = [p.x, p.y, p.z].map(|v| if v == 0. { 0 } else { v.to_bits() });
            let bucket = buckets.entry((self.sources[i].nodes, bits)).or_default();
            for &other in bucket.iter() {
                if same_source_point(&self.sources[other], &self.sources[i], work)? {
                    parent[i] = root(&mut parent, other);
                    break;
                }
            }
            bucket.push(i);
        }
        // Convex collapsed records now describe exact source-collinear fans,
        // not nearest-f32 collisions. Their vertices participate in support
        // stitching; no positive source facet has been discarded or contracted.
        self.collapsed.clear();
        let mut families = vec![Vec::new(); parent.len()];
        for i in 0..parent.len() {
            let r = root(&mut parent, i);
            families[r].push(i);
        }
        let aliases = families.iter().filter(|family| family.is_empty()).count();
        for triangle in self.triangles.iter_mut().chain(&mut self.collapsed) {
            for i in triangle {
                *i = root(&mut parent, *i as usize) as u32;
            }
        }
        for polygon in &mut self.convex_polygons {
            charge(work, polygon.vertices.len())?;
            for vertex in &mut polygon.vertices {
                *vertex = root(&mut parent, *vertex as usize) as u32;
            }
            polygon.vertices.dedup();
            if polygon.vertices.len() > 1 && polygon.vertices.first() == polygon.vertices.last() {
                polygon.vertices.pop();
            }
            if polygon.vertices.len() < 3 {
                polygon.vertices.clear();
            }
        }
        let supports = self.global_supports.as_mut().ok_or("convex stitching requires global support")?;
        for (i, family) in families.iter().enumerate() {
            if family.len() > 1 {
                let mut merged = Vec::new();
                for &alias in family {
                    charge(work, supports[alias].len())?;
                    merged.extend_from_slice(&supports[alias]);
                }
                merged.sort_unstable();
                merged.dedup();
                if merged.len() > MAX_BOOLEAN_CHANNELS {
                    return Err("convex alias exceeds 128 global support IDs".into());
                }
                let extra = merged.len().saturating_sub(supports[i].len());
                if extra > crate::meshing_local::MAX_LOCAL_SUPPORT_VALUES.saturating_sub(self.support_values) {
                    return Err("convex alias support storage exceeds its bound".into());
                }
                self.support_values += extra;
                supports[i] = merged;
            }
        }
        let supports = self.global_supports.as_ref().ok_or("convex stitching requires global support")?;
        let mut edges = BTreeMap::<(u32, u32), usize>::new();
        let mut by_plane_count = BTreeMap::<u64, usize>::new();
        let mut by_plane_domain = BTreeMap::<(u64, [u32; 4]), Vec<u32>>::new();
        for (i, planes) in supports.iter().enumerate() {
            if families[i].is_empty() {
                continue;
            }
            charge(work, planes.len() * 3 + 8)?;
            let mut domain = self.sources[i].nodes;
            domain.sort_unstable();
            // Aliases were formed only inside identical source-node buckets,
            // so this node set represents every witness in this family.
            for &id in planes {
                *by_plane_count.entry(id).or_default() += 1;
                by_plane_domain.entry((id, domain)).or_default().push(i as u32);
            }
        }
        for polygon in &self.convex_polygons {
            charge(work, polygon.vertices.len())?;
            for (i, &a) in polygon.vertices.iter().enumerate() {
                let b = polygon.vertices[(i + 1) % polygon.vertices.len()];
                *edges.entry((a.min(b), a.max(b))).or_default() += 1;
            }
        }
        let source_stitch_start = work.used;
        let mut cuts = BTreeMap::<(u32, u32), Vec<(f64, u32)>>::new();
        let mut deviation = 0.0_f64;
        for (&(a, b), &incidence) in &edges {
            if incidence != 1 {
                continue;
            }
            for &a_source in &families[a as usize] {
                for &b_source in &families[b as usize] {
                    let sa = &self.sources[a_source];
                    let sb = &self.sources[b_source];
                    let domain: Vec<_> = sa
                        .nodes
                        .into_iter()
                        .chain(sb.nodes)
                        .filter(|&i| i != u32::MAX)
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .collect();
                    if domain.len() > 4 || domain.len() < 2 {
                        continue;
                    }
                    let (Some(aa), Some(ab)) = (&sa.affine, &sb.affine) else {
                        continue;
                    };
                    let common: Vec<_> = aa
                        .planes
                        .iter()
                        .filter(|(id, _)| ab.planes.iter().any(|(other, _)| id == other))
                        .map(|(id, _)| *id)
                        .collect();
                    if common.len() < domain.len() - 2 {
                        continue;
                    }
                    let Some((&candidate_plane, _)) = common
                        .iter()
                        .filter_map(|id| by_plane_count.get(id).map(|count| (id, count)))
                        .min_by_key(|(_, count)| *count)
                    else {
                        continue;
                    };
                    let candidates = source_domain_candidates(candidate_plane, &domain, &by_plane_domain, work)?;
                    for middle in candidates {
                        charge(work, 16 + common.len())?;
                        if middle == a || middle == b {
                            continue;
                        }
                        if cuts.get(&(a, b)).is_some_and(|v| v.iter().any(|&(_, i)| i == middle)) {
                            continue;
                        }
                        for &middle_source in &families[middle as usize] {
                            let sm = &self.sources[middle_source];
                            let Some(am) = &sm.affine else {
                                continue;
                            };
                            if !common.iter().all(|id| am.planes.iter().any(|(other, _)| id == other)) {
                                continue;
                            }
                            if sm.nodes.iter().any(|&i| i != u32::MAX && !domain.contains(&i))
                                || !common.iter().all(|id| supports[middle as usize].contains(id))
                            {
                                continue;
                            }
                            // Original f32 coefficients certify the shared constraints
                            // have the rank of a line in this common source simplex.
                            let mut rank = false;
                            let mut affine_span = 0.0_f64;
                            let mut coordinate_scale = 0.0_f64;
                            for source in [sa, sb, sm] {
                                let Some(affine) = &source.affine else {
                                    continue;
                                };
                                if !domain.iter().all(|id| affine.tetra.contains(id)) {
                                    continue;
                                }
                                let columns: Vec<_> = domain
                                    .iter()
                                    .map(|id| affine.tetra.iter().position(|node| node == id).unwrap())
                                    .collect();
                                let rows: Vec<_> = common
                                    .iter()
                                    .filter_map(|id| {
                                        affine.planes.iter().find(|(plane, _)| plane == id).map(|(_, values)| *values)
                                    })
                                    .collect();
                                // More independent common rows constrain a
                                // point, not an edge. A small recovered endpoint
                                // separation must not turn that point into a
                                // stitched line under the residual allowance.
                                let mut point_rank = false;
                                if domain.len() == 3 {
                                    for i in 0..rows.len() {
                                        for j in i + 1..rows.len() {
                                            charge(work, 64)?;
                                            if exact_weights(&columns, &[i, j], &rows).is_ok() {
                                                point_rank = true;
                                                break;
                                            }
                                        }
                                        if point_rank {
                                            break;
                                        }
                                    }
                                } else if domain.len() == 4 {
                                    for i in 0..rows.len() {
                                        for j in i + 1..rows.len() {
                                            for k in j + 1..rows.len() {
                                                charge(work, 2048)?;
                                                if exact_weights(&columns, &[i, j, k], &rows).is_ok() {
                                                    point_rank = true;
                                                    break;
                                                }
                                            }
                                            if point_rank {
                                                break;
                                            }
                                        }
                                        if point_rank {
                                            break;
                                        }
                                    }
                                }
                                if point_rank {
                                    continue;
                                }
                                match domain.len() {
                                    2 => rank = true,
                                    3 => {
                                        rank = rows.iter().any(|row| columns.iter().any(|&i| row[i] != row[columns[0]]))
                                    }
                                    4 => {
                                        for i in 0..rows.len() {
                                            for j in i + 1..rows.len() {
                                                charge(work, 256)?;
                                                let values = [[1.; 4], rows[i], rows[j]];
                                                if (0..4).any(|column| {
                                                    !exact_cofactor(&columns, &[0, 1, 2], column, &values).is_empty()
                                                }) {
                                                    rank = true;
                                                    break;
                                                }
                                            }
                                            if rank {
                                                break;
                                            }
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                                if rank {
                                    for &column in &columns {
                                        let delta = sub(affine.points[column], affine.points[columns[0]]);
                                        affine_span += dot(delta, delta).sqrt();
                                        coordinate_scale =
                                            coordinate_scale.max(affine.points[column].iter().map(|x| x.abs()).sum());
                                    }
                                    break;
                                }
                            }
                            if !rank {
                                continue;
                            }
                            let weights = |source: &SourcePoint| -> Vec<f64> {
                                domain
                                    .iter()
                                    .map(|id| {
                                        source
                                            .nodes
                                            .iter()
                                            .position(|node| node == id)
                                            .map_or(0., |i| source.weights[i])
                                    })
                                    .collect()
                            };
                            let wa = weights(sa);
                            let wb = weights(sb);
                            let wm = weights(sm);
                            let axis = (0..domain.len())
                                .max_by(|&i, &j| (wb[i] - wa[i]).abs().total_cmp(&(wb[j] - wa[j]).abs()))
                                .unwrap();
                            let width = wb[axis] - wa[axis];
                            if width == 0. {
                                continue;
                            }
                            let t = (wm[axis] - wa[axis]) / width;
                            if !(0.0 < t && t < 1.0) {
                                continue;
                            }
                            // These are recovered barycentric coefficients, not world
                            // coordinate proximity. Keep the existing solver's residual
                            // contract, rejecting an inconsistent affine witness.
                            if (0..domain.len()).any(|i| (wm[i] - (wa[i] + t * (wb[i] - wa[i]))).abs() > 1e-10) {
                                return Err(
                                    "convex stitching has inconsistent original affine barycentric support".into()
                                );
                            }
                            let original_delta = sub(sm.point, add(sa.point, scale(sub(sb.point, sa.point), t)));
                            let source_bound = affine_span * 1e-10 + coordinate_scale * (128. * f64::EPSILON);
                            if dot(original_delta, original_delta).sqrt() > source_bound {
                                return Err("convex stitching exceeds its original-sample affine residual bound".into());
                            }
                            let pa = doubles(self.positions[a as usize]);
                            let pb = doubles(self.positions[b as usize]);
                            let pm = doubles(self.positions[middle as usize]);
                            let delta = sub(pm, add(pa, scale(sub(pb, pa), t)));
                            deviation = deviation.max(dot(delta, delta).sqrt());
                            cuts.entry((a, b)).or_default().push((t, middle));
                            break;
                        }
                    }
                }
            }
        }
        for list in cuts.values_mut() {
            list.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
            list.dedup_by_key(|x| x.1);
        }
        if self.face_source_normals.len() != self.triangles.len() {
            return Err("convex output lost original facet orientations".into());
        }
        let mut perimeters = Vec::with_capacity(self.convex_polygons.len());
        let mut splits = 0;
        for polygon in &self.convex_polygons {
            charge(work, polygon.vertices.len())?;
            let mut perimeter = Vec::new();
            for (i, &a) in polygon.vertices.iter().enumerate() {
                let b = polygon.vertices[(i + 1) % polygon.vertices.len()];
                perimeter.push(a);
                if let Some(list) = cuts.get(&(a.min(b), a.max(b))) {
                    charge(work, list.len())?;
                    let mut inserted: Vec<_> = list.iter().map(|&(_, id)| id).collect();
                    if a > b {
                        inserted.reverse();
                    }
                    splits += inserted.len();
                    perimeter.extend(inserted);
                }
            }
            perimeters.push(perimeter);
        }
        let before_vertices: BTreeSet<_> = perimeters.iter().flatten().copied().collect();
        let coalescing_start = work.used;
        let merged_polygons = coalesce_regions(&self.convex_polygons, &mut perimeters, work)?;
        let coalescing_end = work.used;
        let after_vertices: BTreeSet<_> = perimeters.iter().flatten().copied().collect();
        let coalesced_interior_vertices = before_vertices.len() - after_vertices.len();
        let redundant_boundary_vertices =
            remove_redundant_region_vertices(&mut perimeters, &self.sources, &source_rounding, work)?;
        let region_normals: Vec<_> = self.convex_polygons.iter().map(|p| p.normal).collect();
        let region_features: Vec<_> = self.convex_polygons.iter().map(|p| p.feature).collect();
        if let Some(policy) = representation_policy {
            let representation_start = work.used;
            let representation = representation::run(
                self,
                &perimeters,
                &region_features,
                &region_normals,
                &families,
                &source_rounding,
                policy,
                work,
            ).map_err(|error| format!("{error}; conformance phases: coordinate_pools {}, alias_and_index {}, source_stitching_and_perimeters {}, coalescing {}, redundancy {}, representation {} (start {representation_start}); nonempty_regions {}, perimeter_vertices {}, largest_perimeter {}", coordinate_pool_end-coordinate_pool_start, source_stitch_start-coordinate_pool_end, coalescing_start-source_stitch_start, coalescing_end-coalescing_start, representation_start-coalescing_end, work.used-representation_start, perimeters.iter().filter(|p| !p.is_empty()).count(), perimeters.iter().map(Vec::len).sum::<usize>(), perimeters.iter().map(Vec::len).max().unwrap_or(0)))?;
            charge(work, self.triangles.len() * 3)?;
            let used: BTreeSet<_> = self.triangles.iter().flatten().copied().collect();
            let rounded_vertices =
                used.iter().filter(|&&v| self.positions[v as usize] != original_positions[v as usize]).count();
            let source_rounding_displacement = representation.correspondence.max_displacement_m;
            return Ok(ConvexConformance {
                splits,
                aliases,
                deviation,
                source_collinear_faces: self.convex_source_collinear,
                exact_pool_corrections: pool_corrections,
                exact_pool_binary_steps: pool_binary_steps,
                merged_polygons,
                coalesced_interior_vertices,
                redundant_boundary_vertices,
                rounded_vertices,
                source_rounding_displacement,
                representation: Some(representation),
            });
        }
        let mut incidents = vec![Vec::new(); self.positions.len()];
        for (face, perimeter) in perimeters.iter().enumerate() {
            charge(work, perimeter.len())?;
            for &vertex in perimeter {
                incidents[vertex as usize].push(face);
            }
        }
        let mut collapsed_incidents = vec![Vec::new(); self.positions.len()];
        for (face, triangle) in self.collapsed.iter().enumerate() {
            charge(work, 3)?;
            for &vertex in triangle {
                collapsed_incidents[vertex as usize].push(face);
            }
        }
        let mut rounding_count = 0;
        let mut max_source_rounding = 0.0_f64;
        let face_valid = |face: usize, positions: &[Vec3], work: &mut WorkBudget| -> Result<bool, String> {
            if perimeters[face].is_empty() {
                return Ok(true);
            }
            charge(work, 10)?;
            Ok(convex_triangulation(&perimeters[face], positions, region_normals[face], work)?.is_some())
        };
        for (bad_face, bad_perimeter) in perimeters.iter().enumerate() {
            if face_valid(bad_face, &self.positions, work)? {
                continue;
            }
            let mut repaired = false;
            let mut candidate_count = 0;
            let mut cell_rejections = 0;
            let mut facet_rejections = 0;
            let mut collapsed_rejections = 0;
            let mut failed_neighbors = Vec::new();
            let mut candidate_pools = Vec::new();
            let vertices: BTreeSet<_> = bad_perimeter.iter().copied().collect();
            for vertex in vertices {
                charge(work, families[vertex as usize].len() * 24 + 8)?;
                let mut choices: [Vec<f32>; 3] = std::array::from_fn(|_| Vec::new());
                for (witness, &alias) in families[vertex as usize].iter().enumerate() {
                    for (axis, choices) in choices.iter_mut().enumerate() {
                        let pool = &source_rounding[alias];
                        let mut admissible = vec![pool.lower[axis]];
                        if pool.upper[axis] != pool.lower[axis] {
                            admissible.push(pool.upper[axis]);
                        }
                        if witness == 0 {
                            *choices = admissible;
                        } else {
                            choices.retain(|candidate| admissible.contains(candidate));
                        }
                    }
                }
                let mut candidates = Vec::new();
                for &x in &choices[0] {
                    for &y in &choices[1] {
                        for &z in &choices[2] {
                            let candidate = Vec3::new(x, y, z);
                            if ![x, y, z].into_iter().all(f32::is_finite)
                                || candidate == self.positions[vertex as usize]
                            {
                                continue;
                            }
                            let inside_cells = families[vertex as usize].iter().all(|&alias| {
                                self.sources[alias].affine.as_ref().is_some_and(|affine| {
                                    (0..3).all(|axis| {
                                        let low = affine.points.iter().map(|p| p[axis]).fold(f64::INFINITY, f64::min);
                                        let high =
                                            affine.points.iter().map(|p| p[axis]).fold(f64::NEG_INFINITY, f64::max);
                                        d_component(candidate, axis) >= low && d_component(candidate, axis) <= high
                                    })
                                })
                            });
                            if !inside_cells {
                                cell_rejections += 1;
                                continue;
                            }
                            let distance = families[vertex as usize]
                                .iter()
                                .map(|&alias| {
                                    let q = sub(doubles(candidate), source_rounding[alias].exact);
                                    dot(q, q)
                                })
                                .fold(0., f64::max);
                            candidates.push((distance, candidate));
                        }
                    }
                }
                candidates.sort_by(|a, b| {
                    a.0.total_cmp(&b.0).then_with(|| {
                        [a.1.x.to_bits(), a.1.y.to_bits(), a.1.z.to_bits()].cmp(&[
                            b.1.x.to_bits(),
                            b.1.y.to_bits(),
                            b.1.z.to_bits(),
                        ])
                    })
                });
                let previous = self.positions[vertex as usize];
                candidate_pools.push((vertex, candidates.clone()));
                for (distance, candidate) in candidates {
                    candidate_count += 1;
                    charge(work, incidents[vertex as usize].len() + collapsed_incidents[vertex as usize].len() * 10)?;
                    self.positions[vertex as usize] = candidate;
                    let mut valid = true;
                    for &face in &incidents[vertex as usize] {
                        if !face_valid(face, &self.positions, work)? {
                            facet_rejections += 1;
                            if failed_neighbors.len() < 8 {
                                failed_neighbors.push((vertex, face));
                            }
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        for &face in &collapsed_incidents[vertex as usize] {
                            let [a, b, c] = self.collapsed[face].map(|i| doubles(self.positions[i as usize]));
                            let normal = cross(sub(b, a), sub(c, a));
                            if dot(normal, normal) != 0. {
                                collapsed_rejections += 1;
                                valid = false;
                                break;
                            }
                        }
                    }
                    if valid {
                        for &alias in &families[vertex as usize] {
                            self.positions[alias] = candidate;
                        }
                        rounding_count += 1;
                        max_source_rounding = max_source_rounding.max(distance.sqrt());
                        repaired = true;
                        break;
                    }
                    self.positions[vertex as usize] = previous;
                }
                if repaired {
                    break;
                }
            }
            if !repaired {
                // Two adjacent near-boundary points can require simultaneous
                // directed rounding. This bounded atomic pair search retains
                // the same <=8 source-derived candidates per vertex.
                'pairs: for i in 0..candidate_pools.len() {
                    for j in i + 1..candidate_pools.len() {
                        let (a, first) = &candidate_pools[i];
                        let (b, second) = &candidate_pools[j];
                        charge(
                            work,
                            first.len() * second.len()
                                + incidents[*a as usize].len()
                                + incidents[*b as usize].len()
                                + collapsed_incidents[*a as usize].len()
                                + collapsed_incidents[*b as usize].len(),
                        )?;
                        let affected: BTreeSet<_> =
                            incidents[*a as usize].iter().chain(&incidents[*b as usize]).copied().collect();
                        let collapsed: BTreeSet<_> = collapsed_incidents[*a as usize]
                            .iter()
                            .chain(&collapsed_incidents[*b as usize])
                            .copied()
                            .collect();
                        let mut combinations: Vec<_> = first
                            .iter()
                            .flat_map(|&(da, pa)| second.iter().map(move |&(db, pb)| (da + db, da, db, pa, pb)))
                            .collect();
                        combinations.sort_by(|a, b| a.0.total_cmp(&b.0));
                        let previous = [self.positions[*a as usize], self.positions[*b as usize]];
                        for (_, da, db, pa, pb) in combinations {
                            charge(work, affected.len() + collapsed.len() * 10)?;
                            self.positions[*a as usize] = pa;
                            self.positions[*b as usize] = pb;
                            let mut valid = true;
                            for &face in &affected {
                                if !face_valid(face, &self.positions, work)? {
                                    valid = false;
                                    break;
                                }
                            }
                            if valid {
                                for &face in &collapsed {
                                    let [a, b, c] = self.collapsed[face].map(|i| doubles(self.positions[i as usize]));
                                    let n = cross(sub(b, a), sub(c, a));
                                    if dot(n, n) != 0. {
                                        valid = false;
                                        break;
                                    }
                                }
                            }
                            if valid {
                                for &alias in &families[*a as usize] {
                                    self.positions[alias] = pa;
                                }
                                for &alias in &families[*b as usize] {
                                    self.positions[alias] = pb;
                                }
                                rounding_count += 2;
                                max_source_rounding = max_source_rounding.max(da.max(db).sqrt());
                                repaired = true;
                                break 'pairs;
                            }
                            self.positions[*a as usize] = previous[0];
                            self.positions[*b as usize] = previous[1];
                        }
                    }
                }
            }
            if !repaired {
                return Err(format!("convex source-derived f32 rounding cannot preserve all incident facet orientations: face {bad_face}, {rounding_count} prior rounded vertices, {pool_corrections} corrected coordinate pools/{pool_binary_steps} exact bracket search steps, {candidate_count} candidates, {cell_rejections} cell/{facet_rejections} facet/{collapsed_rejections} collapsed rejections, neighbors {failed_neighbors:?}, perimeter {:?}",perimeters[bad_face].iter().take(16).map(|&i|(i,self.positions[i as usize],self.sources[i as usize].point)).collect::<Vec<_>>()));
            }
        }
        // Report the final edge deviation after all shared representation choices.
        for (&(a, b), list) in &cuts {
            for &(t, middle) in list {
                charge(work, 10)?;
                let pa = doubles(self.positions[a as usize]);
                let pb = doubles(self.positions[b as usize]);
                let pm = doubles(self.positions[middle as usize]);
                let delta = sub(pm, add(pa, scale(sub(pb, pa), t)));
                deviation = deviation.max(dot(delta, delta).sqrt());
            }
        }
        let mut triangles = Vec::new();
        let mut faces = Vec::new();
        let mut normals = Vec::new();
        let mut parent_faces = Vec::new();
        for (index, perimeter) in perimeters.iter().enumerate() {
            if perimeter.is_empty() {
                continue;
            }
            let pieces = convex_triangulation(perimeter, &self.positions, region_normals[index], work)?
                .ok_or("convex rounded perimeter lost its validated triangulation")?;
            for triangle in pieces {
                charge(work, 10)?;
                if triangles.len() >= self.limits.map_or(MAX_MESH_TRIANGLES, |v| v[1]) {
                    return Err("convex stitching exceeded triangle budget".into());
                }
                let feature = region_features[index];
                triangles.push(triangle);
                faces.push(feature);
                normals.push(region_normals[index]);
                parent_faces.push(index);
                for &vertex in &triangle {
                    let support = &mut self.global_supports.as_mut().unwrap()[vertex as usize];
                    if !support.contains(&feature) {
                        if support.len() >= MAX_BOOLEAN_CHANNELS
                            || self.support_values >= crate::meshing_local::MAX_LOCAL_SUPPORT_VALUES
                        {
                            return Err("convex split global support storage exceeds its bound".into());
                        }
                        support.push(feature);
                        support.sort_unstable();
                        self.support_values += 1;
                    }
                }
            }
        }
        self.triangles = triangles;
        self.face_features = Some(faces);
        self.face_source_normals = normals;
        charge(work, self.positions.len())?;
        let mut used = vec![false; self.positions.len()];
        let mut incidence = BTreeMap::<(u32, u32), Vec<usize>>::new();
        for (face, triangle) in self.triangles.iter().enumerate() {
            charge(work, 3)?;
            for &vertex in triangle {
                used[vertex as usize] = true;
            }
            for (a, b) in [(triangle[0], triangle[1]), (triangle[1], triangle[2]), (triangle[2], triangle[0])] {
                let entry = incidence.entry((a.min(b), a.max(b))).or_default();
                entry.push(face);
                if entry.len() > 2 {
                    return Err(format!(
                        "convex source triangulation has a nonmanifold edge ({a},{b}), positions {:?}, facets {:?}",
                        [self.positions[a as usize], self.positions[b as usize]],
                        entry
                            .iter()
                            .map(|&i| (
                                self.triangles[i],
                                self.face_features.as_ref().unwrap()[i],
                                parent_faces[i],
                                perimeters[parent_faces[i]]
                                    .iter()
                                    .take(16)
                                    .map(|&j| (j, self.positions[j as usize]))
                                    .collect::<Vec<_>>()
                            ))
                            .collect::<Vec<_>>()
                    ));
                }
            }
        }
        let mut changed_vertices = 0;
        for (vertex, &used) in used.iter().enumerate() {
            if used && self.positions[vertex] != original_positions[vertex] {
                changed_vertices += 1;
                charge(work, families[vertex].len() * 3)?;
                for &alias in &families[vertex] {
                    max_source_rounding = max_source_rounding
                        .max(source_displacement_bound(&source_rounding[alias], self.positions[vertex]));
                }
            }
        }
        Ok(ConvexConformance {
            splits,
            aliases,
            deviation,
            rounded_vertices: changed_vertices,
            source_rounding_displacement: max_source_rounding,
            source_collinear_faces: self.convex_source_collinear,
            exact_pool_corrections: pool_corrections,
            exact_pool_binary_steps: pool_binary_steps,
            merged_polygons,
            coalesced_interior_vertices,
            redundant_boundary_vertices,
            representation: None,
        })
    }

    fn polygon(
        &mut self,
        mut polygon: Vec<Vertex>,
        outward: bool,
        feature: u64,
        source_normal: Option<[f64; 3]>,
        work: &mut WorkBudget,
    ) -> Result<(), String> {
        // A leaf sign reversal changes the angular sorting frame but not the
        // geometric boundary. Normalize the final outward loop first, then
        // select the best worst-triangle fan using representable positions.
        // Canonical-key tie breaking makes equivalent sign representations use
        // the same diagonals without arbitrarily favoring a skinny polygon ear.
        if !outward {
            polygon.reverse();
        }
        // Small convex facets reuse rounded positions and edge calculations.
        // Triangles preserve their oriented corners under cyclic rotation;
        // their legacy floating-point scores need not select the same anchor.
        // Quads and larger polygons keep the established quality/tie rule.
        let small = source_normal.is_some() && (3..=MAX_CACHED_EAR_VERTICES).contains(&polygon.len());
        if polygon.len() == 3 && source_normal.is_some() {
            // A triangle has no diagonal choice. Keep a deterministic starting
            // key while avoiding three equivalent fan-quality evaluations.
            charge(work, 12)?;
            let anchor =
                (1..polygon.len()).fold(
                    0,
                    |best, candidate| {
                        if polygon[candidate].key < polygon[best].key {
                            candidate
                        } else {
                            best
                        }
                    },
                );
            polygon.rotate_left(anchor);
        } else {
            charge(work, polygon.len().saturating_mul(polygon.len()).saturating_mul(if small { 12 } else { 16 }))?;
            let small_scores = small.then(|| small_fan_scores(&polygon));
            let position = |i: usize| polygon[i % polygon.len()].point.map(|p| f64::from(p as f32));
            let quality = |anchor: usize| {
                if let Some(scores) = &small_scores {
                    return scores[anchor];
                }
                (1..polygon.len() - 1)
                    .map(|i| {
                        let a = position(anchor);
                        let b = position(anchor + i);
                        let c = position(anchor + i + 1);
                        let ab = sub(b, a);
                        let ac = sub(c, a);
                        let bc = sub(c, b);
                        let normal = cross(ab, ac);
                        let sum = dot(ab, ab) + dot(ac, ac) + dot(bc, bc);
                        if sum == 0.0 {
                            0.0
                        } else {
                            dot(normal, normal) / (sum * sum)
                        }
                    })
                    .fold(f64::INFINITY, f64::min)
            };
            let mut anchor = 0;
            let mut best = quality(0);
            for candidate in 1..polygon.len() {
                let score = quality(candidate);
                if score > best || (score == best && polygon[candidate].key < polygon[anchor].key) {
                    anchor = candidate;
                    best = score;
                }
            }
            polygon.rotate_left(anchor);
        }
        let source_points = source_normal.map(|_| polygon.iter().map(|v| v.source.clone()).collect::<Vec<_>>());
        let mut indices = Vec::new();
        for vertex in polygon {
            let index = if let Some(&index) = self.vertices.get(&vertex.key) {
                self.constraints[index as usize] |= vertex.planes;
                if let (Some(supports), Some(incoming)) = (&mut self.global_supports, &vertex.global_planes) {
                    charge(work, incoming.len())?;
                    let existing = &mut supports[index as usize];
                    let additional = incoming.iter().filter(|id| !existing.contains(id)).count();
                    if additional > crate::meshing_local::MAX_LOCAL_SUPPORT_VALUES.saturating_sub(self.support_values) {
                        return Err("local Boolean global support storage budget exceeded".into());
                    }
                    self.support_values += additional;
                    existing.extend(incoming);
                    existing.sort_unstable();
                    existing.dedup();
                    if existing.len() > MAX_BOOLEAN_CHANNELS {
                        return Err("local Boolean vertex exceeds 128 global support IDs".into());
                    }
                }
                index
            } else {
                if self.positions.len() >= self.limits.map_or(MAX_MESH_VERTICES, |limits| limits[0]) {
                    return Err("Boolean mesh vertex budget exceeded".into());
                }
                let index = self.positions.len() as u32;
                let p = vertex.point;
                self.positions.push(Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32));
                self.sources.push(vertex.source);
                self.constraints.push(vertex.planes);
                if let Some(supports) = &mut self.global_supports {
                    let incoming = vertex
                        .global_planes
                        .as_ref()
                        .ok_or("local Boolean vertex is missing global support identities")?;
                    charge(work, incoming.len())?;
                    if incoming.len()
                        > crate::meshing_local::MAX_LOCAL_SUPPORT_VALUES.saturating_sub(self.support_values)
                    {
                        return Err("local Boolean global support storage budget exceeded".into());
                    }
                    self.support_values += incoming.len();
                    supports.push(incoming.clone());
                }
                self.vertices.insert(vertex.key, index);
                index
            };
            indices.push(index);
        }
        let mut positive_source = false;
        for i in 1..indices.len() - 1 {
            let triangle = [indices[0], indices[i], indices[i + 1]];
            let [a, b, c] = triangle.map(|i| doubles(self.positions[i as usize]));
            let normal = cross(sub(b, a), sub(c, a));
            let exact_collinear = if let Some(sources) = &source_points {
                source_collinear([&sources[0], &sources[i], &sources[i + 1]], work)?
            } else {
                false
            };
            if exact_collinear || source_normal.is_none() && dot(normal, normal) == 0.0 {
                if self.collapsed.len() >= MAX_MESH_TRIANGLES {
                    return Err("Boolean collapsed-face budget exceeded".into());
                }
                self.collapsed.push(triangle);
                if exact_collinear {
                    self.convex_source_collinear += 1;
                }
            } else {
                positive_source = true;
                if self.triangles.len() >= self.limits.map_or(MAX_MESH_TRIANGLES, |limits| limits[1]) {
                    return Err("Boolean triangle budget exceeded".into());
                }
                self.triangles.push(triangle);
                if let Some(normal) = source_normal {
                    self.face_source_normals.push(normal);
                }
                if let Some(features) = &mut self.face_features {
                    features.push(feature);
                }
            }
        }
        if let (Some(_), Some(points)) = (source_normal, source_points) {
            if positive_source {
                let source = points[0].affine.as_ref().ok_or("convex polygon is missing original cell support")?;
                let plane = CellPlane::new(source, feature, outward, work)?;
                let normal = plane.outward_normal();
                self.convex_polygons.push(ConvexPolygon { vertices: indices, feature, normal, plane });
            }
        }
        Ok(())
    }
}

/// Factor only repeated rounded-point, edge-vector and edge-length evaluation.
/// Each score preserves the legacy operation order, including cyclic differences
/// in floating-point sums, so selecting a different diagonal is not an optimization.
fn small_fan_scores(polygon: &[Vertex]) -> Vec<f64> {
    let n = polygon.len();
    debug_assert!((3..=MAX_CACHED_EAR_VERTICES).contains(&n));
    let mut points = vec![[0.; 3]; n];
    for (i, vertex) in polygon.iter().enumerate() {
        points[i] = vertex.point.map(|v| f64::from(v as f32));
    }
    let mut edges = vec![vec![[0.; 3]; n]; n];
    let mut lengths = vec![vec![0.; n]; n];
    for a in 0..n {
        for b in 0..n {
            if a != b {
                edges[a][b] = sub(points[b], points[a]);
                lengths[a][b] = dot(edges[a][b], edges[a][b]);
            }
        }
    }
    let mut scores = vec![f64::INFINITY; n];
    for a in 0..n {
        for i in 1..n - 1 {
            let [b, c] = [(a + i) % n, (a + i + 1) % n];
            let normal = cross(edges[a][b], edges[a][c]);
            let sum = lengths[a][b] + lengths[a][c] + lengths[b][c];
            let score = if sum == 0. { 0. } else { dot(normal, normal) / (sum * sum) };
            scores[a] = scores[a].min(score);
        }
    }
    scores
}

pub(crate) struct WorkBudget {
    pub(crate) used: usize,
    pub(crate) structural: usize,
    pub(crate) extra_field_evaluations: usize,
    pub(crate) field_cost_per_callback: usize,
    pub(crate) limit: usize,
    pub(crate) exact_support_reductions: usize,
}

/// Cramer's-rule weights from original f32 samples plus the sum-to-one row.
/// Three f32 factors and at most 24 terms stay within normal finite f64 exponent
/// range, even for f32 subnormals. Each minor has at most12 expansion components;
/// the full determinant has at most48. The caller precharges the bounded work.
/// A zero determinant is rejected; only exact-zero cofactors may remove support.
fn exact_weights(local: &[usize], basis: &[usize], values: &[[f64; 4]]) -> Result<Vec<f64>, String> {
    if local.len() == 1 {
        return Ok(vec![1.0]);
    }
    for &row in basis {
        for &column in local {
            let value = values[row][column];
            if !value.is_finite() || f64::from(value as f32) != value {
                return Err("exact Boolean support requires original finite f32 samples".into());
            }
        }
    }
    let cofactors: Vec<_> = (0..local.len()).map(|column| exact_cofactor(local, basis, column, values)).collect();
    let mut determinant = Vec::new();
    for (i, cofactor) in cofactors.iter().enumerate() {
        for &component in cofactor {
            determinant = grow_expansion(&determinant, if i % 2 == 0 { component } else { -component });
        }
    }
    if determinant.is_empty() {
        return Err("Boolean original plane constraints have an exact zero determinant".into());
    }
    let denominator = determinant.iter().sum::<f64>();
    Ok(cofactors
        .iter()
        .enumerate()
        .map(|(i, cofactor)| {
            if cofactor.is_empty() {
                0.0
            } else {
                let numerator = cofactor.iter().sum::<f64>();
                (if i % 2 == 0 { numerator } else { -numerator }) / denominator
            }
        })
        .collect())
}
/// Sign of a queried plane at the rational constrained vertex. All coefficients
/// are original f32 samples. Four f32 factors remain inside finite normal f64
/// exponent range, and each multiplication retains its FMA residual.
fn exact_plane_sign(local: &[usize], basis: &[usize], values: &[[f64; 4]], query: [f64; 4]) -> Result<i8, String> {
    if basis.len() + 1 != local.len() {
        return Err("convex exact sign has inconsistent source rank".into());
    }
    let mut denominator = Vec::new();
    let mut numerator = Vec::new();
    for (column, &node) in local.iter().enumerate() {
        let cofactor = exact_cofactor(local, basis, column, values);
        let sign = if column % 2 == 0 { 1. } else { -1. };
        for component in cofactor {
            let component = sign * component;
            denominator = grow_expansion(&denominator, component);
            let product = component * query[node];
            let error = component.mul_add(query[node], -product);
            numerator = grow_expansion(&numerator, error);
            numerator = grow_expansion(&numerator, product);
        }
    }
    let Some(&denominator) = denominator.last() else {
        return Err("convex exact sign has a zero source determinant".into());
    };
    let Some(&numerator) = numerator.last() else {
        return Ok(0);
    };
    Ok(if numerator.is_sign_negative() == denominator.is_sign_negative() { 1 } else { -1 })
}
fn exact_cofactor(local: &[usize], basis: &[usize], column: usize, values: &[[f64; 4]]) -> Vec<f64> {
    let columns: Vec<_> =
        local.iter().enumerate().filter_map(|(i, &column_id)| (i != column).then_some(column_id)).collect();
    match columns.len() {
        0 => vec![1.0],
        1 => {
            let value = values[basis[0]][columns[0]];
            if value == 0.0 {
                Vec::new()
            } else {
                vec![value]
            }
        }
        2 => {
            let product = values[basis[0]][columns[0]] * values[basis[1]][columns[1]];
            grow_expansion(&grow_expansion(&[], product), -values[basis[0]][columns[1]] * values[basis[1]][columns[0]])
        }
        3 => {
            let matrix: [[f64; 3]; 3] = std::array::from_fn(|r| std::array::from_fn(|c| values[basis[r]][columns[c]]));
            let mut expansion = Vec::new();
            for (permutation, sign) in [
                ([0, 1, 2], 1.0),
                ([1, 2, 0], 1.0),
                ([2, 0, 1], 1.0),
                ([2, 1, 0], -1.0),
                ([1, 0, 2], -1.0),
                ([0, 2, 1], -1.0),
            ] {
                let product = matrix[0][permutation[0]] * matrix[1][permutation[1]];
                let third = matrix[2][permutation[2]];
                let high = product * third;
                let low = product.mul_add(third, -high);
                expansion = grow_expansion(&expansion, sign * low);
                expansion = grow_expansion(&expansion, sign * high);
            }
            expansion
        }
        _ => unreachable!("at most four simplex coordinates"),
    }
}
fn grow_expansion(expansion: &[f64], mut value: f64) -> Vec<f64> {
    let mut result = Vec::with_capacity(expansion.len() + 1);
    for &component in expansion {
        let sum = value + component;
        let virtual_component = sum - value;
        let virtual_value = sum - virtual_component;
        let error = (value - virtual_value) + (component - virtual_component);
        if error != 0.0 {
            result.push(error);
        }
        value = sum;
    }
    if value != 0.0 {
        result.push(value);
    }
    result
}
#[track_caller]
pub(crate) fn charge(work: &mut WorkBudget, amount: usize) -> Result<(), String> {
    if amount > work.limit - work.used {
        Err(format!("Boolean plane arrangement exceeds bounded work at {} (used {}, requested {}, limit {}, structural {}, extra_field_evaluations {}, field_cost {})", std::panic::Location::caller(), work.used, amount, work.limit, work.structural, work.extra_field_evaluations, work.field_cost_per_callback))
    } else {
        work.used += amount;
        work.structural += amount;
        Ok(())
    }
}
fn charge_field(work: &mut WorkBudget) -> Result<(), String> {
    if work.field_cost_per_callback > work.limit - work.used {
        return Err("Boolean extra field evaluations exceed bounded post-grid work".into());
    }
    work.used += work.field_cost_per_callback;
    work.extra_field_evaluations += 1;
    Ok(())
}
fn coords(p: Vec3) -> [f32; 3] {
    [p.x, p.y, p.z]
}
fn doubles(p: Vec3) -> [f64; 3] {
    [f64::from(p.x), f64::from(p.y), f64::from(p.z)]
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] - b[i])
}
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] + b[i])
}
fn scale(a: [f64; 3], s: f64) -> [f64; 3] {
    a.map(|v| v * s)
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

#[cfg(test)]
#[path = "meshing_endpoint_tests.rs"]
mod endpoint_tests;

#[cfg(test)]
#[path = "meshing_fan_tests.rs"]
mod fan_tests;

#[cfg(test)]
#[path = "meshing_source_basis_tests.rs"]
mod source_basis_tests;

#[cfg(test)]
mod solve_compare_tests {
    use super::*;

    #[test]
    fn edge_solver_matches_general_solver_bits() {
        let mut state = 0x9e37_79b9_u32;
        for _ in 0..10_000 {
            let mut values = [[0.0; 4]; 3];
            for row in &mut values {
                for value in row {
                    state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    *value = f64::from((state as f32 / 65_536.0) - 32_768.0);
                }
            }
            let local = [1, 3];
            let general = solve_weights(&local, &[0], &values).map(|weights| weights.map(f64::to_bits));
            let edge = solve_edge_weights(&local, 0, &values).map(|weights| weights.map(f64::to_bits));
            assert_eq!(general, edge, "values={values:?}");
        }
        let values = [[1.0, 1.0, 0.0, 0.0], [-1.0, 1.0, 0.0, 0.0], [0.0; 4]];
        let general = (0..2).find_map(|cut| solve_weights(&[0, 1], &[cut], &values));
        let edge = (0..2).find_map(|cut| solve_edge_weights(&[0, 1], cut, &values));
        assert_eq!(general.map(|weights| weights.map(f64::to_bits)), edge.map(|weights| weights.map(f64::to_bits)));
    }

    #[test]
    fn proportional_relation_cache_replays_the_exact_zero_decision() {
        let planes = [[-1.0, 1.0, 0.0, 0.0], [-2.0, 2.0, 0.0, 0.0]];
        let context = Context::new(
            [0, 1, 2, 3],
            [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            &planes,
            Some(&[11, 12]),
            true,
        );
        let mut vertex_work = WorkBudget {
            used: 0,
            structural: 0,
            extra_field_evaluations: 0,
            field_cost_per_callback: 1,
            limit: 100_000,
            exact_support_reductions: 0,
        };
        let vertex = context.vertex(12, 1, &mut vertex_work).unwrap();
        let mut first = WorkBudget {
            used: 0,
            structural: 0,
            extra_field_evaluations: 0,
            field_cost_per_callback: 1,
            limit: 100_000,
            exact_support_reductions: 0,
        };
        let first_value = context.value(&vertex, 1, &mut first).unwrap();
        let mut second = WorkBudget {
            used: 0,
            structural: 0,
            extra_field_evaluations: 0,
            field_cost_per_callback: 1,
            limit: 100_000,
            exact_support_reductions: 0,
        };
        let second_value = context.value(&vertex, 1, &mut second).unwrap();
        assert_eq!(first_value.to_bits(), second_value.to_bits());
        assert_eq!(first_value, 0.0);
        assert!(second.used < first.used);
    }
}

#[cfg(test)]
mod exact_support_tests {
    use super::*;
    #[test]
    fn cumulative_interval_displacement_cannot_reset_or_borrow_large_axis_precision() {
        let fixed =
            |p: [f32; 3]| SourceRounding { lower: p, upper: p, exact: p.map(f64::from), bounds: [p.map(f64::from); 2] };
        let a = fixed([0., 0., 0.]);
        let b = fixed([1e-7, 0., 0.]);
        let c = fixed([2e-7, 0., 0.]);
        let limit = 1.5e-7;
        assert!(source_displacement_bound(&a, Vec3::new(1e-7, 0., 0.)) < limit);
        assert!(source_displacement_bound(&b, Vec3::new(2e-7, 0., 0.)) < limit);
        assert!(
            [&a, &b, &c].into_iter().map(|p| source_displacement_bound(p, Vec3::new(2e-7, 0., 0.))).fold(0., f64::max)
                > limit
        );
        let low = 1e-8_f32;
        let source = SourceRounding {
            lower: [1e20, low, 0.],
            upper: [1e20, low.next_up(), 0.],
            exact: [f64::from(1e20_f32), f64::from(low), 0.],
            bounds: [[f64::from(1e20_f32), f64::from(low), 0.], [f64::from(1e20_f32), f64::from(low.next_up()), 0.]],
        };
        let bound = source_displacement_bound(&source, Vec3::new(1e20, low, 0.));
        assert!(bound >= f64::from(low.next_up()) - f64::from(low) && bound < 1e-12);
        assert_eq!(source_displacement_bound(&fixed([1e20, low, 0.]), Vec3::new(1e20, low, 0.)), 0.);
        let tiny = 2_f64.powi(-980);
        let tiny_source = SourceRounding {
            lower: [0.; 3],
            upper: [f32::from_bits(1), 0., 0.],
            exact: [tiny, 0., 0.],
            bounds: [[tiny, 0., 0.]; 2],
        };
        assert!(source_displacement_bound(&tiny_source, Vec3::ZERO) >= tiny);
    }
    fn coalescing_fixture(vertices: Vec<u32>, feature: u64, work: &mut WorkBudget) -> ConvexPolygon {
        let source = AffineSource {
            exact_basis: Default::default(),
            tetra: [0, 1, 3, 7],
            points: [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [1., 1., 1.]],
            basis: [feature, u64::MAX, u64::MAX],
            planes: vec![(feature, [-1., -1., -1., 1.])],
        };
        ConvexPolygon {
            vertices,
            feature,
            normal: [0., 0., 1.],
            plane: CellPlane::new(&source, feature, true, work).unwrap(),
        }
    }
    #[test]
    fn exact_region_coalescing_keeps_junctions_used_by_other_regions() {
        let mut work = WorkBudget {
            used: 0,
            structural: 0,
            extra_field_evaluations: 0,
            field_cost_per_callback: 1,
            limit: 64_000_000,
            exact_support_reductions: 0,
        };
        let mut polygons = vec![
            coalescing_fixture(vec![0, 1, 2, 4], 1, &mut work),
            coalescing_fixture(vec![0, 4, 2, 3], 1, &mut work),
        ];
        let mut perimeters: Vec<_> = polygons.iter().map(|p| p.vertices.clone()).collect();
        assert_eq!(coalesce_regions(&polygons, &mut perimeters, &mut work).unwrap(), 1);
        assert!(!perimeters.iter().flatten().any(|&v| v == 4));
        polygons.push(coalescing_fixture(vec![4, 5, 6], 2, &mut work));
        let mut perimeters: Vec<_> = polygons.iter().map(|p| p.vertices.clone()).collect();
        assert_eq!(coalesce_regions(&polygons, &mut perimeters, &mut work).unwrap(), 0);
        assert!(perimeters[0].contains(&4) && perimeters[1].contains(&4) && perimeters[2].contains(&4));
    }
    #[test]
    fn exact_region_coalescing_never_fills_an_annular_hole() {
        let mut work = WorkBudget {
            used: 0,
            structural: 0,
            extra_field_evaluations: 0,
            field_cost_per_callback: 1,
            limit: 64_000_000,
            exact_support_reductions: 0,
        };
        let polygons: Vec<_> = [vec![0, 1, 5, 4], vec![1, 2, 6, 5], vec![2, 3, 7, 6], vec![3, 0, 4, 7]]
            .into_iter()
            .map(|v| coalescing_fixture(v, 1, &mut work))
            .collect();
        let mut perimeters: Vec<_> = polygons.iter().map(|p| p.vertices.clone()).collect();
        let merges = coalesce_regions(&polygons, &mut perimeters, &mut work).unwrap();
        assert!(merges < 3);
        for (a, b) in [(4, 5), (5, 6), (6, 7), (7, 4)] {
            assert!(
                perimeters.iter().any(|p| p.iter().enumerate().any(|(i, &x)| x == b && p[(i + 1) % p.len()] == a)),
                "inner boundary edge{a}-{b} was filled"
            );
        }
    }
    #[test]
    fn exact_same_cell_plane_coalescing_rejects_lost_subnormal_coefficients() {
        let mut work = WorkBudget {
            used: 0,
            structural: 0,
            extra_field_evaluations: 0,
            field_cost_per_callback: 1,
            limit: 64_000_000,
            exact_support_reductions: 0,
        };
        let make = |tetra, points, values| AffineSource {
            exact_basis: Default::default(),
            tetra,
            points,
            basis: [1, u64::MAX, u64::MAX],
            planes: vec![(1, values)],
        };
        let first = make([0, 1, 3, 7], [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [1., 1., 1.]], [-2., -1., 1., 4.]);
        let second = make([0, 3, 2, 7], [[0., 0., 0.], [1., 1., 0.], [0., 1., 0.], [1., 1., 1.]], [-4., 2., 0., 8.]);
        let a = CellPlane::new(&first, 1, true, &mut work).unwrap();
        let b = CellPlane::new(&second, 1, true, &mut work).unwrap();
        assert!(a.same_oriented_plane(&b, &mut work).unwrap());
        assert!(!a.same_oriented_plane(&CellPlane::new(&second, 1, false, &mut work).unwrap(), &mut work).unwrap());
        let tiny = f64::from(f32::from_bits(1));
        let near = make(first.tetra, first.points, [1., tiny, -1., -1.]);
        let different = make(first.tetra, first.points, [1., 0., -1., -1.]);
        let ordinary = |v: [f64; 4]| [v[0], v[1] - v[0], v[2] - v[1], v[3] - v[2]];
        assert_eq!(ordinary(near.planes[0].1), ordinary(different.planes[0].1));
        let near = CellPlane::new(&near, 1, true, &mut work).unwrap();
        let different = CellPlane::new(&different, 1, true, &mut work).unwrap();
        assert!(!near.same_oriented_plane(&different, &mut work).unwrap());
    }
    include!("../tests/fixtures/exact_rounding_oracle.rs");
    include!("../tests/fixtures/source_rounding_oracle.rs");
    #[test]
    fn captured_source_cofactors_reproduce_independent_fraction_brackets() {
        let mut work = WorkBudget {
            used: 0,
            structural: 0,
            extra_field_evaluations: 0,
            field_cost_per_callback: 1,
            limit: 64_000_000,
            exact_support_reductions: 0,
        };
        for fixture in SOURCE_ROUNDING_ORACLE {
            let source = SourcePoint {
                point: fixture.point,
                nodes: fixture.nodes,
                weights: fixture.weights,
                affine: Some(std::sync::Arc::new(AffineSource {
                    exact_basis: Default::default(),
                    tetra: fixture.tetra,
                    points: fixture.points,
                    basis: fixture.basis,
                    planes: fixture.planes.to_vec(),
                })),
            };
            let coordinates = ExactCoordinate::all_from_source(&source, &mut work).unwrap();
            for (axis, coordinate) in coordinates.into_iter().enumerate() {
                let (actual, steps) = coordinate.interval(fixture.point[axis], &mut work).unwrap();
                assert_eq!(
                    actual.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    fixture.expected[axis],
                    "{} axis{axis}",
                    fixture.name
                );
                assert!(steps <= 32);
                assert!(actual.contains(&coordinate.nearest(&actual, &mut work).unwrap()));
            }
        }
    }
    #[test]
    fn exact_coordinate_brackets_match_independent_fraction_oracle() {
        let mut work = WorkBudget {
            used: 0,
            structural: 0,
            extra_field_evaluations: 0,
            field_cost_per_callback: 1,
            limit: 64_000_000,
            exact_support_reductions: 0,
        };
        for &(name, numerator, denominator, hint, expected) in ROUNDING_ORACLE {
            let coordinate = ExactCoordinate {
                numerator: numerator.to_vec(),
                denominator: denominator.to_vec(),
                exact_f32: std::cell::Cell::new(None),
            };
            for hint in [hint, 0.] {
                let (actual, steps) = coordinate.interval(hint, &mut work).unwrap();
                assert_eq!(actual.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), expected, "{name}, hint {hint}");
                assert!(steps <= 32);
            }
        }
    }
    #[test]
    fn source_identity_merges_equal_constraints_but_not_a_shared_f32_rounding() {
        let source = |id: u64, values: [f64; 2]| {
            let t = -values[0] / (values[1] - values[0]);
            SourcePoint {
                point: [10. + t, 0., 0.],
                nodes: [0, 1, u32::MAX, u32::MAX],
                weights: [1. - t, t, 0., 0.],
                affine: Some(std::sync::Arc::new(AffineSource {
                    exact_basis: Default::default(),
                    tetra: [0, 1, 2, 3],
                    points: [[10., 0., 0.], [11., 0., 0.], [10., 1., 0.], [10., 0., 1.]],
                    basis: [id, u64::MAX, u64::MAX],
                    planes: vec![(id, [values[0], values[1], 0., 0.])],
                })),
            }
        };
        let a = source(10, [-1., 1.]);
        let equal = source(11, [-2., 2.]);
        let distinct = source(12, [-1., f64::from(1.0_f32.next_up())]);
        assert_eq!(a.point.map(|v| v as f32), distinct.point.map(|v| v as f32));
        let mut work = WorkBudget {
            used: 0,
            structural: 0,
            extra_field_evaluations: 0,
            field_cost_per_callback: 1,
            limit: 1_000_000,
            exact_support_reductions: 0,
        };
        assert!(same_source_point(&a, &equal, &mut work).unwrap());
        assert!(!same_source_point(&a, &distinct, &mut work).unwrap());
        assert!(work.used > 0);
    }
    #[test]
    fn exact_plane_sign_preserves_zero_and_genuinely_tiny_nonzero_values() {
        let values = [[-1., 1., 0., 0.]];
        assert_eq!(exact_plane_sign(&[0, 1], &[0], &values, [-1., 1., 0., 0.]).unwrap(), 0);
        let tiny = f64::from(f32::from_bits(1));
        assert_eq!(exact_plane_sign(&[0, 1], &[0], &values, [tiny, 0., 0., 0.]).unwrap(), 1);
        assert_eq!(exact_plane_sign(&[0, 1], &[0], &values, [-tiny, 0., 0., 0.]).unwrap(), -1);
        assert_eq!(exact_plane_sign(&[1, 0], &[0], &values, [-tiny, 0., 0., 0.]).unwrap(), -1);
    }
    #[test]
    fn original_flat_support_face_has_exactly_zero_queried_plane_value() {
        let values = [
            [-0.007249993272125721, -0.007249994203448296, -0.007249994203448296, 0.019312487915158272],
            [-0.007249994203448296, -0.007249994203448296, -0.007249994203448296, 0.019312487915158272],
        ];
        assert_eq!(exact_plane_sign(&[1, 3], &[1], &values, values[0]).unwrap(), 0);
        assert_eq!(exact_plane_sign(&[2, 3], &[1], &values, values[0]).unwrap(), 0);
        assert_ne!(exact_plane_sign(&[0, 3], &[1], &values, values[0]).unwrap(), 0);
    }
    #[test]
    fn large_constant_coordinate_cannot_supply_small_axis_rounding_allowance() {
        let large = f64::from(1e20_f32);
        assert_eq!(f32_source_interval(large), vec![1e20_f32]);
        let small = 1e-8_f32;
        let ulp = f64::from(small.next_up()) - f64::from(small);
        let first = f32_source_interval(f64::from(small) + ulp * 0.25);
        let second = f32_source_interval(f64::from(small.next_up().next_up()) + ulp * 0.25);
        assert_eq!(first, vec![small, small.next_up()]);
        assert!(!first.contains(&0.));
        assert!(
            !first.iter().any(|v| second.contains(v)),
            "source intervals must remain disjoint despite the unrelated large coordinate"
        );
    }
    #[test]
    fn exact_certificate_rejects_a_zero_denominator() {
        let values = [[1., 2., 3., 4.], [2., 4., 6., 8.], [0., 1., 0., 1.]];
        assert!(exact_weights(&[0, 1, 2, 3], &[0, 1, 2], &values).unwrap_err().contains("zero determinant"));
    }
    #[test]
    fn genuinely_tiny_nonzero_support_survives_exact_zero_certification() {
        for small in [1e-20_f32, f32::MIN_POSITIVE, f32::from_bits(1)] {
            let values = [[-f64::from(small), 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]];
            let weights = exact_weights(&[0, 1, 2, 3], &[0, 1, 2], &values).unwrap();
            assert!(weights[1] > 0.0);
            assert_eq!(weights[1], f64::from(small));
            assert_eq!(weights[2], 0.0);
            assert_eq!(weights[3], 0.0);
        }
    }
    #[test]
    fn exact_cofactors_match_an_independent_integer_determinant() {
        fn det(m: [[i128; 3]; 3]) -> i128 {
            m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
        }
        let mut state = 0x31ffb10du64;
        for _ in 0..500 {
            let matrix: [[i128; 4]; 3] = std::array::from_fn(|_| {
                std::array::from_fn(|_| {
                    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    ((state >> 32) % 2_000_001) as i128 - 1_000_000
                })
            });
            let values = matrix.map(|row| row.map(|x| f64::from(x as f32)));
            let expected: [i128; 4] = std::array::from_fn(|column| {
                let cols: Vec<_> = (0..4).filter(|&i| i != column).collect();
                let value = det(std::array::from_fn(|r| std::array::from_fn(|c| matrix[r][cols[c]])));
                if column % 2 == 0 {
                    value
                } else {
                    -value
                }
            });
            let denominator = expected.iter().sum::<i128>();
            if denominator == 0 {
                assert!(exact_weights(&[0, 1, 2, 3], &[0, 1, 2], &values).is_err());
                continue;
            }
            let actual = exact_weights(&[0, 1, 2, 3], &[0, 1, 2], &values).unwrap();
            for i in 0..4 {
                let expected = expected[i] as f64 / denominator as f64;
                assert!((actual[i] - expected).abs() <= expected.abs().max(1.) * 5e-15);
            }
        }
    }
}

#[cfg(test)]
mod cross_cell_boundary_plane_tests {
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

    fn plane(cell: [u32; 2], lo: f64, hi: f64, constant: Vec<f64>, z: Vec<f64>, negative: bool) -> CellPlane {
        let mut plane = CellPlane {
            cell,
            bounds: [[0., 0., lo], [1., 1., hi]],
            coefficients: [constant, vec![], vec![], z],
            negative,
            boundary_axis: None,
        };
        plane.boundary_axis = plane.boundary_axis_plane(&mut work()).unwrap();
        plane
    }

    #[test]
    fn opposite_cell_endpoints_match_only_the_exact_oriented_plane() {
        let a = plane([0, 7], 0., 1., vec![-2.], vec![2.], true);
        let b = plane([8, 15], 1., 2., vec![], vec![8.], true);
        assert!(a.same_oriented_plane(&b, &mut work()).unwrap());

        let reversed = plane([8, 15], 1., 2., vec![], vec![-8.], false);
        assert!(a.same_oriented_plane(&reversed, &mut work()).unwrap());

        let inward = plane([8, 15], 1., 2., vec![], vec![8.], false);
        assert!(!a.same_oriented_plane(&inward, &mut work()).unwrap());

        let displaced = plane([8, 15], f64::from(1.0_f32.next_up()), 2., vec![], vec![8.], true);
        assert!(!a.same_oriented_plane(&displaced, &mut work()).unwrap());

        let tiny = f64::from(f32::from_bits(1));
        let near = plane([0, 7], 0., 1., grow_expansion(&[tiny], -1.), vec![1.], true);
        assert!(near.boundary_axis_plane(&mut work()).unwrap().is_none());

        let mut tilted = plane([8, 15], 1., 2., vec![], vec![8.], true);
        tilted.coefficients[1] = vec![tiny];
        assert!(tilted.boundary_axis_plane(&mut work()).unwrap().is_none());
    }

    #[test]
    fn exact_cross_cell_regions_keep_original_cancellation_and_feature_gates() {
        let make = |vertices: Vec<u32>, feature, cell| ConvexPolygon {
            vertices,
            feature,
            normal: [0., 0., 1.],
            plane: plane(cell, 0., 1., vec![], vec![1.], true),
        };

        let polygons = vec![make(vec![0, 1, 2, 3], 1, [0, 7]), make(vec![1, 4, 5, 2], 1, [8, 15])];
        let mut boundaries = polygons.iter().map(|p| p.vertices.clone()).collect::<Vec<_>>();
        assert_eq!(coalesce_regions(&polygons, &mut boundaries, &mut work()).unwrap(), 1);
        assert_eq!(boundaries[0], vec![0, 1, 4, 5, 2, 3]);
        assert!(boundaries[1].is_empty());

        let polygons = vec![make(vec![0, 1, 2, 3], 1, [0, 7]), make(vec![1, 4, 5, 2], 2, [8, 15])];
        let mut boundaries = polygons.iter().map(|p| p.vertices.clone()).collect::<Vec<_>>();
        assert_eq!(coalesce_regions(&polygons, &mut boundaries, &mut work()).unwrap(), 0);
    }
}

#[cfg(test)]
#[path = "exact_coordinate_tests.rs"]
mod coordinate_tests;

#[cfg(test)]
#[path = "ear_clipping_tests.rs"]
mod ear_tests;

#[cfg(test)]
#[path = "meshing_line_tests.rs"]
mod line_tests;

#[cfg(test)]
#[path = "meshing_relation_tests.rs"]
mod relation_tests;
