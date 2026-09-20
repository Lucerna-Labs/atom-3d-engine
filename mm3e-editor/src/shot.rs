//! Camera/exposure policy over complete evaluated scenes. Colors and coverage accumulate
//! in scene-linear light; utility AOVs remain the exact center-time pinhole reference.
use crate::{
    animation::AnimationSample,
    model::{array, Document, Pass, View},
};
use mm3e_kit::{shade, vec::Vec3};
use mm3e_orchestrator::{FilmFrame, Lens};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub const MAX_PIXEL_SAMPLES: u64 = 1_000_000_000;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct FilmSettings {
    /// Keep environment lighting but remove primary-ray sky from output alpha.
    pub transparent_background: bool,
    /// Include raw Z, world normal and material ID channels when writing EXR.
    pub exr_data_channels: bool,
    /// Radius, not diameter, in world meters. Zero selects a pinhole.
    pub aperture_radius_m: f32,
    /// Camera-forward distance to the sharp plane, in meters.
    pub focus_distance_m: f32,
    /// Shutter offsets from the requested animation time, in seconds.
    pub shutter_open_seconds: f32,
    pub shutter_close_seconds: f32,
    pub shutter_samples: u32,
}
impl Default for FilmSettings {
    fn default() -> Self {
        Self {
            transparent_background: false,
            exr_data_channels: false,
            aperture_radius_m: 0.0,
            focus_distance_m: 5.0,
            shutter_open_seconds: 0.0,
            shutter_close_seconds: 0.0,
            shutter_samples: 1,
        }
    }
}
impl FilmSettings {
    pub fn active(&self) -> bool {
        self.transparent_background
            || self.exr_data_channels
            || self.aperture_radius_m > 0.0
            || self.shutter_open_seconds != 0.0
            || self.shutter_close_seconds != 0.0
    }
    pub fn validate(&self, aa: u32) -> Result<(), String> {
        crate::model::range(self.aperture_radius_m, 0.0, 10.0, "aperture radius")?;
        crate::model::range(self.focus_distance_m, 0.001, 10_000.0, "focus distance")?;
        crate::model::range(self.shutter_open_seconds, -1.0, 1.0, "shutter open")?;
        crate::model::range(self.shutter_close_seconds, -1.0, 1.0, "shutter close")?;
        if self.shutter_open_seconds > self.shutter_close_seconds {
            return Err("shutter open must not exceed close".into());
        }
        if !(1..=64).contains(&self.shutter_samples) {
            return Err("shutter_samples must be 1..64".into());
        }
        if self.shutter_open_seconds == self.shutter_close_seconds && self.shutter_samples != 1 {
            return Err("zero-duration shutter must use one sample".into());
        }
        if self.shutter_open_seconds != self.shutter_close_seconds && self.shutter_samples < 2 {
            return Err("a finite shutter interval needs at least two temporal samples".into());
        }
        if self.aperture_radius_m > 0.0 && aa < 2 {
            return Err("depth of field requires at least 2x2 spatial samples".into());
        }
        Ok(())
    }
}

pub fn effective_aa(document: &Document) -> u32 {
    document.settings.spatial_aa.unwrap_or(match document.settings.quality {
        crate::model::Fidelity::Full => 2,
        _ => 1,
    })
}
pub fn sample_cost(document: &Document) -> Result<u64, String> {
    let aa = u64::from(effective_aa(document));
    let samples = u64::from(document.settings.film.shutter_samples);
    let extra = u64::from(
        document.settings.film.shutter_open_seconds != 0.0 || document.settings.film.shutter_close_seconds != 0.0,
    );
    // Film uses an additional center-pinhole ray when spatial samples do not include it.
    // Every image also computes one center-ray diagnostic pass after color rendering.
    let per_frame = aa * aa + u64::from(document.settings.film.active() && aa > 1);
    let frames = if document.settings.film.active() { samples + extra } else { 1 };
    u64::from(document.settings.width)
        .checked_mul(u64::from(document.settings.height))
        .and_then(|n| n.checked_mul(per_frame * frames + 1))
        .ok_or_else(|| "shot sample count overflow".into())
}

