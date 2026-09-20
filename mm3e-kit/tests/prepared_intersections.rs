use mm3e_kit::{
    surface_intersections::{validate_counted, PreparedIntersections},
    Vec3,
};
fn triangle(x: f32) -> [Vec3; 3] {
    [Vec3::new(x, 0., 0.), Vec3::new(x + 1., 0., 0.), Vec3::new(x, 1., 0.)]
}
fn scene(xs: &[f32]) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let mut p = Vec::new();
    let mut t = Vec::new();
    for &x in xs {
        let i = p.len() as u32;
        p.extend(triangle(x));
        t.push([i, i + 1, i + 2]);
    }
    (p, t)
}
fn move_triangle(p: &mut [Vec3], id: usize, delta: Vec3) {
    for p in &mut p[3 * id..3 * id + 3] {
        *p = *p + delta;
    }
}
#[test]
fn changed_geometry_hits_unchanged_baseline_and_retains_pair_and_work() {
    let (p, t) = scene(&[0., 3.]);
    let prepared = PreparedIntersections::new(&p, &t, 1_000_000).unwrap();
    let mut candidate = p;
    move_triangle(&mut candidate, 1, Vec3::new(-2.75, 0.25, 0.));
    let failure = prepared.check_candidate(&candidate, 1_000_000).unwrap_err();
    assert_eq!(failure.triangles, Some([0, 1]));
    assert!(failure.work > 0);
    let exhausted = prepared.check_candidate(&candidate, failure.work - 1).unwrap_err();
    assert_eq!(exhausted.triangles, None);
    assert!(exhausted.message.contains("budget"));
    assert!(exhausted.work < failure.work);
}
#[test]
fn all_accumulated_edits_are_checked_against_each_other_not_stale_boxes() {
    let (p, t) = scene(&[0., 10., 20.]);
    let prepared = PreparedIntersections::new(&p, &t, 1_000_000).unwrap();
    let mut first = p.clone();
    move_triangle(&mut first, 0, Vec3::new(5., 0., 0.));
    let first_report = prepared.check_candidate(&first, 1_000_000).unwrap();
    assert_eq!(first_report.changed_triangles, 1);
    let mut second = first;
    move_triangle(&mut second, 1, Vec3::new(-4.75, 0.25, 0.));
    let failure = prepared.check_candidate(&second, 1_000_000).unwrap_err();
    assert_eq!(failure.triangles, Some([0, 1]));
    assert!(
        prepared.check_candidate(&p, 1_000_000).unwrap().changed_triangles == 0,
        "baseline was mutated by a candidate"
    );
}
#[test]
fn untouched_baseline_contacts_remain_pending_until_complete_validation() {
    let (p, t) = scene(&[0., 0., 10.]);
    let prepared = PreparedIntersections::new(&p, &t, 1_000_000).unwrap();
    assert_eq!(prepared.check_candidate(&p, 1_000_000).unwrap().changed_triangles, 0);
    let mut pending = p.clone();
    move_triangle(&mut pending, 2, Vec3::new(2., 0., 0.));
    let report = prepared.check_candidate(&pending, 1_000_000).unwrap();
    assert_eq!(report.changed_triangles, 1);
    assert_eq!(validate_counted(&pending, &t, 1_000_000).unwrap_err().triangles, Some([0, 1]));
    move_triangle(&mut pending, 1, Vec3::new(3., 0., 0.));
    assert_eq!(prepared.check_candidate(&pending, 1_000_000).unwrap().changed_triangles, 2);
    validate_counted(&pending, &t, 1_000_000).unwrap();
}
#[test]
fn every_incident_face_is_derived_and_valid_shared_simplices_survive() {
    let p = vec![
        Vec3::new(1., 0., 0.),
        Vec3::new(-1., 0., 0.),
        Vec3::new(0., 1., 0.),
        Vec3::new(0., -1., 0.),
        Vec3::new(0., 0., 1.),
        Vec3::new(0., 0., -1.),
    ];
    let t = vec![[0, 2, 4], [2, 1, 4], [1, 3, 4], [3, 0, 4], [2, 0, 5], [1, 2, 5], [3, 1, 5], [0, 3, 5]];
    let prepared = PreparedIntersections::new(&p, &t, 1_000_000).unwrap();
    assert!(prepared.preparation_work > 0);
    let preparation = prepared.preparation_report().clone();
    assert_eq!(preparation.work, prepared.preparation_work);
    assert_eq!(preparation.bvh_chunks, 1);
    assert_eq!(preparation.candidate_pairs, 0);
    assert!(preparation.predicate_tests >= t.len());
    assert_eq!(
        PreparedIntersections::new(&p, &t, prepared.preparation_work).unwrap().preparation_work,
        prepared.preparation_work
    );
    let failure = PreparedIntersections::new(&p, &t, prepared.preparation_work - 1).unwrap_err();
    assert!(failure.message.contains("budget"));
    let mut candidate = p;
    candidate[0].x = 0.9;
    let report = prepared.check_candidate(&candidate, 1_000_000).unwrap();
    assert_eq!(report.changed_vertices, 1);
    assert_eq!(report.changed_triangles, 4);
    assert_eq!(prepared.check_candidate(&candidate, report.work).unwrap(), report);
    assert!(prepared.check_candidate(&candidate, report.work - 1).unwrap_err().message.contains("budget"));
    validate_counted(&candidate, &t, 1_000_000).unwrap();
    assert_eq!(prepared.preparation_report(), &preparation);
}
#[test]
fn changed_face_degeneracy_invalid_dimensions_and_nonfinite_values_reject() {
    let (p, t) = scene(&[0., 3.]);
    let prepared = PreparedIntersections::new(&p, &t, 1_000_000).unwrap();
    let mut collapsed = p.clone();
    collapsed[2] = collapsed[1];
    let failure = prepared.check_candidate(&collapsed, 1_000_000).unwrap_err();
    assert_eq!(failure.triangles, None);
    assert!(failure.message.contains("degenerate"));
    let mut invalid = p.clone();
    invalid[3].z = f32::NAN;
    assert!(prepared.check_candidate(&invalid, 1_000_000).unwrap_err().message.contains("finite"));
    assert!(prepared.check_candidate(&p[..3], 1_000_000).is_err());
    assert!(prepared.check_candidate(&p, 0).is_err());
}

