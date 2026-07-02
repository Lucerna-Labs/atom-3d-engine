//! Real-engine validation of the engine-mix search's lead: does the Arc run's -40% config — which
//! turned OFF fixed over-relaxation and replaced it with STOCHASTIC-omega (a quantum-walk mechanism)
//! and SUBITIZE (the Approximate Number System's "leap when clearly far") — actually beat the shipped
//! marcher on the REAL engine, in real field-evals and real wall-clock?
//!
//! Same discipline as `dual_normal_real.rs`: measured on the engine's own validation scene (the
//! `lod_validate.rs` 5-object scene), with the shipped soft-shadow + AO in the field-eval count, and
//! `Instant`-timed wall-clock. The marcher variant reduces EXACTLY to the shipped over-relaxation
//! march when `stoch=subk=mom=0, omega=1.4` — so baseline and candidate run the identical code path,
//! only the config differs, and a real-engine ABLATION (zero each new operator) shows which one, if
//! any, survives at real resolution (480x270) — where the sim's low-res headline usually deflates
//! (the LOD/TAA lesson). Uses the now-shipped dual normal for BOTH sides (equal footing).
//!
//! Does NOT modify shipped code: the variant march loop is a local reimplementation for measurement.
//! Run:  cargo run -p mm3e-orchestrator --example stoch_subitize_real --release

use mm3e_kit::march::{Hit, Marcher, Ray};
use mm3e_kit::sdf::Field;
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_kit::Material;
use mm3e_kit::{atoms, shade};
use mm3e_orchestrator::{orbit_camera, Light, Object, Prim, Scene};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;

/// The marcher config knobs the engine-mix search tunes (superset of the shipped marcher's).
#[derive(Clone, Copy)]
struct Cfg {
    omega: f32,      // base over-relaxation (1.0 = fixed over-relax OFF)
    stoch: f32,      // stochastic-omega jitter amplitude (quantum-walk)
    subk: f32,       // subitize far-leap factor (ANS)
    mom: f32,        // momentum / predictive-coding step
    lod: f32,        // screen-footprint LOD tolerance slope
    secant_thr: f32, // near-surface secant threshold (0 = off)
}

impl Cfg {
    /// Exactly the shipped over-relaxation march (omega=1.4, no new ops, no LOD).
    fn shipped() -> Cfg {
        Cfg { omega: 1.4, stoch: 0.0, subk: 0.0, mom: 0.0, lod: 0.0, secant_thr: 0.0 }
    }
    /// The Arc engine-mix run's winning config (-40% in the sim).
    fn sim_winner() -> Cfg {
        Cfg { omega: 1.0, stoch: 0.495, subk: 0.792, mom: 0.041, lod: 0.0007, secant_thr: 0.15 }
    }
    /// A pure exact sphere-trace (no over-relax, no ops) for the ground-truth reference.
    fn exact() -> Cfg {
        Cfg { omega: 1.0, stoch: 0.0, subk: 0.0, mom: 0.0, lod: 0.0, secant_thr: 0.0 }
    }
}

