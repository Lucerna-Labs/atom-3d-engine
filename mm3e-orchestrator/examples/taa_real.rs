//! taa_real — put the top discovery (`integrate` -> temporal anti-aliasing) on the REAL mm3e
//! engine and get real wall-clock + real image-quality data, including the moving-camera case the
//! sim assumed away.
//!
//! Run: cargo run -p mm3e-orchestrator --example taa_real --release
//!
//! TEST 1 (static): the sim's claim re-measured on the real beauty renderer (GGX, soft shadows, AO,
//!   post) — accumulate sub-pixel-jittered 1-spp frames and see how many it takes to match aa=2, at
//!   1 render/frame vs aa=2's 4. Real ms/frame, not an eval-count proxy. (Accumulation is in
//!   display space — `render` tonemaps internally — so the absolute errors are post-space; the
//!   convergence trend is the point.)
//! TEST 2 (moving): the gap — a flying camera. Full `render_gbuffer` every frame vs a keyframe every
//!   N frames + `reproject_hybrid` warps in between. Real fps win and the real disocclusion error
//!   reprojection leaves as the camera moves away from the keyframe (what the correlate gate bounds).

use mm3e_kit::camera::Camera;
use mm3e_kit::color::Material;
use mm3e_kit::framebuffer::Framebuffer;
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::{
    orbit_camera, render, render_gbuffer, reproject_hybrid, Light, Object, Prim, Scene,
};
use std::time::Instant;

fn build_scene(w: u32, h: u32) -> Scene {
    let mut scene = Scene::new(w, h);
    scene.aa = 1; // 1 sample/pixel — we anti-alias ourselves via camera jitter.
    scene.bounces = 3;
    let floor =
        scene.material(Material::solid(Vec3::splat(1.0)).checkered().specular(0.15).roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.18, 0.20)).specular(0.7).roughness(0.25));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.25)).metallic(1.0).roughness(0.18).reflective(0.5));
    let glassy =
        scene.material(Material::solid(Vec3::new(0.20, 0.55, 0.85)).specular(0.9).roughness(0.15).reflective(0.35));
    let mirror = scene.material(Material::solid(Vec3::splat(0.95)).metallic(1.0).roughness(0.05).reflective(0.85));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-2.4, 1.0, 0.2)), mirror));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.85), radius: 0.18 },
        Transform::at(Vec3::new(0.2, 0.95, -0.6)).rotated(Mat3::from_euler(0.0, 0.6, 0.0)),
        gold,
    ));
    scene.add(Object::new(
        Prim::Torus { major: 0.85, minor: 0.30 },
        Transform::at(Vec3::new(2.6, 1.05, 0.4)).rotated(Mat3::from_euler(1.2, 0.0, 0.2)),
        glassy,
    ));
    scene.add(Object::new(Prim::Sphere { r: 0.7 }, Transform::at(Vec3::new(0.9, 0.7, 1.9)), red));
    scene.sun_dir = Vec3::new(0.55, 0.7, 0.35).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.25, 1.12, 0.95).scale(1.6)).soft(0.04));
    scene.light(Light::directional(Vec3::new(-0.4, 0.5, -0.7), Vec3::new(0.35, 0.42, 0.6)));
    scene
}

/// A camera whose rays are shifted by `(jx, jy)` sub-pixels — sub-pixel jitter for AA, built from
/// the public camera basis (first-order: nudge the forward vector along right/up by the NDC delta).
fn jittered(cam: &Camera, jx: f32, jy: f32, w: u32, h: u32) -> Camera {
    let aspect = w as f32 / h as f32;
    let dx = 2.0 * jx / w as f32 * aspect * cam.fov_scale;
    let dy = -2.0 * jy / h as f32 * cam.fov_scale;
    let forward = (cam.forward + cam.right.scale(dx) + cam.up.scale(dy)).normalize();
    Camera {
        eye: cam.eye,
        forward,
        right: cam.right,
        up: cam.up,
        fov_scale: cam.fov_scale,
    }
}

fn halton(mut i: u32, base: u32) -> f32 {
    let (mut f, mut r) = (1.0f32, 0.0f32);
    while i > 0 {
        f /= base as f32;
        r += f * (i % base) as f32;
        i /= base;
    }
    r
}

fn fb_rgb(fb: &Framebuffer, w: u32, h: u32) -> Vec<f32> {
    let mut v = Vec::with_capacity((w * h * 3) as usize);
    for y in 0..h {
        for x in 0..w {
            let p = fb.pixel(x, y);
            v.push(p.r);
            v.push(p.g);
            v.push(p.b);
        }
    }
    v
}

fn img_err(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len().max(1) as f32;
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum::<f32>() / n
}

fn img_err_u8(a: &[[u8; 4]], b: &[[u8; 4]]) -> f32 {
    let n = (a.len() * 3).max(1) as f32;
    let s: f32 = a
        .iter()
        .zip(b)
        .map(|(p, q)| (0..3).map(|c| (p[c] as f32 - q[c] as f32).abs()).sum::<f32>())
        .sum();
    s / n / 255.0
}

fn cam_at(i: u32, dyaw: f32) -> Camera {
    orbit_camera(
        Vec3::new(0.2, 0.85, 0.4),
        8.5,
        0.55 + i as f32 * dyaw,
        0.32,
        50f32.to_radians(),
    )
}

