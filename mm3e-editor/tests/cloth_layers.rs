//! Layered animation must reach the actual cloth attachment/contact boundary and
//! its persistent source fingerprint without retaining unrelated shot channels.
use mm3e_editor::{
    animation::{AnimationSample, Playback},
    cloth::{self, BakeClothRequest, ClothPanelRequest},
    face::{self, FaceRequest},
    model::{array, vec, Document, Pass},
    Editor,
};
use mm3e_kit::Vec3;
use serde_json::json;

const DT: f32 = 1.0 / 24.0;
const DURATION: f32 = 4.0 * DT;

fn sample(time: f32) -> AnimationSample {
    AnimationSample { clip: "shot".into(), time, playback: Playback::Clamp }
}
fn request() -> BakeClothRequest {
    BakeClothRequest { id: "patch".into(), clip: "shot".into(), duration: None }
}
fn source() -> Document {
    Document {
        objects: serde_json::from_value(json!([
            {"id":"anchor","shape":{"type":"sphere","radius":0.05}},
            {"id":"scenery","shape":{"type":"sphere","radius":0.1},"position":[8,0,0]}
        ]))
        .unwrap(),
        joints: serde_json::from_value(json!([
            {"id":"parent","pivot":[0,0,0]},
            {"id":"child","parent":"parent","pivot":[0,0,0],"objects":["anchor"]}
        ]))
        .unwrap(),
        clips: serde_json::from_value(json!([
            {"id":"shot","duration":DURATION,"layers":[
                {"id":"body","clip":"middle","mask":[{"type":"joint","id":"parent"}]},
                {"id":"camera","clip":"camera"}
            ]},
            {"id":"middle","duration":DURATION,"layers":[{"id":"motion","clip":"driver"}]},
            {"id":"driver","duration":DURATION,"tracks":[
                {"target":{"type":"joint","id":"parent"},"keys":[{"time":0},{"time":DURATION,"translation":[0.04,0,0]}]},
                {"target":{"type":"joint","id":"child"},"keys":[{"time":0},{"time":DURATION,"translation":[0,0.2,0]}]},
                {"target":{"type":"object","id":"scenery"},"keys":[{"time":0},{"time":DURATION,"translation":[0,1,0]}]}
            ]},
            {"id":"camera","duration":DURATION,"camera_keys":[
                {"time":0,"eye":[0,2,5],"target":[0,1,0],"fov_degrees":40},
                {"time":DURATION,"eye":[1,2,5],"target":[0,1,0],"fov_degrees":45}
            ]}
        ]))
        .unwrap(),
        ..Document::default()
    }
}
fn add_pins(document: &mut Document) {
    let (scene, _) = document.compile_at(&Pass::Beauty, Some(&sample(0.0))).unwrap();
    let transform = scene.objects[0].xform;
    let locals = [[0., 1., 0.], [1., 1., 0.], [0., 1., 1.], [1., 1., 1.]];
    let panel: ClothPanelRequest = serde_json::from_value(json!({
        "id":"patch","origin":array(transform.to_world(Vec3::new(0.5,1.0,0.5))),
        "axis_u":array(transform.rot.mul_vec(Vec3::new(1.,0.,0.))),
        "axis_v":array(transform.rot.mul_vec(Vec3::new(0.,0.,1.))),
        "segments":[1,1],"width_m":transform.scale,"height_m":transform.scale,
        "thickness_m":0.004,"vertex_mass_kg":0.02,
        "pins":locals.iter().enumerate().map(|(i,point)|json!({"vertex":i,"target_object":"anchor","point":point})).collect::<Vec<_>>(),
        "settings":{"fixed_dt":DT,"substeps":2,"iterations":2,"gravity":[0,0,0],"self_collision":false}
    }))
    .unwrap();
    cloth::create_panel(document, &panel).unwrap();
}
fn baked() -> Document {
    let mut document = source();
    add_pins(&mut document);
    cloth::bake(&mut document, &request()).unwrap();
    document
}
fn fresh(document: &Document) -> bool {
    cloth::inspect(document, "patch", None).unwrap()["cache"]["fresh"].as_bool().unwrap()
}

