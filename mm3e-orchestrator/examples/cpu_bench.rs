//! CPU frame-rate benchmark — times the multithreaded CPU renderer on the `spheres` scene at two
//! settings, so it can be compared directly against the GPU `gpu_render` benchmark (same scene).
//!
//! Run: cargo run -p mm3e-orchestrator --example cpu_bench --release

use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_kit::Material;
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};

fn build(width: u32, height: u32, aa: u32, bounces: u32) -> Scene {
    let mut scene = Scene::new(width, height);
    scene.aa = aa;
    scene.bounces = bounces;

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

fn bench(label: &str, scene: &Scene, frames: u32) {
    let cam = orbit_camera(Vec3::new(0.2, 0.85, 0.4), 8.5, 0.55, 0.32, 50f32.to_radians());
    let _ = render(scene, &cam); // warm up
    let t0 = std::time::Instant::now();
    for i in 0..frames {
        let c = orbit_camera(Vec3::new(0.2, 0.85, 0.4), 8.5, 0.55 + i as f32 * 0.01, 0.32, 50f32.to_radians());
        let _ = render(scene, &c);
    }
    let dt = t0.elapsed().as_secs_f32();
    println!(
        "{label}: {}x{} aa={} bounces={} -> {:.2} fps ({:.0} ms/frame) over {frames} frames",
        scene.width,
        scene.height,
        scene.aa,
        scene.bounces,
        frames as f32 / dt,
        dt / frames as f32 * 1000.0
    );
}

fn main() {
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    println!("CPU renderer benchmark ({threads} threads)");
    bench("quality   ", &build(960, 540, 2, 3), 20); // matches the GPU gpu_render benchmark
    bench("interactive", &build(480, 270, 1, 1), 40); // the live-viewer setting
}
