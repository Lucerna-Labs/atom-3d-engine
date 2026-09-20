//! Filesystem-level bundle publication and constant-map demand regression.
use mm3e_editor::{
    delivery::{self, ExportUsdRequest},
    model::Document,
    textures::{self, ColorSpace, EmbeddedPng, TextureAsset},
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};
struct Root(PathBuf);
impl Root {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "mm3e-usd-assets-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Root {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn fixture() -> (Document, ExportUsdRequest) {
    let mut bytes = vec![];
    {
        let mut e = png::Encoder::new(&mut bytes, 1, 1);
        e.set_color(png::ColorType::Rgba);
        let mut w = e.write_header().unwrap();
        w.write_image_data(&[0, 30, 80, 0]).unwrap();
        w.finish().unwrap();
    }
    let mut d = Document { objects:serde_json::from_value(json!([{"id":"sheet","shape":{"type":"surface","vertices":[[-0.6,-0.6,0],[0.6,-0.6,0],[0.6,0.6,0],[-0.6,0.6,0]],"triangles":[[0,1,2],[0,2,3]],"thickness_m":0.2}}])).unwrap(), ..Document::default() };
    d.textures.push(TextureAsset {
        id: "original".into(),
        label: "must survive".into(),
        color_space: ColorSpace::Linear,
        data: EmbeddedPng::from_bytes(bytes).unwrap(),
    });
    // Deliberately discontinuous UVs are irrelevant to an everywhere-clamped map.
    textures::put_uvs(&mut d,serde_json::from_value(json!({"id":"uv","object":"sheet","values":[[0,0],[1,0],[1,1],[10,10],[11,11],[10,11]],"corner_indices":[[0,1,2],[3,4,5]]})).unwrap()).unwrap();
    d.texture_bindings = serde_json::from_value(
        json!([{"object":"sheet","uv_set":"uv","roughness":{"texture":"original","channel":"r"}}]),
    )
    .unwrap();
    let r=serde_json::from_value(json!({"path":"sheet.usda","object_ids":["sheet"],"bounds_min":[-1,-1,-0.5],"bounds_max":[1,1,0.5],"resolution":[24,24,24],"max_surface_error_m":0.08,"max_field_residual":0.04,"texture_delivery":{"filtering":"reader_defined","max_refinement_passes":0}})).unwrap();
    (d, r)
}
#[test]
fn constant_map_bundle_retains_source_and_installs_layer_with_verified_manifest() {
    let root = Root::new();
    let (d, r) = fixture();
    let before = serde_json::to_vec(&d).unwrap();
    let encoded = delivery::encode_with_assets(&d, &r).unwrap();
    assert!(delivery::encode(&d, &r).err().unwrap().contains("external assets"));
    assert!(!String::from_utf8_lossy(&encoded.bytes).contains("primvars:st"));
    assert!(String::from_utf8_lossy(&encoded.bytes).contains("float inputs:roughness = 0.04"));
    let value = delivery::export(&d, &root.0, &r).unwrap();
    assert_eq!(fs::read(root.0.join("sheet.usda")).unwrap(), encoded.bytes);
    let directory = PathBuf::from(value["asset_directory"].as_str().unwrap());
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(directory.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["layer_sha256"], format!("{:x}", Sha256::digest(&encoded.bytes)));
    assert_eq!(manifest["assets"].as_array().unwrap().len(), 1);
    let record = &manifest["assets"][0];
    let source = fs::read(directory.join(record["file"].as_str().unwrap())).unwrap();
    assert_eq!(source, d.textures[0].data.source_bytes());
    assert_eq!(record["sha256"], format!("{:x}", Sha256::digest(&source)));
    assert_eq!(record["bytes"], source.len());
    assert_eq!(serde_json::to_vec(&d).unwrap(), before);
    assert!(delivery::export(&d, &root.0, &r).is_err());
    assert_eq!(fs::read(root.0.join("sheet.usda")).unwrap(), encoded.bytes);
}
#[test]
fn bundle_conflicts_and_unsupported_requests_preserve_existing_files() {
    let root = Root::new();
    let (d, mut r) = fixture();
    r.texture_delivery = None;
    assert!(delivery::export(&d, &root.0, &r).err().unwrap().message.contains("explicit texture_delivery"));
    assert_eq!(fs::read_dir(&root.0).unwrap().count(), 0);
    let (_, r) = fixture();
    let encoded = delivery::encode_with_assets(&d, &r).unwrap();
    let asset = root.0.join(encoded.asset_directory.unwrap());
    fs::create_dir(&asset).unwrap();
    fs::write(asset.join("keep"), b"authored").unwrap();
    assert!(delivery::export(&d, &root.0, &r).is_err());
    assert!(!root.0.join("sheet.usda").exists());
    assert_eq!(fs::read(asset.join("keep")).unwrap(), b"authored");
    assert_eq!(fs::read_dir(asset).unwrap().count(), 1);
}
