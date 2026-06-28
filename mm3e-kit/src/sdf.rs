//! Signed-distance primitives and CSG combinators — the 3-D elevation of MMPE's 2-D
//! `signed_distance`. Each primitive is the `compare` root atom (a distance): negative
//! inside, zero on the surface, positive outside, all in the shape's *local* space. The
//! combinators (`union`/`intersect`/`subtract`/`smooth_union`) are the 3-D analog of the
//! 2-D alpha-over compositing the orchestrator used to merge shapes — here they merge solids.
//!
//! Mechanism only: this module never decides which primitives exist or how they combine.
//! The orchestrator composes these into a single world field `Fn(Vec3) -> Field`.

use crate::vec::Vec3;

/// A sampled field value: the signed distance plus the material id of the nearest surface.
/// Carrying the material through the CSG tree is what lets `union` answer "whose surface is
/// this?" — the 3-D analog of the painter's algorithm picking the top shape's color.
#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub dist: f32,
    pub mat: u32,
}

impl Field {
    pub fn new(dist: f32, mat: u32) -> Field {
        Field { dist, mat }
    }
    pub const FAR: Field = Field { dist: f32::INFINITY, mat: 0 };
}

// ----------------------------------------------------------------------------
// Primitive distances (local space). Exact analytic SDFs — the canonical kit.
// ----------------------------------------------------------------------------

/// Distance to a sphere of radius `r` centered at the local origin.
pub fn sphere(p: Vec3, r: f32) -> f32 {
    p.length() - r
}

/// Distance to an axis-aligned box with half-extents `half`.
pub fn boxed(p: Vec3, half: Vec3) -> f32 {
    let q = p.abs() - half;
    q.max_scalar(0.0).length() + q.max_element().min(0.0)
}

/// Distance to a box with half-extents `half`, edges rounded by `radius`.
pub fn rounded_box(p: Vec3, half: Vec3, radius: f32) -> f32 {
    boxed(p, half - Vec3::splat(radius)) - radius
}

/// Distance to an infinite plane with unit normal `n` at signed offset `h` from the origin.
pub fn plane(p: Vec3, n: Vec3, h: f32) -> f32 {
    p.dot(n) + h
}

/// Distance to a torus in the XZ plane: `major` ring radius, `minor` tube radius.
pub fn torus(p: Vec3, major: f32, minor: f32) -> f32 {
    let q = (p.x * p.x + p.z * p.z).sqrt() - major;
    (q * q + p.y * p.y).sqrt() - minor
}

/// Distance to a Y-axis capped cylinder: half-height `h`, radius `r`.
pub fn cylinder(p: Vec3, h: f32, r: f32) -> f32 {
    let dx = (p.x * p.x + p.z * p.z).sqrt() - r;
    let dy = p.y.abs() - h;
    let ox = dx.max(0.0);
    let oy = dy.max(0.0);
    dx.max(dy).min(0.0) + (ox * ox + oy * oy).sqrt()
}

/// Distance to a capsule: a segment from `a` to `b` of radius `r`.
pub fn capsule(p: Vec3, a: Vec3, b: Vec3, r: f32) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = (pa.dot(ba) / ba.dot(ba)).clamp(0.0, 1.0);
    (pa - ba.scale(h)).length() - r
}

/// Distance to a round cone along +Y: radius `r1` at the base (origin), tapering to `r2` at
/// height `h`, with spherical end caps.
pub fn round_cone(p: Vec3, r1: f32, r2: f32, h: f32) -> f32 {
    let qx = (p.x * p.x + p.z * p.z).sqrt();
    let qy = p.y;
    let b = (r1 - r2) / h.max(1e-6);
    let a = (1.0 - b * b).max(0.0).sqrt();
    let k = qx * -b + qy * a;
    if k < 0.0 {
        return (qx * qx + qy * qy).sqrt() - r1;
    }
    if k > a * h {
        return (qx * qx + (qy - h) * (qy - h)).sqrt() - r2;
    }
    qx * a + qy * b - r1
}

/// Approximate distance to an axis-aligned ellipsoid with per-axis radii `r` (IQ's bound).
pub fn ellipsoid(p: Vec3, r: Vec3) -> f32 {
    let k0 = Vec3::new(p.x / r.x, p.y / r.y, p.z / r.z).length();
    if k0 < 1e-6 {
        return -r.x.min(r.y).min(r.z);
    }
    let k1 = Vec3::new(p.x / (r.x * r.x), p.y / (r.y * r.y), p.z / (r.z * r.z)).length();
    k0 * (k0 - 1.0) / k1.max(1e-9)
}

/// Distance to an octahedron of size `s` (Lipschitz bound form — valid, slightly conservative).
pub fn octahedron(p: Vec3, s: f32) -> f32 {
    let p = p.abs();
    (p.x + p.y + p.z - s) * 0.577_350_3
}

/// Distance to a Y-extruded regular hexagonal prism: in-radius `r`, half-height `h`.
pub fn hex_prism(p: Vec3, r: f32, h: f32) -> f32 {
    let (kx, ky, kz) = (-0.866_025_4_f32, 0.5_f32, 0.577_35_f32);
    let mut a = p.abs();
    let t = 2.0 * (kx * a.x + ky * a.z).min(0.0);
    a.x -= t * kx;
    a.z -= t * ky;
    let clamped = a.x.clamp(-kz * r, kz * r);
    let dx = ((a.x - clamped) * (a.x - clamped) + (a.z - r) * (a.z - r)).sqrt() * (a.z - r).signum();
    let dy = a.y - h;
    dx.max(dy).min(0.0) + (dx.max(0.0).powi(2) + dy.max(0.0).powi(2)).sqrt()
}

