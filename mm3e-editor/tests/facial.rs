use mm3e_editor::{
    animation::{AnimationSample, Playback},
    face::{self, FaceControls, FaceRequest},
    model::{vec, Document, Entity, Pass, Shape},
};
use mm3e_kit::{Ray, Vec3};
use serde_json::json;

fn entity(id: &str, shape: serde_json::Value, position: [f32; 3], albedo: [f32; 3]) -> Entity {
    serde_json::from_value(json!({"id":id,"shape":shape,"position":position,"material":{"albedo":albedo}})).unwrap()
}
fn document() -> Document {
    let mut doc = Document {
        objects: vec![
            entity("actor/head", json!({"type":"ellipsoid","radii":[0.8,1.0,0.8]}), [0.0; 3], [0.75, 0.52, 0.35]),
            entity("actor/left_eye", json!({"type":"sphere","radius":0.15}), [0.28, 0.20, 0.78], [0.02; 3]),
            entity("actor/right_eye", json!({"type":"sphere","radius":0.15}), [-0.28, 0.20, 0.78], [0.02; 3]),
        ],
        ..Document::default()
    };
    doc.settings.width = 64;
    doc.settings.height = 64;
    face::create(&mut doc, &FaceRequest { id: "face".into(), character: "actor".into() }).unwrap();
    doc
}
fn i(doc: &Document, id: &str) -> usize {
    doc.objects.iter().position(|e| e.id == id).unwrap()
}
fn set(doc: &mut Document, blink: f32, jaw: f32) {
    doc.faces[0].controls.blink_left = blink;
    doc.faces[0].controls.blink_right = blink;
    doc.faces[0].controls.jaw_open = jaw;
    face::synchronize(doc).unwrap();
}
fn eye_rays(doc: &Document, scene: &mm3e_orchestrator::Scene, angle: f32) -> usize {
    let eye = i(doc, "actor/left_eye");
    let target = scene.objects[eye].xform.pos;
    let dir = Vec3::new(angle.sin(), 0.0, -angle.cos());
    let mut visible = 0;
    for x in -3..=3 {
        for y in -2..=2 {
            let ray = Ray { origin: target + Vec3::new(x as f32 * 0.014, y as f32 * 0.015, 0.0) - dir * 2.0, dir };
            let hit = scene.marcher.march(&scene.field(), &ray);
            if hit.hit && hit.mat == eye as u32 + 1 {
                visible += 1;
            }
        }
    }
    visible
}

#[test]
fn physical_blink_occludes_unchanged_eyes_and_removing_lids_restores_visibility() {
    let mut doc = document();
    let neutral = serde_json::to_value(&doc.objects[1..3]).unwrap();
    let (open, _) = doc.compile(&Pass::Beauty).unwrap();
    assert!(eye_rays(&doc, &open, 0.0) > 20);
    assert!(eye_rays(&doc, &open, 0.25) > 10);
    set(&mut doc, 1.0, 0.0);
    assert_eq!(serde_json::to_value(&doc.objects[1..3]).unwrap(), neutral);
    let (mut closed, _) = doc.compile(&Pass::Beauty).unwrap();
    assert_eq!(eye_rays(&doc, &closed, 0.0), 0);
    assert_eq!(eye_rays(&doc, &closed, 0.25), 0);
    let eye = i(&doc, "actor/left_eye");
    assert!(matches!(closed.objects[eye].prim,mm3e_orchestrator::Prim::Sphere{r} if r==0.15));
    assert_eq!(closed.objects[eye].xform.pos, open.objects[eye].xform.pos);
    assert_eq!(closed.materials[eye + 1].albedo, open.materials[eye + 1].albedo);
    for id in ["face/left_upper_lid", "face/left_lower_lid"] {
        closed.objects[i(&doc, id)].xform.pos = Vec3::new(20.0, 20.0, 20.0);
    }
    assert!(eye_rays(&doc, &closed, 0.0) > 20, "a real lid-removal negative control must restore the same eyeball");
}

