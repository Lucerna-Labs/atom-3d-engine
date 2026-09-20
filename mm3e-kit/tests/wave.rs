use mm3e_kit::wave::{SampleEncoding, Wave, WaveLimits};

fn fmt(tag: u16, channels: u16, bits: u16) -> Vec<u8> {
    let align = channels * (bits / 8);
    let mut bytes = vec![];
    bytes.extend(tag.to_le_bytes());
    bytes.extend(channels.to_le_bytes());
    bytes.extend(44_100u32.to_le_bytes());
    bytes.extend((44_100 * u32::from(align)).to_le_bytes());
    bytes.extend(align.to_le_bytes());
    bytes.extend(bits.to_le_bytes());
    bytes
}

fn extensible(tag: u16, channels: u16, bits: u16, valid: u16, mask: u32) -> Vec<u8> {
    let mut bytes = fmt(0xfffe, channels, bits);
    bytes.extend(22u16.to_le_bytes());
    bytes.extend(valid.to_le_bytes());
    bytes.extend(mask.to_le_bytes());
    bytes.extend(u32::from(tag).to_le_bytes());
    bytes.extend([0, 0, 0x10, 0, 0x80, 0, 0, 0xaa, 0, 0x38, 0x9b, 0x71]);
    bytes
}

fn riff(chunks: &[([u8; 4], &[u8])]) -> Vec<u8> {
    let mut bytes = b"RIFF\0\0\0\0WAVE".to_vec();
    for (id, payload) in chunks {
        bytes.extend(id);
        bytes.extend((payload.len() as u32).to_le_bytes());
        bytes.extend(*payload);
        if payload.len() & 1 != 0 {
            // RIFF requires a pad byte but does not require its contents to be zero.
            bytes.push(0xa5);
        }
    }
    let len = (bytes.len() - 8) as u32;
    bytes[4..8].copy_from_slice(&len.to_le_bytes());
    bytes
}

fn parse(format: &[u8], data: &[u8]) -> Wave {
    Wave::parse(&riff(&[(*b"fmt ", format), (*b"data", data)]), WaveLimits::default()).unwrap()
}

fn pcm24(values: &[i32]) -> Vec<u8> {
    values.iter().flat_map(|value| value.to_le_bytes()[..3].to_vec()).collect()
}

#[test]
fn integer_containers_decode_signed_limits_and_interleaved_frames_exactly() {
    let cases = [
        (8, vec![0, 255, 128, 127], 128.0),
        (16, [-32768i16, 32767, 0, -1].into_iter().flat_map(i16::to_le_bytes).collect(), 32768.0),
        (24, pcm24(&[-8_388_608, 8_388_607, 0, -1]), 8_388_608.0),
        (32, [i32::MIN, i32::MAX, 0, -1].into_iter().flat_map(i32::to_le_bytes).collect(), 2_147_483_648.0),
    ];
    for (bits, data, denominator) in cases {
        let format = fmt(1, 2, bits);
        let wave = parse(&format, &data);
        assert_eq!(wave.format().encoding, SampleEncoding::PcmInteger);
        assert_eq!(wave.format().channels, 2);
        assert_eq!(wave.format().sample_rate, 44_100);
        assert_eq!(wave.format().bits_per_sample, bits);
        assert_eq!(wave.format().valid_bits_per_sample, bits);
        assert_eq!(wave.format().channel_mask, None);
        assert_eq!(wave.frame_count(), 2);
        assert_eq!(wave.frame(0).unwrap(), vec![-1.0, (denominator - 1.0) / denominator]);
        assert_eq!(wave.frame(1).unwrap(), vec![0.0, -1.0 / denominator]);
        assert_eq!(wave.raw_samples(), data);
        assert_eq!(wave.format_chunk(), format);
        assert_eq!(wave.raw_frame(1).unwrap(), &data[data.len() / 2..]);
        assert_eq!(wave.sample(2, 0), None);
        assert_eq!(wave.sample(0, 2), None);
        assert_eq!(wave.frame(u64::MAX), None);
    }
}

