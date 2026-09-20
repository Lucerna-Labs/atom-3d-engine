use mm3e_editor::{
    animation::{self, AnimationSample, Playback, Target},
    ik::{self, IkRequest},
    model::{Document, Pass},
    protocol::Request,
    Editor,
};
use mm3e_kit::{Quat, Vec3};
use serde_json::json;

fn document() -> Document {
    let objects = serde_json::from_value(json!([
        {"id":"upper","shape":{"type":"capsule","a":[0,0,0],"b":[0.6,0,0],"radius":0.04},"position":[0,1,0]},
        {"id":"lower","shape":{"type":"capsule","a":[0,0,0],"b":[0.5,0,0],"radius":0.04},"position":[0.6,1,0]},
        {"id":"hand","shape":{"type":"sphere","radius":0.06},"position":[1.1,1,0]},
        {"id":"marker","shape":{"type":"sphere","radius":0.01},"position":[2,0,0]}
    ]))
    .unwrap();
    let joints = serde_json::from_value(json!([
        {"id":"base","pivot":[0,0,0]},
        {"id":"shoulder","parent":"base","pivot":[0,1,0],"objects":["upper"]},
        {"id":"elbow","parent":"shoulder","pivot":[0.6,1,0],"objects":["lower"]},
        {"id":"wrist","parent":"elbow","pivot":[1.1,1,0],"objects":["hand"]}
    ]))
    .unwrap();
    let clips=serde_json::from_value(json!([{"id":"reach","duration":1,"tracks":[
        {"target":{"type":"joint","id":"base"},"keys":[{"time":0,"translation":[0.1,0.2,0.05],"rotation_degrees":[0,20,0],"scale":1.2},{"time":1,"translation":[0.1,0.2,0.05],"rotation_degrees":[0,20,0],"scale":1.2}]},
        {"target":{"type":"joint","id":"shoulder"},"easing":"smooth_step","keys":[{"time":0,"translation":[0.01,-0.02,0.03],"rotation_degrees":[0,0,15]},{"time":1,"translation":[0.01,-0.02,0.03],"rotation_degrees":[0,0,25]}]},
        {"target":{"type":"joint","id":"elbow"},"keys":[{"time":0,"rotation_degrees":[0,0,20]},{"time":1,"rotation_degrees":[0,0,-15]}]},
        {"target":{"type":"object","id":"marker"},"keys":[{"time":0},{"time":1,"translation":[0,0.1,0]}]}
    ]}])).unwrap();
    Document { objects, joints, clips, ..Document::default() }
}
fn sample(time: f32) -> AnimationSample {
    AnimationSample { clip: "reach".into(), time, playback: Playback::Clamp }
}
fn pivot(d: &Document, joint: usize, time: f32) -> Vec3 {
    animation::evaluate_joints(d, &sample(time)).unwrap().joint_transforms[joint]
        .to_world(mm3e_editor::model::vec(d.joints[joint].pivot))
}
fn request(d: &Document) -> IkRequest {
    let root = pivot(d, 1, 0.5);
    serde_json::from_value(json!({"clip":"reach","time":0.5,"root_joint":"shoulder","middle_joint":"elbow","tip_joint":"wrist",
        "target":[root.x+0.55,root.y+0.45,root.z+0.3],"pole":[root.x,root.y+0.3,root.z+1.0],"tip_world_rotation_degrees":[25,-40,70]})).unwrap()
}

