//! Stable, area-filtered central slope moments for normal-map minification.
//! Moments describe texture variation; conversion to a shading lobe is policy.
use crate::texture::{fold, index, Sampler};
const MIN_Z: f64 = 1.0 / 65536.0;
#[derive(Clone, Copy, Debug, Default)]
struct Moment {
    mean: [f32; 2],
    variance: f32,
}
#[derive(Debug)]
struct Level {
    width: u32,
    height: u32,
    values: Vec<Moment>,
}
#[derive(Debug)]
pub(crate) struct NormalMoments {
    levels: Vec<Level>,
}
#[derive(Default)]
struct Accumulator {
    weight: f64,
    mean: [f64; 2],
    sum: f64,
}
impl Accumulator {
    fn add(&mut self, m: Moment, weight: f64) {
        if weight <= 0.0 {
            return;
        }
        let total = self.weight + weight;
        let delta = [f64::from(m.mean[0]) - self.mean[0], f64::from(m.mean[1]) - self.mean[1]];
        self.sum +=
            weight * f64::from(m.variance) + (delta[0] * delta[0] + delta[1] * delta[1]) * self.weight * weight / total;
        self.mean[0] += delta[0] * weight / total;
        self.mean[1] += delta[1] * weight / total;
        self.weight = total;
    }
    fn finish(&self) -> Moment {
        Moment { mean: self.mean.map(|v| v as f32), variance: (self.sum / self.weight).max(0.0) as f32 }
    }
}
impl NormalMoments {
    pub(crate) fn new(width: u32, height: u32, pixels: &[[f32; 4]]) -> Result<Self, String> {
        let mut values = Vec::with_capacity(pixels.len());
        for p in pixels {
            let x = 2.0 * f64::from(p[0]) - 1.0;
            let y = 2.0 * f64::from(p[1]) - 1.0;
            let z = 2.0 * f64::from(p[2]) - 1.0;
            if z < 0.0 {
                return Err("normal slope moments require the upper tangent hemisphere".into());
            }
            let denominator = z.max(MIN_Z);
            values.push(Moment { mean: [(x / denominator) as f32, (y / denominator) as f32], variance: 0.0 });
        }
        let mut levels = vec![Level { width, height, values }];
        while levels.last().is_some_and(|l| l.width > 1 || l.height > 1) {
            let source = levels.last().expect("initial normal level");
            let (width, height) = ((source.width / 2).max(1), (source.height / 2).max(1));
            let mut values = Vec::with_capacity(width as usize * height as usize);
            for y in 0..height {
                for x in 0..width {
                    let x0 = f64::from(x) * f64::from(source.width) / f64::from(width);
                    let x1 = f64::from(x + 1) * f64::from(source.width) / f64::from(width);
                    let y0 = f64::from(y) * f64::from(source.height) / f64::from(height);
                    let y1 = f64::from(y + 1) * f64::from(source.height) / f64::from(height);
                    let mut accumulator = Accumulator::default();
                    for sy in y0.floor() as u32..(y1.ceil() as u32).min(source.height) {
                        for sx in x0.floor() as u32..(x1.ceil() as u32).min(source.width) {
                            let weight = (x1.min(f64::from(sx + 1)) - x0.max(f64::from(sx)))
                                * (y1.min(f64::from(sy + 1)) - y0.max(f64::from(sy)));
                            accumulator.add(source.values[sy as usize * source.width as usize + sx as usize], weight);
                        }
                    }
                    values.push(accumulator.finish());
                }
            }
            levels.push(Level { width, height, values });
        }
        Ok(Self { levels })
    }
    pub(crate) fn decoded_bytes(&self) -> usize {
        self.levels.iter().map(|l| l.values.len() * std::mem::size_of::<Moment>()).sum()
    }
    /// Mip level zero is the authored interpolated normal field, with no added
    /// variance. Blending into coarser levels is continuous at that boundary.
    pub(crate) fn variance(&self, uv: [f64; 2], lod: f64, sampler: Sampler) -> f64 {
        if lod <= 0.0 {
            return 0.0;
        }
        let lo = lod.floor() as usize;
        let hi = (lo + 1).min(self.levels.len() - 1);
        let t = lod - lo as f64;
        let mut a = self.sample_level(lo, uv, sampler);
        if lo == 0 {
            a.variance = 0.0;
        }
        let b = self.sample_level(hi, uv, sampler);
        let mut out = Accumulator::default();
        out.add(a, 1.0 - t);
        out.add(b, t);
        f64::from(out.finish().variance)
    }
    fn sample_level(&self, level: usize, uv: [f64; 2], sampler: Sampler) -> Moment {
        let level = &self.levels[level];
        let x = fold(uv[0], sampler.u) * f64::from(level.width) - 0.5;
        let y = (1.0 - fold(uv[1], sampler.v)) * f64::from(level.height) - 0.5;
        let (ix, iy) = (x.floor() as i64, y.floor() as i64);
        let (fx, fy) = (x - x.floor(), y - y.floor());
        let mut out = Accumulator::default();
        for (x, y, w) in [
            (ix, iy, (1.0 - fx) * (1.0 - fy)),
            (ix + 1, iy, fx * (1.0 - fy)),
            (ix, iy + 1, (1.0 - fx) * fy),
            (ix + 1, iy + 1, fx * fy),
        ] {
            let value = level.values
                [index(y, level.height, sampler.v) * level.width as usize + index(x, level.width, sampler.u)];
            out.add(value, w);
        }
        out.finish()
    }
}
