//! Real-engine validation of the discovery-search "dual-number autodiff normal" lead.
//!
//! Three independent searches (the 12h Rust cross-domain catalog sim, and GPU population searches
//! on both an RTX and an Intel Arc) converged on the same claim: a forward-mode dual-number SDF
//! yields the exact surface normal in ONE field evaluation, instead of the four extra samples the
//! shipped tetrahedron trick (`Marcher::normal`) needs. That claim is an eval-COUNT claim — the same
//! shape of claim the LOD sweep made, which oversold itself until tested in real wall-clock. This
//! measures both: real field-eval count AND real `Instant`-timed wall-clock, on the REAL engine's
//! own validation scene (the exact `Scene` from `lod_validate.rs`, not a hand-copied stand-in — the
//! primitive/transform list is read directly off `scene.objects`, so there is no drift risk).
//!
//! Mechanism: a dual number `D { v, g }` carries a value plus its 3-D gradient, propagated by the
//! chain rule through `+ - * sqrt abs min max`. Seeding the world point's x/y/z as the three unit
//! basis directions and running it through the SAME primitive formulas as `mm3e_kit::sdf` (sphere,
//! plane, rounded-box, torus) plus a constant-linear `to_local` transform yields `(distance, exact
//! gradient)` in one pass — this is forward-mode automatic differentiation, not an approximation.
//! `min`/`max` pick a subgradient at the active branch, matching the CSG union's own branch choice.
//!
//! Does NOT touch shipped code: `march.rs`/`sdf.rs`/`lib.rs` are unmodified. The march itself (the
//! dominant cost) is the REAL, unmodified `Marcher::march`; only the post-hit normal computation is
//! swapped for the timing comparison — reusing the SAME shipped `soft_shadow`/`ambient_occlusion`.
//!
//! Run: cargo run -p mm3e-orchestrator --example dual_normal_real --release
//! Verify the dual arithmetic before trusting the numbers: cargo test -p mm3e-orchestrator --example dual_normal_real

use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_kit::Material;
use mm3e_kit::{atoms, shade};
use mm3e_orchestrator::{orbit_camera, Combine, Light, Object, Prim, Scene};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;

// ============================================================================
// Dual number: value + gradient, propagated by the forward-mode chain rule.
// ============================================================================

#[derive(Clone, Copy, Debug)]
struct D {
    v: f32,
    g: Vec3,
}

impl D {
    fn con(v: f32) -> D {
        D { v, g: Vec3::ZERO }
    }
    fn add(self, o: D) -> D {
        D { v: self.v + o.v, g: self.g + o.g }
    }
    fn adds(self, s: f32) -> D {
        D { v: self.v + s, g: self.g }
    }
    fn neg(self) -> D {
        D { v: -self.v, g: -self.g }
    }
    fn scale(self, s: f32) -> D {
        D { v: self.v * s, g: self.g.scale(s) }
    }
    fn mul(self, o: D) -> D {
        D { v: self.v * o.v, g: self.g.scale(o.v) + o.g.scale(self.v) }
    }
    fn abs(self) -> D {
        if self.v >= 0.0 {
            self
        } else {
            self.neg()
        }
    }
    fn max0(self) -> D {
        if self.v >= 0.0 {
            self
        } else {
            D::con(0.0)
        }
    }
    fn sqrt(self) -> D {
        let v = self.v.max(0.0).sqrt();
        let k = if v > 1e-9 { 0.5 / v } else { 0.0 };
        D { v, g: self.g.scale(k) }
    }
    /// Subgradient pick at the active branch — matches `f32::min`'s tie behavior (`<=` keeps self).
    fn dmin(self, o: D) -> D {
        if self.v <= o.v {
            self
        } else {
            o
        }
    }
    fn dmax(self, o: D) -> D {
        if self.v >= o.v {
            self
        } else {
            o
        }
    }
}

/// World point (x,y,z) seeded as three dual variables, so any arithmetic on them carries the exact
/// gradient of the resulting scalar w.r.t. world position.
fn seed(p: Vec3) -> (D, D, D) {
    (
        D { v: p.x, g: Vec3::new(1.0, 0.0, 0.0) },
        D { v: p.y, g: Vec3::new(0.0, 1.0, 0.0) },
        D { v: p.z, g: Vec3::new(0.0, 0.0, 1.0) },
    )
}

