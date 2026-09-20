//! Continuous, separately selectable fitted garments over the authored SDF character.
//!
//! Garments are regenerated from explicit source IDs at each evaluated pose. They do not
//! simulate fabric, strain, inertia, seams or self-collision. Thickness and clearance are
//! world-space meters; source transforms can animate without double-binding the garment.
use crate::{
    animation::{AnimationSample, Target},
    csg::{self, CsgExpr},
    model::{array, identifier, range, vec, Combination, Document, Entity, Modifiers, Pass, Shape, Surface, V3},
};
use mm3e_kit::{
    csg::Expr,
    vec::{Mat3, Transform, Vec3},
};
use mm3e_orchestrator::{Prim, Scene};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;

const BODY_ROLES: &[&str] = &[
    "pelvis",
    "abdomen",
    "chest",
    "neck",
    "head",
    "left_thigh",
    "left_shin",
    "left_foot",
    "left_upper_arm",
    "left_forearm",
    "left_hand",
    "right_thigh",
    "right_shin",
    "right_foot",
    "right_upper_arm",
    "right_forearm",
    "right_hand",
];
const FIT_MODE: &str = "continuous_sdf_follow_source";
pub const MAX_GARMENTS: usize = 16;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GarmentStyle {
    Vest,
    ShortSleeveTop,
    Trousers,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GarmentRequest {
    pub id: String,
    pub character: String,
    pub style: GarmentStyle,
    /// World meters, measured on the authored body scalar field (not an exact Euclidean metric).
    pub clearance_m: f32,
    /// Nominal shell field-band width in world meters; CSG takes half this value.
    pub thickness_m: f32,
    #[serde(default)]
    pub material: Surface,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Garment {
    pub id: String,
    pub character: String,
    pub style: GarmentStyle,
    pub clearance_m: f32,
    pub thickness_m: f32,
    /// Explicit physical body inputs in their authored CSG order. Eyes/facial details are excluded.
    pub source_object_ids: Vec<String>,
    /// Source-local openings follow these objects' exact evaluated transforms.
    pub opening_source_ids: Vec<String>,
    pub fit_mode: String,
}

/// Internal recipe nodes preserve exact posed matrices while serializing the rest document's
/// original Euler transforms. No matrix-to-Euler round trip enters animation evaluation.
#[derive(Clone)]
enum Recipe {
    Shape(Shape),
    BodySource { index: usize, shape: Shape },
    Source { index: usize, shape: Box<Recipe> },
    Union(Box<Recipe>, Box<Recipe>),
    Intersect(Box<Recipe>, Box<Recipe>),
    Subtract(Box<Recipe>, Box<Recipe>),
    SmoothUnion(Box<Recipe>, Box<Recipe>, f32),
    Offset(Box<Recipe>, f32),
    Shell(Box<Recipe>, f32),
}
impl Recipe {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
    fn offset(self, distance: f32) -> Self {
        Self::Offset(self.boxed(), distance)
    }
    fn subtract(self, other: Self) -> Self {
        Self::Subtract(self.boxed(), other.boxed())
    }
    fn rest(&self, document: &Document) -> CsgExpr {
        let child = |node: &Recipe| Box::new(node.rest(document));
        match self {
            Self::Shape(shape) => CsgExpr::Primitive { shape: Box::new(shape.clone()) },
            Self::BodySource { index, shape } => {
                let source = &document.objects[*index];
                CsgExpr::Transform {
                    shape: Box::new(CsgExpr::Primitive { shape: Box::new(shape.clone()) }),
                    position: source.position,
                    rotation_degrees: source.rotation_degrees,
                    scale: source.scale,
                }
            }
            Self::Source { index, shape } => {
                let source = &document.objects[*index];
                CsgExpr::Transform {
                    shape: child(shape),
                    position: source.position,
                    rotation_degrees: source.rotation_degrees,
                    scale: source.scale,
                }
            }
            Self::Union(a, b) => CsgExpr::Union { a: child(a), b: child(b) },
            Self::Intersect(a, b) => CsgExpr::Intersect { a: child(a), b: child(b) },
            Self::Subtract(a, b) => CsgExpr::Subtract { a: child(a), b: child(b) },
            Self::SmoothUnion(a, b, radius) => CsgExpr::SmoothUnion { a: child(a), b: child(b), radius: *radius },
            Self::Offset(shape, distance) => CsgExpr::Offset { shape: child(shape), distance: *distance },
            Self::Shell(shape, thickness) => CsgExpr::Shell { shape: child(shape), thickness: *thickness },
        }
    }
    fn posed(&self, scene: &Scene) -> Result<Expr, String> {
        let child = |node: &Recipe| node.posed(scene).map(Box::new);
        Ok(match self {
            Self::Shape(shape) => csg::from_shape(shape)?,
            Self::BodySource { index, .. } => {
                let object = scene.objects.get(*index).ok_or("garment scene/source index mismatch")?;
                // Facial controls can replace a source head's compiled primitive with an
                // isolated cavity while retaining its authored ellipsoid. Fit against the
                // actually evaluated source field, not its undeformed authoring snapshot.
                let shape = match object.prim {
                    Prim::Csg { id } => {
                        scene.csgs.get(id as usize).ok_or("garment source CSG index is invalid")?.clone()
                    }
                    Prim::Capsule { a, b, r } => Expr::Capsule { a, b, r },
                    Prim::Ellipsoid { r } => Expr::Ellipsoid { r },
                    _ => return Err("unsupported evaluated garment source primitive".into()),
                };
                Expr::Transform { shape: Box::new(shape), xform: object.xform }
            }
            Self::Source { index, shape } => Expr::Transform {
                shape: child(shape)?,
                xform: scene.objects.get(*index).ok_or("garment scene/source index mismatch")?.xform,
            },
            Self::Union(a, b) => Expr::Union { a: child(a)?, b: child(b)? },
            Self::Intersect(a, b) => Expr::Intersect { a: child(a)?, b: child(b)? },
            Self::Subtract(a, b) => Expr::Subtract { a: child(a)?, b: child(b)? },
            Self::SmoothUnion(a, b, k) => Expr::SmoothUnion { a: child(a)?, b: child(b)?, k: *k },
            Self::Offset(shape, distance) => Expr::Offset { shape: child(shape)?, distance: *distance },
            Self::Shell(shape, thickness) => Expr::Shell { shape: child(shape)?, thickness: *thickness },
        })
    }
}

fn opening_roles(style: GarmentStyle) -> Vec<&'static str> {
    match style {
        GarmentStyle::Vest | GarmentStyle::ShortSleeveTop => {
            vec!["abdomen", "neck", "left_upper_arm", "right_upper_arm"]
        }
        GarmentStyle::Trousers => vec!["pelvis", "left_shin", "right_shin"],
    }
}
fn covered_roles(style: GarmentStyle) -> Vec<&'static str> {
    match style {
        GarmentStyle::Vest => vec!["abdomen", "chest"],
        GarmentStyle::ShortSleeveTop => vec!["abdomen", "chest", "left_upper_arm", "right_upper_arm"],
        GarmentStyle::Trousers => vec!["pelvis", "left_thigh", "left_shin", "right_thigh", "right_shin"],
    }
}
fn source_ids(document: &Document, character: &str) -> Result<Vec<String>, String> {
    let expected: BTreeSet<String> = BODY_ROLES.iter().map(|role| format!("{character}/{role}")).collect();
    let found: Vec<String> =
        document.objects.iter().filter(|e| expected.contains(&e.id)).map(|e| e.id.clone()).collect();
    if found.len() != expected.len() {
        return Err(format!(
            "garment creation requires the complete named humanoid body {character} (17 physical parts)"
        ));
    }
    Ok(found)
}
fn index(document: &Document, garment: &Garment, role: &str) -> Result<usize, String> {
    let id = format!("{}/{role}", garment.character);
    document.objects.iter().position(|e| e.id == id).ok_or_else(|| format!("missing garment source {id}"))
}
fn source(document: &Document, index: usize) -> Recipe {
    Recipe::BodySource { index, shape: document.objects[index].shape.clone() }
}
fn local(index: usize, shape: Recipe) -> Recipe {
    Recipe::Source { index, shape: shape.boxed() }
}
fn fold(document: &Document, ids: &[String]) -> Result<Recipe, String> {
    let mut result = None;
    for id in ids {
        let i =
            document.objects.iter().position(|e| e.id == *id).ok_or_else(|| format!("missing garment source {id}"))?;
        let next = source(document, i);
        result = Some(match (result, &document.objects[i].combine) {
            (None, _) => next,
            (Some(previous), Combination::Union) => Recipe::Union(Box::new(previous), next.boxed()),
            (Some(previous), Combination::Smooth { radius }) => {
                Recipe::SmoothUnion(Box::new(previous), next.boxed(), *radius)
            }
            (_, Combination::Subtract) => {
                return Err("subtractive source body objects are not supported by fitted garments".into())
            }
        });
    }
    result.ok_or_else(|| "garment needs body sources".into())
}
fn capsule(document: &Document, i: usize) -> Result<(Vec3, Vec3, f32), String> {
    match document.objects[i].shape {
        Shape::Capsule { a, b, radius } => Ok((vec(a), vec(b), radius)),
        _ => Err(format!("garment source {} must remain a capsule", document.objects[i].id)),
    }
}
fn radii(document: &Document, i: usize) -> Result<V3, String> {
    match document.objects[i].shape {
        Shape::Ellipsoid { radii } => Ok(radii),
        _ => Err(format!("garment source {} must remain an ellipsoid", document.objects[i].id)),
    }
}

