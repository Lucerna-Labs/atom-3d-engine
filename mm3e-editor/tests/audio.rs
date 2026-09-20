use mm3e_editor::{
    audio::{self, AudioAsset, AudioState, EmbeddedWave},
    model::Document,
    protocol::{Request, Response},
    Editor,
};
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

const SPEECH: &[u8] = include_bytes!("fixtures/dialogue/hello-reference.wav");
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "mm3e-audio-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn call(editor: &mut Editor, command: Value, revision: Option<u64>) -> Response {
    editor.handle(
        serde_json::from_value::<Request>(json!({"id":"audio-test","expected_revision":revision,"command":command}))
            .unwrap(),
    )
}
fn ok(editor: &mut Editor, command: Value, revision: Option<u64>) -> Value {
    let result = call(editor, command, revision);
    assert!(result.ok, "{result:?}");
    result.result.unwrap()
}
fn import() -> Value {
    json!({"op":"import_audio","request":{"id":"speech","path":"source.wav"}})
}

#[test]
fn source_is_embedded_and_reloaded_without_external_file_and_exact_slice_bytes_survive() {
    let scratch = Scratch::new();
    fs::write(scratch.0.join("source.wav"), SPEECH).unwrap();
    let mut editor = Editor::new(&scratch.0).unwrap();
    assert!(!call(&mut editor, import(), None).ok);
    let mut dry = import();
    dry["dry_run"] = json!(true);
    let preview = ok(&mut editor, dry, Some(0));
    assert_eq!(preview["sample_frames"], 46936);
    assert_eq!(editor.revision(), 0);
    assert!(editor.document().audio.is_empty());
    let imported = ok(&mut editor, import(), Some(0));
    assert_eq!(imported["source_sha256"], "c17de07edc5c105d80b880be27ffdef8f60011e7778d8bf09101b7e37c550f57");
    let before = serde_json::to_vec(editor.document()).unwrap();
    let state = ok(
        &mut editor,
        json!({"op":"audio_state","request":{"id":"speech","bins":11,"samples":[0,100,12345,46935]}}),
        None,
    );
    let bins = state["waveform"].as_array().unwrap();
    assert_eq!(bins.len(), 11);
    assert_eq!(bins[0]["start_sample"], 0);
    assert_eq!(bins[10]["end_sample_exclusive"], 46936);
    for pair in bins.windows(2) {
        assert_eq!(pair[0]["end_sample_exclusive"], pair[1]["start_sample"]);
    }
    ok(
        &mut editor,
        json!({"op":"export_audio","request":{"id":"speech","path":"slice.wav","range":{"start_sample":100,"frame_count":37}}}),
        None,
    );
    let slice = fs::read(scratch.0.join("slice.wav")).unwrap();
    let loaded = EmbeddedWave::from_bytes(slice).unwrap();
    assert_eq!(loaded.wave().raw_samples(), &editor.document().audio[0].data.wave().raw_samples()[200..274]);
    ok(&mut editor, json!({"op":"save","path":"saved.json"}), None);
    fs::remove_file(scratch.0.join("source.wav")).unwrap();
    drop(editor);
    let mut reopened = Editor::new(&scratch.0).unwrap();
    ok(&mut reopened, json!({"op":"load","path":"saved.json"}), Some(0));
    assert_eq!(serde_json::to_vec(reopened.document()).unwrap(), before);
    assert_eq!(reopened.document().audio[0].data.encoded_bytes(), SPEECH);
    let restored = ok(
        &mut reopened,
        json!({"op":"audio_state","request":{"id":"speech","bins":11,"samples":[0,100,12345,46935]}}),
        None,
    );
    assert_eq!(state, restored);
}

