use mm3e_editor::{
    animation::{AnimationSample, Playback},
    garment::{self, GarmentRequest, GarmentStyle},
    model::{Document, Pass, Surface},
    protocol::Request,
    Editor,
};
use mm3e_kit::{march::Ray, vec::Vec3};
use serde_json::{json, Value};

fn document() -> Document {
    let mut editor = Editor::new(&std::env::temp_dir()).unwrap();
    let request: Request = serde_json::from_value(json!({"id":"create","expected_revision":0,"command":{
        "op":"apply","operations":[{"op":"create_humanoid","id":"hero","height":1.8,"build":1.0,"head_scale":1.0}]
    }}))
    .unwrap();
    let response = editor.handle(request);
    assert!(response.ok, "{response:?}");
    editor.document().clone()
}
fn request(style: GarmentStyle) -> GarmentRequest {
    GarmentRequest {
        id: "costume".into(),
        character: "hero".into(),
        style,
        clearance_m: 0.01,
        thickness_m: 0.004,
        material: Surface { albedo: [0.75, 0.08, 0.12], ..Surface::default() },
    }
}
fn create(style: GarmentStyle) -> Document {
    let mut document = document();
    garment::create(&mut document, &request(style)).unwrap();
    document
}
fn object_index(document: &Document, id: &str) -> usize {
    document.objects.iter().position(|e| e.id == id).unwrap()
}
fn near(value: f32, want: f32) {
    assert!((value - want).abs() < 2e-5, "{value} != {want}");
}
fn field(document: &Document, point: Vec3) -> f32 {
    let (scene, _) = document.compile(&Pass::Beauty).unwrap();
    scene.sample_object(object_index(document, "costume"), point).unwrap().dist
}

#[test]
fn garment_is_distinct_thick_hollow_geometry_and_preserves_body() {
    let mut document = document();
    let body_before = serde_json::to_value(&document.objects).unwrap();
    garment::create(&mut document, &request(GarmentStyle::Vest)).unwrap();
    assert_eq!(body_before, serde_json::to_value(&document.objects[..19]).unwrap());
    assert_eq!(document.objects.len(), 20);
    let (scene, _) = document.compile(&Pass::Beauty).unwrap();
    let index = object_index(&document, "costume");
    let sample = |z| scene.sample_object(index, Vec3::new(0.0, 1.062, z)).unwrap().dist;
    // The abdomen's positive Z axis is an exact ellipsoid distance location. The rest
    // source body does not blend with another part at these points.
    near(sample(0.1116 + 0.01), 0.0);
    near(sample(0.1116 + 0.012), -0.002);
    near(sample(0.1116 + 0.014), 0.0);
    assert!(sample(0.1116 + 0.006) > 0.003);
    assert!(sample(0.0) > 0.08, "garment interior must be genuinely hollow");
    let ray = Ray { origin: Vec3::new(0.0, 1.062, 3.0), dir: Vec3::new(0.0, 0.0, -1.0) };
    let hit = scene.marcher.march(&scene.field(), &ray);
    assert!(hit.hit);
    assert_eq!(hit.mat, scene.objects[index].mat, "rendered front surface must belong to clothing");
    assert_eq!(scene.materials[hit.mat as usize].albedo, Vec3::new(0.75, 0.08, 0.12));
}

#[test]
fn shirt_has_neck_hem_and_sleeve_openings() {
    let document = create(GarmentStyle::ShortSleeveTop);
    let (scene, _) = document.compile(&Pass::Beauty).unwrap();
    let index = object_index(&document, "costume");
    let sample = |p| scene.sample_object(index, p).unwrap().dist;
    // Neck channel goes through the otherwise closed chest cap.
    for step in 0..81 {
        assert!(sample(Vec3::new(0.0, 1.3 + step as f32 * 0.006, 0.0)) >= -1e-6);
    }
    let a = Vec3::new(0.207, 1.359, 0.0);
    let b = Vec3::new(0.378, 1.143, 0.0);
    let axis = (b - a).normalize();
    let sleeve_front = |t| a + (b - a).scale(t) + Vec3::new(0.0, 0.0, 0.0594 + 0.012);
    assert!(sample(sleeve_front(0.50)) < -0.001);
    assert!(sample(sleeve_front(0.80)) > 0.002);
    for step in 0..81 {
        let point = a + (b - a).scale(0.65) + axis.scale(step as f32 * 0.006);
        assert!(sample(point) >= -1e-6, "a cuff must not have an end cap at {point:?}");
    }
    // At the hem, probe the positive Z outline just below the authored opening.
    let p = Vec3::new(0.0, 0.965, 0.1116 * (1.0f32 - ((0.965f32 - 1.062) / 0.18).powi(2)).sqrt() + 0.013);
    assert!(sample(p) > 0.0, "hem extends below its opening");
}

