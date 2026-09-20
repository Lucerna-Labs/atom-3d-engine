use mm3e_editor::{
    animation::{AnimationSample, Playback},
    model::{vec, Document, Pass},
    protocol::{Request, Response},
    textures::{self, ColorSpace, EmbeddedPng, TextureAsset},
    Editor,
};
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mm3e-textures-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn png(width: u32, height: u32, depth: png::BitDepth, bytes: &[u8]) -> Vec<u8> {
    let mut out = vec![];
    {
        let mut encoder = png::Encoder::new(&mut out, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(depth);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(bytes).unwrap();
        writer.finish().unwrap();
    }
    out
}
fn checker() -> Vec<u8> {
    png(2, 2, png::BitDepth::Eight, &[255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255])
}
fn call(editor: &mut Editor, command: Value, revision: Option<u64>) -> Response {
    editor.handle(
        serde_json::from_value::<Request>(json!({"id":"texture-test","expected_revision":revision,"command":command}))
            .unwrap(),
    )
}
fn ok(editor: &mut Editor, command: Value, mutation: bool) -> Value {
    let revision = mutation.then(|| editor.revision());
    let response = call(editor, command, revision);
    assert!(response.ok, "{response:?}");
    response.result.unwrap()
}
fn import() -> Value {
    json!({"op":"import_texture","request":{"id":"print","path":"source.png","color_space":"srgb"}})
}
fn quad() -> Value {
    json!({"id":"sheet","shape":{"type":"surface","vertices":[[0,0,0],[1,0,0],[1,1,0],[0,1,0]],"triangles":[[0,1,2],[0,2,3]],"thickness_m":0.02},"material":{"albedo":[1,1,1]}})
}
fn mapping() -> Value {
    json!({"op":"put_uvs","request":{"id":"sheet/uv","object":"sheet","values":[[0,0],[1,0],[1,1],[0,1]],"corner_indices":[[0,1,2],[0,2,3]]}})
}
fn binding() -> Value {
    json!({"op":"bind_texture","binding":{"object":"sheet","uv_set":"sheet/uv","texture":"print","sampler":{"u":"clamp","v":"clamp","filter":"nearest"}}})
}
fn setup(root: &Scratch) -> Editor {
    fs::write(root.0.join("source.png"), checker()).unwrap();
    let mut editor = Editor::new(&root.0).unwrap();
    ok(&mut editor, import(), true);
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"create","object":quad()},mapping(),binding(),
        {"op":"set_settings","settings":{"width":64,"height":64,"quality":"preview","shadows":false,"ao":false}},
        {"op":"set_camera","camera":{"eye":[0.5,0.5,2],"target":[0.5,0.5,0],"fov_degrees":40}}]}),
        true,
    );
    editor
}
fn at(time: f32) -> AnimationSample {
    AnimationSample { clip: "pose".into(), time, playback: Playback::Clamp }
}

#[test]
fn png_color_alpha_precision_and_native_original_bytes_round_trip() {
    let source = png(1, 1, png::BitDepth::Eight, &[128, 64, 32, 128]);
    let data = EmbeddedPng::from_bytes(source.clone()).unwrap();
    let mut document = Document {
        textures: vec![TextureAsset { id: "x".into(), label: String::new(), color_space: ColorSpace::Srgb, data }],
        ..Document::default()
    };
    let request = serde_json::from_value(json!({"id":"x","uv":[[0.5,0.5]]})).unwrap();
    let state = textures::inspect(&document, &request).unwrap();
    let rgba = state["samples"][0]["premultiplied_linear_rgba"].as_array().unwrap();
    for (i, n) in [128, 64, 32].into_iter().enumerate() {
        let s = f64::from(n) / 255.0;
        let linear = if s <= 0.04045 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) };
        assert!((rgba[i].as_f64().unwrap() - linear * 128.0 / 255.0).abs() < 1e-7);
    }
    let encoded = serde_json::to_vec(&document).unwrap();
    let loaded: Document = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(loaded.textures[0].data.source_bytes(), source);
    let samples = [257u16, 16384, 32768, 50000];
    let raw = samples.into_iter().flat_map(u16::to_be_bytes).collect::<Vec<_>>();
    document.textures[0].data = EmbeddedPng::from_bytes(png(1, 1, png::BitDepth::Sixteen, &raw)).unwrap();
    document.textures[0].color_space = ColorSpace::Linear;
    let state = textures::inspect(&document, &request).unwrap();
    for (i, n) in samples.into_iter().enumerate() {
        let expected = if i == 3 { f64::from(n) / 65535.0 } else { f64::from(n) / 65535.0 * 50000.0 / 65535.0 };
        assert!((state["samples"][0]["premultiplied_linear_rgba"][i].as_f64().unwrap() - expected).abs() < 1e-7);
    }
}

