//! Agent-boundary coverage for sparse clip composition and its geometric consequences.
use mm3e_editor::{protocol::Request, Editor};
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Root(PathBuf);
impl Root {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mm3e-layer-tests-{}-{}-{}",
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
impl Drop for Root {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn send(editor: &mut Editor, command: Value, okay: bool) -> Value {
    let request: Request =
        serde_json::from_value(json!({"id":"layer-test", "expected_revision":editor.revision(), "command":command}))
            .unwrap();
    let response = editor.handle(request);
    assert_eq!(response.ok, okay, "{response:?}");
    response.result.unwrap_or_else(|| serde_json::to_value(response.error).unwrap())
}
fn apply(editor: &mut Editor, operations: Vec<Value>) {
    send(editor, json!({"op":"apply", "operations":operations}), true);
}
fn put(clip: Value) -> Value {
    json!({"op":"put_clip", "clip":clip})
}
fn sphere(id: &str, position: [f32; 3]) -> Value {
    json!({"op":"create", "object":{"id":id,"position":position,"shape":{"type":"sphere","radius":0.2}}})
}
fn pose(editor: &mut Editor, clip: &str, time: f32) -> Value {
    send(editor, json!({"op":"pose","animation":{"clip":clip,"time":time}}), true)
}
fn object<'a>(pose: &'a Value, id: &str) -> &'a Value {
    pose["objects"].as_array().unwrap().iter().find(|o| o["id"] == id).unwrap()
}
fn near(value: &Value, expected: [f64; 3]) {
    for (i, want) in expected.into_iter().enumerate() {
        assert!((value[i].as_f64().unwrap() - want).abs() < 2e-5, "{value} != {expected:?}");
    }
}
fn sample(editor: &mut Editor, id: &str, point: [f32; 3], clip: &str, time: f32) -> f64 {
    send(editor, json!({"op":"sample","id":id,"points":[point],"animation":{"clip":clip,"time":time}}), true)["samples"]
        [0]["value"]
        .as_f64()
        .unwrap()
}

#[test]
fn source_clocks_masks_windows_and_sparse_channels_reach_the_field() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("a", [0.0; 3]),
            sphere("b", [0.0; 3]),
            put(json!({"id":"source","duration":2,"tracks":[
            {"target":{"type":"object","id":"a"},"keys":[{"time":0},{"time":2,"translation":[4,0,0]}]},
            {"target":{"type":"object","id":"b"},"keys":[{"time":0},{"time":2,"translation":[0,8,0]}]}]})),
            put(
                json!({"id":"b_only","duration":2,"tracks":[{"target":{"type":"object","id":"b"},"keys":[{"time":0,"translation":[0,3,0]}]}]}),
            ),
            put(json!({"id":"take","duration":4,"layers":[
            {"id":"clock","clip":"source","start":1,"end":3,"source_start":-0.5,"time_scale":2,"playback":"loop","mask":[{"type":"object","id":"a"}]},
            {"id":"sparse","clip":"b_only"}]})),
            put(
                json!({"id":"clamped","duration":4,"layers":[{"id":"clock","clip":"source","source_start":-1,"time_scale":2}]}),
            ),
        ],
    );
    let before = serde_json::to_value(editor.document()).unwrap();
    // Euclidean loop of -0.5 is 1.5; both window endpoints are included.
    for (time, x) in [(0.999, 0.0), (1.0, 3.0), (1.5, 1.0), (2.0, 3.0), (3.0, 3.0), (3.001, 0.0)] {
        let value = pose(&mut editor, "take", time);
        near(&object(&value, "a")["position"], [x, 0.0, 0.0]);
        near(&object(&value, "b")["position"], [0.0, 3.0, 0.0]);
        assert!(sample(&mut editor, "a", [x as f32, 0.0, 0.0], "take", time) < -0.19);
    }
    near(&object(&pose(&mut editor, "clamped", 0.0), "a")["position"], [0.0; 3]);
    near(&object(&pose(&mut editor, "clamped", 4.0), "a")["position"], [4.0, 0.0, 0.0]);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), before);
}

