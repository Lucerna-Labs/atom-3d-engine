use mm3e_kit::cloth::{measure_cloth_self_contact, Cloth, ClothContact, ClothSettings};
use mm3e_kit::Vec3;

fn grid(n: u32, spacing: f32, height: f32, pin_corners: bool, settings: ClothSettings) -> Cloth {
    let mut positions = Vec::new();
    let mut triangles = Vec::new();
    let mut masses = Vec::new();
    for z in 0..n {
        for x in 0..n {
            positions.push(Vec3::new(
                (x as f32 - (n - 1) as f32 * 0.5) * spacing,
                height,
                (z as f32 - (n - 1) as f32 * 0.5) * spacing,
            ));
            masses.push(if pin_corners && (x == 0 || x == n - 1) && (z == 0 || z == n - 1) { 0.0 } else { 1.0 });
            if x + 1 < n && z + 1 < n {
                let i = z * n + x;
                triangles.push([i, i + n, i + 1]);
                triangles.push([i + 1, i + n, i + n + 1]);
            }
        }
    }
    Cloth::new(positions, triangles, masses, settings).unwrap()
}

#[test]
fn cloth_sags_under_gravity_while_anchors_stay_exact_and_edges_resist_stretch() {
    let settings = ClothSettings { iterations: 24, bend_compliance: 1e-2, ..Default::default() };
    let mut cloth = grid(7, 0.16, 1.0, true, settings);
    let mut anchors = cloth.positions().to_vec();
    // Move the four attachment points inward to provide known geometric slack.
    // A flat inextensible sheet held at its original four corners is taut.
    for i in [0, 6, 42, 48] {
        anchors[i].x *= 0.9;
        anchors[i].z *= 0.9;
        cloth.set_pin_position(i as u32, anchors[i]).unwrap();
    }
    let mut report = None;
    for _ in 0..180 {
        report = Some(cloth.step(|_| None).unwrap());
    }
    let report = report.unwrap();
    let center_y = cloth.positions()[24].y;
    assert!(center_y < 0.93 && center_y > 0.5, "actual gravitational sag: {center_y}");
    assert!(report.max_relative_edge_error < 0.03, "edge strain: {}", report.max_relative_edge_error);
    for i in [0, 6, 42, 48] {
        assert_eq!(cloth.positions()[i], anchors[i]);
        assert_eq!(cloth.velocities()[i], Vec3::ZERO);
    }
}

#[test]
fn unpinned_cloth_falls_and_settles_above_plane_at_configured_thickness() {
    let settings = ClothSettings { collision_thickness: 0.01, ..Default::default() };
    let mut cloth = grid(6, 0.2, 1.0, false, settings);
    let mut contacts = 0;
    for _ in 0..100 {
        let report = cloth.step(|p| Some(ClothContact { distance: p.y, normal: Vec3::new(0.0, 4.0, 0.0) })).unwrap();
        contacts += report.contact_projections;
        assert!(report.max_contact_penetration <= 1e-6);
    }
    assert!(contacts > 0);
    for p in cloth.positions() {
        assert!((p.y - 0.01).abs() <= 1e-6, "{p:?}");
    }
}

#[test]
fn cloth_drapes_over_sphere_with_vertex_contact_and_reports_actual_projection() {
    let settings = ClothSettings {
        iterations: 20,
        bend_compliance: 1.0,
        damping_per_second: 2.0,
        collision_thickness: 0.01,
        ..Default::default()
    };
    let source = grid(9, 0.12, 0.56, false, settings);
    let mut masses = vec![1.0; 81];
    // A real authored center attachment keeps the frictionless sheet on the
    // sphere; without it, a sheet is allowed to slide off under gravity.
    masses[40] = 0.0;
    let mut cloth =
        Cloth::new(source.rest_positions().to_vec(), source.triangles().to_vec(), masses, settings).unwrap();
    let mut contacts = 0;
    for _ in 0..180 {
        let report = cloth.step(|p| Some(ClothContact { distance: p.length() - 0.55, normal: p.normalize() })).unwrap();
        contacts += report.contact_projections;
        assert!(report.max_contact_penetration <= 2e-6);
    }
    assert!(contacts > 100);
    let center = cloth.positions()[40];
    let corner = cloth.positions()[0];
    assert!(center.y > 0.4 && center.y < 0.65, "center draped on sphere: {center:?}");
    assert!(corner.y < center.y - 0.1, "corner must sag around sphere: {corner:?}");
    for p in cloth.positions() {
        assert!(p.length() >= 0.56 - 2e-6, "vertex penetration: {p:?}");
    }
}

