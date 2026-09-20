use mm3e_kit::{
    sdf,
    vec::{Transform, Vec3},
};
use mm3e_orchestrator::{Object, Prim, Scene};

#[test]
fn authored_query_does_not_report_an_acceleration_sphere_as_the_geometry() {
    let mut scene = Scene::new(16, 16);
    let half = Vec3::new(4.0, 0.2, 0.2);
    scene.add(Object::new(Prim::Box { half }, Transform::IDENTITY, 0));
    let p = Vec3::new(0.0, 5.0, 0.0);
    let expected = sdf::boxed(p, half);
    assert!((scene.sample_authored(p).dist - expected).abs() < 1e-6);
    assert!(scene.field()(p).dist < expected - 1.0, "this probe must exercise the accelerated lower bound");
}

#[test]
fn authored_query_preserves_csg_order_and_base_material_on_a_cut() {
    let mut scene = Scene::new(16, 16);
    assert!(scene.sample_authored(Vec3::ZERO).dist.is_infinite());
    scene.add(Object::new(Prim::Box { half: Vec3::splat(2.0) }, Transform::IDENTITY, 7));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::IDENTITY, 3).subtract());
    let cut = scene.sample_authored(Vec3::ZERO);
    assert!((cut.dist - 1.0).abs() < 1e-6);
    assert_eq!(cut.mat, 7);
    scene.add(Object::new(Prim::Sphere { r: 0.25 }, Transform::IDENTITY, 9));
    let fill = scene.sample_authored(Vec3::ZERO);
    assert!((fill.dist + 0.25).abs() < 1e-6);
    assert_eq!(fill.mat, 9);
    assert_eq!(scene.sample_object(0, Vec3::ZERO).unwrap().dist, -2.0);
    assert!(scene.sample_object(999, Vec3::ZERO).is_none());
}