#[test]
fn parent_time_weight_curves_multiply_constant_weight_and_local_tracks_override() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("a", [0.0; 3]),
            sphere("b", [0.0; 3]),
            put(json!({"id":"source","duration":1,"tracks":[
            {"target":{"type":"object","id":"a"},"keys":[{"time":0,"translation":[8,0,0]}]},
            {"target":{"type":"object","id":"b"},"keys":[{"time":0,"translation":[0,8,0]}]}]})),
            put(
                json!({"id":"weighted","duration":4,"layers":[{"id":"blend","clip":"source","start":1,"source_start":0.75,"time_scale":0.1,"weight":0.5,
            "weight_keys":[{"time":1,"value":0},{"time":3,"value":1}]}]}),
            ),
            put(
                json!({"id":"local","duration":4,"layers":[{"id":"blend","clip":"weighted"}],"tracks":[{"target":{"type":"object","id":"a"},"keys":[{"time":0,"translation":[-2,0,0]}]}]}),
            ),
        ],
    );
    for (time, x) in [(1.0, 0.0), (2.0, 2.0), (3.0, 4.0)] {
        let weighted = pose(&mut editor, "weighted", time);
        near(&object(&weighted, "a")["position"], [x, 0.0, 0.0]);
        let local = pose(&mut editor, "local", time);
        near(&object(&local, "a")["position"], [-2.0, 0.0, 0.0]);
        near(&object(&local, "b")["position"], [0.0, x, 0.0]);
    }
}

#[test]
fn override_blends_at_rest_anchor_and_exact_endpoints_preserve_source_geometry() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("orb", [2.0, 0.0, 0.0]),
            put(
                json!({"id":"source","duration":1,"tracks":[{"target":{"type":"object","id":"orb"},"pivot":[0,0,0],"keys":[{"time":0,"rotation_degrees":[0,0,90]}]}]}),
            ),
            put(json!({"id":"half","duration":1,"layers":[{"id":"mix","clip":"source","weight":0.5}]})),
            put(json!({"id":"full","duration":1,"layers":[{"id":"mix","clip":"source","weight":1}]})),
            put(json!({"id":"zero","duration":1,"layers":[{"id":"mix","clip":"source","weight":0}]})),
            put(json!({"id":"rest","duration":1})),
        ],
    );
    let halfway = pose(&mut editor, "half", 0.5);
    near(&object(&halfway, "orb")["position"], [1.0, 1.0, 0.0]);
    let diagonal = std::f64::consts::FRAC_1_SQRT_2;
    near(&object(&halfway, "orb")["basis"][0], [diagonal, diagonal, 0.0]);
    assert!(sample(&mut editor, "orb", [1.0, 1.0, 0.0], "half", 0.5) < -0.19);
    assert_eq!(pose(&mut editor, "full", 0.5)["objects"], pose(&mut editor, "source", 0.5)["objects"]);
    assert_eq!(pose(&mut editor, "zero", 0.5)["objects"], pose(&mut editor, "rest", 0.5)["objects"]);
    assert_eq!(
        sample(&mut editor, "orb", [0.0, 2.0, 0.0], "full", 0.5),
        sample(&mut editor, "orb", [0.0, 2.0, 0.0], "source", 0.5)
    );
}

#[test]
fn additive_reference_preserves_noncommuting_joint_order_translation_and_scale() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("hand", [2.0, 0.0, 0.0]),
            json!({"op":"set_joints","joints":[{"id":"child","parent":"root","pivot":[1,0,0],"objects":["hand"]},{"id":"root","pivot":[0,0,0]}]}),
            put(json!({"id":"base","duration":1,"tracks":[
            {"target":{"type":"joint","id":"root"},"keys":[{"time":0,"rotation_degrees":[0,90,0]}]},
            {"target":{"type":"joint","id":"child"},"keys":[{"time":0,"rotation_degrees":[0,0,90],"translation":[0,1,0],"scale":3}]}]})),
            put(json!({"id":"detail","duration":1,"tracks":[{"target":{"type":"joint","id":"child"},"keys":[
            {"time":0,"rotation_degrees":[90,0,0],"translation":[1,0,0],"scale":2},
            {"time":1,"rotation_degrees":[180,0,0],"translation":[2,0,0],"scale":4}]}]})),
            put(
                json!({"id":"take","duration":1,"layers":[{"id":"base","clip":"base"},{"id":"detail","clip":"detail","mode":"additive","reference_time":0}]}),
            ),
        ],
    );
    let value = pose(&mut editor, "take", 1.0);
    let hand = object(&value, "hand");
    // R_y(parent) * R_z(base child) * R_x(detail relative to its reference).
    near(&hand["basis"][0], [0.0, 1.0, 0.0]);
    near(&hand["basis"][1], [1.0, 0.0, 0.0]);
    near(&hand["basis"][2], [0.0, 0.0, -1.0]);
    near(&hand["position"], [0.0, 7.0, -2.0]);
    assert!((hand["scale"].as_f64().unwrap() - 6.0).abs() < 1e-6);
    assert!(sample(&mut editor, "hand", [0.0, 7.0, -2.0], "take", 1.0) < -1.19);
    // Sampling the reference itself must add exactly no motion or conversion noise.
    assert_eq!(pose(&mut editor, "take", 0.0)["objects"], pose(&mut editor, "base", 0.0)["objects"]);
}

