//! Joint policy at authoring, layered FK, deformation, cloth and delivery boundaries.
use mm3e_editor::{
    animation::{self, AnimationSample, Playback},
    deform,
    model::{vec, Pass},
    protocol::Request,
    Editor,
};
use mm3e_orchestrator::Prim;
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
            "mm3e-joint-limits-{}-{}-{}",
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
        serde_json::from_value(json!({"id":"limits-test","expected_revision":editor.revision(),"command":command}))
            .unwrap();
    let response = editor.handle(request);
    assert_eq!(response.ok, okay, "{response:?}");
    response.result.unwrap_or_else(|| serde_json::to_value(response.error).unwrap())
}
fn apply(editor: &mut Editor, operations: Vec<Value>) {
    send(editor, json!({"op":"apply","operations":operations}), true);
}
fn put(clip: Value) -> Value {
    json!({"op":"put_clip","clip":clip})
}
fn sample(clip: &str, time: f32) -> AnimationSample {
    AnimationSample { clip: clip.into(), time, playback: Playback::Clamp }
}
fn pose(editor: &mut Editor, clip: &str, time: f32) -> Value {
    send(editor, json!({"op":"pose","animation":sample(clip,time)}), true)
}
fn hinge(mode: &str, max: f64) -> Value {
    json!({"type":"hinge","axis":[0,0,1],"min_degrees":-max,"max_degrees":max,"mode":mode})
}
fn set_limit(editor: &mut Editor, id: &str, limit: Value) {
    apply(editor, vec![json!({"op":"set_joint_limit","id":id,"limit":limit})]);
}

fn near(value: &Value, expected: [f64; 3]) {
    for (i, x) in expected.into_iter().enumerate() {
        assert!((value[i].as_f64().unwrap() - x).abs() < 3e-5, "{value} != {expected:?}");
    }
}
fn near_vertices(a: &Value, b: &Value) {
    let a: Vec<[f32; 3]> = serde_json::from_value(a["vertices"].clone()).unwrap();
    let b: Vec<[f32; 3]> = serde_json::from_value(b["vertices"].clone()).unwrap();
    assert_eq!(a.len(), b.len());
    for (a, b) in a.into_iter().zip(b) {
        assert!((vec(a) - vec(b)).length() < 3e-6, "{a:?} != {b:?}");
    }
}
fn basic(editor: &mut Editor) {
    apply(
        editor,
        vec![
            json!({"op":"create","object":{"id":"orb","position":[0.7,0,0],"shape":{"type":"sphere","radius":0.2}}}),
            json!({"op":"set_joints","joints":[{"id":"joint","pivot":[0,0,0],"objects":["orb"]}]}),
            json!({"op":"set_camera","camera":{"eye":[0,0,3],"target":[0,0,0],"fov_degrees":40}}),
            json!({"op":"set_settings","settings":{"width":24,"height":24,"quality":"preview","shadows":false,"ao":false}}),
            put(
                json!({"id":"motion","duration":1,"tracks":[{"target":{"type":"joint","id":"joint"},"keys":[{"time":0},{"time":1,"rotation_degrees":[0,0,90]}]}]}),
            ),
        ],
    );
}