#[test]
fn bending_resists_a_hinge_fold_that_does_not_stretch_any_edge() {
    fn hinge(compliance: f32) -> Cloth {
        let points = vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.5, 1.0, 0.0), Vec3::new(0.5, -1.0, 0.0)];
        let settings = ClothSettings {
            gravity: Vec3::ZERO,
            damping_per_second: 5.0,
            bend_compliance: compliance,
            stretch_compliance: 0.0,
            ..Default::default()
        };
        let mut cloth = Cloth::new(points, vec![[0, 1, 2], [1, 0, 3]], vec![0.0, 0.0, 0.0, 1.0], settings).unwrap();
        let mut state = cloth.state().clone();
        // Rigid rotation of the free triangle about the common edge preserves
        // all edge lengths, isolating genuine angular bending from stretch.
        state.positions[3] = Vec3::new(0.5, -0.5, 3.0_f32.sqrt() * 0.5);
        cloth.restore_state(state).unwrap();
        cloth
    }
    let mut stiff = hinge(1e-4);
    let mut limp = hinge(1e8);
    for _ in 0..120 {
        stiff.step(|_| None).unwrap();
        limp.step(|_| None).unwrap();
    }
    assert!(stiff.positions()[3].z.abs() < 0.005, "{:?}", stiff.positions()[3]);
    assert!(limp.positions()[3].z > 0.8, "{:?}", limp.positions()[3]);
    assert_eq!(stiff.constraint_counts(), (5, 1));
}

#[test]
fn compliant_gravity_equilibrium_matches_hooke_law_at_two_timestep_sizes() {
    let compliance = 0.001;
    fn equilibrium(dt: f32, compliance: f32) -> f32 {
        let mut cloth = Cloth::new(
            vec![Vec3::new(-1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0)],
            vec![[0, 1, 2]],
            vec![0.0, 0.0, 1.0],
            ClothSettings {
                fixed_dt: dt,
                substeps: 1,
                iterations: 48,
                stretch_compliance: compliance,
                damping_per_second: 5.0,
                ..Default::default()
            },
        )
        .unwrap();
        for _ in 0..(10.0 / dt).round() as usize {
            cloth.step(|_| None).unwrap();
        }
        -cloth.positions()[2].y
    }
    let slow = equilibrium(1.0 / 30.0, compliance);
    let fast = equilibrium(1.0 / 120.0, compliance);
    assert!(
        (slow - fast).abs() < 0.0001,
        "equilibrium must not change material stiffness with timestep: {slow}, {fast}"
    );
    for height in [slow, fast] {
        let length = (1.0 + height * height).sqrt();
        let predicted_upward_force = 2.0 * (length - 2.0_f32.sqrt()) / compliance * height / length;
        assert!(
            (predicted_upward_force - 9.81).abs() < 0.04,
            "elastic restoring force should balance weight: {predicted_upward_force}"
        );
    }
}

#[test]
fn deterministic_replay_and_checkpoint_resume_match_every_vertex_and_velocity() {
    let mut first = grid(5, 0.2, 1.0, true, ClothSettings::default());
    let mut replay = first.clone();
    for _ in 0..30 {
        assert_eq!(first.step(|_| None).unwrap(), replay.step(|_| None).unwrap());
    }
    let mut restored = grid(5, 0.2, 1.0, true, ClothSettings::default());
    restored.restore_state(first.state().clone()).unwrap();
    for _ in 0..40 {
        assert_eq!(first.step(|_| None).unwrap(), restored.step(|_| None).unwrap());
        assert_eq!(first.state(), restored.state());
    }
}

#[test]
fn animated_pin_reaches_exact_target_and_moves_the_attached_fabric() {
    let mut cloth = grid(5, 0.2, 1.0, true, ClothSettings { gravity: Vec3::ZERO, ..Default::default() });
    let start = cloth.positions()[0];
    let target = start + Vec3::new(0.0, 0.2, 0.0);
    cloth.set_pin_position(0, target).unwrap();
    cloth.step(|_| None).unwrap();
    assert_eq!(cloth.positions()[0], target);
    assert!(cloth.positions()[1].y > 1.01);
    assert!(cloth.set_pin_position(1, target).is_err());
    assert!(cloth.set_pin_position(u32::MAX, target).is_err());
}

#[test]
fn pinned_contacts_are_reported_without_violating_attachment_constraints() {
    let mut cloth = grid(3, 0.2, -0.1, true, ClothSettings::default());
    let pinned = cloth.positions()[0];
    let report = cloth.step(|p| Some(ClothContact { distance: p.y, normal: Vec3::new(0.0, 1.0, 0.0) })).unwrap();
    assert_eq!(cloth.positions()[0], pinned);
    assert!(report.max_contact_penetration >= 0.103 - 1e-6);
}

#[test]
fn coincident_current_vertices_produce_finite_motion_and_do_not_rewrite_rest_topology() {
    let mut cloth = grid(3, 0.2, 1.0, false, ClothSettings::default());
    let rest = cloth.rest_positions().to_vec();
    let mut collapsed = cloth.state().clone();
    collapsed.positions.fill(Vec3::ZERO);
    cloth.restore_state(collapsed).unwrap();
    cloth.step(|_| None).unwrap();
    for p in cloth.positions().iter().chain(cloth.velocities()) {
        assert!(p.x.is_finite() && p.y.is_finite() && p.z.is_finite());
    }
    assert_eq!(cloth.rest_positions(), rest);
}

