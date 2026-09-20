use mm3e_editor::{model::Pass, protocol::Request, Editor};
use mm3e_kit::{color::Rgba, vec::Vec3};
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);
struct Root(PathBuf);
impl Root {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mm3e-animation-{}-{}-{}",
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
fn send(editor: &mut Editor, command: Value, okay: bool) -> Value {
    let request: Request =
        serde_json::from_value(json!({"id":"test","expected_revision":editor.revision(),"command":command})).unwrap();
    let response = editor.handle(request);
    assert_eq!(response.ok, okay, "{:?}", response);
    response.result.unwrap_or_else(|| serde_json::to_value(response.error).unwrap())
}
fn apply(editor: &mut Editor, operations: Vec<Value>) {
    send(editor, json!({"op":"apply","operations":operations}), true);
}
fn sphere(id: &str, position: [f32; 3]) -> Value {
    json!({"op":"create","object":{"id":id,"position":position,"shape":{"type":"sphere","radius":0.2}}})
}
fn track(target_type: &str, id: &str, keys: Value) -> Value {
    json!({"target":{"type":target_type,"id":id},"keys":keys})
}
fn clip(tracks: Vec<Value>) -> Value {
    json!({"op":"put_clip","clip":{"id":"motion","duration":1,"tracks":tracks}})
}
fn sample(time: f32, playback: &str) -> Value {
    json!({"clip":"motion","time":time,"playback":playback})
}
fn pose(editor: &mut Editor, time: f32, playback: &str) -> Value {
    send(editor, json!({"op":"pose","animation":sample(time,playback)}), true)
}
fn near(actual: &Value, expected: [f64; 3]) {
    for (i, want) in expected.into_iter().enumerate() {
        assert!((actual[i].as_f64().unwrap() - want).abs() < 1e-5, "{actual} != {expected:?}");
    }
}

#[test]
fn object_pivot_rotation_scale_and_translation_reach_real_geometry() {
    let root = Root::new();
    let mut editor = root.editor();
    let mut motion = track(
        "object",
        "orb",
        json!([{"time":0},{"time":1,"translation":[0,1,0],"rotation_degrees":[0,0,90],"scale":2}]),
    );
    motion["pivot"] = json!([1, 0, 0]);
    apply(&mut editor, vec![sphere("orb", [2.0, 0.0, 0.0]), clip(vec![motion])]);
    let original = serde_json::to_value(editor.document()).unwrap();
    let revision = editor.revision();
    let evaluated = pose(&mut editor, 1.0, "clamp");
    near(&evaluated["objects"][0]["position"], [1.0, 3.0, 0.0]);
    assert_eq!(evaluated["objects"][0]["scale"], 2.0);
    let measurement =
        send(&mut editor, json!({"op":"sample","id":"orb","points":[[1,3,0]],"animation":sample(1.0,"clamp")}), true);
    assert!(measurement["samples"][0]["value"].as_f64().unwrap() < -0.39);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), original);
    assert_eq!(editor.revision(), revision);
}

#[test]
fn parent_motion_carries_child_joint_and_bound_offset_geometry() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("hand", [2.0, 0.0, 0.0]),
            json!({"op":"set_joints","joints":[
        {"id":"child","parent":"root","pivot":[1,0,0],"objects":["hand"]},
        {"id":"root","pivot":[0,0,0]}]}),
            clip(vec![
                track("joint", "root", json!([{"time":0},{"time":1,"rotation_degrees":[0,0,90],"scale":2}])),
                track(
                    "joint",
                    "child",
                    json!([{"time":0},{"time":1,"translation":[1,0,0],"rotation_degrees":[0,0,90]}]),
                ),
            ]),
        ],
    );
    let result = pose(&mut editor, 1.0, "clamp");
    near(&result["joints"][0]["world_pivot"], [0.0, 4.0, 0.0]);
    near(&result["objects"][0]["position"], [-2.0, 4.0, 0.0]);
    assert_eq!(result["objects"][0]["scale"], 2.0);
}

