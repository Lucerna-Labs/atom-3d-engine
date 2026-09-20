//! Geometry mechanism: 3-D vectors, a 3×3 rotation matrix, and a rigid+uniform-scale
//! transform. `Vec3::dot` is the `project` root atom; `normalize` is `scale`; `Transform`
//! is the 3-D analog of MMPE's 2-D `Affine`. Pure mechanism — no rendering decisions.

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    /// `project` — the dot product (a vector through a basis).
    pub fn dot(self, o: Vec3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }
    pub fn length_sq(self) -> f32 {
        self.dot(self)
    }
    pub fn length(self) -> f32 {
        self.length_sq().sqrt()
    }
    /// `scale` — divide by the reference length to a unit vector (guarded against zero).
    pub fn normalize(self) -> Vec3 {
        let len = self.length();
        if len > 1e-12 {
            self.scale(1.0 / len)
        } else {
            Vec3::ZERO
        }
    }
    pub fn scale(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
    /// Component-wise (Hadamard) product — used to tint light by surface albedo.
    pub fn cmul(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x * o.x, self.y * o.y, self.z * o.z)
    }
    pub fn abs(self) -> Vec3 {
        Vec3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
    pub fn min(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x.min(o.x), self.y.min(o.y), self.z.min(o.z))
    }
    pub fn max(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x.max(o.x), self.y.max(o.y), self.z.max(o.z))
    }
    pub fn max_scalar(self, m: f32) -> Vec3 {
        Vec3::new(self.x.max(m), self.y.max(m), self.z.max(m))
    }
    pub fn max_element(self) -> f32 {
        self.x.max(self.y).max(self.z)
    }
    pub fn clamp01(self) -> Vec3 {
        Vec3::new(self.x.clamp(0.0, 1.0), self.y.clamp(0.0, 1.0), self.z.clamp(0.0, 1.0))
    }
    /// Component-wise clamp into the box `[lo, hi]`.
    pub fn clamp_to(self, lo: Vec3, hi: Vec3) -> Vec3 {
        self.max(lo).min(hi)
    }
    /// Linear interpolation toward `o` by `t`.
    pub fn mix(self, o: Vec3, t: f32) -> Vec3 {
        self.scale(1.0 - t) + o.scale(t)
    }
    /// Reflect this direction about a unit normal `n`.
    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - n.scale(2.0 * self.dot(n))
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f32) -> Vec3 {
        self.scale(s)
    }
}

/// A 3×3 matrix stored as three column vectors. Used for rotation (its transpose is its
/// inverse). `mul_vec` is the `project` atom specialized to a 3-vector × matrix.
#[derive(Clone, Copy, Debug)]
pub struct Mat3 {
    pub cols: [Vec3; 3],
}

impl Mat3 {
    pub const IDENTITY: Mat3 =
        Mat3 { cols: [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)] };

    pub fn from_cols(c0: Vec3, c1: Vec3, c2: Vec3) -> Mat3 {
        Mat3 { cols: [c0, c1, c2] }
    }

    /// `project`: transform a vector through the matrix.
    pub fn mul_vec(self, v: Vec3) -> Vec3 {
        self.cols[0].scale(v.x) + self.cols[1].scale(v.y) + self.cols[2].scale(v.z)
    }

    /// Transpose — for an orthonormal rotation this is the inverse.
    pub fn transpose(self) -> Mat3 {
        let [a, b, c] = self.cols;
        Mat3::from_cols(Vec3::new(a.x, b.x, c.x), Vec3::new(a.y, b.y, c.y), Vec3::new(a.z, b.z, c.z))
    }

    /// Rotation by `angle` (radians) about a unit `axis`, via Rodrigues' rotation formula.
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Mat3 {
        let k = axis.normalize();
        let (s, c) = (angle.sin(), angle.cos());
        let col = |e: Vec3| e.scale(c) + k.cross(e).scale(s) + k.scale(k.dot(e) * (1.0 - c));
        Mat3::from_cols(col(Vec3::new(1.0, 0.0, 0.0)), col(Vec3::new(0.0, 1.0, 0.0)), col(Vec3::new(0.0, 0.0, 1.0)))
    }

    /// Euler rotation, applied Z then Y then X (yaw-pitch-roll-ish), composed left-to-right.
    pub fn from_euler(x: f32, y: f32, z: f32) -> Mat3 {
        let rx = Mat3::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), x);
        let ry = Mat3::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), y);
        let rz = Mat3::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), z);
        rx * ry * rz
    }
}

