use mm3e_kit::{
    surface_intersections::{validate_certified, validate_counted, EmbeddingProofMethod, PreparedIntersections},
    Vec3,
};
const WORK: usize = 1_000_000;
fn quad() -> (Vec<Vec3>, Vec<[u32; 3]>) {
    (
        vec![
            Vec3::new(0., 0., 0.),
            Vec3::new(1., 0., 0.),
            Vec3::new(1., 1., 0.),
            Vec3::new(0., 1., 0.),
            Vec3::new(0., 0., 0.),
        ],
        vec![[0, 1, 2], [0, 2, 3]],
    )
}
#[test]
fn complete_full_validation_captures_owned_bits_and_ordered_indices_with_compact_debug() {
    let (mut p, mut t) = quad();
    let old = validate_counted(&p, &t, WORK).unwrap();
    let result = validate_certified(&p, &t, WORK).unwrap();
    assert_eq!(result.report, old);
    assert_eq!(result.work, result.report.work + result.certificate_work);
    assert!(result.certificate_work > 0);
    let certificate = result.certificate.clone();
    assert_eq!(certificate.method(), EmbeddingProofMethod::FullPairs);
    assert_eq!(certificate.original_proof_work(), old.work);
    assert_eq!((certificate.vertex_count(), certificate.triangle_count()), (5, 2));
    assert!(certificate.matches(&p, &t, WORK).unwrap().matches);
    let copy_p = p.clone();
    let copy_t = t.clone();
    p.clear();
    t.clear();
    assert!(certificate.matches(&copy_p, &copy_t, WORK).unwrap().matches);
    let debug = format!("{certificate:?}");
    assert!(debug.len() < 180 && debug.contains("vertices") && !debug.contains("positions"));
}
#[test]
fn stale_bits_index_identity_and_face_order_are_not_authorized_by_geometry_similarity() {
    let (p, t) = quad();
    let result = validate_certified(&p, &t, WORK).unwrap();
    let certificate = result.certificate;
    let mut zero = p.clone();
    zero[0].x = -0.;
    assert_eq!(zero, p);
    assert!(!certificate.matches(&zero, &t, WORK).unwrap().matches);
    let mut moved = p.clone();
    moved[0].z = f32::from_bits(1);
    assert!(!certificate.matches(&moved, &t, WORK).unwrap().matches);
    let mut relabeled = t.clone();
    relabeled[0][0] = 4;
    assert_eq!(t[0].map(|i| p[i as usize]), relabeled[0].map(|i| p[i as usize]));
    assert!(!certificate.matches(&p, &relabeled, WORK).unwrap().matches);
    assert!(validate_counted(&p, &relabeled, WORK).is_err());
    let reversed = vec![t[1], t[0]];
    assert!(!certificate.matches(&p, &reversed, WORK).unwrap().matches);
    assert!(!certificate.matches(&p[..4], &t, WORK).unwrap().matches);
    assert!(!certificate.matches(&p, &t[..1], WORK).unwrap().matches);
    let mut forged_report = certificate.matches(&moved, &t, WORK).unwrap();
    forged_report.matches = true;
    forged_report.work = 0;
    assert!(forged_report.matches);
    let failure = PreparedIntersections::new_certified(&moved, &t, &certificate, WORK).unwrap_err();
    assert!(failure.message.contains("does not match") && failure.work > 0);
    assert!(certificate.matches(&p, &t, WORK).unwrap().matches);
}
#[test]
fn incomplete_and_nonempty_inventories_cannot_issue_baseline_certificates() {
    let (p, t) = quad();
    let prepared = PreparedIntersections::new(&p, &t, WORK).unwrap();
    assert!(prepared.baseline_certificate(WORK).unwrap_err().message.contains("completed private"));
    assert!(prepared.baseline_contacts(0, 1).is_err());
    assert!(prepared.baseline_certificate(WORK).is_err());
    prepared.baseline_contacts(0, WORK).unwrap();
    let capture = prepared.baseline_certificate(WORK).unwrap();
    assert!(capture.certificate.matches(&p, &t, WORK).unwrap().matches);
    assert_eq!(capture.certificate.method(), EmbeddingProofMethod::PreparedBaseline);
    let duplicate = vec![t[0], t[0]];
    assert!(PreparedIntersections::new(&p, &duplicate, WORK).is_err());
    let mut overlapping = p.clone();
    overlapping.extend(t[0].map(|i| p[i as usize]));
    let overlapping_faces = vec![t[0], [5, 6, 7]];
    let bad = PreparedIntersections::new(&overlapping, &overlapping_faces, WORK).unwrap();
    assert!(bad.baseline_contacts(0, WORK).is_err());
    assert!(bad.baseline_certificate(WORK).is_err());
    let mut public = bad.baseline_contacts(1, WORK).unwrap();
    public.pairs.clear();
    assert!(bad.baseline_certificate(WORK).unwrap_err().message.contains("unresolved contacts"));
    assert!(validate_certified(&p, &duplicate, WORK).is_err());
    assert!(validate_certified(&overlapping, &overlapping_faces, WORK).is_err());
}
#[test]
fn certified_preparation_reuses_only_matching_complete_proof_and_preserves_original_provenance() {
    let (p, t) = quad();
    let certified = validate_certified(&p, &t, WORK).unwrap();
    let proof_work = certified.certificate.original_proof_work();
    let matched = certified.certificate.matches(&p, &t, WORK).unwrap();
    let plain = PreparedIntersections::new(&p, &t, WORK).unwrap();
    let prepared = PreparedIntersections::new_certified(&p, &t, &certified.certificate, WORK).unwrap();
    assert_eq!(prepared.preparation_work, matched.work + plain.preparation_work + 1);
    let captured = prepared.baseline_certificate(WORK).unwrap();
    assert_eq!(captured.certificate.original_proof_work(), proof_work);
    let final_report = prepared.validate_final(&p, WORK).unwrap();
    assert!(final_report.reused_pairs > 0);
    assert_eq!(prepared.validate_final_certified(&p, WORK).unwrap().report, final_report);
    assert_eq!(prepared.preparation_report().work, prepared.preparation_work);
}
#[test]
fn certified_retopology_binds_the_actual_new_face_slots_and_rejects_bad_candidates() {
    let (p, t) = quad();
    let input = validate_certified(&p, &t, WORK).unwrap();
    let prepared = PreparedIntersections::new_certified(&p, &t, &input.certificate, WORK).unwrap();
    let next = vec![[0, 1, 3], [1, 2, 3]];
    let result = prepared.validate_retopologized_certified(&p, &next, WORK).unwrap();
    assert_eq!(result.report, prepared.validate_retopologized(&p, &next, WORK).unwrap());
    assert_eq!(result.certificate.method(), EmbeddingProofMethod::PreparedRetopologized);
    assert!(result.certificate.matches(&p, &next, WORK).unwrap().matches);
    assert!(!result.certificate.matches(&p, &t, WORK).unwrap().matches);
    let mut bad = next.clone();
    bad[1] = bad[0];
    assert!(prepared.validate_retopologized_certified(&p, &bad, WORK).is_err());
    assert!(result.certificate.matches(&p, &next, WORK).unwrap().matches);
}
#[test]
fn snapshots_matches_prepared_calls_and_certified_final_work_replay_exactly() {
    let (p, t) = quad();
    let result = validate_certified(&p, &t, WORK).unwrap();
    assert_eq!(validate_certified(&p, &t, result.work).unwrap().work, result.work);
    let short = validate_certified(&p, &t, result.work - 1).unwrap_err();
    assert!(short.message.contains("budget") && short.work < result.work);
    let matched = result.certificate.matches(&p, &t, WORK).unwrap();
    assert_eq!(result.certificate.matches(&p, &t, matched.work).unwrap(), matched);
    let failure = result.certificate.matches(&p, &t, matched.work - 1).unwrap_err();
    assert!(failure.message.contains("budget") && failure.work < matched.work);
    let prepared = PreparedIntersections::new_certified(&p, &t, &result.certificate, WORK).unwrap();
    assert_eq!(
        PreparedIntersections::new_certified(&p, &t, &result.certificate, prepared.preparation_work)
            .unwrap()
            .preparation_work,
        prepared.preparation_work
    );
    assert!(PreparedIntersections::new_certified(&p, &t, &result.certificate, prepared.preparation_work - 1)
        .unwrap_err()
        .message
        .contains("budget"));
    let baseline = prepared.baseline_certificate(WORK).unwrap();
    assert_eq!(prepared.baseline_certificate(baseline.work).unwrap().work, baseline.work);
    assert!(prepared.baseline_certificate(baseline.work - 1).unwrap_err().message.contains("budget"));
    let final_proof = prepared.validate_final_certified(&p, WORK).unwrap();
    assert_eq!(prepared.validate_final_certified(&p, final_proof.work).unwrap().work, final_proof.work);
    let failure = prepared.validate_final_certified(&p, final_proof.work - 1).unwrap_err();
    assert!(failure.message.contains("budget") && failure.work < final_proof.work);
    let next = vec![[0, 1, 3], [1, 2, 3]];
    let retop = prepared.validate_retopologized_certified(&p, &next, WORK).unwrap();
    assert_eq!(prepared.validate_retopologized_certified(&p, &next, retop.work).unwrap().work, retop.work);
    assert!(prepared
        .validate_retopologized_certified(&p, &next, retop.work - 1)
        .unwrap_err()
        .message
        .contains("budget"));
    assert!(result.certificate.matches(&p, &t, WORK).unwrap().matches);
}

