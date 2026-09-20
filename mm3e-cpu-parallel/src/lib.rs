//! CPU rendering through the canonical MM3E scene, lighting and post-processing pipeline.
//!
//! A compiled scene retains its complete immutable source. Dynamic spheres are appended as
//! real objects and materials to a per-render snapshot, preserving CSG, volumes, GI and all
//! rendering settings. Static rendering shares the source without copying it.

use mm3e_kit::{
    camera::Camera,
    color::Material,
    framebuffer::Framebuffer,
    vec::{Transform, Vec3},
};
use mm3e_orchestrator::{render_with_threads, Object, Prim, Scene};
use std::sync::Arc;

/// An additional sphere rendered with the scene's normal material and lighting rules.
#[derive(Clone, Copy, Debug)]
pub struct DynSphere {
    pub pos: Vec3,
    pub radius: f32,
    pub albedo: Vec3,
    pub metallic: f32,
}

/// An immutable scene and its configured render worker budget.
pub struct CpuParallelScene {
    scene: Arc<Scene>,
    num_threads: usize,
}

impl CpuParallelScene {
    /// Retain the complete source scene using the host's available worker count.
    pub fn new(scene: Arc<Scene>) -> Self {
        Self { scene, num_threads: available_threads() }
    }

    /// Configured workers; actual render workers are limited to the number of image rows.
    pub fn num_threads(&self) -> usize {
        self.num_threads
    }

    /// Render through the canonical engine. Invalid dynamic input panics with a validation
    /// message; applications accepting external data should use [`Self::try_render`].
    pub fn render(&self, camera: &Camera, dyn_spheres: &[DynSphere]) -> Framebuffer {
        self.try_render(camera, dyn_spheres).expect("invalid dynamic sphere render input")
    }

    /// Validate dynamic inputs before rendering. Position and radius must be finite, radius
    /// positive, and linear albedo/metallicity within [0,1]. The source is never modified.
    pub fn try_render(&self, camera: &Camera, dyn_spheres: &[DynSphere]) -> Result<Framebuffer, String> {
        validate_dynamic(dyn_spheres)?;
        if dyn_spheres.is_empty() {
            return Ok(render_with_threads(&self.scene, camera, self.num_threads));
        }
        if self.scene.materials.len().checked_add(dyn_spheres.len()).is_none_or(|n| n > u32::MAX as usize) {
            return Err("dynamic spheres exceed material ID capacity".into());
        }
        let mut snapshot = self.scene.as_ref().clone();
        for sphere in dyn_spheres {
            let material =
                snapshot.material(Material { albedo: sphere.albedo, metallic: sphere.metallic, ..Material::default() });
            snapshot.add(Object::new(Prim::Sphere { r: sphere.radius }, Transform::at(sphere.pos), material));
        }
        Ok(render_with_threads(&snapshot, camera, self.num_threads))
    }
}

fn validate_dynamic(spheres: &[DynSphere]) -> Result<(), String> {
    for (index, sphere) in spheres.iter().enumerate() {
        if ![sphere.pos.x, sphere.pos.y, sphere.pos.z].iter().all(|v| v.is_finite()) {
            return Err(format!("dynamic sphere {index}: position must be finite"));
        }
        if !sphere.radius.is_finite() || sphere.radius <= 0.0 {
            return Err(format!("dynamic sphere {index}: radius must be finite and positive"));
        }
        if ![sphere.albedo.x, sphere.albedo.y, sphere.albedo.z, sphere.metallic]
            .iter()
            .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
        {
            return Err(format!("dynamic sphere {index}: linear albedo and metallic must be finite and in [0,1]"));
        }
    }
    Ok(())
}

/// Select the worker budget for canonical MM3E rendering.
pub struct CpuParallelRenderer {
    num_threads: usize,
}

impl CpuParallelRenderer {
    pub fn new() -> Self {
        Self { num_threads: available_threads() }
    }

    /// Configure workers; zero is normalized to one. The render caps workers to image rows.
    pub fn with_threads(num_threads: usize) -> Self {
        Self { num_threads: num_threads.max(1) }
    }

    pub fn compile(&self, scene: Arc<Scene>) -> CpuParallelScene {
        CpuParallelScene { scene, num_threads: self.num_threads }
    }

    pub fn num_threads(&self) -> usize {
        self.num_threads
    }
}

impl Default for CpuParallelRenderer {
    fn default() -> Self {
        Self::new()
    }
}

fn available_threads() -> usize {
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
}