#[test]
fn nested_masked_parent_motion_reaches_child_pins_and_native_reload() {
    let document = baked();
    let cache = document.cloths[0].cache.as_ref().unwrap();
    for frame in &cache.frames {
        // Independent expected translation: the parent channel survives the
        // root mask and must still propagate into the unselected child joint.
        let dx = 0.04 * frame.time / DURATION;
        for pin in &document.cloths[0].pins {
            let expected = vec(pin.point) + Vec3::new(dx, 0., 0.);
            assert!((vec(frame.vertices[pin.vertex as usize]) - expected).length() < 5e-6);
        }
    }
    let saved = serde_json::to_vec(&document).unwrap();
    let loaded: Document = serde_json::from_slice(&saved).unwrap();
    assert!(fresh(&loaded));
    assert_eq!(
        cloth::inspect(&document, "patch", Some(&sample(DURATION))).unwrap(),
        cloth::inspect(&loaded, "patch", Some(&sample(DURATION))).unwrap()
    );
    loaded.compile_at(&Pass::Beauty, Some(&sample(DURATION))).unwrap();
}

#[test]
fn contributing_child_values_and_layer_timing_weights_references_stale_cache() {
    let original = baked();
    type Edit = Box<dyn Fn(&mut Document)>;
    let edits: Vec<Edit> = vec![
        Box::new(|d| d.clips[2].tracks[0].keys[1].translation[0] += 0.01),
        Box::new(|d| d.clips[1].layers[0].time_scale = 0.5),
        Box::new(|d| d.clips[1].layers[0].source_start = 0.01),
        Box::new(|d| d.clips[1].layers[0].start = DT),
        Box::new(|d| d.clips[1].layers[0].end = Some(DT)),
        Box::new(|d| d.clips[1].layers[0].weight = 0.5),
        Box::new(|d| d.clips[1].layers[0].playback = Playback::Loop),
        Box::new(|d| {
            d.clips[1].layers[0].weight_keys =
                serde_json::from_value(json!([{"time":0,"value":1},{"time":DURATION,"value":0.5}])).unwrap();
        }),
        Box::new(|d| {
            d.clips[1].layers[0].mode = mm3e_editor::layering::BlendMode::Additive;
            d.clips[1].layers[0].reference_time = Some(DT);
        }),
    ];
    for (i, edit) in edits.into_iter().enumerate() {
        let mut changed = original.clone();
        edit(&mut changed);
        assert!(!fresh(&changed), "physical layered edit {i} did not stale cache");
        assert!(changed.compile_at(&Pass::Beauty, Some(&sample(DURATION))).err().unwrap().contains("stale"));
    }
}

#[test]
fn masked_unrelated_and_disabled_branches_do_not_stale_cache() {
    let mut document = baked();
    let before = serde_json::to_value(&document.cloths[0].cache.as_ref().unwrap().frames).unwrap();
    document.clips[2].tracks[1].keys[1].translation[1] = 0.5;
    document.clips[2].tracks[2].keys[1].translation[1] = 2.0;
    document.clips[3].camera_keys[1].eye[0] = 4.0;
    document.clips[0].layers[1].time_scale = 0.2;
    document.clips[0].layers[0].mask.as_mut().unwrap().push(mm3e_editor::layering::Channel::Camera);
    document.clips[0]
        .layers
        .push(serde_json::from_value(json!({"id":"disabled","clip":"driver","enabled":false})).unwrap());
    assert!(fresh(&document));
    let state = cloth::inspect(&document, "patch", Some(&sample(DURATION))).unwrap();
    assert_eq!(state["cache"]["fresh"], true);
    assert_eq!(before, serde_json::to_value(&document.cloths[0].cache.as_ref().unwrap().frames).unwrap());
}

