//! Durable reference audio. Original WAV bytes are embedded and shared by history
//! snapshots; validated decoding and hashes are reconstructed once on import/load.
use crate::{
    model::{identifier, Document},
    protocol::Failure,
    storage,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use mm3e_kit::wave::{SampleEncoding, Wave, WaveLimits};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs::File, io::Read, path::Path, sync::Arc};

pub const MAX_AUDIO_ASSETS: usize = 16;
pub const MAX_AUDIO_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_AUDIO_CHANNELS: u16 = 8;
pub const MAX_WAVEFORM_BINS: u16 = 2048;

#[derive(Clone, Debug)]
pub struct EmbeddedWave {
    bytes: Arc<[u8]>,
    decoded: Arc<Wave>,
    sha256: String,
    encoded: Arc<str>,
}
impl EmbeddedWave {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
        let decoded = Wave::parse(
            &bytes,
            WaveLimits {
                max_bytes: MAX_AUDIO_BYTES,
                max_frames: 192_000 * 120,
                max_channels: MAX_AUDIO_CHANNELS,
                max_chunks: 1024,
            },
        )?;
        if decoded.frame_count() == 0 {
            return Err("reference audio must contain at least one sample frame".into());
        }
        if !(8000..=192000).contains(&decoded.format().sample_rate) {
            return Err("reference audio sample rate must be 8000..192000 Hz".into());
        }
        let sha256 = format!("{:x}", Sha256::digest(&bytes));
        let encoded = STANDARD.encode(&bytes).into();
        Ok(Self { bytes: bytes.into(), decoded: Arc::new(decoded), sha256, encoded })
    }
    pub fn wave(&self) -> &Wave {
        &self.decoded
    }
    pub fn sha256(&self) -> &str {
        &self.sha256
    }
    pub fn encoded_bytes(&self) -> &[u8] {
        &self.bytes
    }
}
impl Serialize for EmbeddedWave {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.encoded.as_ref().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for EmbeddedWave {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let encoded = String::deserialize(deserializer)?;
        if encoded.len() > MAX_AUDIO_BYTES.div_ceil(3) * 4 {
            return Err(serde::de::Error::custom("embedded reference WAV exceeds encoded byte budget"));
        }
        let bytes = STANDARD.decode(encoded).map_err(serde::de::Error::custom)?;
        Self::from_bytes(bytes).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AudioAsset {
    pub id: String,
    #[serde(default)]
    pub label: String,
    /// Standard padded base64 of the original bounded WAV, including source metadata.
    /// Original data is embedded; there is no filesystem dependency after import.
    #[schemars(with = "String")]
    pub data: EmbeddedWave,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ImportAudio {
    pub id: String,
    #[serde(default)]
    pub label: String,
    pub path: String,
    #[serde(default)]
    pub replace: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AudioRange {
    /// Interleaved sample FRAME index: one simultaneous sample per channel.
    pub start_sample: u64,
    pub frame_count: u64,
}
fn bins() -> u16 {
    64
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AudioState {
    pub id: String,
    #[serde(default)]
    pub range: Option<AudioRange>,
    /// Maximum number of equal-time waveform bins; never more than the sample-frame count.
    #[serde(default = "bins")]
    pub bins: u16,
    /// Optional exact source sample-frame indices, independent of the waveform range.
    #[serde(default)]
    pub samples: Vec<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExportAudio {
    pub id: String,
    pub path: String,
    pub range: AudioRange,
    #[serde(default)]
    pub overwrite: bool,
}

pub fn validate(assets: &[AudioAsset]) -> Result<(), String> {
    if assets.len() > MAX_AUDIO_ASSETS {
        return Err("reference audio exceeds 16 assets".into());
    }
    let mut ids = BTreeSet::new();
    let mut bytes = 0usize;
    for asset in assets {
        identifier(&asset.id)?;
        if !ids.insert(&asset.id) {
            return Err(format!("duplicate audio asset {}", asset.id));
        }
        if asset.label.len() > 1024 {
            return Err("audio label exceeds 1024 bytes".into());
        }
        bytes = bytes.checked_add(asset.data.encoded_bytes().len()).ok_or("audio byte count overflow")?;
        if bytes > MAX_AUDIO_BYTES {
            return Err("embedded reference audio exceeds the shared 8 MiB source-byte budget".into());
        }
    }
    Ok(())
}
pub fn get<'a>(document: &'a Document, id: &str) -> Result<&'a AudioAsset, String> {
    document.audio.iter().find(|asset| asset.id == id).ok_or_else(|| format!("missing audio asset {id}"))
}
pub fn metadata(asset: &AudioAsset) -> Value {
    let wave = asset.data.wave();
    let format = wave.format();
    json!({"id":asset.id,"label":asset.label,"source_sha256":asset.data.sha256(),"source_bytes":asset.data.encoded_bytes().len(),
        "sample_rate":format.sample_rate,"channels":format.channels,"sample_frames":wave.frame_count(),
        "bits_per_sample":format.bits_per_sample,"valid_bits_per_sample":format.valid_bits_per_sample,
        "encoding":match format.encoding {SampleEncoding::PcmInteger=>"pcm_integer",SampleEncoding::IeeeFloat=>"ieee_float"},"channel_mask":format.channel_mask,"extensible":format.extensible,
        "duration_seconds":wave.frame_count() as f64/f64::from(format.sample_rate),
        "semantics":"Embedded source WAV; sample indices identify simultaneous channel frames; no resampling, gain change, speech recognition or lip alignment"})
}
pub fn import(document: &mut Document, root: &Path, request: &ImportAudio) -> Result<Value, Failure> {
    identifier(&request.id).map_err(Failure::invalid)?;
    if request.label.len() > 1024 {
        return Err(Failure::invalid("audio label exceeds 1024 bytes"));
    }
    let existing = document.audio.iter().position(|a| a.id == request.id);
    if existing.is_some() && !request.replace {
        return Err(Failure::invalid("audio asset exists; replace must be explicitly true"));
    }
    let path = storage::path(root, &request.path, true)?;
    if !path.metadata().map_err(|e| Failure::io(e.to_string()))?.is_file() {
        return Err(Failure::invalid("reference audio input must be a regular file"));
    }
    let mut bytes = vec![];
    File::open(path)
        .map_err(|e| Failure::io(e.to_string()))?
        .take(MAX_AUDIO_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| Failure::io(e.to_string()))?;
    if bytes.len() > MAX_AUDIO_BYTES {
        return Err(Failure::invalid("reference audio source exceeds 8 MiB"));
    }
    let asset = AudioAsset {
        id: request.id.clone(),
        label: request.label.clone(),
        data: EmbeddedWave::from_bytes(bytes).map_err(Failure::invalid)?,
    };
    let result = metadata(&asset);
    let mut candidate = document.audio.clone();
    if let Some(i) = existing {
        candidate[i] = asset;
    } else {
        candidate.push(asset);
    }
    validate(&candidate).map_err(Failure::invalid)?;
    document.audio = candidate;
    Ok(result)
}
pub fn remove(document: &mut Document, id: &str) -> Result<(), String> {
    let index = document.audio.iter().position(|a| a.id == id).ok_or_else(|| format!("missing audio asset {id}"))?;
    document.audio.remove(index);
    Ok(())
}
pub fn checked_range(asset: &AudioAsset, range: &AudioRange) -> Result<u64, String> {
    let end = range.start_sample.checked_add(range.frame_count).ok_or("audio range overflow")?;
    if range.frame_count == 0 || end > asset.data.wave().frame_count() {
        return Err("audio range must contain source samples and stay inside the embedded recording".into());
    }
    Ok(end)
}
pub fn inspect(document: &Document, request: &AudioState) -> Result<Value, String> {
    let asset = get(document, &request.id)?;
    let wave = asset.data.wave();
    if !(1..=MAX_WAVEFORM_BINS).contains(&request.bins) || request.samples.len() > 4096 {
        return Err("audio inspection allows 1..2048 waveform bins and at most 4096 exact sample queries".into());
    }
    let range = request.range.clone().unwrap_or(AudioRange { start_sample: 0, frame_count: wave.frame_count() });
    checked_range(asset, &range)?;
    let samples = request
        .samples
        .iter()
        .map(|&sample| {
            wave.frame(sample)
                .map(|values| json!({"sample":sample,"values":values}))
                .ok_or("audio sample index is outside source")
        })
        .collect::<Result<Vec<_>, _>>()?;
    let count = u64::from(request.bins).min(range.frame_count);
    let mut waveform = Vec::with_capacity(count as usize);
    for index in 0..count {
        let start =
            range.start_sample + ((u128::from(index) * u128::from(range.frame_count)) / u128::from(count)) as u64;
        let end =
            range.start_sample + ((u128::from(index + 1) * u128::from(range.frame_count)) / u128::from(count)) as u64;
        let mut channels = vec![];
        for channel in 0..wave.format().channels {
            let mut min = f64::INFINITY;
            let mut max = f64::NEG_INFINITY;
            let mut scale = 0.0;
            let mut squares = 0.0;
            // Scaled sum of squares keeps finite float audio near f64::MAX valid,
            // and retains very quiet samples without overflowing/underflowing v*v.
            for sample in start..end {
                let value = wave.sample(sample, channel).ok_or("invalid decoded audio sample")?;
                min = min.min(value);
                max = max.max(value);
                let magnitude = value.abs();
                if magnitude > scale {
                    squares = 1.0 + squares * (scale / magnitude).powi(2);
                    scale = magnitude;
                } else if scale > 0.0 {
                    squares += (magnitude / scale).powi(2);
                }
            }
            let rms = scale * (squares / (end - start) as f64).sqrt();
            channels.push(json!({"channel":channel,"min":min,"max":max,"rms":rms}));
        }
        waveform.push(json!({"start_sample":start,"end_sample_exclusive":end,"channels":channels}));
    }
    let mut result = metadata(asset);
    result["range"] = json!(range);
    result["waveform"] = json!(waveform);
    result["samples"] = json!(samples);
    Ok(result)
}
pub fn export(document: &Document, root: &Path, request: &ExportAudio) -> Result<Value, Failure> {
    if !request.path.to_ascii_lowercase().ends_with(".wav") {
        return Err(Failure::invalid("audio export path must end in .wav"));
    }
    let asset = get(document, &request.id).map_err(Failure::invalid)?;
    checked_range(asset, &request.range).map_err(Failure::invalid)?;
    let bytes = asset
        .data
        .wave()
        .encode_slice(request.range.start_sample, request.range.frame_count, MAX_AUDIO_BYTES + 4096)
        .map_err(Failure::invalid)?;
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    let path = storage::write(root, &request.path, &bytes, request.overwrite)?;
    Ok(json!({"path":path,"sha256":sha256,"source_sha256":asset.data.sha256(),"range":request.range,
        "sample_rate":asset.data.wave().format().sample_rate,"channels":asset.data.wave().format().channels,
        "semantics":"Exact sample-byte slice with validated format metadata; unrelated source RIFF metadata is omitted; no resampling or conversion"}))
}