#[test]
fn mouth_has_a_true_cavity_width_controls_and_independent_solid_negative_control() {
    let mut doc = document();
    let head = i(&doc, "actor/head");
    let p = Vec3::new(0.0, -0.467, 0.66);
    let (closed, _) = doc.compile(&Pass::Beauty).unwrap();
    assert!(closed.sample_object(head, p).unwrap().dist < 0.0);
    set(&mut doc, 0.0, 1.0);
    let (open, _) = doc.compile(&Pass::Beauty).unwrap();
    assert!(open.sample_object(head, p).unwrap().dist > 0.0);
    assert!(open.sample_authored(p).dist > 0.0, "opening must contain air, not a dark surface decal");
    let ray = Ray { origin: Vec3::new(0.0, -0.467, 3.0), dir: Vec3::new(0.0, 0.0, -1.0) };
    let hit = open.marcher.march(&open.field(), &ray);
    assert!(hit.hit && hit.pos.z < 0.5, "mouth interior must be visibly recessed: {:?}", hit.pos);
    assert_eq!(hit.mat, i(&doc, "face/mouth_interior") as u32 + 1);
    let flank = Vec3::new(0.29, -0.467, 0.62);
    doc.faces[0].controls.lip_wide = -1.0;
    face::synchronize(&mut doc).unwrap();
    let (narrow, _) = doc.compile(&Pass::Beauty).unwrap();
    assert!(narrow.sample_object(head, flank).unwrap().dist < 0.0);
    doc.faces[0].controls.lip_wide = 1.0;
    face::synchronize(&mut doc).unwrap();
    let (wide, _) = doc.compile(&Pass::Beauty).unwrap();
    assert!(wide.sample_object(head, flank).unwrap().dist > 0.0);
    doc.objects.push(entity("witness", json!({"type":"sphere","radius":0.03}), [p.x, p.y, p.z], [1.0, 0.0, 0.0]));
    let (witness, _) = doc.compile(&Pass::Beauty).unwrap();
    assert!(witness.sample_object(head, p).unwrap().dist > 0.0);
    assert!((witness.sample_authored(p).dist + 0.03).abs() < 1e-5);
    assert_eq!(witness.sample_authored(p).mat, doc.objects.len() as u32);
}

#[test]
fn direct_head_object_motion_carries_eyes_lids_lips_and_cavity() {
    let mut doc = document();
    doc.clips.push(serde_json::from_value(json!({"id":"act","duration":1,"tracks":[{"target":{"type":"object","id":"actor/head"},"keys":[{"time":0},{"time":1,"rotation_degrees":[0,35,0],"translation":[0.3,0.2,0.4]}]}],
        "face_tracks":[{"face":"face","channel":"jaw_open","keys":[{"time":0,"value":0},{"time":1,"value":1}]},{"face":"face","channel":"blink_left","keys":[{"time":0,"value":0},{"time":1,"value":1}]}]})).unwrap());
    let sample = AnimationSample { clip: "act".into(), time: 1.0, playback: Playback::Clamp };
    let (scene, _) = doc.compile_at(&Pass::Beauty, Some(&sample)).unwrap();
    let head = scene.objects[i(&doc, "actor/head")].xform;
    let eye = scene.objects[i(&doc, "actor/left_eye")].xform;
    let expected = head.to_world(vec(doc.objects[i(&doc, "actor/left_eye")].position));
    assert!((eye.pos - expected).length() < 1e-6);
    assert_eq!(eye.pos, scene.objects[i(&doc, "face/left_upper_lid")].xform.pos);
    assert_eq!(head.pos, scene.objects[i(&doc, "face/upper_lip")].xform.pos);
    let cavity = head.to_world(Vec3::new(0.0, -0.467, 0.66));
    assert!(scene.sample_object(i(&doc, "actor/head"), cavity).unwrap().dist > 0.0);
    let eye_entity = &doc.objects[i(&doc, "actor/left_eye")];
    assert!(matches!(eye_entity.shape,Shape::Sphere{radius} if radius==0.15));
    let serialized = serde_json::to_vec(&doc).unwrap();
    let loaded: Document = serde_json::from_slice(&serialized).unwrap();
    let (reloaded, _) = loaded.compile_at(&Pass::Beauty, Some(&sample)).unwrap();
    assert_eq!(
        scene.objects[i(&doc, "face/upper_lip")].xform.rot.cols,
        reloaded.objects[i(&doc, "face/upper_lip")].xform.rot.cols
    );
    assert_eq!(scene.sample_authored(cavity).dist, reloaded.sample_authored(cavity).dist);
    let middle = AnimationSample { time: 0.5, ..sample.clone() };
    let _ = doc.compile_at(&Pass::Beauty, Some(&middle)).unwrap();
    let (again, _) = doc.compile_at(&Pass::Beauty, Some(&sample)).unwrap();
    assert_eq!(scene.sample_authored(cavity).dist, again.sample_authored(cavity).dist);
}

