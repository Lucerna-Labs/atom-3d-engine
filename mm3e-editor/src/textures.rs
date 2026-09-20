//! Native PNG sources, independent corner UVs and object-owned albedo bindings.
use crate::{
    model::{array, identifier, vec, Document, Pass, Shape, V3},
    protocol::Failure,
    storage,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use mm3e_kit::texture::{CornerUvs, TextureImage};
use mm3e_orchestrator::Scene;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{Cursor, Read},
    path::Path,
    sync::{Arc, Mutex},
};

pub const MAX_TEXTURE_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_TEXTURE_ASSETS: usize = 32;
pub const MAX_UV_SETS: usize = 512;
pub const MAX_TOTAL_UV_VALUES: usize = 1_048_576;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ColorSpace {
    Srgb,
    Linear,
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Wrap {
    Clamp,
    #[default]
    Repeat,
    Mirror,
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    Nearest,
    Bilinear,
    #[default]
    Trilinear,
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Sampler {
    pub u: Wrap,
    pub v: Wrap,
    pub filter: Filter,
    pub lod_bias: f64,
}
impl Sampler {
    fn core(self) -> Result<mm3e_kit::texture::Sampler, String> {
        if !self.lod_bias.is_finite() || !(-16.0..=16.0).contains(&self.lod_bias) {
            return Err("texture LOD bias must be finite in [-16,16]".into());
        }
        let wrap = |w| match w {
            Wrap::Clamp => mm3e_kit::texture::Wrap::Clamp,
            Wrap::Repeat => mm3e_kit::texture::Wrap::Repeat,
            Wrap::Mirror => mm3e_kit::texture::Wrap::Mirror,
        };
        Ok(mm3e_kit::texture::Sampler {
            u: wrap(self.u),
            v: wrap(self.v),
            filter: match self.filter {
                Filter::Nearest => mm3e_kit::texture::Filter::Nearest,
                Filter::Bilinear => mm3e_kit::texture::Filter::Bilinear,
                Filter::Trilinear => mm3e_kit::texture::Filter::Trilinear,
            },
            lod_bias: self.lod_bias,
        })
    }
}
#[derive(Clone, Copy, Debug, Serialize)]
struct ImageInfo {
    width: u32,
    height: u32,
    decoded_bit_depth: u8,
    alpha_min: f32,
    alpha_max: f32,
}
struct PngSource {
    bytes: Arc<[u8]>,
    encoded: Arc<str>,
    sha256: String,
    info: ImageInfo,
    cache: Mutex<Option<(ColorSpace, bool, TextureImage)>>,
}
#[derive(Clone)]
pub struct EmbeddedPng(Arc<PngSource>);
impl std::fmt::Debug for EmbeddedPng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmbeddedPng").field("sha256", &self.0.sha256).field("info", &self.0.info).finish()
    }
}
impl EmbeddedPng {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
        if bytes.len() > MAX_TEXTURE_BYTES {
            return Err("PNG source exceeds 16 MiB".into());
        }
        let (info, _) = decode_png(&bytes, None)?;
        let encoded = STANDARD.encode(&bytes).into();
        let sha256 = format!("{:x}", Sha256::digest(&bytes));
        Ok(Self(Arc::new(PngSource { bytes: bytes.into(), encoded, sha256, info, cache: Mutex::new(None) })))
    }
    pub fn source_bytes(&self) -> &[u8] {
        &self.0.bytes
    }
    pub fn sha256(&self) -> &str {
        &self.0.sha256
    }
    pub fn dimensions(&self) -> [u32; 2] {
        [self.0.info.width, self.0.info.height]
    }
    /// Source-resolution, straight-alpha pixels with the declared color transfer
    /// decoded. Delivery derives opaque modulation maps without modifying sources.
    pub(crate) fn decoded_pixels(&self, space: ColorSpace) -> Result<Vec<[f32; 4]>, String> {
        let (_, pixels) = decode_png(&self.0.bytes, Some(space))?;
        Ok(pixels.expect("requested decoded pixels"))
    }
    #[cfg(test)]
    fn image(&self, space: ColorSpace) -> Result<TextureImage, String> {
        self.image_for(space, false)
    }
    fn image_for(&self, space: ColorSpace, data: bool) -> Result<TextureImage, String> {
        if data && space != ColorSpace::Linear {
            return Err("material data maps require linear color_space".into());
        }
        let mut cache = self.0.cache.lock().map_err(|_| "texture cache poisoned")?;
        if let Some((existing, existing_data, image)) = &*cache {
            if *existing == space && *existing_data == data {
                return Ok(image.clone());
            }
        }
        let (_, pixels) = decode_png(&self.0.bytes, Some(space))?;
        let pixels = pixels.expect("requested pixels");
        let image = if data {
            TextureImage::from_data_channels(self.0.info.width, self.0.info.height, pixels)?
        } else {
            TextureImage::from_linear_rgba(self.0.info.width, self.0.info.height, pixels)?
        };
        *cache = Some((space, data, image.clone()));
        Ok(image)
    }
    fn release_cache(&self) {
        if let Ok(mut cache) = self.0.cache.lock() {
            *cache = None;
        }
    }
}
impl Serialize for EmbeddedPng {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.encoded.as_ref().serialize(s)
    }
}
impl<'de> Deserialize<'de> for EmbeddedPng {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let encoded = String::deserialize(d)?;
        if encoded.len() > MAX_TEXTURE_BYTES.div_ceil(3) * 4 {
            return Err(serde::de::Error::custom("encoded PNG exceeds source byte cap"));
        }
        Self::from_bytes(STANDARD.decode(encoded).map_err(serde::de::Error::custom)?).map_err(serde::de::Error::custom)
    }
}
type DecodedPng = (ImageInfo, Option<Vec<[f32; 4]>>);
fn decode_png(bytes: &[u8], space: Option<ColorSpace>) -> Result<DecodedPng, String> {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_limits(png::Limits { bytes: 256 * 1024 * 1024 });
    decoder.set_ignore_text_chunk(true);
    decoder.set_ignore_iccp_chunk(true);
    decoder.set_transformations(png::Transformations::EXPAND);
    let mut reader = decoder.read_info().map_err(|e| format!("PNG header: {e}"))?;
    let info = reader.info();
    let (w, h) = (info.width, info.height);
    if info.animation_control.is_some() {
        return Err("animated PNG texture sources are not supported".into());
    }
    if w == 0
        || h == 0
        || w > 8192
        || h > 8192
        || u64::from(w) * u64::from(h) > mm3e_kit::texture::MAX_TEXTURE_PIXELS as u64
    {
        return Err("PNG exceeds 8192 axes or 16777216 source pixels".into());
    }
    let size = reader.output_buffer_size().ok_or("PNG output size overflow")?;
    if size > mm3e_kit::texture::MAX_TEXTURE_PIXELS * 8 {
        return Err("PNG decoded byte count exceeds bound".into());
    }
    let mut buffer = vec![0; size];
    let frame = reader.next_frame(&mut buffer).map_err(|e| format!("PNG pixels: {e}"))?;
    reader.finish().map_err(|e| format!("PNG completion: {e}"))?;
    let step = match frame.bit_depth {
        png::BitDepth::Eight => 1,
        png::BitDepth::Sixteen => 2,
        _ => return Err("PNG expansion did not produce 8/16-bit samples".into()),
    };
    let channels = match frame.color_type {
        png::ColorType::Grayscale => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        _ => return Err("PNG palette expansion failed".into()),
    };
    let mut pixels = space.map(|_| Vec::with_capacity(w as usize * h as usize));
    let mut alpha_min = 1.0_f32;
    let mut alpha_max = 0.0_f32;
    for pixel in buffer[..frame.buffer_size()].chunks_exact(step * channels) {
        let component = |i: usize| {
            if step == 1 {
                f32::from(pixel[i]) / 255.0
            } else {
                f32::from(u16::from_be_bytes([pixel[2 * i], pixel[2 * i + 1]])) / 65535.0
            }
        };
        let mut rgba = match channels {
            1 => {
                let v = component(0);
                [v, v, v, 1.0]
            }
            2 => {
                let v = component(0);
                [v, v, v, component(1)]
            }
            3 => [component(0), component(1), component(2), 1.0],
            _ => [component(0), component(1), component(2), component(3)],
        };
        alpha_min = alpha_min.min(rgba[3]);
        alpha_max = alpha_max.max(rgba[3]);
        if let Some(space) = space {
            if space == ColorSpace::Srgb {
                for value in &mut rgba[..3] {
                    let v = f64::from(*value);
                    *value = if v <= 0.04045 { (v / 12.92) as f32 } else { ((v + 0.055) / 1.055).powf(2.4) as f32 };
                }
            }
            pixels.as_mut().expect("requested pixels").push(rgba);
        }
    }
    Ok((ImageInfo { width: w, height: h, decoded_bit_depth: (step * 8) as u8, alpha_min, alpha_max }, pixels))
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TextureAsset {
    pub id: String,
    #[serde(default)]
    pub label: String,
    pub color_space: ColorSpace,
    #[schemars(with = "String")]
    pub data: EmbeddedPng,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct UvSet {
    pub id: String,
    pub object: String,
    pub vertex_count: usize,
    pub triangles_sha256: String,
    pub values: Vec<[f64; 2]>,
    pub corner_indices: Vec<[u32; 3]>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TextureBinding {
    pub object: String,
    pub uv_set: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<String>,
    #[serde(default)]
    pub sampler: Sampler,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roughness: Option<ScalarMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metallic: Option<ScalarMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive: Option<ColorMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal: Option<NormalMap>,
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    #[default]
    R,
    G,
    B,
    A,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScalarMap {
    pub texture: String,
    #[serde(default)]
    pub channel: Channel,
    #[serde(default)]
    pub sampler: Option<Sampler>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ColorMap {
    pub texture: String,
    #[serde(default)]
    pub sampler: Option<Sampler>,
}
fn normal_strength() -> f32 {
    1.0
}
fn is_disabled(value: &bool) -> bool {
    !*value
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NormalMap {
    pub texture: String,
    #[serde(default = "normal_strength")]
    pub strength: f32,
    #[serde(default)]
    pub flip_y: bool,
    #[serde(default, skip_serializing_if = "is_disabled")]
    pub variance_filter: bool,
    #[serde(default)]
    pub sampler: Option<Sampler>,
}
impl TextureBinding {
    fn images(&self) -> Vec<(&String, bool, Option<Sampler>)> {
        let mut images = vec![];
        if let Some(id) = &self.texture {
            images.push((id, false, None));
        }
        if let Some(map) = &self.roughness {
            images.push((&map.texture, true, map.sampler));
        }
        if let Some(map) = &self.metallic {
            images.push((&map.texture, true, map.sampler));
        }
        if let Some(map) = &self.emissive {
            images.push((&map.texture, false, map.sampler));
        }
        if let Some(map) = &self.normal {
            images.push((&map.texture, true, map.sampler));
        }
        images
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ImportTexture {
    pub id: String,
    #[serde(default)]
    pub label: String,
    pub path: String,
    pub color_space: ColorSpace,
    #[serde(default)]
    pub replace: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PutUvs {
    pub id: String,
    pub object: String,
    pub values: Vec<[f64; 2]>,
    pub corner_indices: Vec<[u32; 3]>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectUvs {
    pub id: String,
    pub object: String,
    /// All coordinates are in the original surface's local rest frame.
    pub origin: V3,
    pub axis_u: V3,
    pub axis_v: V3,
    pub meters_per_uv: [f64; 2],
    #[serde(default)]
    pub offset: [f64; 2],
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct UvState {
    pub id: String,
    #[serde(default)]
    pub triangles: Vec<u32>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TextureState {
    pub id: String,
    #[serde(default)]
    pub uv: Vec<[f64; 2]>,
    #[serde(default)]
    pub lod: f64,
    #[serde(default)]
    pub sampler: Sampler,
    /// Query independent linear channels for packed scalar or normal maps.
    #[serde(default)]
    pub as_data: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExportTexture {
    pub id: String,
    pub path: String,
    #[serde(default)]
    pub overwrite: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MaterialState {
    pub points: Vec<V3>,
    #[serde(default)]
    pub footprint_m: f64,
    #[serde(default)]
    pub animation: Option<crate::animation::AnimationSample>,
}

fn topology(vertices: usize, triangles: &[[u32; 3]]) -> String {
    let mut hash = Sha256::new();
    hash.update((vertices as u64).to_le_bytes());
    for triangle in triangles {
        for corner in triangle {
            hash.update(corner.to_le_bytes());
        }
    }
    format!("{:x}", hash.finalize())
}
pub fn validate(document: &Document) -> Result<(), String> {
    if document.textures.len() > MAX_TEXTURE_ASSETS
        || document.uv_sets.len() > MAX_UV_SETS
        || document.texture_bindings.len() > crate::model::MAX_OBJECTS
    {
        return Err("texture/UV/binding count exceeds editor budget".into());
    }
    let mut textures = BTreeSet::new();
    let mut total_bytes = 0usize;
    let mut total_pixels = 0u64;
    for asset in &document.textures {
        identifier(&asset.id)?;
        if asset.label.len() > 1024 || !textures.insert(&asset.id) {
            return Err("texture ids must be unique and labels at most 1024 bytes".into());
        }
        total_bytes = total_bytes.checked_add(asset.data.source_bytes().len()).ok_or("texture byte overflow")?;
        let [w, h] = asset.data.dimensions();
        total_pixels += u64::from(w) * u64::from(h);
    }
    if total_bytes > MAX_TEXTURE_BYTES || total_pixels > mm3e_kit::texture::MAX_TEXTURE_PIXELS as u64 {
        return Err("textures exceed shared 16 MiB source or 16777216 decoded-pixel budget".into());
    }
    let mut sets = BTreeMap::new();
    let mut values = 0usize;
    for set in &document.uv_sets {
        identifier(&set.id)?;
        identifier(&set.object)?;
        if sets.insert(&set.id, set).is_some() {
            return Err("UV ids must be unique".into());
        }
        let object = document
            .objects
            .iter()
            .find(|o| o.id == set.object)
            .ok_or_else(|| format!("missing UV object {}", set.object))?;
        let Shape::Surface { vertices, triangles, .. } = &object.shape else {
            return Err("UV sets require native triangle surfaces".into());
        };
        if vertices.len() != set.vertex_count || topology(vertices.len(), triangles) != set.triangles_sha256 {
            return Err("UV topology is stale; replace the UV set with the geometry in the same transaction".into());
        }
        values = values.checked_add(set.values.len()).ok_or("UV value count overflow")?;
        if values > MAX_TOTAL_UV_VALUES {
            return Err("document exceeds 1048576 UV values".into());
        }
        CornerUvs::new(set.values.clone(), set.corner_indices.clone(), triangles.len())?;
    }
    if values > MAX_TOTAL_UV_VALUES {
        return Err("document exceeds 1048576 UV values".into());
    }
    let mut owners = BTreeSet::new();
    let mut usages = BTreeSet::new();
    let mut interpreted_pixels = 0_u64;
    for binding in &document.texture_bindings {
        identifier(&binding.object)?;
        identifier(&binding.uv_set)?;
        if !owners.insert(&binding.object) {
            return Err("an object can have only one surface-map binding".into());
        }
        let set = sets.get(&binding.uv_set).ok_or("texture binding references missing UV set")?;
        if set.object != binding.object {
            return Err("texture binding and UV set must reference the same object".into());
        }
        let images = binding.images();
        if images.is_empty() {
            return Err("surface binding requires at least one material map".into());
        }
        for (id, data, sampler) in images {
            identifier(id)?;
            let asset =
                document.textures.iter().find(|t| &t.id == id).ok_or("texture binding references missing texture")?;
            if data && asset.color_space != ColorSpace::Linear {
                return Err("roughness, metallic and normal maps require linear color_space".into());
            }
            sampler.unwrap_or(binding.sampler).core()?;
            if usages.insert((id, data)) {
                let [w, h] = asset.data.dimensions();
                interpreted_pixels += u64::from(w) * u64::from(h);
            }
        }
        if let Some(normal) = &binding.normal {
            if !normal.strength.is_finite() || !(0.0..=8.0).contains(&normal.strength) {
                return Err("normal strength must be finite in [0,8]".into());
            }
        }
        binding.sampler.core()?;
    }
    if interpreted_pixels > mm3e_kit::texture::MAX_TEXTURE_PIXELS as u64 {
        return Err("active color/data texture interpretations exceed 16777216 decoded pixels".into());
    }
    Ok(())
}
pub fn bind_scene(document: &Document, scene: &mut Scene) -> Result<(), String> {
    use mm3e_orchestrator::appearance as core;
    for binding in &document.texture_bindings {
        let index = document.objects.iter().position(|o| o.id == binding.object).ok_or("missing textured object")?;
        let set = document.uv_sets.iter().find(|s| s.id == binding.uv_set).ok_or("missing bound UV set")?;
        let uv = CornerUvs::new(set.values.clone(), set.corner_indices.clone(), set.corner_indices.len())?;
        let map = |id: &str, data: bool, sampler: Option<Sampler>| -> Result<core::TextureMap, String> {
            let image = document.textures.iter().find(|t| t.id == id).ok_or("missing bound texture")?;
            Ok(core::TextureMap {
                image: image.data.image_for(image.color_space, data)?,
                sampler: sampler.unwrap_or(binding.sampler).core()?,
            })
        };
        let scalar = |m: &ScalarMap| -> Result<core::ScalarMap, String> {
            Ok(core::ScalarMap {
                map: map(&m.texture, true, m.sampler)?,
                channel: match m.channel {
                    Channel::R => core::Channel::R,
                    Channel::G => core::Channel::G,
                    Channel::B => core::Channel::B,
                    Channel::A => core::Channel::A,
                },
            })
        };
        let maps = core::SurfaceMaps {
            albedo: binding.texture.as_ref().map(|id| map(id, false, None)).transpose()?,
            roughness: binding.roughness.as_ref().map(scalar).transpose()?,
            metallic: binding.metallic.as_ref().map(scalar).transpose()?,
            emissive: binding.emissive.as_ref().map(|m| map(&m.texture, false, m.sampler)).transpose()?,
            normal: binding
                .normal
                .as_ref()
                .map(|m| {
                    Ok::<_, String>(core::NormalMap {
                        map: map(&m.texture, true, m.sampler)?,
                        strength: m.strength,
                        flip_y: m.flip_y,
                        variance_filter: m.variance_filter,
                    })
                })
                .transpose()?,
        };
        scene.bind_surface_maps(index, uv, maps)?;
    }
    scene.validate_appearance()
}
pub fn import(document: &mut Document, root: &Path, request: &ImportTexture) -> Result<Value, Failure> {
    identifier(&request.id).map_err(Failure::invalid)?;
    if request.label.len() > 1024 {
        return Err(Failure::invalid("texture label exceeds 1024 bytes"));
    }
    let index = document.textures.iter().position(|t| t.id == request.id);
    if index.is_some() && !request.replace {
        return Err(Failure::invalid("texture exists; replace must be explicitly true"));
    }
    let path = storage::path(root, &request.path, true)?;
    if !path.metadata().map_err(|e| Failure::io(e.to_string()))?.is_file() {
        return Err(Failure::invalid("texture input must be a regular file"));
    }
    let mut bytes = vec![];
    File::open(path)
        .map_err(|e| Failure::io(e.to_string()))?
        .take(MAX_TEXTURE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| Failure::io(e.to_string()))?;
    let asset = TextureAsset {
        id: request.id.clone(),
        label: request.label.clone(),
        color_space: request.color_space,
        data: EmbeddedPng::from_bytes(bytes).map_err(Failure::invalid)?,
    };
    let result = metadata(&asset);
    if let Some(i) = index {
        document.textures[i] = asset;
    } else {
        document.textures.push(asset);
    }
    Ok(result)
}
pub fn put_uvs(document: &mut Document, request: PutUvs) -> Result<(), String> {
    let object = document.objects.iter().find(|o| o.id == request.object).ok_or("missing UV object")?;
    let Shape::Surface { vertices, triangles, .. } = &object.shape else {
        return Err("UV sets require a native surface".into());
    };
    let set = UvSet {
        id: request.id,
        object: request.object,
        vertex_count: vertices.len(),
        triangles_sha256: topology(vertices.len(), triangles),
        values: request.values,
        corner_indices: request.corner_indices,
    };
    if let Some(old) = document.uv_sets.iter_mut().find(|u| u.id == set.id) {
        *old = set;
    } else {
        document.uv_sets.push(set);
    }
    Ok(())
}
pub fn bind(document: &mut Document, binding: TextureBinding) {
    if let Some(old) = document.texture_bindings.iter_mut().find(|b| b.object == binding.object) {
        *old = binding;
    } else {
        document.texture_bindings.push(binding);
    }
}
pub fn project_uvs(document: &mut Document, request: ProjectUvs) -> Result<(), String> {
    crate::model::vector(request.origin, "UV projection origin")?;
    crate::model::vector(request.axis_u, "UV projection axis")?;
    crate::model::vector(request.axis_v, "UV projection axis")?;
    let u = vec(request.axis_u);
    let v = vec(request.axis_v);
    if (u.length() - 1.0).abs() > 1e-5 || (v.length() - 1.0).abs() > 1e-5 || u.dot(v).abs() > 1e-5 {
        return Err("UV projection axes must be perpendicular unit vectors".into());
    }
    if request.meters_per_uv.iter().any(|v| !v.is_finite() || !(1e-6..=1e6).contains(v))
        || request.offset.iter().any(|v| !v.is_finite() || v.abs() > 1e6)
    {
        return Err("UV projection scales/offsets are invalid".into());
    }
    let object = document.objects.iter().find(|o| o.id == request.object).ok_or("missing UV projection object")?;
    let Shape::Surface { vertices, triangles, .. } = &object.shape else {
        return Err("UV projection requires a native surface".into());
    };
    let values = vertices
        .iter()
        .map(|p| {
            std::array::from_fn(|axis| {
                let direction = if axis == 0 { request.axis_u } else { request.axis_v };
                let distance = (0..3)
                    .map(|i| (f64::from(p[i]) - f64::from(request.origin[i])) * f64::from(direction[i]))
                    .sum::<f64>();
                distance / request.meters_per_uv[axis] + request.offset[axis]
            })
        })
        .collect();
    let request = PutUvs { id: request.id, object: request.object, values, corner_indices: triangles.clone() };
    put_uvs(document, request)
}
pub fn uv_state(document: &Document, request: &UvState) -> Result<Value, String> {
    if request.triangles.len() > 4096 {
        return Err("UV inspection supports at most 4096 selected triangles".into());
    }
    let set = document.uv_sets.iter().find(|s| s.id == request.id).ok_or("missing UV set")?;
    let ids = if request.triangles.is_empty() {
        (0..set.corner_indices.len().min(32) as u32).collect::<Vec<_>>()
    } else {
        request.triangles.clone()
    };
    let mut triangles = vec![];
    for id in &ids {
        let indices = *set.corner_indices.get(*id as usize).ok_or("UV triangle index is out of range")?;
        triangles.push(json!({"triangle":id,"indices":indices,"values":indices.map(|i|set.values[i as usize])}));
    }
    Ok(
        json!({"id":set.id,"object":set.object,"vertex_count":set.vertex_count,"triangles_sha256":set.triangles_sha256,"uv_value_count":set.values.len(),"triangle_count":set.corner_indices.len(),"triangles":triangles,"default_preview_truncated":request.triangles.is_empty()&&ids.len()<set.corner_indices.len()}),
    )
}
pub fn remove_texture(document: &mut Document, id: &str) -> Result<(), String> {
    let i = document.textures.iter().position(|t| t.id == id).ok_or("missing texture")?;
    document.textures.remove(i);
    Ok(())
}
pub fn remove_uvs(document: &mut Document, id: &str) -> Result<(), String> {
    let i = document.uv_sets.iter().position(|u| u.id == id).ok_or("missing UV set")?;
    document.uv_sets.remove(i);
    Ok(())
}
pub fn unbind(document: &mut Document, object: &str) -> Result<(), String> {
    let i = document.texture_bindings.iter().position(|b| b.object == object).ok_or("missing texture binding")?;
    document.texture_bindings.remove(i);
    Ok(())
}
pub fn metadata(asset: &TextureAsset) -> Value {
    json!({"id":asset.id,"label":asset.label,"color_space":asset.color_space,"source_sha256":asset.data.sha256(),"source_bytes":asset.data.source_bytes().len(),"image":asset.data.0.info,"semantics":"Original embedded PNG; explicit color space overrides metadata; color/data filtering follows the selected material-map role, without changing geometry visibility"})
}
pub fn inspect(document: &Document, request: &TextureState) -> Result<Value, String> {
    if !request.lod.is_finite() {
        return Err("texture LOD must be finite".into());
    }
    request.sampler.core()?;
    if request.uv.len() > 4096 {
        return Err("texture state supports at most 4096 UV samples".into());
    }
    let asset = document.textures.iter().find(|a| a.id == request.id).ok_or("missing texture")?;
    if request.as_data && asset.color_space != ColorSpace::Linear {
        return Err("data-channel inspection requires linear color_space".into());
    }
    let mut value = metadata(asset);
    if !request.uv.is_empty() {
        let image = asset.data.image_for(asset.color_space, request.as_data)?;
        let sampler = request.sampler.core()?;
        let samples = request
            .uv
            .iter()
            .map(|&uv| {
                image.sample(uv, request.lod, sampler).map(|rgba| {
                    let mut value = json!({"uv":uv});
                    value[if request.as_data { "linear_channels" } else { "premultiplied_linear_rgba" }] = json!(rgba);
                    value
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        value["samples"] = json!(samples);
        value["requested_lod"] = json!(request.lod);
        value["sampled_lod"] = json!(image.effective_lod(request.lod, sampler)?);
        value["mip_levels"] = json!(image.level_count());
        value["decoded_mip_bytes"] = json!(image.decoded_bytes());
        value["normal_moment_bytes"] = json!(image.normal_moment_bytes());
    }
    Ok(value)
}
pub fn export(document: &Document, root: &Path, request: &ExportTexture) -> Result<Value, Failure> {
    if !request.path.to_ascii_lowercase().ends_with(".png") {
        return Err(Failure::invalid("texture export path must end in .png"));
    }
    let asset =
        document.textures.iter().find(|a| a.id == request.id).ok_or_else(|| Failure::invalid("missing texture"))?;
    let path = storage::write(root, &request.path, asset.data.source_bytes(), request.overwrite)?;
    Ok(
        json!({"path":path,"source_sha256":asset.data.sha256(),"bytes":asset.data.source_bytes().len(),"declared_color_space":asset.color_space,
        "semantics":"Original PNG bytes and metadata retained unchanged; declared working color interpretation is reported separately"}),
    )
}
pub fn material_state(document: &Document, request: &MaterialState) -> Result<Value, String> {
    if request.points.is_empty() || request.points.len() > 4096 {
        return Err("material_state supports 1..4096 points".into());
    }
    let (scene, _) = document.compile_at(&Pass::Albedo, request.animation.as_ref())?;
    let mut result = vec![];
    for &point in &request.points {
        crate::model::vector(point, "material point")?;
        let sample = scene.sample_material(vec(point), request.footprint_m)?;
        result.push(json!({"point":point,"field":sample.field.dist,"object":sample.object.map(|i|&document.objects[i].id),"triangle":sample.triangle,"uv":sample.uv,"lod":sample.lod,"footprint_lod":sample.footprint_lod,"premultiplied_linear_rgba":sample.premultiplied_rgba,"linear_albedo":array(sample.material.albedo),"roughness":sample.material.roughness,"unfiltered_roughness":sample.unfiltered_roughness,"normal_variance":sample.normal_variance,"metallic":sample.material.metallic,"linear_emissive":array(sample.material.emissive),"geometric_normal":sample.geometric_normal.map(array),"shading_normal":sample.shading_normal.map(array),"map_lods":sample.map_lods}));
    }
    Ok(
        json!({"samples":result,"semantics":"Material owner follows the exact authored CSG fold; UVs project from the nearest posed midsurface; map LODs are ordered albedo, roughness, metallic, emissive, normal; shading normals do not alter geometry"}),
    )
}
/// Historical native sources remain intact; only disposable decoded caches are
/// released when their source is no longer part of the active document.
pub(crate) fn release_history_caches<'a>(active: &Document, history: impl Iterator<Item = &'a Document>) {
    let active: BTreeSet<_> = active.textures.iter().map(|t| Arc::as_ptr(&t.data.0) as usize).collect();
    for document in history {
        for texture in &document.textures {
            if !active.contains(&(Arc::as_ptr(&texture.data.0) as usize)) {
                texture.data.release_cache();
            }
        }
    }
}

#[cfg(test)]
mod cache_tests {
    use super::*;
    #[test]
    fn history_releases_disposable_mips_without_losing_source_data_or_active_cache() {
        let mut bytes = vec![];
        {
            let mut encoder = png::Encoder::new(&mut bytes, 1, 1);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&[255, 0, 0, 255]).unwrap();
            writer.finish().unwrap();
        }
        let active = EmbeddedPng::from_bytes(bytes.clone()).unwrap();
        let old = EmbeddedPng::from_bytes(bytes.clone()).unwrap();
        active.image(ColorSpace::Srgb).unwrap();
        old.image(ColorSpace::Srgb).unwrap();
        let asset =
            |data| TextureAsset { id: "image".into(), label: String::new(), color_space: ColorSpace::Srgb, data };
        let current = Document { textures: vec![asset(active.clone())], ..Document::default() };
        let history = Document {
            textures: vec![asset(old.clone()), TextureAsset { id: "shared".into(), ..asset(active.clone()) }],
            ..Document::default()
        };
        let before = serde_json::to_vec(&history).unwrap();
        release_history_caches(&current, std::iter::once(&history));
        assert!(old.0.cache.lock().unwrap().is_none());
        assert!(active.0.cache.lock().unwrap().is_some());
        assert_eq!(old.source_bytes(), bytes);
        assert_eq!(serde_json::to_vec(&history).unwrap(), before);
        old.image(ColorSpace::Srgb).unwrap();
        assert!(old.0.cache.lock().unwrap().is_some());
    }
}
