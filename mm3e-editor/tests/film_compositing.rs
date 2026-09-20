use exr::prelude::*;
use mm3e_editor::film::{encode_compositing_exr, encode_compositing_exr_with_metadata, encode_rgba_exr_with_metadata};
use mm3e_kit::vec::Vec3;
use mm3e_orchestrator::film::FilmFrame;
use std::{collections::BTreeMap, io::Cursor};

fn frame() -> FilmFrame {
    FilmFrame {
        width: 3,
        height: 2,
        beauty: vec![
            Vec3::new(0.1, 0.2, 0.3),
            Vec3::new(4.0, 8.0, 16.0),
            Vec3::new(2.125, -0.0625, 0.25),
            Vec3::new(8.0, 16.0, 32.0),
            Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(1.0, 0.5, -0.0),
        ],
        foreground: vec![
            Vec3::ZERO,
            Vec3::new(4.0, 8.0, 16.0),
            Vec3::new(2.0, -0.125, 0.125),
            Vec3::new(8.0, 16.0, 32.0),
            Vec3::new(2.0, 2.5, 3.0),
            Vec3::new(1.0, 0.5, -0.0),
        ],
        alpha: vec![0.0, 1.0, 0.25, 1.0, 0.5, 1.0],
        depth: vec![f32::INFINITY, 2.0, f32::INFINITY, 0.125, 512.0, 0.0],
        normals: vec![
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::ZERO,
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        ],
        material_ids: vec![u32::MAX, 0, u32::MAX, 16_777_217, u32::MAX - 1, 7],
    }
}

fn decoded(bytes: Vec<u8>) -> FlatImage {
    read()
        .no_deep_data()
        .largest_resolution_level()
        .all_channels()
        .all_layers()
        .all_attributes()
        .non_parallel()
        .from_buffered(Cursor::new(bytes))
        .unwrap()
}

#[test]
fn compositing_exr_preserves_premultiplied_hdr_coverage_and_exact_unfiltered_data() {
    let source = frame();
    for transparent in [false, true] {
        let image = decoded(encode_compositing_exr(&source, transparent).unwrap());
        assert_eq!(image.layer_data.len(), 1);
        let layer = &image.layer_data[0];
        assert_eq!(layer.size, Vec2(3, 2));
        assert_eq!(
            layer.channel_data.list.iter().map(|c| c.name.to_string()).collect::<Vec<_>>(),
            ["A", "B", "G", "N.X", "N.Y", "N.Z", "R", "Z", "material.ID"]
        );
        let rgb = if transparent { &source.foreground } else { &source.beauty };
        for channel in &layer.channel_data.list {
            let name = channel.name.to_string();
            if name == "material.ID" {
                assert_eq!(channel.sample_data, FlatSamples::U32(source.material_ids.clone()));
                continue;
            }
            let expected: Vec<f32> = match name.as_str() {
                "R" => rgb.iter().map(|p| p.x).collect(),
                "G" => rgb.iter().map(|p| p.y).collect(),
                "B" => rgb.iter().map(|p| p.z).collect(),
                "A" if transparent => source.alpha.clone(),
                "A" => vec![1.0; 6],
                "Z" => source.depth.clone(),
                "N.X" => source.normals.iter().map(|p| p.x).collect(),
                "N.Y" => source.normals.iter().map(|p| p.y).collect(),
                "N.Z" => source.normals.iter().map(|p| p.z).collect(),
                _ => panic!("unexpected EXR channel {name}"),
            };
            let FlatSamples::F32(actual) = &channel.sample_data else { panic!("{name} must be lossless f32") };
            assert_eq!(
                actual.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                "{name} changed when transparent={transparent}"
            );
        }
        let chroma = image.attributes.chromaticities.unwrap();
        assert_eq!(chroma.red, Vec2(0.64, 0.33));
        assert_eq!(chroma.green, Vec2(0.30, 0.60));
        assert_eq!(chroma.blue, Vec2(0.15, 0.06));
        assert_eq!(chroma.white, Vec2(0.3127, 0.3290));
        let attribute = |name: &str| match &layer.attributes.other[&Text::from(name)] {
            AttributeValue::Text(text) => text.to_string(),
            _ => panic!("{name} must be text"),
        };
        assert_eq!(attribute("transferFunction"), "linear");
        assert!(attribute("mm3e:alphaSemantics").contains(if transparent { "premultiplied" } else { "opaque" }));
        assert!(attribute("mm3e:depthSemantics").contains("positive infinity"));
        assert!(attribute("mm3e:normalSemantics").contains("world-space"));
        assert!(attribute("mm3e:materialIdSemantics").contains("not Cryptomatte"));
        assert!(attribute("mm3e:aovSampling").contains("unfiltered pixel-center pinhole"));
    }
}

