//! Overshoot probe — measures how deep inside a surface the marcher's accepted hits land.
//!
//! A correct sphere trace accepts a hit with `|field(hit.pos)| ≈ eps`. A hit *buried* inside the
//! surface (field ≪ −eps) means a boosted step (over-relaxation / secant / subitize) tunneled
//! through the surface and the hit was accepted anyway. Buried hits are the root cause of the
//! chaotic self-shadowing documented in `src/lib.rs`'s
//! `mirror_and_round_and_onion_modifiers_are_dual_safe` test: the shadow ray starts 0.01 off the
//! recorded position, which is not enough to escape a surface it is buried 0.02+ inside.
//!
//! Run: cargo run -p mm3e-orchestrator --example overshoot_probe --release

use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_kit::Material;
use mm3e_orchestrator::{orbit_camera, Light, Object, Prim, Scene};

/// The profiler scene — representative solids.
fn solid_scene(w: u32, h: u32) -> Scene {
    let mut scene = Scene::new(w, h);
    scene.aa = 1;
    scene.bounces = 0;
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.18, 0.2)).specular(0.7).roughness(0.25));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-2.4, 1.0, 0.2)), red));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.85), radius: 0.18 },
        Transform::at(Vec3::new(0.2, 0.95, -0.6)).rotated(Mat3::from_euler(0.0, 0.6, 0.0)),
        red,
    ));
    scene.add(Object::new(Prim::Torus { major: 0.85, minor: 0.30 }, Transform::at(Vec3::new(2.6, 1.05, 0.4)), red));
    scene.sun_dir = Vec3::new(0.55, 0.7, 0.35).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.5)));
    scene
}

/// The thin-shell scene from the lib.rs test comment: onion(0.02) makes a shell only ~0.04 thick —
/// exactly the geometry a tunneling step buries a hit inside of.
fn shell_scene(w: u32, h: u32) -> Scene {
    let mut scene = Scene::new(w, h);
    scene.aa = 1;
    scene.bounces = 0;
    let red = scene.material(Material::solid(Vec3::new(0.8, 0.2, 0.2)).roughness(0.4));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, red));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-1.2, 1.0, 0.0)), red));
    scene.add(
        Object::new(Prim::Sphere { r: 0.3 }, Transform::at(Vec3::new(0.0, 0.9, 0.6)), red)
            .mirror(true, false, false)
            .round(0.1)
            .onion(0.02),
    );
    scene.sun_dir = Vec3::new(0.5, 0.7, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.5)));
    scene
}

fn probe(name: &str, scene: &Scene) {
    let field = scene.field();
    let camera = orbit_camera(Vec3::new(0.0, 0.8, 0.0), 7.0, 0.45, 0.3, 50f32.to_radians());
    let (w, h) = (scene.width, scene.height);
    let mut hits = 0u64;
    let mut buried = 0u64; // field at hit < -2·eps: the hit is inside the surface
    let mut deep = 0u64; // field at hit < -0.01: deeper than the shadow-ray lift-off
    let mut worst = 0.0f32;
    let mut sum_pen = 0.0f64;
    for y in 0..h {
        for x in 0..w {
            let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
            let hit = scene.marcher.march(&field, &ray);
            if !hit.hit {
                continue;
            }
            hits += 1;
            let d = field(hit.pos).dist;
            let eps_here = scene.marcher.eps * (1.0 + hit.t * 0.5);
            if d < -2.0 * eps_here {
                buried += 1;
                sum_pen += (-d) as f64;
                worst = worst.max(-d);
            }
            if d < -0.01 {
                deep += 1;
            }
        }
    }
    println!("{name}: {hits} hits");
    println!("  buried (field < -2*eps):        {buried} ({:.3}% of hits)", buried as f64 / hits as f64 * 100.0);
    println!("  deeper than shadow lift (0.01): {deep}");
    println!(
        "  worst penetration: {worst:.5}   mean (buried only): {:.5}",
        if buried > 0 { sum_pen / buried as f64 } else { 0.0 }
    );
}

fn main() {
    let (w, h) = (480u32, 270u32);
    probe("solid scene ", &solid_scene(w, h));
    probe("thin shells ", &shell_scene(w, h));
}
