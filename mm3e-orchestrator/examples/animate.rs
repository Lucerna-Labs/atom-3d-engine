//! Animation demo: keyframed transforms sampled at a time `t`, one rendered frame per step.
//! Because the engine is stateless (the scene is rebuilt every frame), animation is just
//! "evaluate the tracks, build the scene, render" — time is one more parameter.
//!
//! Run: cargo run -p mm3e-orchestrator --example animate --release [frames]

use mm3e_kit::color::{Material, Rgba};
use mm3e_kit::vec::{Quat, Transform, Vec3};
use mm3e_orchestrator::anim::{ping_pong, Easing, Track};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};

/// Build the scene at time `t` (seconds). Materials/lights are fixed; transforms animate.
fn scene_at(t: f32, bounce: &Track<Vec3>, spin: &Track<Quat>) -> Scene {
    let mut scene = Scene::new(640, 360);
    scene.aa = 2;
    scene.bounces = 1;

    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.2, 0.22)).roughness(0.3).specular(0.8));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.28)).metallic(1.0).roughness(0.2).reflective(0.5));

    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));

    // A bouncing sphere (ping-ponged height track) and a spinning box (quaternion slerp track).
    let pos = bounce.sample(ping_pong(t * 1.4));
    scene.add(Object::new(Prim::Sphere { r: 0.7 }, Transform::at(pos), red));

    let rot = spin.sample(t);
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.7), radius: 0.15 },
        Transform::new(Vec3::new(1.8, 0.8, 0.0), rot.to_mat3(), 1.0),
        gold,
    ));

    scene.sun_dir = Vec3::new(0.5, 0.75, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(2.0)).soft(0.03));
    scene
}

fn main() {
    let frames: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(12);

    // Tracks: a low→high bounce (eased), and a half-turn spin.
    let bounce = Track::new(Easing::SmoothStep).key(0.0, Vec3::new(-1.6, 0.7, 0.0)).key(1.0, Vec3::new(-1.6, 2.4, 0.0));
    let spin = Track::new(Easing::Linear)
        .key(0.0, Quat::IDENTITY)
        .key(2.0, Quat::from_axis_angle(Vec3::new(0.2, 1.0, 0.1), std::f32::consts::TAU));

    for f in 0..frames {
        let t = 2.0 * f as f32 / frames as f32; // one full loop over [0, 2)
        let scene = scene_at(t, &bounce, &spin);
        let cam = orbit_camera(Vec3::new(0.2, 0.9, 0.0), 7.0, 0.5, 0.28, 52f32.to_radians());
        let fb = render(&scene, &cam);
        let path = format!("animate_{f:02}.bmp");
        std::fs::write(&path, fb.to_bmp(Rgba::rgb8(0, 0, 0))).expect("write");
        println!("wrote {path}");
    }
    println!("done: {frames} frames");
}
