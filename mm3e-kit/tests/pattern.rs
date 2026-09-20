use mm3e_kit::{
    pattern::{Pattern2D, PatternMesh, TriangulationOptions, MAX_PATTERN_VERTICES},
    surface::TriangleSurface,
    Vec3,
};
use std::collections::{BTreeMap, BTreeSet};

type Point = [f64; 2];

fn rectangle(x: f64, y: f64, w: f64, h: f64) -> Vec<Point> {
    vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h]]
}

fn two_holes() -> Pattern2D {
    Pattern2D {
        outer: rectangle(0.0, 0.0, 8.0, 6.0),
        holes: vec![rectangle(1.0, 1.0, 2.0, 2.0), rectangle(5.0, 2.0, 2.0, 2.0)],
    }
}

fn cross(a: Point, b: Point, c: Point) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn area(contour: &[Point]) -> f64 {
    contour.iter().zip(contour.iter().cycle().skip(1)).map(|(a, b)| a[0] * b[1] - a[1] * b[0]).sum::<f64>() * 0.5
}

fn inside(p: Point, contour: &[Point]) -> bool {
    let mut winding = 0i32;
    for (a, b) in contour.iter().zip(contour.iter().cycle().skip(1)) {
        if a[1] <= p[1] {
            if b[1] > p[1] && cross(*a, *b, p) > 0.0 {
                winding += 1;
            }
        } else if b[1] <= p[1] && cross(*a, *b, p) < 0.0 {
            winding -= 1;
        }
    }
    winding != 0
}

fn verify_coverage(pattern: &Pattern2D, mesh: &PatternMesh, expected_area: f64) {
    let original: Vec<_> = std::iter::once(&pattern.outer).chain(&pattern.holes).flatten().copied().collect();
    assert_eq!(&mesh.points[..original.len()], original);
    assert_eq!(mesh.control_vertices.len(), pattern.holes.len() + 1);
    let mut offset = 0;
    for (contour, controls) in std::iter::once(&pattern.outer).chain(&pattern.holes).zip(&mesh.control_vertices) {
        assert_eq!(controls, &(offset..offset + contour.len() as u32).collect::<Vec<_>>());
        offset += contour.len() as u32;
    }
    let mut edges = BTreeMap::<[u32; 2], usize>::new();
    let mut referenced = BTreeSet::new();
    let mut actual_area = 0.0;
    for triangle in &mesh.triangles {
        let [a, b, c] = triangle.map(|v| mesh.points[v as usize]);
        let twice = cross(a, b, c);
        assert!(twice > 0.0 && twice.is_finite());
        actual_area += twice * 0.5;
        for i in 0..3 {
            let (a, b) = (triangle[i], triangle[(i + 1) % 3]);
            *edges.entry([a.min(b), a.max(b)]).or_default() += 1;
            referenced.insert(a);
        }
    }
    assert_eq!(referenced.len(), mesh.points.len());
    assert!((actual_area - expected_area).abs() < expected_area * 1e-10);
    let independently_outlined_area =
        area(&pattern.outer).abs() - pattern.holes.iter().map(|hole| area(hole).abs()).sum::<f64>();
    assert!((actual_area - independently_outlined_area).abs() < expected_area * 1e-10);
    assert_eq!(
        mesh.points.len() as isize - edges.len() as isize + mesh.triangles.len() as isize,
        1 - pattern.holes.len() as isize
    );
    for (index, contour) in mesh.boundary_loops.iter().enumerate() {
        let points: Vec<_> = contour.iter().map(|&v| mesh.points[v as usize]).collect();
        assert_eq!(area(&points) > 0.0, index == 0);
        for i in 0..contour.len() {
            let (a, b) = (contour[i], contour[(i + 1) % contour.len()]);
            assert_eq!(edges.remove(&[a.min(b), a.max(b)]), Some(1));
        }
    }
    assert!(edges.values().all(|&incidence| incidence == 2));

    // Independently sample coverage, skipping only points lying on triangulation
    // edges. Every material sample belongs to exactly one triangle; holes to zero.
    let lo = pattern.outer.iter().fold([f64::INFINITY; 2], |v, p| [v[0].min(p[0]), v[1].min(p[1])]);
    let hi = pattern.outer.iter().fold([f64::NEG_INFINITY; 2], |v, p| [v[0].max(p[0]), v[1].max(p[1])]);
    let mut samples = 0;
    for y in 0..41 {
        for x in 0..43 {
            let point = [
                lo[0] + (x as f64 + 0.371) / 43.0 * (hi[0] - lo[0]),
                lo[1] + (y as f64 + 0.613) / 41.0 * (hi[1] - lo[1]),
            ];
            let mut hits = 0;
            let mut on_edge = false;
            for triangle in &mesh.triangles {
                let [a, b, c] = triangle.map(|v| mesh.points[v as usize]);
                let signs = [cross(a, b, point), cross(b, c, point), cross(c, a, point)];
                on_edge |= signs.iter().any(|v| v.abs() < 1e-10);
                hits += usize::from(signs.iter().all(|&v| v > 0.0));
            }
            if !on_edge {
                let material = inside(point, &pattern.outer) && pattern.holes.iter().all(|hole| !inside(point, hole));
                assert_eq!(hits, usize::from(material), "coverage at {point:?}");
                samples += 1;
            }
        }
    }
    assert!(samples > 1500);
    // Verify the actual native cloth surface boundary accepts the resulting mesh.
    TriangleSurface::new(
        mesh.points.iter().map(|p| Vec3::new(p[0] as f32, p[1] as f32, 0.0)).collect(),
        mesh.triangles.clone(),
        0.01,
    )
    .unwrap();
}