#[test]
fn absent_and_admitted_limits_preserve_exact_pose_pixels_and_legacy_serialization() {
    let root = Root::new();
    let mut editor = root.editor();
    basic(&mut editor);
    let legacy = serde_json::to_value(editor.document()).unwrap();
    assert!(legacy["joints"][0].get("rotation_limit").is_none());
    let before = pose(&mut editor, "motion", 0.25);
    assert!(before.get("joint_limits").is_none());
    send(&mut editor, json!({"op":"render","path":"before.png","animation":sample("motion",0.25)}), true);
    send(&mut editor, json!({"op":"save","path":"legacy.json"}), true);
    set_limit(&mut editor, "joint", hinge("reject", 45.0));
    let after = pose(&mut editor, "motion", 0.25);
    assert_eq!(before["objects"], after["objects"]);
    assert_eq!(before["joints"], after["joints"]);
    assert_eq!(after["joint_limits"][0]["violated"], false);
    send(&mut editor, json!({"op":"render","path":"admitted.png","animation":sample("motion",0.25)}), true);
    assert_eq!(fs::read(root.0.join("before.png")).unwrap(), fs::read(root.0.join("admitted.png")).unwrap());
    let stored = serde_json::to_value(editor.document()).unwrap();
    send(&mut editor, json!({"op":"save","path":"limited.json"}), true);
    let mut loaded = root.editor();
    send(&mut loaded, json!({"op":"load","path":"limited.json"}), true);
    assert_eq!(serde_json::to_value(loaded.document()).unwrap(), stored);
    assert_eq!(pose(&mut loaded, "motion", 0.25), after);
    apply(&mut loaded, vec![json!({"op":"clear_joint_limit","id":"joint"})]);
    assert_eq!(serde_json::to_value(loaded.document()).unwrap(), legacy);
    assert_eq!(pose(&mut loaded, "motion", 0.25), before);
    send(&mut loaded, json!({"op":"load","path":"legacy.json"}), true);
    assert_eq!(serde_json::to_value(loaded.document()).unwrap(), legacy);
    assert_eq!(pose(&mut loaded, "motion", 0.25), before);
}

#[test]
fn limits_measure_final_layered_rotation_and_local_keys_remain_authoritative() {
    let root = Root::new();
    let mut editor = root.editor();
    basic(&mut editor);
    apply(
        &mut editor,
        vec![
            put(
                json!({"id":"eighty","duration":1,"tracks":[{"target":{"type":"joint","id":"joint"},"keys":[{"time":0,"rotation_degrees":[0,0,80]}]}]}),
            ),
            put(
                json!({"id":"thirty","duration":1,"tracks":[{"target":{"type":"joint","id":"joint"},"keys":[{"time":0,"rotation_degrees":[0,0,30]}]}]}),
            ),
            put(json!({"id":"half","duration":1,"layers":[{"id":"source","clip":"eighty","weight":0.5}]})),
            put(
                json!({"id":"sixty","duration":1,"layers":[{"id":"one","clip":"thirty"},{"id":"two","clip":"thirty","mode":"additive"}]}),
            ),
            put(
                json!({"id":"override","duration":1,"layers":[{"id":"source","clip":"sixty"}],"tracks":[{"target":{"type":"joint","id":"joint"},"keys":[{"time":0,"rotation_degrees":[0,0,20]}]}]}),
            ),
        ],
    );
    set_limit(&mut editor, "joint", hinge("reject", 45.0));
    let half = pose(&mut editor, "half", 0.5);
    let angle = 40.0_f64.to_radians();
    near(&half["objects"][0]["position"], [0.7 * angle.cos(), 0.7 * angle.sin(), 0.0]);
    assert_eq!(half["joint_limits"][0]["violated"], false);
    send(&mut editor, json!({"op":"pose","animation":sample("eighty",0.5)}), false);
    send(&mut editor, json!({"op":"pose","animation":sample("sixty",0.5)}), false);
    let report = send(&mut editor, json!({"op":"joint_limit_state","animation":sample("sixty",0.5)}), true);
    assert_eq!(report["joints"][0]["would_reject"], true);
    assert!((report["joints"][0]["requested_twist_degrees"].as_f64().unwrap() - 60.0).abs() < 1e-4);
    let twenty = pose(&mut editor, "override", 0.5);
    let angle = 20.0_f64.to_radians();
    near(&twenty["objects"][0]["position"], [0.7 * angle.cos(), 0.7 * angle.sin(), 0.0]);
    let keys = serde_json::to_value(&editor.document().clips).unwrap();
    set_limit(&mut editor, "joint", hinge("project", 45.0));
    let projected = pose(&mut editor, "sixty", 0.5);
    let angle = 45.0_f64.to_radians();
    near(&projected["objects"][0]["position"], [0.7 * angle.cos(), 0.7 * angle.sin(), 0.0]);
    assert_eq!(projected["joint_limits"][0]["applied"], true);
    assert_eq!(serde_json::to_value(&editor.document().clips).unwrap(), keys);
}

