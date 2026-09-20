use mm3e_editor::{
    animation::{AnimationSample, Playback},
    cloth::{self, BakeClothRequest, ClothPanelRequest},
    model::{Document, Pass},
    protocol::Request,
    Editor,
};
use mm3e_kit::Vec3;
use serde_json::{json, Value};

fn document() -> Document {
    let mut document = Document::default();
    document.settings.width = 48;
    document.settings.height = 40;
    document.settings.shadows = false;
    document.settings.ao = false;
    document.objects = serde_json::from_value(json!([
        {"id":"floor","shape":{"type":"plane","normal":[0,1,0],"offset":-0.85},"material":{"albedo":[0.2,0.2,0.2]}},
        {"id":"anchor","shape":{"type":"box","half_extents":[0.45,0.03,0.03]},"position":[0,1,-0.4]}
    ]))
    .unwrap();
    document.clips = serde_json::from_value(json!([{"id":"move","duration":1,"tracks":[
        {"target":{"type":"object","id":"anchor"},"keys":[{"time":0},{"time":1,"translation":[0.15,0.1,0]}]}
    ]}]))
    .unwrap();
    document
}
fn panel(id: &str, collide: bool) -> ClothPanelRequest {
    serde_json::from_value(json!({"id":id,"origin":[0,1,0],"axis_u":[1,0,0],"axis_v":[0,0,1],
        "segments":[4,4],"width_m":0.8,"height_m":0.8,"thickness_m":0.004,"vertex_mass_kg":0.02,
        "pins":[{"vertex":0,"target_object":"anchor","point":[-0.4,0,0]},
            {"vertex":4,"target_object":"anchor","point":[0.4,0,0]}],
        "collision_object_ids":if collide{vec!["floor"]}else{vec![]},
        "settings":{"iterations":24,"bend_compliance":0.05},"material":{"albedo":[0.8,0.08,0.1]}}))
    .unwrap()
}
fn request(id: &str) -> BakeClothRequest {
    BakeClothRequest { id: id.into(), clip: "move".into(), duration: None }
}
fn sample(time: f32) -> AnimationSample {
    AnimationSample { clip: "move".into(), time, playback: Playback::Clamp }
}
fn create() -> Document {
    let mut d = document();
    cloth::create_panel(&mut d, &panel("cape", true)).unwrap();
    d
}
fn vertices(value: &Value) -> Vec<Vec3> {
    value["vertices"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| Vec3::new(p[0].as_f64().unwrap() as f32, p[1].as_f64().unwrap() as f32, p[2].as_f64().unwrap() as f32))
        .collect()
}