// ----------------------------------------------------------------------------
// CSG combinators over `Field` (the constructive solid geometry vocabulary).
// ----------------------------------------------------------------------------

/// Boolean union: the nearer surface wins (and carries its material).
pub fn union(a: Field, b: Field) -> Field {
    if a.dist <= b.dist {
        a
    } else {
        b
    }
}

/// Boolean intersection: the farther bounding surface wins.
pub fn intersect(a: Field, b: Field) -> Field {
    if a.dist >= b.dist {
        a
    } else {
        b
    }
}

/// Boolean subtraction: `a` minus the solid `b`. Keeps `a`'s material.
pub fn subtract(a: Field, b: Field) -> Field {
    Field::new(a.dist.max(-b.dist), a.mat)
}

/// Polynomial smooth-min union with blend radius `k` — the organic blend that gives
/// raymarched scenes their signature look. This is `combine` (a weighted mix) applied to
/// two distance fields. `k → 0` recovers a hard `union`.
pub fn smooth_union(a: Field, b: Field, k: f32) -> Field {
    if k <= 0.0 {
        return union(a, b);
    }
    let h = (0.5 + 0.5 * (b.dist - a.dist) / k).clamp(0.0, 1.0);
    let dist = b.dist * (1.0 - h) + a.dist * h - k * h * (1.0 - h);
    let mat = if h > 0.5 { a.mat } else { b.mat };
    Field::new(dist, mat)
}

/// Smooth subtraction of `b` from `a` with blend radius `k`. Keeps `a`'s material.
pub fn smooth_subtract(a: Field, b: Field, k: f32) -> Field {
    if k <= 0.0 {
        return subtract(a, b);
    }
    let h = (0.5 - 0.5 * (a.dist + b.dist) / k).clamp(0.0, 1.0);
    let dist = a.dist * (1.0 - h) + (-b.dist) * h + k * h * (1.0 - h);
    Field::new(dist, a.mat)
}

/// Smooth intersection of `a` and `b` with blend radius `k`.
pub fn smooth_intersect(a: Field, b: Field, k: f32) -> Field {
    if k <= 0.0 {
        return intersect(a, b);
    }
    let h = (0.5 - 0.5 * (a.dist - b.dist) / k).clamp(0.0, 1.0);
    let dist = a.dist * (1.0 - h) + b.dist * h + k * h * (1.0 - h);
    let mat = if h > 0.5 { b.mat } else { a.mat };
    Field::new(dist, mat)
}

// ----------------------------------------------------------------------------
// Domain operators — point/distance transforms that turn the primitive kit into an
// SDF modeler. Point ops (`repeat`/`twist`/`bend`/`elongate`/`mirror`) are applied to the
// query point *before* the primitive; distance ops (`round`/`onion`) modify its result.
// NOTE: `twist`/`bend`/`repeat` are not perfectly distance-preserving (Lipschitz < 1 locally),
// so the marcher under-relaxes its step (see `Marcher::step_scale`) to stay artifact-free.
// ----------------------------------------------------------------------------

/// Round any shape outward by `r` (the `op_round` of constructive modeling).
pub fn op_round(d: f32, r: f32) -> f32 {
    d - r
}

/// Hollow a solid into a shell of the given `thickness`.
pub fn op_onion(d: f32, thickness: f32) -> f32 {
    d.abs() - thickness
}

/// Elongate (stretch) the domain by half-extents `h`, turning a point shape into a rounded slab.
pub fn op_elongate(p: Vec3, h: Vec3) -> Vec3 {
    p - p.clamp_to(h.scale(-1.0), h)
}

fn rep_axis(v: f32, period: f32) -> f32 {
    if period > 0.0 {
        v - period * (v / period).round()
    } else {
        v
    }
}

/// Infinite repetition of the domain on a lattice of spacing `period` (0 on an axis = no repeat).
pub fn op_repeat(p: Vec3, period: Vec3) -> Vec3 {
    Vec3::new(rep_axis(p.x, period.x), rep_axis(p.y, period.y), rep_axis(p.z, period.z))
}

/// Twist the domain around +Y by `k` radians per unit height.
pub fn op_twist(p: Vec3, k: f32) -> Vec3 {
    let a = k * p.y;
    let (s, c) = (a.sin(), a.cos());
    Vec3::new(c * p.x - s * p.z, p.y, s * p.x + c * p.z)
}

/// Bend the domain around +Z by `k` radians per unit X.
pub fn op_bend(p: Vec3, k: f32) -> Vec3 {
    let a = k * p.x;
    let (s, c) = (a.sin(), a.cos());
    Vec3::new(c * p.x - s * p.y, s * p.x + c * p.y, p.z)
}

/// Mirror the domain so it is symmetric across the given axis planes (component-wise abs).
pub fn op_mirror(p: Vec3, x: bool, y: bool, z: bool) -> Vec3 {
    Vec3::new(if x { p.x.abs() } else { p.x }, if y { p.y.abs() } else { p.y }, if z { p.z.abs() } else { p.z })
}
