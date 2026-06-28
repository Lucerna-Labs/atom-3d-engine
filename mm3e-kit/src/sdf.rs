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
