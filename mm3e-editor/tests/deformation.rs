use mm3e_editor::{
    animation::{AnimationSample, Playback},
    deform::{self, BindSurface, DeformerAsset, SkinningMethod, UpdateDeformer},
    model::{Document, Pass, Shape},
    protocol::Request,
    Editor,
};
use mm3e_kit::vec::{Transform, Vec3};
use mm3e_orchestrator::Prim;
use serde_json::{json, Value};

fn strip() -> Document {
    let mut document = Document::default();
    document.settings.width = 40;
    document.settings.height = 40;
    document.settings.shadows = false;
    document.settings.ao = false;
    document.camera.eye = [1.0, 0.5, 4.0];
    document.camera.target = [1.0, 0.5, 0.0];
    document.objects = serde_json::from_value(json!([{"id":"strip","shape":{"type":"surface",
        "vertices":[[0,-0.2,0],[0,0.2,0],[1,-0.2,0],[1,0.2,0],[2,-0.2,0],[2,0.2,0]],
        "triangles":[[0,2,1],[1,2,3],[2,4,3],[3,4,5]],"thickness_m":0.02}}]))
    .unwrap();
    // Deliberately child-before-parent: the asset palette is stable under document order.
    document.joints = serde_json::from_value(json!([
        {"id":"elbow","parent":"root","pivot":[1,0,0]},
        {"id":"root","pivot":[0,0,0]}
    ]))
    .unwrap();
    document.clips = serde_json::from_value(json!([{"id":"bend","duration":1,"tracks":[
        {"target":{"type":"joint","id":"elbow"},"keys":[{"time":0},{"time":1,"rotation_degrees":[0,0,90]}]}
    ]}]))
    .unwrap();
    document
}
fn asset() -> DeformerAsset {
    serde_json::from_value(json!({"id":"skin","object":"strip","joints":["root","elbow"],
        "weights":[[{"joint":0,"weight":1}],[{"joint":0,"weight":1}],
            [{"joint":0,"weight":0.5},{"joint":1,"weight":0.5}],
            [{"joint":0,"weight":0.5},{"joint":1,"weight":0.5}],
            [{"joint":1,"weight":1}],[{"joint":1,"weight":1}]]}))
    .unwrap()
}
fn bound() -> Document {
    let mut document = strip();
    deform::bind(&mut document, BindSurface { deformer: asset() }).unwrap();
    document
}
fn sample(time: f32) -> AnimationSample {
    AnimationSample { clip: "bend".into(), time, playback: Playback::Clamp }
}
fn near(got: Vec3, want: Vec3) {
    assert!((got - want).length() < 1e-5, "{got:?} != {want:?}");
}
fn points(value: &Value) -> Vec<Vec3> {
    serde_json::from_value::<Vec<[f32; 3]>>(value["vertices"].clone())
        .unwrap()
        .into_iter()
        .map(mm3e_editor::model::vec)
        .collect()
}

#[test]
fn analytic_two_joint_bend_updates_continuous_native_field_and_render_pixels() {
    let document = bound();
    let original = serde_json::to_value(&document).unwrap();
    let info = deform::inspect(&document, "skin", Some(&sample(1.0))).unwrap();
    let p = points(&info);
    for (got, want) in p.iter().zip([
        [0.0, -0.2, 0.0],
        [0.0, 0.2, 0.0],
        [1.1, -0.1, 0.0],
        [0.9, 0.1, 0.0],
        [1.2, 1.0, 0.0],
        [0.8, 1.0, 0.0],
    ]) {
        near(*got, mm3e_editor::model::vec(want));
    }
    let (posed, camera) = document.compile_at(&Pass::Beauty, Some(&sample(1.0))).unwrap();
    let Prim::Surface { id } = posed.objects[0].prim else { panic!("must remain native surface") };
    assert_eq!(posed.surfaces[id as usize].vertices(), p.as_slice());
    assert_eq!(posed.objects[0].xform.pos, Transform::IDENTITY.pos);
    assert_eq!(posed.objects[0].xform.rot.cols, Transform::IDENTITY.rot.cols);
    assert_eq!(posed.objects[0].xform.scale, 1.0);
    // Every retained triangle and shared edge reaches the actual native scalar field.
    for &[a, b, c] in posed.surfaces[id as usize].triangles() {
        let center = (p[a as usize] + p[b as usize] + p[c as usize]).scale(1.0 / 3.0);
        assert!(posed.sample_object(0, center).unwrap().dist < -0.009);
    }
    for t in 0..=20 {
        let midpoint = p[2].mix(p[3], t as f32 / 20.0);
        assert!(posed.sample_object(0, midpoint).unwrap().dist < -0.009);
    }
    assert!(posed.sample_object(0, Vec3::new(2.0, 0.0, 0.0)).unwrap().dist > 0.5);
    let (rest, rest_camera) = document.compile_at(&Pass::Beauty, None).unwrap();
    assert_ne!(
        mm3e_orchestrator::render_linear(&rest, &rest_camera),
        mm3e_orchestrator::render_linear(&posed, &camera)
    );
    assert_eq!(serde_json::to_value(&document).unwrap(), original);
    let halfway = points(&deform::inspect(&document, "skin", Some(&sample(0.5))).unwrap());
    assert!((halfway[4].y - p[4].y).abs() > 0.2);
}

