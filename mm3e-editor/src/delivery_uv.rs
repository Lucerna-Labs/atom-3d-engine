//! Bounded attribute transfer onto the extracted composed field, never a native
//! midsurface substitute. Affine source seam-feature planes conformingly split
//! the existing faces. Additional refinement checks sampled, f32-stored UV error;
//! finite samples are not a general chart-boundary or missing-feature certificate.
use mm3e_kit::{surface::TriangleSurface, Vec3};
use mm3e_orchestrator::{sampling::CountedSceneField, Prim, Scene};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub struct TransferOptions {
    pub max_uv_error_texels: f64,
    pub max_refinement_passes: u32,
    pub max_work: usize,
    pub max_vertices: usize,
    pub max_triangles: usize,
}
pub struct TransferredMesh {
    pub positions: Vec<Vec3>,
    pub triangles: Vec<[u32; 3]>,
    pub material_ids: Vec<usize>,
    /// Values already rounded through f32 and restored to f64 for the caller.
    pub corner_uvs: Vec<[[f64; 2]; 3]>,
    pub report: Value,
}
type P = [f64; 3];
type Uv = [f64; 2];
type SourceMap = BTreeMap<usize, Source>;
type SourceSetup = (SourceMap, Vec<Plane>, usize);
type SourceEdges = BTreeMap<(u32, u32), Vec<(u32, [Uv; 2])>>;
#[derive(Clone, Copy)]
struct Plane {
    normal: P,
    offset: f64,
}
struct Source {
    dimensions: [f64; 2],
    periods: [Option<f64>; 2],
    triangles: BTreeMap<u32, TriangleSurface>,
}
struct Budget {
    used: usize,
    maximum: usize,
    coordinate_queries: usize,
    coincident_merges: usize,
    collinear_splits: usize,
    native_tie_checks: usize,
    native_tie_candidates: usize,
    periodic_rebase_checks: usize,
    periodic_rebased_corners: usize,
}
impl Budget {
    fn charge(&mut self, amount: usize) -> Result<(), String> {
        if amount > self.maximum.saturating_sub(self.used) {
            return Err("USD UV transfer exhausted its aggregate work budget".into());
        }
        self.used += amount;
        Ok(())
    }
    fn remaining(&self) -> usize {
        self.maximum - self.used
    }
}