struct Geometry {
    garment: Recipe,
    body: Recipe,
    covered: Recipe,
    covered_indices: Vec<usize>,
}
fn geometry(document: &Document, garment: &Garment, scene: Option<&Scene>) -> Result<Geometry, String> {
    let roles = covered_roles(garment.style);
    let covered_ids: Vec<String> = garment
        .source_object_ids
        .iter()
        .filter(|id| roles.iter().any(|role| **id == format!("{}/{role}", garment.character)))
        .cloned()
        .collect();
    let covered_indices: Vec<usize> =
        roles.iter().map(|role| index(document, garment, role)).collect::<Result<_, _>>()?;
    let body = fold(document, &garment.source_object_ids)?;
    let covered = fold(document, &covered_ids)?;
    let mut field = Recipe::Shell(
        covered.clone().offset(garment.clearance_m + garment.thickness_m * 0.5).boxed(),
        garment.thickness_m * 0.5,
    );
    // Other physical parts and the original smooth unions can protrude beyond the selected
    // coverage field. Carve the expanded WHOLE body locally, without modifying the character.
    field = field.subtract(body.clone().offset(garment.clearance_m));
    let padding = garment.clearance_m + garment.thickness_m + 0.002;
    let scale = |i: usize| scene.map(|s| s.objects[i].xform.scale).unwrap_or(document.objects[i].scale);
    let clip = |i: usize, normal: Vec3, offset: f32| {
        let plane = Recipe::Shape(Shape::Plane { normal: array(normal), offset });
        // Restrict every opening to its source neighborhood. A global plane would slice a
        // raised opposite leg or another garment branch during a pose.
        let mask = Recipe::Shape(document.objects[i].shape.clone()).offset(padding / scale(i));
        local(i, Recipe::Intersect(plane.boxed(), mask.boxed()))
    };
    match garment.style {
        GarmentStyle::Vest | GarmentStyle::ShortSleeveTop => {
            let abdomen = index(document, garment, "abdomen")?;
            let r = radii(document, abdomen)?;
            let hem_y = -0.45 * r[1];
            field = field.subtract(clip(abdomen, Vec3::new(0.0, 1.0, 0.0), -hem_y));
            let neck = index(document, garment, "neck")?;
            field = field.subtract(source(document, neck).offset(padding));
            for role in ["left_upper_arm", "right_upper_arm"] {
                let i = index(document, garment, role)?;
                let (a, b, radius) = capsule(document, i)?;
                let axis = (b - a).normalize();
                if garment.style == GarmentStyle::Vest {
                    let cutter = Shape::Capsule {
                        a: array(a - axis.scale(radius)),
                        b: array(b),
                        radius: radius + padding / scale(i),
                    };
                    field = field.subtract(local(i, Recipe::Shape(cutter)));
                } else {
                    let cuff = a + (b - a).scale(0.65);
                    field = field.subtract(clip(i, -axis, axis.dot(cuff)));
                }
            }
        }
        GarmentStyle::Trousers => {
            let pelvis = index(document, garment, "pelvis")?;
            let r = radii(document, pelvis)?;
            field = field.subtract(clip(pelvis, Vec3::new(0.0, -1.0, 0.0), 0.60 * r[1]));
            for role in ["left_shin", "right_shin"] {
                let i = index(document, garment, role)?;
                let (a, b, _) = capsule(document, i)?;
                let axis = (b - a).normalize();
                let cuff = a + (b - a).scale(0.94);
                field = field.subtract(clip(i, -axis, axis.dot(cuff)));
            }
        }
    }
    Ok(Geometry { garment: field, body, covered, covered_indices })
}