#[test]
fn swing_twist_projection_reaches_the_same_geometry_as_explicit_component_limits() {
    let root = Root::new();
    let mut editor = root.editor();
    basic(&mut editor);
    apply(
        &mut editor,
        vec![
            put(
                json!({"id":"swing","duration":1,"tracks":[{"target":{"type":"joint","id":"joint"},"keys":[{"time":0,"rotation_degrees":[60,0,30]}]}]}),
            ),
            put(
                json!({"id":"reference","duration":1,"tracks":[{"target":{"type":"joint","id":"joint"},"keys":[{"time":0,"rotation_degrees":[30,0,10]}]}]}),
            ),
        ],
    );
    let mut limit =
        json!({"type":"swing_twist","axis":[0,0,1],"swing_degrees":30,"twist_min_degrees":-10,"twist_max_degrees":10});
    set_limit(&mut editor, "joint", limit.clone());
    send(&mut editor, json!({"op":"pose","animation":sample("swing",0.0)}), false);
    limit["mode"] = json!("project");
    set_limit(&mut editor, "joint", limit);
    let projected = pose(&mut editor, "swing", 0.0);
    let reference = pose(&mut editor, "reference", 0.0);
    let position = &projected["objects"][0]["position"];
    let swing = 30.0_f64.to_radians();
    let twist = 10.0_f64.to_radians();
    near(position, [0.7 * twist.cos(), 0.7 * twist.sin() * swing.cos(), 0.7 * twist.sin() * swing.sin()]);
    for i in 0..3 {
        let want: [f64; 3] = serde_json::from_value(reference["objects"][0]["basis"][i].clone()).unwrap();
        near(&projected["objects"][0]["basis"][i], want);
    }
    let diagnostic = &projected["joint_limits"][0];
    assert!((diagnostic["requested_swing_degrees"].as_f64().unwrap() - 60.0).abs() < 1e-4);
    assert!((diagnostic["requested_twist_degrees"].as_f64().unwrap() - 30.0).abs() < 1e-4);
    let actual =
        send(&mut editor, json!({"op":"sample","id":"orb","points":[position],"animation":sample("swing",0.0)}), true);
    assert!(actual["samples"][0]["value"].as_f64().unwrap() < -0.19);
    assert_eq!(reference["joint_limits"][0]["applied"], false);
}

#[test]
fn projection_preserves_offset_anchor_translation_and_scale_before_noncommuting_parent_fk() {
    let root = Root::new();
    let mut editor = root.editor();
    apply(
        &mut editor,
        vec![
            json!({"op":"create","object":{"id":"hand","position":[2,0,0],"shape":{"type":"sphere","radius":0.1}}}),
            json!({"op":"set_joints","joints":[{"id":"child","parent":"parent","pivot":[1,0,0],"objects":["hand"],"rotation_limit":hinge("project",45.0)},{"id":"parent","pivot":[0,0,0]}]}),
            put(json!({"id":"motion","duration":1,"tracks":[
            {"target":{"type":"joint","id":"parent"},"keys":[{"time":0,"rotation_degrees":[0,90,0],"translation":[0,1,0],"scale":2}]},
            {"target":{"type":"joint","id":"child"},"keys":[{"time":0,"rotation_degrees":[0,0,90],"translation":[0.25,0.5,0],"scale":2}]}]})),
        ],
    );
    let before = serde_json::to_value(editor.document()).unwrap();
    let value = pose(&mut editor, "motion", 0.5);
    near(&value["joints"][0]["world_pivot"], [0.0, 2.0, -2.5]);
    let s = 2.0_f64.sqrt();
    let p = [0.0, 2.0 * (0.5 + s) + 1.0, -2.0 * (1.25 + s)];
    near(&value["objects"][0]["position"], p);
    assert_eq!(value["objects"][0]["scale"], 4.0);
    let measured =
        send(&mut editor, json!({"op":"sample","id":"hand","points":[p],"animation":sample("motion",0.5)}), true);
    assert!(measured["samples"][0]["value"].as_f64().unwrap() < -0.39);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), before);
}

