//! mm3e-cpu-parallel  a pure Rust, zero-dependency CPU parallel backend.
//!
//! This crate replaces the wgpu-based GPU backend with a multi-threaded CPU renderer
//! that uses only std library features. It leverages the same mathematical atoms as the
//! core engine: scan (pixel iteration), fold (sphere tracing), project (ray casting),
//! compare (SDF evaluation), combine (lighting), and order (pixel composition).
//!
//! The rendering is parallelized across CPU cores using std::thread, with work distribution
//! for load balancing. Each thread processes a chunk of pixels independently.
//!
//! Usage: [`CpuParallelRenderer::new`] once, then [`CpuParallelRenderer::render`] per frame.
//! The scene description and all mathematical primitives come from mm3e-kit and mm3e-orchestrator.

use mm3e_kit::camera::Camera;
use mm3e_kit::color::Rgba;
use mm3e_kit::framebuffer::Framebuffer;
use mm3e_kit::march::{Marcher, Ray};
use mm3e_kit::sdf::Field;
use mm3e_kit::vec::Vec3;
use mm3e_orchestrator::Scene;
use std::sync::Arc;
use std::thread;

/// A dynamic sphere (player / physics body) rendered without recompiling.
#[derive(Clone, Copy, Debug)]
pub struct DynSphere {
    pub pos: Vec3,
    pub radius: f32,
    pub albedo: Vec3,
    pub metallic: f32,
}

/// A compiled scene ready for parallel rendering.
pub struct CpuParallelScene {
    /// Width of the render target
    width: u32,
    /// Height of the render target
    height: u32,
    /// Anti-aliasing samples
    aa: u32,
    /// Maximum bounces
    bounces: u32,
    /// The marcher configuration
    marcher: Marcher,
    /// Reference to the scene (for field evaluation)
    scene: Arc<Scene>,
}

impl CpuParallelScene {
    /// Create a new compiled scene from a description.
    pub fn new(scene: Arc<Scene>) -> Self {
        CpuParallelScene {
            width: scene.width,
            height: scene.height,
            aa: scene.aa.max(1),
            bounces: scene.bounces,
            marcher: scene.marcher,
            scene: Arc::clone(&scene),
        }
    }

    /// Render the scene from the given camera perspective.
    pub fn render(&self, camera: &Camera, dyn_spheres: &[DynSphere]) -> Framebuffer {
        let width = self.width;
        let height = self.height;
        
        // Create a framebuffer for the result
        let mut fb = Framebuffer::new(width, height, Rgba::new(0.0, 0.0, 0.0, 1.0));
        
        // For single-threaded or small renders, just do it directly
        if width * height < 10000 || num_cpus() <= 1 {
            self.render_single_threaded(camera, dyn_spheres, &mut fb);
            return fb;
        }
        
        // Parallel rendering with work distribution
        self.render_parallel(camera, dyn_spheres, &mut fb);
        fb
    }

    /// Single-threaded render for small images or single-core systems.
    fn render_single_threaded(&self, camera: &Camera, dyn_spheres: &[DynSphere], fb: &mut Framebuffer) {
        let width = self.width;
        let height = self.height;
        let aa = self.aa;
        let bounces = self.bounces;
        let marcher = &self.marcher;
        
        for y in 0..height {
            for x in 0..width {
                let pixel = self.shade_pixel(x, y, aa, bounces, camera, marcher, dyn_spheres);
                fb.put(x, y, pixel);
            }
        }
    }

