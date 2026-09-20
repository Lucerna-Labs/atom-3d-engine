//! Film image encoding from the renderer's original scene-linear RGB buffer.
//!
//! This path does not invert a tone map or apply exposure/gamma. RGB values use linear
//! Rec.709/sRGB primaries and D65. The RGB plate encoder remains available alongside
//! compositing RGBA, depth, normal and material-ID output. Deep data and OCIO transforms
//! are separate capabilities and are not implied by the EXR container.

use exr::{
    image::{write::channels::WritableChannels, Encoding, Image, Layer, SpecificChannels},
    math::Vec2,
    meta::{
        attribute::{AttributeValue, Chromaticities, Text},
        header::LayerAttributes,
    },
    prelude::WritableImage,
};
use mm3e_kit::vec::Vec3;
use mm3e_orchestrator::film::FilmFrame;
use std::{collections::BTreeMap, io::Cursor};

/// Encode exactly `width * height` row-major, top-to-bottom pixels as lossless f32 RGB EXR.
/// Finite negative and HDR values are retained. Invalid data returns an error before encoding.
pub fn encode_exr(width: u32, height: u32, pixels: &[Vec3]) -> Result<Vec<u8>, String> {
    encode_rgb_exr(width, height, pixels, false)
}

/// Lossless, linear RGB modulation values for surface texture interchange.
/// Uses texture-specific metadata, without changing the film plate contract.
pub(crate) fn encode_texture_exr(width: u32, height: u32, pixels: &[Vec3]) -> Result<Vec<u8>, String> {
    encode_rgb_exr(width, height, pixels, true)
}

fn encode_rgb_exr(width: u32, height: u32, pixels: &[Vec3], texture: bool) -> Result<Vec<u8>, String> {
    if width == 0 || height == 0 || width > i32::MAX as u32 || height > i32::MAX as u32 {
        return Err("EXR dimensions must be positive signed 32-bit integers".into());
    }
    let width = width as usize;
    let height = height as usize;
    let count = width.checked_mul(height).ok_or("EXR dimensions overflow addressable memory")?;
    if pixels.len() != count {
        return Err(format!("EXR requires exactly {count} pixels; received {}", pixels.len()));
    }
    if let Some(index) = pixels.iter().position(|p| !(p.x.is_finite() && p.y.is_finite() && p.z.is_finite())) {
        return Err(format!("EXR pixel {index} contains a non-finite RGB value"));
    }

    let mut attributes = LayerAttributes {
        software_name: Some(Text::from("MM3E agent editor")),
        comments: Some(Text::from(if texture {
            "Linear f32 RGB material modulation texture; no exposure or display transfer applied"
        } else {
            "Scene-linear f32 RGB; no exposure, tone map or display transfer applied"
        })),
        ..LayerAttributes::default()
    };
    for (name, value) in [
        ("colorSpace", "Linear Rec.709 (sRGB primaries), D65"),
        ("transferFunction", "linear"),
        (
            "mm3e:alphaSemantics",
            if texture {
                "RGB material modulation; source alpha incorporated into values; no opacity channel"
            } else {
                "RGB only; opaque scene including background"
            },
        ),
    ] {
        attributes.other.insert(Text::from(name), AttributeValue::Text(Text::from(value)));
    }
    let layer = Layer::new(
        (width, height),
        attributes,
        Encoding::SMALL_LOSSLESS,
        SpecificChannels::rgb(|position: Vec2<usize>| {
            let pixel = pixels[position.y() * width + position.x()];
            (pixel.x, pixel.y, pixel.z)
        }),
    );
    let mut image = Image::from_layer(layer);
    image.attributes.chromaticities = Some(Chromaticities {
        red: Vec2(0.64, 0.33),
        green: Vec2(0.30, 0.60),
        blue: Vec2(0.15, 0.06),
        white: Vec2(0.3127, 0.3290),
    });
    let mut bytes = Cursor::new(Vec::new());
    image.write().non_parallel().to_buffered(&mut bytes).map_err(|e| format!("EXR encoding failed: {e}"))?;
    Ok(bytes.into_inner())
}

/// Encode a scene-linear compositing image without applying any display transform.
///
/// Transparent output uses the renderer's already premultiplied `foreground` RGB and
/// sampled coverage alpha. Opaque output uses `beauty` (including the environment) with
/// alpha one. Neither path divides or multiplies RGB by alpha again.
///
/// Float Z is nonnegative camera-forward depth in world units, zero when the camera starts
/// inside/on geometry, and positive infinity on a miss. `N.X`, `N.Y`, `N.Z` contain world-space normals;
/// `material.ID` is an exact uint32
/// material index with `u32::MAX` on a miss. These data channels are unfiltered pixel-center
/// pinhole samples at the center exposure time, not lens/shutter averages like RGB and alpha.
pub fn encode_compositing_exr(frame: &FilmFrame, transparent_background: bool) -> Result<Vec<u8>, String> {
    encode_compositing_exr_with_metadata(frame, transparent_background, &BTreeMap::new())
}

