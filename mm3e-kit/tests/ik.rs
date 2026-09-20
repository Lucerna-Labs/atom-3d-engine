use mm3e_kit::{
    ik::{solve_two_bone, TwoBoneIk, UnreachablePolicy},
    Quat, Vec3,
};

const REJECT: UnreachablePolicy = UnreachablePolicy::Reject;
const CLAMP: UnreachablePolicy = UnreachablePolicy::Clamp;

fn distance(a: Vec3, b: Vec3) -> f64 {
    (f64::from(a.x) - f64::from(b.x)).hypot(f64::from(a.y) - f64::from(b.y)).hypot(f64::from(a.z) - f64::from(b.z))
}

fn near(a: Vec3, b: Vec3, tolerance: f64) {
    assert!(distance(a, b) <= tolerance, "{a:?} != {b:?}, distance {}, tolerance {tolerance}", distance(a, b));
}

fn assert_lengths(root: Vec3, solution: &TwoBoneIk, relative: f64) {
    for (actual, expected) in [
        (distance(root, solution.middle), solution.segment_lengths[0]),
        (distance(solution.middle, solution.tip), solution.segment_lengths[1]),
    ] {
        assert!((actual - expected).abs() <= expected * relative, "{actual} vs {expected}");
    }
}

fn proper(q: Quat) {
    assert!([q.x, q.y, q.z, q.w].iter().all(|n| n.is_finite()));
    assert!((q.dot(q) - 1.0).abs() < 5e-7);
    let basis = q.to_mat3().cols;
    for i in 0..3 {
        for j in 0..3 {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((basis[i].dot(basis[j]) - expected).abs() < 1e-6);
        }
    }
    assert!((basis[0].cross(basis[1]).dot(basis[2]) - 1.0).abs() < 1e-6);
}

fn fk(root: Vec3, middle: Vec3, tip: Vec3, solution: &TwoBoneIk, tolerance: f64) {
    // Actually execute the two returned WORLD corrections through the engine's
    // matrix conversion. No analytic IK readback is used to reconstruct the elbow.
    let posed_middle = root + solution.root_rotation.to_mat3().mul_vec(middle - root);
    let rotated_second = solution.root_rotation.to_mat3().mul_vec(tip - middle);
    let posed_tip = posed_middle + solution.middle_rotation.to_mat3().mul_vec(rotated_second);
    near(posed_middle, solution.middle, tolerance);
    near(posed_tip, solution.effective_target, tolerance);
    proper(solution.root_rotation);
    proper(solution.middle_rotation);
}

#[test]
fn analytic_right_angle_is_reached_by_the_actual_two_rotation_corrections() {
    let root = Vec3::ZERO;
    let middle = Vec3::new(1.0, 0.0, 0.0);
    let tip = Vec3::new(2.0, 0.0, 0.0);
    let target = Vec3::new(1.0, 1.0, 0.0);
    let solved = solve_two_bone(root, middle, tip, target, Vec3::new(-1.0, 1.0, 0.0), REJECT).unwrap();
    near(solved.middle, Vec3::new(0.0, 1.0, 0.0), 1e-7);
    assert_eq!(solved.tip, target);
    assert_eq!(solved.segment_lengths, [1.0, 1.0]);
    assert_eq!(solved.residual, 0.0);
    assert!(!solved.clamped && !solved.pole_fallback && !solved.target_direction_fallback);
    assert_lengths(root, &solved, 1e-7);
    fk(root, middle, tip, &solved, 5e-7);
}

#[test]
fn analytic_out_of_plane_middle_has_the_correct_triangle_height() {
    let solved = solve_two_bone(
        Vec3::ZERO,
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 2.0),
        REJECT,
    )
    .unwrap();
    near(solved.middle, Vec3::new(0.5, 0.5, 0.5_f32.sqrt()), 1e-7);
    assert_lengths(Vec3::ZERO, &solved, 1e-7);
}

