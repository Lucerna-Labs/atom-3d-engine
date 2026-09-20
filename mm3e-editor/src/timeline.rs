//! Durable integer-frame shots over the existing seconds-based animation evaluator.
//! Audio references map whole-shot frame indices to half-open source PCM windows.
//! This module does not decode, resample, pad, transcribe, or infer speech timing.
use crate::{
    animation::{AnimationSample, Clip, Playback},
    model::{identifier, range},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

pub const MAX_SHOTS: usize = 64;
pub const MAX_SHOT_FRAMES: u32 = 2400;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FrameRate {
    pub numerator: u32,
    pub denominator: u32,
}
impl FrameRate {
    pub fn validate(self) -> Result<(), String> {
        if self.numerator == 0 || self.denominator == 0 {
            return Err("frame rate numerator and denominator must be positive".into());
        }
        // Exact rational comparison at both limits, without a rounded floating rate.
        if u64::from(self.numerator) * 1000 < u64::from(self.denominator)
            || u64::from(self.numerator) > 240 * u64::from(self.denominator)
        {
            return Err("frame rate must be in [0.001,240] frames per second".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AudioPlacement {
    pub asset: String,
    /// Source PCM frame (one sample per channel) corresponding to shot index zero.
    pub start_sample: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TimelineShot {
    pub id: String,
    pub clip: String,
    pub rate: FrameRate,
    pub start_frame: i64,
    pub frame_count: u32,
    /// Absolute shot-frame coordinate at which the animation clip is at zero seconds.
    pub clip_frame_zero: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioPlacement>,
}

/// Metadata supplied by the audio owner after validating actual PCM data.
/// `sample_frames` counts per-channel sample frames, not interleaved scalar samples.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AudioTiming {
    pub sample_rate: u32,
    pub sample_frames: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AudioWindow {
    pub start_sample: u64,
    pub end_sample: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ShotFrame {
    /// Index in the COMPLETE shot. Partial selections retain this index and floor phase.
    pub index: u32,
    pub frame: i64,
    pub scheduled_time: f64,
    /// Exact value passed to existing animation evaluation.
    pub sampled_time: f32,
    pub audio: Option<AudioWindow>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ShotState {
    pub shot: TimelineShot,
    pub end_frame_exclusive: i64,
    pub end_clip_time_exclusive: f64,
    pub audio_timing: Option<AudioTiming>,
    pub audio_range: Option<AudioWindow>,
    /// Fractional audio sample left after the final floor, denominator = rate.numerator.
    pub audio_end_fraction_numerator: Option<u32>,
    pub audio_rounding: String,
    pub time_precision: String,
    pub frames: Vec<ShotFrame>,
}

impl ShotState {
    /// Select absolute frame numbers without recomputing or resetting audio floor phase.
    pub fn select(&self, start_frame: i64, frame_count: u32) -> Result<&[ShotFrame], String> {
        if frame_count == 0 {
            return Err("shot selection must contain at least one frame".into());
        }
        let offset =
            start_frame.checked_sub(self.shot.start_frame).ok_or("shot selection frame subtraction overflow")?;
        if offset < 0 {
            return Err("shot selection starts before the shot".into());
        }
        let offset = usize::try_from(offset).map_err(|_| "shot selection index is outside addressable range")?;
        let end = offset.checked_add(frame_count as usize).ok_or("shot selection frame count overflow")?;
        self.frames.get(offset..end).ok_or_else(|| "shot selection ends outside the shot".into())
    }

    pub fn sample(&self, frame: i64) -> Result<AnimationSample, String> {
        let selected = self.select(frame, 1)?;
        Ok(AnimationSample { clip: self.shot.clip.clone(), time: selected[0].sampled_time, playback: Playback::Clamp })
    }
}

fn scheduled_time(shot: &TimelineShot, frame: i64) -> Result<f64, String> {
    // Subtract exact integers first: large absolute frame numbers can differ by one
    // even when their individual f64 representations would be indistinguishable.
    let relative = frame.checked_sub(shot.clip_frame_zero).ok_or("shot frame minus clip_frame_zero overflow")?;
    if relative < 0 {
        return Err("shot frame precedes clip time zero".into());
    }
    let time = relative as f64 * f64::from(shot.rate.denominator) / f64::from(shot.rate.numerator);
    if !time.is_finite() {
        return Err("shot scheduled time must be finite".into());
    }
    Ok(time)
}

fn audio_boundary(shot: &TimelineShot, timing: AudioTiming, index: u32) -> Result<(u64, u32), String> {
    let placement = shot.audio.as_ref().ok_or("missing shot audio placement")?;
    let product = u128::from(index)
        .checked_mul(u128::from(timing.sample_rate))
        .and_then(|value| value.checked_mul(u128::from(shot.rate.denominator)))
        .ok_or("shot audio frame-to-sample multiplication overflow")?;
    let divisor = u128::from(shot.rate.numerator);
    let whole = u64::try_from(product / divisor).map_err(|_| "shot audio sample index exceeds u64")?;
    let sample = placement.start_sample.checked_add(whole).ok_or("shot audio start_sample addition overflow")?;
    Ok((sample, (product % divisor) as u32))
}

/// Schedule exactly frame_count output instants. The complete half-open shot interval,
/// including its exclusive endpoint, must fit the clip. Existing seconds-sequence
/// scheduling is separate and retains its original inclusive endpoint contract.
pub fn schedule(shot: &TimelineShot, clip: &Clip, audio: Option<AudioTiming>) -> Result<ShotState, String> {
    identifier(&shot.id)?;
    identifier(&shot.clip)?;
    if shot.clip != clip.id {
        return Err(format!("shot {} references clip {}, not {}", shot.id, shot.clip, clip.id));
    }
    shot.rate.validate()?;
    if !(1..=MAX_SHOT_FRAMES).contains(&shot.frame_count) {
        return Err(format!("shot frame_count must be in 1..={MAX_SHOT_FRAMES}"));
    }
    range(clip.duration, 0.001, 3600.0, "clip duration")?;
    let end_frame =
        shot.start_frame.checked_add(i64::from(shot.frame_count)).ok_or("shot exclusive end frame overflow")?;
    let start_time = scheduled_time(shot, shot.start_frame)?;
    let end_time = scheduled_time(shot, end_frame)?;
    // Clip duration is authored as f32. Compare at that actual animation precision:
    // a rational 0.7-second endpoint must agree with its stored 0.7f32 duration.
    // Preserve the exact f64 schedule in metadata rather than adding an epsilon.
    if start_time as f32 > clip.duration || end_time as f32 > clip.duration {
        return Err("complete half-open shot range must fit within the animation clip duration".into());
    }
    let (audio_range, remainder) = match (&shot.audio, audio) {
        (None, None) => (None, None),
        (None, Some(_)) => return Err("audio metadata supplied for a shot without an audio placement".into()),
        (Some(placement), None) => {
            return Err(format!("missing timing metadata for shot audio asset {}", placement.asset))
        }
        (Some(placement), Some(timing)) => {
            identifier(&placement.asset)?;
            if timing.sample_rate == 0 || timing.sample_frames == 0 {
                return Err("shot reference audio requires a positive sample rate and nonempty PCM data".into());
            }
            let (end_sample, remainder) = audio_boundary(shot, timing, shot.frame_count)?;
            if placement.start_sample > timing.sample_frames || end_sample > timing.sample_frames {
                return Err("shot audio window extends beyond the referenced PCM samples".into());
            }
            (Some(AudioWindow { start_sample: placement.start_sample, end_sample }), Some(remainder))
        }
    };
    let mut frames = Vec::<ShotFrame>::with_capacity(shot.frame_count as usize);
    for index in 0..shot.frame_count {
        let frame = shot.start_frame.checked_add(i64::from(index)).ok_or("shot frame index overflow")?;
        let scheduled = scheduled_time(shot, frame)?;
        let sampled = scheduled as f32;
        if !sampled.is_finite() || sampled < 0.0 || sampled > clip.duration {
            return Err("shot sampled animation time is outside the clip".into());
        }
        if frames.last().is_some_and(|previous| sampled <= previous.sampled_time) {
            return Err("distinct shot frames collapse at animation's f32 time precision".into());
        }
        let window = audio
            .map(|timing| {
                let (start_sample, _) = audio_boundary(shot, timing, index)?;
                let (end_sample, _) = audio_boundary(shot, timing, index + 1)?;
                Ok::<_, String>(AudioWindow { start_sample, end_sample })
            })
            .transpose()?;
        frames.push(ShotFrame { index, frame, scheduled_time: scheduled, sampled_time: sampled, audio: window });
    }
    Ok(ShotState {
        shot: shot.clone(), end_frame_exclusive: end_frame, end_clip_time_exclusive: end_time,
        audio_timing: audio, audio_range, audio_end_fraction_numerator: remainder,
        audio_rounding: "source sample window at whole-shot index i is [start_sample + floor(i * sample_rate * rate.denominator / rate.numerator), start_sample + floor((i+1) * sample_rate * rate.denominator / rate.numerator)); partial frame selections retain whole-shot indices; no resampling or padding".into(),
        time_precision: "scheduled times use exact integer frame differences followed by f64 rational-rate conversion; animation samples and clip-range endpoint comparisons use the existing f32 clip precision, with no arbitrary epsilon or time clamping".into(),
        frames,
    })
}

pub fn validate(
    shots: &[TimelineShot],
    clips: &[Clip],
    mut audio_lookup: impl FnMut(&str) -> Result<AudioTiming, String>,
) -> Result<(), String> {
    if shots.len() > MAX_SHOTS {
        return Err(format!("document supports at most {MAX_SHOTS} timeline shots"));
    }
    let mut ids = BTreeSet::new();
    for shot in shots {
        if !ids.insert(&shot.id) {
            return Err(format!("duplicate timeline shot id {}", shot.id));
        }
        let clip = clips
            .iter()
            .find(|clip| clip.id == shot.clip)
            .ok_or_else(|| format!("missing timeline clip {}", shot.clip))?;
        let audio = shot.audio.as_ref().map(|placement| audio_lookup(&placement.asset)).transpose()?;
        schedule(shot, clip, audio).map_err(|error| format!("shot {}: {error}", shot.id))?;
    }
    Ok(())
}

pub fn inspect(shot: &TimelineShot, clip: &Clip, audio: Option<AudioTiming>) -> Result<Value, String> {
    serde_json::to_value(schedule(shot, clip, audio)?).map_err(|error| error.to_string())
}
