//! Bounded, indexed extraction of an evaluated scalar field's isosurface.
//!
//! Every cube uses the same six-tetrahedron decomposition. Intersections share a
//! vertex by global lattice-edge identity, including cube-face diagonals; exact
//! isovalues and roots rounded exactly onto a lattice endpoint share that vertex
//! itself. Triangles point toward increasing
//! field values (out of a negative-inside SDF). No primitive substitution,
//! smoothing, or projection changes the supplied field.
//!
//! This is a uniform, piecewise-linear sampling approximation, not exact CSG or
//! an adaptive feature detector. Components, cavities, and thin parts survive
//! when the lattice resolves their sign changes. Features or boundary crossings
//! entirely between samples can be missed. Compare successively finer grids and
//! independent field/surface samples for the intended delivery tolerance. Output
//! checks detect degenerate faces and bad edge incidence/orientation; they do not
//! establish freedom from all geometric self-intersections or vertex pinches.

use std::collections::HashMap;

use crate::Vec3;

/// Hard work/output bounds, checked before sampling and as geometry is emitted.
pub const MAX_GRID_CELLS: usize = 4_194_304;
pub const MAX_GRID_SAMPLES: usize = 4_500_000;
pub const MAX_MESH_VERTICES: usize = 2_000_000;
pub const MAX_MESH_TRIANGLES: usize = 4_000_000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeshingOptions {
    /// Number of cells per axis (there is one more sample per axis).
    pub resolution: [u32; 3],
    pub iso_level: f32,
    /// Permit an open mesh where sampled solid touches the box. No caps are added.
    pub allow_clipping: bool,
}

impl Default for MeshingOptions {
    fn default() -> Self {
        Self { resolution: [32; 3], iso_level: 0.0, allow_clipping: false }
    }
}

/// Raw sampling and mesh measurements for reporting and convergence comparison.
#[derive(Clone, Debug, PartialEq)]
pub struct MeshingMetadata {
    pub min: Vec3,
    pub max: Vec3,
    pub resolution: [u32; 3],
    pub spacing: Vec3,
    pub iso_level: f32,
    pub field_evaluations: usize,
    pub active_cells: usize,
    /// Unshifted minimum and maximum field samples over the complete lattice.
    pub field_range: [f32; 2],
    pub boundary_minimum: f32,
    /// True when at least one boundary sample is on or inside the isosurface.
    pub boundary_intersection: bool,
    pub clipping_allowed: bool,
    pub boundary_edges: usize,
    pub connected_components: usize,
    /// Grid-isovalue ties and rounded endpoints can produce zero-area faces;
    /// these are omitted, then the final edge incidence is checked.
    pub degenerate_triangles_removed: usize,
    /// Identical f32 positions joined only along collapsed tetrahedron faces.
    pub coincident_vertices_merged: usize,
    /// Existing adjacent triangles split at an exactly collinear middle vertex.
    pub collinear_faces_split: usize,
    /// Zero for scalar extraction; constituent channels for Boolean extraction.
    pub boolean_channels: usize,
    /// Counted post-grid structural work, including arrangement and refinement;
    /// excludes extra callbacks, whose count is reported separately below.
    pub boolean_arrangement_work: usize,
    /// Additional callback count; multiply by the scalar program's callback cost.
    pub boolean_extra_field_evaluations: usize,
    pub boolean_projected_vertices: usize,
    pub boolean_refinement_passes: usize,
    pub boolean_added_vertices: usize,
    pub boolean_added_triangles: usize,
    pub boolean_max_vertex_displacement: f32,
    pub surface_area: f64,
    /// Oriented triangle volume about the box center. A solid volume only when
    /// the mesh is closed and consistently oriented; cavities subtract volume.
    pub signed_volume: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub positions: Vec<Vec3>,
    pub triangles: Vec<[u32; 3]>,
    pub metadata: MeshingMetadata,
}

/// Extract the zero surface, rejecting sampled clipping at the given bounds.
pub fn extract_isosurface(
    field: impl Fn(Vec3) -> f32,
    min: Vec3,
    max: Vec3,
    resolution: [u32; 3],
) -> Result<Mesh, String> {
    extract_isosurface_with_options(field, min, max, MeshingOptions { resolution, ..MeshingOptions::default() })
}

