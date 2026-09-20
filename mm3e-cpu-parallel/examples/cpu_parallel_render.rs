//! Example: render a simple scene using the CPU parallel backend.
//!
//! Run with: cargo run --release --example cpu_parallel_render

use mm3e_cpu_parallel::CpuParallelRenderer;
use mm3e_kit::color::Rgba;
use mm3e_kit::vec::{Transform, Vec3};
use mm3e_orchestrator::{Object, Prim, Scene};
use std::sync::Arc;

fn main() {
    println!("CPU Parallel Renderer Example");
    println!("==============================\n");

    // Create a simple scene
    let mut scene = Scene::new(800, 600);

    // Add a ground plane
    scene.objects.push(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: -1.0 }, Transform::at(Vec3::ZERO), 0));

    // Add some spheres
    scene.objects.push(Object::new(Prim::Sphere { r: 0.5 }, Transform::at(Vec3::new(-1.0, 0.5, -2.0)), 1));

    scene.objects.push(Object::new(Prim::Sphere { r: 0.5 }, Transform::at(Vec3::new(1.0, 0.5, -2.0)), 2));

    // Add a box
    scene.objects.push(Object::new(Prim::Box { half: Vec3::splat(0.5) }, Transform::at(Vec3::new(0.0, 0.5, -3.0)), 3));

    // Add materials
    scene.materials.push(mm3e_kit::color::Material {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        metallic: 0.0,
        roughness: 0.5,
        reflectivity: 0.0,
        specular: 0.5,
        emissive: Vec3::ZERO,
        checker: true,
    });

    scene.materials.push(mm3e_kit::color::Material {
        albedo: Vec3::new(1.0, 0.2, 0.2),
        metallic: 0.0,
        roughness: 0.3,
        reflectivity: 0.0,
        specular: 0.5,
        emissive: Vec3::ZERO,
        checker: false,
    });

    scene.materials.push(mm3e_kit::color::Material {
        albedo: Vec3::new(0.2, 0.2, 1.0),
        metallic: 0.0,
        roughness: 0.3,
        reflectivity: 0.0,
        specular: 0.5,
        emissive: Vec3::ZERO,
        checker: false,
    });

    scene.materials.push(mm3e_kit::color::Material {
        albedo: Vec3::new(0.2, 1.0, 0.2),
        metallic: 0.0,
        roughness: 0.3,
        reflectivity: 0.0,
        specular: 0.5,
        emissive: Vec3::ZERO,
        checker: false,
    });

    // Add a light
    scene.lights.push(mm3e_orchestrator::Light {
        vec: Vec3::new(2.0, 4.0, -2.0),
        color: Vec3::new(1.0, 0.95, 0.9),
        directional: false,
        radius: 0.0,
    });

    // Configure renderer
    scene.aa = 2; // 2x anti-aliasing
    scene.bounces = 1; // 1 reflection bounce
    scene.shadows = true;
    scene.ao = true;
    scene.marcher.max_steps = 128;
    scene.marcher.shadow_steps = 32;
    scene.marcher.ao_samples = 5;

    // Create camera
    let camera = mm3e_kit::camera::Camera::look_at(
        Vec3::new(0.0, 1.5, 0.0),
        Vec3::new(0.0, 0.5, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
    );

    // Create renderer
    let renderer = CpuParallelRenderer::new();
    println!("Using {} threads", renderer.num_threads());

    // Compile scene
    let compiled = renderer.compile(Arc::new(scene));

    // Render
    println!("Rendering...");
    let start = std::time::Instant::now();
    let framebuffer = compiled.render(&camera, &[]);
    let elapsed = start.elapsed();

    println!("Render complete in {:.2?}", elapsed);
    println!("Resolution: {}x{}", framebuffer.width, framebuffer.height);

    // Save to file
    let filename = "cpu_parallel_output.bmp";
    let bmp_data = framebuffer.to_bmp(Rgba::new(0.0, 0.0, 0.0, 1.0));
    if let Err(e) = std::fs::write(filename, bmp_data) {
        eprintln!("Failed to save image: {}", e);
    } else {
        println!("Saved to {}", filename);
    }
}
