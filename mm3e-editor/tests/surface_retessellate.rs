use mm3e_editor::surface_retessellate::{retessellate, Options};
use mm3e_kit::{
    meshing::{extract_isosurface, Mesh},
    Vec3,
};
fn options() -> Options {
    Options { max_work: 100_000_000, max_passes: 32, max_candidates: 1_000_000 }
}
fn prism(concave: bool) -> Mesh {
    let mut mesh = extract_isosurface(|p| p.length() - 1., Vec3::splat(-2.), Vec3::splat(2.), [8; 3]).unwrap();
    let epsilon = if concave { 1. / 1024. } else { -1. / 1024. };
    mesh.positions =
        vec![Vec3::new(0., 0., 0.), Vec3::new(0.5, epsilon, 0.), Vec3::new(1., 0., 0.), Vec3::new(0.5, 1., 0.)];
    mesh.positions.extend(mesh.positions.clone().into_iter().map(|p| Vec3::new(p.x, p.y, -1.)));
    let top = if concave { [[0, 1, 3], [1, 2, 3]] } else { [[0, 1, 2], [0, 2, 3]] };
    mesh.triangles = top.to_vec();
    mesh.triangles.extend(top.map(|[a, b, c]| [a + 4, c + 4, b + 4]));
    for a in 0..4 {
        let b = (a + 1) % 4;
        mesh.triangles.extend([[b, a, a + 4], [b, a + 4, b + 4]]);
    }
    mesh
}
fn parents(mesh: &Mesh) -> Vec<Vec<usize>> {
    (0..mesh.triangles.len()).map(|i| vec![i]).collect()
}
#[test]
fn strict_convex_coplanar_flip_preserves_patch_and_all_ancestors() {
    let mesh = prism(false);
    let result = retessellate(mesh.clone(), parents(&mesh), options()).unwrap();
    assert_eq!(result.mesh.positions, mesh.positions);
    assert_eq!(result.report.geometric_displacement_m, 0.);
    assert!(result.report.flips.iter().any(|flip| flip.faces == [0, 1]));
    assert_eq!(result.face_ancestors[0], [0, 1]);
    assert_eq!(result.face_ancestors[1], [0, 1]);
    for flip in &result.report.flips {
        assert!(flip.new_min_quality_lower > flip.old_min_quality_upper);
    }
    let exact =
        retessellate(mesh.clone(), parents(&mesh), Options { max_work: result.report.work, ..options() }).unwrap();
    assert_eq!(exact.mesh, result.mesh);
    assert_eq!(exact.face_ancestors, result.face_ancestors);
    assert_eq!(exact.report.work, result.report.work);
    let failure = retessellate(mesh.clone(), parents(&mesh), Options { max_work: result.report.work - 1, ..options() })
        .err()
        .unwrap();
    assert!(failure.message.contains("work budget"), "{failure}");
    assert!(failure.work < result.report.work);
}
#[test]
fn arbitrary_exact_plane_flips_but_a_true_subnormal_nonplanarity_does_not() {
    let mut mesh = prism(false);
    for p in &mut mesh.positions {
        p.z += p.x + 2. * p.y;
    }
    let result = retessellate(mesh.clone(), parents(&mesh), options()).unwrap();
    assert!(result.report.flips.iter().any(|flip| flip.faces == [0, 1]));
    mesh.positions[0].z = f32::from_bits(1);
    let result = retessellate(mesh.clone(), parents(&mesh), options()).unwrap();
    assert!(result.report.rejected_nonplanar > 0);
    assert!(!result.report.flips.iter().any(|flip| flip.faces == [0, 1]));
    assert_eq!(&result.mesh.triangles[..2], &mesh.triangles[..2]);
}
#[test]
fn concave_boundary_existing_diagonal_and_ancestry_bound_cannot_be_bypassed() {
    let mesh = prism(true);
    let result = retessellate(mesh.clone(), parents(&mesh), options()).unwrap();
    assert!(result.report.rejected_nonconvex > 0);
    assert!(!result.report.flips.iter().any(|flip| flip.faces == [0, 1]));
    let mut tetra = prism(false);
    tetra.positions = vec![Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.), Vec3::new(0., 0., 1.)];
    tetra.triangles = vec![[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]];
    let result = retessellate(tetra.clone(), parents(&tetra), options()).unwrap();
    assert!(result.report.rejected_existing_diagonal > 0);
    assert!(result.report.flips.is_empty());
    assert_eq!(result.mesh.triangles, tetra.triangles);
    let mesh = prism(false);
    let mut ancestry = parents(&mesh);
    ancestry[0] = (0..128).collect();
    ancestry[1] = (128..256).collect();
    let result = retessellate(mesh.clone(), ancestry.clone(), options()).unwrap();
    assert!(result.report.rejected_ancestry_bound > 0);
    assert_eq!(&result.face_ancestors[..2], &ancestry[..2]);
    assert_eq!(&result.mesh.triangles[..2], &mesh.triangles[..2]);
}
#[test]
fn candidate_pass_and_malformed_ancestry_limits_return_no_partial_mesh() {
    let mesh = prism(false);
    for opts in [Options { max_candidates: 1, ..options() }, Options { max_passes: 1, ..options() }] {
        let error = retessellate(mesh.clone(), parents(&mesh), opts).err().unwrap();
        assert!(error.message.contains("limit"), "{error}");
        assert!(error.work > 0);
    }
    for ancestry in [vec![], vec![vec![]; mesh.triangles.len()], vec![vec![usize::MAX]; mesh.triangles.len()]] {
        assert!(retessellate(mesh.clone(), ancestry, options()).is_err());
    }
}
#[test]
fn frozen_conditioned_bundle_retessellates_both_original_locked_pairs() {
    let value: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/surface_retessellate/conditioned-bundle.json")).unwrap();
    let mut mesh = prism(false);
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
    let ancestry: Vec<Vec<usize>> = serde_json::from_value(value["face_ancestors"].clone()).unwrap();
    let result = retessellate(mesh.clone(), ancestry.clone(), options()).unwrap();
    assert_eq!(result.mesh.positions, mesh.positions);
    for pair in [[4470, 4471], [6225, 6226]] {
        assert!(result.report.flips.iter().any(|f| f.faces == pair), "missing locked pair {pair:?}");
    }
    assert!(result.face_ancestors[4470].contains(&4626) && result.face_ancestors[4470].contains(&4630));
    assert!(result.face_ancestors[4471].contains(&4626) && result.face_ancestors[4471].contains(&4630));
    assert!(result.face_ancestors[6225].contains(&6453) && result.face_ancestors[6225].contains(&6457));
    assert!(result.face_ancestors[6226].contains(&6453) && result.face_ancestors[6226].contains(&6457));
    println!(
        "retessellated bundle: {} flips / {} passes / {} candidates / {} work",
        result.report.flips.len(),
        result.report.passes,
        result.report.candidates,
        result.report.work
    );
    let input_proof =
        mm3e_kit::surface_intersections::validate_certified(&mesh.positions, &mesh.triangles, options().max_work)
            .unwrap();
    let certified = mm3e_editor::surface_retessellate::retessellate_certified(
        mesh.clone(),
        ancestry.clone(),
        options(),
        Some(&input_proof.certificate),
    )
    .unwrap();
    assert_eq!(certified.mesh, result.mesh);
    assert_eq!(certified.face_ancestors, result.face_ancestors);
    assert_eq!(
        serde_json::to_value(&certified.report.flips).unwrap(),
        serde_json::to_value(&result.report.flips).unwrap()
    );
    assert!(certified.report.initial_embedding_reused);
    assert!(
        certified
            .embedding_certificate
            .as_ref()
            .unwrap()
            .matches(&certified.mesh.positions, &certified.mesh.triangles, options().max_work)
            .unwrap()
            .matches
    );
    assert!(certified.report.work < result.report.work);
    println!("certified exact frozen work {} vs standalone {}", certified.report.work, result.report.work);
    let replay =
        retessellate(mesh.clone(), ancestry.clone(), Options { max_work: result.report.work, ..options() }).unwrap();
    assert_eq!(replay.mesh, result.mesh);
    assert_eq!(replay.face_ancestors, result.face_ancestors);
    assert_eq!(replay.report.work, result.report.work);
    let one_less =
        retessellate(mesh, ancestry, Options { max_work: result.report.work - 1, ..options() }).err().unwrap();
    assert!(one_less.message.contains("work budget"), "{one_less}");
    assert!(one_less.work < result.report.work);
    if let Ok(path) = std::env::var("MM3E_RETESSELLATE_DIAGNOSTIC_OUTPUT") {
        use std::io::Write;
        let output = serde_json::json!({"positions":result.mesh.positions.iter().map(|p|[p.x,p.y,p.z]).collect::<Vec<_>>(),"triangles":result.mesh.triangles,"face_ancestors":result.face_ancestors,"spacing":[2./24.,2./24.,1./24.],"max_residual":0.0005000000237487257,"report":result.report});
        let mut file = std::fs::OpenOptions::new().create_new(true).write(true).open(path).unwrap();
        file.write_all(&serde_json::to_vec_pretty(&output).unwrap()).unwrap();
    }
}