#[test]
fn malformed_contact_and_numeric_overflow_fail_atomically() {
    let mut cloth = grid(4, 0.2, 1.0, true, ClothSettings::default());
    let before = cloth.state().clone();
    let mut count = 0;
    let failed = cloth.step(|_| {
        count += 1;
        if count == 15 {
            Some(ClothContact { distance: f32::NAN, normal: Vec3::new(0.0, 1.0, 0.0) })
        } else {
            None
        }
    });
    assert!(failed.is_err());
    assert_eq!(cloth.state(), &before);
    assert!(cloth.step(|_| Some(ClothContact { distance: 0.0, normal: Vec3::ZERO })).is_err());
    assert_eq!(cloth.state(), &before);
    let mut overflow = before;
    overflow.completed_steps = u64::MAX;
    cloth.restore_state(overflow.clone()).unwrap();
    assert!(cloth.step(|_| None).is_err());
    assert_eq!(cloth.state(), &overflow);
}

#[test]
fn free_fall_follows_acceleration_without_a_rest_position_hold() {
    let settings = ClothSettings { gravity: Vec3::new(0.0, -9.81, 0.0), damping_per_second: 0.0, ..Default::default() };
    let mut cloth = grid(3, 0.2, 1.0, false, settings);
    let steps = 12;
    for _ in 0..steps {
        cloth.step(|_| None).unwrap();
    }
    let h = settings.fixed_dt / settings.substeps as f32;
    let substep_count = (steps * settings.substeps) as f32;
    let expected_y = 1.0 + settings.gravity.y * h * h * substep_count * (substep_count + 1.0) * 0.5;
    let expected_speed = settings.gravity.y * h * substep_count;
    for (p, v) in cloth.positions().iter().zip(cloth.velocities()) {
        assert!((p.y - expected_y).abs() < 2e-6, "{p:?}, expected {expected_y}");
        assert!((v.y - expected_speed).abs() < 2e-5, "{v:?}, expected {expected_speed}");
    }
}

#[test]
fn unrepresentable_acceleration_fails_without_committing_partial_simulation() {
    let settings = ClothSettings {
        fixed_dt: 1.0,
        substeps: 1,
        gravity: Vec3::new(f32::MAX, 0.0, 0.0),
        damping_per_second: 0.0,
        ..Default::default()
    };
    let mut cloth = grid(3, 0.2, 1.0, false, settings);
    let mut checkpoint = cloth.state().clone();
    checkpoint.velocities.fill(Vec3::new(f32::MAX, 0.0, 0.0));
    cloth.restore_state(checkpoint.clone()).unwrap();
    assert!(cloth.step(|_| None).is_err());
    assert_eq!(cloth.state(), &checkpoint);
}

#[test]
fn construction_rejects_invalid_geometry_masses_and_unbounded_work() {
    let rest = vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
    let make = |p, t, w, s| Cloth::new(p, t, w, s);
    assert!(make(rest.clone(), vec![[0, 1, 3]], vec![1.0; 3], ClothSettings::default()).is_err());
    assert!(make(rest.clone(), vec![[0, 1, 1]], vec![1.0; 3], ClothSettings::default()).is_err());
    assert!(make(rest.clone(), vec![[0, 1, 2], [2, 1, 0]], vec![1.0; 3], ClothSettings::default()).is_err());
    assert!(make(rest.clone(), vec![[0, 1, 2]], vec![-1.0; 3], ClothSettings::default()).is_err());
    assert!(make(rest.clone(), vec![[0, 1, 2]], vec![f32::NAN; 3], ClothSettings::default()).is_err());
    assert!(
        make(rest.clone(), vec![[0, 1, 2]], vec![1.0; 3], ClothSettings { substeps: 0, ..Default::default() }).is_err()
    );
    assert!(make(
        rest.clone(),
        vec![[0, 1, 2]],
        vec![1.0; 3],
        ClothSettings { fixed_dt: f32::NAN, ..Default::default() }
    )
    .is_err());
    let mut collinear = rest.clone();
    collinear[2] = Vec3::new(2.0, 0.0, 0.0);
    assert!(make(collinear, vec![[0, 1, 2]], vec![1.0; 3], ClothSettings::default()).is_err());
    let mut bad_winding = rest.clone();
    bad_winding.push(Vec3::new(0.0, -1.0, 0.0));
    assert!(make(bad_winding.clone(), vec![[0, 1, 2], [0, 1, 3]], vec![1.0; 4], ClothSettings::default()).is_err());
    bad_winding.push(Vec3::new(0.0, 0.0, 1.0));
    assert!(make(bad_winding, vec![[0, 1, 2], [1, 0, 3], [0, 1, 4]], vec![1.0; 5], ClothSettings::default()).is_err());
    let mut nonfinite = rest;
    nonfinite[1].x = f32::INFINITY;
    assert!(make(nonfinite, vec![[0, 1, 2]], vec![1.0; 3], ClothSettings::default()).is_err());
    let small = grid(10, 0.1, 1.0, false, ClothSettings::default());
    assert!(make(
        small.rest_positions().to_vec(),
        small.triangles().to_vec(),
        vec![1.0; 100],
        ClothSettings { substeps: 128, iterations: 256, contact_iterations: 32, ..Default::default() }
    )
    .is_err());
}

