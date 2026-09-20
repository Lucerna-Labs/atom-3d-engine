//! Unit tests for the kit's math and SDF mechanism — the correctness backbone the whole
//! renderer rests on. Run with `cargo test`.

use mm3e_kit::march::{Marcher, Ray};
use mm3e_kit::sdf::{self, Field};
use mm3e_kit::shade;
use mm3e_kit::vec::{Mat3, Quat, Transform, Vec3};

fn close(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

// ---- vectors & matrices ----

#[test]
fn vec_basics() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    assert!(close(a.dot(b), 32.0, 1e-5));
    assert!(close(a.cross(b).dot(a), 0.0, 1e-5)); // cross ⟂ a
    assert!(close(a.normalize().length(), 1.0, 1e-6));
    assert_eq!(Vec3::new(-1.0, 5.0, -2.0).clamp_to(Vec3::splat(0.0), Vec3::splat(3.0)), Vec3::new(0.0, 3.0, 0.0));
}

#[test]
fn mat3_rotation_is_orthonormal_and_correct() {
    let r = Mat3::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), std::f32::consts::FRAC_PI_2);
    // Rotating +x by 90° about +y gives -z.
    let v = r.mul_vec(Vec3::new(1.0, 0.0, 0.0));
    assert!(close(v.x, 0.0, 1e-5) && close(v.y, 0.0, 1e-5) && close(v.z, -1.0, 1e-5));
    // Transpose is the inverse.
    let back = r.transpose().mul_vec(v);
    assert!(close(back.x, 1.0, 1e-5) && close(back.z, 0.0, 1e-5));
}

#[test]
fn transform_zero_scale_is_finite() {
    // A degenerate (zero-scale) transform must not poison the field with Inf/NaN.
    let t = Transform::new(Vec3::new(1.0, 2.0, 3.0), Mat3::IDENTITY, 0.0);
    let p = t.to_local(Vec3::new(4.0, 5.0, 6.0));
    assert!(p.x.is_finite() && p.y.is_finite() && p.z.is_finite());
}

#[test]
fn transform_to_local_roundtrip() {
    let t = Transform::new(Vec3::new(2.0, -1.0, 3.0), Mat3::from_euler(0.3, 0.5, -0.2), 2.0);
    let world = Vec3::new(1.0, 4.0, -2.0);
    let local = t.to_local(world);
    let back = t.to_world(local);
    assert!((back - world).length() < 1e-4);
}

#[test]
fn transform_hierarchy_composes_rotation_translation_and_scale() {
    let parent =
        Transform::new(Vec3::new(10.0, 0.0, 0.0), Mat3::from_euler(0.0, 0.0, std::f32::consts::FRAC_PI_2), 2.0);
    let child = Transform::new(Vec3::new(1.0, 0.0, 0.0), Mat3::from_euler(std::f32::consts::FRAC_PI_2, 0.0, 0.0), 0.5);
    let world = parent.compose(child);
    assert!((world.pos - Vec3::new(10.0, 2.0, 0.0)).length() < 1e-5);
    assert!(close(world.scale, 1.0, 1e-6));
    let local = Vec3::new(0.0, 1.0, 0.0);
    assert!((world.to_world(local) - Vec3::new(10.0, 2.0, 1.0)).length() < 1e-5);
    assert!((world.to_world(local) - parent.to_world(child.to_world(local))).length() < 1e-5);
    assert!((world.to_local(world.to_world(local)) - local).length() < 1e-5);
}

#[test]
fn pivot_delta_moves_attached_geometry_without_moving_the_joint() {
    let pivot = Vec3::new(2.0, 3.0, 0.0);
    let translation = Vec3::new(-1.0, 0.0, 1.0);
    let rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), std::f32::consts::FRAC_PI_2);
    let delta = Transform::around_pivot(pivot, rotation, 2.0, translation);
    assert!((delta.to_world(pivot) - pivot - translation).length() < 1e-5);
    let rest = Transform::at(pivot + Vec3::new(1.0, 0.0, 0.0));
    let posed = delta.compose(rest);
    assert!((posed.pos - Vec3::new(1.0, 5.0, 1.0)).length() < 1e-5);
    assert!(close(posed.scale, 2.0, 1e-6));
    assert_eq!(rest.pos, Vec3::new(3.0, 3.0, 0.0));
}

