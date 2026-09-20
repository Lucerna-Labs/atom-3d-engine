use mm3e_kit::{
    meshing::Mesh,
    meshing_boolean::{extract_boolean_isosurface, BooleanExpr as Expr, MAX_BOOLEAN_CHANNELS},
    surface::TriangleSurface,
    Vec3,
};
use std::collections::HashMap;

fn leaf(i: usize) -> Box<Expr> {
    Box::new(Expr::Leaf(i))
}
fn original(p: Vec3, time: f32, out: &mut [f32]) {
    out[0] = (p - Vec3::new(0.2 * time, 0.0, 0.0)).length() - 0.6;
    out[1] = (p - Vec3::new(1.0 + 0.35 * time, 0.0, 0.0)).length() - 0.25;
    out[2] = (p - Vec3::new(0.35 + 0.2 * time, 0.0, 0.0)).length() - 0.3;
}
fn original_expr() -> Expr {
    Expr::Subtract(Box::new(Expr::Union(leaf(0), leaf(1))), leaf(2))
}
fn original_mesh(time: f32, resolution: u32) -> Mesh {
    extract_boolean_isosurface(
        3,
        |p, out| original(p, time, out),
        &original_expr(),
        Vec3::new(-1.0, -0.85, -0.85),
        Vec3::new(1.9, 0.85, 0.85),
        [resolution; 3],
    )
    .unwrap_or_else(|error| panic!("original t={time}, resolution={resolution}: {error}"))
}
fn closed(mesh: &Mesh) -> f64 {
    let mut edges: HashMap<(u32, u32), (usize, i32)> = HashMap::new();
    let mut volume = 0.0;
    let mut used = vec![false; mesh.positions.len()];
    for &[a, b, c] in &mesh.triangles {
        let [pa, pb, pc] = [a, b, c].map(|i| mesh.positions[i as usize]);
        assert!((pb - pa).cross(pc - pa).length_sq() > 0.0);
        volume += f64::from(pa.dot(pb.cross(pc))) / 6.0;
        for (i, j) in [(a, b), (b, c), (c, a)] {
            used[i as usize] = true;
            let item = edges.entry((i.min(j), i.max(j))).or_default();
            item.0 += 1;
            item.1 += if i < j { 1 } else { -1 };
        }
    }
    assert!(edges.values().all(|&(n, sum)| n == 2 && sum == 0));
    assert!(used.into_iter().all(|v| v));
    assert!(volume > 0.0);
    volume
}
fn sample_faces(mesh: &Mesh, mut visit: impl FnMut(Vec3)) {
    for &p in &mesh.positions {
        visit(p);
    }
    for tri in &mesh.triangles {
        let [a, b, c] = tri.map(|i| mesh.positions[i as usize]);
        for p in [(a + b) * 0.5, (b + c) * 0.5, (c + a) * 0.5, (a + b + c) * (1.0 / 3.0)] {
            visit(p);
        }
    }
}
fn deviation(a: &Mesh, b: &Mesh) -> f32 {
    let target = TriangleSurface::new(b.positions.clone(), b.triangles.clone(), 1e-8).unwrap();
    let mut maximum = 0.0f32;
    sample_faces(a, |p| maximum = maximum.max(target.distance(p) + target.half_thickness()));
    maximum
}

#[test]
fn original_open_rim_preserves_two_components_and_original_quality_limits_at_every_frame() {
    for time in [0.0, 0.5, 1.0] {
        let coarse = original_mesh(time, 32);
        let fine = original_mesh(time, 64);
        closed(&coarse);
        closed(&fine);
        assert_eq!(coarse.metadata.connected_components, 2, "coarse t={time}");
        assert_eq!(fine.metadata.connected_components, 2, "fine t={time}");
        let geometric = deviation(&coarse, &fine).max(deviation(&fine, &coarse));
        assert!(geometric <= 0.075, "original geometric gate t={time}: {geometric}");
        let mut residual = 0.0f32;
        sample_faces(&fine, |p| {
            let mut values = [0.; 3];
            original(p, time, &mut values);
            residual = residual.max(original_expr().evaluate(&values).unwrap().abs());
        });
        assert!(residual <= 0.025, "original scalar gate t={time}: {residual}");
        println!(
            "t={time} fine_vertices={} fine_triangles={} components=2 deviation={geometric} residual={residual}",
            fine.positions.len(),
            fine.triangles.len()
        );
    }
}