fn layered_triangles(settings: ClothSettings, upper_height: f32, lower_pinned: bool) -> Cloth {
    Cloth::new(
        vec![
            Vec3::new(-4.0, 0.0, -4.0),
            Vec3::new(4.0, 0.0, -4.0),
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(-0.2, upper_height, -0.2),
            Vec3::new(0.2, upper_height, -0.2),
            Vec3::new(0.0, upper_height, 0.2),
        ],
        vec![[0, 1, 2], [3, 4, 5]],
        if lower_pinned { vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0] } else { vec![1.0; 6] },
        settings,
    )
    .unwrap()
}

fn self_contact_settings() -> ClothSettings {
    ClothSettings {
        fixed_dt: 0.1,
        substeps: 1,
        iterations: 16,
        gravity: Vec3::ZERO,
        damping_per_second: 0.0,
        stretch_compliance: 0.0,
        self_collision: true,
        self_collision_thickness: 0.02,
        ..Default::default()
    }
}

#[test]
fn swept_vertex_triangle_contact_stops_layers_crossing_with_disabled_negative_control() {
    let settings = self_contact_settings();
    let mut enabled = layered_triangles(settings, 0.2, true);
    let mut disabled = layered_triangles(ClothSettings { self_collision: false, ..settings }, 0.2, true);
    for cloth in [&mut enabled, &mut disabled] {
        let mut state = cloth.state().clone();
        state.velocities[3..].fill(Vec3::new(0.0, -8.0, 0.0));
        cloth.restore_state(state).unwrap();
    }
    // The final predicted layer is 0.6 below the other layer: a discrete
    // distance<thickness test would entirely miss this crossing.
    let report = enabled.step(|_| None).unwrap();
    disabled.step(|_| None).unwrap();
    assert!(report.self_vertex_triangle_projections >= 3, "{report:?}");
    assert!(report.max_self_contact_penetration < 2e-6, "{report:?}");
    for p in &enabled.positions()[3..] {
        assert!(p.y >= 0.02 - 2e-6, "{p:?}");
    }
    for p in &disabled.positions()[3..] {
        assert!(p.y < -0.59, "disabled contact must permit crossing: {p:?}");
    }
    assert!(report.self_contact_candidates <= u64::from(settings.self_collision_max_candidates));
    assert!(report.self_contact_work <= settings.self_collision_max_work);
    assert_eq!(&enabled.positions()[..3], &enabled.rest_positions()[..3]);
    // Retain separation through the remaining downward approach momentum.
    for _ in 0..8 {
        let report = enabled.step(|_| None).unwrap();
        assert!(report.max_self_contact_penetration < 2e-6);
        assert!(enabled.positions()[3..].iter().all(|p| p.y >= 0.02 - 2e-6));
    }
}

#[test]
fn edge_edge_sweep_catches_intersections_that_all_vertex_triangle_tests_miss() {
    // Opposing triangle projections form a six-pointed star. No vertex lies
    // inside or near the other triangle, but six pairs of edge interiors cross.
    let settings = ClothSettings { iterations: 64, ..self_contact_settings() };
    let make = |settings| {
        Cloth::new(
            vec![
                Vec3::new(-1.0, 0.0, -0.5),
                Vec3::new(1.0, 0.0, -0.5),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(-1.0, 0.1, 0.5),
                Vec3::new(0.0, 0.1, -1.0),
                Vec3::new(1.0, 0.1, 0.5),
            ],
            vec![[0, 1, 2], [3, 4, 5]],
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            settings,
        )
        .unwrap()
    };
    let mut enabled = make(settings);
    let mut disabled = make(ClothSettings { self_collision: false, ..settings });
    for cloth in [&mut enabled, &mut disabled] {
        let mut state = cloth.state().clone();
        state.velocities[3..].fill(Vec3::new(0.0, -2.0, 0.0));
        cloth.restore_state(state).unwrap();
    }
    let report = enabled.step(|_| None).unwrap();
    disabled.step(|_| None).unwrap();
    assert_eq!(report.self_vertex_triangle_projections, 0, "vertex-only response cannot explain this result");
    assert!(report.self_edge_edge_projections > 0, "{report:?}");
    assert!(report.max_self_contact_penetration < 2e-6, "{report:?}");
    // A triangle may tilt and put vertices outside the opposing triangle below
    // its infinite plane. Measure the actual edge intersections independently.
    let mut intersections = 0;
    for [a, b] in [[0, 1], [1, 2], [2, 0]] {
        for [c, d] in [[3, 4], [4, 5], [5, 3]] {
            let p = enabled.positions()[a];
            let q = enabled.positions()[c];
            let r = enabled.positions()[b] - p;
            let s = enabled.positions()[d] - q;
            let cross = |u: Vec3, v: Vec3| u.x * v.z - u.z * v.x;
            let denominator = cross(r, s);
            if denominator.abs() < 1e-8 {
                continue;
            }
            let t = cross(q - p, s) / denominator;
            let u = cross(q - p, r) / denominator;
            if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
                intersections += 1;
                let height = q.y + u * s.y;
                assert!(height >= 0.02 - 2e-6, "edge crossing height: {height}");
            }
        }
    }
    assert_eq!(intersections, 6);
    assert!(disabled.positions()[3..].iter().all(|p| p.y < -0.099));
    assert_eq!(&enabled.positions()[..3], &enabled.rest_positions()[..3]);
}

