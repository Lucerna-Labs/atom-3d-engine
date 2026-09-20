//! Analytic acceptance of fixed-topology deformation, including a deliberate LBS
//! collapse. These mechanisms do not establish anatomical or film-production quality.
use mm3e_kit::{
    deform::{
        deform, BlendShape, Deformation, Influence, SkinningMethod, MAX_BLEND_SHAPES, MAX_DEFORM_JOINTS,
        MAX_DEFORM_VERTICES, MAX_VERTEX_INFLUENCES,
    },
    surface::TriangleSurface,
    Mat3, Quat, Transform, Vec3,
};

const METHODS: [SkinningMethod; 2] = [SkinningMethod::LinearBlend, SkinningMethod::DualQuaternion];

fn rest() -> Vec<Vec3> {
    vec![Vec3::new(1.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0)]
}

fn rows(count: usize, influences: &[(u32, f32)]) -> Vec<Vec<Influence>> {
    vec![influences.iter().map(|&(joint, weight)| Influence { joint, weight }).collect(); count]
}

fn near(actual: Vec3, expected: Vec3, tolerance: f32) {
    assert!((actual - expected).length() < tolerance, "actual {actual:?}, expected {expected:?}");
}

fn bits(points: &[Vec3]) -> Vec<[u32; 3]> {
    points.iter().map(|point| [point.x.to_bits(), point.y.to_bits(), point.z.to_bits()]).collect()
}

fn quarter_turn() -> Mat3 {
    Mat3::from_cols(Vec3::new(0.0, 1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0))
}

#[test]
fn weighted_hinge_matches_analytic_linear_and_dual_quaternion_positions() {
    // Root identity, child turns 90 degrees about world-rest pivot (1,0,0).
    let positions = rest();
    let palette = [Transform::IDENTITY, Transform::new(Vec3::new(1.0, -1.0, 0.0), quarter_turn(), 1.0)];
    let weights =
        vec![rows(1, &[(0, 1.0)]).remove(0), rows(1, &[(0, 0.5), (1, 0.5)]).remove(0), rows(1, &[(1, 1.0)]).remove(0)];
    let linear = deform(&positions, &weights, &[], &[], &palette, SkinningMethod::LinearBlend).unwrap();
    assert_eq!(linear.positions, vec![Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.5, 0.5, 0.0), Vec3::ZERO]);
    let dual = deform(&positions, &weights, &[], &[], &palette, SkinningMethod::DualQuaternion).unwrap();
    near(dual.positions[1], Vec3::new(1.0 + 0.5_f32.sqrt(), 0.5_f32.sqrt(), 0.0), 2e-6);
    assert_eq!(dual.positions[0], positions[0]);
    assert_eq!(dual.positions[2], Vec3::ZERO);
    assert_eq!(linear.bounds, (Vec3::ZERO, Vec3::new(1.5, 0.5, 0.0)));
    assert!((linear.max_displacement - 2.0_f64.sqrt()).abs() < 1e-12);
}

#[test]
fn single_influence_is_the_exact_rigid_transform_for_both_methods() {
    let positions = rest();
    let transform = Transform::new(Vec3::new(3.0, -2.0, 4.0), quarter_turn(), 1.0);
    let expected: Vec<_> = positions.iter().map(|&point| transform.to_world(point)).collect();
    for method in METHODS {
        let result = deform(&positions, &rows(3, &[(0, 1.0)]), &[], &[], &[transform], method).unwrap();
        assert_eq!(bits(&result.positions), bits(&expected));
    }
}

#[test]
fn identity_and_rest_recovery_retain_source_bits_including_signed_zero() {
    let positions = vec![Vec3::new(-0.0, 0.0, -0.0), Vec3::new(1e30, 1e-30, -2.0), Vec3::new(-0.0, 1.0, 2.0)];
    let source = bits(&positions);
    let shapes = [BlendShape { deltas: vec![Vec3::ONE; 3] }];
    let weights = rows(3, &[(0, 0.1), (1, 0.2), (2, 0.7)]);
    for method in METHODS {
        let moving = [Transform::at(Vec3::ONE); 3];
        deform(&positions, &weights, &shapes, &[0.1], &moving, method).unwrap();
        let result = deform(&positions, &weights, &shapes, &[-0.0], &[Transform::IDENTITY; 3], method).unwrap();
        assert_eq!(bits(&result.positions), source);
        assert_eq!(result.max_displacement, 0.0);
        let pure_morph_rest = deform(&positions, &[], &shapes, &[0.0], &[], method).unwrap();
        assert_eq!(bits(&pure_morph_rest.positions), source);
    }
    assert_eq!(bits(&positions), source);
}

