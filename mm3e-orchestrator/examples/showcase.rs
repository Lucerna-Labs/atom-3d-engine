//! Showcase scene: every primitive and every CSG mode in one frame — smooth-min blobs, a
//! box with a sphere carved out of it (subtraction), an emissive light bar, reflections, and
//! higher anti-aliasing. Demonstrates the full feature set of the dependency-free renderer.
//!
//! Run: cargo run -p mm3e-orchestrator --example showcase --release

use mm3e_kit::color::{Material, Rgba};
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};

fn main() {
    let mut scene = Scene::new(1024, 576);
    scene.aa = 3;
    scene.bounces = 2;
    scene.fog_density = 0.008;
    scene.ambient = Vec3::splat(0.08);

    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.7).specular(0.2));
    let jade = scene.material(Material::solid(Vec3::new(0.16, 0.62, 0.42)).specular(0.8).roughness(0.2).reflective(0.25));
    let copper = scene.material(Material::solid(Vec3::new(0.92, 0.55, 0.32)).specular(1.0).roughness(0.18).reflective(0.45));
    let chrome = scene.material(Material::solid(Vec3::splat(0.95)).specular(1.0).roughness(0.04).reflective(0.9));
    let plum = scene.material(Material::solid(Vec3::new(0.55, 0.22, 0.6)).specular(0.7).roughness(0.3));
    let lamp = scene.material(Material::solid(Vec3::splat(1.0)).emissive(Vec3::new(1.6, 1.3, 0.7).scale(3.0)));

    // Ground.
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));

    // Smooth-min blob: three spheres melting together.
    scene.add(Object::new(Prim::Sphere { r: 0.8 }, Transform::at(Vec3::new(-3.2, 0.85, 0.0)), jade));
    scene.add(Object::new(Prim::Sphere { r: 0.6 }, Transform::at(Vec3::new(-2.4, 0.65, 0.5)), jade).smooth(0.6));
    scene.add(Object::new(Prim::Sphere { r: 0.5 }, Transform::at(Vec3::new(-2.7, 1.4, -0.2)), jade).smooth(0.6));

    // Subtraction: a copper box with a sphere bored out of its face.
    scene.add(Object::new(
        Prim::Box { half: Vec3::new(0.9, 0.9, 0.9) },
        Transform::at(Vec3::new(0.0, 0.9, 0.0)).rotated(Mat3::from_euler(0.0, 0.5, 0.0)),
        copper,
    ));
    scene.add(
        Object::new(Prim::Sphere { r: 0.62 }, Transform::at(Vec3::new(0.0, 0.9, 0.95)), copper).subtract(),
    );

    // A chrome torus.
    scene.add(Object::new(
        Prim::Torus { major: 0.9, minor: 0.28 },
        Transform::at(Vec3::new(3.0, 1.0, -0.3)).rotated(Mat3::from_euler(1.1, 0.4, 0.0)),
        chrome,
    ));

    // A plum cylinder pedestal with a sphere on top.
    scene.add(Object::new(Prim::Cylinder { h: 0.6, r: 0.7 }, Transform::at(Vec3::new(2.4, 0.6, 2.2)), plum));
    scene.add(Object::new(Prim::Sphere { r: 0.55 }, Transform::at(Vec3::new(2.4, 1.7, 2.2)), chrome));

    // An emissive light bar floating above, casting warm light.
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::new(2.4, 0.08, 0.18), radius: 0.08 },
        Transform::at(Vec3::new(0.0, 3.6, -1.0)),
        lamp,
    ));

    // Lighting.
    scene.sun_dir = Vec3::new(0.5, 0.75, 0.42).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.2, 1.1, 0.95).scale(1.7)));
    scene.light(Light::point(Vec3::new(0.0, 3.6, -1.0), Vec3::new(1.6, 1.3, 0.7).scale(8.0)));
    scene.light(Light::directional(Vec3::new(-0.5, 0.4, -0.6), Vec3::new(0.3, 0.38, 0.55)));

    let cam = orbit_camera(Vec3::new(0.0, 1.0, 0.3), 10.0, 0.62, 0.30, 48f32.to_radians());
    let fb = render(&scene, &cam);

    let path = "showcase.bmp";
    std::fs::write(path, fb.to_bmp(Rgba::rgb8(0, 0, 0))).expect("write showcase.bmp");
    let full = std::fs::canonicalize(path).unwrap();
    println!("wrote {} ({}x{})", full.display(), scene.width, scene.height);
}
