//! Pinhole/thin-lens camera + primary-ray generation. Building the look-at basis and projecting a
//! pixel onto a world-space ray direction is the `project` root atom (dot products against
//! an orthonormal basis); the perspective spread is `scale` (by `tan(fov/2)`). Mechanism
//! only — the orchestrator decides where the camera is and where it looks.

use crate::march::Ray;
use crate::vec::Vec3;

/// A circular thin lens in world units. The camera eye is the lens center; the focus plane is
/// perpendicular to `Camera::forward` at `focus_distance` from that center. A zero radius
/// gives the original pinhole camera, exactly. No exposure compensation or optical aberrations
/// are implied by this geometric lens model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lens {
    pub aperture_radius: f32,
    pub focus_distance: f32,
}

impl Default for Lens {
    fn default() -> Self {
        Self { aperture_radius: 0.0, focus_distance: 1.0 }
    }
}

impl Lens {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.aperture_radius.is_finite() || self.aperture_radius < 0.0 {
            return Err("aperture_radius must be finite and nonnegative");
        }
        if !self.focus_distance.is_finite() || self.focus_distance <= 0.0 {
            return Err("focus_distance must be finite and positive");
        }
        Ok(())
    }
}

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

    /// Generate a thin-lens ray through a fractional image sample. `aperture_sample` is a
    /// uniform sample in `[0,1]^2`, mapped concentrically onto the circular lens. The caller
    /// controls sampling and must validate the lens and camera. Rays for a given image sample
    /// meet at the same point on the camera-forward focus plane, including off-axis samples.
    pub fn ray_with_lens(
        &self,
        image_sample: [f32; 2],
        image_size: [u32; 2],
        lens: &Lens,
        aperture_sample: [f32; 2],
    ) -> Ray {
        let pinhole = self.ray(image_sample[0], image_sample[1], image_size[0], image_size[1]);
        if lens.aperture_radius == 0.0 {
            return pinhole;
        }
        let [dx, dy] = concentric_disk(aperture_sample);
        let origin = self.eye + self.right.scale(dx * lens.aperture_radius) + self.up.scale(dy * lens.aperture_radius);
        let focus = self.eye + pinhole.dir.scale(lens.focus_distance / pinhole.dir.dot(self.forward));
        Ray { origin, dir: (focus - origin).normalize() }
    }
}

/// Equal-area concentric square-to-disk map; the origin has a separate branch to avoid 0/0.
fn concentric_disk(sample: [f32; 2]) -> [f32; 2] {
    let x = 2.0 * sample[0] - 1.0;
    let y = 2.0 * sample[1] - 1.0;
    if x == 0.0 && y == 0.0 {
        return [0.0, 0.0];
    }
    let (radius, theta) = if x.abs() > y.abs() {
        (x, std::f32::consts::FRAC_PI_4 * (y / x))
    } else {
        (y, std::f32::consts::FRAC_PI_2 - std::f32::consts::FRAC_PI_4 * (x / y))
    };
    [radius * theta.cos(), radius * theta.sin()]
}
