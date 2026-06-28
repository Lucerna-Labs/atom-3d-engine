//! Shading mechanism: the pure light-transport math the orchestrator combines into a pixel.
//! Every function here is the `combine` atom in disguise — a weighted sum of light terms,
//! a mix toward a reflection, a blend into fog. No policy: this module never decides how many
//! lights there are, whether shadows are on, or what the sky looks like in a given scene.

use crate::vec::Vec3;

/// Lambertian (diffuse) response: `max(n·l, 0)`.
pub fn lambert(n: Vec3, l: Vec3) -> f32 {
    n.dot(l).max(0.0)
}

/// Blinn-Phong specular highlight from a roughness in [0, 1] (smaller roughness = tighter).
pub fn specular(n: Vec3, l: Vec3, view: Vec3, roughness: f32) -> f32 {
    let half = (l + view).normalize();
    let shininess = 2.0 / (roughness * roughness + 1e-3) + 2.0;
    n.dot(half).max(0.0).powf(shininess)
}

/// Schlick's Fresnel approximation: reflectance grows toward grazing angles.
pub fn fresnel_schlick(cos_theta: f32, f0: f32) -> f32 {
    let m = (1.0 - cos_theta).clamp(0.0, 1.0);
    f0 + (1.0 - f0) * m.powi(5)
}

/// A simple physically-flavored sky: a horizon→zenith gradient with a sun disk/glow along
/// `sun_dir`. Returns linear HDR radiance (values can exceed 1 for the sun).
pub fn sky(dir: Vec3, sun_dir: Vec3) -> Vec3 {
    let t = (0.5 * (dir.y + 1.0)).clamp(0.0, 1.0);
    let horizon = Vec3::new(0.78, 0.86, 0.96);
    let zenith = Vec3::new(0.20, 0.38, 0.72);
    let base = horizon.mix(zenith, t);
    let sun = dir.dot(sun_dir).max(0.0);
    let glow = sun.powf(8.0) * 0.3 + sun.powf(220.0) * 6.0;
    base + Vec3::splat(glow)
}

/// ACES filmic tone-map (Narkowicz fit), mapping linear HDR into [0, 1].
pub fn aces(x: Vec3) -> Vec3 {
    let f = |v: f32| {
        let v = v.max(0.0);
        ((v * (2.51 * v + 0.03)) / (v * (2.43 * v + 0.59) + 0.14)).clamp(0.0, 1.0)
    };
    Vec3::new(f(x.x), f(x.y), f(x.z))
}

/// Encode linear color to approximate sRGB (gamma 2.2) for display.
pub fn gamma(x: Vec3) -> Vec3 {
    let inv = 1.0 / 2.2;
    Vec3::new(x.x.max(0.0).powf(inv), x.y.max(0.0).powf(inv), x.z.max(0.0).powf(inv))
}

/// Exponential distance fog: blend `color` toward `fog` by `1 − e^(−t·density)`.
pub fn apply_fog(color: Vec3, fog: Vec3, t: f32, density: f32) -> Vec3 {
    let f = 1.0 - (-t * density).exp();
    color.mix(fog, f.clamp(0.0, 1.0))
}