#[test]
fn rigid_dqs_retains_blended_cross_section_on_the_real_surface() {
    let mut document = bound();
    document.deformers[0].method = SkinningMethod::DualQuaternion;
    let info = deform::inspect(&document, "skin", Some(&sample(1.0))).unwrap();
    let p = points(&info);
    let offset = 0.2 * std::f32::consts::FRAC_1_SQRT_2;
    near(p[2], Vec3::new(1.0 + offset, -offset, 0.0));
    near(p[3], Vec3::new(1.0 - offset, offset, 0.0));
    assert!(((p[3] - p[2]).length() - 0.4).abs() < 1e-5);
    let (scene, _) = document.compile_at(&Pass::Beauty, Some(&sample(1.0))).unwrap();
    for point in &p {
        assert!(scene.sample_object(0, *point).unwrap().dist < -0.009);
    }
    let linear = points(&deform::inspect(&bound(), "skin", Some(&sample(1.0))).unwrap());
    assert!((linear[3] - linear[2]).length() < 0.3);
}

#[test]
fn morph_local_then_object_rest_transform_then_bones_order_is_exact() {
    let mut document = strip();
    document.objects[0].position = [3.0, 0.0, 0.0];
    document.objects[0].rotation_degrees = [0.0, 0.0, 90.0];
    document.objects[0].scale = 2.0;
    document.objects[0].shape = Shape::Surface {
        vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        triangles: vec![[0, 1, 2]],
        thickness_m: 0.02,
    };
    document.joints = serde_json::from_value(json!([{"id":"bone","pivot":[3,0,0]}])).unwrap();
    document.clips = serde_json::from_value(json!([{"id":"bend","duration":1,"tracks":[
        {"target":{"type":"joint","id":"bone"},"keys":[{"time":0},{"time":1,"rotation_degrees":[0,0,90]}]}],
        "morph_tracks":[{"deformer":"skin","blendshape":"push","keys":[{"time":0,"weight":0},{"time":1,"weight":1}]}]
    }]))
    .unwrap();
    let deformer = serde_json::from_value(json!({"id":"skin","object":"strip","joints":["bone"],
        "weights":[[{"joint":0,"weight":1}],[{"joint":0,"weight":1}],[{"joint":0,"weight":1}]],
        "blendshapes":[{"id":"push","deltas":[[1,0,0],[1,0,0],[1,0,0]],"weight":0.25}]}))
    .unwrap();
    deform::bind(&mut document, BindSurface { deformer }).unwrap();
    let rest = points(&deform::inspect(&document, "skin", None).unwrap());
    near(rest[0], Vec3::new(3.0, 0.5, 0.0));
    let info = deform::inspect(&document, "skin", Some(&sample(1.0))).unwrap();
    let p = points(&info);
    near(p[0], Vec3::new(1.0, 0.0, 0.0));
    near(p[1], Vec3::new(-1.0, 0.0, 0.0));
    near(p[2], Vec3::new(1.0, -2.0, 0.0));
    assert!((info["thickness_m"].as_f64().unwrap() - 0.04).abs() < 1e-7);
    let (scene, _) = document.compile_at(&Pass::Beauty, Some(&sample(1.0))).unwrap();
    assert!((scene.sample_object(0, p[0]).unwrap().dist + 0.02).abs() < 1e-6);
}