/// Encode compositing channels with bounded, UTF-8 `mm3e:` provenance attributes.
///
/// Extra attributes cannot replace the fixed channel/color semantics. Keys are ASCII,
/// at most 255 bytes, and contain only letters, digits, underscore, colon, dot or hyphen.
/// Values are limited to 4 KiB, except `mm3e:materialMap` (128 KiB). At most 64 attributes
/// and 256 KiB of total extra key/value bytes are accepted. Exposure/reference times in
/// metadata must describe the samples supplied by the caller; this encoder does not render.
pub fn encode_compositing_exr_with_metadata(
    frame: &FilmFrame,
    transparent_background: bool,
    metadata: &BTreeMap<String, String>,
) -> Result<Vec<u8>, String> {
    encode_film_exr(frame, transparent_background, metadata, true)
}

/// Encode only scene-linear f32 R/G/B/A and bounded provenance metadata.
///
/// Alpha/background semantics match [`encode_compositing_exr`]. No depth, normal or ID
/// channels or associated semantic attributes are emitted. Only buffers actually used for
/// RGBA are required: transparent output uses foreground/alpha; opaque output uses beauty
/// and alpha one. Absent or invalid unused data buffers do not prevent RGBA delivery.
pub fn encode_rgba_exr_with_metadata(
    frame: &FilmFrame,
    transparent_background: bool,
    metadata: &BTreeMap<String, String>,
) -> Result<Vec<u8>, String> {
    encode_film_exr(frame, transparent_background, metadata, false)
}

