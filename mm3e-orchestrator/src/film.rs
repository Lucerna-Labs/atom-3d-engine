//! Scene-linear film samples and geometry passes for compositing.
//!
//! Beauty and foreground use the canonical surface shader (including lighting, reflections,
//! environment illumination and fog). Foreground excludes primary rays that miss geometry;
//! it does not remove the environment from reflections or erase fog on visible surfaces.
//! Alpha is opaque-surface sample coverage, never inferred from radiance. This is not deep
//! compositing, transmission, a shadow catcher, or an integrated participating-medium matte.

use mm3e_kit::{camera::Camera, march::Hit, sdf::Field, shade, vec::Vec3};

pub use mm3e_kit::camera::Lens;

use crate::{available_render_threads, frame_len, shade_hit, Scene, TextureFootprint};

/// Row-major scene-linear radiance and raw geometric data, before any exposure or post pass.
/// All buffers have `width * height` elements.
#[derive(Clone, Debug, PartialEq)]
pub struct FilmFrame {
    pub width: u32,
    pub height: u32,
    /// Averaged hit radiance plus primary-ray sky misses, matching canonical pinhole beauty
    /// exactly when lens radius is zero.
    pub beauty: Vec<Vec3>,
    /// Hit radiance divided by the total sample count, already premultiplied by coverage.
    /// Composite over constant scene-linear background B as `foreground + (1-alpha)*B`.
    pub foreground: Vec<Vec3>,
    /// Fraction of the exact beauty sample rays that hit an opaque scene surface.
    pub alpha: Vec<f32>,
    /// Center-pixel pinhole primary-hit camera-forward Z in world units. Positive in front of
    /// the camera, not normalized or tone-mapped, and not distance along an off-axis ray.
    /// Misses are positive infinity. This geometric AOV is deliberately unfiltered: it does
    /// not average foreground and background depths or inherit aperture blur.
    pub depth: Vec<f32>,
    /// Raw world-space center-pixel pinhole surface normals, components in [-1,1]; zero on miss.
    pub normals: Vec<Vec3>,
    /// Center-pixel pinhole material index; `u32::MAX` on miss. No interpolation of categories.
    pub material_ids: Vec<u32>,
}

impl FilmFrame {
    fn empty(width: u32, height: u32) -> Self {
        let len = frame_len(width, height);
        Self {
            width,
            height,
            beauty: vec![Vec3::ZERO; len],
            foreground: vec![Vec3::ZERO; len],
            alpha: vec![0.0; len],
            depth: vec![f32::INFINITY; len],
            normals: vec![Vec3::ZERO; len],
            material_ids: vec![u32::MAX; len],
        }
    }
}

/// Render film using the available CPU worker budget. `scene.aa.max(1)^2` samples integrate
/// both the pixel footprint and aperture; increase AA for smoother defocus. A single sample
/// uses the center of the lens (no integrated defocus). Geometric AOVs use a separate center
/// pinhole sample; `scene.mode` does not affect this always-beauty rendering interface.
///
/// The lens must pass [`Lens::validate`]. As with the canonical renderer, the caller supplies
/// valid scene/camera data and reasonable image and sampling resource budgets.
pub fn render_film(scene: &Scene, camera: &Camera, lens: &Lens) -> FilmFrame {
    render_film_checked(scene, camera, lens)
        .expect("film appearance failed; use render_film_checked for a recoverable error")
}
pub fn render_film_checked(scene: &Scene, camera: &Camera, lens: &Lens) -> Result<FilmFrame, String> {
    render_film_with_threads_checked(scene, camera, lens, available_render_threads())
}

