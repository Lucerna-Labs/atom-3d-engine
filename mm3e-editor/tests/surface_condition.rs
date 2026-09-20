use mm3e_editor::surface_condition::{condition, condition_certified, Options};
use mm3e_kit::{
    meshing::{extract_isosurface, Mesh},
    surface_intersections::validate_certified,
    Vec3,
};
fn fixture() -> Mesh {
    let mut mesh = extract_isosurface(|p| p.length() - 1., Vec3::splat(-2.), Vec3::splat(2.), [8; 3]).unwrap();
    mesh.positions = vec![
        Vec3::new(1., 0., 0.00001),
        Vec3::new(0., 0., -1.),
        Vec3::new(1., 0., 0.),
        Vec3::new(0., 1., 0.),
        Vec3::new(-1., 0., 0.),
        Vec3::new(0., -1., 0.),
    ];
    mesh.triangles = vec![[0, 2, 3], [0, 3, 4], [0, 4, 5], [0, 5, 2], [1, 3, 2], [1, 4, 3], [1, 5, 4], [1, 2, 5]];
    mesh
}
fn options() -> Options {
    Options { max_error_m: 0.00002, max_work: 2_000_000 }
}
fn assert_correspondence(original: &Mesh, final_mesh: &Mesh, map: &[u32], faces: &[usize], limit: f64) {
    assert_eq!(final_mesh.triangles.len(), faces.len());
    for (&face, &source) in final_mesh.triangles.iter().zip(faces) {
        assert_eq!(face, original.triangles[source].map(|i| map[i as usize]));
    }
    for (i, &j) in map.iter().enumerate() {
        let a = original.positions[i];
        let b = final_mesh.positions[j as usize];
        let difference =
            [f64::from(a.x) - f64::from(b.x), f64::from(a.y) - f64::from(b.y), f64::from(a.z) - f64::from(b.z)];
        assert!(difference[0].hypot(difference[1]).hypot(difference[2]) <= limit);
        assert!(original.positions.contains(&b));
    }
    mm3e_kit::mesh_topology::validate(&final_mesh.triangles, 2_000_000).unwrap();
    mm3e_kit::surface_intersections::validate(&final_mesh.positions, &final_mesh.triangles, 64_000_000).unwrap();
}
#[test]
fn endpoint_conditioning_preserves_frozen_face_ancestry_and_exact_work_boundary() {
    let mesh = fixture();
    let result = condition(mesh.clone(), options()).unwrap();
    assert_eq!(result.report.contractions.len(), 1);
    assert_eq!(result.report.topology_snapshots, 1);
    assert!(result.report.topology_snapshot_work > 0);
    assert_eq!(result.mesh.positions.len(), 5);
    assert_eq!(result.mesh.triangles.len(), 6);
    assert_correspondence(
        &mesh,
        &result.mesh,
        &result.original_to_final,
        &result.face_source_indices,
        options().max_error_m,
    );
    assert!(
        result.report.initial_embedding_work > 0
            && result.report.final_embedding_work > 0
            && result.report.source_work > 0
    );
    let replay = condition(mesh.clone(), Options { max_work: result.report.work, ..options() }).unwrap();
    assert_eq!(replay.mesh, result.mesh);
    assert_eq!(replay.original_to_final, result.original_to_final);
    assert_eq!(replay.report.work, result.report.work);
    let short = condition(mesh, Options { max_work: result.report.work - 1, ..options() }).err().unwrap();
    assert!(short.message.contains("work budget"), "{short}");
    assert!(short.work < result.report.work);
}
#[test]
fn zero_allowance_preserves_geometry_and_unresolved_topology_does_not_delete_a_component() {
    let mesh = fixture();
    let result = condition(mesh.clone(), Options { max_error_m: 0., ..options() }).unwrap();
    assert_eq!(result.mesh.positions, mesh.positions);
    assert_eq!(result.mesh.triangles, mesh.triangles);
    assert!(result.report.contractions.is_empty());
    assert_eq!(result.report.topology_snapshots, 0);
    assert_eq!(result.report.topology_snapshot_work, 0);
    assert_eq!(result.report.max_displacement_m, 0.);
    let mut tetra = fixture();
    tetra.positions =
        vec![Vec3::ZERO, Vec3::new(0.00001, 0., 0.), Vec3::new(0., 0.00001, 0.), Vec3::new(0., 0., 0.00001)];
    tetra.triangles = vec![[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]];
    let result = condition(tetra.clone(), options()).unwrap();
    assert!(result.report.rejected_topology > 0);
    assert!(result.report.contractions.is_empty());
    assert_eq!(result.mesh.triangles, tetra.triangles);
    assert_eq!(result.report.remaining_short_edges, 6);
}
#[test]
fn invalid_input_embedding_and_limits_reject_with_real_spent_work() {
    let original = fixture();
    let mut overlap = original.clone();
    overlap.positions.extend(original.positions.iter().copied());
    overlap.triangles.extend(original.triangles.iter().map(|t| t.map(|v| v + 6)));
    let failure = condition(overlap, options()).err().unwrap();
    assert!(failure.message.contains("intersect") || failure.message.contains("overlap"), "{failure}");
    assert!(failure.work > 0);
    for max_error_m in [-1., f64::NAN, f64::INFINITY] {
        assert!(condition(original.clone(), Options { max_error_m, ..options() }).is_err());
    }
}
#[test]
fn certificate_reuse_binds_compact_output_and_stale_input_requires_full_validation() {
    let original = fixture();
    let input = validate_certified(&original.positions, &original.triangles, 2_000_000).unwrap();
    let ordinary = condition(original.clone(), options()).unwrap();
    let output = condition_certified(original.clone(), options(), Some(&input.certificate)).unwrap();
    assert!(output.report.initial_embedding_reused);
    assert_eq!(output.report.initial_embedding_work, output.report.initial_certificate_match_work);
    assert!(output.report.output_certificate_work > 0);
    assert_eq!(output.mesh, ordinary.mesh);
    assert_eq!(output.original_to_final, ordinary.original_to_final);
    assert_eq!(output.face_source_indices, ordinary.face_source_indices);
    let proof = output.embedding_certificate.as_ref().unwrap();
    assert!(proof.matches(&output.mesh.positions, &output.mesh.triangles, 100_000).unwrap().matches);
    assert!(!proof.matches(&original.positions, &original.triangles, 100_000).unwrap().matches);
    let exact = condition_certified(
        original.clone(),
        Options { max_work: output.report.work, ..options() },
        Some(&input.certificate),
    )
    .unwrap();
    assert_eq!(exact.mesh, output.mesh);
    assert_eq!(exact.report.work, output.report.work);
    let short = condition_certified(
        original.clone(),
        Options { max_work: output.report.work - 1, ..options() },
        Some(&input.certificate),
    )
    .err()
    .unwrap();
    assert!(short.message.contains("budget"));
    assert!(short.work < output.report.work);

    let mut stale = original.clone();
    stale.positions[0].z = stale.positions[0].z.next_up();
    let changed = condition_certified(stale, options(), Some(&input.certificate)).unwrap();
    assert!(!changed.report.initial_embedding_reused);
    assert!(changed.report.initial_embedding_work > changed.report.initial_certificate_match_work);
    let mut overlap = original.clone();
    overlap.positions.extend(original.positions.iter().copied());
    overlap.triangles.extend(original.triangles.iter().map(|t| t.map(|v| v + 6)));
    let error = condition_certified(overlap, options(), Some(&input.certificate)).err().unwrap();
    assert!(error.message.contains("intersect") || error.message.contains("overlap"));
}
#[test]
fn frozen_original_bundle_conditions_nanometre_edges_without_changing_endpoint_coordinates() {
    let value: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/surface_condition/bundle-input.json")).unwrap();
    let mut mesh = fixture();
    mesh.positions = value["positions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| Vec3::new(p[0].as_f64().unwrap() as f32, p[1].as_f64().unwrap() as f32, p[2].as_f64().unwrap() as f32))
        .collect();
    mesh.triangles = value["triangles"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| [t[0].as_u64().unwrap() as u32, t[1].as_u64().unwrap() as u32, t[2].as_u64().unwrap() as u32])
        .collect();
    mesh.metadata.min = Vec3::new(-1., -1., -0.5);
    mesh.metadata.max = Vec3::new(1., 1., 0.5);
    mesh.metadata.spacing = Vec3::new(2. / 24., 2. / 24., 1. / 24.);
    let result = condition(mesh.clone(), Options { max_error_m: 0.000049, max_work: 136_191_706 }).unwrap();
    let proof = validate_certified(&mesh.positions, &mesh.triangles, 64_000_000).unwrap();
    let reused = condition_certified(
        mesh.clone(),
        Options { max_error_m: 0.000049, max_work: 136_191_706 },
        Some(&proof.certificate),
    )
    .unwrap();
    assert!(reused.report.initial_embedding_reused);
    assert!(reused.report.work < result.report.work);
    assert_eq!(reused.mesh, result.mesh);
    assert_eq!(reused.face_source_indices, result.face_source_indices);
    assert_eq!(reused.original_to_final, result.original_to_final);
    assert_eq!(
        serde_json::to_value(&reused.report.contractions).unwrap(),
        serde_json::to_value(&result.report.contractions).unwrap()
    );
    assert_eq!(reused.report.max_displacement_m.to_bits(), result.report.max_displacement_m.to_bits());
    assert!(
        reused
            .embedding_certificate
            .as_ref()
            .unwrap()
            .matches(&reused.mesh.positions, &reused.mesh.triangles, 1_000_000)
            .unwrap()
            .matches
    );
    println!("certified bundle conditioning {} work versus {} standalone", reused.report.work, result.report.work);
    println!(
        "bundle conditioning {} vertices / {} faces, {} contractions, {} remaining short edges, {} work, {}m bound",
        result.mesh.positions.len(),
        result.mesh.triangles.len(),
        result.report.contractions.len(),
        result.report.remaining_short_edges,
        result.report.work,
        result.report.max_displacement_m
    );
    // Frozen pre-optimization geometry/ancestry hashes are independent of the
    // source-check implementation and deliberately exclude work counters.
    use sha2::{Digest, Sha256};
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/surface_condition/expected-prepared.json")).unwrap();
    let hash = |bytes: Vec<u8>| format!("{:x}", Sha256::digest(bytes));
    assert_eq!(result.mesh.positions.len() as u64, expected["vertices"].as_u64().unwrap());
    assert_eq!(result.mesh.triangles.len() as u64, expected["triangles"].as_u64().unwrap());
    assert_eq!(result.report.contractions.len() as u64, expected["contractions"].as_u64().unwrap());
    assert_eq!(
        hash(
            result
                .mesh
                .positions
                .iter()
                .flat_map(|p| [p.x, p.y, p.z])
                .flat_map(|v| v.to_bits().to_le_bytes())
                .collect()
        ),
        expected["positions_bits_sha256"].as_str().unwrap()
    );
    assert_eq!(
        hash(result.mesh.triangles.iter().flatten().flat_map(|v| v.to_le_bytes()).collect()),
        expected["triangles_sha256"].as_str().unwrap()
    );
    assert_eq!(
        hash(result.face_source_indices.iter().flat_map(|&v| (v as u64).to_le_bytes()).collect()),
        expected["face_sources_sha256"].as_str().unwrap()
    );
    assert_eq!(
        hash(result.original_to_final.iter().flat_map(|v| v.to_le_bytes()).collect()),
        expected["original_to_final_sha256"].as_str().unwrap()
    );
    let mut contraction_bytes = Vec::new();
    for c in &result.report.contractions {
        contraction_bytes.extend(c.removed_vertex.to_le_bytes());
        contraction_bytes.extend(c.retained_vertex.to_le_bytes());
        contraction_bytes.extend(c.max_displacement_m.to_bits().to_le_bytes());
    }
    assert_eq!(hash(contraction_bytes), expected["contraction_geometry_sha256"].as_str().unwrap());
    assert_eq!(result.report.max_displacement_m.to_bits(), expected["max_displacement_f64_bits"].as_u64().unwrap());
    assert!(!result.report.contractions.is_empty());
    assert_eq!(result.report.topology_snapshots, 1);
    assert!(!result.face_source_indices.contains(&190));
    assert!(result.report.max_displacement_m < 0.000001);
    assert_correspondence(&mesh, &result.mesh, &result.original_to_final, &result.face_source_indices, 0.000049);
    let exact = condition(mesh.clone(), Options { max_error_m: 0.000049, max_work: result.report.work }).unwrap();
    assert_eq!(exact.mesh, result.mesh);
    assert_eq!(exact.face_source_indices, result.face_source_indices);
    assert_eq!(exact.original_to_final, result.original_to_final);
    assert_eq!(exact.report.work, result.report.work);
    let short = condition(mesh, Options { max_error_m: 0.000049, max_work: result.report.work - 1 }).err().unwrap();
    assert!(short.message.contains("work budget"), "{short}");
    assert!(short.work < result.report.work);

    if let Ok(path) = std::env::var("MM3E_CONDITION_DIAGNOSTIC_OUTPUT") {
        use std::io::Write;
        let output = serde_json::json!({"positions":result.mesh.positions.iter().map(|p|[p.x,p.y,p.z]).collect::<Vec<_>>(),"triangles":result.mesh.triangles,"face_source_indices":result.face_source_indices,"original_to_final":result.original_to_final,"spacing":[2./24.,2./24.,1./24.],"max_residual":0.0005000000237487257,"report":result.report});
        let mut file = std::fs::OpenOptions::new().create_new(true).write(true).open(path).unwrap();
        file.write_all(serde_json::to_vec_pretty(&output).unwrap().as_slice()).unwrap();
    }
}
