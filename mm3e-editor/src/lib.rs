//! Agent-operated authoring on the real MM3E renderer. The editor owns policy and documents;
//! geometry evaluation, camera projection and shading remain in the existing kit/orchestrator.
pub mod animation;
pub mod audio;
mod character;
pub mod cloth;
pub mod csg;
pub mod curve_edit;
pub mod deform;
pub mod delivery;
mod delivery_textures;
pub mod delivery_uv;
pub mod dialogue;
pub mod face;
pub mod film;
pub mod garment;
pub mod ik;
pub mod jobs;
pub mod joint_limits;
pub mod layering;
pub mod lip_sync;
pub mod model;
mod native;
mod observe;
pub mod pattern;
pub mod project;
pub mod protocol;
pub mod sequence;
pub mod sewing;
pub mod shot;
pub mod speech;
pub mod speech_backend;
mod storage;
pub mod surface_condition;
mod surface_delivery;
pub mod surface_refine;
pub mod surface_retessellate;
pub mod textures;
pub mod timeline;
pub mod usd;

use model::*;
use protocol::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeSet, VecDeque},
    path::{Path, PathBuf},
};

pub const MAX_HISTORY: usize = 32;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Saved {
    format: String,
    saved_revision: u64,
    document: Document,
}

pub struct Editor {
    document: Document,
    revision: u64,
    undo: VecDeque<Document>,
    redo: Vec<Document>,
    root: PathBuf,
    project: Option<project::ProjectStore>,
    speech_backend: Option<speech_backend::SpeechBackend>,
    speech_backend_error: Option<String>,
}

