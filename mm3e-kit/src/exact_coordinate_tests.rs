use super::*;
fn work() -> WorkBudget {
    WorkBudget {
        used: 0,
        structural: 0,
        extra_field_evaluations: 0,
        field_cost_per_callback: 1,
        limit: 64_000_000,
        exact_support_reductions: 0,
    }
}
fn rational(coordinate: &ExactCoordinate) -> crate::exact_geometry::Point {
    crate::exact_geometry::Point::from_expansions(
        [&coordinate.numerator, &[], &[]],
        &coordinate.denominator,
        10_000_000,
    )
    .unwrap()
    .point
}
fn reference(coordinate: &ExactCoordinate, candidate: f64) -> i8 {
    let a = rational(coordinate);
    let b = crate::exact_geometry::Point::from_expansions([&[candidate], &[], &[]], &[1.], 10_000_000).unwrap().point;
    crate::exact_geometry::compare_axis(&a, &b, 0, 10_000_000).unwrap().sign
}
fn coordinate(n: f64, d: f64) -> ExactCoordinate {
    ExactCoordinate { numerator: vec![n], denominator: vec![d], exact_f32: std::cell::Cell::new(None) }
}
#[test]
fn single_product_signs_match_independent_integer_ratios_at_rounding_cell_boundaries() {
    let mut state = 0x503c_26be_7831_91abu64;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        state
    };
    let mut residual_cases = 0;
    let mut count = 0;
    for _ in 0..512 {
        let d = f64::from_bits(((823 + next() % 401) << 52) | (next() & ((1 << 52) - 1)));
        let c = f64::from_bits((next() & 1) << 63 | ((823 + next() % 401) << 52) | (next() & ((1 << 52) - 1)));
        let high = d * c;
        assert!(high.is_finite());
        residual_cases += usize::from(d.mul_add(c, -high) != 0.);
        for n in [high.next_down(), high, high.next_up()] {
            let coordinate = coordinate(n, d);
            let mut budget = work();
            assert_eq!(coordinate.single_product_compare(c, &mut budget).unwrap(), Some(reference(&coordinate, c)));
            assert_eq!(budget.used, 16);
            count += 1;
        }
    }
    assert!(residual_cases > 500);
    eprintln!("SINGLE_PRODUCT_ORACLE comparisons={count} nonzero_fma_residual_cases={residual_cases}");
    for (n, d, c) in [(1., 2f64.powi(-500), 2f64.powi(-600)), (1., 2f64.powi(800), 2f64.powi(400))] {
        assert_eq!(coordinate(n, d).single_product_compare(c, &mut work()).unwrap(), None);
    }
}
fn source(endpoint_x: f64) -> SourcePoint {
    SourcePoint {
        point: [0.4, 3., -2.],
        nodes: [0, 1, u32::MAX, u32::MAX],
        weights: [0.6, 0.4, 0., 0.],
        affine: Some(std::sync::Arc::new(AffineSource {
            exact_basis: Default::default(),
            tetra: [0, 1, 2, 3],
            points: [[0., 3., -2.], [endpoint_x, 3., -2.], [0., 4., -2.], [0., 3., -1.]],
            basis: [7, u64::MAX, u64::MAX],
            planes: vec![(7, [-f64::from(0.4f32), f64::from(0.6f32), 0., 0.])],
        })),
    }
}
#[test]
fn constant_support_components_and_cached_equalities_are_exact_not_proximity_tests() {
    let coordinates = ExactCoordinate::all_from_source(&source(1.), &mut work()).unwrap();
    assert_eq!(coordinates[0].exact_f32.get(), None);
    assert_eq!(coordinates[1].exact_f32.get(), Some(3.));
    assert_eq!(coordinates[2].exact_f32.get(), Some(-2.));
    for coordinate in coordinates {
        let hint = coordinate.approximate();
        let (interval, _) = coordinate.interval(hint, &mut work()).unwrap();
        for value in
            [interval[0].next_down(), interval[0], *interval.last().unwrap(), interval.last().unwrap().next_up()]
        {
            assert_eq!(coordinate.compare(value, &mut work()).unwrap(), reference(&coordinate, f64::from(value)));
        }
        let exact = coordinate.exact_f32.get();
        if let Some(value) = exact {
            assert_eq!(reference(&coordinate, f64::from(value)), 0);
            assert_eq!(coordinate.tight_interval(&mut work()).unwrap(), Some([f64::from(value); 2]));
        }
    }
    // A tiny but genuinely varying coordinate is never declared constant.
    let near = source(f64::from(f32::from_bits(1)));
    let coordinates = ExactCoordinate::all_from_source(&near, &mut work()).unwrap();
    assert!(coordinates[0].exact_f32.get().is_none());
    let (interval, _) = coordinates[0].interval(coordinates[0].approximate(), &mut work()).unwrap();
    assert_eq!(interval.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), [0, 1]);
    assert!(coordinates[0].exact_f32.get().is_none());
}
#[test]
fn single_product_halfway_nearest_and_cached_comparison_budgets_preserve_exact_results() {
    for (n, d, hint) in [
        (16777217., 16777216., 1.0000000596046448),
        (-16777217., 16777216., -1.0000000596046448),
        (1., 2f64.powi(150), 2f64.powi(-150)),
    ] {
        let c = coordinate(n, d);
        let (interval, _) = c.interval(hint, &mut work()).unwrap();
        assert_eq!(interval.len(), 2);
        let chosen = c.nearest(&interval, &mut work()).unwrap();
        assert_eq!(chosen.to_bits() & 1, 0);
        let midpoint = (f64::from(interval[0]) + f64::from(interval[1])) * 0.5;
        assert_eq!(reference(&c, midpoint), 0);
    }
    let c = coordinate(3., 1.);
    let mut first = work();
    assert_eq!(c.compare(3., &mut first).unwrap(), 0);
    assert_eq!(first.used, 16);
    let mut cached = work();
    cached.limit = 1;
    assert_eq!(c.compare(3., &mut cached).unwrap(), 0);
    assert_eq!(cached.used, 1);
    let mut exhausted = work();
    exhausted.limit = 0;
    assert!(c.compare(3., &mut exhausted).unwrap_err().contains("bounded work"));
}