#[test]
fn nested_intersection_subtraction_and_union_have_correct_scalar_truth_and_geometry() {
    let expr = Expr::Union(Box::new(Expr::Subtract(Box::new(Expr::Intersection(leaf(0), leaf(1))), leaf(2))), leaf(3));
    let field = |p: Vec3, out: &mut [f32]| {
        out[0] = (p - Vec3::new(-0.18, 0., 0.)).length() - 0.7;
        out[1] = (p - Vec3::new(0.18, 0., 0.)).length() - 0.7;
        out[2] = (p - Vec3::new(0., 0.11, 0.)).length() - 0.22;
        out[3] = (p - Vec3::new(1.05, 0., 0.)).length() - 0.2;
    };
    let mesh =
        extract_boolean_isosurface(4, field, &expr, Vec3::new(-1., -1., -1.), Vec3::new(1.5, 1., 1.), [32; 3]).unwrap();
    closed(&mesh);
    assert_eq!(mesh.metadata.connected_components, 3);
    sample_faces(&mesh, |p| {
        let mut values = [0.; 4];
        field(p, &mut values);
        assert!(expr.evaluate(&values).unwrap().abs() < 0.02);
    });
    assert_eq!(expr.evaluate(&[-1., -1., 1., 1.]).unwrap(), -1.);
    assert_eq!(expr.evaluate(&[-1., -1., -1., 1.]).unwrap(), 1.);
}

#[test]
fn identical_leaf_operations_are_deterministic_and_self_subtraction_has_no_surface() {
    let sample = |p: Vec3, out: &mut [f32]| {
        out[0] = p.length() - 0.7;
        out[1] = out[0];
    };
    let min = Vec3::splat(-1.1);
    let max = Vec3::splat(1.1);
    let one = extract_boolean_isosurface(2, sample, &Expr::Leaf(0), min, max, [20; 3]).unwrap();
    for expr in [Expr::Union(leaf(0), leaf(1)), Expr::Intersection(leaf(0), leaf(1))] {
        let mesh = extract_boolean_isosurface(2, sample, &expr, min, max, [20; 3]).unwrap();
        closed(&mesh);
        assert_eq!(mesh.positions, one.positions);
        assert_eq!(mesh.triangles, one.triangles);
    }
    let error =
        extract_boolean_isosurface(2, sample, &Expr::Subtract(leaf(0), leaf(1)), min, max, [20; 3]).unwrap_err();
    assert!(error.contains("no nondegenerate isosurface"), "{error}");
}

#[test]
fn invalid_channels_work_and_clipping_are_rejected_before_output() {
    let min = Vec3::splat(-1.);
    let max = Vec3::splat(1.);
    assert!(extract_boolean_isosurface(
        0,
        |_, _| panic!("invalid channels must not sample"),
        &Expr::Leaf(0),
        min,
        max,
        [8; 3]
    )
    .is_err());
    assert!(extract_boolean_isosurface(
        MAX_BOOLEAN_CHANNELS + 1,
        |_, _| panic!("invalid channels must not sample"),
        &Expr::Leaf(0),
        min,
        max,
        [8; 3]
    )
    .is_err());
    assert!(extract_boolean_isosurface(
        128,
        |_, _| panic!("aggregate storage budget must fail before sampling"),
        &Expr::Leaf(0),
        min,
        max,
        [128; 3]
    )
    .unwrap_err()
    .contains("product"));
    assert!(extract_boolean_isosurface(1, |_, out| out[0] = f32::NAN, &Expr::Leaf(0), min, max, [8; 3])
        .unwrap_err()
        .contains("nonfinite"));
    assert!(extract_boolean_isosurface(1, |p, out| out[0] = p.x, &Expr::Leaf(0), min, max, [8; 3])
        .unwrap_err()
        .contains("bounds"));
    assert!(extract_boolean_isosurface(1, |_, _| {}, &Expr::Leaf(0), min, max, [8; 3]).is_err());
}