#[test]
fn unreachable_inner_and_outer_targets_are_explicitly_rejected_or_clamped_without_stretching() {
    let root = Vec3::ZERO;
    let middle = Vec3::new(2.0, 0.0, 0.0);
    let tip = Vec3::new(3.0, 0.0, 0.0);
    let pole = Vec3::new(0.0, 1.0, 0.0);
    for (target, effective, residual) in [
        (Vec3::new(0.25, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), 0.75),
        (Vec3::new(5.0, 0.0, 0.0), Vec3::new(3.0, 0.0, 0.0), 2.0),
    ] {
        assert!(solve_two_bone(root, middle, tip, target, pole, REJECT).unwrap_err().contains("outside reach"));
        let solved = solve_two_bone(root, middle, tip, target, pole, CLAMP).unwrap();
        assert!(solved.clamped);
        assert_eq!(solved.requested_target, target);
        assert_eq!(solved.effective_target, effective);
        assert_eq!(solved.residual, residual);
        assert_eq!(solved.middle, middle);
        assert_lengths(root, &solved, 1e-7);
        fk(root, middle, tip, &solved, 1e-6);
    }
    // With a longer distal segment, the elbow folds behind the root at minimum reach.
    let solved = solve_two_bone(root, Vec3::new(1.0, 0.0, 0.0), tip, Vec3::new(0.25, 0.0, 0.0), pole, CLAMP).unwrap();
    assert_eq!(solved.middle, Vec3::new(-1.0, 0.0, 0.0));
    assert_eq!(solved.tip, Vec3::new(1.0, 0.0, 0.0));
    assert_lengths(root, &solved, 1e-7);
    fk(root, Vec3::new(1.0, 0.0, 0.0), tip, &solved, 1e-6);
}

#[test]
fn flipping_the_pole_selects_opposite_bends_with_the_same_endpoint() {
    let root = Vec3::ZERO;
    let middle = Vec3::new(1.0, 0.0, 0.0);
    let tip = Vec3::new(2.0, 0.0, 0.0);
    let target = Vec3::new(1.0, 0.0, 0.0);
    let right = solve_two_bone(root, middle, tip, target, Vec3::new(0.0, 2.0, 0.0), REJECT).unwrap();
    let left = solve_two_bone(root, middle, tip, target, Vec3::new(0.0, -2.0, 0.0), REJECT).unwrap();
    assert_eq!(right.tip, left.tip);
    near(right.middle, Vec3::new(0.5, 0.75_f32.sqrt(), 0.0), 1e-7);
    near(left.middle, Vec3::new(0.5, -0.75_f32.sqrt(), 0.0), 1e-7);
    assert!(!right.pole_fallback && !left.pole_fallback);
    fk(root, middle, tip, &right, 1e-6);
    fk(root, middle, tip, &left, 1e-6);
}

#[test]
fn collinear_poles_fall_back_to_the_existing_bend_or_a_deterministic_perpendicular() {
    let root = Vec3::ZERO;
    let middle = Vec3::new(1.0, -1.0, 0.0);
    let tip = Vec3::new(2.0, 0.0, 0.0);
    let target = Vec3::new(1.0, 0.0, 0.0);
    for pole in [root, Vec3::new(3.0, 0.0, 0.0), Vec3::new(1e20, 1e-20, 0.0)] {
        let solved = solve_two_bone(root, middle, tip, target, pole, REJECT).unwrap();
        assert!(solved.pole_fallback);
        assert!(solved.middle.y < 0.0, "existing downward bend must be retained");
        fk(root, middle, tip, &solved, 1e-6);
    }
    let straight_middle = Vec3::new(1.0, 0.0, 0.0);
    let first = solve_two_bone(root, straight_middle, tip, target, root, REJECT).unwrap();
    assert!(first.pole_fallback);
    assert!(first.middle.y > 0.0 && first.middle.z == 0.0);
    for _ in 0..20 {
        let replay = solve_two_bone(root, straight_middle, tip, target, root, REJECT).unwrap();
        assert_eq!(first.middle, replay.middle);
        assert_eq!(quat_bits(first.root_rotation), quat_bits(replay.root_rotation));
    }
}

#[test]
fn equal_length_target_at_root_folds_toward_the_pole_and_reports_the_undefined_axis() {
    let root = Vec3::ZERO;
    let middle = Vec3::new(1.0, 0.0, 0.0);
    let tip = Vec3::new(2.0, 0.0, 0.0);
    let solved = solve_two_bone(root, middle, tip, root, Vec3::new(0.0, 1.0, 0.0), REJECT).unwrap();
    assert_eq!(solved.middle, Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(solved.tip, root);
    assert!(solved.target_direction_fallback && !solved.pole_fallback && !solved.clamped);
    fk(root, middle, tip, &solved, 1e-6);
    let no_pole = solve_two_bone(root, middle, tip, root, root, REJECT).unwrap();
    assert_eq!(no_pole.middle, middle);
    assert!(no_pole.pole_fallback && no_pole.target_direction_fallback);
    fk(root, middle, tip, &no_pole, 1e-6);
    // Starting fully folded is also a valid pair of nonzero segments.
    let folded =
        solve_two_bone(root, middle, root, Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0), REJECT).unwrap();
    assert_lengths(root, &folded, 1e-7);
    fk(root, middle, root, &folded, 1e-6);
}