#[test]
fn quaternion_matrix_roundtrip_including_half_turns() {
    let axes = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 2.0, -3.0).normalize(),
    ];
    for axis in axes {
        for angle in [0.0, 0.01, 1.7, std::f32::consts::PI, -std::f32::consts::PI, 5.9] {
            let original = Mat3::from_axis_angle(axis, angle);
            let quaternion = Quat::from_mat3(original);
            assert!(close(quaternion.dot(quaternion), 1.0, 1e-6));
            let recovered = quaternion.to_mat3();
            for (expected, actual) in original.cols.iter().zip(recovered.cols) {
                assert!((*expected - actual).length() < 1e-5, "axis={axis:?}, angle={angle}");
            }
        }
    }
    let expected = Mat3::from_euler(0.4, -2.1, 0.8);
    let actual = Quat::from_euler(0.4, -2.1, 0.8).to_mat3();
    for (expected, actual) in expected.cols.iter().zip(actual.cols) {
        assert!((*expected - actual).length() < 1e-5);
    }
}

#[test]
fn quaternion_product_matches_matrix_order_and_stays_unit() {
    let a = Quat::from_euler(0.7, -0.2, 0.8);
    let b = Quat::from_euler(-1.1, 0.5, 0.1);
    let point = Vec3::new(0.3, -0.8, 1.2);
    assert!(((a * b).to_mat3().mul_vec(point) - a.to_mat3().mul_vec(b.to_mat3().mul_vec(point))).length() < 1e-5);
    let step = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 0.001);
    let mut accumulated = Quat::IDENTITY;
    for _ in 0..10_000 {
        accumulated = step * accumulated;
    }
    assert!(close(accumulated.dot(accumulated), 1.0, 1e-6));
    let expected = Mat3::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 10.0).mul_vec(point);
    assert!((accumulated.to_mat3().mul_vec(point) - expected).length() < 1e-4);
}

#[test]
fn quat_slerp_endpoints_and_axis() {
    let q = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), std::f32::consts::FRAC_PI_2);
    let a = Quat::IDENTITY.slerp(q, 0.0);
    let b = Quat::IDENTITY.slerp(q, 1.0);
    // Endpoints match.
    let va = a.to_mat3().mul_vec(Vec3::new(1.0, 0.0, 0.0));
    assert!((va - Vec3::new(1.0, 0.0, 0.0)).length() < 1e-4);
    let vb = b.to_mat3().mul_vec(Vec3::new(1.0, 0.0, 0.0));
    assert!((vb - Vec3::new(0.0, 0.0, -1.0)).length() < 1e-3);
}

#[test]
fn slerp_takes_short_path_across_angle_wrap_and_antipodal_quaternions() {
    let axis = Vec3::new(0.0, 1.0, 0.0);
    let start = Quat::from_axis_angle(axis, 170.0_f32.to_radians());
    let end = Quat::from_axis_angle(axis, -170.0_f32.to_radians());
    let midpoint = start.slerp(end, 0.5);
    let point = Vec3::new(1.0, 0.0, 0.0);
    assert!((midpoint.to_mat3().mul_vec(point) + point).length() < 1e-5);
    let antipodal = Quat { x: -start.x, y: -start.y, z: -start.z, w: -start.w };
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let interpolated = start.slerp(antipodal, t);
        assert!(close(interpolated.dot(interpolated), 1.0, 1e-6));
        assert!((interpolated.to_mat3().mul_vec(point) - start.to_mat3().mul_vec(point)).length() < 1e-5);
    }
}

