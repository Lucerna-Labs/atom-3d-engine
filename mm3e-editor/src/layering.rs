//! Sparse, ordered clip composition in world-rest target coordinates.
//! Layers assemble channels first; a clip's own channels provide final overrides.
use crate::{
    animation::{self, Clip, Interpolation, Playback, Target},
    face::{FaceChannel, ScalarKey},
    model::{array, identifier, range, vec, Document, View},
};
use mm3e_kit::{Quat, Transform, Vec3};
use mm3e_orchestrator::anim::Track;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_LAYERS: usize = 16;
pub const MAX_LAYER_DEPTH: usize = 8;
pub const MAX_LAYER_EVALUATIONS: usize = 1024;
fn one() -> f32 {
    1.0
}
fn enabled() -> bool {
    true
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Channel {
    Object { id: String },
    Joint { id: String },
    Face { face: String, channel: FaceChannel },
    Morph { deformer: String, blendshape: String },
    Camera,
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BlendMode {
    #[default]
    Override,
    Additive,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Layer {
    pub id: String,
    pub clip: String,
    #[serde(default)]
    pub start: f32,
    #[serde(default)]
    pub end: Option<f32>,
    #[serde(default)]
    pub source_start: f32,
    #[serde(default = "one")]
    pub time_scale: f32,
    #[serde(default)]
    pub playback: Playback,
    #[serde(default = "one")]
    pub weight: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weight_keys: Vec<ScalarKey>,
    #[serde(default)]
    pub weight_easing: Interpolation,
    #[serde(default)]
    pub mode: BlendMode,
    #[serde(default)]
    pub reference_time: Option<f32>,
    #[serde(default)]
    pub mask: Option<Vec<Channel>>,
    #[serde(default = "enabled")]
    pub enabled: bool,
}
impl Layer {
    pub fn allows(&self, channel: &Channel) -> bool {
        self.mask.as_ref().is_none_or(|m| m.contains(channel))
    }
}
#[derive(Clone, Default)]
pub(crate) struct Resolved {
    pub(crate) transforms: BTreeMap<Target, Transform>,
    pub(crate) faces: BTreeMap<(String, FaceChannel), f32>,
    pub(crate) morphs: BTreeMap<(String, String), f32>,
    pub(crate) camera: Option<View>,
    pub(crate) layers: Vec<Value>,
}
fn channel_exists(document: &Document, channel: &Channel) -> bool {
    match channel {
        Channel::Object { id } => document.objects.iter().any(|o| &o.id == id),
        Channel::Joint { id } => document.joints.iter().any(|j| &j.id == id),
        Channel::Face { face, .. } => document.faces.iter().any(|f| &f.id == face),
        Channel::Morph { deformer, blendshape } => {
            document.deformers.iter().any(|d| &d.id == deformer && d.blendshapes.iter().any(|m| &m.id == blendshape))
        }
        Channel::Camera => true,
    }
}
/// Validate all references, including disabled/masked branches, before any pruning.
pub(crate) fn validate(document: &Document) -> Result<(), String> {
    let clips: BTreeMap<_, _> = document.clips.iter().map(|c| (c.id.as_str(), c)).collect();
    let mut keys = 0usize;
    for clip in &document.clips {
        if clip.layers.len() > MAX_LAYERS {
            return Err(format!("clip {} exceeds {MAX_LAYERS} layers", clip.id));
        }
        let mut ids = BTreeSet::new();
        for layer in &clip.layers {
            identifier(&layer.id)?;
            identifier(&layer.clip)?;
            if !ids.insert(&layer.id) {
                return Err(format!("duplicate layer {} in clip {}", layer.id, clip.id));
            }
            let source = clips
                .get(layer.clip.as_str())
                .ok_or_else(|| format!("layer {} references missing clip {}", layer.id, layer.clip))?;
            range(layer.start, 0.0, clip.duration, "layer start")?;
            range(layer.end.unwrap_or(clip.duration), layer.start, clip.duration, "layer end")?;
            range(layer.source_start, -86400.0, 86400.0, "layer source_start")?;
            range(layer.time_scale, 0.001, 1000.0, "layer time_scale")?;
            range(layer.weight, 0.0, 1.0, "layer weight")?;
            if !layer.weight_keys.is_empty() {
                animation::key_times(layer.weight_keys.iter().map(|k| k.time), clip.duration)?;
                for key in &layer.weight_keys {
                    range(key.value, 0.0, 1.0, "layer weight key")?;
                }
            }
            keys += layer.weight_keys.len();
            if let Some(t) = layer.reference_time {
                if layer.mode != BlendMode::Additive {
                    return Err("reference_time is only valid for an additive layer".into());
                }
                range(t, 0.0, source.duration, "layer reference_time")?;
            }
            if let Some(mask) = &layer.mask {
                if mask.len() > 2048 {
                    return Err("layer mask exceeds 2048 channels".into());
                }
                let mut seen = BTreeSet::new();
                for channel in mask {
                    if !seen.insert(channel) || !channel_exists(document, channel) {
                        return Err(format!("layer {} has duplicate or missing mask channel {channel:?}", layer.id));
                    }
                }
            }
        }
    }
    let authored_keys: usize = document
        .clips
        .iter()
        .map(|c| {
            c.tracks.iter().map(|t| t.keys.len()).sum::<usize>()
                + c.face_tracks.iter().map(|t| t.keys.len()).sum::<usize>()
                + c.morph_tracks.iter().map(|t| t.keys.len()).sum::<usize>()
                + c.camera_keys.len()
        })
        .sum();
    if authored_keys + keys > animation::MAX_KEYS {
        return Err("document exceeds total animation key budget including layer weights".into());
    }
    fn visit<'a>(
        id: &'a str,
        clips: &BTreeMap<&'a str, &'a Clip>,
        stack: &mut BTreeSet<&'a str>,
        done: &mut BTreeMap<&'a str, (usize, usize)>,
    ) -> Result<(usize, usize), String> {
        if let Some(&value) = done.get(id) {
            return Ok(value);
        }
        if !stack.insert(id) {
            return Err(format!("animation layer cycle at {id}"));
        }
        let clip = clips.get(id).ok_or_else(|| format!("missing layer clip {id}"))?;
        let (mut depth, mut work) = (1, 1usize);
        for layer in &clip.layers {
            let (d, w) = visit(layer.clip.as_str(), clips, stack, done)?;
            depth = depth.max(d + 1);
            work = work.saturating_add(w.saturating_mul(if layer.reference_time.is_some() { 2 } else { 1 }));
            if depth > MAX_LAYER_DEPTH || work > MAX_LAYER_EVALUATIONS {
                return Err(format!(
                    "layer graph exceeds depth {MAX_LAYER_DEPTH} or {MAX_LAYER_EVALUATIONS} evaluations at {id}"
                ));
            }
        }
        stack.remove(id);
        done.insert(id, (depth, work));
        Ok((depth, work))
    }
    let mut done = BTreeMap::new();
    for clip in &document.clips {
        visit(&clip.id, &clips, &mut BTreeSet::new(), &mut done)?;
    }
    Ok(())
}
fn anchor(document: &Document, target: &Target) -> Result<Vec3, String> {
    match target {
        Target::Object { id } => document.objects.iter().find(|o| &o.id == id).map(|o| vec(o.position)),
        Target::Joint { id } => document.joints.iter().find(|j| &j.id == id).map(|j| vec(j.pivot)),
    }
    .ok_or_else(|| format!("missing layer transform target {target:?}"))
}
fn same(a: Transform, b: Transform) -> bool {
    a.pos == b.pos && a.scale == b.scale && a.rot.cols == b.rot.cols
}
fn transform_mix(
    base: Transform,
    source: Transform,
    reference: Transform,
    pivot: Vec3,
    weight: f32,
    mode: BlendMode,
) -> Result<Transform, String> {
    if weight == 0.0 {
        return Ok(base);
    }
    if mode == BlendMode::Override && weight == 1.0 {
        return Ok(source);
    }
    if mode == BlendMode::Override && same(base, source) {
        return Ok(base);
    }
    if mode == BlendMode::Additive && same(source, reference) {
        return Ok(base);
    }
    if mode == BlendMode::Additive && weight == 1.0 && same(base, reference) {
        return Ok(source);
    }
    let a = Quat::from_mat3(base.rot);
    let b = Quat::from_mat3(source.rot);
    let t0 = base.to_world(pivot) - pivot;
    let t1 = source.to_world(pivot) - pivot;
    let (translation, rotation, scale) = match mode {
        BlendMode::Override => (
            vec(std::array::from_fn(|i| scalar_mix(array(t0)[i], array(t1)[i], 0.0, weight, mode))),
            a.slerp(b, weight),
            scalar_mix(base.scale, source.scale, 1.0, weight, mode),
        ),
        BlendMode::Additive => {
            let r = Quat::from_mat3(reference.rot);
            let inverse = Quat { x: -r.x, y: -r.y, z: -r.z, w: r.w };
            let delta = inverse * b;
            (
                vec(std::array::from_fn(|i| {
                    scalar_mix(array(t0)[i], array(t1)[i], array(reference.to_world(pivot) - pivot)[i], weight, mode)
                })),
                a * Quat::IDENTITY.slerp(delta, weight),
                (f64::from(base.scale)
                    * ((1.0 - f64::from(weight))
                        + f64::from(weight) * f64::from(source.scale) / f64::from(reference.scale)))
                    as f32,
            )
        }
    };
    if !scale.is_finite()
        || scale <= 0.0
        || [translation.x, translation.y, translation.z].iter().any(|v| !v.is_finite())
    {
        return Err("layer transform is not finite with positive scale".into());
    }
    Ok(Transform::around_pivot(pivot, rotation, scale, translation))
}
fn scalar_mix(base: f32, source: f32, reference: f32, weight: f32, mode: BlendMode) -> f32 {
    if weight == 0.0 {
        return base;
    }
    match mode {
        BlendMode::Override if weight == 1.0 => source,
        BlendMode::Override => {
            (f64::from(base) * (1.0 - f64::from(weight)) + f64::from(source) * f64::from(weight)) as f32
        }
        BlendMode::Additive if weight == 1.0 && base == reference => source,
        BlendMode::Additive => {
            (f64::from(base) + (f64::from(source) - f64::from(reference)) * f64::from(weight)) as f32
        }
    }
}
fn face_default(document: &Document, id: &str, channel: FaceChannel) -> Result<f32, String> {
    let c = &document.faces.iter().find(|f| f.id == id).ok_or_else(|| format!("missing layer face {id}"))?.controls;
    Ok(match channel {
        FaceChannel::BlinkLeft => c.blink_left,
        FaceChannel::BlinkRight => c.blink_right,
        FaceChannel::JawOpen => c.jaw_open,
        FaceChannel::LipRound => c.lip_round,
        FaceChannel::LipWide => c.lip_wide,
        FaceChannel::Smile => c.smile,
        FaceChannel::GazeX => c.gaze_x,
        FaceChannel::GazeY => c.gaze_y,
        FaceChannel::BrowLeft => c.brow_left,
        FaceChannel::BrowRight => c.brow_right,
        FaceChannel::LipSeal => c.lip_seal,
    })
}
fn morph_default(document: &Document, id: &str, name: &str) -> Result<f32, String> {
    document
        .deformers
        .iter()
        .find(|d| d.id == id)
        .and_then(|d| d.blendshapes.iter().find(|m| m.id == name))
        .map(|m| m.weight)
        .ok_or_else(|| format!("missing layer morph {id}/{name}"))
}
fn camera_mix(base: &View, source: &View, reference: &View, weight: f32, mode: BlendMode) -> View {
    if weight == 0.0 {
        return base.clone();
    }
    if mode == BlendMode::Override && weight == 1.0 {
        return source.clone();
    }
    let mut out = base.clone();
    out.eye = std::array::from_fn(|i| scalar_mix(base.eye[i], source.eye[i], reference.eye[i], weight, mode));
    out.target =
        std::array::from_fn(|i| scalar_mix(base.target[i], source.target[i], reference.target[i], weight, mode));
    out.fov_degrees = scalar_mix(base.fov_degrees, source.fov_degrees, reference.fov_degrees, weight, mode);
    out
}
fn mapped_time(layer: &Layer, parent_time: f32, duration: f32) -> f32 {
    let mapped =
        (f64::from(parent_time) - f64::from(layer.start)) * f64::from(layer.time_scale) + f64::from(layer.source_start);
    (match layer.playback {
        Playback::Clamp => mapped.clamp(0.0, f64::from(duration)),
        Playback::Loop => mapped.rem_euclid(f64::from(duration)),
    }) as f32
}
fn apply(
    document: &Document,
    out: &mut Resolved,
    source: &Resolved,
    reference: Option<&Resolved>,
    layer: &Layer,
    weight: f32,
) -> Result<(), String> {
    for (target, &value) in &source.transforms {
        let channel = match target {
            Target::Object { id } => Channel::Object { id: id.clone() },
            Target::Joint { id } => Channel::Joint { id: id.clone() },
        };
        if !layer.allows(&channel) {
            continue;
        }
        let base = out.transforms.get(target).copied().unwrap_or(Transform::IDENTITY);
        let reference = reference.and_then(|r| r.transforms.get(target)).copied().unwrap_or(Transform::IDENTITY);
        out.transforms.insert(
            target.clone(),
            transform_mix(base, value, reference, anchor(document, target)?, weight, layer.mode)?,
        );
    }
    for ((id, channel), &value) in &source.faces {
        if !layer.allows(&Channel::Face { face: id.clone(), channel: *channel }) {
            continue;
        }
        let key = (id.clone(), *channel);
        let rest = face_default(document, id, *channel)?;
        let base = out.faces.get(&key).copied().unwrap_or(rest);
        let reference = reference.and_then(|r| r.faces.get(&key)).copied().unwrap_or(rest);
        out.faces.insert(key, scalar_mix(base, value, reference, weight, layer.mode));
    }
    for ((id, name), &value) in &source.morphs {
        if !layer.allows(&Channel::Morph { deformer: id.clone(), blendshape: name.clone() }) {
            continue;
        }
        let key = (id.clone(), name.clone());
        let rest = morph_default(document, id, name)?;
        let base = out.morphs.get(&key).copied().unwrap_or(rest);
        let reference = reference.and_then(|r| r.morphs.get(&key)).copied().unwrap_or(rest);
        out.morphs.insert(key, scalar_mix(base, value, reference, weight, layer.mode));
    }
    if layer.allows(&Channel::Camera) {
        if let Some(source) = &source.camera {
            out.camera = Some(camera_mix(
                out.camera.as_ref().unwrap_or(&document.camera),
                source,
                reference.and_then(|r| r.camera.as_ref()).unwrap_or(&document.camera),
                weight,
                layer.mode,
            ));
        }
    }
    Ok(())
}
type Channels = BTreeSet<Channel>;
type ResolveCache = BTreeMap<(String, u32, Vec<Channel>), Resolved>;
fn target_channel(target: &Target) -> Channel {
    match target {
        Target::Object { id } => Channel::Object { id: id.clone() },
        Target::Joint { id } => Channel::Joint { id: id.clone() },
    }
}
fn own_channels(clip: &Clip) -> Channels {
    let mut channels: Channels = clip
        .tracks
        .iter()
        .map(|t| target_channel(&t.target))
        .chain(clip.face_tracks.iter().map(|t| Channel::Face { face: t.face.clone(), channel: t.channel }))
        .chain(
            clip.morph_tracks
                .iter()
                .map(|t| Channel::Morph { deformer: t.deformer.clone(), blendshape: t.blendshape.clone() }),
        )
        .collect();
    if !clip.camera_keys.is_empty() {
        channels.insert(Channel::Camera);
    }
    channels
}
fn available_channels(document: &Document, clip: &Clip, memo: &mut BTreeMap<String, Channels>) -> Channels {
    if let Some(channels) = memo.get(&clip.id) {
        return channels.clone();
    }
    let mut channels = own_channels(clip);
    for layer in &clip.layers {
        if !layer.enabled
            || layer.weight == 0.0
            || (!layer.weight_keys.is_empty() && layer.weight_keys.iter().all(|k| k.value == 0.0))
        {
            continue;
        }
        // All references and graph depth were validated before this traversal.
        let child = document.clips.iter().find(|c| c.id == layer.clip).expect("validated layer reference");
        channels.extend(available_channels(document, child, memo).into_iter().filter(|c| layer.allows(c)));
    }
    memo.insert(clip.id.clone(), channels.clone());
    channels
}
fn resolve_inner(
    document: &Document,
    clip: &Clip,
    time: f32,
    memo: &mut ResolveCache,
    remaining: &mut usize,
    demand: &Channels,
    available: &BTreeMap<String, Channels>,
) -> Result<Resolved, String> {
    let key = (clip.id.clone(), time.to_bits(), demand.iter().cloned().collect());
    if let Some(value) = memo.get(&key) {
        return Ok(value.clone());
    }
    *remaining = remaining.checked_sub(1).ok_or("layer evaluation budget exhausted")?;
    let mut result = Resolved::default();
    let inherited: Channels = demand.difference(&own_channels(clip)).cloned().collect();
    for layer in &clip.layers {
        let active = layer.enabled && time >= layer.start && time <= layer.end.unwrap_or(clip.duration);
        let mut weight = layer.weight;
        if !layer.weight_keys.is_empty() {
            let mut track = Track::new(layer.weight_easing.engine());
            for k in &layer.weight_keys {
                track = track.key(k.time, k.value);
            }
            weight *= track.try_sample(time).ok_or("empty layer weight track")?;
        }
        let source_clip = document
            .clips
            .iter()
            .find(|c| c.id == layer.clip)
            .ok_or_else(|| format!("missing layer clip {}", layer.clip))?;
        let source_time = mapped_time(layer, time, source_clip.duration);
        let selected: Channels = available
            .get(&layer.clip)
            .map(|channels| channels.intersection(&inherited).filter(|c| layer.allows(c)).cloned().collect())
            .unwrap_or_default();
        result.layers.push(json!({"parent_clip":clip.id,"parent_time":time,"layer":layer.id,"clip":layer.clip,"source_time":source_time,"weight":weight,"active":active&&weight>0.0&&!selected.is_empty(),"mode":layer.mode}));
        if !active || weight == 0.0 || selected.is_empty() {
            continue;
        }
        let source = resolve_inner(document, source_clip, source_time, memo, remaining, &selected, available)?;
        let reference = layer
            .reference_time
            .map(|t| resolve_inner(document, source_clip, t, memo, remaining, &selected, available))
            .transpose()?;
        apply(document, &mut result, &source, reference.as_ref(), layer, weight)
            .map_err(|e| format!("clip {} layer {}: {e}", clip.id, layer.id))?;
        result.layers.extend(source.layers);
    }
    // Local authored channels are the final authority over an assembled clip.
    for track in &clip.tracks {
        if !demand.contains(&target_channel(&track.target)) {
            continue;
        }
        let pivot = match &track.target {
            Target::Joint { id } => {
                document.joints.iter().find(|j| &j.id == id).map(|j| j.pivot).ok_or("missing joint track target")?
            }
            Target::Object { .. } => track.pivot.unwrap_or(array(anchor(document, &track.target)?)),
        };
        result.transforms.insert(track.target.clone(), animation::delta(track, time, pivot));
    }
    for track in &clip.face_tracks {
        if !demand.contains(&Channel::Face { face: track.face.clone(), channel: track.channel }) {
            continue;
        }
        let mut scalar = Track::new(track.easing.engine());
        for k in &track.keys {
            scalar = scalar.key(k.time, k.value);
        }
        result.faces.insert((track.face.clone(), track.channel), scalar.try_sample(time).ok_or("empty face track")?);
    }
    for track in &clip.morph_tracks {
        if !demand.contains(&Channel::Morph { deformer: track.deformer.clone(), blendshape: track.blendshape.clone() })
        {
            continue;
        }
        let mut scalar = Track::new(track.easing.engine());
        for k in &track.keys {
            scalar = scalar.key(k.time, k.weight);
        }
        result.morphs.insert(
            (track.deformer.clone(), track.blendshape.clone()),
            scalar.try_sample(time).ok_or("empty morph track")?,
        );
    }
    if demand.contains(&Channel::Camera) && !clip.camera_keys.is_empty() {
        let mut eyes = Track::new(clip.camera_easing.engine());
        let mut targets = Track::new(clip.camera_easing.engine());
        let mut fovs = Track::new(clip.camera_easing.engine());
        for k in &clip.camera_keys {
            eyes = eyes.key(k.time, vec(k.eye));
            targets = targets.key(k.time, vec(k.target));
            fovs = fovs.key(k.time, k.fov_degrees);
        }
        let mut view = document.camera.clone();
        view.eye = array(eyes.sample(time));
        view.target = array(targets.sample(time));
        view.fov_degrees = fovs.sample(time);
        result.camera = Some(view);
    }
    memo.insert(key, result.clone());
    Ok(result)
}
pub(crate) fn resolve(document: &Document, clip: &Clip, time: f32) -> Result<Resolved, String> {
    validate(document)?;
    let mut remaining = MAX_LAYER_EVALUATIONS;
    let mut available = BTreeMap::new();
    let demand = available_channels(document, clip, &mut available);
    let result = resolve_inner(document, clip, time, &mut BTreeMap::new(), &mut remaining, &demand, &available)?;
    for ((face, channel), &value) in &result.faces {
        let minimum = if matches!(
            channel,
            FaceChannel::LipWide
                | FaceChannel::Smile
                | FaceChannel::GazeX
                | FaceChannel::GazeY
                | FaceChannel::BrowLeft
                | FaceChannel::BrowRight
        ) {
            -1.0
        } else {
            0.0
        };
        range(value, minimum, 1.0, &format!("layered face {face}/{channel:?}"))?;
    }
    for ((id, name), &value) in &result.morphs {
        let morph = document
            .deformers
            .iter()
            .find(|d| &d.id == id)
            .and_then(|d| d.blendshapes.iter().find(|m| &m.id == name))
            .ok_or("missing resolved morph")?;
        range(value, morph.min_weight, morph.max_weight, &format!("layered morph {id}/{name}"))?;
    }
    if let Some(camera) = &result.camera {
        camera.validate()?;
    }
    Ok(result)
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EditLayer {
    pub clip: String,
    pub action: LayerEdit,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum LayerEdit {
    Upsert { layer: Layer },
    Remove { id: String },
    Move { id: String, index: usize },
}
/// Called inside the editor's atomic candidate transaction. Other clips and
/// local channels remain intact; global graph validation runs on the full batch.
pub(crate) fn edit(document: &mut Document, request: &EditLayer) -> Result<(), String> {
    let clip = document
        .clips
        .iter_mut()
        .find(|c| c.id == request.clip)
        .ok_or_else(|| format!("missing layer destination clip {}", request.clip))?;
    match &request.action {
        LayerEdit::Upsert { layer } => {
            identifier(&layer.id)?;
            if let Some(existing) = clip.layers.iter_mut().find(|l| l.id == layer.id) {
                *existing = layer.clone();
            } else {
                clip.layers.push(layer.clone());
            }
        }
        LayerEdit::Remove { id } => {
            let i = clip.layers.iter().position(|l| &l.id == id).ok_or_else(|| format!("missing layer {id}"))?;
            clip.layers.remove(i);
        }
        LayerEdit::Move { id, index } => {
            let i = clip.layers.iter().position(|l| &l.id == id).ok_or_else(|| format!("missing layer {id}"))?;
            if *index >= clip.layers.len() {
                return Err("layer destination index must be within the final layer list".into());
            }
            let layer = clip.layers.remove(i);
            clip.layers.insert(*index, layer);
        }
    }
    Ok(())
}