#[test]
fn baseline_contacts_are_exhaustive_immutable_and_never_partial_at_limits() {
    let (p, t) = scene(&[0., 0., 0., 10.]);
    let prepared = PreparedIntersections::new(&p, &t, 1_000_000).unwrap();
    let contacts = prepared.baseline_contacts(3, 1_000_000).unwrap();
    assert_eq!(contacts.pairs, [[0, 1], [0, 2], [1, 2]]);
    assert_eq!(prepared.baseline_contacts(3, contacts.work).unwrap(), contacts);
    let capped = prepared.baseline_contacts(2, 1_000_000).unwrap_err();
    assert!(capped.message.contains("pair limit 2"));
    assert!(capped.work > 0);
    assert_eq!(capped.triangles, None);
    let exhausted = prepared.baseline_contacts(3, contacts.work - 1).unwrap_err();
    assert!(exhausted.message.contains("budget"));
    assert!(exhausted.work < contacts.work);
    assert_eq!(exhausted.triangles, None);
    let mut candidate = p;
    move_triangle(&mut candidate, 1, Vec3::new(3., 0., 0.));
    move_triangle(&mut candidate, 2, Vec3::new(6., 0., 0.));
    prepared.check_candidate(&candidate, 1_000_000).unwrap();
    validate_counted(&candidate, &t, 1_000_000).unwrap();
    assert_eq!(prepared.baseline_contacts(3, 1_000_000).unwrap(), contacts);
    let (p, t) = scene(&[0., 3.]);
    let clean = PreparedIntersections::new(&p, &t, 1_000_000).unwrap();
    assert!(clean.baseline_contacts(0, 1_000_000).unwrap().pairs.is_empty());
}

