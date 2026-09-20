use mm3e_kit::exact_geometry::{compare_axis, orient2, orient3, Point};

const WORK: usize = 10_000_000;
fn point(rows: [&[f64]; 4]) -> Point {
    Point::from_expansions([rows[0], rows[1], rows[2]], rows[3], WORK).unwrap().point
}
fn xy(x: &[f64], y: &[f64]) -> Point {
    point([x, y, &[], &[1.]])
}

#[test]
fn every_fraction_oracle_sign_matches_including_actual_cofactors_and_all_binary_exponents() {
    let mut points = Vec::new();
    let (mut orientations, mut comparisons, mut fallbacks) = (0, 0, 0);
    let (mut maximum_point_work, mut maximum_predicate_work) = (0, 0);
    for line in include_str!("fixtures/exact_geometry/oracle.csv").lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<_> = line.split(',').collect();
        match fields[0] {
            "P" => {
                assert_eq!(fields.len(), 6);
                let rows: Vec<Vec<f64>> = fields[2..]
                    .iter()
                    .map(|s| {
                        s.split(';')
                            .filter(|s| !s.is_empty())
                            .map(|s| f64::from_bits(u64::from_str_radix(s, 16).unwrap()))
                            .collect()
                    })
                    .collect();
                let result = Point::from_expansions([&rows[0], &rows[1], &rows[2]], &rows[3], WORK)
                    .unwrap_or_else(|e| panic!("Fraction point {} rejected: {e:?}", fields[1]));
                maximum_point_work = maximum_point_work.max(result.work);
                points.push(result.point);
            }
            "O" => {
                assert_eq!(fields.len(), 8);
                let ids: Vec<usize> = fields[2..7].iter().map(|s| s.parse().unwrap()).collect();
                let expected: i8 = fields[7].parse().unwrap();
                let result = orient2([&points[ids[0]], &points[ids[1]], &points[ids[2]]], [ids[3], ids[4]], WORK)
                    .unwrap_or_else(|e| panic!("Fraction orientation {} rejected: {e:?}", fields[1]));
                assert_eq!(result.sign, expected, "Fraction orientation {}", fields[1]);
                orientations += 1;
                fallbacks += usize::from(result.integer_fallback);
                maximum_predicate_work = maximum_predicate_work.max(result.work);
            }
            "C" => {
                assert_eq!(fields.len(), 6);
                let ids: Vec<usize> = fields[2..5].iter().map(|s| s.parse().unwrap()).collect();
                let expected: i8 = fields[5].parse().unwrap();
                let result = compare_axis(&points[ids[0]], &points[ids[1]], ids[2], WORK)
                    .unwrap_or_else(|e| panic!("Fraction comparison {} rejected: {e:?}", fields[1]));
                assert_eq!(result.sign, expected, "Fraction comparison {}", fields[1]);
                comparisons += 1;
                fallbacks += usize::from(result.integer_fallback);
                maximum_predicate_work = maximum_predicate_work.max(result.work);
            }
            kind => panic!("unknown Fraction fixture record {kind:?}"),
        }
    }
    assert_eq!((points.len(), orientations, comparisons), (4731, 9553, 5041));
    assert!(fallbacks > 1000, "the fixture must exercise exact integer arithmetic, not just interval signs");
    eprintln!("FRACTION_ORACLE points={} orientations={orientations} comparisons={comparisons} integer_fallbacks={fallbacks} max_point_work={maximum_point_work} max_predicate_work={maximum_predicate_work}", points.len());
}

#[test]
fn a_genuine_subnormal_offset_keeps_its_sign_in_nearly_collinear_coordinates() {
    let tiny = f64::from_bits(1);
    let a = xy(&[1.], &[1.]);
    let b = xy(&[2.], &[2.]);
    let c = xy(&[3.], &[3., tiny]);
    let positive = orient2([&a, &b, &c], [0, 1], WORK).unwrap();
    assert_eq!(positive.sign, 1);
    assert!(positive.integer_fallback);
    assert_eq!(orient2([&b, &a, &c], [0, 1], WORK).unwrap().sign, -1);
    assert_eq!(orient2([&a, &b, &c], [1, 0], WORK).unwrap().sign, -1);
    let collinear = xy(&[3.], &[3.]);
    let zero = orient2([&a, &b, &collinear], [0, 1], WORK).unwrap();
    assert_eq!(zero.sign, 0);
    assert!(zero.integer_fallback);
    assert_eq!(compare_axis(&c, &collinear, 1, WORK).unwrap().sign, 1);
}

