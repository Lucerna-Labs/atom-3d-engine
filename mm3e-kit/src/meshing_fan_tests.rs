use super::*;

fn vertex(point: [f64; 3], id: u32) -> Vertex {
    let nodes = [id, u32::MAX, u32::MAX, u32::MAX];
    Vertex {
        key: Key { nodes, planes: [u64::MAX; 3] },
        weights: [1., 0., 0., 0.],
        point,
        faces: 14,
        planes: 0,
        source: SourcePoint { point, nodes, weights: [1., 0., 0., 0.], affine: None },
        global_planes: None,
    }
}

fn legacy_score(polygon: &[Vertex], anchor: usize) -> f64 {
    let position = |i: usize| polygon[i % polygon.len()].point.map(|p| f64::from(p as f32));
    (1..polygon.len() - 1)
        .map(|i| {
            let a = position(anchor);
            let b = position(anchor + i);
            let c = position(anchor + i + 1);
            let ab = sub(b, a);
            let ac = sub(c, a);
            let bc = sub(c, b);
            let normal = cross(ab, ac);
            let sum = dot(ab, ab) + dot(ac, ac) + dot(bc, bc);
            if sum == 0. {
                0.
            } else {
                dot(normal, normal) / (sum * sum)
            }
        })
        .fold(f64::INFINITY, f64::min)
}

#[test]
fn factored_small_fans_preserve_every_score_bit_across_scales_and_rounding() {
    let mut random = 0x5ea8_42f1_01b7_933du64;
    let mut next = || {
        random ^= random << 13;
        random ^= random >> 7;
        random ^= random << 17;
        random
    };
    let mut count = 0;
    for n in [3, 4] {
        for case in 0..50_000 {
            let exponent = (case % 251) - 130;
            let scale = 2_f64.powi(exponent);
            let polygon: Vec<_> = (0..n)
                .map(|i| {
                    let point = std::array::from_fn(|_| {
                        if case % 2 == 0 {
                            // Full f32 exponent and sign range, including zeros.
                            let mut bits = next() as u32;
                            if bits & 0x7f80_0000 == 0x7f80_0000 {
                                bits &= !0x0080_0000;
                            }
                            f64::from(f32::from_bits(bits))
                        } else {
                            // Nearby points, narrow triangles and nontrivial
                            // rounding to the representable positions used in fans.
                            let integer = (next() % 65) as i32 - 32;
                            f64::from((f64::from(integer) * scale) as f32) + scale * ((next() % 3) as f64 - 1.) * 0.25
                        }
                    });
                    vertex(point, (n - i) as u32)
                })
                .collect();
            let scores = small_fan_scores(&polygon);
            for (anchor, score) in scores.iter().enumerate().take(n) {
                assert_eq!(
                    score.to_bits(),
                    legacy_score(&polygon, anchor).to_bits(),
                    "n={n}, case={case}, anchor={anchor}"
                );
                count += 1;
            }
        }
    }
    assert_eq!(count, 350_000);
}

#[test]
fn factored_fans_preserve_degenerate_and_signed_zero_ties() {
    for points in [
        [[0., 0., 0.]; 4],
        [[0., -0., 0.], [1., 0., 0.], [1., 1., -0.], [0., 1., 0.]],
        [[0., 0., 0.], [1., 1., 1.], [2., 2., 2.], [3., 3., 3.]],
    ] {
        for n in [3, 4] {
            let polygon: Vec<_> = points[..n].iter().enumerate().map(|(i, &p)| vertex(p, i as u32)).collect();
            for (anchor, score) in small_fan_scores(&polygon).iter().enumerate().take(n) {
                assert_eq!(score.to_bits(), legacy_score(&polygon, anchor).to_bits());
            }
        }
    }
}

#[test]
fn factored_large_fans_preserve_scores_through_the_cached_polygon_range() {
    let mut random = 0x3f92_1a6c_7b44_0de1u64;
    let mut next = || {
        random ^= random << 13;
        random ^= random >> 7;
        random ^= random << 17;
        random
    };
    let mut checks = 0;
    for n in 5..=16 {
        for case in 0..2_000 {
            let scale = 2_f64.powi((case % 41) - 20);
            let polygon: Vec<_> = (0..n)
                .map(|i| {
                    let angle = (i as f64 + 0.25 * f64::from((case % 7) as u32)) * std::f64::consts::TAU / n as f64;
                    let radius = scale * (1. + f64::from((next() % 31) as u32) * 0.01);
                    vertex(
                        [
                            (angle.cos() * radius) as f32 as f64,
                            (angle.sin() * radius) as f32 as f64,
                            ((next() % 5) as f64 - 2.) * scale * 0.125,
                        ],
                        i as u32,
                    )
                })
                .collect();
            let scores = small_fan_scores(&polygon);
            assert_eq!(scores.len(), n);
            for (anchor, score) in scores.iter().enumerate() {
                assert_eq!(
                    score.to_bits(),
                    legacy_score(&polygon, anchor).to_bits(),
                    "n={n}, case={case}, anchor={anchor}"
                );
                checks += 1;
            }
        }
    }
    assert_eq!(checks, 252_000);
}