pub fn transfer(
    scene: &Scene,
    positions: &[Vec3],
    triangles: &[[u32; 3]],
    material_map: &BTreeMap<u32, usize>,
    options: TransferOptions,
) -> Result<TransferredMesh, String> {
    if !options.max_uv_error_texels.is_finite()
        || options.max_uv_error_texels <= 0.0
        || options.max_refinement_passes > 12
        || options.max_work == 0
    {
        return Err("USD UV transfer requires positive finite texel error/work and at most 12 refinement passes".into());
    }
    if positions.len() > options.max_vertices || triangles.len() > options.max_triangles {
        return Err("USD UV input exceeds aggregate geometry budgets".into());
    }
    let mut budget = Budget {
        used: 0,
        maximum: options.max_work,
        coordinate_queries: 0,
        coincident_merges: 0,
        collinear_splits: 0,
        native_tie_checks: 0,
        native_tie_candidates: 0,
        periodic_rebase_checks: 0,
        periodic_rebased_corners: 0,
    };
    budget.charge(positions.len().saturating_add(triangles.len()))?;
    scene.validate_appearance()?;
    for triangle in triangles {
        valid_triangle(positions, *triangle)?;
    }
    let input_closed = edge_incidents(triangles, &mut budget)?.values().all(|faces| faces.len() == 2);
    let (mut sources, planes, seam_edges) = sources(scene, &mut budget)?;
    let mut output_positions = positions.to_vec();
    let mut output_triangles = triangles.to_vec();
    let mut max_plane_rounding = 0.0_f64;
    for plane in &planes {
        output_triangles = split_plane(
            &mut output_positions,
            &output_triangles,
            *plane,
            &options,
            &mut budget,
            &mut max_plane_rounding,
        )?;
    }
    let field = CountedSceneField::new(scene)?;
    // Subtraction never replaces its accumulated base material owner. This is
    // a structural proof, independent of distances, UVs, and triangle count.
    let fixed_owner = scene
        .objects
        .first()
        .filter(|_| {
            scene.objects.iter().skip(1).all(|object| matches!(object.combine, mm3e_orchestrator::Combine::Subtract))
        })
        .map(|object| (0, object.mat));
    let mut passes = 0;
    let mut probes = 0usize;
    let mut peak_error = 0.0_f64;
    let mut peak_raw_error = 0.0_f64;
    loop {
        if input_closed {
            output_triangles = repair_collinear_boundaries(&output_positions, output_triangles, &options, &mut budget)?;
        }
        let mut failed = BTreeSet::new();
        let mut material_ids = Vec::with_capacity(output_triangles.len());
        let mut corner_uvs = Vec::with_capacity(output_triangles.len());
        let mut maximum = 0.0_f64;
        let mut maximum_raw = 0.0_f64;
        let mut reason = String::new();
        let mut worst_context = String::new();
        for (index, &triangle) in output_triangles.iter().enumerate() {
            let mut face_maximum = 0.0_f64;
            let mut face_raw = 0.0_f64;
            let mut face_context = String::new();
            let mut validation = Vec::new();
            let mut owner_failed = false;
            let points = triangle.map(|i| output_positions[i as usize]);
            let center = point(points, [1.0 / 3.0; 3])?;
            let (owner_index, owner_material) = ownership(&field, center, fixed_owner, &mut budget)?;
            probes += 1;
            let material =
                *material_map.get(&owner_material).ok_or("USD UV sample material is outside export selection")?;
            let textured = sources.contains_key(&owner_index);
            let mut uv = [[0.0; 2]; 3];
            if textured {
                for corner in 0..3 {
                    let inset = one_sided_point(points, std::array::from_fn(|i| if i == corner { 1.0 } else { 0.0 }))?;
                    let exact = boundary_uv(
                        scene,
                        &mut sources,
                        owner_index,
                        points[corner],
                        inset,
                        options.max_uv_error_texels,
                        &mut budget,
                    )?;
                    uv[corner] = exact.map(|v| f64::from(v as f32));
                    let source = &sources[&owner_index];
                    validation.push((std::array::from_fn(|i| if i == corner { 1.0 } else { 0.0 }), exact));
                    face_raw = face_raw.max(texel_error(exact, uv[corner], source.dimensions, [None; 2]));
                    let error = texel_error(exact, uv[corner], source.dimensions, source.periods);
                    if !error.is_finite() || error > options.max_uv_error_texels {
                        return Err(
                            "USD UV coordinates cannot be represented in f32 within the requested texel error".into()
                        );
                    }
                    face_maximum = face_maximum.max(error);
                }
            }
            // Strictly interior points choose a side at exact material/UV seam
            // edges. Additional boundary samples use that face's one-sided map.
            for weights in [
                [1.0 / 3.0; 3],
                [0.6, 0.2, 0.2],
                [0.2, 0.6, 0.2],
                [0.2, 0.2, 0.6],
                [0.499, 0.499, 0.002],
                [0.002, 0.499, 0.499],
                [0.499, 0.002, 0.499],
            ] {
                let p = point(points, weights)?;
                let sample = ownership(&field, p, fixed_owner, &mut budget)?;
                probes += 1;
                if sample != (owner_index, owner_material) {
                    owner_failed = true;
                    failed.insert(index);
                    reason = "unresolved material-owner boundary".into();
                    continue;
                }
                if textured {
                    let (source_triangle, exact, _) = nearest_uv(scene, owner_index, p, &mut budget)?;
                    let reconstructed = interpolate(uv, weights);
                    validation.push((weights, exact));
                    let source = &sources[&owner_index];
                    face_raw = face_raw.max(texel_error(exact, reconstructed, source.dimensions, [None; 2]));
                    let error = texel_error(exact, reconstructed, source.dimensions, source.periods);
                    if error > face_maximum {
                        face_context=format!("face {index} {triangle:?}, positions {points:?}, interior point {p:?}, source triangle {source_triangle}, UV {exact:?}, interpolated {reconstructed:?}, corners {uv:?}");
                    }
                    face_maximum = face_maximum.max(error);
                    if error > options.max_uv_error_texels {
                        failed.insert(index);
                        reason =
                            "unresolved curved/ambiguous UV seam or nonlinear closest-surface interpolation".into();
                    }
                }
            }
            if textured {
                for weights in [[0.5, 0.5, 0.0], [0.0, 0.5, 0.5], [0.5, 0.0, 0.5]] {
                    let p = point(points, weights)?;
                    let inset = one_sided_point(points, weights)?;
                    let exact = boundary_uv(
                        scene,
                        &mut sources,
                        owner_index,
                        p,
                        inset,
                        options.max_uv_error_texels,
                        &mut budget,
                    )?;
                    let source = &sources[&owner_index];
                    validation.push((weights, exact));
                    face_raw = face_raw.max(texel_error(exact, interpolate(uv, weights), source.dimensions, [None; 2]));
                    let error = texel_error(exact, interpolate(uv, weights), source.dimensions, source.periods);
                    if error > face_maximum {
                        face_context=format!("face {index} {triangle:?}, positions {points:?}, boundary point {p:?}, inset {inset:?}, UV {exact:?}, interpolated {:?}, corners {uv:?}",interpolate(uv,weights));
                    }
                    face_maximum = face_maximum.max(error);
                    if error > options.max_uv_error_texels {
                        failed.insert(index);
                        reason = "unresolved one-sided UV edge interpolation".into();
                    }
                }
            }
            if textured
                && !owner_failed
                && failed.contains(&index)
                && sources[&owner_index].periods.iter().any(Option::is_some)
            {
                let lift =
                    BoundaryLift { object: owner_index, points, center, uv, tolerance: options.max_uv_error_texels };
                if let Some((lifted, count)) = lift_periodic_uvs(scene, &mut sources, &lift, &mut budget)? {
                    let source = &sources[&owner_index];
                    let error = validation
                        .iter()
                        .map(|&(weights, exact)| {
                            texel_error(exact, interpolate(lifted, weights), source.dimensions, source.periods)
                        })
                        .fold(0.0, f64::max);
                    if error <= options.max_uv_error_texels {
                        uv = lifted;
                        failed.remove(&index);
                        face_maximum = error;
                        face_raw = validation
                            .iter()
                            .map(|&(weights, exact)| {
                                texel_error(exact, interpolate(lifted, weights), source.dimensions, [None; 2])
                            })
                            .fold(0.0, f64::max);
                        budget.periodic_rebased_corners += count;
                    }
                }
            }
            if face_maximum > maximum {
                worst_context = face_context;
            }
            maximum = maximum.max(face_maximum);
            maximum_raw = maximum_raw.max(face_raw);
            material_ids.push(material);
            corner_uvs.push(uv);
        }
        peak_error = peak_error.max(maximum);
        peak_raw_error = peak_raw_error.max(maximum_raw);
        if failed.is_empty() {
            let vertex_links = audit_vertex_links(&output_triangles, &mut budget)?;
            return Ok(TransferredMesh {
                report: json!({"charged_work":budget.used,"coordinate_probes":budget.coordinate_queries,"owner_probes":probes,
                    "owner_sampling":if fixed_owner.is_some(){"structural_seed_with_subtractors"}else{"counted_field_fold"},
                    "source_seam_edges":seam_edges,"seam_feature_planes":planes.len(),"refinement_passes":passes,
                    "input_vertices":positions.len(),"input_triangles":triangles.len(),
                    "vertices":output_positions.len(),"triangles":output_triangles.len(),
                    "allocated_vertices":output_positions.len(),"referenced_vertices":vertex_links,
                    "max_sampled_uv_error_texels":maximum,"peak_pre_refinement_uv_error_texels":peak_error,
                    "max_sampled_raw_uv_error_texels":maximum_raw,"peak_pre_refinement_raw_uv_error_texels":peak_raw_error,
                    "uv_error_metric":"Euclidean texel error modulo the period preserved by every bound map on each axis; unwrapped corner coordinates are retained",
                    "sampling_periods":sources.iter().map(|(&object,source)|json!({"object":object,"periods":source.periods,"texel_dimensions":source.dimensions})).collect::<Vec<_>>(),
                    "max_seam_plane_rounding_m":max_plane_rounding,
                    "coincident_intersections_merged":budget.coincident_merges,"manifold_vertex_links_checked":vertex_links,
                    "collinear_neighbor_faces_split":budget.collinear_splits,"endpoint_snap_count":0,"max_endpoint_snap_distance_m":0.0,
                    "native_distance_tie_checks":budget.native_tie_checks,"native_distance_tie_candidates":budget.native_tie_candidates,
                    "periodic_rebase_checks":budget.periodic_rebase_checks,"periodic_rebased_corners":budget.periodic_rebased_corners,
                    "periodic_lift_scope":"Only failed faces: integer common-period corner shifts toward the face-interior source triangle, which must also tie the native boundary winner; every original validation probe is rerun. Unique-source winding is retained.",
                    "native_distance_tie_scope":"Only when inward position is unrepresentable: exhaustive source-triangle tie check at the same native f32 distance/query; original winner retained; no continuum or unseen-side guarantee",
                    "uv_storage":"faceVarying; checked after f32 cast/readback",
                    "geometry":"conforming splits of extracted composed faces; no source midsurface substitution",
                    "scope":"Affine seam feature planes plus bounded sampled interpolation checks; not a general chart-boundary or missing-feature certificate"}),
                positions: output_positions,
                triangles: output_triangles,
                material_ids,
                corner_uvs,
            });
        }
        if passes == options.max_refinement_passes {
            return Err(format!(
                "USD UV transfer rejected {reason} after {passes} refinement passes; sampled error {maximum} texels"
            ));
        }
        output_triangles = refine(&mut output_positions, &output_triangles, &failed, &options, &mut budget)
            .map_err(|error| format!("{error}; {reason}, sampled error {maximum} texels; {worst_context}"))?;
        passes += 1;
    }
}

fn ownership(
    field: &CountedSceneField<'_>,
    point: Vec3,
    fixed: Option<(usize, u32)>,
    budget: &mut Budget,
) -> Result<(usize, u32), String> {
    if let Some(owner) = fixed {
        budget.charge(1)?;
        return Ok(owner);
    }
    let owner = field.sample_owner(point, budget.remaining())?;
    budget.charge(owner.work)?;
    Ok((owner.owner.ok_or("USD UV sample has no material owner")?, owner.field.mat))
}

