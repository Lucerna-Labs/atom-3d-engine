//! Scene-file round-trip demo: build a scene in code, serialize it to the `.mm3e` text format,
//! parse it back, and render the *parsed* scene. Proves the data layer that de-Rusts authoring —
//! scenes become editable files instead of recompiled code.
//!
//! Run: cargo run -p mm3e-orchestrator --example scene_file --release

use mm3e_kit::color::{Material, Rgba};
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::{orbit_camera, render, scene_io, Light, Object, Prim, Scene};

fn build() -> (Scene, mm3e_kit::camera::Camera) {
    let mut scene = Scene::new(800, 450);
    scene.aa = 2;
    scene.bounces = 2;

    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.2, 0.22)).roughness(0.25).specular(0.8));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.28)).metallic(1.0).roughness(0.18).reflective(0.5));

    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-1.4, 1.0, 0.0)), gold));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.8), radius: 0.18 },
        Transform::at(Vec3::new(1.3, 0.9, 0.0)).rotated(Mat3::from_euler(0.0, 0.5, 0.0)),
        red,
    ));
    scene.add(
        Object::new(Prim::Torus { major: 0.7, minor: 0.26 }, Transform::at(Vec3::new(0.0, 0.5, 2.0)), red).smooth(0.4),
    );

    scene.sun_dir = Vec3::new(0.5, 0.75, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(2.0)).soft(0.03));

    let cam = orbit_camera(Vec3::new(0.0, 0.8, 0.4), 8.0, 0.55, 0.3, 50f32.to_radians());
    (scene, cam)
}

fn main() {
    let (scene, cam) = build();

    // Serialize → disk → parse back.
    let text = scene_io::serialize(&scene, &cam);
    std::fs::write("scene.mm3e", &text).expect("write scene.mm3e");
    let reparsed = scene_io::serialize(&scene_io::parse(&text).expect("parse").0, &cam);
    assert_eq!(text, reparsed, "scene serialization is not a stable round-trip");
    println!("round-trip stable: {} bytes, {} lines", text.len(), text.lines().count());

    // Render the scene loaded from the file (not the in-memory one) to prove the loader works.
    let loaded = std::fs::read_to_string("scene.mm3e").expect("read");
    let (parsed_scene, parsed_cam) = scene_io::parse(&loaded).expect("parse scene.mm3e");
    let fb = render(&parsed_scene, &parsed_cam);
    std::fs::write("scene_file.bmp", fb.to_bmp(Rgba::rgb8(0, 0, 0))).expect("write");
    println!("wrote {} (from scene.mm3e)", std::fs::canonicalize("scene_file.bmp").unwrap().display());
}