/// `Transform::to_local` (rotation-transpose then translate) carried through dual arithmetic. Scale
/// is 1.0 for every object in the validation scene (asserted by the caller), so the inverse-scale
/// term from the shipped `to_local` is omitted here — not needed for this scene, not general-purpose.
fn to_local_dual(xf: &Transform, x: D, y: D, z: D) -> (D, D, D) {
    let tx = x.adds(-xf.pos.x);
    let ty = y.adds(-xf.pos.y);
    let tz = z.adds(-xf.pos.z);
    let rt = xf.rot.transpose();
    let lx = tx.scale(rt.cols[0].x).add(ty.scale(rt.cols[1].x)).add(tz.scale(rt.cols[2].x));
    let ly = tx.scale(rt.cols[0].y).add(ty.scale(rt.cols[1].y)).add(tz.scale(rt.cols[2].y));
    let lz = tx.scale(rt.cols[0].z).add(ty.scale(rt.cols[1].z)).add(tz.scale(rt.cols[2].z));
    (lx, ly, lz)
}

// ----------------------------------------------------------------------------
// Dual-number replicas of the primitive SDFs actually used by the validation scene, ported term
// for term from `mm3e_kit::sdf`. Only the four `Prim` variants `scene_at` uses are implemented —
// this is a validation harness for that scene, not a general dual-number SDF kit.
// ----------------------------------------------------------------------------

fn d_plane(lx: D, ly: D, lz: D, n: Vec3, h: f32) -> D {
    lx.scale(n.x).add(ly.scale(n.y)).add(lz.scale(n.z)).adds(h)
}
fn d_sphere(lx: D, ly: D, lz: D, r: f32) -> D {
    lx.mul(lx).add(ly.mul(ly)).add(lz.mul(lz)).sqrt().adds(-r)
}
fn d_torus(lx: D, ly: D, lz: D, major: f32, minor: f32) -> D {
    let q = lx.mul(lx).add(lz.mul(lz)).sqrt().adds(-major);
    q.mul(q).add(ly.mul(ly)).sqrt().adds(-minor)
}
fn d_boxed(lx: D, ly: D, lz: D, half: Vec3) -> D {
    let qx = lx.abs().adds(-half.x);
    let qy = ly.abs().adds(-half.y);
    let qz = lz.abs().adds(-half.z);
    let outside = qx.max0().mul(qx.max0()).add(qy.max0().mul(qy.max0())).add(qz.max0().mul(qz.max0())).sqrt();
    let inside = qx.dmax(qy.dmax(qz)).dmin(D::con(0.0));
    outside.add(inside)
}
fn d_rounded_box(lx: D, ly: D, lz: D, half: Vec3, radius: f32) -> D {
    d_boxed(lx, ly, lz, half - Vec3::splat(radius)).adds(-radius)
}

/// One dual-number field evaluation of the whole scene (union of every object): distance + exact
/// unit normal, in a single pass — the mechanism the discovery search flagged as "1 eval vs 4".
fn dual_world(objs: &[(Prim, Transform)], p: Vec3) -> (f32, Vec3) {
    let (x, y, z) = seed(p);
    let mut best: Option<D> = None;
    for (prim, xf) in objs {
        let (lx, ly, lz) = to_local_dual(xf, x, y, z);
        let d = match *prim {
            Prim::Plane { n, h } => d_plane(lx, ly, lz, n, h),
            Prim::Sphere { r } => d_sphere(lx, ly, lz, r),
            Prim::RoundBox { half, radius } => d_rounded_box(lx, ly, lz, half, radius),
            Prim::Torus { major, minor } => d_torus(lx, ly, lz, major, minor),
            other => unreachable!("validation scene uses only Plane/Sphere/RoundBox/Torus, got {other:?}"),
        };
        best = Some(match best {
            None => d,
            Some(b) => b.dmin(d),
        });
    }
    let b = best.expect("scene has at least one object");
    (b.v, b.g.normalize())
}