fn sources(scene: &Scene, budget: &mut Budget) -> Result<SourceSetup, String> {
    let mut sources = BTreeMap::new();
    let mut planes = Vec::new();
    let mut seams = 0;
    for (object_index, object) in scene.objects.iter().enumerate() {
        let Some(binding) = scene.surface_texture(object_index) else { continue };
        if object.mods.mirror.iter().any(|value| *value)
            || object.mods.elongate != Vec3::ZERO
            || object.mods.repeat != Vec3::ZERO
            || object.mods.twist != 0.0
            || object.mods.bend != 0.0
        {
            return Err("USD chart transfer currently requires affine textured object domains; bake domain warps into the authored surface first".into());
        }
        if scene.objects.iter().enumerate().any(|(i, other)| i != object_index && other.mat == object.mat) {
            return Err("USD textured owners require distinct material indices in the export selection".into());
        }
        let Prim::Surface { id } = object.prim else { return Err("USD textured owner is not a native surface".into()) };
        let surface = &scene.surfaces[id as usize];
        let maps = &binding.maps;
        let active_maps = [
            maps.albedo.as_ref(),
            maps.roughness.as_ref().map(|m| &m.map),
            maps.metallic.as_ref().map(|m| &m.map),
            maps.emissive.as_ref(),
            maps.normal.as_ref().map(|m| &m.map),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        let dimensions = active_maps.iter().fold([1.0_f64; 2], |size, map| {
            let d = map.image.dimensions();
            [size[0].max(f64::from(d[0])), size[1].max(f64::from(d[1]))]
        });
        let periods = std::array::from_fn(|axis| {
            use mm3e_kit::texture::Wrap;
            let mut period = 1.0;
            for map in &active_maps {
                match if axis == 0 { map.sampler.u } else { map.sampler.v } {
                    Wrap::Clamp => return None,
                    Wrap::Mirror => period = 2.0,
                    Wrap::Repeat => {}
                }
            }
            Some(period)
        });
        sources.insert(object_index, Source { dimensions, periods, triangles: BTreeMap::new() });
        let mut edges: SourceEdges = BTreeMap::new();
        for (i, tri) in surface.triangles().iter().enumerate() {
            budget.charge(4)?;
            let uv = binding.uv.corners(i as u32).ok_or("missing source UV corners")?;
            for (a, b) in [(0, 1), (1, 2), (2, 0)] {
                let (key, values) = if tri[a] < tri[b] {
                    ((tri[a], tri[b]), [uv[a], uv[b]])
                } else {
                    ((tri[b], tri[a]), [uv[b], uv[a]])
                };
                edges.entry(key).or_default().push((i as u32, values));
            }
        }
        for ((a, b), adjacent) in edges {
            if adjacent.len() != 2 || adjacent[0].1 == adjacent[1].1 {
                continue;
            }
            seams += 1;
            let start = d(surface.vertices()[a as usize]);
            let end = d(surface.vertices()[b as usize]);
            let mut normals = Vec::new();
            for &(triangle, _) in &adjacent {
                let points = surface.triangles()[triangle as usize].map(|i| d(surface.vertices()[i as usize]));
                let normal = unit(cross(sub(points[1], points[0]), sub(points[2], points[0])))?;
                normals.push(normal);
                // Face/edge Voronoi transitions and segment endpoint transitions
                // are affine planes. Keep discontinuous edges even when another
                // UV-continuous route keeps the chart graph connected (wraps).
                for (i, j) in [(0, 1), (1, 2), (2, 0)] {
                    let edge = unit(sub(points[j], points[i]))?;
                    add_plane(&mut planes, scene, object_index, cross(normal, edge), points[i], budget)?;
                    add_plane(&mut planes, scene, object_index, edge, points[i], budget)?;
                    add_plane(&mut planes, scene, object_index, edge, points[j], budget)?;
                }
            }
            add_plane(&mut planes, scene, object_index, add(normals[0], normals[1]), start, budget)?;
            add_plane(&mut planes, scene, object_index, sub(normals[0], normals[1]), start, budget)?;
            let edge = unit(sub(end, start))?;
            for vertex in [start, end] {
                add_plane(&mut planes, scene, object_index, edge, vertex, budget)?;
            }
        }
    }
    Ok((sources, planes, seams))
}
fn add_plane(
    planes: &mut Vec<Plane>,
    scene: &Scene,
    object: usize,
    normal: P,
    point: P,
    budget: &mut Budget,
) -> Result<(), String> {
    budget.charge(1)?;
    let magnitude = norm(normal);
    if magnitude <= 128.0 * f64::EPSILON {
        return Ok(());
    }
    let normal = scale(normal, 1.0 / magnitude);
    let object = &scene.objects[object];
    let world = std::array::from_fn(|i| {
        (0..3).map(|j| d(object.xform.rot.cols[j])[i] * normal[j] / f64::from(object.xform.scale)).sum()
    });
    let size = norm(world);
    let mut plane = Plane {
        normal: scale(world, 1.0 / size),
        offset: (dot(world, d(object.xform.pos)) + dot(normal, point)) / size,
    };
    let first = plane.normal.into_iter().find(|v| v.abs() > 1e-14).unwrap_or(1.0);
    if first < 0.0 {
        plane.normal = scale(plane.normal, -1.0);
        plane.offset = -plane.offset;
    }
    for old in planes.iter() {
        budget.charge(1)?;
        if norm(sub(old.normal, plane.normal)) < 1e-12
            && (old.offset - plane.offset).abs() < 1e-12 * (1.0 + plane.offset.abs())
        {
            return Ok(());
        }
    }
    planes.push(plane);
    Ok(())
}

fn nearest_uv(scene: &Scene, object: usize, p: Vec3, budget: &mut Budget) -> Result<(u32, Uv, f32), String> {
    budget.charge(2)?;
    budget.coordinate_queries += 1;
    let local = scene.surface_texture_local_point(object, p)?;
    let Prim::Surface { id } = scene.objects[object].prim else { return Err("missing UV surface".into()) };
    let hit = scene.surfaces[id as usize]
        .closest_hit_bounded(local, budget.remaining())
        .map_err(|error| format!("USD UV owner {object} nearest-source query {local:?}: {error}"))?;
    budget.charge(hit.work)?;
    let uv =
        scene.surface_texture(object).ok_or("missing UV binding")?.uv.interpolate(hit.triangle, hit.barycentric)?;
    Ok((hit.triangle, uv, hit.distance))
}
struct BoundaryLift {
    object: usize,
    points: [Vec3; 3],
    center: Vec3,
    uv: [Uv; 3],
    tolerance: f64,
}
fn lift_periodic_uvs(
    scene: &Scene,
    sources: &mut SourceMap,
    lift: &BoundaryLift,
    budget: &mut Budget,
) -> Result<Option<([Uv; 3], usize)>, String> {
    budget.periodic_rebase_checks += 1;
    let object = lift.object;
    let anchor = nearest_uv(scene, object, lift.center, budget)?.0;
    let source = sources.get_mut(&object).ok_or("missing periodic lift source")?;
    if let std::collections::btree_map::Entry::Vacant(entry) = source.triangles.entry(anchor) {
        budget.charge(4)?;
        let Prim::Surface { id } = scene.objects[object].prim else {
            return Err("missing periodic lift surface".into());
        };
        let surface = &scene.surfaces[id as usize];
        let vertices = surface.triangles()[anchor as usize].map(|i| surface.vertices()[i as usize]);
        entry.insert(TriangleSurface::new(vertices.to_vec(), vec![[0, 1, 2]], surface.half_thickness())?);
    }
    let mut result = lift.uv;
    let mut changed = 0;
    for (corner, p) in lift.points.into_iter().enumerate() {
        let (_, _, native_distance) = nearest_uv(scene, object, p, budget)?;
        budget.charge(2)?;
        let local = scene.surface_texture_local_point(object, p)?;
        let part = &source.triangles[&anchor];
        let distance = part.distance_bounded(local, budget.remaining())?;
        budget.charge(distance.work)?;
        if distance.distance != native_distance {
            continue;
        }
        let hit = part.closest_hit_bounded(local, budget.remaining())?;
        budget.charge(hit.work)?;
        budget.coordinate_queries += 1;
        let target = scene
            .surface_texture(object)
            .ok_or("missing periodic lift UVs")?
            .uv
            .interpolate(anchor, hit.barycentric)?;
        let candidate = std::array::from_fn(|axis| {
            let old = lift.uv[corner][axis];
            let shifted = if let Some(period) = source.periods[axis] {
                old + ((target[axis] - old) / period).round() * period
            } else {
                old
            };
            f64::from(shifted as f32)
        });
        if candidate != lift.uv[corner]
            && texel_error(candidate, target, source.dimensions, [None; 2]) <= lift.tolerance
            && texel_error(candidate, lift.uv[corner], source.dimensions, source.periods) <= lift.tolerance
        {
            result[corner] = candidate;
            changed += 1;
        }
    }
    Ok((changed > 0).then_some((result, changed)))
}

fn boundary_uv(
    scene: &Scene,
    sources: &mut SourceMap,
    object: usize,
    p: Vec3,
    inset: Option<Vec3>,
    tolerance: f64,
    budget: &mut Budget,
) -> Result<Uv, String> {
    if let Some(inset) = inset {
        return one_sided_uv(scene, sources, object, p, inset, budget);
    }
    let (_, winner, distance) = nearest_uv(scene, object, p, budget)?;
    budget.native_tie_checks += 1;
    let Prim::Surface { id } = scene.objects[object].prim else { return Err("missing UV tie-check surface".into()) };
    let surface = &scene.surfaces[id as usize];
    let local = scene.surface_texture_local_point(object, p)?;
    budget.charge(2)?;
    let source = sources.get_mut(&object).ok_or("missing UV tie-check source")?;
    for triangle in 0..surface.triangles().len() as u32 {
        budget.charge(1)?;
        if let std::collections::btree_map::Entry::Vacant(entry) = source.triangles.entry(triangle) {
            budget.charge(4)?;
            let vertices = surface.triangles()[triangle as usize].map(|i| surface.vertices()[i as usize]);
            entry.insert(TriangleSurface::new(vertices.to_vec(), vec![[0, 1, 2]], surface.half_thickness())?);
        }
        let part = &source.triangles[&triangle];
        let candidate = part.distance_bounded(local, budget.remaining())?;
        budget.charge(candidate.work)?;
        budget.native_tie_candidates += 1;
        if candidate.distance != distance {
            continue;
        }
        let hit = part.closest_hit_bounded(local, budget.remaining())?;
        budget.charge(hit.work)?;
        budget.coordinate_queries += 1;
        let uv = scene
            .surface_texture(object)
            .ok_or("missing UV tie-check binding")?
            .uv
            .interpolate(triangle, hit.barycentric)?;
        if texel_error(winner, uv, source.dimensions, source.periods) > tolerance {
            return Err(
                "USD UV unrepresentable one-sided sample has incompatible native-distance-tied source UVs".into()
            );
        }
    }
    Ok(winner)
}
fn one_sided_uv(
    scene: &Scene,
    sources: &mut BTreeMap<usize, Source>,
    object: usize,
    p: Vec3,
    inset: Vec3,
    budget: &mut Budget,
) -> Result<Uv, String> {
    let triangle = nearest_uv(scene, object, inset, budget)?.0;
    let source = sources.get_mut(&object).ok_or("missing UV transfer source")?;
    if let std::collections::btree_map::Entry::Vacant(entry) = source.triangles.entry(triangle) {
        budget.charge(4)?;
        let Prim::Surface { id } = scene.objects[object].prim else { return Err("missing UV source surface".into()) };
        let surface = &scene.surfaces[id as usize];
        let vertices = surface.triangles()[triangle as usize].map(|i| surface.vertices()[i as usize]);
        entry.insert(TriangleSurface::new(vertices.to_vec(), vec![[0, 1, 2]], surface.half_thickness())?);
    }
    budget.charge(2)?;
    budget.coordinate_queries += 1;
    let local = scene.surface_texture_local_point(object, p)?;
    let hit = source.triangles[&triangle]
        .closest_hit_bounded(local, budget.remaining())
        .map_err(|error| format!("USD UV owner {object} forced source triangle {triangle} query {local:?}: {error}"))?;
    budget.charge(hit.work)?;
    scene.surface_texture(object).ok_or("missing source UV")?.uv.interpolate(triangle, hit.barycentric)
}

fn split_plane(
    positions: &mut Vec<Vec3>,
    triangles: &[[u32; 3]],
    plane: Plane,
    options: &TransferOptions,
    budget: &mut Budget,
    max_rounding: &mut f64,
) -> Result<Vec<[u32; 3]>, String> {
    let mut result = Vec::new();
    let mut intersections = BTreeMap::new();
    let mut collapses = Vec::new();
    for &triangle in triangles {
        budget.charge(1)?;
        let distances = triangle.map(|i| dot(plane.normal, d(positions[i as usize])) - plane.offset);
        if !distances.iter().any(|&v| v > 0.0) || !distances.iter().any(|&v| v < 0.0) {
            push_triangle(&mut result, positions, triangle, options)?;
            continue;
        }
        for positive in [true, false] {
            let mut polygon = Vec::new();
            for edge in 0..3 {
                let next = (edge + 1) % 3;
                let a = triangle[edge];
                let b = triangle[next];
                let da = distances[edge];
                let db = distances[next];
                if if positive { da >= 0.0 } else { da <= 0.0 } {
                    polygon.push(a);
                }
                if (da > 0.0 && db < 0.0) || (da < 0.0 && db > 0.0) {
                    let key = (a.min(b), a.max(b));
                    let index = if let Some(&index) = intersections.get(&key) {
                        index
                    } else {
                        budget.charge(1)?;
                        let fraction = da / (da - db);
                        let position = v(add(
                            d(positions[a as usize]),
                            scale(sub(d(positions[b as usize]), d(positions[a as usize])), fraction),
                        ))?;
                        let index = if position == positions[a as usize] {
                            a
                        } else if position == positions[b as usize] {
                            b
                        } else {
                            add_vertex(positions, position, options)?
                        };
                        *max_rounding =
                            max_rounding.max((dot(plane.normal, d(positions[index as usize])) - plane.offset).abs());
                        intersections.insert(key, index);
                        index
                    };
                    polygon.push(index);
                }
            }
            polygon.dedup();
            if polygon.first() == polygon.last() {
                polygon.pop();
            }
            for i in 1..polygon.len().saturating_sub(1) {
                let triangle = [polygon[0], polygon[i], polygon[i + 1]];
                let [a, b, c] = triangle.map(|i| d(positions[i as usize]));
                if norm(cross(sub(b, a), sub(c, a))) == 0.0 {
                    collapses.push(triangle);
                } else {
                    push_triangle(&mut result, positions, triangle, options)?;
                }
            }
        }
    }
    repair_collapsed_faces(positions, result, &collapses, options, budget)
}

/// Canonicalize only exact coordinates connected by an actual collapsed output
/// edge. A global position weld could join unrelated touching components.
fn repair_collapsed_faces(
    positions: &[Vec3],
    triangles: Vec<[u32; 3]>,
    collapses: &[[u32; 3]],
    options: &TransferOptions,
    budget: &mut Budget,
) -> Result<Vec<[u32; 3]>, String> {
    if collapses.is_empty() {
        return Ok(triangles);
    }
    budget.charge(positions.len())?;
    let mut parent: Vec<u32> = (0..positions.len() as u32).collect();
    fn root(parent: &mut [u32], index: u32) -> u32 {
        let mut value = index;
        while parent[value as usize] != value {
            value = parent[value as usize];
        }
        let mut next = index;
        while parent[next as usize] != next {
            let old = parent[next as usize];
            parent[next as usize] = value;
            next = old;
        }
        value
    }
    for &[a, b, c] in collapses {
        for (a, b) in [(a, b), (b, c), (c, a)] {
            budget.charge(1)?;
            if positions[a as usize] == positions[b as usize] {
                let a = root(&mut parent, a);
                let b = root(&mut parent, b);
                if a != b {
                    parent[a.max(b) as usize] = a.min(b);
                    budget.coincident_merges += 1;
                }
            }
        }
    }
    let mut result = Vec::new();
    for triangle in triangles {
        budget.charge(3)?;
        let triangle = triangle.map(|i| root(&mut parent, i));
        if triangle[0] == triangle[1] || triangle[1] == triangle[2] || triangle[2] == triangle[0] {
            continue;
        }
        valid_triangle(positions, triangle)?;
        result.push(triangle);
    }
    // Distinct collinear positions stay distinct. Removing their zero-area
    // face exposes one long edge opposite two short edges; split the adjacent
    // long-edge face at the existing middle point to keep the mesh conforming.
    let mut pending = Vec::new();
    let mut adjacency = BTreeMap::<(u32, u32), Vec<usize>>::new();
    for &triangle in collapses {
        budget.charge(3)?;
        let [a, b, c] = triangle.map(|i| root(&mut parent, i));
        if a == b || b == c || c == a {
            continue;
        }
        let distance = |a: u32, b: u32| norm(sub(d(positions[a as usize]), d(positions[b as usize])));
        let (ab, bc, ca) = (distance(a, b), distance(b, c), distance(c, a));
        let (a, middle, b) = if ab >= bc && ab >= ca {
            (a, c, b)
        } else if bc >= ca {
            (b, a, c)
        } else {
            (c, b, a)
        };
        let key = (a.min(b), a.max(b));
        adjacency.entry(key).or_default();
        pending.push((key, middle));
    }
    for (face, &[a, b, c]) in result.iter().enumerate() {
        budget.charge(3)?;
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
            budget.charge(1)?;
            let incident = &adjacency[&key];
            if incident.len() != 1 {
                unresolved.push((key, middle));
                continue;
            }
            let index = incident[0];
            let original = result[index];
            let edge = (0..3)
                .find(|&i| {
                    let a = original[i];
                    let b = original[(i + 1) % 3];
                    (a.min(b), a.max(b)) == key
                })
                .ok_or("USD UV collapsed-edge adjacency became inconsistent")?;
            let [a, b, c] = [original[edge], original[(edge + 1) % 3], original[(edge + 2) % 3]];
            let replacement = [[a, middle, c], [middle, b, c]];
            if replacement.iter().any(|&tri| valid_triangle(positions, tri).is_err()) {
                unresolved.push((key, middle));
                continue;
            }
            if result.len() >= options.max_triangles {
                return Err("USD UV collinear repair exceeded triangle budget".into());
            }
            budget.charge(9)?;
            for (a, b) in [(original[0], original[1]), (original[1], original[2]), (original[2], original[0])] {
                if let Some(next) = adjacency.get_mut(&(a.min(b), a.max(b))) {
                    next.retain(|&face| face != index);
                }
            }
            result[index] = replacement[0];
            let appended = result.len();
            result.push(replacement[1]);
            for (face, [a, b, c]) in [(index, replacement[0]), (appended, replacement[1])] {
                for (a, b) in [(a, b), (b, c), (c, a)] {
                    if let Some(next) = adjacency.get_mut(&(a.min(b), a.max(b))) {
                        next.push(face);
                    }
                }
            }
            progress = true;
            budget.collinear_splits += 1;
        }
        if !progress {
            break;
        }
        pending = unresolved;
    }
    Ok(result)
}