/// Extract an indexed approximation from exactly `(nx+1)*(ny+1)*(nz+1)` samples.
///
/// Rejects invalid bounds/options, unrepresentable lattice spacing, work/output
/// budget excess, non-finite field values, no sampled surface, and inconsistent
/// mesh edges. In default mode every boundary field sample must exceed the
/// isovalue and every mesh edge must have two oppositely oriented incidents.
/// `allow_clipping` permits singly incident boundary edges but never adds caps.
pub fn extract_isosurface_with_options(
    field: impl Fn(Vec3) -> f32,
    min: Vec3,
    max: Vec3,
    options: MeshingOptions,
) -> Result<Mesh, String> {
    let lo = components(min);
    let hi = components(max);
    if lo.iter().chain(&hi).any(|v| !v.is_finite()) || (0..3).any(|i| lo[i] >= hi[i]) {
        return Err("meshing bounds must be finite and strictly increasing on every axis".into());
    }
    if !options.iso_level.is_finite() || options.resolution.contains(&0) {
        return Err("meshing requires a finite isovalue and nonzero cell resolution on every axis".into());
    }
    let n = options.resolution.map(|v| v as usize);
    let cells = checked_product(n)?;
    let nodes = n.map(|v| v.saturating_add(1));
    let samples = checked_product(nodes)?;
    if cells > MAX_GRID_CELLS || samples > MAX_GRID_SAMPLES {
        return Err(format!(
            "meshing grid exceeds budget: {cells} cells / {samples} samples; limits {MAX_GRID_CELLS} / {MAX_GRID_SAMPLES}"
        ));
    }
    let mut axes: [Vec<f32>; 3] = std::array::from_fn(|_| Vec::new());
    for i in 0..3 {
        axes[i] = (0..=n[i])
            .map(|j| (f64::from(lo[i]) + (f64::from(hi[i]) - f64::from(lo[i])) * j as f64 / n[i] as f64) as f32)
            .collect();
        if axes[i].windows(2).any(|w| w[0] >= w[1]) {
            return Err(format!("meshing lattice spacing is not representable as f32 on axis {i}"));
        }
    }
    let spacing = Vec3::new(
        ((f64::from(max.x) - f64::from(min.x)) / n[0] as f64) as f32,
        ((f64::from(max.y) - f64::from(min.y)) / n[1] as f64) as f32,
        ((f64::from(max.z) - f64::from(min.z)) / n[2] as f64) as f32,
    );
    if components(spacing).iter().any(|v| !v.is_finite() || *v <= 0.0) {
        return Err("meshing cell spacing is not finite and positive".into());
    }
    let lattice = Lattice { axes, nodes };
    let mut values = Vec::with_capacity(samples);
    let mut field_range = [f32::INFINITY, f32::NEG_INFINITY];
    let mut boundary_minimum = f32::INFINITY;
    for z in 0..=n[2] {
        for y in 0..=n[1] {
            for x in 0..=n[0] {
                let p = lattice.point(lattice.index(x, y, z));
                let value = field(p);
                if !value.is_finite() {
                    return Err(format!("non-finite field sample at ({}, {}, {})", p.x, p.y, p.z));
                }
                field_range[0] = field_range[0].min(value);
                field_range[1] = field_range[1].max(value);
                if x == 0 || y == 0 || z == 0 || x == n[0] || y == n[1] || z == n[2] {
                    boundary_minimum = boundary_minimum.min(value);
                }
                values.push(value);
            }
        }
    }
    let boundary_intersection = boundary_minimum <= options.iso_level;
    if boundary_intersection && !options.allow_clipping {
        return Err(format!(
            "isosurface touches or clips the sampled bounds (boundary minimum {boundary_minimum}, isovalue {}); expand bounds or explicitly allow clipping",
            options.iso_level
        ));
    }
    let mut builder = Builder {
        positions: Vec::new(),
        triangles: Vec::new(),
        edges: HashMap::new(),
        degenerate_triangles_removed: 0,
        degenerate_faces: Vec::new(),
        lattice: &lattice,
        values: &values,
        iso_level: options.iso_level,
    };
    // Freudenthal subdivision around 000--111. Neighboring cubes induce the
    // same diagonal on their shared face, independent of traversal direction.
    const TETS: [[usize; 4]; 6] = [[0, 1, 3, 7], [0, 3, 2, 7], [0, 2, 6, 7], [0, 6, 4, 7], [0, 4, 5, 7], [0, 5, 1, 7]];
    let mut active_cells = 0;
    for z in 0..n[2] {
        for y in 0..n[1] {
            for x in 0..n[0] {
                let cube =
                    std::array::from_fn::<_, 8, _>(|i| lattice.index(x + (i & 1), y + ((i >> 1) & 1), z + (i >> 2)));
                let inside = cube.iter().filter(|&&i| values[i] < options.iso_level).count();
                if inside == 0 || inside == 8 {
                    continue;
                }
                active_cells += 1;
                for tet in TETS {
                    builder.tetrahedron(tet.map(|i| cube[i]))?;
                }
            }
        }
    }
    let degenerate_triangles_removed = builder.degenerate_triangles_removed;
    drop(builder.edges);
    finish_mesh(
        builder.positions,
        builder.triangles,
        builder.degenerate_faces,
        MeshingMetadata {
            min,
            max,
            resolution: options.resolution,
            spacing,
            iso_level: options.iso_level,
            field_evaluations: samples,
            active_cells,
            field_range,
            boundary_minimum,
            boundary_intersection,
            clipping_allowed: options.allow_clipping,
            boundary_edges: 0,
            connected_components: 0,
            degenerate_triangles_removed,
            coincident_vertices_merged: 0,
            collinear_faces_split: 0,
            boolean_channels: 0,
            boolean_arrangement_work: 0,
            boolean_extra_field_evaluations: 0,
            boolean_projected_vertices: 0,
            boolean_refinement_passes: 0,
            boolean_added_vertices: 0,
            boolean_added_triangles: 0,
            boolean_max_vertex_displacement: 0.0,
            surface_area: 0.0,
            signed_volume: 0.0,
        },
    )
}

