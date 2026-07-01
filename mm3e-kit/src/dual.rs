//! Forward-mode dual numbers — a value plus its exact 3-D gradient, propagated by the chain rule.
//! Seeding a world point's x/y/z as the three unit basis directions and running it through these
//! ops yields `(distance, exact gradient)` for any SDF built from `+ - * / sqrt abs min max`, in
//! ONE pass — replacing the tetrahedron trick's four extra samples (`Marcher::normal`) with an
//! algebraically exact gradient. Validated against the real engine in
//! `mm3e-orchestrator/examples/dual_normal_real.rs`: 0.00000° deviation from finite differences,
//! max 0.16° from the shipped tetrahedron normal at real hit points (explained by the tetrahedron's
//! own O(h) truncation error, not a discrepancy in this module) — real wall-clock win: -3.2% whole
//! frame, -50.3% on the isolated normal-computation cost. See the `dual-number-autodiff-lead` memory.
//!
//! `min`/`max` (and therefore CSG union/subtract/smooth-union) pick a subgradient at the active
//! branch — the same choice a finite difference makes locally, so this is not an approximation
//! away from CSG seams. `abs`/clamp-style ops are treated the same way (a.e. differentiable).

use crate::vec::Vec3;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A scalar value plus its gradient with respect to world position.
#[derive(Clone, Copy, Debug)]
pub struct Dual {
    pub v: f32,
    pub g: Vec3,
}

impl Dual {
    pub fn con(v: f32) -> Dual {
        Dual { v, g: Vec3::ZERO }
    }
    /// Multiply by a constant (a linear map — the common case for transform/CSG coefficients).
    pub fn scale(self, s: f32) -> Dual {
        Dual { v: self.v * s, g: self.g.scale(s) }
    }
    pub fn adds(self, s: f32) -> Dual {
        Dual { v: self.v + s, g: self.g }
    }
    pub fn abs(self) -> Dual {
        if self.v >= 0.0 {
            self
        } else {
            self.neg()
        }
    }
    pub fn sqrt(self) -> Dual {
        let v = self.v.max(0.0).sqrt();
        let k = if v > 1e-9 { 0.5 / v } else { 0.0 };
        Dual { v, g: self.g.scale(k) }
    }
    /// Subgradient pick at the active branch (matches `f32::min`'s `<=` tie behavior).
    pub fn dmin(self, o: Dual) -> Dual {
        if self.v <= o.v {
            self
        } else {
            o
        }
    }
    pub fn dmax(self, o: Dual) -> Dual {
        if self.v >= o.v {
            self
        } else {
            o
        }
    }
    pub fn max0(self) -> Dual {
        self.dmax(Dual::con(0.0))
    }
    /// Max against a constant (the eps-clamp pattern `x.max(c)` where `c` isn't necessarily 0).
    pub fn maxc(self, c: f32) -> Dual {
        if self.v >= c {
            self
        } else {
            Dual::con(c)
        }
    }
    pub fn clamp(self, lo: f32, hi: f32) -> Dual {
        self.dmax(Dual::con(lo)).dmin(Dual::con(hi))
    }
}

impl std::ops::Add for Dual {
    type Output = Dual;
    fn add(self, o: Dual) -> Dual {
        Dual { v: self.v + o.v, g: self.g + o.g }
    }
}
impl std::ops::Sub for Dual {
    type Output = Dual;
    fn sub(self, o: Dual) -> Dual {
        Dual { v: self.v - o.v, g: self.g - o.g }
    }
}
impl std::ops::Neg for Dual {
    type Output = Dual;
    fn neg(self) -> Dual {
        Dual { v: -self.v, g: -self.g }
    }
}
/// Product rule: `d(ab) = a'b + ab'`.
impl std::ops::Mul for Dual {
    type Output = Dual;
    fn mul(self, o: Dual) -> Dual {
        Dual { v: self.v * o.v, g: self.g.scale(o.v) + o.g.scale(self.v) }
    }
}
/// Quotient rule: `d(a/b) = (a'b - ab')/b^2`. Only needed where the divisor is itself variable
/// (e.g. ellipsoid); dividing by a constant is `scale(1.0/s)`.
impl std::ops::Div for Dual {
    type Output = Dual;
    fn div(self, o: Dual) -> Dual {
        let inv2 = 1.0 / (o.v * o.v);
        Dual { v: self.v / o.v, g: (self.g.scale(o.v) - o.g.scale(self.v)).scale(inv2) }
    }
}