#[test]
fn trousers_have_two_open_cuffs_and_an_open_waist() {
    let document = create(GarmentStyle::Trousers);
    let (scene, _) = document.compile(&Pass::Beauty).unwrap();
    let i = object_index(&document, "costume");
    let sample = |p| scene.sample_object(i, p).unwrap().dist;
    for sign in [-1.0, 1.0] {
        let a = Vec3::new(sign * 0.066 * 1.8, 0.285 * 1.8, 0.018);
        let b = Vec3::new(sign * 0.069 * 1.8, 0.075 * 1.8, 0.0);
        let front = a + (b - a).scale(0.6) + Vec3::new(0.0, 0.0, 0.035 * 1.8 + 0.012);
        assert!(sample(front) < 0.0, "trouser leg must have independent material volume");
        let axis = (b - a).normalize();
        for step in 0..40 {
            assert!(sample(a + (b - a).scale(0.94) + axis.scale(step as f32 * 0.006)) >= -1e-6);
        }
    }
    for step in 0..50 {
        assert!(sample(Vec3::new(0.0, 0.96 + step as f32 * 0.006, 0.0)) >= -1e-6);
    }
}

#[test]
fn source_animation_updates_the_actual_garment_without_a_second_binding() {
    let mut document = create(GarmentStyle::ShortSleeveTop);
    document.joints = mm3e_editor::animation::rig_humanoid(&document, "hero").unwrap();
    document.clips = serde_json::from_value(json!([{"id":"move","duration":1,"tracks":[
        {"target":{"type":"joint","id":"hero/root"},"keys":[{"time":0},{"time":1,"translation":[0.5,0.2,-0.3]}]},
        {"target":{"type":"joint","id":"hero/left_shoulder"},"keys":[{"time":0},{"time":1,"rotation_degrees":[0,0,55]}]}
    ]}]))
    .unwrap();
    let sample = AnimationSample { clip: "move".into(), time: 1.0, playback: Playback::Clamp };
    let rest_point = Vec3::new(0.0, 1.062, 0.1236);
    let i = object_index(&document, "costume");
    let (scene, _) = document.compile_at(&Pass::Beauty, Some(&sample)).unwrap();
    near(scene.sample_object(i, rest_point + Vec3::new(0.5, 0.2, -0.3)).unwrap().dist, -0.002);
    assert!(scene.sample_object(i, rest_point).unwrap().dist > 0.1);
    let arm = object_index(&document, "hero/left_upper_arm");
    let local_point = Vec3::new(0.207, 1.359, 0.0)
        + (Vec3::new(0.378, 1.143, 0.0) - Vec3::new(0.207, 1.359, 0.0)).scale(0.5)
        + Vec3::new(0.0, 0.0, 0.0714);
    let posed_point = scene.objects[arm].xform.to_world(local_point);
    assert!(scene.sample_object(i, posed_point).unwrap().dist < 0.0, "sleeve must follow the actual evaluated arm");
    assert_eq!(scene.objects[i].xform.pos, Vec3::ZERO);
    assert!(!document.joints.iter().any(|joint| joint.objects.contains(&"costume".to_string())));
}

#[test]
fn fit_report_measures_generated_surfaces_in_rest_and_motion() {
    let mut document = create(GarmentStyle::ShortSleeveTop);
    let fit = garment::inspect_fit(&document, "costume", None, 32).unwrap();
    assert!(fit["surface_samples"].as_u64().unwrap() > 40, "{fit}");
    assert_eq!(fit["body_penetration_samples"], 0);
    assert_eq!(fit["clearance_violation_samples"], 0, "{fit}");
    assert_eq!(fit["sampled_fit_pass"], true);
    document.joints = mm3e_editor::animation::rig_humanoid(&document, "hero").unwrap();
    document.clips = serde_json::from_value(json!([{"id":"wave","duration":1,"tracks":[
        {"target":{"type":"joint","id":"hero/left_shoulder"},"keys":[{"time":0},{"time":1,"rotation_degrees":[0,0,95]}]}
    ]}]))
    .unwrap();
    let fit = garment::inspect_fit(
        &document,
        "costume",
        Some(&AnimationSample { clip: "wave".into(), time: 1.0, playback: Playback::Clamp }),
        32,
    )
    .unwrap();
    assert_eq!(fit["sampled_fit_pass"], true, "{fit}");
}