impl std::ops::Mul for Mat3 {
    type Output = Mat3;
    /// Matrix product `self * other` (compose: apply `other` first, then `self`).
    fn mul(self, o: Mat3) -> Mat3 {
        Mat3::from_cols(self.mul_vec(o.cols[0]), self.mul_vec(o.cols[1]), self.mul_vec(o.cols[2]))
    }
}

/// A rigid transform plus a uniform scale: world point `p` maps to local via
/// `local = Rᵀ·(p − pos) / scale`. Because rotation+translation is an isometry, the
/// world-space distance is just `scale · sdf(local)` — the 3-D analog of MMPE's
/// `Affine::scale_factor` band. This is what keeps an SDF valid after placing it.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub rot: Mat3,
    pub pos: Vec3,
    pub scale: f32,
}

impl Transform {
    pub const IDENTITY: Transform = Transform { rot: Mat3::IDENTITY, pos: Vec3::ZERO, scale: 1.0 };

    pub fn new(pos: Vec3, rot: Mat3, scale: f32) -> Transform {
        Transform { rot, pos, scale }
    }
    pub fn at(pos: Vec3) -> Transform {
        Transform { rot: Mat3::IDENTITY, pos, scale: 1.0 }
    }
    pub fn rotated(mut self, rot: Mat3) -> Transform {
        self.rot = rot;
        self
    }
    pub fn scaled(mut self, scale: f32) -> Transform {
        self.scale = scale;
        self
    }

    /// Map a local-space point into world space.
    pub fn to_world(&self, p: Vec3) -> Vec3 {
        self.pos + self.rot.mul_vec(p.scale(self.scale))
    }

    /// Compose a parent transform with a child: apply `child` first, then `self`.
    /// Rotations are normalized through quaternions to limit hierarchy drift.
    /// Both inputs must be rigid transforms with finite, positive uniform scales;
    /// validating authored values and representable products belongs to the caller.
    pub fn compose(self, child: Transform) -> Transform {
        Transform {
            rot: (Quat::from_mat3(self.rot) * Quat::from_mat3(child.rot)).to_mat3(),
            pos: self.to_world(child.pos),
            scale: self.scale * child.scale,
        }
    }

    /// A delta about a world-space rest pivot, followed by a world-space translation.
    /// Maps `p` to `pivot + translation + rotation * (p - pivot) * scale`.
    /// Use `delta.compose(rest_transform)` to pose an object without changing its rest
    /// geometry. The caller must provide a finite, positive uniform `scale`.
    pub fn around_pivot(pivot: Vec3, rotation: Quat, scale: f32, translation: Vec3) -> Transform {
        let rot = rotation.to_mat3();
        Transform { pos: pivot + translation - rot.mul_vec(pivot.scale(scale)), rot, scale }
    }

    /// Map a world-space point into this object's local space.
    pub fn to_local(&self, p: Vec3) -> Vec3 {
        // Guard a zero scale (matches the zero-length guards in `normalize`) so a degenerate
        // transform yields a finite point rather than Inf/NaN poisoning the whole field.
        let inv = if self.scale.abs() > 1e-12 { 1.0 / self.scale } else { 0.0 };
        self.rot.transpose().mul_vec(p - self.pos).scale(inv)
    }
}

