//! GPU resolution sweep: render the *same* SDF scene at 540p → 4K and report sustained fps, to
//! answer "why does the CPU path manage ~2 fps while a GPU runs 4K?" directly — the same raymarch,
//! moved onto thousands of GPU lanes, holds real time at resolutions the CPU cannot touch.
//!
//! Two numbers per resolution:
//! - `render` — pure GPU shading throughput; what an on-screen game sees (it presents the texture
//!   and never copies it back).
//! - `readback` — shading + the whole image copied back to the CPU each frame (the BMP/AOV path).
//!   At 4K that is a ≈33 MB/frame PCIe copy, so it sits below the render number — a measurement
//!   artifact of pulling pixels to the CPU, not the GPU's shading limit.
//!
//! Run: cargo run -p mm3e-gpu --example gpu_resolution --release

use mm3e_gpu::GpuRenderer;
use mm3e_kit::color::Material;
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

fn cam_at(i: u32) -> mm3e_kit::camera::Camera {
    let yaw = 0.55 + i as f32 * 0.01;
    orbit_camera(Vec3::new(0.2, 0.85, 0.4), 8.5, yaw, 0.32, 50f32.to_radians())
}

fn main() {
    let scene = build_scene();
    let renderer = match GpuRenderer::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("GPU init failed: {e}");
            std::process::exit(1);
        }
    };
    println!("GPU: {}", renderer.adapter_name());
    println!("scene: {} objects, aa={}, bounces={} (GGX PBR + soft shadows + reflections)", 6, scene.aa, scene.bounces);
    println!("{:>6}  {:>11}  {:>12}  {:>12}", "", "pixels", "render fps", "readback fps");

    let resolutions = [(960u32, 540u32, "540p"), (1920, 1080, "1080p"), (2560, 1440, "1440p"), (3840, 2160, "4K")];
    let frames = 120u32;

    for (w, h, label) in resolutions {
        let gpu = renderer.compile(&scene, w, h);

        // Warm up (shader compile + first dispatch settle).
        for i in 0..5 {
            gpu.render_only(&renderer, &cam_at(i));
        }
        renderer.wait_idle();

        // Pure GPU shading throughput: submit a batch, wait once.
        let t0 = std::time::Instant::now();
        for i in 0..frames {
            gpu.render_only(&renderer, &cam_at(i));
        }
        renderer.wait_idle();
        let render_fps = frames as f32 / t0.elapsed().as_secs_f32();

        // Shading + full per-frame CPU readback (the BMP/AOV path).
        let t1 = std::time::Instant::now();
        for i in 0..frames {
            let _ = gpu.render_rgba(&renderer, &cam_at(i));
        }
        let readback_fps = frames as f32 / t1.elapsed().as_secs_f32();

        let mp = (w * h) as f32 / 1.0e6;
        println!("{label:>6}  {w}x{h} ({mp:.1}MP)  {render_fps:10.1}  {readback_fps:12.1}");
    }
}