fn warped_prism() -> Mesh {
    let mut mesh = prism(false);
    mesh.positions[3].z = 0.5;
    mesh
}
#[test]
fn bounded_warp_uses_explicit_allowance_and_exact_work_boundary() {
    use mm3e_editor::surface_retessellate::retessellate_bounded;
    let mesh = warped_prism();
    let zero = retessellate_bounded(mesh.clone(), parents(&mesh), options(), 0.).unwrap();
    assert_eq!(zero.mesh.triangles, mesh.triangles);
    assert!(zero.report.flips.is_empty());
    assert_eq!(zero.report.max_surface_error_m, 0.);
    let output = retessellate_bounded(mesh.clone(), parents(&mesh), options(), 0.001).unwrap();
    assert_eq!(output.mesh.positions, mesh.positions);
    assert!(output.report.flips.iter().any(|f| f.faces == [0, 1]));
    assert!(output.report.max_surface_error_m > 0. && output.report.max_surface_error_m <= 0.001);
    assert_eq!(output.face_ancestors[0], vec![0, 1]);
    assert_eq!(output.face_ancestors[1], vec![0, 1]);
    let exact = retessellate_bounded(
        mesh.clone(),
        parents(&mesh),
        Options { max_work: output.report.work, ..options() },
        0.001,
    )
    .unwrap();
    assert_eq!(exact.mesh, output.mesh);
    assert_eq!(exact.face_ancestors, output.face_ancestors);
    assert_eq!(exact.face_error_bounds_m, output.face_error_bounds_m);
    assert_eq!(exact.report.work, output.report.work);
    let failure = retessellate_bounded(
        mesh.clone(),
        parents(&mesh),
        Options { max_work: output.report.work - 1, ..options() },
        0.001,
    )
    .err()
    .unwrap();
    assert!(failure.message.contains("work budget"), "{failure}");
    assert!(failure.work < output.report.work);
    for bad in [-1., f64::NAN, f64::INFINITY] {
        assert!(retessellate_bounded(mesh.clone(), parents(&mesh), options(), bad).is_err());
    }
}
#[test]
fn disjoint_warped_patches_take_maximum_instead_of_summing_errors() {
    use mm3e_editor::surface_retessellate::retessellate_bounded;
    let mesh = warped_prism();
    let single = retessellate_bounded(mesh.clone(), parents(&mesh), options(), 0.001).unwrap();
    let mut both = mesh.clone();
    both.positions.extend(mesh.positions.iter().map(|p| Vec3::new(p.x + 4., p.y, p.z)));
    both.triangles.extend(mesh.triangles.iter().map(|t| t.map(|v| v + 8)));
    let output = retessellate_bounded(both.clone(), parents(&both), options(), 0.001).unwrap();
    assert_eq!(output.report.flips.len(), single.report.flips.len() * 2);
    // Translated interval arithmetic can have a slightly wider bound; it is
    // still below twice the single-patch bound and must equal the face maximum.
    assert!(output.report.max_surface_error_m < single.report.max_surface_error_m * 1.01);
    assert_eq!(output.report.max_surface_error_m, output.face_error_bounds_m.iter().copied().fold(0., f64::max));
}
#[test]
fn colliding_warp_rolls_back_geometry_ancestry_and_error_bounds() {
    use mm3e_editor::surface_retessellate::retessellate_bounded;
    let mut mesh = warped_prism();
    let center = Vec3::new(0.5, 0., 0.5 * (1. / 1024.) / (1. + 1. / 1024.));
    let radius = 0.0001;
    mesh.positions.extend([
        Vec3::new(center.x + radius, center.y, center.z),
        Vec3::new(center.x - radius, center.y, center.z),
        Vec3::new(center.x, center.y + radius, center.z),
        Vec3::new(center.x, center.y - radius, center.z),
        Vec3::new(center.x, center.y, center.z + radius),
        Vec3::new(center.x, center.y, center.z - radius),
    ]);
    mesh.triangles.extend(
        [[0, 2, 4], [2, 1, 4], [1, 3, 4], [3, 0, 4], [2, 0, 5], [1, 2, 5], [3, 1, 5], [0, 3, 5]]
            .map(|t| t.map(|v| v + 8)),
    );
    let output = retessellate_bounded(mesh.clone(), parents(&mesh), options(), 0.001).unwrap();
    assert!(output.report.rolled_back_intersections > 0);
    assert_eq!(output.mesh.positions, mesh.positions);
    assert_eq!(output.mesh.triangles, mesh.triangles);
    assert_eq!(output.face_ancestors, parents(&mesh));
    assert!(output.face_error_bounds_m.iter().all(|&v| v == 0.));
    assert_eq!(output.report.max_surface_error_m, 0.);
    assert!(output.report.embedding_validation_work > output.report.final_embedding_work);
    assert_eq!(output.report.quality_cache_rollbacks, output.report.rolled_back_intersections * 2);
    let input_proof =
        mm3e_kit::surface_intersections::validate_certified(&mesh.positions, &mesh.triangles, options().max_work)
            .unwrap();
    let certified = mm3e_editor::surface_retessellate::retessellate_bounded_certified(
        mesh.clone(),
        parents(&mesh),
        options(),
        0.001,
        Some(&input_proof.certificate),
    )
    .unwrap();
    assert_eq!(certified.mesh, output.mesh);
    assert_eq!(certified.face_ancestors, output.face_ancestors);
    assert_eq!(certified.face_error_bounds_m, output.face_error_bounds_m);
    assert_eq!(certified.report.rolled_back_intersections, output.report.rolled_back_intersections);
    assert_eq!(certified.report.quality_cache_rollbacks, output.report.quality_cache_rollbacks);
    assert!(certified.report.initial_embedding_reused && certified.report.final_embedding_reused);
    assert_eq!(certified.report.certificate_snapshot_work, 0);
    assert!(
        certified
            .embedding_certificate
            .as_ref()
            .unwrap()
            .matches(&mesh.positions, &mesh.triangles, options().max_work)
            .unwrap()
            .matches
    );
    let replay = retessellate_bounded(
        mesh.clone(),
        parents(&mesh),
        Options { max_work: output.report.work, ..options() },
        0.001,
    )
    .unwrap();
    assert_eq!(replay.mesh, output.mesh);
    assert_eq!(replay.face_ancestors, output.face_ancestors);
    assert_eq!(replay.face_error_bounds_m, output.face_error_bounds_m);
    assert_eq!(replay.report.work, output.report.work);
    assert_eq!(replay.report.rolled_back_intersections, output.report.rolled_back_intersections);
    let short = retessellate_bounded(
        mesh.clone(),
        parents(&mesh),
        Options { max_work: output.report.work - 1, ..options() },
        0.001,
    )
    .err()
    .unwrap();
    assert!(short.message.contains("work budget"), "{short}");
    assert!(short.work < output.report.work);
}
#[test]
fn frozen_exact_retessellated_bundle_repairs_all_four_remaining_warped_slivers() {
    use mm3e_editor::surface_retessellate::retessellate_bounded;
    let value: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/surface_retessellate/exact-retessellated-bundle.json")).unwrap();
    let mut mesh = prism(false);
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
    let ancestry: Vec<Vec<usize>> = serde_json::from_value(value["face_ancestors"].clone()).unwrap();
    let output = retessellate_bounded(mesh.clone(), ancestry.clone(), options(), 0.000049).unwrap();
    assert_eq!(output.mesh.positions, mesh.positions);
    for face in [2029, 2205, 4173, 4347] {
        assert!(output.report.flips.iter().any(|f| f.faces.contains(&face)), "unrepaired source face {face}");
    }
    assert!(output.report.max_surface_error_m > 0. && output.report.max_surface_error_m <= 0.000049);
    println!(
        "bounded bundle {} flips / {} passes / {} work / {}m maximum",
        output.report.flips.len(),
        output.report.passes,
        output.report.work,
        output.report.max_surface_error_m
    );
    let input_proof =
        mm3e_kit::surface_intersections::validate_certified(&mesh.positions, &mesh.triangles, options().max_work)
            .unwrap();
    let certified = mm3e_editor::surface_retessellate::retessellate_bounded_certified(
        mesh.clone(),
        ancestry.clone(),
        options(),
        0.000049,
        Some(&input_proof.certificate),
    )
    .unwrap();
    assert_eq!(certified.mesh, output.mesh);
    assert_eq!(certified.face_ancestors, output.face_ancestors);
    assert_eq!(certified.face_error_bounds_m, output.face_error_bounds_m);
    assert_eq!(
        serde_json::to_value(&certified.report.flips).unwrap(),
        serde_json::to_value(&output.report.flips).unwrap()
    );
    assert!(certified.report.initial_embedding_reused && certified.report.final_embedding_reused);
    assert!(
        certified
            .embedding_certificate
            .as_ref()
            .unwrap()
            .matches(&certified.mesh.positions, &certified.mesh.triangles, options().max_work)
            .unwrap()
            .matches
    );
    assert!(certified.report.work < output.report.work);
    println!("certified bounded frozen work {} vs standalone {}", certified.report.work, output.report.work);
    let exact = retessellate_bounded(
        mesh.clone(),
        ancestry.clone(),
        Options { max_work: output.report.work, ..options() },
        0.000049,
    )
    .unwrap();
    assert_eq!(exact.mesh, output.mesh);
    assert_eq!(exact.face_ancestors, output.face_ancestors);
    assert_eq!(exact.face_error_bounds_m, output.face_error_bounds_m);
    assert_eq!(exact.report.work, output.report.work);
    let one_less =
        retessellate_bounded(mesh, ancestry, Options { max_work: output.report.work - 1, ..options() }, 0.000049)
            .err()
            .unwrap();
    assert!(one_less.message.contains("work budget"), "{one_less}");
    assert!(one_less.work < output.report.work);
    if let Ok(path) = std::env::var("MM3E_WARPED_DIAGNOSTIC_OUTPUT") {
        use std::io::Write;
        let value = serde_json::json!({"positions":output.mesh.positions.iter().map(|p|[p.x,p.y,p.z]).collect::<Vec<_>>(),"triangles":output.mesh.triangles,"face_ancestors":output.face_ancestors,"face_error_bounds_m":output.face_error_bounds_m,"spacing":[2./24.,2./24.,1./24.],"max_residual":0.0005000000237487257,"report":output.report});
        let mut file = std::fs::OpenOptions::new().create_new(true).write(true).open(path).unwrap();
        file.write_all(&serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    }
}

#[test]
fn exact_certified_path_checks_input_identity_and_issues_final_snapshot() {
    use mm3e_editor::surface_retessellate::retessellate_certified;
    use mm3e_kit::surface_intersections::validate_certified;
    let mesh = prism(false);
    let proof = validate_certified(&mesh.positions, &mesh.triangles, options().max_work).unwrap();
    let standalone = retessellate(mesh.clone(), parents(&mesh), options()).unwrap();
    assert!(standalone.embedding_certificate.is_none());
    let output = retessellate_certified(mesh.clone(), parents(&mesh), options(), Some(&proof.certificate)).unwrap();
    assert!(output.report.initial_embedding_reused);
    assert!(output.report.initial_certificate_match_work > 0);
    assert!(output.report.output_certificate_work > 0);
    assert_eq!(output.mesh, standalone.mesh);
    assert_eq!(output.face_ancestors, standalone.face_ancestors);
    assert!(
        !proof.certificate.matches(&output.mesh.positions, &output.mesh.triangles, options().max_work).unwrap().matches
    );
    assert!(
        output
            .embedding_certificate
            .as_ref()
            .unwrap()
            .matches(&output.mesh.positions, &output.mesh.triangles, options().max_work)
            .unwrap()
            .matches
    );
    let replay = retessellate_certified(
        mesh.clone(),
        parents(&mesh),
        Options { max_work: output.report.work, ..options() },
        Some(&proof.certificate),
    )
    .unwrap();
    assert_eq!(replay.mesh, output.mesh);
    assert_eq!(replay.report.work, output.report.work);
    let short = retessellate_certified(
        mesh.clone(),
        parents(&mesh),
        Options { max_work: output.report.work - 1, ..options() },
        Some(&proof.certificate),
    )
    .err()
    .unwrap();
    assert!(short.message.contains("budget"), "{short}");
    assert!(short.work < output.report.work);
    let mut reordered = mesh.clone();
    reordered.triangles.swap(0, 1);
    let changed =
        retessellate_certified(reordered.clone(), parents(&reordered), options(), Some(&proof.certificate)).unwrap();
    assert!(!changed.report.initial_embedding_reused);
    assert!(changed.report.initial_embedding_work > changed.report.initial_certificate_match_work);
    let fresh = retessellate(reordered.clone(), parents(&reordered), options()).unwrap();
    assert_eq!(changed.mesh, fresh.mesh);
    let mut moved = mesh;
    moved.positions[0].z = 0.001;
    let changed = retessellate_certified(moved.clone(), parents(&moved), options(), Some(&proof.certificate)).unwrap();
    assert!(!changed.report.initial_embedding_reused);
    assert!(
        changed
            .embedding_certificate
            .as_ref()
            .unwrap()
            .matches(&changed.mesh.positions, &changed.mesh.triangles, options().max_work)
            .unwrap()
            .matches
    );
}
#[test]
fn bounded_certified_path_preserves_batch_proofs_and_charges_all_matches() {
    use mm3e_editor::surface_retessellate::{retessellate_bounded, retessellate_bounded_certified};
    use mm3e_kit::surface_intersections::validate_certified;
    let mesh = warped_prism();
    let proof = validate_certified(&mesh.positions, &mesh.triangles, options().max_work).unwrap();
    let standalone = retessellate_bounded(mesh.clone(), parents(&mesh), options(), 0.001).unwrap();
    assert!(standalone.embedding_certificate.is_none());
    let output =
        retessellate_bounded_certified(mesh.clone(), parents(&mesh), options(), 0.001, Some(&proof.certificate))
            .unwrap();
    assert!(output.report.initial_embedding_reused && output.report.final_embedding_reused);
    assert!(output.report.certificate_snapshot_work > 0);
    assert_eq!(output.mesh, standalone.mesh);
    assert_eq!(output.face_ancestors, standalone.face_ancestors);
    assert_eq!(output.face_error_bounds_m, standalone.face_error_bounds_m);
    assert!(
        output
            .embedding_certificate
            .as_ref()
            .unwrap()
            .matches(&output.mesh.positions, &output.mesh.triangles, options().max_work)
            .unwrap()
            .matches
    );
    let exact = retessellate_bounded_certified(
        mesh.clone(),
        parents(&mesh),
        Options { max_work: output.report.work, ..options() },
        0.001,
        Some(&proof.certificate),
    )
    .unwrap();
    assert_eq!(exact.mesh, output.mesh);
    assert_eq!(exact.report.work, output.report.work);
    let short = retessellate_bounded_certified(
        mesh.clone(),
        parents(&mesh),
        Options { max_work: output.report.work - 1, ..options() },
        0.001,
        Some(&proof.certificate),
    )
    .err()
    .unwrap();
    assert!(short.message.contains("budget"), "{short}");
    assert!(short.work < output.report.work);
    let unchanged =
        retessellate_bounded_certified(mesh.clone(), parents(&mesh), options(), 0., Some(&proof.certificate)).unwrap();
    assert_eq!(unchanged.mesh.triangles, mesh.triangles);
    assert!(unchanged.report.initial_embedding_reused && unchanged.report.final_embedding_reused);
    assert_eq!(unchanged.report.certificate_snapshot_work, 0);
    let other = prism(false);
    let wrong = validate_certified(&other.positions, &other.triangles, options().max_work).unwrap();
    let fallback =
        retessellate_bounded_certified(mesh.clone(), parents(&mesh), options(), 0.001, Some(&wrong.certificate))
            .unwrap();
    assert!(!fallback.report.initial_embedding_reused);
    assert!(fallback.report.final_embedding_reused);
    assert_eq!(fallback.mesh, output.mesh);
    let without = retessellate_bounded_certified(mesh.clone(), parents(&mesh), options(), 0.001, None).unwrap();
    assert_eq!(without.mesh, output.mesh);
    assert!(without.embedding_certificate.is_some());
}
