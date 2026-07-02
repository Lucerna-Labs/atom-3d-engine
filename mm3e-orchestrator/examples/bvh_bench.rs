//! BVH world-fold benchmark: the linear left fold vs the segmented tree fold, on scenes of
//! growing object count. The two produce bit-identical fields (proven in `tests/engine.rs`);
//! this measures the only thing that differs — evaluation cost. Grids of spheres over a floor
//! plane, rendered at a fixed viewpoint, plus a raw field-evaluation throughput probe.
//!
//! Run: cargo run -p mm3e-orchestrator --example bvh_bench --release

use mm3e_kit::vec::{Transform, Vec3};
use mm3e_kit::Material;
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};
use std::time::Instant;

fn grid_scene(side: u32, w: u32, h: u32) -> Scene {
    let mut scene = Scene::new(w, h);
    scene.aa = 1;
    scene.bounces = 0;
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let colors = [
        Vec3::new(0.85, 0.2, 0.2),
        Vec3::new(0.2, 0.65, 0.3),
        Vec3::new(0.25, 0.4, 0.85),
        Vec3::new(0.9, 0.75, 0.25),
    ];
    let mats: Vec<u32> = colors.iter().map(|&c| scene.material(Material::solid(c).roughness(0.35))).collect();
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    let spacing = 1.6f32;
    let half = (side as f32 - 1.0) * spacing * 0.5;
    for gz in 0..side {
        for gx in 0..side {
            let x = gx as f32 * spacing - half;
            let z = gz as f32 * spacing - half;
            // Deterministic size/height variation so the scene is not trivially regular.
            let r = 0.3 + 0.25 * (((gx * 7 + gz * 13) % 5) as f32 / 4.0);
            scene.add(Object::new(
                Prim::Sphere { r },
                Transform::at(Vec3::new(x, r, z)),
                mats[((gx + gz) % 4) as usize],
            ));
        }
    }
    scene.sun_dir = Vec3::new(0.5, 0.7, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.5)).soft(0.04));
    scene
}

fn main() {
    let (w, h) = (480u32, 270u32);
    println!("{:>8}  {:>9}  {:>12}  {:>12}  {:>8}   (render ms/frame at {w}x{h})", "objects", "grid", "linear", "bvh", "speedup");
    for side in [2u32, 4, 6, 8, 12, 16] {
        let scene = grid_scene(side, w, h);
        let n = scene.objects.len();
        let camera = orbit_camera(Vec3::ZERO, (side as f32) * 2.2 + 4.0, 0.5, 0.5, 50f32.to_radians());

        // Field-eval throughput on a fixed sample set (single-threaded, isolates the fold cost).
        let probes: Vec<Vec3> = (0..40_000)
            .map(|i| {
                let t = i as f32 * 0.618_034;
                Vec3::new((t.sin() * 9.0) % 9.0, (i % 97) as f32 * 0.06, (t.cos() * 9.0) % 9.0)
            })
            .collect();
        let lin = scene.world_linear();
        let t0 = Instant::now();
        let mut sink = 0.0f32;
        for p in &probes {
            sink += lin(*p).dist;
        }
        let lin_ns = t0.elapsed().as_nanos() as f64 / probes.len() as f64;
        let fast = scene.field();
        let t0 = Instant::now();
        for p in &probes {
            sink += fast(*p).dist;
        }
        let bvh_ns = t0.elapsed().as_nanos() as f64 / probes.len() as f64;
        std::hint::black_box(sink);

        // Whole-frame render (multithreaded) through the shipped path (the BVH plan).
        let t0 = Instant::now();
        let _ = render(&scene, &camera);
        let frame_ms = t0.elapsed().as_secs_f64() * 1000.0;

        println!(
            "{n:>8}  {side:>4}x{side:<4}  {lin_ns:>9.0} ns  {bvh_ns:>9.0} ns  {:>7.2}x   render {frame_ms:>7.1} ms",
            lin_ns / bvh_ns
        );
    }
}
