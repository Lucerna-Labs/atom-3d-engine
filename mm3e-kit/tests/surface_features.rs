use mm3e_kit::{surface::TriangleSurface, surface_features::SurfaceFeatures, Vec3};

fn surface() -> TriangleSurface {
    TriangleSurface::new(
        vec![Vec3::new(-0.6, -0.4, 0.), Vec3::new(0.7, -0.3, 0.1), Vec3::new(-0.2, 0.8, 0.05)],
        vec![[0, 1, 2]],
        0.003,
    )
    .unwrap()
}
#[test]
fn prism_capsule_inside_set_matches_actual_distance_on_faces_edges_and_corners() {
    let surface = surface();
    let features = SurfaceFeatures::new(surface.clone(), 0.003, 1_000_000).unwrap();
    let mut state = 48339u32;
    let mut random = || {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(state) / f64::from(u32::MAX)
    };
    let mut inside = 0;
    let mut outside = 0;
    for _ in 0..20_000 {
        let u = random();
        let v = random() * (1. - u);
        let vertices = surface.vertices();
        let mut p = vertices[0] * (1. - u as f32 - v as f32) + vertices[1] * u as f32 + vertices[2] * v as f32;
        p = p + Vec3::new(
            (random() - 0.5) as f32 * 0.03,
            (random() - 0.5) as f32 * 0.03,
            (random() - 0.5) as f32 * 0.03,
        );
        let d = surface.distance(p);
        if d.abs() < 1e-7 {
            continue;
        }
        assert_eq!(features.contains(p), d < 0., "point{p:?},native distance{d}");
        if d < 0. {
            inside += 1
        } else {
            outside += 1
        }
    }
    assert!(inside > 1000 && outside > 1000);
    // Outside the triangular prism, but within the spherical corner feature.
    let p = surface.vertices()[0] + Vec3::new(-0.001, -0.001, 0.);
    assert!(features.contains(p));
    assert!(surface.distance(p) < 0.);
}
#[test]
fn coplanar_opposite_winding_faces_share_canonical_offset_planes() {
    let surface = TriangleSurface::new(
        vec![Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(1., 1., 0.), Vec3::new(0., 1., 0.)],
        vec![[0, 1, 2], [0, 3, 2]],
        0.001,
    )
    .unwrap();
    let features = SurfaceFeatures::new(surface, 0.001, 100_000).unwrap();
    // 2 shared slab planes +5 unique edge planes +5 unique capsules.
    assert_eq!(features.feature_count(), 12);
    for z in [-0.0005, 0.0005] {
        assert!(features.contains(Vec3::new(0.4, 0.7, z)));
    }
    assert!(!features.contains(Vec3::new(0.4, 0.7, 0.002)));
}
#[test]
fn cell_pruning_is_conservative_and_prism_keeps_subcell_sheets() {
    let surface = TriangleSurface::new(
        vec![Vec3::new(-1., -1., 0.), Vec3::new(1., -1., 0.), Vec3::new(0., 1., 0.)],
        vec![[0, 1, 2]],
        0.001,
    )
    .unwrap();
    let features = SurfaceFeatures::new(surface, 0.001, 100_000).unwrap();
    let cell = std::array::from_fn(|i| {
        Vec3::new(
            if i & 1 == 0 { -0.05 } else { 0.05 },
            if i & 2 == 0 { -0.05 } else { 0.05 },
            if i & 4 == 0 { -0.05 } else { 0.05 },
        )
    });
    let data = features.sample_cell(cell, 100_000).unwrap();
    assert!(data.expression.is_some());
    assert!(data.features.iter().any(|f| f.values.iter().any(|&x| x < 0.) && f.values.iter().any(|&x| x > 0.)));
    let expression = data.expression.unwrap();
    for i in 0..8 {
        assert!(expression.evaluate(&data.features.iter().map(|f| f.values[i]).collect::<Vec<_>>()).unwrap() > 0.);
    }
    let empty = features.sample_cell(cell.map(|p| p + Vec3::new(3., 0., 0.)), 100_000).unwrap();
    assert!(empty.expression.is_none());
    assert!(features.sample_cell(cell, 1).is_err());
}

