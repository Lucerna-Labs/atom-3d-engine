use mm3e_editor::{
    model::Document,
    pattern::{self, PatternClothRequest, PatternLoop, PatternPanel, PatternPin},
    protocol::{Request, Response},
    sewing::SeamChain,
    Editor,
};
use serde_json::{json, Value};

fn panel(id: &str) -> PatternPanel {
    serde_json::from_value(json!({
        "id":id,"origin":[0,1,0],"axis_u":[1,0,0],"axis_v":[0,1,0],
        "outer":{"id":"outer","points":[
            {"id":"a","position":[0,0]},{"id":"b","position":[1,0]},
            {"id":"c","position":[1,1]},{"id":"d","position":[0,1]}
        ]}
    }))
    .unwrap()
}

fn request() -> PatternClothRequest {
    serde_json::from_value(json!({"id":"garment","panels":[panel("left"),panel("right")],
        "thickness_m":0.002,"vertex_mass_kg":0.02}))
    .unwrap()
}

fn assert_atomic_rejection(document: &Document, request: &PatternClothRequest, message: &str) {
    let mut candidate = document.clone();
    let before = serde_json::to_vec(document).unwrap();
    let error = pattern::update(&mut candidate, request).unwrap_err();
    assert!(error.contains(message), "expected {message:?}, got {error:?}");
    assert_eq!(serde_json::to_vec(&candidate).unwrap(), before);
}

#[test]
fn empty_and_excessive_hole_lists_fail_before_unbounded_contour_processing() {
    let request = request();
    let mut document = Document::default();
    pattern::create(&mut document, &request).unwrap();
    for (count, message) in
        [(1, "at least three"), (84, "at least three"), (85, "loop or pin count"), (4096, "loop or pin count")]
    {
        let mut bad = request.clone();
        bad.panels[0].holes = vec![PatternLoop { id: "empty".into(), points: vec![] }; count];
        assert!(pattern::preview(&bad.panels[0]).unwrap_err().contains(message));
        assert_atomic_rejection(&document, &bad, message);
    }
    let mut bad = request.clone();
    bad.panels[0].outer.points.clear();
    assert_atomic_rejection(&document, &bad, "at least three");
}

#[test]
fn excessive_pin_lists_are_rejected_and_exact_256_valid_pins_are_supported() {
    let original = request();
    let mut document = Document::default();
    pattern::create(&mut document, &original).unwrap();
    let pin: PatternPin =
        serde_json::from_value(json!({"vertex":{"type":"control","id":"a"},"point":[0,1,0]})).unwrap();
    for count in [257, 4096] {
        let mut bad = original.clone();
        bad.panels[0].pins = vec![pin.clone(); count];
        assert!(pattern::preview(&bad.panels[0]).unwrap_err().contains("loop or pin count"));
        assert_atomic_rejection(&document, &bad, "loop or pin count");
    }
    // Every vertex of this simple polygon is a distinct named control and pin.
    // The exact limit is valid; it must not be implemented as a lower hidden cap.
    let points: Vec<_> = (0..256)
        .map(|i| {
            let angle = i as f64 * std::f64::consts::TAU / 256.0;
            json!({"id":format!("p{i}"),"position":[angle.cos(),angle.sin()]})
        })
        .collect();
    let pins: Vec<_> = points
        .iter()
        .map(|p| {
            json!({"vertex":{"type":"control","id":p["id"]},
            "point":[p["position"][0].as_f64().unwrap() as f32,1.0+p["position"][1].as_f64().unwrap() as f32,0]})
        })
        .collect();
    let mut bounded = panel("bounded");
    bounded.outer = serde_json::from_value(json!({"id":"outer","points":points})).unwrap();
    bounded.pins = serde_json::from_value(json!(pins)).unwrap();
    assert_eq!(pattern::preview(&bounded).unwrap()["points"].as_array().unwrap().len(), 256);
    let mut exact = original;
    exact.panels = vec![bounded];
    pattern::update(&mut document, &exact).unwrap();
    assert_eq!(document.cloths[0].pins.len(), 256);
    assert!(document.cloths[0].inverse_masses.iter().all(|&mass| mass == 0.0));
}