#[test]
fn disabled_zero_and_masked_layers_still_validate_graph_and_edits_are_atomic() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("orb", [0.0; 3]),
            put(json!({"id":"source","duration":1})),
            put(json!({"id":"take","duration":1,"layers":[{"id":"base","clip":"source"}]})),
        ],
    );
    let baseline = serde_json::to_value(editor.document()).unwrap();
    let revision = editor.revision();
    let mut malformed = vec![
        put(json!({"id":"source","duration":1,"layers":[{"id":"cycle","clip":"take","enabled":false}]})),
        put(json!({"id":"source","duration":1,"layers":[{"id":"cycle","clip":"take","weight":0}]})),
        put(json!({"id":"source","duration":1,"layers":[{"id":"cycle","clip":"take","mask":[]}]})),
        put(json!({"id":"bad","duration":1,"layers":[{"id":"absent","clip":"missing","enabled":false}]})),
        put(
            json!({"id":"bad","duration":1,"layers":[{"id":"mask","clip":"source","mask":[{"type":"object","id":"missing"}]}]}),
        ),
        put(
            json!({"id":"bad","duration":1,"layers":[{"id":"mask","clip":"source","mask":[{"type":"object","id":"orb"},{"type":"object","id":"orb"}]}]}),
        ),
        put(json!({"id":"bad","duration":1,"layers":[{"id":"a","clip":"source"},{"id":"a","clip":"source"}]})),
        put(json!({"id":"bad","duration":1,"layers":[{"id":"reference","clip":"source","reference_time":0}]})),
        put(json!({"id":"bad","duration":1,"layers":[{"id":"time","clip":"source","start":0.7,"end":0.3}]})),
        put(json!({"id":"bad","duration":1,"layers":[{"id":"time","clip":"source","time_scale":0}]})),
        json!({"op":"delete_clip","id":"source"}),
    ];
    let too_many: Vec<_> = (0..17).map(|i| json!({"id":format!("layer{i}"),"clip":"source"})).collect();
    malformed.push(put(json!({"id":"bad","duration":1,"layers":too_many})));
    for operation in malformed {
        send(
            &mut editor,
            json!({"op":"apply","operations":[{"op":"translate","ids":["orb"],"delta":[2,0,0]},operation]}),
            false,
        );
        assert_eq!(editor.revision(), revision);
        assert_eq!(serde_json::to_value(editor.document()).unwrap(), baseline);
    }
    // Dependencies can be repaired explicitly inside the same atomic transaction.
    apply(&mut editor, vec![json!({"op":"delete_clip","id":"source"}), put(json!({"id":"take","duration":1}))]);
}

#[test]
fn graph_depth_and_expanded_reference_cost_are_bounded_even_for_inactive_branches() {
    let root = Root::new();
    let mut editor = root.editor();
    let mut operations = vec![put(json!({"id":"n0","duration":1}))];
    for i in 1..8 {
        operations.push(put(json!({"id":format!("n{i}"),"duration":1,"layers":[{"id":"previous","clip":format!("n{}",i-1),"enabled":false}]})));
    }
    apply(&mut editor, operations);
    let baseline = serde_json::to_value(editor.document()).unwrap();
    send(
        &mut editor,
        json!({"op":"apply","operations":[put(json!({"id":"n8","duration":1,"layers":[{"id":"previous","clip":"n7","weight":0}]}))]}),
        false,
    );
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), baseline);
    // Three children with reference poses expand by 6 at each level: 1, 7, 43, 259, 1555.
    let mut operations = vec![put(json!({"id":"work0","duration":1}))];
    for i in 1..4 {
        let layers:Vec<_>=(0..3).map(|j|json!({"id":format!("branch{j}"),"clip":format!("work{}",i-1),"mode":"additive","reference_time":0,"enabled":false})).collect();
        operations.push(put(json!({"id":format!("work{i}"),"duration":1,"layers":layers})));
    }
    apply(&mut editor, operations);
    let layers: Vec<_> = (0..3)
        .map(|j| json!({"id":format!("branch{j}"),"clip":"work3","mode":"additive","reference_time":0,"enabled":false}))
        .collect();
    send(
        &mut editor,
        json!({"op":"apply","operations":[put(json!({"id":"work4","duration":1,"layers":layers}))]}),
        false,
    );
}

