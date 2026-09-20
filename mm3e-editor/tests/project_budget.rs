//! Public persistent admission regressions; the separate process harness covers
//! large transient authoring. No private budget API or reduced test quota is used.
use mm3e_editor::{
    audio::{AudioAsset, EmbeddedWave, MAX_AUDIO_BYTES},
    model::{Document, Pass},
    protocol::{Request, Response},
    Editor,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

const LIMIT: u64 = 64 * 1024 * 1024;
const SPEECH: &[u8] = include_bytes!("fixtures/dialogue/hello-reference.wav");
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mm3e-project-budget-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Serialize)]
struct Saved<'a> {
    format: &'static str,
    saved_revision: u64,
    document: &'a Document,
}
fn saved(document: &Document, revision: u64) -> Saved<'_> {
    Saved { format: "mm3e-agent-project-v1", saved_revision: revision, document }
}
struct Count(u64);
impl Write for Count {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0 = self.0.checked_add(bytes.len() as u64).ok_or_else(|| io::Error::other("test count overflow"))?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn canonical_count(document: &Document, revision: u64) -> u64 {
    let mut count = Count(0);
    serde_json::to_writer_pretty(&mut count, &saved(document, revision)).unwrap();
    count.0
}
fn call(editor: &mut Editor, command: Value) -> Response {
    let request: Request =
        serde_json::from_value(json!({"id":"budget-test","expected_revision":editor.revision(),"command":command}))
            .unwrap();
    editor.handle(request)
}
fn ok(editor: &mut Editor, command: Value) -> Value {
    let response = call(editor, command);
    assert!(response.ok, "{response:?}");
    response.result.unwrap()
}
fn history(editor: &mut Editor) -> (u64, u64) {
    let state = ok(editor, json!({"op":"inspect"}));
    (state["undo_available"].as_u64().unwrap(), state["redo_available"].as_u64().unwrap())
}
fn budget(editor: &mut Editor) -> Value {
    let value = ok(editor, json!({"op":"project_budget"}));
    assert_eq!(value["limit_bytes"], LIMIT);
    let encoded = value["encoded_bytes"].as_u64().unwrap();
    let reserve = value["revision_reserve_bytes"].as_u64().unwrap();
    let remaining = value["remaining_bytes"].as_u64().unwrap();
    assert_eq!(encoded + reserve + remaining, LIMIT);
    value
}

fn oversized_canonical_document() -> Document {
    // Both components separately satisfy their native authoring caps. Pretty-print
    // indentation on 2M real volume samples plus one original, metadata-padded WAV
    // exceeds 64MiB, although the compact serialized source file is below 64MiB.
    // This avoids an expensive triangle BVH solely to exercise serialization admission.
    let mut wav = SPEECH.to_vec();
    let padding = MAX_AUDIO_BYTES - wav.len() - 8;
    assert_eq!(padding % 2, 0);
    wav.extend(b"JUNK");
    wav.extend((padding as u32).to_le_bytes());
    wav.resize(MAX_AUDIO_BYTES, 0);
    let riff_size = (wav.len() - 8) as u32;
    wav[4..8].copy_from_slice(&riff_size.to_le_bytes());
    let data = EmbeddedWave::from_bytes(wav).unwrap();
    let mut entity: mm3e_editor::model::Entity = serde_json::from_value(json!({"id":"volume",
        "shape":{"type":"sphere","radius":1}}))
    .unwrap();
    entity.shape = mm3e_editor::model::Shape::Volume {
        dims: [128, 128, 128],
        min: [0.0; 3],
        cell: [0.01; 3],
        samples: vec![-f32::MIN_POSITIVE; 128 * 128 * 128],
    };
    let document = Document {
        objects: vec![entity],
        audio: vec![AudioAsset { id: "speech".into(), label: String::new(), data }],
        ..Document::default()
    };
    document.compile(&Pass::Beauty).unwrap();
    assert!(canonical_count(&document, 0) > LIMIT);
    document
}

#[test]
fn persistent_load_and_startup_reject_canonical_inflation_preserving_data_history_and_queries() {
    let scratch = Scratch::new();
    let oversized = oversized_canonical_document();
    let compact = serde_json::to_vec(&saved(&oversized, 0)).unwrap();
    assert!((compact.len() as u64) < LIMIT, "fixture must pass the raw-file cap");
    fs::write(scratch.0.join("oversized.json"), &compact).unwrap();
    drop(oversized);
    let mut editor = Editor::open_project(&scratch.0, "active.json").unwrap();
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"create","object":{"id":"marker","shape":{"type":"sphere","radius":0.2}}}]}),
    );
    ok(&mut editor, json!({"op":"apply","operations":[{"op":"update","id":"marker","patch":{"position":[0.25,0,0]}}]}));
    ok(&mut editor, json!({"op":"undo"}));
    let before = serde_json::to_vec(editor.document()).unwrap();
    let before_value = serde_json::to_value(editor.document()).unwrap();
    let revision = editor.revision();
    let history_before = history(&mut editor);
    assert_eq!(history_before, (1, 1));
    let budget_before = budget(&mut editor);
    let durable = fs::read(scratch.0.join("active.json")).unwrap();
    let response = call(&mut editor, json!({"op":"load","path":"oversized.json"}));
    assert!(!response.ok);
    assert_eq!(response.error.unwrap().code, "project_size_limit");
    assert_eq!(editor.revision(), revision);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), before);
    assert_eq!(history(&mut editor), history_before);
    assert_eq!(budget(&mut editor), budget_before);
    assert_eq!(fs::read(scratch.0.join("active.json")).unwrap(), durable);
    assert_eq!(ok(&mut editor, json!({"op":"get_document"})), before_value);
    ok(&mut editor, json!({"op":"validate"}));
    let sample = ok(&mut editor, json!({"op":"sample","id":"marker","points":[[0,0,0]]}));
    assert!(sample["samples"][0]["value"].as_f64().unwrap() < 0.0);
    ok(&mut editor, json!({"op":"save","path":"after-rejection.json"}));
    assert_eq!(fs::read(scratch.0.join("after-rejection.json")).unwrap(), durable);
    // A failed load must retain the redo branch and leave durable writes usable.
    ok(&mut editor, json!({"op":"redo"}));
    assert_eq!(editor.document().objects[0].position, [0.25, 0.0, 0.0]);
    let final_revision = editor.revision();
    drop(editor);
    let reopened = Editor::open_project(&scratch.0, "active.json").unwrap();
    assert_eq!(reopened.revision(), final_revision);
    assert_eq!(reopened.document().objects[0].position, [0.25, 0.0, 0.0]);
    drop(reopened);
    // Repeated rejected startup must release its OS lock and leave the source file intact.
    for _ in 0..2 {
        match Editor::open_project(&scratch.0, "oversized.json") {
            Err(error) => assert_eq!(error.code, "project_size_limit"),
            Ok(_) => panic!("oversized canonical project was installed at startup"),
        }
        assert_eq!(fs::read(scratch.0.join("oversized.json")).unwrap(), compact);
    }
}