/// Objects this harness can differentiate through, read directly off the real `Scene` (no
/// hand-copied scene — eliminates drift between the replica and what actually ships). Panics if the
/// scene ever adds a modifier/smoothing this harness doesn't model, so a silent-wrong-answer failure
/// mode is impossible.
fn extract_dual_objects(scene: &Scene) -> Vec<(Prim, Transform)> {
    scene
        .objects
        .iter()
        .map(|o| {
            assert!(matches!(o.combine, Combine::Union), "dual harness assumes plain Union combine");
            let m = &o.mods;
            let is_default = !(m.mirror[0] || m.mirror[1] || m.mirror[2])
                && m.repeat == Vec3::ZERO
                && m.twist == 0.0
                && m.bend == 0.0
                && m.elongate == Vec3::ZERO
                && m.round == 0.0
                && m.onion == 0.0;
            assert!(is_default, "dual harness assumes no domain modifiers on scene objects");
            assert!((o.xform.scale - 1.0).abs() < 1e-6, "dual harness assumes unit scale");
            (o.prim, o.xform)
        })
        .collect()
}

// ============================================================================
// The validation scene — identical to `lod_validate.rs`'s `scene_at`, so this test sits on the
// same real-engine baseline the LOD sweep already validated.
// ============================================================================

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

// ============================================================================
// Real-engine A/B: same march, same shadow/AO, only the normal-computation method differs.
// ============================================================================

#[derive(Clone, Copy, PartialEq)]
enum NormalMode {
    Tetra,
    Dual,
}

/// One full per-pixel pass — march (shipped, unmodified) then shadow+AO (shipped) for every hit,
/// mirroring `lod_validate.rs`'s `field_evals` harness exactly, except the normal is computed by
/// `mode`. Returns (field-eval count, wall-clock for the WHOLE loop, normal-only wall-clock, hits).
fn run_pass(
    scene: &Scene,
    dual_objs: &[(Prim, Transform)],
    w: u32,
    h: u32,
    mode: NormalMode,
) -> (u64, std::time::Duration, std::time::Duration, u32) {
    let camera = cam();
    let counter = AtomicU64::new(0);
    let base = scene.field();
    let cf = |p: Vec3| {
        counter.fetch_add(1, Relaxed);
        base(p)
    };
    let m = scene.marcher;
    let mut normal_time = std::time::Duration::ZERO;
    let mut hits = 0u32;

    let whole_start = Instant::now();
    for (x, y) in atoms::scan(w, h) {
        let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
        let hit = m.march(&cf, &ray);
        if !hit.hit {
            continue;
        }
        hits += 1;
        let nstart = Instant::now();
        let normal = match mode {
            NormalMode::Tetra => m.normal(&cf, hit.pos),
            NormalMode::Dual => {
                counter.fetch_add(1, Relaxed); // one field-eval, by the search's accounting convention
                dual_world(dual_objs, hit.pos).1
            }
        };
        normal_time += nstart.elapsed();

        for light in &scene.lights {
            let (ldir, ldist) = if light.directional {
                (light.vec, f32::INFINITY)
            } else {
                let d = light.vec - hit.pos;
                (d.normalize(), d.length())
            };
            if shade::lambert(normal, ldir) <= 0.0 {
                continue;
            }
            let _ = m.soft_shadow(&cf, hit.pos + normal.scale(0.01), ldir, ldist.min(m.max_dist), 24.0);
        }
        let _ = m.ambient_occlusion(&cf, hit.pos, normal);
    }
    let whole = whole_start.elapsed();
    (counter.load(Relaxed), whole, normal_time, hits)
}

