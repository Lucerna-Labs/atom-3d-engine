use mm3e_kit::{
    mesh_correspondence::{certify, CorrespondenceReport, Options},
    Vec3,
};

type Enclosure = [[f64; 3]; 2];
const OCTAHEDRON: [[u32; 3]; 8] =
    [[0, 2, 3], [0, 3, 4], [0, 4, 5], [0, 5, 2], [1, 3, 2], [1, 4, 3], [1, 5, 4], [1, 2, 5]];

fn options(max_error_m: f64) -> Options {
    Options { max_error_m, max_work: 100_000 }
}
fn enclosures(points: &[Vec3]) -> Vec<Enclosure> {
    points.iter().map(|p| [[p.x, p.y, p.z].map(f64::from); 2]).collect()
}
fn octahedron() -> Vec<Vec3> {
    vec![
        Vec3::new(0., 0., 1.),
        Vec3::new(0., 0., -1.),
        Vec3::new(1., 0., 0.),
        Vec3::new(0., 1., 0.),
        Vec3::new(-1., 0., 0.),
        Vec3::new(0., -1., 0.),
    ]
}
fn triangle() -> Vec<Vec3> {
    vec![Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.)]
}
fn error(result: Result<CorrespondenceReport, String>, expected: &str) {
    let message = result.unwrap_err();
    assert!(message.contains(expected), "expected {expected:?}, received {message:?}");
}

#[test]
fn exact_identity_and_cyclic_orientation_need_no_borrowed_error_allowance() {
    let p = triangle();
    let report = certify(&enclosures(&p), &[[0, 1, 2]], &[0, 1, 2], &p, &[[1, 2, 0]], options(0.)).unwrap();
    assert_eq!(report.max_displacement_m, 0.);
    assert_eq!(
        (report.vertex_witnesses, report.triangle_images, report.edge_images, report.vertex_images),
        (3, 1, 0, 0)
    );
    error(certify(&enclosures(&p), &[[0, 1, 2]], &[0, 1, 2], &p, &[[0, 2, 1]], options(0.)), "oriented face");
}

#[test]
fn a_true_alias_is_an_additional_coordinate_witness_even_without_its_own_face() {
    let p = triangle();
    let mut original = enclosures(&p);
    original.push(original[0]);
    let report = certify(&original, &[[0, 1, 2]], &[0, 1, 2, 0], &p, &[[0, 1, 2]], options(0.)).unwrap();
    assert_eq!(report.vertex_witnesses, 4);
    assert_eq!(report.max_displacement_m, 0.);
    // A wider certified interval for the same exact origin must still be charged.
    original[3] = [[-0.25, 0., 0.], [0.25, 0., 0.]];
    error(certify(&original, &[[0, 1, 2]], &[0, 1, 2, 0], &p, &[[0, 1, 2]], options(0.2)), "displacement");
    let report = certify(&original, &[[0, 1, 2]], &[0, 1, 2, 0], &p, &[[0, 1, 2]], options(0.3)).unwrap();
    assert_eq!(report.worst_vertex, 3);
    assert!(report.max_displacement_m >= 0.25);
}

#[test]
fn valid_endpoint_contraction_covers_removed_faces_and_disjoint_changes_take_the_maximum() {
    let p = octahedron();
    let mapping = [2, 1, 2, 3, 4, 5];
    let final_faces = [[2, 3, 4], [2, 4, 5], [1, 3, 2], [1, 4, 3], [1, 5, 4], [1, 2, 5]];
    let single = certify(&enclosures(&p), &OCTAHEDRON, &mapping, &p, &final_faces, options(1.5)).unwrap();
    assert_eq!((single.triangle_images, single.edge_images, single.vertex_images), (6, 2, 0));
    assert!(single.max_displacement_m >= 2_f64.sqrt() && single.max_displacement_m < 1.415);
    let mut both = p.clone();
    both.extend(p.iter().map(|v| Vec3::new(v.x + 10., v.y, v.z)));
    let mut originals = OCTAHEDRON.to_vec();
    originals.extend(OCTAHEDRON.map(|t| t.map(|i| i + 6)));
    let mut finals = final_faces.to_vec();
    finals.extend(final_faces.map(|t| t.map(|i| i + 6)));
    let mut ancestry = mapping.to_vec();
    ancestry.extend(mapping.map(|i| i + 6));
    let double = certify(&enclosures(&both), &originals, &ancestry, &both, &finals, options(1.5)).unwrap();
    assert_eq!(double.max_displacement_m, single.max_displacement_m);
    assert_eq!(double.edge_images, 4);
}