#[test]
fn float32_and_float64_keep_over_range_values_subnormals_and_signed_zero() {
    let data32: Vec<_> = [-0.0f32, 2.5, -3.125, f32::from_bits(1)].into_iter().flat_map(f32::to_le_bytes).collect();
    let wave32 = parse(&fmt(3, 2, 32), &data32);
    assert!(wave32.sample(0, 0).unwrap().is_sign_negative());
    assert_eq!(wave32.sample(0, 1), Some(2.5));
    assert_eq!(wave32.sample(1, 0), Some(-3.125));
    assert_eq!(wave32.sample(1, 1), Some(f64::from(f32::from_bits(1))));
    let data64: Vec<_> =
        [-0.0f64, 4.5, -7.25, f64::from_bits(1), f64::MAX, -f64::MAX].into_iter().flat_map(f64::to_le_bytes).collect();
    let wave64 = parse(&fmt(3, 2, 64), &data64);
    assert!(wave64.sample(0, 0).unwrap().is_sign_negative());
    assert_eq!(wave64.frame(0).unwrap(), vec![-0.0, 4.5]);
    assert_eq!(wave64.frame(1).unwrap(), vec![-7.25, f64::from_bits(1)]);
    assert_eq!(wave64.frame(2).unwrap(), vec![f64::MAX, -f64::MAX]);
    for wave in [wave32, wave64] {
        let encoded = wave.encode_slice(1, 1, 4096).unwrap();
        let reread = Wave::parse(&encoded, WaveLimits::default()).unwrap();
        assert_eq!(reread.frame_count(), 1);
        assert_eq!(reread.raw_samples(), wave.raw_frame(1).unwrap());
        assert_eq!(reread.format_chunk(), wave.format_chunk());
        assert!(encoded.windows(4).any(|chunk| chunk == b"fact"));
    }
}

#[test]
fn extensible_pcm_precision_and_channel_metadata_survive_slices_without_quantization() {
    let format = extensible(1, 2, 24, 20, 3);
    let data = pcm24(&[-524_288 << 4, 524_287 << 4, -1 << 4, 0]);
    let wave = parse(&format, &data);
    assert_eq!(wave.format().valid_bits_per_sample, 20);
    assert_eq!(wave.format().channel_mask, Some(3));
    assert!(wave.format().extensible);
    assert_eq!(wave.frame(0).unwrap(), vec![-1.0, 524_287.0 / 524_288.0]);
    assert_eq!(wave.frame(1).unwrap(), vec![-1.0 / 524_288.0, 0.0]);
    let slice = wave.encode_slice(1, 1, 1024).unwrap();
    let output = Wave::parse(&slice, WaveLimits::default()).unwrap();
    assert_eq!(output.format(), wave.format());
    assert_eq!(output.format_chunk(), format);
    assert_eq!(output.raw_samples(), &data[6..]);
    let mut bad_data = data;
    bad_data[0] |= 1;
    assert!(Wave::parse(&riff(&[(*b"fmt ", &format), (*b"data", &bad_data)]), WaveLimits::default())
        .unwrap_err()
        .contains("unused low bits"));
}

#[test]
fn extensible_float_direct_output_and_partial_masks_keep_all_channels_in_order() {
    let data: Vec<_> = [2.5f32, -3.125, 0.25].into_iter().flat_map(f32::to_le_bytes).collect();
    for mask in [0, 3, 7] {
        let format = extensible(3, 3, 32, 32, mask);
        let wave = parse(&format, &data);
        assert_eq!(wave.format().channel_mask, Some(mask));
        assert_eq!(wave.frame(0).unwrap(), vec![2.5, -3.125, 0.25]);
        let encoded = wave.encode_slice(0, 1, 1024).unwrap();
        let output = Wave::parse(&encoded, WaveLimits::default()).unwrap();
        assert_eq!(output.format_chunk(), format);
        assert_eq!(output.raw_samples(), data);
    }
}

