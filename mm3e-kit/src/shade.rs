//! Shading mechanism: the pure light-transport math the orchestrator combines into a pixel.
//! Every function here is the `combine` atom in disguise — a weighted sum of light terms,
//! a mix toward a reflection, a blend into fog. No policy: this module never decides how many
//! lights there are, whether shadows are on, or what the sky looks like in a given scene.

use crate::vec::Vec3;

/// Lambertian (diffuse) response: `max(n·l, 0)`.
pub fn lambert(n: Vec3, l: Vec3) -> f32 {
    n.dot(l).max(0.0)
}

/// Schlick's Fresnel approximation (scalar): reflectance grows toward grazing angles.
pub fn fresnel_schlick(cos_theta: f32, f0: f32) -> f32 {
    let m = (1.0 - cos_theta).clamp(0.0, 1.0);
    f0 + (1.0 - f0) * m.powi(5)
}

/// Schlick's Fresnel for a spectral (per-channel) `f0` — the conductor-tinted reflectance.
pub fn fresnel_schlick_vec(cos_theta: f32, f0: Vec3) -> Vec3 {
    let m = (1.0 - cos_theta).clamp(0.0, 1.0).powi(5);
    f0 + (Vec3::ONE - f0).scale(m)
}

/// Relative luminance of a linear color (Rec. 709 weights).
pub fn luminance(c: Vec3) -> f32 {
    c.x * 0.2126 + c.y * 0.7152 + c.z * 0.0722
}

/// GGX / Trowbridge-Reitz normal-distribution term `D`.
fn d_ggx(noh: f32, a2: f32) -> f32 {
    let d = noh * noh * (a2 - 1.0) + 1.0;
    a2 / (std::f32::consts::PI * d * d).max(1e-7)
}

/// Height-correlated Smith visibility term `V` (already folds in the `1/(4·NoL·NoV)` denominator).
fn v_smith_ggx_correlated(nol: f32, nov: f32, a2: f32) -> f32 {
    let gv = nol * (nov * nov * (1.0 - a2) + a2).sqrt();
    let gl = nov * (nol * nol * (1.0 - a2) + a2).sqrt();
    0.5 / (gv + gl).max(1e-5)
}

/// Cook-Torrance microfacet BRDF for one light direction, evaluated as outgoing radiance per
/// unit incident radiance: `(diffuse + specular)·max(n·l, 0)`. This is the `combine` atom —
/// a weighted sum of the `D·V·F` specular lobe and a Fresnel-balanced Lambert diffuse lobe —
/// over the `project` atom (every term is a dot product) and `compare` (the half-vector).
/// `metallic` darkens diffuse and tints the F0 toward `albedo`; `roughness` widens the lobe;
/// `reflectance` is the dielectric specular level (0.5 → the canonical 4% F0, glTF/Filament).
pub fn brdf(n: Vec3, l: Vec3, view: Vec3, albedo: Vec3, metallic: f32, roughness: f32, reflectance: f32) -> Vec3 {
    let nol = n.dot(l).max(0.0);
    if nol <= 0.0 {
        return Vec3::ZERO;
    }
    let nov = n.dot(view).max(1e-4);
    let h = (l + view).normalize();
    let noh = n.dot(h).max(0.0);
    let voh = view.dot(h).max(0.0);

    let a = (roughness * roughness).max(1e-3);
    let a2 = a * a;
    let f0 = Vec3::splat(0.08 * reflectance.clamp(0.0, 1.0)).mix(albedo, metallic);
    let f = fresnel_schlick_vec(voh, f0);

    let spec = f.scale(d_ggx(noh, a2) * v_smith_ggx_correlated(nol, nov, a2));
    // Energy left for diffuse after specular reflection; metals have no diffuse.
    let kd = (Vec3::ONE - f).scale(1.0 - metallic);
    let diff = kd.cmul(albedo).scale(1.0 / std::f32::consts::PI);

    (diff + spec).scale(nol)
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