fn face_and_morph(editor: &mut Editor) {
    apply(
        editor,
        vec![
            json!({"op":"create","object":{"id":"actor/head","shape":{"type":"ellipsoid","radii":[0.8,1.0,0.8]}}}),
            sphere("actor/left_eye", [0.28, 0.20, 0.78]),
            sphere("actor/right_eye", [-0.28, 0.20, 0.78]),
            json!({"op":"create_face","request":{"id":"face","character":"actor"}}),
            json!({"op":"create","object":{"id":"sheet","shape":{"type":"surface","vertices":[[0,0,0],[1,0,0],[0,1,0]],"triangles":[[0,1,2]],"thickness_m":0.02}}}),
            json!({"op":"bind_surface","request":{"deformer":{"id":"shape","object":"sheet","blendshapes":[
            {"id":"x","deltas":[[1,0,0],[1,0,0],[1,0,0]],"weight":0.2},
            {"id":"y","deltas":[[0,1,0],[0,1,0],[0,1,0]],"weight":0.25}]}}}),
        ],
    );
}

#[test]
fn layered_face_and_morph_curves_change_real_geometry_and_survive_native_save_load() {
    let root = Root::new();
    let mut editor = root.editor();
    face_and_morph(&mut editor);
    apply(
        &mut editor,
        vec![
            put(
                json!({"id":"expression","duration":1,"face_tracks":[{"face":"face","channel":"jaw_open","keys":[{"time":0,"value":0},{"time":1,"value":1}]}],
            "morph_tracks":[{"deformer":"shape","blendshape":"x","keys":[{"time":0,"weight":0.2},{"time":1,"weight":0.8}]}]}),
            ),
            put(
                json!({"id":"blink","duration":1,"face_tracks":[{"face":"face","channel":"blink_left","keys":[{"time":0,"value":0},{"time":1,"value":1}]}]}),
            ),
            put(
                json!({"id":"take","duration":1,"layers":[{"id":"expression","clip":"expression"},{"id":"blink","clip":"blink"}]}),
            ),
            put(
                json!({"id":"masked","duration":1,"layers":[{"id":"expression","clip":"expression","mask":[{"type":"morph","deformer":"shape","blendshape":"x"}]}]}),
            ),
        ],
    );
    // The mouth sample lies inside the neutral head and becomes air after jaw motion.
    assert!(sample(&mut editor, "actor/head", [0.0, -0.467, 0.66], "take", 0.0) < 0.0);
    assert!(sample(&mut editor, "actor/head", [0.0, -0.467, 0.66], "take", 1.0) > 0.0);
    assert!(sample(&mut editor, "actor/head", [0.0, -0.467, 0.66], "masked", 1.0) < 0.0);
    let state =
        send(&mut editor, json!({"op":"deformer_state","id":"shape","animation":{"clip":"take","time":1}}), true);
    near(&state["vertices"][0], [0.8, 0.25, 0.0]);
    assert!(sample(&mut editor, "sheet", [0.8, 0.25, 0.0], "take", 1.0) < -0.009);
    let face = send(&mut editor, json!({"op":"face_state","animation":{"clip":"take","time":1}}), true);
    let poses: Vec<_> = [0.0, 0.31, 1.0].into_iter().map(|t| pose(&mut editor, "take", t)).collect();
    send(&mut editor, json!({"op":"save","path":"layers.json"}), true);
    let mut loaded = root.editor();
    send(&mut loaded, json!({"op":"load","path":"layers.json"}), true);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), serde_json::to_value(loaded.document()).unwrap());
    for (time, before) in [0.0, 0.31, 1.0].into_iter().zip(poses) {
        let after = pose(&mut loaded, "take", time);
        assert_eq!(before["objects"], after["objects"]);
        assert_eq!(before["joints"], after["joints"]);
    }
    assert_eq!(
        state,
        send(&mut loaded, json!({"op":"deformer_state","id":"shape","animation":{"clip":"take","time":1}}), true)
    );
    assert_eq!(face, send(&mut loaded, json!({"op":"face_state","animation":{"clip":"take","time":1}}), true));
    assert_eq!(
        sample(&mut editor, "actor/head", [0.0, -0.467, 0.66], "take", 1.0),
        sample(&mut loaded, "actor/head", [0.0, -0.467, 0.66], "take", 1.0)
    );
}

