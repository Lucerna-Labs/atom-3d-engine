use mm3e_kit::{
    meshing::Mesh,
    meshing_boolean::BooleanExpr,
    meshing_local::{
        extract_local_boolean_isosurface, extract_local_convex_union_isosurface,
        extract_local_convex_union_isosurface_with_representation, ConvexRepresentationPolicy, LocalCell,
        LocalCellData, LocalExtractionOptions, LocalFeature,
    },
    Vec3,
};
use std::collections::{BTreeMap, BTreeSet};

fn options() -> LocalExtractionOptions {
    LocalExtractionOptions { max_work: 64_000_000, max_vertices: 65_536, max_triangles: 131_072 }
}
fn intersection(leaves: impl IntoIterator<Item = BooleanExpr>) -> BooleanExpr {
    fn balanced(mut leaves: Vec<BooleanExpr>) -> BooleanExpr {
        if leaves.len() == 1 {
            return leaves.pop().unwrap();
        }
        let right = leaves.split_off(leaves.len() / 2);
        BooleanExpr::Intersection(Box::new(balanced(leaves)), Box::new(balanced(right)))
    }
    balanced(leaves.into_iter().collect())
}
fn union(parts: impl IntoIterator<Item = BooleanExpr>) -> BooleanExpr {
    parts.into_iter().reduce(|a, b| BooleanExpr::Union(Box::new(a), Box::new(b))).unwrap()
}
fn boxes(
    cell: &LocalCell,
    _remaining: usize,
    centers: &[[f32; 3]],
    half: [f32; 3],
    signed: bool,
) -> Result<LocalCellData, String> {
    let mut features = Vec::new();
    let mut parts = Vec::new();
    if signed {
        features.push(LocalFeature { id: 0, values: [-1.; 8] });
    }
    for center in centers {
        let mut leaves = Vec::new();
        for axis in 0..3 {
            for direction in [-1., 1.] {
                let index = features.len();
                let negate = signed && index % 2 == 1;
                features.push(LocalFeature {
                    id: index as u64 + 1,
                    values: cell.points.map(|p| {
                        let value = direction * ([p.x, p.y, p.z][axis] - center[axis]) - half[axis];
                        if negate {
                            -value
                        } else {
                            value
                        }
                    }),
                });
                leaves.push(if negate {
                    BooleanExpr::Subtract(Box::new(BooleanExpr::Leaf(0)), Box::new(BooleanExpr::Leaf(index)))
                } else {
                    BooleanExpr::Leaf(index)
                });
            }
        }
        parts.push(intersection(leaves));
    }
    Ok(LocalCellData { work: features.len() * 8, features, expression: Some(union(parts)) })
}
fn closed(mesh: &Mesh) {
    let mut edges = BTreeMap::<(u32, u32), (usize, i32)>::new();
    for &[a, b, c] in &mesh.triangles {
        for (a, b) in [(a, b), (b, c), (c, a)] {
            let entry = edges.entry((a.min(b), a.max(b))).or_default();
            entry.0 += 1;
            entry.1 += if a < b { 1 } else { -1 };
        }
    }
    assert!(edges.values().all(|&(n, b)| n == 2 && b == 0));
    assert_eq!(mesh.metadata.connected_components, 1);
    assert!(mesh.metadata.signed_volume > 0.);
}
fn provenance(result: &mm3e_kit::meshing_local::LocalExtraction) {
    for (triangle, &feature) in result.mesh.triangles.iter().zip(&result.face_feature_ids) {
        for &vertex in triangle {
            let support = &result.vertex_support_ids[vertex as usize];
            assert!(support.contains(&feature), "face feature {feature} missing from vertex {vertex}");
            assert!(support.len() <= 128);
            assert!(support.windows(2).all(|pair| pair[0] < pair[1]));
        }
    }
}
fn bounds(mesh: &Mesh) -> ([f32; 3], [f32; 3]) {
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for p in &mesh.positions {
        for (i, v) in [p.x, p.y, p.z].into_iter().enumerate() {
            lo[i] = lo[i].min(v);
            hi[i] = hi[i].max(v);
        }
    }
    (lo, hi)
}
#[test]
fn overlapping_boxes_match_general_boundary_and_analytic_volume() {
    let centers = [[-0.21, 0.03, 0.02], [0.19, 0.03, 0.02]];
    let run = |signed| {
        extract_local_convex_union_isosurface(
            Vec3::splat(-1.),
            Vec3::splat(1.),
            [8; 3],
            options(),
            |cell, remaining| boxes(cell, remaining, &centers, [0.4, 0.31, 0.22], signed),
        )
        .unwrap()
    };
    let result = run(false);
    closed(&result.mesh);
    // The old full arrangement has a known singular duplicate-plane failure
    // on these two boxes. Compare the exact same union boundary represented
    // independently as one box, leaving that legacy behavior unchanged.
    let general =
        extract_local_boolean_isosurface(Vec3::splat(-1.), Vec3::splat(1.), [8; 3], options(), |cell, remaining| {
            boxes(cell, remaining, &[[-0.01, 0.03, 0.02]], [0.6, 0.31, 0.22], false)
        })
        .unwrap();
    let (a, b) = bounds(&result.mesh);
    let (c, d) = bounds(&general.mesh);
    assert!(a.into_iter().chain(b).zip(c.into_iter().chain(d)).all(|(a, b)| (a - b).abs() < 1e-7));
    assert!((result.mesh.metadata.signed_volume - 1.2 * 0.62 * 0.44).abs() < 1e-7);
    assert!((result.mesh.metadata.surface_area - general.mesh.metadata.surface_area).abs() < 1e-6);
    let signed = run(true);
    closed(&signed.mesh);
    assert!((result.mesh.metadata.signed_volume - signed.mesh.metadata.signed_volume).abs() < 1e-7);
    for (face, &feature) in result.mesh.triangles.iter().zip(&result.face_feature_ids) {
        for &v in face {
            assert!(result.vertex_support_ids[v as usize].contains(&feature));
        }
    }
}
#[test]
fn contained_duplicate_and_disjoint_convex_volumes_keep_union_semantics() {
    for centers in [vec![[0.03, 0.02, 0.01]; 2], vec![[0.03, 0.02, 0.01]; 4]] {
        let result = extract_local_convex_union_isosurface(
            Vec3::splat(-1.),
            Vec3::splat(1.),
            [7; 3],
            options(),
            |cell, remaining| boxes(cell, remaining, &centers, [0.4, 0.31, 0.22], false),
        )
        .unwrap();
        closed(&result.mesh);
        assert!((result.mesh.metadata.signed_volume - 0.8 * 0.62 * 0.44).abs() < 1e-7);
    }
    let result = extract_local_convex_union_isosurface(
        Vec3::splat(-1.),
        Vec3::splat(1.),
        [9; 3],
        options(),
        |cell, remaining| boxes(cell, remaining, &[[-0.55, 0., 0.], [0.55, 0., 0.]], [0.2, 0.31, 0.22], false),
    )
    .unwrap();
    assert_eq!(result.mesh.metadata.connected_components, 2);
    assert!((result.mesh.metadata.signed_volume - 2. * 0.4 * 0.62 * 0.44).abs() < 1e-7);
}
#[test]
fn convex_api_rejects_non_dnf_and_preserves_work_and_component_limits() {
    let make = |cell: &LocalCell, remaining| {
        let mut data = boxes(cell, remaining, &[[0.; 3]], [0.4; 3], false)?;
        data.expression = Some(BooleanExpr::Intersection(
            Box::new(BooleanExpr::Union(Box::new(BooleanExpr::Leaf(0)), Box::new(BooleanExpr::Leaf(1)))),
            Box::new(BooleanExpr::Leaf(2)),
        ));
        Ok(data)
    };
    let error =
        extract_local_convex_union_isosurface(Vec3::splat(-1.), Vec3::splat(1.), [5; 3], options(), make).unwrap_err();
    assert!(error.contains("union of convex"), "{error}");
    let mut low = options();
    low.max_work = 100;
    assert!(extract_local_convex_union_isosurface(
        Vec3::splat(-1.),
        Vec3::splat(1.),
        [5; 3],
        low,
        |cell, remaining| boxes(cell, remaining, &[[0.; 3]], [0.4; 3], false)
    )
    .is_err());
}
#[test]
fn many_redundant_components_do_not_partition_unrelated_facets() {
    let centers: Vec<_> = (0..12).map(|i| [-0.22 + i as f32 * 0.04, 0.031, 0.019]).collect();
    let result = extract_local_convex_union_isosurface(
        Vec3::splat(-1.),
        Vec3::splat(1.),
        [2; 3],
        options(),
        |cell, remaining| boxes(cell, remaining, &centers, [0.37, 0.31, 0.22], false),
    )
    .unwrap();
    closed(&result.mesh);
    assert!((result.mesh.metadata.signed_volume - 1.18 * 0.62 * 0.44).abs() < 1e-7);
    assert_eq!(result.report.peak_local_channels, 72);
    let legacy =
        extract_local_boolean_isosurface(Vec3::splat(-1.), Vec3::splat(1.), [2; 3], options(), |cell, remaining| {
            boxes(cell, remaining, &centers, [0.37, 0.31, 0.22], false)
        })
        .unwrap_err();
    assert!(legacy.contains("32 active planes"), "{legacy}");
    let used: BTreeSet<_> = result.face_feature_ids.iter().copied().collect();
    assert!(!used.is_empty());
    assert!(result.report.charged_work() < options().max_work);
}

