//! Real-engine validation of the discovery-search "push the LOD knob" finding.
//!
//! The 4-hour cross-domain search converged on a marcher config that drives the screen-footprint
//! LOD knob (`Marcher::lod_footprint`) up to ~0.008 for its deepest field-eval cut. The shipped
//! doc comment only claims −7%…−16% at lower settings, so this measures what the REAL multithreaded
//! engine actually delivers across the LOD sweep: a single-threaded field-eval count (the
//! `cpu_profile` method) AND image error vs the exact lod=0 truth, on the deterministic profiler
//! scene. This turns the sim's headline into a measured field-eval / image-error tradeoff curve.
//!
//! Run: cargo run -p mm3e-orchestrator --example lod_validate --release

use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_kit::Material;
use mm3e_kit::{atoms, shade};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

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

/// Single-threaded field-eval count for a scene (the `cpu_profile` method): primary march, then
/// soft-shadow and ambient-occlusion for every hit. `lod_footprint` only affects the march phase,
/// so the total below is the honest, shadow+AO-diluted figure — not a march-only best case.
fn field_evals(scene: &Scene, w: u32, h: u32) -> u64 {
    let camera = cam();
    let counter = AtomicU64::new(0);
    let base = scene.field();
    let cf = |p: Vec3| {
        counter.fetch_add(1, Relaxed);
        base(p)
    };
    let m = scene.marcher;
    for (x, y) in atoms::scan(w, h) {
        let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
        let hit = m.march(&cf, &ray);
        if !hit.hit {
            continue;
        }
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
            let _ = m.soft_shadow(&cf, hit.pos + hit.normal.scale(0.01), ldir, ldist.min(m.max_dist), 24.0);
        }
        let _ = m.ambient_occlusion(&cf, hit.pos, hit.normal);
    }
    counter.load(Relaxed)
}

fn main() {
    let (w, h) = (480u32, 270u32);
    let camera = cam();

    // Exact truth: render and count at lod_footprint = 0 (no LOD widening).
    let mut truth_scene = scene_at(w, h);
    truth_scene.marcher.lod_footprint = 0.0;
    let truth = render(&truth_scene, &camera);
    let base_evals = field_evals(&truth_scene, w, h);

    println!("=== screen-footprint LOD validation on the REAL engine @ {w}x{h} AA1 ===");
    println!("baseline lod=0: {base_evals} field-evals (exact truth image)\n");
    println!("  lod_footprint |  field-evals |  Δevals | mean Δ/255 | max Δ/255");
    println!("  --------------+--------------+---------+------------+----------");

    for &lod in &[0.0f32, 0.002, 0.004, 0.006, 0.008, 0.010, 0.014] {
        let mut s = scene_at(w, h);
        s.marcher.lod_footprint = lod;
        let evals = field_evals(&s, w, h);
        let img = render(&s, &camera);

        let (mut sum, mut peak, mut n) = (0.0f64, 0.0f32, 0u64);
        for y in 0..h {
            for x in 0..w {
                let a = truth.pixel(x, y);
                let b = img.pixel(x, y);
                for (ca, cb) in [(a.r, b.r), (a.g, b.g), (a.b, b.b)] {
                    let d = ((ca - cb) * 255.0).abs();
                    sum += d as f64;
                    if d > peak {
                        peak = d;
                    }
                    n += 1;
                }
            }
        }
        let mean = (sum / n as f64) as f32;
        let dpct = (evals as f32 - base_evals as f32) / base_evals as f32 * 100.0;
        println!("  {lod:>13.3} | {evals:>12} | {dpct:>6.1}% | {mean:>10.3} | {peak:>9.2}");
    }
}
