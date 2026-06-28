//! Feature gallery: every SDF primitive plus the domain-operator vocabulary (twist, onion,
//! round, mirror, smooth-union) on one stage, lit with an area light. Shows the breadth of the
//! modeling kit — the kind of authoring surface a real SDF engine exposes.
//!
//! Run: cargo run -p mm3e-orchestrator --example gallery --release

use mm3e_kit::color::{Material, Rgba};
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};

fn main() {
    let mut scene = Scene::new(1100, 560);
    scene.aa = 2;
    scene.bounces = 2;
    scene.marcher.step_scale = 0.6; // the twisted column uses a non-Lipschitz domain warp
    scene.fog_density = 0.01;

    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6).specular(0.2));
    let mut palette = Vec::new();
    for c in [
        Vec3::new(0.85, 0.25, 0.28),
        Vec3::new(0.95, 0.6, 0.2),
        Vec3::new(0.95, 0.82, 0.3),
        Vec3::new(0.35, 0.75, 0.4),
        Vec3::new(0.3, 0.6, 0.85),
        Vec3::new(0.55, 0.4, 0.8),
        Vec3::new(0.9, 0.5, 0.7),
    ] {
        palette.push(scene.material(Material::solid(c).roughness(0.3).specular(0.7)));
    }
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.28)).metallic(1.0).roughness(0.2).reflective(0.5));

    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));

    // Front row: the primitive zoo, evenly spaced along X.
    let row_z = 1.6;
    let prims: [Prim; 7] = [
        Prim::Sphere { r: 0.6 },
        Prim::Box { half: Vec3::splat(0.55) },
        Prim::Torus { major: 0.5, minor: 0.2 },
        Prim::Cone { r1: 0.6, r2: 0.05, h: 1.1 },
        Prim::Ellipsoid { r: Vec3::new(0.4, 0.65, 0.5) },
        Prim::Octahedron { s: 0.8 },
        Prim::HexPrism { r: 0.55, h: 0.5 },
    ];
    for (i, &prim) in prims.iter().enumerate() {
        let x = -3.6 + i as f32 * 1.2;
        scene.add(Object::new(prim, Transform::at(Vec3::new(x, 0.7, row_z)), palette[i]));
    }

    // Back row: domain-operator showcase.
    // A twisted gold column.
    scene.add(
        Object::new(Prim::Box { half: Vec3::new(0.45, 1.3, 0.45) }, Transform::at(Vec3::new(-2.6, 1.3, -1.4)), gold)
            .twist(0.9),
    );
    // A hollow (onion) shell sphere.
    scene.add(Object::new(Prim::Sphere { r: 0.9 }, Transform::at(Vec3::new(-0.6, 1.0, -1.4)), palette[4]).onion(0.06));
    // A heavily rounded box.
    scene.add(
        Object::new(Prim::Box { half: Vec3::splat(0.8) }, Transform::at(Vec3::new(1.4, 0.95, -1.4)), palette[1])
            .round(0.35),
    );
    // A mirrored + smooth-blended twin capsule blob.
    scene.add(Object::new(Prim::Sphere { r: 0.7 }, Transform::at(Vec3::new(3.2, 0.9, -1.4)), palette[3]));
    scene.add(
        Object::new(
            Prim::Capsule { a: Vec3::new(0.0, -0.6, 0.0), b: Vec3::new(0.0, 0.7, 0.0), r: 0.35 },
            Transform::at(Vec3::new(3.7, 0.9, -1.4)).rotated(Mat3::from_euler(0.0, 0.0, 0.5)),
            palette[3],
        )
        .smooth(0.5),
    );

    scene.sun_dir = Vec3::new(0.4, 0.8, 0.45).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.2, 1.12, 0.98).scale(2.2)).soft(0.04));
    scene.light(Light::sphere(Vec3::new(-3.0, 4.0, 3.0), Vec3::new(0.8, 0.85, 1.0).scale(40.0), 1.0));

    let cam = orbit_camera(Vec3::new(0.0, 0.75, 0.2), 8.6, 0.58, 0.40, 52f32.to_radians());
    let fb = render(&scene, &cam);
    std::fs::write("gallery.bmp", fb.to_bmp(Rgba::rgb8(0, 0, 0))).expect("write");
    println!("wrote {} ({}x{})", std::fs::canonicalize("gallery.bmp").unwrap().display(), scene.width, scene.height);
}