fn encode_film_exr(
    frame: &FilmFrame,
    transparent_background: bool,
    metadata: &BTreeMap<String, String>,
    data_channels: bool,
) -> Result<Vec<u8>, String> {
    let width = frame.width;
    let height = frame.height;
    if width == 0 || height == 0 || width > i32::MAX as u32 || height > i32::MAX as u32 {
        return Err("EXR dimensions must be positive signed 32-bit integers".into());
    }
    let width = width as usize;
    let height = height as usize;
    let count = width.checked_mul(height).ok_or("EXR dimensions overflow addressable memory")?;
    for (name, length, required) in [
        ("beauty", frame.beauty.len(), data_channels || !transparent_background),
        ("foreground", frame.foreground.len(), data_channels || transparent_background),
        ("alpha", frame.alpha.len(), data_channels || transparent_background),
        ("depth", frame.depth.len(), data_channels),
        ("normals", frame.normals.len(), data_channels),
        ("material_ids", frame.material_ids.len(), data_channels),
    ] {
        if required && length != count {
            return Err(format!("EXR {name} requires exactly {count} samples; received {length}"));
        }
    }
    for (name, pixels, required) in [
        ("beauty", &frame.beauty, data_channels || !transparent_background),
        ("foreground", &frame.foreground, data_channels || transparent_background),
        ("normals", &frame.normals, data_channels),
    ] {
        if !required {
            continue;
        }
        if let Some(index) = pixels.iter().position(|p| !(p.x.is_finite() && p.y.is_finite() && p.z.is_finite())) {
            return Err(format!("EXR {name} sample {index} contains a non-finite value"));
        }
    }
    if data_channels || transparent_background {
        if let Some(index) = frame.alpha.iter().position(|alpha| !alpha.is_finite() || !(0.0..=1.0).contains(alpha)) {
            return Err(format!("EXR alpha sample {index} must be finite and in [0, 1]"));
        }
    }
    if data_channels {
        if let Some(index) = frame.depth.iter().position(|depth| depth.is_nan() || *depth < 0.0) {
            return Err(format!(
                "EXR depth sample {index} must be nonnegative camera-forward depth or positive infinity"
            ));
        }
    }

    let mut attributes = LayerAttributes {
        software_name: Some(Text::from("MM3E agent editor")),
        comments: Some(Text::from(if data_channels {
            "Scene-linear f32 RGBA and primary-surface data; no exposure, tone map or display transfer applied"
        } else {
            "Scene-linear f32 RGBA; no exposure, tone map or display transfer applied"
        })),
        ..LayerAttributes::default()
    };
    for (name, value) in [
        ("colorSpace", "Linear Rec.709 (sRGB primaries), D65"),
        ("transferFunction", "linear"),
        (
            "mm3e:alphaSemantics",
            if transparent_background {
                "premultiplied foreground RGB; A is geometric sample coverage; environment excluded from RGB"
            } else {
                "opaque RGB beauty including environment; A is 1"
            },
        ),
    ] {
        attributes.other.insert(Text::from(name), AttributeValue::Text(Text::from(value)));
    }
    let data_semantics = [
        ("mm3e:depthSemantics", "Z is nonnegative camera-forward depth in world units; zero for camera origin inside/on geometry; positive infinity on miss"),
        ("mm3e:normalSemantics", "N.X/N.Y/N.Z are world-space surface normals; zero vector on miss"),
        ("mm3e:materialIdSemantics", "material.ID is a uint32 material index; UINT32_MAX on miss; not Cryptomatte"),
        ("mm3e:aovSampling", "Z/N/material.ID: unfiltered pixel-center pinhole reference at center exposure time; RGB/A: color samples may include lens and shutter integration"),
    ];
    if data_channels {
        for (name, value) in data_semantics {
            attributes.other.insert(Text::from(name), AttributeValue::Text(Text::from(value)));
        }
    }
    if metadata.len() > 64 {
        return Err("EXR metadata cannot exceed 64 extra attributes".into());
    }
    let mut metadata_bytes = 0usize;
    for (name, value) in metadata {
        if name.len() > 255
            || !name.starts_with("mm3e:")
            || name.len() == "mm3e:".len()
            || !name.bytes().all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b':' | b'.' | b'-'))
        {
            return Err("EXR metadata keys must be namespaced mm3e: ASCII identifiers of at most 255 bytes".into());
        }
        let name_text = Text::from_slice_unchecked(name.as_bytes());
        if attributes.other.contains_key(&name_text) || data_semantics.iter().any(|(key, _)| *key == name) {
            return Err(format!("EXR metadata cannot override reserved attribute {name}"));
        }
        let limit = if name == "mm3e:materialMap" { 128 * 1024 } else { 4 * 1024 };
        if value.len() > limit {
            return Err(format!("EXR metadata value {name} exceeds {limit} bytes"));
        }
        metadata_bytes += name.len() + value.len();
        if metadata_bytes > 256 * 1024 {
            return Err("EXR metadata exceeds 256 KiB".into());
        }
        // EXR string attributes contain bytes. Preserve UTF-8 instead of Text::from's
        // conversion of Rust Unicode scalars into individual Latin-1 bytes.
        attributes.other.insert(name_text, AttributeValue::Text(Text::from_slice_unchecked(value.as_bytes())));
    }
    let channels = SpecificChannels::build().with_channel("R").with_channel("G").with_channel("B").with_channel("A");
    if !data_channels {
        let channels = channels.with_pixel_fn(|position: Vec2<usize>| {
            let index = position.y() * width + position.x();
            let rgb = if transparent_background { frame.foreground[index] } else { frame.beauty[index] };
            let alpha = if transparent_background { frame.alpha[index] } else { 1.0 };
            (rgb.x, rgb.y, rgb.z, alpha)
        });
        let mut image = Image::from_layer(Layer::new((width, height), attributes, Encoding::SMALL_LOSSLESS, channels));
        image.attributes.chromaticities = Some(rec709_chromaticities());
        return encode_image(&image);
    }
    let channels = channels
        .with_channel("Z")
        .with_channel("N.X")
        .with_channel("N.Y")
        .with_channel("N.Z")
        .with_channel("material.ID")
        .with_pixel_fn(|position: Vec2<usize>| {
            let index = position.y() * width + position.x();
            let rgb = if transparent_background { frame.foreground[index] } else { frame.beauty[index] };
            let alpha = if transparent_background { frame.alpha[index] } else { 1.0 };
            let normal = frame.normals[index];
            (rgb.x, rgb.y, rgb.z, alpha, frame.depth[index], normal.x, normal.y, normal.z, frame.material_ids[index])
        });
    let layer = Layer::new((width, height), attributes, Encoding::SMALL_LOSSLESS, channels);
    let mut image = Image::from_layer(layer);
    image.attributes.chromaticities = Some(rec709_chromaticities());
    encode_image(&image)
}

fn rec709_chromaticities() -> Chromaticities {
    Chromaticities {
        red: Vec2(0.64, 0.33),
        green: Vec2(0.30, 0.60),
        blue: Vec2(0.15, 0.06),
        white: Vec2(0.3127, 0.3290),
    }
}

fn encode_image<'a, Channels: WritableChannels<'a>>(image: &'a Image<Layer<Channels>>) -> Result<Vec<u8>, String> {
    let mut bytes = Cursor::new(Vec::new());
    image.write().non_parallel().to_buffered(&mut bytes).map_err(|e| format!("EXR encoding failed: {e}"))?;
    Ok(bytes.into_inner())
}