#[test]
fn current_authored_head_is_used_after_source_edits_and_bad_tracks_fail() {
    let mut doc = document();
    doc.objects[0].shape = Shape::Ellipsoid { radii: [0.9, 1.1, 0.7] };
    face::synchronize(&mut doc).unwrap();
    face::validate(&doc).unwrap();
    let (scene, _) = doc.compile(&Pass::Beauty).unwrap();
    let test = Vec3::new(0.85, 0.0, 0.0);
    assert!(scene.sample_object(0, test).unwrap().dist < 0.0, "head must use newly authored radii");
    let before = serde_json::to_value(&doc).unwrap();
    assert!(face::create(&mut doc, &FaceRequest { id: "face2".into(), character: "missing".into() }).is_err());
    assert_eq!(before, serde_json::to_value(&doc).unwrap());
    let mut bad = doc.clone();
    bad.faces[0].controls.blink_left = 1.1;
    assert!(face::validate(&bad).is_err());
    let clip = serde_json::from_value(json!({"id":"bad","duration":1,"face_tracks":[
        {"face":"face","channel":"jaw_open","keys":[{"time":0,"value":0}]},
        {"face":"face","channel":"jaw_open","keys":[{"time":0,"value":1}]}]}))
    .unwrap();
    assert!(face::validate_tracks(&doc, &clip).is_err());
}

#[test]
fn closed_mouth_has_no_phantom_interior_hits_outside_the_head() {
    let mut doc = document();
    // Real humanoid dimensions reproduce the near-zero field problem of an extremely flat
    // approximate ellipsoid at the renderer's actual epsilon. Do not scale this test up.
    doc.objects[0].shape = Shape::Ellipsoid { radii: [0.13266, 0.1881, 0.14256] };
    for (idx, sign) in [(1, 1.0), (2, -1.0)] {
        doc.objects[idx].shape = Shape::Sphere { radius: 0.0162 };
        doc.objects[idx].position = [sign * 0.05148, 0.02772, 0.13356];
    }
    face::synchronize(&mut doc).unwrap();
    doc.settings.width = 128;
    doc.settings.height = 160;
    doc.camera.eye = [0.0, -0.009, 0.841];
    doc.camera.target = [0.0, -0.009, -0.009];
    doc.camera.fov_degrees = 31.0;
    let (scene, camera) = doc.compile(&Pass::Beauty).unwrap();
    let interior = i(&doc, "face/mouth_interior") as u32 + 1;
    let field = scene.field();
    for y in 0..scene.height {
        for x in 0..scene.width {
            let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, scene.width, scene.height);
            let hit = scene.marcher.march(&field, &ray);
            assert!(
                !hit.hit || hit.mat != interior,
                "a closed mouth exposes phantom interior at ({x},{y}) / {:?}",
                hit.pos
            );
        }
    }
}