#[test]
fn direct_overrides_shadow_child_channels_but_ordered_shared_references_still_matter() {
    let mut document = source();
    document.clips[0].layers[0].mask = None;
    document.clips[0].layers.insert(
        1,
        serde_json::from_value(json!({
            "id":"second","clip":"driver","weight":0.5,"source_start":DT,
            "mask":[{"type":"joint","id":"parent"}]
        }))
        .unwrap(),
    );
    // The second occurrence of the shared source has a different time and
    // weight. Initial geometry is authored from that actual assembled pose.
    add_pins(&mut document);
    cloth::bake(&mut document, &request()).unwrap();
    let original = document.clone();
    document.clips[0].layers.swap(0, 1);
    assert!(!fresh(&document));
    document = original;
    document.clips[0].tracks = serde_json::from_value(json!([
        {"target":{"type":"joint","id":"parent"},"keys":[{"time":0},{"time":DURATION,"translation":[0.01,0,0]}]},
        {"target":{"type":"joint","id":"child"},"keys":[{"time":0},{"time":DURATION}]}
    ]))
    .unwrap();
    // Reauthor rest geometry because the root-owned parent track replaces the
    // old nonidentity time-zero source sample.
    document.cloths.clear();
    document.objects.retain(|o| o.id != "patch");
    add_pins(&mut document);
    cloth::bake(&mut document, &request()).unwrap();
    document.clips[2].tracks[0].keys[1].translation[0] = 0.09;
    document.clips[2].tracks[1].keys[1].translation[1] = 0.9;
    assert!(fresh(&document), "root direct channels must shadow their assembled inputs");
    document.clips[0].tracks[0].keys[1].translation[0] += 0.01;
    assert!(!fresh(&document));
}

#[test]
fn projected_bake_and_full_pose_honor_direct_override_of_overflowing_layers() {
    let mut document = source();
    document.clips = serde_json::from_value(json!([
        {"id":"scale","duration":DT,"tracks":[
            {"target":{"type":"object","id":"anchor"},"keys":[{"time":0,"scale":1000}]}
        ]},
        {"id":"shot","duration":DT,
            "layers":(0..14).map(|i|json!({"id":format!("scale-{i}"),"clip":"scale","mode":"additive"})).collect::<Vec<_>>(),
            "tracks":[{"target":{"type":"object","id":"anchor"},"keys":[{"time":0}]}]
        }
    ])).unwrap();
    // The root-owned identity channel is the final authority. Its fully pinned
    // cloth must remain at rest; unused scale products must never be evaluated.
    let locals = [[0., 1., 0.], [1., 1., 0.], [0., 1., 1.], [1., 1., 1.]];
    let panel: ClothPanelRequest = serde_json::from_value(json!({
        "id":"patch","origin":[0.5,1,0.5],"axis_u":[1,0,0],"axis_v":[0,0,1],
        "segments":[1,1],"width_m":1,"height_m":1,"thickness_m":0.004,"vertex_mass_kg":0.02,
        "pins":locals.iter().enumerate().map(|(i,point)|json!({"vertex":i,"target_object":"anchor","point":point})).collect::<Vec<_>>(),
        "settings":{"fixed_dt":DT,"substeps":1,"iterations":1,"gravity":[0,0,0],"self_collision":false}
    })).unwrap();
    cloth::create_panel(&mut document, &panel).unwrap();
    cloth::bake(&mut document, &request()).unwrap();
    for frame in &document.cloths[0].cache.as_ref().unwrap().frames {
        assert_eq!(frame.vertices, locals);
        let (scene, _) = document.compile_at(&Pass::Beauty, Some(&sample(frame.time))).unwrap();
        assert_eq!(scene.objects[0].xform.scale, 1.0);
        assert_eq!(scene.objects[0].xform.pos, Vec3::ZERO);
        let state = cloth::inspect(&document, "patch", Some(&sample(frame.time))).unwrap();
        assert_eq!(state["vertices"], json!(locals));
    }
}

#[test]
fn layered_collider_motion_drives_actual_contact() {
    let mut document = Document {
        objects: serde_json::from_value(json!([
            {"id":"body","shape":{"type":"box","half_extents":[2,0.01,2]},"position":[0,-0.01,0]}
        ]))
        .unwrap(),
        clips: serde_json::from_value(json!([
            {"id":"shot","duration":DURATION,"layers":[{"id":"rise","clip":"driver"}]},
            {"id":"driver","duration":DURATION,"tracks":[
                {"target":{"type":"object","id":"body"},"keys":[{"time":0},{"time":DURATION,"translation":[0,0.12,0]}]}
            ]}
        ]))
        .unwrap(),
        ..Document::default()
    };
    let panel: ClothPanelRequest = serde_json::from_value(json!({
        "id":"patch","origin":[0,0.05,0],"axis_u":[1,0,0],"axis_v":[0,0,1],
        "segments":[2,2],"width_m":0.4,"height_m":0.4,"thickness_m":0.002,"vertex_mass_kg":0.02,
        "collision_object_ids":["body"],"settings":{"fixed_dt":DT,"substeps":8,"iterations":24,
        "gravity":[0,0,0],"self_collision":false,"collision_thickness":0.003,"max_penetration_m":0.002}
    }))
    .unwrap();
    cloth::create_panel(&mut document, &panel).unwrap();
    let result = cloth::bake(&mut document, &request()).unwrap();
    assert!(result["diagnostics"]["contact_projections"].as_u64().unwrap() > 0);
    let state = cloth::inspect(&document, "patch", Some(&sample(DURATION))).unwrap();
    let (scene, _) = document.compile_at(&Pass::Beauty, Some(&sample(DURATION))).unwrap();
    for point in state["vertices"].as_array().unwrap() {
        let p: [f32; 3] = serde_json::from_value(point.clone()).unwrap();
        assert!(p[1] > 0.12, "layered body did not raise cloth: {p:?}");
        assert!(scene.sample_object(0, vec(p)).unwrap().dist >= 0.001 - 1e-5);
    }
    document.clips[1].tracks[0].keys[1].translation[1] += 0.01;
    assert!(!fresh(&document));
}