fn jaw(editor: &mut Editor) {
    apply(
        editor,
        vec![
            json!({"op":"create","object":{"id":"jaw-skin","shape":{"type":"surface","vertices":[[-0.3,1,0.2],[0.3,1,0.2],[-0.3,0.8,0.2],[0.3,0.8,0.2],[-0.3,0.6,0.2],[0.3,0.6,0.2]],"triangles":[[0,2,1],[1,2,3],[2,4,3],[3,4,5]],"thickness_m":0.02}}}),
            json!({"op":"set_joints","joints":[{"id":"jaw","parent":"head","pivot":[0,0.8,0]},{"id":"head","pivot":[0,1,0]}]}),
            json!({"op":"bind_surface","request":{"deformer":{"id":"skin","object":"jaw-skin","joints":["head","jaw"],"weights":[[{"joint":0,"weight":1}],[{"joint":0,"weight":1}],[{"joint":0,"weight":0.5},{"joint":1,"weight":0.5}],[{"joint":0,"weight":0.5},{"joint":1,"weight":0.5}],[{"joint":1,"weight":1}],[{"joint":1,"weight":1}]]}}}),
            put(json!({"id":"source","duration":1,"tracks":[
            {"target":{"type":"joint","id":"head"},"keys":[{"time":0,"rotation_degrees":[0,20,0]}]},
            {"target":{"type":"joint","id":"jaw"},"keys":[{"time":0},{"time":1,"rotation_degrees":[60,0,0]}]}]})),
            put(json!({"id":"take","duration":1,"layers":[{"id":"body","clip":"source"}]})),
            put(json!({"id":"reference","duration":1,"tracks":[
            {"target":{"type":"joint","id":"head"},"keys":[{"time":0,"rotation_degrees":[0,20,0]}]},
            {"target":{"type":"joint","id":"jaw"},"keys":[{"time":0,"rotation_degrees":[30,0,0]}]}]})),
        ],
    );
}

#[test]
fn weighted_jaw_projection_reaches_surface_field_and_invalidates_supplied_evaluation() {
    let root = Root::new();
    let mut editor = root.editor();
    jaw(&mut editor);
    let at = sample("take", 1.0);
    let prior = animation::evaluate_joints(editor.document(), &at).unwrap();
    let raw = send(&mut editor, json!({"op":"deformer_state","id":"skin","animation":at}), true);
    set_limit(
        &mut editor,
        "jaw",
        json!({"type":"hinge","axis":[1,0,0],"min_degrees":0,"max_degrees":30,"mode":"project"}),
    );
    let expected =
        send(&mut editor, json!({"op":"deformer_state","id":"skin","animation":sample("reference",1.0)}), true);
    let actual = send(&mut editor, json!({"op":"deformer_state","id":"skin","animation":at}), true);
    near_vertices(&actual, &expected);
    assert_ne!(actual["vertices"], raw["vertices"]);
    let points = actual["vertices"].clone();
    let measured = send(&mut editor, json!({"op":"sample","id":"jaw-skin","points":points,"animation":at}), true);
    for p in measured["samples"].as_array().unwrap() {
        assert!(p["value"].as_f64().unwrap() < -0.009);
    }
    let (mut scene, _) = editor.document().compile(&Pass::Beauty).unwrap();
    let Prim::Surface { id } = scene.objects[0].prim else { panic!("surface fixture") };
    let before = scene.surfaces[id as usize].vertices().to_vec();
    let error = deform::refresh(editor.document(), &mut scene, Some(&at), Some(&prior)).unwrap_err();
    assert!(error.contains("authored inputs"), "{error}");
    assert_eq!(scene.surfaces[id as usize].vertices(), before);
    let current = animation::evaluate_joints(editor.document(), &at).unwrap();
    deform::refresh(editor.document(), &mut scene, Some(&at), Some(&current)).unwrap();
    assert_ne!(scene.surfaces[id as usize].vertices(), before);
}