pub fn validate(document: &Document) -> Result<(), String> {
    if document.garments.len() > MAX_GARMENTS {
        return Err(format!("at most {MAX_GARMENTS} garments are supported"));
    }
    let mut ids = BTreeSet::new();
    for garment in &document.garments {
        identifier(&garment.id)?;
        identifier(&garment.character)?;
        if !ids.insert(&garment.id) {
            return Err(format!("duplicate garment id {}", garment.id));
        }
        range(garment.clearance_m, 0.0001, 0.25, "garment clearance_m")?;
        range(garment.thickness_m, 0.0005, 0.10, "garment thickness_m")?;
        if garment.fit_mode != FIT_MODE {
            return Err(format!("unsupported garment fit mode {}", garment.fit_mode));
        }
        if garment.source_object_ids != source_ids(document, &garment.character)? {
            return Err("garment source IDs must contain the named physical body in authored CSG order".into());
        }
        let openings: Vec<String> =
            opening_roles(garment.style).iter().map(|role| format!("{}/{role}", garment.character)).collect();
        if garment.opening_source_ids != openings {
            return Err("garment opening sources do not match its style".into());
        }
        let entity = document
            .objects
            .iter()
            .find(|e| e.id == garment.id)
            .ok_or_else(|| format!("missing garment object {}", garment.id))?;
        let default_mods = serde_json::to_value(Modifiers::default()).map_err(|e| e.to_string())?;
        if entity.position != [0.0; 3]
            || entity.rotation_degrees != [0.0; 3]
            || entity.scale != 1.0
            || !matches!(entity.combine, Combination::Union)
            || !matches!(entity.shape, Shape::Csg { .. })
            || serde_json::to_value(&entity.modifiers).map_err(|e| e.to_string())? != default_mods
        {
            return Err("generated garment objects require neutral transforms, union and no modifiers; edit the source body or garment recipe".into());
        }
        if document.joints.iter().any(|joint| joint.objects.contains(&garment.id))
            || document.clips.iter().any(|clip| {
                clip.tracks.iter().any(|track| matches!(&track.target,Target::Object{id} if id==&garment.id))
            })
        {
            return Err("source-following garments cannot also be joint-bound or directly transform-animated".into());
        }
        for id in &garment.source_object_ids {
            let source = document.objects.iter().find(|e| e.id == *id).ok_or_else(|| format!("missing source {id}"))?;
            if !matches!(source.shape, Shape::Capsule { .. } | Shape::Ellipsoid { .. }) {
                return Err(format!("garment source {id} requires the named humanoid capsule/ellipsoid geometry"));
            }
            if serde_json::to_value(&source.modifiers).map_err(|e| e.to_string())? != default_mods {
                return Err(format!("garment source {id} has unsupported modifiers; bake/clear these before fitting"));
            }
            if matches!(source.combine, Combination::Subtract) {
                return Err(format!("garment source {id} cannot be subtractive"));
            }
        }
        let recipe = geometry(document, garment, None)?;
        let expression = recipe.garment.rest(document);
        expression.compile()?;
        let expected = Shape::Csg { expression: Box::new(expression) };
        if serde_json::to_value(&entity.shape).map_err(|e| e.to_string())?
            != serde_json::to_value(expected).map_err(|e| e.to_string())?
        {
            return Err(format!("garment {} has stale or edited derived geometry", garment.id));
        }
    }
    Ok(())
}

