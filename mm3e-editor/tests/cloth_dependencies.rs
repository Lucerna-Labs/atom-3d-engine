//! Regressions for simulation-source pruning and stateless scene cloning.
use mm3e_editor::{
    animation::{AnimationSample, Interpolation, Playback},
    cloth::{self, BakeClothRequest, ClothPanelRequest},
    face::{self, FaceRequest},
    garment::{self, GarmentRequest, GarmentStyle},
    model::{array, vec, Document, Pass, Surface},
    Editor,
};
use mm3e_kit::Vec3;
use serde_json::json;

const DT: f32 = 1.0 / 24.0;

fn sample(time: f32) -> AnimationSample {
    AnimationSample { clip: "motion".into(), time, playback: Playback::Clamp }
}

fn request() -> BakeClothRequest {
    BakeClothRequest { id: "patch".into(), clip: "motion".into(), duration: None }
}

fn source() -> Document {
    let mut document = Document::default();
    document.settings.width = 16;
    document.settings.height = 16;
    document.objects = serde_json::from_value(json!([
        {"id":"floor","shape":{"type":"plane","normal":[0,1,0],"offset":2}},
        {"id":"anchor","shape":{"type":"sphere","radius":0.05}},
        {"id":"scenery","shape":{"type":"box","half_extents":[0.5,0.5,0.5]},"position":[10,0,0]}
    ]))
    .unwrap();
    document.clips = serde_json::from_value(json!([{"id":"motion","duration":DT,"tracks":[
        {"target":{"type":"object","id":"anchor"},"keys":[{"time":0},{"time":DT,"translation":[0.01,0,0]}]},
        {"target":{"type":"object","id":"scenery"},"keys":[{"time":0},{"time":DT,"translation":[0,1,0]}]}
    ],"camera_keys":[
        {"time":0,"eye":[0,2,5],"target":[0,1,0],"fov_degrees":40},
        {"time":DT,"eye":[1,2,5],"target":[0,1,0],"fov_degrees":45}
    ]}]))
    .unwrap();
    document
}

fn add_attached_panel(document: &mut Document, collision: bool) {
    // Derive the authored rest rectangle from the *independently compiled*
    // actual time-zero attachment transform, including nonidentity zero keys.
    let index = document.objects.iter().position(|o| o.id == "anchor").unwrap();
    let (scene, _) = document.compile_at(&Pass::Beauty, Some(&sample(0.0))).unwrap();
    let transform = scene.objects[index].xform;
    let locals = [[0., 1., 0.], [1., 1., 0.], [0., 1., 1.], [1., 1., 1.]];
    let panel: ClothPanelRequest = serde_json::from_value(json!({
        "id":"patch","origin":array(transform.to_world(Vec3::new(0.5,1.0,0.5))),
        "axis_u":array(transform.rot.mul_vec(Vec3::new(1.,0.,0.))),
        "axis_v":array(transform.rot.mul_vec(Vec3::new(0.,0.,1.))),
        "segments":[1,1],"width_m":transform.scale,"height_m":transform.scale,
        "thickness_m":0.004,"vertex_mass_kg":0.02,
        "pins":locals.iter().enumerate().map(|(i,point)|json!({"vertex":i,"target_object":"anchor","point":point})).collect::<Vec<_>>(),
        "collision_object_ids":if collision {vec!["floor"]} else {vec![]},
        "settings":{"fixed_dt":DT,"substeps":2,"iterations":2,"gravity":[0,0,0],"self_collision":false}
    }))
    .unwrap();
    cloth::create_panel(document, &panel).unwrap();
}

fn baked() -> Document {
    let mut document = source();
    add_attached_panel(&mut document, true);
    cloth::bake(&mut document, &request()).unwrap();
    document
}

fn assert_fresh(document: &Document, context: &str) {
    let info = cloth::inspect(document, "patch", None).unwrap();
    assert_eq!(info["cache"]["fresh"], true, "unrelated change staled cache: {context}");
    document.compile_at(&Pass::Beauty, Some(&sample(DT))).unwrap();
}

fn assert_stale(document: &Document, context: &str) {
    let info = cloth::inspect(document, "patch", None).unwrap();
    assert_eq!(info["cache"]["fresh"], false, "physical dependency did not stale cache: {context}");
    let error = document.compile_at(&Pass::Beauty, Some(&sample(DT))).err().unwrap();
    assert!(error.contains("stale"), "wrong rejection for {context}: {error}");
}

