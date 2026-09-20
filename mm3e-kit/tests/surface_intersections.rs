use mm3e_kit::{surface_intersections::validate, Vec3};
fn p(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}
fn pair(a: [Vec3; 3], b: [Vec3; 3]) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    (a.into_iter().chain(b).collect(), vec![[0, 1, 2], [3, 4, 5]])
}
fn base() -> [Vec3; 3] {
    [p(0., 0., 0.), p(1., 0., 0.), p(0., 1., 0.)]
}
fn reject(a: [Vec3; 3], b: [Vec3; 3]) {
    let (v, t) = pair(a, b);
    assert!(validate(&v, &t, 1_000_000).err().unwrap().contains("intersect"));
}
#[test]
fn crossings_coplanar_area_containment_and_unshared_contact_are_rejected() {
    reject(base(), [p(0.2, 0.2, -1.), p(0.2, 0.2, 1.), p(0.2, 0.8, 0.)]);
    reject(base(), [p(0.1, 0.1, 0.), p(0.5, 0.1, 0.), p(0.1, 0.5, 0.)]);
    reject(base(), [p(-0.2, 0.4, 0.), p(0.8, 0.4, 0.), p(0.8, 1.4, 0.)]);
    reject(base(), [p(0., 0., 0.), p(-1., 0., 0.), p(0., -1., 0.)]);
    reject(base(), base());
    reject(base(), [p(0.25, 0., -1.), p(0.75, 0., -1.), p(0.5, 0., 1.)]);
    reject(base(), [p(0.2, 0., 0.), p(0.8, 0., 0.), p(0.5, -1., 0.)]);
}
#[test]
fn shared_indexed_edges_and_vertices_accept_only_their_actual_common_simplex() {
    let mut v = base().to_vec();
    v.extend([p(1., -1., 0.), p(0., 0., 1.), p(-1., 0., 0.), p(0., -1., 0.), p(0.2, 0.2, 0.), p(0.4, 0.2, 0.)]);
    for triangles in
        [vec![[0, 1, 2], [1, 0, 3]], vec![[0, 1, 2], [1, 0, 4]], vec![[0, 1, 2], [0, 5, 6]], vec![[0, 1, 2], [0, 4, 5]]]
    {
        validate(&v, &triangles, 1_000_000).unwrap();
    }
    for triangles in [vec![[0, 1, 2], [0, 1, 7]], vec![[0, 1, 2], [0, 7, 8]]] {
        assert!(validate(&v, &triangles, 1_000_000).is_err());
    }
}
#[test]
fn representable_near_misses_and_subnormal_plane_separation_remain_disjoint() {
    for gap in [f32::from_bits(1), 1e-7, 0.01] {
        let (v, t) = pair(base(), base().map(|p| p + Vec3::new(0., 0., gap)));
        validate(&v, &t, 1_000_000).unwrap();
    }
    let (v, t) = pair(base(), [p(0.7, 0.7, 0.), p(1.2, 0.7, 0.), p(0.7, 1.2, 0.)]);
    validate(&v, &t, 1_000_000).unwrap();
    reject(base(), [p(0.2, 0.2, -f32::from_bits(1)), p(0.2, 0.2, f32::from_bits(1)), p(0.2, 0.8, 0.)]);
}
#[test]
fn closed_curved_mesh_is_valid_and_exact_reported_budget_is_enforced() {
    let v = vec![p(1., 0., 0.), p(-1., 0., 0.), p(0., 1., 0.), p(0., -1., 0.), p(0., 0., 1.), p(0., 0., -1.)];
    let t = vec![[0, 2, 4], [2, 1, 4], [1, 3, 4], [3, 0, 4], [2, 0, 5], [1, 2, 5], [3, 1, 5], [0, 3, 5]];
    let report = validate(&v, &t, 1_000_000).unwrap();
    assert!(report.candidate_pairs > 0 && report.predicate_tests > 0);
    assert_eq!(validate(&v, &t, report.work).unwrap(), report);
    assert!(validate(&v, &t, report.work - 1).err().unwrap().contains("budget"));
    assert!(validate(&v, &t, 1).err().unwrap().contains("budget"));
}
#[test]
fn large_translated_geometry_uses_the_original_f32_orientation() {
    let offset = p(65536., -131072., 32768.);
    let a = base().map(|p| offset + p);
    reject(a, [p(0.25, 0.25, -1.), p(0.25, 0.25, 1.), p(0.25, 0.75, 0.)].map(|p| offset + p));
    let (v, t) = pair(a, base().map(|p| offset + p + Vec3::new(0., 0., 0.00390625)));
    validate(&v, &t, 1_000_000).unwrap();
}
#[test]
fn invalid_indices_nonfinite_and_degenerate_inputs_are_rejected() {
    assert!(validate(&base(), &[[0, 1, 9]], 10000).is_err());
    assert!(validate(&[p(f32::NAN, 0., 0.), p(1., 0., 0.), p(0., 1., 0.)], &[[0, 1, 2]], 10000).is_err());
    assert!(validate(&[p(0., 0., 0.), p(1., 0., 0.), p(2., 0., 0.)], &[[0, 1, 2]], 10000).is_err());
}