#[test]
fn a_single_component_keeps_its_32_active_plane_limit() {
    let error =
        extract_local_convex_union_isosurface(Vec3::splat(-1.), Vec3::splat(1.), [1; 3], options(), |cell, _| {
            let mut features = Vec::new();
            for i in 0..64 {
                let angle = std::f64::consts::TAU * i as f64 / 64.;
                features.push(LocalFeature {
                    id: i,
                    values: cell
                        .points
                        .map(|p| (f64::from(p.x) * angle.cos() + f64::from(p.y) * angle.sin() - 0.3) as f32),
                });
            }
            for sign in [-1., 1.] {
                features
                    .push(LocalFeature { id: features.len() as u64, values: cell.points.map(|p| sign * p.z - 0.3) });
            }
            let expression = Some(intersection((0..features.len()).map(BooleanExpr::Leaf)));
            Ok(LocalCellData { work: features.len() * 8, features, expression })
        })
        .unwrap_err();
    assert!(error.contains("32 active halfspaces"), "{error}");
}

#[test]
fn original_tilted_triangle_support_enclosure_has_one_connected_boundary() {
    use mm3e_kit::{surface::TriangleSurface, surface_features::SurfaceFeatures};
    let surface = TriangleSurface::new(
        vec![
            [-0.30000001192092896_f64, -0.2215023636817932, -0.11675917357206345],
            [0.30000001192092896, -0.2215023636817932, -0.11675917357206345],
            [-0.30000001192092896, 0.2927980124950409, 0.192263662815094],
        ]
        .into_iter()
        .map(|p| Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32))
        .collect(),
        vec![[0, 1, 2]],
        0.001,
    )
    .unwrap();
    let features =
        SurfaceFeatures::with_supporting_planes(surface.clone(), f64::from(surface.half_thickness()), 200_000_000)
            .unwrap();
    let result = extract_local_convex_union_isosurface_with_representation(
        Vec3::new(-0.4, -0.4, -0.3),
        Vec3::new(0.4, 0.45, 0.3),
        [32; 3],
        options(),
        ConvexRepresentationPolicy { max_representation_error_m: f64::from(surface.half_thickness()) * 0.0005 },
        |cell, remaining| features.sample_axis_aligned_cell(cell.points[0], cell.points[7], remaining),
    )
    .unwrap();
    println!(
        "tilted support mesh {} vertices / {} faces, {} work, {} conforming splits, {}m max rounding deviation",
        result.mesh.positions.len(),
        result.mesh.triangles.len(),
        result.report.charged_work(),
        result.report.convex_edges_split,
        result.report.convex_max_rounding_edge_deviation_m
    );
    closed(&result.mesh);
    provenance(&result);
    let representation = result.report.representation.as_ref().unwrap();
    assert_eq!(representation.embedding_initial_contacts, 0);
    assert_eq!(representation.embedding_repair_attempts, 0);
    assert!(
        result.report.representation.as_ref().unwrap().correspondence.max_displacement_m
            <= f64::from(surface.half_thickness()) * 0.0005
    );
    for &p in &result.mesh.positions {
        assert!(
            surface.distance(p) >= -1e-7,
            "enclosing output is inside native shell at {p:?}: {}",
            surface.distance(p)
        );
    }
}