fn assert_cached_pins_match_independent_source(document: &Document, original_source: &Document) {
    let index = original_source.objects.iter().position(|o| o.id == "anchor").unwrap();
    let asset = &document.cloths[0];
    for frame in &asset.cache.as_ref().unwrap().frames {
        let (scene, _) = original_source.compile_at(&Pass::Beauty, Some(&sample(frame.time))).unwrap();
        for pin in &asset.pins {
            let expected = scene.objects[index].xform.to_world(vec(pin.point));
            let actual = vec(frame.vertices[pin.vertex as usize]);
            assert!(
                (actual - expected).length() < 5e-6,
                "stored frame at {} pin {}: {actual:?} != stateless pose {expected:?}",
                frame.time,
                pin.vertex
            );
        }
    }
}

#[test]
fn nonidentity_object_zero_key_is_not_applied_twice_in_stored_bake_frames() {
    let mut document = source();
    document.objects[1].position = [0.2, 0.3, -0.1];
    document.objects[1].rotation_degrees = [7., 11., -13.];
    for key in &mut document.clips[0].tracks[0].keys {
        key.translation = [0.4 + key.time, -0.2, 0.1];
        key.rotation_degrees = [10., 20., 30. + key.time * 20.];
    }
    let original_source = document.clone();
    add_attached_panel(&mut document, false);
    cloth::bake(&mut document, &request()).unwrap();
    assert_cached_pins_match_independent_source(&document, &original_source);
}

#[test]
fn nonidentity_ancestor_and_child_zero_keys_are_pruned_and_evaluated_once() {
    let mut document = source();
    document.joints = serde_json::from_value(json!([
        {"id":"root","pivot":[0.1,0.2,0.3]},
        {"id":"child","parent":"root","pivot":[0.2,0.4,0.1],"objects":["anchor"]},
        {"id":"sibling","parent":"root","pivot":[10,0,0],"objects":["scenery"]}
    ]))
    .unwrap();
    document.clips[0].tracks = serde_json::from_value(json!([
        {"target":{"type":"joint","id":"root"},"keys":[
            {"time":0,"translation":[0.1,0.2,0.3],"rotation_degrees":[0,0,15]},
            {"time":DT,"translation":[0.11,0.2,0.3],"rotation_degrees":[0,0,17]}]},
        {"target":{"type":"joint","id":"child"},"keys":[
            {"time":0,"rotation_degrees":[0,12,0]},
            {"time":DT,"rotation_degrees":[0,14,0]}]},
        {"target":{"type":"joint","id":"sibling"},"keys":[{"time":0},{"time":DT,"translation":[0,1,0]}]}
    ]))
    .unwrap();
    let original_source = document.clone();
    add_attached_panel(&mut document, false);
    let baked = cloth::bake(&mut document, &request()).unwrap();
    assert_eq!(baked["source_objects"], 1, "unrelated scenery must not be rebuilt by cloth simulation");
    assert_cached_pins_match_independent_source(&document, &original_source);

    let mut unrelated = document.clone();
    unrelated.joints[2].pivot[0] += 1.0;
    unrelated.clips[0].tracks[2].keys[1].translation[1] += 0.5;
    assert_fresh(&unrelated, "unrelated sibling joint and its track");
    let mut ancestor = document.clone();
    ancestor.joints[0].pivot[0] += 0.01;
    assert_stale(&ancestor, "transitive ancestor pivot");
    let mut track = document.clone();
    track.clips[0].tracks[0].keys[1].translation[0] += 0.01;
    assert_stale(&track, "transitive ancestor animation");
}

