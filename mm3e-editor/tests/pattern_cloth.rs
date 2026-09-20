use mm3e_editor::{
    animation::{AnimationSample, Playback},
    cloth::{self, BakeClothRequest},
    model::{Document, Pass},
    pattern::{self, PatternClothRequest, PatternPanel},
    protocol::Request,
    Editor,
};
use serde_json::{json, Value};

fn boundary(id: &str, points: &[[f64; 2]]) -> Value {
    json!({"id":id,"points":points.iter().enumerate().map(|(i,p)|json!({"id":format!("{id}-{i}"),"position":p})).collect::<Vec<_>>()})
}
fn panel(id: &str, left: bool, hole: bool) -> PatternPanel {
    let outer = if left {
        vec![[-0.4, 0.0], [0.0, 0.0], [0.0, 0.6], [-0.4, 0.6]]
    } else {
        vec![[0.0, 0.0], [0.4, 0.0], [0.4, 0.6], [0.0, 0.6]]
    };
    let holes = if hole {
        vec![boundary("arm", &[[-0.30, 0.30], [-0.16, 0.30], [-0.16, 0.46], [-0.30, 0.46]])]
    } else {
        vec![]
    };
    serde_json::from_value(json!({"id":id,"origin":[0,1,0],"axis_u":[1,0,0],"axis_v":[0,1,0],
        "outer":boundary("outer",&outer),"holes":holes,"max_edge_m":0.2,
        "pins":if left {json!([
            {"vertex":{"type":"control","id":"outer-2"},"target_object":"anchor","point":[0,0,0]},
            {"vertex":{"type":"control","id":"outer-3"},"target_object":"anchor","point":[-0.4,0,0]}
        ])}else{json!([])}}))
    .unwrap()
}
fn seam_indices(preview: &Value) -> Vec<u32> {
    // This fixture explicitly selects the complete x=0 edge, bottom-to-top on BOTH panels.
    let mut points: Vec<_> = preview["points"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i, p)| (p[0].as_f64().unwrap().abs() < 1e-12).then_some((p[1].as_f64().unwrap(), i as u32)))
        .collect();
    points.sort_by(|a, b| a.0.total_cmp(&b.0));
    points.into_iter().map(|p| p.1).collect()
}
fn request(id: &str, sewn: bool) -> PatternClothRequest {
    let panels = vec![panel("left", true, true), panel("right", false, false)];
    let a = pattern::preview(&panels[0]).unwrap();
    let b = pattern::preview(&panels[1]).unwrap();
    serde_json::from_value(json!({"id":id,"panels":panels,
        "seams":if sewn{json!([{"panel_a":"left","chain_a":seam_indices(&a),"panel_b":"right","chain_b":seam_indices(&b)}])}else{json!([])},
        "thickness_m":0.002,"vertex_mass_kg":0.02,
        "settings":{"iterations":48,"substeps":4,"bend_compliance":0.02,"max_seam_error_m":0.001},
        "material":{"albedo":[0.7,0.1,0.08]}})).unwrap()
}
fn document() -> Document {
    Document {
        objects: serde_json::from_value(
            json!([{"id":"anchor","position":[0,1.6,0],"shape":{"type":"sphere","radius":0.02}}]),
        )
        .unwrap(),
        joints: serde_json::from_value(json!([{"id":"shoulder","pivot":[0,1.6,0],"objects":["anchor"]}])).unwrap(),
        clips: serde_json::from_value(json!([{"id":"move","duration":0.3,"tracks":[
            {"target":{"type":"joint","id":"shoulder"},"keys":[{"time":0},{"time":0.3,"rotation_degrees":[0,0,8]}]}
        ]}]))
        .unwrap(),
        settings: mm3e_editor::model::Settings {
            width: 40,
            height: 40,
            shadows: false,
            ao: false,
            ..Default::default()
        },
        ..Document::default()
    }
}
fn sample(time: f32) -> AnimationSample {
    AnimationSample { clip: "move".into(), time, playback: Playback::Clamp }
}
fn point(value: &Value, index: usize) -> mm3e_kit::Vec3 {
    mm3e_kit::Vec3::new(
        value["vertices"][index][0].as_f64().unwrap() as f32,
        value["vertices"][index][1].as_f64().unwrap() as f32,
        value["vertices"][index][2].as_f64().unwrap() as f32,
    )
}
fn seam_gap(value: &Value) -> f32 {
    value["stitch_pairs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| {
            (point(value, s["vertices"][0].as_u64().unwrap() as usize)
                - point(value, s["vertices"][1].as_u64().unwrap() as usize))
            .length()
        })
        .fold(0.0, f32::max)
}

