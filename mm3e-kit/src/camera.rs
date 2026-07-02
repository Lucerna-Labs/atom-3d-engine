//! Pinhole camera + primary-ray generation. Building the look-at basis and projecting a
//! pixel onto a world-space ray direction is the `project` root atom (dot products against
//! an orthonormal basis); the perspective spread is `scale` (by `tan(fov/2)`). Mechanism
//! only — the orchestrator decides where the camera is and where it looks.

use crate::march::Ray;
use crate::vec::Vec3;

/// An orthonormal camera frame plus the field-of-view spread.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub eye: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    /// `tan(fov_y / 2)` — the half-height of the image plane at unit distance.
    pub fov_scale: f32,
}

impl Camera {
    /// A camera at `eye` looking at `target`, with vertical field of view `fov_y` (radians).
    pub fn look_at(eye: Vec3, target: Vec3, up_hint: Vec3, fov_y: f32) -> Camera {
        let forward = (target - eye).normalize();
        let right = forward.cross(up_hint).normalize();
        let up = right.cross(forward);
        Camera { eye, forward, right, up, fov_scale: (fov_y * 0.5).tan() }
    }

    /// The primary ray through image-plane sample `(sx, sy)` (pixel coordinates, fractional
    /// for sub-pixel anti-aliasing) on a `w × h` image. `(0,0)` is the top-left corner.
    pub fn ray(&self, sx: f32, sy: f32, w: u32, h: u32) -> Ray {
        let aspect = w as f32 / h as f32;
        let ndc_x = (2.0 * sx / w as f32 - 1.0) * aspect * self.fov_scale;
        let ndc_y = (1.0 - 2.0 * sy / h as f32) * self.fov_scale;
        let dir = (self.forward + self.right.scale(ndc_x) + self.up.scale(ndc_y)).normalize();
        Ray { origin: self.eye, dir }
    }
}
