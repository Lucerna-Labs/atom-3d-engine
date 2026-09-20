use mm3e_kit::{
    meshing::{extract_isosurface, Mesh},
    meshing_boolean::{extract_boolean_isosurface, BooleanExpr},
    meshing_local::{
        extract_local_boolean_isosurface, LocalCell, LocalCellData, LocalExtraction, LocalExtractionOptions,
        LocalFeature,
    },
    Vec3,
};
use std::collections::{BTreeMap, BTreeSet};

fn options() -> LocalExtractionOptions {
    LocalExtractionOptions { max_work: 64_000_000, max_vertices: 200_000, max_triangles: 400_000 }
}
fn intersection(indices: impl IntoIterator<Item = usize>) -> BooleanExpr {
    indices
        .into_iter()
        .map(BooleanExpr::Leaf)
        .reduce(|a, b| BooleanExpr::Intersection(Box::new(a), Box::new(b)))
        .unwrap()
}
fn box_values(p: Vec3, center: [f32; 3], half: [f32; 3]) -> [f32; 6] {
    let q = [p.x - center[0], p.y - center[1], p.z - center[2]];
    [q[0] - half[0], -q[0] - half[0], q[1] - half[1], -q[1] - half[1], q[2] - half[2], -q[2] - half[2]]
}
fn fixed_cell(cell: &LocalCell, remaining: usize, reverse: bool) -> Result<LocalCellData, String> {
    let work = 48;
    if work > remaining {
        return Err("test source work exhausted".into());
    }
    let mut features: Vec<_> = (0..6)
        .map(|i| LocalFeature {
            id: 1000 + i as u64 * 17,
            values: cell.points.map(|p| box_values(p, [0.1, 0.05, 0.03], [0.63, 0.57, 0.001])[i]),
        })
        .collect();
    if reverse {
        features.reverse();
    }
    let order: Vec<_> = if reverse { (0..6).rev().collect() } else { (0..6).collect() };
    Ok(LocalCellData { features, expression: Some(intersection(order)), work })
}
fn mesh_closed(mesh: &Mesh) {
    let mut edges = BTreeMap::<(u32, u32), (usize, i32)>::new();
    for &[a, b, c] in &mesh.triangles {
        for (a, b) in [(a, b), (b, c), (c, a)] {
            let e = edges.entry((a.min(b), a.max(b))).or_default();
            e.0 += 1;
            e.1 += if a < b { 1 } else { -1 };
        }
    }
    assert!(edges.values().all(|&(count, balance)| count == 2 && balance == 0));
    assert!(mesh.metadata.signed_volume > 0.0);
}
fn labels(result: &LocalExtraction, allowed: &BTreeSet<u64>) {
    assert_eq!(result.face_feature_ids.len(), result.mesh.triangles.len());
    assert_eq!(result.vertex_support_ids.len(), result.mesh.positions.len());
    for (face, &id) in result.mesh.triangles.iter().zip(&result.face_feature_ids) {
        assert!(allowed.contains(&id));
        for &vertex in face {
            assert!(result.vertex_support_ids[vertex as usize].contains(&id));
        }
    }
    for support in &result.vertex_support_ids {
        assert!(!support.is_empty());
        assert!(support.windows(2).all(|v| v[0] < v[1]));
        assert!(support.iter().all(|id| allowed.contains(id)));
    }
}

#[test]
fn thin_slab_with_no_negative_combined_grid_node_matches_global_arrangement() {
    let min = Vec3::splat(-1.);
    let max = Vec3::splat(1.);
    let grid = [8; 3];
    let scalar =
        |p| box_values(p, [0.1, 0.05, 0.03], [0.63, 0.57, 0.001]).into_iter().fold(f32::NEG_INFINITY, f32::max);
    assert!(
        extract_isosurface(scalar, min, max, grid).is_err(),
        "combined scalar control unexpectedly resolved .002m slab"
    );
    let local = extract_local_boolean_isosurface(min, max, grid, options(), |cell, remaining| {
        fixed_cell(cell, remaining, false)
    })
    .unwrap();
    let global = extract_boolean_isosurface(
        6,
        |p, values| values.copy_from_slice(&box_values(p, [0.1, 0.05, 0.03], [0.63, 0.57, 0.001])),
        &intersection(0..6),
        min,
        max,
        grid,
    )
    .unwrap();
    assert_eq!(local.mesh.positions, global.positions);
    assert_eq!(local.mesh.triangles, global.triangles);
    mesh_closed(&local.mesh);
    let low = local.mesh.positions.iter().map(|p| p.z).fold(f32::INFINITY, f32::min);
    let high = local.mesh.positions.iter().map(|p| p.z).fold(f32::NEG_INFINITY, f32::max);
    assert!((low - 0.029).abs() < 1e-6 && (high - 0.031).abs() < 1e-6);
    assert!((local.mesh.metadata.signed_volume - 8.0 * 0.63 * 0.57 * 0.001).abs() < 1e-7);
    labels(&local, &(0..6).map(|i| 1000 + i * 17).collect());
    assert!(local.report.charged_work() <= options().max_work);
}