#[test]
fn unequal_length_target_at_root_clamps_along_the_reported_current_direction() {
    let root = Vec3::ZERO;
    let middle = Vec3::new(0.0, 2.0, 0.0);
    let tip = Vec3::new(0.0, 3.0, 0.0);
    let solved = solve_two_bone(root, middle, tip, root, Vec3::new(1.0, 0.0, 0.0), CLAMP).unwrap();
    assert_eq!(solved.tip, Vec3::new(0.0, 1.0, 0.0));
    assert!(solved.target_direction_fallback && solved.clamped);
    assert_eq!(solved.residual, 1.0);
    assert!(solve_two_bone(root, middle, tip, root, Vec3::ONE, REJECT).is_err());
    fk(root, middle, tip, &solved, 1e-6);
}

#[test]
fn nearly_straight_and_nearly_fully_folded_chains_preserve_lengths_and_true_fk() {
    let root = Vec3::ZERO;
    let middle = Vec3::new(1.0, 0.0, 0.0);
    let tip = Vec3::new(2.0, 0.0, 0.0);
    let pole = Vec3::new(0.0, 1.0, 0.0);
    for target in [
        Vec3::new(f32::from_bits(2.0_f32.to_bits() - 1), 0.0, 0.0),
        Vec3::new(1e-20, 0.0, 0.0),
        Vec3::new(-1.99999, 0.0, 0.0),
        Vec3::new(-2.0, 0.0, 0.0),
    ] {
        let solved = solve_two_bone(root, middle, tip, target, pole, REJECT).unwrap();
        assert_eq!(solved.tip, target);
        assert_lengths(root, &solved, 1e-6);
        fk(root, middle, tip, &solved, 2e-6);
    }
}

#[test]
fn translations_and_uniform_scales_preserve_the_geometric_solution_and_corrections() {
    let points = [
        Vec3::ZERO,
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 0.0),
        Vec3::new(1.0, 1.0, 0.5),
        Vec3::new(0.0, 0.0, 2.0),
    ];
    let reference = solve_two_bone(points[0], points[1], points[2], points[3], points[4], REJECT).unwrap();
    for scale in [1e-20, 1e-10, 0.125, 1.0, 16.0, 1e10, 1e20] {
        let shift = Vec3::new(3.0, -2.0, 5.0).scale(scale);
        let changed = points.map(|point| point.scale(scale) + shift);
        let solved = solve_two_bone(changed[0], changed[1], changed[2], changed[3], changed[4], REJECT).unwrap();
        near(solved.middle, reference.middle.scale(scale) + shift, f64::from(scale) * 3e-6);
        near(solved.tip, reference.tip.scale(scale) + shift, f64::from(scale) * 3e-6);
        assert_lengths(changed[0], &solved, 2e-6);
        assert!(solved.root_rotation.dot(reference.root_rotation).abs() > 0.999999);
        assert!(solved.middle_rotation.dot(reference.middle_rotation).abs() > 0.999999);
        fk(changed[0], changed[1], changed[2], &solved, f64::from(scale) * 4e-6);
    }
}

#[test]
fn nonfinite_or_degenerate_inputs_are_rejected_without_rewriting_sources() {
    let source = [
        Vec3::ZERO,
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        for index in 0..5 {
            let mut values = source;
            values[index].x = value;
            let before = values.map(vec_bits);
            assert!(solve_two_bone(values[0], values[1], values[2], values[3], values[4], CLAMP).is_err());
            assert_eq!(values.map(vec_bits), before);
        }
    }
    for (middle, tip) in [(Vec3::ZERO, Vec3::ONE), (Vec3::ONE, Vec3::ONE)] {
        assert!(solve_two_bone(Vec3::ZERO, middle, tip, Vec3::ONE, Vec3::ONE, CLAMP).is_err());
    }
    let snapshot = source.map(vec_bits);
    solve_two_bone(source[0], source[1], source[2], source[3], source[4], REJECT).unwrap();
    assert_eq!(source.map(vec_bits), snapshot);
}

#[test]
fn current_tip_and_current_pole_recover_exact_input_bits_even_at_extreme_length_ratios() {
    for (root, middle, tip) in [
        (Vec3::new(-0.0, 0.0, -0.0), Vec3::new(1.0, -0.0, 0.0), Vec3::new(2.0, 0.0, 0.0)),
        (Vec3::ZERO, Vec3::new(1e-20, 0.0, 0.0), Vec3::new(1e20, 0.0, 0.0)),
        (Vec3::new(1e20, 0.0, 0.0), Vec3::ZERO, Vec3::new(1e-20, 0.0, 0.0)),
    ] {
        let solved = solve_two_bone(root, middle, tip, tip, middle, REJECT).unwrap();
        assert_eq!(vec_bits(solved.middle), vec_bits(middle));
        assert_eq!(vec_bits(solved.tip), vec_bits(tip));
        assert_eq!(quat_bits(solved.root_rotation), quat_bits(Quat::IDENTITY));
        assert_eq!(quat_bits(solved.middle_rotation), quat_bits(Quat::IDENTITY));
        assert!(!solved.clamped);
    }
}