#[test]
fn final_traversal_requires_a_private_completed_inventory_and_ignores_report_tampering() {
    let (p, t) = scene(&[0., 0., 10.]);
    let prepared = PreparedIntersections::new(&p, &t, 1_000_000).unwrap();
    assert!(prepared
        .validate_final(&p, 1_000_000)
        .unwrap_err()
        .message
        .contains("completed private baseline inventory"));
    assert!(prepared.baseline_contacts(0, 1_000_000).unwrap_err().message.contains("pair limit"));
    assert!(prepared
        .validate_final(&p, 1_000_000)
        .unwrap_err()
        .message
        .contains("completed private baseline inventory"));
    assert!(prepared.baseline_contacts(1, 1).is_err());
    assert!(prepared
        .validate_final(&p, 1_000_000)
        .unwrap_err()
        .message
        .contains("completed private baseline inventory"));
    let mut public_inventory = prepared.baseline_contacts(1, 1_000_000).unwrap();
    public_inventory.pairs.clear();
    public_inventory.work = 0;
    assert_eq!(prepared.validate_final(&p, 1_000_000).unwrap_err().triangles, Some([0, 1]));
    let mut repaired = p.clone();
    move_triangle(&mut repaired, 1, Vec3::new(3., 0., 0.));
    prepared.validate_final(&repaired, 1_000_000).unwrap();
    // A later edit elsewhere cannot hide a reverted, wholly unchanged original contact.
    let mut reverted = p;
    move_triangle(&mut reverted, 2, Vec3::new(3., 0., 0.));
    assert_eq!(prepared.validate_final(&reverted, 1_000_000).unwrap_err().triangles, Some([0, 1]));
}

#[test]
fn complete_final_candidate_pairs_match_uncached_validation_and_report_exact_reuse() {
    let p = vec![
        Vec3::new(1., 0., 0.),
        Vec3::new(-1., 0., 0.),
        Vec3::new(0., 1., 0.),
        Vec3::new(0., -1., 0.),
        Vec3::new(0., 0., 1.),
        Vec3::new(0., 0., -1.),
    ];
    let t = vec![[0, 2, 4], [2, 1, 4], [1, 3, 4], [3, 0, 4], [2, 0, 5], [1, 2, 5], [3, 1, 5], [0, 3, 5]];
    let prepared = PreparedIntersections::new(&p, &t, 1_000_000).unwrap();
    assert!(prepared.baseline_contacts(0, 1_000_000).unwrap().pairs.is_empty());
    let original = prepared.validate_final(&p, 1_000_000).unwrap();
    let uncached = validate_counted(&p, &t, 1_000_000).unwrap();
    assert_eq!(original.candidate_pairs, uncached.candidate_pairs);
    assert_eq!(original.reused_pairs, original.candidate_pairs);
    assert!(original.reused_pairs > 0);
    assert_eq!(original.predicate_tests, prepared.preparation_report().predicate_tests);
    assert!(original.work < uncached.work);
    let mut changed = p;
    changed[0].x = 0.9;
    let complete = prepared.validate_final(&changed, 1_000_000).unwrap();
    let uncached = validate_counted(&changed, &t, 1_000_000).unwrap();
    assert_eq!(complete.candidate_pairs, uncached.candidate_pairs);
    assert!(complete.reused_pairs > 0 && complete.reused_pairs < complete.candidate_pairs);
    assert_eq!(prepared.validate_final(&changed, complete.work).unwrap(), complete);
    let exhausted = prepared.validate_final(&changed, complete.work - 1).unwrap_err();
    assert_eq!(exhausted.triangles, None);
    assert!(exhausted.message.contains("budget") && exhausted.work < complete.work);
    let mut degenerate = changed;
    degenerate[0] = degenerate[2];
    assert!(prepared.validate_final(&degenerate, 1_000_000).unwrap_err().message.contains("degenerate"));
}

