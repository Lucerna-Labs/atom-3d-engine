use mm3e_kit::{
    deform::{self, Influence, SkinningMethod},
    surface::{SurfaceHit, TriangleSurface},
    Mat3, Transform, Vec3,
};

type D3 = [f64; 3];
fn d(p: Vec3) -> D3 {
    [p.x as f64, p.y as f64, p.z as f64]
}
fn add(a: D3, b: D3) -> D3 {
    std::array::from_fn(|i| a[i] + b[i])
}
fn sub(a: D3, b: D3) -> D3 {
    std::array::from_fn(|i| a[i] - b[i])
}
fn scale(a: D3, s: f64) -> D3 {
    a.map(|v| v * s)
}
fn dot(a: D3, b: D3) -> f64 {
    a.into_iter().zip(b).map(|(x, y)| x * y).sum()
}
fn norm(a: D3) -> f64 {
    dot(a, a).sqrt()
}
fn near(a: D3, b: D3, tolerance: f64) {
    assert!(norm(sub(a, b)) <= tolerance, "{a:?} != {b:?}, tolerance {tolerance}");
}
fn bits(v: Vec3) -> [u32; 3] {
    [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()]
}

fn check(surface: &TriangleSurface, query: Vec3, tolerance: f64) -> SurfaceHit {
    let source_vertices = surface.vertices().to_vec();
    let source_triangles = surface.triangles().to_vec();
    let distance = surface.distance_bounded(query, usize::MAX).unwrap();
    let hit = surface.closest_hit_bounded(query, usize::MAX).unwrap_or_else(|error| panic!("query {query:?}: {error}"));
    assert_eq!(hit.distance.to_bits(), surface.distance(query).to_bits());
    assert_eq!(hit.distance.to_bits(), distance.distance.to_bits());
    assert_eq!(bits(hit.normal), bits(surface.normal(query)));
    assert_eq!(hit.work, distance.work + 1);
    assert_eq!(surface.closest_hit_bounded(query, hit.work).unwrap(), hit);
    assert!(surface.closest_hit_bounded(query, hit.work - 1).is_err());
    assert!(hit.barycentric.iter().all(|w| w.is_finite() && (0.0..=1.0).contains(w)));
    assert!((hit.barycentric.iter().sum::<f64>() - 1.0).abs() < 1e-13);
    let triangle = surface.triangles()[hit.triangle as usize];
    let reconstructed = triangle
        .into_iter()
        .zip(hit.barycentric)
        .map(|(i, w)| scale(d(surface.vertices()[i as usize]), w))
        .fold([0.0; 3], add);
    near(reconstructed, hit.closest_point, tolerance);
    assert_eq!(surface.vertices(), source_vertices);
    assert_eq!(surface.triangles(), source_triangles);
    hit
}

fn triangle() -> TriangleSurface {
    TriangleSurface::new(vec![Vec3::ZERO, Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0)], vec![[0, 1, 2]], 0.05)
        .unwrap()
}

#[test]
fn analytic_interior_edge_corner_and_both_shell_sides_have_correct_attributes() {
    let surface = triangle();
    for (query, point, weights) in [
        (Vec3::new(0.5, 1.0, 3.0), [0.5, 1.0, 0.0], [0.5, 0.25, 0.25]),
        (Vec3::new(0.5, 1.0, -3.0), [0.5, 1.0, 0.0], [0.5, 0.25, 0.25]),
        (Vec3::new(1.0, -2.0, 1.0), [1.0, 0.0, 0.0], [0.5, 0.5, 0.0]),
        (Vec3::new(-2.0, -3.0, 1.0), [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        (Vec3::new(0.5, 1.0, 0.0), [0.5, 1.0, 0.0], [0.5, 0.25, 0.25]),
    ] {
        let hit = check(&surface, query, 1e-13);
        assert_eq!(hit.triangle, 0);
        near(hit.closest_point, point, 1e-13);
        near(hit.barycentric, weights, 1e-13);
    }
    assert_eq!(
        bits(surface.closest_hit_bounded(Vec3::new(0.5, 1.0, 0.0), 3).unwrap().normal),
        bits(Vec3::new(0.0, 0.0, 1.0))
    );
}

#[test]
fn shared_edges_corners_and_bvh_ties_use_original_triangle_id_and_corner_order() {
    let vertices =
        vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 0.0), Vec3::new(0.0, 2.0, 0.0)];
    let surface = TriangleSurface::new(vertices, vec![[2, 3, 0], [0, 1, 2]], 0.01).unwrap();
    let edge = check(&surface, Vec3::new(1.0, 1.0, 0.5), 1e-13);
    assert_eq!(edge.triangle, 0);
    near(edge.barycentric, [0.5, 0.0, 0.5], 1e-13);
    let corner = check(&surface, Vec3::new(0.0, 0.0, 1.0), 1e-13);
    assert_eq!(corner.triangle, 0);
    near(corner.barycentric, [0.0, 0.0, 1.0], 1e-13);
    // More than one BVH leaf, with reversed-winding coincident disconnected faces.
    // Traversal ordering must not replace the original lowest-index tie identity.
    let mut vertices = vec![];
    let mut triangles = vec![];
    for i in 0..32 {
        let offset = vertices.len() as u32;
        vertices.extend([Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)]);
        triangles.push(if i == 0 { [offset, offset + 1, offset + 2] } else { [offset, offset + 2, offset + 1] });
    }
    let surface = TriangleSurface::new(vertices, triangles, 0.01).unwrap();
    let hit = check(&surface, Vec3::new(0.25, 0.25, 0.0), 1e-13);
    assert_eq!(hit.triangle, 0);
    near(hit.barycentric, [0.5, 0.25, 0.25], 1e-13);
    assert_eq!(bits(hit.normal), bits(Vec3::new(0.0, 0.0, 1.0)));
}

