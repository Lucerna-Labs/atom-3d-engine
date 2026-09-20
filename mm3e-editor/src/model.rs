//! The durable authoring document. Backend scenes are compiled from this data, never saved
//! through the legacy scene writer (which omits volumes and some rendering settings).

use mm3e_kit::{
    camera::Camera,
    color::Material,
    vec::{Mat3, Transform, Vec3},
    volume::SdfVolume,
};
use mm3e_orchestrator::{Combine, Light, Object, Prim, Quality, RenderMode, Scene};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub type V3 = [f32; 3];
pub const MAX_OBJECTS: usize = 512;
pub const MAX_VOLUME_SAMPLES: usize = 2_097_152;

pub fn vec(v: V3) -> Vec3 {
    Vec3::new(v[0], v[1], v[2])
}
pub fn array(v: Vec3) -> V3 {
    [v.x, v.y, v.z]
}
fn one() -> f32 {
    1.0
}
fn unit_scale() -> V3 {
    [1.0; 3]
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Shape {
    /// Original open-surface topology retained; the field is distance to triangles minus half-thickness.
    Surface {
        vertices: Vec<V3>,
        triangles: Vec<[u32; 3]>,
        thickness_m: f32,
    },
    /// This expression is evaluated locally before this entity enters the scene-wide fold.
    Csg {
        expression: Box<crate::csg::CsgExpr>,
    },
    Sphere {
        radius: f32,
    },
    Box {
        half_extents: V3,
    },
    RoundBox {
        half_extents: V3,
        radius: f32,
    },
    Torus {
        major: f32,
        minor: f32,
    },
    Cylinder {
        half_height: f32,
        radius: f32,
    },
    Capsule {
        a: V3,
        b: V3,
        radius: f32,
    },
    Cone {
        bottom_radius: f32,
        top_radius: f32,
        height: f32,
    },
    Ellipsoid {
        radii: V3,
    },
    Octahedron {
        size: f32,
    },
    HexPrism {
        radius: f32,
        half_height: f32,
    },
    Plane {
        normal: V3,
        offset: f32,
    },
    Volume {
        dims: [usize; 3],
        min: V3,
        cell: V3,
        samples: Vec<f32>,
    },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Combination {
    #[default]
    Union,
    Smooth {
        radius: f32,
    },
    Subtract,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Modifiers {
    pub mirror: [bool; 3],
    pub elongate: V3,
    pub round: f32,
    pub onion: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Surface {
    pub albedo: V3,
    pub metallic: f32,
    pub roughness: f32,
    pub reflectivity: f32,
    pub specular: f32,
    pub emissive: V3,
    pub checker: bool,
}
impl Default for Surface {
    fn default() -> Self {
        Self {
            albedo: [0.32, 0.65, 0.62],
            metallic: 0.0,
            roughness: 0.65,
            reflectivity: 0.0,
            specular: 0.5,
            emissive: [0.0; 3],
            checker: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Entity {
    /// Stable, caller-chosen identity. Array order does not define identity.
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub role: String,
    /// Selection metadata; this is not a transform parent or a CSG isolation boundary.
    #[serde(default)]
    pub group: String,
    pub shape: Shape,
    #[serde(default)]
    pub position: V3,
    #[serde(default)]
    pub rotation_degrees: V3,
    #[serde(default = "one")]
    pub scale: f32,
    #[serde(default)]
    pub material: Surface,
    #[serde(default)]
    pub combine: Combination,
    #[serde(default)]
    pub modifiers: Modifiers,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct View {
    pub eye: V3,
    pub target: V3,
    pub up: V3,
    pub fov_degrees: f32,
}
impl Default for View {
    fn default() -> Self {
        Self { eye: [3.0, 2.0, 5.0], target: [0.0, 0.9, 0.0], up: [0.0, 1.0, 0.0], fov_degrees: 38.0 }
    }
}
impl View {
    pub fn compile(&self) -> Camera {
        Camera::look_at(vec(self.eye), vec(self.target), vec(self.up), self.fov_degrees.to_radians())
    }
    pub fn validate(&self) -> Result<(), String> {
        vector(self.eye, "camera.eye")?;
        vector(self.target, "camera.target")?;
        vector(self.up, "camera.up")?;
        range(self.fov_degrees, 1.0, 175.0, "camera.fov_degrees")?;
        let forward = vec(self.target) - vec(self.eye);
        if forward.length() < 1e-5 || forward.normalize().cross(vec(self.up).normalize()).length() < 1e-5 {
            return Err("camera must have distinct eye/target and a nonparallel, nonzero up vector".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Illumination {
    pub vector: V3,
    #[serde(default = "unit_scale")]
    pub color: V3,
    #[serde(default)]
    pub directional: bool,
    #[serde(default)]
    pub radius: f32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Fidelity {
    Preview,
    #[default]
    Balanced,
    Full,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Pass {
    #[default]
    Beauty,
    Normal,
    Depth,
    Ao,
    Steps,
    Albedo,
}
impl Pass {
    pub fn compile(&self) -> RenderMode {
        match self {
            Self::Beauty => RenderMode::Beauty,
            Self::Normal => RenderMode::Normal,
            Self::Depth => RenderMode::Depth,
            Self::Ao => RenderMode::Ao,
            Self::Steps => RenderMode::Steps,
            Self::Albedo => RenderMode::Albedo,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    pub width: u32,
    pub height: u32,
    pub quality: Fidelity,
    pub exposure: f32,
    pub shadows: bool,
    pub ao: bool,
    pub fog_density: f32,
    pub sky_ambient: V3,
    /// Optional samples per image axis; total spatial samples are the square of this number.
    pub spatial_aa: Option<u32>,
    pub film: crate::shot::FilmSettings,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            width: 320,
            height: 320,
            quality: Fidelity::Balanced,
            exposure: 1.0,
            shadows: true,
            ao: true,
            fog_density: 0.0,
            sky_ambient: [0.35; 3],
            spatial_aa: None,
            film: crate::shot::FilmSettings::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Document {
    pub version: u32,
    /// Right-handed world: Y up, character faces +Z; lengths are meters.
    pub units: String,
    pub objects: Vec<Entity>,
    pub camera: View,
    pub settings: Settings,
    pub lights: Vec<Illumination>,
    #[serde(default)]
    pub garments: Vec<crate::garment::Garment>,
    #[serde(default)]
    pub faces: Vec<crate::face::FaceRig>,
    #[serde(default)]
    pub cloths: Vec<crate::cloth::ClothAsset>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deformers: Vec<crate::deform::DeformerAsset>,
    /// Additive fields: version-1 projects written before animation load with no joints/clips.
    #[serde(default)]
    pub joints: Vec<crate::animation::Joint>,
    #[serde(default)]
    pub clips: Vec<crate::animation::Clip>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audio: Vec<crate::audio::AudioAsset>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shots: Vec<crate::timeline::TimelineShot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub textures: Vec<crate::textures::TextureAsset>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uv_sets: Vec<crate::textures::UvSet>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub texture_bindings: Vec<crate::textures::TextureBinding>,
}
impl Default for Document {
    fn default() -> Self {
        Self {
            version: 1,
            units: "meters".into(),
            garments: vec![],
            faces: vec![],
            cloths: vec![],
            deformers: vec![],
            joints: vec![],
            clips: vec![],
            audio: vec![],
            shots: vec![],
            textures: vec![],
            uv_sets: vec![],
            texture_bindings: vec![],
            objects: vec![],
            camera: View::default(),
            settings: Settings::default(),
            lights: vec![
                Illumination { vector: [0.6, 0.8, 0.7], color: [2.4, 2.2, 1.9], directional: true, radius: 0.06 },
                Illumination { vector: [-0.6, 0.3, 0.2], color: [0.4, 0.6, 0.8], directional: true, radius: 0.1 },
            ],
        }
    }
}

pub fn range(v: f32, lo: f32, hi: f32, name: &str) -> Result<(), String> {
    if v.is_finite() && v >= lo && v <= hi {
        Ok(())
    } else {
        Err(format!("{name} must be finite and in [{lo}, {hi}]"))
    }
}
pub fn vector(v: V3, name: &str) -> Result<(), String> {
    for f in v {
        range(f, -10_000.0, 10_000.0, name)?;
    }
    Ok(())
}
pub fn identifier(id: &str) -> Result<(), String> {
    if id.is_empty() || id.len() > 128 || !id.bytes().all(|c| c.is_ascii_alphanumeric() || b"_-./".contains(&c)) {
        Err("id must contain 1..128 ASCII letters, digits, underscores, hyphens, dots or slashes".into())
    } else {
        Ok(())
    }
}

impl Shape {
    fn compile(&self, scene: &mut Scene) -> Result<Prim, String> {
        let positive = |v, name| range(v, 0.0001, 1000.0, name);
        let extents = |v: V3| -> Result<(), String> {
            for x in v {
                positive(x, "shape extent")?;
            }
            Ok(())
        };
        Ok(match self {
            Self::Surface { vertices, triangles, thickness_m } => {
                range(*thickness_m, 0.0001, 10.0, "surface thickness")?;
                for point in vertices {
                    vector(*point, "surface vertex")?;
                }
                let surface = mm3e_kit::surface::TriangleSurface::new(
                    vertices.iter().copied().map(vec).collect(),
                    triangles.clone(),
                    *thickness_m * 0.5,
                )?;
                Prim::Surface { id: scene.surface(surface)? }
            }
            Self::Csg { expression } => Prim::Csg { id: scene.csg(expression.compile()?)? },
            Self::Sphere { radius } => {
                positive(*radius, "radius")?;
                Prim::Sphere { r: *radius }
            }
            Self::Box { half_extents } => {
                extents(*half_extents)?;
                Prim::Box { half: vec(*half_extents) }
            }
            Self::RoundBox { half_extents, radius } => {
                extents(*half_extents)?;
                range(*radius, 0.0, half_extents.iter().copied().fold(f32::INFINITY, f32::min), "roundbox radius")?;
                Prim::RoundBox { half: vec(*half_extents), radius: *radius }
            }
            Self::Torus { major, minor } => {
                positive(*major, "major")?;
                positive(*minor, "minor")?;
                if minor > major {
                    return Err("torus minor radius must not exceed major radius".into());
                }
                Prim::Torus { major: *major, minor: *minor }
            }
            Self::Cylinder { half_height, radius } => {
                positive(*half_height, "half_height")?;
                positive(*radius, "radius")?;
                Prim::Cylinder { h: *half_height, r: *radius }
            }
            Self::Capsule { a, b, radius } => {
                vector(*a, "capsule.a")?;
                vector(*b, "capsule.b")?;
                positive(*radius, "radius")?;
                if (vec(*a) - vec(*b)).length() < 0.0001 {
                    return Err("capsule endpoints must differ".into());
                }
                Prim::Capsule { a: vec(*a), b: vec(*b), r: *radius }
            }
            Self::Cone { bottom_radius, top_radius, height } => {
                range(*bottom_radius, 0.0, 1000.0, "bottom_radius")?;
                range(*top_radius, 0.0, 1000.0, "top_radius")?;
                positive(*height, "height")?;
                if bottom_radius.max(*top_radius) == 0.0 || (bottom_radius - top_radius).abs() >= *height {
                    return Err("round cone needs a positive radius and abs(radius difference) < height".into());
                }
                Prim::Cone { r1: *bottom_radius, r2: *top_radius, h: *height }
            }
            Self::Ellipsoid { radii } => {
                extents(*radii)?;
                Prim::Ellipsoid { r: vec(*radii) }
            }
            Self::Octahedron { size } => {
                positive(*size, "size")?;
                Prim::Octahedron { s: *size }
            }
            Self::HexPrism { radius, half_height } => {
                positive(*radius, "radius")?;
                positive(*half_height, "half_height")?;
                Prim::HexPrism { r: *radius, h: *half_height }
            }
            Self::Plane { normal, offset } => {
                vector(*normal, "normal")?;
                range(*offset, -10_000.0, 10_000.0, "offset")?;
                if (vec(*normal).length() - 1.0).abs() > 0.0001 {
                    return Err("plane normal must be unit length".into());
                }
                Prim::Plane { n: vec(*normal), h: *offset }
            }
            Self::Volume { dims, min, cell, samples } => {
                let count = dims.iter().try_fold(1usize, |n, d| n.checked_mul(*d));
                if count != Some(samples.len()) || samples.len() > MAX_VOLUME_SAMPLES {
                    return Err("volume dimensions must match samples within the volume budget".into());
                }
                vector(*min, "volume.min")?;
                for c in cell {
                    range(*c, 0.000001, 1000.0, "volume.cell")?;
                }
                for value in samples {
                    range(*value, -1_000_000.0, 1_000_000.0, "volume.sample")?;
                }
                let volume = SdfVolume::new((dims[0], dims[1], dims[2]), vec(*min), vec(*cell), samples.clone())?;
                Prim::Volume { id: scene.volume(volume) }
            }
        })
    }
}

impl Document {
    /// Evaluate time without mutating authored data or editor history. None renders rest state.
    pub fn compile_at(
        &self,
        pass: &Pass,
        animation: Option<&crate::animation::AnimationSample>,
    ) -> Result<(Scene, Camera), String> {
        let (mut scene, camera) = self.compile_at_base(pass, animation)?;
        crate::cloth::refresh(self, &mut scene, animation)?;
        Ok((scene, camera))
    }

    /// Preserve scene indices and material ownership while evaluating only the selected
    /// cloth caches. Unselected unfinished cloth must not block a scoped body observation.
    pub fn compile_at_selected(
        &self,
        pass: &Pass,
        animation: Option<&crate::animation::AnimationSample>,
        selected: &BTreeSet<String>,
    ) -> Result<(Scene, Camera), String> {
        for id in selected {
            if !self.objects.iter().any(|object| &object.id == id) {
                return Err(format!("missing selected object {id}"));
            }
        }
        let (mut scene, camera) = self.compile_at_base(pass, animation)?;
        crate::cloth::refresh_selected(self, &mut scene, animation, selected)?;
        Ok((scene, camera))
    }

    pub(crate) fn compile_at_base(
        &self,
        pass: &Pass,
        animation: Option<&crate::animation::AnimationSample>,
    ) -> Result<(Scene, Camera), String> {
        let (mut scene, mut camera) = self.compile(pass)?;
        let evaluation = if let Some(sample) = animation {
            let evaluation = crate::animation::evaluate(self, &mut scene, &mut camera, sample)?;
            crate::face::refresh(self, &mut scene, Some(sample))?;
            Some(evaluation)
        } else {
            None
        };
        crate::deform::refresh(self, &mut scene, animation, evaluation.as_ref())?;
        if animation.is_some() {
            crate::garment::refresh(self, &mut scene)?;
        }
        Ok((scene, camera))
    }

    /// Validation and lowering share one path. Pose and checked-render evaluation
    /// additionally reject unsupported runtime geometry or appearance precision.
    pub fn compile(&self, pass: &Pass) -> Result<(Scene, Camera), String> {
        if self.version != 1 || self.units != "meters" {
            return Err("expected document version 1 and units meters".into());
        }
        if self.objects.len() > MAX_OBJECTS {
            return Err(format!("at most {MAX_OBJECTS} objects are supported"));
        }
        if self.lights.len() > 16 {
            return Err("at most 16 lights are supported".into());
        }
        crate::animation::validate(self)?;
        crate::dialogue::validate(self)?;
        crate::textures::validate(self)?;
        crate::deform::validate(self)?;
        crate::face::validate(self)?;
        crate::garment::validate(self)?;
        crate::cloth::validate(self)?;
        self.camera.validate()?;
        let s = &self.settings;
        if s.width == 0
            || s.height == 0
            || s.width > 8192
            || s.height > 8192
            || u64::from(s.width) * u64::from(s.height) > 16_777_216
        {
            return Err("render size must be 1..8192 per axis and at most 16777216 pixels".into());
        }
        range(s.exposure, 0.01, 32.0, "exposure")?;
        if s.spatial_aa.is_some_and(|aa| !(1..=8).contains(&aa)) {
            return Err("spatial_aa must be 1..8 samples per axis".into());
        }
        s.film.validate(crate::shot::effective_aa(self))?;
        range(s.fog_density, 0.0, 10.0, "fog_density")?;
        for x in s.sky_ambient {
            range(x, 0.0, 16.0, "sky_ambient")?;
        }
        let mut scene = Scene::new(s.width, s.height);
        let q = match s.quality {
            Fidelity::Preview => Quality::fast(s.width, s.height),
            Fidelity::Balanced => Quality::balanced(s.width, s.height),
            Fidelity::Full => Quality::full(s.width, s.height),
        };
        q.apply(&mut scene);
        if let Some(aa) = s.spatial_aa {
            scene.aa = aa;
        }
        scene.mode = pass.compile();
        scene.shadows = s.shadows;
        scene.ao = s.ao;
        scene.sky_ambient = vec(s.sky_ambient);
        scene.fog_density = s.fog_density;
        scene.post.exposure = s.exposure;
        scene.post.bloom = false;
        for l in &self.lights {
            vector(l.vector, "light.vector")?;
            for c in l.color {
                range(c, 0.0, 1000.0, "light.color")?;
            }
            range(l.radius, 0.0, 1000.0, "light.radius")?;
            if l.directional && vec(l.vector).length() < 1e-5 {
                return Err("directional light must have a direction".into());
            }
            let light = if l.directional {
                Light::directional(vec(l.vector), vec(l.color)).soft(l.radius)
            } else {
                Light::sphere(vec(l.vector), vec(l.color), l.radius)
            };
            scene.light(light);
        }
        let mut ids = BTreeSet::new();
        let mut volume_samples = 0usize;
        for (i, e) in self.objects.iter().enumerate() {
            identifier(&e.id)?;
            if !ids.insert(&e.id) {
                return Err(format!("duplicate object id {}", e.id));
            }
            if e.label.len() > 512 || e.role.len() > 128 || e.group.len() > 128 {
                return Err("object metadata exceeds its size limit".into());
            }
            let validate = || -> Result<(), String> {
                vector(e.position, "position")?;
                vector(e.rotation_degrees, "rotation_degrees")?;
                range(e.scale, 0.001, 1000.0, "scale")?;
                let m = &e.material;
                for c in m.albedo {
                    range(c, 0.0, 1.0, "albedo")?;
                }
                for c in m.emissive {
                    range(c, 0.0, 1000.0, "emissive")?;
                }
                for (name, value) in
                    [("metallic", m.metallic), ("reflectivity", m.reflectivity), ("specular", m.specular)]
                {
                    range(value, 0.0, 1.0, name)?;
                }
                range(m.roughness, 0.04, 1.0, "roughness")?;
                for x in e.modifiers.elongate {
                    range(x, 0.0, 1000.0, "elongate")?;
                }
                range(e.modifiers.round, 0.0, 1000.0, "round")?;
                range(e.modifiers.onion, 0.0, 1000.0, "onion")?;
                if let Combination::Smooth { radius } = e.combine {
                    range(radius, 0.0001, 1000.0, "smooth radius")?;
                }
                if i == 0 && !matches!(e.combine, Combination::Union) {
                    return Err("first object must use union, since it seeds the field".into());
                }
                Ok(())
            };
            validate().map_err(|err| format!("object {}: {err}", e.id))?;
            if let Shape::Volume { samples, .. } = &e.shape {
                volume_samples += samples.len();
                if volume_samples > MAX_VOLUME_SAMPLES {
                    return Err("document exceeds total volume sample budget".into());
                }
            }
            let prim = e.shape.compile(&mut scene).map_err(|err| format!("object {}: {err}", e.id))?;
            let m = &e.material;
            let mat = scene.material(Material {
                albedo: vec(m.albedo),
                metallic: m.metallic,
                roughness: m.roughness,
                reflectivity: m.reflectivity,
                specular: m.specular,
                emissive: vec(m.emissive),
                checker: m.checker,
            });
            let r = e.rotation_degrees.map(f32::to_radians);
            let mut object =
                Object::new(prim, Transform::new(vec(e.position), Mat3::from_euler(r[0], r[1], r[2]), e.scale), mat);
            object.combine = match e.combine {
                Combination::Union => Combine::Union,
                Combination::Smooth { radius } => Combine::Smooth(radius),
                Combination::Subtract => Combine::Subtract,
            };
            object.mods.mirror = e.modifiers.mirror;
            object.mods.elongate = vec(e.modifiers.elongate);
            object.mods.round = e.modifiers.round;
            object.mods.onion = e.modifiers.onion;
            scene.add(object);
        }
        crate::face::refresh(self, &mut scene, None)?;
        crate::garment::refresh(self, &mut scene)?;
        crate::textures::bind_scene(self, &mut scene)?;
        Ok((scene, self.camera.compile()))
    }
}
