use mm3e_kit::surface_intersections::validate_source_contraction;
type Bounds = [[f64; 3]; 2];
const R: f64 = 1. / 1024.;
fn octahedron(width: f64) -> (Vec<Bounds>, Vec<[u32; 3]>) {
    let points = [[R, 0., 0.], [-R, 0., 0.], [0., R, 0.], [0., -R, 0.], [0., 0., R], [0., 0., -R]];
    let bounds = points.into_iter().map(|p| [p.map(|v| v - width), p.map(|v| v + width)]).collect();
    let triangles = vec![[0, 2, 4], [2, 1, 4], [1, 3, 4], [3, 0, 4], [2, 0, 5], [1, 2, 5], [3, 1, 5], [0, 3, 5]];
    (bounds, triangles)
}
fn tetrahedron(bounds: &mut Vec<Bounds>, triangles: &mut Vec<[u32; 3]>, center: [f64; 3], size: f64) {
    let base = bounds.len() as u32;
    for delta in [[size, size, size], [-size, -size, size], [-size, size, -size], [size, -size, -size]] {
        let p = std::array::from_fn(|i| center[i] + delta[i]);
        bounds.push([p, p]);
    }
    triangles.extend([[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]].map(|t| t.map(|i| i + base)));
}
#[test]
fn source_endpoint_embedding_and_nonadjacent_sweeps_have_replayable_charged_certificates() {
    for width in [0., 1e-18] {
        let (bounds, triangles) = octahedron(width);
        for (removed, retained) in [(0, 2), (2, 0)] {
            let report = validate_source_contraction(&bounds, &triangles, removed, retained, 1_000_000).unwrap();
            assert_eq!(report.changed_triangles, 4);
            assert_eq!(report.source_orientations, 2);
            assert!(report.final_pairs > 0);
            assert!(!report.adjacent_sweeps_certified);
            assert_eq!(
                validate_source_contraction(&bounds, &triangles, removed, retained, report.work).unwrap(),
                report
            );
            assert!(validate_source_contraction(&bounds, &triangles, removed, retained, report.work - 1)
                .err()
                .unwrap()
                .contains("budget"));
        }
    }
}
#[test]
fn a_stationary_closed_component_inside_the_swept_region_is_rejected() {
    let (mut bounds, mut triangles) = octahedron(0.);
    tetrahedron(&mut bounds, &mut triangles, [R * 0.2, 0., R * 0.1], R * 0.02);
    let error = validate_source_contraction(&bounds, &triangles, 0, 2, 1_000_000).err().unwrap();
    assert!(error.contains("cannot certify nonadjacent swept separation"), "{error}");
}
#[test]
fn distant_source_faces_with_degenerate_nearest_f32_images_are_still_considered() {
    let (mut bounds, mut triangles) = octahedron(0.);
    tetrahedron(&mut bounds, &mut triangles, [-R * 4., -R * 4., -R * 4.], 2f64.powi(-50));
    assert!(bounds[6..].iter().all(|p| p[0].map(|v| v as f32) == bounds[6][0].map(|v| v as f32)));
    let report = validate_source_contraction(&bounds, &triangles, 0, 2, 1_000_000).unwrap();
    assert!(report.work > 0);
}
#[test]
fn unresolved_intervals_invalid_source_edges_and_exhaustion_reject() {
    let (bounds, triangles) = octahedron(R * 0.5);
    assert!(validate_source_contraction(&bounds, &triangles, 0, 2, 1_000_000)
        .err()
        .unwrap()
        .contains("cannot certify"));
    let (bounds, triangles) = octahedron(0.);
    assert!(validate_source_contraction(&bounds, &triangles, 0, 1, 1_000_000)
        .err()
        .unwrap()
        .contains("do not form a source edge"));
    assert!(validate_source_contraction(&bounds, &triangles, 0, 2, 1).err().unwrap().contains("budget"));
    let mut invalid = bounds;
    invalid[0][0][0] = f64::NAN;
    assert!(validate_source_contraction(&invalid, &triangles, 0, 2, 1_000_000)
        .err()
        .unwrap()
        .contains("finite ordered"));
}

#[test]
fn rejected_search_proposals_retain_their_spent_work() {
    use mm3e_kit::surface_intersections::validate_source_contraction_counted;
    let (mut bounds, mut triangles) = octahedron(0.);
    tetrahedron(&mut bounds, &mut triangles, [R * 0.2, 0., R * 0.1], R * 0.02);
    let rejected = validate_source_contraction_counted(&bounds, &triangles, 0, 2, 1_000_000).unwrap_err();
    assert!(rejected.work > 0 && rejected.work <= 1_000_000);
    assert!(rejected.message.contains("nonadjacent swept"));
    let exhausted = validate_source_contraction_counted(&bounds, &triangles, 0, 2, rejected.work - 1).unwrap_err();
    assert!(exhausted.work < rejected.work);
    assert!(exhausted.message.contains("budget"));
}

#[test]
fn inverted_and_uncertain_changed_source_normals_are_rejected_before_embedding() {
    use mm3e_kit::surface_intersections::validate_source_contraction_counted;
    // This isolates the source-orientation admission gate, not the external
    // full-link/topology prerequisites. The survivor's old normal is +2Z;
    // mapping its first vertex to x>1 reverses that normal.
    let points = [[0., 0., 0.], [2., 0., 0.], [1., -1., 0.], [1., 1., 0.], [0., 0., 1.]];
    let exact: Vec<Bounds> = points.map(|p| [p, p]).into_iter().collect();
    let triangles = [[0, 1, 4], [0, 2, 3]];
    let inverted = validate_source_contraction_counted(&exact, &triangles, 0, 1, 100_000).unwrap_err();
    assert!(inverted.message.contains("positive source orientation for triangle 1"), "{inverted}");
    assert!(inverted.work > 0 && inverted.work < 100_000);
    // The nominal midpoint x=.75 gives a positive dot, but these enclosures
    // also admit x=1.25 and therefore cannot certify the source orientation.
    let mut uncertain = exact;
    uncertain[1] = [[0.25, 0., 0.], [1.25, 0., 0.]];
    let unresolved = validate_source_contraction_counted(&uncertain, &triangles, 0, 1, 100_000).unwrap_err();
    assert!(unresolved.message.contains("positive source orientation for triangle 1"), "{unresolved}");
    assert!(unresolved.work > 0 && unresolved.work < 100_000);
}