#[test]
fn relevant_limit_changes_stale_cloth_and_rebaked_pins_follow_projected_motion() {
    let root = Root::new();
    let mut editor = root.editor();
    let dt = 1.0_f32 / 24.0;
    let duration = 4.0 * dt;
    apply(
        &mut editor,
        vec![
            json!({"op":"create","object":{"id":"anchor","shape":{"type":"sphere","radius":0.05}}}),
            json!({"op":"set_joints","joints":[{"id":"driver","pivot":[0,0,0],"objects":["anchor"]},{"id":"unrelated","pivot":[8,0,0]}]}),
            put(
                json!({"id":"motion","duration":duration,"tracks":[{"target":{"type":"joint","id":"driver"},"keys":[{"time":0},{"time":duration,"rotation_degrees":[0,0,20]}]}]}),
            ),
            json!({"op":"create_cloth_panel","request":{"id":"cloth","origin":[0.25,1,0.25],"axis_u":[1,0,0],"axis_v":[0,0,1],"segments":[1,1],"width_m":0.5,"height_m":0.5,"thickness_m":0.004,"vertex_mass_kg":0.02,
            "pins":[{"vertex":0,"target_object":"anchor","point":[0,1,0]},{"vertex":1,"target_object":"anchor","point":[0.5,1,0]},{"vertex":2,"target_object":"anchor","point":[0,1,0.5]},{"vertex":3,"target_object":"anchor","point":[0.5,1,0.5]}],
            "settings":{"fixed_dt":dt,"substeps":2,"iterations":2,"gravity":[0,0,0],"self_collision":false}}}),
        ],
    );
    let bake = json!({"op":"bake_cloth","request":{"id":"cloth","clip":"motion"}});
    send(&mut editor, bake.clone(), true);
    let cached = serde_json::to_value(&editor.document().cloths).unwrap();
    set_limit(&mut editor, "unrelated", hinge("project", 5.0));
    assert_eq!(send(&mut editor, json!({"op":"cloth_state","id":"cloth"}), true)["cache"]["fresh"], true);
    assert_eq!(serde_json::to_value(&editor.document().cloths).unwrap(), cached);
    set_limit(&mut editor, "driver", hinge("project", 10.0));
    assert_eq!(send(&mut editor, json!({"op":"cloth_state","id":"cloth"}), true)["cache"]["fresh"], false);
    send(&mut editor, json!({"op":"cloth_state","id":"cloth","animation":sample("motion",duration)}), false);
    send(&mut editor, bake, true);
    let state = send(&mut editor, json!({"op":"cloth_state","id":"cloth","animation":sample("motion",duration)}), true);
    assert_eq!(state["cache"]["fresh"], true);
    let angle = 10.0_f64.to_radians();
    for pin in state["pins"].as_array().unwrap() {
        let p = &pin["point"];
        let x = p[0].as_f64().unwrap();
        let y = p[1].as_f64().unwrap();
        near(
            &state["vertices"][pin["vertex"].as_u64().unwrap() as usize],
            [x * angle.cos() - y * angle.sin(), x * angle.sin() + y * angle.cos(), p[2].as_f64().unwrap()],
        );
    }
    assert_ne!(serde_json::to_value(&editor.document().cloths).unwrap(), cached);
}