#[test]
fn final_bvh_finds_new_contacts_that_were_outside_all_baseline_boxes() {
    let (p, t) = scene(&[0., 10.]);
    let prepared = PreparedIntersections::new(&p, &t, 1_000_000).unwrap();
    assert!(prepared.baseline_contacts(0, 1_000_000).unwrap().pairs.is_empty());
    let mut candidate = p;
    move_triangle(&mut candidate, 1, Vec3::new(-9.75, 0.25, 0.));
    let failure = prepared.validate_final(&candidate, 1_000_000).unwrap_err();
    assert_eq!(failure.triangles, Some([0, 1]));
    assert!(failure.work > 0);
}

fn quad_with_probe(warped: bool) -> (Vec<Vec3>, Vec<[u32; 3]>, Vec<[u32; 3]>) {
    let positions = vec![
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 0., 0.),
        Vec3::new(1., 1., if warped { 1. } else { 0. }),
        Vec3::new(0., 1., 0.),
        Vec3::new(0.55, 0.55, 0.2),
        Vec3::new(0.7, 0.55, 0.2),
        Vec3::new(0.55, 0.7, 0.2),
    ];
    (positions, vec![[0, 1, 2], [0, 2, 3], [4, 5, 6]], vec![[0, 1, 3], [1, 2, 3], [4, 5, 6]])
}

#[test]
fn retopologized_warped_quad_rechecks_new_probe_intersection_without_any_vertex_motion() {
    let (positions, original, flipped) = quad_with_probe(true);
    // The original roof z=min(x,y) lies above the entire z=.2 probe.
    // The flipped roof z=x+y-1 crosses it: probe sums range from1.10 to1.25.
    let original_complete = validate_counted(&positions, &original, 1_000_000).unwrap();
    let uncached_failure = validate_counted(&positions, &flipped, 1_000_000).unwrap_err();
    assert_eq!(uncached_failure.triangles, Some([1, 2]));
    let prepared = PreparedIntersections::new(&positions, &original, 1_000_000).unwrap();
    assert!(prepared.baseline_contacts(0, 1_000_000).unwrap().pairs.is_empty());
    let snapshot = positions.clone();
    let failure = prepared.validate_retopologized(&positions, &flipped, 1_000_000).unwrap_err();
    assert_eq!(failure.triangles, uncached_failure.triangles);
    assert!(failure.work > 0 && failure.message.contains("intersect"));
    assert_eq!(positions, snapshot);
    // A failed candidate must neither rewrite the private triangles nor certify
    // its new contact through an updated baseline cache.
    let unchanged = prepared.validate_final(&positions, 1_000_000).unwrap();
    assert_eq!(unchanged.candidate_pairs, original_complete.candidate_pairs);
    assert_eq!(unchanged.reused_pairs, unchanged.candidate_pairs);
    assert_eq!(prepared.validate_retopologized(&positions, &flipped, 1_000_000).unwrap_err(), failure);
}

#[test]
fn retopologized_coplanar_diagonal_flip_passes_and_index_changed_pairs_are_not_reused() {
    let (positions, original, flipped) = quad_with_probe(false);
    let prepared = PreparedIntersections::new(&positions, &original, 1_000_000).unwrap();
    prepared.baseline_contacts(0, 1_000_000).unwrap();
    let uncached = validate_counted(&positions, &flipped, 1_000_000).unwrap();
    let result = prepared.validate_retopologized(&positions, &flipped, 1_000_000).unwrap();
    assert_eq!(result.candidate_pairs, uncached.candidate_pairs);
    assert!(result.candidate_pairs > 0);
    assert_eq!(result.reused_pairs, 0);
    assert!(result.predicate_tests > prepared.preparation_report().predicate_tests);
    // Even cyclic index changes or reordering the face slots conservatively
    // invalidate reuse; equal geometry alone is not the private proof key.
    let cyclic: Vec<_> = original.iter().map(|&[a, b, c]| [b, c, a]).collect();
    let cyclic_report = prepared.validate_retopologized(&positions, &cyclic, 1_000_000).unwrap();
    assert_eq!(cyclic_report.reused_pairs, 0);
    let swapped = vec![original[1], original[0], original[2]];
    assert_eq!(prepared.validate_retopologized(&positions, &swapped, 1_000_000).unwrap().reused_pairs, 0);
}