pub fn sample_times(document: &Document, animation: Option<&AnimationSample>) -> Result<Vec<f32>, String> {
    let settings = &document.settings.film;
    settings.validate(effective_aa(document))?;
    if animation.is_none() && (settings.shutter_open_seconds != 0.0 || settings.shutter_close_seconds != 0.0) {
        return Err("a nonzero shutter requires an explicit animation clip and reference time".into());
    }
    let base = f64::from(animation.map_or(0.0, |a| a.time));
    let open = f64::from(settings.shutter_open_seconds);
    let close = f64::from(settings.shutter_close_seconds);
    let mut times = Vec::new();
    for i in 0..settings.shutter_samples {
        let offset = if settings.shutter_samples == 1 {
            open
        } else {
            open + (close - open) * (f64::from(i) + 0.5) / f64::from(settings.shutter_samples)
        };
        let time = (base + offset) as f32;
        crate::model::range(time, -86_400.0, 86_400.0, "shutter sample time")?;
        if times.last().is_some_and(|previous| time <= *previous) {
            return Err(
                "shutter samples coincide at f32 time precision; reduce sample count or increase shutter interval"
                    .into(),
            );
        }
        times.push(time);
    }
    Ok(times)
}

fn at(animation: Option<&AnimationSample>, time: f32) -> Option<AnimationSample> {
    animation.map(|sample| AnimationSample { time, ..sample.clone() })
}

/// Preflight every temporal pose before a file or sequence directory is created.
pub fn preflight(
    document: &Document,
    pass: &Pass,
    view: Option<&View>,
    animation: Option<&AnimationSample>,
) -> Result<(), String> {
    let settings = &document.settings.film;
    if !matches!(pass, Pass::Beauty) && settings.active() {
        return Err(
            "film lens/shutter/alpha/data output is defined for beauty; use EXR data channels for center-time raw AOVs"
                .into(),
        );
    }
    if sample_cost(document)? > MAX_PIXEL_SAMPLES {
        return Err(format!("render exceeds {MAX_PIXEL_SAMPLES} primary pixel samples"));
    }
    if let Some(view) = view {
        view.validate()?;
    }
    document.compile_at(pass, animation)?;
    if settings.active() {
        for time in sample_times(document, animation)? {
            document.compile_at(pass, at(animation, time).as_ref())?;
        }
    }
    Ok(())
}

pub struct Exposure {
    pub frame: FilmFrame,
    pub camera: mm3e_kit::camera::Camera,
    pub times: Vec<f32>,
    pub metadata: BTreeMap<String, String>,
}

pub fn render(
    document: &Document,
    view: Option<&View>,
    animation: Option<&AnimationSample>,
) -> Result<Exposure, String> {
    preflight(document, &Pass::Beauty, view, animation)?;
    let times = sample_times(document, animation)?;
    let settings = &document.settings.film;
    let lens = Lens { aperture_radius: settings.aperture_radius_m, focus_distance: settings.focus_distance_m };
    let (reference_scene, reference_camera) = document.compile_at(&Pass::Beauty, animation)?;
    let camera = view.map(View::compile).unwrap_or(reference_camera);
    let mut frame = mm3e_orchestrator::render_film_checked(&reference_scene, &camera, &lens)?;
    let center = animation.map_or(0.0, |a| a.time);
    if times.as_slice() != [center] {
        frame.beauty.fill(Vec3::ZERO);
        frame.foreground.fill(Vec3::ZERO);
        frame.alpha.fill(0.0);
        for &time in &times {
            let (scene, evaluated_camera) = document.compile_at(&Pass::Beauty, at(animation, time).as_ref())?;
            let evaluated_camera = view.map(View::compile).unwrap_or(evaluated_camera);
            let sample = mm3e_orchestrator::render_film_checked(&scene, &evaluated_camera, &lens)?;
            for i in 0..frame.beauty.len() {
                frame.beauty[i] = frame.beauty[i] + sample.beauty[i];
                frame.foreground[i] = frame.foreground[i] + sample.foreground[i];
                frame.alpha[i] += sample.alpha[i];
            }
        }
        let inverse = 1.0 / times.len() as f32;
        for i in 0..frame.beauty.len() {
            frame.beauty[i] = frame.beauty[i].scale(inverse);
            frame.foreground[i] = frame.foreground[i].scale(inverse);
            frame.alpha[i] *= inverse;
        }
    }
    for (i, (&alpha, (color, foreground))) in
        frame.alpha.iter().zip(frame.beauty.iter().zip(&frame.foreground)).enumerate()
    {
        if !alpha.is_finite()
            || !(0.0..=1.0).contains(&alpha)
            || array(*color).iter().chain(array(*foreground).iter()).any(|v| !v.is_finite())
        {
            return Err(format!("render produced non-finite color or invalid coverage at pixel {i}"));
        }
    }
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "mm3e:documentFingerprint".into(),
        format!("fnv1a64:{:016x}", mm3e_kit::atoms::hash(&serde_json::to_vec(document).map_err(|e| e.to_string())?)),
    );
    let mapping: BTreeMap<String, &str> =
        document.objects.iter().enumerate().map(|(i, e)| ((i + 1).to_string(), e.id.as_str())).collect();
    metadata.insert("mm3e:materialMap".into(), serde_json::to_string(&mapping).map_err(|e| e.to_string())?);
    metadata.insert(
        "mm3e:shutter".into(),
        serde_json::to_string(&json!({"reference":animation,"sample_times_seconds":times,"settings":settings,
        "integration":"uniform midpoint quadrature in scene-linear premultiplied light","data_aov_time":center}))
        .map_err(|e| e.to_string())?,
    );
    metadata.insert(
        "mm3e:camera".into(),
        serde_json::to_string(&json!({"eye":array(camera.eye),"forward":array(camera.forward),"up":array(camera.up),
        "fov_y_degrees":(2.0*camera.fov_scale.atan()).to_degrees(),
        "lens_radius_m":lens.aperture_radius,"focus_distance_m":lens.focus_distance}))
        .map_err(|e| e.to_string())?,
    );
    Ok(Exposure { frame, camera, times, metadata })
}

