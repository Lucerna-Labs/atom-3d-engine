use mm3e_editor::{
    animation::{AnimationSample, Playback},
    cloth::{self, BakeClothRequest, ClothPanelRequest},
    deform::SkinningMethod,
    model::{Document, Pass},
    protocol::Request,
    Editor,
};
use mm3e_kit::Vec3;
use mm3e_orchestrator::Prim;
use serde_json::{json, Value};

fn sample(time: f32) -> AnimationSample {
    AnimationSample { clip: "rise".into(), time, playback: Playback::Clamp }
}
fn bake_request() -> BakeClothRequest {
    BakeClothRequest { id: "cloth".into(), clip: "rise".into(), duration: None }
}
fn panel(id: &str) -> ClothPanelRequest {
    serde_json::from_value(json!({"id":id,"origin":[0,0.05,0],"axis_u":[1,0,0],"axis_v":[0,0,1],
        "segments":[2,2],"width_m":0.4,"height_m":0.4,"thickness_m":0.002,"vertex_mass_kg":0.02,
        "collision_object_ids":["body"],"settings":{"gravity":[0,0,0],"iterations":24,"substeps":4,
            "collision_thickness":0.003,"max_penetration_m":0.002}}))
    .unwrap()
}
fn fixture(method: SkinningMethod) -> Document {
    let shape = json!({"type":"surface","vertices":[[-2,0,-2],[2,0,-2],[2,0,2],[-2,0,2]],
        "triangles":[[0,2,1],[0,3,2]],"thickness_m":0.02});
    let objects = serde_json::from_value(json!([
        {"id":"body","shape":shape},{"id":"unrelated","shape":shape,"position":[6,0,0]}]))
    .unwrap();
    let joints = serde_json::from_value(json!([
    {"id":"other","pivot":[6,0,0],"objects":[]},
    {"id":"root","pivot":[0,0,0],"objects":[]},
    {"id":"other2","pivot":[6,0,0],"objects":[]},
    {"id":"alternate","parent":"root","pivot":[0,0,0],"objects":[]},
    {"id":"skin","parent":"root","pivot":[0,0,0],"objects":[]},
    ]))
    .unwrap();
    let deformers = serde_json::from_value(json!([
        {"id":"body-deformer","object":"body","method":method,"joints":["skin","alternate"],
            "weights":vec![json!([{"joint":0,"weight":1}]);4],
            "blendshapes":[{"id":"lift","deltas":vec![[0.0,0.15,0.0];4]}]},
        {"id":"unrelated-deformer","object":"unrelated","joints":["other","other2"],
            "weights":vec![json!([{"joint":0,"weight":1}]);4],
            "blendshapes":[{"id":"unused","deltas":vec![[0.0,0.3,0.0];4]}]}]))
    .unwrap();
    let clips=serde_json::from_value(json!([{"id":"rise","duration":0.5,
        "tracks":[
            {"target":{"type":"joint","id":"root"},"keys":[{"time":0},{"time":0.5,"translation":[0,0.1,0]}]},
            {"target":{"type":"joint","id":"skin"},"keys":[{"time":0},{"time":0.5,"translation":[0,0.1,0]}]},
            {"target":{"type":"joint","id":"other"},"keys":[{"time":0},{"time":0.5,"translation":[0,0.4,0]}]}],
        "morph_tracks":[
            {"deformer":"body-deformer","blendshape":"lift","keys":[{"time":0,"weight":0},{"time":0.5,"weight":1}]},
            {"deformer":"unrelated-deformer","blendshape":"unused","keys":[{"time":0,"weight":0},{"time":0.5,"weight":0.9}]}]}])).unwrap();
    let mut d = Document { objects, joints, deformers, clips, ..Document::default() };
    cloth::create_panel(&mut d, &panel("cloth")).unwrap();
    d
}
fn min_height(value: &Value) -> f64 {
    value["vertices"].as_array().unwrap().iter().map(|p| p[1].as_f64().unwrap()).fold(f64::INFINITY, f64::min)
}