#[test]
fn retopologized_unchanged_clones_reuse_only_the_private_complete_baseline_proof() {
    let (mut positions, mut original, _) = quad_with_probe(false);
    let prepared = PreparedIntersections::new(&positions, &original, 1_000_000).unwrap();
    assert!(prepared
        .validate_retopologized(&positions, &original, 1_000_000)
        .unwrap_err()
        .message
        .contains("completed private baseline inventory"));
    let mut inventory = prepared.baseline_contacts(0, 1_000_000).unwrap();
    let expected = prepared.validate_final(&positions, 1_000_000).unwrap();
    let result = prepared.validate_retopologized(&positions.clone(), &original.clone(), 1_000_000).unwrap();
    assert_eq!(result.candidate_pairs, expected.candidate_pairs);
    assert_eq!(result.reused_pairs, result.candidate_pairs);
    assert!(result.reused_pairs > 0);
    assert_eq!(result.predicate_tests, expected.predicate_tests);
    let saved_positions = positions.clone();
    let saved_triangles = original.clone();
    positions.clear();
    original.clear();
    inventory.pairs.push([0, 1]);
    inventory.work = usize::MAX;
    assert_eq!(prepared.validate_retopologized(&saved_positions, &saved_triangles, 1_000_000).unwrap(), result);
    assert!(prepared.baseline_contacts(0, 1_000_000).unwrap().pairs.is_empty());
}

#[test]
fn retopologized_duplicate_coordinate_index_loses_shared_boundary_permission() {
    let (mut positions, original, _) = quad_with_probe(false);
    positions.push(positions[0]); // A distinct, initially unused vertex slot.
    let prepared = PreparedIntersections::new(&positions, &original, 1_000_000).unwrap();
    prepared.baseline_contacts(0, 1_000_000).unwrap();
    let mut changed = original.clone();
    changed[0][0] = 7;
    // All geometric face coordinates and all position bits are unchanged, but
    // faces0/1 now share vertex2 only. Their coincident0-to2 edge is forbidden.
    for (before, after) in original.iter().zip(&changed) {
        assert_eq!(before.map(|i| positions[i as usize]), after.map(|i| positions[i as usize]));
    }
    let uncached = validate_counted(&positions, &changed, 1_000_000).unwrap_err();
    assert_eq!(uncached.triangles, Some([0, 1]));
    let failure = prepared.validate_retopologized(&positions, &changed, 1_000_000).unwrap_err();
    assert_eq!(failure.triangles, uncached.triangles);
    assert!(failure.work > 0);
    prepared.validate_retopologized(&positions, &original, 1_000_000).unwrap();
}

#[test]
fn retopologized_fixed_slots_malformed_indices_and_nonfinite_coordinates_reject() {
    let (positions, original, _) = quad_with_probe(false);
    let prepared = PreparedIntersections::new(&positions, &original, 1_000_000).unwrap();
    prepared.baseline_contacts(0, 1_000_000).unwrap();
    for triangles in [&original[..2], &[original[0], original[1], original[2], original[2]]] {
        let failure = prepared.validate_retopologized(&positions, triangles, 1_000_000).unwrap_err();
        assert!(failure.message.contains("count") || failure.message.contains("topology"), "{}", failure.message);
        assert!(failure.triangles.is_none());
    }
    let mut extra = positions.clone();
    extra.push(Vec3::ZERO);
    for candidate in [&positions[..6], extra.as_slice()] {
        let failure = prepared.validate_retopologized(candidate, &original, 1_000_000).unwrap_err();
        assert!(failure.message.contains("count"), "{}", failure.message);
    }
    let mut invalid = original.clone();
    invalid[0][0] = positions.len() as u32;
    assert!(prepared.validate_retopologized(&positions, &invalid, 1_000_000).is_err());
    invalid[0][0] = u32::MAX;
    assert!(prepared.validate_retopologized(&positions, &invalid, 1_000_000).is_err());
    invalid[0] = [0, 0, 1];
    assert!(prepared
        .validate_retopologized(&positions, &invalid, 1_000_000)
        .unwrap_err()
        .message
        .contains("degenerate"));
    let mut nonfinite = positions.clone();
    nonfinite[0].x = f32::NAN;
    assert!(prepared.validate_retopologized(&nonfinite, &original, 1_000_000).unwrap_err().message.contains("finite"));
    assert!(prepared.validate_retopologized(&positions, &original, 0).is_err());
    prepared.validate_final(&positions, 1_000_000).unwrap();
}

