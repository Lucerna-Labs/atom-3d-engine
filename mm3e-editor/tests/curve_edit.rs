use mm3e_editor::{
    animation::{AnimationSample, Playback},
    curve_edit::{self, CurveEdit, CurveOperation},
    deform::{self, BindSurface, MorphTrack},
    face::{self, FaceChannel, FaceRequest, FaceTrack},
    model::{vec, Document, Pass},
};
use serde_json::{json, Value};

fn document() -> Document {
    let mut document = Document {
        objects: serde_json::from_value(json!([
            {"id":"actor/head","shape":{"type":"ellipsoid","radii":[0.8,1.0,0.8]}},
            {"id":"actor/left_eye","shape":{"type":"sphere","radius":0.15},"position":[0.28,0.2,0.78]},
            {"id":"actor/right_eye","shape":{"type":"sphere","radius":0.15},"position":[-0.28,0.2,0.78]},
            {"id":"patch","position":[3,0,0],"shape":{"type":"surface",
                "vertices":[[0,0,0],[1,0,0],[0,1,0]],"triangles":[[0,1,2]],"thickness_m":0.02}},
            {"id":"marker","position":[5,0,0],"shape":{"type":"sphere","radius":0.05}}
        ]))
        .unwrap(),
        ..Document::default()
    };
    face::create(&mut document, &FaceRequest { id: "face".into(), character: "actor".into() }).unwrap();
    let binding: BindSurface = serde_json::from_value(json!({"deformer":{"id":"skin","object":"patch",
    "blendshapes":[
        {"id":"side","deltas":[[0.125,0,0],[0.125,0,0],[0.125,0,0]]},
        {"id":"mouth-open","deltas":[[0,0,0.5],[0,0,0.5],[0,0,0.5]]},
        {"id":"lift","deltas":[[0,0.25,0],[0,0.25,0],[0,0.25,0]]}
    ]}}))
    .unwrap();
    deform::bind(&mut document, binding).unwrap();
    document.clips = serde_json::from_value(json!([
        {"id":"dialogue","duration":1,
         "tracks":[{"target":{"type":"object","id":"marker"},"easing":"ease_out",
             "keys":[{"time":0,"translation":[0.01,0,0]},{"time":1,"translation":[0.2,0,0]}]}],
         "face_tracks":[
             {"face":"face","channel":"blink_left","keys":[{"time":0,"value":0.1},{"time":1,"value":0.7}]},
             {"face":"face","channel":"jaw_open","keys":[{"time":0,"value":0},{"time":1,"value":0}]},
             {"face":"face","channel":"blink_right","easing":"smooth_step","keys":[{"time":0,"value":0.2},{"time":1,"value":0.6}]}
         ],
         "morph_tracks":[
             {"deformer":"skin","blendshape":"side","keys":[{"time":0,"weight":0.1},{"time":1,"weight":0.5}]},
             {"deformer":"skin","blendshape":"mouth-open","keys":[{"time":0,"weight":0.2},{"time":1,"weight":0.2}]},
             {"deformer":"skin","blendshape":"lift","easing":"ease_in","keys":[{"time":0,"weight":0.2},{"time":1,"weight":0.7}]}
         ],
         "camera_easing":"smooth_step",
         "camera_keys":[{"time":0,"eye":[0,0,4],"target":[0,0,0],"fov_degrees":45},
                        {"time":1,"eye":[0.1,0,4],"target":[0,0,0],"fov_degrees":50}]},
        {"id":"unrelated","duration":2,"tracks":[{"target":{"type":"object","id":"marker"},
             "keys":[{"time":0},{"time":2,"translation":[0,0.4,0]}]}]}
    ]))
    .unwrap();
    document.compile(&Pass::Beauty).unwrap();
    document
}

fn at(time: f32) -> AnimationSample {
    AnimationSample { clip: "dialogue".into(), time, playback: Playback::Clamp }
}

fn request(edit: Value) -> CurveEdit {
    serde_json::from_value(json!({"clip":"dialogue","edit":edit})).unwrap()
}

fn points(document: &Document, time: f32) -> Vec<[f32; 3]> {
    serde_json::from_value(deform::inspect(document, "skin", Some(&at(time))).unwrap()["vertices"].clone()).unwrap()
}

fn assert_rejected(document: &Document, request: &CurveEdit) {
    let mut candidate = document.clone();
    let original = serde_json::to_vec(document).unwrap();
    assert!(curve_edit::apply(&mut candidate, request).is_err());
    assert_eq!(serde_json::to_vec(&candidate).unwrap(), original);
}