#[test]
fn gaze_brow_and_lip_seal_controls_are_independent_and_persisted() {
    let mut doc = document();
    let left_eye = i(&doc, "actor/left_eye");
    let right_eye = i(&doc, "actor/right_eye");
    let (neutral, _) = doc.compile(&Pass::Beauty).unwrap();
    let neutral_left = neutral.objects[left_eye].xform.pos;
    let neutral_right = neutral.objects[right_eye].xform.pos;
    let generated_before = serde_json::to_value(&doc.objects[3..]).unwrap();

    doc.faces[0].controls.gaze_x = 1.0;
    doc.faces[0].controls.gaze_y = -0.5;
    doc.faces[0].controls.brow_left = 0.75;
    doc.faces[0].controls.brow_right = -0.5;
    face::synchronize(&mut doc).unwrap();
    let (aimed, _) = doc.compile(&Pass::Beauty).unwrap();
    let left_delta = aimed.objects[left_eye].xform.pos - neutral_left;
    let right_delta = aimed.objects[right_eye].xform.pos - neutral_right;
    assert!((left_delta.x - 0.024).abs() < 1e-6 && (left_delta.y + 0.009).abs() < 1e-6);
    assert_eq!(left_delta, right_delta);
    assert_eq!(aimed.objects[left_eye].xform.pos, aimed.objects[i(&doc, "face/left_upper_lid")].xform.pos);
    assert_ne!(serde_json::to_value(&doc.objects[3..]).unwrap(), generated_before);

    doc.faces[0].controls.jaw_open = 1.0;
    doc.faces[0].controls.lip_seal = 0.0;
    let open = face::inspect_controls(&doc, None).unwrap();
    doc.faces[0].controls.lip_seal = 1.0;
    let sealed = face::inspect_controls(&doc, None).unwrap();
    assert!(
        sealed["faces"][0]["mouth"]["curve_half_height"].as_f64().unwrap()
            < open["faces"][0]["mouth"]["curve_half_height"].as_f64().unwrap()
    );
    face::synchronize(&mut doc).unwrap();

    let clip = serde_json::from_value(json!({
        "id":"facial-controls",
        "duration":1,
        "face_tracks":[
            {"face":"face","channel":"gaze_x","keys":[{"time":0,"value":0},{"time":1,"value":1}]},
            {"face":"face","channel":"brow_left","keys":[{"time":0,"value":0},{"time":1,"value":0.5}]},
            {"face":"face","channel":"lip_seal","keys":[{"time":0,"value":0},{"time":1,"value":1}]}
        ]
    }))
    .unwrap();
    face::validate_tracks(&doc, &clip).unwrap();
    doc.clips.push(clip);
    let sample = AnimationSample { clip: "facial-controls".into(), time: 1.0, playback: Playback::Clamp };
    let (animated, _) = doc.compile_at(&Pass::Beauty, Some(&sample)).unwrap();
    assert!((animated.objects[left_eye].xform.pos.x - (neutral_left.x + 0.024)).abs() < 1e-6);
    let bytes = serde_json::to_vec(&doc).unwrap();
    let loaded: Document = serde_json::from_slice(&bytes).unwrap();
    let (reloaded, _) = loaded.compile_at(&Pass::Beauty, Some(&sample)).unwrap();
    assert_eq!(animated.objects[left_eye].xform.pos, reloaded.objects[left_eye].xform.pos);
}

#[test]
fn full_lip_seal_closes_the_cavity_and_meets_both_lips_at_maximum_jaw() {
    for smile in [-1.0, 0.0, 1.0] {
        for lip_wide in [-1.0, 1.0] {
            for lip_round in [0.0, 1.0] {
                let mut doc = document();
                doc.faces[0].controls = FaceControls {
                    jaw_open: 1.0,
                    lip_seal: 1.0,
                    smile,
                    lip_wide,
                    lip_round,
                    ..FaceControls::default()
                };
                face::synchronize(&mut doc).unwrap();
                let state = face::inspect_controls(&doc, None).unwrap();
                assert_eq!(state["faces"][0]["mouth"]["cavity_open"], false);
                assert_eq!(state["faces"][0]["mouth"]["curve_half_height"], 0.0);
                assert_eq!(state["faces"][0]["controls"]["jaw_open"], 1.0);
                let (sealed, _) = doc.compile(&Pass::Beauty).unwrap();
                let center_y = -0.35_f32;
                let front = 0.8 * (1.0 - center_y * center_y).sqrt();
                let cavity_center = Vec3::new(0.0, center_y, front - 0.8 * 0.42 * 0.25);
                assert!(sealed.sample_object(0, cavity_center).unwrap().dist < 0.0);
                let contact = Vec3::new(0.0, center_y, front + 0.022 * 0.35 + lip_round * 0.8 * 0.11);
                for id in ["face/upper_lip", "face/lower_lip"] {
                    assert!(sealed.sample_object(i(&doc, id), contact).unwrap().dist < 0.0, "{id} leaves a gap");
                }
                // Full jaw motion must reopen the actual cavity when the seal is released.
                doc.faces[0].controls.lip_seal = 0.0;
                face::synchronize(&mut doc).unwrap();
                let (open, _) = doc.compile(&Pass::Beauty).unwrap();
                assert!(open.sample_object(0, Vec3::new(0.0, -0.467, 0.66)).unwrap().dist > 0.0);
            }
        }
    }
}