#[test]
fn unrelated_geometry_camera_lighting_materials_and_animation_do_not_stale_cache() {
    let original = baked();
    let original_frames = serde_json::to_value(&original.cloths[0].cache.as_ref().unwrap().frames).unwrap();
    let mut variants: Vec<(&str, Document)> = Vec::new();
    let mut geometry = original.clone();
    geometry.objects[2].position = [20., 3., -5.];
    geometry.objects[2].shape = serde_json::from_value(json!({"type":"sphere","radius":2})).unwrap();
    variants.push(("unrelated scenery geometry", geometry));
    let mut material = original.clone();
    for object in &mut material.objects {
        object.material.albedo = [0.1, 0.2, 0.3];
        object.material.roughness = 0.7;
    }
    variants.push(("all surface appearance including cloth and colliders", material));
    let mut camera = original.clone();
    camera.camera.eye[0] += 1.0;
    camera.clips[0].camera_keys[1].eye[1] += 1.0;
    camera.clips[0].camera_easing = Interpolation::EaseIn;
    camera.settings.width = 24;
    camera.lights[0].color = [0.1, 0.2, 0.3];
    variants.push(("camera pose, keys, interpolation, lighting and render settings", camera));
    let mut animation = original.clone();
    animation.clips[0].tracks[1].keys[1].translation[1] += 2.0;
    animation.clips.push(serde_json::from_value(json!({"id":"other","duration":1,"tracks":[]})).unwrap());
    variants.push(("unrelated object track and unused clip", animation));
    for (context, document) in variants {
        assert_fresh(&document, context);
        assert_eq!(serde_json::to_value(&document.cloths[0].cache.as_ref().unwrap().frames).unwrap(), original_frames);
    }
}

#[test]
fn physical_inputs_and_chosen_attachment_motion_stale_the_bake() {
    let original = baked();
    let mut variants: Vec<(&str, Document)> = Vec::new();
    let mut collider = original.clone();
    collider.objects[0].position[1] += 0.01;
    variants.push(("selected collision geometry", collider));
    let mut pin_object = original.clone();
    pin_object.objects[1].position[0] += 0.01;
    variants.push(("pin object rest transform", pin_object));
    let mut motion = original.clone();
    motion.clips[0].tracks[0].keys[1].translation[0] += 0.01;
    variants.push(("chosen pin animation", motion));
    let mut pin = original.clone();
    pin.cloths[0].pins[0].point[0] += 0.01;
    variants.push(("attachment point", pin));
    let mut physical = original.clone();
    physical.cloths[0].settings.gravity[1] -= 0.1;
    variants.push(("physical solver input", physical));
    for (context, document) in variants {
        assert_stale(&document, context);
    }
}

#[test]
fn garment_collider_retains_body_opening_face_and_joint_dependencies() {
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
    garment::create(
        &mut document,
        &GarmentRequest {
            id: "costume".into(),
            character: "hero".into(),
            style: GarmentStyle::Vest,
            clearance_m: 0.01,
            thickness_m: 0.004,
            material: Surface::default(),
        },
    )
    .unwrap();
    document.joints = mm3e_editor::animation::rig_humanoid(&document, "hero").unwrap();
    document.clips = serde_json::from_value(json!([{"id":"motion","duration":DT,"tracks":[
        {"target":{"type":"joint","id":"hero/root"},"keys":[{"time":0},{"time":DT}]}
    ],"face_tracks":[{"face":"face","channel":"jaw_open","keys":[{"time":0,"value":0},{"time":DT,"value":0.2}]}]}]))
    .unwrap();
    // A remote fixed patch isolates dependency construction from cloth contact
    // convergence, while retaining the real generated garment as a collider.
    let panel: ClothPanelRequest = serde_json::from_value(json!({
        "id":"patch","origin":[3,3,3],"axis_u":[1,0,0],"axis_v":[0,0,1],"segments":[1,1],
        "width_m":0.2,"height_m":0.2,"thickness_m":0.004,"vertex_mass_kg":0.02,
        "collision_object_ids":["costume"],"settings":{"fixed_dt":DT,"substeps":1,"iterations":1,"gravity":[0,0,0],"self_collision":false}
    })).unwrap();
    cloth::create_panel(&mut document, &panel).unwrap();
    let result = cloth::bake(&mut document, &request()).unwrap();
    assert!(result["source_objects"].as_u64().unwrap() > 19, "facial and garment sources must survive pruning");
    assert_fresh(&document, "unchanged generated-geometry dependency closure");

    let mut body = document.clone();
    body.objects.iter_mut().find(|o| o.id == "hero/abdomen").unwrap().scale *= 1.01;
    garment::synchronize(&mut body).unwrap();
    assert_stale(&body, "transitive garment body geometry");
    let mut opening = document.clone();
    opening.objects.iter_mut().find(|o| o.id == "hero/neck").unwrap().position[1] += 0.001;
    garment::synchronize(&mut opening).unwrap();
    assert_stale(&opening, "garment opening source");
    let mut facial = document.clone();
    facial.clips[0].face_tracks[0].keys[1].value = 0.4;
    assert_stale(&facial, "head facial animation reached through garment sources");
    let mut ancestor = document.clone();
    ancestor.clips[0].tracks[0].keys[1].translation[0] = 0.01;
    assert_stale(&ancestor, "garment body joint ancestor animation");
}