#[test]
fn diagonal_coplanar_overlap_conforms_both_edges_of_a_retained_facet() {
    let result = extract_local_convex_union_isosurface(
        Vec3::splat(-1.),
        Vec3::splat(1.),
        [2; 3],
        options(),
        |cell, remaining| boxes(cell, remaining, &[[-0.2, -0.2, 0.], [0.2, 0.2, 0.]], [0.4, 0.4, 0.2], false),
    )
    .unwrap();
    closed(&result.mesh);
    provenance(&result);
    assert!((result.mesh.metadata.signed_volume - (2. * 0.8 * 0.8 - 0.4 * 0.4) * 0.4).abs() < 1e-7);
    assert!(result.report.convex_edges_split > 0);
}

#[test]
fn folded_native_sheet_support_enclosure_keeps_one_closed_oriented_boundary() {
    use mm3e_kit::{surface::TriangleSurface, surface_features::SurfaceFeatures};
    let surface = TriangleSurface::new(
        vec![Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(1., 1., 0.), Vec3::new(0., 1., 0.5)],
        vec![[0, 1, 2], [0, 2, 3]],
        0.1,
    )
    .unwrap();
    let features = SurfaceFeatures::with_supporting_planes(surface, 0.1, 64_000_000).unwrap();
    let result = extract_local_convex_union_isosurface_with_representation(
        Vec3::new(-0.3, -0.3, -0.3),
        Vec3::new(1.3, 1.3, 0.8),
        [12; 3],
        options(),
        ConvexRepresentationPolicy { max_representation_error_m: 0.1 * 0.0005 },
        |cell, remaining| features.sample_axis_aligned_cell(cell.points[0], cell.points[7], remaining),
    )
    .unwrap();
    println!(
        "folded support: {} vertices / {} faces, {} charged work; representation {:?}",
        result.mesh.positions.len(),
        result.mesh.triangles.len(),
        result.report.charged_work(),
        result.report.representation
    );
    closed(&result.mesh);
    provenance(&result);
    let representation = result.report.representation.as_ref().unwrap();
    assert!(representation.embedding_initial_contacts > 0);
    assert!(representation.embedding_repair_attempts > 0);
    assert!(representation.embedding_validation_work > representation.embedding.work);
    assert!(result.report.representation.as_ref().unwrap().correspondence.max_displacement_m <= 0.1 * 0.0005);
}

