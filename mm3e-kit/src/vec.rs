//! Geometry mechanism: 3-D vectors, a 3×3 rotation matrix, and a rigid+uniform-scale
//! transform. `Vec3::dot` is the `project` root atom; `normalize` is `scale`; `Transform`
//! is the 3-D analog of MMPE's 2-D `Affine`. Pure mechanism — no rendering decisions.

#[derive(Clone, Copy, Debug, PartialEq)]
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

    /// Map a world-space point into this object's local space.
    pub fn to_local(&self, p: Vec3) -> Vec3 {
        self.rot.transpose().mul_vec(p - self.pos).scale(1.0 / self.scale)
    }
}