#[test]
fn bijective_used_vertex_compaction_rebinds_without_running_geometric_predicates() {
    let (p, t) = quad();
    let original = validate_certified(&p, &t, WORK).unwrap();
    let map = [2, 0, 3, 1, u32::MAX];
    let next_p = vec![p[1], p[3], p[0], p[2]];
    let next_t: Vec<_> = t.iter().map(|t| t.map(|i| map[i as usize])).collect();
    let proof = original.certificate.reindexed(&next_p, &next_t, &map, WORK).unwrap();
    assert_eq!(proof.certificate.method(), EmbeddingProofMethod::ExactReindex);
    assert_eq!(proof.certificate.original_proof_work(), original.report.work);
    assert!(proof.certificate.matches(&next_p, &next_t, WORK).unwrap().matches);
    assert!(!proof.certificate.matches(&p, &t, WORK).unwrap().matches);
    validate_counted(&next_p, &next_t, WORK).unwrap();
    assert_eq!(original.certificate.reindexed(&next_p, &next_t, &map, proof.work).unwrap().work, proof.work);
    let failed = original.certificate.reindexed(&next_p, &next_t, &map, proof.work - 1).unwrap_err();
    assert!(failed.message.contains("budget") && failed.work < proof.work);
    assert!(original.certificate.matches(&p, &t, WORK).unwrap().matches);
    PreparedIntersections::new_certified(&next_p, &next_t, &proof.certificate, WORK)
        .unwrap()
        .validate_final(&next_p, WORK)
        .unwrap();
}
#[test]
fn reindex_cannot_weld_retain_unused_move_coordinates_or_edit_ordered_faces() {
    let (p, t) = quad();
    let original = validate_certified(&p, &t, WORK).unwrap();
    let map = [0, 1, 2, 3, u32::MAX];
    let next = p[..4].to_vec();
    for bad in [[0, 0, 2, 3, u32::MAX], [0, 1, u32::MAX, 3, u32::MAX], [0, 1, 2, 3, 0]] {
        assert!(original.certificate.reindexed(&next, &t, &bad, WORK).is_err());
    }
    assert!(original.certificate.reindexed(&next, &t, &map[..4], WORK).is_err());
    let mut moved = next.clone();
    moved[0].x = -0.;
    assert_eq!(next, moved);
    assert!(original.certificate.reindexed(&moved, &t, &map, WORK).unwrap_err().message.contains("coordinate bits"));
    let mut extra = next.clone();
    extra.push(Vec3::ZERO);
    assert!(original.certificate.reindexed(&extra, &t, &map, WORK).unwrap_err().message.contains("unmatched output"));
    for edited in [vec![t[1], t[0]], vec![[0, 1, 3], [1, 2, 3]], vec![t[0], t[0]]] {
        assert!(original
            .certificate
            .reindexed(&next, &edited, &map, WORK)
            .unwrap_err()
            .message
            .contains("ordered source triangle"));
    }
    assert!(original.certificate.reindexed(&next, &t[..1], &map, WORK).is_err());
    assert!(original.certificate.matches(&p, &t, WORK).unwrap().matches);
}
#[test]
fn represented_box_producer_returns_a_certificate_bound_after_cleanup() {
    use mm3e_kit::meshing_boolean::BooleanExpr;
    use mm3e_kit::meshing_local::{
        extract_local_convex_union_isosurface_with_representation, ConvexRepresentationPolicy, LocalCellData,
        LocalExtractionOptions, LocalFeature,
    };
    let result = extract_local_convex_union_isosurface_with_representation(
        Vec3::splat(-1.),
        Vec3::splat(1.),
        [3; 3],
        LocalExtractionOptions { max_work: 20_000_000, max_vertices: 10000, max_triangles: 20000 },
        ConvexRepresentationPolicy { max_representation_error_m: 1e-5 },
        |cell, _| {
            let mut features = Vec::new();
            for axis in 0..3 {
                for side in [-1., 1.] {
                    let id = features.len();
                    let values = cell.points.map(|p| [p.x, p.y, p.z][axis] * side - 0.4);
                    features.push(LocalFeature { id: id as u64, values });
                }
            }
            let expression = (1..6).fold(BooleanExpr::Leaf(0), |a, i| {
                BooleanExpr::Intersection(Box::new(a), Box::new(BooleanExpr::Leaf(i)))
            });
            Ok(LocalCellData { features, expression: Some(expression), work: 48 })
        },
    )
    .unwrap();
    let representation = result.report.representation.as_ref().unwrap();
    assert!(representation.embedding_certificate_work > 0);
    assert!(representation.embedding_certificate_binding_work > 0);
    assert!(
        representation
            .embedding_certificate
            .matches(&result.mesh.positions, &result.mesh.triangles, WORK)
            .unwrap()
            .matches
    );
    validate_counted(&result.mesh.positions, &result.mesh.triangles, WORK).unwrap();
}

