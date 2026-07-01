//! Rigid bodies with rotation — boxes, capsules, and spheres tumbling against the static SDF world.
//!
//! This is the next rung above the sphere-only [`crate::physics`] module. The field is STILL the
//! collision oracle (`field(p).dist` = how far a sample is from the world surface, the gradient =
//! the contact normal), but each body now carries an **orientation** (`Quat`) and an **inertia
//! tensor**, and contacts apply impulses at a lever arm — so shapes spin, topple, and settle flat
//! instead of just sliding as points. The SDF doctrine still pays off: collision against arbitrary
//! world geometry is the same closed-form `field` the renderer marches — no mesh BVH, no GJK/EPA.
//!
//! Simplification (game-grade, like most real-time engines): the gyroscopic `ω × Iω` torque is
//! omitted, so a free asymmetric body won't precess; angular velocity only changes at contacts.

use mm3e_kit::march::Marcher;
use mm3e_kit::sdf::{self, Field};
use mm3e_kit::vec::{Mat3, Quat, Vec3};

/// Hamilton product `a * b` (scalar-last) — compose two rotations. (`Quat` in the kit has no `mul`.)
fn qmul(a: Quat, b: Quat) -> Quat {
    Quat {
        w: a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
        x: a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y,
        y: a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x,
        z: a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w,
    }
}

/// A rigid body's shape. The local SDF renders it; the sample points drive collision; the inertia
/// tensor drives rotation.
#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Sphere { r: f32 },
    /// Axis-aligned-in-local box with the given half-extents.
    Cube { half: Vec3 },
    /// Capsule swept along the local Y axis: radius `r`, half-height `half_h`.
    Capsule { r: f32, half_h: f32 },
}

impl Shape {
    /// Local-space signed distance — used to render the body into the scene union.
    pub fn sdf(&self, p: Vec3) -> f32 {
        match *self {
            Shape::Sphere { r } => sdf::sphere(p, r),
            Shape::Cube { half } => sdf::boxed(p, half),
            Shape::Capsule { r, half_h } => {
                sdf::capsule(p, Vec3::new(0.0, -half_h, 0.0), Vec3::new(0.0, half_h, 0.0), r)
            }
        }
    }

    /// Bounding radius for broad-phase pair tests.
    pub fn bound(&self) -> f32 {
        match *self {
            Shape::Sphere { r } => r,
            Shape::Cube { half } => half.length(),
            Shape::Capsule { r, half_h } => r + half_h,
        }
    }

    /// Contact sample points in LOCAL space, each paired with the sphere radius swept at that point
    /// (box corners are exact surface points, so radius 0; sphere/capsule are swept spheres).
    fn samples(&self) -> Vec<(Vec3, f32)> {
        match *self {
            Shape::Sphere { r } => vec![(Vec3::ZERO, r)],
            Shape::Capsule { r, half_h } => {
                vec![(Vec3::new(0.0, -half_h, 0.0), r), (Vec3::new(0.0, half_h, 0.0), r)]
            }
            Shape::Cube { half } => {
                let mut v = Vec::with_capacity(8);
                for &sx in &[-1.0f32, 1.0] {
                    for &sy in &[-1.0f32, 1.0] {
                        for &sz in &[-1.0f32, 1.0] {
                            v.push((Vec3::new(sx * half.x, sy * half.y, sz * half.z), 0.0));
                        }
                    }
                }
                v
            }
        }
    }

    /// Diagonal inertia tensor per unit mass, in the local principal frame.
    fn inertia_unit(&self) -> Vec3 {
        match *self {
            Shape::Sphere { r } => Vec3::splat(0.4 * r * r), // 2/5 r²
            Shape::Cube { half } => {
                let (x, y, z) = (2.0 * half.x, 2.0 * half.y, 2.0 * half.z);
                Vec3::new((y * y + z * z) / 12.0, (x * x + z * z) / 12.0, (x * x + y * y) / 12.0)
            }
            Shape::Capsule { r, half_h } => {
                // Solid-cylinder approximation of radius r, full height 2·half_h.
                let h = 2.0 * half_h;
                let ix = (3.0 * r * r + h * h) / 12.0;
                Vec3::new(ix, 0.5 * r * r, ix)
            }
        }
    }
}