#[test]
fn rejected_later_frames_and_shutter_subsamples_fail_before_any_output() {
    let root = Root::new();
    let mut editor = root.editor();
    basic(&mut editor);
    set_limit(&mut editor, "joint", hinge("reject", 30.0));
    pose(&mut editor, "motion", 0.0);
    let before = serde_json::to_value(editor.document()).unwrap();
    let revision = editor.revision();
    for (op, directory) in [("render_sequence", "sequence"), ("create_render_job", "job")] {
        let mut request = json!({"directory":directory,"clip":"motion","start":0,"end":1,"fps":2});
        if op == "create_render_job" {
            request["type"] = json!("sequence");
        }
        send(&mut editor, json!({"op":op,"request":request}), false);
        assert!(!root.0.join(directory).exists());
    }
    assert_eq!(editor.revision(), revision);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), before);
    apply(
        &mut editor,
        vec![
            json!({"op":"set_settings","settings":{"width":24,"height":24,"quality":"preview","shadows":false,"ao":false,
        "film":{"shutter_open_seconds":0,"shutter_close_seconds":1,"shutter_samples":2}}}),
        ],
    );
    send(&mut editor, json!({"op":"render","path":"shutter.png","animation":sample("motion",0.0)}), false);
    assert!(!root.0.join("shutter.png").exists());
}

#[test]
fn frozen_projected_jobs_resume_with_original_limits_after_live_limit_edits() {
    let root = Root::new();
    let mut editor = root.editor();
    basic(&mut editor);
    set_limit(&mut editor, "joint", hinge("project", 30.0));
    send(
        &mut editor,
        json!({"op":"render_sequence","request":{"directory":"reference","clip":"motion","start":0,"end":1,"fps":2}}),
        true,
    );
    send(
        &mut editor,
        json!({"op":"create_render_job","request":{"type":"sequence","directory":"job","clip":"motion","start":0,"end":1,"fps":2}}),
        true,
    );
    assert_eq!(
        send(&mut editor, json!({"op":"step_render_job","directory":"job","max_frames":1}), true)["completed_frames"],
        1
    );
    let first = fs::read(root.0.join("job/frame_0000.png")).unwrap();
    set_limit(&mut editor, "joint", hinge("project", 10.0));
    send(&mut editor, json!({"op":"render","path":"live.png","animation":sample("motion",1.0)}), true);
    assert_ne!(fs::read(root.0.join("live.png")).unwrap(), fs::read(root.0.join("reference/frame_0002.png")).unwrap());
    drop(editor);
    let mut restarted = root.editor();
    assert!(restarted.document().joints.is_empty());
    assert_eq!(
        send(&mut restarted, json!({"op":"step_render_job","directory":"job","max_frames":2}), true)["status"],
        "complete"
    );
    assert_eq!(fs::read(root.0.join("job/frame_0000.png")).unwrap(), first);
    for i in 0..3 {
        assert_eq!(
            fs::read(root.0.join(format!("job/frame_{i:04}.png"))).unwrap(),
            fs::read(root.0.join(format!("reference/frame_{i:04}.png"))).unwrap()
        );
    }
    assert_eq!(
        send(&mut restarted, json!({"op":"render_job_state","directory":"job","verify_outputs":true}), true)["status"],
        "complete"
    );
}

#[test]
fn malformed_limit_transaction_preserves_scene_revision_and_original_joint_data() {
    let root = Root::new();
    let mut editor = root.editor();
    basic(&mut editor);
    let before = serde_json::to_value(editor.document()).unwrap();
    let revision = editor.revision();
    for limit in [
        json!({"type":"hinge","axis":[0,0,0],"min_degrees":-10,"max_degrees":10}),
        json!({"type":"hinge","axis":[0,0,1],"min_degrees":30,"max_degrees":10}),
        json!({"type":"swing_twist","axis":[0,0,1],"swing_degrees":-1,"twist_min_degrees":-20,"twist_max_degrees":20}),
    ] {
        let mut joints = before["joints"].clone();
        joints[0]["rotation_limit"] = limit;
        send(
            &mut editor,
            json!({"op":"apply","operations":[{"op":"translate","ids":["orb"],"delta":[1,0,0]},{"op":"set_joints","joints":joints}]}),
            false,
        );
        assert_eq!(editor.revision(), revision);
        assert_eq!(serde_json::to_value(editor.document()).unwrap(), before);
    }
}
