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
    // Reconstruct the world point: pos + R·(local·scale).
    let back = t.pos + t.rot.mul_vec(local.scale(t.scale));
    assert!((back - world).length() < 1e-4);
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