#[test]
fn source_domain_index_is_exactly_the_original_plane_candidates_with_necessary_node_filter() {
    use std::collections::{BTreeMap, BTreeSet};
    let mut index = BTreeMap::<(u64, [u32; 4]), Vec<u32>>::new();
    let mut records = Vec::new();
    for plane in [3u64, 7] {
        for mask in 1u32..256 {
            if mask.count_ones() > 4 {
                continue;
            }
            let mut key = [u32::MAX; 4];
            let mut at = 0;
            for node in 0..8 {
                if mask & (1 << node) != 0 {
                    key[at] = node;
                    at += 1;
                }
            }
            let id = records.len() as u32;
            records.push((plane, key, id));
            index.entry((plane, key)).or_default().push(id);
        }
    }
    let mut comparisons = 0;
    for mask in 1u32..256 {
        if mask.count_ones() > 4 {
            continue;
        }
        let domain: Vec<u32> = (0..8).filter(|i| mask & (1 << i) != 0).collect();
        for plane in [3u64, 7, 11] {
            let expected: Vec<u32> = records
                .iter()
                .filter(|(p, n, _)| *p == plane && n.iter().all(|n| *n == u32::MAX || domain.contains(n)))
                .map(|r| r.2)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let mut budget = work();
            let actual = source_domain_candidates(plane, &domain, &index, &mut budget).unwrap();
            assert_eq!(actual, expected);
            let spent = budget.used;
            let mut exact = work();
            exact.limit = spent;
            assert_eq!(source_domain_candidates(plane, &domain, &index, &mut exact).unwrap(), actual);
            let mut short = work();
            short.limit = spent - 1;
            assert!(source_domain_candidates(plane, &domain, &index, &mut short).unwrap_err().contains("bounded work"));
            comparisons += 1;
        }
    }
    assert_eq!(comparisons, 486);
    assert!(source_domain_candidates(3, &[], &index, &mut work()).is_err());
    assert!(source_domain_candidates(3, &[0, 1, 2, 3, 4], &index, &mut work()).is_err());
    eprintln!("SOURCE_DOMAIN_INDEX_ORACLE plane_domain_queries={comparisons}");
}