    /// Parallel render using multiple threads.
    fn render_parallel(&self, camera: &Camera, dyn_spheres: &[DynSphere], fb: &mut Framebuffer) {
        let width = self.width;
        let height = self.height;
        let aa = self.aa;
        let bounces = self.bounces;
        let marcher = &self.marcher;
        
        // Determine number of threads
        let num_threads = num_cpus().min(16); // Cap at 16 threads
        
        // Split work into chunks (one chunk per thread, roughly equal pixels)
        let total_pixels = (width * height) as usize;
        let chunk_size = (total_pixels + num_threads - 1) / num_threads;
        
        // Create shared pixel buffer
        let pixels = Arc::new(std::sync::Mutex::new(vec![Rgba::new(0.0, 0.0, 0.0, 1.0); total_pixels]));
        
        // Spawn threads
        let mut handles = vec![];
        
        for thread_idx in 0..num_threads {
            let start = thread_idx * chunk_size;
            let end = std::cmp::min(start + chunk_size, total_pixels);
            
            if start >= end {
                continue;
            }
            
            let pixels_clone = Arc::clone(&pixels);
            let camera_clone = camera.clone();
            let marcher_clone = marcher.clone();
            let scene_clone = Arc::clone(&self.scene);
            let dyn_spheres_clone = dyn_spheres.to_vec();
            let aa_val = aa;
            let bounces_val = bounces;
            let width_val = width;
            let height_val = height;
            
            handles.push(thread::spawn(move || {
                for pixel_idx in start..end {
                    let x = (pixel_idx % width_val as usize) as u32;
                    let y = (pixel_idx / width_val as usize) as u32;
                    
                    let color = shade_pixel_standalone(
                        x, y, width_val, height_val, aa_val, bounces_val,
                        &camera_clone, &marcher_clone, &scene_clone, &dyn_spheres_clone
                    );
                    
                    let mut pixels = pixels_clone.lock().unwrap();
                    pixels[pixel_idx] = color;
                }
            }));
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Copy results to framebuffer
        let pixels = Arc::try_unwrap(pixels).unwrap().into_inner().unwrap();
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                fb.put(x, y, pixels[idx]);
            }
        }
    }

    /// Shade a single pixel (for single-threaded path).
    fn shade_pixel(
        &self,
        x: u32,
        y: u32,
        aa: u32,
        bounces: u32,
        camera: &Camera,
        marcher: &Marcher,
        dyn_spheres: &[DynSphere],
    ) -> Rgba {
        let width = self.width as f32;
        let height = self.height as f32;
        let aspect = width / height;
        
        let mut color = Vec3::ZERO;
        let n = aa.max(1);
        
        for sy in 0..n {
            for sx in 0..n {
                let ox = (sx as f32 + 0.5) / n as f32;
                let oy = (sy as f32 + 0.5) / n as f32;
                
                let px = x as f32 + ox;
                let py = y as f32 + oy;
                
                // Convert pixel coordinates to normalized device coordinates
                let ndcx = (2.0 * px / width - 1.0) * aspect * camera.fov_scale;
                let ndcy = (1.0 - 2.0 * py / height) * camera.fov_scale;
                
                // Create ray
                let rd = (camera.forward + camera.right.scale(ndcx) + camera.up.scale(ndcy)).normalize();
                let ray = Ray { origin: camera.eye, dir: rd };
                
                // Build world field with dynamic spheres
                let world = self.build_world(dyn_spheres);
                
                // Trace and shade the ray
                let pixel_color = trace_and_shade(ray, bounces, marcher, &world);
                color = color + pixel_color;
            }
        }
        
        color = color.scale(1.0 / (n * n) as f32);
        
        // Apply exposure and gamma correction (matching the GPU shader)
        let exposure = self.scene.post.exposure;
        color = color.scale(exposure);
        
        // Simple tone mapping (aces-like)
        color = aces_tonemap(color);
        
        // Gamma correction
        color = Vec3::new(
            color.x.powf(1.0 / 2.2),
            color.y.powf(1.0 / 2.2),
            color.z.powf(1.0 / 2.2),
        );
        
        Rgba::new(color.x.clamp(0.0, 1.0), color.y.clamp(0.0, 1.0), color.z.clamp(0.0, 1.0), 1.0)
    }

    /// Build a world field that includes dynamic spheres.
    fn build_world(&self, dyn_spheres: &[DynSphere]) -> impl Fn(Vec3) -> Field + '_ {
        let scene_world = self.scene.field();
        let dyn_spheres = dyn_spheres.to_vec();
        
        move |p: Vec3| -> Field {
            let mut result = scene_world(p);
            
            // Union with dynamic spheres
            for (i, sphere) in dyn_spheres.iter().enumerate() {
                let dist = (p - sphere.pos).length() - sphere.radius;
                if dist < result.dist {
                    // Material ID for dynamic spheres: start from a high number
                    // to avoid conflicts with scene materials
                    result = Field::new(dist, 1000 + i as u32);
                }
            }
            
            result
        }
    }
}