#[test]
fn compositing_exr_rejects_malformed_dimensions_and_every_truncated_buffer() {
    for (width, height) in [(0, 2), (3, 0), (u32::MAX, 2), (3, u32::MAX), (2, 2)] {
        let mut source = frame();
        source.width = width;
        source.height = height;
        assert!(encode_compositing_exr(&source, true).is_err());
    }
    let truncate: [fn(&mut FilmFrame); 6] = [
        |f| {
            f.beauty.pop();
        },
        |f| {
            f.foreground.pop();
        },
        |f| {
            f.alpha.pop();
        },
        |f| {
            f.depth.pop();
        },
        |f| {
            f.normals.pop();
        },
        |f| {
            f.material_ids.pop();
        },
    ];
    for (mutate, name) in truncate.iter().zip(["beauty", "foreground", "alpha", "depth", "normals", "material_ids"]) {
        let mut source = frame();
        mutate(&mut source);
        for transparent in [false, true] {
            assert!(encode_compositing_exr(&source, transparent).unwrap_err().contains(name));
        }
    }
}

#[test]
fn compositing_exr_rejects_nonfinite_colors_normals_invalid_coverage_and_depth() {
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        for buffer in 0..3 {
            for component in 0..3 {
                let mut source = frame();
                let point = match buffer {
                    0 => &mut source.beauty[2],
                    1 => &mut source.foreground[2],
                    _ => &mut source.normals[2],
                };
                match component {
                    0 => point.x = invalid,
                    1 => point.y = invalid,
                    _ => point.z = invalid,
                }
                for transparent in [false, true] {
                    assert!(encode_compositing_exr(&source, transparent).is_err());
                }
            }
        }
    }
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -0.01, 1.01] {
        let mut source = frame();
        source.alpha[1] = invalid;
        assert!(encode_compositing_exr(&source, true).unwrap_err().contains("alpha"));
    }
    for invalid in [f32::NAN, f32::NEG_INFINITY, -2.0] {
        let mut source = frame();
        source.depth[1] = invalid;
        assert!(encode_compositing_exr(&source, true).unwrap_err().contains("depth"));
    }
}

#[test]
fn camera_origin_inside_geometry_has_valid_zero_exr_depth() {
    use mm3e_kit::{Camera, Transform};
    use mm3e_orchestrator::{render_film_with_threads, Lens, Object, Prim, Scene};
    let mut scene = Scene::new(1, 1);
    scene.aa = 1;
    scene.bounces = 0;
    scene.shadows = false;
    scene.ao = false;
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::IDENTITY, 0));
    let camera = Camera::look_at(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 0.8);
    let frame = render_film_with_threads(&scene, &camera, &Lens::default(), 1);
    assert_eq!(frame.depth, [0.0]);
    let image = decoded(encode_compositing_exr(&frame, true).unwrap());
    let layer = &image.layer_data[0];
    let depth = layer.channel_data.list.iter().find(|c| c.name.as_slice() == b"Z").unwrap();
    assert_eq!(depth.sample_data, FlatSamples::F32(vec![0.0]));
    let semantics = &layer.attributes.other[&Text::from("mm3e:depthSemantics")];
    assert!(
        matches!(semantics, AttributeValue::Text(value) if value.to_string().contains("zero for camera origin inside/on geometry"))
    );
}