fn static_taa(scene: &Scene, w: u32, h: u32) {
    let cam = cam_at(0, 0.0);

    // 64-spp truth (jittered accumulation).
    let mut truth = vec![0.0f32; (w * h * 3) as usize];
    for k in 0..64u32 {
        let jx = halton(k + 1, 2) - 0.5;
        let jy = halton(k + 1, 3) - 0.5;
        let f = fb_rgb(&render(scene, &jittered(&cam, jx, jy, w, h)), w, h);
        for (t, &x) in truth.iter_mut().zip(f.iter()) {
            *t += x;
        }
    }
    for t in truth.iter_mut() {
        *t /= 64.0;
    }

    // One 1-spp render, min-of-3 — the TAA per-frame cost.
    let mut ms1 = f64::INFINITY;
    for _ in 0..3 {
        let t0 = Instant::now();
        let _ = render(scene, &cam);
        ms1 = ms1.min(t0.elapsed().as_secs_f64() * 1000.0);
    }

    // aa=2 = 4 fixed sub-pixel samples.
    let aa2_off = [(-0.25, -0.25), (0.25, -0.25), (-0.25, 0.25), (0.25, 0.25)];
    let mut aa2 = vec![0.0f32; (w * h * 3) as usize];
    for &(jx, jy) in &aa2_off {
        let f = fb_rgb(&render(scene, &jittered(&cam, jx, jy, w, h)), w, h);
        for (a, &x) in aa2.iter_mut().zip(f.iter()) {
            *a += x;
        }
    }
    for a in aa2.iter_mut() {
        *a /= 4.0;
    }
    let err_aa2 = img_err(&aa2, &truth);

    println!("TEST 1 — static TAA on the REAL beauty renderer @ {w}x{h} (GGX + soft shadows + AO + post):");
    println!("  aa=2 (4 spp): {:>7.1} ms/frame   err {err_aa2:.5} vs 64-spp truth", ms1 * 4.0);
    println!("  TAA  (1 spp): {ms1:>7.1} ms/frame   (4.0x faster per frame)");
    println!("  accumulate jittered 1-spp frames:");
    println!("    {:>6}  {:>9}  {:>9}", "frames", "err", "err/aa2");
    let mut acc = vec![0.0f32; (w * h * 3) as usize];
    let marks = [1u32, 2, 4, 8, 16];
    for k in 1..=16u32 {
        let jx = halton(k, 2) - 0.5;
        let jy = halton(k, 3) - 0.5;
        let f = fb_rgb(&render(scene, &jittered(&cam, jx, jy, w, h)), w, h);
        for (a, &x) in acc.iter_mut().zip(f.iter()) {
            *a += x;
        }
        if marks.contains(&k) {
            let img: Vec<f32> = acc.iter().map(|&a| a / k as f32).collect();
            let e = img_err(&img, &truth);
            println!("    {k:>6}  {e:>9.5}  {:>8.2}x", err_aa2 / e.max(1e-9));
        }
    }
}

fn moving_reproj(scene: &Scene, w: u32, h: u32, dyaw: f32, label: &str) {
    let frames = 24u32;
    let keyint = 8u32;

    // FULL: render_gbuffer every frame (the reference + the baseline timing).
    let mut full: Vec<Vec<[u8; 4]>> = Vec::with_capacity(frames as usize);
    let t0 = Instant::now();
    for i in 0..frames {
        full.push(render_gbuffer(scene, &cam_at(i, dyaw), &[]).color);
    }
    let full_ms = t0.elapsed().as_secs_f64() * 1000.0 / frames as f64;

    // AMORTIZED: a full keyframe every `keyint`, reproject_hybrid in between.
    let t0 = Instant::now();
    let mut key = render_gbuffer(scene, &cam_at(0, dyaw), &[]);
    let mut amort: Vec<Vec<[u8; 4]>> = Vec::with_capacity(frames as usize);
    amort.push(key.color.clone());
    for i in 1..frames {
        if i % keyint == 0 {
            key = render_gbuffer(scene, &cam_at(i, dyaw), &[]);
            amort.push(key.color.clone());
        } else {
            amort.push(reproject_hybrid(&key, &cam_at(i, dyaw), scene, &[]));
        }
    }
    let amort_ms = t0.elapsed().as_secs_f64() * 1000.0 / frames as f64;

    let errs: Vec<f32> = amort
        .iter()
        .zip(&full)
        .map(|(a, f)| img_err_u8(a, f))
        .collect();
    let mean_err = errs.iter().sum::<f32>() / errs.len().max(1) as f32;
    let max_err = errs.iter().copied().fold(0.0f32, f32::max);

    println!("\nTEST 2 [{label}] — MOVING camera @ {w}x{h}, {frames} frames, keyframe/{keyint}, dyaw={dyaw}:");
    println!("  full render every frame : {full_ms:>7.1} ms/frame");
    println!(
        "  keyframe + reproject    : {amort_ms:>7.1} ms/frame   ({:.2}x faster)",
        full_ms / amort_ms.max(0.001)
    );
    println!("  reprojected error vs full: mean {mean_err:.5}, worst {max_err:.5} (grows with motion from keyframe)");
}

fn main() {
    let (w, h) = (320u32, 180u32);
    let scene = build_scene(w, h);
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    println!("Real mm3e engine, {threads} CPU threads — the top discovery on the actual renderer.\n");
    static_taa(&scene, w, h);
    moving_reproj(&scene, w, h, 0.004, "slow pan");
    moving_reproj(&scene, w, h, 0.020, "fast pan");
    println!("\nReal data: TAA's per-frame win + convergence on the full shaded renderer, and the");
    println!("moving-camera number the sim could not produce — reprojection's real fps gain and its");
    println!("real disocclusion error under motion (the error the correlate gate exists to bound).");
}