#[test]
fn cancelled_and_negative_homogeneous_denominators_preserve_the_exact_coordinate() {
    let tiny = f64::from_bits(1);
    let positive = point([&[tiny], &[], &[], &[1., tiny, -1.]]);
    let negative = point([&[-tiny], &[], &[], &[1., -tiny, -1.]]);
    let one = xy(&[1.], &[]);
    for p in [&positive, &negative] {
        assert_eq!(compare_axis(p, &one, 0, WORK).unwrap().sign, 0);
        assert_eq!(compare_axis(p, &one, 1, WORK).unwrap().sign, 0);
    }
    let huge = f64::MAX;
    let beyond_f64 = point([&[huge, huge], &[], &[], &[1.]]);
    let maximum = xy(&[huge], &[]);
    let result = compare_axis(&beyond_f64, &maximum, 0, WORK).unwrap();
    assert_eq!(result.sign, 1);
    assert!(result.integer_fallback);
}

#[test]
fn arc_clones_and_separately_reconstructed_equal_points_agree() {
    let a = point([&[1., f64::from_bits(1)], &[2.], &[-3.], &[-7.]]);
    let clone = a.clone();
    let same = point([&[-1., -f64::from_bits(1)], &[-2.], &[3.], &[7.]]);
    for axis in 0..3 {
        let cached = compare_axis(&a, &clone, axis, WORK).unwrap();
        assert_eq!(cached.sign, 0);
        assert!(!cached.integer_fallback);
        assert_eq!(compare_axis(&a, &same, axis, WORK).unwrap().sign, 0);
    }
    assert_eq!(orient2([&a, &clone, &same], [0, 1], WORK).unwrap().sign, 0);
}

#[test]
fn constructor_and_predicate_work_replay_exactly_and_one_less_fails() {
    let x = [2_f64.powi(900), 2_f64.powi(-900), -2_f64.powi(900), 3.];
    let result = Point::from_expansions([&x, &[2.], &[]], &[2., -1.], WORK).unwrap();
    let replay = Point::from_expansions([&x, &[2.], &[]], &[2., -1.], result.work).unwrap();
    assert_eq!(replay.work, result.work);
    assert_eq!(compare_axis(&result.point, &replay.point, 0, WORK).unwrap().sign, 0);
    let failure = Point::from_expansions([&x, &[2.], &[]], &[2., -1.], result.work - 1).unwrap_err();
    assert!(failure.message.contains("budget") && failure.work < result.work);
    let a = xy(&[1.], &[1.]);
    let b = xy(&[2.], &[2.]);
    let c = xy(&[3.], &[3., f64::from_bits(1)]);
    let comparison = compare_axis(&c, &xy(&[3.], &[3.]), 1, WORK).unwrap();
    let same_y = xy(&[3.], &[3.]);
    assert_eq!(compare_axis(&c, &same_y, 1, comparison.work).unwrap(), comparison);
    let failure = compare_axis(&c, &same_y, 1, comparison.work - 1).unwrap_err();
    assert!(failure.message.contains("budget") && failure.work < comparison.work);
    let orientation = orient2([&a, &b, &c], [0, 1], WORK).unwrap();
    assert_eq!(orient2([&a, &b, &c], [0, 1], orientation.work).unwrap(), orientation);
    let failure = orient2([&a, &b, &c], [0, 1], orientation.work - 1).unwrap_err();
    assert!(failure.message.contains("budget") && failure.work < orientation.work);
}