/// Shared geometric/topological completion for scalar and Boolean extraction.
pub(crate) fn finish_mesh(
    mut positions: Vec<Vec3>,
    mut triangles: Vec<[u32; 3]>,
    collapsed_faces: Vec<[u32; 3]>,
    mut metadata: MeshingMetadata,
) -> Result<Mesh, String> {
    if triangles.is_empty() {
        return Err("no nondegenerate isosurface was resolved by this grid; check bounds, field and resolution".into());
    }
    (metadata.coincident_vertices_merged, metadata.collinear_faces_split) =
        repair_collapsed_faces(&positions, &mut triangles, collapsed_faces)?;
    compact_vertices(&mut positions, &mut triangles);
    let origin = Vec3::new(
        (f64::from(metadata.min.x) * 0.5 + f64::from(metadata.max.x) * 0.5) as f32,
        (f64::from(metadata.min.y) * 0.5 + f64::from(metadata.max.y) * 0.5) as f32,
        (f64::from(metadata.min.z) * 0.5 + f64::from(metadata.max.z) * 0.5) as f32,
    );
    let audit = audit_mesh(&positions, &triangles, origin, metadata.clipping_allowed)?;
    metadata.boundary_edges = audit.boundary_edges;
    metadata.connected_components = audit.connected_components;
    metadata.surface_area = audit.surface_area;
    metadata.signed_volume = audit.signed_volume;
    Ok(Mesh { positions, triangles, metadata })
}

fn checked_product(values: [usize; 3]) -> Result<usize, String> {
    values.into_iter().try_fold(1usize, |a, b| a.checked_mul(b)).ok_or_else(|| "meshing grid size overflow".into())
}

struct Lattice {
    axes: [Vec<f32>; 3],
    nodes: [usize; 3],
}

impl Lattice {
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        (z * self.nodes[1] + y) * self.nodes[0] + x
    }

    fn point(&self, index: usize) -> Vec3 {
        Vec3::new(
            self.axes[0][index % self.nodes[0]],
            self.axes[1][(index / self.nodes[0]) % self.nodes[1]],
            self.axes[2][index / (self.nodes[0] * self.nodes[1])],
        )
    }
}

#[derive(Clone, Copy)]
struct Crossing {
    index: u32,
    /// The same edge's midpoint belongs to an equal-magnitude reference field.
    /// Its cap has the same orientation for this tetrahedron's sign pattern,
    /// without the cancellation of nearly coincident f32 output roots.
    orientation_point: [f64; 3],
}