#[test]
fn slerp_normalizes_nonunit_authored_endpoints() {
    let unit = Quat::from_euler(0.0, 1.2, 0.0);
    let scaled = Quat { x: unit.x * 3.0, y: unit.y * 3.0, z: unit.z * 3.0, w: unit.w * 3.0 };
    let start = Quat { w: 2.0, ..Quat::IDENTITY };
    let actual = start.slerp(scaled, 0.5);
    let expected = Quat::from_euler(0.0, 0.6, 0.0);
    assert!(close(actual.dot(actual), 1.0, 1e-6));
    assert!(close(actual.dot(expected).abs(), 1.0, 1e-6));
}

// ---- SDF primitives ----

/// Numerically estimate |∇sdf| (should be ~1 for a true distance field).
fn grad_mag(f: impl Fn(Vec3) -> f32, p: Vec3) -> f32 {
    let h = 1e-3;
    let dx = f(p + Vec3::new(h, 0.0, 0.0)) - f(p - Vec3::new(h, 0.0, 0.0));
    let dy = f(p + Vec3::new(0.0, h, 0.0)) - f(p - Vec3::new(0.0, h, 0.0));
    let dz = f(p + Vec3::new(0.0, 0.0, h)) - f(p - Vec3::new(0.0, 0.0, h));
    Vec3::new(dx, dy, dz).scale(1.0 / (2.0 * h)).length()
}

#[test]
fn sphere_sdf_is_signed_distance() {
    assert!(close(sdf::sphere(Vec3::new(2.0, 0.0, 0.0), 1.0), 1.0, 1e-5)); // outside
    assert!(close(sdf::sphere(Vec3::ZERO, 1.0), -1.0, 1e-5)); // center
    assert!(close(grad_mag(|p| sdf::sphere(p, 1.0), Vec3::new(2.0, 0.3, -0.4)), 1.0, 1e-2));
}

#[test]
fn box_and_torus_are_lipschitz_outside() {
    let bx = |p| sdf::boxed(p, Vec3::splat(1.0));
    assert!(close(bx(Vec3::new(2.0, 0.0, 0.0)), 1.0, 1e-5));
    assert!(close(grad_mag(bx, Vec3::new(2.0, 0.5, 0.3)), 1.0, 2e-2));
    let to = |p| sdf::torus(p, 1.0, 0.3);
    // On the tube center ring (radius 1 in xz), distance ≈ -0.3.
    assert!(close(to(Vec3::new(1.0, 0.0, 0.0)), -0.3, 1e-5));
    assert!(close(grad_mag(to, Vec3::new(2.0, 0.2, 0.0)), 1.0, 2e-2));
}

#[test]
fn new_primitives_have_correct_sign() {
    assert!(sdf::round_cone(Vec3::new(0.0, 0.0, 0.0), 0.5, 0.2, 1.0) < 0.0);
    assert!(sdf::round_cone(Vec3::new(3.0, 0.0, 0.0), 0.5, 0.2, 1.0) > 0.0);
    assert!(sdf::ellipsoid(Vec3::ZERO, Vec3::new(1.0, 2.0, 0.5)) < 0.0);
    assert!(sdf::octahedron(Vec3::ZERO, 1.0) < 0.0);
    assert!(sdf::octahedron(Vec3::new(3.0, 3.0, 3.0), 1.0) > 0.0);
    assert!(sdf::hex_prism(Vec3::ZERO, 1.0, 1.0) < 0.0);
    assert!(sdf::hex_prism(Vec3::new(3.0, 0.0, 0.0), 1.0, 1.0) > 0.0);
}

// ---- CSG combinators ----

#[test]
fn csg_union_subtract_intersect() {
    let a = Field::new(0.5, 1);
    let b = Field::new(-0.3, 2);
    assert_eq!(sdf::union(a, b).mat, 2); // b is nearer
    assert!(close(sdf::union(a, b).dist, -0.3, 1e-6));
    assert!(close(sdf::intersect(a, b).dist, 0.5, 1e-6));
    // Subtract b from a: max(a, -b) = max(0.5, 0.3) = 0.5, keeps a's material.
    let s = sdf::subtract(a, b);
    assert!(close(s.dist, 0.5, 1e-6) && s.mat == 1);
}

