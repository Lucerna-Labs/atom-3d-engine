use mm3e_editor::{
    model::Pass,
    protocol::{Request, Response},
    Editor,
};
use serde_json::{json, Value};
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);
struct Root(PathBuf);
impl Root {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mm3e-editor-test-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn editor(&self) -> Editor {
        Editor::new(&self.0).unwrap()
    }
}
impl Drop for Root {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn send(editor: &mut Editor, command: Value) -> Response {
    let request: Request =
        serde_json::from_value(json!({"id": "test", "expected_revision": editor.revision(), "command": command}))
            .unwrap();
    editor.handle(request)
}
fn okay(editor: &mut Editor, command: Value) -> Value {
    let response = send(editor, command);
    assert!(response.ok, "{:?}", response.error);
    response.result.unwrap()
}
fn sphere(id: &str) -> Value {
    json!({"op": "create", "object": {"id": id, "shape": {"type": "sphere", "radius": 0.5}}})
}
fn apply(editor: &mut Editor, operations: Vec<Value>) -> Value {
    okay(editor, json!({"op": "apply", "operations": operations}))
}
fn snapshot(editor: &mut Editor) -> Value {
    okay(editor, json!({"op": "get_document"}))
}

#[test]
fn invalid_batch_rolls_back_every_change_and_dry_run_is_nonmutating() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(&mut editor, vec![sphere("body")]);
    let original = snapshot(&mut editor);
    let failed = send(
        &mut editor,
        json!({"op": "apply", "operations": [
            {"op": "translate", "ids": ["body"], "delta": [1, 0, 0]},
            {"op": "update", "id": "body", "patch": {"scale": -1}}
        ]}),
    );
    assert!(!failed.ok);
    assert_eq!(editor.revision(), 1);
    assert_eq!(snapshot(&mut editor), original);
    let preview = okay(&mut editor, json!({"op": "apply", "dry_run": true, "operations": [sphere("head")]}));
    assert_eq!(preview["committed"], false);
    assert_eq!(preview["object_count"], 2);
    assert_eq!(editor.revision(), 1);
    assert_eq!(snapshot(&mut editor), original);
}

#[test]
fn revisions_prevent_duplicate_mutations_and_undo_redo_do_not_reuse_revisions() {
    let root = Root::new();
    let mut editor = root.editor();
    let request: Request = serde_json::from_value(json!({"id": "retry", "expected_revision": 0,
        "command": {"op": "apply", "operations": [sphere("body")]}}))
    .unwrap();
    assert!(editor.handle(request.clone()).ok);
    let replay = editor.handle(request);
    assert_eq!(replay.error.unwrap().code, "revision_conflict");
    let body = snapshot(&mut editor);
    okay(&mut editor, json!({"op": "undo"}));
    assert_eq!(editor.revision(), 2);
    assert!(editor.document().objects.is_empty());
    okay(&mut editor, json!({"op": "redo"}));
    assert_eq!(editor.revision(), 3);
    assert_eq!(snapshot(&mut editor), body);
    okay(&mut editor, json!({"op": "undo"}));
    apply(&mut editor, vec![sphere("new_branch")]);
    assert!(!send(&mut editor, json!({"op": "redo"})).ok);
}

#[test]
fn invalid_geometry_camera_and_unknown_fields_are_rejected() {
    let root = Root::new();
    let mut editor = root.editor();
    for object in [
        json!({"id":"bad", "shape":{"type":"sphere","radius":-1}}),
        json!({"id":"bad", "shape":{"type":"plane","normal":[0,2,0],"offset":0}}),
        json!({"id":"bad", "shape":{"type":"capsule","a":[0,0,0],"b":[0,0,0],"radius":1}}),
        json!({"id":"bad", "shape":{"type":"round_box","half_extents":[1,1,1],"radius":2}}),
        json!({"id":"bad", "shape":{"type":"sphere","radius":1},"material":{"roughness":-0.1}}),
        json!({"id":"bad", "shape":{"type":"volume","dims":[2,2,2],"min":[0,0,0],"cell":[1,1,1],
            "samples":[3e38,-3e38,3e38,-3e38,3e38,-3e38,3e38,-3e38]}}),
    ] {
        assert!(!send(&mut editor, json!({"op":"apply","operations":[{"op":"create","object":object}]})).ok);
        assert_eq!(editor.revision(), 0);
    }
    assert!(
        !send(
            &mut editor,
            json!({"op":"apply","operations":[{"op":"set_camera","camera":{"eye":[0,0,0],"target":[0,0,0]}}]})
        )
        .ok
    );
    assert!(serde_json::from_value::<Request>(json!({"id":"x","command":{"op":"inspect","unexpected":true}})).is_err());
    let no_revision: Request = serde_json::from_value(json!({"id":"x","command":{"op":"undo"}})).unwrap();
    assert_eq!(editor.handle(no_revision).error.unwrap().code, "revision_required");
}

