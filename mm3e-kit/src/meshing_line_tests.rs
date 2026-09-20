use super::*;
use std::sync::Arc;

fn work(limit: usize) -> WorkBudget {
    WorkBudget {
        used: 0,
        structural: 0,
        extra_field_evaluations: 0,
        field_cost_per_callback: 1,
        limit,
        exact_support_reductions: 0,
    }
}

fn source(node: u32, point: [f64; 3]) -> SourcePoint {
    let point = point.map(|value| value as f32 as f64);
    SourcePoint {
        point,
        nodes: [node, u32::MAX, u32::MAX, u32::MAX],
        weights: [1., 0., 0., 0.],
        affine: Some(Arc::new(AffineSource {
            exact_basis: Default::default(),
            tetra: [node, node + 1, node + 2, node + 3],
            points: [point, [0.; 3], [0.; 3], [0.; 3]],
            basis: [u64::MAX; 3],
            planes: Vec::new(),
        })),
    }
}

#[test]
fn singleton_line_accepts_cross_tetra_axis_and_diagonal_segments() {
    let axis = [source(10, [0., 2., -3.]), source(11, [1., 2., -3.]), source(12, [2., 2., -3.])];
    let mut budget = work(1_000);
    assert!(singleton_exact_line([&axis[0], &axis[1], &axis[2]], &mut budget).unwrap());
    let diagonal = [source(20, [0., 0., 0.]), source(21, [1., 1., 1.]), source(22, [2., 2., 2.])];
    assert!(singleton_exact_line([&diagonal[0], &diagonal[1], &diagonal[2]], &mut work(1_000)).unwrap());
}

#[test]
fn singleton_line_rejects_noncollinear_and_nonbetween_points_with_exact_residuals() {
    let off_line = [
        source(30, [0., 0., 0.]),
        source(31, [1., 1., 1.]),
        source(32, [2., 2., f64::from(f32::from_bits(0x4000_0001))]),
    ];
    assert!(!singleton_exact_line([&off_line[0], &off_line[1], &off_line[2]], &mut work(10_000)).unwrap());
    let endpoint = [source(40, [0., 0., 0.]), source(41, [2., 2., 2.]), source(42, [1., 1., 1.])];
    assert!(!singleton_exact_line([&endpoint[0], &endpoint[1], &endpoint[2]], &mut work(10_000)).unwrap());
    let outside = [source(50, [0., 0., 0.]), source(51, [3., 3., 3.]), source(52, [1., 1., 1.])];
    assert!(singleton_exact_line([&outside[0], &outside[2], &outside[1]], &mut work(10_000)).unwrap());
    let repeated = [source(60, [1., 1., 1.]), source(61, [1., 1., 1.]), source(62, [1., 1., 1.])];
    assert!(!singleton_exact_line([&repeated[0], &repeated[1], &repeated[2]], &mut work(10_000)).unwrap());
}

#[test]
fn singleton_line_requires_three_singleton_sources_and_charges_exact_budget() {
    let mut multi = source(70, [0., 0., 0.]);
    multi.nodes[1] = 71;
    let middle = source(72, [1., 0., 0.]);
    let end = source(73, [2., 0., 0.]);
    let mut budget = work(1_000);
    assert!(!singleton_exact_line([&multi, &middle, &end], &mut budget).unwrap());

    let line = [source(80, [0., 0., 0.]), source(81, [1., 0., 0.]), source(82, [2., 0., 0.])];
    let mut full = work(10_000);
    assert!(singleton_exact_line([&line[0], &line[1], &line[2]], &mut full).unwrap());
    let used = full.used;
    let mut exact = work(used);
    assert!(singleton_exact_line([&line[0], &line[1], &line[2]], &mut exact).unwrap());
    assert_eq!(exact.used, used);
    assert!(singleton_exact_line([&line[0], &line[1], &line[2]], &mut work(used - 1)).is_err());
}
