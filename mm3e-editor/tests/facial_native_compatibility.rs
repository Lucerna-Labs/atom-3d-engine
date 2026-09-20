//! Authentic September 10 native data must migrate only after exact old-generator verification.
use mm3e_editor::{
    cloth, face,
    model::{Document, Pass},
    protocol::{Request, Response},
    Editor,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

// Emitted by the immutable r9 release binary, SHA256
// 22d7498ccdc8fa91891600b2916a6eb5c249a47e163c0ea516c94339b6677ff3.
// Full production transcript and fixture hash: artifacts/face-legacy-migration-20260919-r2.
const LEGACY: &[u8] = include_bytes!("fixtures/face_controls_legacy_20260910.json");
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mm3e-face-native-{}-{}",
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
fn call(editor: &mut Editor, command: Value) -> Response {
    let request: Request =
        serde_json::from_value(json!({"id":"compatibility","expected_revision":editor.revision(),"command":command}))
            .unwrap();
    editor.handle(request)
}
fn ok(editor: &mut Editor, command: Value) -> Value {
    let response = call(editor, command);
    assert!(response.ok, "{response:?}");
    response.result.unwrap()
}
fn legacy() -> Document {
    serde_json::from_value(serde_json::from_slice::<Value>(LEGACY).unwrap()["document"].clone()).unwrap()
}
fn assert_migrated(document: &Document) {
    let original = legacy();
    let mut expected = original.clone();
    face::synchronize(&mut expected).unwrap();
    assert_eq!(
        serde_json::to_value(document).unwrap(),
        serde_json::to_value(expected).unwrap(),
        "migration changed authored data or retained cache frames"
    );
    let state = face::inspect_controls(document, None).unwrap();
    assert_eq!(state["faces"][0]["mouth"]["cavity_open"], false);
    assert_eq!(state["faces"][0]["controls"]["jaw_open"], 1.0);
    assert!(!cloth::cache_fresh(document, &document.cloths[0]).unwrap());
    document.compile(&Pass::Beauty).unwrap();
}

#[test]
fn authentic_legacy_startup_migrates_in_memory_preserving_source_and_saved_revision() {
    let scratch = Scratch::new();
    assert_eq!(
        format!("{:x}", Sha256::digest(LEGACY)),
        "aa2666b5f6b1c5d937ca2fb70f41297a20e2bda7b4c5d56873be0603c5a45c43"
    );
    fs::write(scratch.0.join("legacy.json"), LEGACY).unwrap();
    assert!(legacy().compile(&Pass::Beauty).is_err(), "fixture no longer reproduces the old derived geometry boundary");
    let mut editor = Editor::open_project(&scratch.0, "legacy.json").unwrap();
    assert_eq!(editor.revision(), 3);
    assert_migrated(editor.document());
    assert_eq!(
        fs::read(scratch.0.join("legacy.json")).unwrap(),
        LEGACY,
        "opening must not rewrite durable source data"
    );
    ok(&mut editor, json!({"op":"save","path":"upgraded.json"}));
    let expected = serde_json::to_value(editor.document()).unwrap();
    drop(editor);
    let current = Editor::open_project(&scratch.0, "upgraded.json").unwrap();
    assert_eq!(serde_json::to_value(current.document()).unwrap(), expected);
    assert_migrated(current.document());
    let mut loaded = Editor::new(&scratch.0).unwrap();
    let response = ok(&mut loaded, json!({"op":"load","path":"upgraded.json"}));
    assert_eq!(response["migrated_faces"], json!([]));
    assert_eq!(serde_json::to_value(loaded.document()).unwrap(), expected);
}

#[test]
fn command_load_migration_is_undoable_and_preserves_authored_fields_and_cache_frames() {
    let scratch = Scratch::new();
    fs::write(scratch.0.join("legacy.json"), LEGACY).unwrap();
    let mut editor = Editor::open_project(&scratch.0, "active.json").unwrap();
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"create","object":{"id":"sentinel","shape":{"type":"sphere","radius":0.1}}}]}),
    );
    let before = serde_json::to_value(editor.document()).unwrap();
    let result = ok(&mut editor, json!({"op":"load","path":"legacy.json"}));
    assert_eq!(result["source_saved_revision"], 3);
    assert_eq!(result["migrated_faces"], json!(["face"]));
    assert_eq!(editor.revision(), 2);
    assert_migrated(editor.document());
    let migrated = serde_json::to_value(editor.document()).unwrap();
    ok(&mut editor, json!({"op":"undo"}));
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), before);
    ok(&mut editor, json!({"op":"redo"}));
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), migrated);
    assert_eq!(fs::read(scratch.0.join("legacy.json")).unwrap(), LEGACY);
    drop(editor);
    let reopened = Editor::open_project(&scratch.0, "active.json").unwrap();
    assert_eq!(serde_json::to_value(reopened.document()).unwrap(), migrated);
}

#[test]
fn edited_or_mixed_legacy_geometry_rejects_without_changing_files_state_or_history() {
    let scratch = Scratch::new();
    let mut editor = Editor::open_project(&scratch.0, "active.json").unwrap();
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"create","object":{"id":"sentinel","shape":{"type":"sphere","radius":0.1}}}]}),
    );
    ok(&mut editor, json!({"op":"apply","operations":[{"op":"update","id":"sentinel","patch":{"position":[1,0,0]}}]}));
    ok(&mut editor, json!({"op":"undo"}));
    let snapshot = serde_json::to_value(editor.document()).unwrap();
    let revision = editor.revision();
    let history = ok(&mut editor, json!({"op":"inspect"}));
    let durable = fs::read(scratch.0.join("active.json")).unwrap();
    let saved: Value = serde_json::from_slice(LEGACY).unwrap();
    let mut corrected = legacy();
    face::synchronize(&mut corrected).unwrap();
    let mut variants = Vec::new();
    for (field, value) in [
        ("label", json!("edited generated lid")),
        ("position", json!([2.28, 0.2, 0.7801])),
        ("combine", json!({"type":"subtract"})),
        ("shape", json!({"type":"sphere","radius":0.01})),
    ] {
        let mut bad = saved.clone();
        bad["document"]["objects"][3][field] = value;
        variants.push((field, bad));
    }
    let mut mixed = saved.clone();
    mixed["document"]["objects"][3] = serde_json::to_value(&corrected.objects[3]).unwrap();
    variants.push(("mixed legacy/current parts", mixed));
    let mut source_edit = saved.clone();
    source_edit["document"]["objects"][0]["shape"]["radii"][0] = json!(0.9);
    variants.push(("edited authored source", source_edit));
    for (name, bad) in variants {
        let bytes = serde_json::to_vec(&bad).unwrap();
        fs::write(scratch.0.join("bad.json"), &bytes).unwrap();
        let response = call(&mut editor, json!({"op":"load","path":"bad.json"}));
        assert!(!response.ok, "{name} was overwritten by migration");
        assert_eq!(editor.revision(), revision);
        assert_eq!(serde_json::to_value(editor.document()).unwrap(), snapshot);
        assert_eq!(ok(&mut editor, json!({"op":"inspect"})), history);
        assert_eq!(fs::read(scratch.0.join("active.json")).unwrap(), durable);
        assert!(Editor::open_project(&scratch.0, "bad.json").is_err(), "startup accepted {name}");
        assert_eq!(fs::read(scratch.0.join("bad.json")).unwrap(), bytes);
    }
    ok(&mut editor, json!({"op":"redo"}));
    assert_eq!(editor.document().objects[0].position, [1.0, 0.0, 0.0]);
}
