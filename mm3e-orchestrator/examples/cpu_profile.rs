//! CPU renderer profiler — proves *where* the per-frame cost goes before optimizing further.
//! Two views: (1) differential timing (toggle a feature off, measure the delta) and (2) a
//! single-threaded count of field evaluations broken down by phase (march / normal / shadow / AO).
//!
//! Run: cargo run -p mm3e-orchestrator --example cpu_profile --release

use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_kit::Material;
use mm3e_kit::{atoms, shade};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;

fn scene_at(w: u32, h: u32) -> Scene {
    let mut scene = Scene::new(w, h);
    scene.aa = 1;
    scene.bounces = 1;
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.18, 0.2)).specular(0.7).roughness(0.25));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.25)).metallic(1.0).roughness(0.18).reflective(0.5));
    let mirror = scene.material(Material::solid(Vec3::splat(0.95)).metallic(1.0).roughness(0.05).reflective(0.85));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-2.4, 1.0, 0.2)), mirror));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.85), radius: 0.18 },
        Transform::at(Vec3::new(0.2, 0.95, -0.6)).rotated(Mat3::from_euler(0.0, 0.6, 0.0)),
        gold,
    ));
    scene.add(Object::new(Prim::Torus { major: 0.85, minor: 0.30 }, Transform::at(Vec3::new(2.6, 1.05, 0.4)), red));
    scene.add(Object::new(Prim::Sphere { r: 0.7 }, Transform::at(Vec3::new(0.9, 0.7, 1.9)), red));
    scene.sun_dir = Vec3::new(0.55, 0.7, 0.35).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.25, 1.12, 0.95).scale(1.6)).soft(0.04));
    scene.light(Light::directional(Vec3::new(-0.4, 0.5, -0.7), Vec3::new(0.35, 0.42, 0.6)));
    scene
}

fn cam() -> mm3e_kit::camera::Camera {
    orbit_camera(Vec3::new(0.2, 0.85, 0.4), 8.5, 0.55, 0.32, 50f32.to_radians())
}

fn time_ms(scene: &Scene, frames: u32) -> f32 {
    let c = cam();
    let _ = render(scene, &c);
    let t0 = Instant::now();
    for _ in 0..frames {
        let _ = render(scene, &c);
    }
    t0.elapsed().as_secs_f32() * 1000.0 / frames as f32
}

fn main() {
    let (w, h) = (480u32, 270u32);
    println!("=== differential timing @ {w}x{h} AA1 (ms/frame, 20 frames) ===");
    let full = time_ms(&scene_at(w, h), 20);
    let no_shadow = time_ms(
        &{
            let mut s = scene_at(w, h);
            s.shadows = false;
            s
        },
        20,
    );
    let no_ao = time_ms(
        &{
            let mut s = scene_at(w, h);
            s.marcher.ao_samples = 0;
            s
        },
        20,
    );
    let no_bounce = time_ms(
        &{
            let mut s = scene_at(w, h);
            s.bounces = 0;
            s
        },
        20,
    );
    let bare = time_ms(
        &{
            let mut s = scene_at(w, h);
            s.shadows = false;
            s.marcher.ao_samples = 0;
            s.bounces = 0;
            s
        },
        20,
    );
    println!("  full (everything on)     {full:6.1}  (= {:.1} fps)", 1000.0 / full);
    println!("  primary march + shade    {bare:6.1}  ({:.0}% of full)", bare / full * 100.0);
    println!("  -> shadows cost          {:6.1}  ({:.0}%)", full - no_shadow, (full - no_shadow) / full * 100.0);
    println!("  -> ambient occlusion     {:6.1}  ({:.0}%)", full - no_ao, (full - no_ao) / full * 100.0);
    println!("  -> reflections (1 bounce){:6.1}  ({:.0}%)", full - no_bounce, (full - no_bounce) / full * 100.0);

    // --- field-evaluation count, single-threaded, broken down by phase ---
    println!("\n=== field evaluations @ {w}x{h} (one frame, single-threaded) ===");
    let scene = scene_at(w, h);
    let camera = cam();
    let counter = AtomicU64::new(0);
    let base = scene.field();
    let cf = |p: Vec3| {
        counter.fetch_add(1, Relaxed);
        base(p)
    };
    let m = scene.marcher;
    let (mut march, mut shadow, mut ao, mut hits, mut misses) = (0u64, 0u64, 0u64, 0u64, 0u64);
    for (x, y) in atoms::scan(w, h) {
        let ray = m_ray(&camera, x, y, w, h);
        let before = counter.load(Relaxed);
        let hit = m.march(&cf, &ray);
        march += counter.load(Relaxed) - before;
        if !hit.hit {
            misses += 1;
            continue;
        }
        hits += 1;
        for light in &scene.lights {
            let (ldir, ldist) = if light.directional {
                (light.vec, f32::INFINITY)
            } else {
                let d = light.vec - hit.pos;
                (d.normalize(), d.length())
            };
            if shade::lambert(hit.normal, ldir) <= 0.0 {
                continue;
            }
            let b = counter.load(Relaxed);
            let _ = m.soft_shadow(&cf, hit.pos + hit.normal.scale(0.01), ldir, ldist.min(m.max_dist), 24.0);
            shadow += counter.load(Relaxed) - b;
        }
        let b = counter.load(Relaxed);
        let _ = m.ambient_occlusion(&cf, hit.pos, hit.normal);
        ao += counter.load(Relaxed) - b;
    }
    let total = counter.load(Relaxed);
    let pixels = (w * h) as u64;
    println!("  pixels {pixels}  hits {hits}  misses {misses}");
    println!("  total field evals      {total:>10}  ({:.0}/pixel)", total as f32 / pixels as f32);
    println!("  march (incl. normals)  {march:>10}  ({:.0}%)", march as f32 / total as f32 * 100.0);
    println!("  shadows                {shadow:>10}  ({:.0}%)", shadow as f32 / total as f32 * 100.0);
    println!("  ambient occlusion      {ao:>10}  ({:.0}%)", ao as f32 / total as f32 * 100.0);
    println!("  (reflections excluded from this single-threaded count)");
}

fn m_ray(c: &mm3e_kit::camera::Camera, x: u32, y: u32, w: u32, h: u32) -> mm3e_kit::march::Ray {
    c.ray(x as f32 + 0.5, y as f32 + 0.5, w, h)
}
