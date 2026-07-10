//! CPU vs GPU, same scene / same camera / same shading — the direct, measured answer to "why does
//! the CPU manage a couple of frames while a GPU drives 4K?" Both paths run the *identical* SDF
//! raymarch (the GPU's WGSL is codegen'd from the same world field the CPU marches); only the
//! hardware differs. The speedup column is the lane-count story, quantified.
//!
//! CPU is timed at 480p and 1080p (a few frames — full-quality SDF marching is slow); the GPU is
//! timed at 480p, 1080p and 4K. The CPU 4K cell is extrapolated from 1080p by the pixel ratio,
//! because rendering it for real would take minutes per frame — which is itself the answer.
//!
//! Run: cargo run -p mm3e-gpu --example cpu_vs_gpu --release

use mm3e_gpu::GpuRenderer;
use mm3e_kit::color::Material;
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};

fn build_scene(width: u32, height: u32) -> Scene {
    let mut scene = Scene::new(width, height);
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

fn cam_at(i: u32) -> mm3e_kit::camera::Camera {
    orbit_camera(Vec3::new(0.2, 0.85, 0.4), 8.5, 0.55 + i as f32 * 0.01, 0.32, 50f32.to_radians())
}

/// Time the multithreaded CPU renderer; returns fps.
fn cpu_fps(w: u32, h: u32, frames: u32) -> f32 {
    let scene = build_scene(w, h);
    let _ = render(&scene, &cam_at(0)); // warm up (page-in, thread spawn)
    let t0 = std::time::Instant::now();
    for i in 0..frames {
        let _ = render(&scene, &cam_at(i));
    }
    frames as f32 / t0.elapsed().as_secs_f32()
}

/// Time pure GPU shading throughput (no per-frame readback); returns fps.
fn gpu_fps(r: &GpuRenderer, w: u32, h: u32, frames: u32) -> f32 {
    let scene = build_scene(w, h);
    let gpu = r.compile(&scene, w, h);
    for i in 0..5 {
        gpu.render_only(r, &cam_at(i));
    }
    r.wait_idle();
    let t0 = std::time::Instant::now();
    for i in 0..frames {
        gpu.render_only(r, &cam_at(i));
    }
    r.wait_idle();
    frames as f32 / t0.elapsed().as_secs_f32()
}

fn main() {
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let renderer = match GpuRenderer::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("GPU init failed: {e}");
            std::process::exit(1);
        }
    };
    println!("Same SDF scene (GGX PBR, soft shadows, reflections, aa=2, 3 bounces).");
    println!("CPU: {threads} threads.  GPU: {}.", renderer.adapter_name());
    println!();
    println!("{:>8}  {:>11}  {:>12}  {:>12}  {:>9}", "res", "pixels", "CPU fps", "GPU fps", "speedup");
    println!("{:->8}  {:->11}  {:->12}  {:->12}  {:->9}", "", "", "", "", "");

    // 480p — both measured.
    let (w, h) = (854u32, 480u32);
    let c480 = cpu_fps(w, h, 4);
    let g480 = gpu_fps(&renderer, w, h, 120);
    row("480p", w, h, c480, g480);

    // 1080p — both measured (CPU is slow here; few frames).
    let (w, h) = (1920u32, 1080u32);
    let c1080 = cpu_fps(w, h, 3);
    let g1080 = gpu_fps(&renderer, w, h, 120);
    row("1080p", w, h, c1080, g1080);

    // 4K — GPU measured; CPU extrapolated from 1080p by the 4× pixel ratio (real render = minutes).
    let (w, h) = (3840u32, 2160u32);
    let g4k = gpu_fps(&renderer, w, h, 80);
    let c4k_est = c1080 / 4.0;
    println!("{:>8}  {:>11}  {:>11.2}* {:>12.1}  {:>8.0}x", "4K", format!("{w}x{h}"), c4k_est, g4k, g4k / c4k_est);
    println!("\n* CPU 4K is extrapolated (1080p fps / 4); rendering it for real is minutes/frame.");
    println!("Same math, same image — the entire gap is parallel hardware lanes, not the algorithm.");
}

fn row(label: &str, w: u32, h: u32, cpu: f32, gpu: f32) {
    let mp = format!("{w}x{h}");
    println!("{label:>8}  {mp:>11}  {cpu:>12.2}  {gpu:>12.1}  {:>8.0}x", gpu / cpu);
}
