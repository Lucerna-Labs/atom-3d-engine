use mm3e_kit::{
    csg::Expr,
    vec::{Mat3, Transform, Vec3},
    Camera, Material, Rgba,
};
use mm3e_orchestrator::{Object, Prim, RenderMode, Scene};

fn cavity() -> Expr {
    Expr::Subtract {
        a: Box::new(Expr::Sphere { r: 1.0 }),
        b: Box::new(Expr::Transform {
            shape: Box::new(Expr::Sphere { r: 0.65 }),
            xform: Transform::at(Vec3::new(0.0, 0.0, 0.8)),
        }),
    }
}

#[test]
fn local_cavity_does_not_cut_an_overlapping_independent_object() {
    let mut scene = Scene::new(32, 32);
    let id = scene.csg(cavity()).unwrap();
    scene.add(Object::new(Prim::Csg { id }, Transform::IDENTITY, 0));
    let point = Vec3::new(0.0, 0.0, 0.8);
    assert!(scene.sample_authored(point).dist > 0.0);
    scene.add(Object::new(Prim::Sphere { r: 0.1 }, Transform::at(point), 8));
    let world = scene.sample_authored(point);
    assert_eq!(world.mat, 8);
    assert!((world.dist + 0.1).abs() < 1e-6);
    assert!(scene.sample_object(0, point).unwrap().dist > 0.0);
    assert!(!scene.is_dual_safe(), "CSG uses explicit finite-difference normals");
}

#[test]
fn local_shell_clipping_is_preserved_under_object_pose() {
    let mut scene = Scene::new(16, 16);
    let id = scene
        .csg(Expr::Intersect {
            a: Box::new(Expr::Shell { shape: Box::new(Expr::Sphere { r: 1.0 }), thickness: 0.04 }),
            b: Box::new(Expr::Plane { n: Vec3::new(0.0, -1.0, 0.0), h: 0.2 }),
        })
        .unwrap();
    let xf = Transform::new(Vec3::new(3.0, 0.5, 1.0), Mat3::from_euler(0.3, 0.7, 0.1), 1.5);
    scene.add(Object::new(Prim::Csg { id }, xf, 0));
    let closed = xf.to_world(Vec3::new(0.0, 0.6, 0.8));
    let open = xf.to_world(Vec3::new(0.0, 0.0, 1.0));
    assert!(scene.sample_authored(closed).dist < 0.0);
    assert!(scene.sample_authored(open).dist > 0.0);
    assert!(scene.sample_authored(xf.pos).dist > 0.0);
}

#[test]
fn accelerated_and_linear_csg_fields_match_and_legacy_export_rejects_csg() {
    let mut scene = Scene::new(24, 24);
    let id = scene.csg(cavity()).unwrap();
    for i in 0..12 {
        scene.add(Object::new(Prim::Csg { id }, Transform::at(Vec3::new(i as f32 * 2.5, 0.0, 0.0)), 0));
    }
    scene.add(Object::new(Prim::Sphere { r: 0.4 }, Transform::at(Vec3::new(1.0, 0.0, 0.0)), 0).smooth(0.1));
    let accelerated = scene.field();
    let linear = scene.world_linear();
    for x in -10..160 {
        for y in -5..=5 {
            for z in -5..=5 {
                let p = Vec3::new(x as f32 * 0.2, y as f32 * 0.3, z as f32 * 0.3);
                let a = accelerated(p);
                let b = linear(p);
                assert!((a.dist - b.dist).abs() < 1e-6);
                assert_eq!(a.mat, b.mat);
            }
        }
    }
    let camera = Camera::look_at(Vec3::new(0.0, 0.0, 4.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 0.7);
    assert!(mm3e_orchestrator::scene_io::try_serialize(&scene, &camera).unwrap_err().contains("CSG"));
}

#[test]
fn registration_failure_is_atomic_and_csg_primitive_renders_identically() {
    let mut scene = Scene::new(40, 40);
    assert!(scene.csg(Expr::Sphere { r: -1.0 }).is_err());
    assert!(scene.csgs.is_empty());
    scene.aa = 1;
    scene.post.bloom = false;
    scene.shadows = false;
    scene.ao = false;
    scene.bounces = 0;
    scene.mode = RenderMode::Albedo;
    let id = scene.csg(Expr::Sphere { r: 1.0 }).unwrap();
    scene.add(Object::new(Prim::Csg { id }, Transform::IDENTITY, 0));
    let camera = Camera::look_at(Vec3::new(0.0, 0.0, 4.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 0.7);
    let csg = mm3e_orchestrator::render(&scene, &camera).to_rgba8(Rgba::new(0.0, 0.0, 0.0, 1.0));
    scene.objects[0].prim = Prim::Sphere { r: 1.0 };
    let primitive = mm3e_orchestrator::render(&scene, &camera).to_rgba8(Rgba::new(0.0, 0.0, 0.0, 1.0));
    assert_eq!(csg, primitive);
}

#[test]
fn scene_linear_radiance_preserves_hdr_and_display_output() {
    let mut scene = Scene::new(24, 24);
    scene.aa = 1;
    scene.bounces = 0;
    scene.shadows = false;
    scene.ao = false;
    scene.fog_density = 0.0;
    let mat = scene.material(Material { emissive: Vec3::new(6.0, 3.0, 2.0), ..Material::default() });
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::IDENTITY, mat));
    let camera = Camera::look_at(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 0.7);
    let linear = mm3e_orchestrator::render_linear(&scene, &camera);
    assert!(linear[12 * 24 + 12].x >= 6.0);
    let clear = Rgba::new(0.0, 0.0, 0.0, 1.0);
    let direct = mm3e_orchestrator::render(&scene, &camera).to_rgba8(clear);
    let resolved = mm3e_orchestrator::post::resolve(&linear, 24, 24, &scene.post).to_rgba8(clear);
    assert_eq!(direct, resolved);
    scene.post.exposure = 0.1;
    scene.mode = RenderMode::Normal;
    assert_eq!(linear, mm3e_orchestrator::render_linear(&scene, &camera));
}
