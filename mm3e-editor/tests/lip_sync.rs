//! Synthetic analysis evidence tests curve authoring only, not recognition accuracy.
use mm3e_editor::{
    animation::{self, AnimationSample, Playback},
    audio::{AudioAsset, EmbeddedWave},
    deform::{self, BindSurface},
    face::{self, FaceRequest},
    lip_sync::{self, GenerateLipSync},
    model::{Document, Pass},
    speech::SpeechReport,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mm3e-lip-sync-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn report(&self, report: &SpeechReport) {
        fs::write(self.0.join("analysis.json"), serde_json::to_vec(report).unwrap()).unwrap();
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn silence(frames: u32) -> EmbeddedWave {
    let mut bytes = b"RIFF".to_vec();
    bytes.extend((36 + frames * 2).to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16u32.to_le_bytes());
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(1u16.to_le_bytes());
    bytes.extend(8000u32.to_le_bytes());
    bytes.extend(16000u32.to_le_bytes());
    bytes.extend(2u16.to_le_bytes());
    bytes.extend(16u16.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend((frames * 2).to_le_bytes());
    bytes.resize(44 + frames as usize * 2, 0);
    EmbeddedWave::from_bytes(bytes).unwrap()
}

fn document(frames: u32) -> Document {
    let mut document = Document {
        audio: vec![AudioAsset { id:"speech".into(), label:"Synthetic curve fixture".into(), data:silence(frames+17) }],
        objects: serde_json::from_value(json!([
            {"id":"actor/head","shape":{"type":"ellipsoid","radii":[0.8,1,0.8]}},
            {"id":"actor/left_eye","position":[0.28,0.2,0.78],"shape":{"type":"sphere","radius":0.15}},
            {"id":"actor/right_eye","position":[-0.28,0.2,0.78],"shape":{"type":"sphere","radius":0.15}},
            {"id":"patch","position":[3,0,0],"shape":{"type":"surface","vertices":[[0,0,0],[1,0,0],[0,1,0]],"triangles":[[0,1,2]],"thickness_m":0.02}},
            {"id":"marker","position":[5,0,0],"shape":{"type":"sphere","radius":0.1}}
        ])).unwrap(),
        joints:serde_json::from_value(json!([{"id":"jaw","pivot":[4,0,0],"objects":["marker"]}])).unwrap(),
        ..Document::default()
    };
    face::create(&mut document, &FaceRequest { id: "face".into(), character: "actor".into() }).unwrap();
    document.faces[0].controls.jaw_open = 0.1;
    face::synchronize(&mut document).unwrap();
    let deformer = serde_json::from_value(json!({"id":"skin","object":"patch","blendshapes":[
        {"id":"open","deltas":vec![[0.0,0.0,0.1];3],"weight":0.2,"max_weight":1}
    ]}))
    .unwrap();
    deform::bind(&mut document, BindSurface { deformer }).unwrap();
    document.clips=serde_json::from_value(json!([{"id":"performance","duration":2,
        "tracks":[{"target":{"type":"object","id":"actor/head"},"keys":[{"time":0},{"time":2,"translation":[0.2,0,0]}]}],
        "face_tracks":[{"face":"face","channel":"blink_left","keys":[{"time":0,"value":0},{"time":1,"value":1}]}]
    }])).unwrap();
    document
}

fn report(document: &Document, frames: u32, cues: Value) -> SpeechReport {
    let duration_cs = frames * 100 / 8000;
    let raw_cues: Vec<_> = cues
        .as_array()
        .unwrap()
        .iter()
        .map(|cue| {
            json!({
        "start":cue["start_cs"].as_u64().unwrap() as f64/100.0,
        "end":cue["end_cs"].as_u64().unwrap() as f64/100.0,"value":cue["shape"]})
        })
        .collect();
    let raw =
        json!({"metadata":{"soundFile":"synthetic.wav","duration":f64::from(duration_cs)/100.0},"mouthCues":raw_cues});
    serde_json::from_value(json!({"format":"mm3e-speech-analysis-v1",
        "source":{"audio":"speech","source_sha256":document.audio[0].data.sha256(),"sample_rate":8000,"channel":0,"range":{"start_sample":17,"frame_count":frames}},
        "backend":{"name":"rhubarb","version":"1.14.0","binary_sha256":"1".repeat(64),"resources_sha256":"2".repeat(64)},
        "recognizer":"english","dialogue_hint":null,"extended_shapes":"X","input_wav_sha256":silence(frames).sha256(),
        "duration_cs":duration_cs,"cues":cues,"clipped_samples":0,
        "raw_output_sha256":format!("{:x}",Sha256::digest(serde_json::to_vec(&raw).unwrap())),"raw_output":raw
    })).unwrap()
}

fn cues() -> Value {
    json!([
        {"start_cs":0,"end_cs":20,"shape":"X"},
        {"start_cs":20,"end_cs":30,"shape":"D"},
        {"start_cs":30,"end_cs":31,"shape":"A"},
        {"start_cs":31,"end_cs":60,"shape":"D"},
        {"start_cs":60,"end_cs":101,"shape":"X"}
    ])
}

fn request() -> GenerateLipSync {
    serde_json::from_value(json!({"analysis_path":"analysis.json","clip":"speech-take","profile":{"poses":[
        {"shape":"X","values":[]},
        {"shape":"D","values":[{"type":"face","face":"face","channel":"jaw_open","value":1},
            {"type":"morph","deformer":"skin","blendshape":"open","value":1},
            {"type":"joint","joint":"jaw","rotation_degrees":[0,0,30]}]},
        {"shape":"A","values":[{"type":"face","face":"face","channel":"jaw_open","value":0},
            {"type":"morph","deformer":"skin","blendshape":"open","value":0}]}
    ]}}))
    .unwrap()
}

fn sample(time: f32) -> AnimationSample {
    AnimationSample { clip: "speech-take".into(), time, playback: Playback::Clamp }
}
fn jaw(document: &Document, time: f32) -> f64 {
    face::inspect_controls(document, Some(&sample(time))).unwrap()["faces"][0]["controls"]["jaw_open"].as_f64().unwrap()
}

#[test]
fn generated_curves_close_at_cues_preserve_other_performance_and_move_real_geometry() {
    let scratch = Scratch::new();
    let mut document = document(8107);
    let original = serde_json::to_value(&document.clips[0]).unwrap();
    let report = report(&document, 8107, cues());
    scratch.report(&report);
    let result = lip_sync::generate(&mut document, &scratch.0, &request()).unwrap();
    assert_eq!(result["edited"], false);
    assert_eq!(result["source_audio"]["status"], "available");
    assert_eq!(serde_json::to_value(&document.clips[0]).unwrap(), original);
    let clip = &document.clips[1];
    assert_eq!(clip.face_tracks.len(), 1);
    assert_eq!(clip.morph_tracks.len(), 1);
    assert_eq!(clip.tracks.len(), 1);
    assert!(clip.layers.is_empty());
    assert!((f64::from(clip.duration) - 8107.0 / 8000.0).abs() < 1e-6);
    assert!(f64::from(clip.duration) >= 8107.0 / 8000.0);
    assert!((jaw(&document, 0.0) - 0.1).abs() < 1e-6);
    assert!((jaw(&document, 0.2) - 1.0).abs() < 1e-6);
    assert!(jaw(&document, 0.295) > 0.999);
    assert!((jaw(&document, 0.2975) - 0.5).abs() < 1e-4);
    assert!(jaw(&document, 0.3).abs() < 1e-6);
    assert!(jaw(&document, 0.305).abs() < 1e-6);
    assert!((jaw(&document, 0.31) - 1.0).abs() < 1e-6);
    assert!((jaw(&document, clip.duration) - 0.1).abs() < 1e-6);
    let opened = document.compile_at(&Pass::Beauty, Some(&sample(0.2))).unwrap().0;
    let closed = document.compile_at(&Pass::Beauty, Some(&sample(0.3))).unwrap().0;
    let point = mm3e_kit::Vec3::new(0.0, -0.467, 0.66);
    assert!(opened.sample_object(0, point).unwrap().dist > 0.0);
    assert!(closed.sample_object(0, point).unwrap().dist < 0.0);
    let morph = deform::inspect(&document, "skin", Some(&sample(0.2))).unwrap();
    assert!((morph["vertices"][0][2].as_f64().unwrap() - 0.1).abs() < 1e-6);
    assert!((opened.objects[4].xform.pos.y - 0.5).abs() < 1e-6);
    assert!(closed.objects[4].xform.pos.y.abs() < 1e-6);
    assert_eq!(
        serde_json::to_value(&clip.lip_sync.as_ref().unwrap().report).unwrap(),
        serde_json::to_value(report).unwrap()
    );
}

#[test]
fn optional_analysis_hash_seals_exact_report_bytes_before_authoring() {
    let scratch = Scratch::new();
    let document = document(8107);
    let report = report(&document, 8107, cues());
    scratch.report(&report);
    let mut sealed = request();
    sealed.analysis_sha256 = Some(format!("{:x}", Sha256::digest(fs::read(scratch.0.join("analysis.json")).unwrap())));
    let mut valid = document.clone();
    lip_sync::generate(&mut valid, &scratch.0, &sealed).unwrap();

    // Reformatting preserves the parsed report, but changes the exact analyzed
    // artifact bytes. An agent's explicitly supplied identity must still fail.
    fs::write(scratch.0.join("analysis.json"), serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    let mut changed = document.clone();
    let original = serde_json::to_value(&changed).unwrap();
    assert!(lip_sync::generate(&mut changed, &scratch.0, &sealed).is_err());
    assert_eq!(serde_json::to_value(&changed).unwrap(), original);
    lip_sync::generate(&mut changed, &scratch.0, &request()).unwrap();
    assert_eq!(serde_json::to_value(changed).unwrap(), serde_json::to_value(valid).unwrap());
}

#[test]
fn independent_profile_or_backend_metadata_corruption_is_rejected_but_curve_edits_remain_valid() {
    let scratch = Scratch::new();
    let mut document = document(8107);
    scratch.report(&report(&document, 8107, cues()));
    lip_sync::generate(&mut document, &scratch.0, &request()).unwrap();
    for backend in [false, true] {
        let mut changed = document.clone();
        let provenance = changed.clips[1].lip_sync.as_mut().unwrap();
        if backend {
            provenance.report.backend.binary_sha256.replace_range(..1, "a");
        } else {
            provenance.profile.poses[1].values[0] = serde_json::from_value(json!({
                "type":"face","face":"face","channel":"jaw_open","value":0.75
            }))
            .unwrap();
        }
        assert!(lip_sync::inspect(&changed, "speech-take").is_err(), "metadata mutation must invalidate provenance");
        assert!(changed.compile(&Pass::Beauty).is_err());
        let loaded: Document = serde_json::from_slice(&serde_json::to_vec(&changed).unwrap()).unwrap();
        assert!(loaded.compile(&Pass::Beauty).is_err());
    }
    document.clips[1].face_tracks[0].keys[1].value = 0.75;
    document.compile(&Pass::Beauty).unwrap();
    assert_eq!(lip_sync::inspect(&document, "speech-take").unwrap()["edited"], true);
}

#[test]
fn zero_transition_is_a_native_step_and_tiny_collapsed_transition_rejects_atomically() {
    let scratch = Scratch::new();
    let mut document = document(8107);
    scratch.report(&report(&document, 8107, cues()));
    let mut step = request();
    step.transition_seconds = 0.0;
    lip_sync::generate(&mut document, &scratch.0, &step).unwrap();
    assert_eq!(jaw(&document, 0.299), 1.0);
    assert_eq!(jaw(&document, 0.3), 0.0);
    let before = serde_json::to_value(&document).unwrap();
    step.transition_seconds = f32::MIN_POSITIVE;
    step.replace = true;
    let error = lip_sync::generate(&mut document, &scratch.0, &step).unwrap_err();
    assert!(error.message.contains("collapse"), "{error:?}");
    assert_eq!(serde_json::to_value(&document).unwrap(), before);
}

#[test]
fn retained_provenance_detects_curve_edits_and_survives_source_and_target_removal() {
    let scratch = Scratch::new();
    let mut document = document(8107);
    scratch.report(&report(&document, 8107, cues()));
    lip_sync::generate(&mut document, &scratch.0, &request()).unwrap();
    let state = lip_sync::inspect(&document, "speech-take").unwrap();
    let loaded: Document = serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
    assert_eq!(lip_sync::inspect(&loaded, "speech-take").unwrap(), state);
    let original = serde_json::to_value(&document.clips[1].lip_sync).unwrap();
    document.clips[1].face_tracks[0].keys[1].value = 0.7;
    assert_eq!(lip_sync::inspect(&document, "speech-take").unwrap()["edited"], true);
    assert_eq!(serde_json::to_value(&document.clips[1].lip_sync).unwrap(), original);
    document.audio.clear();
    document.clips[1].face_tracks.clear();
    document.clips[1].morph_tracks.clear();
    document.clips[1].tracks.clear();
    document.clips[0].face_tracks.clear();
    document.faces.clear();
    document.deformers.clear();
    document.joints.clear();
    document.compile(&Pass::Beauty).unwrap();
    let state = lip_sync::inspect(&document, "speech-take").unwrap();
    assert_eq!(state["source_audio"]["status"], "missing");
    assert_eq!(state["edited"], true);
    let loaded: Document = serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
    assert_eq!(lip_sync::inspect(&loaded, "speech-take").unwrap(), state);
}

#[test]
fn invalid_mapping_and_implicit_replacement_leave_the_document_unchanged() {
    let scratch = Scratch::new();
    let document = document(8107);
    scratch.report(&report(&document, 8107, cues()));
    let before = serde_json::to_value(&document).unwrap();
    let mut variants = vec![];
    let mut missing = request();
    missing.profile.poses.pop();
    variants.push(missing);
    let mut duplicate = request();
    duplicate.profile.poses.push(duplicate.profile.poses[0].clone());
    variants.push(duplicate);
    let mut channel = request();
    let value = channel.profile.poses[1].values[0].clone();
    channel.profile.poses[1].values.push(value);
    variants.push(channel);
    let mut unknown = request();
    unknown.profile.poses[1].values[0] =
        serde_json::from_value(json!({"type":"face","face":"missing","channel":"jaw_open","value":0.5})).unwrap();
    variants.push(unknown);
    let mut bounds = request();
    bounds.profile.poses[1].values[1] =
        serde_json::from_value(json!({"type":"morph","deformer":"skin","blendshape":"open","value":1.01})).unwrap();
    variants.push(bounds);
    for request in variants {
        let mut candidate = document.clone();
        assert!(lip_sync::generate(&mut candidate, &scratch.0, &request).is_err());
        assert_eq!(serde_json::to_value(candidate).unwrap(), before);
    }
    let mut candidate = document.clone();
    lip_sync::generate(&mut candidate, &scratch.0, &request()).unwrap();
    let generated = serde_json::to_value(&candidate).unwrap();
    assert!(lip_sync::generate(&mut candidate, &scratch.0, &request()).is_err());
    assert_eq!(serde_json::to_value(&candidate).unwrap(), generated);
    let mut replacement = request();
    replacement.replace = true;
    lip_sync::generate(&mut candidate, &scratch.0, &replacement).unwrap();
    assert_eq!(serde_json::to_value(candidate).unwrap(), generated);
}

#[test]
fn curve_budgets_are_enforced_without_decimating_short_mouth_closures() {
    let scratch = Scratch::new();
    let mut document = document(80000);
    let cues: Vec<_> = (0..1000).map(|i| json!({"start_cs":i,"end_cs":i+1,"shape":if i%2==0{"A"}else{"D"}})).collect();
    scratch.report(&report(&document, 80000, json!(cues)));
    let before = serde_json::to_value(&document).unwrap();
    let error = lip_sync::generate(&mut document, &scratch.0, &request()).unwrap_err();
    assert!(error.message.contains("1024 keys"), "{error:?}");
    assert_eq!(serde_json::to_value(document).unwrap(), before);
}

#[test]
fn generated_clip_can_be_layered_with_authored_blinks_and_body_motion() {
    let scratch = Scratch::new();
    let mut document = document(8107);
    scratch.report(&report(&document, 8107, cues()));
    lip_sync::generate(&mut document, &scratch.0, &request()).unwrap();
    document.clips.push(
        serde_json::from_value(json!({"id":"combined","duration":2,"layers":[
            {"id":"body-and-eyes","clip":"performance"},{"id":"mouth","clip":"speech-take"}
        ]}))
        .unwrap(),
    );
    let sample = AnimationSample { clip: "combined".into(), time: 0.2, playback: Playback::Clamp };
    let controls = face::inspect_controls(&document, Some(&sample)).unwrap();
    assert!((controls["faces"][0]["controls"]["blink_left"].as_f64().unwrap() - 0.2).abs() < 1e-6);
    assert_eq!(controls["faces"][0]["controls"]["jaw_open"], 1.0);
    let scene = document.compile_at(&Pass::Beauty, Some(&sample)).unwrap().0;
    assert!((scene.objects[0].xform.pos.x - 0.02).abs() < 1e-6);
    animation::validate(&document).unwrap();
}