/// A rigid body: linear + angular state, a shape, and precomputed inverse mass/inertia.
#[derive(Clone, Copy, Debug)]
pub struct RigidBody {
    pub pos: Vec3,
    pub vel: Vec3,
    pub orient: Quat,
    /// World-space angular velocity (rad/s).
    pub ang_vel: Vec3,
    pub shape: Shape,
    pub mat: u32,
    /// 0 ⇒ immovable (infinite mass).
    pub inv_mass: f32,
    /// Local-space diagonal of the inverse inertia tensor.
    pub inv_inertia: Vec3,
    /// True after a step if resting on a roughly-upward surface.
    pub grounded: bool,
}

impl RigidBody {
    pub fn new(pos: Vec3, shape: Shape, mass: f32) -> RigidBody {
        let (inv_mass, inv_inertia) = if mass > 0.0 {
            let iu = shape.inertia_unit();
            (
                1.0 / mass,
                Vec3::new(
                    1.0 / (mass * iu.x.max(1e-6)),
                    1.0 / (mass * iu.y.max(1e-6)),
                    1.0 / (mass * iu.z.max(1e-6)),
                ),
            )
        } else {
            (0.0, Vec3::ZERO)
        };
        RigidBody {
            pos,
            vel: Vec3::ZERO,
            orient: Quat::IDENTITY,
            ang_vel: Vec3::ZERO,
            shape,
            mat: 1,
            inv_mass,
            inv_inertia,
            grounded: false,
        }
    }
    pub fn with_orient(mut self, q: Quat) -> Self {
        self.orient = q.normalize();
        self
    }
    pub fn with_vel(mut self, v: Vec3) -> Self {
        self.vel = v;
        self
    }
    pub fn with_spin(mut self, w: Vec3) -> Self {
        self.ang_vel = w;
        self
    }
    pub fn with_mat(mut self, mat: u32) -> Self {
        self.mat = mat;
        self
    }

    fn rot(&self) -> Mat3 {
        self.orient.to_mat3()
    }

    /// World-space signed distance of this body — for unioning bodies into the render field.
    pub fn sdf(&self, world_p: Vec3) -> f32 {
        let local = self.rot().transpose().mul_vec(world_p - self.pos);
        self.shape.sdf(local)
    }

    /// Apply the world-space inverse inertia tensor to an angular impulse / momentum `l`:
    /// `I⁻¹_world · l = R · (I⁻¹_local ⊙ (Rᵀ·l))`.
    fn apply_inv_inertia(&self, l: Vec3) -> Vec3 {
        let r = self.rot();
        let local = r.transpose().mul_vec(l);
        let scaled = Vec3::new(
            local.x * self.inv_inertia.x,
            local.y * self.inv_inertia.y,
            local.z * self.inv_inertia.z,
        );
        r.mul_vec(scaled)
    }
}

/// A world of rigid bodies stepped against a static SDF field (and each other, broad-phase).
pub struct RigidWorld {
    pub bodies: Vec<RigidBody>,
    pub gravity: Vec3,
    pub restitution: f32,
    pub friction: f32,
    marcher: Marcher,
}

impl Default for RigidWorld {
    fn default() -> Self {
        RigidWorld {
            bodies: Vec::new(),
            gravity: Vec3::new(0.0, -14.0, 0.0),
            restitution: 0.25,
            friction: 0.55,
            marcher: Marcher::default(),
        }
    }
}

impl RigidWorld {
    pub fn new() -> RigidWorld {
        RigidWorld::default()
    }
    pub fn add(&mut self, b: RigidBody) -> usize {
        self.bodies.push(b);
        self.bodies.len() - 1
    }