#[test]
fn radial_snapshot_is_issued_only_by_a_complete_same_input_proof_and_seeds_private_reuse() {
    let p = vec![Vec3::new(1., 1., 1.), Vec3::new(-1., -1., 1.), Vec3::new(-1., 1., -1.), Vec3::new(1., -1., -1.)];
    let t = vec![[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]];
    let result = mm3e_kit::radial_embedding::certify_snapshot(&p, &t, WORK).unwrap();
    assert_eq!(result.certificate.method(), EmbeddingProofMethod::Radial);
    assert_eq!(result.work, result.report.work.work + result.certificate_work);
    assert_eq!(result.certificate.original_proof_work(), result.report.work.work);
    assert!(result.certificate.matches(&p, &t, WORK).unwrap().matches);
    let prepared = PreparedIntersections::new_certified(&p, &t, &result.certificate, WORK).unwrap();
    let checked = prepared.validate_final(&p, WORK).unwrap();
    assert!(checked.reused_pairs > 0);
    assert_eq!(mm3e_kit::radial_embedding::certify_snapshot(&p, &t, result.work).unwrap().work, result.work);
    let failure = mm3e_kit::radial_embedding::certify_snapshot(&p, &t, result.work - 1).unwrap_err();
    assert_eq!(failure.kind, mm3e_kit::radial_embedding::FailureKind::Budget);
    assert!(failure.work.work < result.work);
    assert!(mm3e_kit::radial_embedding::certify_snapshot(&p, &t[..3], WORK).is_err());
}

#[test]
fn mutable_public_preparation_and_validation_counters_cannot_forge_certificate_provenance() {
    let (p, t) = quad();
    let mut prepared = PreparedIntersections::new(&p, &t, WORK).unwrap();
    let actual_preparation = prepared.preparation_report().work;
    prepared.preparation_work = usize::MAX;
    let inventory = prepared.baseline_contacts(0, WORK).unwrap();
    let captured = prepared.baseline_certificate(WORK).unwrap();
    assert_eq!(captured.certificate.original_proof_work(), actual_preparation + inventory.work);
    let mut result = validate_certified(&p, &t, WORK).unwrap();
    let proof_work = result.certificate.original_proof_work();
    result.report.work = 0;
    result.work = 0;
    result.certificate_work = 0;
    assert_eq!(result.certificate.original_proof_work(), proof_work);
    let reused = PreparedIntersections::new_certified(&p, &t, &result.certificate, WORK).unwrap();
    assert!(reused.preparation_work > 0);
    assert_eq!(reused.baseline_certificate(WORK).unwrap().certificate.original_proof_work(), proof_work);
}