/// Add a complete, rendered, independently materialed garment. A failed creation leaves the
/// input document untouched, including failures in material validation or CSG budgets.
pub fn create(document: &mut Document, request: &GarmentRequest) -> Result<(), String> {
    identifier(&request.id)?;
    identifier(&request.character)?;
    if document.objects.iter().any(|e| e.id == request.id) {
        return Err(format!("object {} already exists", request.id));
    }
    let garment = Garment {
        id: request.id.clone(),
        character: request.character.clone(),
        style: request.style,
        clearance_m: request.clearance_m,
        thickness_m: request.thickness_m,
        source_object_ids: source_ids(document, &request.character)?,
        opening_source_ids: opening_roles(request.style)
            .iter()
            .map(|role| format!("{}/{role}", request.character))
            .collect(),
        fit_mode: FIT_MODE.into(),
    };
    range(request.clearance_m, 0.0001, 0.25, "garment clearance_m")?;
    range(request.thickness_m, 0.0005, 0.10, "garment thickness_m")?;
    let recipe = geometry(document, &garment, None)?;
    let expression = recipe.garment.rest(document);
    expression.compile()?;
    let mut candidate = document.clone();
    candidate.objects.push(Entity {
        id: request.id.clone(),
        label: format!("{} {:?}", request.character, request.style),
        role: "garment".into(),
        group: request.character.clone(),
        shape: Shape::Csg { expression: Box::new(expression) },
        position: [0.0; 3],
        rotation_degrees: [0.0; 3],
        scale: 1.0,
        material: request.material.clone(),
        combine: Combination::Union,
        modifiers: Modifiers::default(),
    });
    candidate.garments.push(garment);
    synchronize(&mut candidate)?;
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}