#[test]
fn invalid_import_replacement_and_referenced_audio_removal_are_atomic() {
    let scratch = Scratch::new();
    fs::write(scratch.0.join("source.wav"), SPEECH).unwrap();
    let mut editor = Editor::new(&scratch.0).unwrap();
    ok(&mut editor, import(), Some(0));
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"put_clip","clip":{"id":"talk","duration":3}},
        {"op":"put_shot","shot":{"id":"shot","clip":"talk","rate":{"numerator":24,"denominator":1},"start_frame":1001,"frame_count":69,"clip_frame_zero":1001,"audio":{"asset":"speech","start_sample":317}}}]}),
        Some(1),
    );
    let before = serde_json::to_vec(editor.document()).unwrap();
    let rev = editor.revision();
    fs::write(scratch.0.join("bad.wav"), &SPEECH[..31]).unwrap();
    let short = editor.document().audio[0].data.wave().encode_slice(0, 100, 4096).unwrap();
    fs::write(scratch.0.join("short.wav"), short).unwrap();
    let commands = [
        import(),
        json!({"op":"import_audio","request":{"id":"speech","path":"bad.wav","replace":true}}),
        json!({"op":"import_audio","request":{"id":"speech","path":"short.wav","replace":true}}),
        json!({"op":"import_audio","request":{"id":"escape","path":"../source.wav"}}),
        json!({"op":"apply","operations":[{"op":"remove_audio","id":"speech"}]}),
        json!({"op":"audio_state","request":{"id":"speech","samples":[46936]}}),
        json!({"op":"audio_state","request":{"id":"speech","range":{"start_sample":u64::MAX,"frame_count":2}}}),
        json!({"op":"export_audio","request":{"id":"speech","path":"invalid.wav","range":{"start_sample":46930,"frame_count":20}}}),
    ];
    for command in commands {
        assert!(!call(&mut editor, command, Some(rev)).ok);
        assert_eq!(editor.revision(), rev);
        assert_eq!(serde_json::to_vec(editor.document()).unwrap(), before);
    }
    assert!(!scratch.0.join("invalid.wav").exists());
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"delete_shot","id":"shot"},{"op":"remove_audio","id":"speech"}]}),
        Some(rev),
    );
    assert!(editor.document().audio.is_empty());
    assert!(editor.document().shots.is_empty());
    ok(&mut editor, json!({"op":"undo"}), Some(rev + 1));
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), before);
}

fn float_wav(values: &[f64]) -> Vec<u8> {
    let mut bytes = b"RIFF".to_vec();
    bytes.extend((36 + values.len() * 8).to_le_bytes()[..4].iter());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16u32.to_le_bytes());
    bytes.extend(3u16.to_le_bytes());
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(8000u32.to_le_bytes());
    bytes.extend(64000u32.to_le_bytes());
    bytes.extend(8u16.to_le_bytes());
    bytes.extend(64u16.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend(((values.len() * 8) as u32).to_le_bytes());
    for value in values {
        bytes.extend(value.to_le_bytes());
    }
    bytes
}
#[test]
fn waveform_rms_retains_extreme_finite_audio_and_native_deserialization_rejects_bad_bytes() {
    for magnitude in [1e-300, 1e300, f64::MAX] {
        let data = EmbeddedWave::from_bytes(float_wav(&[magnitude, -magnitude, 0.0, 0.0])).unwrap();
        let document = Document {
            audio: vec![AudioAsset { id: "float".into(), label: String::new(), data }],
            ..Document::default()
        };
        let request: AudioState = serde_json::from_value(json!({"id":"float","bins":1})).unwrap();
        let result = audio::inspect(&document, &request).unwrap();
        let channel = &result["waveform"][0]["channels"][0];
        let rms = channel["rms"].as_f64().unwrap();
        assert!(rms.is_finite());
        assert!((rms / magnitude - 0.5_f64.sqrt()).abs() < 1e-14);
        let mut saved = serde_json::to_value(&document).unwrap();
        saved["audio"][0]["data"] = json!("aGVsbG8=");
        assert!(serde_json::from_value::<Document>(saved).is_err());
    }
}

#[test]
fn embedded_audio_stays_compact_in_pretty_native_projects() {
    let mut bytes = SPEECH.to_vec();
    let padding = 2 * 1024 * 1024;
    bytes.extend(b"JUNK");
    bytes.extend((padding as u32).to_le_bytes());
    bytes.resize(bytes.len() + padding, 0);
    let riff_length = (bytes.len() - 8) as u32;
    bytes[4..8].copy_from_slice(&riff_length.to_le_bytes());
    let asset = AudioAsset {
        id: "compact".into(),
        label: String::new(),
        data: EmbeddedWave::from_bytes(bytes.clone()).unwrap(),
    };
    let serialized = serde_json::to_vec_pretty(&asset).unwrap();
    assert!(
        serialized.len() < bytes.len().div_ceil(3) * 4 + 200,
        "pretty project must not expand each source byte into an indented array item"
    );
    let loaded: AudioAsset = serde_json::from_slice(&serialized).unwrap();
    assert_eq!(loaded.data.encoded_bytes(), bytes);
}
