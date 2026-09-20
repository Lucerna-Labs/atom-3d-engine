//! Delivery acceptance at the evaluated-field, filesystem and editor boundaries.
//! Full USD schema/parser acceptance lives in the independent OpenUSD tests.
use mm3e_editor::{
    delivery::{self, ExportUsdRequest},
    model::{Document, Shape},
    Editor,
};
use mm3e_kit::Vec3;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);
struct Root(PathBuf);
impl Root {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mm3e-delivery-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Root {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn document() -> Document {
    Document {
        objects: serde_json::from_value(json!([
            {"id":"body","shape":{"type":"sphere","radius":1},"material":{"albedo":[0.6,0.1,0.1]}},
            {"id":"cut","shape":{"type":"sphere","radius":0.35},"position":[0.1,0,0],"combine":{"type":"subtract"},"material":{"albedo":[0.1,0.2,0.7]}},
            {"id":"unselected_cut","shape":{"type":"sphere","radius":5},"combine":{"type":"subtract"}}
        ])).unwrap(),
        ..Document::default()
    }
}

fn request() -> ExportUsdRequest {
    ExportUsdRequest {
        path: "asset.usda".into(),
        object_ids: vec!["body".into()],
        bounds_min: [-1.3; 3],
        bounds_max: [1.3; 3],
        resolution: [24; 3],
        max_surface_error_m: 0.08,
        max_field_residual: 0.04,
        clip: None,
        start_seconds: 0.0,
        end_seconds: 0.0,
        frames_per_second: 24.0,
        camera_near_clip_m: 0.0001,
        texture_delivery: None,
    }
}

fn numbers(value: &str) -> Vec<f32> {
    value
        .split(|c: char| c.is_whitespace() || ['[', ']', '(', ')', ','].contains(&c))
        .filter(|s| !s.is_empty())
        .map(|v| v.parse().unwrap())
        .collect()
}

// Read the emitted numeric defaults, so the geometric assertions examine the
// delivered payload itself, independently of its JSON measurement report.
fn geometry(bytes: &[u8]) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let text = std::str::from_utf8(bytes).unwrap();
    let attribute = |name: &str| text.lines().find_map(|line| line.trim().strip_prefix(name)).unwrap();
    let positions = numbers(attribute("point3f[] points = "))
        .as_chunks::<3>()
        .0
        .iter()
        .map(|v| Vec3::new(v[0], v[1], v[2]))
        .collect();
    let triangles = numbers(attribute("int[] faceVertexIndices = "))
        .as_chunks::<3>()
        .0
        .iter()
        .map(|v| [v[0] as u32, v[1] as u32, v[2] as u32])
        .collect();
    (positions, triangles)
}

fn independent_volume_and_edges(positions: &[Vec3], triangles: &[[u32; 3]]) -> f64 {
    let mut edges: HashMap<(u32, u32), (u32, i32)> = HashMap::new();
    let mut volume = 0.0;
    for &[a, b, c] in triangles {
        let [pa, pb, pc] = [a, b, c].map(|i| positions[i as usize]);
        assert!((pb - pa).cross(pc - pa).length_sq() > 0.0);
        volume += f64::from(pa.dot(pb.cross(pc))) / 6.0;
        for (i, j) in [(a, b), (b, c), (c, a)] {
            let edge = edges.entry((i.min(j), i.max(j))).or_default();
            edge.0 += 1;
            edge.1 += if i < j { 1 } else { -1 };
        }
    }
    assert!(edges.values().all(|&(n, direction)| n == 2 && direction == 0));
    volume
}

#[test]
fn composed_selection_preserves_authored_subtraction_order_and_cavity_geometry() {
    let document = document();
    let mut selection = request();
    selection.object_ids = vec!["cut".into(), "body".into()];
    let (bytes, report) = delivery::encode(&document, &selection).unwrap();
    assert_eq!(report["source_entity_ids"], json!(["body", "cut"]));
    assert_eq!(report["frames"][0]["connected_components"], 2);
    let embedding = &report["frames"][0]["embedding_validation"];
    assert_eq!(embedding["intersection_free"], true);
    assert_eq!(embedding["scope"], "final delivered mesh");
    assert!(embedding["additional_work"].as_u64().unwrap() > 0);
    assert!(embedding["validation"]["candidate_pairs"].as_u64().unwrap() > 0);
    let (positions, triangles) = geometry(&bytes);
    let volume = independent_volume_and_edges(&positions, &triangles);
    let expected = 4.0 * std::f64::consts::PI / 3.0 * (1.0 - 0.35f64.powi(3));
    assert!((volume / expected - 1.0).abs() < 0.02, "delivered cavity volume {volume} != {expected}");
    assert!(positions.iter().filter(|&&p| (p - Vec3::new(0.1, 0., 0.)).length() < 0.45).count() > 100);
    for &p in &positions {
        let authored = (p.length() - 1.0).max(-((p - Vec3::new(0.1, 0., 0.)).length() - 0.35));
        assert!(authored.abs() < selection.max_field_residual);
    }
    selection.object_ids.reverse();
    assert_eq!(bytes, delivery::encode(&document, &selection).unwrap().0);
    selection.object_ids = vec!["cut".into()];
    assert!(delivery::encode(&document, &selection).unwrap_err().contains("first selected"));
    selection.object_ids = vec!["body".into(), "unselected_cut".into()];
    assert!(delivery::encode(&document, &selection).unwrap_err().contains("no nondegenerate isosurface"));
}

#[test]
fn coarse_geometry_is_rejected_by_convergence_and_finer_geometry_passes() {
    let document = document();
    let mut coarse = request();
    coarse.resolution = [8; 3];
    coarse.max_surface_error_m = 0.02;
    coarse.max_field_residual = 1.0;
    let error = delivery::encode(&document, &coarse).unwrap_err();
    assert!(error.contains("sampled grid deviation"), "{error}");
    let mut fine = coarse;
    fine.resolution = [32; 3];
    let (_, report) = delivery::encode(&document, &fine).unwrap();
    assert!(report["frames"][0]["sampled_mesh_deviation_m"].as_f64().unwrap() < 0.02);
    assert_eq!(report["comparison_resolution"], json!([16, 16, 16]));
}

#[test]
fn authored_scalar_residual_is_reported_and_gated_separately_from_geometric_distance() {
    let mut document = document();
    let samples = (0..9)
        .flat_map(|z| {
            (0..9).flat_map(move |y| {
                (0..9).map(move |x| {
                    Vec3::new(-1.5 + x as f32 * 0.375, -1.5 + y as f32 * 0.375, -1.5 + z as f32 * 0.375).length() - 1.0
                })
            })
        })
        .collect::<Vec<_>>();
    document.objects[0].shape = Shape::Volume { dims: [9; 3], min: [-1.5; 3], cell: [0.375; 3], samples };
    let mut export = request();
    export.max_surface_error_m = 0.2;
    export.max_field_residual = 1.0;
    let (_, unit) = delivery::encode(&document, &export).unwrap();
    if let Shape::Volume { samples, .. } = &mut document.objects[0].shape {
        for value in samples {
            *value *= 4.0;
        }
    }
    let (_, scaled) = delivery::encode(&document, &export).unwrap();
    let residual = |report: &Value| report["frames"][0]["sampled_field_residual"].as_f64().unwrap();
    let deviation = |report: &Value| report["frames"][0]["sampled_mesh_deviation_m"].as_f64().unwrap();
    assert!(residual(&unit) > 1e-5);
    assert!((residual(&scaled) / residual(&unit) - 4.0).abs() < 1e-4);
    assert!((deviation(&scaled) - deviation(&unit)).abs() < 1e-6);
    export.max_field_residual = (residual(&scaled) * 0.5) as f32;
    let error = delivery::encode(&document, &export).unwrap_err();
    assert!(error.contains("sampled field residual"), "{error}");
}

#[test]
fn thickened_native_triangle_sheet_delivers_both_sides_and_rounded_border() {
    let mut document = document();
    document.objects[0].shape = serde_json::from_value(json!({"type":"surface",
        "vertices":[[-0.5,0,-0.5],[0.5,0,-0.5],[0.5,0,0.5],[-0.5,0,0.5]],
        "triangles":[[0,1,2],[0,2,3]],"thickness_m":0.1}))
    .unwrap();
    let export = ExportUsdRequest {
        bounds_min: [-0.8, -0.3, -0.8],
        bounds_max: [0.8, 0.3, 0.8],
        resolution: [32, 16, 32],
        ..request()
    };
    let (bytes, report) = delivery::encode(&document, &export).unwrap();
    let (positions, triangles) = geometry(&bytes);
    let volume = independent_volume_and_edges(&positions, &triangles);
    assert!(volume > 0.095 && volume < 0.13, "sheet must carry finite thickness, volume={volume}");
    assert_eq!(report["frames"][0]["connected_components"], 1);
    assert!(positions.iter().any(|p| p.y > 0.049));
    assert!(positions.iter().any(|p| p.y < -0.049));
    assert!(positions.iter().any(|p| p.x > 0.535), "rounded border must extend beyond the original midsurface");
    assert!(positions.iter().all(|p| p.y.abs() < 0.05001));
}

#[test]
fn animation_uses_exact_inclusive_frame_schedule_and_evaluated_world_positions() {
    let mut document = document();
    document.clips = serde_json::from_value(json!([{"id":"move","duration":0.25,"tracks":[
    {"target":{"type":"object","id":"body"},"easing":"step","keys":[
        {"time":0},{"time":1.0/24.0,"translation":[0.1,0,0]},
        {"time":2.0/24.0,"translation":[0.2,0,0]},{"time":3.0/24.0,"translation":[0.3,0,0]}
    ]}]}]))
    .unwrap();
    let export = ExportUsdRequest {
        clip: Some("move".into()),
        start_seconds: 1.0 / 24.0,
        end_seconds: 3.0 / 24.0,
        bounds_min: [-1.4; 3],
        bounds_max: [1.4; 3],
        max_surface_error_m: 0.1,
        ..request()
    };
    let (bytes, report) = delivery::encode(&document, &export).unwrap();
    let frames = report["frames"].as_array().unwrap();
    assert_eq!(frames.len(), 3);
    for (frame, time) in frames.iter().zip([1.0, 2.0, 3.0]) {
        assert_eq!(frame["time_code"], time);
        assert!((frame["native_seconds"].as_f64().unwrap() - time / 24.0).abs() < 1e-8);
    }
    let text = std::str::from_utf8(&bytes).unwrap();
    let block = text.split("point3f[] points.timeSamples = {\n").nth(1).unwrap().split('}').next().unwrap();
    let lines: Vec<_> = block.lines().filter(|line| !line.trim().is_empty()).collect();
    assert_eq!(lines.len(), 3);
    for (line, expected_center) in lines.into_iter().zip([0.1f32, 0.2, 0.3]) {
        let values = numbers(line.split_once(':').unwrap().1);
        let xs = values.as_chunks::<3>().0.iter().map(|v| v[0]).collect::<Vec<_>>();
        let center = (xs.iter().copied().fold(f32::INFINITY, f32::min)
            + xs.iter().copied().fold(f32::NEG_INFINITY, f32::max))
            * 0.5;
        assert!((center - expected_center).abs() < 2e-5, "scheduled translation {center} != {expected_center}");
    }
}

#[test]
fn malformed_selection_grid_time_material_and_clipping_are_rejected_without_output() {
    let document = document();
    let root = Root::new();
    let mut cases: Vec<ExportUsdRequest> = vec![];
    let mut invalid = request();
    invalid.object_ids.clear();
    cases.push(invalid);
    let mut invalid = request();
    invalid.object_ids.push("body".into());
    cases.push(invalid);
    let mut invalid = request();
    invalid.object_ids = vec!["missing".into()];
    cases.push(invalid);
    let mut invalid = request();
    invalid.resolution = [9; 3];
    cases.push(invalid);
    let mut invalid = request();
    invalid.bounds_min = invalid.bounds_max;
    cases.push(invalid);
    let mut invalid = request();
    invalid.bounds_min = [-1.; 3];
    invalid.bounds_max = [1.; 3];
    cases.push(invalid);
    let mut invalid = request();
    invalid.max_surface_error_m = f32::NAN;
    cases.push(invalid);
    let mut invalid = request();
    invalid.max_field_residual = 0.0;
    cases.push(invalid);
    let mut invalid = request();
    invalid.end_seconds = 1.0;
    cases.push(invalid);
    let mut invalid = request();
    invalid.clip = Some("missing".into());
    cases.push(invalid);
    let mut invalid = request();
    invalid.frames_per_second = f64::INFINITY;
    cases.push(invalid);
    for invalid in cases {
        assert!(delivery::export(&document, &root.0, &invalid).is_err());
        assert_eq!(fs::read_dir(&root.0).unwrap().count(), 0, "failed export installed an output or temporary file");
    }
    for unsupported in ["checker", "reflectivity"] {
        let mut document = document.clone();
        if unsupported == "checker" {
            document.objects[0].material.checker = true;
        } else {
            document.objects[0].material.reflectivity = 0.2;
        }
        let error = delivery::encode(&document, &request()).unwrap_err();
        assert!(error.contains("unsupported"), "{error}");
    }
    let mut animated = document.clone();
    animated.clips = serde_json::from_value(json!([{"id":"move","duration":10,"tracks":[]}])).unwrap();
    let mut invalid = request();
    invalid.clip = Some("move".into());
    invalid.end_seconds = 0.01;
    assert!(delivery::encode(&animated, &invalid).unwrap_err().contains("fps grid"));
    invalid.end_seconds = 10.0;
    assert!(delivery::encode(&animated, &invalid).unwrap_err().contains("at most 60"));
}

#[test]
fn editor_protocol_writes_complete_delivery_without_mutation_and_never_overwrites() {
    let root = Root::new();
    let mut editor = Editor::new(&root.0).unwrap();
    let create = editor.handle(
        serde_json::from_value(json!({"id":"create","expected_revision":0,"command":{
            "op":"apply","operations":[{"op":"create","object":document().objects[0]}]
        }}))
        .unwrap(),
    );
    assert!(create.ok, "{create:?}");
    let before = serde_json::to_vec(editor.document()).unwrap();
    let revision = editor.revision();
    let export =
        || serde_json::from_value(json!({"id":"deliver","command":{"op":"export_usd","request":request()}})).unwrap();
    let result = editor.handle(export());
    assert!(result.ok, "{result:?}");
    let bytes = fs::read(root.0.join("asset.usda")).unwrap();
    assert!(bytes.starts_with(b"#usda 1.0"));
    assert_eq!(result.result.unwrap()["bytes"], bytes.len());
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), before);
    assert_eq!(editor.revision(), revision);
    let rejected = editor.handle(export());
    assert!(!rejected.ok);
    assert_eq!(fs::read(root.0.join("asset.usda")).unwrap(), bytes);
    assert_eq!(fs::read_dir(&root.0).unwrap().count(), 1);
}