#[test]
fn barycentric_pin_follows_a_deformed_surface_through_bake_and_reload() {
    let mut editor = Editor::new(&std::env::temp_dir()).unwrap();
    let setup = editor.handle(
        serde_json::from_value(json!({
            "id":"setup",
            "expected_revision":0,
            "command":{"op":"apply","operations":[
                {"op":"create","object":{"id":"body","shape":{"type":"surface","vertices":[[0,0,0],[1,0,0],[0,1,0]],"triangles":[[0,1,2]],"thickness_m":0.01}}},
                {"op":"bind_surface","request":{"deformer":{"id":"skin","object":"body","blendshapes":[{"id":"lift","deltas":[[0,0,0.1],[0,0,0.1],[0,0,0.1]]}]} }},
                {"op":"put_clip","clip":{"id":"motion","duration":0.041666668,"morph_tracks":[{"deformer":"skin","blendshape":"lift","keys":[{"time":0,"weight":0},{"time":0.041666668,"weight":1}]}]}}
            ]}
        }))
        .unwrap(),
    );
    assert!(setup.ok, "{setup:?}");
    let mut document = editor.document().clone();
    let panel: ClothPanelRequest = serde_json::from_value(json!({
        "id":"cloth",
        "origin":[0.8333333,0.8333333,0],
        "axis_u":[1,0,0],
        "axis_v":[0,1,0],
        "segments":[1,1],
        "width_m":1,
        "height_m":1,
        "thickness_m":0.004,
        "vertex_mass_kg":0.02,
        "pins":[{"vertex":0,"target_object":"body","point":[0,0,0],"target_triangle":0,"barycentric":[0.33333334,0.33333334,0.33333334]}],
        "settings":{"fixed_dt":0.041666668,"substeps":1,"iterations":1,"gravity":[0,0,0],"self_collision":false}
    }))
    .unwrap();
    cloth::create_panel(&mut document, &panel).unwrap();
    cloth::bake(&mut document, &BakeClothRequest { id: "cloth".into(), clip: "motion".into(), duration: None })
        .unwrap();
    let cache = document.cloths[0].cache.as_ref().unwrap();
    assert_eq!(cache.frames.len(), 2);
    assert!((cache.frames[0].vertices[0][0] - 1.0 / 3.0).abs() < 2e-6);
    assert!((cache.frames[0].vertices[0][1] - 1.0 / 3.0).abs() < 2e-6);
    assert!((cache.frames[1].vertices[0][2] - 0.1).abs() < 2e-5);
    let bytes = serde_json::to_vec(&document).unwrap();
    let loaded: Document = serde_json::from_slice(&bytes).unwrap();
    let loaded_cache = loaded.cloths[0].cache.as_ref().unwrap();
    assert_eq!(serde_json::to_value(&loaded_cache.frames).unwrap(), serde_json::to_value(&cache.frames).unwrap());
    assert!(cloth::cache_fresh(&loaded, &loaded.cloths[0]).unwrap());
    // Actual r9 release fingerprint for this exact source, captured Sep 19.
    // Its unnormalized attachment algorithm must not authorize current playback.
    let mut historical = loaded.clone();
    historical.cloths[0].cache.as_mut().unwrap().source_fnv1a64 = "6cab9f2a562fb2b1".into();
    assert!(!cloth::cache_fresh(&historical, &historical.cloths[0]).unwrap());
    assert!(cloth::inspect(&historical, "cloth", Some(&sample(DT))).unwrap_err().contains("stale"));

    let mut malformed = loaded.clone();
    malformed.cloths[0].pins[0].barycentric = Some([0.5, 0.5, 0.5]);
    assert!(cloth::validate(&malformed).is_err());
    let mut local_only = loaded;
    local_only.cloths[0].pins[0].target_triangle = None;
    local_only.cloths[0].pins[0].barycentric = None;
    assert!(cloth::validate(&local_only).is_err());
}