#[test]
fn seam_counts_and_lengths_are_bounded_before_per_vertex_sets_are_built() {
    let original = request();
    let mut document = Document::default();
    pattern::create(&mut document, &original).unwrap();
    let seam: SeamChain =
        serde_json::from_value(json!({"panel_a":"left","panel_b":"right","chain_a":[0,1],"chain_b":[0,1]})).unwrap();
    let mut bad = original.clone();
    bad.seams = vec![seam.clone(); 257];
    assert_atomic_rejection(&document, &bad, "at most 256 seam chains");
    for count in [5, 4096] {
        let mut bad = original.clone();
        let mut excessive = seam.clone();
        excessive.chain_a = (0..count).collect();
        excessive.chain_b = (0..count).collect();
        bad.seams = vec![excessive];
        assert_atomic_rejection(&document, &bad, "seam chain count exceeds its panel's local vertex count");
    }
    for (a, b) in [(vec![], vec![]), (vec![0], vec![0]), (vec![0, 1], vec![0, 1, 2])] {
        let mut bad = original.clone();
        let mut invalid = seam.clone();
        invalid.chain_a = a;
        invalid.chain_b = b;
        bad.seams = vec![invalid];
        assert_atomic_rejection(&document, &bad, "equal counts of at least two");
    }
    let mut valid = original;
    let mut full_boundary = seam;
    full_boundary.chain_a = vec![0, 1, 2, 3];
    full_boundary.chain_b = vec![0, 1, 2, 3];
    valid.seams = vec![full_boundary];
    pattern::update(&mut document, &valid).unwrap();
    assert_eq!(document.cloths[0].pattern.as_ref().unwrap().seams[0].chain_a.len(), 4);
}

fn send(editor: &mut Editor, command: Value, revision: Option<u64>) -> Response {
    let request: Request =
        serde_json::from_value(json!({"id":"limits","expected_revision":revision,"command":command})).unwrap();
    editor.handle(request)
}

#[test]
fn preview_is_read_only_with_optional_revision_while_mutations_require_current_revision() {
    let mut editor = Editor::new(&std::env::temp_dir()).unwrap();
    let original = serde_json::to_vec(editor.document()).unwrap();
    let preview = json!({"op":"preview_pattern_panel","panel":panel("left")});
    let first = send(&mut editor, preview.clone(), None);
    assert!(first.ok);
    assert_eq!(editor.revision(), 0);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), original);
    let operation = json!({"op":"create_pattern_cloth","request":request()});
    let missing = send(&mut editor, json!({"op":"apply","operations":[operation.clone()]}), None);
    assert_eq!(missing.error.unwrap().code, "revision_required");
    let dry = send(&mut editor, json!({"op":"apply","operations":[operation.clone()],"dry_run":true}), Some(0));
    assert!(dry.ok);
    assert_eq!(editor.revision(), 0);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), original);
    assert!(send(&mut editor, json!({"op":"apply","operations":[operation]}), Some(0)).ok);
    let committed = serde_json::to_vec(editor.document()).unwrap();
    assert_eq!(editor.revision(), 1);
    let stale = send(&mut editor, preview.clone(), Some(0));
    assert_eq!(stale.error.unwrap().code, "revision_conflict");
    for revision in [None, Some(1)] {
        assert_eq!(send(&mut editor, preview.clone(), revision).result, first.result);
        assert_eq!(editor.revision(), 1);
        assert_eq!(serde_json::to_vec(editor.document()).unwrap(), committed);
    }
    let mut bad = panel("left");
    bad.holes = vec![PatternLoop { id: "empty".into(), points: vec![] }];
    assert!(!send(&mut editor, json!({"op":"preview_pattern_panel","panel":bad}), None).ok);
    assert_eq!(editor.revision(), 1);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), committed);
    assert!(send(&mut editor, json!({"op":"undo"}), Some(1)).ok);
    assert_eq!(serde_json::to_vec(editor.document()).unwrap(), original, "previews must not append undo entries");
}