#[test]
fn morph_only_uses_empty_skin_and_shared_easing_clamp_loop_time() {
    let mut document = strip();
    document.joints.clear();
    document.clips = serde_json::from_value(json!([{"id":"bend","duration":1,"morph_tracks":[
        {"deformer":"skin","blendshape":"lift","easing":"ease_in","keys":[{"time":0,"weight":0},{"time":1,"weight":1}]}
    ]}]))
    .unwrap();
    let deformer = serde_json::from_value(json!({"id":"skin","object":"strip","blendshapes":[
        {"id":"lift","deltas":vec![[0.0,1.0,0.0];6],"weight":0.3}
    ]}))
    .unwrap();
    deform::bind(&mut document, BindSurface { deformer }).unwrap();
    assert!(document.deformers[0].joints.is_empty());
    assert!(document.deformers[0].weights.is_empty());
    near(points(&deform::inspect(&document, "skin", None).unwrap())[0], Vec3::new(0.0, 0.1, 0.0));
    let halfway = deform::inspect(&document, "skin", Some(&sample(0.5))).unwrap();
    assert_eq!(halfway["morph_weights"][0]["weight"], 0.25);
    let looped = AnimationSample { playback: Playback::Loop, ..sample(-0.5) };
    assert_eq!(halfway["vertices"], deform::inspect(&document, "skin", Some(&looped)).unwrap()["vertices"]);
    assert_eq!(deform::inspect(&document, "skin", Some(&sample(3.0))).unwrap()["morph_weights"][0]["weight"], 1.0);
    assert_eq!(deform::inspect(&document, "skin", Some(&sample(-3.0))).unwrap()["morph_weights"][0]["weight"], 0.0);
    // An unrelated scaled joint never contaminates the no-skin mode.
    document.joints = serde_json::from_value(json!([{"id":"unrelated","pivot":[0,0,0]}])).unwrap();
    document.clips[0].tracks =
        serde_json::from_value(json!([{"target":{"type":"joint","id":"unrelated"},"keys":[{"time":0,"scale":2}]}]))
            .unwrap();
    document.deformers[0].method = SkinningMethod::DualQuaternion;
    document.compile_at(&Pass::Beauty, Some(&sample(0.5))).unwrap();
}

#[test]
fn invalid_weights_morphs_and_default_collapse_are_atomic() {
    let mut document = bound();
    let original = serde_json::to_value(&document).unwrap();
    for weights in [
        json!([]),
        json!(vec![json!([{"joint":0,"weight":0.0}]); 6]),
        json!(vec![json!([{"joint":0,"weight":0.4}]); 6]),
        json!(vec![json!([{"joint":2,"weight":1.0}]); 6]),
        json!(vec![json!([{"joint":0,"weight":0.5},{"joint":0,"weight":0.5}]); 6]),
    ] {
        let request: UpdateDeformer = serde_json::from_value(json!({"id":"skin","weights":weights})).unwrap();
        assert!(deform::update(&mut document, request).is_err());
        assert_eq!(serde_json::to_value(&document).unwrap(), original);
    }
    for morphs in [
        json!([{"id":"bad","deltas":[[0,0,0]]}]),
        json!([{"id":"bad","deltas":vec![[0,0,0];6],"weight":2}]),
        // Cancels every source position, collapsing all retained triangles at rest.
        json!([{"id":"collapse","weight":1,"deltas":[[0,0.2,0],[0,-0.2,0],[-1,0.2,0],[-1,-0.2,0],[-2,0.2,0],[-2,-0.2,0]]}]),
    ] {
        let request = serde_json::from_value(json!({"id":"skin","blendshapes":morphs})).unwrap();
        assert!(deform::update(&mut document, request).is_err());
        assert_eq!(serde_json::to_value(&document).unwrap(), original);
    }
}