#[test]
fn full_blinks_remain_closed_with_either_brow_extreme() {
    let mut doc = document();
    for brow in [-1.0, -0.5, 0.0, 0.5, 1.0] {
        doc.faces[0].controls = FaceControls {
            blink_left: 1.0,
            blink_right: 1.0,
            brow_left: brow,
            brow_right: -brow,
            ..FaceControls::default()
        };
        face::synchronize(&mut doc).unwrap();
        let (closed, _) = doc.compile(&Pass::Beauty).unwrap();
        for angle in [0.0, 0.25] {
            assert_eq!(eye_rays(&doc, &closed, angle), 0, "brow {brow} reopens a full blink");
        }
        for side in ["left", "right"] {
            let eye = i(&doc, &format!("actor/{side}_eye"));
            let point = closed.objects[eye].xform.pos + Vec3::new(0.0, -0.006, 0.159);
            let upper = closed.sample_object(i(&doc, &format!("face/{side}_upper_lid")), point).unwrap().dist;
            let lower = closed.sample_object(i(&doc, &format!("face/{side}_lower_lid")), point).unwrap().dist;
            assert!(upper.min(lower) < 0.0, "{side} brow leaves the eyelid shell open");
        }
    }
    doc.faces[0].controls.blink_left = 0.0;
    face::synchronize(&mut doc).unwrap();
    let (open, _) = doc.compile(&Pass::Beauty).unwrap();
    assert!(eye_rays(&doc, &open, 0.0) > 0);
}

#[test]
fn neutral_gaze_preserves_translated_rest_positions_bit_for_bit() {
    let mut doc = document();
    for object in &mut doc.objects[..3] {
        object.position[0] += 2.0;
        object.position[1] += 10.0;
    }
    face::synchronize(&mut doc).unwrap();
    doc.clips.push(serde_json::from_value(json!({"id":"idle","duration":1})).unwrap());
    let neutral = AnimationSample { clip: "idle".into(), time: 0.5, playback: Playback::Clamp };
    for signed_zero in [0.0, -0.0] {
        doc.faces[0].controls.gaze_x = signed_zero;
        doc.faces[0].controls.gaze_y = signed_zero;
        let (scene, _) = doc.compile_at(&Pass::Beauty, Some(&neutral)).unwrap();
        for id in ["actor/left_eye", "actor/right_eye"] {
            let index = i(&doc, id);
            let actual = scene.objects[index].xform.pos;
            assert_eq!(
                [actual.x.to_bits(), actual.y.to_bits(), actual.z.to_bits()],
                doc.objects[index].position.map(f32::to_bits)
            );
        }
    }
}

#[test]
fn nonzero_gaze_is_a_head_local_vector_at_translated_rotated_scaled_rest() {
    let mut doc = document();
    doc.objects[0].position = [9999.0, 10.0, 0.0];
    doc.objects[0].rotation_degrees = [0.0, 0.0, 90.0];
    doc.objects[0].scale = 2.0;
    for object in &mut doc.objects[1..3] {
        object.position[0] += 9999.0;
        object.position[1] += 10.0;
    }
    face::synchronize(&mut doc).unwrap();
    let (rest, _) = doc.compile(&Pass::Beauty).unwrap();
    doc.faces[0].controls.gaze_x = 1.0;
    doc.faces[0].controls.gaze_y = -0.5;
    let (aimed, _) = doc.compile(&Pass::Beauty).unwrap();
    for id in ["actor/left_eye", "actor/right_eye"] {
        let index = i(&doc, id);
        // A 90-degree Z rotation maps local (.024, -.009, 0), scaled by two,
        // to world (.018, .048, 0). Translation must not enter vector mapping.
        let expected = rest.objects[index].xform.pos + Vec3::new(0.018, 0.048, 0.0);
        let actual = aimed.objects[index].xform.pos;
        assert_eq!(actual.x.to_bits(), expected.x.to_bits());
        assert!((actual.y - expected.y).abs() < 1e-6);
        assert_eq!(actual.z.to_bits(), expected.z.to_bits());
    }
}