/// Seed a world point's three coordinates as dual variables (unit basis gradients), so any
/// arithmetic on them carries the exact gradient of the resulting scalar w.r.t. world position.
pub fn seed(p: Vec3) -> (Dual, Dual, Dual) {
    (
        Dual { v: p.x, g: Vec3::new(1.0, 0.0, 0.0) },
        Dual { v: p.y, g: Vec3::new(0.0, 1.0, 0.0) },
        Dual { v: p.z, g: Vec3::new(0.0, 0.0, 1.0) },
    )
}

// ----------------------------------------------------------------------------
// Dual-number primitive distances — one-to-one ports of `crate::sdf`'s functions, taking a
// point already in the shape's local space as three dual scalars.
// ----------------------------------------------------------------------------

pub fn sphere(lx: Dual, ly: Dual, lz: Dual, r: f32) -> Dual {
    lx.mul(lx).add(ly.mul(ly)).add(lz.mul(lz)).sqrt().adds(-r)
}

pub fn boxed(lx: Dual, ly: Dual, lz: Dual, half: Vec3) -> Dual {
    let qx = lx.abs().adds(-half.x);
    let qy = ly.abs().adds(-half.y);
    let qz = lz.abs().adds(-half.z);
    let outside = qx.max0().mul(qx.max0()).add(qy.max0().mul(qy.max0())).add(qz.max0().mul(qz.max0())).sqrt();
    let inside = qx.dmax(qy.dmax(qz)).dmin(Dual::con(0.0));
    outside.add(inside)
}

pub fn rounded_box(lx: Dual, ly: Dual, lz: Dual, half: Vec3, radius: f32) -> Dual {
    boxed(lx, ly, lz, half - Vec3::splat(radius)).adds(-radius)
}

pub fn plane(lx: Dual, ly: Dual, lz: Dual, n: Vec3, h: f32) -> Dual {
    lx.scale(n.x).add(ly.scale(n.y)).add(lz.scale(n.z)).adds(h)
}

pub fn torus(lx: Dual, ly: Dual, lz: Dual, major: f32, minor: f32) -> Dual {
    let q = lx.mul(lx).add(lz.mul(lz)).sqrt().adds(-major);
    q.mul(q).add(ly.mul(ly)).sqrt().adds(-minor)
}

pub fn cylinder(lx: Dual, ly: Dual, lz: Dual, h: f32, r: f32) -> Dual {
    let dx = lx.mul(lx).add(lz.mul(lz)).sqrt().adds(-r);
    let dy = ly.abs().adds(-h);
    let inside = dx.dmax(dy).dmin(Dual::con(0.0));
    let ox = dx.max0();
    let oy = dy.max0();
    let outside = ox.mul(ox).add(oy.mul(oy)).sqrt();
    inside.add(outside)
}

pub fn capsule(lx: Dual, ly: Dual, lz: Dual, a: Vec3, b: Vec3, r: f32) -> Dual {
    let (pax, pay, paz) = (lx.adds(-a.x), ly.adds(-a.y), lz.adds(-a.z));
    let ba = b - a;
    let dot_ba_ba = ba.dot(ba).max(1e-12);
    let dot_pa_ba = pax.scale(ba.x).add(pay.scale(ba.y)).add(paz.scale(ba.z));
    let hh = dot_pa_ba.scale(1.0 / dot_ba_ba).clamp(0.0, 1.0);
    let rx = pax.sub(hh.scale(ba.x));
    let ry = pay.sub(hh.scale(ba.y));
    let rz = paz.sub(hh.scale(ba.z));
    rx.mul(rx).add(ry.mul(ry)).add(rz.mul(rz)).sqrt().adds(-r)
}

pub fn round_cone(lx: Dual, ly: Dual, lz: Dual, r1: f32, r2: f32, h: f32) -> Dual {
    let qx = lx.mul(lx).add(lz.mul(lz)).sqrt();
    let qy = ly;
    let b = (r1 - r2) / h.max(1e-6);
    let a = (1.0f32 - b * b).max(0.0).sqrt();
    let k = qx.scale(-b).add(qy.scale(a));
    if k.v < 0.0 {
        qx.mul(qx).add(qy.mul(qy)).sqrt().adds(-r1)
    } else if k.v > a * h {
        let qyh = qy.adds(-h);
        qx.mul(qx).add(qyh.mul(qyh)).sqrt().adds(-r2)
    } else {
        qx.scale(a).add(qy.scale(b)).adds(-r1)
    }
}