#[test]
fn cloth_bake_deforms_rendered_surface_and_preserves_rest_sources() {
    let mut d = create();
    let sources = serde_json::to_value(&d.objects).unwrap();
    let result = cloth::bake(&mut d, &request("cape")).unwrap();
    assert_eq!(result["frames"], 61);
    assert_eq!(serde_json::to_value(&d.objects).unwrap(), sources);
    let info = cloth::inspect(&d, "cape", Some(&sample(1.0))).unwrap();
    let p = vertices(&info);
    assert!((p[0] - Vec3::new(-0.25, 1.1, -0.4)).length() < 1e-5);
    assert!(p[24].y < 0.95, "actual cloth must sag: {:?}", p[24]);
    let (scene, _) = d.compile_at(&Pass::Beauty, Some(&sample(1.0))).unwrap();
    assert!(scene.sample_object(2, p[12]).unwrap().dist < 0.0, "deformed vertex is on actual rendered cloth");
    assert!(
        scene.sample_object(2, Vec3::new(0.0, 1.0, 0.0)).unwrap().dist > 0.005,
        "rest center should leave the actual surface"
    );
}
#[test]
fn actual_contacts_hold_vertices_above_floor_and_no_collider_negative_control_falls_through() {
    let mut d = create();
    cloth::create_panel(&mut d, &panel("control", false)).unwrap();
    cloth::bake(&mut d, &request("cape")).unwrap();
    cloth::bake(&mut d, &request("control")).unwrap();
    let a = cloth::inspect(&d, "cape", Some(&sample(1.0))).unwrap();
    let b = cloth::inspect(&d, "control", Some(&sample(1.0))).unwrap();
    assert!(vertices(&a).iter().all(|p| p.y >= 0.853 - 0.002));
    assert!(vertices(&b).iter().any(|p| p.y < 0.80), "negative control: {:?}", vertices(&b));
    assert!(a["cache"]["diagnostics"]["contact_projections"].as_u64().unwrap() > 0);
    assert_eq!(b["cache"]["diagnostics"]["contact_projections"], 0);
}
#[test]
fn native_roundtrip_and_repeat_bake_reproduce_vertices_and_actual_render_pixels() {
    let mut d = create();
    cloth::bake(&mut d, &request("cape")).unwrap();
    let bytes = serde_json::to_vec(&d).unwrap();
    let restored: Document = serde_json::from_slice(&bytes).unwrap();
    let s = sample(0.537);
    assert_eq!(cloth::inspect(&d, "cape", Some(&s)).unwrap(), cloth::inspect(&restored, "cape", Some(&s)).unwrap());
    let (a, ca) = d.compile_at(&Pass::Beauty, Some(&s)).unwrap();
    let (b, cb) = restored.compile_at(&Pass::Beauty, Some(&s)).unwrap();
    let render_a = mm3e_orchestrator::render_linear(&a, &ca);
    assert_eq!(render_a, mm3e_orchestrator::render_linear(&b, &cb));
    let old = serde_json::to_value(&d.cloths[0].cache).unwrap();
    cloth::bake(&mut d, &request("cape")).unwrap();
    assert_eq!(old, serde_json::to_value(&d.cloths[0].cache).unwrap());
}
#[test]
fn edits_keep_document_authorable_but_reject_stale_render_until_actual_rebake() {
    let mut d = create();
    cloth::bake(&mut d, &request("cape")).unwrap();
    d.clips[0].tracks[0].keys[1].translation[0] = 0.25;
    d.compile(&Pass::Beauty).unwrap();
    assert!(d.compile_at(&Pass::Beauty, Some(&sample(0.5))).err().unwrap().contains("stale"));
    assert_eq!(cloth::inspect(&d, "cape", None).unwrap()["cache"]["fresh"], false);
    cloth::bake(&mut d, &request("cape")).unwrap();
    d.compile_at(&Pass::Beauty, Some(&sample(0.5))).unwrap();
}
#[test]
fn partial_bake_rejects_unbaked_time_and_missing_or_different_clip_cache() {
    let mut d = create();
    assert!(d.compile_at(&Pass::Beauty, Some(&sample(0.5))).err().unwrap().contains("requires bake_cloth"));
    cloth::bake(&mut d, &BakeClothRequest { duration: Some(0.5), ..request("cape") }).unwrap();
    d.compile_at(&Pass::Beauty, Some(&sample(0.5))).unwrap();
    assert!(d.compile_at(&Pass::Beauty, Some(&sample(0.75))).err().unwrap().contains("beyond baked duration"));
}
#[test]
fn rejected_bake_rest_pin_collision_and_budget_leave_document_exactly_unchanged() {
    let mut d = create();
    let before = serde_json::to_vec(&d).unwrap();
    assert!(cloth::bake(&mut d, &BakeClothRequest { duration: Some(0.501), ..request("cape") }).is_err());
    assert_eq!(before, serde_json::to_vec(&d).unwrap());
    d.objects[0].shape = serde_json::from_value(json!({"type":"plane","normal":[0,1,0],"offset":-1.1})).unwrap();
    let before = serde_json::to_vec(&d).unwrap();
    assert!(cloth::bake(&mut d, &request("cape")).unwrap_err().contains("penetration"));
    assert_eq!(before, serde_json::to_vec(&d).unwrap());
    d.objects[0].shape = serde_json::from_value(json!({"type":"plane","normal":[0,1,0],"offset":0})).unwrap();
    d.cloths[0].pins[0].point[0] -= 0.1;
    let before = serde_json::to_vec(&d).unwrap();
    assert!(cloth::bake(&mut d, &request("cape")).unwrap_err().contains("rest vertex"));
    assert_eq!(before, serde_json::to_vec(&d).unwrap());
}
#[test]
fn malformed_cache_and_derived_geometry_changes_are_rejected() {
    let mut d = create();
    cloth::bake(&mut d, &request("cape")).unwrap();
    let mut tampered = d.clone();
    tampered.cloths[0].cache.as_mut().unwrap().frames[1].vertices[0][0] += 0.1;
    assert!(cloth::validate(&tampered).unwrap_err().contains("integrity"));
    d.objects[2].position[1] += 0.1;
    assert!(cloth::validate(&d).unwrap_err().contains("derived"));
}
#[test]
fn selected_colliders_preserve_smooth_csg_and_reject_lone_subtractor() {
    let mut d = document();
    d.objects[0].shape = serde_json::from_value(json!({"type":"sphere","radius":1})).unwrap();
    d.objects[0].position = [-0.5, 0.0, 0.0];
    d.objects.push(serde_json::from_value(json!({"id":"smooth","shape":{"type":"sphere","radius":1},"position":[0.5,0,0],"combine":{"type":"smooth","radius":0.4}})).unwrap());
    let mut p = panel("cape", false);
    p.collision_object_ids = vec!["smooth".into(), "floor".into()];
    cloth::create_panel(&mut d, &p).unwrap();
    let info = cloth::inspect(&d, "cape", None).unwrap();
    // At panel center y=1, either sphere alone is outside; smooth union adds a
    // real bulge. Compare the complete selected authored fold independently.
    let (mut selected, _) = d.compile(&Pass::Beauty).unwrap();
    selected.objects = vec![selected.objects[0], selected.objects[2]];
    let expected = vertices(&info).iter().map(|&p| 0.003 - selected.sample_authored(p).dist).fold(0.0_f32, f32::max);
    assert!((info["sample_vertex_penetration_m"].as_f64().unwrap() as f32 - expected).abs() < 1e-6);
    d.objects[2].combine = serde_json::from_value(json!({"type":"subtract"})).unwrap();
    d.cloths[0].collision_object_ids = vec!["smooth".into()];
    assert!(cloth::validate(&d).unwrap_err().contains("first selected"));
}
#[test]
fn updating_panel_preserves_identity_order_and_explicit_material_but_invalidates_cache() {
    let mut d = create();
    cloth::bake(&mut d, &request("cape")).unwrap();
    d.objects.push(
        serde_json::from_value(json!({"id":"prop","shape":{"type":"sphere","radius":0.1},"position":[2,2,2]})).unwrap(),
    );
    let old_ids: Vec<_> = d.objects.iter().map(|o| o.id.clone()).collect();
    let before = serde_json::to_vec(&d).unwrap();
    let mut bad = panel("cape", true);
    bad.width_m = -1.0;
    assert!(cloth::update_panel(&mut d, &bad).is_err());
    assert_eq!(before, serde_json::to_vec(&d).unwrap());
    let mut changed = panel("cape", true);
    changed.settings.bend_compliance = 0.1;
    changed.material.albedo = [0.1, 0.2, 0.8];
    cloth::update_panel(&mut d, &changed).unwrap();
    assert_eq!(old_ids, d.objects.iter().map(|o| o.id.clone()).collect::<Vec<_>>());
    assert!(d.cloths[0].cache.is_none());
    assert_eq!(d.objects[2].material.albedo, [0.1, 0.2, 0.8]);
    assert!(d.compile_at(&Pass::Beauty, Some(&sample(0.5))).is_err());
}

