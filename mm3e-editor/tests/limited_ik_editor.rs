//! Constrained IK through native authoring, geometry, persistence and cloth dependencies.
use mm3e_editor::{
    animation::{self, AnimationSample, Playback, Target},
    cloth::{self, BakeClothRequest, ClothPanelRequest},
    ik::{self, IkRequest, LimitPolicy, UnreachablePolicy},
    joint_limits,
    model::{array, vec, Document, Pass},
    protocol::Request,
    Editor,
};
use mm3e_kit::{Quat, Vec3};
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mm3e-limited-ik-editor-{}-{}",
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

fn hinge(min: f64, max: f64) -> Value {
    json!({"type":"hinge","axis":[0,0,1],"min_degrees":min,"max_degrees":max})
}
fn document() -> Document {
    Document {
        objects: serde_json::from_value(json!([
            {"id":"upper","shape":{"type":"capsule","a":[0,0,0],"b":[1,0,0],"radius":0.04}},
            {"id":"lower","position":[1,0,0],"shape":{"type":"capsule","a":[0,0,0],"b":[1,0,0],"radius":0.04}},
            {"id":"hand","position":[2,0,0],"shape":{"type":"sphere","radius":0.08}}
        ]))
        .unwrap(),
        joints: serde_json::from_value(json!([
            {"id":"root","pivot":[0,0,0],"objects":["upper"],"rotation_limit":hinge(-90.0,90.0)},
            {"id":"middle","parent":"root","pivot":[1,0,0],"objects":["lower"],"rotation_limit":hinge(0.0,150.0)},
            {"id":"tip","parent":"middle","pivot":[2,0,0],"objects":["hand"]}
        ]))
        .unwrap(),
        clips: serde_json::from_value(json!([{"id":"reach","duration":1}])).unwrap(),
        ..Document::default()
    }
}
fn sample() -> AnimationSample {
    AnimationSample { clip: "reach".into(), time: 0.5, playback: Playback::Clamp }
}
fn request() -> IkRequest {
    serde_json::from_value(
        json!({"clip":"reach","time":0.5,"root_joint":"root","middle_joint":"middle","tip_joint":"tip",
        "target":[1,1,0],"pole":[1,0,0]}),
    )
    .unwrap()
}
fn point(document: &Document, id: &str) -> Vec3 {
    let index = document.joints.iter().position(|j| j.id == id).unwrap();
    animation::evaluate_joints(document, &sample()).unwrap().joint_transforms[index]
        .to_world(vec(document.joints[index].pivot))
}
fn near(a: Vec3, b: Vec3, tolerance: f32) {
    assert!((a - b).length() <= tolerance, "{a:?} != {b:?}");
}
fn verify_geometry(document: &Document, result: &Value) {
    let tip = point(document, "tip");
    near(tip, vec(serde_json::from_value(result["achieved_tip"].clone()).unwrap()), 1e-5);
    let scene = document.compile_at(&Pass::Beauty, Some(&sample())).unwrap().0;
    let index = document.objects.iter().position(|o| o.id == "hand").unwrap();
    assert!(scene.sample_object(index, tip).unwrap().dist < -0.06);
    let state = joint_limits::inspect(document, &sample()).unwrap();
    for joint in state["joints"].as_array().unwrap() {
        assert_eq!(joint["would_reject"], false, "{joint}");
    }
    assert!(result["root_drift_m"].as_f64().unwrap() < 1e-4);
    for i in 0..2 {
        assert!(
            (result["segment_lengths_m"][i].as_f64().unwrap()
                - result["achieved_segment_lengths_m"][i].as_f64().unwrap())
            .abs()
                < 1e-4
        );
    }
}
fn send(editor: &mut Editor, command: Value, okay: bool) -> Value {
    let response = editor.handle(
        serde_json::from_value::<Request>(
            json!({"id":"limited-ik-test","expected_revision":editor.revision(),"command":command}),
        )
        .unwrap(),
    );
    assert_eq!(response.ok, okay, "{response:?}");
    response.result.unwrap_or_else(|| serde_json::to_value(response.error).unwrap())
}

