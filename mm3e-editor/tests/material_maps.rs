use mm3e_editor::{
    protocol::{Request, Response},
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
        let p = std::env::temp_dir().join(format!(
            "mm3e-material-maps-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn png(pixel: [u8; 4]) -> Vec<u8> {
    let mut b = vec![];
    {
        let mut e = png::Encoder::new(&mut b, 1, 1);
        e.set_color(png::ColorType::Rgba);
        e.set_depth(png::BitDepth::Eight);
        let mut w = e.write_header().unwrap();
        w.write_image_data(&pixel).unwrap();
        w.finish().unwrap();
    }
    b
}
fn call(e: &mut Editor, c: Value, mutate: bool) -> Response {
    let r: Request =
        serde_json::from_value(json!({"id":"pbr","expected_revision":mutate.then(||e.revision()),"command":c}))
            .unwrap();
    e.handle(r)
}
fn ok(e: &mut Editor, c: Value, mutate: bool) -> Value {
    let r = call(e, c, mutate);
    assert!(r.ok, "{r:?}");
    r.result.unwrap()
}
fn setup(root: &Scratch) -> Editor {
    let mut e = Editor::new(&root.0).unwrap();
    for (id, rgba, space) in [
        ("packed", [64, 128, 192, 26], "linear"),
        ("normal", [204, 128, 230, 0], "linear"),
        ("glow", [128, 64, 255, 128], "linear"),
        ("color", [128, 64, 255, 255], "srgb"),
    ] {
        fs::write(root.0.join(format!("{id}.png")), png(rgba)).unwrap();
        ok(
            &mut e,
            json!({"op":"import_texture","request":{"id":id,"path":format!("{id}.png"),"color_space":space}}),
            true,
        );
    }
    ok(
        &mut e,
        json!({"op":"apply","operations":[{"op":"create","object":{"id":"panel","shape":{"type":"surface","vertices":[[0,0,0],[1,0,0],[0,1,0]],"triangles":[[0,1,2]],"thickness_m":0.02},"material":{"albedo":[0.4,0.5,0.6],"roughness":0.8,"metallic":0.7,"emissive":[4,8,16]}}},
        {"op":"project_uvs","request":{"id":"uv","object":"panel","origin":[0,0,0],"axis_u":[1,0,0],"axis_v":[0,1,0],"meters_per_uv":[1,1]}},
        {"op":"set_settings","settings":{"width":48,"height":48,"quality":"preview","shadows":false,"ao":false}},
        {"op":"set_camera","camera":{"eye":[0.3,0.3,2],"target":[0.3,0.3,0],"fov_degrees":30}}]}),
        true,
    );
    e
}
fn binding() -> Value {
    json!({"object":"panel","uv_set":"uv","roughness":{"texture":"packed","channel":"r"},"metallic":{"texture":"packed","channel":"a"},"emissive":{"texture":"glow"},"normal":{"texture":"normal"}})
}
fn bind(e: &mut Editor, binding: Value) {
    ok(e, json!({"op":"apply","operations":[{"op":"bind_texture","binding":binding}]}), true);
}
#[test]
fn no_albedo_map_required_and_packed_channels_remain_independent_of_alpha() {
    let root = Scratch::new();
    let mut e = setup(&root);
    bind(&mut e, binding());
    let state = ok(&mut e, json!({"op":"material_state","request":{"points":[[0.2,0.2,0.01]]}}), false);
    let s = &state["samples"][0];
    assert!((s["roughness"].as_f64().unwrap() - 0.8 * 64.0 / 255.0).abs() < 1e-6);
    assert!((s["metallic"].as_f64().unwrap() - 0.7 * 26.0 / 255.0).abs() < 1e-6);
    for (i, base) in [4.0, 8.0, 16.0].into_iter().enumerate() {
        let channel = [128.0, 64.0, 255.0][i];
        assert!((s["linear_emissive"][i].as_f64().unwrap() - base * channel / 255.0 * 128.0 / 255.0).abs() < 2e-6);
    }
    let normal = s["shading_normal"].as_array().unwrap();
    assert!(normal[0].as_f64().unwrap() > 0.59);
    assert!(normal[2].as_f64().unwrap() > 0.79);
    let data =
        ok(&mut e, json!({"op":"texture_state","request":{"id":"packed","as_data":true,"uv":[[0.5,0.5]]}}), false);
    assert!((data["samples"][0]["linear_channels"][0].as_f64().unwrap() - 64.0 / 255.0).abs() < 1e-7);
}
#[test]
fn invalid_map_roles_and_uvs_are_atomic_and_old_color_only_bindings_still_work() {
    let root = Scratch::new();
    let mut e = setup(&root);
    bind(&mut e, json!({"object":"panel","uv_set":"uv","texture":"color"}));
    let before = serde_json::to_vec(e.document()).unwrap();
    let rev = e.revision();
    let mut bad = binding();
    bad["normal"]["texture"] = json!("color");
    let mut empty = binding();
    for k in ["roughness", "metallic", "emissive", "normal"] {
        empty.as_object_mut().unwrap().remove(k);
    }
    let mut strength = binding();
    strength["normal"]["strength"] = json!(9);
    for b in [bad, empty, strength] {
        let r = call(&mut e, json!({"op":"apply","operations":[{"op":"bind_texture","binding":b}]}), true);
        assert!(!r.ok);
        assert_eq!(e.revision(), rev);
        assert_eq!(serde_json::to_vec(e.document()).unwrap(), before);
    }
    assert!(!call(&mut e, json!({"op":"texture_state","request":{"id":"color","as_data":true}}), false).ok);
    bind(&mut e, binding());
    let before = serde_json::to_vec(e.document()).unwrap();
    assert!(!call(&mut e,json!({"op":"apply","operations":[{"op":"put_uvs","request":{"id":"uv","object":"panel","values":[[0,0]],"corner_indices":[[0,0,0]]}}]}),true).ok);
    assert_eq!(serde_json::to_vec(e.document()).unwrap(), before);
}
#[test]
fn maps_and_pixels_reload_exactly_without_original_sources() {
    let root = Scratch::new();
    let mut e = setup(&root);
    bind(&mut e, binding());
    let before = serde_json::to_vec(e.document()).unwrap();
    ok(&mut e, json!({"op":"render","path":"before.png"}), false);
    ok(&mut e, json!({"op":"save","path":"project.json"}), false);
    for name in ["packed", "normal", "glow", "color"] {
        fs::remove_file(root.0.join(format!("{name}.png"))).unwrap();
    }
    drop(e);
    let mut reopened = Editor::new(&root.0).unwrap();
    ok(&mut reopened, json!({"op":"load","path":"project.json"}), true);
    assert_eq!(serde_json::to_vec(reopened.document()).unwrap(), before);
    ok(&mut reopened, json!({"op":"render","path":"after.png"}), false);
    assert_eq!(fs::read(root.0.join("before.png")).unwrap(), fs::read(root.0.join("after.png")).unwrap());
}
