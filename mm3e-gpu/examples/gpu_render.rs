//! GPU render: build a scene, compile it to a WGSL compute shader, run it on the GPU, and save a
//! BMP — the GPU twin of the CPU `spheres` example, for a direct visual + timing comparison.
//!
//! Run: cargo run -p mm3e-gpu --example gpu_render --release

use mm3e_gpu::GpuRenderer;
use mm3e_kit::color::{Material, Rgba};
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::{orbit_camera, Light, Object, Prim, Scene};

fn build_scene() -> Scene {
    let mut scene = Scene::new(960, 540);
    scene.aa = 2;
    scene.bounces = 3;

    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().specular(0.15).roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.18, 0.20)).specular(0.7).roughness(0.25));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.25)).metallic(1.0).roughness(0.18).reflective(0.5));
    let glassy =
        scene.material(Material::solid(Vec3::new(0.20, 0.55, 0.85)).specular(0.9).roughness(0.15).reflective(0.35));
    let mirror = scene.material(Material::solid(Vec3::splat(0.95)).metallic(1.0).roughness(0.05).reflective(0.85));

    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-2.4, 1.0, 0.2)), mirror));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.85), radius: 0.18 },
        Transform::at(Vec3::new(0.2, 0.95, -0.6)).rotated(Mat3::from_euler(0.0, 0.6, 0.0)),
        gold,
    ));
    scene.add(Object::new(
        Prim::Torus { major: 0.85, minor: 0.30 },
        Transform::at(Vec3::new(2.6, 1.05, 0.4)).rotated(Mat3::from_euler(1.2, 0.0, 0.2)),
        glassy,
    ));
    scene.add(Object::new(Prim::Sphere { r: 0.7 }, Transform::at(Vec3::new(0.9, 0.7, 1.9)), red));
    scene.add(
        Object::new(
            Prim::Capsule { a: Vec3::new(0.0, -0.5, 0.0), b: Vec3::new(0.0, 0.6, 0.0), r: 0.4 },
            Transform::at(Vec3::new(1.55, 0.7, 1.9)),
            red,
        )
        .smooth(0.5),
    );

    scene.sun_dir = Vec3::new(0.55, 0.7, 0.35).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.25, 1.12, 0.95).scale(1.6)).soft(0.04));
    scene.light(Light::directional(Vec3::new(-0.4, 0.5, -0.7), Vec3::new(0.35, 0.42, 0.6)));
    scene
}

fn main() {
    let scene = build_scene();
    let cam = orbit_camera(Vec3::new(0.2, 0.85, 0.4), 8.5, 0.55, 0.32, 50f32.to_radians());

    let renderer = match GpuRenderer::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("GPU init failed: {e}");
            std::process::exit(1);
        }
    };
    println!("GPU: {}", renderer.adapter_name());

    let gpu_scene = renderer.compile(&scene, scene.width, scene.height);

    // Warm up (shader compile + first dispatch), then save one frame.
    let fb = gpu_scene.render(&renderer, &cam);
    std::fs::write("gpu_render.bmp", fb.to_bmp(Rgba::rgb8(0, 0, 0))).expect("write");
    println!("wrote {} ({}x{})", std::fs::canonicalize("gpu_render.bmp").unwrap().display(), scene.width, scene.height);

    // Time sustained throughput (render + readback per frame).
    let frames = 120u32;
    let t0 = std::time::Instant::now();
    for i in 0..frames {
        let yaw = 0.55 + i as f32 * 0.01;
        let c = orbit_camera(Vec3::new(0.2, 0.85, 0.4), 8.5, yaw, 0.32, 50f32.to_radians());
        let _ = gpu_scene.render_rgba(&renderer, &c);
    }
    let dt = t0.elapsed().as_secs_f32();
    println!(
        "rendered {frames} frames in {dt:.2}s = {:.1} fps at {}x{}",
        frames as f32 / dt,
        scene.width,
        scene.height
    );
}
