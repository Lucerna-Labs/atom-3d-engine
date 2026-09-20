use mm3e_kit::{
    limited_ik::{self, JointPose, Request, Solution},
    rotation_limit::RotationLimit,
    Quat, Transform, Vec3,
};

fn hinge(axis: Vec3, min: f64, max: f64) -> Option<RotationLimit> {
    Some(RotationLimit::Hinge { axis, min, max })
}
fn base() -> Request {
    Request {
        parent: Transform::IDENTITY,
        joints: [0.0, 1.0, 2.0].map(|x| JointPose {
            pivot: Vec3::new(x, 0.0, 0.0),
            translation: Vec3::ZERO,
            scale: 1.0,
            rotation: Quat::IDENTITY,
        }),
        limits: [None; 3],
        analytic_seed: [Quat::IDENTITY; 2],
        middle_target: Vec3::new(1.0, 0.0, 0.0),
        tip_target: Vec3::new(2.0, 0.0, 0.0),
        tip_orientation: None,
        tolerance_m: 1e-4,
        orientation_tolerance_rad: 1e-3,
        max_evaluations: 16_384,
    }
}

/// Independent FK expanded from world-rest pivots; it does not call the solver
/// or Transform::compose/around_pivot to reproduce their translation handling.
fn independent(request: &Request, rotations: [Quat; 3]) -> [Vec3; 3] {
    let [root, middle, tip] = request.joints;
    let root_point = request.parent.to_world(root.pivot + root.translation);
    let first = rotations[0].to_mat3().mul_vec((middle.pivot + middle.translation - root.pivot).scale(root.scale));
    let middle_point = root_point + request.parent.rot.mul_vec(first.scale(request.parent.scale));
    let distal = rotations[1].to_mat3().mul_vec((tip.pivot + tip.translation - middle.pivot).scale(middle.scale));
    let distal = rotations[0].to_mat3().mul_vec(distal.scale(root.scale));
    let tip_point = middle_point + request.parent.rot.mul_vec(distal.scale(request.parent.scale));
    [root_point, middle_point, tip_point]
}
fn near(a: Vec3, b: Vec3, tolerance: f32) {
    assert!((a - b).length() <= tolerance, "{a:?} != {b:?}");
}
fn check(request: &Request, solution: &Solution) {
    let expected = independent(request, solution.rotations);
    for (actual, want) in [solution.root, solution.middle, solution.tip].into_iter().zip(expected) {
        near(actual, want, 2e-5);
    }
    let original = independent(request, request.joints.map(|j| j.rotation));
    near(solution.root, original[0], request.tolerance_m as f32);
    for i in 0..2 {
        let old = (original[i + 1] - original[i]).length();
        let new = (expected[i + 1] - expected[i]).length();
        assert!((old - new).abs() < 2e-5, "segment {i}: {old} -> {new}");
    }
    for (limit, rotation) in request.limits.into_iter().zip(solution.rotations) {
        if let Some(limit) = limit {
            assert!(!limit.project(rotation).unwrap().violated, "{limit:?}: {rotation:?}");
        }
    }
    assert!(solution.evaluations <= request.max_evaluations);
}

#[test]
fn constructive_current_and_analytic_hinge_poses_preserve_exact_cases() {
    let mut request = base();
    request.limits = [hinge(Vec3::new(0.0, 0.0, 1.0), -1.0, 1.0), hinge(Vec3::new(0.0, 0.0, 1.0), 0.0, 2.0), None];
    let current = limited_ik::solve(&request).unwrap();
    assert!(current.converged);
    assert_eq!(current.evaluations, 1);
    for q in current.rotations {
        assert_eq!([q.x, q.y, q.z, q.w], [0.0, 0.0, 0.0, 1.0]);
    }
    request.analytic_seed = [Quat::IDENTITY, Quat::from_euler(0.0, 0.0, std::f32::consts::FRAC_PI_2)];
    request.tip_target = Vec3::new(1.0, 1.0, 0.0);
    let result = limited_ik::solve(&request).unwrap();
    assert!(result.converged);
    assert_eq!(result.evaluations, 2);
    check(&request, &result);
    near(result.tip, request.tip_target, 1e-5);
}

#[test]
fn projected_search_finds_hinged_target_and_requested_bend_from_poor_seeds() {
    let mut request = base();
    request.limits = [hinge(Vec3::new(0.0, 0.0, 1.0), -0.8, 0.8), hinge(Vec3::new(0.0, 0.0, 1.0), 0.0, 2.0), None];
    let desired = [Quat::from_euler(0.0, 0.0, 0.35), Quat::from_euler(0.0, 0.0, 0.8), Quat::IDENTITY];
    let goal = independent(&request, desired);
    request.middle_target = goal[1];
    request.tip_target = goal[2];
    request.analytic_seed = [Quat::from_euler(0.0, 0.0, 2.0), Quat::from_euler(0.0, 0.0, -1.0)];
    let result = limited_ik::solve(&request).unwrap();
    assert!(result.converged, "{result:?}");
    assert!(result.evaluations > 2);
    check(&request, &result);
}

