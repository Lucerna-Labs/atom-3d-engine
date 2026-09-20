use mm3e_editor::{
    animation::{AnimationSample, Playback},
    cloth::{self, BakeClothRequest},
    model::{Document, Pass},
    sewing::{self, SewnClothRequest},
};
use serde_json::{json, Value};
use std::io::Write;

fn request(id: &str, sewn: bool) -> SewnClothRequest {
    serde_json::from_value(json!({"id":id,"label":"Two-piece hanging garment","group":"wardrobe",
        "panels":[
            {"id":"left","origin":[-0.2,1,0],"axis_u":[1,0,0],"axis_v":[0,1,0],"segments":[2,3],"width_m":0.4,"height_m":0.6,
                "pins":[{"vertex":9,"target_object":"anchor","point":[-0.4,0,0]},{"vertex":11,"target_object":"anchor","point":[0,0,0]}]},
            {"id":"right","origin":[0.2,1,0],"axis_u":[1,0,0],"axis_v":[0,1,0],"segments":[2,3],"width_m":0.4,"height_m":0.6}],
        "seams":if sewn {json!([{"panel_a":"left","chain_a":[2,5,8,11],"panel_b":"right","chain_b":[0,3,6,9],"rest_length_m":0,"compliance":0}])}else{json!([])},
        "thickness_m":0.002,"vertex_mass_kg":0.02,
        "settings":{"iterations":48,"substeps":4,"bend_compliance":0.02,"max_seam_error_m":0.001},
        "material":{"albedo":[0.6,0.1,0.08],"roughness":0.75}
    })).unwrap()
}
fn sources() -> Vec<Value> {
    vec![
        json!({"op":"create","object":{"id":"anchor","shape":{"type":"sphere","radius":0.03},"position":[0,1.3,0]}}),
        json!({"op":"set_joints","joints":[{"id":"shoulder","pivot":[0,1.3,0],"objects":[]},
            {"id":"attachment","parent":"shoulder","pivot":[0,1.3,0],"objects":["anchor"]}]}),
        json!({"op":"put_clip","clip":{"id":"hang","duration":0.5,"tracks":[
            {"target":{"type":"joint","id":"shoulder"},"keys":[{"time":0},{"time":0.5,"rotation_degrees":[0,0,10]}]},
            {"target":{"type":"joint","id":"attachment"},"keys":[{"time":0},{"time":0.5,"translation":[0.02,0.02,0]}]}]}}),
    ]
}
fn document() -> Document {
    let mut d = Document::default();
    let operations = sources();
    d.objects = serde_json::from_value(json!([operations[0]["object"].clone()])).unwrap();
    d.joints = serde_json::from_value(operations[1]["joints"].clone()).unwrap();
    d.clips = serde_json::from_value(json!([operations[2]["clip"].clone()])).unwrap();
    d
}
fn sample(time: f32) -> AnimationSample {
    AnimationSample { clip: "hang".into(), time, playback: Playback::Clamp }
}
fn bake(id: &str) -> BakeClothRequest {
    BakeClothRequest { id: id.into(), clip: "hang".into(), duration: None }
}
fn point(v: &Value, index: usize) -> [f64; 3] {
    std::array::from_fn(|axis| v["vertices"][index][axis].as_f64().unwrap())
}
fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
}
fn seam_gap(value: &Value) -> f64 {
    (0..4).map(|row| distance(point(value, 2 + row * 3), point(value, 12 + row * 3))).fold(0.0, f64::max)
}