#[test]
fn a_moving_pinned_layer_transfers_contact_reaction_to_the_free_triangle() {
    let settings = ClothSettings { stretch_compliance: 1e6, ..self_contact_settings() };
    let template = layered_triangles(settings, 0.1, false);
    let mut cloth = Cloth::new(
        template.rest_positions().to_vec(),
        template.triangles().to_vec(),
        vec![1.0, 2.0, 3.0, 0.0, 0.0, 0.0],
        settings,
    )
    .unwrap();
    let targets = cloth.positions()[3..].iter().map(|p| Vec3::new(p.x, -0.1, p.z)).collect::<Vec<_>>();
    for (i, &p) in targets.iter().enumerate() {
        cloth.set_pin_position(i as u32 + 3, p).unwrap();
    }
    let report = cloth.step(|_| None).unwrap();
    assert!(report.self_vertex_triangle_projections > 0, "{report:?}");
    assert_eq!(&cloth.positions()[3..], targets);
    assert!(
        cloth.positions()[..3].iter().all(|p| p.y < -0.03),
        "reaction must move free fabric: {:?}",
        cloth.positions()
    );
    assert!(cloth.positions()[2].y < cloth.positions()[1].y && cloth.positions()[1].y < cloth.positions()[0].y);
    assert!(report.max_self_contact_penetration < 1e-5, "{report:?}");
}

#[test]
fn external_coulomb_friction_slows_sliding_and_zero_friction_preserves_it() {
    let settings = ClothSettings { damping_per_second: 0.0, collision_thickness: 0.01, ..Default::default() };
    let mut frictionless = grid(3, 0.2, 0.01, false, settings);
    let mut rough = grid(3, 0.2, 0.01, false, ClothSettings { friction_coefficient: 0.8, ..settings });
    for cloth in [&mut frictionless, &mut rough] {
        let mut state = cloth.state().clone();
        state.velocities.fill(Vec3::new(1.0, 0.0, 0.0));
        cloth.restore_state(state).unwrap();
    }
    for _ in 0..30 {
        for cloth in [&mut frictionless, &mut rough] {
            cloth.step(|p| Some(ClothContact { distance: p.y, normal: Vec3::new(0.0, 1.0, 0.0) })).unwrap();
        }
    }
    assert!((frictionless.positions()[4].x - 0.5).abs() < 2e-5);
    assert!(rough.positions()[4].x > 0.01 && rough.positions()[4].x < 0.15, "{:?}", rough.positions()[4]);
    assert!(rough.velocities()[4].x.abs() < 1e-4);
    assert!(frictionless.velocities()[4].x > 0.999);
}

#[test]
fn self_contact_friction_uses_relative_layer_sliding_with_zero_friction_control() {
    let settings = ClothSettings {
        fixed_dt: 1.0 / 60.0,
        substeps: 4,
        iterations: 8,
        gravity: Vec3::new(0.0, -9.81, 0.0),
        ..self_contact_settings()
    };
    let mut frictionless = layered_triangles(settings, 0.02, true);
    let mut rough = layered_triangles(ClothSettings { friction_coefficient: 0.8, ..settings }, 0.02, true);
    for cloth in [&mut frictionless, &mut rough] {
        let mut state = cloth.state().clone();
        state.velocities[3..].fill(Vec3::new(1.0, 0.0, 0.0));
        cloth.restore_state(state).unwrap();
    }
    for _ in 0..30 {
        for cloth in [&mut frictionless, &mut rough] {
            let report = cloth.step(|_| None).unwrap();
            assert!(report.max_self_contact_penetration < 2e-6, "{report:?}");
        }
    }
    assert!((frictionless.positions()[5].x - 0.5).abs() < 2e-5);
    assert!(rough.positions()[5].x > 0.01 && rough.positions()[5].x < 0.15, "{:?}", rough.positions()[5]);
    assert!(rough.velocities()[5].x.abs() < 1e-4);
    assert!(frictionless.velocities()[5].x > 0.999);
}