#[test]
fn actual_lbs_and_dqs_body_fields_drive_cloth_and_both_morph_and_skin_matter() {
    for method in [SkinningMethod::LinearBlend, SkinningMethod::DualQuaternion] {
        let mut d = fixture(method);
        let authored = serde_json::to_value(&d.objects).unwrap();
        let result = cloth::bake(&mut d, &bake_request()).unwrap();
        assert_eq!(authored, serde_json::to_value(&d.objects).unwrap());
        assert_eq!(result["source_deformers"], 1);
        assert_eq!(result["source_objects"], 1);
        assert_eq!(result["deformation_evaluations"], 121);
        assert_eq!(result["deformation_kernel_work_per_evaluation"], 14);
        assert_eq!(result["deformation_rebuild_work_per_evaluation"], 6);
        assert_eq!(result["estimated_deformation_work"], 2420);
        assert!(result["diagnostics"]["contact_projections"].as_u64().unwrap() > 0);
        let state = cloth::inspect(&d, "cloth", Some(&sample(0.5))).unwrap();
        let top = min_height(&state);
        assert!(top > 0.35, "actual morph-plus-skin collider did not raise cloth: {top}");
        let (scene, _) = d.compile_at(&Pass::Beauty, Some(&sample(0.5))).unwrap();
        let Prim::Surface { id } = scene.objects[0].prim else { panic!("body field was not retained") };
        let surface = &scene.surfaces[id as usize];
        assert!(surface.vertices().iter().all(|p| (p.y - 0.35).abs() < 1e-6));
        assert!((surface.half_thickness() - 0.01).abs() < 1e-7);
        for p in state["vertices"].as_array().unwrap() {
            let p =
                Vec3::new(p[0].as_f64().unwrap() as f32, p[1].as_f64().unwrap() as f32, p[2].as_f64().unwrap() as f32);
            let actual = scene.sample_object(0, p).unwrap().dist;
            assert!(actual >= 0.001 - 1e-5, "native body shell penetration: {actual}");
            assert!(scene.sample_object(2, p).unwrap().dist < 0.0, "cloth cache did not reach rendered triangle field");
        }
        let mut frozen = fixture(method);
        frozen.deformers.clear();
        frozen.clips[0].morph_tracks.clear();
        let frozen_result = cloth::bake(&mut frozen, &bake_request()).unwrap();
        let frozen_height = min_height(&cloth::inspect(&frozen, "cloth", Some(&sample(0.5))).unwrap());
        assert!((frozen_height - 0.05).abs() < 1e-6);
        assert_eq!(
            result["estimated_work"].as_u64().unwrap() - frozen_result["estimated_work"].as_u64().unwrap(),
            2420
        );
        let mut skin_only = fixture(method);
        skin_only.deformers[0].blendshapes.clear();
        skin_only.clips[0].morph_tracks.retain(|t| t.deformer != "body-deformer");
        cloth::bake(&mut skin_only, &bake_request()).unwrap();
        let skin_height = min_height(&cloth::inspect(&skin_only, "cloth", Some(&sample(0.5))).unwrap());
        assert!((top - skin_height - 0.15).abs() < 0.002);
        let mut morph_only = fixture(method);
        morph_only.deformers[0].joints.clear();
        morph_only.deformers[0].weights.clear();
        cloth::bake(&mut morph_only, &bake_request()).unwrap();
        let morph_height = min_height(&cloth::inspect(&morph_only, "cloth", Some(&sample(0.5))).unwrap());
        assert!((top - morph_height - 0.2).abs() < 0.002);
        println!("{method:?}: actual cloth y={top}, frozen={frozen_height}, skin-only={skin_height}, morph-only={morph_height}, contact projections={}",result["diagnostics"]["contact_projections"]);
    }
}

#[test]
fn relevant_deformation_changes_stale_cache_but_unrelated_sources_do_not() {
    let mut d = fixture(SkinningMethod::LinearBlend);
    cloth::bake(&mut d, &bake_request()).unwrap();
    let base = cloth::inspect(&d, "cloth", Some(&sample(0.5))).unwrap();
    type Edit = Box<dyn Fn(&mut Document)>;
    let relevant: Vec<Edit> = vec![
        Box::new(|d| d.deformers[0].weights[0][0].joint = 1),
        Box::new(|d| d.deformers[0].blendshapes[0].deltas[0][1] += 0.01),
        Box::new(|d| d.clips[0].morph_tracks[0].keys[1].weight = 0.8),
        Box::new(|d| d.clips[0].tracks[0].keys[1].translation[1] = 0.12),
        Box::new(|d| d.clips[0].tracks[1].keys[1].translation[1] = 0.12),
        Box::new(|d| d.deformers[0].method = SkinningMethod::DualQuaternion),
    ];
    for edit in relevant {
        let mut changed = d.clone();
        edit(&mut changed);
        changed.compile(&Pass::Beauty).unwrap();
        assert_eq!(cloth::inspect(&changed, "cloth", None).unwrap()["cache"]["fresh"], false);
        assert!(changed.compile_at(&Pass::Beauty, Some(&sample(0.5))).err().unwrap().contains("stale"));
    }
    let mut unrelated = d.clone();
    unrelated.deformers[1].weights[0][0].joint = 1;
    unrelated.deformers[1].blendshapes[0].deltas[0][2] += 0.01;
    unrelated.clips[0].morph_tracks[1].keys[1].weight = 0.7;
    unrelated.clips[0].tracks[2].keys[1].translation[1] = 0.5;
    unrelated.camera.eye = [3.0, 3.0, 6.0];
    unrelated.lights[0].color = [9.0, 2.0, 1.0];
    unrelated.clips[0].camera_keys = serde_json::from_value(json!([
        {"time":0,"eye":[3,3,6],"target":[0,0,0],"fov_degrees":40},
        {"time":0.5,"eye":[4,3,6],"target":[0,0,0],"fov_degrees":42}]))
    .unwrap();
    let fresh = cloth::inspect(&unrelated, "cloth", Some(&sample(0.5))).unwrap();
    assert_eq!(fresh["cache"]["fresh"], true);
    assert_eq!(base["vertices"], fresh["vertices"]);
    assert_eq!(base["cache"]["source_fnv1a64"], fresh["cache"]["source_fnv1a64"]);
    let saved = serde_json::to_vec(&unrelated).unwrap();
    let loaded: Document = serde_json::from_slice(&saved).unwrap();
    assert_eq!(fresh, cloth::inspect(&loaded, "cloth", Some(&sample(0.5))).unwrap());
}