#[test]
fn ik_reaches_world_target_through_translated_rotated_scaled_ancestor_and_preserves_other_data() {
    let mut d = document();
    let original = serde_json::to_value(&d).unwrap();
    let r = request(&d);
    let before_root = pivot(&d, 1, 0.5);
    let before_l1 = (pivot(&d, 2, 0.5) - before_root).length();
    let before_l2 = (pivot(&d, 3, 0.5) - pivot(&d, 2, 0.5)).length();
    let result = ik::solve(&mut d, &r).unwrap();
    assert!((pivot(&d, 3, 0.5) - mm3e_editor::model::vec(r.target)).length() < r.tolerance_m, "{result}");
    assert!((pivot(&d, 1, 0.5) - before_root).length() < 1e-6);
    assert!(((pivot(&d, 2, 0.5) - pivot(&d, 1, 0.5)).length() - before_l1).abs() < 1e-6);
    assert!(((pivot(&d, 3, 0.5) - pivot(&d, 2, 0.5)).length() - before_l2).abs() < 1e-6);
    let after = serde_json::to_value(&d).unwrap();
    assert_eq!(original["objects"], after["objects"]);
    assert_eq!(original["joints"], after["joints"]);
    assert_eq!(original["clips"][0]["tracks"][0], after["clips"][0]["tracks"][0]);
    assert_eq!(original["clips"][0]["tracks"][3], after["clips"][0]["tracks"][3]);
    for index in [1, 2] {
        assert_eq!(original["clips"][0]["tracks"][index]["keys"][0], after["clips"][0]["tracks"][index]["keys"][0]);
        assert_eq!(original["clips"][0]["tracks"][index]["keys"][1], after["clips"][0]["tracks"][index]["keys"][2]);
    }
    let shoulder = &d.clips[0].tracks[1];
    assert_eq!(serde_json::to_value(shoulder.easing).unwrap(), "smooth_step");
    assert_eq!(shoulder.keys[1].translation, [0.01, -0.02, 0.03]);
    assert_eq!(shoulder.keys[1].scale, 1.0);
    let radians = r.tip_world_rotation_degrees.unwrap().map(f32::to_radians);
    let expected = Quat::from_euler(radians[0], radians[1], radians[2]).to_mat3();
    let evaluated = animation::evaluate_joints(&d, &sample(0.5)).unwrap();
    for (a, b) in expected.cols.into_iter().zip(evaluated.joint_transforms[3].rot.cols) {
        assert!((a - b).length() < 1e-5);
    }
    let (scene, _) = d.compile_at(&Pass::Beauty, Some(&sample(0.5))).unwrap();
    assert!(scene.sample_object(2, mm3e_editor::model::vec(r.target)).unwrap().dist < -0.05);
}

#[test]
fn unreachable_rejection_is_atomic_and_clamping_is_explicit_without_stretch() {
    let mut d = document();
    let before = serde_json::to_vec(&d).unwrap();
    let mut r = request(&d);
    r.target = [10, 10, 10].map(|x| x as f32);
    assert!(ik::solve(&mut d, &r).is_err());
    assert_eq!(serde_json::to_vec(&d).unwrap(), before);
    r.unreachable = ik::UnreachablePolicy::Clamp;
    let result = ik::solve(&mut d, &r).unwrap();
    assert_eq!(result["clamped"], true);
    assert!(result["requested_target_error_m"].as_f64().unwrap() > 1.0);
    assert!(result["target_error_m"].as_f64().unwrap() < 1e-4);
}

#[test]
fn malformed_chain_time_and_targets_leave_document_unchanged() {
    type Mutation = fn(&mut IkRequest);
    let mutations: [Mutation; 5] = [
        |r| r.middle_joint = "base".into(),
        |r| r.tip_joint = "shoulder".into(),
        |r| r.time = 2.0,
        |r| r.target[0] = f32::NAN,
        |r| r.tolerance_m = 0.0,
    ];
    for mutate in mutations {
        let mut d = document();
        let before = serde_json::to_vec(&d).unwrap();
        let mut r = request(&d);
        mutate(&mut r);
        assert!(ik::solve(&mut d, &r).is_err());
        assert_eq!(before, serde_json::to_vec(&d).unwrap());
    }
}