#[test]
fn agent_operations_create_bake_and_inspect_cloth_through_revision_guarded_protocol() {
    let mut editor = Editor::new(&std::env::temp_dir()).unwrap();
    let d = document();
    let operations = vec![
        json!({"op":"create","object":d.objects[0]}),
        json!({"op":"create","object":d.objects[1]}),
        json!({"op":"put_clip","clip":d.clips[0]}),
        json!({"op":"create_cloth_panel","request":panel("cape",true)}),
    ];
    let create_request: Request = serde_json::from_value(
        json!({"id":"create","expected_revision":0,"command":{"op":"apply","operations":operations}}),
    )
    .unwrap();
    let response = editor.handle(create_request);
    assert!(response.ok, "{response:?}");
    assert_eq!(editor.revision(), 1);
    let before_dry_run = serde_json::to_vec(editor.document()).unwrap();
    let dry_request=serde_json::from_value(json!({"id":"dry","expected_revision":1,"command":{"op":"bake_cloth","request":request("cape"),"dry_run":true}})).unwrap();
    let dry = editor.handle(dry_request);
    assert!(dry.ok, "{dry:?}");
    assert_eq!(editor.revision(), 1);
    assert_eq!(before_dry_run, serde_json::to_vec(editor.document()).unwrap());
    let bake_request = serde_json::from_value(
        json!({"id":"bake","expected_revision":1,"command":{"op":"bake_cloth","request":request("cape")}}),
    )
    .unwrap();
    let baked = editor.handle(bake_request);
    assert!(baked.ok, "{baked:?}");
    assert_eq!(editor.revision(), 2);
    let request = serde_json::from_value(
        json!({"id":"inspect","command":{"op":"cloth_state","id":"cape","animation":sample(0.5)}}),
    )
    .unwrap();
    let response = editor.handle(request);
    assert!(response.ok, "{response:?}");
}