#[test]
fn tight_seams_hold_joint_attached_garment_while_unsewn_panels_separate() {
    let mut d = document();
    sewing::create(&mut d, &request("garment", true)).unwrap();
    sewing::create(&mut d, &request("control", false)).unwrap();
    let authored = serde_json::to_value(&d.objects).unwrap();
    let result = cloth::bake(&mut d, &bake("garment")).unwrap();
    cloth::bake(&mut d, &bake("control")).unwrap();
    assert_eq!(authored, serde_json::to_value(&d.objects).unwrap());
    assert!(result["diagnostics"]["seam_projections"].as_u64().unwrap() > 0);
    assert!(result["diagnostics"]["max_seam_length_error"].as_f64().unwrap_or(0.0) < 0.001);
    let sewn = cloth::inspect(&d, "garment", Some(&sample(0.5))).unwrap();
    let control = cloth::inspect(&d, "control", Some(&sample(0.5))).unwrap();
    assert!(seam_gap(&sewn) < 0.001, "{sewn}");
    assert!(seam_gap(&control) > 0.3, "unsewn panel must fall away: {control}");
    println!(
        "tight garment seam gap={}m; no-seam control gap={}m; seam projections={}",
        seam_gap(&sewn),
        seam_gap(&control),
        result["diagnostics"]["seam_projections"]
    );
    assert_eq!(sewn["sewing"]["panels"][0]["id"], "left");
    assert_eq!(sewn["stitch_pairs"][0]["vertices"], json!([2, 12]));
    let (scene, _) = d.compile_at(&Pass::Beauty, Some(&sample(0.5))).unwrap();
    for (index, local) in [(9, [-0.4, 0.0, 0.0]), (11, [0.0, 0.0, 0.0])] {
        let wanted = scene.objects[0].xform.to_world(mm3e_editor::model::vec(local));
        assert!(distance(point(&sewn, index), [wanted.x as f64, wanted.y as f64, wanted.z as f64]) < 1e-6);
    }
    let p = point(&sewn, 5);
    assert!(scene.sample_object(1, mm3e_kit::Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32)).unwrap().dist < 0.0);
    // The source dependency closure must retain both joint levels and their tracks.
    d.clips[0].tracks[0].keys[1].rotation_degrees[2] = 12.0;
    assert!(d.compile_at(&Pass::Beauty, Some(&sample(0.5))).err().unwrap().contains("stale"));
}

#[test]
fn explicit_separated_assembly_closes_gap_without_modifying_rest_pattern() {
    let mut d = document();
    let mut r = request("assembly", true);
    r.panels[1].origin[0] += 0.1;
    // An open assembly pose is explicit and must fit the authored frame-zero tolerance.
    r.settings.max_seam_error_m = 0.11;
    sewing::create(&mut d, &r).unwrap();
    let rest = d.cloths[0].rest_vertices.clone();
    assert!(seam_gap(&cloth::inspect(&d, "assembly", None).unwrap()) > 0.099);
    cloth::bake(&mut d, &bake("assembly")).unwrap();
    assert_eq!(d.cloths[0].rest_vertices, rest);
    assert!(seam_gap(&cloth::inspect(&d, "assembly", Some(&sample(0.5))).unwrap()) < 0.001);
    let mut bad = document();
    r.settings.max_seam_error_m = 0.001;
    sewing::create(&mut bad, &r).unwrap();
    let before = serde_json::to_vec(&bad).unwrap();
    assert!(cloth::bake(&mut bad, &bake("assembly")).unwrap_err().contains("seam length error"));
    assert_eq!(before, serde_json::to_vec(&bad).unwrap());
}

#[test]
fn sampled_seam_gate_measures_positions_and_seam_edits_invalidate_cache() {
    let mut d = document();
    sewing::create(&mut d, &request("garment", true)).unwrap();
    cloth::bake(&mut d, &bake("garment")).unwrap();
    let mut changed_recipe = d.clone();
    changed_recipe.cloths[0].sewing.as_mut().unwrap().seams[0].compliance = 0.0001;
    assert!(cloth::inspect(&changed_recipe, "garment", None).unwrap()["cache"]["fresh"] == false);
    assert!(changed_recipe.compile_at(&Pass::Beauty, Some(&sample(0.5))).err().unwrap().contains("stale"));
    // An internally consistent checksum does not make wrong physical positions safe.
    // Preserve stale low diagnostics and prove that the actual evaluated seam is rechecked.
    let cache = d.cloths[0].cache.as_mut().unwrap();
    cache.frames[1].vertices[12][0] += 0.02;
    cache.frames_fnv1a64 = format!("{:016x}", mm3e_kit::atoms::hash(&serde_json::to_vec(&cache.frames).unwrap()));
    cloth::validate(&d).unwrap();
    let pose = sample(d.cloths[0].settings.fixed_dt);
    let info = cloth::inspect(&d, "garment", Some(&pose)).unwrap();
    assert!(info["sample_max_seam_length_error_m"].as_f64().unwrap() > 0.01);
    assert_eq!(info["sample_within_bake_tolerances"], false);
    assert!(d.compile_at(&Pass::Beauty, Some(&pose)).err().unwrap().contains("seam length error"));
}