#[test]
fn conflicting_ownership_rigid_motion_and_dqs_ancestor_scale_are_rejected() {
    let baseline = strip();
    let mut document = baseline.clone();
    document.joints[0].objects.push("strip".into());
    assert!(deform::bind(&mut document, BindSurface { deformer: asset() }).unwrap_err().contains("rigid"));
    let mut document = baseline.clone();
    document.clips[0].tracks[0].target = mm3e_editor::animation::Target::Object { id: "strip".into() };
    assert!(deform::bind(&mut document, BindSurface { deformer: asset() }).unwrap_err().contains("object transform"));
    let mut document = baseline.clone();
    document.objects[0].modifiers.mirror[0] = true;
    assert!(deform::bind(&mut document, BindSurface { deformer: asset() }).unwrap_err().contains("modifiers"));
    let mut document = bound();
    assert!(deform::bind(&mut document, BindSurface { deformer: asset() }).is_err());
    let mut document = baseline;
    document.clips[0].tracks[0].target = mm3e_editor::animation::Target::Joint { id: "root".into() };
    document.clips[0].tracks[0].keys[1].scale = 2.0;
    let mut deformer = asset();
    deformer.method = SkinningMethod::DualQuaternion;
    assert!(deform::bind(&mut document, BindSurface { deformer }).unwrap_err().contains("unit scale"));
}

#[test]
fn binding_cloth_owned_surface_is_rejected_without_mutating_its_recipe() {
    let mut document = Document::default();
    let panel = serde_json::from_value(json!({"id":"cloth","origin":[0,1,0],
        "axis_u":[1,0,0],"axis_v":[0,0,1],"segments":[1,1],"width_m":1,"height_m":1,
        "thickness_m":0.004,"vertex_mass_kg":0.02,"pins":[],"collision_object_ids":[]}))
    .unwrap();
    mm3e_editor::cloth::create_panel(&mut document, &panel).unwrap();
    let before = serde_json::to_value(&document).unwrap();
    let deformer = serde_json::from_value(json!({"id":"skin","object":"cloth"})).unwrap();
    let error = deform::bind(&mut document, BindSurface { deformer }).unwrap_err();
    assert!(error.contains("ownership"), "{error}");
    assert_eq!(serde_json::to_value(&document).unwrap(), before);
}

#[test]
fn scalar_channel_validation_and_removal_preserve_other_animation() {
    let mut document = bound();
    document.deformers[0].blendshapes = serde_json::from_value(json!([
        {"id":"lift","deltas":vec![[0.0,0.0,1.0];6]}
    ]))
    .unwrap();
    let valid = json!({"deformer":"skin","blendshape":"lift","keys":[{"time":0,"weight":0},{"time":1,"weight":1}]});
    for invalid in [
        json!({"deformer":"missing","blendshape":"lift","keys":[{"time":0,"weight":0}]}),
        json!({"deformer":"skin","blendshape":"missing","keys":[{"time":0,"weight":0}]}),
        json!({"deformer":"skin","blendshape":"lift","keys":[{"time":0,"weight":2}]}),
        json!({"deformer":"skin","blendshape":"lift","keys":[{"time":0,"weight":0},{"time":0,"weight":1}]}),
        json!({"deformer":"skin","blendshape":"lift","keys":[{"time":2,"weight":1}]}),
    ] {
        let mut candidate = document.clone();
        candidate.clips[0].morph_tracks = serde_json::from_value(json!([invalid])).unwrap();
        assert!(candidate.compile(&Pass::Beauty).is_err());
    }
    let mut duplicate = document.clone();
    duplicate.clips[0].morph_tracks = serde_json::from_value(json!([valid.clone(), valid.clone()])).unwrap();
    assert!(duplicate.compile(&Pass::Beauty).is_err());
    document.clips[0].morph_tracks = serde_json::from_value(json!([valid])).unwrap();
    let before = serde_json::to_value(&document).unwrap();
    let update = serde_json::from_value(json!({"id":"skin","blendshapes":[]})).unwrap();
    assert!(deform::update(&mut document, update).is_err());
    assert_eq!(serde_json::to_value(&document).unwrap(), before);
    let objects = serde_json::to_value(&document.objects).unwrap();
    let tracks = serde_json::to_value(&document.clips[0].tracks).unwrap();
    deform::remove(&mut document, "skin").unwrap();
    assert_eq!(serde_json::to_value(&document.objects).unwrap(), objects);
    assert_eq!(serde_json::to_value(&document.clips[0].tracks).unwrap(), tracks);
    assert!(document.clips[0].morph_tracks.is_empty());
}