struct Builder<'a> {
    positions: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    edges: HashMap<(usize, usize), u32>,
    degenerate_triangles_removed: usize,
    degenerate_faces: Vec<[u32; 3]>,
    lattice: &'a Lattice,
    values: &'a [f32],
    iso_level: f32,
}

impl Builder<'_> {
    fn intersection(&mut self, a: usize, b: usize) -> Result<Crossing, String> {
        let original_a = doubles(self.lattice.point(a));
        let original_b = doubles(self.lattice.point(b));
        let orientation_point = std::array::from_fn(|i| original_a[i] * 0.5 + original_b[i] * 0.5);
        let crossing = |index| Crossing { index, orientation_point };
        let (a, b) = if self.values[a] == self.iso_level {
            (a, a)
        } else if self.values[b] == self.iso_level {
            (b, b)
        } else if a < b {
            (a, b)
        } else {
            (b, a)
        };
        if let Some(&index) = self.edges.get(&(a, b)) {
            return Ok(crossing(index));
        }
        let pa = components(self.lattice.point(a));
        let pb = components(self.lattice.point(b));
        let t = if a == b {
            0.0
        } else {
            (f64::from(self.iso_level) - f64::from(self.values[a]))
                / (f64::from(self.values[b]) - f64::from(self.values[a]))
        };
        let p = std::array::from_fn::<_, 3, _>(|i| (f64::from(pa[i]) * (1.0 - t) + f64::from(pb[i]) * t) as f32);
        // Distinct edge roots can round to the same lattice endpoint in the
        // delivered f32 geometry even when that endpoint's scalar is not exactly
        // the isovalue. Give only these *exactly identical endpoint positions*
        // the lattice vertex identity. No distance tolerance merges nearby roots
        // or distinct sheets.
        let key = if p == pa {
            (a, a)
        } else if p == pb {
            (b, b)
        } else {
            (a, b)
        };
        if let Some(&index) = self.edges.get(&key) {
            return Ok(crossing(index));
        }
        if self.positions.len() >= MAX_MESH_VERTICES {
            return Err(format!("meshing vertex budget exceeded ({MAX_MESH_VERTICES})"));
        }
        let index = self.positions.len() as u32;
        self.positions.push(Vec3::new(p[0], p[1], p[2]));
        self.edges.insert(key, index);
        Ok(crossing(index))
    }

    fn tetrahedron(&mut self, nodes: [usize; 4]) -> Result<(), String> {
        let mut negative = [0; 4];
        let mut positive = [0; 4];
        let (mut ni, mut pi) = (0, 0);
        for node in nodes {
            if self.values[node] < self.iso_level {
                negative[ni] = node;
                ni += 1;
            } else {
                positive[pi] = node;
                pi += 1;
            }
        }
        if ni == 0 || ni == 4 {
            return Ok(());
        }
        // Every positive node lies above the interpolating plane and every
        // negative node below it, so this direction fixes outward winding.
        let center = |ids: &[usize]| {
            let mut sum = [0.0f64; 3];
            for &id in ids {
                let p = components(self.lattice.point(id));
                for i in 0..3 {
                    sum[i] += f64::from(p[i]) / ids.len() as f64;
                }
            }
            sum
        };
        let direction = sub(center(&positive[..pi]), center(&negative[..ni]));
        if ni == 1 {
            let tri = [
                self.intersection(negative[0], positive[0])?,
                self.intersection(negative[0], positive[1])?,
                self.intersection(negative[0], positive[2])?,
            ];
            self.triangle(tri, direction)?;
        } else if ni == 3 {
            let tri = [
                self.intersection(positive[0], negative[0])?,
                self.intersection(positive[0], negative[1])?,
                self.intersection(positive[0], negative[2])?,
            ];
            self.triangle(tri, direction)?;
        } else {
            let a = self.intersection(negative[0], positive[0])?;
            let b = self.intersection(negative[0], positive[1])?;
            let c = self.intersection(negative[1], positive[0])?;
            let d = self.intersection(negative[1], positive[1])?;
            self.triangle([a, b, c], direction)?;
            self.triangle([b, d, c], direction)?;
        }
        Ok(())
    }

    fn triangle(&mut self, crossings: [Crossing; 3], direction: [f64; 3]) -> Result<(), String> {
        let mut tri = crossings.map(|c| c.index);
        let [a, b, c] = tri.map(|i| doubles(self.positions[i as usize]));
        let normal = cross(sub(b, a), sub(c, a));
        if tri[0] == tri[1] || tri[1] == tri[2] || tri[2] == tri[0] || dot(normal, normal) == 0.0 {
            self.degenerate_triangles_removed += 1;
            if tri[0] != tri[1] && tri[1] != tri[2] && tri[2] != tri[0] {
                if self.degenerate_faces.len() >= MAX_MESH_TRIANGLES {
                    return Err("meshing collapsed-face repair budget exceeded".into());
                }
                self.degenerate_faces.push(tri);
            }
            return Ok(());
        }
        // Orientation is topological for a fixed tetrahedron sign pattern.
        // Testing the rounded output normal can reverse a tiny, otherwise
        // correctly connected face after a one-ULP coordinate displacement.
        let [a, b, c] = crossings.map(|crossing| crossing.orientation_point);
        if dot(cross(sub(b, a), sub(c, a)), direction) < 0.0 {
            tri.swap(1, 2);
        }
        if self.triangles.len() >= MAX_MESH_TRIANGLES {
            return Err(format!("meshing triangle budget exceeded ({MAX_MESH_TRIANGLES})"));
        }
        self.triangles.push(tri);
        Ok(())
    }
}