#[test]
fn friction_does_not_damp_motion_without_contact_or_common_layer_translation() {
    let settings = ClothSettings { friction_coefficient: 100.0, ..self_contact_settings() };
    let mut cloth = layered_triangles(settings, 0.02, false);
    let mut state = cloth.state().clone();
    state.velocities.fill(Vec3::new(1.0, 0.0, 0.0));
    cloth.restore_state(state).unwrap();
    for _ in 0..5 {
        cloth.step(|_| None).unwrap();
    }
    for (p, rest) in cloth.positions().iter().zip(cloth.rest_positions()) {
        assert!((p.x - rest.x - 0.5).abs() < 2e-5, "{p:?}");
    }
    for velocity in cloth.velocities() {
        assert!((velocity.x - 1.0).abs() < 2e-5, "{velocity:?}");
    }
}

#[test]
fn topological_neighbors_do_not_collide_with_their_own_sheet() {
    let settings = ClothSettings { self_collision_thickness: 0.5, ..self_contact_settings() };
    // All pairs in a single triangle belong to the excluded one-ring, even
    // though the configured thickness is larger than its entire edge lengths.
    let mut cloth = Cloth::new(
        vec![Vec3::ZERO, Vec3::new(0.1, 0.0, 0.0), Vec3::new(0.0, 0.1, 0.0)],
        vec![[0, 1, 2]],
        vec![1.0; 3],
        settings,
    )
    .unwrap();
    let before = cloth.state().clone();
    let report = cloth.step(|_| None).unwrap();
    assert_eq!(report.self_contact_candidates, 0);
    assert_eq!(report.self_contact_projections, 0);
    assert_eq!(cloth.positions(), before.positions);
}

#[test]
fn self_contact_checkpoint_replay_is_exact_and_budget_failures_are_atomic() {
    let settings =
        ClothSettings { gravity: Vec3::new(0.0, -9.81, 0.0), friction_coefficient: 0.3, ..self_contact_settings() };
    let mut first = layered_triangles(settings, 0.05, true);
    for _ in 0..3 {
        first.step(|_| None).unwrap();
    }
    let mut restored = layered_triangles(settings, 0.05, true);
    restored.restore_state(first.state().clone()).unwrap();
    for _ in 0..8 {
        assert_eq!(first.step(|_| None).unwrap(), restored.step(|_| None).unwrap());
        assert_eq!(first.state(), restored.state());
    }
    for limited in [
        ClothSettings { self_collision_max_candidates: 1, ..settings },
        ClothSettings { self_collision_max_work: 1, ..settings },
        ClothSettings { self_collision_max_work: 500, ..settings },
    ] {
        let mut cloth = layered_triangles(limited, 0.02, true);
        let before = cloth.state().clone();
        let failure = cloth.step(|_| None).unwrap_err();
        assert!(failure.0.contains("budget exhausted"), "{failure}");
        assert_eq!(cloth.state(), &before);
    }
}

#[test]
fn self_contact_normal_and_friction_reactions_conserve_free_system_momentum() {
    let settings = ClothSettings { friction_coefficient: 0.8, ..self_contact_settings() };
    let template = layered_triangles(settings, 0.2, false);
    let weights = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let mut cloth =
        Cloth::new(template.rest_positions().to_vec(), template.triangles().to_vec(), weights.clone(), settings)
            .unwrap();
    let mut state = cloth.state().clone();
    state.velocities[3..].fill(Vec3::new(1.0, -8.0, 0.0));
    cloth.restore_state(state.clone()).unwrap();
    let momentum = |velocities: &[Vec3]| -> [f64; 3] {
        velocities.iter().zip(&weights).fold([0.0; 3], |mut sum, (v, &w)| {
            for (i, component) in [v.x, v.y, v.z].into_iter().enumerate() {
                sum[i] += f64::from(component) / f64::from(w);
            }
            sum
        })
    };
    let initial = momentum(&state.velocities);
    let report = cloth.step(|_| None).unwrap();
    assert!(report.self_contact_projections > 0);
    let final_momentum = momentum(cloth.velocities());
    for axis in 0..3 {
        assert!(
            (initial[axis] - final_momentum[axis]).abs() < 2e-5,
            "internal normal/friction impulses must balance: {initial:?}, {final_momentum:?}"
        );
    }
}