#[test]
fn native_save_reload_preserves_geometry_camera_settings_and_baked_samples() {
    let root = Root::new();
    let mut editor = root.editor();
    let samples: Vec<f32> = (0..27).map(|i| (i as f32 - 13.0) * 0.017).collect();
    apply(
        &mut editor,
        vec![
            json!({"op":"create","object":{"id":"volume","shape":{"type":"volume",
        "dims":[3,3,3],"min":[-1,-1,-1],"cell":[1,1,1],"samples":samples}}}),
            json!({"op":"set_settings","settings":{"width":48,"height":32,"quality":"full","exposure":1.3}}),
            json!({"op":"set_camera","camera":{"eye":[4,2,6],"target":[0,0.3,0],"fov_degrees":41}}),
        ],
    );
    let before = snapshot(&mut editor);
    let observation = okay(&mut editor, json!({"op":"sample","points":[[0.1,0.2,0.3]]}));
    okay(&mut editor, json!({"op":"save","path":"project.json"}));
    let saved_bytes = fs::read(root.0.join("project.json")).unwrap();
    let mut reopened = root.editor();
    okay(&mut reopened, json!({"op":"load","path":"project.json"}));
    assert_eq!(snapshot(&mut reopened), before);
    assert_eq!(okay(&mut reopened, json!({"op":"sample","points":[[0.1,0.2,0.3]]})), observation);
    assert_eq!(reopened.document().compile(&Pass::Beauty).unwrap().0.volumes.len(), 1);
    assert!(!send(&mut reopened, json!({"op":"save","path":"project.json"})).ok);
    assert_eq!(fs::read(root.0.join("project.json")).unwrap(), saved_bytes);
}

#[test]
fn humanoid_has_editable_roles_and_duplicate_recipe_does_not_partially_append() {
    let root = Root::new();
    let mut editor = root.editor();
    let recipe = json!({"op":"create_humanoid","id":"hero","height":1.8,"build":1.0,"head_scale":1.1});
    apply(&mut editor, vec![recipe.clone()]);
    assert_eq!(editor.document().objects.len(), 19);
    assert!(editor.document().objects.iter().any(|e| e.id == "hero/head" && e.role == "head"));
    let head = okay(&mut editor, json!({"op":"inspect","id":"hero/head"}));
    apply(&mut editor, vec![json!({"op":"update","id":"hero/head","patch":{"scale":1.2}})]);
    assert_ne!(okay(&mut editor, json!({"op":"inspect","id":"hero/head"})), head);
    let revision = editor.revision();
    assert!(!send(&mut editor, json!({"op":"apply","operations":[recipe]})).ok);
    assert_eq!(editor.revision(), revision);
    assert_eq!(editor.document().objects.len(), 19);
}

#[test]
fn paths_cannot_escape_root_and_invalid_load_preserves_the_document() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(&mut editor, vec![sphere("body")]);
    for path in ["../escape.json", "/tmp/escape.json"] {
        assert!(!send(&mut editor, json!({"op":"save","path":path})).ok);
    }
    let before = snapshot(&mut editor);
    fs::write(root.0.join("bad.json"), b"{\"format\":\"bad\"}").unwrap();
    assert!(!send(&mut editor, json!({"op":"load","path":"bad.json"})).ok);
    assert_eq!(snapshot(&mut editor), before);
    assert_eq!(editor.revision(), 1);
}

#[test]
fn part_scoped_observation_distinguishes_overlapping_geometry() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("chest"),
            json!({"op":"create","object":{"id":"arm",
        "shape":{"type":"sphere","radius":1.0},"position":[0.5,0,0]}}),
        ],
    );
    let combined = okay(&mut editor, json!({"op":"sample","points":[[0.75,0,0]]}));
    let part = okay(&mut editor, json!({"op":"sample","id":"chest","points":[[0.75,0,0]]}));
    assert_eq!(combined["samples"][0]["material_owner_id"], "arm");
    assert_eq!(part["samples"][0]["material_owner_id"], "chest");
    assert!((part["samples"][0]["value"].as_f64().unwrap() - 0.25).abs() < 1e-6);
    assert_eq!(part["scope"], "object_before_scene_csg");
    assert!(!send(&mut editor, json!({"op":"sample","id":"missing","points":[[0,0,0]]})).ok);
}