#[test]
fn nested_skin_and_morph_channels_reach_cloth_contact_with_additive_reference() {
    let mut document = Document {
        objects: serde_json::from_value(json!([
            {"id":"body","shape":{"type":"surface","vertices":[[-2,0,-2],[2,0,-2],[2,0,2],[-2,0,2]],
                "triangles":[[0,2,1],[0,3,2]],"thickness_m":0.02}}
        ]))
        .unwrap(),
        joints: serde_json::from_value(json!([
            {"id":"parent","pivot":[0,0,0]},
            {"id":"skin","parent":"parent","pivot":[0,0,0]}
        ]))
        .unwrap(),
        deformers: serde_json::from_value(json!([
            {"id":"body-skin","object":"body","joints":["skin"],
                "weights":vec![json!([{"joint":0,"weight":1}]);4],
                "blendshapes":[{"id":"lift","deltas":vec![[0.0,0.1,0.0];4]}]}
        ]))
        .unwrap(),
        clips: serde_json::from_value(json!([
            {"id":"shot","duration":DURATION,"layers":[{"id":"assembly","clip":"middle"}]},
            {"id":"middle","duration":DURATION,"layers":[{"id":"pose","clip":"driver",
                "mode":"additive","reference_time":0,"mask":[
                    {"type":"joint","id":"parent"},{"type":"morph","deformer":"body-skin","blendshape":"lift"}
                ]}]},
            {"id":"driver","duration":DURATION,"tracks":[
                {"target":{"type":"joint","id":"parent"},"keys":[
                    {"time":0,"translation":[0,0.1,0]},{"time":DURATION,"translation":[0,0.2,0]}]}
            ],"morph_tracks":[{"deformer":"body-skin","blendshape":"lift","keys":[
                {"time":0,"weight":0.2},{"time":DURATION,"weight":1}]}]}
        ]))
        .unwrap(),
        ..Document::default()
    };
    let panel: ClothPanelRequest = serde_json::from_value(json!({
        "id":"patch","origin":[0,0.05,0],"axis_u":[1,0,0],"axis_v":[0,0,1],
        "segments":[2,2],"width_m":0.4,"height_m":0.4,"thickness_m":0.002,"vertex_mass_kg":0.02,
        "collision_object_ids":["body"],"settings":{"fixed_dt":DT,"substeps":8,"iterations":24,
            "gravity":[0,0,0],"self_collision":false,"collision_thickness":0.003,"max_penetration_m":0.002}
    }))
    .unwrap();
    cloth::create_panel(&mut document, &panel).unwrap();
    cloth::bake(&mut document, &request()).unwrap();
    let (scene, _) = document.compile_at(&Pass::Beauty, Some(&sample(DURATION))).unwrap();
    let mm3e_orchestrator::Prim::Surface { id } = scene.objects[0].prim else { panic!("missing body surface") };
    // Independent expected body pose: (.2 - .1) joint translation plus
    // (1 - .2) * .1 morph displacement = .18 meters.
    assert!(scene.surfaces[id as usize].vertices().iter().all(|point| (point.y - 0.18).abs() < 1e-6));
    let state = cloth::inspect(&document, "patch", Some(&sample(DURATION))).unwrap();
    for point in state["vertices"].as_array().unwrap() {
        let p: [f32; 3] = serde_json::from_value(point.clone()).unwrap();
        assert!((p[1] - 0.193).abs() < 0.002, "morph/skin collider missed cloth: {p:?}");
        assert!(scene.sample_object(0, vec(p)).unwrap().dist >= 0.001 - 1e-5);
    }
    let mut reference_changed = document.clone();
    reference_changed.clips[1].layers[0].reference_time = Some(DT);
    assert!(!fresh(&reference_changed));
    document.clips[2].morph_tracks[0].keys[0].weight = 0.3;
    assert!(!fresh(&document), "source reference-pose key must participate in freshness");
}