#[test]
fn palette_low_bit_grayscale_rgb_and_apng_policy_are_explicit() {
    let encode = |color, depth, data: &[u8], palette: bool, animated: bool| {
        let mut bytes = vec![];
        {
            let mut e = png::Encoder::new(&mut bytes, 2, 1);
            e.set_color(color);
            e.set_depth(depth);
            if palette {
                e.set_palette(vec![255, 0, 0, 0, 255, 0]);
                e.set_trns(vec![0, 255]);
            }
            if animated {
                e.set_animated(1, 0).unwrap();
            }
            let mut writer = e.write_header().unwrap();
            writer.write_image_data(data).unwrap();
            writer.finish().unwrap();
        }
        bytes
    };
    for (bytes, expected) in [
        (
            encode(png::ColorType::Indexed, png::BitDepth::One, &[0b0100_0000], true, false),
            [[0.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 1.0]],
        ),
        (
            encode(png::ColorType::Grayscale, png::BitDepth::Two, &[0b0001_0000], false, false),
            [[0.0, 0.0, 0.0, 1.0], [1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, 1.0]],
        ),
        (
            encode(png::ColorType::Rgb, png::BitDepth::Eight, &[255, 0, 0, 0, 255, 0], false, false),
            [[1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0]],
        ),
    ] {
        let d = Document {
            textures: vec![TextureAsset {
                id: "image".into(),
                label: String::new(),
                color_space: ColorSpace::Linear,
                data: EmbeddedPng::from_bytes(bytes).unwrap(),
            }],
            ..Document::default()
        };
        let state = textures::inspect(
            &d,
            &serde_json::from_value(json!({"id":"image","uv":[[0.25,0.5],[0.75,0.5]],"sampler":{"filter":"nearest"}}))
                .unwrap(),
        )
        .unwrap();
        for (i, expected) in expected.into_iter().enumerate() {
            for (c, value) in expected.into_iter().enumerate() {
                assert!((state["samples"][i]["premultiplied_linear_rgba"][c].as_f64().unwrap() - value).abs() < 1e-7);
            }
        }
    }
    assert!(EmbeddedPng::from_bytes(encode(
        png::ColorType::Rgb,
        png::BitDepth::Eight,
        &[255, 0, 0, 0, 255, 0],
        false,
        true
    ))
    .unwrap_err()
    .contains("animated"));
    let mut broken = checker();
    let last = broken.len() - 1;
    broken[last] ^= 1;
    assert!(EmbeddedPng::from_bytes(broken).is_err());
}

#[test]
fn import_dry_run_preserves_state_and_saved_texture_renders_without_source_file() {
    let root = Scratch::new();
    fs::write(root.0.join("source.png"), checker()).unwrap();
    let mut editor = Editor::new(&root.0).unwrap();
    assert!(!call(&mut editor, import(), None).ok);
    let mut dry = import();
    dry["dry_run"] = json!(true);
    let preview = ok(&mut editor, dry, true);
    assert_eq!(preview["committed"], false);
    assert_eq!(editor.revision(), 0);
    assert!(editor.document().textures.is_empty());
    drop(editor);
    let mut editor = setup(&root);
    ok(&mut editor, json!({"op":"render","path":"before.png"}), false);
    let before = serde_json::to_vec(editor.document()).unwrap();
    let material = ok(&mut editor, json!({"op":"material_state","request":{"points":[[0.3,0.2,0.01]]}}), false);
    assert_eq!(material["samples"][0]["object"], "sheet");
    assert_eq!(material["samples"][0]["linear_albedo"], json!([0.0, 0.0, 1.0]));
    ok(&mut editor, json!({"op":"save","path":"project.json"}), false);
    fs::remove_file(root.0.join("source.png")).unwrap();
    drop(editor);
    let mut reopened = Editor::new(&root.0).unwrap();
    ok(&mut reopened, json!({"op":"load","path":"project.json"}), true);
    assert_eq!(serde_json::to_vec(reopened.document()).unwrap(), before);
    ok(&mut reopened, json!({"op":"render","path":"after.png"}), false);
    assert_eq!(fs::read(root.0.join("before.png")).unwrap(), fs::read(root.0.join("after.png")).unwrap());
    ok(&mut reopened, json!({"op":"export_texture","request":{"id":"print","path":"recovered.png"}}), false);
    assert_eq!(fs::read(root.0.join("recovered.png")).unwrap(), checker());
}