#[test]
fn composed_ancestry_charges_the_original_point_not_the_largest_individual_step() {
    let mut p = octahedron();
    p[0] = Vec3::new(0., 0., 0.0001);
    p[2] = Vec3::new(0.1, 0., 0.);
    p[3] = Vec3::new(0.2, 0.001, 0.);
    // The original 0 follows 0 -> 2 -> 3. Each step is below 0.101m,
    // while its original-to-final displacement exceeds 0.2m.
    let map = [3, 1, 3, 3, 4, 5];
    let faces = [[3, 4, 5], [1, 4, 3], [1, 5, 4], [1, 3, 5]];
    error(certify(&enclosures(&p), &OCTAHEDRON, &map, &p, &faces, options(0.15)), "displacement");
    let report = certify(&enclosures(&p), &OCTAHEDRON, &map, &p, &faces, options(0.21)).unwrap();
    assert_eq!(report.worst_vertex, 0);
    assert_eq!((report.triangle_images, report.edge_images, report.vertex_images), (4, 3, 1));
    assert!(report.max_displacement_m > 0.2 && report.max_displacement_m < 0.201);
}

#[test]
fn deleting_a_patch_or_component_and_inventing_a_face_cannot_pass_vertex_only_checks() {
    let p = vec![Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.), Vec3::new(0., 0., 1.)];
    let tetra = [[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]];
    error(certify(&enclosures(&p), &tetra, &[0, 1, 2, 3], &p, &tetra[..3], options(0.)), "original face image");
    error(certify(&enclosures(&p), &tetra[..3], &[0, 1, 2, 3], &p, &tetra, options(0.)), "no original predecessor");
    let mut both = p.clone();
    both.extend(p.iter().map(|v| Vec3::new(v.x + 10., v.y, v.z)));
    let mut original = tetra.to_vec();
    original.extend(tetra.map(|t| t.map(|i| i + 4)));
    error(
        certify(&enclosures(&both), &original, &[0, 1, 2, 3, 4, 5, 6, 7], &both, &tetra, options(0.)),
        "no image in the final surface",
    );
}

#[test]
fn warped_quad_retriangulation_is_rejected_despite_zero_vertex_motion() {
    let p = vec![Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(1., 1., 1.), Vec3::new(0., 1., 0.)];
    // Original diagonal midpoint (.5,.5,.5) lies >= .5/sqrt(3) from
    // the alternative triangulation. Equal vertex positions do not certify it.
    error(
        certify(&enclosures(&p), &[[0, 1, 2], [0, 2, 3]], &[0, 1, 2, 3], &p, &[[0, 1, 3], [1, 2, 3]], options(0.)),
        "original face image",
    );
}

#[test]
fn a_collapsed_face_must_land_on_an_actual_final_edge() {
    let p = vec![Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(1., 1., 0.), Vec3::new(0., 1., 0.)];
    let mut original = enclosures(&p);
    original.push(original[1]);
    error(
        certify(
            &original,
            &[[0, 1, 2], [0, 2, 3], [1, 3, 4]],
            &[0, 1, 2, 3, 1],
            &p,
            &[[0, 1, 2], [0, 2, 3]],
            options(0.),
        ),
        "not a final edge",
    );
}

#[test]
fn malformed_bounds_ancestry_indices_and_positions_fail_explicitly() {
    let p = triangle();
    let original = enclosures(&p);
    for enclosure in
        [[[1., 0., 0.], [0., 0., 0.]], [[f64::NAN, 0., 0.], [0., 0., 0.]], [[0., 0., 0.], [f64::INFINITY, 0., 0.]]]
    {
        let mut bad = original.clone();
        bad[0] = enclosure;
        error(certify(&bad, &[[0, 1, 2]], &[0, 1, 2], &p, &[[0, 1, 2]], options(1.)), "finite ordered");
    }
    error(certify(&original, &[[0, 1, 2]], &[0, 1], &p, &[[0, 1, 2]], options(1.)), "complete vertex ancestry");
    error(certify(&original, &[[0, 1, 2]], &[0, 1, 9], &p, &[[0, 1, 2]], options(1.)), "no image");
    error(certify(&original, &[[0, 1, 9]], &[0, 1, 2], &p, &[[0, 1, 2]], options(1.)), "original triangle index");
    error(certify(&original, &[[0, 1, 2]], &[0, 1, 2], &p, &[[0, 1, 9]], options(1.)), "final triangle index");
    error(certify(&original, &[[0, 0, 2]], &[0, 1, 2], &p, &[[0, 1, 2]], options(1.)), "repeats a vertex");
    error(certify(&original, &[[0, 1, 2]], &[0, 1, 2], &p, &[[0, 0, 2]], options(1.)), "repeats a vertex");
    error(
        certify(&original, &[[0, 1, 2]], &[0, 1, 2], &p, &[[0, 1, 2], [1, 2, 0]], options(1.)),
        "duplicate oriented face",
    );
    let mut bad = p.clone();
    bad[1].x = f32::INFINITY;
    error(certify(&original, &[[0, 1, 2]], &[0, 1, 2], &bad, &[[0, 1, 2]], options(1.)), "positions must be finite");
    for limit in [-1., f64::NAN, f64::INFINITY] {
        error(certify(&original, &[[0, 1, 2]], &[0, 1, 2], &p, &[[0, 1, 2]], options(limit)), "finite nonnegative");
    }
}