#[test]
fn facial_layers_are_physical_only_when_selected_geometry_depends_on_face() {
    let mut editor = Editor::new(&std::env::temp_dir()).unwrap();
    let response = editor.handle(
        serde_json::from_value(json!({"id":"character","expected_revision":0,"command":{
            "op":"apply","operations":[{"op":"create_humanoid","id":"hero","height":1.8,"build":1.0,"head_scale":1.0}]
        }}))
        .unwrap(),
    );
    assert!(response.ok, "{response:?}");
    let mut document = editor.document().clone();
    face::create(&mut document, &FaceRequest { id: "face".into(), character: "hero".into() }).unwrap();
    document.clips = serde_json::from_value(json!([
        {"id":"shot","duration":DURATION,"layers":[{"id":"performance","clip":"face-clip"}]},
        {"id":"face-clip","duration":DURATION,"face_tracks":[
            {"face":"face","channel":"jaw_open","keys":[{"time":0,"value":0},{"time":DURATION,"value":0.2}]}
        ]}
    ]))
    .unwrap();
    for collider in ["hero/head", "hero/left_foot"] {
        let mut d = document.clone();
        d.objects.iter_mut().find(|object| object.id == collider).unwrap().combine =
            mm3e_editor::model::Combination::Union;
        let panel: ClothPanelRequest = serde_json::from_value(json!({
            "id":"patch","origin":[3,3,3],"axis_u":[1,0,0],"axis_v":[0,0,1],
            "segments":[1,1],"width_m":0.2,"height_m":0.2,"thickness_m":0.004,"vertex_mass_kg":0.02,
            "collision_object_ids":[collider],"settings":{"fixed_dt":DT,"substeps":1,"iterations":1,
                "gravity":[0,0,0],"self_collision":false}
        }))
        .unwrap();
        cloth::create_panel(&mut d, &panel).unwrap();
        cloth::bake(&mut d, &request()).unwrap();
        d.clips[1].face_tracks[0].keys[1].value = 0.4;
        assert_eq!(fresh(&d), collider != "hero/head", "incorrect face dependency for {collider}");
    }
}

#[test]
fn authentic_flat_v3_v4_cache_fingerprints_survive_unrelated_layer_additions() {
    for bytes in [
        include_bytes!("fixtures/pre_deform_cloth_v3.json").as_slice(),
        include_bytes!("fixtures/pre_deform_cloth_v4.json").as_slice(),
    ] {
        let saved: serde_json::Value = serde_json::from_slice(bytes).unwrap();
        let mut document: Document = serde_json::from_value(saved["document"].clone()).unwrap();
        let cache = document.cloths[0].cache.as_ref().unwrap().clone();
        assert!(cloth::cache_fresh(&document, &document.cloths[0]).unwrap());
        document.clips.push(
            serde_json::from_value(json!({"id":"only-camera","duration":cache.duration,
                "camera_keys":[{"time":0,"eye":[0,2,5],"target":[0,1,0],"fov_degrees":40}]
            }))
            .unwrap(),
        );
        document
            .clips
            .iter_mut()
            .find(|clip| clip.id == cache.clip)
            .unwrap()
            .layers
            .push(serde_json::from_value(json!({"id":"camera-layer","clip":"only-camera"})).unwrap());
        assert!(cloth::cache_fresh(&document, &document.cloths[0]).unwrap(), "{} flat hash changed", cache.algorithm);
        let animation = AnimationSample { clip: cache.clip, time: cache.duration, playback: Playback::Clamp };
        let state = cloth::inspect(&document, &document.cloths[0].id, Some(&animation)).unwrap();
        assert_eq!(state["cache"]["source_fnv1a64"], cache.source_fnv1a64);
        assert_eq!(state["cache"]["frames_fnv1a64"], cache.frames_fnv1a64);
        document.compile_at(&Pass::Beauty, Some(&animation)).unwrap();
    }
}
