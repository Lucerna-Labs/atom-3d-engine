//! Test the TPU-discovered step signals on the real MM3E scene/field evaluator.
//!
//! This does not change the shipped marcher. It runs candidate step rules through the
//! real orchestrator scene, counts field evaluations, and checks hit/depth agreement
//! against exact sphere tracing.
//!
//! Run: cargo run -p mm3e-orchestrator --example tpu_signals_real --release

use mm3e_kit::march::{Hit, Marcher, Ray};
use mm3e_kit::sdf::Field;
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_kit::{atoms, shade, Material};
use mm3e_orchestrator::{orbit_camera, Light, Object, Prim, Scene};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
enum Rule {
    Exact,
    ExactRef,
    GuardedOmega { omega: f32, secant: bool },
    RawRadiusIndex,
    RawOverlapT,
}

#[derive(Clone, Copy)]
struct Case {
    name: &'static str,
    rule: Rule,
}

struct Stats {
    total_evals: u64,
    primary_evals: u64,
    duration: Duration,
    hits: u32,
    depths: Vec<f32>,
}

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

fn cameras() -> Vec<mm3e_kit::camera::Camera> {
    let target = Vec3::new(0.2, 0.85, 0.4);
    [(8.5, 0.55, 0.32), (7.4, 1.35, 0.22), (8.0, 2.45, 0.38), (7.8, 3.40, 0.30), (9.1, 4.35, 0.44), (8.3, 5.20, 0.26)]
        .into_iter()
        .map(|(radius, yaw, pitch)| orbit_camera(target, radius, yaw, pitch, 50f32.to_radians()))
        .collect()
}

fn clipped_step(step: f32, fallback: f32) -> f32 {
    if step.is_finite() {
        step.clamp(0.0, 20.0)
    } else {
        fallback.clamp(0.0, 20.0)
    }
}

fn march_rule<F, N>(field: &F, m: &Marcher, rule: Rule, normal_fn: N, ray: &Ray) -> Hit
where
    F: Fn(Vec3) -> Field + ?Sized,
    N: Fn(Vec3) -> Vec3,
{
    let (mut omega, use_secant) = match rule {
        Rule::GuardedOmega { omega, secant } => (omega, secant),
        _ => (1.0, false),
    };
    let mut t = 0.0f32;
    let mut prev_radius = 0.0f32;
    let mut last_step = 0.0f32;
    let mut t_prev = 0.0f32;
    let mut d_prev = f32::INFINITY;

    let max_steps = match rule {
        Rule::ExactRef => m.max_steps * 2,
        _ => m.max_steps,
    };

    for i in 0..max_steps {
        let p = ray.at(t);
        let f = field(p);
        let radius = f.dist.abs();
        let eps = m.eps * (1.0 + t * 0.5);
        let overlap = radius + prev_radius - last_step;
        let mut step;

        if matches!(rule, Rule::GuardedOmega { .. }) && omega > 1.0 && radius + prev_radius < last_step {
            step = last_step - omega * last_step;
            omega = 1.0;
        } else {
            if f.dist < eps {
                return Hit { hit: true, t, pos: p, normal: normal_fn(p), mat: f.mat, steps: i };
            }

            step = match rule {
                Rule::Exact | Rule::ExactRef => f.dist,
                Rule::GuardedOmega { .. } => f.dist * omega,
                Rule::RawRadiusIndex => clipped_step((radius + i as f32 * 0.02) / 0.588, f.dist),
                Rule::RawOverlapT => clipped_step(overlap + t * 0.1, f.dist),
            };

            if use_secant && f.dist < 0.08 && d_prev.is_finite() {
                let dt = t - t_prev;
                let dd = f.dist - d_prev;
                if dt > 1e-6 && dd < -1e-6 {
                    step = (-f.dist * dt / dd).clamp(f.dist, f.dist * 4.0);
                }
            }
        }

        prev_radius = radius;
        d_prev = f.dist;
        t_prev = t;
        last_step = step;
        t += step;
        if t > m.max_dist {
            break;
        }
    }

    Hit { hit: false, t, pos: ray.at(t), normal: Vec3::ZERO, mat: 0, steps: max_steps }
}

