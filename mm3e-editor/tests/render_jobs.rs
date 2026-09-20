//! Exercise persistent jobs through the same strict commands used by JSONL agents.
use exr::prelude::*;
use mm3e_editor::{audio::EmbeddedWave, protocol::Request, protocol::Response, Editor};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Cursor, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
};

const SPEECH: &[u8] = include_bytes!("fixtures/dialogue/hello-reference.wav");
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mm3e-render-job-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn editor(&self) -> Editor {
        Editor::new(&self.0).unwrap()
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn call(editor: &mut Editor, command: Value, expected_revision: Option<u64>) -> Response {
    editor.handle(
        serde_json::from_value::<Request>(
            json!({"id":"render-job-test","expected_revision":expected_revision,"command":command}),
        )
        .unwrap(),
    )
}
fn ok(editor: &mut Editor, command: Value) -> Value {
    let response = call(editor, command, None);
    assert!(response.ok, "{response:?}");
    response.result.unwrap()
}
fn apply(editor: &mut Editor, operations: Value) {
    let revision = editor.revision();
    let response = call(editor, json!({"op":"apply","operations":operations}), Some(revision));
    assert!(response.ok, "{response:?}");
}
fn scene(editor: &mut Editor) {
    apply(
        editor,
        json!([
            {"op":"set_settings","settings":{"width":16,"height":16,"quality":"preview","shadows":false,"ao":false}},
            {"op":"set_camera","camera":{"eye":[0,0,3],"target":[0,0,0],"fov_degrees":40}},
            {"op":"create","object":{"id":"orb","shape":{"type":"sphere","radius":0.42},
                "material":{"albedo":[0.8,0.1,0.05],"emissive":[0.7,0.02,0.01]}}},
            {"op":"put_clip","clip":{"id":"motion","duration":1,"tracks":[
                {"target":{"type":"object","id":"orb"},"keys":[
                    {"time":0,"translation":[-0.45,0,0]},
                    {"time":1,"translation":[0.45,0,0]}
                ]}
            ]}}
        ]),
    );
}
fn sequence(directory: &str, end: f32) -> Value {
    json!({"directory":directory,"clip":"motion","start":0,"end":end,"fps":2})
}
fn create_sequence(editor: &mut Editor, directory: &str, end: f32) -> Value {
    let mut request = sequence(directory, end);
    request["type"] = json!("sequence");
    ok(editor, json!({"op":"create_render_job","request":request}))
}
fn json_file(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn png_pixels(path: impl AsRef<Path>) -> Vec<u8> {
    let mut reader = png::Decoder::new(Cursor::new(fs::read(path).unwrap())).read_info().unwrap();
    assert_eq!((reader.info().width, reader.info().height), (16, 16));
    let mut pixels = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut pixels).unwrap();
    pixels.truncate(info.buffer_size());
    pixels
}
fn exr_channels(path: impl AsRef<Path>) -> Vec<(String, Vec<u32>)> {
    let decoded = read()
        .no_deep_data()
        .largest_resolution_level()
        .all_channels()
        .all_layers()
        .all_attributes()
        .non_parallel()
        .from_buffered(Cursor::new(fs::read(path).unwrap()))
        .unwrap();
    assert_eq!(decoded.layer_data.len(), 1);
    let layer = &decoded.layer_data[0];
    assert_eq!(layer.size, exr::math::Vec2(16, 16));
    layer
        .channel_data
        .list
        .iter()
        .map(|channel| (channel.name.to_string(), channel.sample_data.values_as_f32().map(f32::to_bits).collect()))
        .collect()
}

#[test]
fn bounded_steps_survive_reopen_and_use_frozen_scene_with_identical_rendered_pixels() {
    let scratch = Scratch::new();
    let mut editor = scratch.editor();
    scene(&mut editor);
    ok(&mut editor, json!({"op":"render_sequence","request":sequence("reference",1.0)}));
    let initial_document = serde_json::to_vec(editor.document()).unwrap();
    let created = create_sequence(&mut editor, "job", 1.0);
    assert_eq!(created["status"], "pending");
    assert_eq!(created["completed_frames"], 0);
    assert_eq!(created["total_frames"], 3);
    assert_eq!(created["source_revision"], 1);
    assert!(created["pending_frame"].is_null());
    assert!(created["manifest_path"].is_null());
    assert_eq!(editor.revision(), 1);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), initial_document);
    for field in ["snapshot_sha256", "binary_sha256", "plan_sha256"] {
        assert_eq!(created[field].as_str().unwrap().len(), 64);
    }
    assert!(!scratch.0.join("job/frame_0000.png").exists());

    // Omitted max_frames must perform exactly one frame, not render the whole job.
    let one = ok(&mut editor, json!({"op":"step_render_job","directory":"job"}));
    assert_eq!(one["completed_frames"], 1);
    assert_eq!(one["status"], "pending");
    assert!(!scratch.0.join("job/manifest.json").exists());
    assert!(!scratch.0.join("job/frame_0001.png").exists());
    let first_bytes = fs::read(scratch.0.join("job/frame_0000.png")).unwrap();
    apply(
        &mut editor,
        json!([
            {"op":"update","id":"orb","patch":{"material":{"albedo":[0,0,0],"emissive":[0,8,0]}}},
            {"op":"set_camera","camera":{"eye":[0,0,5],"target":[0,0,0],"fov_degrees":65}}
        ]),
    );
    ok(&mut editor, json!({"op":"render_sequence","request":sequence("edited",1.0)}));
    let cancelled = ok(&mut editor, json!({"op":"cancel_render_job","directory":"job"}));
    assert_eq!(cancelled["status"], "cancelled");
    assert_eq!(cancelled["completed_frames"], 1);
    assert_eq!(editor.revision(), 2);
    drop(editor);

    // A newly opened editor has no authoring scene or clip; the job still owns both.
    let mut reopened = scratch.editor();
    assert!(reopened.document().objects.is_empty());
    let state = ok(&mut reopened, json!({"op":"render_job_state","directory":"job"}));
    assert_eq!(state["outputs_verified"], false);
    assert_eq!(state["status"], "cancelled");
    let paused = ok(&mut reopened, json!({"op":"step_render_job","directory":"job","max_frames":32}));
    assert_eq!(paused["completed_frames"], 1);
    assert!(!scratch.0.join("job/frame_0001.png").exists());
    let resumed = ok(&mut reopened, json!({"op":"resume_render_job","directory":"job"}));
    assert_eq!(resumed["status"], "pending");
    let complete = ok(&mut reopened, json!({"op":"step_render_job","directory":"job","max_frames":2}));
    assert_eq!(complete["status"], "complete");
    assert_eq!(complete["completed_frames"], 3);
    assert_eq!(complete["source_revision"], 1);
    assert_eq!(reopened.revision(), 0);
    assert!(reopened.document().objects.is_empty());
    assert_eq!(fs::read(scratch.0.join("job/frame_0000.png")).unwrap(), first_bytes);

    let reference = json_file(scratch.0.join("reference/manifest.json"));
    let manifest = json_file(scratch.0.join("job/manifest.json"));
    for index in 0..3 {
        let name = format!("frame_{index:04}.png");
        let actual = png_pixels(scratch.0.join("job").join(&name));
        assert_eq!(actual, png_pixels(scratch.0.join("reference").join(&name)));
        assert_ne!(actual, png_pixels(scratch.0.join("edited").join(&name)));
        let frame = &manifest["frames"][index];
        for field in ["rgba_fnv1a64", "time", "scheduled_time", "view"] {
            assert_eq!(frame[field], reference["frames"][index][field]);
        }
        let bytes = fs::read(scratch.0.join("job").join(name)).unwrap();
        assert_eq!(frame["encoded_file_sha256"], format!("{:x}", Sha256::digest(&bytes)));
        assert_eq!(frame["encoded_file_bytes"], bytes.len());
    }
    assert_ne!(png_pixels(scratch.0.join("job/frame_0000.png")), png_pixels(scratch.0.join("job/frame_0002.png")));
    let manifest_bytes = fs::read(scratch.0.join("job/manifest.json")).unwrap();
    let repeated = ok(&mut reopened, json!({"op":"step_render_job","directory":"job"}));
    assert_eq!(repeated["status"], "complete");
    assert_eq!(fs::read(scratch.0.join("job/manifest.json")).unwrap(), manifest_bytes);
    let verified = ok(&mut reopened, json!({"op":"render_job_state","directory":"job","verify_outputs":true}));
    assert_eq!(verified["outputs_verified"], true);
}