#[test]
fn new_control_defaults_preserve_legacy_serialization_and_each_invalid_value_is_rejected() {
    let legacy = json!({"blink_left":0.0,"blink_right":0.0,"jaw_open":0.0,"lip_round":0.0,"lip_wide":0.0,"smile":0.0});
    let defaults: FaceControls = serde_json::from_value(legacy.clone()).unwrap();
    assert_eq!(serde_json::to_value(&defaults).unwrap(), legacy);
    assert_eq!(
        mm3e_kit::atoms::hash(&serde_json::to_vec(&serde_json::to_value(&defaults).unwrap()).unwrap()),
        mm3e_kit::atoms::hash(&serde_json::to_vec(&legacy).unwrap())
    );
    for channel in ["gaze_x", "gaze_y", "brow_left", "brow_right", "lip_seal"] {
        let mut explicit_zero = legacy.clone();
        explicit_zero[channel] = json!(-0.0);
        let zero: FaceControls = serde_json::from_value(explicit_zero).unwrap();
        assert_eq!(serde_json::to_value(&zero).unwrap(), legacy, "{channel} changes a legacy zero fingerprint");
        let active: FaceControls = serde_json::from_value(json!({channel:0.5})).unwrap();
        assert_eq!(serde_json::to_value(&active).unwrap()[channel], 0.5);
        let loaded: FaceControls = serde_json::from_slice(&serde_json::to_vec(&active).unwrap()).unwrap();
        assert_eq!(serde_json::to_value(active).unwrap(), serde_json::to_value(loaded).unwrap());
        for invalid in [-1.1, 1.1] {
            let controls: FaceControls = serde_json::from_value(json!({channel:invalid})).unwrap();
            assert!(controls.validate().is_err(), "invalid {channel} was accepted");
            let doc = document();
            let clip = serde_json::from_value(json!({"id":"bad","duration":1,"face_tracks":[
                {"face":"face","channel":channel,"keys":[{"time":0,"value":invalid}]}]
            }))
            .unwrap();
            assert!(face::validate_tracks(&doc, &clip).is_err(), "invalid {channel} track was accepted");
        }
    }
    let negative_seal: FaceControls = serde_json::from_value(json!({"lip_seal":-0.1})).unwrap();
    assert!(negative_seal.validate().is_err());
}

#[test]
fn animated_seal_and_blink_close_with_brows_and_reload_without_accumulation() {
    let mut doc = document();
    doc.faces[0].controls.jaw_open = 1.0;
    doc.faces[0].controls.brow_left = 1.0;
    doc.faces[0].controls.brow_right = 0.75;
    face::synchronize(&mut doc).unwrap();
    doc.clips.push(
        serde_json::from_value(json!({
            "id":"close","duration":1,"face_tracks":[
                {"face":"face","channel":"lip_seal","keys":[{"time":0,"value":0},{"time":1,"value":1}]},
                {"face":"face","channel":"blink_left","keys":[{"time":0,"value":0},{"time":1,"value":1}]},
                {"face":"face","channel":"blink_right","keys":[{"time":0,"value":0},{"time":1,"value":1}]}
            ]
        }))
        .unwrap(),
    );
    let before = serde_json::to_vec(&doc).unwrap();
    let mut closed_probe = None;
    for time in [1.0, 0.0, 0.5, 1.0] {
        let sample = AnimationSample { clip: "close".into(), time, playback: Playback::Clamp };
        let (scene, _) = doc.compile_at(&Pass::Beauty, Some(&sample)).unwrap();
        let state = face::inspect_controls(&doc, Some(&sample)).unwrap();
        if time == 1.0 {
            assert_eq!(state["faces"][0]["mouth"]["cavity_open"], false);
            assert_eq!(eye_rays(&doc, &scene, 0.0), 0);
            let value = scene.sample_object(0, Vec3::new(0.0, -0.35, 0.66)).unwrap().dist;
            assert!(value < 0.0);
            if let Some(previous) = closed_probe {
                assert_eq!(value, previous);
            }
            closed_probe = Some(value);
        } else if time == 0.0 {
            assert_eq!(state["faces"][0]["mouth"]["cavity_open"], true);
            assert!(eye_rays(&doc, &scene, 0.0) > 0);
        }
    }
    assert_eq!(serde_json::to_vec(&doc).unwrap(), before);
    let reloaded: Document = serde_json::from_slice(&before).unwrap();
    let sample = AnimationSample { clip: "close".into(), time: 1.0, playback: Playback::Clamp };
    let (scene, _) = reloaded.compile_at(&Pass::Beauty, Some(&sample)).unwrap();
    assert_eq!(eye_rays(&reloaded, &scene, 0.0), 0);
    assert_eq!(Some(scene.sample_object(0, Vec3::new(0.0, -0.35, 0.66)).unwrap().dist), closed_probe);
}