/// Replace a garment recipe in place. Stable entity order, ID and independent material
/// ownership survive; invalid source/fit/material changes leave the document untouched.
pub fn update(document: &mut Document, request: &GarmentRequest) -> Result<(), String> {
    identifier(&request.id)?;
    identifier(&request.character)?;
    range(request.clearance_m, 0.0001, 0.25, "garment clearance_m")?;
    range(request.thickness_m, 0.0005, 0.10, "garment thickness_m")?;
    let garment_index = document
        .garments
        .iter()
        .position(|g| g.id == request.id)
        .ok_or_else(|| format!("missing garment {}", request.id))?;
    let object_index = document
        .objects
        .iter()
        .position(|e| e.id == request.id)
        .ok_or_else(|| format!("missing garment object {}", request.id))?;
    let replacement = Garment {
        id: request.id.clone(),
        character: request.character.clone(),
        style: request.style,
        clearance_m: request.clearance_m,
        thickness_m: request.thickness_m,
        source_object_ids: source_ids(document, &request.character)?,
        opening_source_ids: opening_roles(request.style)
            .iter()
            .map(|role| format!("{}/{role}", request.character))
            .collect(),
        fit_mode: FIT_MODE.into(),
    };
    let mut candidate = document.clone();
    candidate.garments[garment_index] = replacement;
    candidate.objects[object_index].material = request.material.clone();
    candidate.objects[object_index].group = request.character.clone();
    synchronize(&mut candidate)?;
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}

/// Keep persisted rest snapshots in step with edits to source shapes and transforms. The
/// recipe metadata is authoritative; the generated CSG expression is a derived snapshot.
pub fn synchronize(document: &mut Document) -> Result<(), String> {
    let updates: Vec<_> = document
        .garments
        .iter()
        .map(|garment| {
            let expression = geometry(document, garment, None)?.garment.rest(document);
            expression.compile()?;
            Ok((garment.id.clone(), expression))
        })
        .collect::<Result<_, String>>()?;
    for (id, expression) in updates {
        let entity =
            document.objects.iter_mut().find(|e| e.id == id).ok_or_else(|| format!("missing garment object {id}"))?;
        entity.shape = Shape::Csg { expression: Box::new(expression) };
    }
    Ok(())
}

/// Replace only the compiled garment field, preserving its distinct material and identity.
/// Called after body pose evaluation and for the rest scene. No animation state is cached.
pub fn refresh(document: &Document, scene: &mut Scene) -> Result<(), String> {
    if scene.objects.len() != document.objects.len() {
        return Err("garment scene/document object mismatch".into());
    }
    let mut updates = Vec::with_capacity(document.garments.len());
    for garment in &document.garments {
        let i = document
            .objects
            .iter()
            .position(|e| e.id == garment.id)
            .ok_or_else(|| format!("missing garment object {}", garment.id))?;
        let Prim::Csg { id } = scene.objects[i].prim else {
            return Err("compiled garment must use local CSG".into());
        };
        let expression = geometry(document, garment, Some(scene))?.garment.posed(scene)?;
        expression.validate()?;
        if id as usize >= scene.csgs.len() {
            return Err("compiled garment CSG index is invalid".into());
        }
        updates.push((id as usize, expression, i));
    }
    for (id, expression, i) in updates {
        scene.csgs[id] = expression;
        scene.objects[i].xform = Transform::new(Vec3::ZERO, Mat3::IDENTITY, 1.0);
    }
    Ok(())
}

