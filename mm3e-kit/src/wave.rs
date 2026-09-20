//! Bounded, immutable RIFF/WAVE reference audio. No file I/O, resampling, mixing or
//! sample conversion: the validated interleaved sample bytes remain unchanged.
//!
//! Format references:
//! - <https://learn.microsoft.com/en-us/windows/win32/xaudio2/resource-interchange-file-format--riff->
//! - <https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatex>
//! - <https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ksmedia/ns-ksmedia-waveformatextensible>
//! - <https://learn.microsoft.com/en-us/windows-hardware/drivers/audio/converting-between-format-tags-and-subformat-guids>

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleEncoding {
    /// Unsigned offset binary in 8-bit containers, signed two's complement otherwise.
    PcmInteger,
    IeeeFloat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveFormat {
    pub encoding: SampleEncoding,
    pub sample_rate: u32,
    pub channels: u16,
    /// Container size, not merely the precision of extensible PCM samples.
    pub bits_per_sample: u16,
    pub valid_bits_per_sample: u16,
    /// None for classic WAVEFORMAT; Some(0) means extensible direct/unassigned output.
    /// Partial masks are retained as authored, without inventing channel assignments.
    pub channel_mask: Option<u32>,
    pub extensible: bool,
    pub block_align: u16,
    pub byte_rate: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct WaveLimits {
    /// Entire input RIFF file, including ancillary chunks and padding.
    pub max_bytes: usize,
    pub max_frames: u64,
    pub max_channels: u16,
    /// Number of top-level RIFF child chunks walked. Ancillary payloads are opaque.
    pub max_chunks: usize,
}
impl Default for WaveLimits {
    fn default() -> Self {
        Self { max_bytes: 256 * 1024 * 1024, max_frames: 100_000_000, max_channels: 32, max_chunks: 4096 }
    }
}

/// Only `parse` can construct a Wave. Accessors cannot mutate its validated metadata
/// or sample bytes. Clones are independent owned copies; evaluation has no cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wave {
    format: WaveFormat,
    format_chunk: Vec<u8>,
    samples: Vec<u8>,
    frame_count: u64,
    had_fact: bool,
}

impl Wave {
    /// Parse one complete little-endian RIFF/WAVE file. Supported encodings are integer
    /// PCM8/16/24/32 and IEEE float32/64, including their 40-byte extensible descriptors.
    /// Extensible PCM requires 1..=container valid bits and zero unused low bits.
    /// Every floating-point sample must be finite; values outside [-1,1] remain intact.
    ///
    /// Chunk order is unrestricted. Every declared payload and required odd pad byte
    /// must fit the RIFF boundary exactly. Duplicate fmt/data/fact chunks, conflicting
    /// fact counts, inconsistent byte rates/alignment, partial frames, and trailing
    /// bytes are errors. Float input without fact is accepted because complete frames
    /// are independently determined by data length and block alignment.
    ///
    /// Unknown ancillary chunks are skipped as opaque bounded payloads. Segmented
    /// wavl/slnt audio, compression, big-endian RIFX, RF64/BW64 and unknown format
    /// extensions are explicitly unsupported. This does not validate ancillary tags.
    pub fn parse(bytes: &[u8], limits: WaveLimits) -> Result<Self, String> {
        if bytes.len() > limits.max_bytes {
            return Err(format!("WAVE input exceeds byte limit {}", limits.max_bytes));
        }
        if bytes.len() < 12 {
            return Err("truncated RIFF/WAVE header".into());
        }
        if &bytes[..4] != b"RIFF" {
            return Err("unsupported audio container: expected little-endian RIFF/WAVE (not RF64/RIFX/BW64)".into());
        }
        if &bytes[8..12] != b"WAVE" {
            return Err("RIFF form is not WAVE audio".into());
        }
        let riff_size = u32_at(bytes, 4)? as usize;
        let end = riff_size.checked_add(8).ok_or("RIFF length overflow")?;
        if riff_size < 4 || end != bytes.len() {
            return Err("RIFF size does not match complete input (truncation or trailing bytes)".into());
        }
        let mut offset = 12;
        let mut chunks = 0usize;
        let mut fmt = None;
        let mut data = None;
        let mut fact = None;
        while offset < end {
            chunks = chunks.checked_add(1).ok_or("WAVE chunk count overflow")?;
            if chunks > limits.max_chunks {
                return Err(format!("WAVE exceeds chunk limit {}", limits.max_chunks));
            }
            let header_end = offset.checked_add(8).filter(|&n| n <= end).ok_or("truncated WAVE chunk header")?;
            let id = &bytes[offset..offset + 4];
            let size = u32_at(bytes, offset + 4)? as usize;
            let payload_end =
                header_end.checked_add(size).filter(|&n| n <= end).ok_or("truncated WAVE chunk payload")?;
            let next =
                payload_end.checked_add(size & 1).filter(|&n| n <= end).ok_or("missing WAVE odd-chunk pad byte")?;
            let payload = &bytes[header_end..payload_end];
            match id {
                b"fmt " => {
                    if fmt.replace(payload).is_some() {
                        return Err("duplicate WAVE fmt chunk".into());
                    }
                }
                b"data" => {
                    if data.replace(payload).is_some() {
                        return Err("duplicate WAVE data chunk (segmented audio is unsupported)".into());
                    }
                }
                b"fact" => {
                    let frames = u32_at(payload, 0).map_err(|_| "truncated WAVE fact frame count")?;
                    if fact.replace(frames).is_some() {
                        return Err("duplicate WAVE fact chunk".into());
                    }
                }
                b"slnt" => return Err("unsupported WAVE slnt/segmented audio".into()),
                b"LIST" if payload.starts_with(b"wavl") => return Err("unsupported WAVE wavl/segmented audio".into()),
                _ => {}
            }
            offset = next;
        }
        let fmt = fmt.ok_or("WAVE requires a fmt chunk")?;
        let data = data.ok_or("WAVE requires a data chunk")?;
        let format = parse_format(fmt, limits.max_channels)?;
        if data.len() % usize::from(format.block_align) != 0 {
            return Err("WAVE data length is not a whole number of interleaved frames".into());
        }
        let frame_count = (data.len() / usize::from(format.block_align)) as u64;
        if frame_count > limits.max_frames {
            return Err(format!("WAVE exceeds frame limit {}", limits.max_frames));
        }
        if fact.is_some_and(|count| u64::from(count) != frame_count) {
            return Err("WAVE fact frame count conflicts with complete data frames".into());
        }
        let bytes_per_sample = usize::from(format.bits_per_sample / 8);
        let unused = format.bits_per_sample - format.valid_bits_per_sample;
        let unused_mask = (1u64 << unused) - 1;
        for (index, sample) in data.chunks_exact(bytes_per_sample).enumerate() {
            match format.encoding {
                SampleEncoding::IeeeFloat if !decode_sample(format, sample).is_finite() => {
                    return Err(format!("WAVE floating-point sample {index} is nonfinite"));
                }
                SampleEncoding::PcmInteger if unused > 0 && unsigned_sample(sample) & unused_mask != 0 => {
                    return Err(format!("WAVE PCM sample {index} has nonzero unused low bits"));
                }
                _ => {}
            }
        }
        Ok(Self {
            format,
            format_chunk: copy_bytes(fmt)?,
            samples: copy_bytes(data)?,
            frame_count,
            had_fact: fact.is_some(),
        })
    }

    pub fn format(&self) -> &WaveFormat {
        &self.format
    }
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
    /// Exact original interleaved data payload; excludes its RIFF pad byte.
    pub fn raw_samples(&self) -> &[u8] {
        &self.samples
    }
    /// Exact original fmt payload, including its classic/extensible representation.
    pub fn format_chunk(&self) -> &[u8] {
        &self.format_chunk
    }
    pub fn raw_frame(&self, frame: u64) -> Option<&[u8]> {
        if frame >= self.frame_count {
            return None;
        }
        let start = usize::try_from(frame.checked_mul(u64::from(self.format.block_align))?).ok()?;
        self.samples.get(start..start.checked_add(usize::from(self.format.block_align))?)
    }
    /// PCM maps to [-1,1), with the negative limit exactly -1. Float values are read
    /// without clipping, including values above 1, subnormals and signed zero.
    pub fn sample(&self, frame: u64, channel: u16) -> Option<f64> {
        if channel >= self.format.channels {
            return None;
        }
        let data = self.raw_frame(frame)?;
        let size = usize::from(self.format.bits_per_sample / 8);
        let start = usize::from(channel) * size;
        Some(decode_sample(self.format, &data[start..start + size]))
    }
    pub fn frame(&self, frame: u64) -> Option<Vec<f64>> {
        self.raw_frame(frame)?;
        Some((0..self.format.channels).map(|channel| self.sample(frame, channel).unwrap()).collect())
    }

    /// Encode `[start_frame, start_frame + frame_count)` without sample conversion.
    /// The original fmt bytes and selected sample bytes are retained exactly. A fact
    /// chunk is written for float/extensible formats or if the source had one, with
    /// the new exact frame count. Unrelated ancillary metadata is omitted: retaining
    /// old cue positions or sample loops on a sliced clip would be misleading.
    /// Empty, in-range slices are supported. Every size is checked before allocation.
    pub fn encode_slice(&self, start_frame: u64, frame_count: u64, max_output_bytes: usize) -> Result<Vec<u8>, String> {
        let end_frame = start_frame.checked_add(frame_count).ok_or("WAVE slice frame range overflow")?;
        if end_frame > self.frame_count {
            return Err("WAVE slice is outside the source frame range".into());
        }
        let align = u64::from(self.format.block_align);
        let start = usize::try_from(start_frame.checked_mul(align).ok_or("WAVE slice byte offset overflow")?)
            .map_err(|_| "WAVE slice byte offset exceeds address space")?;
        let data_len = usize::try_from(frame_count.checked_mul(align).ok_or("WAVE slice byte length overflow")?)
            .map_err(|_| "WAVE slice byte length exceeds address space")?;
        let data_size = u32::try_from(data_len).map_err(|_| "WAVE data exceeds RIFF's 32-bit chunk size")?;
        let fact = self.had_fact || self.format.extensible || self.format.encoding == SampleEncoding::IeeeFloat;
        let fmt_len = self.format_chunk.len();
        let total = 12usize
            .checked_add(8 + fmt_len + (fmt_len & 1))
            .and_then(|n| n.checked_add(if fact { 12 } else { 0 }))
            .and_then(|n| n.checked_add(8))
            .and_then(|n| n.checked_add(data_len))
            .and_then(|n| n.checked_add(data_len & 1))
            .ok_or("WAVE encoded file length overflow")?;
        if total > max_output_bytes {
            return Err(format!("WAVE output exceeds byte limit {max_output_bytes}"));
        }
        let riff_size = u32::try_from(total - 8).map_err(|_| "WAVE output exceeds RIFF's 32-bit file size")?;
        let frames = u32::try_from(frame_count).map_err(|_| "WAVE fact count exceeds 32 bits")?;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(total).map_err(|_| "cannot allocate bounded WAVE output")?;
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&riff_size.to_le_bytes());
        bytes.extend_from_slice(b"WAVEfmt ");
        bytes.extend_from_slice(&(fmt_len as u32).to_le_bytes());
        bytes.extend_from_slice(&self.format_chunk);
        if fmt_len & 1 != 0 {
            bytes.push(0);
        }
        if fact {
            bytes.extend_from_slice(b"fact");
            bytes.extend_from_slice(&4u32.to_le_bytes());
            bytes.extend_from_slice(&frames.to_le_bytes());
        }
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_size.to_le_bytes());
        bytes.extend_from_slice(&self.samples[start..start + data_len]);
        if data_len & 1 != 0 {
            bytes.push(0);
        }
        Ok(bytes)
    }
}