#[test]
fn additive_scalar_rest_reference_is_respected_and_out_of_range_results_fail() {
    let root = Root::new();
    let mut editor = root.editor();
    face_and_morph(&mut editor);
    apply(
        &mut editor,
        vec![
            put(
                json!({"id":"source","duration":1,"morph_tracks":[{"deformer":"shape","blendshape":"x","keys":[{"time":0,"weight":0.2},{"time":1,"weight":0.8}]}]}),
            ),
            put(
                json!({"id":"half","duration":1,"layers":[{"id":"add","clip":"source","mode":"additive","weight":0.5}]}),
            ),
            put(
                json!({"id":"overflow","duration":1,"layers":[{"id":"base","clip":"source"},{"id":"add","clip":"source","mode":"additive"}]}),
            ),
            put(
                json!({"id":"repaired","duration":1,"layers":[{"id":"input","clip":"overflow"}],"morph_tracks":[{"deformer":"shape","blendshape":"x","keys":[{"time":0,"weight":0.6}]}]}),
            ),
        ],
    );
    let state =
        send(&mut editor, json!({"op":"deformer_state","id":"shape","animation":{"clip":"half","time":1}}), true);
    near(&state["vertices"][0], [0.5, 0.25, 0.0]);
    let before = serde_json::to_value(editor.document()).unwrap();
    let revision = editor.revision();
    send(&mut editor, json!({"op":"pose","animation":{"clip":"overflow","time":1}}), false);
    send(&mut editor, json!({"op":"deformer_state","id":"shape","animation":{"clip":"overflow","time":1}}), false);
    send(
        &mut editor,
        json!({"op":"sample","id":"sheet","points":[[1.4,0.25,0]],"animation":{"clip":"overflow","time":1}}),
        false,
    );
    assert_eq!(editor.revision(), revision);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), before);
    let repaired =
        send(&mut editor, json!({"op":"deformer_state","id":"shape","animation":{"clip":"repaired","time":1}}), true);
    near(&repaired["vertices"][0], [0.6, 0.25, 0.0]);
}

#[test]
fn camera_layers_reach_render_with_typed_masks_local_overrides_and_invalid_pose_rejection() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("orb", [0.0; 3]),
            json!({"op":"set_settings","settings":{"width":32,"height":32}}),
            json!({"op":"set_camera","camera":{"eye":[0,0,3],"target":[0,0,0],"fov_degrees":40}}),
            put(
                json!({"id":"source","duration":1,"camera_keys":[{"time":0,"eye":[2,0,5],"target":[0,0,0],"fov_degrees":60}]}),
            ),
            put(
                json!({"id":"half","duration":1,"layers":[{"id":"camera","clip":"source","weight":0.5,"mask":[{"type":"camera"}]}]}),
            ),
            put(json!({"id":"full","duration":1,"layers":[{"id":"camera","clip":"source"}]})),
            put(
                json!({"id":"masked","duration":1,"layers":[{"id":"camera","clip":"source","mask":[{"type":"object","id":"orb"}]}]}),
            ),
            put(
                json!({"id":"back","duration":1,"camera_keys":[{"time":0,"eye":[0,0,-3],"target":[0,0,0],"fov_degrees":40}]}),
            ),
            put(json!({"id":"singular","duration":1,"layers":[{"id":"camera","clip":"back","weight":0.5}]})),
            put(
                json!({"id":"local","duration":1,"layers":[{"id":"camera","clip":"singular"}],"camera_keys":[{"time":0,"eye":[0,2,4],"target":[0,0,0],"fov_degrees":45}]}),
            ),
        ],
    );
    for (clip, eye, fov) in [
        ("half", [1.0, 0.0, 4.0], 50.0),
        ("full", [2.0, 0.0, 5.0], 60.0),
        ("source", [2.0, 0.0, 5.0], 60.0),
        ("masked", [0.0, 0.0, 3.0], 40.0),
        ("local", [0.0, 2.0, 4.0], 45.0),
    ] {
        let result = send(
            &mut editor,
            json!({"op":"render","path":format!("{clip}.png"),"animation":{"clip":clip,"time":0.5}}),
            true,
        );
        near(&result["view"]["eye"], eye);
        assert!((result["view"]["fov_degrees"].as_f64().unwrap() - fov).abs() < 1e-5);
    }
    assert_eq!(fs::read(root.0.join("full.png")).unwrap(), fs::read(root.0.join("source.png")).unwrap());
    assert_ne!(fs::read(root.0.join("half.png")).unwrap(), fs::read(root.0.join("masked.png")).unwrap());
    send(&mut editor, json!({"op":"pose","animation":{"clip":"singular","time":0.5}}), false);
    send(&mut editor, json!({"op":"render","path":"invalid.png","animation":{"clip":"singular","time":0.5}}), false);
    assert!(!root.0.join("invalid.png").exists());
}