#[test]
fn tangent_zero_volume_and_vertex_contacts_are_explicit_errors() {
    let field = |p: Vec3, out: &mut [f32]| {
        out[0] = (p - Vec3::new(-0.4, 0., 0.)).length() - 0.4;
        out[1] = (p - Vec3::new(0.4, 0., 0.)).length() - 0.4;
    };
    let min = Vec3::splat(-1.);
    let max = Vec3::splat(1.);
    let intersection =
        extract_boolean_isosurface(2, field, &Expr::Intersection(leaf(0), leaf(1)), min, max, [20; 3]).unwrap_err();
    assert!(
        intersection.contains("no nondegenerate isosurface") || intersection.contains("singular"),
        "{intersection}"
    );
    let union = extract_boolean_isosurface(2, field, &Expr::Union(leaf(0), leaf(1)), min, max, [20; 3]).unwrap_err();
    assert!(union.contains("singular") || union.contains("nonmanifold"), "{union}");
}

#[test]
fn close_distinct_boolean_components_are_not_merged_by_a_tolerance() {
    let field = |p: Vec3, out: &mut [f32]| {
        let center = Vec3::new(0.400_000_1, 0., 0.);
        out[0] = (p - center).length() - 0.4;
        out[1] = (p + center).length() - 0.4;
    };
    let mesh = extract_boolean_isosurface(
        2,
        field,
        &Expr::Union(leaf(0), leaf(1)),
        Vec3::splat(-1.),
        Vec3::splat(1.),
        [20; 3],
    )
    .unwrap();
    closed(&mesh);
    assert_eq!(mesh.metadata.connected_components, 2);
    assert!(mesh.positions.iter().any(|p| p.x > 0. && p.x < 1e-6));
    assert!(mesh.positions.iter().any(|p| p.x < 0. && p.x > -1e-6));
    assert!(mesh.positions.iter().all(|p| p.x != 0.));
    assert_eq!(mesh.metadata.boolean_channels, 2);
    assert!(mesh.metadata.boolean_arrangement_work > 0);
}

#[test]
fn caller_arrangement_budget_is_checked_before_overshoot_and_never_returns_partial_geometry() {
    use mm3e_kit::meshing_boolean::extract_boolean_isosurface_with_work_limit;
    let expr = Expr::Leaf(0);
    let min = Vec3::splat(-1.1);
    let max = Vec3::splat(1.1);
    let field = |p: Vec3, out: &mut [f32]| out[0] = p.length() - 0.7;
    assert!(extract_boolean_isosurface_with_work_limit(
        1,
        |_, _| panic!("zero budget must fail before sampling"),
        &expr,
        min,
        max,
        [20; 3],
        0
    )
    .is_err());
    let error = extract_boolean_isosurface_with_work_limit(1, field, &expr, min, max, [20; 3], 1).unwrap_err();
    assert!(error.contains("bounded work"), "{error}");
    let mesh = extract_boolean_isosurface(1, field, &expr, min, max, [20; 3]).unwrap();
    let exact = mesh.metadata.boolean_arrangement_work;
    assert_eq!(mesh, extract_boolean_isosurface_with_work_limit(1, field, &expr, min, max, [20; 3], exact).unwrap());
    assert!(extract_boolean_isosurface_with_work_limit(1, field, &expr, min, max, [20; 3], exact - 1)
        .unwrap_err()
        .contains("bounded work"));
}

fn backwards_faces(mesh: &Mesh, field: impl Fn(Vec3) -> f32) -> usize {
    mesh.triangles
        .iter()
        .filter(|tri| {
            let [a, b, c] = tri.map(|i| mesh.positions[i as usize]);
            let a64 = [f64::from(a.x), f64::from(a.y), f64::from(a.z)];
            let b64 = [f64::from(b.x), f64::from(b.y), f64::from(b.z)];
            let c64 = [f64::from(c.x), f64::from(c.y), f64::from(c.z)];
            let u = std::array::from_fn::<_, 3, _>(|i| b64[i] - a64[i]);
            let v = std::array::from_fn::<_, 3, _>(|i| c64[i] - a64[i]);
            let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
            let length = n.iter().map(|v| v * v).sum::<f64>().sqrt();
            assert!(length > 0.0);
            let n = n.map(|v| (v / length) as f32);
            let center = (a + b + c) * (1.0 / 3.0);
            let center = [center.x, center.y, center.z];
            let plus = std::array::from_fn::<_, 3, _>(|i| (f64::from(center[i]) + f64::from(n[i]) * 0.001) as f32);
            let minus = std::array::from_fn::<_, 3, _>(|i| (f64::from(center[i]) - f64::from(n[i]) * 0.001) as f32);
            f64::from(field(Vec3::new(plus[0], plus[1], plus[2]))) + 1e-5
                < f64::from(field(Vec3::new(minus[0], minus[1], minus[2])))
        })
        .count()
}

