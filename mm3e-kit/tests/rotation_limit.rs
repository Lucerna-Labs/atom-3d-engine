use mm3e_kit::{
    rotation_limit::{RotationLimit, ANGULAR_TOLERANCE, MAX_QUANTIZATION_ADJUSTMENT},
    Quat, Vec3,
};
use std::f64::consts::{PI, TAU};

const X: Vec3 = Vec3::new(1.0, 0.0, 0.0);
const Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);
fn axis(a: Vec3) -> [f64; 3] {
    let a = [f64::from(a.x), f64::from(a.y), f64::from(a.z)];
    let n = a[0].hypot(a[1]).hypot(a[2]);
    a.map(|x| x / n)
}
fn unit(q: Quat) -> [f64; 4] {
    let q = [f64::from(q.x), f64::from(q.y), f64::from(q.z), f64::from(q.w)];
    let n = q.iter().fold(0.0_f64, |n, x| n.hypot(*x));
    q.map(|x| x / n)
}
fn quaternion(q: [f64; 4]) -> Quat {
    let n = q.iter().fold(0.0_f64, |n, x| n.hypot(*x));
    let q = q.map(|x| (x / n) as f32);
    Quat { x: q[0], y: q[1], z: q[2], w: q[3] }
}
fn rotation(a: Vec3, angle: f64) -> Quat {
    let a = axis(a);
    let (s, c) = (0.5 * angle).sin_cos();
    quaternion([a[0] * s, a[1] * s, a[2] * s, c])
}
fn hamilton(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    let [x, y, z, w] = a;
    let [i, j, k, r] = b;
    [
        w * i + x * r + y * k - z * j,
        w * j - x * k + y * r + z * i,
        w * k + x * j - y * i + z * r,
        w * r - x * i - y * j - z * k,
    ]
}
fn product(a: Quat, b: Quat) -> Quat {
    quaternion(hamilton(unit(a), unit(b)))
}
fn distance(a: Quat, b: Quat) -> f64 {
    let [x, y, z, w] = unit(a);
    let relative = hamilton([-x, -y, -z, w], unit(b));
    2.0 * relative[0].hypot(relative[1]).hypot(relative[2]).atan2(relative[3].abs())
}
fn same(a: Quat, b: Quat, tolerance: f64) {
    assert!(distance(a, b) <= tolerance, "{a:?} != {b:?}: angle {}", distance(a, b));
}
fn bits(q: Quat) -> [u32; 4] {
    [q.x.to_bits(), q.y.to_bits(), q.z.to_bits(), q.w.to_bits()]
}
fn opposite(q: Quat) -> Quat {
    Quat { x: -q.x, y: -q.y, z: -q.z, w: -q.w }
}
fn proper(q: Quat) {
    let values = [q.x, q.y, q.z, q.w];
    assert!(values.iter().all(|x| x.is_finite()));
    let norm = values.iter().fold(0.0_f64, |n, &x| n.hypot(f64::from(x)));
    assert!((norm - 1.0).abs() < 5e-7, "non-unit output {q:?}: {norm}");
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
fn cross([x, y, z]: [f64; 3], [i, j, k]: [f64; 3]) -> [f64; 3] {
    [y * k - z * j, z * i - x * k, x * j - y * i]
}
fn swing_by_rotated_axis(q: Quat, a: Vec3) -> f64 {
    let a = axis(a);
    let [x, y, z, w] = unit(q);
    let first = cross([x, y, z], a);
    let second = cross([x, y, z], first);
    let rotated = std::array::from_fn(|i| a[i] + 2.0 * (w * first[i] + second[i]));
    let sine = cross(a, rotated).iter().fold(0.0_f64, |n, x| n.hypot(*x));
    sine.atan2(dot(a, rotated))
}
fn cyclic(a: f64, b: f64) -> f64 {
    let d = (a - b).rem_euclid(TAU);
    d.min(TAU - d)
}
fn interval_distance(angle: f64, min: f64, max: f64) -> f64 {
    if (min..=max).contains(&angle) {
        0.0
    } else {
        cyclic(angle, min).min(cyclic(angle, max))
    }
}

#[test]
fn hinge_removes_off_axis_rotation_and_preserves_its_requested_twist() {
    let limit = RotationLimit::Hinge { axis: Z, min: -0.5, max: 0.7 };
    let request = product(rotation(X, 0.6), rotation(Z, 0.3));
    let result = limit.project(request).unwrap();
    assert!(result.violated);
    assert!((result.swing - 0.6).abs() < 1e-7);
    assert!((result.twist - 0.3).abs() < 1e-7);
    assert!((result.angular_error - 0.6).abs() < 1e-7);
    same(result.rotation, rotation(Z, 0.3), 1e-7);
    assert!(swing_by_rotated_axis(result.rotation, Z) < 1e-7);
    proper(result.rotation);
}

#[test]
fn cone_and_twist_follow_the_declared_separate_clamping_rule() {
    let limit = RotationLimit::SwingTwist { axis: Z, swing: 0.4, twist_min: -0.2, twist_max: 0.25 };
    let request = product(rotation(X, 0.8), rotation(Z, 0.6));
    let result = limit.project(request).unwrap();
    let expected = product(rotation(X, 0.4), rotation(Z, 0.25));
    same(result.rotation, expected, 1e-7);
    assert!((swing_by_rotated_axis(result.rotation, Z) - 0.4).abs() < 1e-7);
    assert!((result.swing - 0.8).abs() < 1e-7);
    assert!((result.twist - 0.6).abs() < 1e-7);
    assert!((result.angular_error - distance(request, expected)).abs() < 1e-7);
    assert!(result.violated && !result.singular_twist);
}

#[test]
fn cyclic_twist_chooses_the_correct_endpoint_across_plus_minus_pi() {
    for (requested, min, max, endpoint) in [
        (-3.13, -0.2, 3.0, 3.0),
        (3.13, -3.0, 0.2, -3.0),
        (-PI + 1e-7, -0.1, PI - 1e-4, PI - 1e-4),
        (PI - 1e-7, -PI + 1e-4, 0.1, -PI + 1e-4),
    ] {
        let limit = RotationLimit::Hinge { axis: Z, min, max };
        let result = limit.project(rotation(Z, requested)).unwrap();
        same(result.rotation, rotation(Z, endpoint), 2e-7);
        assert!((result.angular_error - cyclic(requested, endpoint)).abs() < 1e-7);
    }
    let pi = Quat { x: 0., y: 0., z: 1., w: 0. };
    for (min, max) in [(-PI, 0.0), (0.0, PI), (-PI, PI)] {
        let result = RotationLimit::Hinge { axis: Z, min, max }.project(pi).unwrap();
        assert!(!result.violated);
        assert_eq!(bits(result.rotation), bits(pi));
    }
    let tie = RotationLimit::Hinge { axis: Z, min: -1., max: 1. }.project(pi).unwrap();
    same(tie.rotation, rotation(Z, -1.), 1e-7);
}

#[test]
fn compliant_rotations_identity_and_tiny_angles_keep_exact_bits() {
    let limit = RotationLimit::Hinge { axis: Z, min: -0.5, max: 0.7 };
    for request in [Quat::IDENTITY, rotation(Z, 0.2), rotation(Z, -0.49), rotation(X, 0.5 * ANGULAR_TOLERANCE)] {
        let result = limit.project(request).unwrap();
        assert!(!result.violated);
        assert_eq!(bits(result.rotation), bits(request));
    }
    let locked = RotationLimit::Hinge { axis: Z, min: 0., max: 0. };
    let tiny = locked.project(rotation(X, 1e-12)).unwrap();
    assert!(tiny.angular_error > 0.9e-12 && tiny.angular_error < 1.1e-12, "tiny angle was lost: {tiny:?}");
    let outside = locked.project(rotation(X, 2.0 * ANGULAR_TOLERANCE)).unwrap();
    assert!(outside.violated);
    assert_eq!(bits(outside.rotation), bits(Quat::IDENTITY));
}

#[test]
fn singular_perpendicular_half_turns_have_a_deterministic_zero_twist_gauge() {
    let request = Quat { x: 1., y: 0., z: 0., w: 0. };
    let hinge = RotationLimit::Hinge { axis: Z, min: -1., max: 1. }.project(request).unwrap();
    assert!(hinge.singular_twist && hinge.violated);
    assert_eq!(hinge.swing, PI);
    assert_eq!(hinge.twist, 0.0);
    assert_eq!(hinge.angular_error, PI);
    assert_eq!(bits(hinge.rotation), bits(Quat::IDENTITY));
    for swing in [0.0, 0.8, PI] {
        let limit = RotationLimit::SwingTwist { axis: Z, swing, twist_min: -0.2, twist_max: 0.3 };
        let projected = limit.project(request).unwrap();
        let negative = limit.project(opposite(request)).unwrap();
        assert!(projected.singular_twist && negative.singular_twist);
        assert_eq!(projected.twist, 0.0);
        same(projected.rotation, rotation(X, swing), 1e-7);
        same(projected.rotation, negative.rotation, 1e-12);
        assert_eq!(projected.angular_error, negative.angular_error);
        assert!((projected.angular_error - (PI - swing)).abs() < 1e-12);
    }
}

#[test]
fn dynamic_tiny_and_large_axes_and_scaled_quaternions_normalize_without_overflow() {
    for a in [
        Vec3::new(1., 2., -3.),
        Vec3::new(f32::MAX, f32::MAX, -f32::MAX),
        Vec3::new(f32::from_bits(1), f32::from_bits(2), -f32::from_bits(3)),
    ] {
        let request = rotation(a, 0.8);
        let limit = RotationLimit::Hinge { axis: a, min: -0.2, max: 0.4 };
        let result = limit.project(request).unwrap();
        same(result.rotation, rotation(a, 0.4), 1e-7);
        assert!((result.twist - 0.8).abs() < 1e-7);
        proper(result.rotation);
        let unbounded = RotationLimit::SwingTwist { axis: a, swing: PI, twist_min: -PI, twist_max: PI };
        for factor in [1e-30_f32, 16.0, 1e30] {
            let scaled =
                Quat { x: request.x * factor, y: request.y * factor, z: request.z * factor, w: request.w * factor };
            let normalized = unbounded.project(scaled).unwrap();
            assert!(!normalized.violated);
            proper(normalized.rotation);
            same(normalized.rotation, request, 2e-7);
        }
    }
    let subnormal = Quat { x: f32::from_bits(1), y: 0., z: 0., w: f32::from_bits(1) };
    let result = RotationLimit::Hinge { axis: X, min: -PI, max: PI }.project(subnormal).unwrap();
    proper(result.rotation);
    same(result.rotation, rotation(X, PI / 2.0), 1e-7);
}

#[test]
fn invalid_axes_bounds_and_quaternions_are_rejected() {
    let mut invalid = vec![
        RotationLimit::Hinge { axis: Vec3::ZERO, min: -1., max: 1. },
        RotationLimit::Hinge { axis: Vec3::new(f32::INFINITY, 0., 1.), min: -1., max: 1. },
        RotationLimit::Hinge { axis: Vec3::new(f32::NAN, 0., 1.), min: -1., max: 1. },
    ];
    for (min, max) in
        [(0.1, 1.), (-1., -0.1), (1., -1.), (-PI - 1e-10, 0.), (0., PI + 1e-10), (f64::NAN, 0.), (0., f64::INFINITY)]
    {
        invalid.push(RotationLimit::Hinge { axis: Z, min, max });
    }
    for swing in [-1e-12, PI + 1e-12, f64::NAN, f64::INFINITY] {
        invalid.push(RotationLimit::SwingTwist { axis: Z, swing, twist_min: -1., twist_max: 1. });
    }
    for limit in invalid {
        assert!(limit.validate().is_err(), "{limit:?}");
        assert!(limit.project(Quat::IDENTITY).is_err(), "{limit:?}");
    }
    let valid = RotationLimit::Hinge { axis: Z, min: 0., max: 0. };
    for request in [
        Quat { x: 0., y: 0., z: 0., w: 0. },
        Quat { x: f32::NAN, y: 0., z: 0., w: 1. },
        Quat { x: 0., y: 0., z: 0., w: f32::INFINITY },
    ] {
        assert!(valid.project(request).is_err());
    }
}

#[test]
fn near_singular_cones_remain_finite_compliant_and_idempotent() {
    for gap in [0.0, 1e-10, 1e-7, 1e-6, 1e-5, 1e-3] {
        for maximum in [0.3, PI - 1e-6, PI] {
            let limit = RotationLimit::SwingTwist { axis: Z, swing: maximum, twist_min: -0.2, twist_max: 0.3 };
            let request = product(rotation(X, PI - gap), rotation(Z, 0.9));
            let first = limit.project(request).unwrap_or_else(|error| panic!("gap {gap}, max {maximum}: {error}"));
            proper(first.rotation);
            let again = limit.project(first.rotation).unwrap();
            assert!(!again.violated, "gap {gap}, max {maximum}: {first:?}; repeat {again:?}");
            assert_eq!(bits(first.rotation), bits(again.rotation));
            assert!(swing_by_rotated_axis(first.rotation, Z) <= maximum + ANGULAR_TOLERANCE);
            let native = Quat::from_mat3(first.rotation.to_mat3());
            assert!(!limit.project(native).unwrap().violated, "native readback gap {gap}, max {maximum}");
        }
    }
    // A non-Cartesian axis adds f32 dot-product roundoff at the singularity;
    // the gauge and readback must remain stable after quantization as well.
    for a in [Vec3::new(1., 2., 3.), Vec3::new(1e-35, 2e-35, -1e-35)] {
        let perpendicular = cross(axis(a), [1., 0., 0.]);
        let perpendicular = Vec3::new(perpendicular[0] as f32, perpendicular[1] as f32, perpendicular[2] as f32);
        for gap in [0.0, 1e-7, 1e-5] {
            let request = product(rotation(perpendicular, PI - gap), rotation(a, 0.9));
            let limit = RotationLimit::SwingTwist { axis: a, swing: PI - 1e-6, twist_min: -0.2, twist_max: 0.3 };
            let first = limit.project(request).unwrap_or_else(|error| panic!("axis {a:?}, gap {gap}: {error}"));
            let again = limit.project(first.rotation).unwrap();
            proper(first.rotation);
            assert!(!again.violated, "axis {a:?}, gap {gap}: {first:?}; repeat {again:?}");
            assert_eq!(bits(first.rotation), bits(again.rotation));
            let native = Quat::from_mat3(first.rotation.to_mat3());
            assert!(!limit.project(native).unwrap().violated, "native readback axis {a:?}, gap {gap}");
            if gap == 0.0 {
                assert!(first.singular_twist);
            }
        }
    }
}

#[test]
fn conservative_quantization_repair_is_reported_and_excessive_repairs_are_rejected() {
    let a = Vec3::new(1., 2., 3.);
    let perpendicular = cross(axis(a), [1., 0., 0.]);
    let perpendicular = Vec3::new(perpendicular[0] as f32, perpendicular[1] as f32, perpendicular[2] as f32);
    let request = product(rotation(perpendicular, PI - 1e-5), rotation(a, 0.9));
    let limit = RotationLimit::SwingTwist { axis: a, swing: PI - 1e-6, twist_min: -0.2, twist_max: 0.3 };
    let repaired = limit.project(request).unwrap();
    assert!(repaired.quantization_adjusted, "{repaired:?}");
    assert!(repaired.quantization_adjustment > ANGULAR_TOLERANCE);
    assert!(repaired.quantization_adjustment <= MAX_QUANTIZATION_ADJUSTMENT);
    assert!(!limit.project(repaired.rotation).unwrap().violated);
    let native = Quat::from_mat3(repaired.rotation.to_mat3());
    assert!(!limit.project(native).unwrap().violated);
    println!(
        "reported near-pole interior adjustment {} radians; requested correction {} radians",
        repaired.quantization_adjustment, repaired.angular_error
    );

    // This retained case cannot satisfy the explicit 0.01-radian repair budget
    // after native matrix readback. It must reject rather than return the
    // original unstable boundary, enlarge the tolerance, or apply a larger fix.
    let a = Vec3::new(1e30, -3e30, 2e30);
    let perpendicular = cross(axis(a), [1., 0., 0.]);
    let perpendicular = Vec3::new(perpendicular[0] as f32, perpendicular[1] as f32, perpendicular[2] as f32);
    let request = product(rotation(perpendicular, PI - 1e-5), rotation(a, 0.9));
    let limit = RotationLimit::SwingTwist { axis: a, swing: PI - 1e-6, twist_min: -0.2, twist_max: 0.3 };
    let error = limit.project(request).unwrap_err();
    assert!(error.contains("cannot be represented stably in f32"), "{error}");
    assert!(error.contains("0.01 radians"), "{error}");
}

struct Random(u64);
impl Random {
    fn number(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn signed(&mut self) -> f64 {
        2.0 * self.number() - 1.0
    }
}

#[test]
fn deterministic_random_sweep_preserves_sign_invariance_compliance_and_idempotence() {
    let mut random = Random(0x7359_22ad_a136_100d);
    for case in 0..6000 {
        let a = Vec3::new(random.signed() as f32, random.signed() as f32, random.signed() as f32);
        let min = -PI * random.number();
        let max = PI * random.number();
        let swing = if case % 31 == 0 { PI } else { PI * random.number() };
        let limit = if case % 4 == 0 {
            RotationLimit::Hinge { axis: a, min, max }
        } else {
            RotationLimit::SwingTwist { axis: a, swing, twist_min: min, twist_max: max }
        };
        let request = quaternion([random.signed(), random.signed(), random.signed(), random.signed()]);
        let first = limit.project(request).unwrap();
        let signed = limit.project(opposite(request)).unwrap();
        proper(first.rotation);
        assert_eq!(first.violated, signed.violated);
        assert_eq!(first.singular_twist, signed.singular_twist);
        assert_eq!(first.quantization_adjusted, signed.quantization_adjusted);
        assert_eq!(first.quantization_adjustment, signed.quantization_adjustment);
        assert_eq!(first.angular_error, signed.angular_error);
        assert_eq!(first.swing, signed.swing);
        assert_eq!(first.twist, signed.twist);
        same(first.rotation, signed.rotation, 1e-12);
        let again = limit.project(first.rotation).unwrap();
        assert!(!again.violated, "case {case}, {limit:?}: {first:?}; repeat {again:?}");
        assert_eq!(bits(first.rotation), bits(again.rotation), "case {case} not idempotent");
        let maximum_swing = if matches!(limit, RotationLimit::Hinge { .. }) { 0.0 } else { swing };
        assert!(swing_by_rotated_axis(first.rotation, a) <= maximum_swing + ANGULAR_TOLERANCE);
        let [x, y, z, w] = unit(first.rotation);
        let along = dot([x, y, z], axis(a));
        let raw_angle = 2.0 * along.atan2(w);
        let twist = (raw_angle + PI).rem_euclid(TAU) - PI;
        // Twist coordinates are singular near a half-turn. Weight their cyclic
        // error by the half-angle magnitude to test physical angular compliance.
        assert!(
            interval_distance(twist, min, max) * w.hypot(along) < 2.0 * ANGULAR_TOLERANCE,
            "case {case}: twist {twist}, interval {min}..{max}"
        );
    }
}

#[test]
fn hinge_projection_is_at_least_as_close_as_independent_dense_angle_candidates() {
    let mut random = Random(0x8462_8ca1_9912_3347);
    for _ in 0..100 {
        let a = Vec3::new(random.signed() as f32, random.signed() as f32, random.signed() as f32);
        let min = -PI * random.number();
        let max = PI * random.number();
        let request = quaternion([random.signed(), random.signed(), random.signed(), random.signed()]);
        let projected = RotationLimit::Hinge { axis: a, min, max }.project(request).unwrap();
        for step in 0..=256 {
            let candidate = rotation(a, min + (max - min) * f64::from(step) / 256.0);
            assert!(distance(request, projected.rotation) <= distance(request, candidate) + 2e-7);
        }
    }
}