#[test]
fn thin_valid_triangles_retain_interior_weights_without_gram_determinant_cancellation() {
    for height in [1e-8f32, 1e-20, f32::MIN_POSITIVE] {
        let surface = TriangleSurface::new(
            vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, height, 0.0)],
            vec![[0, 1, 2]],
            0.0001,
        )
        .unwrap();
        for z in [0.0, 0.125, 0.2, -0.2] {
            let query = Vec3::new(0.75, height * 0.5, z);
            let hit = check(&surface, query, 1e-13);
            near(hit.barycentric, [0.25, 0.25, 0.5], 1e-12);
            assert!((hit.closest_point[1] / f64::from(height) - 0.5).abs() < 1e-12);
        }
    }
    let a = 16_777_216.0f32;
    let surface =
        TriangleSurface::new(vec![Vec3::ZERO, Vec3::new(a, a, 0.0), Vec3::new(a, a + 2.0, 0.0)], vec![[0, 1, 2]], 0.01)
            .unwrap();
    let hit = check(&surface, Vec3::new(a * 0.75, a * 0.75 + 1.0, 4.0), 1e-7);
    near(hit.barycentric, [0.25, 0.25, 0.5], 1e-12);
}

#[test]
fn zero_plane_projection_roundoff_is_retained_but_gross_reconstruction_loss_is_rejected() {
    let surface = TriangleSurface::new(
        vec![Vec3::ZERO, Vec3::new(5.232696, 0.0, 0.0), Vec3::new(0.0, 3.590821, 0.0)],
        vec![[0, 1, 2]],
        0.01,
    )
    .unwrap();
    let query = Vec3::new(1.0, 1.0, 9.577636);
    let hit = check(&surface, query, 2e-13);
    assert_eq!(hit.closest_point, [1.0, 1.0, -1.7763568394002505e-15]);
    let v = 1.0 / f64::from(surface.vertices()[1].x);
    let w = 1.0 / f64::from(surface.vertices()[2].y);
    near(hit.barycentric, [1.0 - v - w, v, w], 1e-12);
    // At this extreme query scale, the legacy plane projection cannot retain the
    // plane's x=1 offset in f64. Preserve distance/normal behavior and reject new
    // attributes instead of returning weights that reconstruct a different point.
    let surface = TriangleSurface::new(
        vec![Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 1.0)],
        vec![[0, 1, 2]],
        0.01,
    )
    .unwrap();
    let query = Vec3::new(1e38, 0.25, 0.25);
    let distance = surface.distance(query);
    let normal = surface.normal(query);
    assert!(distance.is_finite());
    assert!(surface.closest_hit_bounded(query, 3).unwrap_err().contains("reconstruct"));
    assert_eq!(surface.distance_bounded(query, 2).unwrap().distance.to_bits(), distance.to_bits());
    assert_eq!(bits(surface.normal(query)), bits(normal));
}