pub fn ellipsoid(lx: Dual, ly: Dual, lz: Dual, r: Vec3) -> Dual {
    let k0x = lx.scale(1.0 / r.x);
    let k0y = ly.scale(1.0 / r.y);
    let k0z = lz.scale(1.0 / r.z);
    let k0 = k0x.mul(k0x).add(k0y.mul(k0y)).add(k0z.mul(k0z)).sqrt();
    if k0.v < 1e-6 {
        return Dual::con(-r.x.min(r.y).min(r.z));
    }
    let k1x = lx.scale(1.0 / (r.x * r.x));
    let k1y = ly.scale(1.0 / (r.y * r.y));
    let k1z = lz.scale(1.0 / (r.z * r.z));
    let k1 = k1x.mul(k1x).add(k1y.mul(k1y)).add(k1z.mul(k1z)).sqrt().maxc(1e-9);
    k0.mul(k0.adds(-1.0)).div(k1)
}

pub fn octahedron(lx: Dual, ly: Dual, lz: Dual, s: f32) -> Dual {
    lx.abs().add(ly.abs()).add(lz.abs()).adds(-s).scale(0.577_350_3)
}

pub fn hex_prism(lx: Dual, ly: Dual, lz: Dual, r: f32, h: f32) -> Dual {
    let (kx, ky, kz) = (-0.866_025_4_f32, 0.5_f32, 0.577_35_f32);
    let ax0 = lx.abs();
    let az0 = lz.abs();
    let ay = ly.abs().adds(-h);
    let tpre = ax0.scale(kx).add(az0.scale(ky));
    let t = tpre.dmin(Dual::con(0.0)).scale(2.0);
    let ax = ax0.sub(t.scale(kx));
    let az = az0.sub(t.scale(ky));
    let clamped = ax.clamp(-kz * r, kz * r);
    let diffx = ax.sub(clamped);
    let diffz = az.adds(-r);
    let sign = if diffz.v >= 0.0 { 1.0f32 } else { -1.0f32 };
    let dx = diffx.mul(diffx).add(diffz.mul(diffz)).sqrt().scale(sign);
    let dy = ay;
    let inside = dx.dmax(dy).dmin(Dual::con(0.0));
    let ox = dx.max0();
    let oy = dy.max0();
    let outside = ox.mul(ox).add(oy.mul(oy)).sqrt();
    inside.add(outside)
}

// ----------------------------------------------------------------------------
// Point-space domain modifiers (applied before the primitive) and distance-space modifiers
// (applied after) — dual ports of the `op_*` functions actually reachable via `Modifiers`.
// `twist`/`bend` are NOT ported (rotate-by-a-position-dependent-angle needs dual sin/cos and are
// used by exactly one example scene in the whole codebase) — callers detect their use and fall
// back to the tetrahedron normal for that scene, never producing a wrong gradient.
// ----------------------------------------------------------------------------

pub fn op_mirror(lx: Dual, ly: Dual, lz: Dual, mx: bool, my: bool, mz: bool) -> (Dual, Dual, Dual) {
    (if mx { lx.abs() } else { lx }, if my { ly.abs() } else { ly }, if mz { lz.abs() } else { lz })
}

pub fn op_elongate(lx: Dual, ly: Dual, lz: Dual, hgt: Vec3) -> (Dual, Dual, Dual) {
    let cx = lx.clamp(-hgt.x, hgt.x);
    let cy = ly.clamp(-hgt.y, hgt.y);
    let cz = lz.clamp(-hgt.z, hgt.z);
    (lx.sub(cx), ly.sub(cy), lz.sub(cz))
}

fn rep_axis(v: Dual, period: f32) -> Dual {
    if period > 0.0 {
        // The cell index is a locally-constant integer (translation within a cell has gradient 1);
        // only wrong at the measure-zero cell boundary, matching the tetrahedron method's own
        // behavior there (a one-sided finite difference also picks one cell arbitrarily).
        v.adds(-(v.v / period).round() * period)
    } else {
        v
    }
}
pub fn op_repeat(lx: Dual, ly: Dual, lz: Dual, period: Vec3) -> (Dual, Dual, Dual) {
    (rep_axis(lx, period.x), rep_axis(ly, period.y), rep_axis(lz, period.z))
}

pub fn op_round(d: Dual, r: f32) -> Dual {
    d.adds(-r)
}
pub fn op_onion(d: Dual, thickness: f32) -> Dual {
    d.abs().adds(-thickness)
}

// ----------------------------------------------------------------------------
// CSG combinators — dual ports of the three modes `Combine` actually exposes.
// ----------------------------------------------------------------------------