fn edge_incidents(triangles: &[[u32; 3]], budget: &mut Budget) -> Result<BTreeMap<(u32, u32), Vec<usize>>, String> {
    let mut edges = BTreeMap::<(u32, u32), Vec<usize>>::new();
    for (face, &[a, b, c]) in triangles.iter().enumerate() {
        budget.charge(3)?;
        for (a, b) in [(a, b), (b, c), (c, a)] {
            let faces = edges.entry((a.min(b), a.max(b))).or_default();
            faces.push(face);
            if faces.len() > 2 {
                return Err("USD UV transfer created a nonmanifold edge".into());
            }
        }
    }
    Ok(edges)
}

/// Several collapsed faces may constrain different middle points on the same
/// original long edge. Reconcile the entire exact collinear boundary component
/// at once, subdividing each incident face by the same ordered vertex sequence.
/// No new position, proximity weld, nonzero-area removal, or hole cap is used.
fn repair_collinear_boundaries(
    positions: &[Vec3],
    triangles: Vec<[u32; 3]>,
    options: &TransferOptions,
    budget: &mut Budget,
) -> Result<Vec<[u32; 3]>, String> {
    let edges = edge_incidents(&triangles, budget)?;
    let mut neighbors = BTreeMap::<u32, Vec<u32>>::new();
    for (&(a, b), faces) in &edges {
        if faces.len() == 1 {
            neighbors.entry(a).or_default().push(b);
            neighbors.entry(b).or_default().push(a);
        }
    }
    if neighbors.is_empty() {
        return Ok(triangles);
    }
    let mut seen = BTreeSet::new();
    let mut aliases = BTreeMap::<u32, u32>::new();
    let mut replacements = BTreeMap::new();
    for &seed in neighbors.keys() {
        if seen.contains(&seed) {
            continue;
        }
        let mut pending = vec![seed];
        let mut component = Vec::new();
        while let Some(index) = pending.pop() {
            budget.charge(1)?;
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
            .ok_or("empty UV boundary component")?;
        let direction = sub(d(positions[far as usize]), origin);
        if norm(direction) == 0.0
            || component.iter().any(|&i| norm(cross(sub(d(positions[i as usize]), origin), direction)) != 0.0)
        {
            return Err(
                "USD UV transfer has a non-collinear boundary that cannot be repaired without changing geometry".into(),
            );
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
                    budget.coincident_merges += 1;
                    continue;
                }
            }
            ordered.push(index);
        }
        let component_set: BTreeSet<_> = component.iter().copied().collect();
        budget.charge(edges.len())?;
        for (&(i, j), faces) in &edges {
            if faces.len() != 1 || !component_set.contains(&i) {
                continue;
            }
            budget.charge(1)?;
            let face = faces[0];
            let original = triangles[face];
            let edge = (0..3)
                .find(|&a| {
                    let b = original[a];
                    let c = original[(a + 1) % 3];
                    (b.min(c), b.max(c)) == (i, j)
                })
                .ok_or("missing UV boundary incidence")?;
            let map = |i: u32| *aliases.get(&i).unwrap_or(&i);
            let (a, b, c) = (map(original[edge]), map(original[(edge + 1) % 3]), map(original[(edge + 2) % 3]));
            let (lo, hi) = (
                d(positions[a as usize])[axis].min(d(positions[b as usize])[axis]),
                d(positions[a as usize])[axis].max(d(positions[b as usize])[axis]),
            );
            let mut chain: Vec<_> = ordered
                .iter()
                .copied()
                .filter(|&index| {
                    let value = d(positions[index as usize])[axis];
                    value >= lo && value <= hi
                })
                .collect();
            if d(positions[a as usize])[axis] > d(positions[b as usize])[axis] {
                chain.reverse();
            }
            if chain.first() != Some(&a) || chain.last() != Some(&b) {
                return Err("USD UV boundary subdivision lost an endpoint".into());
            }
            if chain.len() > 2 {
                if replacements.contains_key(&face) {
                    return Err("USD UV boundary repair requires incompatible face subdivisions".into());
                }
                let pieces = chain.windows(2).map(|pair| [pair[0], pair[1], c]).collect::<Vec<_>>();
                budget.charge(pieces.len())?;
                budget.collinear_splits += pieces.len() - 1;
                replacements.insert(face, pieces);
            }
        }
    }
    let mut result = Vec::new();
    for (face, triangle) in triangles.into_iter().enumerate() {
        budget.charge(3)?;
        let pieces = replacements.remove(&face).unwrap_or_else(|| vec![triangle]);
        for triangle in pieces {
            let triangle = triangle.map(|i| *aliases.get(&i).unwrap_or(&i));
            push_triangle(&mut result, positions, triangle, options)?;
        }
    }
    if edge_incidents(&result, budget)?.values().any(|faces| faces.len() != 2) {
        return Err("USD UV exact collinear boundary subdivision did not restore closed incidence".into());
    }
    Ok(result)
}

