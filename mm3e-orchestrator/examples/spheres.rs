//! Hero scene: CSG primitives on a checkered floor under a sun, with soft shadows, ambient
//! occlusion, one reflection bounce, and distance fog — all from pure-math 3-D SDFs marched
//! by the kit. No GPU, no Vello, no external crates.
//!
//! Run: cargo run -p mm3e-orchestrator --example spheres --release

use mm3e_kit::vec::{Mat3, Transform};
use mm3e_kit::{color::Material, vec::Vec3};
use mm3e_orchestrator::{orbit_camera, render, Combine, Light, Object, Prim, Scene};

fn main() {
    let mut scene = Scene::new(960, 540);
    scene.aa = 2;
    scene.bounces = 2;

    // Materials.
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().specular(0.15).roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.18, 0.20)).specular(0.7).roughness(0.25));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.25)).metallic(1.0).roughness(0.18).reflective(0.5));
    let glassy =
        scene.material(Material::solid(Vec3::new(0.20, 0.55, 0.85)).specular(0.9).roughness(0.15).reflective(0.35));
    let mirror = scene.material(Material::solid(Vec3::splat(0.95)).metallic(1.0).roughness(0.05).reflective(0.85));

    // Ground plane (y = 0, normal up).
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));

    // A mirror sphere.
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-2.4, 1.0, 0.2)), mirror));

    // A gold rounded box, tilted.
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.85), radius: 0.18 },
        Transform::at(Vec3::new(0.2, 0.95, -0.6)).rotated(Mat3::from_euler(0.0, 0.6, 0.0)),
        gold,
    ));

    // A blue torus standing up.
    scene.add(Object::new(
        Prim::Torus { major: 0.85, minor: 0.30 },
        Transform::at(Vec3::new(2.6, 1.05, 0.4)).rotated(Mat3::from_euler(1.2, 0.0, 0.2)),
        glassy,
    ));

    // A red sphere smooth-blended with a capsule into one organic blob.
    scene.add(Object::new(Prim::Sphere { r: 0.7 }, Transform::at(Vec3::new(0.9, 0.7, 1.9)), red));
    scene.add(
        Object::new(
            Prim::Capsule { a: Vec3::new(0.0, -0.5, 0.0), b: Vec3::new(0.0, 0.6, 0.0), r: 0.4 },
            Transform::at(Vec3::new(1.55, 0.7, 1.9)),
            red,
        )
        .smooth(0.5),
    );
    let _ = Combine::Union; // (combine modes are exercised across the scene)

    // Lights: a warm sun + a cool fill.
    scene.sun_dir = Vec3::new(0.55, 0.7, 0.35).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.25, 1.12, 0.95).scale(1.6)));
    scene.light(Light::directional(Vec3::new(-0.4, 0.5, -0.7), Vec3::new(0.35, 0.42, 0.6)));

    // Frame it with an orbiting camera and render.
    let cam = orbit_camera(Vec3::new(0.2, 0.85, 0.4), 8.5, 0.55, 0.32, 50f32.to_radians());
    let fb = render(&scene, &cam);

    let bytes = fb.to_bmp(mm3e_kit::color::Rgba::rgb8(0, 0, 0));
    let path = "spheres.bmp";
    std::fs::write(path, bytes).expect("write spheres.bmp");
    let full = std::fs::canonicalize(path).unwrap();
    println!("wrote {} ({}x{})", full.display(), scene.width, scene.height);
}
