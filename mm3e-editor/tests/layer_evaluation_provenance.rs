use mm3e_editor::{
    animation::{self, AnimationSample, Playback},
    deform::{self, BindSurface},
    face::{self, FaceRequest},
    model::{vec, Document, Pass, Shape},
};
use mm3e_kit::Vec3;
use mm3e_orchestrator::{Prim, Scene};
use serde_json::json;

fn sample(clip: &str, time: f32) -> AnimationSample {
    AnimationSample { clip: clip.into(), time, playback: Playback::Clamp }
}

fn deformed() -> Document {
    let mut document = Document {
        objects: serde_json::from_value(json!([{"id":"patch","shape":{"type":"surface",
        "vertices":[[0,0,0],[1,0,0],[0,1,0]],"triangles":[[0,1,2]],"thickness_m":0.02}}]))
        .unwrap(),
        joints: serde_json::from_value(json!([{"id":"root","pivot":[0,0,0]}])).unwrap(),
        ..Document::default()
    };
    let deformer = serde_json::from_value(json!({"id":"skin","object":"patch","joints":["root"],
        "weights":vec![json!([{"joint":0,"weight":1}]);3],
        "blendshapes":[{"id":"lift","deltas":vec![[0,0,1];3],"weight":0.2}]}))
    .unwrap();
    deform::bind(&mut document, BindSurface { deformer }).unwrap();
    document.clips = serde_json::from_value(json!([
        {"id":"source","duration":1,"tracks":[{"target":{"type":"joint","id":"root"},
            "keys":[{"time":0,"translation":[1,0,0]}]}],
            "morph_tracks":[{"deformer":"skin","blendshape":"lift","keys":[{"time":0,"weight":0.8}]}]},
        {"id":"take","duration":1,"layers":[{"id":"base","clip":"source","weight":0.5}]},
        {"id":"other","duration":1,"layers":[{"id":"base","clip":"source","weight":0.25}]}
    ]))
    .unwrap();
    document.compile(&Pass::Beauty).unwrap();
    document
}

fn vertices(scene: &Scene) -> Vec<Vec3> {
    let Prim::Surface { id } = scene.objects[0].prim else { panic!("expected native surface") };
    scene.surfaces[id as usize].vertices().to_vec()
}

#[test]
fn supplied_evaluation_from_a_different_clip_at_the_same_time_is_rejected_before_mutation() {
    let document = deformed();
    let evaluation = animation::evaluate_joints(&document, &sample("take", 0.5)).unwrap();
    let (mut scene, _) = document.compile(&Pass::Beauty).unwrap();
    let before = vertices(&scene);
    let error = deform::refresh(&document, &mut scene, Some(&sample("other", 0.5)), Some(&evaluation)).unwrap_err();
    assert!(error.contains("same canonical animation sample"), "{error}");
    assert_eq!(vertices(&scene), before);

    deform::refresh(&document, &mut scene, Some(&sample("take", 0.5)), Some(&evaluation)).unwrap();
    let expected = document.compile_at(&Pass::Beauty, Some(&sample("take", 0.5))).unwrap().0;
    assert_eq!(vertices(&scene), vertices(&expected));
    assert!((vertices(&scene)[0] - Vec3::new(0.5, 0.0, 0.5)).length() < 1e-6);
}

#[test]
fn relabeled_time_and_modified_joint_payloads_are_rejected_before_geometry_changes() {
    let mut document = deformed();
    document.clips[0].tracks[0].keys =
        serde_json::from_value(json!([{"time":0},{"time":1,"translation":[2,0,0]}])).unwrap();
    for tamper_time in [true, false] {
        let mut evaluation = animation::evaluate_joints(&document, &sample("take", 0.5)).unwrap();
        let at = if tamper_time {
            evaluation.time = 0.75;
            sample("take", 0.75)
        } else {
            evaluation.joint_transforms[0].pos.y += 1.0;
            sample("take", 0.5)
        };
        let (mut scene, _) = document.compile(&Pass::Beauty).unwrap();
        let before = vertices(&scene);
        let error = deform::refresh(&document, &mut scene, Some(&at), Some(&evaluation)).unwrap_err();
        assert!(error.contains("same canonical animation sample"), "{error}");
        assert_eq!(vertices(&scene), before);
    }
}

