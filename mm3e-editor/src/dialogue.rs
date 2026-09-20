//! Shared shot-clock observations and image/audio delivery through the native renderer.
use crate::{
    animation, audio,
    model::{Document, Pass, View},
    protocol::Failure,
    sequence::{self, FrameTime, SequenceFormat, SequenceRequest},
    storage,
    timeline::{self, AudioTiming, ShotState, TimelineShot},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::Path;

fn audio_timing(document: &Document, id: &str) -> Result<AudioTiming, String> {
    let wave = audio::get(document, id)?.data.wave();
    Ok(AudioTiming { sample_rate: wave.format().sample_rate, sample_frames: wave.frame_count() })
}
pub fn validate(document: &Document) -> Result<(), String> {
    audio::validate(&document.audio)?;
    timeline::validate(&document.shots, &document.clips, |id| audio_timing(document, id))
}
pub fn state(document: &Document, id: &str) -> Result<ShotState, String> {
    let shot = document.shots.iter().find(|shot| shot.id == id).ok_or_else(|| format!("missing shot {id}"))?;
    let clip = document
        .clips
        .iter()
        .find(|clip| clip.id == shot.clip)
        .ok_or_else(|| format!("missing shot clip {}", shot.clip))?;
    let audio = shot.audio.as_ref().map(|placement| audio_timing(document, &placement.asset)).transpose()?;
    timeline::schedule(shot, clip, audio)
}
pub fn inspect(document: &Document, id: &str) -> Result<Value, String> {
    let state = state(document, id)?;
    let mut result = serde_json::to_value(&state).map_err(|e| e.to_string())?;
    if let Some(placement) = &state.shot.audio {
        result["audio_asset"] = audio::metadata(audio::get(document, &placement.asset)?);
    }
    Ok(result)
}
pub fn put(document: &mut Document, shot: TimelineShot) -> Result<(), String> {
    let mut candidate = document.clone();
    if let Some(existing) = candidate.shots.iter_mut().find(|s| s.id == shot.id) {
        *existing = shot;
    } else {
        candidate.shots.push(shot);
    }
    validate(&candidate)?;
    *document = candidate;
    Ok(())
}
pub fn remove(document: &mut Document, id: &str) -> Result<(), String> {
    let i = document.shots.iter().position(|s| s.id == id).ok_or_else(|| format!("missing shot {id}"))?;
    document.shots.remove(i);
    Ok(())
}
pub fn pose(document: &Document, shot: &str, frame: i64) -> Result<Value, String> {
    let state = state(document, shot)?;
    let sample = state.sample(frame)?;
    let mut result = animation::inspect_pose(document, &sample)?;
    result["shot"] = json!(shot);
    result["shot_frame"] = json!(state.select(frame, 1)?[0]);
    Ok(result)
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FrameSelection {
    pub start_frame: i64,
    pub frame_count: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RenderShot {
    pub shot: String,
    /// New output directory, relative to the editor root.
    pub directory: String,
    #[serde(default)]
    pub selection: Option<FrameSelection>,
    #[serde(default)]
    pub pass: Pass,
    #[serde(default)]
    pub view: Option<View>,
    #[serde(default)]
    pub format: SequenceFormat,
}
pub fn render(document: &Document, root: &Path, request: &RenderShot) -> Result<Value, Failure> {
    let state = state(document, &request.shot).map_err(Failure::invalid)?;
    let selected = if let Some(selection) = &request.selection {
        state.select(selection.start_frame, selection.frame_count).map_err(Failure::invalid)?
    } else {
        &state.frames
    };
    let first = selected.first().ok_or_else(|| Failure::invalid("shot has no selected frames"))?;
    let last = selected.last().ok_or_else(|| Failure::invalid("shot has no selected frames"))?;
    // Validate and encode required audio before reserving a directory or rendering.
    // A partial selection keeps the COMPLETE shot's sample-window floor phase.
    let prepared_audio = prepare_audio(document, &state, first, last)?;
    let range = SequenceRequest {
        directory: request.directory.clone(),
        clip: state.shot.clip.clone(),
        start: first.sampled_time,
        end: last.sampled_time,
        fps: (f64::from(state.shot.rate.numerator) / f64::from(state.shot.rate.denominator)) as f32,
        playback: animation::Playback::Clamp,
        pass: request.pass.clone(),
        view: request.view.clone(),
        format: request.format.clone(),
    };
    let times = selected
        .iter()
        .map(|frame| FrameTime { scheduled: frame.scheduled_time, sampled: frame.sampled_time })
        .collect();
    let mut result = sequence::render_scheduled(document, root, &range, times, |manifest| {
        manifest["format"] = json!("mm3e-shot-sequence");
        manifest["version"] = json!(1);
        manifest["shot"] = json!(state.shot);
        manifest["selection"] = json!({"start_frame":first.frame,"frame_count":selected.len()});
        manifest["audio_rounding"] = json!(state.audio_rounding);
        manifest["time_semantics"]=json!("Scheduled time = (absolute frame - clip_frame_zero) * rate.denominator / rate.numerator; frame count is explicit with an exclusive end; time is the exact f32 sample supplied to native animation");
        for (entry, frame) in manifest["frames"]
            .as_array_mut()
            .ok_or_else(|| Failure::invalid("missing rendered frame records"))?
            .iter_mut()
            .zip(selected)
        {
            entry["shot_index"] = json!(frame.index);
            entry["shot_frame"] = json!(frame.frame);
            entry["audio_window"] = json!(frame.audio);
        }
        if let Some((bytes, mut info)) = prepared_audio {
            let relative = Path::new(&request.directory).join("audio.wav");
            let relative = relative.to_str().ok_or_else(|| Failure::invalid("shot audio path must be UTF-8"))?;
            let path = storage::write(root, relative, &bytes, false)?;
            info["path"] = json!(path);
            manifest["audio"] = info;
        } else {
            manifest["audio"] = Value::Null;
        }
        Ok(())
    })?;
    result["shot"] = json!(state.shot);
    result["selection"] = json!({"start_frame":first.frame,"frame_count":selected.len()});
    Ok(result)
}

fn prepare_audio(
    document: &Document,
    state: &ShotState,
    first: &timeline::ShotFrame,
    last: &timeline::ShotFrame,
) -> Result<Option<(Vec<u8>, Value)>, Failure> {
    let prepared_audio = if let Some(placement) = &state.shot.audio {
        let asset = audio::get(document, &placement.asset).map_err(Failure::invalid)?;
        let first_window = first.audio.ok_or_else(|| Failure::invalid("shot is missing its audio window"))?;
        let last_window = last.audio.ok_or_else(|| Failure::invalid("shot is missing its audio window"))?;
        let frames = last_window
            .end_sample
            .checked_sub(first_window.start_sample)
            .ok_or_else(|| Failure::invalid("shot audio window overflow"))?;
        if frames == 0 {
            return Err(Failure::invalid("selected shot range contains no source audio samples"));
        }
        let bytes = asset
            .data
            .wave()
            .encode_slice(first_window.start_sample, frames, audio::MAX_AUDIO_BYTES + 4096)
            .map_err(Failure::invalid)?;
        let info = json!({"asset":asset.id,"source_sha256":asset.data.sha256(),"sha256":format!("{:x}",Sha256::digest(&bytes)),
            "start_sample":first_window.start_sample,"end_sample_exclusive":last_window.end_sample,"sample_frames":frames,
            "sample_rate":asset.data.wave().format().sample_rate,"channels":asset.data.wave().format().channels,
            "sample_semantics":"Whole-shot integer floor boundaries retained for partial selections; samples copied without conversion, padding or resampling"});
        Some((bytes, info))
    } else {
        None
    };
    Ok(prepared_audio)
}

/// Freeze the exact shot clock and prepared PCM sidecar for a persistent job.
pub(crate) fn prepare_job(document: &Document, request: &RenderShot) -> Result<sequence::PreparedExport, Failure> {
    let state = state(document, &request.shot).map_err(Failure::invalid)?;
    let selected = if let Some(selection) = &request.selection {
        state.select(selection.start_frame, selection.frame_count).map_err(Failure::invalid)?
    } else {
        &state.frames
    };
    let first = selected.first().ok_or_else(|| Failure::invalid("shot has no frames"))?;
    let last = selected.last().ok_or_else(|| Failure::invalid("shot has no frames"))?;
    let audio = prepare_audio(document, &state, first, last)?;
    let range = SequenceRequest {
        directory: request.directory.clone(),
        clip: state.shot.clip.clone(),
        start: first.sampled_time,
        end: last.sampled_time,
        fps: (f64::from(state.shot.rate.numerator) / f64::from(state.shot.rate.denominator)) as f32,
        playback: animation::Playback::Clamp,
        pass: request.pass.clone(),
        view: request.view.clone(),
        format: request.format.clone(),
    };
    let times: Vec<_> =
        selected.iter().map(|f| FrameTime { scheduled: f.scheduled_time, sampled: f.sampled_time }).collect();
    let mut manifest = sequence::prepare(document, &range, &times)?;
    manifest["format"] = json!("mm3e-shot-sequence");
    manifest["shot"] = json!(state.shot);
    manifest["selection"] = json!({"start_frame":first.frame,"frame_count":selected.len()});
    manifest["audio_rounding"] = json!(state.audio_rounding);
    manifest["time_semantics"]=json!("Scheduled time = (absolute frame - clip_frame_zero) * rate.denominator / rate.numerator; frame count is explicit with an exclusive end; time is the exact f32 sample supplied to native animation");
    manifest["frames"]=json!(selected.iter().enumerate().map(|(i,f)|json!({"index":i,"time":f.sampled_time,"scheduled_time":f.scheduled_time,"shot_index":f.index,"shot_frame":f.frame,"audio_window":f.audio})).collect::<Vec<_>>());
    manifest["audio"] = audio.as_ref().map(|(_, info)| info.clone()).unwrap_or(Value::Null);
    Ok(sequence::PreparedExport { range, times, manifest, audio: audio.map(|(bytes, _)| bytes) })
}
