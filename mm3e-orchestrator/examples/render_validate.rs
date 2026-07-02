//! The renderer validation harness (renderer report, next-steps #3/#4): every marcher
//! configuration measured against a conservative ground truth on the same scenes, with real
//! image metrics — field evals, hit agreement (with a silhouette-band breakdown), depth error,
//! normal error, and material mismatch — instead of "fewer steps" alone. This is the harness
//! that keeps speed knobs honest: a config that cuts evals by bending depth or flipping
//! silhouette pixels shows it here.
//!
//! Ground truth: pure conservative sphere tracing (no over-relaxation, no leaps, no secant) with
//! a deep step budget. Every other config shares the scene's eps so depth is comparable.
//!
//! Run: cargo run -p mm3e-orchestrator --example render_validate --release

use mm3e_kit::camera::Camera;
use mm3e_kit::march::Marcher;
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_kit::Material;
use mm3e_orchestrator::{orbit_camera, Light, Object, Prim, Scene};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

const W: u32 = 480;
const H: u32 = 270;

struct Config {
    name: &'static str,
    marcher: Marcher,
    /// Use the exact dual-number gradient instead of the tetrahedron stencil (dual-safe scenes).
    dual_normals: bool,
}

fn configs() -> Vec<Config> {
    let base = Marcher::default();
    vec![
        Config { name: "baseline (omega 1.4)", marcher: base, dual_normals: false },
        Config { name: "dual normals", marcher: base, dual_normals: true },
        Config { name: "secant on", marcher: Marcher { secant: true, ..base }, dual_normals: false },
        Config { name: "subitize 0.2", marcher: Marcher { subitize: 0.2, ..base }, dual_normals: false },
        Config { name: "subitize 0.4", marcher: Marcher { subitize: 0.4, ..base }, dual_normals: false },
        Config { name: "lod 0.006", marcher: Marcher { lod_footprint: 0.006, ..base }, dual_normals: false },
    ]
}

/// Conservative reference: no boosted steps at all (step_scale just under 1 disables
/// over-relaxation, subitize and secant), deep budget.
fn ground_truth_marcher() -> Marcher {
    Marcher { step_scale: 0.999_999, max_steps: 1024, ..Marcher::default() }
}

fn solid_scene() -> Scene {
    let mut scene = Scene::new(W, H);
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.18, 0.2)).roughness(0.25));
    let gold = scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.25)).metallic(1.0).roughness(0.18));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-2.4, 1.0, 0.2)), gold));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.85), radius: 0.18 },
        Transform::at(Vec3::new(0.2, 0.95, -0.6)).rotated(Mat3::from_euler(0.0, 0.6, 0.0)),
        red,
    ));
    scene.add(Object::new(Prim::Torus { major: 0.85, minor: 0.3 }, Transform::at(Vec3::new(2.6, 1.05, 0.4)), red));
    scene.sun_dir = Vec3::new(0.55, 0.7, 0.35).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.5)));
    scene
}

fn shell_scene() -> Scene {
    let mut scene = Scene::new(W, H);
    let red = scene.material(Material::solid(Vec3::new(0.8, 0.2, 0.2)).roughness(0.4));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, red));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-1.2, 1.0, 0.0)), red).onion(0.02));
    scene.add(
        Object::new(Prim::Sphere { r: 0.3 }, Transform::at(Vec3::new(0.6, 0.9, 0.6)), red)
            .mirror(true, false, false)
            .round(0.1)
            .onion(0.02),
    );
    scene.sun_dir = Vec3::new(0.5, 0.7, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.5)));
    scene
}

fn sky_scene() -> (Scene, Camera) {
    let scene = solid_scene();
    let cam = Camera::look_at(
        Vec3::new(0.0, 1.5, 8.0),
        Vec3::new(0.0, 4.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        52f32.to_radians(),
    );
    (scene, cam)
}

struct Sample {
    hit: bool,
    t: f32,
    normal: Vec3,
    mat: u32,
}

/// March every pixel with `marcher`, returning per-pixel samples and the field-eval count.
fn march_frame(scene: &Scene, camera: &Camera, marcher: &Marcher, dual: bool) -> (Vec<Sample>, u64) {
    let counter = AtomicU64::new(0);
    let base = scene.field();
    let field = |p: Vec3| {
        counter.fetch_add(1, Relaxed);
        base(p)
    };
    let mut out = Vec::with_capacity((W * H) as usize);
    for y in 0..H {
        for x in 0..W {
            let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, W, H);
            let hit = if dual {
                marcher.march_with(&field, |p| scene.normal_dual(p), &ray)
            } else {
                marcher.march(&field, &ray)
            };
            out.push(Sample { hit: hit.hit, t: hit.t, normal: hit.normal, mat: hit.mat });
        }
    }
    (out, counter.load(Relaxed))
}

