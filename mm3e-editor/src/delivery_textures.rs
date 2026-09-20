//! Source-preserving UsdPreviewSurface texture transport. The USD consumer owns
//! filtering; this module does not claim to reproduce native LOD or BRDF behavior.
use crate::{
    film,
    model::{vec, Document},
    textures::{Channel, ColorSpace, Sampler, Wrap},
    usd::{UsdMaterial, UsdTexture, UsdTextureChannel, UsdTextureColorSpace, UsdTextureRole, UsdTextureWrap},
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub(crate) const MAX_ASSET_BYTES: usize = 256 * 1024 * 1024;
type DecodedSource = (String, Vec<[f32; 4]>, [u32; 2]);

#[derive(Default)]
pub(crate) struct Assets {
    /// Basenames only. The caller owns and reserves the delivery asset directory.
    pub files: BTreeMap<String, Vec<u8>>,
    pub conversions: Vec<Value>,
    bytes: usize,
}
impl Assets {
    fn insert(&mut self, name: String, bytes: Vec<u8>) -> Result<(), String> {
        if let Some(existing) = self.files.get(&name) {
            if existing != &bytes {
                return Err("USD asset name collision".into());
            }
            return Ok(());
        }
        self.bytes = self.bytes.checked_add(bytes.len()).ok_or("USD asset byte overflow")?;
        if self.bytes > MAX_ASSET_BYTES {
            return Err("USD assets exceed 256 MiB delivery budget".into());
        }
        self.files.insert(name, bytes);
        Ok(())
    }
}

fn wrap(value: Wrap) -> UsdTextureWrap {
    match value {
        Wrap::Clamp => UsdTextureWrap::Clamp,
        Wrap::Repeat => UsdTextureWrap::Repeat,
        Wrap::Mirror => UsdTextureWrap::Mirror,
    }
}
fn channel(value: Channel) -> (usize, UsdTextureChannel) {
    match value {
        Channel::R => (0, UsdTextureChannel::R),
        Channel::G => (1, UsdTextureChannel::G),
        Channel::B => (2, UsdTextureChannel::B),
        Channel::A => (3, UsdTextureChannel::A),
    }
}
fn texture(
    role: UsdTextureRole,
    path: String,
    channel: UsdTextureChannel,
    sampler: Sampler,
    scale: [f32; 4],
) -> UsdTexture {
    UsdTexture {
        role,
        asset_path: path,
        channel,
        source_color_space: UsdTextureColorSpace::Raw,
        wrap_s: wrap(sampler.u),
        wrap_t: wrap(sampler.v),
        scale,
        bias: [0.0; 4],
    }
}

/// Converts all selected bindings or rejects before any output is reserved.
pub(crate) fn prepare(
    document: &Document,
    indices: &[usize],
    directory: &str,
    materials: &mut [UsdMaterial],
) -> Result<Assets, String> {
    let mut assets = Assets::default();
    for (slot, &index) in indices.iter().enumerate() {
        let object = &document.objects[index];
        let Some(binding) = document.texture_bindings.iter().find(|b| b.object == object.id) else { continue };
        if binding.normal.as_ref().is_some_and(|m| m.strength != 0.0) {
            return Err(format!(
                "USD object {} has an active normal map; composed-surface tangent-frame transfer is not implemented",
                object.id
            ));
        }
        let mut source = |id: &str| -> Result<DecodedSource, String> {
            let asset = document.textures.iter().find(|a| a.id == id).ok_or("USD texture source is missing")?;
            let name = format!("source-{}.png", asset.data.sha256());
            assets.insert(name.clone(), asset.data.source_bytes().to_vec())?;
            Ok((name, asset.data.decoded_pixels(asset.color_space)?, asset.data.dimensions()))
        };
        // Preserve all original images even when their evaluated contribution is a
        // constant or a disabled normal. Content-addressed files deduplicate aliases.
        let mut originals = BTreeMap::new();
        for id in binding
            .texture
            .iter()
            .map(String::as_str)
            .chain(binding.roughness.iter().map(|m| m.texture.as_str()))
            .chain(binding.metallic.iter().map(|m| m.texture.as_str()))
            .chain(binding.emissive.iter().map(|m| m.texture.as_str()))
            .chain(binding.normal.iter().map(|m| m.texture.as_str()))
        {
            if !originals.contains_key(id) {
                originals.insert(id.to_owned(), source(id)?);
            }
        }
        let material = &mut materials[slot];
        for (role, id, sampler, factor) in [
            (UsdTextureRole::Diffuse, binding.texture.as_deref(), binding.sampler, object.material.albedo),
            (
                UsdTextureRole::Emissive,
                binding.emissive.as_ref().map(|m| m.texture.as_str()),
                binding.emissive.as_ref().and_then(|m| m.sampler).unwrap_or(binding.sampler),
                object.material.emissive,
            ),
        ] {
            let Some(id) = id else { continue };
            let (original, pixels, [w, h]) = &originals[id];
            let diffuse = matches!(role, UsdTextureRole::Diffuse);
            let values: Vec<_> = pixels
                .iter()
                .map(|p| {
                    vec(std::array::from_fn(|i| {
                        // Same f32 premultiplication as TextureImage::from_linear_rgba.
                        let premultiplied = p[i] * p[3];
                        if diffuse {
                            (premultiplied + 1.0 - p[3]).clamp(0.0, 1.0)
                        } else {
                            premultiplied
                        }
                    }))
                })
                .collect();
            let bytes = film::encode_texture_exr(*w, *h, &values)?;
            let hash = format!("{:x}", Sha256::digest(&bytes));
            let name = format!("linear-{hash}.exr");
            assets.insert(name.clone(), bytes)?;
            material.textures.push(texture(
                role,
                format!("{directory}/{name}"),
                UsdTextureChannel::Rgb,
                sampler,
                [factor[0], factor[1], factor[2], 1.0],
            ));
            assets.conversions.push(json!({"object":object.id,"role":if diffuse {"diffuse"}else{"emissive"},"source":original,"derived":name,"width":w,"height":h,
                "source_texture_id":id,"source_color_space":document.textures.iter().find(|a|a.id == id).expect("validated texture").color_space,
                "operation":if diffuse {"linear RGB * alpha + (1 - alpha); constant material factor in shader scale"}else{"linear RGB * alpha; constant material factor in shader scale"},
                "native_sampler":sampler,"delivered_filtering":"reader_defined","opacity":1.0}));
        }
        for (role, map, factor) in [
            (UsdTextureRole::Roughness, binding.roughness.as_ref(), object.material.roughness),
            (UsdTextureRole::Metallic, binding.metallic.as_ref(), object.material.metallic),
        ] {
            let Some(map) = map else { continue };
            let asset = document.textures.iter().find(|a| a.id == map.texture).ok_or("USD scalar texture missing")?;
            if asset.color_space != ColorSpace::Linear {
                return Err("USD scalar maps require native linear interpretation".into());
            }
            let (source, pixels, _) = &originals[&map.texture];
            let (component, out) = channel(map.channel);
            let minimum = pixels.iter().map(|p| factor * p[component]).fold(f32::INFINITY, f32::min);
            let maximum = pixels.iter().map(|p| factor * p[component]).fold(f32::NEG_INFINITY, f32::max);
            if matches!(role, UsdTextureRole::Roughness) && minimum < 0.04 && maximum > 0.04 {
                return Err(format!("USD object {} roughness straddles the native 0.04 floor; pixelwise baking would change filtered values", object.id));
            }
            let constant_floor = matches!(role, UsdTextureRole::Roughness) && maximum <= 0.04;
            if constant_floor {
                material.roughness = 0.04;
            } else {
                material.textures.push(texture(
                    role,
                    format!("{directory}/{source}"),
                    out,
                    map.sampler.unwrap_or(binding.sampler),
                    [factor; 4],
                ));
            }
            assets.conversions.push(json!({"object":object.id,"role":if matches!(role,UsdTextureRole::Roughness){"roughness"}else{"metallic"},
                "source":source,"channel":map.channel,"factor":factor,"constant_floor":constant_floor,
                "native_sampler":map.sampler.unwrap_or(binding.sampler),"delivered_filtering":"reader_defined"}));
        }
        if let Some(normal) = &binding.normal {
            assets.conversions.push(json!({"object":object.id,"role":"normal","source":originals[&normal.texture].0,
                "delivered":"geometric normal","reason":"authored normal strength is zero","strength":normal.strength}));
        }
    }
    Ok(assets)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::textures::{EmbeddedPng, TextureAsset};
    use exr::prelude::*;
    use mm3e_kit::texture::{Sampler as CoreSampler, TextureImage};
    use std::io::Cursor;

    fn fixture(pixels: &[u8], space: ColorSpace) -> (Document, Vec<UsdMaterial>) {
        let mut bytes = vec![];
        {
            let mut encoder = png::Encoder::new(&mut bytes, 2, 2);
            encoder.set_color(png::ColorType::Rgba);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(pixels).unwrap();
            writer.finish().unwrap();
        }
        let mut document = Document { objects: serde_json::from_value(json!([{"id":"cloth","shape":{"type":"sphere","radius":1},"material":{"albedo":[0.7,0.2,0.9],"emissive":[3.0,2.0,1.0]}}])).unwrap(), ..Document::default() };
        document.textures.push(TextureAsset {
            id: "image".into(),
            label: "original".into(),
            color_space: space,
            data: EmbeddedPng::from_bytes(bytes).unwrap(),
        });
        document.texture_bindings = serde_json::from_value(
            json!([{"object":"cloth","uv_set":"uv","texture":"image","emissive":{"texture":"image"}}]),
        )
        .unwrap();
        let materials = vec![UsdMaterial {
            id: "cloth".into(),
            diffuse_color: [0.7, 0.2, 0.9],
            roughness: 0.65,
            metallic: 0.0,
            ior: 1.5,
            emissive_color: [3.0, 2.0, 1.0],
            textures: vec![],
        }];
        (document, materials)
    }
    fn exr_pixels(bytes: &[u8]) -> Vec<[f32; 4]> {
        let image = read()
            .no_deep_data()
            .largest_resolution_level()
            .all_channels()
            .all_layers()
            .all_attributes()
            .non_parallel()
            .from_buffered(Cursor::new(bytes))
            .unwrap();
        let layer = &image.layer_data[0];
        let mut result = vec![[0., 0., 0., 1.]; layer.size.area()];
        for channel in &layer.channel_data.list {
            let axis = match channel.name.to_string().as_str() {
                "R" => 0,
                "G" => 1,
                "B" => 2,
                _ => panic!("unexpected channel"),
            };
            let exr::image::FlatSamples::F32(values) = &channel.sample_data else { panic!("f32 required") };
            for (pixel, value) in result.iter_mut().zip(values) {
                pixel[axis] = *value;
            }
        }
        result
    }
    #[test]
    fn opaque_color_conversion_preserves_original_and_native_filtered_modulation() {
        let (document, mut materials) =
            fixture(&[255, 0, 0, 0, 0, 255, 0, 128, 90, 40, 230, 255, 200, 160, 30, 73], ColorSpace::Srgb);
        let assets = prepare(&document, &[0], "assets", &mut materials).unwrap();
        let source = &document.textures[0];
        assert_eq!(assets.files[&format!("source-{}.png", source.data.sha256())], source.data.source_bytes());
        let native =
            TextureImage::from_linear_rgba(2, 2, source.data.decoded_pixels(ColorSpace::Srgb).unwrap()).unwrap();
        for map in &materials[0].textures {
            let pixels = exr_pixels(&assets.files[map.asset_path.strip_prefix("assets/").unwrap()]);
            let derived = TextureImage::from_data_channels(2, 2, pixels).unwrap();
            for uv in [[0.13, 0.26], [0.49, 0.51], [0.9, 0.1], [-0.25, 1.2], [0., 0.]] {
                for lod in [0.0, 0.4, 1.0, 3.0] {
                    let raw = native.sample(uv, lod, CoreSampler::default()).unwrap();
                    let out = derived.sample(uv, lod, CoreSampler::default()).unwrap();
                    for c in 0..3 {
                        let expected =
                            if matches!(map.role, UsdTextureRole::Diffuse) { raw[c] + 1. - raw[3] } else { raw[c] };
                        assert!((out[c] - expected).abs() < 2e-7, "UV {uv:?} LOD {lod}: {} != {expected}", out[c]);
                    }
                }
            }
        }
    }
    #[test]
    fn scalar_channels_keep_source_alpha_independent_and_refuse_mixed_roughness_floor() {
        let (mut document, mut materials) =
            fixture(&[100, 10, 20, 0, 255, 80, 90, 128, 100, 255, 200, 255, 150, 100, 10, 64], ColorSpace::Linear);
        document.texture_bindings=serde_json::from_value(json!([{"object":"cloth","uv_set":"uv","roughness":{"texture":"image","channel":"r"},"metallic":{"texture":"image","channel":"a"}}])).unwrap();
        document.objects[0].material.metallic = 0.7;
        let assets = prepare(&document, &[0], "assets", &mut materials).unwrap();
        assert_eq!(assets.files.len(), 1);
        assert_eq!(materials[0].textures.len(), 2);
        assert!(matches!(materials[0].textures[1].channel, UsdTextureChannel::A));
        assert_eq!(materials[0].textures[1].scale, [0.7; 4]);
        document.texture_bindings[0].roughness.as_mut().unwrap().channel = Channel::A;
        assert!(prepare(&document, &[0], "assets", &mut materials).err().unwrap().contains("straddles"));
    }
    #[test]
    fn constant_roughness_floor_is_explicit_and_active_normal_is_never_dropped() {
        let (mut document, mut materials) = fixture(&[0; 16], ColorSpace::Linear);
        document.texture_bindings =
            serde_json::from_value(json!([{"object":"cloth","uv_set":"uv","roughness":{"texture":"image"}}])).unwrap();
        let assets = prepare(&document, &[0], "assets", &mut materials).unwrap();
        assert_eq!(materials[0].roughness, 0.04);
        assert!(materials[0].textures.is_empty());
        assert_eq!(assets.conversions[0]["constant_floor"], true);
        document.texture_bindings[0].normal = Some(serde_json::from_value(json!({"texture":"image"})).unwrap());
        assert!(prepare(&document, &[0], "assets", &mut materials).err().unwrap().contains("tangent-frame"));
    }
}