#[test]
fn per_cell_channel_order_does_not_change_welds_or_global_feature_provenance() {
    let a = extract_local_boolean_isosurface(Vec3::splat(-1.), Vec3::splat(1.), [8; 3], options(), |cell, budget| {
        fixed_cell(cell, budget, false)
    })
    .unwrap();
    let b = extract_local_boolean_isosurface(Vec3::splat(-1.), Vec3::splat(1.), [8; 3], options(), |cell, budget| {
        fixed_cell(cell, budget, (cell.index[0] + cell.index[1] + cell.index[2]) % 2 == 1)
    })
    .unwrap();
    assert_eq!(a.mesh.positions, b.mesh.positions);
    assert_eq!(a.mesh.triangles, b.mesh.triangles);
    assert_eq!(a.mesh.metadata, b.mesh.metadata);
    assert_eq!(a.face_feature_ids, b.face_feature_ids);
    assert_eq!(a.vertex_support_ids, b.vertex_support_ids);
}

#[test]
fn original_thin_triangle_local_and_global_use_the_same_unmodified_feature_program() {
    use mm3e_kit::{surface::TriangleSurface, surface_features::SurfaceFeatures};
    let surface = TriangleSurface::new(
        vec![Vec3::new(-0.4, -0.3, 0.), Vec3::new(0.4, -0.3, 0.), Vec3::new(0., 0.4, 0.)],
        vec![[0, 1, 2]],
        0.001,
    )
    .unwrap();
    let features = SurfaceFeatures::new(surface, 0.001, 1_000_000).unwrap();
    let program = features.global_program(1000000).unwrap();
    let lo = Vec3::new(-0.5, -0.5, -0.044);
    let hi = Vec3::new(0.5, 0.5, 0.036);
    let grid = [8, 8, 10];
    let global = extract_boolean_isosurface(
        program.ids.len(),
        |p, out| out.copy_from_slice(&features.sample_features(&program.ids, p, 1000000).unwrap().values),
        &program.expression,
        lo,
        hi,
        grid,
    );
    let local =
        extract_local_boolean_isosurface(lo, hi, grid, options(), |cell, work| features.sample_cell(cell.points, work));
    println!(
        "original thin triangle global: {:?}; local: {:?}",
        global.as_ref().map(|m| (m.positions.len(), m.triangles.len())),
        local.as_ref().map(|m| (m.mesh.positions.len(), m.mesh.triangles.len()))
    );
    let local = local.unwrap();
    mesh_closed(&local.mesh);
    labels(&local, &program.ids.iter().copied().collect());
    assert!(local.report.exact_support_reductions > 0);
    for (point, ids) in local.mesh.positions.iter().zip(&local.vertex_support_ids) {
        let values = features.sample_features(ids, *point, 1000000).unwrap();
        assert!(values.values.iter().all(|value| value.abs() < 1e-6));
    }
}

#[test]
fn sparse_local_cells_support_more_than_128_global_features_without_a_dense_grid() {
    let count = 25usize; // 150 distinct global planes, at most two boxes per cell.
    let result = extract_local_boolean_isosurface(
        Vec3::new(-0.5, -0.7, -0.8),
        Vec3::new(count as f32 - 0.5, 0.7, 0.8),
        [2 * count as u32, 4, 4],
        options(),
        |cell, remaining| {
            let mut work = 0;
            let mut features = Vec::new();
            let mut expr = None;
            for i in 0..count {
                if work >= remaining {
                    return Err("test AABB-query work exhausted".into());
                }
                work += 1;
                let center = [i as f32 + 0.03, 0.04, 0.03];
                let half = [0.27, 0.23, 0.19];
                let lo = [cell.points[0].x, cell.points[0].y, cell.points[0].z];
                let hi = [cell.points[7].x, cell.points[7].y, cell.points[7].z];
                if (0..3).any(|axis| center[axis] + half[axis] < lo[axis] || center[axis] - half[axis] > hi[axis]) {
                    continue;
                }
                if remaining - work < 48 {
                    return Err("test feature-sample work exhausted".into());
                }
                work += 48;
                let start = features.len();
                for channel in 0..6 {
                    features.push(LocalFeature {
                        id: 10000 + i as u64 * 16 + channel as u64,
                        values: cell.points.map(|p| box_values(p, center, half)[channel]),
                    });
                }
                let piece = intersection(start..start + 6);
                expr = Some(match expr {
                    None => piece,
                    Some(previous) => BooleanExpr::Union(Box::new(previous), Box::new(piece)),
                });
            }
            Ok(LocalCellData { features, expression: expr, work })
        },
    )
    .unwrap();
    mesh_closed(&result.mesh);
    assert_eq!(result.mesh.metadata.connected_components, count);
    assert_eq!(result.report.global_features, 6 * count);
    assert!(result.report.peak_local_channels <= 12);
    assert!(result.report.proven_outside_cells > 0);
    assert!(result.report.peak_shared_values < 50_000);
    labels(&result, &(0..count).flat_map(|i| (0..6).map(move |j| 10000 + i as u64 * 16 + j)).collect());
}