// Independent reference: constrained least-squares plane coordinates plus explicit
// segment candidates. Production uses normal projection and directed half-plane tests.
fn oracle(query: D3, vertices: [D3; 3]) -> (D3, [f64; 3], f64) {
    let [a, b, c] = vertices;
    let ab = sub(b, a);
    let ac = sub(c, a);
    let ap = sub(query, a);
    let d00 = dot(ab, ab);
    let d01 = dot(ab, ac);
    let d11 = dot(ac, ac);
    let d20 = dot(ap, ab);
    let d21 = dot(ap, ac);
    let determinant = d00 * d11 - d01 * d01;
    let mut candidates = vec![];
    if determinant > 0.0 {
        let v = (d11 * d20 - d01 * d21) / determinant;
        let w = (d00 * d21 - d01 * d20) / determinant;
        if v >= 0.0 && w >= 0.0 && v + w <= 1.0 {
            candidates.push((add(a, add(scale(ab, v), scale(ac, w))), [1.0 - v - w, v, w]));
        }
    }
    for (i, j) in [(0, 1), (1, 2), (2, 0)] {
        let edge = sub(vertices[j], vertices[i]);
        let t = (dot(sub(query, vertices[i]), edge) / dot(edge, edge)).clamp(0.0, 1.0);
        let point = add(vertices[i], scale(edge, t));
        let mut weights = [0.0; 3];
        weights[i] = 1.0 - t;
        weights[j] = t;
        candidates.push((point, weights));
    }
    candidates
        .into_iter()
        .map(|(point, weights)| {
            let q = sub(query, point);
            (point, weights, dot(q, q))
        })
        .min_by(|a, b| a.2.total_cmp(&b.2))
        .unwrap()
}

#[test]
fn attributes_match_an_independent_brute_force_oracle_on_curved_meshes_at_three_scales() {
    for factor in [1e-12f32, 1.0, 1e12] {
        let mut vertices = vec![];
        let mut triangles = vec![];
        for y in 0..=8 {
            for x in 0..=8 {
                let (x, y) = (x as f32 / 8.0, y as f32 / 8.0);
                vertices.push(Vec3::new(x, y, 0.13 * (x * 5.0).sin() + 0.09 * (y * 7.0).cos()).scale(factor));
            }
        }
        for y in 0..8 {
            for x in 0..8 {
                let a = y * 9 + x;
                triangles.extend([[a, a + 1, a + 10], [a, a + 10, a + 9]]);
            }
        }
        let surface = TriangleSurface::new(vertices, triangles, 0.003 * factor).unwrap();
        for i in 0..150 {
            let query = Vec3::new(
                ((i * 37) % 139) as f32 / 70.0 - 0.5,
                ((i * 53) % 131) as f32 / 65.0 - 0.5,
                ((i * 71) % 149) as f32 / 55.0 - 1.3,
            )
            .scale(factor);
            let hit = check(&surface, query, f64::from(factor) * 1e-10);
            let query = d(query);
            let mut candidates: Vec<_> = surface
                .triangles()
                .iter()
                .enumerate()
                .map(|(index, triangle)| {
                    let result = oracle(query, triangle.map(|i| d(surface.vertices()[i as usize])));
                    (index as u32, result)
                })
                .collect();
            candidates.sort_by(|a, b| a.1 .2.total_cmp(&b.1 .2).then(a.0.cmp(&b.0)));
            let (index, (point, weights, squared)) = candidates[0];
            let actual = sub(query, hit.closest_point);
            let epsilon = f64::from(factor).powi(2) * 1e-10;
            assert!((dot(actual, actual) - squared).abs() < epsilon);
            if candidates[1].1 .2 - squared > epsilon {
                assert_eq!(hit.triangle, index);
                near(hit.closest_point, point, f64::from(factor) * 1e-10);
                near(hit.barycentric, weights, 1e-9);
            }
        }
    }
}