#[test]
fn invalid_creation_and_unsupported_source_edits_do_not_pass_validation() {
    let mut document = document();
    let before = serde_json::to_value(&document).unwrap();
    let mut invalid = request(GarmentStyle::Vest);
    invalid.thickness_m = 0.0;
    assert!(garment::create(&mut document, &invalid).is_err());
    assert_eq!(before, serde_json::to_value(&document).unwrap());
    invalid = request(GarmentStyle::Vest);
    invalid.material.roughness = 0.0;
    assert!(garment::create(&mut document, &invalid).is_err());
    assert_eq!(before, serde_json::to_value(&document).unwrap());
    garment::create(&mut document, &request(GarmentStyle::Vest)).unwrap();
    let i = object_index(&document, "hero/chest");
    document.objects[i].modifiers.round = 0.01;
    assert!(garment::validate(&document).unwrap_err().contains("unsupported modifiers"));
}

#[test]
fn derived_clothing_rejects_double_binding_and_persists_with_source_edits() {
    let mut document = create(GarmentStyle::Vest);
    let i = object_index(&document, "costume");
    document.objects[i].position[0] = 0.1;
    assert!(garment::validate(&document).is_err());
    document.objects[i].position[0] = 0.0;
    document.joints =
        serde_json::from_value(json!([{"id":"incorrect","pivot":[0,0,0],"objects":["costume"]}])).unwrap();
    assert!(garment::validate(&document).unwrap_err().contains("joint-bound"));
    document.joints.clear();
    for entity in &mut document.objects {
        if entity.id.starts_with("hero/") {
            entity.position[0] += 0.3;
        }
    }
    garment::synchronize(&mut document).unwrap();
    let reloaded: Document = serde_json::from_value(serde_json::to_value(&document).unwrap()).unwrap();
    let p = Vec3::new(0.3, 1.062, 0.1236);
    near(field(&reloaded, p), -0.002);
    assert_eq!(serde_json::to_value(&document).unwrap(), serde_json::to_value(&reloaded).unwrap());
}

#[test]
fn fit_uses_whole_body_so_noncovered_smooth_bulges_cannot_pierce_clothing() {
    let mut document = create(GarmentStyle::Vest);
    let i = object_index(&document, "hero/pelvis");
    // Enlarge and raise an uncovered body part into the original abdomen shell.
    document.objects[i].position[1] = 1.01;
    document.objects[i].scale = 1.15;
    garment::synchronize(&mut document).unwrap();
    let report: Value = garment::inspect_fit(&document, "costume", None, 32).unwrap();
    assert_eq!(report["body_penetration_samples"], 0, "{report}");
    assert_eq!(report["clearance_violation_samples"], 0, "{report}");
    assert!(report["surface_samples"].as_u64().unwrap() > 0);
}

#[test]
fn updating_garment_parameters_preserves_identity_order_and_explicit_material() {
    let mut document = create(GarmentStyle::Vest);
    let index = object_index(&document, "costume");
    let original_order: Vec<_> = document.objects.iter().map(|e| e.id.clone()).collect();
    let mut next = request(GarmentStyle::ShortSleeveTop);
    next.clearance_m = 0.016;
    next.thickness_m = 0.008;
    next.material.albedo = [0.10, 0.70, 0.15];
    garment::update(&mut document, &next).unwrap();
    assert_eq!(index, object_index(&document, "costume"));
    assert_eq!(original_order, document.objects.iter().map(|e| e.id.clone()).collect::<Vec<_>>());
    assert_eq!(document.objects[index].material.albedo, next.material.albedo);
    near(field(&document, Vec3::new(0.0, 1.062, 0.1116 + 0.020)), -0.004);
    next.style = GarmentStyle::Trousers;
    garment::update(&mut document, &next).unwrap();
    assert_eq!(document.garments[0].opening_source_ids, ["hero/pelvis", "hero/left_shin", "hero/right_shin"]);
    let before = serde_json::to_value(&document).unwrap();
    next.character = "missing".into();
    assert!(garment::update(&mut document, &next).is_err());
    assert_eq!(before, serde_json::to_value(&document).unwrap());
}

#[test]
fn stale_or_tampered_saved_garment_geometry_is_rejected() {
    let mut document = create(GarmentStyle::Vest);
    let i = object_index(&document, "costume");
    document.objects[i].shape = mm3e_editor::model::Shape::Csg {
        expression: Box::new(mm3e_editor::csg::CsgExpr::primitive(mm3e_editor::model::Shape::Sphere { radius: 0.1 })),
    };
    assert!(garment::validate(&document).unwrap_err().contains("stale or edited"));
    garment::synchronize(&mut document).unwrap();
    garment::validate(&document).unwrap();
}