#[test]
fn sparse_layer_edits_preserve_local_channels_and_history_while_order_changes_real_pose() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("a", [0.0; 3]),
            sphere("b", [0.0; 3]),
            put(
                json!({"id":"low","duration":1,"tracks":[{"target":{"type":"object","id":"a"},"keys":[{"time":0,"translation":[2,0,0]}]}]}),
            ),
            put(
                json!({"id":"high","duration":1,"tracks":[{"target":{"type":"object","id":"a"},"keys":[{"time":0,"translation":[10,0,0]}]}]}),
            ),
            put(
                json!({"id":"take","duration":1,"layers":[{"id":"low","clip":"low"},{"id":"high","clip":"high","weight":0.5}],
            "tracks":[{"target":{"type":"object","id":"b"},"keys":[{"time":0,"translation":[0,-3,0]}]}]}),
            ),
        ],
    );
    let edit = |clip: &str, action: Value| json!({"op":"edit_layer","request":{"clip":clip,"action":action}});
    let document = |editor: &Editor| serde_json::to_value(editor.document()).unwrap();
    let original = document(&editor);
    near(&object(&pose(&mut editor, "take", 0.5), "a")["position"], [6.0, 0.0, 0.0]);
    let upsert = edit("take", json!({"op":"upsert","layer":{"id":"low","clip":"low","weight":0.5}}));
    let revision = editor.revision();
    let preview = send(&mut editor, json!({"op":"apply","dry_run":true,"operations":[upsert.clone()]}), true);
    assert_eq!(preview["committed"], false);
    assert_eq!(editor.revision(), revision);
    assert_eq!(document(&editor), original);
    apply(&mut editor, vec![upsert]);
    let updated = document(&editor);
    assert_eq!(updated["clips"][0], original["clips"][0]);
    assert_eq!(updated["clips"][1], original["clips"][1]);
    assert_eq!(updated["clips"][2]["tracks"], original["clips"][2]["tracks"]);
    assert_eq!(updated["clips"][2]["layers"][0]["id"], "low");
    assert_eq!(updated["clips"][2]["layers"][1]["id"], "high");
    near(&object(&pose(&mut editor, "take", 0.5), "a")["position"], [5.5, 0.0, 0.0]);
    apply(&mut editor, vec![edit("take", json!({"op":"move","id":"low","index":1}))]);
    let reordered = document(&editor);
    let observed = pose(&mut editor, "take", 0.5);
    near(&object(&observed, "a")["position"], [3.5, 0.0, 0.0]);
    near(&object(&observed, "b")["position"], [0.0, -3.0, 0.0]);
    assert!(sample(&mut editor, "a", [3.5, 0.0, 0.0], "take", 0.5) < -0.19);
    send(&mut editor, json!({"op":"undo"}), true);
    assert_eq!(document(&editor), updated);
    near(&object(&pose(&mut editor, "take", 0.5), "a")["position"], [5.5, 0.0, 0.0]);
    send(&mut editor, json!({"op":"redo"}), true);
    assert_eq!(document(&editor), reordered);
    near(&object(&pose(&mut editor, "take", 0.5), "a")["position"], [3.5, 0.0, 0.0]);
    let revision = editor.revision();
    for invalid in [
        edit("missing", json!({"op":"upsert","layer":{"id":"new","clip":"low"}})),
        edit("take", json!({"op":"upsert","layer":{"id":"low","clip":"missing"}})),
        edit("take", json!({"op":"upsert","layer":{"id":"cycle","clip":"take"}})),
        edit("take", json!({"op":"move","id":"low","index":2})),
        edit("take", json!({"op":"move","id":"missing","index":0})),
        edit("take", json!({"op":"remove","id":"missing"})),
    ] {
        send(
            &mut editor,
            json!({"op":"apply","operations":[{"op":"translate","ids":["b"],"delta":[0,2,0]},invalid]}),
            false,
        );
        assert_eq!(editor.revision(), revision);
        assert_eq!(document(&editor), reordered);
    }
    apply(&mut editor, vec![edit("take", json!({"op":"remove","id":"low"}))]);
    near(&object(&pose(&mut editor, "take", 0.5), "a")["position"], [5.0, 0.0, 0.0]);
    assert_eq!(document(&editor)["clips"][2]["tracks"], original["clips"][2]["tracks"]);
    send(&mut editor, json!({"op":"undo"}), true);
    assert_eq!(document(&editor), reordered);
}