#[test]
fn shot_job_retains_exact_partial_audio_phase_and_linear_frames_without_source_files() {
    let scratch = Scratch::new();
    let mut editor = scratch.editor();
    scene(&mut editor);
    fs::write(scratch.0.join("speech.wav"), SPEECH).unwrap();
    let revision = editor.revision();
    let imported =
        call(&mut editor, json!({"op":"import_audio","request":{"id":"speech","path":"speech.wav"}}), Some(revision));
    assert!(imported.ok, "{imported:?}");
    apply(
        &mut editor,
        json!([{"op":"put_shot","shot":{
            "id":"dialogue","clip":"motion","rate":{"numerator":30000,"denominator":1001},
            "start_frame":1001,"frame_count":10,"clip_frame_zero":1001,
            "audio":{"asset":"speech","start_sample":317}
        }}]),
    );
    let render = json!({"shot":"dialogue","directory":"reference","format":"exr",
        "selection":{"start_frame":1004,"frame_count":3}});
    ok(&mut editor, json!({"op":"render_shot","request":render}));
    let mut request = render;
    request["directory"] = json!("job");
    request["type"] = json!("shot");
    let created = ok(&mut editor, json!({"op":"create_render_job","request":request}));
    assert_eq!(created["source_revision"], 3);
    fs::remove_file(scratch.0.join("speech.wav")).unwrap();
    drop(editor);
    let mut reopened = scratch.editor();
    ok(&mut reopened, json!({"op":"step_render_job","directory":"job"}));
    assert!(!scratch.0.join("job/audio.wav").exists());
    assert!(!scratch.0.join("job/manifest.json").exists());
    ok(&mut reopened, json!({"op":"cancel_render_job","directory":"job"}));
    drop(reopened);
    let mut reopened = scratch.editor();
    ok(&mut reopened, json!({"op":"resume_render_job","directory":"job"}));
    let result = ok(&mut reopened, json!({"op":"step_render_job","directory":"job","max_frames":32}));
    assert_eq!(result["status"], "complete");
    let actual_bytes = fs::read(scratch.0.join("job/audio.wav")).unwrap();
    assert_eq!(actual_bytes, fs::read(scratch.0.join("reference/audio.wav")).unwrap());
    let source = EmbeddedWave::from_bytes(SPEECH.to_vec()).unwrap();
    let actual = EmbeddedWave::from_bytes(actual_bytes).unwrap();
    let sample_rate = u64::from(source.wave().format().sample_rate);
    let first = 317 + 3 * sample_rate * 1001 / 30000;
    let end = 317 + 6 * sample_rate * 1001 / 30000;
    let source_bytes = source.wave().raw_samples();
    let bytes_per_frame = source_bytes.len() / source.wave().frame_count() as usize;
    assert_eq!(
        actual.wave().raw_samples(),
        &source_bytes[first as usize * bytes_per_frame..end as usize * bytes_per_frame]
    );
    let manifest = json_file(scratch.0.join("job/manifest.json"));
    assert_eq!(manifest["audio"]["start_sample"], first);
    assert_eq!(manifest["audio"]["end_sample_exclusive"], end);
    assert_eq!(manifest["audio"]["sample_frames"], end - first);
    for index in 0..3 {
        let name = format!("frame_{index:04}.exr");
        assert_eq!(
            exr_channels(scratch.0.join("job").join(&name)),
            exr_channels(scratch.0.join("reference").join(name))
        );
        let frame = &manifest["frames"][index];
        assert_eq!(frame["shot_frame"], 1004 + index);
        assert_eq!(frame["shot_index"], 3 + index);
        assert_eq!(frame["audio_window"]["start_sample"], 317 + (3 + index as u64) * sample_rate * 1001 / 30000);
        assert_eq!(frame["audio_window"]["end_sample"], 317 + (4 + index as u64) * sample_rate * 1001 / 30000);
    }
    assert_eq!(reopened.revision(), 0);
    assert!(reopened.document().audio.is_empty());

    let path = scratch.0.join("job/audio.wav");
    let mut corrupt = fs::read(&path).unwrap();
    let final_byte = corrupt.last_mut().unwrap();
    *final_byte ^= 1;
    fs::write(&path, &corrupt).unwrap();
    for command in [
        json!({"op":"render_job_state","directory":"job","verify_outputs":true}),
        json!({"op":"step_render_job","directory":"job"}),
    ] {
        let result = call(&mut reopened, command, None);
        assert_eq!(result.error.unwrap().code, "render_job_corrupt");
        assert_eq!(fs::read(&path).unwrap(), corrupt);
    }
}

