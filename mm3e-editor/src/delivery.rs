//! Bounded delivery of the evaluated, selected CSG field as a USD mesh cache.
//! Native editing operators remain in the native project. Both extraction grids
//! and independent face samples must pass before any output file is installed.
use crate::{
    animation::{AnimationSample, Playback},
    model::{array, vec, Combination, Document, Pass, Shape, V3},
    protocol::Failure,
    storage,
    usd::{self, CameraFrame, FrameMesh, UsdAsset, UsdCamera, UsdMaterial, UsdMesh, UsdRepresentation},
};
use mm3e_kit::{
    meshing::{extract_isosurface, Mesh},
    meshing_boolean::{extract_boolean_isosurface_with_options, BooleanExtractionOptions},
    surface::TriangleSurface,
    Vec3,
};
use mm3e_orchestrator::{meshing::BooleanField, sampling::CountedSceneField};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

pub const MAX_EXPORT_FRAMES: usize = 60;
const MAX_VERTICES: usize = 500_000;
const MAX_TRIANGLES: usize = 1_000_000;
const MAX_FIELD_WORK: u64 = 200_000_000;
fn fps() -> f64 {
    24.0
}
fn near_clip() -> f32 {
    0.0001
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExportUsdRequest {
    /// New .usda file inside the editor root. Existing files are never replaced.
    pub path: String,
    /// Selected entities compose in document order, including selected subtractors.
    pub object_ids: Vec<String>,
    pub bounds_min: V3,
    pub bounds_max: V3,
    /// Base cells for the final extraction, even numbers in 8..=128. Local source
    /// correction/refinement is reported separately; the comparison uses half these counts.
    pub resolution: [u32; 3],
    /// Maximum sampled bidirectional distance between the two extracted meshes.
    /// This is a convergence check in meters, not a certified Hausdorff bound.
    pub max_surface_error_m: f32,
    /// Maximum absolute authored scalar at fine-mesh vertices, edge midpoints and centroids.
    /// Some authored fields are distance approximations; this is not a metric error bound.
    pub max_field_residual: f32,
    #[serde(default)]
    pub clip: Option<String>,
    #[serde(default)]
    pub start_seconds: f64,
    #[serde(default)]
    pub end_seconds: f64,
    #[serde(default = "fps")]
    pub frames_per_second: f64,
    /// Consumer camera near plane in meters. Native ray tracing starts at the eye.
    #[serde(default = "near_clip")]
    pub camera_near_clip_m: f32,
    /// Explicitly opt into chart transfer and consumer-controlled texture filtering.
    #[serde(default)]
    pub texture_delivery: Option<TextureDelivery>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TextureFiltering {
    ReaderDefined,
}
fn uv_error() -> f64 {
    0.25
}
fn uv_passes() -> u32 {
    8
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TextureDelivery {
    pub filtering: TextureFiltering,
    #[serde(default = "uv_error")]
    pub max_uv_error_texels: f64,
    #[serde(default = "uv_passes")]
    pub max_refinement_passes: u32,
}

fn times(request: &ExportUsdRequest) -> Result<Vec<(f64, f32)>, String> {
    let (start, end, fps) = (request.start_seconds, request.end_seconds, request.frames_per_second);
    if !start.is_finite()
        || !end.is_finite()
        || start < 0.0
        || end < start
        || end > 86_400.0
        || !fps.is_finite()
        || !(0.001..=240.0).contains(&fps)
    {
        return Err("USD requires 0 <= start <= end <= 86400 seconds and fps in 0.001..=240".into());
    }
    if request.clip.is_none() && (start != 0.0 || end != 0.0) {
        return Err("USD time ranges require an explicit clip; omit it for the authored rest pose".into());
    }
    let intervals = (end - start) * fps;
    if (intervals - intervals.round()).abs() > 1e-6 || intervals.round() >= MAX_EXPORT_FRAMES as f64 {
        return Err(format!("USD end must land on the fps grid, with at most {MAX_EXPORT_FRAMES} inclusive frames"));
    }
    let mut result = vec![];
    for index in 0..=intervals.round() as usize {
        let seconds = if index == intervals.round() as usize { end } else { start + index as f64 / fps };
        let sample = seconds as f32;
        if result.last().is_some_and(|&(_, previous)| sample <= previous) {
            return Err("USD samples collide at native animation time precision".into());
        }
        result.push((seconds * fps, sample));
    }
    Ok(result)
}

fn primitive_work(shape: &Shape) -> Result<u64, String> {
    Ok(match shape {
        Shape::Csg { expression } => expression.compile()?.validate()? as u64,
        Shape::Surface { triangles, .. } => triangles.len().max(1) as u64,
        Shape::Volume { .. } => 8,
        _ => 1,
    })
}

fn mesh_surface(mesh: &Mesh) -> Result<TriangleSurface, String> {
    // The tiny thickness is removed from distance queries; the BVH stores the
    // actual triangles unchanged. Per-frame geometry limits are enforced here.
    TriangleSurface::new(mesh.positions.clone(), mesh.triangles.clone(), 1e-8)
}

fn closed_topology(triangles: &[[u32; 3]]) -> Result<(), String> {
    let mut edges = BTreeMap::<(u32, u32), (usize, i32)>::new();
    for &[a, b, c] in triangles {
        for (from, to) in [(a, b), (b, c), (c, a)] {
            let count = edges.entry((from.min(to), from.max(to))).or_default();
            count.0 += 1;
            count.1 += if from < to { 1 } else { -1 };
        }
    }
    if edges.values().any(|&(count, winding)| count != 2 || winding != 0) {
        return Err("USD UV transfer changed the closed, consistently oriented composed topology".into());
    }
    Ok(())
}

fn face_points(mesh: &Mesh, mut visit: impl FnMut(Vec3) -> Result<(), String>) -> Result<(), String> {
    for &p in &mesh.positions {
        visit(p)?;
    }
    for &[a, b, c] in &mesh.triangles {
        let [a, b, c] = [a, b, c].map(|i| mesh.positions[i as usize]);
        for p in [(a + b) * 0.5, (b + c) * 0.5, (c + a) * 0.5, (a + b + c) * (1.0 / 3.0)] {
            visit(p)?;
        }
    }
    Ok(())
}

fn deviation(from: &Mesh, to: &TriangleSurface) -> Result<(f32, V3), String> {
    let mut maximum = 0.0_f32;
    let mut location = array(from.positions[0]);
    face_points(from, |point| {
        let distance = to.distance(point) + to.half_thickness();
        if !distance.is_finite() {
            return Err("USD convergence distance is not finite".into());
        }
        if distance > maximum {
            maximum = distance;
            location = array(point);
        }
        Ok(())
    })?;
    Ok((maximum, location))
}

pub fn export(document: &Document, root: &Path, request: &ExportUsdRequest) -> Result<Value, Failure> {
    let target = storage::path(root, &request.path, false)?;
    if target.extension().and_then(|s| s.to_str()) != Some("usda") {
        return Err(Failure::invalid("USD output must use a .usda extension"));
    }
    if target.exists() {
        return Err(Failure::invalid("USD output already exists; use a new path"));
    }
    if request.texture_delivery.is_some()
        && document.texture_bindings.iter().any(|b| request.object_ids.contains(&b.object))
    {
        let directory = target.parent().expect("resolved parent").join(asset_directory_name(request));
        match fs::symlink_metadata(&directory) {
            Ok(_) => return Err(Failure::invalid(format!("USD asset directory {} already exists and is preserved; it may be a partial prior delivery; inspect it and use a new output path",directory.display()))),
            Err(error) if error.kind()==std::io::ErrorKind::NotFound => {},
            Err(error)=>return Err(Failure::io(error.to_string())),
        }
    }
    let encoded = encode_with_assets(document, request).map_err(Failure::invalid)?;
    publish_encoded(root, target, request, encoded)
}

/// Publish a fully validated payload beside the destination resolved before
/// encoding. Keeping this boundary separate prevents later pathname resolution
/// from moving the layer away from its reserved companion assets.
fn publish_encoded(
    root: &Path,
    target: PathBuf,
    request: &ExportUsdRequest,
    encoded: EncodedUsd,
) -> Result<Value, Failure> {
    let EncodedUsd { bytes, report, assets, asset_directory } = encoded;
    if let Some(directory) = asset_directory {
        let path = target.parent().expect("resolved target parent").join(&directory);
        fs::create_dir(&path).map_err(|e| Failure::io(format!("cannot reserve USD assets {}: {e}", path.display())))?;
        let result = (|| {
            let asset_root = path.canonicalize().map_err(|e| Failure::io(e.to_string()))?;
            if asset_root != path {
                return Err(Failure::invalid("reserved USD asset directory changed its resolved location"));
            }
            let mut records = vec![];
            for (name, data) in &assets {
                storage::write(&asset_root, name, data, false)?;
                records.push(json!({"file":name,"bytes":data.len(),"sha256":format!("{:x}",Sha256::digest(data))}));
            }
            let manifest = json!({"format":"mm3e-usd-assets-v1","layer":target.file_name().and_then(|s|s.to_str()),
                "layer_sha256":format!("{:x}",Sha256::digest(&bytes)),"assets":records,"delivery":report,
                "completion":"The referenced layer is installed last; this manifest alone does not prove layer publication"});
            storage::write(
                &asset_root,
                "manifest.json",
                &serde_json::to_vec_pretty(&manifest).map_err(|e| Failure::invalid(e.to_string()))?,
                false,
            )?;
            // Freeze the resolved parent used for the asset reservation. An
            // authored in-root symlink may change while an export is running;
            // resolving the original request again could orphan the textures.
            storage::write(
                target.parent().expect("resolved parent"),
                target.file_name().and_then(|s| s.to_str()).expect("UTF-8 requested filename"),
                &bytes,
                false,
            )
        })();
        let target = result.map_err(|error| Failure {
            code: error.code,
            message: format!(
                "{}; USD assets preserved at {}; inspect the layer and use a new output path to retry",
                error.message,
                path.display()
            ),
        })?;
        return Ok(json!({"path":target,"bytes":bytes.len(),"asset_directory":path,"delivery":report}));
    }
    let target = storage::write(root, &request.path, &bytes, false)?;
    Ok(json!({"path":target,"bytes":bytes.len(),"delivery":report}))
}

pub fn encode(document: &Document, request: &ExportUsdRequest) -> Result<(Vec<u8>, Value), String> {
    let encoded = encode_with_assets(document, request)?;
    if !encoded.assets.is_empty() {
        return Err(
            "textured USD encoding includes external assets; use encode_with_assets or the export_usd command".into()
        );
    }
    Ok((encoded.bytes, encoded.report))
}

pub struct EncodedUsd {
    pub bytes: Vec<u8>,
    pub report: Value,
    /// Basenames relative to asset_directory, which is relative to the layer.
    pub assets: BTreeMap<String, Vec<u8>>,
    pub asset_directory: Option<String>,
}

fn asset_directory_name(request: &ExportUsdRequest) -> String {
    format!("mm3e-assets-{:x}", Sha256::digest(request.path.as_bytes()))
}

pub fn encode_with_assets(document: &Document, request: &ExportUsdRequest) -> Result<EncodedUsd, String> {
    let textured = document.texture_bindings.iter().any(|binding| request.object_ids.contains(&binding.object));
    if textured && request.texture_delivery.is_none() {
        return Err("textured USD requires explicit texture_delivery with filtering: reader_defined; native rendering filters are not portable".into());
    }
    if let Some(options) = &request.texture_delivery {
        if !options.max_uv_error_texels.is_finite()
            || !(0.000001..=1.0).contains(&options.max_uv_error_texels)
            || options.max_refinement_passes > 12
        {
            return Err(
                "USD texture delivery requires UV error in 0.000001..=1 texels and at most 12 refinement passes".into(),
            );
        }
    }
    let times = times(request)?;
    if !request.camera_near_clip_m.is_finite() || !(0.000001..=1.0).contains(&request.camera_near_clip_m) {
        return Err("USD camera_near_clip_m must be finite in 0.000001..=1".into());
    }
    for (name, value) in
        [("max_surface_error_m", request.max_surface_error_m), ("max_field_residual", request.max_field_residual)]
    {
        if !value.is_finite() || value <= 0.0 || value > 1000.0 {
            return Err(format!("USD {name} must be finite in (0,1000]"));
        }
    }
    crate::model::vector(request.bounds_min, "USD bounds_min")?;
    crate::model::vector(request.bounds_max, "USD bounds_max")?;
    if (0..3).any(|i| request.bounds_min[i] >= request.bounds_max[i]) {
        return Err("USD bounds must strictly increase on every axis".into());
    }
    if request.resolution.iter().any(|&n| !(8..=128).contains(&n) || n % 2 != 0) {
        return Err("USD resolution requires even final cell counts in 8..=128".into());
    }
    let ids: BTreeSet<_> = request.object_ids.iter().collect();
    if ids.is_empty() || ids.len() != request.object_ids.len() {
        return Err("USD object_ids must be nonempty and unique".into());
    }
    let indices: Vec<_> =
        document.objects.iter().enumerate().filter(|(_, o)| ids.contains(&o.id)).map(|(i, _)| i).collect();
    if indices.len() != ids.len() {
        return Err("USD selection contains missing object IDs".into());
    }
    if !matches!(document.objects[indices[0]].combine, Combination::Union) {
        return Err("first selected object in document order must use Union to seed the composed field".into());
    }
    if let Some(id) = &request.clip {
        let clip = document.clips.iter().find(|clip| &clip.id == id).ok_or_else(|| format!("missing USD clip {id}"))?;
        if request.end_seconds > f64::from(clip.duration) + 1e-7 {
            return Err("USD range exceeds the selected clip duration".into());
        }
    }
    let cost = indices.iter().try_fold(0u64, |sum, &i| primitive_work(&document.objects[i].shape).map(|n| sum + n))?;
    let coarse = request.resolution.map(|n| n / 2);
    let samples = |grid: [u32; 3]| grid.iter().map(|&n| u64::from(n) + 1).product::<u64>();
    let minimum_cost = indices.iter().try_fold(0u64, |sum, &i| match &document.objects[i].shape {
        Shape::Surface { .. } => Ok(sum + 1),
        shape => primitive_work(shape).map(|cost| sum + cost),
    })?;
    let minimum_work = (samples(request.resolution) + samples(coarse)) * minimum_cost * times.len() as u64;
    if minimum_work > MAX_FIELD_WORK {
        return Err(
            "USD grid exceeds bounded field-evaluation work; reduce range/grid or select simpler geometry".into()
        );
    }
    let mut materials = vec![];
    for &i in &indices {
        let object = &document.objects[i];
        let m = &object.material;
        if m.checker || m.reflectivity != 0.0 {
            return Err(format!("USD material {} uses unsupported checker or artistic reflectivity; author a supported material explicitly before export",object.id));
        }
        let root_f0 = (0.08 * m.specular).sqrt();
        materials.push(UsdMaterial {
            id: object.id.clone(),
            diffuse_color: m.albedo,
            roughness: m.roughness,
            metallic: m.metallic,
            ior: (1.0 + root_f0) / (1.0 - root_f0),
            emissive_color: m.emissive,
            textures: vec![],
        });
    }
    let asset_directory = textured.then(|| asset_directory_name(request));
    let assets = if let Some(directory) = &asset_directory {
        crate::delivery_textures::prepare(document, &indices, directory, &mut materials)?
    } else {
        crate::delivery_textures::Assets::default()
    };
    let requires_uv = materials.iter().any(|material| !material.textures.is_empty());
    let mut frames = vec![];
    let mut cameras = vec![];
    let mut reports = vec![];
    let mut total_vertices = 0;
    let mut total_triangles = 0;
    let mut work = 0_u64;
    let mut arrangement_work = 0_u64;
    let mut source_feature_work = 0_u64;
    for &(time, sample_time) in &times {
        let sample = request.clip.as_ref().map(|clip| AnimationSample {
            clip: clip.clone(),
            time: sample_time,
            playback: Playback::Clamp,
        });
        let selected = request.object_ids.iter().cloned().collect();
        let (mut scene, camera) = document.compile_at_selected(&Pass::Beauty, sample.as_ref(), &selected)?;
        let material_map: BTreeMap<_, _> =
            indices.iter().enumerate().map(|(slot, &i)| (scene.objects[i].mat, slot)).collect();
        scene.objects = indices.iter().map(|&i| scene.objects[i]).collect();
        if textured {
            scene.appearance.retain_objects(&indices)?;
            for (slot, material) in materials.iter().enumerate() {
                if material.textures.is_empty() {
                    scene.appearance.remove_binding(slot);
                }
            }
            scene.validate_appearance()?;
        } else {
            scene.appearance.clear();
        }
        let program = BooleanField::from_scene(&scene)?;
        let preserves_boundaries = program.channel_count() > 1;
        let source_features = crate::surface_delivery::eligible(&scene);
        let extraction_cost = if preserves_boundaries { program.estimated_work_per_point() } else { cost };
        if preserves_boundaries {
            work += (samples(request.resolution) + samples(coarse)) * extraction_cost;
        }
        if work + arrangement_work + source_feature_work > MAX_FIELD_WORK {
            return Err("USD boundary-preserving extraction exceeds bounded field-evaluation work".into());
        }
        let sampler = if preserves_boundaries { None } else { Some(CountedSceneField::new(&scene)?) };
        let query_work = Cell::new(0_u64);
        let attribute_work = Cell::new(0_u64);
        let query_count = Cell::new(0_u64);
        let query_error = RefCell::<Option<String>>::new(None);
        let available = MAX_FIELD_WORK - work - arrangement_work - source_feature_work;
        if !source_features
            && sampler.as_ref().is_some_and(|sampler| {
                (samples(request.resolution) + samples(coarse)) * sampler.minimum_work_per_point() as u64 > available
            })
        {
            return Err("USD grid exceeds remaining counted field-work budget".into());
        }
        let sample_field = |point| {
            let Some(sampler) = sampler.as_ref() else { return scene.sample_authored(point) };
            if query_error.borrow().is_some() {
                return mm3e_kit::sdf::Field::new(f32::NAN, u32::MAX);
            }
            match sampler
                .sample(point, available.saturating_sub(query_work.get()).saturating_sub(attribute_work.get()) as usize)
            {
                Ok(result) => {
                    query_work.set(query_work.get() + result.work as u64);
                    query_count.set(query_count.get() + 1);
                    result.field
                }
                Err(error) => {
                    *query_error.borrow_mut() = Some(error);
                    mm3e_kit::sdf::Field::new(f32::NAN, u32::MAX)
                }
            }
        };
        let evaluate = |point| sample_field(point).dist;
        let field_error = |error: String| query_error.borrow().clone().unwrap_or(error);
        let extract = |grid, remaining_work, project_and_refine| {
            if preserves_boundaries {
                extract_boolean_isosurface_with_options(
                    program.channel_count(),
                    |p, values| program.sample(p, values),
                    program.expression(),
                    vec(request.bounds_min),
                    vec(request.bounds_max),
                    grid,
                    BooleanExtractionOptions {
                        max_postgrid_work: remaining_work,
                        field_cost_per_callback: extraction_cost as usize,
                        project_and_refine,
                        normal_probe_distance_m: 0.001,
                        normal_comparison_tolerance: 1e-5,
                        ..BooleanExtractionOptions::default()
                    },
                )
            } else {
                extract_isosurface(evaluate, vec(request.bounds_min), vec(request.bounds_max), grid)
            }
        };
        let mut source_reports = Vec::new();
        let (mut fine, comparison, frame_arrangement_work) = if source_features {
            let mut extract_source = |resolution| -> Result<mm3e_kit::meshing::Mesh, String> {
                let result = crate::surface_delivery::extract(
                    &scene,
                    crate::surface_delivery::Options {
                        min: vec(request.bounds_min),
                        max: vec(request.bounds_max),
                        resolution,
                        max_work: (MAX_FIELD_WORK - work - arrangement_work - source_feature_work) as usize,
                        max_vertices: (MAX_VERTICES - total_vertices).min(mm3e_kit::surface::MAX_SURFACE_VERTICES),
                        max_triangles: (MAX_TRIANGLES - total_triangles).min(mm3e_kit::surface::MAX_SURFACE_TRIANGLES),
                        max_field_residual: request.max_field_residual,
                    },
                )?;
                source_feature_work += result.work;
                attribute_work.set(attribute_work.get() + result.work);
                source_reports.push(result.report);
                Ok(result.mesh)
            };
            let fine = extract_source(request.resolution)
                .map_err(|e| format!("USD at {sample_time}s source-feature fine grid: {e}"))?;
            let comparison = extract_source(coarse).map_err(|e| {
                let completed = source_reports.first().map(|report| json!({
                    "charged_work":report["charged_work"],
                    "extraction":report["local_extraction"]["arrangement_work"],
                    "conditioning":report["conditioning"]["work"],
                    "conditioning_reused_input_embedding":report["conditioning"]["initial_embedding_reused"],
                    "retessellation":report["retessellation"]["work"],
                    "bounded_retessellation":report["bounded_retessellation"]["work"],
                    "refinement":report["refinement"]["charged_work"],
                }));
                format!("USD at {sample_time}s source-feature comparison grid: {e}; completed fine-grid work {completed:?}; aggregate source work {source_feature_work}/{MAX_FIELD_WORK}")
            })?;
            (fine, comparison, 0)
        } else {
            let fine = extract(
                request.resolution,
                (MAX_FIELD_WORK - work - arrangement_work - source_feature_work) as usize,
                true,
            )
            .map_err(|e| format!("USD at {sample_time}s fine grid: {}", field_error(e)))?;
            arrangement_work += fine.metadata.boolean_arrangement_work as u64;
            work += fine.metadata.boolean_extra_field_evaluations as u64 * extraction_cost;
            let comparison =
                extract(coarse, (MAX_FIELD_WORK - work - arrangement_work - source_feature_work) as usize, false)
                    .map_err(|e| format!("USD at {sample_time}s convergence grid: {}", field_error(e)))?;
            let frame_arrangement_work =
                (fine.metadata.boolean_arrangement_work + comparison.metadata.boolean_arrangement_work) as u64;
            arrangement_work += comparison.metadata.boolean_arrangement_work as u64;
            work += comparison.metadata.boolean_extra_field_evaluations as u64 * extraction_cost;
            (fine, comparison, frame_arrangement_work)
        };
        if work + arrangement_work + source_feature_work > MAX_FIELD_WORK {
            return Err("USD extraction exceeds aggregate scalar and Boolean arrangement work budget".into());
        }
        if fine.metadata.connected_components != comparison.metadata.connected_components {
            return Err(format!("USD at {sample_time}s grid convergence changed connected component count: fine {} versus comparison {}; resolution {:?} versus {:?}; increase resolution or tighten bounds",fine.metadata.connected_components,comparison.metadata.connected_components,request.resolution,coarse));
        }
        let mut transferred_materials = None;
        let mut corner_uvs = None;
        let mut texture_transfer = Value::Null;
        if requires_uv {
            let options = request.texture_delivery.as_ref().expect("validated texture request");
            let transfer = crate::delivery_uv::transfer(
                &scene,
                &fine.positions,
                &fine.triangles,
                &material_map,
                crate::delivery_uv::TransferOptions {
                    max_uv_error_texels: options.max_uv_error_texels,
                    max_refinement_passes: options.max_refinement_passes,
                    max_work: (MAX_FIELD_WORK - work - arrangement_work - source_feature_work)
                        .saturating_sub(query_work.get()) as usize,
                    max_vertices: (MAX_VERTICES - total_vertices).min(mm3e_kit::surface::MAX_SURFACE_VERTICES),
                    max_triangles: (MAX_TRIANGLES - total_triangles).min(mm3e_kit::surface::MAX_SURFACE_TRIANGLES),
                },
            )
            .map_err(|e| format!("USD texture transfer at {sample_time}s: {e}"))?;
            let charged = transfer.report["charged_work"].as_u64().ok_or("USD transfer did not report charged work")?;
            work += charged;
            attribute_work.set(attribute_work.get() + charged);
            fine.positions = transfer.positions;
            fine.triangles = transfer.triangles;
            let topology_work = 3 * fine.triangles.len() as u64;
            if work + arrangement_work + source_feature_work + query_work.get() + topology_work > MAX_FIELD_WORK {
                return Err("USD UV topology verification exceeds aggregate work budget".into());
            }
            work += topology_work;
            attribute_work.set(attribute_work.get() + topology_work);
            closed_topology(&fine.triangles)?;
            transferred_materials = Some(transfer.material_ids);
            corner_uvs =
                Some(transfer.corner_uvs.into_iter().map(|triangle| triangle.map(|uv| uv.map(|v| v as f32))).collect());
            texture_transfer = transfer.report;
            texture_transfer["closed_oriented_topology_verified"] = json!(true);
            texture_transfer["topology_verification_work"] = json!(topology_work);
        }
        total_vertices += fine.positions.len();
        total_triangles += fine.triangles.len();
        if total_vertices > MAX_VERTICES || total_triangles > MAX_TRIANGLES {
            return Err("USD exceeds aggregate 500000 vertices or 1000000 triangles".into());
        }
        let embedding_validation = if source_features && !requires_uv {
            // No operation changed the refined positions or indices. Its
            // complete geometric check is already in source_feature_work.
            let checked = &source_reports[0]["refinement"]["embedding_validation"];
            if checked["intersection_free"].as_bool() != Some(true) {
                return Err("USD native refinement omitted its geometric intersection check".into());
            }
            json!({"intersection_free":true,"additional_work":0,
                "scope":"unchanged refined mesh","validation":checked})
        } else {
            // UV transfer can introduce vertices and faces after refinement.
            // Validate the actual delivered geometry, including scalar/Boolean
            // extraction routes that do not use native-surface refinement.
            let checked = mm3e_kit::surface_intersections::validate(
                &fine.positions,
                &fine.triangles,
                (MAX_FIELD_WORK - work - arrangement_work - source_feature_work).saturating_sub(query_work.get())
                    as usize,
            )
            .map_err(|error| format!("USD final geometry at {sample_time}s: {error}"))?;
            work += checked.work as u64;
            attribute_work.set(attribute_work.get() + checked.work as u64);
            json!({"intersection_free":true,"additional_work":checked.work,
                "scope":"final delivered mesh","validation":{"work":checked.work,
                    "candidate_pairs":checked.candidate_pairs,"reused_pairs":checked.reused_pairs,"predicate_tests":checked.predicate_tests,
                    "exact_predicates":checked.exact_predicates,"bvh_chunks":checked.bvh_chunks}})
        };
        let fine_surface = mesh_surface(&fine)?;
        let coarse_surface = mesh_surface(&comparison)?;
        let fine_deviation = deviation(&fine, &coarse_surface)?;
        let coarse_deviation = deviation(&comparison, &fine_surface)?;
        let (geometric_error, deviation_point) =
            if fine_deviation.0 >= coarse_deviation.0 { fine_deviation } else { coarse_deviation };
        if geometric_error > request.max_surface_error_m {
            return Err(format!("USD at {sample_time}s sampled grid deviation {geometric_error}m at {deviation_point:?} exceeds {}m; fine-to-comparison {}m, comparison-to-fine {}m; increase resolution or tighten bounds",request.max_surface_error_m,fine_deviation.0,coarse_deviation.0));
        }
        if preserves_boundaries {
            work += (fine.positions.len() + 5 * fine.triangles.len()) as u64 * cost;
        }
        if work + arrangement_work + source_feature_work > MAX_FIELD_WORK {
            return Err("USD validation exceeds bounded field-evaluation work".into());
        }
        let mut residual = 0.0_f32;
        let mut residual_point = array(fine.positions[0]);
        face_points(&fine, |p| {
            let scalar = evaluate(p).abs();
            if !scalar.is_finite() {
                return Err("USD validation returned a nonfinite field".into());
            }
            if scalar > residual {
                residual = scalar;
                residual_point = array(p);
            }
            Ok(())
        })
        .map_err(field_error)?;
        if residual > request.max_field_residual {
            return Err(format!("USD at {sample_time}s sampled field residual {residual} at {residual_point:?} exceeds {}; increase resolution or tighten bounds",request.max_field_residual));
        }
        let material_ids = if let Some(materials) = transferred_materials {
            materials
        } else {
            fine.triangles
                .iter()
                .map(|triangle| {
                    let [a, b, c] = triangle.map(|i| fine.positions[i as usize]);
                    let owner = sample_field((a + b + c) * (1.0 / 3.0)).mat;
                    if let Some(error) = query_error.borrow().clone() {
                        return Err(error);
                    }
                    material_map
                        .get(&owner)
                        .copied()
                        .ok_or_else(|| "USD field produced a material outside the selection".to_string())
                })
                .collect::<Result<Vec<_>, _>>()?
        };
        if let Some(error) = query_error.borrow().clone() {
            return Err(error);
        }
        work += query_work.get();
        if work + arrangement_work + source_feature_work > MAX_FIELD_WORK {
            return Err("USD geometry and attribute validation exceeds aggregate work budget".into());
        }
        reports.push(json!({"time_code":time,"native_seconds":sample_time,"vertices":fine.positions.len(),"triangles":fine.triangles.len(),
            "texture_transfer":texture_transfer,"source_feature_extractions":source_reports,"embedding_validation":embedding_validation,
            "extraction":if source_features {"local_surface_features"}else if preserves_boundaries {"projected_boolean_planes"}else{"single_scalar_tetrahedra"},
            "comparison_extraction":if source_features {"local_surface_features"}else if preserves_boundaries {"affine_boolean_planes"}else{"single_scalar_tetrahedra"},
            "scalar_channels":program.channel_count(),"opaque_smooth_channels":program.opaque_smooth_channels(),
            "field_work_mode":if source_features {"counted_local_features_plus_native_validation"}else if preserves_boundaries {"static_boolean_program_estimate"}else{"counted_scene_and_bvh"},
            "counted_field_work":query_work.get(),"counted_field_queries":query_count.get(),
            "boolean_arrangement_work":frame_arrangement_work,
            "boolean_extra_field_evaluations":fine.metadata.boolean_extra_field_evaluations+comparison.metadata.boolean_extra_field_evaluations,
            "boolean_projected_vertices":fine.metadata.boolean_projected_vertices,
            "boolean_refinement_passes":fine.metadata.boolean_refinement_passes,
            "boolean_added_vertices":fine.metadata.boolean_added_vertices,"boolean_added_triangles":fine.metadata.boolean_added_triangles,
            "boolean_max_vertex_displacement_m":fine.metadata.boolean_max_vertex_displacement,
            "normal_probe_distance_m":if preserves_boundaries {Some(0.001)}else{None},
            "normal_comparison_tolerance":if preserves_boundaries {Some(1e-5)}else{None},
            "sampled_mesh_deviation_m":geometric_error,"sampled_field_residual":residual,"connected_components":fine.metadata.connected_components,
            "sampled_mesh_deviation_point":deviation_point,"sampled_field_residual_point":residual_point,
            "fine_to_comparison_m":fine_deviation.0,"comparison_to_fine_m":coarse_deviation.0,
            "coincident_vertices_merged":fine.metadata.coincident_vertices_merged,
            "collinear_faces_split":fine.metadata.collinear_faces_split,
            "degenerate_triangles_removed":fine.metadata.degenerate_triangles_removed,
            "boundary_edges":fine.metadata.boundary_edges,"signed_volume_m3":fine.metadata.signed_volume,"surface_area_m2":fine.metadata.surface_area,
            "field_evaluations":fine.metadata.field_evaluations+comparison.metadata.field_evaluations}));
        frames.push(FrameMesh {
            time,
            positions: fine.positions.into_iter().map(array).collect(),
            triangles: fine.triangles,
            material_ids,
            corner_uvs,
        });
        let film = &document.settings.film;
        cameras.push(CameraFrame {
            time,
            position: array(camera.eye),
            target: array(camera.eye + camera.forward),
            up: array(camera.up),
            vertical_fov_degrees: (2.0 * camera.fov_scale.atan()).to_degrees(),
            aspect_ratio: document.settings.width as f32 / document.settings.height as f32,
            near_clip: request.camera_near_clip_m,
            far_clip: scene.marcher.max_dist,
            aperture_radius_m: film.aperture_radius_m,
            focus_distance_m: film.focus_distance_m,
            shutter_open_seconds: f64::from(film.shutter_open_seconds),
            shutter_close_seconds: f64::from(film.shutter_close_seconds),
        });
    }
    let source_ids: Vec<_> = indices.iter().map(|&i| document.objects[i].id.clone()).collect();
    let asset = UsdAsset {
        frames_per_second: request.frames_per_second,
        start_time_code: times[0].0,
        end_time_code: times.last().unwrap().0,
        meshes: vec![UsdMesh {
            id: "selected_field".into(),
            source_entity_ids: source_ids.clone(),
            representation: UsdRepresentation::ComposedSdfBake,
            double_sided: false,
            frames,
        }],
        materials,
        camera: Some(UsdCamera { id: "camera".into(), frames: cameras }),
    };
    let bytes = usd::encode_usda(&asset)?;
    if bytes.len() > 512 * 1024 * 1024 {
        return Err("USD exceeds 512 MiB encoded output".into());
    }
    let fingerprint = mm3e_kit::atoms::hash(&serde_json::to_vec(document).map_err(|e| e.to_string())?);
    let mut report = json!({"format":"usda","representation":"evaluated_composed_field_mesh_cache","source_entity_ids":source_ids,
        "source_fnv1a64":format!("{fingerprint:016x}"),"frames":reports,"resolution":request.resolution,"comparison_resolution":coarse,
        "estimated_field_work":work,"boolean_arrangement_work":arrangement_work,"source_feature_work":source_feature_work,"charged_work_total":work+arrangement_work+source_feature_work,
        "field_work_semantics":"Single-scalar extraction and native validation precharge counted scene/BVH units; estimated_field_work also includes UV transfer and final geometric intersection validation. Classic multi-channel extraction uses its scalar-program estimate plus separate Boolean arrangement work. source_feature_work is an exclusive ledger for feature construction, local source queries/convex operations, bounded conditioning, exact coplanar and bounded warped-patch retessellation, native refinement and provenance. All three ledgers share the same aggregate cap; units are not CPU instructions or a wall-time guarantee",
        "units":"meters","up_axis":"Y","color_encoding":"scene-linear Rec.709/D65 RGB","camera_near_clip_m":request.camera_near_clip_m,
        "semantics":["World-space points and topology sampled at authored time codes; between-frame reader interpolation is not validated",
            "Selection composes in original document order; unselected global subtractors do not participate",
            "Full thickened fields are meshed, including native triangle sheets; no primitive or midsurface replacement",
            "Hard Boolean boundaries are kept as separate affine constituent planes within tetrahedra; smooth subtrees remain exact-sampled opaque scalar channels",
            "Boolean vertices are corrected against the sampled source curves with bounded projection and conforming local refinement; displacement and extra sampling are reported",
            "Each face uses the material owner at its centroid; exact material boundaries across a face are approximated",
            "The consumer camera uses camera_near_clip_m; native tracing starts at the eye",
            "Convergence and scalar residual checks are finite samples, not a Hausdorff or missing-feature certificate"],
        "not_delivered":["native SDF edit operators","skeletons or skin weights","simulation settings","UVs/textures","lights/environment","color-management configuration"]});
    if textured {
        report["texture_delivery"] = json!({"options":request.texture_delivery,"conversions":assets.conversions,
            "asset_directory":asset_directory,"asset_files":assets.files.len(),"filtering":"reader_defined",
            "semantics":"Source images retained; color/alpha modulation converted to opaque linear EXR. Native texture LOD/filter and renderer BRDF are not reproduced by the USD consumer."});
        report["not_delivered"] = json!([
            "native SDF edit operators",
            "skeletons or skin weights",
            "simulation settings",
            "native filtering/LOD",
            "lights/environment",
            "color-management configuration"
        ]);
    }
    Ok(EncodedUsd { bytes, report, assets: assets.files, asset_directory })
}

#[cfg(test)]
mod publication_tests {
    use super::*;
    use crate::usd::{UsdTexture, UsdTextureChannel, UsdTextureColorSpace, UsdTextureRole, UsdTextureWrap};

    struct Root(PathBuf);
    impl Root {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "mm3e-usd-publish-{}-{}",
                std::process::id(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
            ));
            fs::create_dir(&path).unwrap();
            Self(path.canonicalize().unwrap())
        }
    }
    impl Drop for Root {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn payload() -> EncodedUsd {
        let mut source = vec![];
        {
            let mut encoder = png::Encoder::new(&mut source, 1, 1);
            encoder.set_color(png::ColorType::Rgb);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&[128, 64, 255]).unwrap();
            writer.finish().unwrap();
        }
        let asset = UsdAsset {
            frames_per_second: 24.0,
            start_time_code: 0.0,
            end_time_code: 0.0,
            meshes: vec![UsdMesh {
                id: "sheet".into(),
                source_entity_ids: vec!["sheet".into()],
                representation: UsdRepresentation::NativeSurface,
                double_sided: true,
                frames: vec![FrameMesh {
                    time: 0.0,
                    positions: vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
                    triangles: vec![[0, 1, 2]],
                    material_ids: vec![0],
                    corner_uvs: Some(vec![[[0., 0.], [1., 0.], [0., 1.]]]),
                }],
            }],
            materials: vec![UsdMaterial {
                id: "surface".into(),
                diffuse_color: [1.; 3],
                roughness: 0.5,
                metallic: 0.0,
                ior: 1.5,
                emissive_color: [0.; 3],
                textures: vec![UsdTexture {
                    role: UsdTextureRole::Diffuse,
                    asset_path: "assets/source.png".into(),
                    channel: UsdTextureChannel::Rgb,
                    source_color_space: UsdTextureColorSpace::Raw,
                    wrap_s: UsdTextureWrap::Repeat,
                    wrap_t: UsdTextureWrap::Repeat,
                    scale: [1.; 4],
                    bias: [0.; 4],
                }],
            }],
            camera: None,
        };
        EncodedUsd {
            bytes: usd::encode_usda(&asset).unwrap(),
            report: json!({"test":"actual encoded textured triangle"}),
            assets: BTreeMap::from([("source.png".into(), source)]),
            asset_directory: Some("assets".into()),
        }
    }
    fn request(path: &str) -> ExportUsdRequest {
        serde_json::from_value(json!({"path":path,"object_ids":["sheet"],"bounds_min":[-1,-1,-1],"bounds_max":[2,2,1],
            "resolution":[8,8,8],"max_surface_error_m":0.1,"max_field_residual":0.1,
            "texture_delivery":{"filtering":"reader_defined"}}))
        .unwrap()
    }

    #[cfg(unix)]
    #[test]
    fn changing_an_in_root_parent_alias_does_not_separate_layer_from_its_assets() {
        use std::os::unix::fs::symlink;
        let root = Root::new();
        let a = root.0.join("A");
        let b = root.0.join("B");
        fs::create_dir(&a).unwrap();
        fs::create_dir(&b).unwrap();
        symlink("A", root.0.join("alias")).unwrap();
        let request = request("alias/scene.usda");
        let target = storage::path(&root.0, &request.path, false).unwrap();
        let encoded = payload();
        let wanted_layer = encoded.bytes.clone();
        let wanted_source = encoded.assets["source.png"].clone();
        // This is the exact state change possible while extraction runs. No
        // timing race or production hook is needed at the publisher boundary.
        fs::remove_file(root.0.join("alias")).unwrap();
        symlink("B", root.0.join("alias")).unwrap();
        let result = publish_encoded(&root.0, target, &request, encoded).unwrap();
        assert_eq!(result["path"], json!(a.join("scene.usda")));
        assert_eq!(result["asset_directory"], json!(a.join("assets")));
        assert_eq!(fs::read(a.join("scene.usda")).unwrap(), wanted_layer);
        assert_eq!(fs::read(a.join("assets/source.png")).unwrap(), wanted_source);
        assert_eq!(fs::read_dir(&b).unwrap().count(), 0);
        let manifest: Value = serde_json::from_slice(&fs::read(a.join("assets/manifest.json")).unwrap()).unwrap();
        assert_eq!(manifest["layer"], "scene.usda");
        assert_eq!(manifest["layer_sha256"], format!("{:x}", Sha256::digest(&wanted_layer)));
        assert_eq!(manifest["assets"][0]["sha256"], format!("{:x}", Sha256::digest(&wanted_source)));
    }

    #[test]
    fn competing_layer_install_preserves_the_winner_and_owned_partial_bundle() {
        let root = Root::new();
        let request = request("scene.usda");
        let target = storage::path(&root.0, &request.path, false).unwrap();
        let encoded = payload();
        let expected_source = encoded.assets["source.png"].clone();
        fs::write(&target, b"concurrent owner").unwrap();
        let error = publish_encoded(&root.0, target, &request, encoded).unwrap_err();
        assert!(error.message.contains("assets preserved"));
        assert_eq!(fs::read(root.0.join("scene.usda")).unwrap(), b"concurrent owner");
        assert_eq!(fs::read(root.0.join("assets/source.png")).unwrap(), expected_source);
        let manifest: Value = serde_json::from_slice(&fs::read(root.0.join("assets/manifest.json")).unwrap()).unwrap();
        assert!(manifest["completion"].as_str().unwrap().contains("alone does not prove"));
        assert_ne!(manifest["layer_sha256"], format!("{:x}", Sha256::digest(b"concurrent owner")));
    }
}
