//! A minimal particle system — short-lived sparks/debris that integrate under gravity and drag.
//! Each particle is just a small sphere, so it renders through the very same dynamic-sphere union
//! the player and physics balls use: particles are not a special case, only more primitives.
//! Emission directions are a deterministic Fibonacci sphere (no RNG dependency), so a burst is
//! reproducible from its seed.

use mm3e_kit::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub pos: Vec3,
    pub vel: Vec3,
    pub life: f32,
    pub size: f32,
    pub color: Vec3,
}

/// A pool of live particles.
pub struct Particles {
    pub items: Vec<Particle>,
    pub gravity: Vec3,
    pub drag: f32,
}

impl Default for Particles {
    fn default() -> Self {
        Particles { items: Vec::new(), gravity: Vec3::new(0.0, -9.0, 0.0), drag: 1.4 }
    }
}

impl Particles {
    pub fn new() -> Particles {
        Particles::default()
    }

    /// Emit `count` particles from `origin`, scattered over a sphere at ~`speed`, tinted `color`.
    /// `seed` varies the speed jitter so successive bursts differ without an RNG.
    pub fn burst(&mut self, origin: Vec3, count: u32, speed: f32, color: Vec3, seed: u32) {
        let golden = 2.399_963_2_f32; // golden angle
        for i in 0..count {
            let t = (i as f32 + 0.5) / count as f32;
            let y = 1.0 - 2.0 * t;
            let r = (1.0 - y * y).max(0.0).sqrt();
            let a = i as f32 * golden;
            let dir = Vec3::new(r * a.cos(), y.abs().max(0.2), r * a.sin()).normalize();
            let jitter = 0.6 + 0.4 * (((i.wrapping_mul(2654435761).wrapping_add(seed)) >> 8) & 0xff) as f32 / 255.0;
            self.items.push(Particle {
                pos: origin,
                vel: dir.scale(speed * jitter),
                life: 0.6 + 0.4 * jitter,
                size: 0.07,
                color,
            });
        }
    }

    /// Integrate every particle and drop the dead ones.
    pub fn update(&mut self, dt: f32) {
        let g = self.gravity;
        let drag = self.drag;
        for p in self.items.iter_mut() {
            p.vel = p.vel + g.scale(dt);
            p.vel = p.vel.scale((1.0 - drag * dt).max(0.0));
            p.pos = p.pos + p.vel.scale(dt);
            p.life -= dt;
        }
        self.items.retain(|p| p.life > 0.0);
    }

    pub fn alive(&self) -> &[Particle] {
        &self.items
    }
}