#[test]
fn concave_l_shape_and_u_neckline_cover_only_authored_material() {
    for (outer, expected_area) in [
        (vec![[0.0, 0.0], [3.0, 0.0], [3.0, 1.0], [1.0, 1.0], [1.0, 3.0], [0.0, 3.0]], 5.0),
        (vec![[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [3.0, 4.0], [3.0, 2.0], [1.0, 2.0], [1.0, 4.0], [0.0, 4.0]], 12.0),
    ] {
        for reverse in [false, true] {
            let mut pattern = Pattern2D { outer: outer.clone(), holes: vec![] };
            if reverse {
                pattern.outer.reverse();
            }
            let mesh = pattern.triangulate(TriangulationOptions::default()).unwrap();
            assert_eq!(mesh.triangles.len(), pattern.outer.len() - 2);
            verify_coverage(&pattern, &mesh, expected_area);
        }
    }
}

#[test]
fn two_holes_and_concave_neckline_with_armholes_preserve_all_constraints() {
    let shirt = Pattern2D {
        outer: vec![[0.0, 0.0], [6.0, 0.0], [6.0, 6.0], [4.0, 6.0], [4.0, 4.0], [2.0, 4.0], [2.0, 6.0], [0.0, 6.0]],
        holes: vec![rectangle(0.5, 2.0, 1.0, 2.0), rectangle(4.5, 2.0, 1.0, 2.0)],
    };
    for (pattern, expected_area) in [(two_holes(), 40.0), (shirt, 28.0)] {
        for order in [false, true] {
            for winding in [false, true] {
                let mut pattern = pattern.clone();
                if order {
                    pattern.holes.reverse();
                }
                if winding {
                    pattern.outer.reverse();
                    for hole in &mut pattern.holes {
                        hole.reverse();
                    }
                }
                let mesh = pattern.triangulate(TriangulationOptions::default()).unwrap();
                assert_eq!(mesh.triangles.len(), mesh.points.len() + 2 * pattern.holes.len() - 2);
                verify_coverage(&pattern, &mesh, expected_area);
                assert_eq!(pattern.triangulate(TriangulationOptions::default()).unwrap(), mesh);
            }
        }
    }
}

#[test]
fn conforming_refinement_bounds_every_edge_and_keeps_stable_seam_controls() {
    let pattern = two_holes();
    let options = TriangulationOptions { max_edge_length: Some(1.25), ..Default::default() };
    let mesh = pattern.triangulate(options).unwrap();
    verify_coverage(&pattern, &mesh, 40.0);
    assert!(mesh.points.len() > 12 && mesh.points.len() <= 256);
    assert!(mesh.boundary_loops.iter().all(|contour| contour.len() > 4));
    for triangle in &mesh.triangles {
        for i in 0..3 {
            let a = mesh.points[triangle[i] as usize];
            let b = mesh.points[triangle[(i + 1) % 3] as usize];
            assert!((a[0] - b[0]).hypot(a[1] - b[1]) <= 1.25);
        }
    }
    // Trace the explicit refined seam chain between every pair of input controls.
    for (loop_index, original) in std::iter::once(&pattern.outer).chain(&pattern.holes).enumerate() {
        let boundary = &mesh.boundary_loops[loop_index];
        let controls = &mesh.control_vertices[loop_index];
        let forward = (area(original) > 0.0) == (loop_index == 0);
        for i in 0..controls.len() {
            let a = mesh.points[controls[i] as usize];
            let b = mesh.points[controls[(i + 1) % controls.len()] as usize];
            let mut cursor = boundary.iter().position(|&v| v == controls[i]).unwrap();
            let end = controls[(i + 1) % controls.len()];
            let mut steps = 0;
            loop {
                cursor = if forward {
                    (cursor + 1) % boundary.len()
                } else {
                    (cursor + boundary.len() - 1) % boundary.len()
                };
                steps += 1;
                let p = mesh.points[boundary[cursor] as usize];
                assert!(cross(a, b, p).abs() < 1e-12);
                assert!(p[0] >= a[0].min(b[0]) && p[0] <= a[0].max(b[0]));
                assert!(p[1] >= a[1].min(b[1]) && p[1] <= a[1].max(b[1]));
                if boundary[cursor] == end {
                    break;
                }
                assert!(steps < boundary.len());
            }
        }
    }
    assert_eq!(pattern.triangulate(options).unwrap(), mesh);
    assert_eq!(pattern.triangulate(TriangulationOptions { max_vertices: mesh.points.len(), ..options }).unwrap(), mesh);
    assert!(pattern
        .triangulate(TriangulationOptions { max_vertices: mesh.points.len() - 1, ..options })
        .unwrap_err()
        .contains("vertex budget"));
    eprintln!(
        "two-hole pattern: {} vertices, {} triangles, {} work",
        mesh.points.len(),
        mesh.triangles.len(),
        mesh.work
    );
}

#[test]
fn work_budget_is_exact_deterministic_and_failure_cannot_modify_input() {
    let pattern = two_holes();
    let original = pattern.clone();
    let options = TriangulationOptions { max_edge_length: Some(1.5), ..Default::default() };
    let mesh = pattern.triangulate(options).unwrap();
    assert_eq!(pattern.triangulate(TriangulationOptions { max_work: mesh.work, ..options }).unwrap(), mesh);
    for max_work in [0, 1, 40, mesh.work / 2, mesh.work - 1] {
        assert!(pattern.triangulate(TriangulationOptions { max_work, ..options }).unwrap_err().contains("work budget"));
        assert_eq!(pattern, original);
    }
    assert_eq!(pattern.triangulate(options).unwrap(), mesh);
}

#[test]
fn rejects_self_intersections_overlaps_touches_nested_holes_and_too_close_features() {
    let outer = rectangle(0.0, 0.0, 8.0, 6.0);
    let cases = [
        Pattern2D { outer: vec![[0.0, 0.0], [2.0, 2.0], [0.0, 2.0], [2.0, 0.0]], holes: vec![] },
        Pattern2D { outer: vec![[0.0, 0.0], [2.0, 0.0], [1.0, 0.0]], holes: vec![] },
        Pattern2D { outer: vec![[0.0, 0.0], [1e-14, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![] },
        Pattern2D {
            outer: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [1.0, 1.0], [0.0, 2.0], [1.0, 1.0]],
            holes: vec![],
        },
        Pattern2D { outer: outer.clone(), holes: vec![rectangle(9.0, 1.0, 1.0, 1.0)] },
        Pattern2D { outer: outer.clone(), holes: vec![rectangle(7.0, 1.0, 2.0, 1.0)] },
        Pattern2D { outer: outer.clone(), holes: vec![rectangle(0.0, 1.0, 1.0, 1.0)] },
        Pattern2D { outer: outer.clone(), holes: vec![rectangle(1e-14, 1.0, 1.0, 1.0)] },
        Pattern2D { outer: outer.clone(), holes: vec![rectangle(1.0, 1.0, 2.0, 2.0), rectangle(2.0, 2.0, 2.0, 2.0)] },
        Pattern2D { outer: outer.clone(), holes: vec![rectangle(1.0, 1.0, 2.0, 2.0), rectangle(3.0, 1.0, 2.0, 2.0)] },
        Pattern2D { outer: outer.clone(), holes: vec![rectangle(1.0, 1.0, 4.0, 4.0), rectangle(2.0, 2.0, 1.0, 1.0)] },
    ];
    for (index, pattern) in cases.into_iter().enumerate() {
        assert!(pattern.triangulate(TriangulationOptions::default()).is_err(), "invalid case {index}");
    }
    let mut duplicate_closure = Pattern2D { outer, holes: vec![] };
    duplicate_closure.outer.push(duplicate_closure.outer[0]);
    assert!(duplicate_closure.triangulate(TriangulationOptions::default()).is_err());
}

#[test]
fn straight_boundary_controls_and_finite_extreme_scales_are_preserved() {
    let original = vec![[0.0, 0.0], [1.0, 0.0], [2.0, 0.0], [2.0, 2.0], [1.0, 2.0], [0.0, 2.0]];
    let pattern = Pattern2D { outer: original.clone(), holes: vec![] };
    let mesh = pattern.triangulate(TriangulationOptions::default()).unwrap();
    verify_coverage(&pattern, &mesh, 4.0);
    for scale in [1e-150, 1.0, 1e150] {
        let pattern =
            Pattern2D { outer: original.iter().map(|p| [p[0] * scale, p[1] * scale]).collect(), holes: vec![] };
        let mesh = pattern.triangulate(TriangulationOptions::default()).unwrap();
        assert_eq!(mesh.points, pattern.outer);
        assert_eq!(mesh.triangles.len(), 4);
        assert!(mesh.points.iter().flatten().all(|v| v.is_finite()));
    }
}

#[test]
fn rejects_nonfinite_controls_invalid_options_and_insufficient_vertex_budget() {
    let pattern = Pattern2D { outer: rectangle(0.0, 0.0, 1.0, 1.0), holes: vec![] };
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        for axis in 0..2 {
            let mut bad = pattern.clone();
            bad.outer[1][axis] = invalid;
            assert!(bad.triangulate(TriangulationOptions::default()).unwrap_err().contains("finite"));
        }
    }
    for length in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(pattern
            .triangulate(TriangulationOptions { max_edge_length: Some(length), ..Default::default() })
            .is_err());
    }
    for max_vertices in [0, 2, 3, MAX_PATTERN_VERTICES + 1] {
        assert!(pattern.triangulate(TriangulationOptions { max_vertices, ..Default::default() }).is_err());
    }
    for outer in [vec![], vec![[0.0, 0.0]], vec![[0.0, 0.0], [1.0, 0.0]]] {
        assert!(Pattern2D { outer, holes: vec![] }.triangulate(TriangulationOptions::default()).is_err());
    }
    assert!(Pattern2D { outer: pattern.outer, holes: vec![vec![]] }
        .triangulate(TriangulationOptions::default())
        .is_err());
}

