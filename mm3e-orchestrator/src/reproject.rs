//! Engine-native frame generation — **camera reprojection** (Level 1).
//!
//! A real frame captures colour + depth (a G-buffer); the cheap "fake" frames between two real
//! frames warp those pixels into the new camera using the depth the engine already computes,
//! instead of re-marching the whole field. The simulation stays honest — physics, input, and game
//! logic run every tick; only the expensive raymarched *image* is generated less often.
//!
//! Method: forward-warp each old pixel to its world position (`eye + dir·depth`), project it into
//! the new camera, keep the nearest (a z-buffer), then fill disocclusion holes along each row.
//! Newly revealed surfaces are slightly wrong until the next real frame corrects them — the
//! standard reprojection trade. This uses only data the engine owns (depth + camera), so it is
//! deterministic and testable, not a black box.

use mm3e_kit::camera::Camera;
use mm3e_kit::framebuffer::Framebuffer;
use mm3e_kit::vec::Vec3;
use mm3e_kit::Rgba;

/// A rendered frame plus the depth, object tags, and camera needed to reproject it.
pub struct GFrame {
    pub width: u32,
    pub height: u32,
    /// Display-space RGBA8, row-major.
    pub color: Vec<[u8; 4]>,
    /// Primary-ray hit distance per pixel; `f32::INFINITY` for sky.
    pub depth: Vec<f32>,
    /// Index of the moving object each pixel belongs to (`-1` = static geometry / sky). Level 2
    /// reprojection shifts these pixels by their object's world-space motion, not just the camera's.
    pub obj: Vec<i32>,
    pub camera: Camera,
}

impl GFrame {
    /// Wrap a colour buffer as a [`Framebuffer`] (e.g. to save a reprojected frame to BMP).
    pub fn color_to_framebuffer(width: u32, height: u32, color: &[[u8; 4]]) -> Framebuffer {
        let mut fb = Framebuffer::new(width, height, Rgba::new(0.0, 0.0, 0.0, 1.0));
        for y in 0..height {
            for x in 0..width {
                let c = color[(y * width + x) as usize];
                fb.put(x, y, Rgba::new(c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0, 1.0));
            }
        }
        fb
    }
}

/// Project a world point into `camera`, returning `(screen_x, screen_y, forward_depth)` or `None`
/// if it is behind the camera. The inverse of the camera's primary-ray generation.
fn project(camera: &Camera, world: Vec3, w: u32, h: u32) -> Option<(f32, f32, f32)> {
    let v = world - camera.eye;
    let fz = v.dot(camera.forward);
    if fz <= 1e-4 {
        return None;
    }
    let aspect = w as f32 / h as f32;
    let ndc_x = v.dot(camera.right) / fz; // = (2·sx/w − 1)·aspect·fov_scale
    let ndc_y = v.dot(camera.up) / fz; // = (1 − 2·sy/h)·fov_scale
    let sx = (ndc_x / (aspect * camera.fov_scale) + 1.0) * 0.5 * w as f32;
    let sy = (1.0 - ndc_y / camera.fov_scale) * 0.5 * h as f32;
    Some((sx, sy, fz))
}

/// Reproject `prev` into `new_camera`, returning a fresh display RGBA8 buffer. `mover_deltas[k]` is
/// the world-space displacement of moving object `k` since `prev` was rendered (pass `&[]` for
/// camera-only Level 1 reprojection); pixels tagged with object `k` are shifted by that delta.
/// Forward-warp `prev` into `new_camera` (with per-object motion), returning the warped colour
/// buffer and a `filled` mask (false where reprojection left a disocclusion hole). Shared by the
/// cheap `reproject` (row hole-fill) and the orchestrator's Level-3 hybrid (rerender the holes).
pub fn warp(prev: &GFrame, new_camera: &Camera, mover_deltas: &[Vec3]) -> (Vec<[u8; 4]>, Vec<bool>) {
    let (w, h) = (prev.width, prev.height);
    let n = (w * h) as usize;
    let mut out = vec![[0u8; 4]; n];
    let mut filled = vec![false; n];
    let mut zbuf = vec![f32::INFINITY; n];

    for y in 0..h {
        for x in 0..w {
            let i = (y * w + x) as usize;
            let dir = prev.camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h).dir;
            // Sky (infinite depth) is reprojected as a far point so it rotates with the camera.
            let d = if prev.depth[i].is_finite() { prev.depth[i] } else { 1.0e6 };
            let mut world = prev.camera.eye + dir.scale(d);
            // Level 2: if this pixel belongs to a moving object, advance it by the object's motion.
            let oi = prev.obj[i];
            if oi >= 0 {
                if let Some(delta) = mover_deltas.get(oi as usize) {
                    world = world + *delta;
                }
            }
            if let Some((sx, sy, fz)) = project(new_camera, world, w, h) {
                // `sx = x + 0.5` is the centre of pixel x, so the target pixel index is floor(sx).
                let (nx, ny) = (sx.floor() as i32, sy.floor() as i32);
                if nx >= 0 && ny >= 0 && (nx as u32) < w && (ny as u32) < h {
                    let j = (ny as u32 * w + nx as u32) as usize;
                    if fz < zbuf[j] {
                        zbuf[j] = fz;
                        out[j] = prev.color[i];
                        filled[j] = true;
                    }
                }
            }
        }
    }
    (out, filled)
}

/// Reproject `prev` into `new_camera` with cheap row hole-fill (Levels 1 + 2). For correct
/// disocclusion fills, see the orchestrator's `reproject_hybrid` (Level 3), which rerenders holes.
pub fn reproject(prev: &GFrame, new_camera: &Camera, mover_deltas: &[Vec3]) -> Vec<[u8; 4]> {
    let (w, h) = (prev.width, prev.height);
    let (mut out, mut filled) = warp(prev, new_camera, mover_deltas);

    // Hole fill: each unfilled pixel copies the last filled pixel on its row (cheap, removes the
    // thin gaps reprojection leaves). Pixels with no filled neighbour on the row stay background.
    for y in 0..h {
        let mut last: Option<[u8; 4]> = None;
        for x in 0..w {
            let i = (y * w + x) as usize;
            if filled[i] {
                last = Some(out[i]);
            } else if let Some(c) = last {
                out[i] = c;
                filled[i] = true;
            }
        }
        // second sweep right→left to fill the leading edge of each row
        let mut last: Option<[u8; 4]> = None;
        for x in (0..w).rev() {
            let i = (y * w + x) as usize;
            if filled[i] {
                last = Some(out[i]);
            } else if let Some(c) = last {
                out[i] = c;
            }
        }
    }
    out
}