#[test]
fn blend_shapes_are_added_before_bone_rotation_and_support_signed_weights() {
    let positions = rest();
    let shapes = [
        BlendShape { deltas: vec![Vec3::new(1.0, 0.0, 0.0); 3] },
        BlendShape { deltas: vec![Vec3::new(0.0, 0.0, 2.0); 3] },
    ];
    let transform = Transform::new(Vec3::new(3.0, 0.0, 0.0), quarter_turn(), 1.0);
    for method in METHODS {
        // Two identical influences exercise the actual blended paths, not only the single-joint shortcut.
        let result = deform(
            &positions,
            &rows(3, &[(0, 0.25), (1, 0.75)]),
            &shapes,
            &[0.5, -0.25],
            &[transform, transform],
            method,
        )
        .unwrap();
        near(result.positions[0], Vec3::new(3.0, 1.5, -0.5), 1e-6);
        let after_skinning = transform.to_world(positions[0]) + Vec3::new(0.5, 0.0, -0.5);
        assert!((result.positions[0] - after_skinning).length() > 0.7);
        let morph_only = deform(&positions, &[], &shapes, &[0.5, -0.25], &[], method).unwrap();
        assert_eq!(morph_only.positions[0], Vec3::new(1.5, 0.0, -0.5));
    }
}

#[test]
fn twist_cylinder_quantifies_lbs_collapse_and_dual_quaternion_radius_retention() {
    let segments = 24;
    let rings = 5;
    let mut positions = Vec::new();
    let mut weights = Vec::new();
    let mut triangles = Vec::new();
    for ring in 0..rings {
        let t = ring as f32 / (rings - 1) as f32;
        for segment in 0..segments {
            let angle = std::f32::consts::TAU * segment as f32 / segments as f32;
            positions.push(Vec3::new(angle.cos(), t * 2.0, angle.sin()));
            weights.push(if ring == 0 {
                rows(1, &[(0, 1.0)]).remove(0)
            } else if ring == rings - 1 {
                rows(1, &[(1, 1.0)]).remove(0)
            } else {
                rows(1, &[(0, 1.0 - t), (1, t)]).remove(0)
            });
            if ring > 0 {
                let a = ((ring - 1) * segments + segment) as u32;
                let b = ((ring - 1) * segments + (segment + 1) % segments) as u32;
                let c = (ring * segments + segment) as u32;
                let d = (ring * segments + (segment + 1) % segments) as u32;
                triangles.extend([[a, c, b], [b, c, d]]);
            }
        }
    }
    let source_positions = bits(&positions);
    let source_triangles = triangles.clone();
    // Exact opposed +/-90 degree Y rotations: the middle LBS ring collapses onto the axis.
    let plus = Mat3::from_cols(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let minus = plus.transpose();
    let palette = [Transform::IDENTITY.rotated(plus), Transform::IDENTITY.rotated(minus)];
    let linear = deform(&positions, &weights, &[], &[], &palette, SkinningMethod::LinearBlend).unwrap();
    let dual = deform(&positions, &weights, &[], &[], &palette, SkinningMethod::DualQuaternion).unwrap();
    let middle = (rings / 2) * segments;
    let radius = |point: Vec3| f64::from(point.x).hypot(f64::from(point.z));
    let linear_max = linear.positions[middle..middle + segments].iter().map(|&p| radius(p)).fold(0.0, f64::max);
    let dual_min = dual.positions[middle..middle + segments].iter().map(|&p| radius(p)).fold(f64::INFINITY, f64::min);
    assert_eq!(linear_max, 0.0, "analytic LBS middle ring must collapse completely");
    assert!(dual_min > 0.999999, "DQS middle-ring radius {dual_min}");
    for (&source, &posed) in positions.iter().zip(&dual.positions) {
        assert!((radius(source) - radius(posed)).abs() < 2e-7);
        assert_eq!(source.y, posed.y);
    }
    // The unchanged triangle indices remain directly renderable with DQS. Deliberate
    // LBS collapse is rejected by the actual surface consumer, not silently repaired.
    TriangleSurface::new(dual.positions.clone(), triangles.clone(), 0.01).unwrap();
    assert!(TriangleSurface::new(linear.positions.clone(), triangles.clone(), 0.01).is_err());
    assert_eq!(bits(&positions), source_positions);
    assert_eq!(triangles, source_triangles);
    assert_eq!(dual.positions.len(), positions.len());
}

#[test]
fn antipodal_authored_quaternions_and_reordered_weights_give_equivalent_poses() {
    let positions = rest();
    let a = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 179.0_f32.to_radians());
    let b = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), -179.0_f32.to_radians());
    let negative = Quat { x: -b.x, y: -b.y, z: -b.z, w: -b.w };
    let make = |b: Quat| {
        [
            Transform::new(Vec3::new(3.0, 1.0, -2.0), a.to_mat3(), 1.0),
            Transform::new(Vec3::new(-2.0, 2.0, 1.0), b.to_mat3(), 1.0),
        ]
    };
    let reference =
        deform(&positions, &rows(3, &[(0, 0.5), (1, 0.5)]), &[], &[], &make(b), SkinningMethod::DualQuaternion)
            .unwrap();
    let flipped =
        deform(&positions, &rows(3, &[(0, 0.5), (1, 0.5)]), &[], &[], &make(negative), SkinningMethod::DualQuaternion)
            .unwrap();
    let reordered =
        deform(&positions, &rows(3, &[(1, 0.5), (0, 0.5)]), &[], &[], &make(b), SkinningMethod::DualQuaternion)
            .unwrap();
    assert_eq!(reference, flipped);
    assert_eq!(reference, reordered);
}