/// A manifold vertex link is one connected cycle (closed) or one connected
/// chain (boundary). Edge incidence alone would miss two shells touching only
/// at a newly welded vertex. The caller separately requires closed delivery.
fn audit_vertex_links(triangles: &[[u32; 3]], budget: &mut Budget) -> Result<usize, String> {
    let mut faces = BTreeSet::new();
    let mut edges = BTreeMap::<(u32, u32), (usize, i32)>::new();
    let mut links = BTreeMap::<u32, Vec<(u32, u32)>>::new();
    for &[a, b, c] in triangles {
        budget.charge(7)?;
        let mut face = [a, b, c];
        face.sort_unstable();
        if !faces.insert(face) {
            return Err("USD UV transfer created duplicate geometric faces".into());
        }
        for (a, b) in [(a, b), (b, c), (c, a)] {
            let entry = edges.entry((a.min(b), a.max(b))).or_default();
            entry.0 += 1;
            entry.1 += if a < b { 1 } else { -1 };
            if entry.0 > 2 {
                return Err("USD UV transfer created a nonmanifold edge".into());
            }
        }
        for (center, edge) in [(a, (b, c)), (b, (c, a)), (c, (a, b))] {
            links.entry(center).or_default().push(edge);
        }
    }
    if edges.values().any(|&(count, balance)| count == 2 && balance != 0) {
        return Err("USD UV transfer reversed a shared-edge winding".into());
    }
    for (&vertex, link) in &links {
        let mut neighbors = BTreeMap::<u32, Vec<u32>>::new();
        for &(a, b) in link {
            budget.charge(2)?;
            neighbors.entry(a).or_default().push(b);
            neighbors.entry(b).or_default().push(a);
        }
        let endpoints = neighbors.values().filter(|next| next.len() == 1).count();
        if !matches!(endpoints, 0 | 2) || neighbors.values().any(|next| next.is_empty() || next.len() > 2) {
            return Err(format!("USD UV transfer created a nonmanifold link at vertex {vertex}"));
        }
        let mut pending = vec![*neighbors.keys().next().ok_or("empty UV vertex link")?];
        let mut visited = BTreeSet::new();
        while let Some(next) = pending.pop() {
            budget.charge(1)?;
            if visited.insert(next) {
                pending.extend(&neighbors[&next]);
            }
        }
        if visited.len() != neighbors.len() {
            return Err(format!("USD UV transfer created disconnected vertex link at {vertex}"));
        }
    }
    Ok(links.len())
}
fn refine(
    positions: &mut Vec<Vec3>,
    triangles: &[[u32; 3]],
    failed: &BTreeSet<usize>,
    options: &TransferOptions,
    budget: &mut Budget,
) -> Result<Vec<[u32; 3]>, String> {
    let mut edges = BTreeMap::new();
    for &index in failed {
        let [a, b, c] = triangles[index];
        let mut splittable = false;
        for (a, b) in [(a, b), (b, c), (c, a)] {
            let key = (a.min(b), a.max(b));
            if let std::collections::btree_map::Entry::Vacant(entry) = edges.entry(key) {
                budget.charge(1)?;
                let p = v(scale(add(d(positions[a as usize]), d(positions[b as usize])), 0.5))?;
                if p == positions[a as usize] || p == positions[b as usize] {
                    continue;
                }
                entry.insert(add_vertex(positions, p, options)?);
            }
            splittable = true;
        }
        if !splittable {
            return Err(format!(
                "USD UV refinement exhausted f32 position precision on all edges of face {index} {a},{b},{c}"
            ));
        }
    }
    let mut result = Vec::new();
    for &[a, b, c] in triangles {
        budget.charge(1)?;
        let edge = |a: u32, b: u32| edges.get(&(a.min(b), a.max(b))).copied();
        let (ab, bc, ca) = (edge(a, b), edge(b, c), edge(c, a));
        let pieces: Vec<[u32; 3]> = match (ab, bc, ca) {
            (None, None, None) => vec![[a, b, c]],
            (Some(m), None, None) => vec![[a, m, c], [m, b, c]],
            (None, Some(m), None) => vec![[b, m, a], [m, c, a]],
            (None, None, Some(m)) => vec![[c, m, b], [m, a, b]],
            (Some(x), Some(y), None) => vec![[x, b, y], [a, x, y], [a, y, c]],
            (None, Some(y), Some(z)) => vec![[y, c, z], [b, y, z], [b, z, a]],
            (Some(x), None, Some(z)) => vec![[z, a, x], [c, z, x], [c, x, b]],
            (Some(x), Some(y), Some(z)) => vec![[a, x, z], [x, b, y], [z, y, c], [x, y, z]],
        };
        for triangle in pieces {
            push_triangle(&mut result, positions, triangle, options)?;
        }
    }
    Ok(result)
}
fn add_vertex(positions: &mut Vec<Vec3>, point: Vec3, options: &TransferOptions) -> Result<u32, String> {
    if positions.len() >= options.max_vertices || positions.len() >= u32::MAX as usize {
        return Err("USD UV transfer exceeded aggregate vertex budget".into());
    }
    let index = positions.len() as u32;
    positions.push(point);
    Ok(index)
}
fn push_triangle(
    out: &mut Vec<[u32; 3]>,
    positions: &[Vec3],
    triangle: [u32; 3],
    options: &TransferOptions,
) -> Result<(), String> {
    if triangle[0] == triangle[1] || triangle[1] == triangle[2] || triangle[2] == triangle[0] {
        return Ok(());
    }
    valid_triangle(positions, triangle)?;
    if out.len() >= options.max_triangles {
        return Err("USD UV transfer exceeded aggregate triangle budget".into());
    }
    out.push(triangle);
    Ok(())
}
fn valid_triangle(positions: &[Vec3], triangle: [u32; 3]) -> Result<(), String> {
    if triangle.iter().any(|&i| i as usize >= positions.len()) {
        return Err("USD UV triangle index outside positions".into());
    }
    let [a, b, c] = triangle.map(|i| d(positions[i as usize]));
    if a.iter().chain(&b).chain(&c).any(|x| !x.is_finite()) || norm(cross(sub(b, a), sub(c, a))) == 0.0 {
        return Err(format!("USD UV splitting encountered nonfinite/degenerate triangle {triangle:?} at {a:?}, {b:?}, {c:?}; choose representable bounds"));
    }
    Ok(())
}
fn d(p: Vec3) -> P {
    [f64::from(p.x), f64::from(p.y), f64::from(p.z)]
}
fn v(p: P) -> Result<Vec3, String> {
    let p = p.map(|x| x as f32);
    if p.iter().any(|x| !x.is_finite()) {
        Err("USD UV geometry exceeds f32 range".into())
    } else {
        Ok(Vec3::new(p[0], p[1], p[2]))
    }
}
fn add(a: P, b: P) -> P {
    std::array::from_fn(|i| a[i] + b[i])
}
fn sub(a: P, b: P) -> P {
    std::array::from_fn(|i| a[i] - b[i])
}
fn scale(a: P, s: f64) -> P {
    a.map(|x| x * s)
}
fn dot(a: P, b: P) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}
fn norm(a: P) -> f64 {
    a[0].hypot(a[1]).hypot(a[2])
}
fn unit(a: P) -> Result<P, String> {
    let n = norm(a);
    if !n.is_finite() || n == 0.0 {
        Err("USD UV source feature is degenerate".into())
    } else {
        Ok(scale(a, 1.0 / n))
    }
}
fn cross([x, y, z]: P, [a, b, c]: P) -> P {
    [y * c - z * b, z * a - x * c, x * b - y * a]
}
fn point(points: [Vec3; 3], weights: [f64; 3]) -> Result<Vec3, String> {
    v((0..3).fold([0.; 3], |sum, i| add(sum, scale(d(points[i]), weights[i]))))
}
fn one_sided_point(points: [Vec3; 3], weights: [f64; 3]) -> Result<Option<Vec3>, String> {
    let boundary = point(points, weights)?;
    let center = scale((0..3).fold([0.; 3], |sum, i| add(sum, d(points[i]))), 1.0 / 3.0);
    let exact = (0..3).fold([0.; 3], |sum, i| add(sum, scale(d(points[i]), weights[i])));
    for fraction in [0.001, 0.01, 0.05, 0.25, 0.5, 1.0] {
        let candidate = v(add(scale(exact, 1.0 - fraction), scale(center, fraction)))?;
        if candidate != boundary && dot(sub(d(candidate), d(boundary)), sub(center, d(boundary))) > 0.0 {
            return Ok(Some(candidate));
        }
    }
    Ok(None)
}
fn interpolate(uv: [Uv; 3], weights: [f64; 3]) -> Uv {
    std::array::from_fn(|a| (0..3).map(|i| uv[i][a] * weights[i]).sum())
}
fn texel_error(a: Uv, b: Uv, dimensions: [f64; 2], periods: [Option<f64>; 2]) -> f64 {
    let error: [f64; 2] = std::array::from_fn(|axis| {
        let mut delta = if let Some(period) = periods[axis] {
            (a[axis].rem_euclid(period) - b[axis].rem_euclid(period)).abs()
        } else {
            (a[axis] - b[axis]).abs()
        };
        if let Some(period) = periods[axis] {
            delta = delta.min(period - delta);
        }
        delta * dimensions[axis]
    });
    error[0].hypot(error[1])
}
#[cfg(test)]
mod tests {
    use super::*;
    fn budget() -> Budget {
        Budget {
            used: 0,
            maximum: 100000,
            coordinate_queries: 0,
            coincident_merges: 0,
            collinear_splits: 0,
            native_tie_checks: 0,
            native_tie_candidates: 0,
            periodic_rebase_checks: 0,
            periodic_rebased_corners: 0,
        }
    }
    fn options() -> TransferOptions {
        TransferOptions {
            max_uv_error_texels: 0.2,
            max_refinement_passes: 5,
            max_work: 100000,
            max_vertices: 1000,
            max_triangles: 1000,
        }
    }
    #[test]
    fn exact_coincident_split_edge_repairs_all_incident_faces_and_preserves_links() {
        let positions = vec![
            Vec3::ZERO,
            Vec3::new(1., 0., 0.),
            Vec3::new(0., 1., 0.),
            Vec3::new(0., 0., 1.),
            Vec3::new(0.5, 0.5, 0.),
            Vec3::new(0.5, 0.5, 0.),
        ];
        // A topological subdivision of a tetrahedron edge with two consecutive
        // intersection IDs at the same f32 position. Both incident zero faces
        // collapse only after the common exact-coordinate edge is canonicalized.
        let triangles = vec![[0, 2, 5], [0, 5, 4], [0, 4, 1], [1, 4, 3], [4, 5, 3], [5, 2, 3], [0, 1, 3], [2, 0, 3]];
        let mut budget = budget();
        let repaired =
            repair_collapsed_faces(&positions, triangles, &[[0, 5, 4], [4, 5, 3]], &options(), &mut budget).unwrap();
        assert_eq!(budget.coincident_merges, 1);
        assert_eq!(repaired.len(), 6);
        assert_eq!(audit_vertex_links(&repaired, &mut budget).unwrap(), 5);
        assert!(repaired.iter().flatten().all(|&i| i != 5));
    }
    #[test]
    fn vertex_link_audit_rejects_two_closed_shells_touching_at_one_index() {
        let triangles = vec![[0, 2, 1], [0, 1, 3], [1, 2, 3], [2, 0, 3], [0, 5, 4], [0, 4, 6], [4, 5, 6], [5, 0, 6]];
        let error = audit_vertex_links(&triangles, &mut budget()).unwrap_err();
        assert!(error.contains("disconnected vertex link"), "{error}");
    }
    #[test]
    fn unrelated_coincident_vertices_are_not_globally_welded() {
        let positions = vec![
            Vec3::ZERO,
            Vec3::new(1., 0., 0.),
            Vec3::new(0., 1., 0.),
            Vec3::new(0., 0., 1.),
            Vec3::ZERO,
            Vec3::new(-1., 0., 0.),
            Vec3::new(0., -1., 0.),
            Vec3::new(0., 0., -1.),
        ];
        let triangles = vec![[0, 2, 1], [0, 1, 3], [1, 2, 3], [2, 0, 3], [4, 6, 5], [4, 5, 7], [5, 6, 7], [6, 4, 7]];
        let mut budget = budget();
        let repaired = repair_collapsed_faces(&positions, triangles.clone(), &[], &options(), &mut budget).unwrap();
        assert_eq!(triangles, repaired);
        assert_eq!(budget.coincident_merges, 0);
        assert_eq!(audit_vertex_links(&repaired, &mut budget).unwrap(), 8);
    }
    #[test]
    fn exact_collinear_face_repairs_neighbor_without_moving_distinct_vertices() {
        let positions = vec![
            Vec3::ZERO,
            Vec3::new(1., 0., 0.),
            Vec3::new(0.5, 0., 0.),
            Vec3::new(0., 1., 0.),
            Vec3::new(0., 0., 1.),
        ];
        let original = positions.clone();
        let triangles = vec![[0, 3, 2], [2, 3, 1], [0, 1, 4], [1, 3, 4], [3, 0, 4]];
        let mut budget = budget();
        let result = repair_collapsed_faces(&positions, triangles, &[[0, 2, 1]], &options(), &mut budget).unwrap();
        assert_eq!(positions, original);
        assert_eq!(budget.coincident_merges, 0);
        assert_eq!(budget.collinear_splits, 1);
        assert_eq!(result.len(), 6);
        assert_eq!(audit_vertex_links(&result, &mut budget).unwrap(), 5);
        assert!(result.iter().flatten().any(|&i| i == 2));
    }
    #[test]
    fn large_constant_coordinate_does_not_erase_small_axis_chart_side() {
        let mut positions = vec![Vec3::new(0., 1e8, 0.), Vec3::new(1e-6, 1e8, 0.), Vec3::new(0., 1e8, 1.)];
        let original = positions.clone();
        let mut budget = budget();
        let mut rounding = 0.0;
        let result = split_plane(
            &mut positions,
            &[[0, 1, 2]],
            Plane { normal: [1., 0., 0.], offset: 1e-7 },
            &options(),
            &mut budget,
            &mut rounding,
        )
        .unwrap();
        assert_eq!(&positions[..3], original);
        assert_eq!(result.len(), 3);
        assert!(positions.iter().skip(3).all(|p| p.x == 1e-7_f32));
        assert!(result.iter().any(|tri| tri.iter().all(|&i| positions[i as usize].x <= 1e-7_f32)));
        let area = |tri: [u32; 3]| {
            let [a, b, c] = tri.map(|i| d(positions[i as usize]));
            0.5 * norm(cross(sub(b, a), sub(c, a)))
        };
        assert!((result.iter().map(|&tri| area(tri)).sum::<f64>() - area([0, 1, 2])).abs() < 1e-13);
        assert_eq!(budget.coincident_merges, 0);
        assert!(rounding < 1e-14);
    }
    #[test]
    fn periodic_error_reduces_coordinates_before_subtraction() {
        let error =
            texel_error([999999.9999999999, 0.], [-999999.9999999995, 0.], [8192., 8192.], [Some(1.), Some(1.)]);
        assert_eq!(error, 4.76837158203125e-6);
        assert!(error > 4e-6);
    }
    #[test]
    fn multiple_collinear_middle_points_reconcile_the_whole_boundary_cycle() {
        let positions = vec![
            Vec3::ZERO,
            Vec3::new(1., 0., 0.),
            Vec3::new(0.25, 0., 0.),
            Vec3::new(0.75, 0., 0.),
            Vec3::new(0., 1., 0.),
            Vec3::new(0., 0., 1.),
        ];
        // One tetrahedron side carries A->M1->M2->B, the other still A->B.
        let triangles = vec![[0, 4, 2], [2, 4, 3], [3, 4, 1], [0, 1, 5], [1, 4, 5], [4, 0, 5]];
        let before = positions.clone();
        let mut budget = budget();
        let result = repair_collinear_boundaries(&positions, triangles, &options(), &mut budget).unwrap();
        assert_eq!(positions, before);
        assert_eq!(result.len(), 8);
        assert_eq!(budget.collinear_splits, 2);
        assert_eq!(audit_vertex_links(&result, &mut budget).unwrap(), 6);
        assert!(edge_incidents(&result, &mut budget).unwrap().values().all(|faces| faces.len() == 2));
    }
    #[test]
    fn native_distance_tie_fallback_accepts_repeat_and_rejects_mixed_wraps() {
        use mm3e_kit::{
            texture::{CornerUvs, Sampler, TextureImage, Wrap},
            Material, Transform,
        };
        use mm3e_orchestrator::{
            appearance::{Channel, ScalarMap, SurfaceMaps, TextureMap},
            Object,
        };
        for secondary in [None, Some(Wrap::Mirror), Some(Wrap::Clamp)] {
            let mut scene = Scene::new(4, 4);
            let surface = TriangleSurface::new(
                vec![Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(1., 1., 0.), Vec3::new(0., 1., 0.)],
                vec![[0, 1, 2], [0, 2, 3]],
                0.01,
            )
            .unwrap();
            let id = scene.surface(surface).unwrap();
            let mat = scene.material(Material::default());
            scene.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, mat));
            let image = TextureImage::from_linear_rgba(
                4,
                1,
                vec![[1., 0., 0., 1.], [0., 1., 0., 1.], [0., 0., 1., 1.], [1., 1., 0., 1.]],
            )
            .unwrap();
            let sampler = Sampler::default();
            assert_eq!(
                image.sample([0.125, 0.25], 0., sampler).unwrap(),
                image.sample([1.125, 0.25], 0., sampler).unwrap()
            );
            let roughness = secondary.map(|wrap| ScalarMap {
                map: TextureMap {
                    image: TextureImage::from_data_channels(
                        4,
                        1,
                        vec![[0.1, 0., 0., 1.], [0.9, 0., 0., 1.], [0.3, 0., 0., 1.], [0.7, 0., 0., 1.]],
                    )
                    .unwrap(),
                    sampler: Sampler { u: wrap, ..sampler },
                },
                channel: Channel::R,
            });
            let uv = CornerUvs::new(
                vec![[0.125, 0.25]; 3].into_iter().chain(vec![[1.125, 0.25]; 3]).collect(),
                vec![[0, 1, 2], [3, 4, 5]],
                2,
            )
            .unwrap();
            scene
                .bind_surface_maps(
                    0,
                    uv,
                    SurfaceMaps { albedo: Some(TextureMap { image, sampler }), roughness, ..SurfaceMaps::default() },
                )
                .unwrap();
            let mut budget = budget();
            let (mut sources, _, _) = sources(&scene, &mut budget).unwrap();
            let result = boundary_uv(&scene, &mut sources, 0, Vec3::new(0.5, 0.5, 0.02), None, 0.01, &mut budget);
            if secondary.is_none() {
                assert_eq!(result.unwrap(), [0.125, 0.25]);
                assert_eq!(budget.native_tie_candidates, 2);
            } else {
                assert!(result.unwrap_err().contains("incompatible native-distance-tied"));
            }
        }
    }
}