#[test]
fn jsonl_command_requires_revision_supports_dry_run_undo_redo_and_reports_reachable_pose() {
    let mut editor = Editor::new(&std::env::temp_dir()).unwrap();
    let d = document();
    let mut ops = d.objects.iter().map(|object| json!({"op":"create","object":object})).collect::<Vec<_>>();
    ops.push(json!({"op":"set_joints","joints":d.joints}));
    ops.push(json!({"op":"put_clip","clip":d.clips[0]}));
    let author = editor.handle(
        serde_json::from_value::<Request>(
            json!({"id":"author","expected_revision":0,"command":{"op":"apply","operations":ops}}),
        )
        .unwrap(),
    );
    assert!(author.ok);
    let r = request(editor.document());
    let before = serde_json::to_vec(editor.document()).unwrap();
    let run = |editor: &mut Editor, dry: bool, rev: Option<u64>| {
        editor.handle(
            serde_json::from_value::<Request>(
                json!({"id":"ik","expected_revision":rev,"command":{"op":"solve_ik","request":r,"dry_run":dry}}),
            )
            .unwrap(),
        )
    };
    assert!(!run(&mut editor, false, None).ok);
    assert!(run(&mut editor, true, Some(1)).ok);
    assert_eq!(editor.revision(), 1);
    assert_eq!(before, serde_json::to_vec(editor.document()).unwrap());
    let committed = run(&mut editor, false, Some(1));
    assert!(committed.ok, "{committed:?}");
    assert_eq!(editor.revision(), 2);
    let changed = serde_json::to_vec(editor.document()).unwrap();
    assert_ne!(changed, before);
    for (op, rev, expected) in [("undo", 2, &before), ("redo", 3, &changed)] {
        let response = editor.handle(
            serde_json::from_value::<Request>(json!({"id":op,"expected_revision":rev,"command":{"op":op}})).unwrap(),
        );
        assert!(response.ok);
        assert_eq!(&serde_json::to_vec(editor.document()).unwrap(), expected);
    }
    assert!(editor.document().clips[0].tracks.iter().any(|track| track.target == Target::Joint { id: "wrist".into() }));
}

#[test]
fn ik_edits_composite_clip_without_erasing_inherited_translation_scale_or_source_keys() {
    let mut d = document();
    d.clips[0].id = "source-motion".into();
    for index in [1, 2] {
        for key in &mut d.clips[0].tracks[index].keys {
            key.scale = 1.1;
        }
    }
    let source = serde_json::to_value(&d.clips[0]).unwrap();
    d.clips.push(
        serde_json::from_value(json!({"id":"reach","duration":1,"layers":[{"id":"body","clip":"source-motion"}]}))
            .unwrap(),
    );
    d.compile(&Pass::Beauty).unwrap();
    let original_root = pivot(&d, 1, 0.5);
    let lengths = [(pivot(&d, 2, 0.5) - original_root).length(), (pivot(&d, 3, 0.5) - pivot(&d, 2, 0.5)).length()];
    let r = request(&d);
    let result = ik::solve(&mut d, &r).unwrap();
    assert!((pivot(&d, 3, 0.5) - mm3e_editor::model::vec(r.target)).length() < r.tolerance_m, "{result}");
    assert!((pivot(&d, 1, 0.5) - original_root).length() < 2e-6);
    assert!(((pivot(&d, 2, 0.5) - pivot(&d, 1, 0.5)).length() - lengths[0]).abs() < 2e-6);
    assert!(((pivot(&d, 3, 0.5) - pivot(&d, 2, 0.5)).length() - lengths[1]).abs() < 2e-6);
    assert_eq!(serde_json::to_value(&d.clips[0]).unwrap(), source);
    assert_eq!(d.clips[1].layers.len(), 1);
    let shoulder = d.clips[1].tracks.iter().find(|t| t.target == Target::Joint { id: "shoulder".into() }).unwrap();
    assert_eq!(shoulder.keys[0].scale, 1.1);
    for (actual, expected) in shoulder.keys[0].translation.into_iter().zip([0.01, -0.02, 0.03]) {
        assert!((actual - expected).abs() < 2e-6);
    }
    let (scene, _) = d.compile_at(&Pass::Beauty, Some(&sample(0.5))).unwrap();
    assert!(scene.sample_object(2, mm3e_editor::model::vec(r.target)).unwrap().dist < 0.0);
}