#[test]
fn retopologized_success_and_rejection_budgets_replay_without_mutating_baseline() {
    let (positions, original, flipped) = quad_with_probe(false);
    let prepared = PreparedIntersections::new(&positions, &original, 1_000_000).unwrap();
    prepared.baseline_contacts(0, 1_000_000).unwrap();
    let good = prepared.validate_retopologized(&positions, &flipped, 1_000_000).unwrap();
    assert_eq!(prepared.validate_retopologized(&positions, &flipped, good.work).unwrap(), good);
    let short = prepared.validate_retopologized(&positions, &flipped, good.work - 1).unwrap_err();
    assert!(short.message.contains("budget") && short.work > 0 && short.work < good.work);
    let (warped, original, flipped) = quad_with_probe(true);
    let prepared = PreparedIntersections::new(&warped, &original, 1_000_000).unwrap();
    let baseline = prepared.baseline_contacts(0, 1_000_000).unwrap();
    let failure = prepared.validate_retopologized(&warped, &flipped, 1_000_000).unwrap_err();
    assert_eq!(prepared.validate_retopologized(&warped, &flipped, failure.work).unwrap_err(), failure);
    let shortened = prepared.validate_retopologized(&warped, &flipped, failure.work - 1).unwrap_err();
    assert!(shortened.message.contains("budget") && shortened.work < failure.work);
    assert!(shortened.triangles.is_none());
    let total = failure.work * 2 - 1;
    let retried = prepared.validate_retopologized(&warped, &flipped, total - failure.work).unwrap_err();
    assert!(retried.message.contains("budget") && failure.work + retried.work <= total);
    assert_eq!(prepared.baseline_contacts(0, 1_000_000).unwrap(), baseline);
    prepared.validate_final(&warped, 1_000_000).unwrap();
}

#[test]
fn retopologized_coordinate_only_change_invalidates_reuse_even_with_identical_index_triples() {
    let (positions, triangles, _) = quad_with_probe(false);
    let prepared = PreparedIntersections::new(&positions, &triangles, 1_000_000).unwrap();
    prepared.baseline_contacts(0, 1_000_000).unwrap();
    let mut changed = positions.clone();
    for p in &mut changed[4..7] {
        p.z = 0.;
    }
    let uncached = validate_counted(&changed, &triangles, 1_000_000).unwrap_err();
    let cached = prepared.validate_retopologized(&changed, &triangles, 1_000_000).unwrap_err();
    assert_eq!(cached.triangles, uncached.triangles);
    assert!(cached.work > 0);
    prepared.validate_retopologized(&positions, &triangles, 1_000_000).unwrap();
}

#[test]
fn retopologized_edit_elsewhere_cannot_hide_an_untouched_original_contact() {
    let (positions, triangles) = scene(&[0., 0., 10.]);
    let prepared = PreparedIntersections::new(&positions, &triangles, 1_000_000).unwrap();
    let inventory = prepared.baseline_contacts(1, 1_000_000).unwrap();
    assert_eq!(inventory.pairs, [[0, 1]]);
    let mut changed = triangles.clone();
    changed[2] = [7, 8, 6];
    let failure = prepared.validate_retopologized(&positions, &changed, 1_000_000).unwrap_err();
    assert_eq!(failure.triangles, Some([0, 1]));
    assert_eq!(prepared.baseline_contacts(1, 1_000_000).unwrap(), inventory);
}