/// A unit quaternion `(x, y, z, w)` for smooth rotation interpolation (animation). Stored
/// scalar-last. `slerp` is the spherical interpolation animation needs; `to_mat3` hands a
/// rotation to a `Transform` so the rest of the engine never sees quaternions.
#[derive(Clone, Copy, Debug)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub const IDENTITY: Quat = Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    /// A rotation of `angle` radians about a unit `axis`.
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Quat {
        let half = angle * 0.5;
        let s = half.sin();
        let a = axis.normalize();
        Quat { x: a.x * s, y: a.y * s, z: a.z * s, w: half.cos() }
    }

    /// Convert an orthonormal rotation matrix to a unit quaternion. Selecting the
    /// largest diagonal component keeps half-turns stable when the trace is negative.
    pub fn from_mat3(rot: Mat3) -> Quat {
        let [a, b, c] = rot.cols;
        let trace = a.x + b.y + c.z;
        let q = if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            Quat { x: (b.z - c.y) / s, y: (c.x - a.z) / s, z: (a.y - b.x) / s, w: 0.25 * s }
        } else if a.x > b.y && a.x > c.z {
            let s = (1.0 + a.x - b.y - c.z).sqrt() * 2.0;
            Quat { x: 0.25 * s, y: (b.x + a.y) / s, z: (c.x + a.z) / s, w: (b.z - c.y) / s }
        } else if b.y > c.z {
            let s = (1.0 + b.y - a.x - c.z).sqrt() * 2.0;
            Quat { x: (b.x + a.y) / s, y: 0.25 * s, z: (c.y + b.z) / s, w: (c.x - a.z) / s }
        } else {
            let s = (1.0 + c.z - a.x - b.y).sqrt() * 2.0;
            Quat { x: (c.x + a.z) / s, y: (c.y + b.z) / s, z: 0.25 * s, w: (a.y - b.x) / s }
        };
        q.normalize()
    }

    /// Euler rotation in radians, in the same Z-then-Y-then-X order as `Mat3`.
    pub fn from_euler(x: f32, y: f32, z: f32) -> Quat {
        Quat::from_mat3(Mat3::from_euler(x, y, z))
    }

    pub fn dot(self, o: Quat) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z + self.w * o.w
    }

    pub fn normalize(self) -> Quat {
        let n = self.dot(self).sqrt();
        if n > 1e-12 {
            Quat { x: self.x / n, y: self.y / n, z: self.z / n, w: self.w / n }
        } else {
            Quat::IDENTITY
        }
    }

    /// Spherical linear interpolation toward `o` by `t` (shortest path).
    pub fn slerp(self, o: Quat, t: f32) -> Quat {
        let start = self.normalize();
        let mut end = o.normalize();
        let mut cos = start.dot(end);
        if cos < 0.0 {
            cos = -cos;
            end = Quat { x: -end.x, y: -end.y, z: -end.z, w: -end.w };
        }
        if cos > 0.9995 {
            // Nearly parallel — fall back to normalized lerp.
            return Quat {
                x: start.x + (end.x - start.x) * t,
                y: start.y + (end.y - start.y) * t,
                z: start.z + (end.z - start.z) * t,
                w: start.w + (end.w - start.w) * t,
            }
            .normalize();
        }
        let theta = cos.clamp(-1.0, 1.0).acos();
        let sin = theta.sin();
        let a = ((1.0 - t) * theta).sin() / sin;
        let b = (t * theta).sin() / sin;
        Quat {
            x: start.x * a + end.x * b,
            y: start.y * a + end.y * b,
            z: start.z * a + end.z * b,
            w: start.w * a + end.w * b,
        }
        .normalize()
    }

    /// The equivalent 3×3 rotation matrix.
    pub fn to_mat3(self) -> Mat3 {
        let q = self.normalize();
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);
        Mat3::from_cols(
            Vec3::new(1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y + z * w), 2.0 * (x * z - y * w)),
            Vec3::new(2.0 * (x * y - z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z + x * w)),
            Vec3::new(2.0 * (x * z + y * w), 2.0 * (y * z - x * w), 1.0 - 2.0 * (x * x + y * y)),
        )
    }
}

impl std::ops::Mul for Quat {
    type Output = Quat;

    /// Normalized Hamilton product: apply `other` first, then `self`.
    fn mul(self, other: Quat) -> Quat {
        let a = self.normalize();
        let b = other.normalize();
        Quat {
            x: a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y,
            y: a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x,
            z: a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w,
            w: a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
        }
        .normalize()
    }
}
