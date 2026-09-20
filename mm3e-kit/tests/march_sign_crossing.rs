use mm3e_kit::{
    march::{Marcher, Ray},
    sdf::{self, Field},
    vec::Vec3,
};

#[test]
fn accelerated_sign_crossing_returns_plane_surface_instead_of_buried_hit() {
    let field = |p: Vec3| Field::new(p.z, 7);
    for subitize in [0.0, 0.6] {
        for secant in [false, true] {
            let marcher = Marcher { subitize, secant, ..Marcher::default() };
            for x in [0.0_f32, 0.1, 0.5] {
                let ray = Ray { origin: Vec3::new(0.0, 0.0, 3.0), dir: Vec3::new(x, 0.0, -1.0).normalize() };
                let hit = marcher.march(&field, &ray);
                let expected = 3.0 / -ray.dir.z;
                assert!(hit.hit);
                assert!((hit.t - expected).abs() < 0.002, "x={x} subitize={subitize} secant={secant}: t={}", hit.t);
                assert!(hit.pos.z >= -0.00001 && hit.pos.z < 0.002);
                assert_eq!(hit.mat, 7);
            }
        }
    }
}

#[test]
fn touching_sphere_distance_bounds_do_not_authorize_a_negative_hit() {
    let field = |p: Vec3| Field::new(sdf::sphere(p, 1.0), 3);
    // Start close enough that the 1.4x step remains in the front half: the unsigned safe
    // radii then sum to the full step, just as for a plane. The old strict > guard missed it.
    let ray = Ray { origin: Vec3::new(0.0, 0.0, 2.0), dir: Vec3::new(0.0, 0.0, -1.0) };
    let hit = Marcher::default().march(&field, &ray);
    assert!(hit.hit);
    assert!((hit.t - 1.0).abs() < 0.00001);
    assert!((hit.pos.z - 1.0).abs() < 0.00001);
}

#[test]
fn ray_origin_inside_solid_preserves_immediate_hit_policy() {
    let marcher = Marcher::default();
    let ray = Ray { origin: Vec3::new(0.0, 0.0, -0.5), dir: Vec3::new(0.0, 0.0, -1.0) };
    let hit = marcher.march(&|p: Vec3| Field::new(p.z, 4), &ray);
    assert!(hit.hit);
    assert_eq!(hit.t, 0.0);
    assert_eq!(hit.pos, ray.origin);
    assert_eq!(hit.mat, 4);
}
