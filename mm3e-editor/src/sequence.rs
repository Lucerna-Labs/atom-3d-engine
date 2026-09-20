//! Bounded animation export through the same renderer used by still observations.
//!
//! A sequence owns a newly created directory. It never replaces an existing directory,
//! and the manifest is written last so its presence marks a fully rendered sequence.

use crate::{
    animation::{AnimationSample, Playback},
    model::{Document, Pass, View},
    observe,
    protocol::Failure,
    storage,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, path::Path};

pub const MAX_SEQUENCE_FRAMES: usize = 2400;
pub const MAX_SEQUENCE_PIXELS: u64 = 8_000_000_000;
pub const MAX_SEQUENCE_PIXEL_SAMPLES: u64 = 32_000_000_000;

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SequenceFormat {
    #[default]
    Png,
    Exr,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SequenceRequest {
    /// A new directory relative to the editor root. Its parent must already exist.
    pub directory: String,
    pub clip: String,
    /// Inclusive sampling range in seconds. Samples are start + frame_index / fps.
    pub start: f32,
    pub end: f32,
    pub fps: f32,
    #[serde(default)]
    pub playback: Playback,
    #[serde(default)]
    pub pass: Pass,
    #[serde(default)]
    pub view: Option<View>,
    #[serde(default)]
    pub format: SequenceFormat,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub(crate) struct FrameTime {
    pub(crate) scheduled: f64,
    pub(crate) sampled: f32,
}

pub(crate) struct PreparedExport {
    pub(crate) range: SequenceRequest,
    pub(crate) times: Vec<FrameTime>,
    pub(crate) manifest: Value,
    pub(crate) audio: Option<Vec<u8>>,
}

pub(crate) fn frame_times(request: &SequenceRequest) -> Result<Vec<FrameTime>, Failure> {
    if !request.start.is_finite() || !request.end.is_finite() || request.start < 0.0 || request.end < request.start {
        return Err(Failure::invalid("sequence times must be finite with 0 <= start <= end"));
    }
    if !request.fps.is_finite() || request.fps <= 0.0 {
        return Err(Failure::invalid("sequence fps must be finite and positive"));
    }
    let mut times = Vec::<FrameTime>::new();
    // Calculate from the index, avoiding the accumulating error of repeated additions.
    // One extra candidate detects an oversized request without allocating all its frames.
    for index in 0..=MAX_SEQUENCE_FRAMES {
        let scheduled = f64::from(request.start) + index as f64 / f64::from(request.fps);
        if scheduled > f64::from(request.end) {
            break;
        }
        if times.len() == MAX_SEQUENCE_FRAMES {
            return Err(Failure::invalid(format!("sequence exceeds {MAX_SEQUENCE_FRAMES} frames")));
        }
        let sampled = scheduled as f32;
        if times.last().is_some_and(|previous| sampled <= previous.sampled) {
            return Err(Failure::invalid(
                "sequence frame times are indistinguishable at animation's f32 time precision; reduce fps or start",
            ));
        }
        times.push(FrameTime { scheduled, sampled });
    }
    Ok(times)
}

fn incomplete(error: Failure, directory: &Path, completed: usize) -> Failure {
    Failure {
        code: error.code,
        message: format!(
            "sequence export failed after {completed} completed image frames in {}: {}. \
             The directory may contain partial output and is preserved; use a new directory to retry",
            directory.display(),
            error.message
        ),
    }
}

pub fn render(document: &Document, root: &Path, request: &SequenceRequest) -> Result<Value, Failure> {
    let times = frame_times(request)?;
    render_scheduled(document, root, request, times, |_| Ok(()))
}

/// Shared real renderer for legacy seconds ranges and exact authored shot schedules.
/// The finalizer writes any required sidecar before the completion manifest exists.
pub(crate) fn render_scheduled(
    document: &Document,
    root: &Path,
    request: &SequenceRequest,
    times: Vec<FrameTime>,
    finalize: impl FnOnce(&mut Value) -> Result<(), Failure>,
) -> Result<Value, Failure> {
    let target = storage::path(root, &request.directory, false)?;
    match fs::symlink_metadata(&target) {
        Ok(_) => return Err(Failure::invalid("sequence directory already exists; choose a new directory")),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(Failure::io(error.to_string())),
    }
    let mut manifest = prepare(document, request, &times)?;
    let total_pixels = manifest["total_pixels"].as_u64().expect("validated pixel count");
    // create_dir is an atomic no-clobber reservation, including competing exporters.
    // Individual files also use the storage module's no-clobber writes. A portable
    // directory rename cannot promise no-clobber, so failures explicitly retain output.
    fs::create_dir(&target).map_err(|error| Failure::io(format!("cannot create sequence directory: {error}")))?;
    let mut frames = Vec::with_capacity(times.len());
    for (index, time) in times.iter().enumerate() {
        let extension = match request.format {
            SequenceFormat::Png => "png",
            SequenceFormat::Exr => "exr",
        };
        let name = format!("frame_{index:04}.{extension}");
        let relative = Path::new(&request.directory).join(name);
        let relative = relative.to_str().expect("UTF-8 directory plus ASCII filename");
        let sample =
            AnimationSample { clip: request.clip.clone(), time: time.sampled, playback: request.playback.clone() };
        let result =
            observe::image(document, root, relative, &request.pass, request.view.as_ref(), false, Some(&sample))
                .map_err(|error| incomplete(error, &target, frames.len()))?;
        frames.push(json!({
            "index": index,
            "path": result["path"],
            "time": time.sampled,
            "scheduled_time": time.scheduled,
            "rgba_fnv1a64": result["rgba_fnv1a64"],
            "linear_rgb_fnv1a64": result["linear_rgb_fnv1a64"],
            "color_encoding": result["color_encoding"],
            "film":result["film"],"channels":result["channels"],
            "view": result["view"],
        }));
    }
    manifest["frames"] = json!(frames);
    finalize(&mut manifest).map_err(|error| incomplete(error, &target, frames.len()))?;
    let manifest_relative = Path::new(&request.directory).join("manifest.json");
    let manifest_relative = manifest_relative.to_str().expect("UTF-8 directory plus ASCII filename");
    let bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| incomplete(Failure::io(error.to_string()), &target, frames.len()))?;
    let manifest_path = storage::write(root, manifest_relative, &bytes, false)
        .map_err(|error| incomplete(error, &target, frames.len()))?;
    Ok(json!({
        "directory": target,
        "manifest_path": manifest_path,
        "frame_count": frames.len(),
        "total_pixels": total_pixels,
        "width": document.settings.width,
        "height": document.settings.height,
        "clip": request.clip,
        "fps": request.fps,
        "start": request.start,
        "end": request.end,
        "first_frame_time": times.first().expect("inclusive valid range has a first frame").sampled,
        "last_frame_time": times.last().expect("inclusive valid range has a last frame").sampled,
    }))
}

/// Preflight and describe a complete immutable export without creating output.
pub(crate) fn prepare(document: &Document, request: &SequenceRequest, times: &[FrameTime]) -> Result<Value, Failure> {
    if matches!(request.format, SequenceFormat::Exr) && !matches!(request.pass, Pass::Beauty) {
        return Err(Failure::invalid(
            "EXR sequences require beauty; diagnostic display passes are not scene-linear data",
        ));
    }
    if times.is_empty() || times.len() > MAX_SEQUENCE_FRAMES {
        return Err(Failure::invalid("scheduled sequence requires 1..2400 frames"));
    }
    for (i, time) in times.iter().enumerate() {
        if !time.scheduled.is_finite()
            || !time.sampled.is_finite()
            || time.sampled != time.scheduled as f32
            || (i > 0 && (time.scheduled <= times[i - 1].scheduled || time.sampled <= times[i - 1].sampled))
        {
            return Err(Failure::invalid(
                "scheduled sequence times must be finite, distinct and match actual f32 sampling",
            ));
        }
    }
    let view = request.view.as_ref().unwrap_or(&document.camera);
    view.validate().map_err(Failure::invalid)?;
    let pixels_per_frame = u64::from(document.settings.width) * u64::from(document.settings.height);
    let total_pixels = pixels_per_frame
        .checked_mul(times.len() as u64)
        .ok_or_else(|| Failure::invalid("sequence pixel count overflow"))?;
    if total_pixels > MAX_SEQUENCE_PIXELS {
        return Err(Failure::invalid(format!("sequence exceeds {MAX_SEQUENCE_PIXELS} total pixels")));
    }
    let total_samples = crate::shot::sample_cost(document)
        .map_err(Failure::invalid)?
        .checked_mul(times.len() as u64)
        .ok_or_else(|| Failure::invalid("sequence sample count overflow"))?;
    if total_samples > MAX_SEQUENCE_PIXEL_SAMPLES {
        return Err(Failure::invalid(format!("sequence exceeds {MAX_SEQUENCE_PIXEL_SAMPLES} primary pixel samples")));
    }
    // Validate all evaluated poses before creating output; this also checks the clip and
    // renderer settings. A malformed late keyframe must not leave an avoidable partial export.
    for time in times {
        let sample =
            AnimationSample { clip: request.clip.clone(), time: time.sampled, playback: request.playback.clone() };
        crate::shot::preflight(document, &request.pass, request.view.as_ref(), Some(&sample))
            .map_err(Failure::invalid)?;
    }
    let document_bytes = serde_json::to_vec(document).map_err(|error| Failure::invalid(error.to_string()))?;
    let document_fingerprint = mm3e_kit::atoms::hash(&document_bytes);

    let manifest = json!({
        "format": "mm3e-animation-sequence",
        "version": 1,
        "clip": request.clip,
        "start": request.start,
        "end": request.end,
        "fps": request.fps,
        "playback": request.playback,
        "pass": request.pass,
        "image_format": request.format,
        "view_override": request.view,
        "document_fnv1a64": format!("{document_fingerprint:016x}"),
        "quality": document.settings.quality,
        "width": document.settings.width,
        "height": document.settings.height,
        "frame_count": times.len(),
        "total_pixels": total_pixels,
        "primary_pixel_samples":total_samples,
        "backend": "mm3e_orchestrator_cpu",
        "time_semantics": "scheduled_time = start + index/fps in f64, inclusive <= end; time is the exact f32 sample supplied to animation; playback maps sample time into the clip",
        "fingerprint_semantics": "rgba_fnv1a64 hashes display RGBA8; linear_rgb_fnv1a64 hashes row-major RGB f32 little-endian channel bytes; neither hashes the encoded file; document_fnv1a64 hashes serde_json::to_vec of the full authored document",
        "frames": [],
    });
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct TestRoot(PathBuf);
    impl TestRoot {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "mm3e-sequence-tests-{}-{}",
                std::process::id(),
                TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path.canonicalize().unwrap())
        }
    }
    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn request(start: f32, end: f32, fps: f32) -> SequenceRequest {
        SequenceRequest {
            directory: "sequence".into(),
            clip: "motion".into(),
            start,
            end,
            fps,
            playback: Playback::default(),
            pass: Pass::default(),
            view: None,
            format: SequenceFormat::Png,
        }
    }

    #[test]
    fn inclusive_sampling_keeps_exact_endpoints_without_accumulation() {
        let times = frame_times(&request(0.0, 2.0, 12.0)).unwrap();
        assert_eq!(times.len(), 25);
        assert_eq!(times[0].sampled, 0.0);
        assert_eq!(times[24].sampled, 2.0);
        for (index, time) in times.iter().enumerate() {
            assert_eq!(time.scheduled, index as f64 / 12.0);
        }
    }

    #[test]
    fn sampling_supports_single_frame_and_non_grid_end() {
        assert_eq!(frame_times(&request(3.0, 3.0, 60.0)).unwrap().len(), 1);
        let times = frame_times(&request(0.0, 1.1, 2.0)).unwrap();
        assert_eq!(times.len(), 3);
        assert_eq!(times[2].sampled, 1.0);
    }

    #[test]
    fn sampling_rejects_bad_ranges_and_unrepresentable_steps() {
        for r in [
            request(-1.0, 1.0, 1.0),
            request(2.0, 1.0, 1.0),
            request(f32::NAN, 1.0, 1.0),
            request(0.0, f32::INFINITY, 1.0),
            request(0.0, 1.0, 0.0),
            request(0.0, 1.0, -1.0),
            request(0.0, 1.0, f32::INFINITY),
            request(1_000_000.0, 1_000_001.0, 120.0),
        ] {
            assert!(frame_times(&r).is_err(), "accepted invalid request: {r:?}");
        }
    }

    #[test]
    fn inclusive_frame_cap_accounts_for_the_first_frame() {
        assert_eq!(
            frame_times(&request(0.0, (MAX_SEQUENCE_FRAMES - 1) as f32, 1.0)).unwrap().len(),
            MAX_SEQUENCE_FRAMES
        );
        assert!(frame_times(&request(0.0, MAX_SEQUENCE_FRAMES as f32, 1.0)).is_err());
        assert!(frame_times(&request(0.0, f32::MAX, f32::MAX)).is_err());
    }

    #[test]
    fn sequence_schema_defaults_are_accepted_but_unknown_fields_are_rejected() {
        let mut value = json!({"directory": "motion", "clip": "wave", "start": 0, "end": 1, "fps": 24});
        let parsed: SequenceRequest = serde_json::from_value(value.clone()).unwrap();
        assert!(parsed.view.is_none());
        value["overwrite"] = json!(true);
        assert!(serde_json::from_value::<SequenceRequest>(value).is_err());
    }

    #[test]
    fn oversized_render_requests_create_no_directory() {
        let root = TestRoot::new();
        let mut document = Document::default();
        document.settings.width = 4096;
        document.settings.height = 4096;
        let error = render(&document, &root.0, &request(0.0, 999.0, 1.0)).unwrap_err();
        assert!(error.message.contains("total pixels"));
        assert!(!root.0.join("sequence").exists());
        let error = render(&document, &root.0, &request(0.0, MAX_SEQUENCE_FRAMES as f32, 1.0)).unwrap_err();
        assert!(error.message.contains("frames"));
        assert!(!root.0.join("sequence").exists());
    }

    #[test]
    fn existing_directories_are_never_replaced() {
        let root = TestRoot::new();
        let directory = root.0.join("sequence");
        fs::create_dir(&directory).unwrap();
        fs::write(directory.join("keep.txt"), b"keep existing content").unwrap();
        let error = render(&Document::default(), &root.0, &request(0.0, 1.0, 1.0)).unwrap_err();
        assert!(error.message.contains("already exists"));
        assert_eq!(fs::read(directory.join("keep.txt")).unwrap(), b"keep existing content");
        assert_eq!(fs::read_dir(directory).unwrap().count(), 1);
    }

    #[test]
    fn sidecar_collision_preserves_images_and_foreign_file_without_a_completion_manifest() {
        let root = TestRoot::new();
        let document = Document {
            settings: serde_json::from_value(
                json!({"width":8,"height":8,"quality":"preview","shadows":false,"ao":false}),
            )
            .unwrap(),
            clips: serde_json::from_value(json!([{"id":"motion","duration":1}])).unwrap(),
            ..Document::default()
        };
        let error = render_scheduled(
            &document,
            &root.0,
            &request(0.0, 0.0, 24.0),
            vec![FrameTime { scheduled: 0.0, sampled: 0.0 }],
            |_| {
                fs::write(root.0.join("sequence/audio.wav"), b"concurrent output").unwrap();
                storage::write(&root.0, "sequence/audio.wav", b"must not replace", false)?;
                Ok(())
            },
        )
        .unwrap_err();
        assert!(error.message.contains("after 1 completed image frames"));
        assert!(root.0.join("sequence/frame_0000.png").is_file());
        assert_eq!(fs::read(root.0.join("sequence/audio.wav")).unwrap(), b"concurrent output");
        assert!(!root.0.join("sequence/manifest.json").exists());
    }

    #[cfg(unix)]
    #[test]
    fn symlink_parent_cannot_export_outside_root() {
        let root = TestRoot::new();
        let outside = TestRoot::new();
        std::os::unix::fs::symlink(&outside.0, root.0.join("escape")).unwrap();
        let mut r = request(0.0, 1.0, 1.0);
        r.directory = "escape/sequence".into();
        let error = render(&Document::default(), &root.0, &r).unwrap_err();
        assert!(error.message.contains("escapes the editor root"));
        assert!(!outside.0.join("sequence").exists());
    }
}
