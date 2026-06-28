//! Post-processing: resolve a linear-HDR scene-color buffer into a displayable framebuffer.
//!
//! This is the payoff of keeping a float HDR target instead of tone-mapping per pixel: full-frame
//! operators become possible. Here we implement physically-flavored **bloom** (bright-pass +
//! separable Gaussian blur, added back), then **exposure**, **ACES** tone-mapping, and **gamma**.
//! Every operator is one of the atoms over a 2-D buffer — bloom's blur is `combine` (a weighted
//! sum under a Gaussian kernel) over `scan` (the pixel grid); the bright-pass is `compare`
//! (luminance vs threshold). Policy lives here; the kit supplies only the pure math.

use mm3e_kit::{color::Rgba, framebuffer::Framebuffer, shade, vec::Vec3};

/// Post-processing settings.
#[derive(Clone, Copy, Debug)]
pub struct Post {
    pub exposure: f32,
    pub bloom: bool,
    /// Luminance above which a pixel contributes to bloom.
    pub bloom_threshold: f32,
    /// How strongly the blurred bright-pass is added back.
    pub bloom_intensity: f32,
    /// Gaussian blur radius in pixels (per separable pass).
    pub bloom_radius: u32,
}

impl Default for Post {
    fn default() -> Self {
        Self { exposure: 1.0, bloom: true, bloom_threshold: 1.0, bloom_intensity: 0.06, bloom_radius: 12 }
    }
}

/// Resolve a linear-HDR `hdr` buffer (`w·h`, row-major) into a tone-mapped framebuffer.
pub fn resolve(hdr: &[Vec3], w: u32, h: u32, post: &Post) -> Framebuffer {
    let n = (w * h) as usize;
    let mut color = hdr.to_vec();

    if post.bloom && post.bloom_intensity > 0.0 {
        let bloom = bloom_pass(hdr, w, h, post.bloom_threshold, post.bloom_radius);
        for i in 0..n {
            color[i] = color[i] + bloom[i].scale(post.bloom_intensity);
        }
    }

    // Tone-map + gamma every pixel after all HDR operators have run.
    let clear = Rgba::from_vec3(shade::gamma(shade::aces(Vec3::ZERO)));
    let mut fb = Framebuffer::new(w, h, clear);
    for y in 0..h {
        for x in 0..w {
            let c = color[(y * w + x) as usize].scale(post.exposure);
            fb.put(x, y, Rgba::from_vec3(shade::gamma(shade::aces(c))));
        }
    }
    fb
}

/// Bright-pass (luminance over `threshold`) blurred by a separable Gaussian of radius `radius`.
fn bloom_pass(hdr: &[Vec3], w: u32, h: u32, threshold: f32, radius: u32) -> Vec<Vec3> {
    let n = (w * h) as usize;
    let mut bright = vec![Vec3::ZERO; n];
    for i in 0..n {
        let l = shade::luminance(hdr[i]);
        if l > threshold {
            // Keep only the energy above the knee, preserving hue.
            bright[i] = hdr[i].scale((l - threshold) / l.max(1e-4));
        }
    }
    let kernel = gaussian_kernel(radius);
    let horizontal = blur_axis(&bright, w, h, &kernel, true);
    blur_axis(&horizontal, w, h, &kernel, false)
}

/// A normalized 1-D Gaussian kernel of the given radius (σ = radius/2).
fn gaussian_kernel(radius: u32) -> Vec<f32> {
    let r = radius.max(1) as i32;
    let sigma = (r as f32 / 2.0).max(1e-3);
    let mut k = Vec::with_capacity((2 * r + 1) as usize);
    let mut sum = 0.0;
    for i in -r..=r {
        let v = (-(i * i) as f32 / (2.0 * sigma * sigma)).exp();
        k.push(v);
        sum += v;
    }
    for v in &mut k {
        *v /= sum;
    }
    k
}

/// One separable Gaussian pass along x (`horizontal`) or y, clamping at the borders.
fn blur_axis(src: &[Vec3], w: u32, h: u32, kernel: &[f32], horizontal: bool) -> Vec<Vec3> {
    let r = (kernel.len() / 2) as i32;
    let (wi, hi) = (w as i32, h as i32);
    let mut out = vec![Vec3::ZERO; src.len()];
    for y in 0..hi {
        for x in 0..wi {
            let mut acc = Vec3::ZERO;
            for (ki, &kw) in kernel.iter().enumerate() {
                let off = ki as i32 - r;
                let (sx, sy) = if horizontal {
                    ((x + off).clamp(0, wi - 1), y)
                } else {
                    (x, (y + off).clamp(0, hi - 1))
                };
                acc = acc + src[(sy * wi + sx) as usize].scale(kw);
            }
            out[(y * wi + x) as usize] = acc;
        }
    }
    out
}
