use super::*;

fn budget(limit: usize) -> WorkBudget {
    WorkBudget {
        used: 0,
        structural: 0,
        extra_field_evaluations: 0,
        field_cost_per_callback: 1,
        limit,
        exact_support_reductions: 0,
    }
}

fn source(mask: u8, rows: &[[f64; 4]]) -> SourcePoint {
    let tetra = [103, 101, 100, 102];
    let mut active: Vec<_> = (0..4).filter(|&i| mask & (1 << i) != 0).map(|i| tetra[i]).collect();
    active.sort_unstable();
    let mut nodes = [u32::MAX; 4];
    nodes[..active.len()].copy_from_slice(&active);
    let mut basis = [u64::MAX; 3];
    let planes = rows
        .iter()
        .enumerate()
        .map(|(i, &row)| {
            basis[i] = 1000 + i as u64;
            (basis[i], row)
        })
        .collect();
    SourcePoint {
        point: [0.; 3],
        nodes,
        weights: [0.; 4],
        affine: Some(std::sync::Arc::new(AffineSource {
            tetra,
            points: [[0.; 3]; 4],
            basis,
            planes,
            exact_basis: Default::default(),
        })),
    }
}

#[test]
fn independent_fraction_basis_signs_cold_warm_and_exact_budgets() {
    let mut cases = 0;
    let mut unequal_queries = 0;
    for (line, text) in include_str!("../tests/fixtures/source_basis/oracle.csv").lines().enumerate() {
        let columns: Vec<_> = text.split(',').collect();
        assert_eq!(columns.len(), 20);
        let rank: usize = columns[1].parse().unwrap();
        let mask: u8 = columns[2].parse().unwrap();
        let scalar = |index: usize| f64::from(f32::from_bits(columns[index].parse().unwrap()));
        let rows: Vec<[f64; 4]> = (0..rank - 1).map(|r| std::array::from_fn(|c| scalar(4 + r * 4 + c))).collect();
        let query: [f64; 4] = std::array::from_fn(|c| scalar(16 + c));
        let original = source(mask, &rows);
        let nodes = original.affine.as_ref().unwrap().tetra;
        let mut full = budget(1_000_000);
        let reference = source_plane_sign_uncached(&original, &nodes, &query, &mut full);
        let mut cold = budget(1_000_000);
        let actual = source_plane_sign(&original, &nodes, &query, &mut cold);
        if columns[3] == "singular" {
            assert!(reference.is_err(), "oracle singular line {line}");
            assert!(actual.is_err(), "cached singular line {line}");
            assert!(original.affine.as_ref().unwrap().exact_basis.get().is_none());
            continue;
        }
        let expected: i8 = columns[3].parse().unwrap();
        assert_eq!(reference.unwrap(), expected, "old Fraction line {line}");
        assert_eq!(actual.unwrap(), expected, "cold Fraction line {line}");
        unequal_queries += usize::from(expected != 0);
        let mut replay = budget(cold.used);
        assert_eq!(source_plane_sign(&source(mask, &rows), &nodes, &query, &mut replay).unwrap(), expected);
        assert_eq!(replay.used, cold.used);
        let mut short = budget(cold.used - 1);
        assert!(
            source_plane_sign(&source(mask, &rows), &nodes, &query, &mut short).is_err(),
            "cold one-less line {line}"
        );
        assert!(short.used <= short.limit);
        let mut warm = budget(1_000_000);
        assert_eq!(source_plane_sign(&original, &nodes, &query, &mut warm).unwrap(), expected);
        let mut replay = budget(warm.used);
        assert_eq!(source_plane_sign(&original, &nodes, &query, &mut replay).unwrap(), expected);
        assert_eq!(replay.used, warm.used);
        let mut short = budget(warm.used - 1);
        assert!(source_plane_sign(&original, &nodes, &query, &mut short).is_err(), "warm one-less line {line}");
        assert!(short.used <= short.limit);
        cases += 1;
    }
    eprintln!("Independent Fraction: {cases} nonsingular cases; {unequal_queries} nonzero fallback signs; cold and warm exact/one-less budgets");
    assert_eq!(cases, 10120);
    assert!(unequal_queries > 2000);
}

#[test]
fn non_f32_cross_product_rounding_and_zero_pivot_controls() {
    let rows = [[1., -1., 0., 0.], [0., 1., -1., 0.]];
    let original = source(0b0111, &rows);
    let nodes = original.affine.as_ref().unwrap().tetra;
    let mut work = budget(1_000_000);
    assert_eq!(source_plane_sign(&original, &nodes, &[0., 1., 0., 0.], &mut work).unwrap(), 1);
    let epsilon = 2f64.powi(-52);
    let b = [1., 1. + epsilon, 0., 0.];
    let q = [1. + epsilon, 1. + 2. * epsilon, 0., 0.];
    assert_eq!(b[0] * q[1], b[1] * q[0], "rounded f64 cross products conceal epsilon squared");
    assert_ne!(b[1].mul_add(q[0], -(b[0] * q[1])), 0.);
    let invalid_basis = source(0b0111, &[b, [0., 0., 1., 0.]]);
    assert!(source_plane_sign(&invalid_basis, &nodes, &q, &mut budget(1_000_000)).is_err());
    assert!(source_plane_sign(&original, &nodes, &q, &mut budget(1_000_000)).is_err());
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(source_plane_sign(&original, &nodes, &[bad, 0., 0., 0.], &mut budget(1_000_000)).is_err());
    }
}

#[test]
fn deep_clone_mutation_cannot_reuse_an_old_basis_equation() {
    let rows = [[1., -1., 0., 0.], [0., 1., -1., 0.]];
    let original = source(0b0111, &rows);
    let nodes = original.affine.as_ref().unwrap().tetra;
    assert_eq!(source_plane_sign(&original, &nodes, &rows[0], &mut budget(1_000_000)).unwrap(), 0);
    let mut copied = original.affine.as_ref().unwrap().as_ref().clone();
    assert!(copied.exact_basis.get().is_none());
    copied.planes[0].1 = [1., -2., 0., 0.];
    let edited = SourcePoint { affine: Some(std::sync::Arc::new(copied)), ..original.clone() };
    assert_eq!(source_plane_sign(&edited, &nodes, &rows[0], &mut budget(1_000_000)).unwrap(), 1);
    assert_eq!(source_plane_sign(&original, &nodes, &rows[0], &mut budget(1_000_000)).unwrap(), 0);
}
