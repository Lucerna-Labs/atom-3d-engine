//! AOV / debug-pass demo: render the same scene as beauty plus normals, depth, ambient
//! occlusion, albedo, and the sphere-tracing step heatmap (the key perf diagnostic). The kind
//! of frame-inspection tooling a real engine ships.
//!
//! Run: cargo run -p mm3e-orchestrator --example aov --release

use mm3e_kit::color::{Material, Rgba};
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, RenderMode, Scene};

fn main() {
    let mut scene = Scene::new(720, 480);
    scene.aa = 2;

    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.2, 0.22)).roughness(0.3));
    let metal =
        scene.material(Material::solid(Vec3::new(0.9, 0.78, 0.4)).metallic(1.0).roughness(0.18).reflective(0.4));

    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-1.3, 1.0, 0.0)), red));
    scene.add(Object::new(Prim::Torus { major: 0.8, minor: 0.3 }, Transform::at(Vec3::new(1.4, 1.0, 0.0)), metal));
    scene.add(Object::new(
        Prim::Box { half: Vec3::splat(0.7) },
        Transform::at(Vec3::new(0.1, 0.7, 1.8)).rotated(Mat3::from_euler(0.3, 0.6, 0.0)),
        red,
    ));

    scene.sun_dir = Vec3::new(0.5, 0.75, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.8)).soft(0.05));

    let cam = orbit_camera(Vec3::new(0.0, 0.85, 0.4), 7.5, 0.5, 0.3, 52f32.to_radians());

    for (mode, name) in [
        (RenderMode::Beauty, "aov_beauty"),
        (RenderMode::Normal, "aov_normal"),
        (RenderMode::Depth, "aov_depth"),
        (RenderMode::Ao, "aov_ao"),
        (RenderMode::Albedo, "aov_albedo"),
        (RenderMode::Steps, "aov_steps"),
    ] {
        scene.mode = mode;
        let fb = render(&scene, &cam);
        let path = format!("{name}.bmp");
        std::fs::write(&path, fb.to_bmp(Rgba::rgb8(0, 0, 0))).expect("write");
        println!("wrote {path}");
    }
}