#[test]
fn radial_reach_does_not_hide_limits_or_an_infeasible_pole() {
    let mut request = base();
    request.limits = [hinge(Vec3::new(0.0, 0.0, 1.0), -0.1, 0.1), hinge(Vec3::new(0.0, 0.0, 1.0), 0.0, 0.1), None];
    request.middle_target = Vec3::new(0.0, 1.0, 0.0);
    request.tip_target = Vec3::new(0.0, 2.0, 0.0);
    request.max_evaluations = 200;
    let result = limited_ik::solve(&request).unwrap();
    assert!(!result.converged);
    assert!(result.tip_error > 1.0);
    check(&request, &result);
    request.limits = [
        hinge(Vec3::new(0.0, 0.0, 1.0), -std::f64::consts::PI, std::f64::consts::PI),
        hinge(Vec3::new(0.0, 0.0, 1.0), 0.0, std::f64::consts::PI),
        None,
    ];
    request.tip_target = Vec3::new(1.0, 1.0, 0.0);
    request.analytic_seed = [Quat::IDENTITY, Quat::from_euler(0.0, 0.0, std::f32::consts::FRAC_PI_2)];
    let result = limited_ik::solve(&request).unwrap();
    assert!(!result.converged);
    assert!(result.middle_error > 0.01 || result.tip_error > 0.01);
    check(&request, &result);
}

#[test]
fn world_rest_pivots_translations_scales_and_parent_are_frozen() {
    let mut request = base();
    request.parent = Transform::new(Vec3::new(-2.0, 0.5, 1.0), Quat::from_euler(0.1, 0.3, 0.5).to_mat3(), 1.7);
    request.joints = [
        JointPose {
            pivot: Vec3::new(2.0, 3.0, 1.0),
            translation: Vec3::new(0.2, -0.1, 0.3),
            scale: 1.25,
            rotation: Quat::from_euler(0.0, 0.0, 0.2),
        },
        JointPose {
            pivot: Vec3::new(3.0, 3.0, 1.0),
            translation: Vec3::new(0.1, 0.2, 0.0),
            scale: 0.75,
            rotation: Quat::from_euler(0.0, 0.1, 0.0),
        },
        JointPose {
            pivot: Vec3::new(4.0, 3.0, 1.0),
            translation: Vec3::new(0.0, 0.1, 0.2),
            scale: 0.8,
            rotation: Quat::from_euler(0.0, 0.0, 0.4),
        },
    ];
    request.analytic_seed = [Quat::from_euler(0.0, 0.4, 0.0), Quat::from_euler(-0.2, 0.0, 0.0)];
    let target =
        independent(&request, [request.analytic_seed[0], request.analytic_seed[1], request.joints[2].rotation]);
    request.middle_target = target[1];
    request.tip_target = target[2];
    let result = limited_ik::solve(&request).unwrap();
    assert!(result.converged, "{result:?}");
    check(&request, &result);
    let unchanged = request.joints[2].rotation;
    let actual = result.rotations[2];
    assert_eq!([actual.x, actual.y, actual.z, actual.w], [unchanged.x, unchanged.y, unchanged.z, unchanged.w]);
}

#[test]
fn non_cartesian_bone_twist_meets_tip_orientation_with_a_locked_tip() {
    let mut request = base();
    let axis = Vec3::new(1.0, 1.0, 0.0).normalize();
    request.joints[1].pivot = axis;
    request.joints[2].pivot = axis.scale(2.0);
    request.middle_target = axis;
    request.tip_target = axis.scale(2.0);
    request.limits = [
        hinge(Vec3::new(0.0, 0.0, 1.0), 0.0, 0.0),
        hinge(axis, 0.0, std::f64::consts::FRAC_PI_2),
        hinge(Vec3::new(0.0, 0.0, 1.0), 0.0, 0.0),
    ];
    request.tip_orientation = Some(Quat::from_axis_angle(axis, std::f32::consts::FRAC_PI_3));
    request.tolerance_m = 1e-5;
    request.orientation_tolerance_rad = 1e-4;
    let result = limited_ik::solve(&request).unwrap();
    assert!(result.converged, "{result:?}");
    assert!(result.orientation_error.unwrap() < 1e-4);
    check(&request, &result);
    let tip = result.rotations[2];
    assert_eq!([tip.x, tip.y, tip.z, tip.w], [0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn budgets_are_deterministic_and_malformed_inputs_fail_explicitly() {
    let mut request = base();
    request.tip_target = Vec3::new(0.0, 2.0, 0.0);
    request.max_evaluations = 1;
    let first = limited_ik::solve(&request).unwrap();
    let second = limited_ik::solve(&request).unwrap();
    assert_eq!(format!("{first:?}"), format!("{second:?}"));
    assert!(!first.converged);
    assert!(first.budget_exhausted);
    assert_eq!(first.evaluations, 1);
    check(&request, &first);
    for index in 0..6 {
        let mut bad = base();
        match index {
            0 => bad.max_evaluations = 0,
            1 => bad.parent.rot.cols[0].x = 2.0,
            2 => bad.joints[1].scale = 0.0,
            3 => bad.tip_target.x = f32::NAN,
            4 => bad.analytic_seed[0] = Quat { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            _ => {
                bad.joints[2].rotation = Quat::from_euler(1.0, 0.0, 0.0);
                bad.limits[2] = hinge(Vec3::new(0.0, 0.0, 1.0), 0.0, 0.0);
            }
        }
        assert!(limited_ik::solve(&bad).is_err(), "case {index}");
    }
}
