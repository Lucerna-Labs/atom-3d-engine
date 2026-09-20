//! Immutable linear-light texture mipmaps and indexed corner UV interpolation.
//! Images use top-to-bottom source rows; UV v=0 is the bottom edge. RGB is stored
//! premultiplied by alpha so filtering does not pull colors from transparent texels.
use std::sync::{Arc, OnceLock};

pub const MAX_TEXTURE_PIXELS: usize = 16_777_216;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrap {
    Clamp,
    Repeat,
    Mirror,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filter {
    Nearest,
    Bilinear,
    Trilinear,
}
#[derive(Clone, Copy, Debug)]
pub struct Sampler {
    pub u: Wrap,
    pub v: Wrap,
    pub filter: Filter,
    pub lod_bias: f64,
}
impl Default for Sampler {
    fn default() -> Self {
        Self { u: Wrap::Repeat, v: Wrap::Repeat, filter: Filter::Trilinear, lod_bias: 0.0 }
    }
}

#[derive(Debug)]
struct Level {
    width: u32,
    height: u32,
    pixels: Vec<[f32; 4]>,
}
#[derive(Clone, Debug)]
pub struct TextureImage(Arc<Pyramid>);
#[derive(Debug)]
struct Pyramid {
    levels: Vec<Level>,
    data: bool,
    bounds: [[f32; 2]; 4],
    normal_moments: OnceLock<Result<crate::normal_filter::NormalMoments, String>>,
}
impl TextureImage {
    /// Straight-alpha input in explicitly decoded LINEAR light, in top-down row order.
    pub fn from_linear_rgba(width: u32, height: u32, pixels: Vec<[f32; 4]>) -> Result<Self, String> {
        Self::build(width, height, pixels, false)
    }
    /// Independent linear data channels; alpha is a fourth channel and never
    /// premultiplies RGB. Suitable for packed scalar maps and tangent normals.
    pub fn from_data_channels(width: u32, height: u32, pixels: Vec<[f32; 4]>) -> Result<Self, String> {
        Self::build(width, height, pixels, true)
    }
    fn build(width: u32, height: u32, pixels: Vec<[f32; 4]>, data: bool) -> Result<Self, String> {
        let count = (width as usize).checked_mul(height as usize).ok_or("texture dimensions overflow")?;
        if width == 0
            || height == 0
            || width > 8192
            || height > 8192
            || count > MAX_TEXTURE_PIXELS
            || count != pixels.len()
        {
            return Err("texture dimensions must match 1..8192 axes and at most 16777216 pixels".into());
        }
        if pixels.iter().flatten().any(|v| !v.is_finite() || !(0.0..=1.0).contains(v)) {
            return Err("linear texture RGBA components must be finite in [0,1]".into());
        }
        let pixels: Vec<_> =
            pixels.into_iter().map(|p| if data { p } else { [p[0] * p[3], p[1] * p[3], p[2] * p[3], p[3]] }).collect();
        let bounds = std::array::from_fn(|i| {
            [
                pixels.iter().map(|p| p[i]).fold(f32::INFINITY, f32::min),
                pixels.iter().map(|p| p[i]).fold(f32::NEG_INFINITY, f32::max),
            ]
        });
        let mut levels = vec![Level { width, height, pixels }];
        while levels.last().is_some_and(|l| l.width > 1 || l.height > 1) {
            let parent = levels.last().expect("initial level");
            let (width, height) = ((parent.width / 2).max(1), (parent.height / 2).max(1));
            let mut pixels = Vec::with_capacity(width as usize * height as usize);
            // Area resampling retains the mean for odd and non-power-of-two images.
            for y in 0..height {
                for x in 0..width {
                    let x0 = f64::from(x) * f64::from(parent.width) / f64::from(width);
                    let x1 = f64::from(x + 1) * f64::from(parent.width) / f64::from(width);
                    let y0 = f64::from(y) * f64::from(parent.height) / f64::from(height);
                    let y1 = f64::from(y + 1) * f64::from(parent.height) / f64::from(height);
                    let mut sum = [0.0_f64; 4];
                    let area = (x1 - x0) * (y1 - y0);
                    for sy in y0.floor() as u32..(y1.ceil() as u32).min(parent.height) {
                        for sx in x0.floor() as u32..(x1.ceil() as u32).min(parent.width) {
                            let weight = (x1.min(f64::from(sx + 1)) - x0.max(f64::from(sx)))
                                * (y1.min(f64::from(sy + 1)) - y0.max(f64::from(sy)));
                            let pixel = parent.pixels[sy as usize * parent.width as usize + sx as usize];
                            for channel in 0..4 {
                                sum[channel] += f64::from(pixel[channel]) * weight;
                            }
                        }
                    }
                    pixels.push(sum.map(|v| (v / area) as f32));
                }
            }
            levels.push(Level { width, height, pixels });
        }
        Ok(Self(Arc::new(Pyramid { levels, data, bounds, normal_moments: OnceLock::new() })))
    }
    pub fn is_data(&self) -> bool {
        self.0.data
    }
    pub fn channel_bounds(&self) -> [[f32; 2]; 4] {
        self.0.bounds
    }
    pub fn dimensions(&self) -> [u32; 2] {
        [self.0.levels[0].width, self.0.levels[0].height]
    }
    pub fn level_count(&self) -> usize {
        self.0.levels.len()
    }
    pub fn decoded_bytes(&self) -> usize {
        self.0.levels.iter().map(|l| l.pixels.len() * std::mem::size_of::<[f32; 4]>()).sum::<usize>()
            + self.normal_moment_bytes()
    }
    pub fn normal_moment_bytes(&self) -> usize {
        self.0.normal_moments.get().and_then(|v| v.as_ref().ok()).map_or(0, |m| m.decoded_bytes())
    }
    pub fn normal_variance(&self, uv: [f64; 2], lod: f64, sampler: Sampler) -> Result<f64, String> {
        if !self.is_data() {
            return Err("normal variance requires independent linear data channels".into());
        }
        if uv.iter().any(|v| !v.is_finite()) {
            return Err("normal variance UV must be finite".into());
        }
        let level = self.effective_lod(lod, sampler)?;
        if level <= 0.0 {
            return Ok(0.0);
        }
        let moments = self
            .0
            .normal_moments
            .get_or_init(|| {
                let base = &self.0.levels[0];
                crate::normal_filter::NormalMoments::new(base.width, base.height, &base.pixels)
            })
            .as_ref()
            .map_err(Clone::clone)?;
        Ok(moments.variance(uv, level, sampler))
    }
    pub fn effective_lod(&self, lod: f64, sampler: Sampler) -> Result<f64, String> {
        if !lod.is_finite() || !sampler.lod_bias.is_finite() {
            return Err("texture LOD must be finite".into());
        }
        Ok(match sampler.filter {
            Filter::Nearest | Filter::Bilinear => 0.0,
            Filter::Trilinear => (lod + sampler.lod_bias).clamp(0.0, (self.0.levels.len() - 1) as f64),
        })
    }
    /// Returns filtered linear PREMULTIPLIED RGBA. LOD is log2 texels per footprint.
    pub fn sample(&self, uv: [f64; 2], lod: f64, sampler: Sampler) -> Result<[f32; 4], String> {
        if uv.iter().any(|v| !v.is_finite()) || !lod.is_finite() || !sampler.lod_bias.is_finite() {
            return Err("texture UV/LOD must be finite".into());
        }
        let level = self.effective_lod(lod, sampler)?;
        match sampler.filter {
            Filter::Nearest => Ok(sample_level(&self.0.levels[0], uv, sampler, true)),
            Filter::Bilinear => Ok(sample_level(&self.0.levels[0], uv, sampler, false)),
            Filter::Trilinear => {
                let lo = level.floor() as usize;
                let hi = (lo + 1).min(self.0.levels.len() - 1);
                let t = level - lo as f64;
                let a = sample_level(&self.0.levels[lo], uv, sampler, false);
                let b = sample_level(&self.0.levels[hi], uv, sampler, false);
                Ok(std::array::from_fn(|i| (f64::from(a[i]) * (1.0 - t) + f64::from(b[i]) * t) as f32))
            }
        }
    }
}
pub(crate) fn fold(value: f64, wrap: Wrap) -> f64 {
    match wrap {
        Wrap::Clamp => value.clamp(0.0, 1.0),
        Wrap::Repeat => value.rem_euclid(1.0),
        Wrap::Mirror => {
            let v = value.rem_euclid(2.0);
            if v <= 1.0 {
                v
            } else {
                2.0 - v
            }
        }
    }
}
pub(crate) fn index(value: i64, size: u32, wrap: Wrap) -> usize {
    match wrap {
        Wrap::Repeat => value.rem_euclid(i64::from(size)) as usize,
        _ => value.clamp(0, i64::from(size) - 1) as usize,
    }
}
fn sample_level(level: &Level, uv: [f64; 2], sampler: Sampler, nearest: bool) -> [f32; 4] {
    let x = fold(uv[0], sampler.u) * f64::from(level.width) - 0.5;
    let y = (1.0 - fold(uv[1], sampler.v)) * f64::from(level.height) - 0.5;
    let pixel = |x: i64, y: i64| {
        level.pixels[index(y, level.height, sampler.v) * level.width as usize + index(x, level.width, sampler.u)]
    };
    if nearest {
        let ix = index((fold(uv[0], sampler.u) * f64::from(level.width)).floor() as i64, level.width, sampler.u);
        let iy = level.height as usize
            - 1
            - index((fold(uv[1], sampler.v) * f64::from(level.height)).floor() as i64, level.height, sampler.v);
        return level.pixels[iy * level.width as usize + ix];
    }
    let (ix, iy) = (x.floor() as i64, y.floor() as i64);
    let (fx, fy) = (x - x.floor(), y - y.floor());
    let (a, b, c, d) = (pixel(ix, iy), pixel(ix + 1, iy), pixel(ix, iy + 1), pixel(ix + 1, iy + 1));
    std::array::from_fn(|i| {
        ((f64::from(a[i]) * (1.0 - fx) + f64::from(b[i]) * fx) * (1.0 - fy)
            + (f64::from(c[i]) * (1.0 - fx) + f64::from(d[i]) * fx) * fy) as f32
    })
}

/// Separate corner indices preserve UV seams without splitting physical vertices.
#[derive(Clone, Debug)]
pub struct CornerUvs {
    values: Arc<Vec<[f64; 2]>>,
    indices: Arc<Vec<[u32; 3]>>,
}
impl CornerUvs {
    pub fn new(values: Vec<[f64; 2]>, indices: Vec<[u32; 3]>, triangle_count: usize) -> Result<Self, String> {
        if triangle_count == 0
            || triangle_count > crate::surface::MAX_SURFACE_TRIANGLES
            || indices.len() != triangle_count
            || values.is_empty()
            || values.len() > 3 * crate::surface::MAX_SURFACE_TRIANGLES
        {
            return Err("corner UV counts must match the surface within its topology budget".into());
        }
        if values.iter().flatten().any(|v| !v.is_finite() || v.abs() > 1_000_000.0)
            || indices.iter().flatten().any(|i| *i as usize >= values.len())
        {
            return Err("corner UV values/indices are invalid".into());
        }
        Ok(Self { values: Arc::new(values), indices: Arc::new(indices) })
    }
    pub fn triangle_count(&self) -> usize {
        self.indices.len()
    }
    pub fn corners(&self, triangle: u32) -> Option<[[f64; 2]; 3]> {
        self.indices.get(triangle as usize).map(|indices| indices.map(|i| self.values[i as usize]))
    }
    pub fn interpolate(&self, triangle: u32, weights: [f64; 3]) -> Result<[f64; 2], String> {
        if weights.iter().any(|w| !w.is_finite() || !(0.0..=1.0).contains(w))
            || (weights.iter().sum::<f64>() - 1.0).abs() > 1e-12
        {
            return Err("UV interpolation needs normalized barycentric weights".into());
        }
        let corners = self.corners(triangle).ok_or("missing UV triangle")?;
        Ok(std::array::from_fn(|axis| (0..3).map(|i| corners[i][axis] * weights[i]).sum()))
    }
}