    /// Advance by `dt` against the static `field`. Fixed sub-steps for stability: integrate, then a
    /// few Gauss-Seidel iterations resolve every penetrating sample with a contact impulse (linear +
    /// angular + Coulomb friction), then a broad-phase sphere pass separates bodies.
    pub fn step(&mut self, dt: f32, field: &dyn Fn(Vec3) -> Field) {
        let sub = 4;
        let h = dt / sub as f32;
        for _ in 0..sub {
            for b in self.bodies.iter_mut() {
                if b.inv_mass == 0.0 {
                    continue;
                }
                b.vel = b.vel + self.gravity.scale(h);
                b.pos = b.pos + b.vel.scale(h);
                // Integrate orientation: compose the rotation ω·h about ω̂ onto the current pose.
                let speed = b.ang_vel.length();
                if speed > 1e-9 {
                    let delta = Quat::from_axis_angle(b.ang_vel, speed * h);
                    b.orient = qmul(delta, b.orient).normalize();
                }
                b.grounded = false;
                resolve_world(b, field, &self.marcher, self.restitution, self.friction);
                // Safety clamps so a bad contact can never explode the sim.
                let vmax = 80.0;
                if b.vel.length() > vmax {
                    b.vel = b.vel.normalize().scale(vmax);
                }
                if b.ang_vel.length() > vmax {
                    b.ang_vel = b.ang_vel.normalize().scale(vmax);
                }
            }
            self.resolve_pairs();
        }
    }

    /// Broad-phase: keep bounding spheres from overlapping (positions + normal velocity only — no
    /// rotational transfer here; the field contacts above carry the angular physics).
    fn resolve_pairs(&mut self) {
        let n = self.bodies.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let (im, jm) = (self.bodies[i].inv_mass, self.bodies[j].inv_mass);
                if im == 0.0 && jm == 0.0 {
                    continue;
                }
                let delta = self.bodies[j].pos - self.bodies[i].pos;
                let dist = delta.length();
                let min = self.bodies[i].shape.bound() + self.bodies[j].shape.bound();
                if dist < min && dist > 1e-6 {
                    let nrm = delta.scale(1.0 / dist);
                    let total = im + jm;
                    let push = (min - dist) / total;
                    self.bodies[i].pos = self.bodies[i].pos - nrm.scale(push * im);
                    self.bodies[j].pos = self.bodies[j].pos + nrm.scale(push * jm);
                    let rel = (self.bodies[j].vel - self.bodies[i].vel).dot(nrm);
                    if rel < 0.0 {
                        let jimp = -(1.0 + self.restitution) * rel / total;
                        self.bodies[i].vel = self.bodies[i].vel - nrm.scale(jimp * im);
                        self.bodies[j].vel = self.bodies[j].vel + nrm.scale(jimp * jm);
                    }
                }
            }
        }
    }
}

/// Resolve a single body against the static world field with sequential contact impulses.
fn resolve_world(b: &mut RigidBody, field: &dyn Fn(Vec3) -> Field, marcher: &Marcher, e: f32, fric: f32) {
    let samples = b.shape.samples();
    for _ in 0..6 {
        let rot = b.rot();
        let mut any = false;
        for &(lp, sr) in samples.iter() {
            let wp = b.pos + rot.mul_vec(lp);
            let d = field(wp).dist;
            let pen = sr - d; // box corner: sr=0 ⇒ penetrating when d<0
            if pen <= 1e-5 {
                continue;
            }
            any = true;
            let n = marcher.normal(field, wp);
            let contact = wp - n.scale(sr);
            // Positional correction along the contact normal.
            b.pos = b.pos + n.scale(pen);
            if n.y > 0.5 {
                b.grounded = true;
            }
            let r = contact - b.pos;
            let v_contact = b.vel + b.ang_vel.cross(r);
            let vn = v_contact.dot(n);
            if vn >= 0.0 {
                continue; // separating — no impulse
            }
            // Normal impulse: jn = −(1+e)·vn / (m⁻¹ + [(I⁻¹(r×n))×r]·n)
            let rn = r.cross(n);
            let k_n = b.inv_mass + b.apply_inv_inertia(rn).cross(r).dot(n);
            let jn = -(1.0 + e) * vn / k_n.max(1e-6);
            b.vel = b.vel + n.scale(jn * b.inv_mass);
            b.ang_vel = b.ang_vel + b.apply_inv_inertia(rn.scale(jn));
            // Coulomb friction along the tangential slide, clamped to μ·|jn|.
            let tangent = v_contact - n.scale(vn);
            if tangent.length_sq() > 1e-10 {
                let t = tangent.normalize();
                let rt = r.cross(t);
                let k_t = b.inv_mass + b.apply_inv_inertia(rt).cross(r).dot(t);
                let max_f = fric * jn.abs();
                let jt = (-v_contact.dot(t) / k_t.max(1e-6)).clamp(-max_f, max_f);
                b.vel = b.vel + t.scale(jt * b.inv_mass);
                b.ang_vel = b.ang_vel + b.apply_inv_inertia(rt.scale(jt));
            }
        }
        if !any {
            break;
        }
    }
}