pub fn union(a: Dual, b: Dual) -> Dual {
    a.dmin(b)
}
pub fn subtract(a: Dual, b: Dual) -> Dual {
    a.dmax(b.neg())
}
pub fn smooth_union(a: Dual, b: Dual, k: f32) -> Dual {
    if k <= 0.0 {
        return union(a, b);
    }
    let h = b.sub(a).scale(0.5 / k).adds(0.5).clamp(0.0, 1.0);
    let one_minus_h = Dual::con(1.0).sub(h);
    b.mul(one_minus_h).add(a.mul(h)).sub(h.mul(one_minus_h).scale(k))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Central-difference reference gradient of any `f: Vec3 -> f32` at `p`.
    fn finite_diff(f: impl Fn(Vec3) -> f32, p: Vec3) -> Vec3 {
        let h = 1e-4;
        let gx = (f(p + Vec3::new(h, 0.0, 0.0)) - f(p - Vec3::new(h, 0.0, 0.0))) / (2.0 * h);
        let gy = (f(p + Vec3::new(0.0, h, 0.0)) - f(p - Vec3::new(0.0, h, 0.0))) / (2.0 * h);
        let gz = (f(p + Vec3::new(0.0, 0.0, h)) - f(p - Vec3::new(0.0, 0.0, h))) / (2.0 * h);
        Vec3::new(gx, gy, gz).normalize()
    }

    fn check(name: &str, dual_f: impl Fn(Vec3) -> Dual, scalar_f: impl Fn(Vec3) -> f32, points: &[Vec3]) {
        for &p in points {
            let d = dual_f(p);
            let fd = finite_diff(&scalar_f, p);
            let dn = d.g.normalize();
            let cos = dn.dot(fd).clamp(-1.0, 1.0);
            let deg = cos.acos().to_degrees();
            assert!(deg < 0.5, "{name} gradient mismatch at {p:?}: {deg:.4} deg (dual={dn:?} fd={fd:?})");
            let sv = scalar_f(p);
            assert!((d.v - sv).abs() < 1e-3, "{name} value mismatch at {p:?}: dual={} scalar={sv}", d.v);
        }
    }

    #[test]
    fn sphere_gradient_matches_finite_difference() {
        let pts = [Vec3::new(1.0, 0.3, -0.2), Vec3::new(-0.6, 1.1, 0.4), Vec3::new(0.05, -1.2, 0.9)];
        check(
            "sphere",
            |p| {
                let (x, y, z) = seed(p);
                sphere(x, y, z, 1.0)
            },
            |p| crate::sdf::sphere(p, 1.0),
            &pts,
        );
    }

    #[test]
    fn boxed_gradient_matches_finite_difference() {
        let half = Vec3::new(0.6, 0.8, 0.5);
        let pts = [Vec3::new(0.9, 0.2, 0.1), Vec3::new(0.3, 1.1, 0.05), Vec3::new(-0.9, -0.3, 0.7)];
        check(
            "boxed",
            |p| {
                let (x, y, z) = seed(p);
                boxed(x, y, z, half)
            },
            |p| crate::sdf::boxed(p, half),
            &pts,
        );
    }

    #[test]
    fn rounded_box_gradient_matches_finite_difference() {
        let half = Vec3::splat(0.85);
        let pts = [Vec3::new(1.0, 0.2, -0.3), Vec3::new(0.1, 1.05, 0.4), Vec3::new(-1.0, -0.4, 0.2)];
        check(
            "rounded_box",
            |p| {
                let (x, y, z) = seed(p);
                rounded_box(x, y, z, half, 0.18)
            },
            |p| crate::sdf::rounded_box(p, half, 0.18),
            &pts,
        );
    }

    #[test]
    fn plane_gradient_matches_finite_difference() {
        let n = Vec3::new(0.0, 1.0, 0.0);
        let pts = [Vec3::new(1.0, 0.5, 2.0), Vec3::new(-3.0, 2.2, 0.0)];
        check(
            "plane",
            |p| {
                let (x, y, z) = seed(p);
                plane(x, y, z, n, 0.0)
            },
            |p| crate::sdf::plane(p, n, 0.0),
            &pts,
        );
    }

    #[test]
    fn torus_gradient_matches_finite_difference() {
        let pts = [Vec3::new(0.95, 0.05, 0.1), Vec3::new(0.6, 0.25, 0.6), Vec3::new(-0.9, -0.1, 0.2)];
        check(
            "torus",
            |p| {
                let (x, y, z) = seed(p);
                torus(x, y, z, 0.85, 0.3)
            },
            |p| crate::sdf::torus(p, 0.85, 0.3),
            &pts,
        );
    }

    #[test]
    fn cylinder_gradient_matches_finite_difference() {
        let pts = [Vec3::new(0.6, 0.5, 0.1), Vec3::new(0.2, 1.1, -0.3), Vec3::new(-0.5, -1.0, 0.4)];
        check(
            "cylinder",
            |p| {
                let (x, y, z) = seed(p);
                cylinder(x, y, z, 1.0, 0.6)
            },
            |p| crate::sdf::cylinder(p, 1.0, 0.6),
            &pts,
        );
    }

    #[test]
    fn capsule_gradient_matches_finite_difference() {
        let a = Vec3::new(0.0, -0.5, 0.0);
        let b = Vec3::new(0.0, 0.5, 0.0);
        let pts = [Vec3::new(0.4, 0.0, 0.1), Vec3::new(0.1, 0.9, 0.05), Vec3::new(-0.3, -0.9, 0.2)];
        check(
            "capsule",
            |p| {
                let (x, y, z) = seed(p);
                capsule(x, y, z, a, b, 0.3)
            },
            |p| crate::sdf::capsule(p, a, b, 0.3),
            &pts,
        );
    }

    #[test]
    fn round_cone_gradient_matches_finite_difference() {
        let pts = [Vec3::new(0.5, 0.5, 0.0), Vec3::new(0.2, 1.4, 0.1), Vec3::new(0.35, -0.2, -0.2)];
        check(
            "round_cone",
            |p| {
                let (x, y, z) = seed(p);
                round_cone(x, y, z, 0.5, 0.15, 1.5)
            },
            |p| crate::sdf::round_cone(p, 0.5, 0.15, 1.5),
            &pts,
        );
    }

    #[test]
    fn ellipsoid_gradient_matches_finite_difference() {
        let r = Vec3::new(0.8, 1.2, 0.5);
        let pts = [Vec3::new(0.75, 0.1, 0.05), Vec3::new(0.1, 1.15, 0.1), Vec3::new(-0.3, -0.5, 0.45)];
        check(
            "ellipsoid",
            |p| {
                let (x, y, z) = seed(p);
                ellipsoid(x, y, z, r)
            },
            |p| crate::sdf::ellipsoid(p, r),
            &pts,
        );
    }

    #[test]
    fn octahedron_gradient_matches_finite_difference() {
        let pts = [Vec3::new(0.3, 0.3, 0.3), Vec3::new(0.5, 0.05, 0.1), Vec3::new(-0.2, 0.4, -0.15)];
        check(
            "octahedron",
            |p| {
                let (x, y, z) = seed(p);
                octahedron(x, y, z, 0.7)
            },
            |p| crate::sdf::octahedron(p, 0.7),
            &pts,
        );
    }

    #[test]
    fn hex_prism_gradient_matches_finite_difference() {
        let pts = [Vec3::new(0.5, 0.1, 0.05), Vec3::new(0.1, 0.9, 0.4), Vec3::new(-0.4, -0.3, 0.5)];
        check(
            "hex_prism",
            |p| {
                let (x, y, z) = seed(p);
                hex_prism(x, y, z, 0.6, 1.0)
            },
            |p| crate::sdf::hex_prism(p, 0.6, 1.0),
            &pts,
        );
    }

    #[test]
    fn smooth_union_gradient_matches_finite_difference() {
        let pts = [Vec3::new(0.3, 0.0, 0.0), Vec3::new(0.0, 0.3, 0.0), Vec3::new(0.6, 0.6, 0.0)];
        check(
            "smooth_union",
            |p| {
                let (x, y, z) = seed(p);
                let a = sphere(x, y, z, 0.5);
                let (x2, y2, z2) = seed(p);
                let b = sphere(x2.adds(-0.7), y2, z2, 0.5);
                smooth_union(a, b, 0.3)
            },
            |p| {
                let a = crate::sdf::sphere(p, 0.5);
                let b = crate::sdf::sphere(p - Vec3::new(0.7, 0.0, 0.0), 0.5);
                crate::sdf::smooth_union(crate::sdf::Field::new(a, 0), crate::sdf::Field::new(b, 0), 0.3).dist
            },
            &pts,
        );
    }

    #[test]
    fn subtract_gradient_matches_finite_difference() {
        let pts = [Vec3::new(0.9, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.9), Vec3::new(0.5, 0.5, 0.0)];
        check(
            "subtract",
            |p| {
                let (x, y, z) = seed(p);
                let a = sphere(x, y, z, 1.0);
                let (x2, y2, z2) = seed(p);
                let b = sphere(x2, y2, z2, 0.6);
                subtract(a, b)
            },
            |p| {
                let a = crate::sdf::sphere(p, 1.0);
                let b = crate::sdf::sphere(p, 0.6);
                crate::sdf::subtract(crate::sdf::Field::new(a, 0), crate::sdf::Field::new(b, 0)).dist
            },
            &pts,
        );
    }
}
