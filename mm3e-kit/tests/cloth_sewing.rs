use mm3e_kit::{
    cloth::{
        measure_cloth_self_contact, measure_cloth_self_contact_with_seams, Cloth, ClothContact, ClothSeam,
        ClothSettings, MAX_CLOTH_SEAMS,
    },
    Vec3,
};

fn panels(gap: f32, masses: Vec<f32>, settings: ClothSettings, seams: Vec<ClothSeam>) -> Cloth {
    Cloth::new_with_seams(
        vec![
            Vec3::new(-1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::ZERO,
            Vec3::new(gap, 1.0, 0.0),
            Vec3::new(1.0 + gap, 1.0, 0.0),
            Vec3::new(gap, 0.0, 0.0),
            Vec3::new(1.0 + gap, 0.0, 0.0),
        ],
        vec![[0, 1, 2], [1, 3, 2], [4, 5, 6], [5, 7, 6]],
        masses,
        settings,
        seams,
    )
    .unwrap()
}

fn seam(a: u32, b: u32, rest_length: f32, compliance: f32) -> ClothSeam {
    ClothSeam { vertices: [a, b], rest_length, compliance }
}

fn isolated_settings() -> ClothSettings {
    ClothSettings {
        substeps: 1,
        iterations: 1,
        gravity: Vec3::ZERO,
        damping_per_second: 0.0,
        stretch_compliance: 1e8,
        bend_compliance: 1e8,
        ..Default::default()
    }
}

#[test]
fn two_separate_square_panels_zip_their_edge_under_gravity_with_an_unsewn_control() {
    let settings = ClothSettings {
        iterations: 32,
        stretch_compliance: 0.0,
        damping_per_second: 2.0,
        self_collision: true,
        ..Default::default()
    };
    let mut masses = vec![1.0; 8];
    masses[0] = 0.0;
    masses[2] = 0.0;
    let mut sewn = panels(0.2, masses.clone(), settings, vec![seam(1, 4, 0.0, 0.0), seam(3, 6, 0.0, 0.0)]);
    let mut unsewn = panels(0.2, masses, settings, Vec::new());
    let rest = sewn.rest_positions().to_vec();
    let topology = sewn.triangles().to_vec();
    let fabric_constraint_counts = sewn.constraint_counts();
    let mut projections = 0;
    let mut final_report = None;
    for _ in 0..90 {
        let report = sewn.step(|_| None).unwrap();
        unsewn.step(|_| None).unwrap();
        assert!(report.max_seam_length_error < 2e-6, "{report:?}");
        projections += report.seam_projections;
        final_report = Some(report);
    }
    assert!(projections > 0);
    assert!(final_report.unwrap().max_relative_edge_error < 0.02);
    assert!(
        (unsewn.positions()[1] - unsewn.positions()[4]).length() > 0.5,
        "without stitches the free panel must separate"
    );
    assert!((sewn.positions()[7] - rest[7]).length() > 0.2, "sewn fabric must still move under gravity");
    for i in [0, 2] {
        assert_eq!(sewn.positions()[i], rest[i]);
    }
    assert_eq!(sewn.rest_positions(), rest);
    assert_eq!(sewn.triangles(), topology);
    assert_eq!(sewn.constraint_counts(), fabric_constraint_counts);
}

#[test]
fn sewing_redistributes_reaction_by_inverse_mass_and_preserves_exact_pins() {
    for (wa, wb, expected_x) in [(1.0, 3.0, 0.05), (0.0, 3.0, 0.0), (2.0, 0.0, 0.2)] {
        let mut masses = vec![1.0; 8];
        masses[1] = wa;
        masses[4] = wb;
        let mut cloth = panels(0.2, masses, isolated_settings(), vec![seam(1, 4, 0.0, 0.0)]);
        let before = cloth.positions().to_vec();
        let report = cloth.step(|_| None).unwrap();
        assert_eq!(report.seam_projections, 1);
        assert!(report.max_seam_length_error < 1e-6);
        assert!((cloth.positions()[1].x - expected_x).abs() < 1e-6);
        assert!((cloth.positions()[4].x - expected_x).abs() < 1e-6);
        if wa == 0.0 {
            assert_eq!(cloth.positions()[1], before[1]);
        }
        if wb == 0.0 {
            assert_eq!(cloth.positions()[4], before[4]);
        }
    }
}

#[test]
fn coincident_zero_length_stitches_stay_finite_and_nonzero_targets_can_separate_them() {
    for target in [0.0, 0.1] {
        let mut cloth = panels(0.0, vec![1.0; 8], isolated_settings(), vec![seam(1, 4, target, 0.0)]);
        let report = cloth.step(|_| None).unwrap();
        assert!(report.max_seam_length_error < 1e-6, "{report:?}");
        assert!(((cloth.positions()[1] - cloth.positions()[4]).length() - target).abs() < 1e-6);
        for p in cloth.positions().iter().chain(cloth.velocities()) {
            assert!(p.x.is_finite() && p.y.is_finite() && p.z.is_finite());
        }
    }
    let settings = ClothSettings { gravity: Vec3::new(0.0, -9.81, 0.0), ..isolated_settings() };
    let mut falling = panels(0.0, vec![1.0; 8], settings, vec![seam(1, 4, 0.0, 0.0)]);
    falling.step(|_| None).unwrap();
    assert!(falling.positions()[1].y < 1.0, "zero-length sewing must not become a world-space hold");
}

#[test]
fn sewing_compliance_changes_actual_closure_and_conflicting_pins_are_reported() {
    let mut stiff = panels(0.2, vec![1.0; 8], isolated_settings(), vec![seam(1, 4, 0.0, 0.0)]);
    let mut soft = panels(0.2, vec![1.0; 8], isolated_settings(), vec![seam(1, 4, 0.0, 1e8)]);
    assert!(stiff.step(|_| None).unwrap().max_seam_length_error < 1e-6);
    assert!(soft.step(|_| None).unwrap().max_seam_length_error > 0.19);
    let mut pinned = panels(0.2, vec![0.0; 8], isolated_settings(), vec![seam(1, 4, 0.0, 0.0)]);
    let before = pinned.positions().to_vec();
    let report = pinned.step(|_| None).unwrap();
    assert_eq!(report.seam_projections, 0);
    assert!((report.max_seam_length_error - 0.2).abs() < 1e-6);
    assert_eq!(pinned.positions(), before);
}

#[test]
fn sewing_checkpoint_replay_and_canonical_pair_order_are_exact() {
    let settings = ClothSettings { gravity: Vec3::new(0.0, -9.81, 0.0), iterations: 8, ..Default::default() };
    let mut first = panels(0.2, vec![1.0; 8], settings, vec![seam(1, 4, 0.0, 1e-5), seam(3, 6, 0.0, 1e-5)]);
    let mut reordered = panels(0.2, vec![1.0; 8], settings, vec![seam(6, 3, 0.0, 1e-5), seam(4, 1, 0.0, 1e-5)]);
    for _ in 0..8 {
        assert_eq!(first.step(|_| None).unwrap(), reordered.step(|_| None).unwrap());
        assert_eq!(first.state(), reordered.state());
    }
    let mut resumed = panels(0.2, vec![1.0; 8], settings, first.seams().to_vec());
    resumed.restore_state(first.state().clone()).unwrap();
    for _ in 0..8 {
        assert_eq!(first.step(|_| None).unwrap(), resumed.step(|_| None).unwrap());
        assert_eq!(first.state(), resumed.state());
    }
}

#[test]
fn empty_seam_constructor_and_replacement_preserve_the_existing_solver_bitwise() {
    let settings = ClothSettings { self_collision: true, ..Default::default() };
    let template = panels(0.2, vec![1.0; 8], settings, Vec::new());
    let mut old =
        Cloth::new(template.rest_positions().to_vec(), template.triangles().to_vec(), vec![1.0; 8], settings).unwrap();
    let mut explicit_empty = template;
    explicit_empty.set_seams(Vec::new()).unwrap();
    for _ in 0..8 {
        assert_eq!(old.step(|_| None).unwrap(), explicit_empty.step(|_| None).unwrap());
        assert_eq!(old.state(), explicit_empty.state());
    }
}

#[test]
fn sewing_validation_rejects_bad_or_reversed_duplicate_pairs_without_changing_state() {
    let initial = seam(1, 4, 0.0, 0.0);
    let mut cloth = panels(0.2, vec![1.0; 8], isolated_settings(), vec![initial]);
    let state = cloth.state().clone();
    for seams in [
        vec![initial, seam(4, 1, 0.0, 0.0)],
        vec![seam(1, 1, 0.0, 0.0)],
        vec![seam(1, 8, 0.0, 0.0)],
        vec![seam(1, 4, -0.1, 0.0)],
        vec![seam(1, 4, f32::NAN, 0.0)],
        vec![seam(1, 4, 0.0, -0.1)],
        vec![seam(1, 4, 0.0, f32::INFINITY)],
        vec![initial; MAX_CLOTH_SEAMS + 1],
    ] {
        assert!(cloth.set_seams(seams).is_err());
        assert_eq!(cloth.seams(), &[initial]);
        assert_eq!(cloth.state(), &state);
    }
    let mut positions = Vec::new();
    let mut triangles = Vec::new();
    for row in 0..10 {
        for column in 0..10 {
            positions.push(Vec3::new(column as f32, row as f32, 0.0));
            if row < 9 && column < 9 {
                let i = row * 10 + column;
                triangles.extend([[i, i + 10, i + 1], [i + 1, i + 10, i + 11]]);
            }
        }
    }
    let mut budgeted = Cloth::new(
        positions,
        triangles,
        vec![1.0; 100],
        ClothSettings { substeps: 128, iterations: 256, contact_iterations: 1, ..Default::default() },
    )
    .unwrap();
    let before = budgeted.state().clone();
    let seams = (0..100).flat_map(|a| (a + 1..100).map(move |b| seam(a, b, 0.0, 0.0))).take(1000).collect();
    assert!(budgeted.set_seams(seams).unwrap_err().0.contains("projection work budget"));
    assert!(budgeted.seams().is_empty());
    assert_eq!(budgeted.state(), &before);
}

#[test]
fn static_clearance_uses_direct_seams_without_excluding_unsewn_neighbors_or_other_layers() {
    let settings = ClothSettings { self_collision: true, ..Default::default() };
    let seams = vec![seam(1, 4, 0.0, 0.0), seam(3, 6, 0.0, 0.0)];
    let cloth = panels(0.0, vec![1.0; 8], settings, seams.clone());
    assert!(
        measure_cloth_self_contact(cloth.positions(), cloth.triangles(), settings).unwrap().max_penetration > 0.0059
    );
    let linked = measure_cloth_self_contact_with_seams(cloth.positions(), cloth.triangles(), &seams, settings).unwrap();
    assert_eq!(linked.max_penetration, 0.0);
    assert_eq!(cloth.measure_self_contact(cloth.positions()).unwrap(), linked);
    let partial =
        measure_cloth_self_contact_with_seams(cloth.positions(), cloth.triangles(), &seams[..1], settings).unwrap();
    assert!(partial.max_penetration > 0.0059, "the unsewn endpoint remains a real contact");
    let mut positions = cloth.positions().to_vec();
    positions.extend([Vec3::new(-0.6, 0.4, 0.001), Vec3::new(-0.4, 0.4, 0.001), Vec3::new(-0.5, 0.6, 0.001)]);
    let mut triangles = cloth.triangles().to_vec();
    triangles.push([8, 9, 10]);
    assert!(
        measure_cloth_self_contact_with_seams(&positions, &triangles, &seams, settings).unwrap().max_penetration
            > 0.0049
    );
}

#[test]
fn failure_after_a_real_sewing_correction_rolls_back_the_entire_step() {
    let seams = vec![seam(1, 4, 0.0, 0.0)];
    let mut cloth = panels(0.2, vec![1.0; 8], isolated_settings(), seams.clone());
    let before = cloth.state().clone();
    let mut observed_sewing_correction = false;
    let failure = cloth.step(|p| {
        if (p.x - 0.1).abs() < 1e-6 && (p.y - 1.0).abs() < 1e-6 {
            observed_sewing_correction = true;
            Some(ClothContact { distance: f32::NAN, normal: Vec3::new(0.0, 1.0, 0.0) })
        } else {
            None
        }
    });
    assert!(observed_sewing_correction, "failure must occur after actual seam movement in solver scratch state");
    assert!(failure.is_err());
    assert_eq!(cloth.state(), &before);
    assert_eq!(cloth.seams(), seams);
}