#[test]
fn local_extraction_sees_both_slab_sheets_when_combined_scalar_has_no_negative_nodes() {
    use mm3e_kit::meshing::{extract_isosurface, MAX_MESH_TRIANGLES, MAX_MESH_VERTICES};
    use mm3e_kit::meshing_local::{extract_local_boolean_isosurface, LocalExtractionOptions};
    let surface = TriangleSurface::new(
        vec![Vec3::new(-0.4, -0.3, 0.), Vec3::new(0.4, -0.3, 0.), Vec3::new(0., 0.4, 0.)],
        vec![[0, 1, 2]],
        0.001,
    )
    .unwrap();
    let features = SurfaceFeatures::new(surface.clone(), 0.001, 1_000_000).unwrap();
    let lo = Vec3::new(-0.5, -0.5, -0.044);
    let hi = Vec3::new(0.5, 0.5, 0.036);
    let grid = [8, 8, 10];
    assert!(extract_isosurface(|p| surface.distance(p), lo, hi, grid).is_err());
    let result = extract_local_boolean_isosurface(
        lo,
        hi,
        grid,
        LocalExtractionOptions {
            max_work: 10_000_000,
            max_vertices: MAX_MESH_VERTICES,
            max_triangles: MAX_MESH_TRIANGLES,
        },
        |cell, work| features.sample_cell(cell.points, work),
    )
    .unwrap();
    assert_eq!(result.mesh.metadata.boundary_edges, 0);
    assert_eq!(result.mesh.metadata.connected_components, 1);
    let min = result.mesh.positions.iter().map(|p| p.z).fold(f32::INFINITY, f32::min);
    let max = result.mesh.positions.iter().map(|p| p.z).fold(f32::NEG_INFINITY, f32::max);
    assert!((min + 0.001).abs() < 1e-8 && (max - 0.001).abs() < 1e-8);
    let residual = result
        .mesh
        .triangles
        .iter()
        .map(|t| {
            let [a, b, c] = t.map(|i| result.mesh.positions[i as usize]);
            surface.distance((a + b + c) * (1. / 3.)).abs()
        })
        .fold(0_f32, f32::max);
    assert!(residual > 0.0001, "affine feature sampling must not be claimed to have refined the rounded rim");
}

#[test]
fn axis_aligned_query_avoids_redundant_domain_work_without_changing_program_values() {
    let features = SurfaceFeatures::new(surface(), 0.003, 100_000).unwrap();
    for center in [Vec3::ZERO, Vec3::new(-0.3, 0.1, 0.1), Vec3::new(3., 1., 0.)] {
        let lo = center - Vec3::splat(0.04);
        let hi = center + Vec3::splat(0.04);
        let points = std::array::from_fn(|i| {
            Vec3::new(
                if i & 1 == 0 { lo.x } else { hi.x },
                if i & 2 == 0 { lo.y } else { hi.y },
                if i & 4 == 0 { lo.z } else { hi.z },
            )
        });
        let generic = features.sample_cell(points, 100_000).unwrap();
        let aligned = features.sample_axis_aligned_cell(lo, hi, 100_000).unwrap();
        assert_eq!(generic.expression, aligned.expression);
        assert_eq!(
            generic.features.iter().map(|f| (f.id, f.values)).collect::<Vec<_>>(),
            aligned.features.iter().map(|f| (f.id, f.values)).collect::<Vec<_>>()
        );
        assert_eq!(generic.work - aligned.work, 12);
    }
    assert!(features.sample_axis_aligned_cell(Vec3::ZERO, Vec3::ZERO, 100).is_err());
    assert!(features.sample_features(&[u64::MAX], Vec3::ZERO, 100).is_err());
}

#[test]
fn supporting_capsules_contain_the_native_field_instead_of_detached_inside_islands() {
    let surface = surface();
    let features = SurfaceFeatures::with_supporting_planes(surface.clone(), 0.003, 1_000_000).unwrap();
    let program = features.global_program(1_000_000).unwrap();
    let mut seed = 97432u32;
    let mut random = || {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(seed) / f64::from(u32::MAX)
    };
    let mut interior = 0;
    for _ in 0..10_000 {
        let u = random();
        let v = random() * (1. - u);
        let vs = surface.vertices();
        let p = vs[0] * (1. - u as f32 - v as f32)
            + vs[1] * u as f32
            + vs[2] * v as f32
            + Vec3::new(
                (random() - 0.5) as f32 * 0.008,
                (random() - 0.5) as f32 * 0.008,
                (random() - 0.5) as f32 * 0.008,
            );
        let native = surface.distance(p);
        let values = features.sample_features(&program.ids, p, 1_000_000).unwrap();
        let proxy = program.expression.evaluate(&values.values).unwrap();
        if native < -1e-7 {
            assert!(proxy <= 0., "native interior omitted at{p:?}:native{native},proxy{proxy}");
            interior += 1;
        }
    }
    assert!(interior > 1000);
}

fn triangle_intersects_box(vertices: [Vec3; 3], center: Vec3, radius: f64) -> bool {
    let mut polygon: Vec<[f64; 3]> =
        vertices.iter().map(|p| [f64::from(p.x), f64::from(p.y), f64::from(p.z)]).collect();
    let center = [f64::from(center.x), f64::from(center.y), f64::from(center.z)];
    // Independent geometric clipping, rather than repeating the SAT axis test.
    for (axis, &coordinate) in center.iter().enumerate() {
        for sign in [-1.0, 1.0] {
            if polygon.is_empty() {
                return false;
            }
            let distance = |p: [f64; 3]| sign * (p[axis] - coordinate) - radius;
            let mut next = Vec::new();
            for i in 0..polygon.len() {
                let a = polygon[i];
                let b = polygon[(i + 1) % polygon.len()];
                let da = distance(a);
                let db = distance(b);
                if da <= 0. {
                    next.push(a);
                }
                if (da < 0. && db > 0.) || (da > 0. && db < 0.) {
                    let t = da / (da - db);
                    next.push(std::array::from_fn(|i| a[i] + t * (b[i] - a[i])));
                }
            }
            polygon = next;
        }
    }
    !polygon.is_empty()
}