#[test]
fn preview_retains_controls_named_holes_and_exact_material_area() {
    let panel = panel("front", true, true);
    let preview = pattern::preview(&panel).unwrap();
    assert_eq!(preview["boundary_loops"][0]["id"], "outer");
    assert_eq!(preview["boundary_loops"][1]["id"], "arm");
    for boundary in std::iter::once(&panel.outer).chain(&panel.holes) {
        for p in &boundary.points {
            let index = preview["control_vertices"][&p.id].as_u64().unwrap() as usize;
            assert_eq!(preview["points"][index], json!(p.position));
        }
    }
    let points: Vec<[f64; 2]> = serde_json::from_value(preview["points"].clone()).unwrap();
    let triangles: Vec<[u32; 3]> = serde_json::from_value(preview["triangles"].clone()).unwrap();
    let area: f64 = triangles
        .iter()
        .map(|t| {
            let [a, b, c] = t.map(|i| points[i as usize]);
            let area = ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])) * 0.5;
            assert!(area > 0.0);
            area
        })
        .sum();
    assert!((area - (0.4 * 0.6 - 0.14 * 0.16)).abs() < 1e-12);
    assert!(points.len() <= 256);
    for triangle in triangles {
        for [a, b] in [[triangle[0], triangle[1]], [triangle[1], triangle[2]], [triangle[2], triangle[0]]] {
            assert!(
                (points[a as usize][0] - points[b as usize][0]).hypot(points[a as usize][1] - points[b as usize][1])
                    <= 0.2 + 1e-12
            );
        }
    }
}

#[test]
fn sewn_pattern_bakes_native_mesh_while_unsewn_piece_falls_and_hole_stays_open() {
    let mut d = document();
    let r = request("garment", true);
    pattern::create(&mut d, &r).unwrap();
    let mut control = request("control", false);
    // Keep both assets at the same rest coordinates; compare their actual object fields.
    control.material.albedo = [0.1, 0.7, 0.1];
    pattern::create(&mut d, &control).unwrap();
    let source = serde_json::to_value(&d.objects).unwrap();
    let baked =
        cloth::bake(&mut d, &BakeClothRequest { id: "garment".into(), clip: "move".into(), duration: None }).unwrap();
    cloth::bake(&mut d, &BakeClothRequest { id: "control".into(), clip: "move".into(), duration: None }).unwrap();
    assert!(baked["diagnostics"]["seam_projections"].as_u64().unwrap() > 0);
    let sewn = cloth::inspect(&d, "garment", Some(&sample(0.3))).unwrap();
    let free = cloth::inspect(&d, "control", Some(&sample(0.3))).unwrap();
    assert!(seam_gap(&sewn) < 0.001);
    let left_count = d.cloths[0].pattern.as_ref().unwrap().meshes[0].points.len();
    assert!((point(&sewn, left_count) - point(&free, left_count)).length() > 0.1);
    let mesh = &d.cloths[0].pattern.as_ref().unwrap().meshes[0];
    let hole = &mesh.boundary_loops[1].vertices;
    let center = hole.iter().map(|i| point(&sewn, *i as usize)).fold(mm3e_kit::Vec3::ZERO, |a, b| a + b)
        * (1.0 / hole.len() as f32);
    let (scene, _) = d.compile_at(&Pass::Beauty, Some(&sample(0.3))).unwrap();
    assert!(scene.sample_object(1, center).unwrap().dist > 0.04, "hole got capped: {center:?}");
    for i in 0..d.cloths[0].rest_vertices.len() {
        assert!(scene.sample_object(1, point(&sewn, i)).unwrap().dist < 0.0);
    }
    assert_eq!(source, serde_json::to_value(&d.objects).unwrap());
    assert!(cloth::cache_fresh(&d, &d.cloths[0]).unwrap());
    d.clips[0].tracks[0].keys[1].rotation_degrees[2] = 10.0;
    assert!(!cloth::cache_fresh(&d, &d.cloths[0]).unwrap());
}