#[test]
fn malformed_inputs_and_retried_zero_denominators_reject_with_counted_work() {
    for nonfinite in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let e = Point::from_expansions([&[nonfinite], &[], &[]], &[1.], WORK).unwrap_err();
        assert!(e.message.contains("finite") && e.work > 0);
    }
    let excessive = vec![0.; 1024];
    assert!(Point::from_expansions([&excessive, &[], &[]], &[1.], WORK).unwrap_err().message.contains("term bound"));
    assert!(Point::from_expansions([&[], &[], &[]], &[], WORK).unwrap_err().message.contains("term bound"));
    let cancelled = [f64::MAX, f64::from_bits(1), -f64::MAX, -f64::from_bits(1)];
    let first = Point::from_expansions([&[1.], &[], &[]], &cancelled, WORK).unwrap_err();
    assert!(first.message.contains("denominator is zero") && first.work > 0);
    assert_eq!(Point::from_expansions([&[1.], &[], &[]], &cancelled, first.work).unwrap_err(), first);
    let allowance = first.work * 2 - 1;
    let second = Point::from_expansions([&[1.], &[], &[]], &cancelled, allowance - first.work).unwrap_err();
    assert!(second.message.contains("budget") && second.work > 0 && first.work + second.work <= allowance);
    let p = xy(&[1.], &[1.]);
    for axes in [[0, 0], [0, 3], [usize::MAX, 1]] {
        let e = orient2([&p, &p, &p], axes, WORK).unwrap_err();
        assert!(e.message.contains("axes are invalid") && e.work > 0);
        assert_eq!(orient2([&p, &p, &p], axes, e.work).unwrap_err(), e);
    }
    let e = compare_axis(&p, &p, 3, WORK).unwrap_err();
    assert!(e.message.contains("axis is invalid") && e.work > 0);
    assert_eq!(compare_axis(&p, &p, 3, e.work).unwrap_err(), e);
}

#[test]
fn orient3_matches_independent_fraction_determinants_rotations_scales_and_extreme_cancellation() {
    let mut points = Vec::new();
    let (mut queries, mut fallbacks, mut maximum_point_work, mut maximum_predicate_work) = (0, 0, 0, 0);
    let mut sign_counts = [0usize; 3];
    for line in include_str!("fixtures/exact_geometry_3d/oracle.csv").lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<_> = line.split(',').collect();
        match fields[0] {
            "P" => {
                assert_eq!(fields.len(), 6);
                let rows: Vec<Vec<f64>> = fields[2..]
                    .iter()
                    .map(|row| {
                        row.split(';')
                            .filter(|s| !s.is_empty())
                            .map(|s| f64::from_bits(u64::from_str_radix(s, 16).unwrap()))
                            .collect()
                    })
                    .collect();
                let result = Point::from_expansions([&rows[0], &rows[1], &rows[2]], &rows[3], WORK)
                    .unwrap_or_else(|e| panic!("Fraction 3D point {} rejected: {e:?}", fields[1]));
                maximum_point_work = maximum_point_work.max(result.work);
                points.push(result.point);
            }
            "O3" => {
                assert_eq!(fields.len(), 7);
                let ids: Vec<usize> = fields[2..6].iter().map(|s| s.parse().unwrap()).collect();
                let expected: i8 = fields[6].parse().unwrap();
                let result = orient3([&points[ids[0]], &points[ids[1]], &points[ids[2]], &points[ids[3]]], WORK)
                    .unwrap_or_else(|e| panic!("Fraction orient3 {} rejected: {e:?}", fields[1]));
                assert_eq!(result.sign, expected, "Fraction orient3 {}", fields[1]);
                queries += 1;
                sign_counts[(expected + 1) as usize] += 1;
                fallbacks += usize::from(result.integer_fallback);
                maximum_predicate_work = maximum_predicate_work.max(result.work);
            }
            kind => panic!("unknown 3D Fraction fixture record {kind:?}"),
        }
    }
    assert_eq!((points.len(), queries), (6890, 7971));
    assert_eq!(sign_counts, [3385, 1200, 3386]);
    assert!(fallbacks > 1000, "3D oracle must exercise exact integer determinants, not only interval signs");
    eprintln!("FRACTION_ORACLE_3D points={} orientations={queries} signs={sign_counts:?} integer_fallbacks={fallbacks} max_point_work={maximum_point_work} max_predicate_work={maximum_predicate_work}", points.len());
}