#[test]
fn riff_chunk_order_odd_padding_and_opaque_ancillary_payloads_are_handled() {
    let format = fmt(1, 1, 8);
    let data = [0, 128, 255];
    let bytes = riff(&[(*b"JUNK", &[1, 2, 3]), (*b"data", &data), (*b"LIST", b"INFOignored"), (*b"fmt ", &format)]);
    let wave = Wave::parse(&bytes, WaveLimits::default()).unwrap();
    assert_eq!(wave.frame_count(), 3);
    assert_eq!(wave.raw_samples(), data);
    let output = wave.encode_slice(0, 3, 1024).unwrap();
    assert_eq!(output.len(), 48);
    assert_eq!(*output.last().unwrap(), 0);
    assert!(!output.windows(4).any(|window| window == b"JUNK" || window == b"LIST"));
    assert_eq!(Wave::parse(&output, WaveLimits::default()).unwrap().raw_samples(), data);
}

#[test]
fn fact_counts_are_checked_and_rewritten_to_the_exact_slice_frame_count() {
    let format = fmt(3, 2, 32);
    let data = [0u8; 32];
    let bytes = riff(&[(*b"fact", &4u32.to_le_bytes()), (*b"fmt ", &format), (*b"data", &data)]);
    let wave = Wave::parse(&bytes, WaveLimits::default()).unwrap();
    let output = wave.encode_slice(1, 2, 1024).unwrap();
    assert_eq!(Wave::parse(&output, WaveLimits::default()).unwrap().frame_count(), 2);
    let index = output.windows(4).position(|bytes| bytes == b"fact").unwrap();
    assert_eq!(&output[index + 8..index + 12], &2u32.to_le_bytes());
    for count in [0u32, 3, 5, u32::MAX] {
        let bad = riff(&[(*b"fmt ", &format), (*b"fact", &count.to_le_bytes()), (*b"data", &data)]);
        assert!(Wave::parse(&bad, WaveLimits::default()).unwrap_err().contains("fact frame count"));
    }
}

#[test]
fn duplicates_conflicts_missing_chunks_and_segmented_audio_are_explicit_errors() {
    let format = fmt(1, 1, 8);
    let second_format = fmt(1, 2, 8);
    let data = [128, 128];
    for chunks in [
        vec![(*b"fmt ", format.as_slice())],
        vec![(*b"data", data.as_slice())],
        vec![(*b"fmt ", &format), (*b"fmt ", &format), (*b"data", &data)],
        vec![(*b"fmt ", &format), (*b"fmt ", &second_format), (*b"data", &data)],
        vec![(*b"fmt ", &format), (*b"data", &data), (*b"data", &[])],
        vec![(*b"fmt ", &format), (*b"data", &data), (*b"fact", &[2, 0, 0, 0]), (*b"fact", &[2, 0, 0, 0])],
        vec![(*b"fmt ", &format), (*b"data", &data), (*b"fact", &[2, 0, 0])],
        vec![(*b"fmt ", &format), (*b"data", &data), (*b"LIST", b"wavl")],
        vec![(*b"fmt ", &format), (*b"data", &data), (*b"slnt", &[0, 0, 0, 0])],
    ] {
        assert!(Wave::parse(&riff(&chunks), WaveLimits::default()).is_err());
    }
}