#[test]
fn changing_a_global_feature_at_a_shared_node_is_rejected() {
    let error = extract_local_boolean_isosurface(Vec3::splat(-1.), Vec3::splat(1.), [4; 3], options(), |cell, _| {
        Ok(LocalCellData {
            features: vec![LocalFeature {
                id: 901,
                values: cell.points.map(|p| p.length() - 0.5 + if cell.index[0] > 0 { 0.01 } else { 0. }),
            }],
            expression: Some(BooleanExpr::Leaf(0)),
            work: 8,
        })
    })
    .unwrap_err();
    assert!(error.contains("changed value at shared node"), "{error}");
}

#[test]
fn malformed_cells_clipping_and_budget_limits_reject_without_partial_mesh() {
    for kind in 0..7 {
        let result =
            extract_local_boolean_isosurface(Vec3::splat(-1.), Vec3::splat(1.), [4; 3], options(), |_, remaining| {
                Ok(match kind {
                    0 => LocalCellData { features: vec![], expression: Some(BooleanExpr::Leaf(0)), work: 0 },
                    1 => LocalCellData {
                        features: vec![LocalFeature { id: 1, values: [1.; 8] }],
                        expression: None,
                        work: 0,
                    },
                    2 => LocalCellData {
                        features: vec![LocalFeature { id: 1, values: [1.; 8] }; 2],
                        expression: Some(BooleanExpr::Leaf(0)),
                        work: 0,
                    },
                    3 => LocalCellData {
                        features: vec![LocalFeature { id: u64::MAX, values: [1.; 8] }],
                        expression: Some(BooleanExpr::Leaf(0)),
                        work: 0,
                    },
                    4 => LocalCellData {
                        features: vec![LocalFeature { id: 1, values: [f32::NAN; 8] }],
                        expression: Some(BooleanExpr::Leaf(0)),
                        work: 0,
                    },
                    5 => LocalCellData {
                        features: vec![LocalFeature { id: 1, values: [1.; 8] }],
                        expression: Some(BooleanExpr::Leaf(1)),
                        work: 0,
                    },
                    _ => LocalCellData { features: vec![], expression: None, work: remaining + 1 },
                })
            });
        assert!(result.is_err(), "malformed case{kind} accepted");
    }
    let clipped = extract_local_boolean_isosurface(Vec3::splat(-1.), Vec3::splat(1.), [4; 3], options(), |_, _| {
        Ok(LocalCellData {
            features: vec![LocalFeature { id: 1, values: [-1.; 8] }],
            expression: Some(BooleanExpr::Leaf(0)),
            work: 0,
        })
    })
    .unwrap_err();
    assert!(clipped.contains("clips sampled bounds"));
    for limited in [
        LocalExtractionOptions { max_work: 1, ..options() },
        LocalExtractionOptions { max_vertices: 3, ..options() },
        LocalExtractionOptions { max_triangles: 1, ..options() },
    ] {
        assert!(extract_local_boolean_isosurface(
            Vec3::splat(-1.),
            Vec3::splat(1.),
            [8; 3],
            limited,
            |cell, budget| fixed_cell(cell, budget, false)
        )
        .is_err());
    }
}

#[test]
fn sorted_channel_remapping_preserves_noncommutative_subtraction() {
    let original =
        |p: Vec3| [(p - Vec3::new(-0.2, 0., 0.)).length() - 0.5, (p - Vec3::new(0.15, 0., 0.)).length() - 0.3];
    let global = extract_boolean_isosurface(
        2,
        |p, v| v.copy_from_slice(&original(p)),
        &BooleanExpr::Subtract(Box::new(BooleanExpr::Leaf(0)), Box::new(BooleanExpr::Leaf(1))),
        Vec3::splat(-1.),
        Vec3::splat(1.),
        [12; 3],
    )
    .unwrap();
    let local = extract_local_boolean_isosurface(Vec3::splat(-1.), Vec3::splat(1.), [12; 3], options(), |cell, _| {
        Ok(LocalCellData {
            features: vec![
                LocalFeature { id: 30, values: cell.points.map(|p| original(p)[0]) },
                LocalFeature { id: 10, values: cell.points.map(|p| original(p)[1]) },
            ],
            expression: Some(BooleanExpr::Subtract(Box::new(BooleanExpr::Leaf(0)), Box::new(BooleanExpr::Leaf(1)))),
            work: 16,
        })
    })
    .unwrap();
    mesh_closed(&local.mesh);
    // Global feature ordering may choose different triangulation diagonals, but
    // the exact oriented volume and composed fields must agree at their vertices.
    assert!((local.mesh.metadata.signed_volume - global.metadata.signed_volume).abs() < 1e-7);
    assert_eq!(local.mesh.metadata.connected_components, global.metadata.connected_components);
    labels(&local, &BTreeSet::from([10, 30]));
}