/// [`render_film`] with an explicit worker budget. Zero selects one worker. Output is
/// deterministic in worker count, including aperture samples (no thread-local RNG).
pub fn render_film_with_threads(scene: &Scene, camera: &Camera, lens: &Lens, threads: usize) -> FilmFrame {
    render_film_with_threads_checked(scene, camera, lens, threads)
        .expect("film appearance failed; use render_film_with_threads_checked for a recoverable error")
}
pub fn render_film_with_threads_checked(
    scene: &Scene,
    camera: &Camera,
    lens: &Lens,
    threads: usize,
) -> Result<FilmFrame, String> {
    lens.validate().map_err(str::to_owned)?;
    scene.validate_appearance()?;
    let frame = render_film_unchecked(scene, camera, lens, threads);
    scene.appearance.status()?;
    Ok(frame)
}
fn render_film_unchecked(scene: &Scene, camera: &Camera, lens: &Lens, threads: usize) -> FilmFrame {
    assert!(lens.validate().is_ok(), "invalid thin lens");
    let (width, height) = (scene.width, scene.height);
    if width == 0 || height == 0 {
        return FilmFrame::empty(width, height);
    }
    let threads = threads.max(1).min(height as usize);
    let rows = height as usize / threads;
    let extra_rows = height as usize % threads;
    let dual_safe = scene.is_dual_safe();
    let bands: Vec<(usize, FilmFrame)> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|worker| {
                let start = worker * rows + worker.min(extra_rows);
                let count = rows + usize::from(worker < extra_rows);
                scope.spawn(move || {
                    let field = scene.world();
                    let mut band = FilmFrame::empty(width, count as u32);
                    for y in start..start + count {
                        for x in 0..width as usize {
                            let index = (y - start) * width as usize + x;
                            let sample = sample_pixel(scene, &field, camera, lens, [x as u32, y as u32], dual_safe);
                            band.beauty[index] = sample.beauty;
                            band.foreground[index] = sample.foreground;
                            band.alpha[index] = sample.alpha;
                            if sample.center.hit {
                                band.depth[index] = (sample.center.pos - camera.eye).dot(camera.forward);
                                band.normals[index] = sample.center.normal;
                                band.material_ids[index] = sample.center.mat;
                            }
                        }
                    }
                    (start, band)
                })
            })
            .collect();
        handles.into_iter().map(|handle| handle.join().expect("film render worker panicked")).collect()
    });
    let mut frame = FilmFrame::empty(width, height);
    for (start, band) in bands {
        let start = start * width as usize;
        let range = start..start + band.beauty.len();
        frame.beauty[range.clone()].copy_from_slice(&band.beauty);
        frame.foreground[range.clone()].copy_from_slice(&band.foreground);
        frame.alpha[range.clone()].copy_from_slice(&band.alpha);
        frame.depth[range.clone()].copy_from_slice(&band.depth);
        frame.normals[range.clone()].copy_from_slice(&band.normals);
        frame.material_ids[range].copy_from_slice(&band.material_ids);
    }
    frame
}

struct PixelSample {
    beauty: Vec3,
    foreground: Vec3,
    alpha: f32,
    center: Hit,
}

fn sample_pixel<F: Fn(Vec3) -> Field + ?Sized>(
    scene: &Scene,
    field: &F,
    camera: &Camera,
    lens: &Lens,
    pixel: [u32; 2],
    dual_safe: bool,
) -> PixelSample {
    let [x, y] = pixel;
    let march = |ray: &mm3e_kit::march::Ray| {
        if dual_safe {
            scene.marcher.march_with(field, |point| scene.normal_dual(point), ray)
        } else {
            scene.marcher.march(field, ray)
        }
    };
    let center_ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, scene.width, scene.height);
    let center = march(&center_ray);
    let n = scene.aa.max(1);
    let inv_samples = 1.0 / (u64::from(n) * u64::from(n)) as f32;
    let mut beauty = Vec3::ZERO;
    let mut foreground = Vec3::ZERO;
    let mut hits = 0_u64;
    for sy in 0..n {
        for sx in 0..n {
            let image_sample = [x as f32 + (sx as f32 + 0.5) / n as f32, y as f32 + (sy as f32 + 0.5) / n as f32];
            let aperture_sample = aperture_sample(pixel, [sx, sy], n);
            let ray = if n == 1 {
                center_ray
            } else {
                camera.ray_with_lens(image_sample, [scene.width, scene.height], lens, aperture_sample)
            };
            let hit = if n == 1 { center } else { march(&ray) };
            if hit.hit {
                let radiance =
                    shade_hit(scene, field, &hit, &ray, 0, dual_safe, TextureFootprint::pixel(camera, scene, n));
                beauty = beauty + radiance;
                foreground = foreground + radiance;
                hits += 1;
            } else {
                beauty = beauty + shade::sky(ray.dir, scene.sun_dir);
            }
        }
    }
    PixelSample {
        beauty: beauty.scale(inv_samples),
        foreground: foreground.scale(inv_samples),
        alpha: hits as f32 * inv_samples,
        center,
    }
}

/// One jittered sample per aperture grid cell. The integer map has determinant one, hence is
/// a bijection modulo any n; pixel-dependent cyclic shifts and independent within-cell jitter
/// avoid locking a screen-grid coordinate to the same aperture coordinate across the image.
fn aperture_sample(pixel: [u32; 2], sample: [u32; 2], n: u32) -> [f32; 2] {
    if n == 1 {
        return [0.5, 0.5];
    }
    let pixel_seed = hash(pixel[0] ^ hash(pixel[1]));
    let sx = u64::from(sample[0]);
    let sy = u64::from(sample[1]);
    let n64 = u64::from(n);
    let cell_x = ((sx + sy + u64::from(pixel_seed)) % n64) as u32;
    let cell_y = ((sx + 2 * sy + u64::from(hash(pixel_seed))) % n64) as u32;
    let seed = hash(pixel_seed ^ hash(sample[0]) ^ hash(sample[1].wrapping_add(0xa511_e9b3)));
    let unit = |value: u32| (value >> 8) as f32 * (1.0 / 16_777_216.0);
    [(cell_x as f32 + unit(seed)) / n as f32, (cell_y as f32 + unit(hash(seed))) / n as f32]
}

fn hash(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^ (value >> 16)
}