#[test]
fn texture_tracks_actual_joint_and_morph_deformation_with_immutable_uvs() {
    let root = Scratch::new();
    let mut editor = setup(&root);
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"set_joints","joints":[{"id":"root","pivot":[0,0,0]}]},
        {"op":"bind_surface","request":{"deformer":{"id":"skin","object":"sheet","method":"dual_quaternion","joints":["root"],"weights":[[{"joint":0,"weight":1}],[{"joint":0,"weight":1}],[{"joint":0,"weight":1}],[{"joint":0,"weight":1}]],"blendshapes":[{"id":"lift","deltas":[[0,0,0.2],[0,0,0.2],[0,0,0.2],[0,0,0.2]]}]}}},
        {"op":"put_clip","clip":{"id":"pose","duration":1,"tracks":[{"target":{"type":"joint","id":"root"},"keys":[{"time":0},{"time":1,"translation":[0.3,0.4,0.2],"rotation_degrees":[0,0,75]}]}],"morph_tracks":[{"deformer":"skin","blendshape":"lift","keys":[{"time":0,"weight":0},{"time":1,"weight":1}]}]}}]}),
        true,
    );
    let uv = serde_json::to_vec(&editor.document().uv_sets).unwrap();
    let source = serde_json::to_vec(editor.document()).unwrap();
    for time in [0.0, 0.5, 1.0] {
        let state = mm3e_editor::deform::inspect(editor.document(), "skin", Some(&at(time))).unwrap();
        let positions: Vec<[f32; 3]> = serde_json::from_value(state["vertices"].clone()).unwrap();
        let point = vec(positions[0]) * 0.7 + vec(positions[1]) * 0.1 + vec(positions[2]) * 0.2;
        let (scene, _) = editor.document().compile_at(&Pass::Beauty, Some(&at(time))).unwrap();
        let material = scene.sample_material(point, 0.0).unwrap();
        let uv = material.uv.unwrap();
        assert!((uv[0] - 0.3).abs() < 1e-6 && (uv[1] - 0.2).abs() < 1e-6);
        assert_eq!(material.material.albedo, mm3e_kit::Vec3::new(0.0, 0.0, 1.0));
    }
    assert_eq!(serde_json::to_vec(&editor.document().uv_sets).unwrap(), uv);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), source);
}

#[test]
fn invalid_references_topology_and_input_preserve_revision_history_and_document() {
    let root = Scratch::new();
    let mut editor = setup(&root);
    fs::write(root.0.join("bad.png"), [1, 2, 3]).unwrap();
    let before = serde_json::to_vec(editor.document()).unwrap();
    let revision = editor.revision();
    let mut bad_mapping = mapping();
    bad_mapping["request"]["corner_indices"] = json!([[0, 1, 999], [0, 2, 3]]);
    let mut bad_binding = binding();
    bad_binding["binding"]["sampler"]["lod_bias"] = json!(17);
    let mut shape = quad()["shape"].clone();
    shape["triangles"] = json!([[0, 2, 1], [0, 3, 2]]);
    for command in [
        import(),
        json!({"op":"import_texture","request":{"id":"print","path":"bad.png","color_space":"linear","replace":true}}),
        json!({"op":"apply","operations":[{"op":"remove_texture","id":"print"}]}),
        json!({"op":"apply","operations":[{"op":"remove_uvs","id":"sheet/uv"}]}),
        json!({"op":"apply","operations":[bad_mapping]}),
        json!({"op":"apply","operations":[bad_binding]}),
        json!({"op":"apply","operations":[{"op":"update","id":"sheet","patch":{"shape":shape}}]}),
    ] {
        assert!(!call(&mut editor, command, Some(revision)).ok);
        assert_eq!(editor.revision(), revision);
        assert_eq!(serde_json::to_vec(editor.document()).unwrap(), before);
    }
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"unbind_texture","object":"sheet"},{"op":"remove_uvs","id":"sheet/uv"},{"op":"remove_texture","id":"print"}]}),
        true,
    );
    ok(&mut editor, json!({"op":"undo"}), true);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), before);
}