/// Pixels whose 3×3 reference neighborhood mixes hits and misses — the silhouette band, where
/// aggressive marching does its damage.
fn silhouette_band(reference: &[Sample]) -> Vec<bool> {
    let (w, h) = (W as i32, H as i32);
    let mut band = vec![false; reference.len()];
    for y in 0..h {
        for x in 0..w {
            let here = reference[(y * w + x) as usize].hit;
            let mut mixed = false;
            for dy in -1..=1i32 {
                for dx in -1..=1i32 {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx >= 0 && ny >= 0 && nx < w && ny < h && reference[(ny * w + nx) as usize].hit != here {
                        mixed = true;
                    }
                }
            }
            band[(y * w + x) as usize] = mixed;
        }
    }
    band
}

fn validate(scene_name: &str, scene: &Scene, camera: &Camera) {
    let (reference, ref_evals) = march_frame(scene, camera, &ground_truth_marcher(), false);
    let band = silhouette_band(&reference);
    let band_n = band.iter().filter(|&&b| b).count().max(1);

    println!("\n{scene_name} — ground truth: conservative march, {ref_evals} field evals");
    println!(
        "{:<22} {:>10} {:>8} {:>10} {:>10} {:>10} {:>9} {:>8}",
        "config", "evals", "vs ref", "hit-agree", "sil-agree", "depth-err", "norm-err", "mat-mis"
    );
    for cfg in configs() {
        if cfg.dual_normals && !scene.is_dual_safe() {
            println!("{:<22} {:>10}", cfg.name, "(scene not dual-safe, skipped)");
            continue;
        }
        let (frame, evals) = march_frame(scene, camera, &cfg.marcher, cfg.dual_normals);
        let (mut agree, mut band_agree) = (0usize, 0usize);
        let (mut depth_sum, mut depth_n, mut depth_max) = (0.0f64, 0u64, 0.0f32);
        let (mut norm_sum, mut norm_n) = (0.0f64, 0u64);
        let (mut mat_mis, mut both_hit) = (0u64, 0u64);
        for i in 0..frame.len() {
            let (a, b) = (&reference[i], &frame[i]);
            if a.hit == b.hit {
                agree += 1;
                if band[i] {
                    band_agree += 1;
                }
            }
            if a.hit && b.hit {
                both_hit += 1;
                let d = (a.t - b.t).abs();
                depth_sum += d as f64;
                depth_n += 1;
                depth_max = depth_max.max(d);
                let cos = a.normal.dot(b.normal).clamp(-1.0, 1.0);
                norm_sum += cos.acos().to_degrees() as f64;
                norm_n += 1;
                if a.mat != b.mat {
                    mat_mis += 1;
                }
            }
        }
        println!(
            "{:<22} {:>10} {:>7.2}x {:>9.4}% {:>9.4}% {:>7.5} m {:>8.3}° {:>7.4}%",
            cfg.name,
            evals,
            evals as f64 / ref_evals as f64,
            agree as f64 / frame.len() as f64 * 100.0,
            band_agree as f64 / band_n as f64 * 100.0,
            depth_sum / depth_n.max(1) as f64,
            norm_sum / norm_n.max(1) as f64,
            mat_mis as f64 / both_hit.max(1) as f64 * 100.0
        );
        let _ = depth_max;
    }
}

fn main() {
    let orbit = orbit_camera(Vec3::new(0.2, 0.85, 0.2), 7.5, 0.5, 0.3, 50f32.to_radians());
    validate("solid scene (down-look)", &solid_scene(), &orbit);
    validate("thin shells", &shell_scene(), &orbit);
    let (sky, sky_cam) = sky_scene();
    validate("sky framing (miss-heavy)", &sky, &sky_cam);
}