#[test]
fn self_contact_reports_immovable_penetrations_and_validates_resource_limits() {
    let settings = self_contact_settings();
    let template = layered_triangles(settings, 0.005, true);
    let mut pinned =
        Cloth::new(template.rest_positions().to_vec(), template.triangles().to_vec(), vec![0.0; 6], settings).unwrap();
    let before = pinned.positions().to_vec();
    let report = pinned.step(|_| None).unwrap();
    assert!(report.max_self_contact_penetration >= 0.015 - 1e-6, "{report:?}");
    assert_eq!(report.self_contact_projections, 0);
    assert_eq!(pinned.positions(), before);
    for settings in [
        ClothSettings { self_collision_thickness: 0.0, ..settings },
        ClothSettings { friction_coefficient: -0.1, ..settings },
        ClothSettings { self_collision_max_candidates: 0, ..settings },
        ClothSettings { self_collision_max_work: 50_000_001, ..settings },
    ] {
        assert!(settings.validate().is_err());
    }
}

#[test]
fn connected_folded_strip_keeps_nonadjacent_layers_apart_under_gravity() {
    let settings = ClothSettings {
        fixed_dt: 1.0 / 30.0,
        substeps: 4,
        iterations: 24,
        gravity: Vec3::new(0.0, -9.81, 0.0),
        bend_compliance: 1e8,
        ..self_contact_settings()
    };
    let mut points = Vec::new();
    let mut triangles = Vec::new();
    for row in 0..8 {
        let (height, z) = if row < 4 { (0.0, (row as f32 - 3.0) * 0.2) } else { (0.04, (4.0 - row as f32) * 0.2) };
        for x in [-0.5, 0.5] {
            points.push(Vec3::new(x, height, z));
        }
        if row < 7 {
            let i = row * 2;
            triangles.push([i, i + 2, i + 1]);
            triangles.push([i + 1, i + 2, i + 3]);
        }
    }
    let masses = (0..16).map(|i| if i < 8 { 0.0 } else { 1.0 }).collect::<Vec<_>>();
    let mut enabled = Cloth::new(points.clone(), triangles.clone(), masses.clone(), settings).unwrap();
    let mut disabled =
        Cloth::new(points, triangles, masses, ClothSettings { self_collision: false, ..settings }).unwrap();
    let mut projected = 0;
    for _ in 0..30 {
        let report = enabled.step(|_| None).unwrap();
        disabled.step(|_| None).unwrap();
        projected += report.self_contact_projections;
        assert!(report.max_self_contact_penetration < 2e-5, "{report:?}");
        assert!(report.self_contact_work <= settings.self_collision_max_work);
        assert!(report.self_contact_candidates <= u64::from(settings.self_collision_max_candidates));
        assert!(enabled.positions()[12..].iter().all(|p| p.y >= 0.02 - 3e-5), "{:?}", enabled.positions());
    }
    assert!(projected > 0);
    assert!(
        disabled.positions()[12..].iter().all(|p| p.y < -0.02),
        "disabled fold must fall through: {:?}",
        disabled.positions()
    );
    assert_eq!(&enabled.positions()[..8], &enabled.rest_positions()[..8]);
}

#[test]
fn static_self_contact_query_detects_overlap_created_between_clear_cache_endpoints() {
    let settings = self_contact_settings();
    let cloth = layered_triangles(settings, 0.1, true);
    let first = cloth.positions().to_vec();
    let mut second = first.clone();
    for p in &mut second[3..] {
        p.y = -0.1;
    }
    let midpoint = first.iter().zip(&second).map(|(&a, &b)| (a + b) * 0.5).collect::<Vec<_>>();
    let checkpoint = cloth.state().clone();
    for endpoint in [&first, &second] {
        let report = measure_cloth_self_contact(endpoint, cloth.triangles(), settings).unwrap();
        assert_eq!(report.max_penetration, 0.0, "endpoints must independently pass static clearance");
    }
    let measured = measure_cloth_self_contact(&midpoint, cloth.triangles(), settings).unwrap();
    assert!((measured.max_penetration - settings.self_collision_thickness).abs() < 1e-7);
    assert!(measured.candidates > 0 && measured.candidates <= u64::from(settings.self_collision_max_candidates));
    assert!(measured.work <= settings.self_collision_max_work);
    assert_eq!(cloth.measure_self_contact(&midpoint).unwrap(), measured);
    assert_eq!(cloth.state(), &checkpoint, "query must not install the supplied pose or move authored pins");
    assert_eq!(first, checkpoint.positions);

    // Measurement depends on geometry only, including completely immovable
    // contacts. The explicit query also works with simulation contact disabled.
    for mass in [0.0, 1.0] {
        let other = Cloth::new(
            first.clone(),
            cloth.triangles().to_vec(),
            vec![mass; 6],
            ClothSettings { self_collision: false, ..settings },
        )
        .unwrap();
        let before = other.state().clone();
        assert_eq!(other.measure_self_contact(&midpoint).unwrap(), measured);
        assert_eq!(other.state(), &before);
    }
}