#[test]
fn cloth_uvs_follow_cached_motion_and_visual_edits_preserve_physical_cache() {
    let root = Scratch::new();
    fs::write(root.0.join("source.png"), checker()).unwrap();
    let mut editor = Editor::new(&root.0).unwrap();
    ok(&mut editor, import(), true);
    ok(
        &mut editor,
        json!({"op":"apply","operations":[
            {"op":"create","object":{"id":"floor","shape":{"type":"plane","normal":[0,1,0],"offset":-0.85}}},
            {"op":"create","object":{"id":"anchor","position":[0,1,-0.4],"shape":{"type":"box","half_extents":[0.45,0.03,0.03]}}},
            {"op":"put_clip","clip":{"id":"move","duration":1,"tracks":[{"target":{"type":"object","id":"anchor"},"keys":[{"time":0},{"time":1,"translation":[0.15,0.1,0]}]}]}},
            {"op":"create_cloth_panel","request":{"id":"cape","origin":[0,1,0],"axis_u":[1,0,0],"axis_v":[0,0,1],"segments":[4,4],"width_m":0.8,"height_m":0.8,"thickness_m":0.004,"vertex_mass_kg":0.02,
                "pins":[{"vertex":0,"target_object":"anchor","point":[-0.4,0,0]},{"vertex":4,"target_object":"anchor","point":[0.4,0,0]}],"collision_object_ids":["floor"],"settings":{"iterations":24,"bend_compliance":0.05},"material":{"albedo":[1,1,1]}}}
        ]}),
        true,
    );
    ok(&mut editor, json!({"op":"bake_cloth","request":{"id":"cape","clip":"move"}}), true);
    let cache = serde_json::to_vec(&editor.document().cloths[0].cache).unwrap();
    let vertices = editor.document().cloths[0].rest_vertices.clone();
    let triangles = editor.document().cloths[0].triangles.clone();
    let values =
        vertices.iter().map(|p| [(f64::from(p[0]) + 0.4) / 0.8, (f64::from(p[2]) + 0.4) / 0.8]).collect::<Vec<_>>();
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"put_uvs","request":{"id":"cape/uv","object":"cape","values":values,"corner_indices":triangles}},
        {"op":"bind_texture","binding":{"object":"cape","uv_set":"cape/uv","texture":"print","sampler":{"u":"clamp","v":"clamp","filter":"nearest"}}}]}),
        true,
    );
    assert_eq!(serde_json::to_vec(&editor.document().cloths[0].cache).unwrap(), cache);
    assert_eq!(mm3e_editor::cloth::inspect(editor.document(), "cape", None).unwrap()["cache"]["fresh"], true);
    for time in [0.0, 0.5, 1.0] {
        let animation = AnimationSample { clip: "move".into(), time, playback: Playback::Clamp };
        let state = mm3e_editor::cloth::inspect(editor.document(), "cape", Some(&animation)).unwrap();
        let positions: Vec<[f32; 3]> = serde_json::from_value(state["vertices"].clone()).unwrap();
        let [a, b, c] = triangles[0].map(|i| vec(positions[i as usize]));
        let point = a * 0.3 + b * 0.3 + c * 0.4;
        let (scene, _) = editor.document().compile_at(&Pass::Beauty, Some(&animation)).unwrap();
        let material = scene.sample_material(point, 0.0).unwrap();
        assert_eq!(material.object, Some(2));
        let expected = std::array::from_fn::<_, 2, _>(|axis| {
            values[triangles[0][0] as usize][axis] * 0.3
                + values[triangles[0][1] as usize][axis] * 0.3
                + values[triangles[0][2] as usize][axis] * 0.4
        });
        for (a, b) in material.uv.unwrap().into_iter().zip(expected) {
            assert!((a - b).abs() < 2e-5);
        }
    }
    fs::write(root.0.join("source.png"), png(1, 1, png::BitDepth::Eight, &[255, 255, 0, 255])).unwrap();
    let mut replace = import();
    replace["request"]["replace"] = json!(true);
    ok(&mut editor, replace, true);
    assert_eq!(serde_json::to_vec(&editor.document().cloths[0].cache).unwrap(), cache);
    assert_eq!(mm3e_editor::cloth::inspect(editor.document(), "cape", None).unwrap()["cache"]["fresh"], true);
    ok(&mut editor, json!({"op":"undo"}), true);
    assert_eq!(editor.document().textures[0].data.source_bytes(), checker());
}