/// The marcher variant. With `stoch=subk=mom=lod=secant=0, omega=1.4` it is bit-for-bit the
/// shipped `Marcher::march` step logic — verified by `variant_reduces_to_shipped` below. `normal_fn`
/// supplies the hit normal (here always the shipped dual normal, for equal footing on both sides).
fn march_variant<F, N>(field: &F, m: &Marcher, cfg: &Cfg, normal_fn: N, ray: &Ray) -> Hit
where
    F: Fn(Vec3) -> Field + ?Sized,
    N: Fn(Vec3) -> Vec3,
{
    #[derive(Clone, Copy, PartialEq)]
    enum Boost {
        Omega,
        Stoch,
        Momentum,
        Leap,
        Secant,
    }

    let mut omega = cfg.omega;
    let mut stoch = cfg.stoch;
    let mut subk = cfg.subk;
    let mut mom = cfg.mom;
    let mut secant_thr = cfg.secant_thr;
    let mut boost = Boost::Omega;
    let mut t = 0.0f32;
    let mut prev_radius = 0.0f32;
    let mut step_len = 0.0f32;
    let mut t_prev = 0.0f32;
    let mut d_prev = f32::INFINITY;
    for i in 0..m.max_steps {
        let p = ray.at(t);
        let f = field(p);
        let radius = f.dist.abs();
        let eps = m.eps * (1.0 + t * 0.5) + cfg.lod * t;
        // Overlap-guard fallback uses the omega STATE (not the jittered value) — matching the sim and
        // the shipped marcher. With omega=1.0 (fixed over-relax off) this guard is inactive.
        if step_len > radius + prev_radius {
            let gap_lo = t_prev + prev_radius;
            let gap_hi = t - radius;
            let mid = 0.5 * (gap_lo + gap_hi);
            let covered = f.dist > 0.0 && field(ray.at(mid)).dist >= 0.5 * (gap_hi - gap_lo);
            if !covered {
                t = gap_lo;
                match boost {
                    Boost::Omega => omega = omega.min(1.0),
                    Boost::Stoch => stoch = 0.0,
                    Boost::Momentum => mom = 0.0,
                    Boost::Leap => subk = 0.0,
                    Boost::Secant => secant_thr = 0.0,
                }
                boost = Boost::Omega;
                step_len = 0.0;
                prev_radius = 0.0;
                d_prev = f32::INFINITY;
                continue;
            }
        }
        if f.dist < eps {
            return Hit { hit: true, t, pos: p, normal: normal_fn(p), mat: f.mat, steps: i };
        }
        // stochastic over-relaxation (quantum-walk): deterministic per-(t, step) jitter of omega.
        let jitter = ((t * 12.9898 + i as f32 * 78.233).sin() * 43758.547).fract().abs();
        let omega_eff = (omega + stoch * (jitter - 0.5) * 2.0).clamp(1.0, 2.0);
        step_len = f.dist * omega_eff;
        boost = if omega_eff > omega { Boost::Stoch } else { Boost::Omega };
        // momentum / predictive-coding: add a fraction of the approach rate.
        if mom > 0.0 && d_prev.is_finite() {
            let extra = mom * (d_prev - f.dist).max(0.0);
            if extra > 0.0 {
                step_len += extra;
                boost = Boost::Momentum;
            }
        }
        // subitize (ANS): extra leap when the distance is clearly far.
        if subk > 0.0 && f.dist > eps * 6.0 {
            step_len *= 1.0 + subk;
            boost = Boost::Leap;
        }
        // secant near-surface refinement (overrides the above when near, matching the sim order).
        if secant_thr > 0.0 && f.dist < secant_thr && d_prev.is_finite() {
            let dt = t - t_prev;
            let dd = f.dist - d_prev;
            if dt > 1e-6 && dd < -1e-6 {
                step_len = (-f.dist * dt / dd).clamp(f.dist, f.dist * 4.0);
                boost = Boost::Secant;
            }
        }
        prev_radius = radius;
        d_prev = f.dist;
        t_prev = t;
        t += step_len;
        if t > m.max_dist {
            break;
        }
    }
    Hit { hit: false, t, pos: ray.at(t), normal: Vec3::ZERO, mat: 0, steps: m.max_steps }
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

fn cam() -> mm3e_kit::camera::Camera {
    orbit_camera(Vec3::new(0.2, 0.85, 0.4), 8.5, 0.55, 0.32, 50f32.to_radians())
}

/// Full per-pixel pass with the given config: primary march (variant), then shipped soft-shadow + AO
/// per hit — mirroring `lod_validate.rs`'s `field_evals`. Returns (field-evals, wall-clock, hits, and
/// per-pixel hit-t for correctness). Normal is the shipped dual normal on both sides.
fn pass(scene: &Scene, w: u32, h: u32, cfg: &Cfg) -> (u64, std::time::Duration, u32, Vec<f32>) {
    let camera = cam();
    let counter = AtomicU64::new(0);
    let base = scene.field();
    let cf = |p: Vec3| {
        counter.fetch_add(1, Relaxed);
        base(p)
    };
    let m = scene.marcher;
    let mut hits = 0u32;
    let mut depths = vec![f32::INFINITY; (w * h) as usize];
    let t0 = Instant::now();
    for (x, y) in atoms::scan(w, h) {
        let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
        let hit = march_variant(&cf, &m, cfg, |p| scene.normal_dual(p), &ray);
        if !hit.hit {
            continue;
        }
        hits += 1;
        depths[(y * w + x) as usize] = hit.t;
        for light in &scene.lights {
            let (ldir, ldist) =
                if light.directional { (light.vec, f32::INFINITY) } else { (light.vec - hit.pos, f32::INFINITY) };
            if shade::lambert(hit.normal, ldir) <= 0.0 {
                continue;
            }
            let _ = m.soft_shadow(&cf, hit.pos + hit.normal.scale(0.01), ldir, ldist.min(m.max_dist), 24.0);
        }
        let _ = m.ambient_occlusion(&cf, hit.pos, hit.normal);
    }
    (counter.load(Relaxed), t0.elapsed(), hits, depths)
}

/// Correctness of a config's primary hits vs the exact reference depths: fraction of pixels whose
/// hit/miss agrees, and mean relative depth error where both hit. A config that tunnels or misses
/// surfaces shows up as a low agreement / high depth error here.
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

    println!("=== stochastic-omega + subitize validation on the REAL engine @ {w}x{h} AA1 ===");
    println!("(the Arc engine-mix search's -40% lead; this is its first real-engine test)\n");

    // Ground-truth depths: pure exact sphere trace.
    let (_, _, _, truth) = pass(&scene, w, h, &Cfg::exact());

    const REPEATS: u32 = 3;
    let configs: [(&str, Cfg); 4] = [
        ("shipped (over-relax)", Cfg::shipped()),
        ("sim-winner (full)", Cfg::sim_winner()),
        ("  ablate stochastic-w", Cfg { stoch: 0.0, ..Cfg::sim_winner() }),
        ("  ablate subitize", Cfg { subk: 0.0, ..Cfg::sim_winner() }),
    ];

    // Baseline field-evals for the % column = the shipped config.
    let base_evals = pass(&scene, w, h, &Cfg::shipped()).0;

    println!("  {:<24} | field-evals |  Δevals | wall-clock |  Δtime | hit-agree | depth-err", "config");
    println!("  {}", "-".repeat(96));
    for (name, cfg) in configs {
        let mut evals = 0u64;
        let mut dur = std::time::Duration::ZERO;
        let mut hits = 0u32;
        let mut depths = Vec::new();
        for _ in 0..REPEATS {
            let (e, d, hh, dp) = pass(&scene, w, h, &cfg);
            evals = e;
            dur += d;
            hits = hh;
            depths = dp;
        }
        let ms = dur.as_secs_f64() * 1000.0 / REPEATS as f64;
        let (agree, derr) = correctness(&depths, &truth);
        let devals = (evals as f64 - base_evals as f64) / base_evals as f64 * 100.0;
        // wall-clock delta is vs the shipped config's time — filled after we have it (shipped is row 0).
        let base_ms = if name.starts_with("shipped") { ms } else { f64::NAN };
        static BASE_MS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        if name.starts_with("shipped") {
            BASE_MS.store(base_ms.to_bits(), Relaxed);
        }
        let bms = f64::from_bits(BASE_MS.load(Relaxed));
        let dtime = if name.starts_with("shipped") { 0.0 } else { (ms - bms) / bms * 100.0 };
        println!(
            "  {:<24} | {:>11} | {:>6.1}% | {:>7.2}ms | {:>5.1}% | {:>8.1}% | {:>8.4}   ({hits} hits)",
            name,
            evals,
            devals,
            ms,
            dtime,
            agree * 100.0,
            derr
        );
    }

    // Subitize is the driver, and (like LOD) it's a speed/quality dial — sweep it to show the
    // tradeoff curve at real resolution, where the sim's fixed 0.792 may be more aggressive than ideal.
    println!("\nSubitize tradeoff sweep (winner's other params fixed, vs exact-depth reference):");
    println!("  subitize |  Δevals | depth-err | hit-agree");
    for subk in [0.0f32, 0.2, 0.4, 0.6, 0.792] {
        let cfg = Cfg { subk, ..Cfg::sim_winner() };
        let (evals, _, _, depths) = pass(&scene, w, h, &cfg);
        let (agree, derr) = correctness(&depths, &truth);
        let dev = (evals as f64 - base_evals as f64) / base_evals as f64 * 100.0;
        println!("  {subk:>8.3} | {dev:>6.1}% | {derr:>9.4} | {:>8.1}%", agree * 100.0);
    }

    println!();
    println!("Reading: a REAL win needs (1) fewer field-evals AND lower wall-clock (Δtime negative),");
    println!("(2) hit-agreement ~100% and small depth-err (no tunneling), on THIS higher-res scene it was");
    println!("NOT tuned on. The ablation rows show whether stochastic-w or subitize is doing the work —");
    println!("or whether the sim's -40% was a low-res / tuning-scene artifact that doesn't survive.");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The variant marcher with all new ops off + omega=1.4 must reproduce the shipped
    /// `Marcher::march` hit exactly (same t, same hit/miss) — proving the baseline row is a true
    /// baseline and the only difference in the sim-winner row is the new operators.
    #[test]
    fn variant_reduces_to_shipped_when_ops_off() {
        let (w, h) = (128u32, 72u32);
        let scene = scene_at(w, h);
        let camera = cam();
        let field = scene.field();
        let m = scene.marcher;
        let cfg = Cfg::shipped();
        let mut checked = 0u32;
        let mut max_dt = 0.0f32;
        for (x, y) in atoms::scan(w, h) {
            let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
            let shipped = m.march(&field, &ray); // real Marcher::march (tetra normal, but t is what we compare)
            let variant = march_variant(&field, &m, &cfg, |_p| Vec3::ZERO, &ray);
            assert_eq!(shipped.hit, variant.hit, "hit/miss differs at ({x},{y})");
            if shipped.hit {
                max_dt = max_dt.max((shipped.t - variant.t).abs());
                checked += 1;
            }
        }
        assert!(checked > 500, "expected many hits, got {checked}");
        assert!(max_dt < 1e-4, "variant(off) diverged from shipped march by t={max_dt}");
    }
}