#[test]
fn exact_constrained_authoring_supports_dry_run_history_native_reload_and_real_field() {
    let scratch = Scratch::new();
    let mut editor = Editor::new(&scratch.0).unwrap();
    let source = document();
    let mut operations: Vec<_> = source.objects.iter().map(|o| json!({"op":"create","object":o})).collect();
    operations.push(json!({"op":"set_joints","joints":source.joints}));
    operations.push(json!({"op":"put_clip","clip":source.clips[0]}));
    send(&mut editor, json!({"op":"apply","operations":operations}), true);
    let original = serde_json::to_value(editor.document()).unwrap();
    let revision = editor.revision();
    send(&mut editor, json!({"op":"solve_ik","request":request(),"dry_run":true}), true);
    assert_eq!(editor.revision(), revision);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), original);
    let result = send(&mut editor, json!({"op":"solve_ik","request":request()}), true);
    assert_eq!(result["limit_search"]["within_tolerance"], true);
    assert_eq!(result["limit_search"]["approximation_accepted"], false);
    near(point(editor.document(), "middle"), Vec3::new(1.0, 0.0, 0.0), 1e-5);
    near(point(editor.document(), "tip"), Vec3::new(1.0, 1.0, 0.0), 1e-5);
    verify_geometry(editor.document(), &result);
    let modified = serde_json::to_value(editor.document()).unwrap();
    let pose = send(&mut editor, json!({"op":"pose","animation":sample()}), true);
    send(&mut editor, json!({"op":"save","path":"limited.json"}), true);
    send(&mut editor, json!({"op":"undo"}), true);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), original);
    send(&mut editor, json!({"op":"redo"}), true);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), modified);
    let mut reloaded = Editor::new(&scratch.0).unwrap();
    send(&mut reloaded, json!({"op":"load","path":"limited.json"}), true);
    assert_eq!(serde_json::to_value(reloaded.document()).unwrap(), modified);
    assert_eq!(send(&mut reloaded, json!({"op":"pose","animation":sample()}), true), pose);
    verify_geometry(reloaded.document(), &result);
}

#[test]
fn impossible_pole_rejects_atomically_and_explicit_approximation_retains_residuals() {
    let mut document = document();
    let original = serde_json::to_value(&document).unwrap();
    let mut request = request();
    request.pole = [0.0, 1.0, 0.0];
    request.limit_evaluations = 256;
    let error = ik::solve(&mut document, &request).unwrap_err();
    assert!(error.contains("no verified constrained IK"), "{error}");
    assert!(error.contains("not a proof"));
    assert_eq!(serde_json::to_value(&document).unwrap(), original);
    request.limit_policy = LimitPolicy::BestFeasible;
    let result = ik::solve(&mut document, &request).unwrap();
    assert_eq!(result["clamped"], false);
    assert_eq!(result["effective_target"], json!([1.0, 1.0, 0.0]));
    assert_eq!(result["limit_search"]["within_tolerance"], false);
    assert_eq!(result["limit_search"]["approximation_accepted"], true);
    assert!(result["middle_error_m"].as_f64().unwrap() > 0.01 || result["target_error_m"].as_f64().unwrap() > 0.01);
    verify_geometry(&document, &result);
}

#[test]
fn radial_clamping_and_angular_approximation_are_separate_explicit_policies() {
    let mut document = document();
    document.joints[0].rotation_limit = Some(serde_json::from_value(hinge(-10.0, 10.0)).unwrap());
    document.joints[1].rotation_limit = Some(serde_json::from_value(hinge(0.0, 10.0)).unwrap());
    let original = serde_json::to_value(&document).unwrap();
    let mut request = request();
    request.target = [0.0, 3.0, 0.0];
    request.pole = [0.0, 1.0, 0.0];
    request.limit_policy = LimitPolicy::BestFeasible;
    request.limit_evaluations = 256;
    assert!(ik::solve(&mut document, &request).unwrap_err().contains("outside reach"));
    assert_eq!(serde_json::to_value(&document).unwrap(), original);
    request.unreachable = UnreachablePolicy::Clamp;
    let result = ik::solve(&mut document, &request).unwrap();
    assert_eq!(result["clamped"], true);
    assert_eq!(result["effective_target"], json!([0.0, 2.0, 0.0]));
    assert_eq!(result["limit_search"]["approximation_accepted"], true);
    assert!(result["target_error_m"].as_f64().unwrap() > 1.0);
    assert_ne!(result["achieved_tip"], result["effective_target"]);
    verify_geometry(&document, &result);
}

