//! Durable animation policy and stateless pose evaluation. Key interpolation and transform
//! math reuse the engine. Joints bind rigid SDF parts using rest-world pivot deltas.
use crate::model::{array, identifier, range, vec, vector, Document, Pass, Shape, V3};
use mm3e_kit::{
    camera::Camera,
    vec::{Quat, Transform},
};
use mm3e_orchestrator::{
    anim::{Easing, Track},
    Scene,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_JOINTS: usize = 256;
pub const MAX_CLIPS: usize = 64;
pub const MAX_KEYS: usize = 65_536;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Joint {
    pub id: String,
    #[serde(default)]
    pub parent: Option<String>,
    /// Pivot in the document's WORLD coordinates at rest, not parent-local coordinates.
    pub pivot: V3,
    /// Each object can be bound to at most one joint. Descendants inherit ancestor motion.
    #[serde(default)]
    pub objects: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_limit: Option<crate::joint_limits::JointRotationLimit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Target {
    Object { id: String },
    Joint { id: String },
}

fn one() -> f32 {
    1.0
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TransformKey {
    pub time: f32,
    /// Delta from rest in world-rest axes, before the parent joint's posed transform.
    #[serde(default)]
    pub translation: V3,
    /// Delta from rest. Euler degrees use the engine's Rx*Ry*Rz convention, then quaternion slerp.
    #[serde(default)]
    pub rotation_degrees: V3,
    #[serde(default = "one")]
    pub scale: f32,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Interpolation {
    Step,
    #[default]
    Linear,
    SmoothStep,
    EaseIn,
    EaseOut,
}
impl Interpolation {
    pub(crate) fn engine(self) -> Easing {
        match self {
            Self::Step => Easing::Step,
            Self::Linear => Easing::Linear,
            Self::SmoothStep => Easing::SmoothStep,
            Self::EaseIn => Easing::EaseIn,
            Self::EaseOut => Easing::EaseOut,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MotionTrack {
    pub target: Target,
    pub keys: Vec<TransformKey>,
    #[serde(default)]
    pub easing: Interpolation,
    /// Object-only override in world-rest space; otherwise the object's rest position is used.
    /// Joint tracks always use the joint's pivot and reject this field when non-null.
    #[serde(default)]
    pub pivot: Option<V3>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CameraKey {
    pub time: f32,
    pub eye: V3,
    pub target: V3,
    pub fov_degrees: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Clip {
    pub id: String,
    pub duration: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<crate::layering::Layer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lip_sync: Option<Box<crate::lip_sync::LipSyncProvenance>>,
    #[serde(default)]
    pub tracks: Vec<MotionTrack>,
    #[serde(default)]
    pub face_tracks: Vec<crate::face::FaceTrack>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub morph_tracks: Vec<crate::deform::MorphTrack>,
    /// Absolute world camera values; the rest camera's up hint remains in force.
    #[serde(default)]
    pub camera_keys: Vec<CameraKey>,
    #[serde(default)]
    pub camera_easing: Interpolation,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Playback {
    #[default]
    Clamp,
    Loop,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AnimationSample {
    pub clip: String,
    /// Seconds. Clamp holds endpoints; loop uses Euclidean modulo (including negative time).
    pub time: f32,
    #[serde(default)]
    pub playback: Playback,
}

pub(crate) fn key_times(times: impl Iterator<Item = f32>, duration: f32) -> Result<(), String> {
    let mut previous = None;
    let mut count = 0;
    for time in times {
        range(time, 0.0, duration, "key time")?;
        if previous.is_some_and(|p| time <= p) {
            return Err("key times must be strictly increasing and distinct".into());
        }
        previous = Some(time);
        count += 1;
    }
    if count == 0 || count > 1024 {
        return Err("a track must contain 1..1024 keys".into());
    }
    Ok(())
}

fn joint_order(joints: &[Joint]) -> Result<Vec<usize>, String> {
    let indices: BTreeMap<&str, usize> = joints.iter().enumerate().map(|(i, j)| (j.id.as_str(), i)).collect();
    let mut order = vec![];
    let mut marks = vec![0u8; joints.len()];
    fn visit(
        i: usize,
        joints: &[Joint],
        indices: &BTreeMap<&str, usize>,
        marks: &mut [u8],
        order: &mut Vec<usize>,
    ) -> Result<(), String> {
        match marks[i] {
            1 => return Err(format!("joint hierarchy cycle at {}", joints[i].id)),
            2 => return Ok(()),
            _ => {}
        }
        marks[i] = 1;
        if let Some(parent) = &joints[i].parent {
            let p = *indices
                .get(parent.as_str())
                .ok_or_else(|| format!("joint {}: missing parent {parent}", joints[i].id))?;
            visit(p, joints, indices, marks, order)?;
        }
        marks[i] = 2;
        order.push(i);
        Ok(())
    }
    for i in 0..joints.len() {
        visit(i, joints, &indices, &mut marks, &mut order)?;
    }
    Ok(order)
}

pub fn validate(document: &Document) -> Result<(), String> {
    if document.joints.len() > MAX_JOINTS || document.clips.len() > MAX_CLIPS {
        return Err("animation exceeds joint/clip budget".into());
    }
    let objects: BTreeSet<&str> = document.objects.iter().map(|e| e.id.as_str()).collect();
    let mut joints = BTreeSet::new();
    let mut bindings = BTreeSet::new();
    for joint in &document.joints {
        identifier(&joint.id)?;
        vector(joint.pivot, "joint pivot")?;
        if let Some(limit) = &joint.rotation_limit {
            limit.validate().map_err(|e| format!("joint {}: {e}", joint.id))?;
        }
        if !joints.insert(joint.id.as_str()) {
            return Err(format!("duplicate joint id {}", joint.id));
        }
        for object in &joint.objects {
            if !objects.contains(object.as_str()) {
                return Err(format!("joint {}: missing object {object}", joint.id));
            }
            if !bindings.insert(object.as_str()) {
                return Err(format!("object {object} is bound more than once"));
            }
        }
    }
    joint_order(&document.joints)?;
    let mut clip_ids = BTreeSet::new();
    let mut total_keys = 0usize;
    for clip in &document.clips {
        identifier(&clip.id)?;
        if !clip_ids.insert(&clip.id) {
            return Err(format!("duplicate clip id {}", clip.id));
        }
        range(clip.duration, 0.001, 3600.0, "clip duration")?;
        if let Some(provenance) = &clip.lip_sync {
            crate::lip_sync::validate_provenance(provenance)?;
        }
        if clip.tracks.len() > 512 {
            return Err("a clip supports at most 512 transform tracks".into());
        }
        let mut targets = BTreeSet::new();
        for track in &clip.tracks {
            if !targets.insert(&track.target) {
                return Err(format!("clip {} has competing tracks for {:?}", clip.id, track.target));
            }
            match &track.target {
                Target::Object { id } if !objects.contains(id.as_str()) => {
                    return Err(format!("missing animation object {id}"))
                }
                Target::Joint { id } if !joints.contains(id.as_str()) => {
                    return Err(format!("missing animation joint {id}"))
                }
                Target::Joint { .. } if track.pivot.is_some() => {
                    return Err("joint tracks use the joint pivot; a track pivot is invalid".into())
                }
                _ => {}
            }
            if let Some(pivot) = track.pivot {
                vector(pivot, "track pivot")?;
            }
            key_times(track.keys.iter().map(|k| k.time), clip.duration)?;
            for key in &track.keys {
                vector(key.translation, "key translation")?;
                vector(key.rotation_degrees, "key rotation_degrees")?;
                range(key.scale, 0.001, 1000.0, "key scale")?;
            }
            total_keys += track.keys.len();
        }
        if !clip.camera_keys.is_empty() {
            key_times(clip.camera_keys.iter().map(|k| k.time), clip.duration)?;
            for key in &clip.camera_keys {
                let mut view = document.camera.clone();
                view.eye = key.eye;
                view.target = key.target;
                view.fov_degrees = key.fov_degrees;
                view.validate()?;
            }
        }
        total_keys += clip.camera_keys.len();
        crate::face::validate_tracks(document, clip)?;
        total_keys += clip.face_tracks.iter().map(|track| track.keys.len()).sum::<usize>();
        crate::deform::validate_tracks(document, clip)?;
        total_keys += clip.morph_tracks.iter().map(|track| track.keys.len()).sum::<usize>();
        if total_keys > MAX_KEYS {
            return Err("document exceeds total animation key budget".into());
        }
    }
    crate::layering::validate(document)
}

pub(crate) fn delta(track: &MotionTrack, time: f32, pivot: V3) -> Transform {
    let easing = track.easing.engine();
    let mut translations = Track::new(easing);
    let mut rotations = Track::new(easing);
    let mut scales = Track::new(easing);
    for key in &track.keys {
        let angles = key.rotation_degrees.map(f32::to_radians);
        translations = translations.key(key.time, vec(key.translation));
        rotations = rotations.key(key.time, Quat::from_euler(angles[0], angles[1], angles[2]));
        scales = scales.key(key.time, key.scale);
    }
    Transform::around_pivot(vec(pivot), rotations.sample(time), scales.sample(time), translations.sample(time))
}

fn valid_transform(t: &Transform) -> Result<(), String> {
    range(t.scale, 1e-8, 1e8, "evaluated scale")?;
    for value in array(t.pos) {
        range(value, -1e8, 1e8, "evaluated position")?;
    }
    for column in t.rot.cols {
        for value in array(column) {
            range(value, -1.0001, 1.0001, "evaluated rotation basis")?;
        }
    }
    Ok(())
}

pub struct Evaluation {
    pub time: f32,
    pub joint_transforms: Vec<Transform>,
    pub(crate) resolved: Option<crate::layering::Resolved>,
    pub(crate) clip: String,
    pub(crate) source_signature: u64,
    pub(crate) payload_signature: u64,
    pub(crate) limit_observations: Vec<Value>,
}

/// Seal the public evaluated payload against accidental mutation before reuse.
pub(crate) fn evaluation_payload_signature(time: f32, transforms: &[Transform]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    let mut add = |v: f32| {
        for b in v.to_le_bytes() {
            hash = (hash ^ u64::from(b)).wrapping_mul(0x100000001b3);
        }
    };
    add(time);
    for t in transforms {
        for v in array(t.pos) {
            add(v);
        }
        add(t.scale);
        for column in t.rot.cols {
            for v in array(column) {
                add(v);
            }
        }
    }
    hash
}

/// Provenance for reusing evaluated channels: omit geometry/material payloads that
/// do not influence animation, but retain every authored animation input.
pub(crate) fn evaluation_signature(document: &Document) -> Result<u64, String> {
    let anchors: Vec<_> = document.objects.iter().map(|o| (&o.id, o.position)).collect();
    let faces: Vec<_> = document.faces.iter().map(|f| (&f.id, &f.controls)).collect();
    let morphs: Vec<_> = document
        .deformers
        .iter()
        .map(|d| {
            (&d.id, d.blendshapes.iter().map(|m| (&m.id, m.weight, m.min_weight, m.max_weight)).collect::<Vec<_>>())
        })
        .collect();
    let clips: Vec<_> = document
        .clips
        .iter()
        .map(|c| {
            (&c.id, c.duration, &c.tracks, &c.face_tracks, &c.morph_tracks, &c.camera_keys, c.camera_easing, &c.layers)
        })
        .collect();
    let bytes = serde_json::to_vec(&(&document.joints, clips, anchors, &document.camera, faces, morphs))
        .map_err(|e| e.to_string())?;
    Ok(mm3e_kit::atoms::hash(&bytes))
}

/// Resolve one canonical clip time for transforms, morphs and dependent simulation.
pub fn sample_time<'a>(document: &'a Document, sample: &AnimationSample) -> Result<(&'a Clip, f32), String> {
    range(sample.time, -86_400.0, 86_400.0, "animation time")?;
    let clip = document
        .clips
        .iter()
        .find(|clip| clip.id == sample.clip)
        .ok_or_else(|| format!("missing clip {}", sample.clip))?;
    range(clip.duration, 0.001, 3600.0, "clip duration")?;
    let time = match sample.playback {
        Playback::Clamp => sample.time.clamp(0.0, clip.duration),
        Playback::Loop => sample.time.rem_euclid(clip.duration),
    };
    Ok((clip, time))
}

/// Joint deltas in document joint order, independent of cameras and object geometry.
pub fn evaluate_joints(document: &Document, sample: &AnimationSample) -> Result<Evaluation, String> {
    let (clip, time) = sample_time(document, sample)?;
    let resolved = (!clip.layers.is_empty()).then(|| crate::layering::resolve(document, clip, time)).transpose()?;
    let tracks: BTreeMap<&Target, &MotionTrack> = clip.tracks.iter().map(|track| (&track.target, track)).collect();
    let indices: BTreeMap<&str, usize> = document.joints.iter().enumerate().map(|(i, j)| (j.id.as_str(), i)).collect();
    let mut joint_transforms = vec![Transform::IDENTITY; document.joints.len()];
    let mut limit_observations = vec![];
    for i in joint_order(&document.joints)? {
        let joint = &document.joints[i];
        let parent =
            joint.parent.as_ref().map(|id| joint_transforms[indices[id.as_str()]]).unwrap_or(Transform::IDENTITY);
        let target = Target::Joint { id: joint.id.clone() };
        let local = if let Some(resolved) = &resolved {
            resolved.transforms.get(&target).copied().unwrap_or(Transform::IDENTITY)
        } else {
            tracks.get(&target).map(|track| delta(track, time, joint.pivot)).unwrap_or(Transform::IDENTITY)
        };
        let (local, observation) =
            crate::joint_limits::apply(joint, local).map_err(|e| format!("clip {} at {time}s: {e}", clip.id))?;
        if let Some(report) = observation {
            limit_observations.push(report);
        }
        joint_transforms[i] = parent.compose(local);
        valid_transform(&joint_transforms[i])?;
    }
    let payload_signature = evaluation_payload_signature(time, &joint_transforms);
    Ok(Evaluation {
        time,
        joint_transforms,
        payload_signature,
        limit_observations,
        resolved,
        clip: clip.id.clone(),
        source_signature: evaluation_signature(document)?,
    })
}

/// Apply a validated document's clip to an independent, freshly compiled rest scene.
/// Parent deltas compose with child rest-pivot deltas, then with each object's rest transform.
pub fn evaluate(
    document: &Document,
    scene: &mut Scene,
    camera: &mut Camera,
    sample: &AnimationSample,
) -> Result<Evaluation, String> {
    let (clip, time) = sample_time(document, sample)?;
    let evaluation = evaluate_joints(document, sample)?;
    let tracks: BTreeMap<&Target, &MotionTrack> = clip.tracks.iter().map(|t| (&t.target, t)).collect();
    let joint_transforms = &evaluation.joint_transforms;
    let mut bindings = BTreeMap::new();
    for (i, joint) in document.joints.iter().enumerate() {
        for object in &joint.objects {
            bindings.insert(object.as_str(), i);
        }
    }
    for (entity, object) in document.objects.iter().zip(&mut scene.objects) {
        let target = Target::Object { id: entity.id.clone() };
        let local = if let Some(resolved) = &evaluation.resolved {
            resolved.transforms.get(&target).copied().unwrap_or(Transform::IDENTITY)
        } else {
            tracks
                .get(&target)
                .map(|t| delta(t, time, t.pivot.unwrap_or(entity.position)))
                .unwrap_or(Transform::IDENTITY)
        };
        let parent = bindings.get(entity.id.as_str()).map(|i| joint_transforms[*i]).unwrap_or(Transform::IDENTITY);
        if tracks.contains_key(&target)
            || evaluation.resolved.as_ref().is_some_and(|r| r.transforms.contains_key(&target))
            || bindings.contains_key(entity.id.as_str())
        {
            let combined = parent.compose(local);
            // A bind with no motion must retain the original basis exactly. Re-normalizing
            // an arbitrary rest rotation can change f32 coefficients and rendered pixels.
            let identity = combined.pos == mm3e_kit::vec::Vec3::ZERO
                && combined.scale == 1.0
                && combined.rot.cols == mm3e_kit::vec::Mat3::IDENTITY.cols;
            if !identity {
                object.xform = combined.compose(object.xform);
                valid_transform(&object.xform)?;
            }
        }
    }
    if let Some(resolved) = &evaluation.resolved {
        if let Some(view) = &resolved.camera {
            *camera = view.compile();
        }
    } else if !clip.camera_keys.is_empty() {
        let mut eyes = Track::new(clip.camera_easing.engine());
        let mut targets = Track::new(clip.camera_easing.engine());
        let mut fovs = Track::new(clip.camera_easing.engine());
        for key in &clip.camera_keys {
            eyes = eyes.key(key.time, vec(key.eye));
            targets = targets.key(key.time, vec(key.target));
            fovs = fovs.key(key.time, key.fov_degrees);
        }
        let mut view = document.camera.clone();
        view.eye = array(eyes.sample(time));
        view.target = array(targets.sample(time));
        view.fov_degrees = fovs.sample(time);
        view.validate()?;
        *camera = view.compile();
    }
    Ok(evaluation)
}

pub fn inspect_pose(document: &Document, sample: &AnimationSample) -> Result<Value, String> {
    let (mut scene, mut camera) = document.compile(&Pass::Beauty)?;
    let evaluation = evaluate(document, &mut scene, &mut camera, sample)?;
    crate::face::refresh(document, &mut scene, Some(sample))?;
    crate::deform::refresh(document, &mut scene, Some(sample), Some(&evaluation))?;
    crate::garment::refresh(document, &mut scene)?;
    crate::cloth::refresh(document, &mut scene, Some(sample))?;
    let deformation = crate::deform::summaries(document, &scene, Some(sample))?;
    let objects: Vec<_> = document
        .objects
        .iter()
        .zip(scene.objects)
        .map(|(e, o)| {
            json!({"id":e.id,
        "position":array(o.xform.pos),"basis":o.xform.rot.cols.map(array),"scale":o.xform.scale})
        })
        .collect();
    let joints: Vec<_> = document.joints.iter().zip(evaluation.joint_transforms).map(|(j,t)| json!({"id":j.id,
        "parent":j.parent,"world_pivot":array(t.to_world(vec(j.pivot))),"basis":t.rot.cols.map(array),"scale":t.scale,"objects":j.objects})).collect();
    let mut result = json!({"clip":sample.clip,"requested_time":sample.time,"time":evaluation.time,"playback":sample.playback,
        "objects":objects,"joints":joints,"camera":{"eye":array(camera.eye),"forward":array(camera.forward),
        "up":array(camera.up),"fov_degrees":(2.0*camera.fov_scale.atan()).to_degrees()},
        "facial_controls":crate::face::inspect_controls(document,Some(sample))?,"deformation":deformation,
        "binding":"rigid parts; world-rest pivots; source-following facial/garment geometry; object transforms include ancestor motion"});
    if !evaluation.limit_observations.is_empty() {
        result["joint_limits"] = json!(evaluation.limit_observations);
    }
    if let Some(resolved) = evaluation.resolved {
        result["layer_samples"] = json!(resolved.layers);
    }
    Ok(result)
}

/// Bind the existing named humanoid recipe at its current rest geometry. No geometry is changed.
pub fn rig_humanoid(document: &Document, id: &str) -> Result<Vec<Joint>, String> {
    identifier(id)?;
    let (scene, _) = document.compile(&Pass::Beauty)?;
    let lookup = |role: &str| -> Result<usize, String> {
        let name = format!("{id}/{role}");
        document.objects.iter().position(|e| e.id == name).ok_or_else(|| format!("humanoid rig requires object {name}"))
    };
    let center = |role: &str| -> Result<V3, String> { Ok(array(scene.objects[lookup(role)?].xform.pos)) };
    let endpoint = |role: &str, end: bool| -> Result<V3, String> {
        let i = lookup(role)?;
        if let Shape::Capsule { a, b, .. } = document.objects[i].shape {
            Ok(array(scene.objects[i].xform.to_world(vec(if end { b } else { a }))))
        } else {
            Err(format!("humanoid rig requires capsule {id}/{role}"))
        }
    };
    let mut joints = vec![];
    let mut add = |name: &str, parent: Option<&str>, pivot: V3, roles: &[&str]| -> Result<(), String> {
        for role in roles {
            lookup(role)?;
        }
        joints.push(Joint {
            id: format!("{id}/{name}"),
            parent: parent.map(|p| format!("{id}/{p}")),
            pivot,
            objects: roles.iter().map(|r| format!("{id}/{r}")).collect(),
            rotation_limit: None,
        });
        Ok(())
    };
    add("root", None, center("pelvis")?, &["pelvis"])?;
    add("spine", Some("root"), center("abdomen")?, &["abdomen", "chest"])?;
    add("neck", Some("spine"), endpoint("neck", false)?, &["neck"])?;
    add("head", Some("neck"), endpoint("neck", true)?, &["head", "left_eye", "right_eye"])?;
    for side in ["left", "right"] {
        let upper = format!("{side}_upper_arm");
        let fore = format!("{side}_forearm");
        let hand = format!("{side}_hand");
        let shoulder = format!("{side}_shoulder");
        let elbow = format!("{side}_elbow");
        let wrist = format!("{side}_wrist");
        add(&shoulder, Some("spine"), endpoint(&upper, false)?, &[&upper])?;
        add(&elbow, Some(&shoulder), endpoint(&fore, false)?, &[&fore])?;
        add(&wrist, Some(&elbow), endpoint(&fore, true)?, &[&hand])?;
        let thigh = format!("{side}_thigh");
        let shin = format!("{side}_shin");
        let foot = format!("{side}_foot");
        let hip = format!("{side}_hip");
        let knee = format!("{side}_knee");
        let ankle = format!("{side}_ankle");
        add(&hip, Some("root"), endpoint(&thigh, false)?, &[&thigh])?;
        add(&knee, Some(&hip), endpoint(&shin, false)?, &[&shin])?;
        add(&ankle, Some(&knee), endpoint(&shin, true)?, &[&foot])?;
    }
    Ok(joints)
}