fn pass(scene: &Scene, w: u32, h: u32, cameras: &[mm3e_kit::camera::Camera], rule: Rule) -> Stats {
    let counter = AtomicU64::new(0);
    let base = scene.field();
    let counted_field = |p: Vec3| {
        counter.fetch_add(1, Relaxed);
        base(p)
    };
    let m = scene.marcher;
    let mut hits = 0u32;
    let mut primary_evals = 0u64;
    let pixels_per_camera = (w * h) as usize;
    let mut depths = vec![f32::INFINITY; pixels_per_camera * cameras.len()];
    let t0 = Instant::now();

    for (ci, camera) in cameras.iter().enumerate() {
        let depth_base = ci * pixels_per_camera;
        for (x, y) in atoms::scan(w, h) {
            let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
            let before = counter.load(Relaxed);
            let hit = march_rule(&counted_field, &m, rule, |p| scene.normal_dual(p), &ray);
            primary_evals += counter.load(Relaxed) - before;
            if !hit.hit {
                continue;
            }

            hits += 1;
            depths[depth_base + (y * w + x) as usize] = hit.t;
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
                let _ =
                    m.soft_shadow(&counted_field, hit.pos + hit.normal.scale(0.01), ldir, ldist.min(m.max_dist), 24.0);
            }
            let _ = m.ambient_occlusion(&counted_field, hit.pos, hit.normal);
        }
    }

    Stats { total_evals: counter.load(Relaxed), primary_evals, duration: t0.elapsed(), hits, depths }
}

fn correctness(depths: &[f32], truth: &[f32]) -> (f32, f32) {
    let (mut agree, mut n_depth, mut sum_rel) = (0u64, 0u64, 0.0f64);
    for (d, t) in depths.iter().zip(truth) {
        let dh = d.is_finite();
        let th = t.is_finite();
        if dh == th {
            agree += 1;
        }
        if dh && th {
            sum_rel += ((d - t).abs() / (t + 1e-3)) as f64;
            n_depth += 1;
        }
    }
    (agree as f32 / depths.len() as f32, (sum_rel / n_depth.max(1) as f64) as f32)
}

fn main() {
    let (w, h) = (480u32, 270u32);
    let scene = scene_at(w, h);
    let cameras = cameras();

    println!("=== TPU step-signal validation on the REAL engine @ {w}x{h} AA1 ===");
    println!(
        "truth = 2x-step exact sphere tracing; candidates use the real scene field, shadows, AO, and {} camera views\n",
        cameras.len()
    );

    let truth = pass(&scene, w, h, &cameras, Rule::ExactRef);
    let shipped = pass(&scene, w, h, &cameras, Rule::GuardedOmega { omega: 1.4, secant: true });
    let cases = [
        Case { name: "exact d (same budget)", rule: Rule::Exact },
        Case { name: "shipped omega=1.40", rule: Rule::GuardedOmega { omega: 1.4, secant: true } },
        Case { name: "TPU omega=1.70", rule: Rule::GuardedOmega { omega: 1.70, secant: true } },
        Case { name: "TPU raw (radius+i*.02)/.588", rule: Rule::RawRadiusIndex },
        Case { name: "TPU raw overlap+t*.1", rule: Rule::RawOverlapT },
    ];

    println!("baseline shipped total field-evals: {}", shipped.total_evals);
    println!("baseline shipped primary field-evals: {}\n", shipped.primary_evals);
    println!(
        "  {:<29} | {:>11} | {:>7} | {:>11} | {:>7} | {:>8} | {:>9} | {:>9} | {:>6}",
        "case", "total", "dtotal", "primary", "dprim", "ms", "hit-agree", "depth-err", "hits"
    );
    println!("  {}", "-".repeat(116));

    for case in cases {
        let stats = pass(&scene, w, h, &cameras, case.rule);

        let (agree, depth_err) = correctness(&stats.depths, &truth.depths);
        let dtotal = (stats.total_evals as f64 - shipped.total_evals as f64) / shipped.total_evals as f64 * 100.0;
        let dprim = (stats.primary_evals as f64 - shipped.primary_evals as f64) / shipped.primary_evals as f64 * 100.0;
        println!(
            "  {:<29} | {:>11} | {:>6.1}% | {:>11} | {:>6.1}% | {:>7.2} | {:>8.2}% | {:>9.5} | {:>6}",
            case.name,
            stats.total_evals,
            dtotal,
            stats.primary_evals,
            dprim,
            stats.duration.as_secs_f64() * 1000.0,
            agree * 100.0,
            depth_err,
            stats.hits
        );
    }

    println!("\nReading: a signal only survives if it cuts primary evals while keeping hit-agreement near 100% and depth error tiny.");
}