#[test]
fn manual_morph_timing_replaces_one_track_in_place_and_changes_the_native_field() {
    let mut document = document();
    let before_points = points(&document, 0.25);
    let (before_scene, _) = document.compile_at(&Pass::Beauty, Some(&at(0.25))).unwrap();
    let track = json!({"deformer":"skin","blendshape":"mouth-open","easing":"linear",
        "keys":[{"time":0,"weight":0},{"time":0.25,"weight":1},{"time":0.75,"weight":0},{"time":1,"weight":0}]});
    let request = request(json!({"op":"upsert_morph","track":track}));
    let request_before = serde_json::to_vec(&request).unwrap();
    let mut expected = serde_json::to_value(&document).unwrap();
    let CurveOperation::UpsertMorph { track } = &request.edit else { unreachable!() };
    expected["clips"][0]["morph_tracks"][1] = serde_json::to_value(track).unwrap();
    curve_edit::apply(&mut document, &request).unwrap();
    assert_eq!(serde_json::to_value(&document).unwrap(), expected, "only the selected curve may change");
    assert_eq!(serde_json::to_vec(&request).unwrap(), request_before, "the borrowed request remains unchanged");
    let after_points = points(&document, 0.25);
    for (before, after) in before_points.iter().zip(&after_points) {
        assert_eq!([before[0], before[1]], [after[0], after[1]]);
        assert!((after[2] - before[2] - 0.4).abs() < 1e-6);
    }
    let center = (vec(after_points[0]) + vec(after_points[1]) + vec(after_points[2])) * (1.0 / 3.0);
    let object = document.objects.iter().position(|o| o.id == "patch").unwrap();
    let (after_scene, _) = document.compile_at(&Pass::Beauty, Some(&at(0.25))).unwrap();
    assert!(before_scene.sample_object(object, center).unwrap().dist > 0.3);
    assert!(after_scene.sample_object(object, center).unwrap().dist < -0.009);
    assert!(points(&document, 0.75).iter().all(|point| point[2] == 0.0));
}

#[test]
fn facial_upsert_changes_real_mouth_geometry_and_new_channels_append_without_reordering() {
    let mut document = document();
    let probe = vec([0.0, -0.467, 0.66]);
    let head = document.objects.iter().position(|o| o.id == "actor/head").unwrap();
    let (before, _) = document.compile_at(&Pass::Beauty, Some(&at(0.5))).unwrap();
    assert!(before.sample_object(head, probe).unwrap().dist < 0.0);
    let edit = request(json!({"op":"upsert_face","track":{"face":"face","channel":"jaw_open","easing":"smooth_step",
        "keys":[{"time":0,"value":0},{"time":0.5,"value":1},{"time":1,"value":0}]}}));
    let mut expected = serde_json::to_value(&document).unwrap();
    let CurveOperation::UpsertFace { track } = &edit.edit else { unreachable!() };
    expected["clips"][0]["face_tracks"][1] = serde_json::to_value(track).unwrap();
    curve_edit::apply(&mut document, &edit).unwrap();
    assert_eq!(serde_json::to_value(&document).unwrap(), expected);
    let controls = face::inspect_controls(&document, Some(&at(0.5))).unwrap();
    assert_eq!(controls["faces"][0]["controls"]["jaw_open"], 1.0);
    let (after, _) = document.compile_at(&Pass::Beauty, Some(&at(0.5))).unwrap();
    assert!(after.sample_object(head, probe).unwrap().dist > 0.0);
    let append = request(json!({"op":"upsert_face","track":{"face":"face","channel":"smile",
        "keys":[{"time":0,"value":-0.3},{"time":1,"value":0.8}]}}));
    let CurveOperation::UpsertFace { track } = &append.edit else { unreachable!() };
    expected["clips"][0]["face_tracks"].as_array_mut().unwrap().push(serde_json::to_value(track).unwrap());
    curve_edit::apply(&mut document, &append).unwrap();
    assert_eq!(serde_json::to_value(&document).unwrap(), expected);
}