#[test]
fn classification_is_invariant_under_all_triangle_windings_and_pair_order() {
    let permutations = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let cases = [
        ([p(0.2, 0.2, -1.), p(0.2, 0.2, 1.), p(0.2, 0.8, 0.)], false),
        ([p(0.7, 0.7, 0.), p(1.2, 0.7, 0.), p(0.7, 1.2, 0.)], true),
        ([p(0.1, 0.1, 0.), p(0.5, 0.1, 0.), p(0.1, 0.5, 0.)], false),
        ([p(0.25, 0., -1.), p(0.75, 0., -1.), p(0.5, 0., 1.)], false),
    ];
    for (b, accepted) in cases {
        for pa in permutations {
            for pb in permutations {
                let a = pa.map(|i| base()[i]);
                let b = pb.map(|i| b[i]);
                for (a, b) in [(a, b), (b, a)] {
                    let (v, t) = pair(a, b);
                    assert_eq!(validate(&v, &t, 1_000_000).is_ok(), accepted, "{a:?} {b:?}");
                }
            }
        }
    }
}

#[test]
fn bvh_chunk_boundaries_preserve_global_identity_and_cross_chunk_contacts() {
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    for i in 0..22_000 {
        let base = vertices.len() as u32;
        let x = (i * 3) as f32;
        vertices.extend([p(x, 0., 0.), p(x + 1., 0., 0.), p(x, 1., 0.)]);
        triangles.push([base, base + 1, base + 2]);
    }
    let report = validate(&vertices, &triangles, 20_000_000).unwrap();
    assert_eq!(report.bvh_chunks, 2);
    assert_eq!(report.candidate_pairs, 0);
    let base = vertices.len() as u32;
    vertices.extend([p(0., 0., 0.), p(1., 0., 0.), p(0., 1., 0.)]);
    triangles.push([base, base + 1, base + 2]);
    let error = validate(&vertices, &triangles, 20_000_000).err().unwrap();
    assert!(error.contains("surface triangles 0 and 22000 intersect"), "{error}");
}

#[test]
fn exact_fallback_handles_tilted_coplanarity_without_coordinate_plane_shortcuts() {
    let a = [p(0., 0., 0.), p(1., 0., 1.), p(0., 1., -1.)];
    let b = [p(0.75, 0.75, 0.), p(1.25, 0.75, 0.5), p(0.75, 1.25, -0.5)];
    let (v, t) = pair(a, b);
    let report = validate(&v, &t, 1_000_000).unwrap();
    assert_eq!(report.candidate_pairs, 1);
    assert!(report.exact_predicates > 0);
    reject(a, [p(0.125, 0.125, 0.), p(0.5, 0.125, 0.375), p(0.125, 0.5, -0.375)]);
    let (v, t) = pair(a, a.map(|p| p + Vec3::new(0., 0., 1. / (1u32 << 22) as f32)));
    validate(&v, &t, 1_000_000).unwrap();
}

#[test]
fn counted_rejection_exposes_only_proven_pair_ids_and_retains_failed_work() {
    use mm3e_kit::surface_intersections::validate_counted;
    let (v, t) = pair(base(), base());
    let failure = validate_counted(&v, &t, 1_000_000).unwrap_err();
    assert_eq!(failure.triangles, Some([0, 1]));
    assert!(failure.work > 0 && failure.work < 1_000_000);
    assert!(failure.message.contains("intersect or overlap beyond their shared indexed boundary"));
    let exhausted = validate_counted(&v, &t, failure.work - 1).unwrap_err();
    assert_eq!(exhausted.triangles, None);
    assert!(exhausted.work < failure.work);
    assert!(exhausted.message.contains("budget"));
}