#[test]
fn a_different_actual_executable_can_inspect_but_cannot_append_frames() {
    let scratch = Scratch::new();
    let mut editor = scratch.editor();
    scene(&mut editor);
    let created = create_sequence(&mut editor, "job", 1.0);
    let original_state = fs::read(scratch.0.join("job/state.json")).unwrap();
    // The Cargo test executable prepared this job. Launch the actual editor
    // executable to prove the fingerprint guard without editing any job metadata.
    let mut process = Command::new(env!("CARGO_BIN_EXE_mm3e-editor"))
        .arg("--root")
        .arg(&scratch.0)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = process.stdin.take().unwrap();
    for command in
        [json!({"op":"render_job_state","directory":"job"}), json!({"op":"step_render_job","directory":"job"})]
    {
        writeln!(input, "{}", json!({"id":"other-executable","command":command})).unwrap();
    }
    drop(input);
    let output = process.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let responses: Vec<Value> =
        String::from_utf8(output.stdout).unwrap().lines().map(|line| serde_json::from_str(line).unwrap()).collect();
    assert_eq!(responses.len(), 2);
    assert_eq!(responses[0]["ok"], true);
    assert_eq!(responses[0]["result"]["binary_sha256"], created["binary_sha256"]);
    assert_eq!(responses[1]["ok"], false);
    assert_eq!(responses[1]["error"]["code"], "render_job_renderer_mismatch");
    assert_eq!(fs::read(scratch.0.join("job/state.json")).unwrap(), original_state);
    assert!(!scratch.0.join("job/frame_0000.png").exists());
    assert_eq!(fs::read_dir(scratch.0.join("job/.render-staging")).unwrap().count(), 0);
}