fn parse_format(bytes: &[u8], max_channels: u16) -> Result<WaveFormat, String> {
    if bytes.len() < 16 {
        return Err("truncated WAVE fmt descriptor".into());
    }
    let tag = u16_at(bytes, 0)?;
    let channels = u16_at(bytes, 2)?;
    let sample_rate = u32_at(bytes, 4)?;
    let byte_rate = u32_at(bytes, 8)?;
    let block_align = u16_at(bytes, 12)?;
    let bits_per_sample = u16_at(bytes, 14)?;
    if channels == 0 || channels > max_channels {
        return Err(format!("WAVE channels must be 1..={max_channels}"));
    }
    if sample_rate == 0 {
        return Err("WAVE sample rate must be positive".into());
    }
    let (encoding, valid_bits_per_sample, channel_mask, extensible) = match tag {
        1 | 3 => {
            if bytes.len() != 16 && bytes.len() != 18 {
                return Err("unsupported/truncated classic WAVE format extension".into());
            }
            // Microsoft explicitly specifies that PCM cbSize is ignored, even if a
            // producer left a nonzero value. Preserve it instead of rewriting it.
            if tag == 3 && bytes.len() == 18 && u16_at(bytes, 16)? != 0 {
                return Err("unsupported IEEE-float WAVE format extension".into());
            }
            (
                if tag == 1 { SampleEncoding::PcmInteger } else { SampleEncoding::IeeeFloat },
                bits_per_sample,
                None,
                false,
            )
        }
        0xfffe => {
            if bytes.len() != 40 || u16_at(bytes, 16)? != 22 {
                return Err(
                    "unsupported/truncated WAVEFORMATEXTENSIBLE extension (requires 40-byte fmt, cbSize 22)".into()
                );
            }
            let guid = &bytes[24..40];
            let suffix = [0, 0, 0, 0, 0x10, 0, 0x80, 0, 0, 0xaa, 0, 0x38, 0x9b, 0x71];
            if guid[2..] != suffix {
                return Err("unsupported WAVEFORMATEXTENSIBLE subformat GUID".into());
            }
            let encoding = match u16_at(guid, 0)? {
                1 => SampleEncoding::PcmInteger,
                3 => SampleEncoding::IeeeFloat,
                _ => return Err("unsupported compressed WAVEFORMATEXTENSIBLE subformat".into()),
            };
            let valid = u16_at(bytes, 18)?;
            let mask = u32_at(bytes, 20)?;
            if mask & !0x0003_ffff != 0 {
                return Err("unsupported reserved WAVE speaker-mask bits".into());
            }
            (encoding, valid, Some(mask), true)
        }
        _ => return Err(format!("unsupported WAVE encoding tag {tag:#06x} (uncompressed PCM/IEEE float required)")),
    };
    let valid_container = match encoding {
        SampleEncoding::PcmInteger => matches!(bits_per_sample, 8 | 16 | 24 | 32),
        SampleEncoding::IeeeFloat => matches!(bits_per_sample, 32 | 64),
    };
    if !valid_container {
        return Err(format!("unsupported WAVE {:?} container width {bits_per_sample}", encoding));
    }
    if valid_bits_per_sample == 0 || valid_bits_per_sample > bits_per_sample {
        return Err("WAVE valid sample bits must be 1..=container bits".into());
    }
    if encoding == SampleEncoding::IeeeFloat && valid_bits_per_sample != bits_per_sample {
        return Err("IEEE-float WAVE valid bits must equal the complete float container".into());
    }
    let expected_align = u32::from(channels) * u32::from(bits_per_sample / 8);
    if expected_align != u32::from(block_align) {
        return Err("WAVE blockAlign conflicts with channels and sample container size".into());
    }
    let expected_rate = sample_rate.checked_mul(expected_align).ok_or("WAVE byteRate arithmetic overflow")?;
    if expected_rate != byte_rate {
        return Err("WAVE byteRate conflicts with sample rate and blockAlign".into());
    }
    Ok(WaveFormat {
        encoding,
        sample_rate,
        channels,
        bits_per_sample,
        valid_bits_per_sample,
        channel_mask,
        extensible,
        block_align,
        byte_rate,
    })
}

