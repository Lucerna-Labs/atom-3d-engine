//! The sphere tracer — the 3-D elevation of MMPE's 2-D `scan_convert`.
//!
//! Where the 2-D kit walked the pixel grid and asked an SDF "am I inside this shape?", the
//! 3-D kit walks *along a ray* and asks the world field "how far to the nearest surface?",
//! stepping by exactly that safe distance. That march is the `fold` atom: reduce a stream of
//! ray steps to a single hit. Surface normals come from the field *gradient* (four `compare`
//! samples, the tetrahedron trick); soft shadows and ambient occlusion are short secondary
//! marches. All mechanism — the field, lights, and step budget are the orchestrator's policy.

use crate::sdf::Field;
use crate::vec::Vec3;

/// A ray with a unit direction.
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.dir.scale(t)
    }
}

/// The result of tracing one ray into the world field.
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    /// True if a surface was reached before the far plane / step budget ran out.
    pub hit: bool,
    /// Distance along the ray to the hit (or the far distance on a miss).
    pub t: f32,
    pub pos: Vec3,
    pub normal: Vec3,
    pub mat: u32,
    pub steps: u32,
}

/// Budget + tolerances for a march. Defaults are tuned for the example scenes. The step/shadow/AO
/// budgets are the quality knobs an adaptive renderer turns down while moving and up when still.
#[derive(Clone, Copy, Debug)]
pub struct Marcher {
    pub max_steps: u32,
    pub max_dist: f32,
    pub eps: f32,
    /// Fraction of the safe distance to actually step. 1.0 for pure Lipschitz fields; lower
    /// (≈0.6) when the scene uses non-distance-preserving domain warps (twist/bend/repeat).
    pub step_scale: f32,
    /// Max iterations for the soft-shadow march (lower = faster, harder shadows).
    pub shadow_steps: u32,
    /// Ambient-occlusion sample count (0 disables AO).
    pub ao_samples: u32,
    /// Screen-footprint LOD (a graphics level-of-detail transfer): widen the hit tolerance by
    /// `lod_footprint · t` so distant / sub-pixel geometry resolves in fewer steps. 0.0 = off
    /// (exact). Mechanism only — the orchestrator dials it up while the camera moves (where the
    /// fidelity cost is imperceptible) and back to 0 for converged stills. Measured −7%…−16%
    /// field-evals for an image mean Δ of 1.3%…3.5% as it rises.
    pub lod_footprint: f32,
    /// Subitize empty-space leap (a **numerical-cognition** transfer — the Approximate Number
    /// System's "instantly recognize it's clearly far, so leap"): when the safe distance is well
    /// clear of the surface (`d > 6·eps`), multiply the step by `1 + subitize`. 0.0 = off. The
    /// unconditional overlap guard is the safety net: a leap that would tunnel is undone (retreat
    /// to the last safe frontier) and the knob decays off for that ray, so hits are never buried
    /// inside surfaces. Honest economics under the corrected guard (`examples/subitize_econ.rs`):
    /// it pays on miss-heavy / open framings and can cost on hit-dominated framings, where leaps
    /// near geometry trip the guard and pay a retreat. The orchestrator should dial it up for
    /// open/sky-heavy shots and off for close-ups.
    pub subitize: f32,
    /// Secant / regula-falsi near-surface refinement: predict the surface root from the last two
    /// samples and step toward it, capped at 4*d. Off by default: honest re-measurement under the
    /// corrected overlap guard shows it can cost more than it saves on this renderer's profiler
    /// scene. Preserved as a primitive because the overshoot-then-verify shape may still transfer
    /// to traversal domains where field evaluation is much more expensive than verification.
    pub secant: bool,
    /// Half-width of the tetrahedron normal stencil. The default keeps analytic scenes on the
    /// historical normal sample radius; sampled SDF volumes should widen this to roughly half a
    /// grid cell so cell-scale interpolation wobble does not become normal/AO/shadow noise.
    pub normal_h: f32,
}

impl Default for Marcher {
    fn default() -> Self {
        Self {
            max_steps: 160,
            max_dist: 120.0,
            eps: 0.0006,
            step_scale: 1.0,
            shadow_steps: 64,
            ao_samples: 5,
            lod_footprint: 0.0,
            subitize: 0.0,
            secant: false,
            normal_h: 0.0009,
        }
    }
}