#[test]
fn rgba_only_output_omits_data_channels_and_claims_and_validates_only_required_buffers() {
    for transparent in [false, true] {
        let mut source = frame();
        source.depth = vec![f32::NAN];
        source.normals = vec![Vec3::splat(f32::INFINITY)];
        source.material_ids.clear();
        if transparent {
            source.beauty.clear();
        } else {
            source.foreground.clear();
            source.alpha = vec![f32::NAN];
        }
        let metadata = BTreeMap::from([("mm3e:shutter".into(), "{\"samples\":4}".into())]);
        let image = decoded(encode_rgba_exr_with_metadata(&source, transparent, &metadata).unwrap());
        let layer = &image.layer_data[0];
        assert_eq!(
            layer.channel_data.list.iter().map(|c| c.name.to_string()).collect::<Vec<_>>(),
            ["A", "B", "G", "R"]
        );
        for key in ["mm3e:depthSemantics", "mm3e:normalSemantics", "mm3e:materialIdSemantics", "mm3e:aovSampling"] {
            assert!(!layer.attributes.other.contains_key(&Text::from(key)));
            let forbidden = BTreeMap::from([(key.into(), "unsupported claim".into())]);
            assert!(encode_rgba_exr_with_metadata(&source, transparent, &forbidden).is_err());
        }
        assert_eq!(
            layer.attributes.other[&Text::from("mm3e:shutter")],
            AttributeValue::Text(Text::from("{\"samples\":4}"))
        );
        let complete = decoded(encode_compositing_exr(&frame(), transparent).unwrap());
        for channel in &layer.channel_data.list {
            let reference = complete.layer_data[0].channel_data.list.iter().find(|c| c.name == channel.name).unwrap();
            assert_eq!(channel.sample_data, reference.sample_data);
        }
        if transparent {
            source.alpha.clear();
        } else {
            source.beauty.clear();
        }
        assert!(encode_rgba_exr_with_metadata(&source, transparent, &metadata).is_err());
    }
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut source = frame();
        source.foreground[0].x = invalid;
        assert!(encode_rgba_exr_with_metadata(&source, true, &BTreeMap::new()).is_err());
        source.beauty[0].y = invalid;
        assert!(encode_rgba_exr_with_metadata(&source, false, &BTreeMap::new()).is_err());
    }
}

#[test]
fn compositing_exr_preserves_utf8_provenance_and_blocks_metadata_overrides_and_oversized_headers() {
    let source = frame();
    let text = "{\"0\":\"Étoile 🦊\"}";
    let metadata =
        BTreeMap::from([("mm3e:materialMap".into(), text.into()), ("mm3e:referenceTime".into(), "1.25".into())]);
    let image = decoded(encode_compositing_exr_with_metadata(&source, true, &metadata).unwrap());
    assert_eq!(
        image.layer_data[0].attributes.other[&Text::from("mm3e:materialMap")],
        AttributeValue::Text(Text::from_slice_unchecked(text.as_bytes()))
    );
    for key in [
        "R",
        "channels",
        "chromaticities",
        "colorSpace",
        "transferFunction",
        "software",
        "mm3e:",
        "mm3e:alphaSemantics",
        "mm3e:depthSemantics",
        "mm3e:normalSemantics",
        "mm3e:materialIdSemantics",
        "mm3e:aovSampling",
        "mm3e:a\0b",
        "mm3e:space space",
        "mm3e:é",
    ] {
        let metadata = BTreeMap::from([(key.to_owned(), "invalid override".into())]);
        assert!(encode_compositing_exr_with_metadata(&source, true, &metadata).is_err(), "accepted {key}");
    }
    for (key, value) in [
        (format!("mm3e:{}", "a".repeat(251)), "v".into()),
        ("mm3e:shutter".into(), "a".repeat(4097)),
        ("mm3e:materialMap".into(), "a".repeat(128 * 1024 + 1)),
    ] {
        let metadata = BTreeMap::from([(key, value)]);
        assert!(encode_compositing_exr_with_metadata(&source, true, &metadata).is_err());
    }
    let metadata = (0..65).map(|index| (format!("mm3e:key{index}"), String::new())).collect();
    assert!(encode_compositing_exr_with_metadata(&source, true, &metadata).is_err());
    let mut metadata: BTreeMap<_, _> = (0..40).map(|index| (format!("mm3e:key{index}"), "a".repeat(4096))).collect();
    metadata.insert("mm3e:materialMap".into(), "a".repeat(128 * 1024));
    assert!(encode_compositing_exr_with_metadata(&source, true, &metadata).unwrap_err().contains("256 KiB"));
}