#[test]
fn projected_original_rim_passes_the_unchanged_native_normal_probe_at_all_frames() {
    use mm3e_kit::meshing_boolean::{extract_boolean_isosurface_with_options, BooleanExtractionOptions};
    for time in [0., 0.5, 1.] {
        let field = |p| {
            let mut out = [0.; 3];
            original(p, time, &mut out);
            out[0].min(out[1]).max(-out[2])
        };
        let mut meshes = Vec::new();
        for resolution in [64, 32] {
            let mesh = extract_boolean_isosurface_with_options(
                3,
                |p, out| original(p, time, out),
                &original_expr(),
                Vec3::new(-1., -0.85, -0.85),
                Vec3::new(1.9, 0.85, 0.85),
                [resolution; 3],
                BooleanExtractionOptions {
                    project_and_refine: resolution == 64,
                    field_cost_per_callback: 3,
                    ..Default::default()
                },
            )
            .unwrap_or_else(|e| panic!("projected original t={time}, grid={resolution}: {e}"));
            closed(&mesh);
            assert_eq!(mesh.metadata.connected_components, 2);
            if resolution == 64 {
                assert_eq!(backwards_faces(&mesh, field), 0, "t={time}, grid={resolution}");
                assert!(mesh.metadata.boolean_extra_field_evaluations > 0);
            }
            let mut residual = 0.0f32;
            sample_faces(&mesh, |p| residual = residual.max(field(p).abs()));
            if resolution == 64 {
                assert!(residual <= 0.025);
            }
            println!("projected t={time} grid={resolution} vertices={} triangles={} added={} passes={} extra_calls={} displacement={} residual={residual}",
                mesh.positions.len(),mesh.triangles.len(),mesh.metadata.boolean_added_vertices,mesh.metadata.boolean_refinement_passes,
                mesh.metadata.boolean_extra_field_evaluations,mesh.metadata.boolean_max_vertex_displacement);
            meshes.push(mesh);
        }
        assert!(deviation(&meshes[0], &meshes[1]).max(deviation(&meshes[1], &meshes[0])) <= 0.075);
    }
}

#[test]
fn refined_nested_booleans_and_two_sided_shell_keep_actual_field_normals() {
    use mm3e_kit::meshing_boolean::{extract_boolean_isosurface_with_options, BooleanExtractionOptions};
    let expr = Expr::Union(Box::new(Expr::Subtract(Box::new(Expr::Intersection(leaf(0), leaf(1))), leaf(2))), leaf(3));
    let field = |p: Vec3, out: &mut [f32]| {
        out[0] = (p - Vec3::new(-0.18, 0., 0.)).length() - 0.7;
        out[1] = (p - Vec3::new(0.18, 0., 0.)).length() - 0.7;
        out[2] = (p - Vec3::new(0., 0.11, 0.)).length() - 0.22;
        out[3] = (p - Vec3::new(1.05, 0., 0.)).length() - 0.2;
    };
    let options =
        BooleanExtractionOptions { project_and_refine: true, field_cost_per_callback: 4, ..Default::default() };
    let mesh = extract_boolean_isosurface_with_options(
        4,
        field,
        &expr,
        Vec3::splat(-1.),
        Vec3::new(1.5, 1., 1.),
        [32; 3],
        options,
    )
    .unwrap();
    closed(&mesh);
    assert_eq!(mesh.metadata.connected_components, 3);
    assert_eq!(
        backwards_faces(&mesh, |p| {
            let mut v = [0.; 4];
            field(p, &mut v);
            expr.evaluate(&v).unwrap()
        }),
        0
    );
    let expr = Expr::Intersection(leaf(0), leaf(1));
    let shell = |p: Vec3, out: &mut [f32]| {
        let d = p.length() - 0.7;
        out[0] = d - 0.04;
        out[1] = -d - 0.04;
    };
    let mesh =
        extract_boolean_isosurface_with_options(2, shell, &expr, Vec3::splat(-1.), Vec3::splat(1.), [32; 3], options)
            .unwrap();
    closed(&mesh);
    assert_eq!(mesh.metadata.connected_components, 2);
    assert_eq!(backwards_faces(&mesh, |p| (p.length() - 0.7).abs() - 0.04), 0);
}