#[test]
fn all_truncations_missing_pad_trailing_bytes_and_forged_lengths_fail_without_panics() {
    let format = fmt(1, 1, 8);
    let bytes = riff(&[(*b"fmt ", &format), (*b"JUNK", &[9]), (*b"data", &[0, 128, 255])]);
    for length in 0..bytes.len() {
        assert!(Wave::parse(&bytes[..length], WaveLimits::default()).is_err(), "accepted truncation at {length}");
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(Wave::parse(&trailing, WaveLimits::default()).is_err());
    let mut missing_pad = bytes.clone();
    missing_pad.pop();
    let size = (missing_pad.len() - 8) as u32;
    missing_pad[4..8].copy_from_slice(&size.to_le_bytes());
    assert!(Wave::parse(&missing_pad, WaveLimits::default()).unwrap_err().contains("pad byte"));
    for offset in [4, 16] {
        let mut forged = bytes.clone();
        forged[offset..offset + 4].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(Wave::parse(&forged, WaveLimits::default()).is_err());
    }
}

#[test]
fn invalid_rate_alignment_channels_and_partial_frames_are_rejected() {
    let base = fmt(1, 2, 24);
    let input =
        |format: &[u8], data: &[u8]| Wave::parse(&riff(&[(*b"fmt ", format), (*b"data", data)]), WaveLimits::default());
    for (offset, value) in [(2, 0u16), (12, 0), (12, 5), (14, 12), (14, 64)] {
        let mut format = base.clone();
        format[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
        assert!(input(&format, &[0; 6]).is_err());
    }
    for (offset, value) in [(4, 0u32), (8, 0), (8, 1)] {
        let mut format = base.clone();
        format[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        assert!(input(&format, &[0; 6]).is_err());
    }
    let mut overflow = base;
    overflow[4..8].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(input(&overflow, &[0; 6]).unwrap_err().contains("overflow"));
    assert!(input(&fmt(1, 2, 24), &[0; 5]).unwrap_err().contains("whole number"));
}

#[test]
fn unsupported_encodings_extensible_variants_and_containers_do_not_silently_convert() {
    let data = [0u8; 8];
    let mut formats = vec![fmt(6, 1, 8), fmt(17, 1, 8), fmt(3, 1, 16), fmt(1, 1, 64)];
    let mut bad_guid = extensible(1, 1, 16, 16, 1);
    bad_guid[39] ^= 1;
    formats.extend([
        bad_guid,
        extensible(6, 1, 16, 16, 1),
        extensible(1, 1, 16, 0, 1),
        extensible(1, 1, 16, 17, 1),
        extensible(3, 1, 32, 24, 1),
        extensible(1, 1, 16, 16, 0x8000_0000),
    ]);
    let mut extra = extensible(1, 1, 16, 16, 1);
    extra.extend([0, 0]);
    extra[16..18].copy_from_slice(&24u16.to_le_bytes());
    formats.push(extra);
    for format in formats {
        assert!(Wave::parse(&riff(&[(*b"fmt ", &format), (*b"data", &data)]), WaveLimits::default()).is_err());
    }
    let original = riff(&[(*b"fmt ", &fmt(1, 1, 8)), (*b"data", &data)]);
    for id in [b"RIFX", b"RF64", b"BW64", b"riff"] {
        let mut bytes = original.clone();
        bytes[..4].copy_from_slice(id);
        assert!(Wave::parse(&bytes, WaveLimits::default()).unwrap_err().contains("unsupported audio container"));
    }
}

#[test]
fn classic_pcm_cbsize_is_ignored_as_specified_but_float_extensions_are_not() {
    let mut pcm = fmt(1, 1, 16);
    pcm.extend(9u16.to_le_bytes());
    let wave = parse(&pcm, &[0, 0]);
    let output = wave.encode_slice(0, 1, 1024).unwrap();
    assert_eq!(Wave::parse(&output, WaveLimits::default()).unwrap().format_chunk(), pcm);
    let mut float = fmt(3, 1, 32);
    float.extend(0u16.to_le_bytes());
    parse(&float, &[0; 4]);
    float[16] = 1;
    assert!(Wave::parse(&riff(&[(*b"fmt ", &float), (*b"data", &[0; 4])]), WaveLimits::default()).is_err());
}

#[test]
fn every_nonfinite_float_sample_is_rejected_even_in_an_otherwise_silent_file() {
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let data: Vec<_> = [0.0, 0.0, value, 0.0].into_iter().flat_map(f32::to_le_bytes).collect();
        assert!(Wave::parse(&riff(&[(*b"fmt ", &fmt(3, 2, 32)), (*b"data", &data)]), WaveLimits::default())
            .unwrap_err()
            .contains("sample 2 is nonfinite"));
    }
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let data: Vec<_> = [0.0, 0.0, value, 0.0].into_iter().flat_map(f64::to_le_bytes).collect();
        assert!(Wave::parse(
            &riff(&[(*b"fmt ", &extensible(3, 2, 64, 64, 3)), (*b"data", &data)]),
            WaveLimits::default()
        )
        .unwrap_err()
        .contains("sample 2 is nonfinite"));
    }
}

#[test]
fn parser_budgets_and_slice_limits_fail_before_unbounded_allocation() {
    let bytes = riff(&[(*b"fmt ", &fmt(1, 2, 16)), (*b"data", &[0; 16])]);
    for limits in [
        WaveLimits { max_bytes: bytes.len() - 1, ..Default::default() },
        WaveLimits { max_frames: 3, ..Default::default() },
        WaveLimits { max_channels: 1, ..Default::default() },
        WaveLimits { max_chunks: 1, ..Default::default() },
    ] {
        assert!(Wave::parse(&bytes, limits).is_err());
    }
    let wave = Wave::parse(&bytes, WaveLimits::default()).unwrap();
    let encoded = wave.encode_slice(1, 2, 1024).unwrap();
    assert!(wave.encode_slice(1, 2, encoded.len() - 1).is_err());
    assert!(wave.encode_slice(u64::MAX, 1, usize::MAX).unwrap_err().contains("overflow"));
    assert!(wave.encode_slice(3, 2, usize::MAX).is_err());
    assert!(wave.encode_slice(5, 0, usize::MAX).is_err());
    let empty = wave.encode_slice(4, 0, 1024).unwrap();
    let empty = Wave::parse(&empty, WaveLimits { max_frames: 0, ..Default::default() }).unwrap();
    assert_eq!(empty.frame_count(), 0);
    assert_eq!(empty.sample(0, 0), None);
}

#[test]
fn byte_preservation_and_out_of_order_parallel_reads_do_not_mutate_source_or_state() {
    let source =
        riff(&[(*b"fmt ", &fmt(1, 2, 24)), (*b"data", &pcm24(&[0, 0, -8_388_608, 8_388_607, -1, 123456, 0, 0]))]);
    let before = source.clone();
    let wave = Wave::parse(&source, WaveLimits::default()).unwrap();
    let expected = wave.encode_slice(1, 2, 1024).unwrap();
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    assert_eq!(wave.frame(2).unwrap(), vec![-1.0 / 8_388_608.0, 123456.0 / 8_388_608.0]);
                    assert_eq!(wave.sample(0, 0), Some(0.0));
                    wave.encode_slice(1, 2, 1024).unwrap()
                })
            })
            .collect();
        for job in jobs {
            assert_eq!(job.join().unwrap(), expected);
        }
    });
    assert_eq!(source, before);
    assert_eq!(Wave::parse(&expected, WaveLimits::default()).unwrap().raw_samples(), &wave.raw_samples()[6..18]);
}

