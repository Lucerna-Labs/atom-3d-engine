use super::*;
use crate::exact_geometry::{compare_axis, orient3 as rational_orient3, Point};

const WORK: usize = 10_000_000;
fn budget(maximum: usize) -> Budget {
    Budget { maximum, report: IntersectionReport::default(), failed_pair: None }
}
fn rational(p: Vec3) -> Point {
    Point::from_expansions([&[f64::from(p.x)], &[f64::from(p.y)], &[f64::from(p.z)]], &[1.], WORK).unwrap().point
}

// Deliberately incomplete arithmetic variants establish that the fixture
// detects the loss of each distinct exactness obligation. They are test-only.
fn omitted_component_sign(p: [[f64; 3]; 4], omit: usize) -> i8 {
    let mut work = budget(WORK);
    let mut v = [[[0.; 2]; 3]; 3];
    for row in 0..3 {
        for axis in 0..3 {
            v[row][axis] = difference(p[row][axis], p[3][axis], &mut work).unwrap();
            if omit == 0 {
                v[row][axis][0] = 0.;
            }
        }
    }
    let mut sum = Vec::new();
    for (axes, sign) in
        [([0, 1, 2], 1.), ([1, 2, 0], 1.), ([2, 0, 1], 1.), ([0, 2, 1], -1.), ([1, 0, 2], -1.), ([2, 1, 0], -1.)]
    {
        for a in v[0][axes[0]] {
            for b in v[1][axes[1]] {
                let high = a * b;
                let low = if omit == 1 { 0. } else { a.mul_add(b, -high) };
                for c in v[2][axes[2]] {
                    for component in [low, high] {
                        let high = component * c;
                        let low = if omit == 2 { 0. } else { component.mul_add(c, -high) };
                        for term in [low, high] {
                            if term != 0. {
                                sum = grow(&sum, sign * term, &mut work).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
    expansion_sign(&sum)
}

#[test]
fn filtered_and_forced_difference_fallback_match_fraction_and_rational_oracles() {
    let mut count = 0;
    let mut fallbacks = 0;
    let mut omitted_failures = [0usize; 3];
    let mut signs = [0usize; 3];
    let mut maximum_forced_work = 0;
    for row in include_str!("../tests/fixtures/surface_orient3/oracle.csv").lines().filter(|s| !s.starts_with('#')) {
        let fields: Vec<_> = row.split(',').collect();
        assert_eq!(fields.len(), 14);
        let values: Vec<f32> =
            fields[1..13].iter().map(|v| f32::from_bits(u32::from_str_radix(v, 16).unwrap())).collect();
        let p: [Vec3; 4] = std::array::from_fn(|i| Vec3::new(values[3 * i], values[3 * i + 1], values[3 * i + 2]));
        let expected: i8 = fields[13].parse().unwrap();
        let mut work = budget(WORK);
        assert_eq!(orient3(p[0], p[1], p[2], p[3], &mut work).unwrap(), expected, "filtered {}", fields[0]);
        fallbacks += work.report.exact_predicates;
        let mut forced = budget(WORK);
        assert_eq!(orient3_expansion(p.map(point), &mut forced).unwrap(), expected, "forced {}", fields[0]);
        maximum_forced_work = maximum_forced_work.max(forced.report.work);
        let exact = p.map(rational);
        assert_eq!(
            rational_orient3([&exact[0], &exact[1], &exact[2], &exact[3]], WORK).unwrap().sign,
            expected,
            "verified rational reference {}",
            fields[0]
        );
        for (omit, failures) in omitted_failures.iter_mut().enumerate() {
            *failures += usize::from(omitted_component_sign(p.map(point), omit) != expected);
        }
        signs[(expected + 1) as usize] += 1;
        count += 1;
    }
    assert_eq!(count, 4298);
    assert_eq!(signs, [1909, 480, 1909]);
    assert!(fallbacks > 500);
    assert!(
        omitted_failures.iter().all(|&n| n > 0),
        "each exact component must have an effective negative control: {omitted_failures:?}"
    );
    eprintln!("F32_ORIENT3_ORACLE cases={count} exact_fallbacks={fallbacks} omitted_difference_first_product_third_product_failures={omitted_failures:?} max_forced_work={maximum_forced_work}");
}

#[test]
fn difference_tail_is_exact_across_f32_least_bits_and_full_exponent_span() {
    for (a, b) in [
        (f32::MAX, f32::from_bits(1)),
        (f32::MAX, -f32::from_bits(1)),
        (f32::MAX, -f32::MAX),
        (f32::MIN_POSITIVE, f32::from_bits(1)),
        (1., 2f32.powi(-100)),
        (-1., 2f32.powi(-149)),
        (0., -0.),
    ] {
        let (a, b) = (f64::from(a), f64::from(b));
        let mut work = budget(6);
        let parts = difference(a, b, &mut work).unwrap();
        assert_eq!(work.report.work, 6);
        let actual = Point::from_expansions([&parts, &[], &[]], &[1.], WORK).unwrap().point;
        let expected = Point::from_expansions([&[a, -b], &[], &[]], &[1.], WORK).unwrap().point;
        assert_eq!(compare_axis(&actual, &expected, 0, WORK).unwrap().sign, 0);
        let mut short = budget(5);
        assert!(difference(a, b, &mut short).unwrap_err().contains("budget"));
        assert_eq!(short.report.work, 0);
    }
}

#[test]
fn forced_difference_fallback_exact_budget_replays_and_failures_keep_charges() {
    let large = 2f32.powi(100);
    let p = [
        Vec3::new(large, 1., 0.),
        Vec3::new(2. * large, 0., 1.),
        Vec3::new(3. * large, 1., 1.),
        Vec3::new(f32::from_bits(1), 0., 0.),
    ]
    .map(point);
    let mut work = budget(WORK);
    let expected = orient3_expansion(p, &mut work).unwrap();
    assert_ne!(expected, 0);
    let actual_work = work.report.work;
    let mut exact = budget(actual_work);
    assert_eq!(orient3_expansion(p, &mut exact).unwrap(), expected);
    assert_eq!(exact.report.work, actual_work);
    let mut short = budget(actual_work - 1);
    assert!(orient3_expansion(p, &mut short).unwrap_err().contains("budget"));
    assert!(short.report.work > 0 && short.report.work < actual_work);
    let allowance = 2 * short.report.work - 1;
    let mut retry = budget(allowance - short.report.work);
    assert!(orient3_expansion(p, &mut retry).unwrap_err().contains("budget"));
    assert!(short.report.work + retry.report.work <= allowance);
}