#[test]
fn original_corner_parameters_remain_attached_after_actual_lbs_and_dqs_vertex_deformation() {
    let rest = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0)];
    let influences = vec![
        vec![Influence { joint: 0, weight: 1.0 }],
        vec![Influence { joint: 0, weight: 0.5 }, Influence { joint: 1, weight: 0.5 }],
        vec![Influence { joint: 1, weight: 1.0 }],
    ];
    let deltas = vec![
        Transform::at(Vec3::new(0.2, 0.1, 0.0)),
        Transform::new(Vec3::new(0.4, 0.0, 0.3), Mat3::from_euler(0.0, 0.0, 0.8), 1.0),
    ];
    let weights = [0.25, 0.25, 0.5];
    for method in [SkinningMethod::LinearBlend, SkinningMethod::DualQuaternion] {
        let result = deform::deform(&rest, &influences, &[], &[], &deltas, method).unwrap();
        let surface = TriangleSurface::new(result.positions, vec![[0, 1, 2]], 0.004).unwrap();
        let anchor = surface.vertices().iter().zip(weights).map(|(p, w)| scale(d(*p), w)).fold([0.0; 3], add);
        let query = Vec3::new(anchor[0] as f32, anchor[1] as f32, anchor[2] as f32);
        let hit = check(&surface, query, 1e-12);
        near(hit.barycentric, weights, 1e-6);
        // The prospective UV mechanism: interpolate immutable ORIGINAL corner values
        // using the returned posed-triangle weights. No world-position mapping is used.
        let uv_corners = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];
        let uv: [f64; 2] =
            std::array::from_fn(|axis| uv_corners.iter().zip(hit.barycentric).map(|(uv, w)| uv[axis] * w).sum());
        assert!((uv[0] - 0.75).abs() < 1e-6 && (uv[1] - 0.5).abs() < 1e-6);
        assert_ne!(surface.vertices(), rest);
    }
}

#[test]
fn attribute_step_is_precharged_and_invalid_queries_do_not_change_legacy_results() {
    let surface = triangle();
    let point = Vec3::new(0.5, 1.0, 0.5);
    let distance = surface.distance_bounded(point, usize::MAX).unwrap();
    let full = surface.closest_hit_bounded(point, usize::MAX).unwrap();
    assert_eq!(distance.work, 2);
    assert_eq!(full.work, 3);
    for budget in 0..full.work {
        assert!(surface.closest_hit_bounded(point, budget).is_err());
    }
    assert_eq!(surface.closest_hit_bounded(point, full.work).unwrap(), full);
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(surface.closest_hit_bounded(Vec3::new(bad, 0.0, 0.0), usize::MAX).is_err());
        assert_eq!(surface.distance(Vec3::new(bad, 0.0, 0.0)), f32::INFINITY);
    }
    let huge = Vec3::splat(f32::MAX);
    assert_eq!(surface.distance(huge), f32::INFINITY);
    assert_eq!(surface.distance_bounded(huge, usize::MAX).unwrap().distance, f32::INFINITY);
    assert!(surface.closest_hit_bounded(huge, usize::MAX).is_err());
}

mod lip_uv_repro {
    include!("fixtures/lip_uv_surface_repro.rs");
}

#[test]
fn actual_lip_seam_edge_recovers_weights_without_changing_legacy_winner_or_point() {
    let vertices: Vec<_> = lip_uv_repro::VERTEX_BITS
        .into_iter()
        .map(|p| Vec3::new(f32::from_bits(p[0]), f32::from_bits(p[1]), f32::from_bits(p[2])))
        .collect();
    let query = Vec3::new(f32::from_bits(3185442786), f32::from_bits(1068187821), f32::from_bits(1049687993));
    let surface = TriangleSurface::new(vertices.clone(), lip_uv_repro::TRIANGLES.to_vec(), 0.0015).unwrap();
    assert_eq!(surface.distance(query).to_bits(), 3071271387);
    let hit = check(&surface, query, 2e-14);
    assert_eq!(hit.triangle, 33, "the lower original triangle ID must remain the winning edge tie");
    assert_eq!(hit.closest_point, [-0.10800137353225886, 1.3390171102622228, 0.28405863170874757]);
    assert_eq!(hit.barycentric[1], 0.0, "winning feature is edge c-to-a, not the rounded plane interior");
    let [a, _, c] = surface.triangles()[33].map(|i| d(vertices[i as usize]));
    let ca = sub(a, c);
    let t = (dot(sub(d(query), c), ca) / dot(ca, ca)).clamp(0.0, 1.0);
    near(hit.barycentric, [t, 0.0, 1.0 - t], 2e-14);
    let adjacent =
        TriangleSurface::new(surface.triangles()[34].map(|i| vertices[i as usize]).to_vec(), vec![[0, 1, 2]], 0.0015)
            .unwrap();
    let adjacent_hit = check(&adjacent, query, 2e-14);
    assert_eq!(adjacent_hit.distance.to_bits(), hit.distance.to_bits());
    assert_eq!(adjacent_hit.closest_point, hit.closest_point);
}