/// Standalone pixel shading function for parallel threads.
fn shade_pixel_standalone(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    aa: u32,
    bounces: u32,
    camera: &Camera,
    marcher: &Marcher,
    scene: &Scene,
    dyn_spheres: &[DynSphere],
) -> Rgba {
    let width_f = width as f32;
    let height_f = height as f32;
    let aspect = width_f / height_f;
    
    let mut color = Vec3::ZERO;
    let n = aa.max(1);
    
    // Build world field
    let scene_world = scene.field();
    let dyn_spheres_clone = dyn_spheres.to_vec();
    let world = move |p: Vec3| -> Field {
        let mut result = scene_world(p);
        for (i, sphere) in dyn_spheres_clone.iter().enumerate() {
            let dist = (p - sphere.pos).length() - sphere.radius;
            if dist < result.dist {
                result = Field::new(dist, 1000 + i as u32);
            }
        }
        result
    };
    
    for sy in 0..n {
        for sx in 0..n {
            let ox = (sx as f32 + 0.5) / n as f32;
            let oy = (sy as f32 + 0.5) / n as f32;
            
            let px = x as f32 + ox;
            let py = y as f32 + oy;
            
            let ndcx = (2.0 * px / width_f - 1.0) * aspect * camera.fov_scale;
            let ndcy = (1.0 - 2.0 * py / height_f) * camera.fov_scale;
            
            let rd = (camera.forward + camera.right.scale(ndcx) + camera.up.scale(ndcy)).normalize();
            let ray = Ray { origin: camera.eye, dir: rd };
            
            // Trace and shade
            let pixel_color = trace_and_shade(ray, bounces, marcher, &world);
            color = color + pixel_color;
        }
    }
    
    color = color.scale(1.0 / (n * n) as f32);
    
    // Apply exposure and gamma
    let exposure = scene.post.exposure;
    color = color.scale(exposure);
    color = aces_tonemap(color);
    color = Vec3::new(
        color.x.powf(1.0 / 2.2),
        color.y.powf(1.0 / 2.2),
        color.z.powf(1.0 / 2.2),
    );
    
    Rgba::new(color.x.clamp(0.0, 1.0), color.y.clamp(0.0, 1.0), color.z.clamp(0.0, 1.0), 1.0)
}

/// Trace a ray and compute lighting.
fn trace_and_shade(
    ray: Ray,
    max_bounces: u32,
    marcher: &Marcher,
    world: &impl Fn(Vec3) -> Field,
) -> Vec3 {
    let mut color = Vec3::ZERO;
    let atten = Vec3::ONE;
    
    for _bounce in 0..=max_bounces {
        let hit = marcher.march(world, &ray);
        
        if !hit.hit {
            // Sky background
            color = color + atten.cmul(sky(ray.dir));
            break;
        }
        
        // Simplified material for now - in a full implementation we'd look up materials
        let albedo = Vec3::splat(0.8);
        
        // Direct lighting (simplified for now)
        let rad = albedo.scale(0.5); // Simplified lighting
        
        color = color + atten.cmul(rad);
        break;
    }
    
    color
}

/// Sky background function.
fn sky(d: Vec3) -> Vec3 {
    let t = (0.5 * (d.y + 1.0)).clamp(0.0, 1.0);
    let base = Vec3::new(0.78, 0.86, 0.96).mix(Vec3::new(0.20, 0.38, 0.72), t);
    let sun_dir = Vec3::new(0.5, 0.7, -0.5).normalize();
    let s = d.dot(sun_dir).max(0.0);
    base + Vec3::splat(s.powf(8.0) * 0.3 + s.powf(220.0) * 6.0)
}

/// ACES tonemapping.
fn aces_tonemap(x: Vec3) -> Vec3 {
    let a = x.max(Vec3::ZERO);
    let a_len = a.length();
    if a_len < 1e-6 {
        return Vec3::ZERO;
    }
    let numerator = a.scale(2.51 * a_len + 0.03);
    let denominator = a.scale(2.43 * a_len + 0.59) + Vec3::splat(0.14);
    // Component-wise division
    Vec3::new(
        (numerator.x / denominator.x).clamp(0.0, 1.0),
        (numerator.y / denominator.y).clamp(0.0, 1.0),
        (numerator.z / denominator.z).clamp(0.0, 1.0),
    )
}

/// CPU parallel renderer.
pub struct CpuParallelRenderer {
    /// Number of threads to use
    num_threads: usize,
}

impl CpuParallelRenderer {
    /// Create a new CPU parallel renderer.
    pub fn new() -> Self {
        CpuParallelRenderer {
            num_threads: num_cpus(),
        }
    }

    /// Create with a specific number of threads.
    pub fn with_threads(num_threads: usize) -> Self {
        CpuParallelRenderer { num_threads }
    }

    /// Compile a scene for rendering.
    pub fn compile(&self, scene: Arc<Scene>) -> CpuParallelScene {
        CpuParallelScene::new(scene)
    }

    /// Get the number of threads being used.
    pub fn num_threads(&self) -> usize {
        self.num_threads
    }
}

impl Default for CpuParallelRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the number of available CPU cores.
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}
