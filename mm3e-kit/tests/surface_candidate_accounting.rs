use mm3e_kit::{
    surface::TriangleSurface,
    surface_intersections::{validate_counted, IntersectionFailure, PreparedIntersections},
    Vec3,
};
const WORK: usize = 1_000_000;
fn geometry() -> (Vec<Vec3>, Vec<[u32; 3]>) {
    (vec![Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.)], vec![[0, 1, 2]])
}

#[test]
fn counted_query_retains_every_completed_visit_without_exposing_partial_candidates() {
    let (positions, triangles) = geometry();
    let surface = TriangleSurface::new(positions, triangles, 0.1).unwrap();
    let lo = Vec3::ZERO;
    let hi = Vec3::new(1., 1., 0.);
    // This BVH consists of one leaf node containing exactly one triangle.
    let result = surface.triangle_candidates_counted(lo, hi, 0., 2).unwrap();
    assert_eq!(result.work, 2);
    assert_eq!(result.triangles, vec![0]);
    assert_eq!(surface.triangle_candidates_bounded(lo, hi, 0., 2).unwrap(), result);
    for cap in 0..2 {
        let error = surface.triangle_candidates_counted(lo, hi, 0., cap).unwrap_err();
        assert_eq!(error.work, cap);
        assert_eq!(error.message, format!("surface candidate work budget exceeded ({cap})"));
        assert_eq!(surface.triangle_candidates_bounded(lo, hi, 0., cap).unwrap_err(), error.message);
    }
    // A rejected root box completes after one visit and returns no candidates.
    let outside = surface.triangle_candidates_counted(Vec3::splat(3.), Vec3::splat(4.), 0., 1).unwrap();
    assert_eq!(outside.work, 1);
    assert!(outside.triangles.is_empty());
    assert_eq!(surface.triangle_candidates_counted(lo, hi, 0., 2).unwrap(), result);
}

#[test]
fn invalid_counted_queries_report_zero_traversal_and_preserve_legacy_messages() {
    let (positions, triangles) = geometry();
    let surface = TriangleSurface::new(positions, triangles, 0.1).unwrap();
    for (lo, hi, pad) in [
        (Vec3::splat(f32::NAN), Vec3::ZERO, 0.),
        (Vec3::ZERO, Vec3::splat(f32::INFINITY), 0.),
        (Vec3::splat(1.), Vec3::ZERO, 0.),
        (Vec3::ZERO, Vec3::ZERO, -0.1),
        (Vec3::ZERO, Vec3::ZERO, f32::NAN),
    ] {
        for cap in [0, WORK] {
            let error = surface.triangle_candidates_counted(lo, hi, pad, cap).unwrap_err();
            assert_eq!(error.work, 0);
            assert_eq!(surface.triangle_candidates_bounded(lo, hi, pad, cap).unwrap_err(), error.message);
        }
    }
}

// Each operation below ends with the same independently known one-leaf query:
// two traversal visits followed by one candidate-list processing unit. There
// are no distinct face pairs, so no pair predicate can obscure either boundary.
fn query_boundaries(operation: impl Fn(usize) -> Result<usize, IntersectionFailure>, expected_context: &str) {
    let complete = operation(WORK).unwrap();
    assert_eq!(operation(complete).unwrap(), complete);
    let exhausted_query = complete - 2;
    let error = operation(exhausted_query).unwrap_err();
    assert!(error.message.contains(expected_context), "{error}");
    assert!(error.message.contains("surface candidate work budget exceeded (1)"), "{error}");
    assert_eq!(error.work, exhausted_query, "the last completed BVH visit must remain charged");
    assert_eq!(error.triangles, None);
    let exhausted_processing = complete - 1;
    let error = operation(exhausted_processing).unwrap_err();
    assert!(error.message.contains("surface intersection validation exhausted work budget"), "{error}");
    assert_eq!(error.work, exhausted_processing, "completed query work must survive later processing failure");
    assert_eq!(error.triangles, None);
}

#[test]
fn complete_validation_retains_failed_query_and_completed_query_work() {
    let (positions, triangles) = geometry();
    query_boundaries(
        |cap| validate_counted(&positions, &triangles, cap).map(|report| report.work),
        "surface intersection candidate query failed",
    );
}

#[test]
fn baseline_inventory_retains_query_work_without_publishing_partial_proof() {
    let (positions, triangles) = geometry();
    // Three bounds units + two BVH visits + one candidate + one proof publish.
    for (cap, message) in [(4, "prepared baseline contact query failed"), (5, "validation exhausted work budget")] {
        let prepared = PreparedIntersections::new(&positions, &triangles, WORK).unwrap();
        let error = prepared.baseline_contacts(1, cap).unwrap_err();
        assert!(error.message.contains(message), "{error}");
        assert_eq!(error.work, cap);
        assert_eq!(error.triangles, None);
        assert!(prepared.baseline_certificate(WORK).unwrap_err().message.contains("completed private inventory"));
        let complete = prepared.baseline_contacts(1, 7).unwrap();
        assert_eq!(complete.work, 7);
        assert!(complete.pairs.is_empty());
        assert!(prepared.baseline_certificate(WORK).is_ok());
    }
}

#[test]
fn prepared_final_validation_retains_both_query_failure_boundaries() {
    let (positions, triangles) = geometry();
    let prepared = PreparedIntersections::new(&positions, &triangles, WORK).unwrap();
    prepared.baseline_contacts(1, WORK).unwrap();
    query_boundaries(
        |cap| prepared.validate_final(&positions, cap).map(|report| report.work),
        "prepared final candidate query failed",
    );
    query_boundaries(
        |cap| prepared.validate_retopologized(&positions, &triangles, cap).map(|report| report.work),
        "prepared final candidate query failed",
    );
}

#[test]
fn changed_candidate_validation_retains_both_query_failure_boundaries() {
    let (positions, triangles) = geometry();
    let prepared = PreparedIntersections::new(&positions, &triangles, WORK).unwrap();
    let moved: Vec<_> = positions.iter().map(|&p| p + Vec3::new(0.125, 0., 0.)).collect();
    query_boundaries(
        |cap| prepared.check_candidate(&moved, cap).map(|report| report.work),
        "prepared surface intersection candidate query failed",
    );
    assert_eq!(prepared.check_candidate(&positions, WORK).unwrap().changed_vertices, 0);
}