#[test]
#[ignore = "independent Python wave + libsndfile reader; run explicitly where python3/libsndfile are available"]
fn independent_readers_confirm_exact_pcm24_and_unclipped_float_slice_exports() {
    use std::{
        fs,
        process::Command,
        time::{SystemTime, UNIX_EPOCH},
    };
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let directory = std::env::var_os("MM3E_WAVE_READER_ARTIFACTS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join(format!("mm3e-wave-readers-{}-{suffix}", std::process::id())));
    fs::create_dir_all(&directory).unwrap();
    let pcm = parse(&fmt(1, 2, 24), &pcm24(&[0, 0, -8_388_608, 8_388_607, -1, 123456, 0, 0]));
    fs::write(directory.join("pcm24.wav"), pcm.encode_slice(1, 2, 1024).unwrap()).unwrap();
    let f32_data: Vec<_> =
        [0.0f32, 0.0, 2.5, -3.125, -0.0, f32::MIN_POSITIVE, 0.0, 0.0].into_iter().flat_map(f32::to_le_bytes).collect();
    let f64_data: Vec<_> =
        [0.0f64, 0.0, 4.5, -7.25, -0.0, 1e-280, 0.0, 0.0].into_iter().flat_map(f64::to_le_bytes).collect();
    for (name, wave) in [
        ("float32.wav", parse(&fmt(3, 2, 32), &f32_data)),
        ("float64.wav", parse(&fmt(3, 2, 64), &f64_data)),
        ("extensible-float.wav", parse(&extensible(3, 2, 32, 32, 3), &f32_data)),
    ] {
        fs::write(directory.join(name), wave.encode_slice(1, 2, 1024).unwrap()).unwrap();
    }
    let script = r#"
import ctypes, ctypes.util, json, math, pathlib, sys, wave
root = pathlib.Path(sys.argv[1])
with wave.open(str(root / 'pcm24.wav'), 'rb') as reader:
    assert (reader.getnchannels(), reader.getsampwidth(), reader.getframerate(), reader.getnframes()) == (2, 3, 44100, 2)
    assert reader.readframes(2) == bytes.fromhex('000080ffff7fffffff40e201')
class Info(ctypes.Structure):
    _fields_ = [('frames', ctypes.c_int64), ('samplerate', ctypes.c_int), ('channels', ctypes.c_int), ('format', ctypes.c_int), ('sections', ctypes.c_int), ('seekable', ctypes.c_int)]
library = ctypes.util.find_library('sndfile')
assert library, 'libsndfile is required for independent float reader acceptance'
sf = ctypes.CDLL(library)
sf.sf_open.argtypes = [ctypes.c_char_p, ctypes.c_int, ctypes.POINTER(Info)]
sf.sf_open.restype = ctypes.c_void_p
sf.sf_readf_double.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_double), ctypes.c_int64]
sf.sf_readf_double.restype = ctypes.c_int64
sf.sf_close.argtypes = [ctypes.c_void_p]
sf.sf_close.restype = ctypes.c_int
report = {'pcm24_exact_sample_bytes': True, 'float_files': []}
for name, expected, subtype in [('float32.wav', [2.5, -3.125, -0.0, 2**-126], 6), ('float64.wav', [4.5, -7.25, -0.0, 1e-280], 7), ('extensible-float.wav', [2.5, -3.125, -0.0, 2**-126], 6)]:
    info = Info()
    handle = sf.sf_open(str(root / name).encode(), 0x10, ctypes.byref(info))
    assert handle, name
    try:
        assert (info.frames, info.samplerate, info.channels, info.format & 0xffff) == (2, 44100, 2, subtype), (name, info.format)
        values = (ctypes.c_double * 4)()
        assert sf.sf_readf_double(handle, values, 2) == 2
        assert list(values) == expected, (name, list(values), expected)
        assert math.copysign(1, values[2]) == -1
        report['float_files'].append({'name': name, 'samples': list(values), 'frames': info.frames, 'rate': info.samplerate, 'channels': info.channels})
    finally:
        assert sf.sf_close(handle) == 0
(root / 'independent-reader.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps(report))
"#;
    fs::write(directory.join("reader.py"), script).unwrap();
    let output = Command::new("python3").arg(directory.join("reader.py")).arg(&directory).output().unwrap();
    fs::write(directory.join("stdout.log"), &output.stdout).unwrap();
    fs::write(directory.join("stderr.log"), &output.stderr).unwrap();
    assert!(
        output.status.success(),
        "independent reader failed; artifacts {}: {}",
        directory.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    println!("Independent WAV reader artifacts: {}", directory.display());
}
