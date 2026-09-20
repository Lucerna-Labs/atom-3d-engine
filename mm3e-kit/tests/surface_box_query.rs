use mm3e_kit::{
    deform::{self, Influence, SkinningMethod},
    surface::TriangleSurface,
    Quat, Transform, Vec3,
};

fn components(v: Vec3) -> [f64; 3] {
    [v.x, v.y, v.z].map(f64::from)
}
fn brute(surface: &TriangleSurface, min: Vec3, max: Vec3, padding: f32) -> Vec<u32> {
    let min = components(min);
    let max = components(max);
    let pad = f64::from(padding);
    surface
        .triangles()
        .iter()
        .enumerate()
        .filter_map(|(index, triangle)| {
            let vertices = triangle.map(|i| components(surface.vertices()[i as usize]));
            let overlaps = (0..3).all(|axis| {
                let lo = vertices.iter().map(|v| v[axis]).fold(f64::INFINITY, f64::min) - pad;
                let hi = vertices.iter().map(|v| v[axis]).fold(f64::NEG_INFINITY, f64::max) + pad;
                lo <= max[axis] && hi >= min[axis]
            });
            overlaps.then_some(index as u32)
        })
        .collect()
}
fn grid() -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let vertices = (0..=8)
        .flat_map(|y| {
            (0..=8).map(move |x| Vec3::new(x as f32, y as f32, 0.1 * (x as f32).sin() + 0.2 * (y as f32).cos()))
        })
        .collect();
    let mut triangles = vec![];
    for y in 0..8 {
        for x in 0..8 {
            let a = y * 9 + x;
            triangles.extend([[a, a + 1, a + 10], [a, a + 10, a + 9]]);
        }
    }
    triangles.reverse(); // Original IDs differ deliberately from spatial order.
    (vertices, triangles)
}
fn compare(surface: &TriangleSurface, min: Vec3, max: Vec3, pad: f32) {
    let expected = brute(surface, min, max, pad);
    let got = surface.triangle_candidates_bounded(min, max, pad, usize::MAX).unwrap();
    assert_eq!(got.triangles, expected);
    assert!(got.triangles.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(surface.triangle_candidates_bounded(min, max, pad, got.work).unwrap(), got);
    assert!(surface.triangle_candidates_bounded(min, max, pad, got.work - 1).is_err());
}

#[test]
fn bvh_candidates_match_brute_force_and_stable_original_ids() {
    let (vertices, triangles) = grid();
    let surface = TriangleSurface::new(vertices, triangles, 0.03).unwrap();
    for i in 0..120 {
        let center = Vec3::new(
            (i * 17 % 113) as f32 / 10. - 1.,
            (i * 31 % 109) as f32 / 10. - 1.,
            (i * 7 % 13) as f32 / 10. - 0.6,
        );
        let half = Vec3::new(0.15, 0.25, 0.2);
        compare(&surface, center - half, center + half, (i % 3) as f32 * 0.05);
    }
    let all = surface.triangle_candidates_bounded(Vec3::splat(-20.), Vec3::splat(20.), 0., 1000).unwrap();
    assert_eq!(all.triangles, (0..surface.triangles().len() as u32).collect::<Vec<_>>());
    let outside = surface.triangle_candidates_bounded(Vec3::splat(100.), Vec3::splat(101.), 0., 1).unwrap();
    assert_eq!(outside.work, 1);
    assert!(outside.triangles.is_empty());
}

#[test]
fn touching_faces_edges_points_and_explicit_padding_use_closed_boxes() {
    let surface =
        TriangleSurface::new(vec![Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.)], vec![[0, 1, 2]], 0.2)
            .unwrap();
    for p in [Vec3::ZERO, Vec3::new(1., 1., 0.), Vec3::new(0.5, 0.5, 0.)] {
        assert_eq!(surface.triangle_candidates_bounded(p, p, 0., 2).unwrap().triangles, vec![0]);
    }
    let touching = Vec3::new(0.5, 0.5, 0.125);
    assert!(surface.triangle_candidates_bounded(touching, touching, 0., 2).unwrap().triangles.is_empty());
    assert_eq!(surface.triangle_candidates_bounded(touching, touching, 0.125, 2).unwrap().triangles, vec![0]);
    let just_outside = Vec3::new(0.5, 0.5, 0.125_f32.next_up());
    assert!(surface.triangle_candidates_bounded(just_outside, just_outside, 0.125, 2).unwrap().triangles.is_empty());
    // (1,1,0) is outside the triangle but inside its AABB; candidates are not
    // falsely advertised as exact triangle/box intersections.
}

