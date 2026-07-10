//! GPU regression for orbit-camera blackout and full-turn periodicity.
//!
//! Run: cargo run -p mm3e-gpu --example orbit_regression --release

use mm3e_gpu::GpuRenderer;
use mm3e_kit::color::Material;
use mm3e_kit::vec::{Transform, Vec3};
use mm3e_orchestrator::{orbit_camera, Light, Object, Prim, Scene};

fn mean_rgb(rgba: &[u8]) -> f64 {
    rgba.chunks_exact(4)
        .map(|pixel| (u32::from(pixel[0]) + u32::from(pixel[1]) + u32::from(pixel[2])) as f64 / 3.0)
        .sum::<f64>()
        / (rgba.len() / 4) as f64
}

fn main() {
    let mut scene = Scene::new(320, 180);
    scene.aa = 1;
    scene.bounces = 1;
    scene.marcher.max_steps = 96;

    let floor = scene.material(Material::solid(Vec3::splat(0.85)).checkered().roughness(0.7));
    let subject = scene.material(Material::solid(Vec3::new(0.85, 0.25, 0.18)).specular(0.5).roughness(0.3));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(0.0, 1.0, 0.0)), subject));
    scene.sun_dir = Vec3::new(0.5, 0.75, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.8)));

    let renderer = GpuRenderer::new().unwrap_or_else(|error| panic!("GPU init failed: {error}"));
    println!("GPU: {}", renderer.adapter_name());
    let gpu_scene = renderer.compile(&scene, scene.width, scene.height);
    let target = Vec3::new(0.0, 0.85, 0.0);
    let base_yaw = 0.5f32;
    let mut first = Vec::new();
    let mut minimum_mean = f64::MAX;

    for degree in (0..=360).step_by(5) {
        let yaw = base_yaw + (degree as f32).to_radians();
        let camera = orbit_camera(target, 7.0, yaw, 0.3, 52f32.to_radians());
        let rgba = gpu_scene.render_rgba(&renderer, &camera);
        let mean = mean_rgb(&rgba);
        minimum_mean = minimum_mean.min(mean);
        assert!(mean > 5.0, "black frame at {degree} degrees (mean RGB {mean:.3})");
        if degree == 0 {
            first = rgba;
        } else if degree == 360 {
            let (total_error, max_error) = first.iter().zip(&rgba).fold((0u64, 0u8), |(total, max), (a, b)| {
                let error = a.abs_diff(*b);
                (total + u64::from(error), max.max(error))
            });
            let mean_error = total_error as f64 / rgba.len() as f64;
            assert!(mean_error < 0.25, "full-turn mean byte error {mean_error:.4} is too high");
            println!("full-turn image error: mean byte {mean_error:.4}, max byte {max_error}");
        }
    }

    for step in 0..=36 {
        let pitch = 0.02 + (1.45 - 0.02) * step as f32 / 36.0;
        let camera = orbit_camera(target, 7.0, base_yaw, pitch, 52f32.to_radians());
        assert!(camera.eye.y > 0.0, "camera crossed below the floor at pitch {pitch:.3}");
        let mean = mean_rgb(&gpu_scene.render_rgba(&renderer, &camera));
        minimum_mean = minimum_mean.min(mean);
        assert!(mean > 5.0, "black frame at vertical orbit pitch {pitch:.3} (mean RGB {mean:.3})");
    }

    println!(
        "PASS: 73 yaw frames + 37 above-floor pitch frames; minimum mean RGB {minimum_mean:.2}; full turn is visually equivalent"
    );
}