#[test]
fn invalid_outlines_density_seams_and_provenance_fail_atomically() {
    let mut d = document();
    pattern::create(&mut d, &request("garment", true)).unwrap();
    let before = serde_json::to_value(&d).unwrap();
    let mut bad = request("garment", true);
    bad.panels[0].max_edge_m = Some(0.001);
    assert!(pattern::update(&mut d, &bad).is_err());
    assert_eq!(serde_json::to_value(&d).unwrap(), before);
    let mut bad = request("garment", true);
    bad.panels[0].holes =
        (0..90).map(|i| mm3e_editor::pattern::PatternLoop { id: format!("empty-{i}"), points: vec![] }).collect();
    assert!(pattern::update(&mut d, &bad).unwrap_err().contains("loop or pin count"));
    assert_eq!(serde_json::to_value(&d).unwrap(), before);
    let mut bad = request("garment", true);
    bad.panels[0].holes[0].points[0].position = [0.2, 0.3];
    assert!(pattern::update(&mut d, &bad).is_err());
    let mut bad = request("garment", true);
    bad.panels[0].outer.points.swap(1, 2);
    assert!(pattern::update(&mut d, &bad).is_err());
    let mut bad = request("garment", true);
    bad.seams[0].chain_b.reverse();
    let mut reversed = d.clone();
    pattern::update(&mut reversed, &bad).unwrap();
    assert_eq!(reversed.cloths[0].pattern.as_ref().unwrap().seams, bad.seams);
    assert!(
        cloth::bake(&mut reversed, &BakeClothRequest { id: "garment".into(), clip: "move".into(), duration: None })
            .is_err(),
        "explicit reversed correspondence must not be guessed away to pass the initial seam tolerance"
    );
    assert_eq!(serde_json::to_value(&d).unwrap(), before);
    let mut corrupted = d.clone();
    corrupted.cloths[0].pattern.as_mut().unwrap().meshes[0].boundary_loops[1].vertices.reverse();
    assert!(corrupted.compile(&Pass::Beauty).is_err());
    let mut corrupted = d.clone();
    corrupted.cloths[0].pattern.as_mut().unwrap().meshes[0].points[0][0] += 0.01;
    assert!(corrupted.compile(&Pass::Beauty).is_err());
}

fn send(editor: &mut Editor, command: Value, okay: bool) -> Value {
    let request: Request =
        serde_json::from_value(json!({"id":"pattern","expected_revision":editor.revision(),"command":command}))
            .unwrap();
    let response = editor.handle(request);
    assert_eq!(response.ok, okay, "{response:?}");
    response.result.unwrap_or_else(|| serde_json::to_value(response.error).unwrap())
}

#[test]
fn pattern_protocol_update_undo_save_and_cold_reload_keep_native_topology() {
    let root = std::env::temp_dir().join(format!(
        "mm3e-pattern-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir(&root).unwrap();
    let mut editor = Editor::new(&root).unwrap();
    let d = document();
    let r = request("garment", true);
    let preview = send(&mut editor, json!({"op":"preview_pattern_panel","panel":r.panels[0]}), true);
    assert_eq!(editor.revision(), 0);
    send(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"create","object":d.objects[0]},
        {"op":"set_joints","joints":d.joints},{"op":"put_clip","clip":d.clips[0]},
        {"op":"create_pattern_cloth","request":r}]}),
        true,
    );
    assert_eq!(
        serde_json::to_value(&editor.document().cloths[0].pattern.as_ref().unwrap().meshes[0]).unwrap(),
        preview
    );
    send(&mut editor, json!({"op":"bake_cloth","request":{"id":"garment","clip":"move"}}), true);
    let original = serde_json::to_value(editor.document()).unwrap();
    let state = send(&mut editor, json!({"op":"cloth_state","id":"garment","animation":sample(0.3)}), true);
    let mut updated = r.clone();
    updated.panels[0].holes[0].points[0].position[0] -= 0.01;
    // Resolve seams from the unchanged perimeter preview explicitly after authoring edits.
    updated.seams[0].chain_a = seam_indices(&pattern::preview(&updated.panels[0]).unwrap());
    send(&mut editor, json!({"op":"apply","operations":[{"op":"update_pattern_cloth","request":updated}]}), true);
    assert!(editor.document().cloths[0].cache.is_none());
    send(&mut editor, json!({"op":"undo"}), true);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), original);
    send(&mut editor, json!({"op":"save","path":"pattern.json"}), true);
    let mut reopened = Editor::new(&root).unwrap();
    send(&mut reopened, json!({"op":"load","path":"pattern.json"}), true);
    assert_eq!(serde_json::to_value(reopened.document()).unwrap(), original);
    assert_eq!(send(&mut reopened, json!({"op":"cloth_state","id":"garment","animation":sample(0.3)}), true), state);
    std::fs::remove_dir_all(root).unwrap();
}