#[test]
fn removing_face_and_morph_curves_preserves_every_other_field_and_restores_defaults() {
    let mut document = document();
    let mut expected = serde_json::to_value(&document).unwrap();
    for (edit, array) in [
        (json!({"op":"remove_face","face":"face","channel":"jaw_open"}), "face_tracks"),
        (json!({"op":"remove_morph","deformer":"skin","blendshape":"mouth-open"}), "morph_tracks"),
    ] {
        let edit = request(edit);
        expected["clips"][0][array].as_array_mut().unwrap().remove(1);
        curve_edit::apply(&mut document, &edit).unwrap();
        assert_eq!(serde_json::to_value(&document).unwrap(), expected);
        assert_rejected(&document, &edit);
    }
    assert!(points(&document, 0.5).iter().all(|point| point[2] == 0.0));
    assert_eq!(face::inspect_controls(&document, Some(&at(0.5))).unwrap()["faces"][0]["controls"]["jaw_open"], 0.0);
    // Re-adding a removed morph appends it instead of reordering surviving tracks.
    let append = request(json!({"op":"upsert_morph","track":{"deformer":"skin","blendshape":"mouth-open",
        "keys":[{"time":0,"weight":0.5},{"time":1,"weight":0.5}]}}));
    let CurveOperation::UpsertMorph { track } = &append.edit else { unreachable!() };
    expected["clips"][0]["morph_tracks"].as_array_mut().unwrap().push(serde_json::to_value(track).unwrap());
    curve_edit::apply(&mut document, &append).unwrap();
    assert_eq!(serde_json::to_value(&document).unwrap(), expected);
    assert!(points(&document, 0.5).iter().all(|point| point[2] == 0.25));
}

#[test]
fn invalid_values_key_times_references_and_missing_removals_are_atomic() {
    let document = document();
    for keys in [
        json!([]),
        json!([{"time":0,"value":1.1}]),
        json!([{"time":-0.1,"value":0}]),
        json!([{"time":1.1,"value":0}]),
        json!([{"time":0.5,"value":0},{"time":0.5,"value":1}]),
        json!([{"time":0.5,"value":0},{"time":0.25,"value":1}]),
    ] {
        assert_rejected(
            &document,
            &request(json!({"op":"upsert_face","track":{"face":"face","channel":"jaw_open","keys":keys}})),
        );
    }
    for keys in [
        json!([]),
        json!([{"time":0,"weight":-0.1}]),
        json!([{"time":0,"weight":1.1}]),
        json!([{"time":1.1,"weight":0}]),
        json!([{"time":0.25,"weight":0},{"time":0.25,"weight":1}]),
        json!([{"time":0.75,"weight":0},{"time":0.25,"weight":1}]),
    ] {
        assert_rejected(
            &document,
            &request(json!({"op":"upsert_morph","track":{"deformer":"skin","blendshape":"mouth-open","keys":keys}})),
        );
    }
    for edit in [
        json!({"op":"remove_face","face":"face","channel":"smile"}),
        json!({"op":"remove_morph","deformer":"skin","blendshape":"missing"}),
        json!({"op":"upsert_face","track":{"face":"missing","channel":"jaw_open","keys":[{"time":0,"value":0}]}}),
        json!({"op":"upsert_morph","track":{"deformer":"skin","blendshape":"missing","keys":[{"time":0,"weight":0}]}}),
        json!({"op":"remove_face","face":"invalid id","channel":"jaw_open"}),
    ] {
        assert_rejected(&document, &request(edit));
    }
    let mut missing = request(json!({"op":"remove_face","face":"face","channel":"jaw_open"}));
    missing.clip = "missing".into();
    assert_rejected(&document, &missing);
    let mut nonfinite: FaceTrack =
        serde_json::from_value(json!({"face":"face","channel":"jaw_open","keys":[{"time":0,"value":0}]})).unwrap();
    nonfinite.keys[0].value = f32::NAN;
    assert_rejected(
        &document,
        &CurveEdit { clip: "dialogue".into(), edit: CurveOperation::UpsertFace { track: nonfinite } },
    );
    let mut nonfinite: MorphTrack =
        serde_json::from_value(json!({"deformer":"skin","blendshape":"mouth-open","keys":[{"time":0,"weight":0}]}))
            .unwrap();
    nonfinite.keys[0].time = f32::INFINITY;
    assert_rejected(
        &document,
        &CurveEdit { clip: "dialogue".into(), edit: CurveOperation::UpsertMorph { track: nonfinite } },
    );
}

#[test]
fn request_schema_rejects_unknown_fields_and_keeps_tagged_operations_explicit() {
    let valid = json!({"clip":"dialogue","edit":{"op":"remove_face","face":"face","channel":"jaw_open"}});
    let parsed: CurveEdit = serde_json::from_value(valid.clone()).unwrap();
    assert!(matches!(parsed.edit, CurveOperation::RemoveFace { channel: FaceChannel::JawOpen, .. }));
    for invalid in [
        json!({"clip":"dialogue","other":true,"edit":valid["edit"]}),
        json!({"clip":"dialogue","edit":{"op":"remove_face","face":"face","channel":"jaw_open","other":true}}),
        json!({"clip":"dialogue","edit":{"op":"guess_audio"}}),
    ] {
        assert!(serde_json::from_value::<CurveEdit>(invalid).is_err());
    }
}