#[test]
fn bounds_invalid_requests_and_revision_conflicts_create_no_work_or_authoring_changes() {
    let scratch = Scratch::new();
    let mut editor = scratch.editor();
    scene(&mut editor);
    let before = serde_json::to_vec(editor.document()).unwrap();
    for request in [
        json!({"type":"sequence","directory":"invalid","clip":"missing","start":0,"end":1,"fps":2}),
        json!({"type":"sequence","directory":"invalid","clip":"motion","start":1,"end":0,"fps":2}),
        json!({"type":"sequence","directory":"invalid","clip":"motion","start":0,"end":1,"fps":0}),
        json!({"type":"shot","directory":"invalid","shot":"missing"}),
        json!({"type":"sequence","directory":"../escape","clip":"motion","start":0,"end":1,"fps":2}),
    ] {
        assert!(!call(&mut editor, json!({"op":"create_render_job","request":request}), None).ok);
        assert!(!scratch.0.join("invalid").exists());
    }
    create_sequence(&mut editor, "job", 1.0);
    let state_before = fs::read(scratch.0.join("job/state.json")).unwrap();
    for max_frames in [0, 33, u32::MAX] {
        let result = call(&mut editor, json!({"op":"step_render_job","directory":"job","max_frames":max_frames}), None);
        assert_eq!(result.error.unwrap().code, "render_job_invalid");
        assert_eq!(fs::read(scratch.0.join("job/state.json")).unwrap(), state_before);
        assert!(!scratch.0.join("job/frame_0000.png").exists());
    }
    for command in [
        json!({"op":"create_render_job","request":{"type":"sequence","directory":"conflict","clip":"motion","start":0,"end":0,"fps":2}}),
        json!({"op":"step_render_job","directory":"job"}),
        json!({"op":"render_job_state","directory":"job"}),
        json!({"op":"cancel_render_job","directory":"job"}),
        json!({"op":"resume_render_job","directory":"job"}),
    ] {
        let result = call(&mut editor, command, Some(0));
        assert_eq!(result.error.unwrap().code, "revision_conflict");
    }
    assert!(!scratch.0.join("conflict").exists());
    assert_eq!(fs::read(scratch.0.join("job/state.json")).unwrap(), state_before);
    assert_eq!(json_file(scratch.0.join("job/control.json"))["cancelled"], false);
    assert_eq!(editor.revision(), 1);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), before);
    for command in [
        json!({"op":"step_render_job","directory":"job","unexpected":true}),
        json!({"op":"create_render_job","request":{"type":"sequence","directory":"bad","clip":"motion","start":0,"end":0,"fps":2,"overwrite":true}}),
    ] {
        assert!(serde_json::from_value::<Request>(json!({"id":"strict","command":command})).is_err());
    }
}

