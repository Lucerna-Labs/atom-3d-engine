//! Subitize economics under the corrected overlap guard: where does the empty-space leap
//! actually pay? Counts field evals with subitize off/on across framings from hit-dominated
//! (camera looking down at geometry) to miss-dominated (horizon framing, mostly sky rays).
//!
//! Run: cargo run -p mm3e-orchestrator --example subitize_econ --release

use mm3e_kit::camera::Camera;
use mm3e_kit::vec::{Transform, Vec3};
use mm3e_kit::{atoms, Material};
use mm3e_orchestrator::{orbit_camera, Light, Object, Prim, Scene};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

fn demo_scene(w: u32, h: u32) -> Scene {
    let mut scene = Scene::new(w, h);
    scene.aa = 1;
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.2, 0.2)).roughness(0.3));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-1.2, 1.0, 0.0)), red).round(0.1));
    scene.add(Object::new(Prim::Torus { major: 0.7, minor: 0.25 }, Transform::at(Vec3::new(1.2, 1.0, 0.0)), red));
    scene.add(
        Object::new(Prim::Box { half: Vec3::splat(0.6) }, Transform::at(Vec3::new(0.0, 0.6, 1.6)), red).onion(0.05),
    );
    scene.sun_dir = Vec3::new(0.5, 0.75, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(2.0)));
    scene
}

fn measure(scene: &Scene, c: &Camera, label: &str) {
    let (w, h) = (scene.width, scene.height);
    let run = |subitize: f32| -> (u64, u64, Vec<bool>) {
        let counter = AtomicU64::new(0);
        let base = scene.field();
        let cf = |p: Vec3| {
            counter.fetch_add(1, Relaxed);
            base(p)
        };
        let mut m = scene.marcher;
        m.subitize = subitize;
        let mut hits = Vec::with_capacity((w * h) as usize);
        for (x, y) in atoms::scan(w, h) {
            let ray = c.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
            hits.push(m.march(&cf, &ray).hit);
        }
        let n_hit = hits.iter().filter(|&&b| b).count() as u64;
        (counter.load(Relaxed), n_hit, hits)
    };
    let (e0, n_hit, h0) = run(0.0);
    let (e1, _, h1) = run(0.4);
    let agree = h0.iter().zip(&h1).filter(|(a, b)| a == b).count() as f32 / h0.len() as f32;
    let hit_pct = n_hit as f64 / (w * h) as f64 * 100.0;
    println!(
        "{label:<28} hits {hit_pct:5.1}%   evals {e0:>8} -> {e1:>8}  ({:+5.1}%)   hit-agree {agree:.4}",
        (e1 as f64 - e0 as f64) / e0 as f64 * 100.0
    );
}

fn main() {
    let (w, h) = (320u32, 180u32);
    let scene = demo_scene(w, h);
    // Hit-dominated: the tests/engine.rs framing (looking down into the scene).
    measure(
        &scene,
        &orbit_camera(Vec3::new(0.0, 0.8, 0.3), 7.0, 0.5, 0.3, 52f32.to_radians()),
        "test framing (down-look)",
    );
    // Level framing: horizon visible, roughly half sky.
    measure(
        &scene,
        &orbit_camera(Vec3::new(0.0, 1.2, 0.0), 9.0, 0.5, 0.02, 52f32.to_radians()),
        "level framing (half sky)",
    );
    // Miss-dominated: camera tilted up, mostly sky with geometry at the bottom.
    measure(
        &scene,
        &Camera::look_at(
            Vec3::new(0.0, 1.5, 8.0),
            Vec3::new(0.0, 4.5, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            52f32.to_radians(),
        ),
        "sky framing (miss-heavy)",
    );
}