#[test]
fn static_self_contact_query_catches_edge_only_interpolation_overlap() {
    let settings = self_contact_settings();
    let first = vec![
        Vec3::new(-1.0, 0.0, -0.5),
        Vec3::new(1.0, 0.0, -0.5),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(-1.0, 0.1, 0.5),
        Vec3::new(0.0, 0.1, -1.0),
        Vec3::new(1.0, 0.1, 0.5),
    ];
    let triangles = vec![[0, 1, 2], [3, 4, 5]];
    let mut second = first.clone();
    for p in &mut second[3..] {
        p.y = -0.1;
    }
    let midpoint = first.iter().zip(&second).map(|(&a, &b)| (a + b) * 0.5).collect::<Vec<_>>();
    assert_eq!(measure_cloth_self_contact(&first, &triangles, settings).unwrap().max_penetration, 0.0);
    assert_eq!(measure_cloth_self_contact(&second, &triangles, settings).unwrap().max_penetration, 0.0);
    let measured = measure_cloth_self_contact(&midpoint, &triangles, settings).unwrap();
    assert!((measured.max_penetration - settings.self_collision_thickness).abs() < 1e-7);
}

#[test]
fn static_self_contact_query_budget_failures_are_deterministic_and_read_only() {
    let template = layered_triangles(self_contact_settings(), 0.005, true);
    let positions = template.positions().to_vec();
    let triangles = template.triangles().to_vec();
    let untouched = (positions.clone(), triangles.clone());
    for limited in [
        ClothSettings { self_collision_max_candidates: 1, ..self_contact_settings() },
        ClothSettings { self_collision_max_work: 1, ..self_contact_settings() },
    ] {
        let first = measure_cloth_self_contact(&positions, &triangles, limited).unwrap_err();
        let second = measure_cloth_self_contact(&positions, &triangles, limited).unwrap_err();
        assert_eq!(first, second);
        assert!(first.0.contains("budget exhausted"), "{first}");
        assert_eq!((&positions, &triangles), (&untouched.0, &untouched.1));
    }
    let measured = measure_cloth_self_contact(&positions, &triangles, self_contact_settings()).unwrap();
    assert_eq!(measured, measure_cloth_self_contact(&positions, &triangles, self_contact_settings()).unwrap());
}

#[test]
fn static_self_contact_query_validates_topology_and_pose_but_handles_collapsed_features() {
    let settings = self_contact_settings();
    let template = layered_triangles(settings, 0.005, true);
    let positions = template.positions().to_vec();
    let triangles = template.triangles().to_vec();
    let mut nonfinite = positions.clone();
    nonfinite[0].x = f32::NAN;
    assert!(measure_cloth_self_contact(&nonfinite, &triangles, settings).is_err());
    assert!(template.measure_self_contact(&positions[..5]).is_err());
    assert!(measure_cloth_self_contact(&[], &[], settings).is_err());
    assert!(measure_cloth_self_contact(&positions, &[[0, 1, 6]], settings).is_err());
    assert!(measure_cloth_self_contact(&positions, &[[0, 1, 1]], settings).is_err());
    assert!(measure_cloth_self_contact(&positions, &[[0, 1, 2], [2, 1, 0]], settings).is_err());
    assert!(measure_cloth_self_contact(&positions[..4], &[[0, 1, 2], [0, 1, 3]], settings).is_err());
    assert!(measure_cloth_self_contact(&positions[..5], &[[0, 1, 2], [1, 0, 3], [0, 1, 4]], settings).is_err());
    assert!(measure_cloth_self_contact(
        &positions,
        &triangles,
        ClothSettings { self_collision: false, self_collision_thickness: 0.0, ..settings }
    )
    .is_err());
    let collapsed = vec![Vec3::ZERO; 6];
    let report = measure_cloth_self_contact(&collapsed, &triangles, settings).unwrap();
    assert_eq!(report.max_penetration, settings.self_collision_thickness);
    // The same solver exclusion prevents a collapsed single triangle from
    // colliding with its own edges/vertices, even at large positive thickness.
    let local = measure_cloth_self_contact(&collapsed[..3], &[[0, 1, 2]], settings).unwrap();
    assert_eq!(local.max_penetration, 0.0);
    assert_eq!(local.candidates, 0);
}

#[test]
fn static_self_contact_clearance_is_not_a_global_surface_intersection_proof() {
    // The small vertical triangle cuts through the large horizontal triangle's
    // interior, but its crossing edges stay far from the large triangle edges
    // and every vertex remains far from the opposing face. VT/EE surface
    // distance alone cannot diagnose this preexisting edge-through-face state.
    let positions = vec![
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(2.0, -2.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, -0.5, -1.0),
        Vec3::new(0.0, -0.5, 1.0),
        Vec3::new(0.0, 0.5, 1.0),
    ];
    let measured = measure_cloth_self_contact(&positions, &[[0, 1, 2], [3, 4, 5]], self_contact_settings()).unwrap();
    assert_eq!(measured.max_penetration, 0.0, "this API explicitly reports feature clearance, not global topology");
}
