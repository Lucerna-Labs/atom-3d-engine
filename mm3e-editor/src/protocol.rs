use crate::animation::{AnimationSample, Clip, Joint};
use crate::model::*;
use crate::sequence::SequenceRequest;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Patch {
    pub label: Option<String>,
    pub role: Option<String>,
    pub group: Option<String>,
    pub shape: Option<Shape>,
    pub position: Option<V3>,
    pub rotation_degrees: Option<V3>,
    pub scale: Option<f32>,
    pub material: Option<Surface>,
    pub combine: Option<Combination>,
    pub modifiers: Option<Modifiers>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Operation {
    ProjectUvs {
        request: crate::textures::ProjectUvs,
    },
    PutUvs {
        request: crate::textures::PutUvs,
    },
    BindTexture {
        binding: crate::textures::TextureBinding,
    },
    RemoveTexture {
        id: String,
    },
    RemoveUvs {
        id: String,
    },
    UnbindTexture {
        object: String,
    },
    PutShot {
        shot: crate::timeline::TimelineShot,
    },
    DeleteShot {
        id: String,
    },
    RemoveAudio {
        id: String,
    },
    EditCurve {
        request: crate::curve_edit::CurveEdit,
    },
    CreatePatternCloth {
        request: crate::pattern::PatternClothRequest,
    },
    UpdatePatternCloth {
        request: crate::pattern::PatternClothRequest,
    },
    BindSurface {
        request: crate::deform::BindSurface,
    },
    UpdateDeformer {
        request: crate::deform::UpdateDeformer,
    },
    RemoveDeformer {
        id: String,
    },
    SetMorphWeights {
        id: String,
        weights: Vec<crate::deform::MorphWeight>,
    },
    CreateSewnCloth {
        request: crate::sewing::SewnClothRequest,
    },
    UpdateSewnCloth {
        request: crate::sewing::SewnClothRequest,
    },
    CreateClothPanel {
        request: crate::cloth::ClothPanelRequest,
    },
    UpdateClothPanel {
        request: crate::cloth::ClothPanelRequest,
    },
    RemoveCloth {
        id: String,
    },
    CreateFace {
        request: crate::face::FaceRequest,
    },
    SetFaceControls {
        id: String,
        controls: crate::face::FaceControls,
    },
    RemoveFace {
        id: String,
    },
    CreateGarment {
        request: crate::garment::GarmentRequest,
    },
    UpdateGarment {
        request: crate::garment::GarmentRequest,
    },
    RemoveGarment {
        id: String,
    },
    /// Replace the joint hierarchy/bindings as part of the atomic transaction.
    SetJoints {
        joints: Vec<Joint>,
    },
    /// Bind the existing 19-part humanoid in its current rest pose.
    RigHumanoid {
        id: String,
    },
    /// Create or replace a named clip, including all its keys.
    EditLayer {
        request: crate::layering::EditLayer,
    },
    SetJointLimit {
        id: String,
        limit: crate::joint_limits::JointRotationLimit,
    },
    ClearJointLimit {
        id: String,
    },
    PutClip {
        clip: Clip,
    },
    DeleteClip {
        id: String,
    },
    Create {
        object: Entity,
    },
    Update {
        id: String,
        patch: Patch,
    },
    Delete {
        id: String,
    },
    Translate {
        ids: Vec<String>,
        delta: V3,
    },
    SetCamera {
        camera: View,
    },
    SetSettings {
        settings: Settings,
    },
    SetLights {
        lights: Vec<Illumination>,
    },
    /// Creates 19 individually editable parts. Group metadata does not create a rig.
    CreateHumanoid {
        id: String,
        height: f32,
        build: f32,
        head_scale: f32,
        #[serde(default)]
        origin: V3,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Command {
    UvState {
        request: crate::textures::UvState,
    },
    ExportTexture {
        request: crate::textures::ExportTexture,
    },
    ImportTexture {
        request: crate::textures::ImportTexture,
        #[serde(default)]
        dry_run: bool,
    },
    TextureState {
        request: crate::textures::TextureState,
    },
    MaterialState {
        request: crate::textures::MaterialState,
    },
    /// Exact canonical saved-project size with headroom for future revision digits.
    ProjectBudget,
    ImportAudio {
        request: crate::audio::ImportAudio,
        #[serde(default)]
        dry_run: bool,
    },
    SpeechBackendState,
    AnalyzeSpeech {
        request: crate::speech::AnalyzeSpeech,
    },
    GenerateLipSync {
        request: crate::lip_sync::GenerateLipSync,
        #[serde(default)]
        dry_run: bool,
    },
    LipSyncState {
        clip: String,
    },
    AudioState {
        request: crate::audio::AudioState,
    },
    ExportAudio {
        request: crate::audio::ExportAudio,
    },
    ShotState {
        id: String,
    },
    JointLimitState {
        animation: AnimationSample,
    },
    PoseShot {
        shot: String,
        frame: i64,
    },
    RenderShot {
        request: crate::dialogue::RenderShot,
    },
    CreateRenderJob {
        request: crate::jobs::RenderJobRequest,
    },
    StepRenderJob {
        directory: String,
        #[serde(default = "one_job_frame")]
        max_frames: u32,
    },
    RenderJobState {
        directory: String,
        #[serde(default)]
        verify_outputs: bool,
    },
    CancelRenderJob {
        directory: String,
    },
    ResumeRenderJob {
        directory: String,
    },
    PreviewPatternPanel {
        panel: crate::pattern::PatternPanel,
    },
    SolveIk {
        request: crate::ik::IkRequest,
        #[serde(default)]
        dry_run: bool,
    },
    DeformerState {
        id: String,
        #[serde(default)]
        animation: Option<AnimationSample>,
    },
    /// Deliver the evaluated selected field as an independently readable USD mesh cache.
    ExportUsd {
        request: crate::delivery::ExportUsdRequest,
    },
    BakeCloth {
        request: crate::cloth::BakeClothRequest,
        #[serde(default)]
        dry_run: bool,
    },
    ClothState {
        id: String,
        #[serde(default)]
        animation: Option<AnimationSample>,
    },
    FaceState {
        #[serde(default)]
        animation: Option<AnimationSample>,
    },
    GarmentFit {
        id: String,
        #[serde(default)]
        animation: Option<AnimationSample>,
        #[serde(default = "default_fit_samples")]
        samples_per_source: usize,
    },
    /// Inspect evaluated world transforms and joint pivots without changing authored state.
    Pose {
        animation: AnimationSample,
    },
    /// Export a bounded inclusive range of real rendered PNG frames and its manifest.
    RenderSequence {
        request: SequenceRequest,
    },
    Describe,
    Inspect {
        #[serde(default)]
        id: Option<String>,
    },
    GetDocument,
    Validate,
    /// All changes commit together, or none do. Requires expected_revision, including dry runs.
    Apply {
        operations: Vec<Operation>,
        #[serde(default)]
        dry_run: bool,
    },
    Undo,
    Redo,
    Sample {
        points: Vec<V3>,
        /// Optional stable object ID: sample this part before the scene-wide CSG fold.
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        animation: Option<AnimationSample>,
    },
    /// Pixel coordinates in the configured image; returned identity is the material owner.
    Pick {
        x: f32,
        y: f32,
        #[serde(default)]
        view: Option<View>,
        #[serde(default)]
        animation: Option<AnimationSample>,
    },
    Render {
        path: String,
        #[serde(default)]
        pass: Pass,
        #[serde(default)]
        view: Option<View>,
        #[serde(default)]
        overwrite: bool,
        #[serde(default)]
        animation: Option<AnimationSample>,
    },
    Save {
        path: String,
        #[serde(default)]
        overwrite: bool,
    },
    Load {
        path: String,
    },
    /// Bakes OBJ geometry into the engine's SDF volume; original mesh topology is not retained.
    ImportObj {
        id: String,
        path: String,
        resolution: usize,
        padding: f32,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Request {
    /// Correlation only. Retry a mutation by inspecting revision first; IDs are not deduplication keys.
    pub id: String,
    /// Required on apply/undo/redo/load/import_obj. Revisions never move backward within a session.
    #[serde(default)]
    pub expected_revision: Option<u64>,
    pub command: Command,
}

fn default_fit_samples() -> usize {
    64
}

#[derive(Clone, Debug, Serialize)]
pub struct Failure {
    pub code: &'static str,
    pub message: String,
}
impl Failure {
    pub fn invalid(message: impl Into<String>) -> Self {
        Self { code: "invalid", message: message.into() }
    }
    pub fn io(message: impl Into<String>) -> Self {
        Self { code: "io", message: message.into() }
    }
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub ok: bool,
    pub revision: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Failure>,
}

fn one_job_frame() -> u32 {
    1
}
