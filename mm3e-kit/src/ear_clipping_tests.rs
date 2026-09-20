use super::*;
include!("../tests/fixtures/ear_clipping_reference.rs");
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
fn compare(points: &[[f64; 3]], normal: [f64; 3]) -> (usize, usize) {
    let ids: Vec<u32> = (0..points.len()).map(|i| 100 + 3 * i as u32).collect();
    let coordinate = |id: u32| points[(id as usize - 100) / 3];
    let mut old = work();
    let expected = reference_ear_clipping(&ids, coordinate, normal, &mut old).unwrap();
    let mut new = work();
    let actual = convex_triangulation_with(&ids, coordinate, normal, &mut new).unwrap();
    assert_eq!(actual, expected, "points={points:?} normal={normal:?}");
    (old.used, new.used)
}
fn reference_status(ring: &[(u32, u32)], i: usize, points: &[[f64; 3]]) -> Option<f64> {
    let n = ring.len();
    let ids = [ring[(i + n - 1) % n].0, ring[i].0, ring[(i + 1) % n].0];
    let [a, b, c] = ids.map(|i| points[i as usize]);
    let normal = [0., 0., 1.];
    let face_cross = cross(sub(b, a), sub(c, a));
    if dot(face_cross, normal) <= 0. {
        return None;
    }
    if ring.iter().filter(|r| !ids.contains(&r.0)).any(|&(id, _)| {
        let p = points[id as usize];
        [(a, b), (b, c), (c, a)].into_iter().all(|(a, b)| dot(cross(sub(b, a), sub(p, a)), normal) >= 0.)
    }) {
        return None;
    }
    let ab = sub(b, a);
    let ac = sub(c, a);
    let bc = sub(c, b);
    let sum = dot(ab, ab) + dot(ac, ac) + dot(bc, bc);
    Some(dot(face_cross, face_cross) / (sum * sum))
}
#[test]
fn cached_small_polygons_match_frozen_reference_triangles_and_large_path_stays_unchanged() {
    let mut seed = 0x18c3_672d_7d16_ac91u64;
    let mut count = 0;
    let (mut old_work, mut new_work) = (0, 0);
    for n in 3..=20 {
        for case in 0..96 {
            let mut points = Vec::new();
            for i in 0..n {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let r = if case % 3 == 0 { 1. } else { 0.4 + (seed >> 32) as f64 / u32::MAX as f64 };
                let angle = (i as f64) * std::f64::consts::TAU / n as f64;
                points.push([r * angle.cos(), r * angle.sin(), 0.]);
            }
            for flip in [false, true] {
                let mut p = points.clone();
                if flip {
                    p.reverse();
                }
                let sign = if flip { -1. } else { 1. };
                let (a, b) = compare(&p, [0., 0., sign]);
                if (5..=16).contains(&n) {
                    old_work += a;
                    new_work += b;
                } else {
                    assert_eq!(a, b);
                }
                count += 1;
            }
        }
    }
    assert_eq!(count, 3456);
    assert!(new_work < old_work);
    eprintln!("EAR_REFERENCE polygon_cases={count} old_work={old_work} cached_work={new_work}");
    for points in [
        vec![[0., 0., 0.], [2., 0., 0.], [2., 2., 0.], [1., 1., 0.], [0., 2., 0.]],
        vec![[0., 0., 0.], [2., 2., 0.], [0., 2., 0.], [2., 0., 0.], [3., 1., 0.]],
        vec![[0., 0., 0.], [1., 0., 0.], [2., 0., 0.], [2., 2., 0.], [0., 2., 0.]],
    ] {
        compare(&points, [0., 0., 1.]);
    }
}
#[test]
fn blocker_masks_and_two_neighbor_refreshes_match_reference_after_each_possible_removal() {
    let points = [[0., 0., 0.], [4., 0., 0.], [4., 4., 0.], [2., 1., 0.], [0., 4., 0.]];
    let initial: Vec<_> = (0..5).map(|i| (i, i)).collect();
    let mut original = Vec::new();
    for i in 0..5 {
        original.push(evaluate_cached_ear(&initial, i, [0., 0., 1.], &|id| points[id as usize], &mut work()).unwrap());
    }
    assert_ne!(original[0].blockers & (1 << 3), 0);
    let mut unblocked = false;
    for removed in 0..5 {
        let mut ring = initial.clone();
        let mut cached = original.clone();
        ring.remove(removed);
        cached.remove(removed);
        let active = ((1u32 << 5) - 1) & !(1 << removed);
        let next = removed % ring.len();
        let previous = (next + ring.len() - 1) % ring.len();
        for i in [previous, next] {
            cached[i] = evaluate_cached_ear(&ring, i, [0., 0., 1.], &|id| points[id as usize], &mut work()).unwrap();
        }
        for (i, ear) in cached.iter().enumerate() {
            let eligible = if ear.blockers & active == 0 { ear.quality } else { None };
            assert_eq!(eligible.map(f64::to_bits), reference_status(&ring, i, &points).map(f64::to_bits));
            if removed == 3 && ring[i].0 == 0 {
                assert!(ear.blockers != 0 && eligible.is_some());
                unblocked = true;
            }
        }
    }
    assert!(unblocked, "a removed nonneighbor blocker must release an unchanged ear");
}
#[test]
fn exact_cached_budget_replays_and_one_less_cannot_return_partial_triangulation() {
    let points = [
        [0., 0., 0.],
        [5., 0., 0.],
        [5., 5., 0.],
        [4., 5., 0.],
        [4., 1., 0.],
        [3., 1., 0.],
        [3., 4., 0.],
        [2., 4., 0.],
        [2., 1., 0.],
        [1., 1., 0.],
        [1., 5., 0.],
        [0., 5., 0.],
    ];
    let ids: Vec<_> = (0..points.len() as u32).collect();
    let mut full = work();
    let result = convex_triangulation_with(&ids, |i| points[i as usize], [0., 0., 1.], &mut full).unwrap();
    assert!(result.is_some());
    let mut exact = work();
    exact.limit = full.used;
    assert_eq!(convex_triangulation_with(&ids, |i| points[i as usize], [0., 0., 1.], &mut exact).unwrap(), result);
    assert_eq!(exact.used, full.used);
    let mut short = work();
    short.limit = full.used - 1;
    assert!(convex_triangulation_with(&ids, |i| points[i as usize], [0., 0., 1.], &mut short)
        .unwrap_err()
        .contains("bounded work"));
    assert!(short.used < full.used);
}

#[test]
fn tilted_translated_and_nearly_degenerate_ear_sequences_keep_reference_bits() {
    let mut count = 0;
    for n in 5..=16 {
        for seed in 0..32 {
            let points: Vec<_> = (0..n)
                .map(|i| {
                    let a = (i as f64) * std::f64::consts::TAU / n as f64;
                    let r = if (i + seed) % 3 == 0 { 0.37 } else { 1. };
                    [r * a.cos(), r * a.sin()]
                })
                .collect();
            for (scale, offset) in [(1., 0.), (2f64.powi(20), 2f64.powi(40)), (2f64.powi(-30), 1.)] {
                let transformed: Vec<_> = points
                    .iter()
                    .map(|&[x, y]| [offset + scale * (x + y), -offset + scale * (x - y), offset + scale * (x + 2. * y)])
                    .collect();
                compare(&transformed, [3., -1., -2.]);
                count += 1;
            }
        }
    }
    assert_eq!(count, 1152);
}
