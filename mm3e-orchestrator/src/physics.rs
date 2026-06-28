//! SDF-native physics — rigid sphere bodies colliding against the world field.
//!
//! The field *is* the collision oracle: `field(p).dist` gives the signed distance (so a body of
//! radius `r` is penetrating whenever `dist < r`), and the field gradient gives the contact
//! normal. The GJK/EPA/convex-decomposition machinery a mesh engine needs to recover those two
//! quantities is closed-form here — collision falls out of the same `Fn(Vec3) -> Field` the
//! renderer marches. This is the doctrine's payoff applied to gameplay.

use mm3e_kit::march::Marcher;
use mm3e_kit::sdf::Field;
use mm3e_kit::vec::Vec3;

/// A spherical rigid body.
#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub pos: Vec3,
    pub vel: Vec3,
    pub radius: f32,
    /// True after the last step if the body is resting on a roughly-upward surface (for jumping).
    pub grounded: bool,
}

impl Body {
    pub fn new(pos: Vec3, radius: f32) -> Body {
        Body { pos, vel: Vec3::ZERO, radius, grounded: false }
    }
    /// Set the horizontal (XZ) velocity from a desired direction and speed (arcade control).
    pub fn drive(&mut self, dir: Vec3, speed: f32) {
        self.vel.x = dir.x * speed;
        self.vel.z = dir.z * speed;
    }
    /// Launch upward if currently grounded.
    pub fn jump(&mut self, speed: f32) {
        if self.grounded {
            self.vel.y = speed;
        }
    }
}

/// A world of sphere bodies stepped against a static SDF field (and each other).
pub struct PhysicsWorld {
    pub bodies: Vec<Body>,
    pub gravity: Vec3,
    pub restitution: f32,
    pub friction: f32,
    marcher: Marcher,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        PhysicsWorld {
            bodies: Vec::new(),
            gravity: Vec3::new(0.0, -14.0, 0.0),
            restitution: 0.3,
            friction: 0.12,
            marcher: Marcher::default(),
        }
    }
}

impl PhysicsWorld {
    pub fn new() -> PhysicsWorld {
        PhysicsWorld::default()
    }
    pub fn add(&mut self, b: Body) -> usize {
        self.bodies.push(b);
        self.bodies.len() - 1
    }

    /// Advance the simulation by `dt` seconds against the static `field`. Uses fixed sub-steps for
    /// stability, resolves field collisions along the gradient normal, then sphere–sphere contacts.
    pub fn step(&mut self, dt: f32, field: &dyn Fn(Vec3) -> Field) {
        let sub = 4;
        let h = dt / sub as f32;
        for _ in 0..sub {
            for b in self.bodies.iter_mut() {
                b.vel = b.vel + self.gravity.scale(h);
                b.pos = b.pos + b.vel.scale(h);
                b.grounded = false;
                // A few resolution iterations push the body out of any penetration.
                for _ in 0..3 {
                    let d = field(b.pos).dist;
                    if d >= b.radius {
                        break;
                    }
                    let n = self.marcher.normal(field, b.pos);
                    b.pos = b.pos + n.scale(b.radius - d);
                    let vn = b.vel.dot(n);
                    if vn < 0.0 {
                        let normal_v = n.scale(vn);
                        let tangent_v = b.vel - normal_v;
                        // Friction damps the tangential slide; restitution bounces the normal part.
                        b.vel = tangent_v.scale(1.0 - self.friction) - normal_v.scale(self.restitution);
                    }
                    if n.y > 0.5 {
                        b.grounded = true;
                    }
                }
            }
            self.resolve_pairs();
        }
    }

    fn resolve_pairs(&mut self) {
        let n = self.bodies.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let delta = self.bodies[j].pos - self.bodies[i].pos;
                let dist = delta.length();
                let min = self.bodies[i].radius + self.bodies[j].radius;
                if dist < min && dist > 1e-6 {
                    let nrm = delta.scale(1.0 / dist);
                    let push = (min - dist) * 0.5;
                    self.bodies[i].pos = self.bodies[i].pos - nrm.scale(push);
                    self.bodies[j].pos = self.bodies[j].pos + nrm.scale(push);
                    let vi = self.bodies[i].vel.dot(nrm);
                    let vj = self.bodies[j].vel.dot(nrm);
                    if vi - vj > 0.0 {
                        let amount = (vi - vj) * (0.5 + 0.5 * self.restitution);
                        self.bodies[i].vel = self.bodies[i].vel - nrm.scale(amount);
                        self.bodies[j].vel = self.bodies[j].vel + nrm.scale(amount);
                    }
                }
            }
        }
    }
}