#[test]
fn rotating_parent_pins_follow_exact_midpoint_arc_in_inspection_and_rendered_geometry() {
    let mut d = Document {
        objects: serde_json::from_value(json!([{"id":"anchor","shape":{"type":"sphere","radius":0.1}}])).unwrap(),
        ..Document::default()
    };
    let dt = 1.0 / 24.0_f32;
    d.clips = serde_json::from_value(json!([{"id":"turn","duration":dt,"tracks":[{
        "target":{"type":"object","id":"anchor"},"keys":[{"time":0},{"time":dt,"rotation_degrees":[0,0,90]}]
    }]}]))
    .unwrap();
    let request: ClothPanelRequest =
        serde_json::from_value(json!({"id":"patch","origin":[1.5,0.5,0],"axis_u":[1,0,0],"axis_v":[0,1,0],
        "segments":[1,1],"width_m":1,"height_m":1,"thickness_m":0.004,"vertex_mass_kg":0.02,
        "pins":[{"vertex":0,"target_object":"anchor","point":[1,0,0]},
            {"vertex":1,"target_object":"anchor","point":[2,0,0]},
            {"vertex":2,"target_object":"anchor","point":[1,1,0]},
            {"vertex":3,"target_object":"anchor","point":[2,1,0]}],
        "settings":{"fixed_dt":dt,"substeps":4,"gravity":[0,0,0]}}))
        .unwrap();
    cloth::create_panel(&mut d, &request).unwrap();
    cloth::bake(&mut d, &BakeClothRequest { id: "patch".into(), clip: "turn".into(), duration: None }).unwrap();
    let before = serde_json::to_vec(&d).unwrap();
    let s = AnimationSample { clip: "turn".into(), time: dt * 0.5, playback: Playback::Clamp };
    let info = cloth::inspect(&d, "patch", Some(&s)).unwrap();
    let evaluated = vertices(&info);
    let expected = Vec3::new(0.5_f32.sqrt(), 0.5_f32.sqrt(), 0.0);
    assert!(
        (evaluated[0] - expected).length() < 2e-6,
        "pin followed cached chord instead of exact arc: {:?}",
        evaluated[0]
    );
    assert!((evaluated[0] - Vec3::new(0.5, 0.5, 0.0)).length() > 0.25);
    let (scene, _) = d.compile_at(&Pass::Beauty, Some(&s)).unwrap();
    let mm3e_orchestrator::Prim::Surface { id } = scene.objects[1].prim else {
        panic!("cloth must compile as actual triangle surface")
    };
    assert_eq!(scene.surfaces[id as usize].vertices(), evaluated);
    assert!(info["sample_max_relative_edge_error"].as_f64().unwrap() < 1e-6);
    assert_eq!(info["sample_within_bake_tolerances"], true);
    assert_eq!(serde_json::to_vec(&d).unwrap(), before);
}

#[test]
fn final_physical_substep_matches_declared_frame_time_for_endpoint_step_track() {
    let mut d = Document {
        objects: serde_json::from_value(json!([{"id":"anchor","shape":{"type":"sphere","radius":0.1}}])).unwrap(),
        ..Document::default()
    };
    let dt = 1.0 / 24.0_f32;
    let duration = 5.0 * dt;
    d.clips = serde_json::from_value(json!([{"id":"step","duration":duration,"tracks":[{
        "target":{"type":"object","id":"anchor"},"easing":"step",
        "keys":[{"time":0},{"time":duration,"translation":[0.2,0,0]}]
    }]}]))
    .unwrap();
    let request: ClothPanelRequest =
        serde_json::from_value(json!({"id":"patch","origin":[0,0,0],"axis_u":[1,0,0],"axis_v":[0,1,0],
        "segments":[1,1],"width_m":2,"height_m":2,"thickness_m":0.004,"vertex_mass_kg":0.02,
        "pins":[{"vertex":0,"target_object":"anchor","point":[-1,-1,0]},
            {"vertex":1,"target_object":"anchor","point":[1,-1,0]},
            {"vertex":2,"target_object":"anchor","point":[-1,1,0]},
            {"vertex":3,"target_object":"anchor","point":[1,1,0]}],
        "settings":{"fixed_dt":dt,"substeps":3,"iterations":1,"gravity":[0,0,0]}}))
        .unwrap();
    cloth::create_panel(&mut d, &request).unwrap();
    cloth::bake(&mut d, &BakeClothRequest { id: "patch".into(), clip: "step".into(), duration: None }).unwrap();
    let last = d.cloths[0].cache.as_ref().unwrap().frames.last().unwrap();
    assert_eq!(last.time, duration);
    assert!(
        (last.vertices[0][0] - -0.8).abs() < 1e-6,
        "cache endpoint omitted actual step motion: {:?}",
        last.vertices[0]
    );
    assert!(last.diagnostics.max_speed > 10.0, "cache must record the actual endpoint jump velocity");
}