#[test]
fn foreign_frame_collision_preserves_bytes_and_retry_reconciles_the_pending_frame() {
    let scratch = Scratch::new();
    let mut editor = scratch.editor();
    scene(&mut editor);
    ok(&mut editor, json!({"op":"render_sequence","request":sequence("reference",0.0)}));
    create_sequence(&mut editor, "job", 0.0);
    let foreign = b"existing file must survive a colliding renderer";
    fs::write(scratch.0.join("job/frame_0000.png"), foreign).unwrap();
    let result = call(&mut editor, json!({"op":"step_render_job","directory":"job"}), None);
    assert!(!result.ok);
    assert_eq!(fs::read(scratch.0.join("job/frame_0000.png")).unwrap(), foreign);
    assert!(!scratch.0.join("job/manifest.json").exists());
    let state = ok(&mut editor, json!({"op":"render_job_state","directory":"job"}));
    assert_eq!(state["completed_frames"], 0);
    assert_eq!(state["pending_frame"], 0);
    let staged = fs::read_dir(scratch.0.join("job/.render-staging"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| p.extension().is_some_and(|e| e == "png"))
        .unwrap();
    let committed_bytes = fs::read(&staged).unwrap();
    let state_before = json_file(scratch.0.join("job/state.json"));
    drop(editor);
    // Represent an operator moving the unrelated collision aside; the job itself
    // must never delete or overwrite that file to make progress.
    fs::rename(scratch.0.join("job/frame_0000.png"), scratch.0.join("foreign.png")).unwrap();
    let mut reopened = scratch.editor();
    let result = ok(&mut reopened, json!({"op":"step_render_job","directory":"job"}));
    assert_eq!(result["status"], "complete");
    assert_eq!(result["completed_frames"], 1);
    assert_eq!(fs::read(scratch.0.join("job/frame_0000.png")).unwrap(), committed_bytes);
    assert_eq!(fs::read(scratch.0.join("foreign.png")).unwrap(), foreign);
    assert_eq!(json_file(scratch.0.join("job/state.json"))["next_attempt"], state_before["next_attempt"]);
    assert_eq!(
        png_pixels(scratch.0.join("job/frame_0000.png")),
        png_pixels(scratch.0.join("reference/frame_0000.png"))
    );
}

#[test]
fn corrupted_snapshot_plan_committed_frame_and_manifest_are_never_silently_reused() {
    for target in ["snapshot.json", "job.json", "frame_0000.png", "manifest.json"] {
        let scratch = Scratch::new();
        let mut editor = scratch.editor();
        scene(&mut editor);
        create_sequence(&mut editor, "job", 0.0);
        if matches!(target, "frame_0000.png" | "manifest.json") {
            ok(&mut editor, json!({"op":"step_render_job","directory":"job"}));
        }
        let path = scratch.0.join("job").join(target);
        let mut changed = fs::read(&path).unwrap();
        // Whitespace remains valid JSON while changing immutable byte identity.
        changed.push(b' ');
        fs::write(&path, &changed).unwrap();
        let state_before = fs::read(scratch.0.join("job/state.json")).unwrap();
        assert!(
            !call(&mut editor, json!({"op":"render_job_state","directory":"job","verify_outputs":true}), None).ok,
            "accepted corrupt {target}"
        );
        assert!(
            !call(&mut editor, json!({"op":"step_render_job","directory":"job"}), None).ok,
            "rendered after corrupt {target}"
        );
        assert_eq!(fs::read(&path).unwrap(), changed);
        assert_eq!(fs::read(scratch.0.join("job/state.json")).unwrap(), state_before);
        assert_eq!(editor.revision(), 1);
    }
}

fn assert_corrupt_renderer_record_rejected(field: &str) {
    let scratch = Scratch::new();
    let mut editor = scratch.editor();
    scene(&mut editor);
    create_sequence(&mut editor, "job", 0.5);
    let first = ok(&mut editor, json!({"op":"step_render_job","directory":"job"}));
    assert_eq!(first["completed_frames"], 1);
    assert_eq!(first["status"], "pending");
    let path = scratch.0.join("job/state.json");
    let mut state = json_file(&path);
    let record = &mut state["frames"][0]["record"];
    if field == "rgba_fnv1a64" {
        let mut fingerprint = record[field].as_str().unwrap().as_bytes().to_vec();
        fingerprint[0] = if fingerprint[0] == b'0' { b'1' } else { b'0' };
        record[field] = json!(String::from_utf8(fingerprint).unwrap());
    } else {
        assert_eq!(field, "view");
        record["view"]["eye"][0] = json!(42);
    }
    // Preserve the valid image, schedule, file hash and other committed fields.
    // Only one observation changes; this is independent metadata corruption,
    // not an adversary replacing both data and its integrity digest.
    fs::write(&path, serde_json::to_vec_pretty(&state).unwrap()).unwrap();
    let corrupted_bytes = fs::read(&path).unwrap();
    let frame_bytes = fs::read(scratch.0.join("job/frame_0000.png")).unwrap();
    let verified = call(&mut editor, json!({"op":"render_job_state","directory":"job","verify_outputs":true}), None);
    let stepped = call(&mut editor, json!({"op":"step_render_job","directory":"job"}), None);
    assert!(
        !verified.ok && !stepped.ok,
        "corrupt renderer record {field} was certified or reused: verification={verified:?}, step={stepped:?}"
    );
    assert_eq!(fs::read(&path).unwrap(), corrupted_bytes);
    assert_eq!(fs::read(scratch.0.join("job/frame_0000.png")).unwrap(), frame_bytes);
    assert!(!scratch.0.join("job/frame_0001.png").exists());
    assert!(!scratch.0.join("job/manifest.json").exists());
    assert_eq!(editor.revision(), 1);
}

#[test]
fn corrupted_renderer_pixel_fingerprint_is_not_certified_or_copied_to_delivery() {
    assert_corrupt_renderer_record_rejected("rgba_fnv1a64");
}

#[test]
fn corrupted_renderer_camera_is_not_certified_or_copied_to_delivery() {
    assert_corrupt_renderer_record_rejected("view");
}

#[test]
fn directory_and_manifest_collisions_are_preserved_and_completion_can_be_retried() {
    let scratch = Scratch::new();
    let mut editor = scratch.editor();
    scene(&mut editor);
    fs::create_dir(scratch.0.join("occupied")).unwrap();
    fs::write(scratch.0.join("occupied/keep.txt"), b"keep").unwrap();
    let result = call(
        &mut editor,
        json!({"op":"create_render_job","request":{
            "type":"sequence","directory":"occupied","clip":"motion","start":0,"end":0,"fps":2
        }}),
        None,
    );
    assert!(!result.ok);
    assert_eq!(fs::read_dir(scratch.0.join("occupied")).unwrap().count(), 1);
    assert_eq!(fs::read(scratch.0.join("occupied/keep.txt")).unwrap(), b"keep");
    create_sequence(&mut editor, "job", 0.0);
    fs::write(scratch.0.join("job/manifest.json"), b"foreign manifest").unwrap();
    assert!(!call(&mut editor, json!({"op":"step_render_job","directory":"job"}), None).ok);
    assert_eq!(fs::read(scratch.0.join("job/manifest.json")).unwrap(), b"foreign manifest");
    let state = ok(&mut editor, json!({"op":"render_job_state","directory":"job"}));
    assert_eq!(state["completed_frames"], 1);
    assert_eq!(state["status"], "pending");
    assert!(state["manifest_path"].is_null());
    let frame = fs::read(scratch.0.join("job/frame_0000.png")).unwrap();
    fs::rename(scratch.0.join("job/manifest.json"), scratch.0.join("foreign-manifest.json")).unwrap();
    let state = ok(&mut editor, json!({"op":"step_render_job","directory":"job"}));
    assert_eq!(state["status"], "complete");
    assert_eq!(fs::read(scratch.0.join("job/frame_0000.png")).unwrap(), frame);
    assert_eq!(fs::read(scratch.0.join("foreign-manifest.json")).unwrap(), b"foreign manifest");
}

#[cfg(unix)]
#[test]
fn symlinked_job_metadata_and_destinations_are_rejected_without_touching_the_target() {
    for target in ["snapshot.json", "state.json", "control.json", "frame_0000.png"] {
        let scratch = Scratch::new();
        let mut editor = scratch.editor();
        scene(&mut editor);
        create_sequence(&mut editor, "job", 0.0);
        let job_path = scratch.0.join("job").join(target);
        let original = if job_path.exists() { fs::read(&job_path).unwrap() } else { b"foreign image".to_vec() };
        let foreign = scratch.0.join("foreign");
        fs::write(&foreign, &original).unwrap();
        if job_path.exists() {
            fs::remove_file(&job_path).unwrap();
        }
        std::os::unix::fs::symlink(&foreign, &job_path).unwrap();
        assert!(
            !call(&mut editor, json!({"op":"step_render_job","directory":"job"}), None).ok,
            "accepted symlink {target}"
        );
        assert_eq!(fs::read(&foreign).unwrap(), original);
        assert!(fs::symlink_metadata(&job_path).unwrap().file_type().is_symlink());
        assert!(!scratch.0.join("job/manifest.json").exists());
    }
}