#[test]
fn rest_object_transform_and_scaled_shell_thickness_reach_physical_contact() {
    let mut d = fixture(SkinningMethod::DualQuaternion);
    d.objects[0].position = [0.0, 0.2, 0.0];
    d.objects[0].scale = 2.0;
    let mut rest = panel("cloth");
    rest.origin[1] = 0.25;
    cloth::update_panel(&mut d, &rest).unwrap();
    cloth::bake(&mut d, &bake_request()).unwrap();
    let state = cloth::inspect(&d, "cloth", Some(&sample(0.5))).unwrap();
    let (scene, _) = d.compile_at(&Pass::Beauty, Some(&sample(0.5))).unwrap();
    let Prim::Surface { id } = scene.objects[0].prim else { panic!("expected actual deformed body field") };
    let body = &scene.surfaces[id as usize];
    // Rest translation .2 + object-scaled morph .15*2 + two joint translations .1+.1.
    assert!(body.vertices().iter().all(|p| (p.y - 0.7).abs() < 1e-6));
    assert!((body.half_thickness() - 0.02).abs() < 1e-7);
    assert!((scene.objects[0].xform.scale - 1.0).abs() < 1e-7);
    assert!((min_height(&state) - 0.723).abs() < 0.002, "scaled physical shell was dropped: {state}");
}

#[test]
fn pins_to_deformed_surfaces_are_explicitly_rejected_without_partial_authoring() {
    let mut d = fixture(SkinningMethod::LinearBlend);
    let before = serde_json::to_vec(&d).unwrap();
    let mut request = panel("pinned");
    request.pins =
        serde_json::from_value(json!([{"vertex":0,"target_object":"body","point":[-0.2,0.05,-0.2]}])).unwrap();
    let error = cloth::create_panel(&mut d, &request).unwrap_err();
    assert!(error.contains("deformed surface"), "{error}");
    assert_eq!(before, serde_json::to_vec(&d).unwrap());
}

#[test]
fn scoped_body_sample_works_with_unbaked_cloth_and_full_render_still_requires_bake() {
    let d = fixture(SkinningMethod::LinearBlend);
    let root = std::env::temp_dir().join(format!(
        "mm3e-deform-cloth-scoped-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir(&root).unwrap();
    std::fs::write(
        root.join("project.json"),
        serde_json::to_vec(&json!({"format":"mm3e-agent-project-v1","saved_revision":0,"document":d})).unwrap(),
    )
    .unwrap();
    let mut editor = Editor::open_project(&root, "project.json").unwrap();
    let call = |editor: &mut Editor, command: Value| {
        editor.handle(serde_json::from_value::<Request>(json!({"id":"probe","command":command})).unwrap())
    };
    let scoped = call(&mut editor, json!({"op":"sample","id":"body","points":[[0,0.35,0]],"animation":sample(0.5)}));
    assert!(scoped.ok, "{:?}", scoped.error);
    assert!((scoped.result.unwrap()["samples"][0]["value"].as_f64().unwrap() + 0.01).abs() < 1e-5);
    let full = call(&mut editor, json!({"op":"render","path":"must-not-exist.png","animation":sample(0.5)}));
    assert!(!full.ok);
    assert!(full.error.unwrap().message.contains("bake_cloth"));
    assert!(!root.join("must-not-exist.png").exists());
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn authentic_pre_deformation_v3_and_v4_projects_keep_cache_integrity_and_freshness() {
    for bytes in [
        include_bytes!("fixtures/pre_deform_cloth_v3.json").as_slice(),
        include_bytes!("fixtures/pre_deform_cloth_v4.json").as_slice(),
    ] {
        let saved: Value = serde_json::from_slice(bytes).unwrap();
        assert!(saved["document"].get("deformers").is_none());
        let d: Document = serde_json::from_value(saved["document"].clone()).unwrap();
        let cache = d.cloths[0].cache.as_ref().unwrap();
        let id = d.cloths[0].id.clone();
        let sample = AnimationSample { clip: cache.clip.clone(), time: cache.duration, playback: Playback::Clamp };
        let info = cloth::inspect(&d, &id, Some(&sample)).unwrap();
        assert_eq!(info["cache"]["fresh"], true, "authentic {} cache unexpectedly stale", cache.algorithm);
        assert_eq!(info["cache"]["source_fnv1a64"], cache.source_fnv1a64);
        assert_eq!(info["cache"]["frames_fnv1a64"], cache.frames_fnv1a64);
        d.compile_at(&Pass::Beauty, Some(&sample)).unwrap();
    }
}