#[test]
fn multiple_curved_holes_bridge_deterministically_across_input_start_points() {
    for seed in 0..12 {
        let mut holes = Vec::new();
        for row in 0..2 {
            for column in 0..3 {
                let mut hole = Vec::new();
                for vertex in 0..8 {
                    let angle = (vertex + seed) as f64 * std::f64::consts::TAU / 8.0;
                    hole.push([2.0 + column as f64 * 3.0 + angle.cos() * 0.7, 2.0 + row as f64 * 4.0 + angle.sin()]);
                }
                if (seed + row + column) % 2 == 0 {
                    hole.reverse();
                }
                holes.push(hole);
            }
        }
        holes.rotate_left(seed % 6);
        let mut pattern = Pattern2D { outer: rectangle(0.0, 0.0, 10.0, 8.0), holes };
        pattern.outer.rotate_left(seed % 4);
        let mesh = pattern.triangulate(TriangulationOptions::default()).unwrap();
        let expected_area = 80.0 - pattern.holes.iter().map(|hole| area(hole).abs()).sum::<f64>();
        verify_coverage(&pattern, &mesh, expected_area);
        assert_eq!(pattern.triangulate(TriangulationOptions::default()).unwrap(), mesh);
    }
}

#[test]
fn refinement_checks_physical_lengths_and_rejects_unrepresentable_midpoint_triangles() {
    for (scale, translation) in [(1e-150, 0.0), (1.0, 0.0), (1e150, 0.0), (1.0, 1e12)] {
        let pattern = Pattern2D { outer: rectangle(translation, translation, scale, scale), holes: vec![] };
        let target = 0.3 * scale;
        let mesh =
            pattern.triangulate(TriangulationOptions { max_edge_length: Some(target), ..Default::default() }).unwrap();
        for triangle in mesh.triangles {
            for i in 0..3 {
                let a = mesh.points[triangle[i] as usize];
                let b = mesh.points[triangle[(i + 1) % 3] as usize];
                assert!((a[0] - b[0]).hypot(a[1] - b[1]) <= target);
            }
        }
    }
    let pattern = Pattern2D { outer: vec![[1e16, 1e16], [1e16 + 8.0, 1e16], [1e16, 1e16 + 2.0]], holes: vec![] };
    assert!(pattern
        .triangulate(TriangulationOptions { max_edge_length: Some(4.0), ..Default::default() })
        .unwrap_err()
        .contains("precision"));
}