fn barycentric_source(translation: f32) -> Document {
    Document {
        objects: serde_json::from_value(json!([{"id":"body","position":[translation,0,0],
            "shape":{"type":"surface","vertices":[[0,0,0],[1,0,0],[0,1,0]],
                "triangles":[[0,1,2]],"thickness_m":0.01}}]))
        .unwrap(),
        deformers: serde_json::from_value(json!([{"id":"skin","object":"body",
            "blendshapes":[{"id":"lift","deltas":[[0,0,0.001],[0,0,0.004],[0,0,0.008]]}]}]))
        .unwrap(),
        clips: serde_json::from_value(json!([{"id":"motion","duration":DT,"morph_tracks":[{
            "deformer":"skin","blendshape":"lift","easing":"smooth_step",
            "keys":[{"time":0,"weight":0},{"time":DT,"weight":1}]}]}]))
        .unwrap(),
        ..Document::default()
    }
}

fn barycentric_panel(origin: [f32; 3], weights: [f32; 3]) -> ClothPanelRequest {
    serde_json::from_value(json!({"id":"patch","origin":origin,
        "axis_u":[1,0,0],"axis_v":[0,1,0],"segments":[1,1],"width_m":1,"height_m":1,
        "thickness_m":0.004,"vertex_mass_kg":0.02,
        "pins":[{"vertex":0,"target_object":"body","point":[0,0,0],"target_triangle":0,"barycentric":weights}],
        "settings":{"fixed_dt":DT,"substeps":2,"iterations":2,"gravity":[0,0,0],"self_collision":false}}))
    .unwrap()
}

#[test]
fn barycentric_attachment_preserves_translation_and_charges_each_evaluation() {
    // All stored weights are valid, but their exact sum is slightly above one.
    // A raw world-coordinate weighted sum incorrectly moves the translated pin
    // by half a millimetre and rejects its unchanged frame-zero attachment.
    for translation in [0.0, 1000.0, -1000.0] {
        let mut document = barycentric_source(translation);
        let panel = barycentric_panel([translation + 0.5, 0.5, 0.0], [1.0, 0.0000005, 0.0]);
        cloth::create_panel(&mut document, &panel).unwrap();
        let authored = serde_json::to_value(&document.objects).unwrap();
        let bake = cloth::bake(&mut document, &request()).unwrap();
        assert_eq!(bake["barycentric_attachment_evaluations"], 3);
        assert_eq!(bake["estimated_attachment_work"], 105);
        assert!(bake["charged_work_total"].as_u64().unwrap() >= 105);
        assert_eq!(serde_json::to_value(&document.objects).unwrap(), authored);
        let state = cloth::inspect(&document, "patch", Some(&sample(DT * 0.25))).unwrap();
        let pin: [f32; 3] = serde_json::from_value(state["vertices"][0].clone()).unwrap();
        assert!((f64::from(pin[0]) - f64::from(translation) - 0.0000005 / 1.0000005).abs() < 1e-6);
        let lift = (0.001 + 0.0000005 * 0.004) / 1.0000005 * 0.15625;
        assert!((f64::from(pin[2]) - lift).abs() < 1e-8);
        let loaded: Document = serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
        assert!(cloth::cache_fresh(&loaded, &loaded.cloths[0]).unwrap());
        assert_eq!(state, cloth::inspect(&loaded, "patch", Some(&sample(DT * 0.25))).unwrap());
    }
}