#[test]
fn planar_projection_and_scoped_uv_inspection_preserve_geometry_and_revision() {
    let root = Scratch::new();
    let mut editor = setup(&root);
    let objects = serde_json::to_vec(&editor.document().objects).unwrap();
    let projection = json!({"op":"project_uvs","request":{"id":"sheet/uv","object":"sheet","origin":[0,0,0],"axis_u":[1,0,0],"axis_v":[0,1,0],"meters_per_uv":[2,4],"offset":[0.125,0.25]}});
    ok(&mut editor, json!({"op":"apply","operations":[projection]}), true);
    let revision = editor.revision();
    let state = ok(&mut editor, json!({"op":"uv_state","request":{"id":"sheet/uv","triangles":[0]}}), false);
    assert_eq!(state["triangles"][0]["values"], json!([[0.125, 0.25], [0.625, 0.25], [0.625, 0.5]]));
    assert_eq!(editor.revision(), revision);
    assert_eq!(serde_json::to_vec(&editor.document().objects).unwrap(), objects);
    let before = serde_json::to_vec(editor.document()).unwrap();
    let mut invalid = projection;
    invalid["request"]["axis_v"] = json!([1, 0, 0]);
    assert!(!call(&mut editor, json!({"op":"apply","operations":[invalid]}), Some(revision)).ok);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), before);
}

#[test]
fn runtime_texture_failure_produces_no_file_and_preserves_the_authored_document() {
    let root = Scratch::new();
    fs::write(root.0.join("source.png"), checker()).unwrap();
    let mut editor = Editor::new(&root.0).unwrap();
    ok(&mut editor, import(), true);
    let tiny = 1e-38_f32;
    ok(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"create","object":{"id":"tiny","shape":{"type":"surface","vertices":[[tiny,0,0],[tiny,tiny,0],[tiny,0,tiny]],"triangles":[[0,1,2]],"thickness_m":0.02}}},
            {"op":"put_uvs","request":{"id":"tiny/uv","object":"tiny","values":[[0,0],[1,0],[0,1]],"corner_indices":[[0,1,2]]}},
            {"op":"bind_texture","binding":{"object":"tiny","uv_set":"tiny/uv","texture":"print"}},
            {"op":"set_settings","settings":{"width":1,"height":1,"quality":"preview","shadows":false,"ao":false}},
            {"op":"set_camera","camera":{"eye":[0.1,tiny*0.25,tiny*0.25],"target":[tiny,tiny*0.25,tiny*0.25],"fov_degrees":40}}
        ]}),
        true,
    );
    let before = serde_json::to_vec(editor.document()).unwrap();
    let revision = editor.revision();
    let response = call(&mut editor, json!({"op":"render","path":"must-not-exist.png"}), None);
    assert!(!response.ok);
    assert!(response.error.unwrap().message.contains("reconstruct"));
    assert!(!root.0.join("must-not-exist.png").exists());
    assert_eq!(editor.revision(), revision);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), before);
}