#[test]
fn smooth_union_is_bounded_below_hard_union() {
    let a = Field::new(0.4, 1);
    let b = Field::new(0.5, 2);
    let hard = sdf::union(a, b).dist;
    let soft = sdf::smooth_union(a, b, 0.3).dist;
    assert!(soft <= hard + 1e-6); // smooth-min never exceeds the hard min
    assert!(soft >= hard - 0.3); // ...and stays within k of it
                                 // k → 0 recovers the hard union.
    assert!(close(sdf::smooth_union(a, b, 0.0).dist, hard, 1e-6));
}

// ---- domain operators ----

#[test]
fn domain_ops() {
    // Infinite repetition folds the domain into a [-period/2, period/2] cell.
    let q = sdf::op_repeat(Vec3::new(2.6, -3.1, 0.2), Vec3::splat(1.0));
    assert!(q.x.abs() <= 0.5 + 1e-5 && q.y.abs() <= 0.5 + 1e-5);
    // round shifts the surface outward by exactly r.
    assert!(close(sdf::op_round(0.7, 0.2), 0.5, 1e-6));
    // onion makes a shell symmetric about the original surface.
    assert!(close(sdf::op_onion(0.0, 0.1), -0.1, 1e-6));
    // mirror folds negative coordinates to positive.
    let m = sdf::op_mirror(Vec3::new(-2.0, 1.0, -3.0), true, false, true);
    assert_eq!(m, Vec3::new(2.0, 1.0, 3.0));
}

#[test]
fn sdf_volume_samples_and_guards_bad_layouts() {
    let vol = mm3e_kit::volume::SdfVolume::new(
        (2, 2, 2),
        Vec3::ZERO,
        Vec3::splat(1.0),
        vec![
            0.0, 1.0, 0.0, 1.0, // z=0 rows
            0.0, 1.0, 0.0, 1.0, // z=1 rows
        ],
    )
    .expect("valid volume");
    assert!(close(vol.sample(Vec3::new(0.5, 0.25, 0.75)), 0.5, 1e-6));
    assert!(vol.sample(Vec3::new(-1.0, 0.5, 0.5)) >= 1.0);
    assert!(vol.recommended_normal_h() > 0.5);

    let bad = mm3e_kit::volume::SdfVolume { dims: (2, 2, 2), min: Vec3::ZERO, cell: Vec3::splat(1.0), data: vec![0.0] };
    assert_eq!(bad.sample(Vec3::splat(0.5)), f32::INFINITY);
}

#[test]
fn sdf_volume_sdfv_roundtrips_and_rejects_bad_input() {
    use mm3e_kit::volume::SdfVolume;

    let vol = SdfVolume::new(
        (2, 2, 2),
        Vec3::new(-1.0, -2.0, -3.0),
        Vec3::new(0.5, 0.75, 1.25),
        vec![0.0, 0.5, 1.0, 1.5, -0.25, 0.25, 0.75, 1.25],
    )
    .expect("valid volume");
    let bytes = vol.to_sdfv_bytes().expect("encode sdfv");
    assert_eq!(&bytes[..8], b"MM3ESDF1");

    let round = SdfVolume::from_sdfv_bytes(&bytes).expect("decode sdfv");
    assert_eq!(round.dims, vol.dims);
    assert_eq!(round.min, vol.min);
    assert_eq!(round.cell, vol.cell);
    assert_eq!(round.data, vol.data);
    assert!(close(round.sample(Vec3::new(-0.75, -1.5, -2.5)), vol.sample(Vec3::new(-0.75, -1.5, -2.5)), 1e-6));

    assert!(SdfVolume::from_sdfv_bytes(&bytes[..12]).is_err());
    let mut bad = bytes.clone();
    bad[0] = b'X';
    assert!(SdfVolume::from_sdfv_bytes(&bad).is_err());

    let path = std::env::temp_dir().join(format!("mm3e-volume-{}-sdfv-roundtrip.sdfv", std::process::id()));
    vol.save_sdfv(&path).expect("save sdfv");
    let loaded = SdfVolume::load_sdfv(&path).expect("load sdfv");
    let _ = std::fs::remove_file(&path);
    assert_eq!(loaded.data, vol.data);
}
// ---- raymarcher & shading ----

