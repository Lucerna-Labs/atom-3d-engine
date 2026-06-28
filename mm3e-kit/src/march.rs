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

/// Budget + tolerances for a march. Defaults are tuned for the example scenes.
#[derive(Clone, Copy, Debug)]
pub struct Marcher {
    pub max_steps: u32,
    pub max_dist: f32,
    pub eps: f32,
    /// Fraction of the safe distance to actually step. 1.0 for pure Lipschitz fields; lower
    /// (≈0.6) when the scene uses non-distance-preserving domain warps (twist/bend/repeat).
    pub step_scale: f32,
}

impl Default for Marcher {
    fn default() -> Self {
        Self { max_steps: 160, max_dist: 120.0, eps: 0.0006, step_scale: 1.0 }
    }
}

impl Marcher {
    /// Sphere-trace one ray through `field`, folding ray steps down to a `Hit`.
    pub fn march(&self, field: &dyn Fn(Vec3) -> Field, ray: &Ray) -> Hit {
        let mut t = 0.0f32;
        for i in 0..self.max_steps {
            let p = ray.at(t);
            let f = field(p);
            // Relax the surface tolerance with distance so far-away pixels don't over-march.
            let eps = self.eps * (1.0 + t * 0.5);
            if f.dist < eps {
                return Hit { hit: true, t, pos: p, normal: self.normal(field, p), mat: f.mat, steps: i };
            }
            t += f.dist * self.step_scale;
            if t > self.max_dist {
                break;
            }
        }
        Hit { hit: false, t, pos: ray.at(t), normal: Vec3::ZERO, mat: 0, steps: self.max_steps }
    }

    /// Surface normal as the normalized field gradient (tetrahedron sampling: four `compare`s).
    pub fn normal(&self, field: &dyn Fn(Vec3) -> Field, p: Vec3) -> Vec3 {
        let h = 0.0009;
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
    pub fn soft_shadow(&self, field: &dyn Fn(Vec3) -> Field, origin: Vec3, dir: Vec3, max_t: f32, k: f32) -> f32 {
        let mut res = 1.0f32;
        let mut t = 0.02;
        for _ in 0..64 {
            let h = field(origin + dir.scale(t)).dist;
            if h < 0.0008 {
                return 0.0;
            }
            res = res.min(k * h / t);
            t += h.clamp(0.01, 0.4);
            if t > max_t {
                break;
            }
        }
        res.clamp(0.0, 1.0)
    }

    /// Ambient occlusion in [0, 1] by probing the field along the normal (1 = fully open).
    pub fn ambient_occlusion(&self, field: &dyn Fn(Vec3) -> Field, p: Vec3, n: Vec3) -> f32 {
        let mut occ = 0.0f32;
        let mut sca = 1.0f32;
        for i in 0..5 {
            let hr = 0.01 + 0.12 * i as f32 / 4.0;
            let d = field(p + n.scale(hr)).dist;
            occ += (hr - d) * sca;
            sca *= 0.92;
        }
        (1.0 - 2.6 * occ).clamp(0.0, 1.0)
    }
}
