//! Durable deformation of native triangle surfaces. Morph deltas are authored in the
//! source object's rest-local coordinates, then transformed to rest-world coordinates
//! before skinning by the animation system's joint deltas. Source geometry is never baked.
use crate::{
    animation::{self, AnimationSample, Clip, Evaluation, Interpolation, Target},
    model::{array, identifier, range, vec, vector, Document, Entity, Pass, Shape, V3},
};
use mm3e_kit::{
    deform as kernel,
    surface::TriangleSurface,
    vec::{Mat3, Transform},
};
use mm3e_orchestrator::{anim::Track, Prim, Scene};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_DEFORMERS: usize = 64;
pub const MAX_TOTAL_VERTICES: usize = 262_144;
pub const MAX_TOTAL_MORPH_DELTAS: usize = 1_048_576;
pub const MAX_TOTAL_WORK: usize = 4_194_304;
pub const MAX_MORPH_TRACKS: usize = 2048;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkinningMethod {
    #[default]
    LinearBlend,
    /// Rigid dual quaternion blending. Scaled joint motion is rejected, never approximated.
    DualQuaternion,
}
impl SkinningMethod {
    fn kernel(self) -> kernel::SkinningMethod {
        match self {
            Self::LinearBlend => kernel::SkinningMethod::LinearBlend,
            Self::DualQuaternion => kernel::SkinningMethod::DualQuaternion,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct JointWeight {
    /// Index into this asset's explicit joint palette, not document joint array order.
    pub joint: u32,
    pub weight: f32,
}
fn one() -> f32 {
    1.0
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BlendShape {
    pub id: String,
    /// Dense rest-local displacement vectors in exact source vertex order.
    pub deltas: Vec<V3>,
    #[serde(default)]
    pub weight: f32,
    #[serde(default)]
    pub min_weight: f32,
    #[serde(default = "one")]
    pub max_weight: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeformerAsset {
    pub id: String,
    /// Existing authored native Surface object. Geometry and material remain in objects.
    pub object: String,
    #[serde(default)]
    pub method: SkinningMethod,
    #[serde(default)]
    pub joints: Vec<String>,
    /// One row per source vertex; rows contain 1..8 distinct positive influences,
    /// summing to one within 1e-6. Morph-only assets use empty joints and empty weights,
    /// and execute the kernel's explicit no-skin mode.
    #[serde(default)]
    pub weights: Vec<Vec<JointWeight>>,
    #[serde(default)]
    pub blendshapes: Vec<BlendShape>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BindSurface {
    pub deformer: DeformerAsset,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateDeformer {
    pub id: String,
    #[serde(default)]
    pub method: Option<SkinningMethod>,
    #[serde(default)]
    pub joints: Option<Vec<String>>,
    #[serde(default)]
    pub weights: Option<Vec<Vec<JointWeight>>>,
    #[serde(default)]
    pub blendshapes: Option<Vec<BlendShape>>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RemoveDeformer {
    pub id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MorphKey {
    pub time: f32,
    pub weight: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MorphTrack {
    pub deformer: String,
    pub blendshape: String,
    pub keys: Vec<MorphKey>,
    #[serde(default)]
    pub easing: Interpolation,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MorphWeight {
    pub id: String,
    pub weight: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeformerState {
    pub id: String,
    pub object: String,
    pub method: SkinningMethod,
    pub animation: Option<AnimationSample>,
    pub evaluated_time: Option<f32>,
    /// Exact evaluated world-space positions used by the native triangle field.
    pub vertices: Vec<V3>,
    pub triangles: Vec<[u32; 3]>,
    /// Rest object scale is incorporated; skinning retains this world-space shell thickness.
    pub thickness_m: f32,
    /// Bounds of vertex centers. The actual field additionally includes half shell thickness.
    pub bounds: [V3; 2],
    pub max_displacement_m: f64,
    pub morph_weights: Vec<MorphWeight>,
}

pub fn owns_object(document: &Document, id: &str) -> bool {
    document.deformers.iter().any(|asset| asset.object == id)
}
fn entity<'a>(document: &'a Document, asset: &DeformerAsset) -> Result<&'a Entity, String> {
    document
        .objects
        .iter()
        .find(|e| e.id == asset.object)
        .ok_or_else(|| format!("missing deformer object {}", asset.object))
}
fn rest_transform(source: &Entity) -> Transform {
    let r = source.rotation_degrees.map(f32::to_radians);
    Transform::new(vec(source.position), Mat3::from_euler(r[0], r[1], r[2]), source.scale)
}

/// Upper bound on dense vertex, morph, and influence evaluation work in a single pose.
/// Saturation makes this safe to inspect before document validation.
pub fn work(document: &Document) -> usize {
    document.deformers.iter().fold(0usize, |total, asset| {
        let vertices = document
            .objects
            .iter()
            .find(|e| e.id == asset.object)
            .and_then(|e| match &e.shape {
                Shape::Surface { vertices, .. } => Some(vertices.len()),
                _ => None,
            })
            .unwrap_or(0);
        total
            .saturating_add(vertices)
            .saturating_add(asset.weights.iter().fold(0usize, |n, row| n.saturating_add(row.len())))
            .saturating_add(asset.blendshapes.iter().fold(0usize, |n, morph| n.saturating_add(morph.deltas.len())))
    })
}

pub fn validate(document: &Document) -> Result<(), String> {
    if document.deformers.len() > MAX_DEFORMERS || work(document) > MAX_TOTAL_WORK {
        return Err("document exceeds deformer count/work budget".into());
    }
    let joints: BTreeMap<_, _> = document.joints.iter().map(|j| (j.id.as_str(), j)).collect();
    let mut ids = BTreeSet::new();
    let mut owners = BTreeSet::new();
    let mut vertices_total = 0usize;
    let mut deltas_total = 0usize;
    for asset in &document.deformers {
        identifier(&asset.id)?;
        identifier(&asset.object)?;
        if !ids.insert(asset.id.as_str()) || !owners.insert(asset.object.as_str()) {
            return Err("duplicate deformer id or multiple deformers own one surface".into());
        }
        let source = entity(document, asset)?;
        let Shape::Surface { vertices, .. } = &source.shape else {
            return Err(format!("deformer {} requires an authored native surface", asset.id));
        };
        if !(3..=kernel::MAX_DEFORM_VERTICES).contains(&vertices.len()) {
            return Err("deformer source vertex count is outside the supported budget".into());
        }
        vertices_total = vertices_total.saturating_add(vertices.len());
        if vertices_total > MAX_TOTAL_VERTICES {
            return Err("document exceeds total deformed vertex budget".into());
        }
        if document.cloths.iter().any(|c| c.id == source.id)
            || document.garments.iter().any(|g| g.id == source.id)
            || document.faces.iter().any(|f| f.generated_object_ids.contains(&source.id))
        {
            return Err(
                "deformation cannot share generated geometry ownership with cloth, face, or fitted garments".into()
            );
        }
        if document.joints.iter().any(|j| j.objects.contains(&source.id)) {
            return Err("a deformed surface cannot also have a rigid joint object binding".into());
        }
        if document
            .clips
            .iter()
            .any(|c| c.tracks.iter().any(|t| matches!(&t.target, Target::Object { id } if id == &source.id)))
        {
            return Err(
                "animate a deformed surface through its joints or morph weights, not an object transform track".into(),
            );
        }
        let m = &source.modifiers;
        if m.mirror.iter().any(|b| *b) || m.elongate != [0.0; 3] || m.round != 0.0 || m.onion != 0.0 {
            return Err("deformed surfaces require neutral source field modifiers".into());
        }
        if asset.joints.len() > kernel::MAX_DEFORM_JOINTS || asset.blendshapes.len() > kernel::MAX_BLEND_SHAPES {
            return Err("deformer exceeds joint palette or blendshape budget".into());
        }
        let mut palette = BTreeSet::new();
        for id in &asset.joints {
            if !joints.contains_key(id.as_str()) || !palette.insert(id.as_str()) {
                return Err(format!("deformer joint palette has a missing or duplicate joint {id}"));
            }
        }
        if asset.joints.is_empty() {
            if !asset.weights.is_empty() {
                return Err("morph-only deformer requires empty joints and weights".into());
            }
        } else {
            if asset.weights.len() != vertices.len() {
                return Err("deformer weights must match the source vertex count".into());
            }
            for (index, row) in asset.weights.iter().enumerate() {
                if row.is_empty() || row.len() > kernel::MAX_VERTEX_INFLUENCES {
                    return Err(format!("deformer vertex {index} requires 1..8 influences"));
                }
                let mut used = BTreeSet::new();
                let mut sum = 0.0f64;
                for influence in row {
                    if influence.joint as usize >= asset.joints.len() || !used.insert(influence.joint) {
                        return Err(format!("deformer vertex {index} has a duplicate or out-of-range joint index"));
                    }
                    if !influence.weight.is_finite() || influence.weight <= 0.0 {
                        return Err(format!("deformer vertex {index} weights must be finite and positive"));
                    }
                    sum += f64::from(influence.weight);
                }
                if (sum - 1.0).abs() > kernel::WEIGHT_SUM_TOLERANCE {
                    return Err(format!("deformer vertex {index} weights must sum to one within 1e-6"));
                }
            }
        }
        let mut morphs = BTreeSet::new();
        for morph in &asset.blendshapes {
            identifier(&morph.id)?;
            if !morphs.insert(&morph.id) || morph.deltas.len() != vertices.len() {
                return Err("blendshapes require distinct ids and one rest-local delta per source vertex".into());
            }
            deltas_total = deltas_total.saturating_add(morph.deltas.len());
            if deltas_total > MAX_TOTAL_MORPH_DELTAS {
                return Err("document exceeds total blendshape delta budget".into());
            }
            range(morph.min_weight, -1000.0, 1000.0, "blendshape minimum weight")?;
            range(morph.max_weight, morph.min_weight, 1000.0, "blendshape maximum weight")?;
            range(morph.weight, morph.min_weight, morph.max_weight, "blendshape rest weight")?;
            for delta in &morph.deltas {
                vector(*delta, "blendshape rest-local delta")?;
            }
        }
        if asset.method == SkinningMethod::DualQuaternion {
            // Include all ancestors: their scales compose into each palette joint delta.
            let mut relevant = BTreeSet::new();
            for id in &asset.joints {
                let mut current = Some(id.as_str());
                while let Some(id) = current {
                    if !relevant.insert(id) {
                        break;
                    }
                    current = joints.get(id).and_then(|joint| joint.parent.as_deref());
                }
            }
            if document.clips.iter().any(|c| {
                c.tracks.iter().any(|t| {
                    matches!(&t.target, Target::Joint { id } if relevant.contains(id.as_str()))
                        && t.keys.iter().any(|k| k.scale != 1.0)
                })
            }) {
                return Err(
                    "dual quaternion deformation requires unit scale on palette joints and their ancestors".into()
                );
            }
        }
        let Shape::Surface { triangles, thickness_m, .. } = &source.shape else { unreachable!() };
        let (rest, _) = evaluate_asset(document, asset, None, None, None)?;
        TriangleSurface::new(rest.positions, triangles.clone(), *thickness_m * source.scale * 0.5)
            .map_err(|e| format!("deformer {} default evaluated surface: {e}", asset.id))?;
    }
    for clip in &document.clips {
        validate_tracks(document, clip)?;
    }
    Ok(())
}

pub fn validate_tracks(document: &Document, clip: &Clip) -> Result<(), String> {
    if clip.morph_tracks.len() > MAX_MORPH_TRACKS {
        return Err("clip exceeds morph track budget".into());
    }
    let mut channels = BTreeSet::new();
    for track in &clip.morph_tracks {
        if !channels.insert((&track.deformer, &track.blendshape)) {
            return Err("clip has duplicate morph weight channels".into());
        }
        let asset = document
            .deformers
            .iter()
            .find(|a| a.id == track.deformer)
            .ok_or_else(|| format!("morph track references missing deformer {}", track.deformer))?;
        let morph = asset
            .blendshapes
            .iter()
            .find(|m| m.id == track.blendshape)
            .ok_or_else(|| format!("morph track references missing blendshape {}", track.blendshape))?;
        animation::key_times(track.keys.iter().map(|k| k.time), clip.duration)?;
        for key in &track.keys {
            range(key.weight, morph.min_weight, morph.max_weight, "animated blendshape weight")?;
        }
    }
    Ok(())
}

pub fn bind(document: &mut Document, request: BindSurface) -> Result<(), String> {
    let mut candidate = document.clone();
    candidate.deformers.push(request.deformer);
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}
pub fn update(document: &mut Document, request: UpdateDeformer) -> Result<(), String> {
    let mut candidate = document.clone();
    let asset = candidate
        .deformers
        .iter_mut()
        .find(|a| a.id == request.id)
        .ok_or_else(|| format!("missing deformer {}", request.id))?;
    if let Some(method) = request.method {
        asset.method = method;
    }
    if let Some(joints) = request.joints {
        asset.joints = joints;
    }
    if let Some(weights) = request.weights {
        asset.weights = weights;
    }
    if let Some(blendshapes) = request.blendshapes {
        asset.blendshapes = blendshapes;
    }
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}

/// Patch named default controls without retransmitting immutable dense morph deltas.
/// Clip tracks still override these defaults when the same channel is animated.
pub fn set_morph_weights(document: &mut Document, id: &str, weights: Vec<MorphWeight>) -> Result<(), String> {
    if weights.is_empty() || weights.len() > kernel::MAX_BLEND_SHAPES {
        return Err("set_morph_weights requires 1..64 distinct controls".into());
    }
    let mut candidate = document.clone();
    let asset =
        candidate.deformers.iter_mut().find(|asset| asset.id == id).ok_or_else(|| format!("missing deformer {id}"))?;
    let mut seen = BTreeSet::new();
    for control in weights {
        if !seen.insert(control.id.clone()) {
            return Err("duplicate default morph control".into());
        }
        let target = asset
            .blendshapes
            .iter_mut()
            .find(|shape| shape.id == control.id)
            .ok_or_else(|| format!("missing blendshape {}", control.id))?;
        range(control.weight, target.min_weight, target.max_weight, "default morph weight")?;
        target.weight = control.weight;
    }
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}
/// Remove the deformation and its scalar channels, preserving the original surface.
pub fn remove(document: &mut Document, id: &str) -> Result<(), String> {
    if !document.deformers.iter().any(|a| a.id == id) {
        return Err(format!("missing deformer {id}"));
    }
    let mut candidate = document.clone();
    candidate.deformers.retain(|a| a.id != id);
    for clip in &mut candidate.clips {
        clip.morph_tracks.retain(|t| t.deformer != id);
        for layer in &mut clip.layers {
            if let Some(mask) = &mut layer.mask {
                mask.retain(
                    |channel| !matches!(channel, crate::layering::Channel::Morph { deformer, .. } if deformer == id),
                );
            }
        }
    }
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}

fn layered_weights(document: &Document, at: Option<(&Clip, f32)>) -> Result<Option<crate::layering::Resolved>, String> {
    at.filter(|(clip, _)| !clip.layers.is_empty())
        .map(|(clip, time)| crate::layering::resolve(document, clip, time))
        .transpose()
}

fn sampled_weights(
    asset: &DeformerAsset,
    at: Option<(&Clip, f32)>,
    resolved: Option<&crate::layering::Resolved>,
) -> Result<Vec<f32>, String> {
    asset
        .blendshapes
        .iter()
        .map(|morph| {
            let mut weight = morph.weight;
            if let Some(resolved) = resolved {
                if let Some(&value) = resolved.morphs.get(&(asset.id.clone(), morph.id.clone())) {
                    weight = value;
                }
            } else if let Some((clip, time)) = at {
                if let Some(track) =
                    clip.morph_tracks.iter().find(|t| t.deformer == asset.id && t.blendshape == morph.id)
                {
                    let mut scalar = Track::new(track.easing.engine());
                    for key in &track.keys {
                        scalar = scalar.key(key.time, key.weight);
                    }
                    weight = scalar.try_sample(time).ok_or("cannot sample an empty morph track")?;
                }
            }
            range(weight, morph.min_weight, morph.max_weight, "evaluated blendshape weight")?;
            Ok(weight)
        })
        .collect()
}

fn joint_evaluation(document: &Document, sample: Option<&AnimationSample>) -> Result<Option<Evaluation>, String> {
    sample.map(|sample| animation::evaluate_joints(document, sample)).transpose()
}

fn evaluate_asset(
    document: &Document,
    asset: &DeformerAsset,
    at: Option<(&Clip, f32)>,
    evaluation: Option<&Evaluation>,
    resolved: Option<&crate::layering::Resolved>,
) -> Result<(kernel::Deformation, Vec<f32>), String> {
    let source = entity(document, asset)?;
    let Shape::Surface { vertices, .. } = &source.shape else {
        return Err("deformer source must be a surface".into());
    };
    let transform = rest_transform(source);
    let rest: Vec<_> = vertices.iter().map(|p| transform.to_world(vec(*p))).collect();
    let morphs: Vec<_> = asset
        .blendshapes
        .iter()
        .map(|m| kernel::BlendShape {
            deltas: m.deltas.iter().map(|d| transform.rot.mul_vec(vec(*d).scale(transform.scale))).collect(),
        })
        .collect();
    let weights = sampled_weights(asset, at, resolved)?;
    let (influences, deltas) = if asset.joints.is_empty() {
        (vec![], vec![])
    } else {
        let indices: BTreeMap<_, _> = document.joints.iter().enumerate().map(|(i, j)| (j.id.as_str(), i)).collect();
        let deltas = asset
            .joints
            .iter()
            .map(|id| {
                let index = *indices.get(id.as_str()).ok_or_else(|| format!("missing deformer joint {id}"))?;
                evaluation
                    .map(|e| e.joint_transforms.get(index).copied().ok_or("joint evaluation size mismatch".into()))
                    .unwrap_or(Ok(Transform::IDENTITY))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let influences = asset
            .weights
            .iter()
            .map(|row| row.iter().map(|w| kernel::Influence { joint: w.joint, weight: w.weight }).collect())
            .collect();
        (influences, deltas)
    };
    Ok((kernel::deform(&rest, &influences, &morphs, &weights, &deltas, asset.method.kernel())?, weights))
}

/// Refresh actual native fields atomically. Source vertices, triangles, transform and
/// animation data stay authored; final vertices are world-space and compiled xform is I.
pub fn refresh(
    document: &Document,
    scene: &mut Scene,
    sample: Option<&AnimationSample>,
    evaluation: Option<&Evaluation>,
) -> Result<(), String> {
    if document.deformers.is_empty() {
        return Ok(());
    }
    let at = sample.map(|s| animation::sample_time(document, s)).transpose()?;
    if let (Some((clip, time)), Some(evaluation)) = (at, evaluation) {
        if evaluation.clip != clip.id
            || evaluation.time != time
            || evaluation.joint_transforms.len() != document.joints.len()
            || evaluation.source_signature != animation::evaluation_signature(document)?
            || evaluation.payload_signature
                != animation::evaluation_payload_signature(evaluation.time, &evaluation.joint_transforms)
        {
            return Err(
                "deformation requires joint deltas from the same canonical animation sample and authored inputs".into(),
            );
        }
    }
    let computed = if sample.is_some() && evaluation.is_none() { joint_evaluation(document, sample)? } else { None };
    let evaluation = if sample.is_some() { evaluation.or(computed.as_ref()) } else { None };
    let inherited = evaluation.and_then(|evaluation| evaluation.resolved.as_ref());
    let computed_resolved = if inherited.is_none() { layered_weights(document, at)? } else { None };
    let resolved = inherited.or(computed_resolved.as_ref());
    let mut updates = Vec::with_capacity(document.deformers.len());
    for asset in &document.deformers {
        let source = entity(document, asset)?;
        let index =
            document.objects.iter().position(|e| e.id == asset.object).ok_or("missing deformed scene object")?;
        let object = scene.objects.get(index).ok_or("deformed scene object index mismatch")?;
        let Prim::Surface { id } = object.prim else {
            return Err("deformed compiled primitive must be a native surface".into());
        };
        if id as usize >= scene.surfaces.len() {
            return Err("deformed compiled surface index is invalid".into());
        }
        let Shape::Surface { triangles, thickness_m, .. } = &source.shape else {
            return Err("deformer source must be a surface".into());
        };
        let (result, _) = evaluate_asset(document, asset, at, evaluation, resolved)?;
        let surface = TriangleSurface::new(result.positions, triangles.clone(), *thickness_m * source.scale * 0.5)
            .map_err(|e| format!("deformer {} evaluated surface: {e}", asset.id))?;
        updates.push((index, id as usize, surface));
    }
    for (index, id, surface) in updates {
        scene.surfaces[id] = surface;
        scene.objects[index].xform = Transform::IDENTITY;
    }
    Ok(())
}

pub fn inspect(document: &Document, id: &str, sample: Option<&AnimationSample>) -> Result<Value, String> {
    validate(document)?;
    animation::validate(document)?;
    let asset = document.deformers.iter().find(|a| a.id == id).ok_or_else(|| format!("missing deformer {id}"))?;
    let source = entity(document, asset)?;
    let Shape::Surface { triangles, thickness_m, .. } = &source.shape else {
        return Err("deformer source must be a surface".into());
    };
    let at = sample.map(|s| animation::sample_time(document, s)).transpose()?;
    let evaluation = joint_evaluation(document, sample)?;
    let inherited = evaluation.as_ref().and_then(|evaluation| evaluation.resolved.as_ref());
    let computed_resolved = if inherited.is_none() { layered_weights(document, at)? } else { None };
    let resolved = inherited.or(computed_resolved.as_ref());
    let (result, weights) = evaluate_asset(document, asset, at, evaluation.as_ref(), resolved)?;
    // The same topology acceptance as refresh prevents inspection reporting an unusable pose.
    TriangleSurface::new(result.positions.clone(), triangles.clone(), *thickness_m * source.scale * 0.5)?;
    let state = DeformerState {
        id: asset.id.clone(),
        object: asset.object.clone(),
        method: asset.method,
        animation: sample.cloned(),
        evaluated_time: at.map(|(_, time)| time),
        vertices: result.positions.into_iter().map(array).collect(),
        triangles: triangles.clone(),
        thickness_m: *thickness_m * source.scale,
        bounds: [array(result.bounds.0), array(result.bounds.1)],
        max_displacement_m: result.max_displacement,
        morph_weights: asset
            .blendshapes
            .iter()
            .zip(weights)
            .map(|(m, weight)| MorphWeight { id: m.id.clone(), weight })
            .collect(),
    };
    serde_json::to_value(state).map_err(|e| e.to_string())
}

/// Compact pose information taken from the same already refreshed native scene.
pub fn summaries(document: &Document, scene: &Scene, sample: Option<&AnimationSample>) -> Result<Value, String> {
    let at = sample.map(|s| animation::sample_time(document, s)).transpose()?;
    let resolved = layered_weights(document, at)?;
    let mut result = vec![];
    for asset in &document.deformers {
        let index =
            document.objects.iter().position(|e| e.id == asset.object).ok_or("missing deformed scene object")?;
        let Prim::Surface { id } = scene.objects.get(index).ok_or("deformed scene object index mismatch")?.prim else {
            return Err("deformed compiled primitive must be a native surface".into());
        };
        let surface = scene.surfaces.get(id as usize).ok_or("deformed compiled surface index is invalid")?;
        let weights = sampled_weights(asset, at, resolved.as_ref())?;
        result.push(serde_json::json!({"id":asset.id,"object":asset.object,"method":asset.method,
            "vertex_count":surface.vertices().len(),"triangle_count":surface.triangles().len(),
            "world_shell_bounds":[array(surface.bounds().0),array(surface.bounds().1)],
            "thickness_m":surface.half_thickness()*2.0,"evaluated_time":at.map(|(_,time)|time),
            "morph_weights":asset.blendshapes.iter().zip(weights).map(|(m,weight)|MorphWeight{id:m.id.clone(),weight}).collect::<Vec<_>>() }));
    }
    Ok(Value::Array(result))
}