#[test]
fn malformed_weights_are_rejected_without_silent_normalization() {
    let positions = rest();
    for method in METHODS {
        for row in [
            vec![],
            vec![(0, 0.0)],
            vec![(0, -1.0)],
            vec![(0, f32::NAN)],
            vec![(0, f32::INFINITY)],
            vec![(0, 0.4), (1, 0.4)],
            vec![(0, 0.6), (1, 0.5)],
            vec![(0, 0.5), (0, 0.5)],
            vec![(2, 1.0)],
        ] {
            assert!(
                deform(&positions, &rows(3, &row), &[], &[], &[Transform::IDENTITY; 2], method).is_err(),
                "accepted malformed {row:?}"
            );
        }
        // Genuine f32 sum roundoff receives the documented canonical normalization.
        let rounded = rows(3, &[(0, 0.1), (1, 0.2), (2, 0.7)]);
        let result = deform(&positions, &rounded, &[], &[], &[Transform::at(Vec3::ONE); 3], method).unwrap();
        near(result.positions[0], positions[0] + Vec3::ONE, 1e-6);
    }
}

#[test]
fn malformed_cardinalities_nonfinite_data_and_budget_excess_are_rejected() {
    let positions = rest();
    let weights = rows(3, &[(0, 1.0)]);
    let palette = [Transform::IDENTITY];
    let run = |p: &[Vec3], w: &[Vec<Influence>], shapes: &[BlendShape], values: &[f32], joints: &[Transform]| {
        deform(p, w, shapes, values, joints, SkinningMethod::LinearBlend)
    };
    assert!(run(&positions[..2], &weights[..2], &[], &[], &palette).is_err());
    assert!(run(&positions, &weights[..2], &[], &[], &palette).is_err());
    assert!(run(&positions, &[], &[], &[], &palette).is_err());
    assert!(run(&positions, &weights, &[], &[], &[]).is_err());
    assert!(run(&vec![Vec3::ZERO; MAX_DEFORM_VERTICES + 1], &[], &[], &[], &[]).is_err());
    assert!(run(&positions, &weights, &[], &[], &vec![Transform::IDENTITY; MAX_DEFORM_JOINTS + 1]).is_err());
    let too_many: Vec<_> = (0..=MAX_VERTEX_INFLUENCES as u32).map(|joint| (joint, 1.0 / 9.0)).collect();
    assert!(run(&positions, &rows(3, &too_many), &[], &[], &[Transform::IDENTITY; 9]).is_err());
    let valid_shape = BlendShape { deltas: vec![Vec3::ONE; 3] };
    assert!(run(&positions, &weights, std::slice::from_ref(&valid_shape), &[], &palette).is_err());
    assert!(run(
        &positions,
        &weights,
        &vec![valid_shape.clone(); MAX_BLEND_SHAPES + 1],
        &vec![0.0; MAX_BLEND_SHAPES + 1],
        &palette
    )
    .is_err());
    assert!(run(&positions, &weights, &[BlendShape { deltas: vec![Vec3::ONE; 2] }], &[0.0], &palette).is_err());
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut broken = positions.clone();
        broken[0].x = invalid;
        assert!(run(&broken, &weights, &[], &[], &palette).is_err());
        let mut shape = valid_shape.clone();
        shape.deltas[1].y = invalid;
        assert!(run(&positions, &weights, &[shape], &[0.0], &palette).is_err());
        assert!(run(&positions, &weights, std::slice::from_ref(&valid_shape), &[invalid], &palette).is_err());
    }
    let maxed = vec![Vec3::splat(f32::MAX); 3];
    assert!(run(&maxed, &weights, &[], &[], &[Transform::IDENTITY.scaled(2.0)]).is_err());
    assert!(run(&maxed, &weights, &[valid_shape], &[f32::MAX], &palette).is_err());
}

