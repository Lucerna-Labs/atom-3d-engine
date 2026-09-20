use mm3e_kit::{surface::TriangleSurface, Vec3};

fn separated_triangles(count: usize) -> TriangleSurface {
    let mut vertices = Vec::with_capacity(count * 3);
    let mut triangles = Vec::with_capacity(count);
    for index in 0..count {
        let x = index as f32 * 4.0;
        let start = vertices.len() as u32;
        vertices.extend([Vec3::new(x, 0.0, 0.0), Vec3::new(x + 1.0, 0.0, 0.0), Vec3::new(x, 1.0, 0.0)]);
        triangles.push([start, start + 1, start + 2]);
    }
    TriangleSurface::new(vertices, triangles, 0.01).unwrap()
}

fn individual_triangles(surface: &TriangleSurface) -> Vec<TriangleSurface> {
    surface
        .triangles()
        .iter()
        .map(|triangle| {
            TriangleSurface::new(
                triangle.iter().map(|&index| surface.vertices()[index as usize]).collect(),
                vec![[0, 1, 2]],
                surface.half_thickness(),
            )
            .unwrap()
        })
        .collect()
}

#[test]
fn bounded_distance_matches_existing_and_brute_force_individual_triangles_bitwise() {
    // A bent mesh exercises interior projections, border/vertex minima, traversal
    // reordering and ties at shared vertices, including tiny and huge coordinates.
    for scale in [1e-15, 1.0, 1e15] {
        let side = 8u32;
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        for y in 0..=side {
            for x in 0..=side {
                let (x, y) = (x as f32 * 0.125, y as f32 * 0.125);
                vertices.push(Vec3::new(x, y, 0.15 * (6.0 * x).sin() + 0.1 * (7.0 * y).cos()).scale(scale));
            }
        }
        for y in 0..side {
            for x in 0..side {
                let a = y * (side + 1) + x;
                triangles.extend([[a, a + 1, a + side + 2], [a, a + side + 2, a + side + 1]]);
            }
        }
        let surface = TriangleSurface::new(vertices, triangles, 0.004 * scale).unwrap();
        let individual = individual_triangles(&surface);
        let mut points = surface.vertices().to_vec();
        for index in 0..120 {
            points.push(
                Vec3::new(
                    ((index * 37) % 137) as f32 / 60.0 - 0.5,
                    ((index * 53) % 139) as f32 / 70.0 - 0.5,
                    ((index * 71) % 149) as f32 / 40.0 - 2.0,
                )
                .scale(scale),
            );
        }
        for point in points {
            let result = surface.distance_bounded(point, usize::MAX).unwrap();
            let brute = individual.iter().map(|triangle| triangle.distance(point)).fold(f32::INFINITY, f32::min);
            assert_eq!(result.distance.to_bits(), surface.distance(point).to_bits(), "query: {point:?}");
            assert_eq!(result.distance.to_bits(), brute.to_bits(), "brute force query: {point:?}");
            assert_eq!(surface.distance_bounded(point, result.work).unwrap(), result);
            assert!(surface.distance_bounded(point, result.work - 1).is_err());
        }
    }
}

#[test]
fn counts_root_children_ordering_repeated_bounds_and_triangle_tests() {
    let point = Vec3::new(0.25, 0.25, 0.25);
    // One root check plus all triangles for a leaf. For a split root, count
    // root + two ordering tests + two popped child tests + near-leaf triangles.
    for (triangles, expected) in [(1, 2), (8, 9), (9, 9), (16, 13)] {
        let surface = separated_triangles(triangles);
        let result = surface.distance_bounded(point, usize::MAX).unwrap();
        assert_eq!(result.work, expected, "{triangles} triangles");
        assert_eq!(surface.distance_bounded(point, expected).unwrap(), result);
        // Failures before the root, either ordering child, or any leaf triangle
        // must all remain errors, even after an exact candidate has been found.
        for cap in 0..expected {
            assert!(surface.distance_bounded(point, cap).is_err(), "{triangles} triangles, cap {cap}");
        }
        assert_eq!(surface.distance_bounded(point, usize::MAX).unwrap(), result);
    }
}

#[test]
fn pruned_queries_count_far_less_work_than_visiting_every_triangle() {
    let surface = separated_triangles(4096);
    // Nine internal levels: root + (two ordering tests and two popped children)
    // per level + eight triangles in the sole unpruned leaf = 45 units.
    for index in [0, 17, 255, 2048, 4095] {
        let point = Vec3::new(index as f32 * 4.0 + 0.25, 0.25, -0.25);
        let result = surface.distance_bounded(point, usize::MAX).unwrap();
        assert_eq!(result.work, 45);
        assert!(result.work * 80 < surface.triangles().len());
        assert_eq!(result.distance.to_bits(), surface.distance(point).to_bits());
        for _ in 0..3 {
            assert_eq!(surface.distance_bounded(point, 45).unwrap(), result);
        }
        assert!(surface.distance_bounded(point, 44).is_err());
    }
}

#[test]
fn fully_visited_tree_counts_all_tests_and_preserves_winding_tie_normal() {
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    for index in 0..16 {
        let start = vertices.len() as u32;
        vertices.extend([Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)]);
        // Geometrically coincident disconnected faces: no AABB can be pruned.
        // The first face's positive winding wins the exact medial-surface tie.
        triangles.push(if index == 0 { [start, start + 1, start + 2] } else { [start, start + 2, start + 1] });
    }
    let surface = TriangleSurface::new(vertices, triangles, 0.01).unwrap();
    let point = Vec3::new(0.25, 0.25, 0.0);
    let result = surface.distance_bounded(point, 21).unwrap();
    assert_eq!(result.work, 3 + 2 + 16);
    assert_eq!(result.distance.to_bits(), (-0.01f32).to_bits());
    assert!(surface.distance_bounded(point, 20).is_err());
    let normal = surface.normal(point);
    assert_eq!(
        [normal.x.to_bits(), normal.y.to_bits(), normal.z.to_bits()],
        [0.0f32.to_bits(), 0.0f32.to_bits(), 1.0f32.to_bits()]
    );
}

#[test]
fn rejects_nonfinite_queries_and_zero_budget_without_changing_legacy_behavior() {
    let surface = separated_triangles(1);
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        for point in [Vec3::new(invalid, 0.0, 0.0), Vec3::new(0.0, invalid, 0.0), Vec3::new(0.0, 0.0, invalid)] {
            for cap in [0, 2, usize::MAX] {
                assert!(surface.distance_bounded(point, cap).unwrap_err().contains("finite"));
            }
            assert_eq!(surface.distance(point), f32::INFINITY);
            assert_eq!(surface.normal(point), Vec3::ZERO);
        }
    }
    assert!(surface.distance_bounded(Vec3::ZERO, 0).unwrap_err().contains("budget"));
    let valid = surface.distance_bounded(Vec3::ZERO, 2).unwrap();
    assert_eq!(valid.work, 2);
    assert_eq!(valid.distance.to_bits(), surface.distance(Vec3::ZERO).to_bits());
}