#[test]
fn original_flat_focused_oracle_support_enclosure_keeps_both_shell_sheets() {
    use mm3e_kit::{surface::TriangleSurface, surface_features::SurfaceFeatures};
    let surface = TriangleSurface::new(
        vec![Vec3::new(-0.3, -0.25, 0.014), Vec3::new(0.3, -0.25, 0.014), Vec3::new(-0.3, 0.35, 0.014)],
        vec![[0, 1, 2]],
        0.001,
    )
    .unwrap();
    let features =
        SurfaceFeatures::with_supporting_planes(surface.clone(), f64::from(surface.half_thickness()), 64_000_000)
            .unwrap();
    let result = extract_local_convex_union_isosurface_with_representation(
        Vec3::new(-0.4, -0.4, -0.03),
        Vec3::new(0.4, 0.45, 0.05),
        [32, 32, 8],
        options(),
        ConvexRepresentationPolicy { max_representation_error_m: f64::from(surface.half_thickness()) * 0.0005 },
        |cell, remaining| features.sample_axis_aligned_cell(cell.points[0], cell.points[7], remaining),
    )
    .unwrap();
    println!(
        "flat support: {} vertices / {} faces, {} charged work; representation {:?}",
        result.mesh.positions.len(),
        result.mesh.triangles.len(),
        result.report.charged_work(),
        result.report.representation
    );
    closed(&result.mesh);
    provenance(&result);
    let representation = result.report.representation.as_ref().unwrap();
    assert!(representation.rounding_searches.iter().any(|search| search.status == "solved"));
    assert!(representation.embedding_initial_contacts > 0);
    assert!(representation.embedding.reused_pairs > 0);
    // Independent reference verification of the returned geometry, without
    // any prepared baseline proof. This does not enlarge the producer's cap.
    mm3e_kit::surface_intersections::validate(&result.mesh.positions, &result.mesh.triangles, options().max_work)
        .unwrap();
    assert!(
        result.report.representation.as_ref().unwrap().correspondence.max_displacement_m
            <= f64::from(surface.half_thickness()) * 0.0005
    );
    let replay = |max_work| {
        extract_local_convex_union_isosurface_with_representation(
            Vec3::new(-0.4, -0.4, -0.03),
            Vec3::new(0.4, 0.45, 0.05),
            [32, 32, 8],
            LocalExtractionOptions { max_work, ..options() },
            ConvexRepresentationPolicy { max_representation_error_m: f64::from(surface.half_thickness()) * 0.0005 },
            |cell, remaining| features.sample_axis_aligned_cell(cell.points[0], cell.points[7], remaining),
        )
    };
    let exact = replay(result.report.charged_work()).unwrap();
    assert_eq!(exact.report.charged_work(), result.report.charged_work());
    assert_eq!(exact.mesh.positions, result.mesh.positions);
    assert_eq!(exact.mesh.triangles, result.mesh.triangles);
    assert!(replay(result.report.charged_work() - 1).unwrap_err().contains("work"));
}