#[test]
fn last_u64_revision_commits_once_then_rejection_preserves_history_disk_and_readonly_budget() {
    let scratch = Scratch::new();
    let document = Document::default();
    fs::write(scratch.0.join("near-max.json"), serde_json::to_vec(&saved(&document, u64::MAX - 1)).unwrap()).unwrap();
    let mut editor = Editor::open_project(&scratch.0, "near-max.json").unwrap();
    assert_eq!(editor.revision(), u64::MAX - 1);
    assert_eq!(budget(&mut editor)["revision_reserve_bytes"], 0);
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"create","object":{"id":"marker","shape":{"type":"sphere","radius":0.2}}}]}),
    );
    assert_eq!(editor.revision(), u64::MAX);
    let durable = fs::read(scratch.0.join("near-max.json")).unwrap();
    let state = budget(&mut editor);
    assert_eq!(state["encoded_bytes"], durable.len());
    let before = serde_json::to_vec(editor.document()).unwrap();
    let history_before = history(&mut editor);
    let rejected = call(&mut editor, json!({"op":"undo"}));
    assert!(!rejected.ok);
    assert!(rejected.error.unwrap().message.contains("revision space exhausted"));
    assert_eq!(editor.revision(), u64::MAX);
    assert_eq!(history(&mut editor), history_before);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), before);
    assert_eq!(fs::read(scratch.0.join("near-max.json")).unwrap(), durable);
    assert_eq!(budget(&mut editor), state);
    ok(&mut editor, json!({"op":"sample","id":"marker","points":[[0,0,0]]}));
    drop(editor);
    let mut reopened = Editor::open_project(&scratch.0, "near-max.json").unwrap();
    assert_eq!(reopened.revision(), u64::MAX);
    budget(&mut reopened);
    assert_eq!(serde_json::to_vec(reopened.document()).unwrap(), before);
}