/// Union the static world field with every body — the render field a marcher can draw.
pub fn body_field(world: Field, bodies: &[RigidBody], p: Vec3) -> Field {
    let mut f = world;
    for b in bodies {
        f = sdf::union(f, Field::new(b.sdf(p), b.mat));
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ground(p: Vec3) -> Field {
        Field::new(p.y, 0) // infinite plane at y=0, solid below
    }

    #[test]
    fn tilted_box_settles_on_ground() {
        let mut w = RigidWorld::new();
        w.add(
            RigidBody::new(Vec3::new(0.0, 3.0, 0.0), Shape::Cube { half: Vec3::splat(0.5) }, 1.0)
                .with_orient(Quat::from_axis_angle(Vec3::new(1.0, 0.3, 0.2), 0.6)),
        );
        let field = |p: Vec3| ground(p);
        for _ in 0..900 {
            w.step(1.0 / 120.0, &field);
        }
        let b = w.bodies[0];
        assert!(b.pos.y > 0.3, "box sank through the ground: y={}", b.pos.y);
        assert!(b.pos.y < 1.3, "box never fell: y={}", b.pos.y);
        assert!(b.vel.length() < 0.3, "box never came to rest: |v|={}", b.vel.length());
        assert!(b.ang_vel.length() < 0.6, "box still tumbling: |w|={}", b.ang_vel.length());
        assert!(b.grounded, "box should be grounded at rest");
    }

    #[test]
    fn free_spin_is_conserved() {
        let mut w = RigidWorld::new();
        w.gravity = Vec3::ZERO;
        w.add(
            RigidBody::new(Vec3::new(0.0, 10.0, 0.0), Shape::Cube { half: Vec3::splat(0.4) }, 1.0)
                .with_spin(Vec3::new(0.0, 5.0, 0.0)),
        );
        let far = |_p: Vec3| Field::new(1000.0, 0); // no surface to touch
        let w0 = w.bodies[0].ang_vel.length();
        for _ in 0..480 {
            w.step(1.0 / 120.0, &far);
        }
        let b = w.bodies[0];
        assert!((b.ang_vel.length() - w0).abs() < 1e-3, "spin not conserved: {} -> {}", w0, b.ang_vel.length());
        // The orientation actually advanced (it isn't still the identity).
        assert!(b.orient.dot(Quat::IDENTITY).abs() < 0.999, "body never rotated");
    }

    #[test]
    fn sphere_rolls_downhill_and_stays_on_slope() {
        // A sphere on an inclined plane picks up downhill motion and stays resting on the surface.
        let mut w = RigidWorld::new();
        // Plane 0.3·x + y = 0: higher at −x, lower at +x, so downhill is +x.
        let nrm = Vec3::new(0.3, 1.0, 0.0).normalize();
        w.add(RigidBody::new(Vec3::new(0.0, 2.0, 0.0), Shape::Sphere { r: 0.5 }, 1.0));
        let field = move |p: Vec3| Field::new(p.dot(nrm), 0);
        for _ in 0..200 {
            w.step(1.0 / 120.0, &field);
        }
        let b = w.bodies[0];
        let above = b.pos.dot(nrm); // center's signed distance to the plane ≈ radius when resting
        assert!(b.pos.x > 0.3, "sphere should have rolled downhill (+x): x={}", b.pos.x);
        assert!((above - 0.5).abs() < 0.3, "sphere left the plane surface: above={}", above);
    }
}