#[test]
fn full_weight_additive_scale_ratio_remains_positive_across_authored_dynamic_range() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            sphere("orb", [0.0; 3]),
            put(
                json!({"id":"large","duration":1,"tracks":[{"target":{"type":"object","id":"orb"},"keys":[{"time":0,"scale":1000}]}]}),
            ),
            put(
                json!({"id":"swing","duration":1,"tracks":[{"target":{"type":"object","id":"orb"},"keys":[{"time":0,"scale":1000},{"time":1,"scale":0.001}]}]}),
            ),
            put(
                json!({"id":"base","duration":1,"layers":[{"id":"one","clip":"large"},{"id":"two","clip":"large","mode":"additive"}]}),
            ),
            put(
                json!({"id":"detail","duration":1,"layers":[{"id":"one","clip":"swing"},{"id":"two","clip":"swing","mode":"additive"}]}),
            ),
            put(
                json!({"id":"take","duration":1,"layers":[{"id":"base","clip":"base"},{"id":"detail","clip":"detail","mode":"additive","reference_time":0}]}),
            ),
        ],
    );
    // Both source endpoints and the expected 1e-6 output lie within evaluated
    // scale limits. Computing 1+(ratio-1) in f32 instead destroys the tiny ratio.
    let detail = pose(&mut editor, "detail", 1.0);
    let source_scale = object(&detail, "orb")["scale"].as_f64().unwrap();
    let result = pose(&mut editor, "take", 1.0);
    let actual = object(&result, "orb")["scale"].as_f64().unwrap();
    assert!((actual - source_scale).abs() / source_scale < 1e-5, "{actual} != {source_scale}");
    // Also exercise the ratio arithmetic when base and reference differ, so an
    // exact-equality fast path cannot hide cancellation in the general blend.
    apply(
        &mut editor,
        vec![
            json!({"op":"edit_layer","request":{"clip":"base","action":{"op":"upsert","layer":{"id":"two","clip":"large","mode":"additive","weight":0.5}}}}),
        ],
    );
    let base = object(&pose(&mut editor, "base", 1.0), "orb")["scale"].as_f64().unwrap();
    let reference = object(&pose(&mut editor, "detail", 0.0), "orb")["scale"].as_f64().unwrap();
    let actual = object(&pose(&mut editor, "take", 1.0), "orb")["scale"].as_f64().unwrap();
    let expected = base * source_scale / reference;
    assert!((actual - expected).abs() / expected < 1e-5, "{actual} != {expected}");
    let weight = f32::from_bits(1.0f32.to_bits() - 1);
    apply(
        &mut editor,
        vec![put(
            json!({"id":"near","duration":1,"layers":[{"id":"base","clip":"base"},{"id":"detail","clip":"detail","weight":weight}]}),
        )],
    );
    let actual = object(&pose(&mut editor, "near", 1.0), "orb")["scale"].as_f64().unwrap();
    let expected = base * (1.0 - f64::from(weight)) + source_scale * f64::from(weight);
    assert!((actual - expected).abs() / expected < 1e-5, "{actual} != {expected}");
}