fn unsigned_sample(bytes: &[u8]) -> u64 {
    bytes.iter().enumerate().fold(0, |value, (index, &byte)| value | (u64::from(byte) << (index * 8)))
}
fn decode_sample(format: WaveFormat, bytes: &[u8]) -> f64 {
    match (format.encoding, format.bits_per_sample) {
        (SampleEncoding::PcmInteger, 8) => (f64::from(bytes[0]) - 128.0) / 128.0,
        (SampleEncoding::PcmInteger, bits) => {
            let raw = unsigned_sample(bytes);
            let shift = 64 - bits;
            let signed = ((raw << shift) as i64) >> shift;
            signed as f64 / ((1u64 << (bits - 1)) as f64)
        }
        (SampleEncoding::IeeeFloat, 32) => f64::from(f32::from_le_bytes(bytes.try_into().unwrap())),
        (SampleEncoding::IeeeFloat, 64) => f64::from_le_bytes(bytes.try_into().unwrap()),
        _ => unreachable!("Wave formats are validated at construction"),
    }
}
fn u16_at(bytes: &[u8], offset: usize) -> Result<u16, String> {
    Ok(u16::from_le_bytes(bytes.get(offset..offset + 2).ok_or("truncated WAVE integer")?.try_into().unwrap()))
}
fn u32_at(bytes: &[u8], offset: usize) -> Result<u32, String> {
    Ok(u32::from_le_bytes(bytes.get(offset..offset + 4).ok_or("truncated WAVE integer")?.try_into().unwrap()))
}
fn copy_bytes(source: &[u8]) -> Result<Vec<u8>, String> {
    let mut copy = Vec::new();
    copy.try_reserve_exact(source.len()).map_err(|_| "cannot allocate bounded WAVE sample storage")?;
    copy.extend_from_slice(source);
    Ok(copy)
}
