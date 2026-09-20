//! Persistent bounded render jobs. Scene snapshots and plans are immutable; a
//! stable OS lock serializes progress while a separate control lock permits
//! cancellation between frames. This follows the project's cooperating-process
//! filesystem model, not a hostile-filesystem security boundary.
use crate::{
    animation::AnimationSample,
    dialogue,
    model::Document,
    native, observe,
    project::ProjectStore,
    protocol::Failure,
    sequence::{self, FrameTime, SequenceFormat, SequenceRequest},
    storage,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub const MAX_JOB_STEP_FRAMES: u32 = 32;
const MAX_FRAME_BYTES: u64 = 1_073_741_824;
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RenderJobRequest {
    Sequence(SequenceRequest),
    Shot(dialogue::RenderShot),
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    format: String,
    directory: PathBuf,
    source_revision: u64,
    snapshot_sha256: String,
    binary_sha256: String,
    range: SequenceRequest,
    times: Vec<FrameTime>,
    manifest: Value,
    audio: Option<Blob>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Blob {
    bytes: u64,
    sha256: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Frame {
    file: Blob,
    record: Value,
    /// Detect independent metadata corruption as well as changed image bytes.
    record_sha256: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pending {
    index: usize,
    attempt: u64,
    frame: Frame,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    plan_sha256: String,
    next_attempt: u64,
    frames: Vec<Frame>,
    pending: Option<Pending>,
    completed_manifest: Option<Blob>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Control {
    plan_sha256: String,
    cancelled: bool,
}
struct Job {
    root: PathBuf,
    plan: Plan,
    plan_sha256: String,
}
fn invalid(message: impl Into<String>) -> Failure {
    Failure { code: "render_job_invalid", message: message.into() }
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn encoded<T: Serialize>(value: &T) -> Result<Vec<u8>, Failure> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| invalid(e.to_string()))?;
    if bytes.len() as u64 > storage::MAX_DOCUMENT_BYTES {
        return Err(invalid("render-job metadata exceeds 64 MiB"));
    }
    Ok(bytes)
}
fn regular(path: &Path) -> Result<fs::Metadata, Failure> {
    let meta = fs::symlink_metadata(path).map_err(|e| Failure::io(format!("{}: {e}", path.display())))?;
    if !meta.is_file() || meta.file_type().is_symlink() {
        return Err(invalid(format!("job file must be regular and not a symlink: {}", path.display())));
    }
    Ok(meta)
}
fn read(root: &Path, name: &str) -> Result<Vec<u8>, Failure> {
    regular(&root.join(name))?;
    storage::read(root, name)
}
fn file_blob(path: &Path) -> Result<Blob, Failure> {
    let metadata = regular(path)?;
    if metadata.len() > MAX_FRAME_BYTES {
        return Err(invalid("render-job file exceeds 1 GiB"));
    }
    let mut file = File::open(path).map_err(|e| Failure::io(e.to_string()))?;
    let mut hash = Sha256::new();
    let mut bytes = 0;
    let mut buffer = [0u8; 65536];
    loop {
        let n = file.read(&mut buffer).map_err(|e| Failure::io(e.to_string()))?;
        if n == 0 {
            break;
        }
        bytes += n as u64;
        if bytes > MAX_FRAME_BYTES {
            return Err(invalid("render-job file grew past 1 GiB"));
        }
        hash.update(&buffer[..n]);
    }
    Ok(Blob { bytes, sha256: format!("{:x}", hash.finalize()) })
}
fn verify(path: &Path, expected: &Blob) -> Result<(), Failure> {
    if file_blob(path)? != *expected {
        return Err(Failure {
            code: "render_job_corrupt",
            message: format!("file bytes differ from committed render-job record: {}", path.display()),
        });
    }
    Ok(())
}
fn binary_hash() -> Result<String, Failure> {
    static HASH: OnceLock<Result<String, String>> = OnceLock::new();
    HASH.get_or_init(|| {
        std::env::current_exe()
            .map_err(|e| e.to_string())
            .and_then(|p| file_blob(&p).map(|b| b.sha256).map_err(|e| e.message))
    })
    .as_ref()
    .cloned()
    .map_err(|s| Failure::io(format!("cannot fingerprint renderer executable: {s}")))
}
fn sync_dir(path: &Path) -> Result<(), Failure> {
    #[cfg(unix)]
    File::open(path).and_then(|f| f.sync_all()).map_err(|e| Failure::io(e.to_string()))?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}
impl Job {
    fn open(root: &Path, directory: &str) -> Result<Self, Failure> {
        let target = storage::path(root, directory, false)?;
        let meta = fs::symlink_metadata(&target).map_err(|e| Failure::io(e.to_string()))?;
        if !meta.is_dir() || meta.file_type().is_symlink() {
            return Err(invalid("render job must be a non-symlink directory"));
        }
        let bytes = read(&target, "job.json")?;
        let plan: Plan = serde_json::from_slice(&bytes).map_err(|e| invalid(format!("invalid job plan: {e}")))?;
        if plan.format != "mm3e-render-job-v1"
            || plan.directory != target
            || plan.times.is_empty()
            || plan.times.len() > sequence::MAX_SEQUENCE_FRAMES
        {
            return Err(invalid("job format, location or frame count differs from the frozen plan"));
        }
        if plan.manifest["frames"].as_array().is_none_or(|v| v.len() != plan.times.len()) {
            return Err(invalid("job frame annotations do not match its schedule"));
        }
        let stage = fs::symlink_metadata(target.join(".render-staging")).map_err(|e| Failure::io(e.to_string()))?;
        if !stage.is_dir() || stage.file_type().is_symlink() {
            return Err(invalid("job staging directory is invalid"));
        }
        Ok(Self { root: target, plan, plan_sha256: sha(&bytes) })
    }
    fn state(&self, bytes: &[u8]) -> Result<State, Failure> {
        let state: State = serde_json::from_slice(bytes).map_err(|e| invalid(e.to_string()))?;
        if state.plan_sha256 != self.plan_sha256
            || state.frames.len() > self.plan.times.len()
            || state.pending.as_ref().is_some_and(|p| {
                p.index != state.frames.len() || p.index >= self.plan.times.len() || p.attempt >= state.next_attempt
            })
            || state.completed_manifest.is_some()
                && (state.frames.len() != self.plan.times.len() || state.pending.is_some())
        {
            return Err(invalid("render-job progress does not match its immutable plan"));
        }
        for (i, f) in state.frames.iter().enumerate() {
            Self::verify_record(f)?;
            self.check_record(i, &f.record)?;
        }
        if let Some(p) = &state.pending {
            Self::verify_record(&p.frame)?;
            self.check_record(p.index, &p.frame.record)?;
        }
        Ok(state)
    }
    fn verify_record(frame: &Frame) -> Result<(), Failure> {
        let bytes = serde_json::to_vec(&frame.record).map_err(|e| invalid(e.to_string()))?;
        if sha(&bytes) != frame.record_sha256 {
            return Err(Failure {
                code: "render_job_corrupt",
                message: "renderer observation metadata differs from its committed digest".into(),
            });
        }
        Ok(())
    }
    fn check_record(&self, index: usize, record: &Value) -> Result<(), Failure> {
        let planned = &self.plan.manifest["frames"][index];
        for key in ["index", "time", "scheduled_time", "shot_index", "shot_frame", "audio_window"] {
            if record[key] != planned[key] {
                return Err(invalid(format!("committed frame {index} has different {key}")));
            }
        }
        if record["path"] != json!(self.root.join(self.frame_name(index))) {
            return Err(invalid("committed frame path differs from the owned destination"));
        }
        Ok(())
    }
    fn read_state(&self) -> Result<State, Failure> {
        self.state(&read(&self.root, "state.json")?)
    }
    fn open_state(&self) -> Result<(ProjectStore, State), Failure> {
        regular(&self.root.join("state.json"))?;
        let (store, bytes) = ProjectStore::open(&self.root, "state.json")?;
        let state = self.state(&bytes.ok_or_else(|| invalid("missing job state"))?)?;
        Ok((store, state))
    }
    fn control(&self) -> Result<Control, Failure> {
        let c: Control =
            serde_json::from_slice(&read(&self.root, "control.json")?).map_err(|e| invalid(e.to_string()))?;
        if c.plan_sha256 != self.plan_sha256 {
            return Err(invalid("job control belongs to a different plan"));
        }
        Ok(c)
    }
    fn frame_name(&self, index: usize) -> String {
        format!(
            "frame_{index:04}.{}",
            match self.plan.range.format {
                SequenceFormat::Png => "png",
                SequenceFormat::Exr => "exr",
            }
        )
    }
    fn staging_name(&self, index: usize, attempt: u64) -> String {
        format!(
            ".render-staging/{attempt:016}-{index:04}.{}",
            match self.plan.range.format {
                SequenceFormat::Png => "png",
                SequenceFormat::Exr => "exr",
            }
        )
    }
    fn snapshot(&self) -> Result<Document, Failure> {
        let bytes = read(&self.root, "snapshot.json")?;
        if sha(&bytes) != self.plan.snapshot_sha256 {
            return Err(Failure { code: "render_job_corrupt", message: "frozen scene snapshot hash differs".into() });
        }
        let saved: crate::Saved = serde_json::from_slice(&bytes).map_err(|e| invalid(e.to_string()))?;
        if saved.format != "mm3e-agent-project-v1" || saved.saved_revision != self.plan.source_revision {
            return Err(invalid("snapshot envelope differs from the job plan"));
        }
        Ok(saved.document)
    }
    fn verify_frames(&self, state: &State) -> Result<(), Failure> {
        for (i, f) in state.frames.iter().enumerate() {
            verify(&self.root.join(self.frame_name(i)), &f.file)?;
        }
        Ok(())
    }
    fn summary(&self, state: &State, verified: bool) -> Result<Value, Failure> {
        let cancelled = self.control()?.cancelled;
        Ok(
            json!({"directory":self.root,"status":if state.completed_manifest.is_some(){"complete"}else if cancelled{"cancelled"}else{"pending"},
            "completed_frames":state.frames.len(),"total_frames":self.plan.times.len(),"pending_frame":state.pending.as_ref().map(|p|p.index),
            "source_revision":self.plan.source_revision,"snapshot_sha256":self.plan.snapshot_sha256,"binary_sha256":self.plan.binary_sha256,"plan_sha256":self.plan_sha256,
            "manifest_path":state.completed_manifest.as_ref().map(|_|self.root.join("manifest.json")),"outputs_verified":verified,
            "semantics":"Completed frames belong to the frozen native snapshot and executable. Cancellation is observed between frames; an in-flight frame may finish. A pending status alone does not imply a worker is running."}),
        )
    }
}

pub fn create(document: &Document, revision: u64, root: &Path, request: &RenderJobRequest) -> Result<Value, Failure> {
    let sequence::PreparedExport { range, times, mut manifest, audio } = match request {
        RenderJobRequest::Sequence(range) => {
            let times = sequence::frame_times(range)?;
            let mut m = sequence::prepare(document, range, &times)?;
            m["frames"] = json!(times
                .iter()
                .enumerate()
                .map(|(i, t)| json!({"index":i,"time":t.sampled,"scheduled_time":t.scheduled}))
                .collect::<Vec<_>>());
            sequence::PreparedExport { range: range.clone(), times, manifest: m, audio: None }
        }
        RenderJobRequest::Shot(request) => dialogue::prepare_job(document, request)?,
    };
    let target = storage::path(root, &range.directory, false)?;
    if fs::symlink_metadata(&target).is_ok() {
        return Err(invalid("render-job directory already exists; use job commands to resume its frozen plan"));
    }
    let snapshot = native::bytes(document, revision)?;
    let audio_blob = audio.as_ref().map(|b| Blob { bytes: b.len() as u64, sha256: sha(b) });
    if audio.is_some() {
        manifest["audio"]["path"] = json!(target.join("audio.wav"));
    }
    let plan = Plan {
        format: "mm3e-render-job-v1".into(),
        directory: target.clone(),
        source_revision: revision,
        snapshot_sha256: sha(&snapshot),
        binary_sha256: binary_hash()?,
        range,
        times,
        manifest,
        audio: audio_blob,
    };
    let plan_bytes = encoded(&plan)?;
    let plan_hash = sha(&plan_bytes);
    fs::create_dir(&target).map_err(|e| Failure::io(e.to_string()))?;
    sync_dir(target.parent().expect("rooted directory"))?;
    fs::create_dir(target.join(".render-staging")).map_err(|e| Failure::io(e.to_string()))?;
    storage::write(&target, "snapshot.json", &snapshot, false)?;
    if let Some(bytes) = audio {
        storage::write(&target, ".render-staging/audio.wav", &bytes, false)?;
    }
    let state = State {
        plan_sha256: plan_hash.clone(),
        next_attempt: 0,
        frames: vec![],
        pending: None,
        completed_manifest: None,
    };
    storage::write(&target, "state.json", &encoded(&state)?, false)?;
    storage::write(&target, "control.json", &encoded(&Control { plan_sha256: plan_hash, cancelled: false })?, false)?;
    // The immutable plan is the readiness marker; partial preparation remains
    // preserved and cannot be mistaken for an initialized resumable job.
    storage::write(&target, "job.json", &plan_bytes, false)?;
    Job::open(root, &plan.range.directory)?.summary(&state, false)
}

/// Install only a previously committed byte identity. An existing matching file
/// is the recoverable post-install crash case; a different file is never replaced.
fn install(root: &Path, source: &str, target: &str, blob: &Blob) -> Result<(), Failure> {
    let destination = root.join(target);
    match fs::symlink_metadata(&destination) {
        Ok(_) => verify(&destination, blob)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            verify(&root.join(source), blob)?;
            fs::hard_link(root.join(source), &destination).map_err(|e| Failure {
                code: "output_state_uncertain",
                message: format!(
                    "job file installation uncertain: {e}; retry the job step to reconcile committed byte identity"
                ),
            })?;
        }
        Err(e) => return Err(Failure::io(e.to_string())),
    }
    sync_dir(root).map_err(|e| Failure {
        code: "output_installed_uncertain",
        message: format!("job file installed but directory synchronization failed: {}; retry job step", e.message),
    })
}
fn persist(store: &mut ProjectStore, state: &State) -> Result<(), Failure> {
    store.write(&encoded(state)?)
}
fn commit_pending(job: &Job, store: &mut ProjectStore, state: &mut State) -> Result<(), Failure> {
    if let Some(pending) = state.pending.as_ref() {
        install(
            &job.root,
            &job.staging_name(pending.index, pending.attempt),
            &job.frame_name(pending.index),
            &pending.frame.file,
        )?;
        let pending = state.pending.take().expect("checked pending");
        state.frames.push(pending.frame);
        persist(store, state)?;
    }
    Ok(())
}
fn finish(job: &Job, store: &mut ProjectStore, state: &mut State) -> Result<(), Failure> {
    job.verify_frames(state)?;
    if let Some(blob) = &job.plan.audio {
        install(&job.root, ".render-staging/audio.wav", "audio.wav", blob)?;
    }
    let mut manifest = job.plan.manifest.clone();
    manifest["frames"] = json!(state
        .frames
        .iter()
        .map(|f| {
            let mut r = f.record.clone();
            r["encoded_file_sha256"] = json!(f.file.sha256);
            r["encoded_file_bytes"] = json!(f.file.bytes);
            r
        })
        .collect::<Vec<_>>());
    manifest["render_job"] = json!({"plan_sha256":job.plan_sha256,"snapshot_sha256":job.plan.snapshot_sha256,"binary_sha256":job.plan.binary_sha256,"source_revision":job.plan.source_revision});
    let bytes = encoded(&manifest)?;
    let blob = Blob { bytes: bytes.len() as u64, sha256: sha(&bytes) };
    match fs::symlink_metadata(job.root.join("manifest.json")) {
        Ok(_) => verify(&job.root.join("manifest.json"), &blob)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            storage::write(&job.root, "manifest.json", &bytes, false)?;
        }
        Err(e) => return Err(Failure::io(e.to_string())),
    }
    state.completed_manifest = Some(blob);
    persist(store, state)
}
pub fn step(root: &Path, directory: &str, max_frames: u32) -> Result<Value, Failure> {
    if !(1..=MAX_JOB_STEP_FRAMES).contains(&max_frames) {
        return Err(invalid("max_frames must be 1..32"));
    }
    let job = Job::open(root, directory)?;
    if binary_hash()? != job.plan.binary_sha256 {
        return Err(Failure{code:"render_job_renderer_mismatch",message:"job requires the exact executable that prepared its snapshot; use that preserved binary or create a new job".into()});
    }
    let (mut store, mut state) = job.open_state()?;
    let document = job.snapshot()?;
    // Verify existing bytes before reporting reuse. This deliberately favors
    // recovery correctness over skipping filesystem reads for large prefixes.
    job.verify_frames(&state)?;
    if let Some(blob) = &state.completed_manifest {
        verify(&job.root.join("manifest.json"), blob)?;
        if let Some(audio) = &job.plan.audio {
            verify(&job.root.join("audio.wav"), audio)?;
        }
        return job.summary(&state, true);
    }
    if job.control()?.cancelled {
        return job.summary(&state, true);
    }
    let initial = state.frames.len();
    commit_pending(&job, &mut store, &mut state)?;
    while state.frames.len() < job.plan.times.len() && state.frames.len() - initial < max_frames as usize {
        if job.control()?.cancelled {
            break;
        }
        let index = state.frames.len();
        let attempt = state.next_attempt;
        state.next_attempt = attempt.checked_add(1).ok_or_else(|| invalid("render-job attempt counter exhausted"))?;
        persist(&mut store, &state)?;
        let time = job.plan.times[index];
        let sample = AnimationSample {
            clip: job.plan.range.clip.clone(),
            time: time.sampled,
            playback: job.plan.range.playback.clone(),
        };
        let name = job.staging_name(index, attempt);
        let result = observe::image(
            &document,
            &job.root,
            &name,
            &job.plan.range.pass,
            job.plan.range.view.as_ref(),
            false,
            Some(&sample),
        )?;
        let file = file_blob(&job.root.join(&name))?;
        let mut record = job.plan.manifest["frames"][index].clone();
        for key in ["rgba_fnv1a64", "linear_rgb_fnv1a64", "color_encoding", "film", "channels", "view"] {
            record[key] = result[key].clone();
        }
        record["path"] = json!(job.root.join(job.frame_name(index)));
        state.pending = Some(Pending {
            index,
            attempt,
            frame: Frame {
                file,
                record_sha256: sha(&serde_json::to_vec(&record).map_err(|e| invalid(e.to_string()))?),
                record,
            },
        });
        persist(&mut store, &state)?;
        commit_pending(&job, &mut store, &mut state)?;
    }
    if state.frames.len() == job.plan.times.len() && !job.control()?.cancelled {
        finish(&job, &mut store, &mut state)?;
    }
    job.summary(&state, true)
}
pub fn state(root: &Path, directory: &str, verify_outputs: bool) -> Result<Value, Failure> {
    let job = Job::open(root, directory)?;
    if verify_outputs {
        let (_store, state) = job.open_state()?;
        job.snapshot()?;
        job.verify_frames(&state)?;
        if let Some(blob) = &state.completed_manifest {
            verify(&job.root.join("manifest.json"), blob)?;
            if let Some(audio) = &job.plan.audio {
                verify(&job.root.join("audio.wav"), audio)?;
            }
        }
        job.summary(&state, true)
    } else {
        job.summary(&job.read_state()?, false)
    }
}
pub fn control(root: &Path, directory: &str, cancelled: bool) -> Result<Value, Failure> {
    let job = Job::open(root, directory)?;
    regular(&job.root.join("control.json"))?;
    let (mut store, bytes) = ProjectStore::open(&job.root, "control.json")?;
    let mut control: Control = serde_json::from_slice(&bytes.ok_or_else(|| invalid("missing job control"))?)
        .map_err(|e| invalid(e.to_string()))?;
    if control.plan_sha256 != job.plan_sha256 {
        return Err(invalid("job control does not match plan"));
    }
    control.cancelled = cancelled;
    store.write(&encoded(&control)?)?;
    job.summary(&job.read_state()?, false)
}