#[test]
fn marcher_hits_a_sphere_and_normal_points_back() {
    let field = |p: Vec3| Field::new(sdf::sphere(p - Vec3::new(0.0, 0.0, -5.0), 1.0), 0);
    let m = Marcher::default();
    let ray = Ray { origin: Vec3::ZERO, dir: Vec3::new(0.0, 0.0, -1.0) };
    let hit = m.march(&field, &ray);
    assert!(hit.hit);
    assert!(close(hit.t, 4.0, 1e-2)); // sphere front face at z = -4
    assert!(hit.normal.dot(Vec3::new(0.0, 0.0, 1.0)) > 0.9); // normal faces the camera
}

#[test]
fn boosted_steps_never_accept_hits_buried_inside_a_surface() {
    let field = |p: Vec3| Field::new(sdf::sphere(p, 1.0).abs() - 0.02, 0);
    for &subitize in &[0.0f32, 0.6] {
        let m = Marcher { subitize, ..Marcher::default() };
        let mut hits = 0u32;
        for iy in -12..=12 {
            for ix in -12..=12 {
                let dir = Vec3::new(ix as f32 * 0.02, iy as f32 * 0.02, -1.0).normalize();
                let ray = Ray { origin: Vec3::new(0.0, 0.0, 5.0), dir };
                let hit = m.march(&field, &ray);
                if !hit.hit {
                    continue;
                }
                hits += 1;
                let d = field(hit.pos).dist;
                let eps = m.eps * (1.0 + hit.t * 0.5);
                assert!(
                    d > -2.0 * eps,
                    "hit buried inside thin shell: subitize={subitize} ix={ix} iy={iy} t={} d={} eps={}",
                    hit.t,
                    d,
                    eps
                );
            }
        }
        assert!(hits > 100, "thin shell regression should exercise many hits, got {hits}");
    }
}

#[test]
fn font_draws_glyphs() {
    use mm3e_kit::font;
    assert_eq!(font::glyph(' '), [0; 7]);
    assert!(font::glyph('A').iter().any(|&r| r != 0));
    // Drawing onto an RGBA buffer marks some pixels and leaves blank space untouched.
    let (w, h) = (64u32, 16u32);
    let mut buf = vec![0u8; (w * h * 4) as usize];
    font::draw_text(&mut buf, w, h, 2, 2, 1, "A1", [255, 255, 255]);
    assert!(buf.iter().any(|&b| b != 0), "draw_text should mark pixels");
    // A character drawn out of bounds must not panic or write.
    font::draw_text(&mut buf, w, h, -100, -100, 2, "Z", [255, 0, 0]);
}

#[test]
fn shading_invariants() {
    let n = Vec3::new(0.0, 1.0, 0.0);
    let l = Vec3::new(0.0, 1.0, 0.0);
    let v = Vec3::new(0.0, 1.0, 0.0);
    let b = shade::brdf(n, l, v, Vec3::splat(0.8), 0.0, 0.3, 0.5);
    assert!(b.x >= 0.0 && b.y >= 0.0 && b.z >= 0.0);
    // Fresnel grows from f0 toward 1 at grazing.
    assert!(shade::fresnel_schlick(1.0, 0.04) < shade::fresnel_schlick(0.0, 0.04));
    // ACES stays in gamut.
    let t = shade::aces(Vec3::splat(20.0));
    assert!(t.x <= 1.0 && t.x >= 0.0);
}