#[test]
fn barycentric_skin_and_morph_pins_follow_transformed_surface_at_subframes() {
    for method in ["linear_blend", "dual_quaternion"] {
        let mut document = barycentric_source(0.0);
        document.objects[0].position = [3.0, 2.0, 1.0];
        document.objects[0].rotation_degrees = [0.0, 0.0, 90.0];
        document.objects[0].scale = 2.0;
        document.joints = serde_json::from_value(json!([{"id":"bone","pivot":[3,2,1]}])).unwrap();
        document.deformers[0].method = serde_json::from_value(json!(method)).unwrap();
        document.deformers[0].joints = vec!["bone".into()];
        document.deformers[0].weights = serde_json::from_value(json!([
            [{"joint":0,"weight":1}],[{"joint":0,"weight":1}],[{"joint":0,"weight":1}]
        ]))
        .unwrap();
        document.clips[0].tracks = serde_json::from_value(json!([{
            "target":{"type":"joint","id":"bone"},"keys":[{"time":0},
                {"time":DT,"rotation_degrees":[0,0,10],"translation":[0,0,0.002]}]
        }]))
        .unwrap();
        cloth::create_panel(&mut document, &barycentric_panel([2.5, 3.0, 1.0], [0.25, 0.25, 0.5])).unwrap();
        cloth::bake(&mut document, &request()).unwrap();
        let state = cloth::inspect(&document, "patch", Some(&sample(DT * 0.25))).unwrap();
        let pin: [f32; 3] = serde_json::from_value(state["vertices"][0].clone()).unwrap();
        let angle = 2.5_f64.to_radians();
        let want = [
            3.0 - angle.cos() - 0.5 * angle.sin(),
            2.0 - angle.sin() + 0.5 * angle.cos(),
            1.0 + 2.0 * 0.00525 * 0.15625 + 0.0005,
        ];
        for axis in 0..3 {
            assert!((f64::from(pin[axis]) - want[axis]).abs() < 2e-6, "{method}: {pin:?} vs {want:?}");
        }
        let frames = &document.cloths[0].cache.as_ref().unwrap().frames;
        let chord = frames[0].vertices[0][2] * 0.75 + frames[1].vertices[0][2] * 0.25;
        assert!((pin[2] - chord).abs() > 0.0009, "pin incorrectly followed cached linear interpolation");
        let mut loaded: Document = serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
        assert!(cloth::cache_fresh(&loaded, &loaded.cloths[0]).unwrap());
        if let mm3e_editor::model::Shape::Surface { triangles, .. } = &mut loaded.objects[0].shape {
            triangles[0] = [2, 1, 0];
        }
        assert!(!cloth::cache_fresh(&loaded, &loaded.cloths[0]).unwrap(), "source topology must bind the cache");
        assert!(cloth::inspect(&loaded, "patch", Some(&sample(DT * 0.25))).unwrap_err().contains("stale"));
    }
}

#[test]
fn pattern_barycentric_pins_retain_normalized_attachment_and_reject_out_of_tolerance_sum() {
    use mm3e_editor::pattern::{self, PatternClothRequest};
    let request_for = |weights: [f32; 3]| -> PatternClothRequest {
        serde_json::from_value(json!({"id":"patch","panels":[{"id":"front",
            "origin":[1000,0,0],"axis_u":[1,0,0],"axis_v":[0,1,0],
            "outer":{"id":"edge","points":[{"id":"a","position":[0,0]},
                {"id":"b","position":[1,0]},{"id":"c","position":[1,1]},{"id":"d","position":[0,1]}]},
            "pins":[{"vertex":{"type":"control","id":"a"},"target_object":"body","point":[0,0,0],
                "target_triangle":0,"barycentric":weights}]}],"thickness_m":0.004,"vertex_mass_kg":0.02,
            "settings":{"fixed_dt":DT,"substeps":2,"iterations":2,"gravity":[0,0,0],"self_collision":false}}))
        .unwrap()
    };
    let mut document = barycentric_source(1000.0);
    let weights = [1.0, 0.0000005, 0.0];
    let request = request_for(weights);
    pattern::create(&mut document, &request).unwrap();
    assert_eq!(document.cloths[0].pins[0].barycentric, Some(weights));
    cloth::bake(&mut document, &BakeClothRequest { id: "patch".into(), clip: "motion".into(), duration: None })
        .unwrap();
    let loaded: Document = serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
    assert!(cloth::cache_fresh(&loaded, &loaded.cloths[0]).unwrap());
    let state = cloth::inspect(&loaded, "patch", Some(&sample(DT * 0.25))).unwrap();
    let vertex = loaded.cloths[0].pins[0].vertex as usize;
    assert_eq!(state["vertices"][vertex][0], 1000.0);
    // f32 addition rounds this sum down inside 1e-6 despite the stored
    // coefficients lying outside that tolerance. Preview and creation agree.
    let malformed = request_for([1.0, 0.00000101, 0.0]);
    assert!(pattern::preview(&malformed.panels[0]).unwrap_err().contains("summing to one"));
    let before = serde_json::to_value(&document).unwrap();
    assert!(pattern::update(&mut document, &malformed).is_err());
    assert_eq!(before, serde_json::to_value(&document).unwrap());
    let mut loose = barycentric_source(1000.0);
    let bad_panel = barycentric_panel([1000.5, 0.5, 0.0], [1.0, 0.00000101, 0.0]);
    assert!(cloth::create_panel(&mut loose, &bad_panel).unwrap_err().contains("summing to one"));
    assert!(loose.cloths.is_empty());
}