#[test]
fn orient3_signed_volume_is_invariant_under_independent_negative_homogeneous_scales() {
    let a = point([&[], &[], &[], &[1.]]);
    let b = point([&[2.], &[], &[], &[1.]]);
    let c = point([&[], &[3.], &[], &[1.]]);
    let d = point([&[], &[], &[5.], &[1.]]);
    assert_eq!(orient3([&a, &b, &c, &d], WORK).unwrap().sign, -1);
    assert_eq!(orient3([&b, &a, &c, &d], WORK).unwrap().sign, 1);
    for mask in 0..16 {
        let original = [[0., 0., 0.], [2., 0., 0.], [0., 3., 0.], [0., 0., 5.]];
        let scaled: Vec<_> = original
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                let multiplier = if mask & (1 << i) == 0 { i as f64 + 1. } else { -(i as f64 + 1.) };
                point([&[p[0] * multiplier], &[p[1] * multiplier], &[p[2] * multiplier], &[multiplier]])
            })
            .collect();
        assert_eq!(orient3([&scaled[0], &scaled[1], &scaled[2], &scaled[3]], WORK).unwrap().sign, -1);
    }
    let clone = d.clone();
    assert_eq!(orient3([&a, &b, &d, &clone], WORK).unwrap().sign, 0);
    let same = point([&[], &[], &[-15.], &[-3.]]);
    assert_eq!(orient3([&a, &b, &d, &same], WORK).unwrap().sign, 0);
}

#[test]
fn orient3_preserves_a_one_subnormal_departure_from_an_oblique_plane() {
    let tiny = f64::from_bits(1);
    let a = point([&[1.], &[1.], &[1.], &[1.]]);
    let b = point([&[2.], &[1.], &[2.], &[1.]]);
    let c = point([&[1.], &[2.], &[2.], &[1.]]);
    for (offset, expected) in [(tiny, -1), (-tiny, 1), (0., 0)] {
        let d = point([&[2.], &[2.], &[3., offset], &[1.]]);
        let report = orient3([&a, &b, &c, &d], WORK).unwrap();
        assert_eq!(report.sign, expected);
        assert!(report.integer_fallback);
        assert_eq!(orient3([&a, &c, &b, &d], WORK).unwrap().sign, -expected);
    }
}

#[test]
fn orient3_exact_budgets_and_counted_retries_preserve_the_original_points() {
    let a = point([&[1.], &[1.], &[1.], &[1.]]);
    let b = point([&[2.], &[1.], &[2.], &[1.]]);
    let c = point([&[1.], &[2.], &[2.], &[1.]]);
    let d = point([&[2.], &[2.], &[3., f64::from_bits(1)], &[1.]]);
    let arguments = [&a, &b, &c, &d];
    let report = orient3(arguments, WORK).unwrap();
    assert!(report.integer_fallback && report.work > 1);
    assert_eq!(orient3(arguments, report.work).unwrap(), report);
    let failure = orient3(arguments, report.work - 1).unwrap_err();
    assert!(failure.message.contains("budget") && failure.work > 0 && failure.work < report.work);
    assert_eq!(orient3(arguments, report.work - 1).unwrap_err(), failure);
    // A retry may spend only its remaining allowance. Neither failed predicate
    // can silently reset cumulative work or mutate a cached point.
    let total_allowance = failure.work * 2 - 1;
    let retry = orient3(arguments, total_allowance - failure.work).unwrap_err();
    assert!(retry.message.contains("budget") && retry.work > 0);
    assert!(failure.work + retry.work <= total_allowance);
    assert_eq!(orient3(arguments, WORK).unwrap(), report);
    assert_eq!(orient3([&d, &b, &c, &a], WORK).unwrap().sign, -report.sign);
    let no_work = orient3(arguments, 0).unwrap_err();
    assert_eq!(no_work.work, 0);
    assert!(no_work.message.contains("budget"));
}
