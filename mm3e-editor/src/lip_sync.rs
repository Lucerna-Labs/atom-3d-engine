//! Editable, source-attributed mouth curves. A recognizer's discrete cue sequence
//! supplies timing; an explicit character profile supplies the actual rig poses.
use crate::{
    animation::{self, Clip, Interpolation, MotionTrack, Target, TransformKey},
    deform::{MorphKey, MorphTrack},
    face::{FaceChannel, FaceTrack, ScalarKey},
    model::{identifier, range, vector, Document, Pass, V3},
    protocol::Failure,
    speech::{self, MouthShape, SpeechReport},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

const FORMAT: &str = "mm3e.lip-sync.v1";
const MAX_CHANNELS: usize = 512;
fn one() -> f32 {
    1.0
}
fn transition() -> f32 {
    0.04
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GenerateLipSync {
    pub analysis_path: String,
    /// Exact report-file SHA-256 returned by analyze_speech; checked during the same read.
    #[serde(default)]
    pub analysis_sha256: Option<String>,
    pub clip: String,
    pub profile: LipSyncProfile,
    #[serde(default = "transition")]
    pub transition_seconds: f32,
    #[serde(default)]
    pub replace: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LipSyncProfile {
    pub poses: Vec<MouthPose>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MouthPose {
    pub shape: MouthShape,
    pub values: Vec<PoseValue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum PoseValue {
    Face {
        face: String,
        channel: FaceChannel,
        value: f32,
    },
    Morph {
        deformer: String,
        blendshape: String,
        value: f32,
    },
    Joint {
        joint: String,
        #[serde(default)]
        translation: V3,
        #[serde(default)]
        rotation_degrees: V3,
        #[serde(default = "one")]
        scale: f32,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Channel {
    Face(String, FaceChannel),
    Morph(String, String),
    Joint(String),
}
impl PoseValue {
    fn channel(&self) -> Channel {
        match self {
            Self::Face { face, channel, .. } => Channel::Face(face.clone(), *channel),
            Self::Morph { deformer, blendshape, .. } => Channel::Morph(deformer.clone(), blendshape.clone()),
            Self::Joint { joint, .. } => Channel::Joint(joint.clone()),
        }
    }
    fn validate(&self) -> Result<(), String> {
        match self {
            Self::Face { face, channel, value } => {
                identifier(face)?;
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
                range(*value, minimum, 1.0, "lip-sync facial value")
            }
            Self::Morph { deformer, blendshape, value } => {
                identifier(deformer)?;
                identifier(blendshape)?;
                range(*value, -1000.0, 1000.0, "lip-sync morph value")
            }
            Self::Joint { joint, translation, rotation_degrees, scale } => {
                identifier(joint)?;
                vector(*translation, "lip-sync joint translation")?;
                vector(*rotation_degrees, "lip-sync joint rotation")?;
                range(*scale, 0.001, 1000.0, "lip-sync joint scale")
            }
        }
    }
    fn authored_default(&self, document: &Document) -> Result<Self, String> {
        self.validate()?;
        Ok(match self {
            Self::Face { face, channel, .. } => {
                let controls = &document
                    .faces
                    .iter()
                    .find(|f| &f.id == face)
                    .ok_or_else(|| format!("lip-sync profile references missing face {face}"))?
                    .controls;
                let value = match channel {
                    FaceChannel::BlinkLeft => controls.blink_left,
                    FaceChannel::BlinkRight => controls.blink_right,
                    FaceChannel::JawOpen => controls.jaw_open,
                    FaceChannel::LipRound => controls.lip_round,
                    FaceChannel::LipWide => controls.lip_wide,
                    FaceChannel::Smile => controls.smile,
                    FaceChannel::GazeX => controls.gaze_x,
                    FaceChannel::GazeY => controls.gaze_y,
                    FaceChannel::BrowLeft => controls.brow_left,
                    FaceChannel::BrowRight => controls.brow_right,
                    FaceChannel::LipSeal => controls.lip_seal,
                };
                Self::Face { face: face.clone(), channel: *channel, value }
            }
            Self::Morph { deformer, blendshape, value } => {
                let morph = document
                    .deformers
                    .iter()
                    .find(|d| &d.id == deformer)
                    .and_then(|d| d.blendshapes.iter().find(|m| &m.id == blendshape))
                    .ok_or_else(|| format!("lip-sync profile references missing morph {deformer}/{blendshape}"))?;
                range(*value, morph.min_weight, morph.max_weight, "lip-sync authored morph range")?;
                Self::Morph { deformer: deformer.clone(), blendshape: blendshape.clone(), value: morph.weight }
            }
            Self::Joint { joint, .. } => {
                if !document.joints.iter().any(|j| &j.id == joint) {
                    return Err(format!("lip-sync profile references missing joint {joint}"));
                }
                Self::Joint { joint: joint.clone(), translation: [0.0; 3], rotation_degrees: [0.0; 3], scale: 1.0 }
            }
        })
    }
}

/// Original evidence and mapping remain available after curve edits or source
/// removal. Defaults capture the values used for omitted sparse pose channels.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LipSyncProvenance {
    pub format: String,
    pub report: SpeechReport,
    pub profile: LipSyncProfile,
    pub channel_defaults: Vec<PoseValue>,
    pub transition_seconds: f32,
    pub generated_curves_sha256: String,
    /// Detects independent changes to retained evidence or its mapping/settings.
    pub provenance_sha256: String,
}

fn provenance_digest(provenance: &LipSyncProvenance) -> Result<String, String> {
    let bytes = serde_json::to_vec(&json!({
        "format":provenance.format,"report":provenance.report,"profile":provenance.profile,
        "channel_defaults":provenance.channel_defaults,"transition_seconds":provenance.transition_seconds,
        "generated_curves_sha256":provenance.generated_curves_sha256,
    }))
    .map_err(|error| error.to_string())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn profile_channels(profile: &LipSyncProfile, report: &SpeechReport) -> Result<BTreeSet<Channel>, String> {
    if profile.poses.is_empty() || profile.poses.len() > 9 {
        return Err("lip-sync profile requires 1..9 distinct mouth shapes".into());
    }
    let mut shapes = BTreeSet::new();
    let mut channels = BTreeSet::new();
    for pose in &profile.poses {
        if !shapes.insert(pose.shape) {
            return Err("lip-sync profile contains a duplicate mouth shape".into());
        }
        if pose.values.len() > MAX_CHANNELS {
            return Err("lip-sync pose exceeds 512 channels".into());
        }
        let mut local = BTreeSet::new();
        for value in &pose.values {
            value.validate()?;
            let channel = value.channel();
            if !local.insert(channel.clone()) {
                return Err("lip-sync pose repeats a target channel".into());
            }
            channels.insert(channel);
        }
    }
    if channels.is_empty() || channels.len() > MAX_CHANNELS {
        return Err("lip-sync profile requires 1..512 target channels".into());
    }
    for cue in &report.cues {
        if !shapes.contains(&cue.shape) {
            return Err(format!("lip-sync profile has no pose for {:?}", cue.shape));
        }
    }
    Ok(channels)
}

pub fn validate_provenance(provenance: &LipSyncProvenance) -> Result<(), String> {
    if provenance.format != FORMAT {
        return Err("unsupported lip-sync provenance format".into());
    }
    speech::validate_report(&provenance.report)?;
    range(provenance.transition_seconds, 0.0, 10.0, "lip-sync transition seconds")?;
    let channels = profile_channels(&provenance.profile, &provenance.report)?;
    if provenance.channel_defaults.len() != channels.len() {
        return Err("lip-sync defaults do not match profile channels".into());
    }
    let mut defaults = BTreeSet::new();
    for value in &provenance.channel_defaults {
        value.validate()?;
        if !defaults.insert(value.channel()) {
            return Err("duplicate lip-sync default channel".into());
        }
    }
    if defaults != channels {
        return Err("lip-sync defaults do not match profile channels".into());
    }
    let digest = &provenance.generated_curves_sha256;
    if digest.len() != 64 || !digest.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
        return Err("invalid lip-sync generated curve SHA-256".into());
    }
    if provenance.provenance_sha256 != provenance_digest(provenance)? {
        return Err("lip-sync provenance SHA-256 differs from retained evidence and mapping".into());
    }
    Ok(())
}

fn fingerprint(clip: &Clip, channels: &BTreeSet<Channel>) -> Result<String, String> {
    let mut tracks = Vec::with_capacity(channels.len());
    for channel in channels {
        tracks.push(match channel {
            Channel::Face(face, channel) => json!({"type":"face","face":face,"channel":channel,
                "track":clip.face_tracks.iter().find(|t| &t.face == face && &t.channel == channel)}),
            Channel::Morph(deformer, blendshape) => json!({"type":"morph","deformer":deformer,"blendshape":blendshape,
                "track":clip.morph_tracks.iter().find(|t| &t.deformer == deformer && &t.blendshape == blendshape)}),
            Channel::Joint(joint) => json!({"type":"joint","joint":joint,
                "track":clip.tracks.iter().find(|t| matches!(&t.target, Target::Joint{id} if id == joint))}),
        });
    }
    let bytes = serde_json::to_vec(&json!({"duration":clip.duration,"tracks":tracks})).map_err(|e| e.to_string())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn duration(report: &SpeechReport) -> Result<(f64, f32), String> {
    let exact = report.source.range.frame_count as f64 / f64::from(report.source.sample_rate);
    let mut native = exact as f32;
    // Keep the last pose through the exact source range. Rounding up holds that
    // same pose for at most one f32 ULP and does not invent another mouth cue.
    if f64::from(native) < exact {
        native = f32::from_bits(native.to_bits() + 1);
    }
    range(native, 0.001, 3600.0, "lip-sync clip duration")?;
    Ok((exact, native))
}

fn push_key(keys: &mut Vec<(f32, PoseValue)>, time: f64, value: &PoseValue) -> Result<(), String> {
    let native = time as f32;
    if !time.is_finite() || !native.is_finite() || time < 0.0 {
        return Err("lip-sync key time is not representable".into());
    }
    if let Some((previous, prior)) = keys.last() {
        if native <= *previous {
            return Err("distinct lip-sync key times collapse in the native f32 animation clock".into());
        }
        // Remove only an interior point of an exactly constant segment. We do
        // not approximate scalar ramps or quaternion paths to meet a budget.
        if keys.len() > 1 && prior == value && keys[keys.len() - 2].1 == *value {
            keys.pop();
        }
    }
    keys.push((native, value.clone()));
    if keys.len() > 1024 {
        return Err("generated lip-sync channel exceeds 1024 keys; select a shorter audio range".into());
    }
    Ok(())
}

fn build(document: &Document, report: SpeechReport, request: &GenerateLipSync) -> Result<Clip, String> {
    speech::validate_report(&report)?;
    if report.cues.is_empty() {
        return Err("speech analysis contains no mouth cues".into());
    }
    identifier(&request.clip)?;
    range(request.transition_seconds, 0.0, 10.0, "lip-sync transition seconds")?;
    let channels = profile_channels(&request.profile, &report)?;
    let mut defaults = BTreeMap::new();
    let mut poses = BTreeMap::new();
    for pose in &request.profile.poses {
        let mut values = BTreeMap::new();
        for value in &pose.values {
            let default = value.authored_default(document)?;
            defaults.insert(value.channel(), default);
            values.insert(value.channel(), value.clone());
        }
        poses.insert(pose.shape, values);
    }
    let (_, native_duration) = duration(&report)?;
    let easing = if request.transition_seconds == 0.0 { Interpolation::Step } else { Interpolation::Linear };
    let mut clip: Clip =
        serde_json::from_value(json!({"id":request.clip,"duration":native_duration})).map_err(|e| e.to_string())?;
    let mut total_keys = 0;
    for channel in &channels {
        let pose_value = |shape: MouthShape| -> &PoseValue { poses[&shape].get(channel).unwrap_or(&defaults[channel]) };
        let mut keys = vec![];
        let mut current = pose_value(report.cues[0].shape);
        push_key(&mut keys, 0.0, current)?;
        for cues in report.cues.windows(2) {
            let next = pose_value(cues[1].shape);
            if current != next {
                let boundary = f64::from(cues[1].start_cs) / 100.0;
                if request.transition_seconds > 0.0 {
                    let adjacent =
                        f64::from((cues[0].end_cs - cues[0].start_cs).min(cues[1].end_cs - cues[1].start_cs)) / 200.0;
                    let width = f64::from(request.transition_seconds).min(adjacent);
                    push_key(&mut keys, boundary - width, current)?;
                }
                push_key(&mut keys, boundary, next)?;
                current = next;
            }
        }
        push_key(&mut keys, f64::from(native_duration), current)?;
        total_keys += keys.len();
        if total_keys > animation::MAX_KEYS {
            return Err("generated lip-sync exceeds total animation key budget".into());
        }
        match channel {
            Channel::Face(face, channel) => clip.face_tracks.push(FaceTrack {
                face: face.clone(),
                channel: *channel,
                easing,
                keys: keys
                    .into_iter()
                    .map(|(time, value)| {
                        let PoseValue::Face { value, .. } = value else { unreachable!() };
                        ScalarKey { time, value }
                    })
                    .collect(),
            }),
            Channel::Morph(deformer, blendshape) => clip.morph_tracks.push(MorphTrack {
                deformer: deformer.clone(),
                blendshape: blendshape.clone(),
                easing,
                keys: keys
                    .into_iter()
                    .map(|(time, value)| {
                        let PoseValue::Morph { value, .. } = value else { unreachable!() };
                        MorphKey { time, weight: value }
                    })
                    .collect(),
            }),
            Channel::Joint(joint) => clip.tracks.push(MotionTrack {
                target: Target::Joint { id: joint.clone() },
                pivot: None,
                easing,
                keys: keys
                    .into_iter()
                    .map(|(time, value)| {
                        let PoseValue::Joint { translation, rotation_degrees, scale, .. } = value else {
                            unreachable!()
                        };
                        TransformKey { time, translation, rotation_degrees, scale }
                    })
                    .collect(),
            }),
        }
    }
    let mut provenance = LipSyncProvenance {
        format: FORMAT.into(),
        report,
        profile: request.profile.clone(),
        channel_defaults: defaults.into_values().collect(),
        transition_seconds: request.transition_seconds,
        generated_curves_sha256: fingerprint(&clip, &channels)?,
        provenance_sha256: String::new(),
    };
    provenance.provenance_sha256 = provenance_digest(&provenance)?;
    validate_provenance(&provenance)?;
    clip.lip_sync = Some(Box::new(provenance));
    Ok(clip)
}

pub fn generate(document: &mut Document, root: &Path, request: &GenerateLipSync) -> Result<Value, Failure> {
    identifier(&request.clip).map_err(Failure::invalid)?;
    let existing = document.clips.iter().position(|clip| clip.id == request.clip);
    if existing.is_some() && !request.replace {
        return Err(Failure::invalid("lip-sync target clip exists; replacement must be explicit"));
    }
    let report =
        speech::read_report_checked(document, root, &request.analysis_path, request.analysis_sha256.as_deref())?;
    let clip = build(document, report, request).map_err(Failure::invalid)?;
    let mut candidate = document.clone();
    if let Some(index) = existing {
        candidate.clips[index] = clip;
    } else {
        candidate.clips.push(clip);
    }
    candidate.compile(&Pass::Beauty).map_err(Failure::invalid)?;
    let result = inspect(&candidate, &request.clip).map_err(Failure::invalid)?;
    *document = candidate;
    Ok(result)
}

pub fn inspect(document: &Document, id: &str) -> Result<Value, String> {
    let clip = document.clips.iter().find(|clip| clip.id == id).ok_or_else(|| format!("missing clip {id}"))?;
    let provenance = clip.lip_sync.as_ref().ok_or("clip has no generated lip-sync provenance")?;
    validate_provenance(provenance)?;
    let channels = profile_channels(&provenance.profile, &provenance.report)?;
    let current = fingerprint(clip, &channels)?;
    let report = &provenance.report;
    let source = document.audio.iter().find(|asset| asset.id == report.source.audio);
    let source_status = match source {
        None => "missing",
        Some(asset)
            if asset.data.sha256() != report.source.source_sha256
                || asset.data.wave().format().sample_rate != report.source.sample_rate
                || report.source.channel >= asset.data.wave().format().channels
                || crate::audio::checked_range(asset, &report.source.range).is_err() =>
        {
            "changed"
        }
        Some(_) => "available",
    };
    let (exact, generated_duration) = duration(report)?;
    let cue_rounding = report
        .cues
        .iter()
        .map(|cue| {
            let exact = f64::from(cue.start_cs) / 100.0;
            (f64::from(exact as f32) - exact).abs()
        })
        .fold(0.0, f64::max);
    Ok(json!({"clip":id,"edited":current != provenance.generated_curves_sha256,
        "generated_curves_sha256":provenance.generated_curves_sha256,"current_curves_sha256":current,
        "source_audio":{"id":report.source.audio,"status":source_status,"sha256":report.source.source_sha256,"range":report.source.range},
        "source_duration_seconds":exact,"generated_duration_seconds":generated_duration,"current_duration_seconds":clip.duration,
        "duration_rounding_seconds":f64::from(generated_duration)-exact,"maximum_cue_boundary_rounding_seconds":cue_rounding,
        "channel_count":channels.len(),
        "provenance":provenance,
        "limitations":"Editable first-pass mouth timing from recognition; no forced alignment, acting approval, or automatic character pose calibration."}))
}