#[test]
fn common_box_kernel_matches_independent_triangle_box_clipping() {
    let source = surface();
    let radius = 0.075;
    let features = SurfaceFeatures::with_common_box_kernel(source.clone(), radius, 100_000).unwrap();
    assert!(features.feature_count() <= 26);
    let program = features.global_program(100_000).unwrap();
    let triangle = source.triangles()[0].map(|i| source.vertices()[i as usize]);
    let mut seed = 194733u32;
    let mut random = || {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(seed) / f64::from(u32::MAX)
    };
    let mut inside = 0;
    let mut outside = 0;
    for _ in 0..20_000 {
        let p =
            Vec3::new((random() * 1.6 - 0.8) as f32, (random() * 1.5 - 0.55) as f32, (random() * 0.4 - 0.15) as f32);
        let expected = triangle_intersects_box(triangle, p, radius);
        assert_eq!(features.contains(p), expected, "triangle-box membership at {p:?}");
        let values = features.sample_features(&program.ids, p, 100_000).unwrap();
        assert_eq!(program.expression.evaluate(&values.values).unwrap() <= 0., expected);
        if expected {
            inside += 1;
        } else {
            outside += 1;
        }
    }
    assert!(inside > 1000 && outside > 1000);
    let exact_work = features.construction_work;
    assert!(SurfaceFeatures::with_common_box_kernel(source.clone(), radius, exact_work).is_ok());
    assert!(SurfaceFeatures::with_common_box_kernel(source, radius, exact_work - 1).is_err());
}

#[test]
fn common_box_kernel_encloses_native_sphere_but_remains_an_initial_proxy() {
    let source = surface();
    let radius = 0.003;
    let features = SurfaceFeatures::with_common_box_kernel(source.clone(), radius, 100_000).unwrap();
    let mut seed = 67139u32;
    let mut random = || {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        f64::from(seed) / f64::from(u32::MAX)
    };
    let mut checked = 0;
    for _ in 0..10_000 {
        let u = random();
        let v = random() * (1. - u);
        let vertices = source.vertices();
        let p = vertices[0] * (1. - u as f32 - v as f32)
            + vertices[1] * u as f32
            + vertices[2] * v as f32
            + Vec3::new(
                ((random() - 0.5) * 0.008) as f32,
                ((random() - 0.5) * 0.008) as f32,
                ((random() - 0.5) * 0.008) as f32,
            );
        if source.distance(p) < -1e-7 {
            assert!(features.contains(p), "native interior omitted at {p:?}");
            checked += 1;
        }
    }
    assert!(checked > 1000);
    let source =
        TriangleSurface::new(vec![Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.)], vec![[0, 1, 2]], 0.125)
            .unwrap();
    let features = SurfaceFeatures::with_common_box_kernel(source.clone(), 0.125, 100_000).unwrap();
    for &vertex in source.vertices() {
        for x in [-0.125, 0.125] {
            for y in [-0.125, 0.125] {
                for z in [-0.125, 0.125] {
                    assert!(features.contains(vertex + Vec3::new(x, y, z)));
                }
            }
        }
    }
    let cube_corner = Vec3::splat(-0.125);
    assert!(features.contains(cube_corner));
    assert!(source.distance(cube_corner) > 0.08, "cube must not be mistaken for the native rounded surface");
}

#[test]
fn common_kernel_preserves_convex_planar_union_across_opposite_windings() {
    let source = TriangleSurface::new(
        vec![Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(1., 1., 0.), Vec3::new(0., 1., 0.)],
        vec![[0, 1, 2], [0, 3, 2]],
        0.125,
    )
    .unwrap();
    let features = SurfaceFeatures::with_common_box_kernel(source, 0.125, 100_000).unwrap();
    assert_eq!(features.feature_count(), 10, "shared global planes must be deduplicated");
    for x in 0..=10 {
        for y in 0..=10 {
            for z in [-0.125, 0., 0.125] {
                assert!(features.contains(Vec3::new(-0.125 + x as f32 * 0.125, -0.125 + y as f32 * 0.125, z)));
            }
        }
    }
    assert!(!features.contains(Vec3::new(0.5, 0.5, 0.126)));
    let lo = Vec3::new(0.45, 0.45, -0.2);
    let hi = Vec3::new(0.55, 0.55, 0.2);
    let cell = features.sample_axis_aligned_cell(lo, hi, 100_000).unwrap();
    assert!(cell.expression.is_some());
    assert!(features
        .sample_axis_aligned_cell(lo + Vec3::splat(2.), hi + Vec3::splat(2.), 100_000)
        .unwrap()
        .expression
        .is_none());
}