#[test]
fn locked_tip_orientation_is_achieved_by_upstream_non_cartesian_bone_twist() {
    let mut document = document();
    let angles = [20.0_f32, 30.0, 10.0];
    let radians = angles.map(f32::to_radians);
    let desired = Quat::from_euler(radians[0], radians[1], radians[2]);
    let axis = Vec3::new(desired.x, desired.y, desired.z).normalize();
    document.joints[1].pivot = array(axis);
    document.joints[2].pivot = array(axis.scale(2.0));
    document.objects[1].position = array(axis);
    document.objects[2].position = array(axis.scale(2.0));
    document.joints[0].rotation_limit = Some(serde_json::from_value(hinge(0.0, 0.0)).unwrap());
    document.joints[1].rotation_limit = Some(
        serde_json::from_value(json!({"type":"hinge","axis":array(axis),"min_degrees":0,"max_degrees":90})).unwrap(),
    );
    document.joints[2].rotation_limit = Some(serde_json::from_value(hinge(0.0, 0.0)).unwrap());
    let mut request = request();
    request.target = array(axis.scale(2.0));
    request.pole = array(axis);
    request.tip_world_rotation_degrees = Some(angles);
    let result = ik::solve(&mut document, &request).unwrap();
    assert_eq!(result["limit_search"]["within_tolerance"], true);
    assert!(result["tip_orientation_error_degrees"].as_f64().unwrap() <= 0.1);
    verify_geometry(&document, &result);
    let tip_track = document.clips[0].tracks.iter().find(|t| t.target == Target::Joint { id: "tip".into() }).unwrap();
    assert_eq!(tip_track.keys[0].rotation_degrees, [0.0; 3]);
    let pose = animation::evaluate_joints(&document, &sample()).unwrap();
    for (actual, want) in pose.joint_transforms[2].rot.cols.into_iter().zip(desired.to_mat3().cols) {
        assert!((actual - want).length() < 0.002);
    }
}

#[test]
fn layered_translation_scale_and_source_keys_survive_constrained_rotation_authoring() {
    let mut document = document();
    document.joints.insert(0, serde_json::from_value(json!({"id":"base","pivot":[0,0,0]})).unwrap());
    document.joints[1].parent = Some("base".into());
    for joint in &mut document.joints[1..3] {
        joint.rotation_limit=Some(serde_json::from_value(json!({"type":"swing_twist","axis":[0,0,1],"swing_degrees":170,"twist_min_degrees":-170,"twist_max_degrees":170})).unwrap());
    }
    document.clips=serde_json::from_value(json!([
        {"id":"source","duration":1,"tracks":[
            {"target":{"type":"joint","id":"base"},"keys":[{"time":0,"translation":[0.1,0.2,0.05],"rotation_degrees":[0,20,0],"scale":1.2}]},
            {"target":{"type":"joint","id":"root"},"keys":[{"time":0,"translation":[0.01,-0.02,0.03],"scale":1.1}]},
            {"target":{"type":"joint","id":"middle"},"keys":[{"time":0,"translation":[0.03,0.02,0.01],"scale":0.9}]},
            {"target":{"type":"joint","id":"tip"},"keys":[{"time":0,"translation":[-0.01,0.01,0],"scale":0.8}]}
        ]},
        {"id":"reach","duration":1,"layers":[{"id":"inherited","clip":"source"}]}
    ])).unwrap();
    let before_root = point(&document, "root");
    let source = serde_json::to_value(&document.clips[0]).unwrap();
    let mut witness = document.clone();
    witness.clips[1].tracks=serde_json::from_value(json!([
        {"target":{"type":"joint","id":"root"},"keys":[{"time":0,"translation":[0.01,-0.02,0.03],"scale":1.1,"rotation_degrees":[0,0,30]}]},
        {"target":{"type":"joint","id":"middle"},"keys":[{"time":0,"translation":[0.03,0.02,0.01],"scale":0.9,"rotation_degrees":[20,0,0]}]}
    ])).unwrap();
    let mut request = request();
    request.target = array(point(&witness, "tip"));
    request.pole = array(point(&witness, "middle"));
    let result = ik::solve(&mut document, &request).unwrap();
    assert_eq!(result["limit_search"]["within_tolerance"], true);
    near(point(&document, "root"), before_root, 2e-6);
    assert_eq!(serde_json::to_value(&document.clips[0]).unwrap(), source);
    for (id, translation, scale) in [("root", [0.01, -0.02, 0.03], 1.1), ("middle", [0.03, 0.02, 0.01], 0.9)] {
        let track = document.clips[1].tracks.iter().find(|t| t.target == Target::Joint { id: id.into() }).unwrap();
        near(vec(track.keys[0].translation), vec(translation), 2e-6);
        assert_eq!(track.keys[0].scale, scale);
    }
    assert_eq!(document.clips[1].layers.len(), 1);
    verify_geometry(&document, &result);
}

