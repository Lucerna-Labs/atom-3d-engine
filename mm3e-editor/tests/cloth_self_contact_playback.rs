use mm3e_editor::{
    animation::{AnimationSample, Playback},
    cloth::{self, BakeClothRequest},
    model::{Document, Pass},
};
use serde_json::{json, Value};

fn layered_document(height: f32, self_collision: bool, tolerance: f32, pinned: bool) -> Document {
    let dt = 1.0 / 24.0_f32;
    let points = vec![
        [-4.0, 0.0, -4.0],
        [4.0, 0.0, -4.0],
        [0.0, 0.0, 4.0],
        [-0.2, height, -0.2],
        [0.2, height, -0.2],
        [0.0, height, 0.2],
    ];
    let triangles = vec![[0, 1, 2], [3, 4, 5]];
    let pins: Vec<Value> = if pinned {
        points
            .iter()
            .enumerate()
            .map(|(i, point)| json!({"vertex":i,"target_object":if i < 3 {None}else{Some("anchor")},"point":point}))
            .collect()
    } else {
        Vec::new()
    };
    Document {
        objects: serde_json::from_value(json!([
            {"id":"anchor","shape":{"type":"sphere","radius":0.1}},
            {"id":"layers","shape":{"type":"surface","vertices":points,"triangles":triangles,"thickness_m":0.004}}
        ]))
        .unwrap(),
        clips: serde_json::from_value(json!([{"id":"cross","duration":dt,"tracks":[{
            "target":{"type":"object","id":"anchor"},"keys":[{"time":0},{"time":dt,"translation":[0,-0.2,0]}]
        }]}]))
        .unwrap(),
        cloths: serde_json::from_value(json!([{
            "id":"layers","rest_vertices":points,"triangles":triangles,"thickness_m":0.004,
            "inverse_masses":vec![if pinned {0.0}else{1.0};6],"pins":pins,"collision_object_ids":[],
            "settings":{"fixed_dt":dt,"substeps":1,"iterations":1,"gravity":[0,0,0],
                "self_collision":self_collision,"self_collision_thickness":0.02,"max_penetration_m":tolerance}
        }]))
        .unwrap(),
        ..Document::default()
    }
}

fn bake(document: &mut Document) -> Result<Value, String> {
    cloth::bake(document, &BakeClothRequest { id: "layers".into(), clip: "cross".into(), duration: None })
}

fn sample(fraction: f32) -> AnimationSample {
    AnimationSample { clip: "cross".into(), time: fraction * (1.0 / 24.0), playback: Playback::Clamp }
}

#[test]
fn generated_cache_rechecks_fractional_pin_contact_and_preserves_the_document_on_rejection() {
    let mut document = layered_document(0.1, true, 0.002, true);
    let baked = bake(&mut document).unwrap();
    // Both actual baked frames are clear. A kinematic layer crosses between
    // them; the exact evaluated attachment pose must still be checked at display.
    let cache = document.cloths[0].cache.as_ref().unwrap();
    assert_eq!(cache.frames.len(), 2);
    assert_eq!(cache.diagnostics.max_self_contact_penetration, 0.0);
    assert!(cache.frames[0].diagnostics.self_contact_work > 0, "frame zero must perform a real clearance query");
    assert_eq!(baked["charged_self_contact_work"], cache.diagnostics.self_contact_work);
    assert_eq!(baked["estimated_work"], baked["estimated_work_base"]);
    assert_eq!(
        baked["charged_work_total"].as_u64().unwrap(),
        baked["estimated_work_base"].as_u64().unwrap() + cache.diagnostics.self_contact_work
    );
    assert!(baked["charged_work_total"].as_u64().unwrap() <= 100_000_000);
    for fraction in [0.0, 0.25, 0.75, 1.0] {
        let info = cloth::inspect(&document, "layers", Some(&sample(fraction))).unwrap();
        assert_eq!(info["sample_max_self_contact_penetration_m"], 0.0);
        assert_eq!(info["sample_within_bake_tolerances"], true);
        document.compile_at(&Pass::Beauty, Some(&sample(fraction))).unwrap();
    }
    let before = serde_json::to_vec(&document).unwrap();
    let info = cloth::inspect(&document, "layers", Some(&sample(0.5))).unwrap();
    assert!(info["sample_max_self_contact_penetration_m"].as_f64().unwrap() > 0.0199);
    assert!(info["sample_self_contact_candidates"].as_u64().unwrap() > 0);
    assert!(info["sample_self_contact_work"].as_u64().unwrap() > 0);
    assert_eq!(info["sample_within_bake_tolerances"], false);
    assert_eq!(info["sample_vertex_penetration_m"], 0.0);
    assert!(info["sample_max_relative_edge_error"].as_f64().unwrap() < 1e-6);
    let error = document.compile_at(&Pass::Beauty, Some(&sample(0.5))).err().unwrap();
    assert!(
        error.contains("sampled pose exceeds") && error.contains("self-contact feature clearance deficit"),
        "{error}"
    );
    assert_eq!(before, serde_json::to_vec(&document).unwrap());
}