#[test]
fn clamp_loop_negative_time_and_shortest_arc_are_explicit() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("orb", [0.0; 3]),
            clip(vec![track(
                "object",
                "orb",
                json!([
        {"time":0,"translation":[0,0,0],"rotation_degrees":[0,0,170]},
        {"time":1,"translation":[2,0,0],"rotation_degrees":[0,0,-170]}]),
            )]),
        ],
    );
    let midpoint = pose(&mut editor, 0.5, "clamp");
    near(&midpoint["objects"][0]["basis"][0], [-1.0, 0.0, 0.0]);
    near(&midpoint["objects"][0]["position"], [1.0, 0.0, 0.0]);
    assert_eq!(pose(&mut editor, -3.0, "clamp")["objects"], pose(&mut editor, 0.0, "clamp")["objects"]);
    assert_eq!(pose(&mut editor, 3.0, "clamp")["objects"], pose(&mut editor, 1.0, "clamp")["objects"]);
    assert_eq!(pose(&mut editor, 1.0, "loop")["objects"], pose(&mut editor, 0.0, "clamp")["objects"]);
    assert_eq!(pose(&mut editor, -0.5, "loop")["objects"], midpoint["objects"]);
    assert_eq!(pose(&mut editor, 2.5, "loop")["objects"], midpoint["objects"]);
}

#[test]
fn unanimated_rig_preserves_rotated_rest_basis_and_pixels_exactly() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            json!({"op":"create","object":{"id":"box","shape":{"type":"box","half_extents":[0.7,0.2,0.4]},
        "rotation_degrees":[17.1,32.6,-68.3]}}),
            json!({"op":"set_settings","settings":{"width":48,"height":48}}),
            json!({"op":"set_camera","camera":{"eye":[0,0,3],"target":[0,0,0]}}),
            json!({"op":"set_joints","joints":[{"id":"joint","pivot":[0.1,0.2,-0.3],"objects":["box"]}]}),
            clip(vec![]),
        ],
    );
    let doc = editor.document();
    let (rest, camera) = doc.compile(&Pass::Beauty).unwrap();
    let animation = serde_json::from_value(sample(0.5, "clamp")).unwrap();
    let (posed, posed_camera) = doc.compile_at(&Pass::Beauty, Some(&animation)).unwrap();
    assert_eq!(rest.objects[0].xform.rot.cols, posed.objects[0].xform.rot.cols);
    assert_eq!(rest.objects[0].xform.pos, posed.objects[0].xform.pos);
    let pixels = |s, c| mm3e_orchestrator::render(s, c).to_rgba8(Rgba::rgb8(0, 0, 0));
    assert_eq!(pixels(&rest, &camera), pixels(&posed, &posed_camera));
}

#[test]
fn invalid_hierarchy_and_tracks_reject_the_whole_transaction() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(&mut editor, vec![sphere("orb", [0.0; 3])]);
    let baseline = serde_json::to_value(editor.document()).unwrap();
    let revision = editor.revision();
    let malformed = vec![
        json!({"op":"set_joints","joints":[{"id":"a","parent":"b","pivot":[0,0,0]},{"id":"b","parent":"a","pivot":[0,0,0]}]}),
        json!({"op":"set_joints","joints":[{"id":"a","pivot":[0,0,0],"objects":["orb"]},{"id":"b","pivot":[0,0,0],"objects":["orb"]}]}),
        json!({"op":"set_joints","joints":[{"id":"a","parent":"missing","pivot":[0,0,0]}]}),
        clip(vec![track("object", "missing", json!([{"time":0}]))]),
        clip(vec![track("object", "orb", json!([]))]),
        clip(vec![track("object", "orb", json!([{"time":0},{"time":0}]))]),
        clip(vec![track("object", "orb", json!([{"time":0,"scale":0}]))]),
        clip(vec![track("object", "orb", json!([{"time":2}]))]),
        clip(vec![track("object", "orb", json!([{"time":0}])), track("object", "orb", json!([{"time":1}]))]),
    ];
    for operation in malformed {
        send(
            &mut editor,
            json!({"op":"apply","operations":[{"op":"translate","ids":["orb"],"delta":[1,0,0]},operation]}),
            false,
        );
        assert_eq!(editor.revision(), revision);
        assert_eq!(serde_json::to_value(editor.document()).unwrap(), baseline);
    }
}