#[test]
#[ignore = "requires independent Python OpenEXR and numpy; set PYTHONPATH for the decoder installation"]
fn independent_openexr_decoder_recovers_compositing_channels_and_utf8_metadata() {
    let output = std::env::temp_dir().join(format!(
        "mm3e-compositing-reference-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir(&output).unwrap();
    let metadata = BTreeMap::from([("mm3e:materialMap".into(), "{\"0\":\"Étoile 🦊\"}".into())]);
    for (transparent, name) in [(true, "transparent.exr"), (false, "opaque.exr")] {
        let bytes = encode_compositing_exr_with_metadata(&frame(), transparent, &metadata).unwrap();
        std::fs::write(output.join(name), bytes).unwrap();
    }
    for (transparent, name) in [(true, "transparent-rgba.exr"), (false, "opaque-rgba.exr")] {
        let bytes = encode_rgba_exr_with_metadata(&frame(), transparent, &metadata).unwrap();
        std::fs::write(output.join(name), bytes).unwrap();
    }
    let script = r#"
import sys, json
from pathlib import Path
import OpenEXR
import numpy as np
root = Path(sys.argv[1])
foreground = np.array([[0,0,0],[4,8,16],[2,-0.125,0.125],[8,16,32],[2,2.5,3],[1,0.5,-0.0]], dtype=np.float32).reshape(2,3,3)
beauty = np.array([[0.1,0.2,0.3],[4,8,16],[2.125,-0.0625,0.25],[8,16,32],[4,5,6],[1,0.5,-0.0]], dtype=np.float32).reshape(2,3,3)
alpha = np.array([0,1,0.25,1,0.5,1],dtype=np.float32).reshape(2,3)
depth = np.array([np.inf,2,np.inf,0.125,512,0],dtype=np.float32).reshape(2,3)
normals = np.array([[0,0,0],[0,0,1],[0,0,0],[-1,0,0],[0,1,0],[0,0,-1]],dtype=np.float32).reshape(2,3,3)
ids = np.array([4294967295,0,4294967295,16777217,4294967294,7],dtype=np.uint32).reshape(2,3)
for transparent, data, name in [(True,True,'transparent.exr'),(False,True,'opaque.exr'),(True,False,'transparent-rgba.exr'),(False,False,'opaque-rgba.exr')]:
    with OpenEXR.File(str(root/name),separate_channels=True) as image:
        assert len(image.parts)==1
        header, channels = image.header(), image.channels()
        assert set(channels)==({'R','G','B','A','Z','N.X','N.Y','N.Z','material.ID'} if data else {'R','G','B','A'})
        expected = dict(zip(('R','G','B'), np.moveaxis(foreground if transparent else beauty,2,0)))
        expected.update({'A':alpha if transparent else np.ones((2,3),dtype=np.float32)})
        if data:
            expected.update(dict(zip(('N.X','N.Y','N.Z'),np.moveaxis(normals,2,0))))
            expected.update({'Z':depth,'material.ID':ids})
        for key,value in expected.items():
            actual=channels[key].pixels
            assert actual.shape==value.shape and actual.dtype==value.dtype, (key,actual.dtype)
            assert np.array_equal(actual.view(np.uint32),value.view(np.uint32)), key
        assert header['transferFunction']=='linear'
        assert header['colorSpace']=='Linear Rec.709 (sRGB primaries), D65'
        assert json.loads(header['mm3e:materialMap'])=={'0':'Étoile 🦊'}
        if data:
            assert 'unfiltered pixel-center pinhole' in header['mm3e:aovSampling']
            assert 'zero for camera origin inside/on geometry' in header['mm3e:depthSemantics']
        else:
            assert not {'mm3e:aovSampling','mm3e:depthSemantics','mm3e:normalSemantics','mm3e:materialIdSemantics'} & set(header)
        assert np.allclose(header['chromaticities'],[0.64,0.33,0.30,0.60,0.15,0.06,0.3127,0.3290])
        assert np.array_equal(header['dataWindow'][0],[0,0])
        assert np.array_equal(header['dataWindow'][1],[2,1])
print(json.dumps({'passed':True,'decoder':'OpenEXR '+OpenEXR.__version__,'files':4,'checks':['lossless nine typed channels or RGBA-only','RGBA-only omits data semantic claims','premultiplied HDR RGB','opaque environment and alpha','fractional coverage','zero origin-inside and positive infinite background depth','signed world normals','exact uint32 IDs above 2^24','UTF-8 material map','linear Rec709 chromaticities','row-major orientation']}))
"#;
    let result = std::process::Command::new("python3").arg("-c").arg(script).arg(&output).output().unwrap();
    std::fs::write(output.join("stdout.json"), &result.stdout).unwrap();
    std::fs::write(output.join("stderr.txt"), &result.stderr).unwrap();
    assert!(
        result.status.success(),
        "independent decode failed; artifacts at {}: {}",
        output.display(),
        String::from_utf8_lossy(&result.stderr)
    );
    println!("Independent decoder evidence: {}\n{}", output.display(), String::from_utf8_lossy(&result.stdout));
}