#[test]
fn collapsed_animated_pose_fails_without_partial_scene_mutation() {
    let mut document = bound();
    let deltas = vec![
        [0.0, 0.2, 0.0],
        [0.0, -0.2, 0.0],
        [-1.0, 0.2, 0.0],
        [-1.0, -0.2, 0.0],
        [-2.0, 0.2, 0.0],
        [-2.0, -0.2, 0.0],
    ];
    document.deformers[0].blendshapes = serde_json::from_value(json!([{"id":"collapse","deltas":deltas}])).unwrap();
    document.clips[0].tracks.clear();
    document.clips[0].morph_tracks = serde_json::from_value(json!([
        {"deformer":"skin","blendshape":"collapse","keys":[{"time":0,"weight":0},{"time":1,"weight":1}]}
    ]))
    .unwrap();
    let (mut scene, _) = document.compile_at(&Pass::Beauty, None).unwrap();
    let before = scene.surfaces[0].vertices().to_vec();
    assert!(deform::refresh(&document, &mut scene, Some(&sample(1.0)), None).is_err());
    assert_eq!(scene.surfaces[0].vertices(), before);
    assert!(document.compile_at(&Pass::Beauty, Some(&sample(1.0))).is_err());
}

fn send(editor: &mut Editor, command: Value, okay: bool) -> Value {
    let request: Request =
        serde_json::from_value(json!({"id":"deformation","expected_revision":editor.revision(),"command":command}))
            .unwrap();
    let result = editor.handle(request);
    assert_eq!(result.ok, okay, "{result:?}");
    result.result.unwrap_or_else(|| serde_json::to_value(result.error).unwrap())
}

#[test]
fn editor_protocol_undo_reload_and_material_rest_geometry_edits_reach_native_field() {
    let root = std::env::temp_dir().join(format!(
        "mm3e-deformation-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir(&root).unwrap();
    let mut editor = Editor::new(&root).unwrap();
    let source = strip();
    send(
        &mut editor,
        json!({"op":"apply","operations":[
            {"op":"create","object":source.objects[0]},
            {"op":"set_joints","joints":source.joints},
            {"op":"put_clip","clip":source.clips[0]},
            {"op":"bind_surface","request":{"deformer":asset()}}
        ]}),
        true,
    );
    let original = serde_json::to_value(editor.document()).unwrap();
    let before = send(&mut editor, json!({"op":"deformer_state","id":"skin","animation":sample(1.0)}), true);
    send(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"update","id":"strip","patch":{"material":{"albedo":[0.8,0.1,0.1]}}}]}),
        true,
    );
    assert_eq!(
        send(&mut editor, json!({"op":"deformer_state","id":"skin","animation":sample(1.0)}), true)["vertices"],
        before["vertices"]
    );
    send(&mut editor, json!({"op":"undo"}), true);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), original);
    send(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"update","id":"strip","patch":{"position":[0,0,1]}}]}),
        true,
    );
    let moved = send(&mut editor, json!({"op":"deformer_state","id":"skin","animation":sample(1.0)}), true);
    assert_eq!(moved["vertices"][0][2], 1.0);
    send(&mut editor, json!({"op":"undo"}), true);
    let revision = editor.revision();
    send(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"update_deformer","request":{"id":"skin","weights":[]}}]}),
        false,
    );
    assert_eq!(editor.revision(), revision);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), original);
    send(&mut editor, json!({"op":"save","path":"deformation.json"}), true);
    send(&mut editor, json!({"op":"apply","operations":[{"op":"remove_deformer","id":"skin"}]}), true);
    assert!(editor.document().deformers.is_empty());
    send(&mut editor, json!({"op":"undo"}), true);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), original);
    let mut reopened = Editor::new(&root).unwrap();
    send(&mut reopened, json!({"op":"load","path":"deformation.json"}), true);
    assert_eq!(send(&mut reopened, json!({"op":"deformer_state","id":"skin","animation":sample(1.0)}), true), before);
    let (a, ca) = editor.document().compile_at(&Pass::Beauty, Some(&sample(0.37))).unwrap();
    let (b, cb) = reopened.document().compile_at(&Pass::Beauty, Some(&sample(0.37))).unwrap();
    assert_eq!(a.surfaces[0].vertices(), b.surfaces[0].vertices());
    // Keep this actual render small even though Editor starts with default settings.
    let mut a = a;
    let mut b = b;
    a.width = 24;
    a.height = 24;
    b.width = 24;
    b.height = 24;
    assert_eq!(mm3e_orchestrator::render_linear(&a, &ca), mm3e_orchestrator::render_linear(&b, &cb));
    std::fs::remove_dir_all(root).unwrap();
}