/// Finite geometric fit probes over the actual rendered pose. This is not continuous
/// collision detection, cloth simulation or a Euclidean-distance certificate.
pub fn inspect_fit(
    document: &Document,
    id: &str,
    animation: Option<&AnimationSample>,
    samples_per_source: usize,
) -> Result<Value, String> {
    if !(16..=256).contains(&samples_per_source) {
        return Err("samples_per_source must be 16..256".into());
    }
    let garment = document.garments.iter().find(|g| g.id == id).ok_or_else(|| format!("missing garment {id}"))?;
    let (scene, _) = document.compile_at(&Pass::Beauty, animation)?;
    let geometry = geometry(document, garment, Some(&scene))?;
    let body = geometry.body.posed(&scene)?;
    let covered = geometry.covered.posed(&scene)?;
    let cloth = geometry.garment.posed(&scene)?;
    let target = garment.clearance_m + garment.thickness_m * 0.5;
    // These are geometric search bounds, NOT acceleration-field underestimates. The
    // approximate ellipsoid intentionally has no Expr::bound acceleration certificate.
    let mut low = Vec3::splat(f32::INFINITY);
    let mut high = Vec3::splat(f32::NEG_INFINITY);
    let mut blend_padding = 0.0;
    for i in &geometry.covered_indices {
        let (center, radius) = match document.objects[*i].shape {
            Shape::Capsule { a, b, radius } => {
                ((vec(a) + vec(b)).scale(0.5), (vec(a) - vec(b)).length() * 0.5 + radius)
            }
            Shape::Ellipsoid { radii } => (Vec3::ZERO, radii.into_iter().fold(0.0, f32::max)),
            _ => return Err("unsupported garment coverage shape".into()),
        };
        let xform = scene.objects[*i].xform;
        let center = xform.to_world(center);
        let radius = radius * xform.scale;
        low = low.min(center - Vec3::splat(radius));
        high = high.max(center + Vec3::splat(radius));
        if let Combination::Smooth { radius } = document.objects[*i].combine {
            blend_padding += radius * 0.25;
        }
    }
    let center = (low + high).scale(0.5);
    let radius = (high - low).length() * 0.5 + blend_padding + target * 2.0;
    let tolerance = (garment.thickness_m * 0.005).clamp(1e-6, 0.00005);
    let mut retained = 0usize;
    let mut samples = 0usize;
    let mut rejected = 0usize;
    let mut unbracketed = 0usize;
    let mut penetrations = 0usize;
    let mut clearance_violations = 0usize;
    let mut minimum = f32::INFINITY;
    let mut maximum_penetration = 0.0f32;
    let mut failures = vec![];
    for i in &geometry.covered_indices {
        let local_center = match document.objects[*i].shape {
            Shape::Capsule { a, b, .. } => (vec(a) + vec(b)).scale(0.5),
            _ => Vec3::ZERO,
        };
        let origin = scene.objects[*i].xform.to_world(local_center);
        let horizon = (origin - center).length() + radius + target + 0.01;
        for direction in 0..samples_per_source {
            let y = 1.0 - 2.0 * (direction as f32 + 0.5) / samples_per_source as f32;
            let angle = direction as f32 * 2.399_963_1;
            let radial = (1.0 - y * y).sqrt();
            let dir = Vec3::new(radial * angle.cos(), y, radial * angle.sin());
            let mut lo = 0.0;
            let mut hi = None;
            for step in 1..=96 {
                let distance = horizon * step as f32 / 96.0;
                if covered.distance(origin + dir.scale(distance)) >= target {
                    hi = Some(distance);
                    break;
                }
                lo = distance;
            }
            let Some(mut hi) = hi else {
                unbracketed += 1;
                continue;
            };
            for _ in 0..28 {
                let mid = (lo + hi) * 0.5;
                if covered.distance(origin + dir.scale(mid)) < target {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            let midpoint = origin + dir.scale((lo + hi) * 0.5);
            if cloth.distance(midpoint) >= -tolerance {
                rejected += 1;
                continue;
            }
            retained += 1;
            let h = (garment.thickness_m * 0.05).clamp(1e-6, 0.0001);
            let axis = [Vec3::new(h, 0.0, 0.0), Vec3::new(0.0, h, 0.0), Vec3::new(0.0, 0.0, h)];
            let gradient = Vec3::new(
                covered.distance(midpoint + axis[0]) - covered.distance(midpoint - axis[0]),
                covered.distance(midpoint + axis[1]) - covered.distance(midpoint - axis[1]),
                covered.distance(midpoint + axis[2]) - covered.distance(midpoint - axis[2]),
            )
            .normalize();
            if gradient.length() < 0.9 {
                unbracketed += 2;
                continue;
            }
            for side in [-1.0, 1.0] {
                let outside =
                    midpoint + gradient.scale(side * (garment.thickness_m * 2.0 + garment.clearance_m * 0.25));
                if cloth.distance(outside) <= 0.0 {
                    unbracketed += 1;
                    continue;
                }
                let mut inside = midpoint;
                let mut outside = outside;
                for _ in 0..28 {
                    let mid = (inside + outside).scale(0.5);
                    if cloth.distance(mid) < 0.0 {
                        inside = mid;
                    } else {
                        outside = mid;
                    }
                }
                let point = (inside + outside).scale(0.5);
                let clearance = body.distance(point);
                samples += 1;
                minimum = minimum.min(clearance);
                maximum_penetration = maximum_penetration.max((-clearance).max(0.0));
                let penetration = clearance < -tolerance;
                let violation = clearance < garment.clearance_m - tolerance;
                penetrations += usize::from(penetration);
                clearance_violations += usize::from(violation);
                if violation && failures.len() < 32 {
                    failures
                        .push(json!({"point":array(point),"body_field":clearance,"source":document.objects[*i].id}));
                }
            }
        }
    }
    Ok(json!({"id":id,"animation":animation,"fit_mode":FIT_MODE,
        "full_thickness_m":garment.thickness_m,"thickness_semantics":"Nominal shell field-band width; approximate body fields and clearance cuts can change local physical thickness.","requested_clearance_m":garment.clearance_m,"tolerance_m":tolerance,
        "directional_probes":geometry.covered_indices.len()*samples_per_source,"samples_per_source":samples_per_source,
        "retained_shell_midpoints":retained,"opening_or_occluded_probes":rejected,"unbracketed_probes":unbracketed,
        "surface_samples":samples,"minimum_body_field":if samples>0 {Some(minimum)} else {None},
        "body_penetration_samples":penetrations,"clearance_violation_samples":clearance_violations,
        "maximum_body_penetration":maximum_penetration,"violations":failures,
        "sampled_fit_pass":samples>0 && penetrations==0 && clearance_violations==0,
        "scope":"Finite deterministic inner/outer surface probes of the rendered garment against its explicitly selected body CSG. Values are authored scalar fields, not exact Euclidean distances. Openings and coverage are not exhaustively sampled. No fabric dynamics, cloth self-collision, inter-garment collision or continuous-time certificate."}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_body_uses_the_evaluated_facial_cavity_instead_of_the_rest_head() {
        let mut document = Document {
            objects: crate::character::humanoid("hero", 1.8, 1.0, 1.0, [0.0; 3]).unwrap(),
            ..Document::default()
        };
        create(
            &mut document,
            &GarmentRequest {
                id: "shirt".into(),
                character: "hero".into(),
                style: GarmentStyle::Vest,
                clearance_m: 0.01,
                thickness_m: 0.004,
                material: Surface::default(),
            },
        )
        .unwrap();
        crate::face::create(&mut document, &crate::face::FaceRequest { id: "face".into(), character: "hero".into() })
            .unwrap();
        document.faces[0].controls.jaw_open = 1.0;
        crate::face::synchronize(&mut document).unwrap();
        let (scene, _) = document.compile(&Pass::Beauty).unwrap();
        let garment = &document.garments[0];
        let recipe = geometry(&document, garment, Some(&scene)).unwrap();
        let evaluated = recipe.body.posed(&scene).unwrap();
        let rest = recipe.body.rest(&document).compile().unwrap();
        let point = Vec3::new(0.0, 1.55, 0.112);
        assert!(rest.distance(point) < -0.01, "negative control must lie inside the uncarved head");
        assert!(evaluated.distance(point) > 0.01, "evaluated mouth cavity must be absent from physical source body");
        let mut actual_body = Scene::new(1, 1);
        actual_body.csgs = scene.csgs.clone();
        for id in &garment.source_object_ids {
            let i = document.objects.iter().position(|e| e.id == *id).unwrap();
            actual_body.objects.push(scene.objects[i]);
        }
        assert!((actual_body.sample_authored(point).dist - evaluated.distance(point)).abs() < 1e-6);
    }
}