#[test]
fn crafted_free_vertex_cache_interpolation_is_checked_even_when_both_stored_frames_are_clear() {
    let mut document = layered_document(0.1, true, 0.002, false);
    bake(&mut document).unwrap();
    // Deliberate cache fixture, not a claim that these endpoint motions were
    // produced by the solver. Keep native integrity checks valid so the test
    // reaches the actual free-vertex interpolation and static clearance gate.
    let cache = document.cloths[0].cache.as_mut().unwrap();
    for p in &mut cache.frames[1].vertices[3..] {
        p[1] = -0.1;
    }
    cache.frames_fnv1a64 = format!("{:016x}", mm3e_kit::atoms::hash(&serde_json::to_vec(&cache.frames).unwrap()));
    cloth::validate(&document).unwrap();
    assert!(document.cloths[0].pins.is_empty());
    for fraction in [0.0, 1.0] {
        let info = cloth::inspect(&document, "layers", Some(&sample(fraction))).unwrap();
        assert_eq!(info["sample_max_self_contact_penetration_m"], 0.0);
        document.compile_at(&Pass::Beauty, Some(&sample(fraction))).unwrap();
    }
    let info = cloth::inspect(&document, "layers", Some(&sample(0.5))).unwrap();
    assert!(info["sample_max_self_contact_penetration_m"].as_f64().unwrap() > 0.0199);
    assert_eq!(info["sample_within_bake_tolerances"], false);
    assert!(document
        .compile_at(&Pass::Beauty, Some(&sample(0.5)))
        .err()
        .unwrap()
        .contains("self-contact feature clearance deficit"));
}

#[test]
fn fractional_self_contact_gate_obeys_authored_tolerance_and_disabled_negative_control() {
    for (enabled, tolerance) in [(true, 0.03), (false, 0.002)] {
        let mut document = layered_document(0.1, enabled, tolerance, true);
        bake(&mut document).unwrap();
        let info = cloth::inspect(&document, "layers", Some(&sample(0.5))).unwrap();
        assert_eq!(info["sample_within_bake_tolerances"], true);
        if enabled {
            assert!(info["sample_max_self_contact_penetration_m"].as_f64().unwrap() > 0.0199);
            assert!(info["sample_self_contact_work"].as_u64().unwrap() > 0);
        } else {
            assert_eq!(info["sample_max_self_contact_penetration_m"], 0.0);
            assert_eq!(info["sample_self_contact_candidates"], 0);
            assert_eq!(info["sample_self_contact_work"], 0);
        }
        document.compile_at(&Pass::Beauty, Some(&sample(0.5))).unwrap();
    }
}

#[test]
fn fractional_cache_query_budget_failure_is_reported_and_rejects_rendering() {
    let mut document = layered_document(0.1, true, 0.002, false);
    document.cloths[0].settings.self_collision_max_candidates = 1;
    bake(&mut document).unwrap();
    // The same explicit fixture as the free-interpolation test: endpoints pass
    // their one-candidate budget, but the interpolated contact needs more work.
    let cache = document.cloths[0].cache.as_mut().unwrap();
    for p in &mut cache.frames[1].vertices[3..] {
        p[1] = -0.1;
    }
    cache.frames_fnv1a64 = format!("{:016x}", mm3e_kit::atoms::hash(&serde_json::to_vec(&cache.frames).unwrap()));
    for fraction in [0.0, 1.0] {
        document.compile_at(&Pass::Beauty, Some(&sample(fraction))).unwrap();
    }
    let before = serde_json::to_vec(&document).unwrap();
    let first = cloth::inspect(&document, "layers", Some(&sample(0.5))).unwrap_err();
    assert!(
        first.contains("static self-contact clearance query") && first.contains("candidate budget exhausted"),
        "{first}"
    );
    assert_eq!(first, cloth::inspect(&document, "layers", Some(&sample(0.5))).unwrap_err());
    assert!(document
        .compile_at(&Pass::Beauty, Some(&sample(0.5)))
        .err()
        .unwrap()
        .contains("candidate budget exhausted"));
    assert_eq!(before, serde_json::to_vec(&document).unwrap());
}

#[test]
fn initial_bake_measures_pinned_self_contact_and_rejects_penetration_or_query_budget_atomically() {
    let mut document = layered_document(0.005, true, 0.002, true);
    let before = serde_json::to_vec(&document).unwrap();
    let error = bake(&mut document).unwrap_err();
    assert!(error.contains("0.000000s") && error.contains("self-contact penetration"), "{error}");
    assert_eq!(before, serde_json::to_vec(&document).unwrap());

    let mut accepted = layered_document(0.005, true, 0.03, true);
    bake(&mut accepted).unwrap();
    let cache = accepted.cloths[0].cache.as_ref().unwrap();
    assert!((cache.frames[0].diagnostics.max_self_contact_penetration - 0.015).abs() < 1e-6);
    assert!(cache.frames[0].diagnostics.self_contact_candidates > 0);
    assert!(cache.diagnostics.max_self_contact_penetration >= 0.015 - 1e-6);

    let mut limited = layered_document(0.1, true, 0.002, true);
    limited.cloths[0].settings.self_collision_max_work = 1;
    let before = serde_json::to_vec(&limited).unwrap();
    let first = bake(&mut limited).unwrap_err();
    assert!(
        first.contains("static self-contact clearance query") && first.contains("work budget exhausted"),
        "{first}"
    );
    assert_eq!(first, bake(&mut limited).unwrap_err());
    assert_eq!(before, serde_json::to_vec(&limited).unwrap());
}