#[test]
fn outward_bounds_preserve_tiny_nonzero_displacements_and_reject_overflow() {
    let p = triangle();
    for tiny in [1e-200, f64::from_bits(1)] {
        let mut original = enclosures(&p);
        original[0] = [[tiny, 0., 0.]; 2];
        let report = certify(&original, &[[0, 1, 2]], &[0, 1, 2], &p, &[[0, 1, 2]], options(1e-199)).unwrap();
        assert!(report.max_displacement_m >= tiny && report.max_displacement_m > 0.);
        error(certify(&original, &[[0, 1, 2]], &[0, 1, 2], &p, &[[0, 1, 2]], options(0.)), "displacement");
    }
    let mut original = enclosures(&p);
    original[0] = [[f64::MAX, f64::MAX, 0.]; 2];
    error(certify(&original, &[[0, 1, 2]], &[0, 1, 2], &p, &[[0, 1, 2]], options(f64::MAX)), "finite bounds");
}

#[test]
fn reported_work_and_error_bounds_replay_exactly_and_one_less_work_fails() {
    let p = octahedron();
    let map = [2, 1, 2, 3, 4, 5];
    let faces = [[2, 3, 4], [2, 4, 5], [1, 3, 2], [1, 4, 3], [1, 5, 4], [1, 2, 5]];
    let original = enclosures(&p);
    let report = certify(&original, &OCTAHEDRON, &map, &p, &faces, options(2.)).unwrap();
    let exact = Options { max_work: report.work, max_error_m: report.max_displacement_m };
    assert_eq!(certify(&original, &OCTAHEDRON, &map, &p, &faces, exact).unwrap(), report);
    error(
        certify(&original, &OCTAHEDRON, &map, &p, &faces, Options { max_work: report.work - 1, ..exact }),
        "work budget",
    );
    error(
        certify(
            &original,
            &OCTAHEDRON,
            &map,
            &p,
            &faces,
            Options { max_error_m: report.max_displacement_m.next_down(), ..exact },
        ),
        "displacement",
    );
}

