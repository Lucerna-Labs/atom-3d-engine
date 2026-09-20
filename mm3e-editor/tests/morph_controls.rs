use mm3e_editor::{
    deform::{self, MorphWeight},
    protocol::Request,
    Editor,
};
use serde_json::{json, Value};

fn command(editor: &mut Editor, command: Value, mutation: bool) -> Value {
    let mut request = json!({"id":"control","command":command});
    if mutation {
        request["expected_revision"] = json!(editor.revision());
    }
    serde_json::to_value(editor.handle(serde_json::from_value::<Request>(request).unwrap())).unwrap()
}

#[test]
fn sparse_default_morph_controls_preserve_dense_sources_and_are_atomic() {
    let mut editor = Editor::new(&std::env::temp_dir()).unwrap();
    let setup = command(
        &mut editor,
        json!({"op":"apply","operations":[
            {"op":"create","object":{"id":"sheet","shape":{"type":"surface","vertices":[[0,0,0],[1,0,0],[0,1,0]],"triangles":[[0,1,2]],"thickness_m":0.01}}},
            {"op":"bind_surface","request":{"deformer":{"id":"shape","object":"sheet","blendshapes":[
                {"id":"x","deltas":[[0.2,0,0],[0.2,0,0],[0.2,0,0]]},
                {"id":"y","deltas":[[0,0.3,0],[0,0.3,0],[0,0.3,0]]}]}}},
            {"op":"put_clip","clip":{"id":"take","duration":1,"morph_tracks":[{"deformer":"shape","blendshape":"x","keys":[{"time":0,"weight":0.8},{"time":1,"weight":0.8}]}]}}
        ]}),
        true,
    );
    assert_eq!(setup["ok"], true, "{setup}");
    let before = serde_json::to_value(editor.document()).unwrap();
    let edit = command(
        &mut editor,
        json!({"op":"apply","operations":[{"op":"set_morph_weights","id":"shape","weights":[{"id":"x","weight":0.5},{"id":"y","weight":0.25}]}]}),
        true,
    );
    assert_eq!(edit["ok"], true, "{edit}");
    let after = serde_json::to_value(editor.document()).unwrap();
    assert_eq!(before["objects"], after["objects"]);
    for i in 0..2 {
        assert_eq!(
            before["deformers"][0]["blendshapes"][i]["deltas"],
            after["deformers"][0]["blendshapes"][i]["deltas"]
        );
    }
    let rest = command(&mut editor, json!({"op":"deformer_state","id":"shape"}), false);
    let animated =
        command(&mut editor, json!({"op":"deformer_state","id":"shape","animation":{"clip":"take","time":0.4}}), false);
    assert!((rest["result"]["vertices"][0][0].as_f64().unwrap() - 0.1).abs() < 1e-6);
    assert!((rest["result"]["vertices"][0][1].as_f64().unwrap() - 0.075).abs() < 1e-6);
    assert!(
        (animated["result"]["vertices"][0][0].as_f64().unwrap() - 0.16).abs() < 1e-6,
        "tracks override default controls"
    );
    for weights in [
        json!([]),
        json!([{"id":"x","weight":0.1},{"id":"x","weight":0.2}]),
        json!([{"id":"x","weight":0.1},{"id":"missing","weight":0.2}]),
        json!([{"id":"y","weight":2}]),
    ] {
        let revision = editor.revision();
        let response = command(
            &mut editor,
            json!({"op":"apply","operations":[{"op":"set_morph_weights","id":"shape","weights":weights}]}),
            true,
        );
        assert_eq!(response["ok"], false);
        assert_eq!(editor.revision(), revision);
        assert_eq!(serde_json::to_value(editor.document()).unwrap(), after);
    }
    let mut source = editor.document().clone();
    let bytes = serde_json::to_vec(&source).unwrap();
    assert!(deform::set_morph_weights(&mut source, "shape", vec![MorphWeight { id: "x".into(), weight: f32::NAN }])
        .is_err());
    assert_eq!(serde_json::to_vec(&source).unwrap(), bytes);
    assert_eq!(command(&mut editor, json!({"op":"undo"}), true)["ok"], true);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), before);
    assert_eq!(command(&mut editor, json!({"op":"redo"}), true)["ok"], true);
    assert_eq!(serde_json::to_value(editor.document()).unwrap(), after);
}