#[test]
fn satisfied_collinear_inner_boundary_keeps_the_measured_tiny_distal_bone_for_any_pole() {
    let root = Vec3::new(1e20, 0.0, 0.0);
    let middle = Vec3::ZERO;
    let tip = Vec3::new(1e-20, 0.0, 0.0);
    for pole in [middle, Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 2.0)] {
        let solved = solve_two_bone(root, middle, tip, tip, pole, REJECT).unwrap();
        assert_eq!(vec_bits(solved.middle), vec_bits(middle));
        assert_eq!(vec_bits(solved.tip), vec_bits(tip));
        assert_eq!(quat_bits(solved.root_rotation), quat_bits(Quat::IDENTITY));
        assert_eq!(quat_bits(solved.middle_rotation), quat_bits(Quat::IDENTITY));
        assert_lengths(root, &solved, 0.0);
        fk(root, middle, tip, &solved, 0.0);
        assert_eq!(solved.residual, 0.0);
        assert!(!solved.clamped);
    }
}

#[test]
fn small_nonzero_segments_are_not_rejected_by_an_absolute_epsilon() {
    for unit in [f32::MIN_POSITIVE, f32::from_bits(1)] {
        let root = Vec3::ZERO;
        let middle = Vec3::new(unit, 0.0, 0.0);
        let tip = Vec3::new(2.0 * unit, 0.0, 0.0);
        let solved =
            solve_two_bone(root, middle, tip, Vec3::new(unit, unit, 0.0), Vec3::new(-unit, unit, 0.0), REJECT).unwrap();
        assert_lengths(root, &solved, 1e-6);
        near(solved.middle, Vec3::new(0.0, unit, 0.0), f64::from(unit) * 1e-6);
        fk(root, middle, tip, &solved, f64::from(unit) * 1e-6);
    }
}

#[test]
fn finite_input_with_an_unrepresentable_output_fails_instead_of_stretching_or_overflowing() {
    let extreme = f32::MAX;
    let failure = solve_two_bone(
        Vec3::new(extreme, 0.0, 0.0),
        Vec3::new(-extreme, 0.0, 0.0),
        Vec3::new(extreme, 0.0, 0.0),
        Vec3::new(extreme, 0.0, 0.0),
        Vec3::new(extreme, extreme, 0.0),
        REJECT,
    )
    .unwrap_err();
    assert!(failure.contains("finite f32 position range"), "{failure}");
    let root = Vec3::new(1e8, 1e8, 0.0);
    let failure = solve_two_bone(
        root,
        root + Vec3::new(8.0, 0.0, 0.0),
        root + Vec3::new(16.0, 0.0, 0.0),
        root + Vec3::new(8.0, 8.0, 0.0),
        root + Vec3::new(0.0, 0.0, 8.0),
        REJECT,
    )
    .unwrap_err();
    assert!(failure.contains("cannot preserve segment lengths"), "{failure}");
}

#[test]
fn spatial_targets_execute_correct_fk_and_detect_the_wrong_rotation_order() {
    let root = Vec3::new(0.4, -0.2, 1.0);
    let middle = Vec3::new(1.1, 0.5, 1.3);
    let tip = Vec3::new(1.6, 1.3, 0.8);
    let mut distinguishable_wrong_orders = 0;
    for index in 0..80 {
        let phase = index as f32 * 0.731;
        let axis = Vec3::new(phase.cos(), (phase * 0.43).sin(), (phase * 0.81).cos()).normalize();
        let target = root + axis.scale(0.3 + (index % 17) as f32 * 0.095);
        let pole = root + Vec3::new((phase * 0.23).sin(), 1.0, (phase * 0.91).cos());
        let solved = solve_two_bone(root, middle, tip, target, pole, REJECT).unwrap();
        assert_lengths(root, &solved, 1e-6);
        fk(root, middle, tip, &solved, 2e-6);
        let wrong = solved.middle
            + solved.root_rotation.to_mat3().mul_vec(solved.middle_rotation.to_mat3().mul_vec(tip - middle));
        if distance(wrong, target) > 0.1 {
            distinguishable_wrong_orders += 1;
        }
    }
    assert!(distinguishable_wrong_orders > 60);
}

fn vec_bits(point: Vec3) -> [u32; 3] {
    [point.x.to_bits(), point.y.to_bits(), point.z.to_bits()]
}

fn quat_bits(rotation: Quat) -> [u32; 4] {
    [rotation.x.to_bits(), rotation.y.to_bits(), rotation.z.to_bits(), rotation.w.to_bits()]
}