/// PNG requires straight alpha. Unpremultiply in linear light before display conversion,
/// leaving transparent pixels RGB zero rather than baking in a background color.
pub fn display_rgba(frame: &FilmFrame, transparent: bool, exposure: f32) -> Vec<u8> {
    let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
    let mut pixels = Vec::with_capacity(frame.beauty.len() * 4);
    for i in 0..frame.beauty.len() {
        let alpha = if transparent { frame.alpha[i] } else { 1.0 };
        let encoded_alpha = byte(alpha);
        if encoded_alpha == 0 {
            pixels.extend_from_slice(&[0, 0, 0, 0]);
            continue;
        }
        let color = if transparent {
            if alpha > 0.0 {
                frame.foreground[i].scale(1.0 / alpha)
            } else {
                Vec3::ZERO
            }
        } else {
            frame.beauty[i]
        };
        let display = shade::gamma(shade::aces(color.scale(exposure)));
        pixels.extend_from_slice(&[byte(display.x), byte(display.y), byte(display.z), encoded_alpha]);
    }
    pixels
}

pub fn summary(exposure: &Exposure, settings: &FilmSettings) -> Value {
    json!({"settings":settings,"temporal_sample_times":exposure.times,"alpha":"opaque surface sample coverage; no transmission or volumetric matte",
        "data_aovs":if settings.exr_data_channels {Some("unfiltered center-time pinhole reference; camera-forward Z, world normal, raw material ID")}else{None}})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quantized_transparency_does_not_retain_hidden_rgb() {
        let alpha = 1.0 / 512.0;
        let frame = FilmFrame {
            width: 1,
            height: 1,
            beauty: vec![Vec3::splat(4.0)],
            foreground: vec![Vec3::splat(4.0 * alpha)],
            alpha: vec![alpha],
            depth: vec![1.0],
            normals: vec![Vec3::new(0.0, 0.0, 1.0)],
            material_ids: vec![1],
        };
        assert_eq!(display_rgba(&frame, true, 1.0), vec![0, 0, 0, 0]);
        assert_eq!(display_rgba(&frame, false, 1.0)[3], 255);
    }
    #[test]
    fn ray_budget_counts_data_reference_and_diagnostic_rays() {
        let mut document = Document::default();
        document.settings.width = 4;
        document.settings.height = 3;
        document.settings.spatial_aa = Some(4);
        assert_eq!(sample_cost(&document).unwrap(), 12 * 17);
        document.settings.film.exr_data_channels = true;
        assert_eq!(sample_cost(&document).unwrap(), 12 * 18);
        document.settings.film.shutter_open_seconds = -0.1;
        document.settings.film.shutter_close_seconds = 0.1;
        document.settings.film.shutter_samples = 8;
        assert_eq!(sample_cost(&document).unwrap(), 12 * (17 * 9 + 1));
    }
}