impl Marcher {
    /// Sphere-trace one ray through `field`, folding ray steps down to a `Hit`.
    ///
    /// Uses **enhanced sphere tracing** (Keinert et al. 2014): step by `ω · distance` with
    /// `ω = 1.4`, and whenever two successive unbounding spheres fail to overlap (the signal that
    /// a boosted step — over-relaxation, subitize, or secant — jumped past a surface), retreat to
    /// the last provably safe frontier and continue conservatively. The guard is unconditional,
    /// so no boosted step can ever tunnel and record a hit buried inside a surface. This skips
    /// long empty stretches — exactly the horizon/grazing rays that dominate the cost — without
    /// moving the hit point. A `step_scale < 1` (set for non-Lipschitz domain warps) disables
    /// over-relaxation and just under-relaxes, as before.
    pub fn march<F: Fn(Vec3) -> Field + ?Sized>(&self, field: &F, ray: &Ray) -> Hit {
        self.march_with(field, |p| self.normal(field, p), ray)
    }

    /// Sphere-trace exactly as [`Marcher::march`], except the hit normal comes from `normal_fn`
    /// instead of the tetrahedron stencil. `march()` is `march_with(field, |p| self.normal(field,
    /// p), ray)` — same loop, same behavior, so every existing caller is unaffected. This hook
    /// exists for callers that have an exact analytic gradient available (e.g. a dual-number scene
    /// field, see `mm3e_kit::dual` and `Scene::field_dual`), which is both correct and cheaper than
    /// four extra tetrahedron samples per hit. A generic closure (not `&dyn Fn`) keeps this
    /// monomorphized and inlining, matching the rest of this file's hot-path style.
    pub fn march_with<F, N>(&self, field: &F, normal_fn: N, ray: &Ray) -> Hit
    where
        F: Fn(Vec3) -> Field + ?Sized,
        N: Fn(Vec3) -> Vec3,
    {
        #[derive(Clone, Copy, PartialEq)]
        enum Boost {
            Omega,
            Leap,
            Secant,
        }

        let mut omega = if self.step_scale >= 1.0 { 1.4 } else { self.step_scale };
        let mut subitize = if self.step_scale >= 1.0 { self.subitize } else { 0.0 };
        let mut secant_on = self.secant && self.step_scale >= 1.0;
        let mut boost = Boost::Omega;
        let mut t = 0.0f32;
        let mut prev_radius = 0.0f32;
        let mut step_len = 0.0f32;
        // Secant state for near-surface root refinement (a control-numerical-opt transfer).
        let mut t_prev = 0.0f32;
        let mut d_prev = f32::INFINITY;
        for i in 0..self.max_steps {
            let p = ray.at(t);
            let f = field(p);
            let radius = f.dist.abs();
            let eps = self.eps * (1.0 + t * 0.5) + self.lod_footprint * t;
            // Over-relaxation failure: the two safe spheres don't overlap, or a step crosses
            // into the solid. Across a signed-distance zero the spheres can touch EXACTLY
            // (e.g. a head-on plane), so the strict overlap inequality alone misses that case.
            // An origin already inside remains an immediate t=0 hit; only stepped samples are
            // rolled back. This assumes the field supplies a conservative distance bound.
            if (step_len > 0.0 && f.dist < 0.0) || step_len > radius + prev_radius {
                // The last guaranteed safe frontier is the previous sample plus its safe radius.
                // Resample there instead of letting an accelerated step report a buried hit.
                let gap_lo = t_prev + prev_radius;
                let gap_hi = t - radius;
                let mid = 0.5 * (gap_lo + gap_hi);
                let covered = f.dist > 0.0 && field(ray.at(mid)).dist >= 0.5 * (gap_hi - gap_lo);
                if !covered {
                    t = gap_lo;
                    match boost {
                        Boost::Omega => omega = omega.min(1.0),
                        Boost::Leap => subitize = 0.0,
                        Boost::Secant => secant_on = false,
                    }
                    boost = Boost::Omega;
                    prev_radius = 0.0;
                    step_len = 0.0;
                    d_prev = f32::INFINITY;
                    continue;
                }
            }
            if f.dist < eps {
                return Hit { hit: true, t, pos: p, normal: normal_fn(p), mat: f.mat, steps: i };
            }
            step_len = f.dist * omega;
            boost = Boost::Omega;
            // Subitize (a numerical-cognition transfer — the Approximate Number System's "leap
            // when it's clearly far"): well clear of any surface, multiply the step. The
            // over-relaxation overlap guard above is the safety net, so an over-leap is undone
            // rather than tunneling. 0.0 = off. See `Marcher::subitize`.
            if subitize > 0.0 && f.dist > eps * 6.0 {
                step_len *= 1.0 + subitize;
                boost = Boost::Leap;
            }
            // Secant / regula-falsi root refinement near the surface (control-numerical-opt):
            // estimate dd/dt from the last two samples and step toward the predicted root. On
            // grazing rays the slope is shallow, so the secant step exceeds the safe sphere step
            // — exactly where sphere tracing crawls — capped at 4·d (the over-relaxation overlap
            // test is the safety net). Gated to Lipschitz fields, like over-relaxation. Measured
            // on the profiler scene: march-phase field-evals -14%, total -5.3%, image mean Δ 0.19%.
            if secant_on && f.dist < 0.08 && d_prev.is_finite() {
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
            if t > self.max_dist {
                break;
            }
        }
        Hit { hit: false, t, pos: ray.at(t), normal: Vec3::ZERO, mat: 0, steps: self.max_steps }
    }

    /// Surface normal as the normalized field gradient (tetrahedron sampling: four `compare`s).
    pub fn normal<F: Fn(Vec3) -> Field + ?Sized>(&self, field: &F, p: Vec3) -> Vec3 {
        let h = self.normal_h;
        let k0 = Vec3::new(1.0, -1.0, -1.0);
        let k1 = Vec3::new(-1.0, -1.0, 1.0);
        let k2 = Vec3::new(-1.0, 1.0, -1.0);
        let k3 = Vec3::new(1.0, 1.0, 1.0);
        let g = k0.scale(field(p + k0.scale(h)).dist)
            + k1.scale(field(p + k1.scale(h)).dist)
            + k2.scale(field(p + k2.scale(h)).dist)
            + k3.scale(field(p + k3.scale(h)).dist);
        g.normalize()
    }

    /// Soft shadow factor in [0, 1] from `origin` toward `dir`, marching to `max_t`.
    /// `k` controls penumbra hardness (larger = sharper). The classic SDF shadow trick.
    pub fn soft_shadow<F: Fn(Vec3) -> Field + ?Sized>(
        &self,
        field: &F,
        origin: Vec3,
        dir: Vec3,
        max_t: f32,
        k: f32,
    ) -> f32 {
        let mut res = 1.0f32;
        // Stochastic soft shadow: the `hash` atom (a crypto-hashing → graphics transfer, the one
        // root atom the marcher otherwise skips) jitters the start by a blue-noise hash of the ray
        // origin, so the coarser shadow steps below dither across pixels instead of banding.
        // Measured −25.3% shadow-phase field-evals (−13.8% total, 29.4 → 31.2 fps) at an image mean
        // Δ of ~0.04–0.06/255 — edge-localized at penumbra boundaries (like secant), not free but
        // cheap for the biggest cost. Deterministic (hash of position), so still renders stably.
        let hb = origin.x * 127.1 + origin.y * 311.7 + origin.z * 74.7;
        let jitter = (hb.sin() * 43758.547).fract().abs();
        let mut t = 0.02 + jitter * 0.16;
        for _ in 0..self.shadow_steps {
            let h = field(origin + dir.scale(t)).dist;
            if h < 0.0008 {
                return 0.0;
            }
            res = res.min(k * h / t);
            // Step by the safe distance with a cap that grows with `t`: fine near the caster
            // (where the penumbra is shaped) and large leaps through open space far away.
            t += h.clamp(0.06, 0.5 + 0.7 * t);
            if t > max_t {
                break;
            }
        }
        res.clamp(0.0, 1.0)
    }

    /// Ambient occlusion in [0, 1] by probing the field along the normal (1 = fully open).
    /// `ao_samples == 0` skips the work and returns a fully-open 1.0. The probe base offset
    /// scales with `normal_h`, while analytic scenes keep the historical 0.01 base.
    pub fn ambient_occlusion<F: Fn(Vec3) -> Field + ?Sized>(&self, field: &F, p: Vec3, n: Vec3) -> f32 {
        let n_samples = self.ao_samples;
        if n_samples == 0 {
            return 1.0;
        }
        let base = self.normal_h.max(0.01);
        let span = (n_samples.max(2) - 1) as f32;
        let mut occ = 0.0f32;
        let mut sca = 1.0f32;
        for i in 0..n_samples {
            let hr = base + 0.12 * i as f32 / span;
            let d = field(p + n.scale(hr)).dist;
            occ += (hr - d) * sca;
            sca *= 0.92;
        }
        (1.0 - 2.6 * occ).clamp(0.0, 1.0)
    }
}