#[test]
fn actual_flat_source_star_has_a_direct_bound_without_claiming_stored_mesh_validity() {
    // Complete endpoint-star extraction from the frozen r15 flat source capture.
    // Source SHA: 0ffc50961d11e585ca841ccf69d4782fcbcaa11c36d9d1cb4977b7e4e66e04e1.
    // Global IDs: 0,1,5,6,7,12,16,18,19,30,52. Bounds are cofactor-certified
    // exact-coordinate f32 brackets; bits preserve them without decimal recoding.
    let bounds_bits: [[[u32; 3]; 2]; 11] = [
        [[0xbe99cfe5, 0xbe808313, 0x3c75c290], [0xbe99cfe4, 0xbe808312, 0x3c75c290]],
        [[0xbe99999a, 0xbe808313, 0x3c75c290], [0xbe99999a, 0xbe808312, 0x3c75c290]],
        [[0xbe99cfe5, 0xbe808313, 0x3c75c290], [0xbe99cfe4, 0xbe808312, 0x3c75c290]],
        [[0xbe99cfe5, 0xbe808313, 0x3c54fdf4], [0xbe99cfe4, 0xbe808312, 0x3c54fdf4]],
        [[0xbe99cfe5, 0xbe808313, 0x3c54fdf4], [0xbe99cfe4, 0xbe808312, 0x3c54fdf4]],
        [[0xbe9a1cad, 0xbe80364b, 0x3c5e96eb], [0xbe9a1cac, 0xbe80364a, 0x3c5e96ec]],
        [[0xbe99f649, 0xbe805caf, 0x3c59ca6f], [0xbe99f648, 0xbe805cae, 0x3c59ca70]],
        [[0xbe99cfe5, 0xbe80364b, 0x3c75c290], [0xbe99cfe4, 0xbe80364a, 0x3c75c290]],
        [[0xbe99f649, 0xbe805caf, 0x3c70f614], [0xbe99f648, 0xbe805cae, 0x3c70f615]],
        [[0xbe9a1cad, 0xbe80364b, 0x3c54fdf4], [0xbe9a1cac, 0xbe80364a, 0x3c54fdf4]],
        [[0xbe99999a, 0xbe800000, 0x3c75c290], [0xbe99999a, 0xbe800000, 0x3c75c290]],
    ];
    let stored_bits: [[u32; 3]; 11] = [
        [0xbe99cfe5, 0xbe808312, 0x3c75c290],
        [0xbe99999a, 0xbe808312, 0x3c75c290],
        [0xbe99cfe5, 0xbe808312, 0x3c75c290],
        [0xbe99cfe5, 0xbe808312, 0x3c54fdf4],
        [0xbe99cfe5, 0xbe808312, 0x3c54fdf4],
        [0xbe9a1cac, 0xbe80364b, 0x3c5e96eb],
        [0xbe99f649, 0xbe805caf, 0x3c59ca70],
        [0xbe99cfe5, 0xbe80364b, 0x3c75c290],
        [0xbe99f649, 0xbe805caf, 0x3c70f614],
        [0xbe9a1cac, 0xbe80364b, 0x3c54fdf4],
        [0xbe99999a, 0xbe800000, 0x3c75c290],
    ];
    let bounds = bounds_bits.map(|pair| pair.map(|xyz| xyz.map(|v| f64::from(f32::from_bits(v)))));
    let positions = stored_bits.map(|xyz| {
        let [x, y, z] = xyz.map(f32::from_bits);
        Vec3::new(x, y, z)
    });
    let original = [
        [0, 1, 10],
        [0, 10, 7],
        [0, 7, 2],
        [1, 0, 3],
        [2, 8, 5],
        [2, 5, 9],
        [2, 9, 6],
        [2, 6, 4],
        [0, 2, 4],
        [0, 4, 3],
        [2, 7, 8],
    ];
    let final_faces =
        [[0, 1, 10], [0, 10, 7], [1, 0, 3], [0, 8, 5], [0, 5, 9], [0, 9, 6], [0, 6, 4], [0, 4, 3], [0, 7, 8]];
    let map = [0, 1, 0, 3, 4, 5, 6, 7, 8, 9, 10];
    let report = certify(&bounds, &original, &map, &positions, &final_faces, options(5e-7)).unwrap();
    assert_eq!((report.triangle_images, report.edge_images, report.vertex_images), (9, 2, 0));
    let exact_interval_radius = (2049.0 / 2_f64.powi(60)).sqrt();
    assert!(report.max_displacement_m >= exact_interval_radius);
    assert!(report.max_displacement_m <= exact_interval_radius * (1. + 1e-12));
    // Global source6/7 collide in nearest-f32 storage. The distance certificate
    // intentionally does not bless this zero-area final face as valid geometry.
    assert_eq!(positions[3], positions[4]);
    assert!(final_faces.contains(&[0, 4, 3]));
}

#[test]
fn counted_rejections_spend_real_work_and_replay_with_a_cumulative_allowance() {
    use mm3e_kit::mesh_correspondence::certify_counted;
    let p = triangle();
    let exact = enclosures(&p);
    let mut moved = exact.clone();
    moved[0] = [[0.5, 0., 0.]; 2];
    for (bounds, original, expected) in [(&moved, [0, 1, 2], "displacement"), (&exact, [0, 2, 1], "oriented face")] {
        let attempt = |work| {
            certify_counted(
                bounds,
                &[original],
                &[0, 1, 2],
                &p,
                &[[0, 1, 2]],
                Options { max_error_m: 0., max_work: work },
            )
            .unwrap_err()
        };
        let first = attempt(100_000);
        assert!(first.message.contains(expected), "{}", first.message);
        assert!(first.work > 0);
        let replay = attempt(first.work);
        assert_eq!((replay.work, replay.message.as_str()), (first.work, first.message.as_str()));
        let short = attempt(first.work - 1);
        assert!(short.message.contains("budget"), "{}", short.message);
        assert!(short.work < first.work);
        // A caller carrying one shared allowance cannot run two full rejected
        // candidates for the price of one, even though neither produces a mesh.
        let allowance = first.work * 2 - 1;
        let next = attempt(allowance - first.work);
        assert!(next.message.contains("budget"), "{}", next.message);
        assert!(next.work > 0 && first.work + next.work <= allowance);
    }
}
