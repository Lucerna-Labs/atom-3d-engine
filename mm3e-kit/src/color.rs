//! Color + material data. Pure data; makes no rendering decisions. `Rgba` is straight-alpha
//! and lives at the framebuffer boundary; all shading math runs in linear `Vec3` and converts
//! to `Rgba` only at the end (after tone-mapping + gamma).

use crate::vec::Vec3;

/// Straight-alpha RGBA in [0, 1].
#[derive(Clone, Copy, Debug)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
    }
    /// Opaque RGB from a linear/display `Vec3` (channels clamped to [0, 1]).
    pub fn from_vec3(v: Vec3) -> Self {
        let v = v.clamp01();
        Self::new(v.x, v.y, v.z, 1.0)
    }
    pub fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }
}

/// A surface description. `albedo`/`emissive` are linear-space colors; `specular`,
/// `roughness`, and `reflectivity` drive the shading model. `checker` flags the
/// orchestrator's procedural floor (the kit carries the flag, never decides to honor it).
#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub albedo: Vec3,
    pub specular: f32,
    pub roughness: f32,
    pub reflectivity: f32,
    pub emissive: Vec3,
    pub checker: bool,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: Vec3::splat(0.8),
            specular: 0.5,
            roughness: 0.35,
            reflectivity: 0.0,
            emissive: Vec3::ZERO,
            checker: false,
        }
    }
}

impl Material {
    pub fn solid(albedo: Vec3) -> Self {
        Self { albedo, ..Self::default() }
    }
    pub fn specular(mut self, s: f32) -> Self {
        self.specular = s;
        self
    }
    pub fn roughness(mut self, r: f32) -> Self {
        self.roughness = r;
        self
    }
    pub fn reflective(mut self, r: f32) -> Self {
        self.reflectivity = r;
        self
    }
    pub fn emissive(mut self, e: Vec3) -> Self {
        self.emissive = e;
        self
    }
    pub fn checkered(mut self) -> Self {
        self.checker = true;
        self
    }
}