#[test]
fn deleting_a_bound_target_requires_explicit_dependency_repair() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("orb", [0.0; 3]),
            json!({"op":"set_joints","joints":[{"id":"joint","pivot":[0,0,0],"objects":["orb"]}]}),
            clip(vec![track("joint", "joint", json!([{"time":0}]))]),
        ],
    );
    send(&mut editor, json!({"op":"apply","operations":[{"op":"delete","id":"orb"}]}), false);
    assert_eq!(editor.document().objects.len(), 1);
    apply(
        &mut editor,
        vec![
            json!({"op":"delete_clip","id":"motion"}),
            json!({"op":"set_joints","joints":[]}),
            json!({"op":"delete","id":"orb"}),
        ],
    );
    assert!(editor.document().objects.is_empty());
}

#[test]
fn invalid_intermediate_camera_fails_sampling_and_sequence_before_output() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![json!({"op":"put_clip","clip":{"id":"motion","duration":1,"camera_keys":[
        {"time":0,"eye":[0,0,1],"target":[0,0,0],"fov_degrees":40},
        {"time":1,"eye":[0,0,-1],"target":[0,0,0],"fov_degrees":40}]}})],
    );
    send(&mut editor, json!({"op":"pose","animation":sample(0.5,"clamp")}), false);
    send(
        &mut editor,
        json!({"op":"render_sequence","request":{"directory":"bad","clip":"motion","start":0,"end":1,"fps":2}}),
        false,
    );
    assert!(!root.0.join("bad").exists());
    assert_eq!(editor.revision(), 1);
}

#[test]
fn camera_keyframes_reach_render_and_explicit_view_override_wins() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("orb", [0.0; 3]),
            json!({"op":"set_settings","settings":{"width":32,"height":32}}),
            json!({"op":"put_clip","clip":{"id":"motion","duration":1,"camera_keys":[
        {"time":0,"eye":[0,0,3],"target":[0,0,0],"fov_degrees":40},
        {"time":1,"eye":[2,0,3],"target":[0,0,0],"fov_degrees":50}]}}),
        ],
    );
    let result = send(&mut editor, json!({"op":"render","path":"camera.png","animation":sample(0.5,"clamp")}), true);
    near(&result["view"]["eye"], [1.0, 0.0, 3.0]);
    assert!((result["view"]["fov_degrees"].as_f64().unwrap() - 45.0).abs() < 1e-4);
    let overridden = send(
        &mut editor,
        json!({"op":"render","path":"override.png","animation":sample(0.5,"clamp"),
        "view":{"eye":[0,2,4],"target":[0,0,0]}}),
        true,
    );
    near(&overridden["view"]["eye"], [0.0, 2.0, 4.0]);
}

#[test]
fn pre_animation_projects_load_with_empty_animation_and_keep_their_geometry() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(&mut editor, vec![sphere("old", [0.0; 3])]);
    let mut old = serde_json::to_value(editor.document()).unwrap();
    old.as_object_mut().unwrap().remove("clips");
    old.as_object_mut().unwrap().remove("joints");
    fs::write(
        root.0.join("old.json"),
        serde_json::to_vec(&json!({"format":"mm3e-agent-project-v1","saved_revision":1,"document":old})).unwrap(),
    )
    .unwrap();
    let mut fresh = root.editor();
    send(&mut fresh, json!({"op":"load","path":"old.json"}), true);
    assert!(fresh.document().clips.is_empty());
    assert!(fresh.document().joints.is_empty());
    assert_eq!(fresh.document().objects[0].id, "old");
    let (scene, _) = fresh.document().compile(&Pass::Beauty).unwrap();
    assert!((scene.sample_authored(Vec3::ZERO).dist + 0.2).abs() < 1e-6);
}
