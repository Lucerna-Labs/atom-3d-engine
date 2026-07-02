//! Global-illumination demo: a white room with a red wall and a blue wall. With GI baked, the
//! white floor and central sphere pick up red/blue **color bleed** — the classic proof that
//! indirect light is working. Toggle `BAKE` to compare against direct-only lighting.
//!
//! Run: cargo run -p mm3e-orchestrator --example gi_demo --release

use mm3e_kit::color::{Material, Rgba};
use mm3e_kit::vec::{Transform, Vec3};
use mm3e_orchestrator::{render, Light, Object, Prim, Scene};

fn main() {
    const BAKE: bool = true;

    let mut scene = Scene::new(800, 600);
    scene.aa = 3;
    scene.bounces = 1;
    scene.fog_density = 0.0;
    scene.sky_ambient = Vec3::splat(0.18); // dim sky so bounce light reads clearly
    scene.ambient = Vec3::splat(0.0);

    let white = scene.material(Material::solid(Vec3::splat(0.85)).roughness(0.7));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.12, 0.12)).roughness(0.7));
    let blue = scene.material(Material::solid(Vec3::new(0.12, 0.2, 0.85)).roughness(0.7));
    let chrome = scene.material(Material::solid(Vec3::splat(0.9)).metallic(1.0).roughness(0.12).reflective(0.7));

    // Room: floor, back wall, ceiling, red left wall, blue right wall (thick boxes).
    let wall = |c: Vec3, half: Vec3, m: u32| Object::new(Prim::Box { half }, Transform::at(c), m);
    scene.add(wall(Vec3::new(0.0, -0.1, 0.0), Vec3::new(4.0, 0.1, 4.0), white)); // floor
    scene.add(wall(Vec3::new(0.0, 4.1, 0.0), Vec3::new(4.0, 0.1, 4.0), white)); // ceiling
    scene.add(wall(Vec3::new(0.0, 2.0, -4.1), Vec3::new(4.0, 4.0, 0.1), white)); // back
    scene.add(wall(Vec3::new(-4.1, 2.0, 0.0), Vec3::new(0.1, 4.0, 4.0), red)); // left (red)
    scene.add(wall(Vec3::new(4.1, 2.0, 0.0), Vec3::new(0.1, 4.0, 4.0), blue)); // right (blue)

    // A white sphere and a chrome sphere to catch the bleed.
    scene.add(Object::new(Prim::Sphere { r: 1.1 }, Transform::at(Vec3::new(-1.3, 1.1, 0.4)), white));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(1.5, 1.0, -0.6)), chrome));

    // A ceiling light (emissive bar) + a soft key light.
    let lamp = scene.material(Material::solid(Vec3::splat(1.0)).emissive(Vec3::splat(3.0)));
    scene.add(Object::new(
        Prim::Box { half: Vec3::new(1.4, 0.06, 1.4) },
        Transform::at(Vec3::new(0.0, 3.9, 0.0)),
        lamp,
    ));
    scene.sun_dir = Vec3::new(0.0, 1.0, 0.2).normalize();
    scene.light(Light::sphere(Vec3::new(0.0, 3.7, 0.0), Vec3::splat(14.0), 1.2));

    if BAKE {
        scene.bake_gi((20, 14, 20), 5);
    }

    // Camera looking into the room.
    let cam = mm3e_kit::camera::Camera::look_at(
        Vec3::new(0.0, 2.0, 7.5),
        Vec3::new(0.0, 1.6, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        50f32.to_radians(),
    );
    let fb = render(&scene, &cam);
    std::fs::write("gi_demo.bmp", fb.to_bmp(Rgba::rgb8(0, 0, 0))).expect("write");
    println!("wrote {} ({}x{})", std::fs::canonicalize("gi_demo.bmp").unwrap().display(), scene.width, scene.height);
}