#[test]
fn changed_animation_inputs_invalidate_supplied_evaluation_but_mesh_edits_do_not() {
    let document = deformed();
    let at = sample("take", 0.5);
    let evaluation = animation::evaluate_joints(&document, &at).unwrap();
    let mut changed = Vec::new();
    let mut keys = document.clone();
    keys.clips[0].tracks[0].keys[0].translation[0] = 2.0;
    changed.push(("source keys", keys));
    let mut layer = document.clone();
    layer.clips[1].layers[0].weight = 0.75;
    changed.push(("layer weight", layer));
    let mut joint = document.clone();
    joint.joints[0].pivot[1] = 0.5;
    changed.push(("joint pivot", joint));
    let mut anchor = document.clone();
    anchor.objects[0].position[0] = 0.1;
    changed.push(("object anchor", anchor));
    let mut morph = document.clone();
    morph.deformers[0].blendshapes[0].weight = 0.4;
    changed.push(("morph default", morph));
    let mut camera = document.clone();
    camera.camera.eye[0] += 0.1;
    changed.push(("camera default", camera));
    for (name, changed) in changed {
        let (mut scene, _) = changed.compile(&Pass::Beauty).unwrap();
        let before = vertices(&scene);
        let error = deform::refresh(&changed, &mut scene, Some(&at), Some(&evaluation)).unwrap_err();
        assert!(error.contains("authored inputs"), "{name}: {error}");
        assert_eq!(vertices(&scene), before, "{name}");
        let fresh = animation::evaluate_joints(&changed, &at).unwrap();
        deform::refresh(&changed, &mut scene, Some(&at), Some(&fresh)).unwrap();
        assert_eq!(vertices(&scene), vertices(&changed.compile_at(&Pass::Beauty, Some(&at)).unwrap().0), "{name}");
    }

    let mut mesh = document.clone();
    let Shape::Surface { vertices: points, .. } = &mut mesh.objects[0].shape else { unreachable!() };
    points[1][0] = 1.5;
    mesh.deformers[0].blendshapes[0].deltas[1][2] = 0.5;
    let (mut scene, _) = mesh.compile(&Pass::Beauty).unwrap();
    deform::refresh(&mesh, &mut scene, Some(&at), Some(&evaluation)).unwrap();
    assert_eq!(vertices(&scene), vertices(&mesh.compile_at(&Pass::Beauty, Some(&at)).unwrap().0));
    assert!((vertices(&scene)[1] - Vec3::new(2.0, 0.0, 0.25)).length() < 1e-6);
}

#[test]
fn layered_eye_delta_is_applied_once_inside_the_animated_head_and_local_keys_override_it() {
    let mut document = Document {
        objects: serde_json::from_value(json!([
            {"id":"actor/head","position":[1,2,0],"shape":{"type":"ellipsoid","radii":[0.8,1,0.8]}},
            {"id":"actor/left_eye","position":[1.28,2.2,0.78],"shape":{"type":"sphere","radius":0.15}},
            {"id":"actor/right_eye","position":[0.72,2.2,0.78],"shape":{"type":"sphere","radius":0.15}}
        ]))
        .unwrap(),
        ..Document::default()
    };
    face::create(&mut document, &FaceRequest { id: "face".into(), character: "actor".into() }).unwrap();
    document.clips = serde_json::from_value(json!([
        {"id":"body","duration":1,"tracks":[{"target":{"type":"object","id":"actor/head"},
            "keys":[{"time":0},{"time":1,"rotation_degrees":[0,0,90],"translation":[2,0,0]}]}]},
        {"id":"eyes","duration":1,"tracks":[{"target":{"type":"object","id":"actor/left_eye"},
            "keys":[{"time":0},{"time":1,"translation":[0.1,0,0]}]}]},
        {"id":"take","duration":1,"layers":[{"id":"body","clip":"body"},{"id":"eyes","clip":"eyes"}]},
        {"id":"override","duration":1,"layers":[{"id":"base","clip":"take"}],
            "tracks":[{"target":{"type":"object","id":"actor/left_eye"},"keys":[{"time":0,"translation":[0.2,0,0]}]}]}
    ]))
    .unwrap();
    let original = serde_json::to_value(&document).unwrap();
    for (clip, y) in [("take", 2.38), ("override", 2.48)] {
        let (scene, _) = document.compile_at(&Pass::Beauty, Some(&sample(clip, 1.0))).unwrap();
        let left = scene.objects[1].xform;
        // World-rest pivot composition at this three-meter offset accumulates a few f32 ULPs.
        assert!((left.pos - Vec3::new(2.8, y, 0.78)).length() < 2e-6, "{clip}: {:?}", left.pos);
        assert!((scene.objects[2].xform.pos - Vec3::new(2.8, 1.72, 0.78)).length() < 2e-6);
        for name in ["face/left_upper_lid", "face/left_lower_lid"] {
            let index = document.objects.iter().position(|o| o.id == name).unwrap();
            assert_eq!(scene.objects[index].xform.pos, left.pos);
            assert_eq!(scene.objects[index].xform.rot.cols, left.rot.cols);
        }
        assert!(scene.sample_object(1, left.pos).unwrap().dist < -0.149);
        assert!(scene.sample_object(1, vec(document.objects[1].position)).unwrap().dist > 1.0);
    }
    assert_eq!(serde_json::to_value(&document).unwrap(), original);
}

#[test]
fn removing_a_deformer_preserves_empty_masks_and_unrelated_layer_channels() {
    let mut document = deformed();
    document.clips[1].layers = serde_json::from_value(json!([
        {"id":"only","clip":"source","mask":[{"type":"morph","deformer":"skin","blendshape":"lift"}]},
        {"id":"mixed","clip":"source","mask":[{"type":"morph","deformer":"skin","blendshape":"lift"},{"type":"joint","id":"root"}]},
        {"id":"all","clip":"source"}
    ])).unwrap();
    deform::remove(&mut document, "skin").unwrap();
    let clips = serde_json::to_value(&document.clips).unwrap();
    assert!(document.clips[0].morph_tracks.is_empty());
    assert_eq!(clips[1]["layers"][0]["mask"], json!([]));
    assert_eq!(clips[1]["layers"][1]["mask"], json!([{"type":"joint","id":"root"}]));
    assert!(clips[1]["layers"][2]["mask"].is_null());
    assert_eq!(clips[0]["tracks"].as_array().unwrap().len(), 1);
    animation::validate(&document).unwrap();
    let evaluation = animation::evaluate_joints(&document, &sample("take", 0.5)).unwrap();
    assert_eq!(evaluation.joint_transforms[0].pos, Vec3::new(1.0, 0.0, 0.0));
}
