//! Turntable: render N frames orbiting the scene, each to its own BMP. The engine is fully
//! deterministic, so this doubles as an animation pipeline — no GPU, no external crates.
//!
//! Run: cargo run -p mm3e-orchestrator --example turntable --release

use mm3e_kit::color::{Material, Rgba};
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};

fn build_scene() -> Scene {
    let mut scene = Scene::new(640, 360);
    scene.aa = 2;
    scene.bounces = 1;

    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.65).specular(0.2));
    let teal = scene.material(Material::solid(Vec3::new(0.18, 0.6, 0.65)).specular(0.8).roughness(0.2).reflective(0.3));
    let amber =
        scene.material(Material::solid(Vec3::new(0.95, 0.6, 0.18)).metallic(1.0).roughness(0.22).reflective(0.3));

    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.9), radius: 0.2 },
        Transform::at(Vec3::new(-1.2, 0.9, 0.0)).rotated(Mat3::from_euler(0.0, 0.4, 0.0)),
        teal,
    ));
    scene.add(Object::new(
        Prim::Torus { major: 0.85, minor: 0.28 },
        Transform::at(Vec3::new(1.4, 0.95, 0.0)).rotated(Mat3::from_euler(1.1, 0.0, 0.3)),
        amber,
    ));
    scene.add(Object::new(Prim::Sphere { r: 0.7 }, Transform::at(Vec3::new(0.2, 0.7, 1.6)), teal).smooth(0.4));

    scene.sun_dir = Vec3::new(0.5, 0.72, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.2, 1.1, 0.95).scale(1.7)));
    scene.light(Light::directional(Vec3::new(-0.4, 0.5, -0.6), Vec3::new(0.32, 0.4, 0.55)));
    scene
}

fn main() {
    let frames: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(8);
    let scene = build_scene();

    for f in 0..frames {
        let yaw = std::f32::consts::TAU * f as f32 / frames as f32;
        let cam = orbit_camera(Vec3::new(0.0, 0.8, 0.4), 8.0, yaw, 0.30, 50f32.to_radians());
        let fb = render(&scene, &cam);
        let path = format!("turntable_{f:02}.bmp");
        std::fs::write(&path, fb.to_bmp(Rgba::rgb8(0, 0, 0))).expect("write frame");
        println!("wrote {path}");
    }
    println!("done: {frames} frames");
}
