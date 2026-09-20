use mm3e_kit::{
    cloth::{Cloth, ClothSettings},
    Vec3,
};

fn panel(self_collision: bool) -> Cloth {
    let mut positions = Vec::new();
    let mut triangles = Vec::new();
    for row in 0..7 {
        for column in 0..7 {
            positions.push(Vec3::new((column as f32 / 6.0 - 0.5) * 0.8, 1.0, (row as f32 / 6.0 - 0.5) * 0.8));
            if row < 6 && column < 6 {
                let i = row * 7 + column;
                triangles.push([i, i + 7, i + 1]);
                triangles.push([i + 1, i + 7, i + 8]);
            }
        }
    }
    let mut masses = vec![50.0; 49];
    masses[0] = 0.0;
    masses[6] = 0.0;
    Cloth::new(
        positions,
        triangles,
        masses,
        ClothSettings {
            fixed_dt: (1.0 / 60.0) / 4.0,
            substeps: 1,
            iterations: 24,
            bend_compliance: 0.05,
            self_collision,
            self_collision_thickness: 0.006,
            friction_coefficient: 0.3,
            ..Default::default()
        },
    )
    .unwrap()
}

#[test]
fn flat_panel_broadphase_work_is_bounded_and_cannot_change_contact_free_motion() {
    let mut enabled = panel(true);
    let mut disabled = panel(false);
    let mut replay = enabled.clone();
    let mut work = 0;
    let mut candidates = 0;
    for _ in 0..12 {
        let report = enabled.step(|_| None).unwrap();
        disabled.step(|_| None).unwrap();
        assert_eq!(report, replay.step(|_| None).unwrap());
        assert_eq!(enabled.state(), replay.state());
        assert_eq!(enabled.state(), disabled.state());
        assert_eq!(report.self_contact_projections, 0);
        work += report.self_contact_work;
        candidates += report.self_contact_candidates;
    }
    eprintln!("49-vertex panel / 12 physical substeps: counted collision work={work}, candidates={candidates}");
    // Baseline was 5,785,920 units for the same 14,700 candidates and no
    // projections. Keep an explicit work regression without raising any cap.
    assert_eq!(candidates, 14_700);
    assert!(work <= 1_500_000, "broadphase work regressed: {work}");
}