#[test]
fn obj_import_uses_real_baking_and_survives_native_save_load() {
    let root = Root::new();
    fs::write(root.0.join("cube.obj"), "v -1 -1 -1\nv 1 -1 -1\nv 1 1 -1\nv -1 1 -1\nv -1 -1 1\nv 1 -1 1\nv 1 1 1\nv -1 1 1\nf 1 4 3 2\nf 5 6 7 8\nf 1 2 6 5\nf 4 8 7 3\nf 1 5 8 4\nf 2 3 7 6\n").unwrap();
    let mut editor = root.editor();
    let result =
        okay(&mut editor, json!({"op":"import_obj","id":"cube","path":"cube.obj","resolution":12,"padding":0.2}));
    assert_eq!(result["triangles"], 12);
    let probe = json!({"op":"sample","id":"cube","points":[[0,0,0],[2,0,0]]});
    let samples = okay(&mut editor, probe.clone());
    assert!(samples["samples"][0]["value"].as_f64().unwrap() < 0.0);
    assert!(samples["samples"][1]["value"].as_f64().unwrap() > 0.0);
    let compact = okay(&mut editor, json!({"op":"inspect","id":"cube"}));
    assert_eq!(compact["shape"]["sample_count"], 12 * 12 * 12);
    assert!(compact["shape"].get("samples").is_none());
    okay(&mut editor, json!({"op":"save","path":"cube.json"}));
    let mut fresh = root.editor();
    okay(&mut fresh, json!({"op":"load","path":"cube.json"}));
    assert_eq!(okay(&mut fresh, probe), samples);
}

#[test]
fn jsonl_framing_recovers_after_a_malformed_and_oversized_request() {
    let root = Root::new();
    let mut child = Command::new(env!("CARGO_BIN_EXE_mm3e-editor"))
        .arg("--root")
        .arg(&root.0)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    writeln!(input, "{{bad json").unwrap();
    input.write_all(&vec![b'x'; 4 * 1024 * 1024 + 10]).unwrap();
    writeln!(input).unwrap();
    writeln!(input, "{}", json!({"id":"recovered","command":{"op":"validate"}})).unwrap();
    drop(input);
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let responses: Vec<Value> =
        String::from_utf8(output.stdout).unwrap().lines().map(|line| serde_json::from_str(line).unwrap()).collect();
    assert_eq!(responses.len(), 3);
    assert_eq!(responses[0]["ok"], false);
    assert_eq!(responses[1]["ok"], false);
    assert_eq!(responses[2]["id"], "recovered");
    assert_eq!(responses[2]["ok"], true);
    assert_eq!(responses[2]["revision"], 0);
}

fn process(root: &Root, requests: &[Value]) -> Vec<Value> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mm3e-editor"))
        .arg("--root")
        .arg(&root.0)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    for request in requests {
        writeln!(input, "{request}").unwrap();
    }
    drop(input);
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap().lines().map(|line| serde_json::from_str(line).unwrap()).collect()
}

#[test]
fn real_jsonl_binary_renders_changed_pixels_and_reopens_the_saved_scene_identically() {
    let root = Root::new();
    let requests = vec![
        json!({"id":"create","expected_revision":0,"command":{"op":"apply","operations":[sphere("body"),
            {"op":"set_camera","camera":{"eye":[0,0,3],"target":[0,0,0],"fov_degrees":45}},
            {"op":"set_settings","settings":{"width":48,"height":48,"quality":"balanced"}}]}}),
        json!({"id":"before","command":{"op":"render","path":"before.png"}}),
        json!({"id":"edit","expected_revision":1,"command":{"op":"apply","operations":[{"op":"update","id":"body","patch":{"scale":1.5}}]}}),
        json!({"id":"after","command":{"op":"render","path":"after.png"}}),
        json!({"id":"pick","command":{"op":"pick","x":24,"y":24}}),
        json!({"id":"save","command":{"op":"save","path":"sphere.json"}}),
    ];
    let responses = process(&root, &requests);
    assert_eq!(responses.len(), requests.len());
    for response in &responses {
        assert_eq!(response["ok"], true, "{response}");
    }
    assert_ne!(responses[1]["result"]["rgba_fnv1a64"], responses[3]["result"]["rgba_fnv1a64"]);
    assert!(
        responses[3]["result"]["metrics"]["coverage"].as_f64().unwrap()
            > responses[1]["result"]["metrics"]["coverage"].as_f64().unwrap()
    );
    assert_eq!(responses[4]["result"]["material_owner_id"], "body");
    let reopened = process(
        &root,
        &[
            json!({"id":"load","expected_revision":0,"command":{"op":"load","path":"sphere.json"}}),
            json!({"id":"render","command":{"op":"render","path":"reopened.png"}}),
        ],
    );
    assert_eq!(reopened[1]["ok"], true, "{reopened:?}");
    assert_eq!(responses[3]["result"]["rgba_fnv1a64"], reopened[1]["result"]["rgba_fnv1a64"]);
    assert_eq!(fs::read(root.0.join("after.png")).unwrap(), fs::read(root.0.join("reopened.png")).unwrap());
    let decoder = png::Decoder::new(std::io::Cursor::new(fs::read(root.0.join("after.png")).unwrap()));
    let mut reader = decoder.read_info().unwrap();
    let mut pixels = vec![0; reader.output_buffer_size().unwrap()];
    let frame = reader.next_frame(&mut pixels).unwrap();
    assert_eq!((frame.width, frame.height), (48, 48));
    assert!(pixels.as_chunks::<4>().0.iter().all(|pixel| pixel[3] == 255));
}
