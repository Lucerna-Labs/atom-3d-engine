//! Procedural SDF facial controls. Lids are clipped physical shells, lips are swept capsule
//! curves, and an isolated cavity changes the head's evaluated geometry. The authored head
//! and eyes remain the source of truth. This is not weighted skinning or automatic audio sync.
use crate::{
    animation::{AnimationSample, Clip, Interpolation, Playback, Target},
    csg::{from_shape, CsgExpr},
    model::{identifier, range, vec, Combination, Document, Entity, Modifiers, Shape, Surface, V3},
};
use mm3e_kit::{
    csg::Expr,
    vec::{Mat3, Quat, Transform},
};
use mm3e_orchestrator::{anim::Track, Prim, Scene};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;

pub const MAX_FACES: usize = 32;
pub const FACE_CHANNELS: usize = 11;
const PARTS: [&str; 7] = [
    "left_upper_lid",
    "left_lower_lid",
    "right_upper_lid",
    "right_lower_lid",
    "upper_lip",
    "lower_lip",
    "mouth_interior",
];

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FaceRequest {
    pub id: String,
    pub character: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct FaceControls {
    pub blink_left: f32,
    pub blink_right: f32,
    pub jaw_open: f32,
    pub lip_round: f32,
    pub lip_wide: f32,
    pub smile: f32,
    /// Signed translation of both eyes and lids in head-local horizontal/vertical units.
    #[serde(skip_serializing_if = "zero_control")]
    pub gaze_x: f32,
    #[serde(skip_serializing_if = "zero_control")]
    pub gaze_y: f32,
    /// Signed lid-cut displacement per side; fades to zero during a full blink.
    #[serde(skip_serializing_if = "zero_control")]
    pub brow_left: f32,
    #[serde(skip_serializing_if = "zero_control")]
    pub brow_right: f32,
    /// At one, close the procedural aperture without changing the authored jaw control.
    #[serde(skip_serializing_if = "zero_control")]
    pub lip_seal: f32,
}
// New neutral controls must preserve legacy serialized face/cache inputs. Both signs
// of zero are neutral; non-finite values are not neutral and still require validation.
fn zero_control(value: &f32) -> bool {
    *value == 0.0
}
impl FaceControls {
    pub fn validate(&self) -> Result<(), String> {
        for (name, value) in [
            ("blink_left", self.blink_left),
            ("blink_right", self.blink_right),
            ("jaw_open", self.jaw_open),
            ("lip_round", self.lip_round),
            ("gaze_x", self.gaze_x),
            ("gaze_y", self.gaze_y),
            ("brow_left", self.brow_left),
            ("brow_right", self.brow_right),
            ("lip_seal", self.lip_seal),
        ] {
            range(
                value,
                if matches!(name, "gaze_x" | "gaze_y" | "brow_left" | "brow_right") { -1.0 } else { 0.0 },
                1.0,
                name,
            )?;
        }
        range(self.lip_wide, -1.0, 1.0, "lip_wide")?;
        range(self.smile, -1.0, 1.0, "smile")
    }
    fn set(&mut self, channel: FaceChannel, value: f32) {
        match channel {
            FaceChannel::BlinkLeft => self.blink_left = value,
            FaceChannel::BlinkRight => self.blink_right = value,
            FaceChannel::JawOpen => self.jaw_open = value,
            FaceChannel::LipRound => self.lip_round = value,
            FaceChannel::LipWide => self.lip_wide = value,
            FaceChannel::Smile => self.smile = value,
            FaceChannel::GazeX => self.gaze_x = value,
            FaceChannel::GazeY => self.gaze_y = value,
            FaceChannel::BrowLeft => self.brow_left = value,
            FaceChannel::BrowRight => self.brow_right = value,
            FaceChannel::LipSeal => self.lip_seal = value,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FaceRig {
    pub id: String,
    pub character: String,
    #[serde(default)]
    pub controls: FaceControls,
    pub generated_object_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum FaceChannel {
    BlinkLeft,
    BlinkRight,
    JawOpen,
    LipRound,
    LipWide,
    Smile,
    GazeX,
    GazeY,
    BrowLeft,
    BrowRight,
    LipSeal,
}
impl FaceChannel {
    fn validate(self, value: f32) -> Result<(), String> {
        range(
            value,
            if matches!(
                self,
                Self::LipWide | Self::Smile | Self::GazeX | Self::GazeY | Self::BrowLeft | Self::BrowRight
            ) {
                -1.0
            } else {
                0.0
            },
            1.0,
            "facial key value",
        )
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScalarKey {
    pub time: f32,
    pub value: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FaceTrack {
    pub face: String,
    pub channel: FaceChannel,
    pub keys: Vec<ScalarKey>,
    #[serde(default)]
    pub easing: Interpolation,
}

fn ids(face: &str) -> Vec<String> {
    PARTS.iter().map(|p| format!("{face}/{p}")).collect()
}
fn index(document: &Document, id: &str) -> Result<usize, String> {
    document.objects.iter().position(|e| e.id == id).ok_or_else(|| format!("facial rig requires object {id}"))
}
fn transform(e: &Entity) -> Transform {
    let r = e.rotation_degrees.map(f32::to_radians);
    Transform::new(vec(e.position), Mat3::from_euler(r[0], r[1], r[2]), e.scale)
}
fn relative(parent: Transform, world: Transform) -> Transform {
    Transform::new(parent.to_local(world.pos), parent.rot.transpose() * world.rot, world.scale / parent.scale)
}
fn plain(e: &Entity) -> bool {
    e.modifiers.mirror == [false; 3]
        && e.modifiers.elongate == [0.0; 3]
        && e.modifiers.round == 0.0
        && e.modifiers.onion == 0.0
}
struct Sources<'a> {
    head: usize,
    eyes: [usize; 2],
    head_entity: &'a Entity,
    radii: V3,
}
fn sources<'a>(doc: &'a Document, rig: &FaceRig) -> Result<Sources<'a>, String> {
    let head = index(doc, &format!("{}/head", rig.character))?;
    let eyes =
        [index(doc, &format!("{}/left_eye", rig.character))?, index(doc, &format!("{}/right_eye", rig.character))?];
    let h = &doc.objects[head];
    let Shape::Ellipsoid { radii } = h.shape else {
        return Err("facial rig requires an authored ellipsoid head".into());
    };
    if !plain(h) {
        return Err("facial rig requires an unmodified ellipsoid head".into());
    }
    from_shape(&h.shape)?;
    for eye in eyes {
        let e = &doc.objects[eye];
        if !matches!(e.shape, Shape::Sphere { .. }) || !plain(e) || !matches!(e.combine, Combination::Union) {
            return Err("facial rig requires unmodified sphere eyes with hard union".into());
        }
        from_shape(&e.shape)?;
        let head_joint = doc.joints.iter().find(|j| j.objects.contains(&h.id)).map(|j| &j.id);
        let eye_joint = doc.joints.iter().find(|j| j.objects.contains(&e.id)).map(|j| &j.id);
        if head_joint != eye_joint {
            return Err("facial eyes must share the head's joint binding (or both be unbound)".into());
        }
    }
    Ok(Sources { head, eyes, head_entity: h, radii })
}

pub fn create(document: &mut Document, request: &FaceRequest) -> Result<(), String> {
    identifier(&request.id)?;
    identifier(&request.character)?;
    if document.faces.len() >= MAX_FACES {
        return Err("too many facial rigs".into());
    }
    if document.faces.iter().any(|f| f.id == request.id || f.character == request.character) {
        return Err("facial rig id and source character must be unique".into());
    }
    let rig = FaceRig {
        id: request.id.clone(),
        character: request.character.clone(),
        controls: FaceControls::default(),
        generated_object_ids: ids(&request.id),
    };
    sources(document, &rig)?;
    for id in &rig.generated_object_ids {
        identifier(id)?;
        if document.objects.iter().any(|e| &e.id == id) {
            return Err(format!("facial object id already exists: {id}"));
        }
    }
    let objects = rest_objects(document, &rig)?;
    document.objects.extend(objects);
    document.faces.push(rig);
    Ok(())
}

/// Rebuild only derived rest geometry using current authored source geometry. Preserve material
/// edits to generated parts. No neutral cached head can outlive a source-head edit.
pub fn synchronize(document: &mut Document) -> Result<(), String> {
    for rig in document.faces.clone() {
        if rig.generated_object_ids != ids(&rig.id) {
            return Err("facial generated object ids do not match their stable contract".into());
        }
        rig.controls.validate()?;
        let generated = rest_objects(document, &rig)?;
        for mut entity in generated {
            let i = index(document, &entity.id)?;
            entity.material = document.objects[i].material.clone();
            document.objects[i] = entity;
        }
    }
    Ok(())
}

// Materials are authored independently; every other serialized entity field is owned
// by the generator and must match exactly, including metadata and source transforms.
fn derived_matches(got: &Entity, want: &Entity) -> Result<bool, String> {
    let mut got = got.clone();
    got.material = want.material.clone();
    Ok(serde_json::to_value(got).map_err(|e| e.to_string())?
        == serde_json::to_value(want).map_err(|e| e.to_string())?)
}

/// Native-read compatibility for the verified September 10 procedural generator only.
/// Plan every replacement before changing the candidate; never repair arbitrary stale,
/// partially regenerated, or edited derived objects. The caller still fully validates
/// the resulting document before it can become editor state or durable project data.
pub(crate) fn migrate_legacy_native(document: &mut Document) -> Result<Vec<String>, String> {
    if document.faces.len() > MAX_FACES {
        return Err("too many facial rigs".into());
    }
    let mut replacements = Vec::new();
    let mut migrated = Vec::new();
    for rig in &document.faces {
        if rig.generated_object_ids != ids(&rig.id) {
            return Err("facial generated object ids do not match their stable contract".into());
        }
        let current = rest_objects(document, rig)?;
        let indices: Vec<usize> =
            rig.generated_object_ids.iter().map(|id| index(document, id)).collect::<Result<_, _>>()?;
        let matches = |expected: &[Entity]| -> Result<bool, String> {
            for (&index, want) in indices.iter().zip(expected) {
                if !derived_matches(&document.objects[index], want)? {
                    return Ok(false);
                }
            }
            Ok(true)
        };
        if matches(&current)? {
            continue;
        }
        let legacy = rest_objects_for_geometry(document, rig, RestGeometry::September10)?;
        if !matches(&legacy)? {
            return Err(format!(
                "facial rig {} has stale or edited derived geometry; it matches neither the current nor the verified September 10 generator",
                rig.id
            ));
        }
        for (index, mut replacement) in indices.into_iter().zip(current) {
            replacement.material = document.objects[index].material.clone();
            replacements.push((index, replacement));
        }
        migrated.push(rig.id.clone());
    }
    for (index, replacement) in replacements {
        document.objects[index] = replacement;
    }
    Ok(migrated)
}

pub fn validate(document: &Document) -> Result<(), String> {
    if document.faces.len() > MAX_FACES {
        return Err("too many facial rigs".into());
    }
    let mut face_ids = BTreeSet::new();
    let mut characters = BTreeSet::new();
    let mut owned = BTreeSet::new();
    for rig in &document.faces {
        identifier(&rig.id)?;
        identifier(&rig.character)?;
        rig.controls.validate()?;
        if !face_ids.insert(&rig.id) || !characters.insert(&rig.character) {
            return Err("duplicate facial rig or character".into());
        }
        if rig.generated_object_ids != ids(&rig.id) {
            return Err("facial generated object ids do not match their stable contract".into());
        }
        sources(document, rig)?;
        let expected = rest_objects(document, rig)?;
        for (id, want) in rig.generated_object_ids.iter().zip(expected) {
            if !owned.insert(id) {
                return Err("facial objects cannot be shared between rigs".into());
            }
            let got = &document.objects[index(document, id)?];
            if !derived_matches(got, &want)? {
                return Err(format!("facial object {id} has stale or edited derived geometry"));
            }
            if document.joints.iter().any(|j| j.objects.contains(id)) {
                return Err(
                    "facial generated geometry follows the head and cannot have an independent joint binding".into()
                );
            }
            if document
                .clips
                .iter()
                .any(|c| c.tracks.iter().any(|t| matches!(&t.target,Target::Object{id:target} if target==id)))
            {
                return Err("animate facial controls instead of generated object transforms".into());
            }
        }
    }
    Ok(())
}

pub fn validate_tracks(document: &Document, clip: &Clip) -> Result<(), String> {
    if clip.face_tracks.len() > MAX_FACES * FACE_CHANNELS {
        return Err("clip exceeds facial track budget".into());
    }
    let mut channels = BTreeSet::new();
    for track in &clip.face_tracks {
        if !document.faces.iter().any(|f| f.id == track.face) {
            return Err(format!("facial track references missing face {}", track.face));
        }
        if !channels.insert((&track.face, track.channel)) {
            return Err("clip has duplicate facial control channels".into());
        }
        crate::animation::key_times(track.keys.iter().map(|k| k.time), clip.duration)?;
        for key in &track.keys {
            track.channel.validate(key.value)?;
        }
    }
    Ok(())
}

fn time<'a>(doc: &'a Document, sample: Option<&AnimationSample>) -> Result<Option<(&'a Clip, f32)>, String> {
    let Some(sample) = sample else { return Ok(None) };
    range(sample.time, -86400.0, 86400.0, "animation time")?;
    let clip = doc.clips.iter().find(|c| c.id == sample.clip).ok_or_else(|| format!("missing clip {}", sample.clip))?;
    let t = match sample.playback {
        Playback::Clamp => sample.time.clamp(0.0, clip.duration),
        Playback::Loop => sample.time.rem_euclid(clip.duration),
    };
    Ok(Some((clip, t)))
}
fn layered_controls(doc: &Document, at: Option<(&Clip, f32)>) -> Result<Option<crate::layering::Resolved>, String> {
    at.filter(|(clip, _)| !clip.layers.is_empty())
        .map(|(clip, time)| crate::layering::resolve(doc, clip, time))
        .transpose()
}
fn controls(rig: &FaceRig, at: Option<(&Clip, f32)>, resolved: Option<&crate::layering::Resolved>) -> FaceControls {
    let mut result = rig.controls.clone();
    if let Some(resolved) = resolved {
        for ((face, channel), &value) in &resolved.faces {
            if face == &rig.id {
                result.set(*channel, value);
            }
        }
    } else if let Some((clip, time)) = at {
        for t in &clip.face_tracks {
            if t.face == rig.id {
                let mut track = Track::new(t.easing.engine());
                for key in &t.keys {
                    track = track.key(key.time, key.value);
                }
                result.set(t.channel, track.sample(time));
            }
        }
    }
    result
}

#[derive(Clone, Copy)]
enum RestGeometry {
    Current,
    September10,
}

struct Mouth {
    width: f32,
    half_height: f32,
    center_y: f32,
    cavity: CsgExpr,
    cavity_open: bool,
    interior: CsgExpr,
    lips: [CsgExpr; 2],
}
fn translated(shape: CsgExpr, position: V3) -> CsgExpr {
    CsgExpr::Transform { shape: Box::new(shape), position, rotation_degrees: [0.0; 3], scale: 1.0 }
}
fn ellipse_surface(r: V3, x: f32, y: f32) -> f32 {
    r[2] * (1.0 - (x / r[0]).powi(2) - (y / r[1]).powi(2)).max(0.02).sqrt()
}
fn mouth(r: V3, c: &FaceControls) -> Mouth {
    mouth_for_geometry(r, c, RestGeometry::Current)
}
fn mouth_for_geometry(r: V3, c: &FaceControls, geometry: RestGeometry) -> Mouth {
    let thickness = r[1] * 0.022;
    let width = r[0] * 0.38 * (1.0 + 0.35 * c.lip_wide) * (1.0 - 0.45 * c.lip_round);
    let opening = match geometry {
        RestGeometry::Current => 1.0 - c.lip_seal,
        RestGeometry::September10 => 1.0 - 0.85 * c.lip_seal,
    };
    let half_height = r[1] * 0.18 * c.jaw_open * opening;
    let center_y = -0.35 * r[1] - 0.65 * half_height;
    let lift = c.smile * 0.07 * r[1];
    let front = ellipse_surface(r, 0.0, center_y);
    let depth = r[2] * 0.42;
    let cavity_height = (half_height - thickness * 0.65).max(thickness * 0.15);
    let cavity = translated(
        CsgExpr::primitive(Shape::Ellipsoid {
            radii: [(width - thickness * 0.45).max(thickness), cavity_height, depth],
        }),
        [0.0, center_y, front - depth * 0.25],
    );
    // The interior can be very thin while the mouth is closed. Use an exact rounded-box
    // distance: an extremely flat approximate ellipsoid can produce tiny positive fields
    // far outside its surface and make the marcher report phantom mouth streaks.
    let interior_half = [width * 0.92, cavity_height * 0.95, depth * 0.10];
    let interior = translated(
        CsgExpr::primitive(Shape::RoundBox {
            half_extents: interior_half,
            radius: interior_half.iter().copied().fold(f32::INFINITY, f32::min) * 0.5,
        }),
        [0.0, center_y, front - depth * 1.05],
    );
    let lips = std::array::from_fn(|side| {
        let points: Vec<V3> = (0..=24)
            .map(|i| {
                let angle = (i as f32 / 24.0 + side as f32) * std::f32::consts::PI;
                let x = width * angle.cos();
                let y = center_y + (half_height + 0.30 * thickness) * angle.sin() + lift * (x / width).powi(2);
                [x, y, ellipse_surface(r, x, y) + thickness * 0.35 + c.lip_round * r[2] * 0.11]
            })
            .collect();
        let mut curve = CsgExpr::primitive(Shape::Capsule { a: points[0], b: points[1], radius: thickness });
        for p in points[1..].windows(2) {
            curve = CsgExpr::SmoothUnion {
                a: Box::new(curve),
                b: Box::new(CsgExpr::primitive(Shape::Capsule { a: p[0], b: p[1], radius: thickness })),
                radius: thickness * 0.25,
            };
        }
        curve
    });
    Mouth { width, half_height, center_y, cavity, cavity_open: half_height > thickness * 0.70, interior, lips }
}
fn lid(radius: f32, blink: f32, brow: f32, upper: bool) -> CsgExpr {
    lid_for_geometry(radius, blink, brow, upper, RestGeometry::Current)
}
fn lid_for_geometry(radius: f32, blink: f32, brow: f32, upper: bool, geometry: RestGeometry) -> CsgExpr {
    let shell = CsgExpr::Shell {
        shape: Box::new(CsgExpr::primitive(Shape::Sphere { radius: radius * 1.06 })),
        thickness: radius * 0.055,
    };
    let brow = match geometry {
        RestGeometry::Current => brow * (1.0 - blink),
        RestGeometry::September10 => brow,
    };
    let cut = if upper {
        (1.0 - blink) * 0.8 * radius - blink * 0.15 * radius + brow * 0.18 * radius
    } else {
        -(1.0 - blink) * 0.5 * radius - blink * 0.15 * radius + brow * 0.04 * radius
    };
    let (normal, offset) = if upper { ([0.0, -1.0, 0.0], cut) } else { ([0.0, 1.0, 0.0], -cut) };
    CsgExpr::Intersect { a: Box::new(shell), b: Box::new(CsgExpr::primitive(Shape::Plane { normal, offset })) }
}

fn rest_objects(doc: &Document, rig: &FaceRig) -> Result<Vec<Entity>, String> {
    rest_objects_for_geometry(doc, rig, RestGeometry::Current)
}
fn rest_objects_for_geometry(doc: &Document, rig: &FaceRig, geometry: RestGeometry) -> Result<Vec<Entity>, String> {
    let source = sources(doc, rig)?;
    let c = &rig.controls;
    c.validate()?;
    let mouth = mouth_for_geometry(source.radii, c, geometry);
    let mut entities = Vec::new();
    for (part, role) in PARTS.iter().enumerate() {
        let (expression, placement, material) = if part < 4 {
            let side = part / 2;
            let eye = &doc.objects[source.eyes[side]];
            let Shape::Sphere { radius } = eye.shape else { unreachable!() };
            (
                lid_for_geometry(
                    radius,
                    if side == 0 { c.blink_left } else { c.blink_right },
                    if side == 0 { c.brow_left } else { c.brow_right },
                    part % 2 == 0,
                    geometry,
                ),
                eye,
                source.head_entity.material.clone(),
            )
        } else if part < 6 {
            (
                mouth.lips[part - 4].clone(),
                source.head_entity,
                Surface { albedo: [0.42, 0.16, 0.18], roughness: 0.55, ..Surface::default() },
            )
        } else {
            (
                mouth.interior.clone(),
                source.head_entity,
                Surface { albedo: [0.055, 0.009, 0.014], roughness: 0.9, ..Surface::default() },
            )
        };
        expression.compile()?;
        entities.push(Entity {
            id: format!("{}/{role}", rig.id),
            label: role.replace('_', " "),
            role: (*role).into(),
            group: rig.character.clone(),
            shape: Shape::Csg { expression: Box::new(expression) },
            position: placement.position,
            rotation_degrees: placement.rotation_degrees,
            scale: placement.scale,
            material,
            combine: Combination::Union,
            modifiers: Modifiers::default(),
        });
    }
    Ok(entities)
}

fn replace(scene: &mut Scene, object: usize, expression: Expr) -> Result<(), String> {
    expression.validate()?;
    if let Prim::Csg { id } = scene.objects[object].prim {
        scene.csgs[id as usize] = expression;
    } else {
        scene.objects[object].prim = Prim::Csg { id: scene.csg(expression)? };
    }
    Ok(())
}

/// Derive exactly one pose from authored controls and source shapes. Call after body animation;
/// repeated and out-of-order evaluations never accumulate deformation.
pub fn refresh(doc: &Document, scene: &mut Scene, sample: Option<&AnimationSample>) -> Result<(), String> {
    let at = time(doc, sample)?;
    let resolved = layered_controls(doc, at)?;
    for rig in &doc.faces {
        let source = sources(doc, rig)?;
        let c = controls(rig, at, resolved.as_ref());
        c.validate()?;
        let m = mouth(source.radii, &c);
        let head_rest = transform(source.head_entity);
        let head_pose = scene.objects[source.head].xform;
        let base = from_shape(&source.head_entity.shape)?;
        replace(
            scene,
            source.head,
            if m.cavity_open { Expr::Subtract { a: Box::new(base), b: Box::new(m.cavity.compile()?) } } else { base },
        )?;
        for (side, &eye_index) in source.eyes.iter().enumerate() {
            let eye = &doc.objects[eye_index];
            let Shape::Sphere { radius } = eye.shape else { unreachable!() };
            // Head and eyes share an ancestor binding. Compose any explicit eye-object track
            // inside the exact posed head, so a head-object track is a real facial parent.
            let mut eye_rest = transform(eye);
            if let Some(resolved) = &resolved {
                if let Some(delta) = resolved.transforms.get(&Target::Object { id: eye.id.clone() }) {
                    eye_rest = delta.compose(eye_rest);
                }
            } else if let Some((clip, t)) = at {
                if let Some(track) =
                    clip.tracks.iter().find(|tr| matches!(&tr.target,Target::Object{id} if id==&eye.id))
                {
                    let mut translations = Track::new(track.easing.engine());
                    let mut rotations = Track::new(track.easing.engine());
                    let mut scales = Track::new(track.easing.engine());
                    for key in &track.keys {
                        let r = key.rotation_degrees.map(f32::to_radians);
                        translations = translations.key(key.time, vec(key.translation));
                        rotations = rotations.key(key.time, Quat::from_euler(r[0], r[1], r[2]));
                        scales = scales.key(key.time, key.scale);
                    }
                    eye_rest = Transform::around_pivot(
                        vec(track.pivot.unwrap_or(eye.position)),
                        rotations.sample(t),
                        scales.sample(t),
                        translations.sample(t),
                    )
                    .compose(eye_rest);
                }
            }
            // Zero gaze is the authored transform exactly. Map nonzero gaze as a vector:
            // adding then subtracting the head position loses low bits under translation.
            if c.gaze_x != 0.0 || c.gaze_y != 0.0 {
                let gaze_local = vec([c.gaze_x * radius * 0.16, c.gaze_y * radius * 0.12, 0.0]);
                eye_rest.pos = eye_rest.pos + head_rest.rot.mul_vec(gaze_local.scale(head_rest.scale));
            }
            let eye_pose = if head_pose.pos == head_rest.pos
                && head_pose.rot.cols == head_rest.rot.cols
                && head_pose.scale == head_rest.scale
            {
                eye_rest
            } else {
                head_pose.compose(relative(head_rest, eye_rest))
            };
            range(eye_pose.scale, 1e-8, 1e8, "evaluated facial scale")?;
            for v in [eye_pose.pos.x, eye_pose.pos.y, eye_pose.pos.z] {
                range(v, -1e8, 1e8, "evaluated facial position")?;
            }
            scene.objects[eye_index].xform = eye_pose;
            for upper in [true, false] {
                let part = side * 2 + usize::from(!upper);
                let i = index(doc, &rig.generated_object_ids[part])?;
                replace(
                    scene,
                    i,
                    lid(
                        radius,
                        if side == 0 { c.blink_left } else { c.blink_right },
                        if side == 0 { c.brow_left } else { c.brow_right },
                        upper,
                    )
                    .compile()?,
                )?;
                scene.objects[i].xform = eye_pose;
            }
        }
        for part in 4..7 {
            let i = index(doc, &rig.generated_object_ids[part])?;
            replace(scene, i, if part < 6 { m.lips[part - 4].compile()? } else { m.interior.compile()? })?;
            scene.objects[i].xform = head_pose;
        }
    }
    Ok(())
}

pub fn inspect_controls(doc: &Document, sample: Option<&AnimationSample>) -> Result<Value, String> {
    let at = time(doc, sample)?;
    let resolved = layered_controls(doc, at)?;
    let mut faces = vec![];
    for rig in &doc.faces {
        let source = sources(doc, rig)?;
        let c = controls(rig, at, resolved.as_ref());
        c.validate()?;
        let m = mouth(source.radii, &c);
        faces.push(json!({"id":rig.id,"character":rig.character,"controls":c,"generated_object_ids":rig.generated_object_ids,
            "mouth":{"curve_half_width":m.width,"curve_half_height":m.half_height,"center_y":m.center_y,"cavity_open":m.cavity_open}}));
    }
    Ok(json!({"faces":faces,"animation":sample,"time":at.map(|(_,t)|t),
        "mechanism":"physical SDF eyelid shells, procedural lip curves and head-local mouth cavity",
        "limitations":"gaze translates eyes and lids; brow adjusts lid cuts; no eyeball aim rotation, eyebrow geometry, weighted facial skinning, anatomical jaw hinge, teeth/tongue or automatic audio alignment"}))
}