fn attach_cloth(document: &mut Document) -> BakeClothRequest {
    let pins = [[-0.05, 0.3, -0.05], [0.05, 0.3, -0.05], [-0.05, 0.3, 0.05], [0.05, 0.3, 0.05]];
    let panel:ClothPanelRequest=serde_json::from_value(json!({"id":"sleeve","origin":[2,0.3,0],"axis_u":[1,0,0],"axis_v":[0,0,1],
        "segments":[1,1],"width_m":0.1,"height_m":0.1,"thickness_m":0.004,"vertex_mass_kg":0.02,
        "pins":pins.iter().enumerate().map(|(i,p)|json!({"vertex":i,"target_object":"hand","point":p})).collect::<Vec<_>>(),
        "settings":{"fixed_dt":1.0/24.0,"substeps":2,"iterations":2,"gravity":[0,0,0],"self_collision":false}})).unwrap();
    cloth::create_panel(document, &panel).unwrap();
    let bake = BakeClothRequest { id: "sleeve".into(), clip: "reach".into(), duration: None };
    cloth::bake(document, &bake).unwrap();
    bake
}

#[test]
fn first_ik_keys_hold_back_to_zero_and_rebake_rejects_changed_rest_pin_alignment_atomically() {
    let mut document = document();
    let bake = attach_cloth(&mut document);
    let result = ik::solve(&mut document, &request()).unwrap();
    assert_eq!(result["stale_cloth_caches"], json!(["sleeve"]));
    let before = serde_json::to_value(&document).unwrap();
    assert!(cloth::bake(&mut document, &bake).unwrap_err().contains("does not meet its rest vertex at clip time zero"));
    assert_eq!(serde_json::to_value(&document).unwrap(), before);
}

#[test]
fn constrained_ik_reports_stale_cloth_and_rebake_follows_the_new_hand_pose() {
    let mut document = document();
    // Preserve the authored attachment rest at zero before adding a later pose.
    document.clips[0].tracks = serde_json::from_value(json!([
        {"target":{"type":"joint","id":"root"},"keys":[{"time":0}]},
        {"target":{"type":"joint","id":"middle"},"keys":[{"time":0}]}
    ]))
    .unwrap();
    let bake = attach_cloth(&mut document);
    let frames = serde_json::to_value(&document.cloths[0].cache).unwrap();
    let result = ik::solve(&mut document, &request()).unwrap();
    assert_eq!(result["stale_cloth_caches"], json!(["sleeve"]));
    assert_eq!(serde_json::to_value(&document.cloths[0].cache).unwrap(), frames);
    assert!(document.compile_at(&Pass::Beauty, Some(&sample())).err().unwrap().contains("stale"));
    cloth::bake(&mut document, &bake).unwrap();
    assert_eq!(cloth::inspect(&document, "sleeve", None).unwrap()["cache"]["fresh"], true);
    for frame in &document.cloths[0].cache.as_ref().unwrap().frames {
        let at = AnimationSample { time: frame.time, ..sample() };
        let hand_delta = animation::evaluate_joints(&document, &at).unwrap().joint_transforms[2];
        for pin in &document.cloths[0].pins {
            let expected = hand_delta.to_world(vec(document.objects[2].position) + vec(pin.point));
            near(vec(frame.vertices[pin.vertex as usize]), expected, 1e-5);
        }
    }
    document.compile_at(&Pass::Beauty, Some(&sample())).unwrap();
}