impl Editor {
    pub fn new(root: &Path) -> Result<Self, Failure> {
        let root = root.canonicalize().map_err(|e| Failure::io(e.to_string()))?;
        if !root.is_dir() {
            return Err(Failure::invalid("editor root must be a directory"));
        }
        Ok(Self {
            document: Document::default(),
            revision: 0,
            undo: VecDeque::new(),
            redo: vec![],
            root,
            project: None,
            speech_backend: None,
            speech_backend_error: None,
        })
    }
    /// Open one durable native project with an OS writer lock. Completed mutations persist
    /// before the live state/revision changes; earlier saved revisions survive process restart.
    pub fn open_project(root: &Path, path: &str) -> Result<Self, Failure> {
        let mut editor = Self::new(root)?;
        let (store, bytes) = project::ProjectStore::open(&editor.root, path)?;
        if let Some(bytes) = bytes {
            let mut saved: Saved = serde_json::from_slice(&bytes).map_err(|e| Failure::invalid(e.to_string()))?;
            if saved.format != "mm3e-agent-project-v1" {
                return Err(Failure::invalid("unsupported project format"));
            }
            face::migrate_legacy_native(&mut saved.document).map_err(Failure::invalid)?;
            saved.document.compile(&Pass::Beauty).map_err(Failure::invalid)?;
            native::budget(&saved.document, saved.saved_revision)?;
            editor.document = saved.document;
            editor.revision = saved.saved_revision;
            editor.project = Some(store);
        } else {
            let bytes = saved_bytes(&editor.document, editor.revision)?;
            let mut store = store;
            store.write(&bytes)?;
            editor.project = Some(store);
        }
        Ok(editor)
    }
    /// Host configuration only: scene documents and JSON commands cannot select an executable.
    pub fn set_speech_backend(&mut self, executable: &Path) -> Result<(), Failure> {
        self.speech_backend = Some(speech_backend::SpeechBackend::open(executable)?);
        self.speech_backend_error = None;
        Ok(())
    }
    /// A broken optional bundled analyzer must not prevent unrelated authoring/rendering.
    pub fn try_optional_speech_backend(&mut self, executable: &Path) {
        if let Err(error) = self.set_speech_backend(executable) {
            self.speech_backend_error = Some(error.message);
        }
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn handle(&mut self, request: Request) -> Response {
        let result = self.execute(request.expected_revision, request.command);
        match result {
            Ok(result) => {
                Response { id: Some(request.id), ok: true, revision: self.revision, result: Some(result), error: None }
            }
            Err(error) => {
                Response { id: Some(request.id), ok: false, revision: self.revision, result: None, error: Some(error) }
            }
        }
    }

    fn execute(&mut self, expected: Option<u64>, command: Command) -> Result<Value, Failure> {
        if self.project.as_ref().is_some_and(project::ProjectStore::is_poisoned) {
            return Err(Failure {
                code: "recovery_required",
                message:
                    "project commit outcome is uncertain; restart this persistent session and reload before continuing"
                        .into(),
            });
        }
        let mutating = matches!(
            command,
            Command::Apply { .. }
                | Command::Undo
                | Command::Redo
                | Command::Load { .. }
                | Command::ImportObj { .. }
                | Command::BakeCloth { .. }
                | Command::SolveIk { .. }
                | Command::ImportAudio { .. }
                | Command::ImportTexture { .. }
                | Command::GenerateLipSync { .. }
        );
        if mutating && expected.is_none() {
            return Err(Failure {
                code: "revision_required",
                message: "mutations require expected_revision; inspect first".into(),
            });
        }
        if expected.is_some_and(|v| v != self.revision) {
            return Err(Failure {
                code: "revision_conflict",
                message: format!("expected {expected:?}; current revision is {}", self.revision),
            });
        }
        if mutating && self.revision == u64::MAX {
            return Err(Failure::invalid("revision space exhausted"));
        }
        match command {
            Command::CreateRenderJob { request } => jobs::create(&self.document, self.revision, &self.root, &request),
            Command::StepRenderJob { directory, max_frames } => jobs::step(&self.root, &directory, max_frames),
            Command::RenderJobState { directory, verify_outputs } => {
                jobs::state(&self.root, &directory, verify_outputs)
            }
            Command::CancelRenderJob { directory } => jobs::control(&self.root, &directory, true),
            Command::ResumeRenderJob { directory } => jobs::control(&self.root, &directory, false),
            Command::UvState { request } => textures::uv_state(&self.document, &request).map_err(Failure::invalid),
            Command::ExportTexture { request } => textures::export(&self.document, &self.root, &request),
            Command::ImportTexture { request, dry_run } => {
                let mut candidate = self.document.clone();
                let mut result = textures::import(&mut candidate, &self.root, &request)?;
                candidate.compile(&Pass::Beauty).map_err(Failure::invalid)?;
                if dry_run {
                    native::budget(&candidate, self.revision + 1)?;
                } else {
                    self.commit(candidate)?;
                }
                result["committed"] = json!(!dry_run);
                Ok(result)
            }
            Command::TextureState { request } => textures::inspect(&self.document, &request).map_err(Failure::invalid),
            Command::MaterialState { request } => {
                textures::material_state(&self.document, &request).map_err(Failure::invalid)
            }
            Command::ProjectBudget => serde_json::to_value(native::budget(&self.document, self.revision)?)
                .map_err(|error| Failure::invalid(error.to_string())),
            Command::ImportAudio { request, dry_run } => {
                let mut candidate = self.document.clone();
                let mut result = audio::import(&mut candidate, &self.root, &request)?;
                candidate.compile(&Pass::Beauty).map_err(Failure::invalid)?;
                result["committed"] = json!(!dry_run);
                if dry_run {
                    native::budget(&candidate, self.revision + 1)?;
                } else {
                    self.commit(candidate)?;
                }
                Ok(result)
            }
            Command::SpeechBackendState => Ok(
                json!({"configured":self.speech_backend.is_some(),"configuration_error":self.speech_backend_error,"backend":self.speech_backend.as_ref().map(|b|b.identity()),"semantics":"Speech analysis is local and requires a host-configured backend; JSON project data cannot choose executable code"}),
            ),
            Command::AnalyzeSpeech { request } => {
                let backend=self.speech_backend.as_ref().ok_or_else(||Failure{code:"speech_backend_unavailable",message:self.speech_backend_error.clone().unwrap_or_else(||"configure a local Rhubarb 1.14.0 backend with --speech-backend; existing baked lip curves do not need it".into())})?;
                let mut result = speech::analyze(&self.document, &self.root, backend, &request)?;
                result["source_revision"] = json!(self.revision);
                Ok(result)
            }
            Command::GenerateLipSync { request, dry_run } => {
                let mut candidate = self.document.clone();
                let mut result = lip_sync::generate(&mut candidate, &self.root, &request)?;
                candidate.compile(&Pass::Beauty).map_err(Failure::invalid)?;
                result["committed"] = json!(!dry_run);
                if dry_run {
                    native::budget(&candidate, self.revision + 1)?;
                } else {
                    self.commit(candidate)?;
                }
                Ok(result)
            }
            Command::LipSyncState { clip } => lip_sync::inspect(&self.document, &clip).map_err(Failure::invalid),
            Command::AudioState { request } => audio::inspect(&self.document, &request).map_err(Failure::invalid),
            Command::ExportAudio { request } => audio::export(&self.document, &self.root, &request),
            Command::ShotState { id } => dialogue::inspect(&self.document, &id).map_err(Failure::invalid),
            Command::JointLimitState { animation } => {
                joint_limits::inspect(&self.document, &animation).map_err(Failure::invalid)
            }
            Command::PoseShot { shot, frame } => dialogue::pose(&self.document, &shot, frame).map_err(Failure::invalid),
            Command::RenderShot { request } => dialogue::render(&self.document, &self.root, &request),
            Command::PreviewPatternPanel { panel } => pattern::preview(&panel).map_err(Failure::invalid),
            Command::SolveIk { request, dry_run } => {
                let mut candidate = self.document.clone();
                let mut result = ik::solve(&mut candidate, &request).map_err(Failure::invalid)?;
                result["committed"] = json!(!dry_run);
                if dry_run {
                    native::budget(&candidate, self.revision + 1)?;
                } else {
                    self.commit(candidate)?;
                }
                Ok(result)
            }
            Command::DeformerState { id, animation } => {
                deform::inspect(&self.document, &id, animation.as_ref()).map_err(Failure::invalid)
            }
            Command::ExportUsd { request } => delivery::export(&self.document, &self.root, &request),
            Command::ClothState { id, animation } => {
                cloth::inspect(&self.document, &id, animation.as_ref()).map_err(Failure::invalid)
            }
            Command::BakeCloth { request, dry_run } => {
                let mut candidate = self.document.clone();
                let mut result = cloth::bake(&mut candidate, &request).map_err(Failure::invalid)?;
                candidate.compile(&Pass::Beauty).map_err(Failure::invalid)?;
                result["committed"] = json!(!dry_run);
                if dry_run {
                    native::budget(&candidate, self.revision + 1)?;
                } else {
                    self.commit(candidate)?;
                }
                Ok(result)
            }
            Command::FaceState { animation } => {
                face::inspect_controls(&self.document, animation.as_ref()).map_err(Failure::invalid)
            }
            Command::GarmentFit { id, animation, samples_per_source } => {
                garment::inspect_fit(&self.document, &id, animation.as_ref(), samples_per_source)
                    .map_err(Failure::invalid)
            }
            Command::Pose { animation } => {
                animation::inspect_pose(&self.document, &animation).map_err(Failure::invalid)
            }
            Command::RenderSequence { request } => sequence::render(&self.document, &self.root, &request),
            Command::Describe => {
                let mut limits = json!({"objects": MAX_OBJECTS, "undo_snapshots": MAX_HISTORY, "total_volume_samples": MAX_VOLUME_SAMPLES,
                    "joints": animation::MAX_JOINTS, "clips": animation::MAX_CLIPS, "animation_keys": animation::MAX_KEYS,
                    "shots":timeline::MAX_SHOTS,"audio_assets":audio::MAX_AUDIO_ASSETS,"audio_source_bytes":audio::MAX_AUDIO_BYTES,
                    "texture_assets":textures::MAX_TEXTURE_ASSETS,"texture_source_bytes":textures::MAX_TEXTURE_BYTES,
                    "texture_pixels":mm3e_kit::texture::MAX_TEXTURE_PIXELS,"uv_sets":textures::MAX_UV_SETS,"uv_values":textures::MAX_TOTAL_UV_VALUES,
                    "audio_channels":audio::MAX_AUDIO_CHANNELS,"waveform_bins":audio::MAX_WAVEFORM_BINS,
                    "clip_layers": layering::MAX_LAYERS, "layer_depth": layering::MAX_LAYER_DEPTH, "layer_evaluations": layering::MAX_LAYER_EVALUATIONS, "render_job_step_frames": jobs::MAX_JOB_STEP_FRAMES, "sequence_frames": sequence::MAX_SEQUENCE_FRAMES, "sequence_pixels": sequence::MAX_SEQUENCE_PIXELS,
                    "cloth_assets":cloth::MAX_CLOTHS,"cloth_vertices_per_asset":cloth::MAX_CLOTH_VERTICES,
                    "cloth_frames":cloth::MAX_CLOTH_FRAMES,"cloth_cached_vertices":cloth::MAX_CLOTH_CACHED_VERTICES,
                    "sewn_panels_per_garment":sewing::MAX_SEWN_PANELS,"sewn_stitch_pairs":1024,
                    "pattern_triangulation_work":pattern::MAX_PATTERN_WORK,
                    "deformers":deform::MAX_DEFORMERS,"deformation_vertices":deform::MAX_TOTAL_VERTICES,
                    "morph_deltas":deform::MAX_TOTAL_MORPH_DELTAS,"deformation_work_per_pose":deform::MAX_TOTAL_WORK,
                    "usd_frames":delivery::MAX_EXPORT_FRAMES,"usd_grid_cells_per_axis":128,
                    "usd_boolean_channels":mm3e_kit::meshing_boolean::MAX_BOOLEAN_CHANNELS,
                    "usd_boolean_sample_values":mm3e_kit::meshing_boolean::MAX_BOOLEAN_SAMPLE_VALUES,
                    "render_pixels": 16_777_216, "request_bytes": 4 * 1024 * 1024, "document_bytes": storage::MAX_DOCUMENT_BYTES,
                    "maximum_native_revision_reserve_bytes":19});
                limits["constrained_ik_evaluations"] = json!(mm3e_kit::limited_ik::MAX_EVALUATIONS);
                limits["joint_limit_angular_tolerance_radians"] = json!(mm3e_kit::rotation_limit::ANGULAR_TOLERANCE);
                limits["speech_analysis_seconds"] = json!(speech::MAX_ANALYSIS_SECONDS);
                limits["speech_report_bytes"] = json!(speech::MAX_REPORT_BYTES);
                Ok(json!({
                    "protocol": "mm3e-editor-jsonl-v1", "request_schema": schemars::schema_for!(Request),
                    "document_schema": schemars::schema_for!(Document), "backend": "MM3E CPU SDF renderer",
                    "limits":limits,
                    "semantics": ["Mutations require expected_revision; apply batches are atomic; dry_run never commits",
                        "Request IDs correlate responses; they do not deduplicate requests",
                        "All accepted snapshots and dry runs fit canonical native project serialization, including up to 19 bytes reserved for future revision growth; project_budget reports exact usage",
                        "Units are meters; right-handed Y-up world; character front is +Z; rotations are degrees",
                        "Object order defines a global CSG fold; groups are selection metadata only",
                        "Clips animate rigid parts through world-rest pivots; pose/render/sample/pick do not mutate the document",
                        "Clip time is seconds; clamp holds endpoints, loop maps by Euclidean modulo; full turns need intermediate rotation keys",
                        "Shots use integer frame IDs and a rational frame rate with an explicit frame count; scheduled times are reported alongside the actual native f32 animation samples",
                        "Embedded reference WAVs preserve original bytes and SHA-256 identity; waveform observations and frame-to-audio windows use source sample frames without resampling",
                        "render_shot preserves whole-shot audio window phase for partial selections and writes exact sample-byte WAV sidecars before its completion manifest",
                        "Clips support ordered sparse override/additive layers with masks, source clocks, parent-time weight curves and optional additive reference time; local clip channels override the assembly",
                        "Explicit hinge or swing/twist rotation limits apply after layer composition and before parent FK; reject is default, project is opt-in and never rewrites authored keys; joint_limit_state exposes projection and quantization diagnostics",
                        "Constrained IK preserves translations/scales, verifies target and pole-selected middle plus optional orientation, and offers explicit best_feasible approximation with residuals; no global feasibility/nearest-pose guarantee",
                        "analyze_speech uses a host-configured local Rhubarb backend and preserves original audio plus bounded raw evidence; Linux process capture is supported, and dialogue is a recognition hint rather than forced alignment",
                        "generate_lip_sync converts verified cue reports through an explicit face/morph/joint pose profile into editable native clips, with pre-boundary transitions, sealed historical provenance, dry-run and edit detection",
                        "edit_layer changes one layer or its order atomically; inactive references are still validated, and contributing child motion participates in cloth cache freshness",
                        "create_render_job freezes a native snapshot and executable fingerprint; step_render_job renders at most 1..32 frames including recovered pending frames, and verifies prior encoded bytes before reuse",
                        "render_job_state exposes committed progress; cancel_render_job and resume_render_job control frame-boundary cancellation through a separate OS lock; pending status is not evidence of a live worker",
                        "edit_curve replaces or removes one facial or morph track while preserving unrelated clip data; it does not infer speech or audio alignment",
                        "Native surfaces support independent indexed corner UVs and embedded PNG albedo textures; source color space is explicit and sRGB is decoded before linear premultiplied filtering",
                        "Surface bindings also support independent roughness/metallic data channels, emissive color maps and tangent-space normal maps; data maps require linear interpretation and never premultiply channels by alpha",
                        "Normal maps perturb shading only; geometric normals still control depth, coverage, AO, ray offsets and opaque-surface visibility; frames derive from posed UV triangles and are not MikkTSpace vertex tangents",
                        "Normal-map variance_filter is opt-in (default false): minified slope variance broadens isotropic GGX roughness approximately; false preserves legacy filtering. material_state reports unfiltered_roughness and applied normal_variance; this is not exact GGX convolution",
                        "Texture UVs follow the nearest posed midsurface after object-domain mapping; material-owner provenance follows existing CSG rules, including base ownership on subtraction",
                        "Trilinear texture filtering uses an isotropic projected-ray-footprint approximation; alpha modulates opaque base color and does not change geometry or coverage",
                        "Checked CPU rendering rejects appearance evaluation failures before writing images; textured GPU and legacy text are unsupported",
                        "Textured export_usd requires explicit texture_delivery with reader_defined filtering: composed geometry is retained, corner UV transfer is bounded, original PNGs and compatible color/scalar maps are packaged beside the layer; active tangent normals, mixed roughness-floor values and unresolved seams reject",
                        "Sample returns the unpruned authored scalar, which need not be Euclidean distance",
                        "All saved native geometry, including baked volumes, is embedded; undo history is session-local",
                        "No implicit camera framing or material/geometry substitutions occur",
                        "--project FILE.json holds an OS writer lock and persists accepted mutations before acknowledging them; transient Save uses the same locks",
                        "A failed installation is not proof of rollback; commit_uncertain on the active project blocks further commands until restart, while output uncertainty errors require destination inspection before retry",
                        "Fitted garments follow evaluated body fields; facial lids/lips use local geometric controls; neither implies fabric/tissue simulation",
                        "Loose cloth is separately authored and baked; sampled poses enforce configured feature-clearance and strain tolerances without a global collision certificate",
                        "Sewn garments retain rectangular panel recipes and explicit ordered boundary correspondences; sewing constrains separate vertices and preserves rest geometry",
                        "Pattern panels support concave planar outlines and holes; preview returns final boundary indices and named control mappings for explicit seams and pins",
                        "solve_ik authors verified rotation keys for a direct three-joint chain at an explicit time; targets and poles are world points; unreachable targets reject unless clamp is requested",
                        "IK preserves sampled local translation and scale; it is key authoring, not a persistent target constraint between keys",
                        "Native surfaces support linear-blend and rigid dual-quaternion skinning; rest-local morph deltas apply before rest-world joint deltas",
                        "Morph defaults can be patched by name without retransmitting deltas; clip morph tracks override the corresponding defaults",
                        "Scoped object samples and USD selection evaluate only selected cloth caches; full renders still require every participating cloth pose",
                        "USD exports selected composed fields as sampled polygon caches; finite convergence checks do not certify unresolved features"],
                    "not_implemented": ["visual editor UI", "anatomical tissue simulation", "anatomy and aesthetic evaluator",
                        "anatomical IK joint limits and persistent IK constraints", "clip blending and retargeting",
                        "general constraint solver", "production retopology", "texture painting and displacement", "anisotropic normal-distribution filtering", "MCP transport",
                        "warp safety certification", "measured fabric constitutive models and universal collision certification", "automatic audio-to-mouth alignment"],
                    "unsupported_authoring_modifiers": ["twist", "bend", "infinite repeat"]
                }))
            }
            Command::Inspect { id } => {
                if let Some(id) = id {
                    let object = self.document.objects.iter().find(|e| e.id == id).ok_or_else(|| missing(&id))?;
                    Ok(observe::entity(object))
                } else {
                    Ok(
                        json!({"document": observe::inspection(&self.document), "undo_available": self.undo.len(), "redo_available": self.redo.len(),
                            "persistence":{"project":self.project.as_ref().map(|store|store.path()),"durable":self.project.is_some(),"undo_history_persisted":false}}),
                    )
                }
            }
            Command::GetDocument => serde_json::to_value(&self.document).map_err(|e| Failure::invalid(e.to_string())),
            Command::Validate => {
                self.document.compile(&Pass::Beauty).map_err(Failure::invalid)?;
                let native_budget = native::budget(&self.document, self.revision)?;
                Ok(
                    json!({"valid": true,"native_project":native_budget, "checks": ["schema version", "unique ids", "finite bounded parameters", "camera frame",
                    "primitive domains", "volume layout", "renderer lowering", "acyclic joints and unique bindings", "clip targets and key domains",
                    "deformation palettes and weights", "morph channels and default posed triangles", "serialized native project budget and revision headroom"],
                    "not_evaluated": ["anatomy", "aesthetics", "global surface intersections", "all animated deformation samples", "all-view visibility"]}),
                )
            }
            Command::Apply { operations, dry_run } => {
                if operations.is_empty() || operations.len() > 512 {
                    return Err(Failure::invalid("apply needs 1..512 operations"));
                }
                let mut candidate = self.document.clone();
                for (i, op) in operations.into_iter().enumerate() {
                    apply(&mut candidate, op)
                        .map_err(|e| Failure { code: e.code, message: format!("operation {i}: {}", e.message) })?;
                    if candidate.objects.len() > MAX_OBJECTS {
                        return Err(Failure::invalid("batch exceeds object budget"));
                    }
                }
                face::synchronize(&mut candidate).map_err(Failure::invalid)?;
                garment::synchronize(&mut candidate).map_err(Failure::invalid)?;
                candidate.compile(&Pass::Beauty).map_err(Failure::invalid)?;
                if dry_run {
                    native::budget(&candidate, self.revision + 1)?;
                }
                let result = json!({"committed": !dry_run, "object_count": candidate.objects.len(),
                    "candidate": if dry_run { Some(observe::inspection(&candidate)) } else { None }});
                if !dry_run {
                    self.commit(candidate)?;
                }
                Ok(result)
            }
            Command::Undo => {
                let previous = self.undo.back().cloned().ok_or_else(|| Failure::invalid("nothing to undo"))?;
                self.persist(&previous, self.revision + 1)?;
                self.undo.pop_back();
                self.redo.push(std::mem::replace(&mut self.document, previous));
                self.release_texture_history();
                self.revision += 1;
                Ok(json!({"undone": true}))
            }
            Command::Redo => {
                let next = self.redo.last().cloned().ok_or_else(|| Failure::invalid("nothing to redo"))?;
                self.persist(&next, self.revision + 1)?;
                self.redo.pop();
                self.undo.push_back(std::mem::replace(&mut self.document, next));
                self.release_texture_history();
                self.revision += 1;
                Ok(json!({"redone": true}))
            }
            Command::Sample { points, id, animation } => {
                if points.is_empty() || points.len() > 4096 {
                    return Err(Failure::invalid("sample needs 1..4096 points"));
                }
                let object_index = id
                    .as_ref()
                    .map(|id| self.document.objects.iter().position(|e| &e.id == id).ok_or_else(|| missing(id)))
                    .transpose()?;
                let selected = id.as_ref().map(|id| BTreeSet::from([id.clone()]));
                let (scene, _) = match selected.as_ref() {
                    Some(ids) => self.document.compile_at_selected(&Pass::Beauty, animation.as_ref(), ids),
                    None => self.document.compile_at(&Pass::Beauty, animation.as_ref()),
                }
                .map_err(Failure::invalid)?;
                let mut samples = vec![];
                for point in points {
                    vector(point, "sample point").map_err(Failure::invalid)?;
                    let f = match object_index {
                        Some(index) => {
                            scene.sample_object(index, vec(point)).expect("id resolved in this scene snapshot")
                        }
                        None => scene.sample_authored(vec(point)),
                    };
                    if !scene.objects.is_empty() && !f.dist.is_finite() {
                        return Err(Failure::invalid("authored field produced a non-finite value"));
                    }
                    samples.push(json!({"point": point, "value": if f.dist.is_finite() { Some(f.dist) } else { None },
                        "material_owner_id": if scene.objects.is_empty() { None } else { observe::owner(&self.document, f.mat) }}));
                }
                Ok(
                    json!({"semantics": "unpruned authored scalar field; not guaranteed Euclidean distance; null means empty scene",
                        "object_id": id, "animation": animation, "scope": if object_index.is_some() { "object_before_scene_csg" } else { "combined_scene" }, "samples": samples}),
                )
            }
            Command::Pick { x, y, view, animation } => {
                observe::pick(&self.document, x, y, view.as_ref(), animation.as_ref())
            }
            Command::Render { path, pass, view, overwrite, animation } => {
                observe::image(&self.document, &self.root, &path, &pass, view.as_ref(), overwrite, animation.as_ref())
            }
            Command::Save { path, overwrite } => {
                let bytes = saved_bytes(&self.document, self.revision)?;
                let target = storage::path(&self.root, &path, false)?;
                let target = if target.exists() {
                    target.canonicalize().map_err(|e| Failure::io(e.to_string()))?
                } else {
                    target
                };
                if !overwrite && target.exists() {
                    return Err(Failure::invalid("output exists; set overwrite to true to replace it"));
                }
                if let Some(store) = self.project.as_mut().filter(|store| store.path() == target) {
                    store.write(&bytes)?;
                } else {
                    let (mut store, existing) = project::ProjectStore::open(&self.root, &path)?;
                    if !overwrite && existing.is_some() {
                        return Err(Failure::invalid("output exists; set overwrite to true to replace it"));
                    }
                    store.write(&bytes)?;
                }
                let path = target;
                Ok(
                    json!({"path": path, "saved_revision": self.revision, "bytes": bytes.len(), "undo_history_saved": false}),
                )
            }
            Command::Load { path } => {
                let mut saved: Saved = serde_json::from_slice(&storage::read(&self.root, &path)?)
                    .map_err(|e| Failure::invalid(e.to_string()))?;
                if saved.format != "mm3e-agent-project-v1" {
                    return Err(Failure::invalid("unsupported project format"));
                }
                let migrated_faces = face::migrate_legacy_native(&mut saved.document).map_err(Failure::invalid)?;
                saved.document.compile(&Pass::Beauty).map_err(Failure::invalid)?;
                self.commit(saved.document)?;
                Ok(
                    json!({"loaded": true, "source_saved_revision": saved.saved_revision, "object_count": self.document.objects.len(), "migrated_faces": migrated_faces}),
                )
            }
            Command::ImportObj { id, path, resolution, padding } => {
                identifier(&id).map_err(Failure::invalid)?;
                if !(4..=96).contains(&resolution) {
                    return Err(Failure::invalid("OBJ bake resolution must be 4..96"));
                }
                range(padding, 0.01, 10.0, "padding").map_err(Failure::invalid)?;
                if self.document.objects.iter().any(|e| e.id == id) {
                    return Err(Failure::invalid("object id already exists"));
                }
                let bytes = storage::read(&self.root, &path)?;
                let src = std::str::from_utf8(&bytes).map_err(|e| Failure::invalid(e.to_string()))?;
                let mesh = mm3e_orchestrator::mesh::parse_obj(src).map_err(Failure::invalid)?;
                if mesh.triangles.len() > 200_000 {
                    return Err(Failure::invalid("OBJ exceeds 200000 triangles"));
                }
                for position in &mesh.positions {
                    vector(array(*position), "OBJ vertex").map_err(Failure::invalid)?;
                }
                let volume = mm3e_orchestrator::mesh::bake_sdf(&mesh, resolution, padding).map_err(Failure::invalid)?;
                let shape = Shape::Volume {
                    dims: [volume.dims.0, volume.dims.1, volume.dims.2],
                    min: array(volume.min),
                    cell: array(volume.cell),
                    samples: volume.data,
                };
                let mut candidate = self.document.clone();
                candidate.objects.push(Entity {
                    id: id.clone(),
                    label: id,
                    role: "imported_geometry".into(),
                    group: String::new(),
                    shape,
                    position: [0.0; 3],
                    rotation_degrees: [0.0; 3],
                    scale: 1.0,
                    material: Surface::default(),
                    combine: Combination::Union,
                    modifiers: Modifiers::default(),
                });
                candidate.compile(&Pass::Beauty).map_err(Failure::invalid)?;
                self.commit(candidate)?;
                Ok(json!({"imported": true, "triangles": mesh.triangles.len(), "resolution": resolution,
                    "representation": "baked SDF volume; original OBJ topology, UVs and material assignments are not retained"}))
            }
        }
    }

    fn persist(&mut self, next: &Document, revision: u64) -> Result<(), Failure> {
        if let Some(store) = &mut self.project {
            store.write(&saved_bytes(next, revision)?)?;
        } else {
            native::budget(next, revision)?;
        }
        Ok(())
    }

    fn commit(&mut self, next: Document) -> Result<(), Failure> {
        self.persist(&next, self.revision + 1)?;
        self.undo.push_back(std::mem::replace(&mut self.document, next));
        if self.undo.len() > MAX_HISTORY {
            self.undo.pop_front();
        }
        self.redo.clear();
        self.release_texture_history();
        self.revision += 1;
        Ok(())
    }
    fn release_texture_history(&self) {
        textures::release_history_caches(&self.document, self.undo.iter().chain(self.redo.iter()));
    }
}

fn saved_bytes(document: &Document, revision: u64) -> Result<Vec<u8>, Failure> {
    native::bytes(document, revision)
}

fn missing(id: &str) -> Failure {
    Failure { code: "not_found", message: format!("no object with id {id}") }
}

fn apply(document: &mut Document, operation: Operation) -> Result<(), Failure> {
    match operation {
        Operation::CreatePatternCloth { request } => pattern::create(document, &request).map_err(Failure::invalid)?,
        Operation::UpdatePatternCloth { request } => pattern::update(document, &request).map_err(Failure::invalid)?,
        Operation::BindSurface { request } => deform::bind(document, request).map_err(Failure::invalid)?,
        Operation::UpdateDeformer { request } => deform::update(document, request).map_err(Failure::invalid)?,
        Operation::RemoveDeformer { id } => deform::remove(document, &id).map_err(Failure::invalid)?,
        Operation::SetMorphWeights { id, weights } => {
            deform::set_morph_weights(document, &id, weights).map_err(Failure::invalid)?
        }
        Operation::CreateSewnCloth { request } => sewing::create(document, &request).map_err(Failure::invalid)?,
        Operation::UpdateSewnCloth { request } => sewing::update(document, &request).map_err(Failure::invalid)?,
        Operation::CreateClothPanel { request } => cloth::create_panel(document, &request).map_err(Failure::invalid)?,
        Operation::UpdateClothPanel { request } => cloth::update_panel(document, &request).map_err(Failure::invalid)?,
        Operation::RemoveCloth { id } => cloth::remove(document, &id).map_err(Failure::invalid)?,
        Operation::CreateFace { request } => face::create(document, &request).map_err(Failure::invalid)?,
        Operation::SetFaceControls { id, controls } => {
            controls.validate().map_err(Failure::invalid)?;
            document
                .faces
                .iter_mut()
                .find(|f| f.id == id)
                .ok_or_else(|| Failure::invalid(format!("missing face {id}")))?
                .controls = controls;
        }
        Operation::RemoveFace { id } => {
            let index = document
                .faces
                .iter()
                .position(|f| f.id == id)
                .ok_or_else(|| Failure::invalid(format!("missing face {id}")))?;
            let removed = document.faces.remove(index);
            document.objects.retain(|e| !removed.generated_object_ids.contains(&e.id));
        }
        Operation::CreateGarment { request } => garment::create(document, &request).map_err(Failure::invalid)?,
        Operation::UpdateGarment { request } => garment::update(document, &request).map_err(Failure::invalid)?,
        Operation::RemoveGarment { id } => {
            let index = document
                .garments
                .iter()
                .position(|g| g.id == id)
                .ok_or_else(|| Failure::invalid(format!("missing garment {id}")))?;
            document.garments.remove(index);
            document.objects.retain(|e| e.id != id);
        }
        Operation::SetJoints { joints } => document.joints = joints,
        Operation::RigHumanoid { id } => {
            let joints = animation::rig_humanoid(document, &id).map_err(Failure::invalid)?;
            document.joints.extend(joints);
        }
        Operation::EditLayer { request } => layering::edit(document, &request).map_err(Failure::invalid)?,
        Operation::SetJointLimit { id, limit } => {
            let joint = document.joints.iter_mut().find(|j| j.id == id).ok_or_else(|| missing(&id))?;
            limit.validate().map_err(Failure::invalid)?;
            joint.rotation_limit = Some(limit);
        }
        Operation::ClearJointLimit { id } => {
            let joint = document.joints.iter_mut().find(|j| j.id == id).ok_or_else(|| missing(&id))?;
            joint.rotation_limit = None;
        }
        Operation::PutClip { clip } => {
            if let Some(existing) = document.clips.iter_mut().find(|c| c.id == clip.id) {
                *existing = clip;
            } else {
                document.clips.push(clip);
            }
        }
        Operation::EditCurve { request } => curve_edit::apply(document, &request).map_err(Failure::invalid)?,
        Operation::PutUvs { request } => textures::put_uvs(document, request).map_err(Failure::invalid)?,
        Operation::ProjectUvs { request } => textures::project_uvs(document, request).map_err(Failure::invalid)?,
        Operation::BindTexture { binding } => textures::bind(document, binding),
        Operation::RemoveTexture { id } => textures::remove_texture(document, &id).map_err(Failure::invalid)?,
        Operation::RemoveUvs { id } => textures::remove_uvs(document, &id).map_err(Failure::invalid)?,
        Operation::UnbindTexture { object } => textures::unbind(document, &object).map_err(Failure::invalid)?,
        Operation::PutShot { shot } => dialogue::put(document, shot).map_err(Failure::invalid)?,
        Operation::DeleteShot { id } => dialogue::remove(document, &id).map_err(Failure::invalid)?,
        Operation::RemoveAudio { id } => audio::remove(document, &id).map_err(Failure::invalid)?,
        Operation::DeleteClip { id } => {
            let index = document
                .clips
                .iter()
                .position(|c| c.id == id)
                .ok_or_else(|| Failure::invalid(format!("missing clip {id}")))?;
            document.clips.remove(index);
        }
        Operation::Create { object } => {
            if document.objects.iter().any(|e| e.id == object.id) {
                return Err(Failure::invalid("object id already exists"));
            }
            document.objects.push(object);
        }
        Operation::Update { id, patch } => {
            if document.cloths.iter().any(|c| c.id == id)
                && (patch.shape.is_some()
                    || patch.position.is_some()
                    || patch.rotation_degrees.is_some()
                    || patch.scale.is_some()
                    || patch.modifiers.is_some()
                    || patch.combine.is_some()
                    || patch.role.is_some()
                    || patch.group.is_some()
                    || patch.label.is_some())
            {
                return Err(Failure::invalid(
                    "cloth mesh follows its rest topology and bake; edit cloth parameters through cloth operations",
                ));
            }
            if document.faces.iter().any(|f| f.generated_object_ids.contains(&id))
                && (patch.shape.is_some()
                    || patch.position.is_some()
                    || patch.rotation_degrees.is_some()
                    || patch.scale.is_some()
                    || patch.modifiers.is_some()
                    || patch.combine.is_some()
                    || patch.role.is_some()
                    || patch.group.is_some()
                    || patch.label.is_some())
            {
                return Err(Failure::invalid("facial geometry follows its sources and controls; use set_face_controls or edit the source head/eyes"));
            }
            if document.garments.iter().any(|g| g.id == id)
                && (patch.shape.is_some()
                    || patch.position.is_some()
                    || patch.rotation_degrees.is_some()
                    || patch.scale.is_some()
                    || patch.modifiers.is_some()
                    || patch.combine.is_some()
                    || patch.role.is_some()
                    || patch.group.is_some())
            {
                return Err(Failure::invalid(
                    "garment geometry follows its sources; edit its source geometry or recreate the garment parameters",
                ));
            }
            let e = document.objects.iter_mut().find(|e| e.id == id).ok_or_else(|| missing(&id))?;
            if let Some(v) = patch.label {
                e.label = v;
            }
            if let Some(v) = patch.role {
                e.role = v;
            }
            if let Some(v) = patch.group {
                e.group = v;
            }
            if let Some(v) = patch.shape {
                e.shape = v;
            }
            if let Some(v) = patch.position {
                e.position = v;
            }
            if let Some(v) = patch.rotation_degrees {
                e.rotation_degrees = v;
            }
            if let Some(v) = patch.scale {
                e.scale = v;
            }
            if let Some(v) = patch.material {
                e.material = v;
            }
            if let Some(v) = patch.combine {
                e.combine = v;
            }
            if let Some(v) = patch.modifiers {
                e.modifiers = v;
            }
        }
        Operation::Delete { id } => {
            if deform::owns_object(document, &id) {
                return Err(Failure::invalid("remove the deformer before deleting its source surface"));
            }
            if document.cloths.iter().any(|c| c.id == id) {
                return Err(Failure::invalid("use remove_cloth to delete mesh and simulation metadata"));
            }
            if document.faces.iter().any(|f| f.generated_object_ids.contains(&id)) {
                return Err(Failure::invalid("use remove_face to delete facial generated parts and metadata"));
            }
            if document.garments.iter().any(|g| g.id == id) {
                return Err(Failure::invalid("use remove_garment to delete generated garment and its metadata"));
            }
            let index = document.objects.iter().position(|e| e.id == id).ok_or_else(|| missing(&id))?;
            document.objects.remove(index);
        }
        Operation::Translate { ids, delta } => {
            if ids.iter().any(|id| document.cloths.iter().any(|c| &c.id == id)) {
                return Err(Failure::invalid("cloth motion uses pins and simulation; update its panel or attachment"));
            }
            if ids.iter().any(|id| document.faces.iter().any(|f| f.generated_object_ids.contains(id))) {
                return Err(Failure::invalid("facial parts follow the source head and eyes; edit those transforms"));
            }
            if ids.iter().any(|id| document.garments.iter().any(|g| &g.id == id)) {
                return Err(Failure::invalid("garments follow source transforms; translate their sources"));
            }
            vector(delta, "translation delta").map_err(Failure::invalid)?;
            let unique: BTreeSet<&String> = ids.iter().collect();
            if ids.is_empty() || unique.len() != ids.len() {
                return Err(Failure::invalid("translate requires distinct, nonempty ids"));
            }
            for id in ids {
                let e = document.objects.iter_mut().find(|e| e.id == id).ok_or_else(|| missing(&id))?;
                e.position = std::array::from_fn(|i| e.position[i] + delta[i]);
            }
        }
        Operation::SetCamera { camera } => document.camera = camera,
        Operation::SetSettings { settings } => document.settings = settings,
        Operation::SetLights { lights } => document.lights = lights,
        Operation::CreateHumanoid { id, height, build, head_scale, origin } => {
            document
                .objects
                .extend(character::humanoid(&id, height, build, head_scale, origin).map_err(Failure::invalid)?);
        }
    }
    Ok(())
}
