//! Native loose-cloth authoring and deterministic simulation caches.
//!
//! Collision preserves the authored CSG fold of explicitly selected fields, sampled at vertices
//! at every fixed physical substep. Optional bounded cloth self-contact uses
//! swept vertex-triangle and edge-edge features. Cached poses are checked again
//! for static feature clearance after interpolation and exact pin evaluation.
//! These checks do not certify continuous or globally intersection-free motion.
use crate::{
    animation::{AnimationSample, Playback, Target},
    model::{
        array, identifier, range, vec, vector, Combination, Document, Entity, Modifiers, Pass, Shape, Surface, V3,
    },
};
use mm3e_kit::{
    cloth::{
        measure_cloth_self_contact_with_seams, Cloth, ClothContact, ClothSeam, ClothSelfContactReport, ClothSettings,
        ClothStepReport,
    },
    surface::TriangleSurface,
    vec::Transform,
    Vec3,
};
use mm3e_orchestrator::{Prim, Scene};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub const MAX_CLOTHS: usize = 8;
pub const MAX_CLOTH_VERTICES: usize = 256;
pub const MAX_CLOTH_FRAMES: usize = 601;
pub const MAX_CLOTH_CACHED_VERTICES: usize = 320_000;
const MAX_CLOTH_BAKE_WORK: u64 = 100_000_000;
// A validated attachment evaluates nine per-weight checks, two sum operations,
// one residual subtraction/absolute value/comparison, three divisions, and six
// affine arithmetic operations per coordinate. Charge this complete bound.
const BARYCENTRIC_ATTACHMENT_WORK: u64 = 35;
const BARYCENTRIC_ATTACHMENT_ALGORITHM: &str = "normalized-affine-f64-v1";
const ALGORITHM: &str = "xpbd-dihedral-self-contact-v3";
const SEWN_ALGORITHM: &str = "xpbd-dihedral-sewn-self-contact-v4";
const PATTERN_ALGORITHM: &str = "xpbd-dihedral-outline-pattern-v5";
const PREVIOUS_ALGORITHMS: &[&str] = &["xpbd-dihedral-vertex-contact-v1", "xpbd-dihedral-vertex-contact-v2"];

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct ClothSolverSettings {
    pub fixed_dt: f32,
    pub substeps: u32,
    pub iterations: u32,
    pub gravity: V3,
    pub damping_per_second: f32,
    pub stretch_compliance: f32,
    pub bend_compliance: f32,
    pub collision_thickness: f32,
    pub contact_iterations: u32,
    pub self_collision: bool,
    pub self_collision_thickness: f32,
    pub friction_coefficient: f32,
    pub self_collision_max_candidates: u32,
    pub self_collision_max_work: u64,
    pub max_penetration_m: f32,
    pub max_relative_edge_error: f32,
    /// Absolute seam length error, including the authored initial assembly pose.
    #[serde(default = "default_seam_error", skip_serializing_if = "is_default_seam_error")]
    pub max_seam_error_m: f32,
}
fn default_seam_error() -> f32 {
    0.002
}
fn is_default_seam_error(value: &f32) -> bool {
    *value == default_seam_error()
}
impl Default for ClothSolverSettings {
    fn default() -> Self {
        let core = ClothSettings::default();
        Self {
            fixed_dt: core.fixed_dt,
            substeps: core.substeps,
            iterations: core.iterations,
            gravity: array(core.gravity),
            damping_per_second: core.damping_per_second,
            stretch_compliance: core.stretch_compliance,
            bend_compliance: core.bend_compliance,
            collision_thickness: core.collision_thickness,
            contact_iterations: core.contact_iterations,
            self_collision: core.self_collision,
            self_collision_thickness: core.self_collision_thickness,
            friction_coefficient: core.friction_coefficient,
            self_collision_max_candidates: core.self_collision_max_candidates,
            self_collision_max_work: core.self_collision_max_work,
            max_penetration_m: 0.002,
            max_relative_edge_error: 0.25,
            max_seam_error_m: default_seam_error(),
        }
    }
}
impl ClothSolverSettings {
    fn core(&self) -> Result<ClothSettings, String> {
        range(self.fixed_dt, 1.0 / 240.0, 1.0 / 24.0, "cloth fixed_dt")?;
        if self.substeps == 0 || self.substeps > 16 || self.iterations == 0 || self.iterations > 64 {
            return Err("cloth needs 1..16 substeps and 1..64 iterations".into());
        }
        vector(self.gravity, "cloth gravity")?;
        range(self.max_penetration_m, 0.0, 0.1, "cloth maximum penetration")?;
        range(self.max_relative_edge_error, 0.001, 1.0, "cloth maximum relative edge error")?;
        range(self.max_seam_error_m, 0.0, 1.0, "cloth maximum seam error")?;
        let settings = ClothSettings {
            fixed_dt: self.fixed_dt / self.substeps as f32,
            substeps: 1,
            iterations: self.iterations,
            gravity: vec(self.gravity),
            damping_per_second: self.damping_per_second,
            stretch_compliance: self.stretch_compliance,
            bend_compliance: self.bend_compliance,
            collision_thickness: self.collision_thickness,
            contact_iterations: self.contact_iterations,
            self_collision: self.self_collision,
            self_collision_thickness: self.self_collision_thickness,
            friction_coefficient: self.friction_coefficient,
            self_collision_max_candidates: self.self_collision_max_candidates,
            self_collision_max_work: self.self_collision_max_work,
        };
        settings.validate().map_err(|e| e.to_string())?;
        Ok(settings)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ClothPin {
    pub vertex: u32,
    /// None uses a fixed world-space point; Some uses a named object's local point
    /// unless a triangle/barycentric attachment is supplied.
    #[serde(default)]
    pub target_object: Option<String>,
    pub point: V3,
    /// Optional source triangle on a continuously deformed target surface.
    /// When present, `barycentric` must also be present and the pair is used
    /// instead of the object-local `point` attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_triangle: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barycentric: Option<[f32; 3]>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ClothPanelRequest {
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub group: String,
    /// Center of the rest panel in world meters.
    pub origin: V3,
    /// Unit perpendicular world directions; vertex order is v rows then u columns.
    pub axis_u: V3,
    pub axis_v: V3,
    pub segments: [u32; 2],
    pub width_m: f32,
    pub height_m: f32,
    pub thickness_m: f32,
    pub vertex_mass_kg: f32,
    #[serde(default)]
    pub pins: Vec<ClothPin>,
    #[serde(default)]
    pub collision_object_ids: Vec<String>,
    #[serde(default)]
    pub settings: ClothSolverSettings,
    #[serde(default)]
    pub material: Surface,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BakeClothRequest {
    pub id: String,
    pub clip: String,
    /// Inclusive range starts at zero; omitted duration uses the full clip.
    #[serde(default)]
    pub duration: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ClothDiagnostics {
    pub max_relative_edge_error: f32,
    pub max_contact_penetration: f32,
    pub max_speed: f32,
    pub contact_projections: u64,
    pub degenerate_bend_projections: u64,
    #[serde(default, skip_serializing_if = "zero")]
    pub self_contact_projections: u64,
    #[serde(default, skip_serializing_if = "zero")]
    pub self_vertex_triangle_projections: u64,
    #[serde(default, skip_serializing_if = "zero")]
    pub self_edge_edge_projections: u64,
    #[serde(default, skip_serializing_if = "zero")]
    pub self_contact_candidates: u64,
    #[serde(default, skip_serializing_if = "zero")]
    pub self_contact_work: u64,
    #[serde(default, skip_serializing_if = "zero")]
    pub max_self_contact_penetration: f32,
    #[serde(default, skip_serializing_if = "zero")]
    pub seam_projections: u64,
    #[serde(default, skip_serializing_if = "zero")]
    pub max_seam_length_error: f32,
}
fn zero<T: Default + PartialEq>(v: &T) -> bool {
    v == &T::default()
}
impl ClothDiagnostics {
    fn empty() -> Self {
        Self {
            max_relative_edge_error: 0.0,
            max_contact_penetration: 0.0,
            max_speed: 0.0,
            contact_projections: 0,
            degenerate_bend_projections: 0,
            self_contact_projections: 0,
            self_vertex_triangle_projections: 0,
            self_edge_edge_projections: 0,
            self_contact_candidates: 0,
            self_contact_work: 0,
            max_self_contact_penetration: 0.0,
            seam_projections: 0,
            max_seam_length_error: 0.0,
        }
    }
    fn add(&mut self, report: &ClothStepReport) {
        self.max_relative_edge_error = self.max_relative_edge_error.max(report.max_relative_edge_error);
        self.max_contact_penetration = self.max_contact_penetration.max(report.max_contact_penetration);
        self.max_speed = self.max_speed.max(report.max_speed);
        self.contact_projections += report.contact_projections;
        self.degenerate_bend_projections += report.degenerate_bend_projections;
        self.self_contact_projections += report.self_contact_projections;
        self.self_vertex_triangle_projections += report.self_vertex_triangle_projections;
        self.self_edge_edge_projections += report.self_edge_edge_projections;
        self.self_contact_candidates += report.self_contact_candidates;
        self.self_contact_work += report.self_contact_work;
        self.max_self_contact_penetration = self.max_self_contact_penetration.max(report.max_self_contact_penetration);
        self.seam_projections += report.seam_projections;
        self.max_seam_length_error = self.max_seam_length_error.max(report.max_seam_length_error);
    }
    fn add_self_clearance(&mut self, report: ClothSelfContactReport) {
        self.max_self_contact_penetration = self.max_self_contact_penetration.max(report.max_penetration);
        self.self_contact_candidates += report.candidates;
        self.self_contact_work += report.work;
    }
    fn validate(&self) -> Result<(), String> {
        for (name, value) in [
            ("strain", self.max_relative_edge_error),
            ("penetration", self.max_contact_penetration),
            ("speed", self.max_speed),
            ("self penetration", self.max_self_contact_penetration),
            ("seam length error", self.max_seam_length_error),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(format!("cloth diagnostic {name} must be finite and nonnegative"));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ClothFrame {
    pub time: f32,
    pub vertices: Vec<V3>,
    pub diagnostics: ClothDiagnostics,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ClothCache {
    pub algorithm: String,
    pub clip: String,
    pub duration: f32,
    pub source_fnv1a64: String,
    pub frames_fnv1a64: String,
    pub frames: Vec<ClothFrame>,
    /// Worst residuals and accumulated projection counts across EVERY substep.
    pub diagnostics: ClothDiagnostics,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ClothAsset {
    pub id: String,
    pub rest_vertices: Vec<V3>,
    pub triangles: Vec<[u32; 3]>,
    pub thickness_m: f32,
    pub inverse_masses: Vec<f32>,
    pub pins: Vec<ClothPin>,
    pub collision_object_ids: Vec<String>,
    pub settings: ClothSolverSettings,
    /// Independent panel recipes and exact ordered boundary correspondences.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sewing: Option<crate::sewing::SewnPattern>,
    /// Named arbitrary planar outlines/holes plus retained deterministic 2D topology.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<crate::pattern::PatternAsset>,
    #[serde(default)]
    pub cache: Option<ClothCache>,
}

fn index(document: &Document, id: &str) -> Result<usize, String> {
    document.objects.iter().position(|o| o.id == id).ok_or_else(|| format!("missing cloth-related object {id}"))
}
fn hash<T: Serialize>(value: &T) -> Result<String, String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    Ok(format!("{:016x}", mm3e_kit::atoms::hash(&bytes)))
}
fn algorithm(asset: &ClothAsset) -> &'static str {
    if asset.pattern.is_some() {
        PATTERN_ALGORITHM
    } else if asset.sewing.is_some() {
        SEWN_ALGORITHM
    } else {
        ALGORITHM
    }
}
fn fingerprint(document: &Document, asset: &ClothAsset, clip_id: &str) -> Result<String, String> {
    let source = simulation_document(document, asset, clip_id)?;
    let clip = &source.clips[0];
    let objects: Vec<_> = source
        .objects
        .iter()
        .map(|o| {
            json!({"id":o.id,"shape":o.shape,"position":o.position,"rotation_degrees":o.rotation_degrees,
            "scale":o.scale,"combine":o.combine,"modifiers":o.modifiers})
        })
        .collect();
    let mut value = json!({"algorithm":algorithm(asset),"objects":objects,"joints":source.joints,"clip":clip,
    "faces":source.faces,"garments":source.garments,"asset":{
        "id":asset.id,"vertices":asset.rest_vertices,"triangles":asset.triangles,"thickness":asset.thickness_m,
        "inverse_masses":asset.inverse_masses,"pins":asset.pins,"colliders":asset.collision_object_ids,"settings":asset.settings
    }});
    if source.faces.iter().any(|face| {
        let controls = &face.controls;
        [controls.gaze_x, controls.gaze_y, controls.brow_left, controls.brow_right, controls.lip_seal]
            .iter()
            .any(|value| *value != 0.0)
    }) || source.clips.iter().flat_map(|clip| &clip.face_tracks).any(|track| {
        matches!(
            track.channel,
            crate::face::FaceChannel::GazeX
                | crate::face::FaceChannel::GazeY
                | crate::face::FaceChannel::BrowLeft
                | crate::face::FaceChannel::BrowRight
                | crate::face::FaceChannel::LipSeal
        )
    }) {
        // These channels now close lids/lips and transform gaze correctly.
        // Only physically retained dependencies invalidate prior face caches.
        value["facial_evaluation"] = "closed-seal-blink-vector-gaze-v2".into();
    }
    if asset.pins.iter().any(|pin| pin.target_triangle.is_some()) {
        // Old barycentric caches used unnormalized world-coordinate products.
        // Mark only this route stale; historical local/world pins retain hashes.
        value["asset"]["attachment_algorithm"] = BARYCENTRIC_ATTACHMENT_ALGORITHM.into();
    }
    if let Some(pattern) = &asset.sewing {
        value["asset"]["sewing"] = serde_json::to_value(pattern).map_err(|e| e.to_string())?;
    }
    if let Some(pattern) = &asset.pattern {
        value["asset"]["pattern"] = serde_json::to_value(pattern).map_err(|e| e.to_string())?;
    }
    // Keep the historical root `clip` key unchanged. Flat clips (including roots
    // whose layers have no physical contribution) retain their original bytes.
    if source.clips.len() > 1 {
        value["layer_clips"] = serde_json::to_value(&source.clips[1..]).map_err(|e| e.to_string())?;
    }
    // These keys did not exist in v3/v4 sources. Empty/unrelated deformation
    // must not stale otherwise identical old caches merely by extending the DTO.
    if source.deformers.is_empty() {
        value["clip"].as_object_mut().expect("serialized clip is an object").remove("morph_tracks");
    } else {
        value["deformers"] = serde_json::to_value(&source.deformers).map_err(|e| e.to_string())?;
        if clip.morph_tracks.is_empty() {
            value["clip"].as_object_mut().expect("serialized clip is an object").remove("morph_tracks");
        }
    }
    hash(&value)
}

/// Cache freshness without refreshing cloth geometry; callers can edit a pose and rebake.
pub fn cache_fresh(document: &Document, asset: &ClothAsset) -> Result<bool, String> {
    let Some(cache) = &asset.cache else { return Ok(false) };
    Ok(cache.algorithm == algorithm(asset) && fingerprint(document, asset, &cache.clip)? == cache.source_fnv1a64)
}

/// Keep the complete transitive geometric dependencies of collision fields and pins,
/// but never rebuild unrelated scenery, film settings, cameras or other cloth caches.
fn simulation_document(document: &Document, asset: &ClothAsset, clip_id: &str) -> Result<Document, String> {
    // Validate the original graph before pruning: disabled or masked branches
    // must not hide dangling references, cycles, or excessive expansion.
    crate::layering::validate(document)?;
    let mut needed: BTreeSet<String> = asset
        .collision_object_ids
        .iter()
        .cloned()
        .chain(asset.pins.iter().filter_map(|pin| pin.target_object.clone()))
        .collect();
    let mut faces = BTreeSet::new();
    let mut garments = BTreeSet::new();
    loop {
        let before = (needed.len(), faces.len(), garments.len());
        for face in &document.faces {
            let head = format!("{}/head", face.character);
            let left = format!("{}/left_eye", face.character);
            let right = format!("{}/right_eye", face.character);
            if needed.contains(&head)
                || needed.contains(&left)
                || needed.contains(&right)
                || face.generated_object_ids.iter().any(|id| needed.contains(id))
            {
                faces.insert(face.id.clone());
                needed.extend([head, left, right]);
                needed.extend(face.generated_object_ids.iter().cloned());
            }
        }
        for garment in &document.garments {
            if needed.contains(&garment.id) {
                garments.insert(garment.id.clone());
                needed.extend(garment.source_object_ids.iter().cloned());
                needed.extend(garment.opening_source_ids.iter().cloned());
            }
        }
        if before == (needed.len(), faces.len(), garments.len()) {
            break;
        }
    }
    let mut joints: BTreeSet<String> = document
        .joints
        .iter()
        .filter(|j| j.objects.iter().any(|id| needed.contains(id)))
        .map(|j| j.id.clone())
        .collect();
    let deformer_ids: BTreeSet<_> = document
        .deformers
        .iter()
        .filter(|deformer| needed.contains(&deformer.object))
        .map(|deformer| deformer.id.clone())
        .collect();
    for deformer in &document.deformers {
        if deformer_ids.contains(&deformer.id) {
            joints.extend(deformer.joints.iter().cloned());
        }
    }
    loop {
        let before = joints.len();
        for joint in &document.joints {
            if joints.contains(&joint.id) {
                if let Some(parent) = &joint.parent {
                    joints.insert(parent.clone());
                }
            }
        }
        if before == joints.len() {
            break;
        }
    }
    let mut source = Document {
        objects: document.objects.iter().filter(|o| needed.contains(&o.id)).cloned().collect(),
        ..Document::default()
    };
    if let Some(first) = source.objects.first_mut() {
        // This extra source need not be a collider. The first COLLIDER is checked
        // against the original authored order and must already be a union seed.
        first.combine = Combination::Union;
    }
    source.faces = document.faces.iter().filter(|f| faces.contains(&f.id)).cloned().collect();
    source.garments = document.garments.iter().filter(|g| garments.contains(&g.id)).cloned().collect();
    source.deformers = document.deformers.iter().filter(|d| deformer_ids.contains(&d.id)).cloned().collect();
    source.joints = document
        .joints
        .iter()
        .filter(|j| joints.contains(&j.id))
        .cloned()
        .map(|mut j| {
            j.objects.retain(|id| needed.contains(id));
            j
        })
        .collect();
    source.clips = physical_clip_graph(document, clip_id, |channel| match channel {
        crate::layering::Channel::Object { id } => needed.contains(id),
        crate::layering::Channel::Joint { id } => joints.contains(id),
        crate::layering::Channel::Face { face, .. } => faces.contains(face),
        crate::layering::Channel::Morph { deformer, .. } => deformer_ids.contains(deformer),
        crate::layering::Channel::Camera => false,
    })?;
    source.lights.clear();
    source.settings.width = 1;
    source.settings.height = 1;
    Ok(source)
}

/// Project sparse animation channels through the complete physical dependency
/// closure. Shared clips accumulate demand from every live incoming reference;
/// an outer mask cannot discard an ancestor joint that is itself selected.
fn physical_clip_graph(
    document: &Document,
    root: &str,
    physical: impl Fn(&crate::layering::Channel) -> bool,
) -> Result<Vec<crate::animation::Clip>, String> {
    use crate::layering::{Channel, Layer};
    type Channels = BTreeSet<Channel>;
    fn target_channel(target: &Target) -> Channel {
        match target {
            Target::Object { id } => Channel::Object { id: id.clone() },
            Target::Joint { id } => Channel::Joint { id: id.clone() },
        }
    }
    fn own(clip: &crate::animation::Clip) -> Channels {
        clip.tracks
            .iter()
            .map(|track| target_channel(&track.target))
            .chain(
                clip.face_tracks.iter().map(|track| Channel::Face { face: track.face.clone(), channel: track.channel }),
            )
            .chain(
                clip.morph_tracks.iter().map(|track| Channel::Morph {
                    deformer: track.deformer.clone(),
                    blendshape: track.blendshape.clone(),
                }),
            )
            .collect()
    }
    fn contributing(layer: &Layer) -> bool {
        layer.enabled
            && layer.weight > 0.0
            && (layer.weight_keys.is_empty() || layer.weight_keys.iter().any(|key| key.value > 0.0))
            && !layer.mask.as_ref().is_some_and(Vec::is_empty)
    }
    fn outputs(
        id: &str,
        clips: &BTreeMap<&str, &crate::animation::Clip>,
        memo: &mut BTreeMap<String, Channels>,
    ) -> Channels {
        if let Some(channels) = memo.get(id) {
            return channels.clone();
        }
        let clip = clips[id];
        let mut channels = own(clip);
        for layer in clip.layers.iter().filter(|layer| contributing(layer)) {
            channels.extend(outputs(&layer.clip, clips, memo).into_iter().filter(|channel| layer.allows(channel)));
        }
        memo.insert(id.into(), channels.clone());
        channels
    }
    let clips: BTreeMap<_, _> = document.clips.iter().map(|clip| (clip.id.as_str(), clip)).collect();
    if !clips.contains_key(root) {
        return Err(format!("missing cloth clip {root}"));
    }
    // `simulation_document` has validated the complete bounded acyclic graph.
    let mut available = BTreeMap::new();
    let root_channels = outputs(root, &clips, &mut available).into_iter().filter(physical).collect();
    let mut demand: BTreeMap<String, Channels> = BTreeMap::from([(root.into(), root_channels)]);
    let mut pending = VecDeque::from([root.to_owned()]);
    while let Some(id) = pending.pop_front() {
        let clip = clips[id.as_str()];
        let direct = own(clip);
        // Clip-owned tracks override all assembled values for the same sparse
        // channel, so their shadowed inputs are not simulation dependencies.
        let inherited: Channels = demand[&id].difference(&direct).cloned().collect();
        for layer in clip.layers.iter().filter(|layer| contributing(layer)) {
            let selected: Channels = available[&layer.clip]
                .intersection(&inherited)
                .filter(|channel| layer.allows(channel))
                .cloned()
                .collect();
            if selected.is_empty() {
                continue;
            }
            let child = demand.entry(layer.clip.clone()).or_default();
            let before = child.len();
            child.extend(selected);
            if child.len() != before {
                pending.push_back(layer.clip.clone());
            }
        }
    }
    let mut projected = BTreeMap::new();
    for (id, channels) in &demand {
        let mut clip = clips[id.as_str()].clone();
        // Historical speech evidence does not change collision or pin motion.
        clip.lip_sync = None;
        let direct = own(&clip);
        let inherited: Channels = channels.difference(&direct).cloned().collect();
        clip.tracks.retain(|track| channels.contains(&target_channel(&track.target)));
        clip.face_tracks
            .retain(|track| channels.contains(&Channel::Face { face: track.face.clone(), channel: track.channel }));
        clip.morph_tracks.retain(|track| {
            channels
                .contains(&Channel::Morph { deformer: track.deformer.clone(), blendshape: track.blendshape.clone() })
        });
        clip.camera_keys.clear();
        clip.camera_easing = crate::animation::Interpolation::default();
        clip.layers.retain_mut(|layer| {
            if !contributing(layer) {
                return false;
            }
            let selected: Channels = available[&layer.clip]
                .intersection(&inherited)
                .filter(|channel| layer.allows(channel))
                .cloned()
                .collect();
            if selected.is_empty() {
                return false;
            }
            // Remove irrelevant mask entries that may name pruned objects, while
            // preserving the original explicit-mask form and contributing order.
            if let Some(mask) = &mut layer.mask {
                mask.retain(|channel| selected.contains(channel));
            }
            true
        });
        projected.insert(id.clone(), clip);
    }
    fn append(
        id: &str,
        clips: &mut BTreeMap<String, crate::animation::Clip>,
        result: &mut Vec<crate::animation::Clip>,
    ) {
        let Some(clip) = clips.remove(id) else { return };
        let children: Vec<_> = clip.layers.iter().map(|layer| layer.clip.clone()).collect();
        result.push(clip);
        for child in children {
            append(&child, clips, result);
        }
    }
    let mut result = Vec::new();
    append(root, &mut projected, &mut result);
    Ok(result)
}

fn shape_work(shape: &Shape) -> Result<(u64, u64), String> {
    Ok(match shape {
        Shape::Csg { expression } => {
            let nodes = expression.compile()?.validate()? as u64;
            (nodes, nodes)
        }
        Shape::Surface { vertices, triangles, .. } => {
            (triangles.len().max(1) as u64, (vertices.len() + triangles.len()) as u64)
        }
        Shape::Volume { samples, .. } => (8, samples.len() as u64),
        _ => (1, 1),
    })
}
fn solver(asset: &ClothAsset) -> Result<Cloth, String> {
    Cloth::new_with_seams(
        asset.rest_vertices.iter().copied().map(vec).collect(),
        asset.triangles.clone(),
        asset.inverse_masses.clone(),
        asset.settings.core()?,
        stitches(asset)?,
    )
    .map_err(|e| e.to_string())
}
fn stitches(asset: &ClothAsset) -> Result<Vec<ClothSeam>, String> {
    if asset.sewing.is_some() && asset.pattern.is_some() {
        return Err("cloth cannot have both rectangular sewing and outline pattern ownership".into());
    }
    if let Some(pattern) = &asset.pattern {
        return crate::pattern::stitches(pattern);
    }
    asset
        .sewing
        .as_ref()
        .map(crate::sewing::build)
        .transpose()
        .map(|built| built.map(|b| b.stitches).unwrap_or_default())
}
fn validate_asset(document: &Document, asset: &ClothAsset) -> Result<(), String> {
    identifier(&asset.id)?;
    if !(3..=MAX_CLOTH_VERTICES).contains(&asset.rest_vertices.len()) {
        return Err(format!("cloth {} must have 3..={MAX_CLOTH_VERTICES} vertices", asset.id));
    }
    range(asset.thickness_m, 0.0005, 0.05, "cloth thickness_m")?;
    if asset.settings.collision_thickness < asset.thickness_m * 0.5 {
        return Err("cloth collision_thickness must cover at least half the rendered thickness".into());
    }
    if asset.settings.self_collision && asset.settings.self_collision_thickness < asset.thickness_m {
        return Err("cloth self-collision layer gap must cover the full rendered sheet thickness".into());
    }
    for &p in &asset.rest_vertices {
        vector(p, "cloth rest vertex")?;
    }
    if asset.sewing.is_some() && asset.pattern.is_some() {
        return Err("cloth cannot have both rectangular sewing and outline pattern ownership".into());
    }
    if let Some(pattern) = &asset.pattern {
        let built = crate::pattern::build(pattern)?;
        if built.vertices != asset.rest_vertices || built.triangles != asset.triangles || built.pins != asset.pins {
            return Err(
                "outline pattern recipes must reproduce retained rest vertices, topology and pins exactly".into()
            );
        }
        let pinned: BTreeSet<_> = built.pins.iter().map(|pin| pin.vertex as usize).collect();
        if asset.inverse_masses.len() != built.vertices.len()
            || asset
                .inverse_masses
                .iter()
                .enumerate()
                .any(|(i, mass)| *mass != if pinned.contains(&i) { 0.0 } else { 1.0 / pattern.vertex_mass_kg })
        {
            return Err("outline pattern masses must match its retained mass and pin recipe".into());
        }
    }
    solver(asset)?;
    if let Some(pattern) = &asset.sewing {
        let built = crate::sewing::build(pattern)?;
        if built.vertices != asset.rest_vertices || built.triangles != asset.triangles || built.pins != asset.pins {
            return Err(
                "sewn panel recipes must reproduce the retained rest vertices, topology and pins exactly".into()
            );
        }
    }
    let object = &document.objects[index(document, &asset.id)?];
    let Shape::Surface { vertices, triangles, thickness_m } = &object.shape else {
        return Err("cloth object must retain its native triangle surface".into());
    };
    if vertices != &asset.rest_vertices
        || triangles != &asset.triangles
        || *thickness_m != asset.thickness_m
        || object.position != [0.0; 3]
        || object.rotation_degrees != [0.0; 3]
        || object.scale != 1.0
        || !matches!(object.combine, Combination::Union)
        || object.modifiers.mirror != [false; 3]
        || object.modifiers.elongate != [0.0; 3]
        || object.modifiers.round != 0.0
        || object.modifiers.onion != 0.0
    {
        return Err(
            "cloth geometry/transform/modifiers are derived; change rest mesh, pins or solver through cloth authoring"
                .into(),
        );
    }
    if document.joints.iter().any(|j| j.objects.contains(&asset.id))
        || document
            .clips
            .iter()
            .flat_map(|c| &c.tracks)
            .any(|t| matches!(&t.target, Target::Object{id} if id == &asset.id))
    {
        return Err("cloth objects cannot also have rigid animation bindings; animate the attachment objects".into());
    }
    let mut pins = BTreeSet::new();
    for pin in &asset.pins {
        if pin.vertex as usize >= asset.rest_vertices.len() || !pins.insert(pin.vertex) {
            return Err("cloth pins must address distinct existing vertices".into());
        }
        vector(pin.point, "cloth pin point")?;
        if asset.inverse_masses[pin.vertex as usize] != 0.0 {
            return Err("cloth pin must have zero inverse mass".into());
        }
        match (&pin.target_object, pin.target_triangle, pin.barycentric) {
            (Some(id), Some(triangle), Some(weights)) => {
                let target = index(document, id)?;
                if !crate::deform::owns_object(document, id) {
                    return Err("barycentric cloth attachments require a continuously deformed target surface".into());
                }
                let Shape::Surface { vertices, triangles, .. } = &document.objects[target].shape else {
                    return Err("barycentric cloth attachments require a native target surface".into());
                };
                let face = triangles
                    .get(triangle as usize)
                    .ok_or("cloth pin target triangle is outside the deformed surface")?;
                if face.iter().any(|&vertex| vertex as usize >= vertices.len()) {
                    return Err("cloth attachment triangle references a missing source vertex".into());
                }
                normalized_barycentric(weights)?;
            }
            (Some(_), None, None) | (None, None, None) => {}
            _ => {
                return Err(
                    "cloth pin target_triangle and barycentric must be supplied together with target_object".into()
                )
            }
        }
        if let Some(id) = &pin.target_object {
            index(document, id)?;
            if pin.target_triangle.is_none() && crate::deform::owns_object(document, id) {
                return Err("cloth pins on a continuously deformed surface require target_triangle and barycentric attachment data".into());
            }
            if document.cloths.iter().any(|c| &c.id == id) {
                return Err("cloth cannot attach to another simulated cloth object".into());
            }
        }
    }
    if asset.inverse_masses.iter().enumerate().any(|(i, w)| *w == 0.0 && !pins.contains(&(i as u32))) {
        return Err("every zero inverse mass needs an explicit cloth pin".into());
    }
    let mut colliders = BTreeSet::new();
    for id in &asset.collision_object_ids {
        index(document, id)?;
        if !colliders.insert(id) || document.cloths.iter().any(|c| &c.id == id) {
            return Err("cloth collider IDs must be unique non-cloth objects".into());
        }
    }
    collider_indices(document, asset)?;
    if let Some(cache) = &asset.cache {
        if (cache.algorithm != algorithm(asset) && !PREVIOUS_ALGORITHMS.contains(&cache.algorithm.as_str()))
            || cache.frames.len() < 2
            || cache.frames.len() > MAX_CLOTH_FRAMES
        {
            return Err("unsupported or unbounded cloth cache".into());
        }
        range(cache.duration, asset.settings.fixed_dt, 10.0, "cloth cache duration")?;
        if cache.frames_fnv1a64 != hash(&cache.frames)? {
            return Err("cloth cache frame integrity fingerprint mismatch".into());
        }
        for (i, frame) in cache.frames.iter().enumerate() {
            let want = i as f32 * asset.settings.fixed_dt;
            if !frame.time.is_finite()
                || (frame.time - want).abs() > 1e-5
                || frame.vertices.len() != asset.rest_vertices.len()
            {
                return Err("cloth cache has invalid frame times or vertex count".into());
            }
            for &p in &frame.vertices {
                vector(p, "cloth cached vertex")?;
            }
            frame.diagnostics.validate()?;
        }
        if (cache.duration - cache.frames.last().unwrap().time).abs() > 1e-5 {
            return Err("cloth cache endpoint does not match its duration".into());
        }
        cache.diagnostics.validate()?;
    }
    Ok(())
}
pub fn validate(document: &Document) -> Result<(), String> {
    if document.cloths.len() > MAX_CLOTHS {
        return Err(format!("at most {MAX_CLOTHS} cloth assets are supported"));
    }
    let mut ids = BTreeSet::new();
    let mut samples = 0;
    for asset in &document.cloths {
        if !ids.insert(&asset.id) {
            return Err("duplicate cloth asset id".into());
        }
        validate_asset(document, asset)?;
        if let Some(cache) = &asset.cache {
            samples += cache.frames.len() * asset.rest_vertices.len();
        }
    }
    if samples > MAX_CLOTH_CACHED_VERTICES {
        return Err("document exceeds the 320000 cloth cached-vertex budget".into());
    }
    Ok(())
}

pub(crate) fn panel_geometry(
    origin: V3,
    axis_u: V3,
    axis_v: V3,
    segments: [u32; 2],
    width_m: f32,
    height_m: f32,
) -> Result<(Vec<V3>, Vec<[u32; 3]>), String> {
    vector(origin, "cloth origin")?;
    vector(axis_u, "cloth axis_u")?;
    vector(axis_v, "cloth axis_v")?;
    let u = vec(axis_u);
    let v = vec(axis_v);
    if (u.length() - 1.0).abs() > 1e-5 || (v.length() - 1.0).abs() > 1e-5 || u.dot(v).abs() > 1e-5 {
        return Err("cloth axes must be unit and perpendicular".into());
    }
    range(width_m, 0.01, 20.0, "cloth width_m")?;
    range(height_m, 0.01, 20.0, "cloth height_m")?;
    let [nx, ny] = segments;
    if nx == 0 || ny == 0 || nx > 255 || ny > 255 || (nx + 1) * (ny + 1) > MAX_CLOTH_VERTICES as u32 {
        return Err("cloth panel segments must be positive with at most 256 total vertices".into());
    }
    let mut vertices = vec![];
    let mut triangles = vec![];
    for y in 0..=ny {
        for x in 0..=nx {
            vertices.push(array(
                vec(origin)
                    + u * ((x as f32 / nx as f32 - 0.5) * width_m)
                    + v * ((y as f32 / ny as f32 - 0.5) * height_m),
            ));
            if x < nx && y < ny {
                let i = y * (nx + 1) + x;
                triangles.push([i, i + 1, i + nx + 1]);
                triangles.push([i + 1, i + nx + 2, i + nx + 1]);
            }
        }
    }
    Ok((vertices, triangles))
}

pub fn create_panel(document: &mut Document, request: &ClothPanelRequest) -> Result<(), String> {
    identifier(&request.id)?;
    if document.objects.iter().any(|o| o.id == request.id) {
        return Err(format!("object {} already exists", request.id));
    }
    range(request.vertex_mass_kg, 0.0001, 100.0, "cloth vertex_mass_kg")?;
    let (vertices, triangles) = panel_geometry(
        request.origin,
        request.axis_u,
        request.axis_v,
        request.segments,
        request.width_m,
        request.height_m,
    )?;
    let mut inverse_masses = vec![1.0 / request.vertex_mass_kg; vertices.len()];
    for pin in &request.pins {
        let w = inverse_masses.get_mut(pin.vertex as usize).ok_or("cloth pin vertex is outside panel")?;
        *w = 0.0;
    }
    let asset = ClothAsset {
        id: request.id.clone(),
        rest_vertices: vertices.clone(),
        triangles: triangles.clone(),
        thickness_m: request.thickness_m,
        inverse_masses,
        pins: request.pins.clone(),
        collision_object_ids: request.collision_object_ids.clone(),
        settings: request.settings.clone(),
        sewing: None,
        pattern: None,
        cache: None,
    };
    let mut candidate = document.clone();
    candidate.objects.push(Entity {
        id: request.id.clone(),
        label: request.label.clone(),
        group: request.group.clone(),
        role: "simulated_cloth".into(),
        shape: Shape::Surface { vertices, triangles, thickness_m: request.thickness_m },
        position: [0.0; 3],
        rotation_degrees: [0.0; 3],
        scale: 1.0,
        material: request.material.clone(),
        combine: Combination::Union,
        modifiers: Modifiers::default(),
    });
    candidate.cloths.push(asset);
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}
/// Replace a panel recipe atomically, preserving its stable identity and object
/// order. The explicit request supplies material/metadata; old dynamics are
/// deliberately invalidated because topology, rest shape or physical inputs may change.
pub fn update_panel(document: &mut Document, request: &ClothPanelRequest) -> Result<(), String> {
    if document.cloths.iter().any(|c| c.id == request.id && c.pattern.is_some()) {
        return Err("use update_pattern_cloth to preserve outline, hole and seam authoring".into());
    }
    if document.cloths.iter().any(|c| c.id == request.id && c.sewing.is_some()) {
        return Err("use update_sewn_cloth to preserve explicit panel and seam authoring".into());
    }
    if !document.cloths.iter().any(|c| c.id == request.id) {
        return Err(format!("missing cloth {}", request.id));
    }
    let position = index(document, &request.id)?;
    let asset_position = document.cloths.iter().position(|c| c.id == request.id).unwrap();
    let mut candidate = document.clone();
    candidate.cloths.retain(|c| c.id != request.id);
    candidate.objects.remove(position);
    create_panel(&mut candidate, request)?;
    let object = candidate.objects.pop().unwrap();
    let asset = candidate.cloths.pop().unwrap();
    candidate.objects.insert(position, object);
    candidate.cloths.insert(asset_position, asset);
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}

pub fn remove(document: &mut Document, id: &str) -> Result<(), String> {
    if !document.cloths.iter().any(|c| c.id == id) {
        return Err(format!("missing cloth {id}"));
    }
    let mut candidate = document.clone();
    candidate.cloths.retain(|c| c.id != id);
    candidate.objects.retain(|o| o.id != id);
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}

fn collider_indices(document: &Document, asset: &ClothAsset) -> Result<Vec<usize>, String> {
    let requested: BTreeSet<_> = asset.collision_object_ids.iter().collect();
    let indices: Vec<_> =
        document.objects.iter().enumerate().filter_map(|(i, o)| requested.contains(&o.id).then_some(i)).collect();
    if let Some(&i) = indices.first() {
        if !matches!(document.objects[i].combine, Combination::Union) {
            return Err("first selected cloth collider in authored order must use union".into());
        }
    }
    Ok(indices)
}
fn collision_scene(scene: &Scene, indices: &[usize]) -> Scene {
    let mut selected = scene.clone();
    selected.objects = indices.iter().map(|&i| scene.objects[i]).collect();
    selected.appearance.clear();
    selected
}
fn contact(scene: &Scene, p: Vec3) -> Option<ClothContact> {
    if scene.objects.is_empty() {
        return None;
    }
    let distance = scene.sample_authored(p).dist;
    // Actual composed authored field, including smooth blends and subtraction;
    // never sample the acceleration bound or include the simulated garment.
    let base = array(p);
    let magnitude = base.iter().map(|v| v.abs()).fold(1.0_f32, f32::max);
    let h = 0.0001_f32.max(4.0 * f32::EPSILON * magnitude);
    let derivative = |axis: usize| {
        let mut lower = base;
        let mut upper = base;
        lower[axis] = (base[axis] - h).min(base[axis].next_down());
        upper[axis] = (base[axis] + h).max(base[axis].next_up());
        let lo = scene.sample_authored(vec(lower)).dist;
        let hi = scene.sample_authored(vec(upper)).dist;
        // Divide by each axis's actual representable span. At large coordinates
        // one nominal epsilon can round differently (or to zero) on each axis.
        ((f64::from(hi) - f64::from(lo)) / (f64::from(upper[axis]) - f64::from(lower[axis]))) as f32
    };
    let normal = Vec3::new(derivative(0), derivative(1), derivative(2));
    Some(ClothContact { distance, normal })
}
/// Validate the exact stored f32 weights in f64, then normalize their accepted
/// rounding residual. Shared with pattern authoring so preview and bake agree.
pub(crate) fn normalized_barycentric(weights: [f32; 3]) -> Result<[f64; 3], String> {
    let weights = weights.map(f64::from);
    let sum = weights[0] + weights[1] + weights[2];
    if weights.iter().any(|weight| !weight.is_finite() || *weight < 0.0 || *weight > 1.0) || (sum - 1.0).abs() > 1e-6 {
        return Err(
            "cloth barycentric attachment requires finite nonnegative weights summing to one within 1e-6".into()
        );
    }
    Ok(weights.map(|weight| weight / sum))
}

fn pin_targets(document: &Document, scene: &Scene, asset: &ClothAsset) -> Result<Vec<(u32, Vec3)>, String> {
    asset
        .pins
        .iter()
        .map(|pin| {
            Ok((
                pin.vertex,
                match (&pin.target_object, pin.target_triangle, pin.barycentric) {
                    (Some(id), Some(triangle), Some(weights)) => {
                        let object = &scene.objects[index(document, id)?];
                        let Prim::Surface { id: surface } = object.prim else {
                            return Err("barycentric cloth attachment target is not a native surface".into());
                        };
                        let surface = scene
                            .surfaces
                            .get(surface as usize)
                            .ok_or("barycentric cloth attachment surface index is invalid")?;
                        let indices = surface
                            .triangles()
                            .get(triangle as usize)
                            .ok_or("barycentric cloth attachment triangle is outside the evaluated surface")?;
                        let vertices = surface.vertices();
                        let weights = normalized_barycentric(weights)?;
                        let [a, b, c] = indices.map(|index| array(vertices[index as usize]).map(f64::from));
                        // Difference coordinates preserve translation even when the
                        // stored weights sum only approximately to one. Round once
                        // after accumulation, rather than each world-space product.
                        vec(std::array::from_fn(|axis| {
                            (a[axis] + (b[axis] - a[axis]) * weights[1] + (c[axis] - a[axis]) * weights[2]) as f32
                        }))
                    }
                    (Some(id), None, None) => scene.objects[index(document, id)?].xform.to_world(vec(pin.point)),
                    (None, None, None) => vec(pin.point),
                    _ => return Err("cloth pin target data is incomplete".into()),
                },
            ))
        })
        .collect()
}
fn penetration(scene: &Scene, vertices: &[V3], thickness: f32) -> Result<f32, String> {
    if scene.objects.is_empty() {
        return Ok(0.0);
    }
    let mut maximum = 0.0_f32;
    for &p in vertices {
        let distance = scene.sample_authored(vec(p)).dist;
        if !distance.is_finite() {
            return Err("nonfinite cloth collision field".into());
        }
        maximum = maximum.max(thickness - distance);
    }
    Ok(maximum)
}
fn self_contact_clearance(asset: &ClothAsset, vertices: &[V3]) -> Result<ClothSelfContactReport, String> {
    if !asset.settings.self_collision {
        return Ok(ClothSelfContactReport { max_penetration: 0.0, candidates: 0, work: 0 });
    }
    measure_cloth_self_contact_with_seams(
        &vertices.iter().copied().map(vec).collect::<Vec<_>>(),
        &asset.triangles,
        &stitches(asset)?,
        asset.settings.core()?,
    )
    .map_err(|e| format!("cloth {} static self-contact clearance query: {e}", asset.id))
}
fn accept(asset: &ClothAsset, diagnostics: &ClothDiagnostics, time: f32) -> Result<(), String> {
    diagnostics.validate()?;
    if diagnostics.max_seam_length_error > asset.settings.max_seam_error_m {
        return Err(format!(
            "cloth {} bake rejected at {time:.6}s: seam length error {}m exceeds {}m",
            asset.id, diagnostics.max_seam_length_error, asset.settings.max_seam_error_m
        ));
    }
    if diagnostics.max_contact_penetration > asset.settings.max_penetration_m {
        return Err(format!(
            "cloth {} bake rejected at {time:.6}s: vertex penetration {}m exceeds {}m",
            asset.id, diagnostics.max_contact_penetration, asset.settings.max_penetration_m
        ));
    }
    if diagnostics.max_self_contact_penetration > asset.settings.max_penetration_m {
        return Err(format!(
            "cloth {} bake rejected at {time:.6}s: self-contact penetration {}m exceeds {}m",
            asset.id, diagnostics.max_self_contact_penetration, asset.settings.max_penetration_m
        ));
    }
    if diagnostics.max_relative_edge_error > asset.settings.max_relative_edge_error {
        return Err(format!(
            "cloth {} bake rejected at {time:.6}s: relative edge error {} exceeds {}",
            asset.id, diagnostics.max_relative_edge_error, asset.settings.max_relative_edge_error
        ));
    }
    if diagnostics.degenerate_bend_projections > 0 {
        return Err(format!("cloth {} bake rejected at {time:.6}s: degenerate bending hinge", asset.id));
    }
    Ok(())
}

fn charged_bake_work(base: u64, self_contact_work: u64) -> Result<u64, String> {
    let total = base.checked_add(self_contact_work).ok_or("cloth bake work counter overflow")?;
    if total > MAX_CLOTH_BAKE_WORK {
        return Err("cloth bake exceeds 100000000 charged constraint/field/self-contact evaluations; lower resolution, duration or solver work".into());
    }
    Ok(total)
}

pub fn bake(document: &mut Document, request: &BakeClothRequest) -> Result<Value, String> {
    validate(document)?;
    let asset = document
        .cloths
        .iter()
        .find(|c| c.id == request.id)
        .ok_or_else(|| format!("missing cloth {}", request.id))?
        .clone();
    let clip =
        document.clips.iter().find(|c| c.id == request.clip).ok_or_else(|| format!("missing clip {}", request.clip))?;
    let duration = request.duration.unwrap_or(clip.duration);
    range(duration, asset.settings.fixed_dt, clip.duration.min(10.0), "cloth bake duration")?;
    let count = (duration / asset.settings.fixed_dt).round() as usize;
    if count + 1 > MAX_CLOTH_FRAMES || (count as f32 * asset.settings.fixed_dt - duration).abs() > 1e-5 {
        return Err("cloth bake requires a fixed_dt-aligned duration and at most 601 inclusive frames".into());
    }
    let other_samples: usize = document
        .cloths
        .iter()
        .filter(|c| c.id != asset.id)
        .map(|c| c.cache.as_ref().map(|v| v.frames.len() * c.rest_vertices.len()).unwrap_or(0))
        .sum();
    if other_samples + (count + 1) * asset.rest_vertices.len() > MAX_CLOTH_CACHED_VERTICES {
        return Err("cloth bake exceeds document cached-vertex budget".into());
    }
    // Other baked cloth is not a physical source: dependencies are authored
    // bodies and attachment transforms. Avoid revalidating immutable old cache
    // payloads during every substep when replacing a cache.
    let source_document = simulation_document(document, &asset, &request.clip)?;
    let sample_at = |time| AnimationSample { clip: request.clip.clone(), time, playback: Playback::Clamp };
    let (base_scene, _) = source_document.compile(&Pass::Beauty)?;
    let indices = collider_indices(&source_document, &asset)?;
    let mut cloth = solver(&asset)?;
    let (edges, bends) = cloth.constraint_counts();
    let collider_cost = indices
        .iter()
        .try_fold(0u64, |sum, &i| shape_work(&source_document.objects[i].shape).map(|(query, _)| sum + query))?;
    let rebuild_cost = source_document
        .objects
        .iter()
        .try_fold(0u64, |sum, object| shape_work(&object.shape).map(|(_, build)| sum + build))?;
    let per_iteration = (edges + bends + cloth.seams().len()) as u64
        + asset.rest_vertices.len() as u64 * u64::from(asset.settings.contact_iterations) * (1 + 7 * collider_cost);
    let physical_substeps = count as u64 * u64::from(asset.settings.substeps);
    let deformation_evaluations = if source_document.deformers.is_empty() { 0 } else { physical_substeps + 1 };
    // Charge dense morph accumulation, actual sparse influences, skin palette
    // entries, and rebuilding the deformed triangle fields at every invocation.
    let deformation_cost = crate::deform::work(&source_document) as u64
        + source_document.deformers.iter().map(|d| d.joints.len() as u64).sum::<u64>();
    let deformed_rebuild_cost = source_document
        .objects
        .iter()
        .filter(|o| crate::deform::owns_object(&source_document, &o.id))
        .try_fold(0u64, |sum, o| shape_work(&o.shape).map(|(_, build)| sum + build))?;
    let deformation_work = (deformation_cost + deformed_rebuild_cost) * deformation_evaluations;
    let attachment_evaluations =
        (physical_substeps + 1) * asset.pins.iter().filter(|pin| pin.target_triangle.is_some()).count() as u64;
    let attachment_work = attachment_evaluations * BARYCENTRIC_ATTACHMENT_WORK;
    let work = (per_iteration * u64::from(asset.settings.iterations) + rebuild_cost) * physical_substeps
        + deformation_work
        + attachment_work;
    if work > MAX_CLOTH_BAKE_WORK {
        return Err("cloth bake exceeds 100000000 estimated constraint/field evaluations; lower resolution, duration or solver work".into());
    }
    let mut initial = base_scene.clone();
    let mut initial_camera = source_document.camera.compile();
    let initial_evaluation =
        crate::animation::evaluate(&source_document, &mut initial, &mut initial_camera, &sample_at(0.0))?;
    crate::face::refresh(&source_document, &mut initial, Some(&sample_at(0.0)))?;
    crate::deform::refresh(&source_document, &mut initial, Some(&sample_at(0.0)), Some(&initial_evaluation))?;
    crate::garment::refresh(&source_document, &mut initial)?;
    // Authored pins must coincide at frame zero: silently snapping a rest mesh
    // would change the garment geometry and fabricate an initial velocity.
    for (i, p) in pin_targets(&source_document, &initial, &asset)? {
        if (p - vec(asset.rest_vertices[i as usize])).length() > 1e-5 {
            return Err(format!("cloth pin {i} does not meet its rest vertex at clip time zero"));
        }
    }
    let mut diagnostics = ClothDiagnostics::empty();
    diagnostics.max_seam_length_error = seam_error(&asset, &asset.rest_vertices)?;
    diagnostics.max_contact_penetration =
        penetration(&collision_scene(&initial, &indices), &asset.rest_vertices, asset.settings.collision_thickness)?;
    diagnostics.add_self_clearance(self_contact_clearance(&asset, &asset.rest_vertices)?);
    charged_bake_work(work, diagnostics.self_contact_work)?;
    accept(&asset, &diagnostics, 0.0)?;
    let mut frames =
        vec![ClothFrame { time: 0.0, vertices: asset.rest_vertices.clone(), diagnostics: diagnostics.clone() }];
    for frame in 1..=count {
        let mut frame_diagnostics = ClothDiagnostics::empty();
        for substep in 1..=asset.settings.substeps {
            let frame_start = (frame - 1) as f32 * asset.settings.fixed_dt;
            let frame_end = frame as f32 * asset.settings.fixed_dt;
            // Match the exact timestamp stored in this frame. Independently
            // multiplying an accumulated substep index can fall one f32 ULP
            // before an authored Step key and omit an endpoint discontinuity.
            let time = if substep == asset.settings.substeps {
                frame_end
            } else {
                (f64::from(frame_start)
                    + (f64::from(frame_end) - f64::from(frame_start)) * f64::from(substep)
                        / f64::from(asset.settings.substeps)) as f32
            };
            let mut scene = base_scene.clone();
            let mut camera = source_document.camera.compile();
            let evaluation = crate::animation::evaluate(&source_document, &mut scene, &mut camera, &sample_at(time))?;
            crate::face::refresh(&source_document, &mut scene, Some(&sample_at(time)))?;
            crate::deform::refresh(&source_document, &mut scene, Some(&sample_at(time)), Some(&evaluation))?;
            crate::garment::refresh(&source_document, &mut scene)?;
            for (i, p) in pin_targets(&source_document, &scene, &asset)? {
                cloth.set_pin_position(i, p).map_err(|e| e.to_string())?;
            }
            let colliders = collision_scene(&scene, &indices);
            let report =
                cloth.step(|p| contact(&colliders, p)).map_err(|e| format!("cloth bake at {time:.6}s: {e}"))?;
            frame_diagnostics.add(&report);
            diagnostics.add(&report);
            charged_bake_work(work, diagnostics.self_contact_work)?;
            accept(&asset, &frame_diagnostics, time)?;
        }
        let vertices = cloth.positions().iter().copied().map(array).collect::<Vec<_>>();
        for &p in &vertices {
            vector(p, "baked cloth vertex")?;
        }
        // Every delivered frame must remain a valid actual triangle surface.
        TriangleSurface::new(
            vertices.iter().copied().map(vec).collect(),
            asset.triangles.clone(),
            asset.thickness_m * 0.5,
        )?;
        frames.push(ClothFrame {
            time: frame as f32 * asset.settings.fixed_dt,
            vertices,
            diagnostics: frame_diagnostics,
        });
    }
    let cache = ClothCache {
        algorithm: algorithm(&asset).into(),
        clip: request.clip.clone(),
        duration: count as f32 * asset.settings.fixed_dt,
        source_fnv1a64: fingerprint(document, &asset, &request.clip)?,
        frames_fnv1a64: hash(&frames)?,
        frames,
        diagnostics,
    };
    let charged_work_total = charged_bake_work(work, cache.diagnostics.self_contact_work)?;
    let result = json!({"id":asset.id,"clip":cache.clip,"frames":cache.frames.len(),"duration":cache.duration,
        "source_fnv1a64":cache.source_fnv1a64,"frames_fnv1a64":cache.frames_fnv1a64,"diagnostics":cache.diagnostics,
        "semantics":semantics(),"source_objects":source_document.objects.len(),
        "estimated_work":work,"estimated_work_base":work,
        "source_deformers":source_document.deformers.len(),"deformation_evaluations":deformation_evaluations,
        "estimated_deformation_work":deformation_work,"deformation_kernel_work_per_evaluation":deformation_cost,
        "barycentric_attachment_evaluations":attachment_evaluations,"estimated_attachment_work":attachment_work,
        "deformation_rebuild_work_per_evaluation":deformed_rebuild_cost,
        "estimated_work_scope":"base constraints, external field queries, source rebuilds, relevant dense morph/sparse skin deformation and normalized barycentric attachment arithmetic; excludes self-contact work",
        "charged_self_contact_work":cache.diagnostics.self_contact_work,"charged_work_total":charged_work_total});
    let mut candidate = document.clone();
    candidate.cloths.iter_mut().find(|c| c.id == request.id).unwrap().cache = Some(cache);
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(result)
}
fn semantics() -> Value {
    json!({"simulation":"fixed-step XPBD edge stretch, signed dihedral bending and explicit seam distance constraints; moving object-local pins follow their evaluated joint ancestry",
        "collision":"discrete vertex probes against selected object fields in authored CSG order; numerical field-gradient normals",
        "delivery":"native triangle positions and separate panel identities; free vertices interpolate baked samples, pins follow exact evaluated attachment transforms; sampled poses recheck external vertex clearance, edge strain, seam length error and enabled self-contact feature clearance against authored tolerances",
        "sewing":"ordered boundary correspondence is explicit; separate panel vertices stay distinct and are constrained, never welded; the initial assembly pose must meet the authored seam tolerance, with no hidden snap or warmup",
        "fingerprints":"FNV-1a64 accidental-change detection, not a cryptographic signature; transitive collision/pin geometry and their chosen animation tracks",
        "self_contact":"optional bounded swept vertex-triangle and edge-edge proximity with inverse-mass correction; Coulomb positional friction when enabled",
        "sample_self_contact":"static vertex-triangle and edge-edge distance with own/one-ring exclusions, including pins; zero deficit does not prove absence of every preexisting edge-through-face intersection or certify continuous motion",
        "not_implemented":["general arbitrary-motion CCD certification","external triangle-interior collision","rigid-body reaction forces","measured woven-fabric constitutive law","same-panel darts, seam allowances and grading"]})
}
fn evaluated_vertices(
    document: &Document,
    asset: &ClothAsset,
    sample: Option<&AnimationSample>,
    posed_scene: &Scene,
) -> Result<Vec<V3>, String> {
    let Some(sample) = sample else { return Ok(asset.rest_vertices.clone()) };
    let cache = asset
        .cache
        .as_ref()
        .ok_or_else(|| format!("cloth {} requires bake_cloth for animation {}", asset.id, sample.clip))?;
    if cache.algorithm != algorithm(asset) {
        return Err(format!(
            "cloth {} cache is stale for solver {}; rebake using {}",
            asset.id,
            cache.algorithm,
            algorithm(asset)
        ));
    }
    if cache.clip != sample.clip {
        return Err(format!("cloth {} cache is for {}; bake requested clip {}", asset.id, cache.clip, sample.clip));
    }
    if fingerprint(document, asset, &cache.clip)? != cache.source_fnv1a64 {
        return Err(format!("cloth {} cache is stale; rebake after source, pin, solver or clip changes", asset.id));
    }
    let clip = document.clips.iter().find(|c| c.id == sample.clip).ok_or("missing cloth playback clip")?;
    range(sample.time, -86400.0, 86400.0, "cloth playback time")?;
    let time = match sample.playback {
        Playback::Clamp => sample.time.clamp(0.0, clip.duration),
        Playback::Loop => sample.time.rem_euclid(clip.duration),
    };
    if time > cache.duration + 1e-5 {
        return Err(format!("cloth {} requested time {time} is beyond baked duration {}", asset.id, cache.duration));
    }
    let f = (time / asset.settings.fixed_dt).clamp(0.0, (cache.frames.len() - 1) as f32);
    let a = f.floor() as usize;
    let b = (a + 1).min(cache.frames.len() - 1);
    let t = f - a as f32;
    let mut vertices: Vec<V3> = cache.frames[a]
        .vertices
        .iter()
        .zip(&cache.frames[b].vertices)
        .map(|(p, q)| array(vec(*p).mix(vec(*q), t)))
        .collect();
    // A kinematic attachment follows the authored eased/quaternion transform,
    // not the chord between two cached world points. Use the same exact posed
    // base scene for inspection and rendering; no new physics is invented here.
    for (vertex, target) in pin_targets(document, posed_scene, asset)? {
        let point = array(target);
        vector(point, "evaluated cloth attachment")?;
        vertices[vertex as usize] = point;
    }
    Ok(vertices)
}
pub fn refresh(document: &Document, scene: &mut Scene, sample: Option<&AnimationSample>) -> Result<(), String> {
    refresh_where(document, scene, sample, |_| true)
}
/// Evaluate only selected cloth assets while retaining the document's object
/// indices in the supplied scene. Unselected unbaked cloth cannot block a query
/// about an independent body. Full-scene rendering continues to use `refresh`.
pub fn refresh_selected(
    document: &Document,
    scene: &mut Scene,
    sample: Option<&AnimationSample>,
    selected_ids: &BTreeSet<String>,
) -> Result<(), String> {
    refresh_where(document, scene, sample, |asset| selected_ids.contains(&asset.id))
}
fn refresh_where(
    document: &Document,
    scene: &mut Scene,
    sample: Option<&AnimationSample>,
    selected: impl Fn(&ClothAsset) -> bool,
) -> Result<(), String> {
    let mut updates = vec![];
    for asset in document.cloths.iter().filter(|asset| selected(asset)) {
        let i = index(document, &asset.id)?;
        let Prim::Surface { id } = scene.objects[i].prim else {
            return Err("cloth compiled primitive must be a surface".into());
        };
        let vertices = evaluated_vertices(document, asset, sample, scene)?;
        if sample.is_some() {
            let indices = collider_indices(document, asset)?;
            let actual_penetration =
                penetration(&collision_scene(scene, &indices), &vertices, asset.settings.collision_thickness)?;
            let actual_edge_error = relative_edge_error(asset, &vertices)?;
            let actual_self_contact = self_contact_clearance(asset, &vertices)?;
            let actual_seam_error = seam_error(asset, &vertices)?;
            if actual_penetration > asset.settings.max_penetration_m
                || actual_self_contact.max_penetration > asset.settings.max_penetration_m
                || actual_edge_error > asset.settings.max_relative_edge_error
                || actual_seam_error > asset.settings.max_seam_error_m
            {
                if actual_seam_error > asset.settings.max_seam_error_m {
                    return Err(format!(
                        "cloth {} sampled pose seam length error {}m exceeds {}m",
                        asset.id, actual_seam_error, asset.settings.max_seam_error_m
                    ));
                }
                return Err(format!("cloth {} sampled pose exceeds authored tolerances: vertex penetration {}m (limit {}m), self-contact feature clearance deficit {}m (limit {}m), relative edge error {} (limit {}); inspect cloth_state and rebake with smaller fixed_dt or repair attachments/colliders. These are static feature clearance/edge checks, not continuous collision or global surface-intersection certification",asset.id,actual_penetration,asset.settings.max_penetration_m,actual_self_contact.max_penetration,asset.settings.max_penetration_m,actual_edge_error,asset.settings.max_relative_edge_error));
            }
        }
        let surface = TriangleSurface::new(
            vertices.into_iter().map(vec).collect(),
            asset.triangles.clone(),
            asset.thickness_m * 0.5,
        )?;
        if id as usize >= scene.surfaces.len() {
            return Err("cloth compiled surface id is invalid".into());
        }
        updates.push((i, id as usize, surface));
    }
    for (i, id, surface) in updates {
        scene.surfaces[id] = surface;
        scene.objects[i].xform = Transform::IDENTITY;
    }
    Ok(())
}
fn relative_edge_error(asset: &ClothAsset, vertices: &[V3]) -> Result<f32, String> {
    let mut edges = BTreeSet::new();
    for &[a, b, c] in &asset.triangles {
        for (i, j) in [(a, b), (b, c), (c, a)] {
            edges.insert((i.min(j) as usize, i.max(j) as usize));
        }
    }
    let length =
        |a: V3, b: V3| a.iter().zip(b).map(|(&x, y)| (f64::from(x) - f64::from(y)).powi(2)).sum::<f64>().sqrt();
    let mut maximum = 0.0_f64;
    for (a, b) in edges {
        let rest = length(asset.rest_vertices[a], asset.rest_vertices[b]);
        maximum = maximum.max((length(vertices[a], vertices[b]) / rest - 1.0).abs());
    }
    if !maximum.is_finite() || maximum > f64::from(f32::MAX) {
        return Err("evaluated cloth edge error is not finite".into());
    }
    Ok(maximum as f32)
}
fn seam_error(asset: &ClothAsset, vertices: &[V3]) -> Result<f32, String> {
    let mut maximum = 0.0_f64;
    for seam in stitches(asset)? {
        let [a, b] = seam.vertices.map(|i| vertices[i as usize]);
        let length = a.iter().zip(b).map(|(&x, y)| (f64::from(x) - f64::from(y)).powi(2)).sum::<f64>().sqrt();
        maximum = maximum.max((length - f64::from(seam.rest_length)).abs());
    }
    if !maximum.is_finite() || maximum > f64::from(f32::MAX) {
        return Err("nonfinite sampled seam length error".into());
    }
    Ok(maximum as f32)
}

pub fn inspect(document: &Document, id: &str, sample: Option<&AnimationSample>) -> Result<Value, String> {
    validate(document)?;
    let asset = document.cloths.iter().find(|c| c.id == id).ok_or_else(|| format!("missing cloth {id}"))?;
    let (scene, _) = document.compile_at_base(&Pass::Beauty, sample)?;
    let vertices = evaluated_vertices(document, asset, sample, &scene)?;
    let indices = collider_indices(document, asset)?;
    let current_penetration =
        penetration(&collision_scene(&scene, &indices), &vertices, asset.settings.collision_thickness)?;
    let current_edge_error = relative_edge_error(asset, &vertices)?;
    let current_self_contact = self_contact_clearance(asset, &vertices)?;
    let current_seam_error = seam_error(asset, &vertices)?;
    let within_tolerances = current_penetration <= asset.settings.max_penetration_m
        && current_self_contact.max_penetration <= asset.settings.max_penetration_m
        && current_edge_error <= asset.settings.max_relative_edge_error
        && current_seam_error <= asset.settings.max_seam_error_m;
    let freshness = asset.cache.as_ref().map(|_| cache_fresh(document, asset).unwrap_or(false));
    Ok(json!({"id":id,"vertices":vertices,"triangles":asset.triangles,"thickness_m":asset.thickness_m,
        "sewing":asset.sewing,"pattern":asset.pattern,"stitch_pairs":stitches(asset)?.iter().map(|s|json!({"vertices":s.vertices,"rest_length_m":s.rest_length,"compliance":s.compliance})).collect::<Vec<_>>(),
        "pins":asset.pins,"collision_object_ids":asset.collision_object_ids,"settings":asset.settings,
        "animation":sample,"cache":asset.cache.as_ref().map(|cache|json!({"clip":cache.clip,"frames":cache.frames.len(),"duration":cache.duration,
            "fresh":freshness,"source_fnv1a64":cache.source_fnv1a64,"frames_fnv1a64":cache.frames_fnv1a64,"diagnostics":cache.diagnostics})),
        "sample_vertex_penetration_m":current_penetration,"sample_max_relative_edge_error":current_edge_error,
        "sample_max_self_contact_penetration_m":current_self_contact.max_penetration,
        "sample_max_seam_length_error_m":current_seam_error,
        "sample_self_contact_candidates":current_self_contact.candidates,"sample_self_contact_work":current_self_contact.work,
        "sample_within_bake_tolerances":within_tolerances,
        "sample_diagnostics_scope":"actual evaluated vertices after exact pin attachment; discrete field probes, edge lengths and enabled static self-contact feature clearance, not continuous collision or global surface-intersection certification",
        "semantics":semantics()}))
}
