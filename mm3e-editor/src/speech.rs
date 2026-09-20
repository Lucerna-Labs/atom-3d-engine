//! Local speech-analysis evidence. Original audio remains embedded and unchanged;
//! recognizer inputs and mouth cues use an explicit selected sample range.
use crate::{audio::AudioRange, model::Document, protocol::Failure};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

pub const MAX_ANALYSIS_SECONDS: u64 = 120;
pub const MAX_REPORT_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_CUES: usize = 12000;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Recognizer {
    #[default]
    English,
    Phonetic,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
pub enum MouthShape {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    X,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BackendIdentity {
    pub name: String,
    pub version: String,
    pub binary_sha256: String,
    pub resources_sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SpeechSource {
    pub audio: String,
    pub source_sha256: String,
    pub sample_rate: u32,
    pub channel: u16,
    pub range: AudioRange,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MouthCue {
    pub start_cs: u32,
    pub end_cs: u32,
    pub shape: MouthShape,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SpeechReport {
    pub format: String,
    pub source: SpeechSource,
    pub backend: BackendIdentity,
    pub recognizer: Recognizer,
    pub dialogue_hint: Option<String>,
    pub extended_shapes: String,
    pub input_wav_sha256: String,
    pub duration_cs: u32,
    pub cues: Vec<MouthCue>,
    pub clipped_samples: u64,
    pub raw_output_sha256: String,
    pub raw_output: Value,
}
fn extended() -> String {
    "X".into()
}
fn timeout() -> u32 {
    120
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AnalyzeSpeech {
    pub audio: String,
    pub directory: String,
    #[serde(default)]
    pub range: Option<AudioRange>,
    /// Mono recordings infer channel zero; multichannel input requires selection.
    #[serde(default)]
    pub channel: Option<u16>,
    #[serde(default)]
    pub recognizer: Recognizer,
    #[serde(default)]
    pub dialogue_hint: Option<String>,
    #[serde(default = "extended")]
    pub extended_shapes: String,
    #[serde(default = "timeout")]
    pub timeout_seconds: u32,
}

use crate::{audio, model::identifier, speech_backend::SpeechBackend, storage};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::Read,
};
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn hash_valid(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn shape_allowed(shape: MouthShape, extended: &str) -> bool {
    match shape {
        MouthShape::G => extended.contains('G'),
        MouthShape::H => extended.contains('H'),
        MouthShape::X => extended.contains('X'),
        _ => true,
    }
}
fn validate_options(recognizer: Recognizer, hint: Option<&str>, extended: &str) -> Result<(), String> {
    if extended.len() > 3
        || extended.bytes().any(|b| !b"GHX".contains(&b))
        || extended.bytes().enumerate().any(|(i, b)| extended.as_bytes()[..i].contains(&b))
    {
        return Err("extended_shapes must contain distinct letters from GHX".into());
    }
    if let Some(hint) = hint {
        if recognizer != Recognizer::English {
            return Err("dialogue hints require the English recognizer".into());
        }
        if hint.trim().is_empty() || hint.len() > 65536 || hint.contains('\0') {
            return Err("dialogue hint must contain 1..65536 UTF-8 bytes without NUL".into());
        }
    }
    Ok(())
}
fn cs(value: &Value) -> Result<u32, String> {
    let value = value.as_f64().ok_or("speech cue time must be numeric")?;
    let units = value * 100.0;
    if !units.is_finite()
        || units < 0.0
        || units > MAX_ANALYSIS_SECONDS as f64 * 100.0
        || (units - units.round()).abs() > 1e-7
    {
        return Err("speech cue times must be nonnegative centiseconds within the analysis budget".into());
    }
    Ok(units.round() as u32)
}
fn parse_raw(raw: &Value) -> Result<(u32, Vec<MouthCue>), String> {
    let sound = raw["metadata"]["soundFile"].as_str().ok_or("speech result missing metadata.soundFile")?;
    if sound.is_empty() || sound.len() > 8192 {
        return Err("invalid speech result source label".into());
    }
    let duration = cs(&raw["metadata"]["duration"])?;
    let array = raw["mouthCues"].as_array().ok_or("speech result missing mouthCues")?;
    if array.is_empty() || array.len() > MAX_CUES {
        return Err("speech result requires 1..12000 mouth cues".into());
    }
    let mut cues = Vec::with_capacity(array.len());
    for cue in array {
        let shape: MouthShape =
            serde_json::from_value(cue["value"].clone()).map_err(|_| "unknown speech mouth shape")?;
        cues.push(MouthCue { start_cs: cs(&cue["start"])?, end_cs: cs(&cue["end"])?, shape });
    }
    Ok((duration, cues))
}
pub(crate) fn validate_report(report: &SpeechReport) -> Result<(), String> {
    if report.format != "mm3e-speech-analysis-v1" {
        return Err("unsupported speech report format".into());
    }
    identifier(&report.source.audio)?;
    if !(8000..=192000).contains(&report.source.sample_rate)
        || report.source.channel >= audio::MAX_AUDIO_CHANNELS
        || report.source.range.frame_count == 0
        || report.source.range.frame_count > u64::from(report.source.sample_rate) * MAX_ANALYSIS_SECONDS
        || report
            .source
            .range
            .start_sample
            .checked_add(report.source.range.frame_count)
            .is_none_or(|v| v > 192000 * 120)
    {
        return Err("speech source range/rate/channel is invalid".into());
    }
    if report.clipped_samples > report.source.range.frame_count {
        return Err("speech clipped sample count exceeds the source range".into());
    }
    for hash in [
        &report.source.source_sha256,
        &report.input_wav_sha256,
        &report.raw_output_sha256,
        &report.backend.binary_sha256,
        &report.backend.resources_sha256,
    ] {
        if !hash_valid(hash) {
            return Err("speech report requires lowercase SHA-256 identities".into());
        }
    }
    if report.backend.name != "rhubarb" || report.backend.version != "1.14.0" {
        return Err("speech report requires the declared Rhubarb 1.14.0 backend".into());
    }
    validate_options(report.recognizer, report.dialogue_hint.as_deref(), &report.extended_shapes)?;
    let expected = report.source.range.frame_count * 100 / u64::from(report.source.sample_rate);
    if report.duration_cs == 0 || u64::from(report.duration_cs) != expected {
        return Err("recognizer duration must equal the source duration truncated to centiseconds".into());
    }
    if report.cues.is_empty() || report.cues.len() > MAX_CUES {
        return Err("speech report has an invalid cue count".into());
    }
    let mut previous = 0;
    for cue in &report.cues {
        if cue.start_cs != previous
            || cue.end_cs <= cue.start_cs
            || cue.end_cs > report.duration_cs
            || !shape_allowed(cue.shape, &report.extended_shapes)
        {
            return Err("speech cues must be contiguous positive intervals using enabled shapes".into());
        }
        previous = cue.end_cs;
    }
    if previous != report.duration_cs {
        return Err("speech cues must cover the reported centisecond duration".into());
    }
    let raw_bytes = serde_json::to_vec(&report.raw_output).map_err(|e| e.to_string())?;
    if raw_bytes.len() > MAX_REPORT_BYTES || digest(&raw_bytes) != report.raw_output_sha256 {
        return Err("speech raw-output identity differs".into());
    }
    let (duration, cues) = parse_raw(&report.raw_output)?;
    if duration != report.duration_cs
        || cues.len() != report.cues.len()
        || cues
            .iter()
            .zip(&report.cues)
            .any(|(a, b)| a.start_cs != b.start_cs || a.end_cs != b.end_cs || a.shape != b.shape)
    {
        return Err("speech normalized cues differ from preserved recognizer output".into());
    }
    Ok(())
}
pub(crate) fn read_report_checked(
    document: &Document,
    root: &Path,
    path: &str,
    expected_hash: Option<&str>,
) -> Result<SpeechReport, Failure> {
    let path = storage::path(root, path, true)?;
    let mut bytes = Vec::new();
    File::open(&path)
        .map_err(|e| Failure::io(e.to_string()))?
        .take((MAX_REPORT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| Failure::io(e.to_string()))?;
    if bytes.len() > MAX_REPORT_BYTES {
        return Err(Failure::invalid("speech report exceeds 2 MiB"));
    }
    if let Some(expected) = expected_hash {
        if !hash_valid(expected) || digest(&bytes) != expected {
            return Err(Failure {
                code: "speech_report_mismatch",
                message: "speech report file differs from the requested SHA-256 identity".into(),
            });
        }
    }
    let report: SpeechReport =
        serde_json::from_slice(&bytes).map_err(|e| Failure::invalid(format!("invalid speech report: {e}")))?;
    validate_report(&report).map_err(Failure::invalid)?;
    let asset = audio::get(document, &report.source.audio).map_err(Failure::invalid)?;
    if asset.data.sha256() != report.source.source_sha256
        || asset.data.wave().format().sample_rate != report.source.sample_rate
        || report.source.channel >= asset.data.wave().format().channels
        || report.source.range.start_sample + report.source.range.frame_count > asset.data.wave().frame_count()
    {
        return Err(Failure {
            code: "speech_source_mismatch",
            message: "speech report does not match the current audio asset and selected range".into(),
        });
    }
    let (prepared, clipped) =
        analysis_wave(asset, &report.source.range, report.source.channel).map_err(Failure::invalid)?;
    if digest(&prepared) != report.input_wav_sha256 || clipped != report.clipped_samples {
        return Err(Failure {
            code: "speech_source_mismatch",
            message: "speech analysis input identity differs from the selected original audio samples".into(),
        });
    }
    Ok(report)
}

/// A deterministic analysis-only conversion. Preserve source frame count/rate;
/// select one declared channel, quantize signed PCM16, and report clipped floats.
fn analysis_wave(asset: &audio::AudioAsset, range: &AudioRange, channel: u16) -> Result<(Vec<u8>, u64), String> {
    let wave = asset.data.wave();
    let data_size =
        range.frame_count.checked_mul(2).and_then(|n| u32::try_from(n).ok()).ok_or("speech WAV size overflow")?;
    let mut bytes = Vec::with_capacity(data_size as usize + 44);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_size).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&wave.format().sample_rate.to_le_bytes());
    bytes.extend_from_slice(&(wave.format().sample_rate * 2).to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_size.to_le_bytes());
    let mut clipped = 0;
    for index in range.start_sample..range.start_sample + range.frame_count {
        let value = wave.sample(index, channel).ok_or("missing selected speech sample")?;
        if !(-1.0..=1.0).contains(&value) {
            clipped += 1;
        }
        let pcm = (value.clamp(-1.0, 1.0) * 32768.0).round().clamp(-32768.0, 32767.0) as i16;
        bytes.extend_from_slice(&pcm.to_le_bytes());
    }
    Ok((bytes, clipped))
}
pub fn analyze(
    document: &Document,
    root: &Path,
    backend: &SpeechBackend,
    request: &AnalyzeSpeech,
) -> Result<Value, Failure> {
    let asset = audio::get(document, &request.audio).map_err(Failure::invalid)?;
    validate_options(request.recognizer, request.dialogue_hint.as_deref(), &request.extended_shapes)
        .map_err(Failure::invalid)?;
    if !(1..=600).contains(&request.timeout_seconds) {
        return Err(Failure::invalid("speech timeout_seconds must be 1..600"));
    }
    let wave = asset.data.wave();
    let range = request.range.clone().unwrap_or(AudioRange { start_sample: 0, frame_count: wave.frame_count() });
    if range.frame_count == 0
        || range.frame_count > u64::from(wave.format().sample_rate) * MAX_ANALYSIS_SECONDS
        || range.start_sample.checked_add(range.frame_count).is_none_or(|n| n > wave.frame_count())
        || range.frame_count * 100 < u64::from(wave.format().sample_rate)
    {
        return Err(Failure::invalid("speech range must be within the audio and contain 0.01..120 seconds"));
    }
    let channel = match request.channel {
        Some(c) if c < wave.format().channels => c,
        Some(_) => return Err(Failure::invalid("speech channel is outside the audio")),
        None if wave.format().channels == 1 => 0,
        None => return Err(Failure::invalid("multichannel speech audio requires an explicit channel index")),
    };
    let target = storage::path(root, &request.directory, false)?;
    if fs::symlink_metadata(&target).is_ok() {
        return Err(Failure::invalid("speech analysis directory already exists; use a new directory"));
    }
    let (input, clipped) = analysis_wave(asset, &range, channel).map_err(Failure::invalid)?;
    fs::create_dir(&target).map_err(|e| Failure::io(e.to_string()))?;
    let work = (|| -> Result<Value, Failure> {
        storage::write(&target, "input.wav", &input, false)?;
        if let Some(text) = &request.dialogue_hint {
            storage::write(&target, "dialogue.txt", text.as_bytes(), false)?;
        }
        let raw = backend.run(&target, request)?;
        let (duration, cues) = parse_raw(&raw).map_err(Failure::invalid)?;
        let report = SpeechReport {
            format: "mm3e-speech-analysis-v1".into(),
            source: SpeechSource {
                audio: asset.id.clone(),
                source_sha256: asset.data.sha256().into(),
                sample_rate: wave.format().sample_rate,
                channel,
                range: range.clone(),
            },
            backend: backend.identity().clone(),
            recognizer: request.recognizer,
            dialogue_hint: request.dialogue_hint.clone(),
            extended_shapes: request.extended_shapes.clone(),
            input_wav_sha256: digest(&input),
            duration_cs: duration,
            cues,
            clipped_samples: clipped,
            raw_output_sha256: digest(&serde_json::to_vec(&raw).map_err(|e| Failure::invalid(e.to_string()))?),
            raw_output: raw,
        };
        validate_report(&report).map_err(Failure::invalid)?;
        let bytes = serde_json::to_vec_pretty(&report).map_err(|e| Failure::invalid(e.to_string()))?;
        if bytes.len() > MAX_REPORT_BYTES {
            return Err(Failure::invalid("speech report exceeds 2 MiB"));
        }
        let path = storage::write(&target, "analysis.json", &bytes, false)?;
        let raw_bytes = fs::read(target.join("raw.json")).map_err(|e| Failure::io(e.to_string()))?;
        Ok(
            json!({"analysis_path":path,"analysis_sha256":digest(&bytes),"raw_file_sha256":digest(&raw_bytes),"source":report.source,"backend":report.backend,"cues":report.cues,"clipped_samples":clipped,"analyzed_seconds":range.frame_count as f64/f64::from(wave.format().sample_rate),"reported_seconds":f64::from(duration)/100.0,"unreported_tail_seconds":range.frame_count as f64/f64::from(wave.format().sample_rate)-f64::from(duration)/100.0,"semantics":"Local recognizer mouth-shape cues at centisecond resolution; dialogue is a recognition hint, not forced transcript alignment; original source bytes are unchanged. Prepared PCM16 selects one channel without resampling; clipped_samples counts source values outside [-1,1]."}),
        )
    })();
    work.map_err(|error| Failure {
        code: error.code,
        message: format!(
            "speech analysis failed in {}: {}; prepared audio and logs are preserved",
            target.display(),
            error.message
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn wav(rate: u32, channels: u16, bits: u16, tag: u16, data: &[u8]) -> Vec<u8> {
        let block = channels * (bits / 8);
        let mut b = vec![];
        b.extend_from_slice(b"RIFF");
        b.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
        b.extend_from_slice(b"WAVEfmt ");
        b.extend_from_slice(&16u32.to_le_bytes());
        b.extend_from_slice(&tag.to_le_bytes());
        b.extend_from_slice(&channels.to_le_bytes());
        b.extend_from_slice(&rate.to_le_bytes());
        b.extend_from_slice(&(rate * u32::from(block)).to_le_bytes());
        b.extend_from_slice(&block.to_le_bytes());
        b.extend_from_slice(&bits.to_le_bytes());
        b.extend_from_slice(b"data");
        b.extend_from_slice(&(data.len() as u32).to_le_bytes());
        b.extend_from_slice(data);
        b
    }
    fn asset(bytes: Vec<u8>) -> audio::AudioAsset {
        audio::AudioAsset {
            id: "voice".into(),
            label: String::new(),
            data: audio::EmbeddedWave::from_bytes(bytes).unwrap(),
        }
    }
    #[test]
    fn pcm16_analysis_copy_preserves_every_signed_endpoint_and_original_bytes() {
        let samples = [-32768_i16, -32767, -1, 0, 1, 32766, 32767];
        let raw: Vec<_> = samples.iter().flat_map(|v| v.to_le_bytes()).collect();
        let bytes = wav(16000, 1, 16, 1, &raw);
        let audio = asset(bytes.clone());
        let (prepared, clipped) =
            analysis_wave(&audio, &AudioRange { start_sample: 0, frame_count: samples.len() as u64 }, 0).unwrap();
        assert_eq!(prepared, bytes);
        assert_eq!(clipped, 0);
        assert_eq!(audio.data.encoded_bytes(), bytes);
    }
    #[test]
    fn channel_range_and_float_clipping_are_explicit_without_resampling() {
        let samples = [0.0_f32, 0.25, 0.5, -1.5, -0.5, 1.5, 0.25, -0.25];
        let raw: Vec<_> = samples.iter().flat_map(|v| v.to_le_bytes()).collect();
        let bytes = wav(44100, 2, 32, 3, &raw);
        let audio = asset(bytes.clone());
        let (prepared, clipped) = analysis_wave(&audio, &AudioRange { start_sample: 1, frame_count: 3 }, 1).unwrap();
        let wave = mm3e_kit::wave::Wave::parse(&prepared, mm3e_kit::wave::WaveLimits::default()).unwrap();
        assert_eq!(wave.format().sample_rate, 44100);
        assert_eq!(wave.format().channels, 1);
        assert_eq!(wave.frame_count(), 3);
        assert_eq!(clipped, 2);
        assert_eq!(
            wave.raw_samples(),
            [-32768_i16, 32767, -8192].iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<_>>()
        );
        assert_eq!(audio.data.encoded_bytes(), bytes);
    }
    fn report() -> SpeechReport {
        let raw = json!({"metadata":{"soundFile":"input.wav","duration":0.02},"mouthCues":[{"start":0.0,"end":0.01,"value":"A"},{"start":0.01,"end":0.02,"value":"X"}]});
        SpeechReport {
            format: "mm3e-speech-analysis-v1".into(),
            source: SpeechSource {
                audio: "voice".into(),
                source_sha256: "a".repeat(64),
                sample_rate: 16000,
                channel: 0,
                range: AudioRange { start_sample: 11, frame_count: 330 },
            },
            backend: BackendIdentity {
                name: "rhubarb".into(),
                version: "1.14.0".into(),
                binary_sha256: "b".repeat(64),
                resources_sha256: "c".repeat(64),
            },
            recognizer: Recognizer::English,
            dialogue_hint: None,
            extended_shapes: "X".into(),
            input_wav_sha256: "d".repeat(64),
            duration_cs: 2,
            cues: vec![
                MouthCue { start_cs: 0, end_cs: 1, shape: MouthShape::A },
                MouthCue { start_cs: 1, end_cs: 2, shape: MouthShape::X },
            ],
            clipped_samples: 0,
            raw_output_sha256: digest(&serde_json::to_vec(&raw).unwrap()),
            raw_output: raw,
        }
    }
    #[test]
    fn report_cues_are_an_exact_contiguous_projection_of_raw_centiseconds() {
        let original = report();
        validate_report(&original).unwrap();
        let mut changed = original.clone();
        changed.cues[1].start_cs = 0;
        assert!(validate_report(&changed).is_err());
        let mut changed = original.clone();
        changed.cues[0].shape = MouthShape::D;
        assert!(validate_report(&changed).is_err());
        let mut changed = original.clone();
        changed.extended_shapes.clear();
        assert!(validate_report(&changed).is_err());
        let mut changed = original.clone();
        changed.source.range.frame_count = 480;
        assert!(validate_report(&changed).is_err());
        let mut changed = original.clone();
        changed.raw_output["mouthCues"][0]["start"] = json!(0.001);
        changed.raw_output_sha256 = digest(&serde_json::to_vec(&changed.raw_output).unwrap());
        assert!(validate_report(&changed).is_err());
        let mut changed = original;
        changed.clipped_samples = 331;
        assert!(validate_report(&changed).is_err());
    }
}