/// Reconcile topology lost when representable f32 positions collapse a face.
/// Equal coordinates are joined only when an actual collapsed triangle connects
/// them. Distinct collinear points stay distinct: subdivide the adjacent long-edge
/// triangle at the existing middle point, leaving its geometric surface unchanged.
/// No positional tolerance, projection, component removal or hole cap is used.
fn repair_collapsed_faces(
    positions: &[Vec3],
    triangles: &mut Vec<[u32; 3]>,
    collapsed: Vec<[u32; 3]>,
) -> Result<(usize, usize), String> {
    if collapsed.is_empty() {
        return Ok((0, 0));
    }
    let mut parents: Vec<usize> = (0..positions.len()).collect();
    let mut joined = 0;
    for &[a, b, c] in &collapsed {
        for (i, j) in [(a, b), (b, c), (c, a)] {
            if positions[i as usize] == positions[j as usize] {
                let ri = root(&mut parents, i as usize);
                let rj = root(&mut parents, j as usize);
                if ri != rj {
                    parents[ri] = rj;
                    joined += 1;
                }
            }
        }
    }
    for tri in triangles.iter_mut() {
        for index in tri {
            *index = root(&mut parents, *index as usize) as u32;
        }
    }
    // Candidate long edges index only the small collapsed-face neighborhood.
    let mut pending = Vec::new();
    let mut adjacency: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
    for tri in collapsed {
        let [a, b, c] = tri.map(|i| root(&mut parents, i as usize) as u32);
        if a == b || b == c || c == a {
            continue;
        }
        let distance = |i: u32, j: u32| {
            let d = sub(doubles(positions[i as usize]), doubles(positions[j as usize]));
            dot(d, d)
        };
        let ab = distance(a, b);
        let bc = distance(b, c);
        let ca = distance(c, a);
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
    for (index, &[a, b, c]) in triangles.iter().enumerate() {
        for (i, j) in [(a, b), (b, c), (c, a)] {
            if let Some(incidents) = adjacency.get_mut(&(i.min(j), i.max(j))) {
                incidents.push(index);
            }
        }
    }
    let mut split = 0;
    let mut attempts = 0usize;
    while !pending.is_empty() {
        let mut progress = false;
        let mut unresolved = Vec::new();
        for (key, middle) in pending {
            attempts += 1;
            if attempts > MAX_MESH_TRIANGLES * 8 {
                return Err("meshing collapsed-face neighborhood exceeds repair work budget".into());
            }
            let incidents = &adjacency[&key];
            if incidents.len() != 1 {
                unresolved.push((key, middle));
                continue;
            }
            let index = incidents[0];
            let original = triangles[index];
            let edge = (0..3)
                .find(|&i| {
                    let a = original[i];
                    let b = original[(i + 1) % 3];
                    (a.min(b), a.max(b)) == key
                })
                .expect("adjacency stores an incident edge");
            let [a, b, c] = [original[edge], original[(edge + 1) % 3], original[(edge + 2) % 3]];
            let replacement = [[a, middle, c], [middle, b, c]];
            if replacement.iter().any(|tri| {
                let [a, b, c] = tri.map(|i| doubles(positions[i as usize]));
                let normal = cross(sub(b, a), sub(c, a));
                dot(normal, normal) == 0.0
            }) {
                unresolved.push((key, middle));
                continue;
            }
            if triangles.len() >= MAX_MESH_TRIANGLES {
                return Err("meshing triangle budget exceeded during collapsed-face repair".into());
            }
            for (i, j) in [(original[0], original[1]), (original[1], original[2]), (original[2], original[0])] {
                if let Some(incidents) = adjacency.get_mut(&(i.min(j), i.max(j))) {
                    incidents.retain(|&face| face != index);
                }
            }
            triangles[index] = replacement[0];
            let appended = triangles.len();
            triangles.push(replacement[1]);
            for (face, [a, b, c]) in [(index, replacement[0]), (appended, replacement[1])] {
                for (i, j) in [(a, b), (b, c), (c, a)] {
                    if let Some(incidents) = adjacency.get_mut(&(i.min(j), i.max(j))) {
                        incidents.push(face);
                    }
                }
            }
            split += 1;
            progress = true;
        }
        if !progress {
            // The final strict incidence audit determines whether unresolved
            // collapsed neighborhoods are harmless or must reject this mesh.
            break;
        }
        pending = unresolved;
    }
    Ok((joined, split))
}

fn compact_vertices(positions: &mut Vec<Vec3>, triangles: &mut [[u32; 3]]) {
    let mut remap = vec![u32::MAX; positions.len()];
    for tri in triangles.iter() {
        for &index in tri {
            remap[index as usize] = 0;
        }
    }
    let mut count = 0;
    for i in 0..positions.len() {
        if remap[i] != u32::MAX {
            remap[i] = count as u32;
            positions[count] = positions[i];
            count += 1;
        }
    }
    positions.truncate(count);
    for tri in triangles {
        for index in tri {
            *index = remap[*index as usize];
        }
    }
}

struct Audit {
    boundary_edges: usize,
    connected_components: usize,
    surface_area: f64,
    signed_volume: f64,
}

fn audit_mesh(positions: &[Vec3], triangles: &[[u32; 3]], origin: Vec3, allow_clipping: bool) -> Result<Audit, String> {
    let mut edges: HashMap<(u32, u32), (u32, i32)> = HashMap::new();
    let mut parents: Vec<usize> = (0..positions.len()).collect();
    let mut area = 0.0;
    let mut volume = 0.0;
    for &[a, b, c] in triangles {
        for (i, j) in [(a, b), (b, c), (c, a)] {
            let (key, direction) = if i < j { ((i, j), 1) } else { ((j, i), -1) };
            let entry = edges.entry(key).or_default();
            entry.0 += 1;
            entry.1 += direction;
            let ri = root(&mut parents, i as usize);
            let rj = root(&mut parents, j as usize);
            parents[ri] = rj;
        }
        let [pa, pb, pc] = [a, b, c].map(|i| sub(doubles(positions[i as usize]), doubles(origin)));
        let normal = cross(sub(pb, pa), sub(pc, pa));
        area += dot(normal, normal).sqrt() * 0.5;
        volume += dot(pa, cross(pb, pc)) / 6.0;
    }
    let boundary_edges = edges.values().filter(|&&(n, _)| n == 1).count();
    let inconsistent = edges.values().filter(|&&(n, sum)| n > 2 || (n == 2 && sum != 0)).count();
    if inconsistent > 0 {
        return Err(format!("sampled isosurface has {inconsistent} nonmanifold or inconsistently oriented edges; adjust sampling or resolve singular field contacts"));
    }
    if boundary_edges > 0 && !allow_clipping {
        return Err(format!("sampled isosurface has {boundary_edges} open edges; adjust bounds/resolution to resolve degenerate intersections"));
    }
    let connected_components = (0..positions.len()).filter(|&i| root(&mut parents, i) == i).count();
    Ok(Audit { boundary_edges, connected_components, surface_area: area, signed_volume: volume })
}

fn root(parents: &mut [usize], mut i: usize) -> usize {
    while parents[i] != i {
        parents[i] = parents[parents[i]];
        i = parents[i];
    }
    i
}

fn components(p: Vec3) -> [f32; 3] {
    [p.x, p.y, p.z]
}

fn doubles(p: Vec3) -> [f64; 3] {
    [f64::from(p.x), f64::from(p.y), f64::from(p.z)]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