fn main() {
    let (w, h) = (480u32, 270u32);
    let scene = scene_at(w, h);
    let dual_objs = extract_dual_objects(&scene);

    println!("=== dual-number autodiff normal validation on the REAL engine @ {w}x{h} AA1 ===");
    println!("(three independent searches picked this lead; this is its first real-engine test)\n");

    // Correctness first: does the dual gradient reproduce the shipped tetrahedron normal at real
    // hit points, on the real scene? If this drifts, the timing numbers below aren't trustworthy.
    let camera = cam();
    let field = scene.field();
    let mut max_dev = 0.0f32;
    let mut sum_dev = 0.0f64;
    let mut n_checked = 0u32;
    for (x, y) in atoms::scan(w, h) {
        if (x + y) % 37 != 0 {
            continue; // a representative sample, not every pixel — this is a correctness spot-check
        }
        let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
        let hit = scene.marcher.march(&field, &ray);
        if !hit.hit {
            continue;
        }
        let tetra_n = scene.marcher.normal(&field, hit.pos);
        let dual_n = dual_world(&dual_objs, hit.pos).1;
        let cos = tetra_n.dot(dual_n).clamp(-1.0, 1.0);
        let deg = cos.acos().to_degrees();
        max_dev = max_dev.max(deg);
        sum_dev += deg as f64;
        n_checked += 1;
    }
    let mean_dev = (sum_dev / n_checked.max(1) as f64) as f32;
    println!("correctness (dual normal vs shipped tetrahedron, {n_checked} sampled hit points):");
    println!("  mean angular deviation: {mean_dev:.4} deg | max: {max_dev:.4} deg\n");

    // Real wall-clock + field-eval A/B, full per-pixel pass (march + normal + shadow + AO), each
    // method run 3 times and averaged (the timing signal this whole exercise exists to check).
    const REPEATS: u32 = 3;
    let mut tetra = (0u64, std::time::Duration::ZERO, std::time::Duration::ZERO, 0u32);
    let mut dual = (0u64, std::time::Duration::ZERO, std::time::Duration::ZERO, 0u32);
    for _ in 0..REPEATS {
        let t = run_pass(&scene, &dual_objs, w, h, NormalMode::Tetra);
        tetra.0 = t.0;
        tetra.1 += t.1;
        tetra.2 += t.2;
        tetra.3 = t.3;
        let d = run_pass(&scene, &dual_objs, w, h, NormalMode::Dual);
        dual.0 = d.0;
        dual.1 += d.1;
        dual.2 += d.2;
        dual.3 = d.3;
    }
    let tetra_whole_ms = tetra.1.as_secs_f64() * 1000.0 / REPEATS as f64;
    let dual_whole_ms = dual.1.as_secs_f64() * 1000.0 / REPEATS as f64;
    let tetra_norm_us = tetra.2.as_secs_f64() * 1e6 / REPEATS as f64;
    let dual_norm_us = dual.2.as_secs_f64() * 1e6 / REPEATS as f64;

    println!("field-evals (march + shadow + AO + normal-method), {} hits:", tetra.3);
    println!("  tetrahedron (4 evals/hit): {}", tetra.0);
    println!(
        "  dual-number (1 eval/hit):  {}  ({:+.1}% vs tetrahedron)",
        dual.0,
        (dual.0 as f64 - tetra.0 as f64) / tetra.0 as f64 * 100.0
    );
    println!();
    println!("REAL WALL-CLOCK (avg of {REPEATS} runs, release build):");
    println!("  whole per-pixel pass (march+normal+shadow+AO):");
    println!("    tetrahedron: {tetra_whole_ms:.2} ms");
    println!(
        "    dual-number: {dual_whole_ms:.2} ms  ({:+.1}%)",
        (dual_whole_ms - tetra_whole_ms) / tetra_whole_ms * 100.0
    );
    println!("  normal-computation only (isolated, summed over all {} hits):", tetra.3);
    println!("    tetrahedron: {tetra_norm_us:.1} us total ({:.3} us/hit)", tetra_norm_us / tetra.3.max(1) as f64);
    println!(
        "    dual-number: {dual_norm_us:.1} us total ({:.3} us/hit)  ({:+.1}%)",
        dual_norm_us / dual.3.max(1) as f64,
        (dual_norm_us - tetra_norm_us) / tetra_norm_us * 100.0
    );
    println!();
    println!("Reading: eval-count savings and wall-clock savings can DIVERGE (the LOD-sweep lesson) —");
    println!("dual arithmetic does more FLOPs per call than one scalar tetrahedron sample, so a real");
    println!("win requires the wall-clock line to also show a negative percentage, not just the count.");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Adversarial check #1: the analytic dual gradient must match a fine central-difference
    /// gradient of the SAME dual-number scene, at points scattered near (not exactly on) each
    /// object's own surface, away from union seams where "the normal" isn't uniquely defined.
    #[test]
    fn dual_gradient_matches_finite_difference() {
        let scene = scene_at(64, 64);
        let objs = extract_dual_objects(&scene);
        let probe_points = [
            Vec3::new(0.0, 0.001, 0.0),  // near the plane
            Vec3::new(-2.4, 2.0, 0.2),   // top of the mirror sphere
            Vec3::new(-1.4, 1.0, 0.2),   // side of the mirror sphere
            Vec3::new(1.05, 0.95, -0.6), // near a round-box face
            Vec3::new(0.2, 1.8, -0.6),   // round-box, another face
            Vec3::new(3.45, 1.05, 0.4),  // outer torus rim
            Vec3::new(2.6, 1.35, 0.4),   // torus top
            Vec3::new(0.9, 1.4, 1.9),    // small sphere top
        ];
        let h = 1e-4;
        let mut max_rel = 0.0f32;
        for p in probe_points {
            let (_, analytic_n) = dual_world(&objs, p);
            let dx = (dual_world(&objs, p + Vec3::new(h, 0.0, 0.0)).0
                - dual_world(&objs, p - Vec3::new(h, 0.0, 0.0)).0)
                / (2.0 * h);
            let dy = (dual_world(&objs, p + Vec3::new(0.0, h, 0.0)).0
                - dual_world(&objs, p - Vec3::new(0.0, h, 0.0)).0)
                / (2.0 * h);
            let dz = (dual_world(&objs, p + Vec3::new(0.0, 0.0, h)).0
                - dual_world(&objs, p - Vec3::new(0.0, 0.0, h)).0)
                / (2.0 * h);
            let fd_n = Vec3::new(dx, dy, dz).normalize();
            let cos = analytic_n.dot(fd_n).clamp(-1.0, 1.0);
            let deg = cos.acos().to_degrees();
            max_rel = max_rel.max(deg);
            assert!(deg < 0.5, "dual gradient vs finite-difference mismatch at {p:?}: {deg:.4} deg");
        }
        println!("max deviation from finite-difference across probes: {max_rel:.5} deg");
    }

    /// Adversarial check #2: the dual replica must reproduce the REAL engine's own tetrahedron
    /// normal at real march hit points — proving the replica is faithful to what actually ships,
    /// not just internally self-consistent.
    #[test]
    fn dual_matches_real_engine_normal_at_hit_points() {
        let (w, h) = (96u32, 54u32);
        let scene = scene_at(w, h);
        let objs = extract_dual_objects(&scene);
        let camera = cam();
        let field = scene.field();
        let mut max_dev = 0.0f32;
        let mut checked = 0u32;
        for (x, y) in atoms::scan(w, h) {
            let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
            let hit = scene.marcher.march(&field, &ray);
            if !hit.hit {
                continue;
            }
            let tetra_n = scene.marcher.normal(&field, hit.pos);
            let dual_n = dual_world(&objs, hit.pos).1;
            let cos = tetra_n.dot(dual_n).clamp(-1.0, 1.0);
            let deg = cos.acos().to_degrees();
            max_dev = max_dev.max(deg);
            checked += 1;
        }
        assert!(checked > 100, "expected many real hit points, got {checked}");
        assert!(max_dev < 3.0, "dual normal deviates from the real tetrahedron normal by {max_dev:.3} deg");
        println!("checked {checked} real hit points, max deviation {max_dev:.4} deg");
    }

    #[test]
    fn extract_dual_objects_matches_scene_primitive_count() {
        let scene = scene_at(16, 16);
        let objs = extract_dual_objects(&scene);
        assert_eq!(objs.len(), scene.objects.len());
        assert_eq!(objs.len(), 5, "validation scene is expected to have exactly 5 objects");
    }
}