#[test]
fn malformed_boundaries_correspondence_recipes_and_updates_are_atomic() {
    type Change = Box<dyn Fn(&mut SewnClothRequest)>;
    let cases: Vec<Change> = vec![
        Box::new(|r| r.panels[1].id = "left".into()),
        Box::new(|r| r.panels[1].segments = [15, 15]),
        Box::new(|r| r.panels[1].axis_v = [1.0, 0.0, 0.0]),
        Box::new(|r| r.seams[0].chain_a = vec![2, 8, 11]),
        Box::new(|r| r.seams[0].chain_a = vec![2, 5, 5, 11]),
        Box::new(|r| r.seams[0].chain_a = vec![1, 4, 7, 10]),
        Box::new(|r| r.seams[0].chain_b = vec![0, 3, 6]),
        Box::new(|r| r.seams[0].panel_b = "missing".into()),
        Box::new(|r| r.seams[0].panel_b = "left".into()),
        Box::new(|r| r.seams.push(r.seams[0].clone())),
        Box::new(|r| r.seams[0].compliance = f32::NAN),
        Box::new(|r| r.settings.max_seam_error_m = -0.1),
    ];
    for change in cases {
        let mut d = document();
        let before = serde_json::to_vec(&d).unwrap();
        let mut r = request("garment", true);
        change(&mut r);
        assert!(sewing::create(&mut d, &r).is_err());
        assert_eq!(before, serde_json::to_vec(&d).unwrap());
    }
    let mut d = document();
    let r = request("garment", true);
    sewing::create(&mut d, &r).unwrap();
    cloth::bake(&mut d, &bake("garment")).unwrap();
    let mut bad = r.clone();
    bad.seams[0].chain_b.reverse();
    // Explicit reverse correspondence is retained as given, even when physically poor.
    sewing::update(&mut d, &bad).unwrap();
    assert!(d.cloths[0].cache.is_none());
    assert_eq!(d.cloths[0].sewing.as_ref().unwrap().seams[0].chain_b, vec![9, 6, 3, 0]);
    assert!(cloth::bake(&mut d, &bake("garment")).unwrap_err().contains("seam length error"));
    sewing::update(&mut d, &r).unwrap();
    cloth::bake(&mut d, &bake("garment")).unwrap();
    let before = serde_json::to_vec(&d).unwrap();
    bad.panels[0].width_m = -1.0;
    assert!(sewing::update(&mut d, &bad).is_err());
    assert_eq!(before, serde_json::to_vec(&d).unwrap());
    d.cloths[0].sewing.as_mut().unwrap().panels[0].origin[0] += 0.01;
    assert!(cloth::validate(&d).unwrap_err().contains("reproduce"));
}

#[test]
fn native_sewn_cache_and_diagnostics_survive_cold_actual_editor_process_reload() {
    let root = std::env::temp_dir().join(format!(
        "mm3e-sewn-process-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir(&root).unwrap();
    let run = |requests: Vec<Value>| -> Vec<Value> {
        let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_mm3e-editor"))
            .args(["--root", root.to_str().unwrap(), "--project", "sewn.json"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let mut input = child.stdin.take().unwrap();
        for request in requests {
            writeln!(input, "{request}").unwrap();
        }
        drop(input);
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(|line| {
                let response: Value = serde_json::from_str(line).unwrap();
                assert_eq!(response["ok"], true, "{response}");
                response
            })
            .collect()
    };
    let mut operations = sources();
    operations.push(json!({"op":"create_sewn_cloth","request":request("garment",true)}));
    let inspect =
        json!({"id":"state","command":{"op":"cloth_state","id":"garment","animation":{"clip":"hang","time":0.5}}});
    let get = json!({"id":"document","command":{"op":"get_document"}});
    let first = run(vec![
        json!({"id":"author","expected_revision":0,"command":{"op":"apply","operations":operations}}),
        json!({"id":"bake","expected_revision":1,"command":{"op":"bake_cloth","request":{"id":"garment","clip":"hang"}}}),
        inspect.clone(),
        get.clone(),
    ]);
    let second = run(vec![inspect, get]);
    assert_eq!(first[2]["result"], second[0]["result"]);
    assert_eq!(first[3]["result"], second[1]["result"]);
    assert!(seam_gap(&second[0]["result"]) < 0.001);
    assert_eq!(second[0]["revision"], 2);
    std::fs::remove_dir_all(root).unwrap();
}