#[test]
fn between_frame_vertex_collision_is_reported_for_debugging_and_rejected_for_rendering() {
    let mut d = Document {
        objects: serde_json::from_value(json!([{"id":"ball","shape":{"type":"sphere","radius":0.1}}])).unwrap(),
        ..Document::default()
    };
    let dt = 1.0 / 24.0_f32;
    d.clips = serde_json::from_value(json!([{"id":"cross","duration":dt,"tracks":[{
        "target":{"type":"object","id":"ball"},"keys":[{"time":0},{"time":dt,"translation":[2,0,0]}]
    }]}]))
    .unwrap();
    let request: ClothPanelRequest =
        serde_json::from_value(json!({"id":"patch","origin":[1.1,0.1,0],"axis_u":[1,0,0],"axis_v":[0,1,0],
        "segments":[1,1],"width_m":0.2,"height_m":0.2,"thickness_m":0.004,"vertex_mass_kg":0.02,
        "collision_object_ids":["ball"],"settings":{"fixed_dt":dt,"substeps":1,"gravity":[0,0,0]}}))
        .unwrap();
    cloth::create_panel(&mut d, &request).unwrap();
    cloth::bake(&mut d, &BakeClothRequest { id: "patch".into(), clip: "cross".into(), duration: None }).unwrap();
    let s = AnimationSample { clip: "cross".into(), time: dt * 0.5, playback: Playback::Clamp };
    let info = cloth::inspect(&d, "patch", Some(&s)).unwrap();
    assert!(info["sample_vertex_penetration_m"].as_f64().unwrap() > 0.09);
    assert_eq!(info["sample_within_bake_tolerances"], false);
    assert!(d.cloths[0].cache.as_ref().unwrap().diagnostics.max_contact_penetration < 1e-6);
    let error = d.compile_at(&Pass::Beauty, Some(&s)).err().unwrap();
    assert!(error.contains("sampled pose exceeds") && error.contains("smaller fixed_dt"), "{error}");
}

#[test]
fn contact_normals_remain_representable_under_large_permitted_world_translation() {
    fn run(offset: f32) -> Vec<Vec3> {
        let mut d = Document {
            objects: serde_json::from_value(
                json!([{"id":"floor","shape":{"type":"plane","normal":[0,1,0],"offset":-offset}}]),
            )
            .unwrap(),
            ..Document::default()
        };
        let dt = 1.0 / 60.0_f32;
        d.clips = serde_json::from_value(json!([{"id":"fall","duration":dt}])).unwrap();
        let request: ClothPanelRequest =
            serde_json::from_value(json!({"id":"patch","origin":[0,offset+1.0,0],"axis_u":[1,0,0],"axis_v":[0,0,1],
            "segments":[1,1],"width_m":1,"height_m":1,"thickness_m":0.004,"vertex_mass_kg":0.02,
            "collision_object_ids":["floor"],"settings":{"fixed_dt":dt,"substeps":1,"iterations":1}}))
            .unwrap();
        cloth::create_panel(&mut d, &request).unwrap();
        cloth::bake(&mut d, &BakeClothRequest { id: "patch".into(), clip: "fall".into(), duration: None }).unwrap();
        let info = cloth::inspect(
            &d,
            "patch",
            Some(&AnimationSample { clip: "fall".into(), time: dt, playback: Playback::Clamp }),
        )
        .unwrap();
        vertices(&info).into_iter().map(|p| p - Vec3::new(0.0, offset, 0.0)).collect()
    }
    let near = run(0.0);
    let far = run(5000.0);
    for (a, b) in near.iter().zip(far) {
        assert!((*a - b).length() < 0.001, "world translation exceeded expected f32 spatial precision: {a:?} vs {b:?}");
    }
}

#[test]
fn previous_solver_cache_loads_as_authorable_but_requires_rebake_for_playback() {
    let mut d = create();
    cloth::bake(&mut d, &request("cape")).unwrap();
    d.cloths[0].cache.as_mut().unwrap().algorithm = "xpbd-dihedral-vertex-contact-v1".into();
    let restored: Document = serde_json::from_slice(&serde_json::to_vec(&d).unwrap()).unwrap();
    restored.compile(&Pass::Beauty).unwrap();
    assert_eq!(cloth::inspect(&restored, "cape", None).unwrap()["cache"]["fresh"], false);
    assert!(restored.compile_at(&Pass::Beauty, Some(&sample(0.5))).err().unwrap().contains("stale for solver"));
}