#[test]
fn refined_projection_handles_translated_scaled_fields_with_native_float_probes() {
    use mm3e_kit::meshing_boolean::{extract_boolean_isosurface_with_options, BooleanExtractionOptions};
    let shift = Vec3::new(2.1, -1.3, 3.7);
    let scale = 1.3;
    let field = |p: Vec3, out: &mut [f32]| {
        original((p - shift) * (1.0 / scale), 0.5, out);
        for v in out {
            *v *= scale;
        }
    };
    let mesh = extract_boolean_isosurface_with_options(
        3,
        field,
        &original_expr(),
        shift + Vec3::new(-1., -0.85, -0.85) * scale,
        shift + Vec3::new(1.9, 0.85, 0.85) * scale,
        [64; 3],
        BooleanExtractionOptions { project_and_refine: true, field_cost_per_callback: 3, ..Default::default() },
    )
    .unwrap();
    closed(&mesh);
    assert_eq!(mesh.metadata.connected_components, 2);
    assert_eq!(
        backwards_faces(&mesh, |p| {
            let mut v = [0.; 3];
            field(p, &mut v);
            v[0].min(v[1]).max(-v[2])
        }),
        0
    );
}

#[test]
fn projection_extra_callbacks_are_precharged_and_nonfinite_results_never_deliver_geometry() {
    use mm3e_kit::meshing_boolean::{extract_boolean_isosurface_with_options, BooleanExtractionOptions};
    use std::cell::Cell;
    let min = Vec3::splat(-1.1);
    let max = Vec3::splat(1.1);
    let expr = Expr::Leaf(0);
    let grid = [12; 3];
    let samples = 13usize.pow(3);
    let calls = Cell::new(0usize);
    let field = |p: Vec3, out: &mut [f32]| {
        calls.set(calls.get() + 1);
        out[0] = p.length() - 0.7;
    };
    let options = BooleanExtractionOptions {
        project_and_refine: true,
        field_cost_per_callback: usize::MAX,
        ..Default::default()
    };
    assert!(extract_boolean_isosurface_with_options(1, field, &expr, min, max, grid, options)
        .unwrap_err()
        .contains("extra field evaluations"));
    assert_eq!(calls.get(), samples, "work must be charged before the first extra callback");
    calls.set(0);
    let invalid = |p: Vec3, out: &mut [f32]| {
        calls.set(calls.get() + 1);
        out[0] = if calls.get() > samples { f32::NAN } else { p.length() - 0.7 };
    };
    let options = BooleanExtractionOptions { field_cost_per_callback: 3, ..options };
    assert!(extract_boolean_isosurface_with_options(1, invalid, &expr, min, max, grid, options)
        .unwrap_err()
        .contains("nonfinite"));
    assert_eq!(calls.get(), samples + 1);
    let simple = |p: Vec3, out: &mut [f32]| out[0] = p.length() - 0.7;
    let mesh = extract_boolean_isosurface_with_options(1, simple, &expr, min, max, grid, options).unwrap();
    let work = mesh.metadata.boolean_arrangement_work + mesh.metadata.boolean_extra_field_evaluations * 3;
    assert_eq!(
        mesh,
        extract_boolean_isosurface_with_options(
            1,
            simple,
            &expr,
            min,
            max,
            grid,
            BooleanExtractionOptions { max_postgrid_work: work, ..options }
        )
        .unwrap()
    );
    assert!(extract_boolean_isosurface_with_options(
        1,
        simple,
        &expr,
        min,
        max,
        grid,
        BooleanExtractionOptions { max_postgrid_work: work - 1, ..options }
    )
    .is_err());
}