#[test]
fn large_coordinates_tiny_padding_and_finite_expansion_never_exclude_overlap() {
    for scale in [f32::MIN_POSITIVE, 1e-20, 1., 1e20] {
        let x = 8. * scale;
        let vertices = vec![Vec3::new(x, 0., 0.), Vec3::new(x + scale, 0., 0.), Vec3::new(x, scale, 0.)];
        let surface = TriangleSurface::new(vertices, vec![[0, 1, 2]], (scale * 0.01).max(f32::from_bits(1))).unwrap();
        let p = Vec3::new(x, 0., 0.);
        assert_eq!(surface.triangle_candidates_bounded(p, p, f32::from_bits(1), 2).unwrap().triangles, vec![0]);
        compare(&surface, p, p, scale * 0.1);
        let before = Vec3::new(x.next_down(), 0., 0.);
        assert!(surface
            .triangle_candidates_bounded(before, before, f32::from_bits(1), 2)
            .unwrap()
            .triangles
            .is_empty());
        assert_eq!(
            surface
                .triangle_candidates_bounded(Vec3::splat(-f32::MAX), Vec3::splat(f32::MAX), f32::MAX, 2)
                .unwrap()
                .triangles,
            vec![0]
        );
    }
}

#[test]
fn deformed_rotated_world_coordinate_surfaces_keep_conservative_candidates() {
    let (rest, triangles) = grid();
    let influences: Vec<_> = rest
        .iter()
        .map(|p| {
            [Influence { joint: 0, weight: 1. - p.x / 8. }, Influence { joint: 1, weight: p.x / 8. }]
                .into_iter()
                .filter(|influence| influence.weight > 0.0)
                .collect()
        })
        .collect();
    let deltas = [
        Transform::new(Vec3::new(3., -2., 1.), Quat::from_euler(0.3, -0.2, 0.5).to_mat3(), 1.),
        Transform::new(Vec3::new(4., -1., 2.), Quat::from_euler(-0.1, 0.4, -0.3).to_mat3(), 1.),
    ];
    for method in [SkinningMethod::LinearBlend, SkinningMethod::DualQuaternion] {
        let deformed = deform::deform(&rest, &influences, &[], &[], &deltas, method).unwrap();
        let surface = TriangleSurface::new(deformed.positions, triangles.clone(), 0.04).unwrap();
        for vertex in surface.vertices().iter().step_by(3) {
            compare(&surface, *vertex - Vec3::splat(0.1), *vertex + Vec3::splat(0.1), 0.05);
        }
    }
}

#[test]
fn invalid_queries_and_budget_failures_do_not_change_legacy_distance_or_hit() {
    let (vertices, triangles) = grid();
    let surface = TriangleSurface::new(vertices, triangles, 0.01).unwrap();
    let p = Vec3::new(1.25, 2.5, 0.5);
    let hit = surface.closest_hit_bounded(p, 1000).unwrap();
    let distance = surface.distance(p);
    for (min, max, pad) in [
        (Vec3::splat(f32::NAN), Vec3::ZERO, 0.),
        (Vec3::ZERO, Vec3::splat(f32::INFINITY), 0.),
        (Vec3::new(1., 0., 0.), Vec3::ZERO, 0.),
        (Vec3::ZERO, Vec3::ZERO, -0.01),
        (Vec3::ZERO, Vec3::ZERO, f32::NAN),
        (Vec3::ZERO, Vec3::ZERO, f32::INFINITY),
    ] {
        assert!(surface.triangle_candidates_bounded(min, max, pad, 1000).is_err());
    }
    assert!(surface.triangle_candidates_bounded(Vec3::splat(-20.), Vec3::splat(20.), 0., 0).is_err());
    assert_eq!(surface.closest_hit_bounded(p, 1000).unwrap(), hit);
    assert_eq!(surface.distance(p).to_bits(), distance.to_bits());
}