#[test]
fn proper_transform_and_explicit_scale_contract_is_enforced() {
    let positions = rest();
    let weights = rows(3, &[(0, 1.0)]);
    for method in METHODS {
        for invalid in [
            Transform::IDENTITY.scaled(0.0),
            Transform::IDENTITY.scaled(-1.0),
            Transform::IDENTITY.scaled(f32::NAN),
            Transform::at(Vec3::splat(f32::INFINITY)),
            Transform::IDENTITY.rotated(Mat3::from_cols(
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            )),
            Transform::IDENTITY.rotated(Mat3::from_cols(
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.5, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            )),
            Transform::IDENTITY.rotated(Mat3::from_cols(Vec3::splat(f32::NAN), Vec3::ONE, Vec3::ONE)),
        ] {
            assert!(deform(&positions, &weights, &[], &[], &[invalid], method).is_err());
        }
    }
    let scaled = [Transform::IDENTITY.scaled(2.0)];
    let result = deform(&positions, &weights, &[], &[], &scaled, SkinningMethod::LinearBlend).unwrap();
    assert_eq!(result.positions[0], Vec3::new(2.0, 0.0, 0.0));
    assert!(deform(&positions, &weights, &[], &[], &scaled, SkinningMethod::DualQuaternion)
        .unwrap_err()
        .contains("unit scale"));
    // No implicit scale tolerance or bypass for a currently unused palette joint.
    for scale in [f32::from_bits(1.0_f32.to_bits() + 1), 0.99999, 3.0] {
        let palette = [Transform::IDENTITY, Transform::IDENTITY.scaled(scale)];
        assert!(deform(&positions, &weights, &[], &[], &palette, SkinningMethod::DualQuaternion).is_err());
    }
    // Weighted positive uniform scales are supported by linear skinning.
    let result = deform(
        &positions,
        &rows(3, &[(0, 0.25), (1, 0.75)]),
        &[],
        &[],
        &[Transform::IDENTITY.scaled(2.0), Transform::IDENTITY.scaled(4.0)],
        SkinningMethod::LinearBlend,
    )
    .unwrap();
    assert_eq!(result.positions[0], Vec3::new(3.5, 0.0, 0.0));
}

#[test]
fn parallel_replay_is_deterministic_source_is_immutable_and_failures_do_not_poison_recovery() {
    let positions = rest();
    let source = bits(&positions);
    let weights = rows(3, &[(0, 0.3), (1, 0.7)]);
    let shapes = [BlendShape { deltas: vec![Vec3::new(0.2, -0.1, 0.0); 3] }];
    let source_shapes = shapes.clone();
    let source_weights = weights.clone();
    for method in METHODS {
        let evaluate = |time: f32| -> Result<Deformation, String> {
            let palette = [
                Transform::IDENTITY,
                Transform::new(Vec3::new(time, 0.0, 0.0), Mat3::from_euler(time, -time * 2.0, 0.3), 1.0),
            ];
            deform(&positions, &weights, &shapes, &[time * 0.1], &palette, method)
        };
        let expected = evaluate(0.5).unwrap();
        for time in [0.0, 1.0, 0.2, -0.4, 0.5] {
            evaluate(time).unwrap();
        }
        assert!(evaluate(f32::NAN).is_err());
        assert_eq!(expected, evaluate(0.5).unwrap());
        std::thread::scope(|scope| {
            let jobs: Vec<_> = (0..8).map(|_| scope.spawn(|| evaluate(0.5).unwrap())).collect();
            for job in jobs {
                assert_eq!(expected, job.join().unwrap());
            }
        });
    }
    assert_eq!(bits(&positions), source);
    assert_eq!(shapes, source_shapes);
    assert_eq!(weights, source_weights);
}

#[test]
fn maximum_supported_vertices_and_influences_have_finite_exact_identity_results() {
    let positions = vec![Vec3::new(1.0, -0.0, 2.0); MAX_DEFORM_VERTICES];
    let influences: Vec<_> = (0..MAX_VERTEX_INFLUENCES as u32).map(|joint| (joint, 0.125)).collect();
    let weights = rows(MAX_DEFORM_VERTICES, &influences);
    let palette = vec![Transform::IDENTITY; MAX_DEFORM_JOINTS];
    for method in METHODS {
        let result = deform(&positions, &weights, &[], &[], &palette, method).unwrap();
        assert_eq!(bits(&result.positions), bits(&positions));
        assert_eq!(result.max_displacement, 0.0);
    }
}