#[test]
fn normal_probe_policy_parameters_validate_without_changing_film_defaults() {
    use mm3e_kit::meshing_boolean::{extract_boolean_isosurface_with_options, BooleanExtractionOptions};
    let defaults = BooleanExtractionOptions::default();
    assert_eq!(defaults.normal_probe_distance_m, 0.001_f64);
    assert_eq!(defaults.normal_comparison_tolerance, 1e-5_f64);
    let min = Vec3::splat(-1.1);
    let max = Vec3::splat(1.1);
    let expr = Expr::Leaf(0);
    for invalid in [0.0, -1.0, f64::INFINITY, f64::NAN] {
        assert!(extract_boolean_isosurface_with_options(
            1,
            |_, _| panic!("invalid policy must fail before sampling"),
            &expr,
            min,
            max,
            [8; 3],
            BooleanExtractionOptions { normal_probe_distance_m: invalid, ..defaults }
        )
        .is_err());
    }
    for invalid in [-1.0, f64::INFINITY, f64::NAN] {
        assert!(extract_boolean_isosurface_with_options(
            1,
            |_, _| panic!("invalid policy must fail before sampling"),
            &expr,
            min,
            max,
            [8; 3],
            BooleanExtractionOptions { normal_comparison_tolerance: invalid, ..defaults }
        )
        .is_err());
    }
    let mesh = extract_boolean_isosurface_with_options(
        1,
        |p, out| out[0] = p.length() - 0.7,
        &expr,
        min,
        max,
        [12; 3],
        BooleanExtractionOptions {
            project_and_refine: true,
            normal_probe_distance_m: 0.002,
            normal_comparison_tolerance: 0.0,
            ..defaults
        },
    )
    .unwrap();
    closed(&mesh);
}

#[test]
fn negative_channel_intersection_preserves_original_subtraction_geometry_and_quality() {
    use mm3e_kit::meshing_boolean::{extract_boolean_isosurface_with_options, BooleanExtractionOptions};
    let negative_expr = Expr::Intersection(Box::new(Expr::Union(leaf(0), leaf(1))), leaf(2));
    for time in [1.0, 0.0, 0.5] {
        let mut references = Vec::new();
        for negative in [false, true] {
            let expr = if negative { &negative_expr } else { &original_expr() };
            let channel = |p: Vec3, out: &mut [f32]| {
                original(p, time, out);
                if negative {
                    out[2] = -out[2];
                }
            };
            let scalar = |p| {
                let mut values = [0.; 3];
                original(p, time, &mut values);
                values[0].min(values[1]).max(-values[2])
            };
            let coarse = extract_boolean_isosurface(
                3,
                channel,
                expr,
                Vec3::new(-1., -0.85, -0.85),
                Vec3::new(1.9, 0.85, 0.85),
                [32; 3],
            )
            .unwrap();
            let fine = extract_boolean_isosurface_with_options(
                3,
                channel,
                expr,
                Vec3::new(-1., -0.85, -0.85),
                Vec3::new(1.9, 0.85, 0.85),
                [64; 3],
                BooleanExtractionOptions { project_and_refine: true, field_cost_per_callback: 3, ..Default::default() },
            )
            .unwrap_or_else(|e| panic!("negative={negative}, time={time}: {e}"));
            closed(&coarse);
            closed(&fine);
            assert_eq!(coarse.metadata.connected_components, 2);
            assert_eq!(fine.metadata.connected_components, 2);
            assert_eq!(backwards_faces(&fine, scalar), 0);
            assert!(deviation(&coarse, &fine).max(deviation(&fine, &coarse)) <= 0.075);
            let mut residual = 0.0f32;
            sample_faces(&fine, |p| residual = residual.max(scalar(p).abs()));
            assert!(residual <= 0.025);
            println!(
                "representation negative={negative} t={time}: vertices={} triangles={} refinement_passes={}",
                fine.positions.len(),
                fine.triangles.len(),
                fine.metadata.boolean_refinement_passes
            );
            references.push((coarse, fine));
        }
        assert_eq!(
            references[0].0.positions, references[1].0.positions,
            "affine points depend on equivalent sign representation"
        );
        assert_eq!(
            references[0].0.triangles, references[1].0.triangles,
            "affine diagonals depend on equivalent sign representation"
        );
        assert_eq!(
            references[0].1.positions, references[1].1.positions,
            "projected points depend on equivalent sign representation"
        );
        assert_eq!(
            references[0].1.triangles, references[1].1.triangles,
            "refinement depends on equivalent sign representation"
        );
    }
}
