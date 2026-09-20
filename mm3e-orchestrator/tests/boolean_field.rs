use mm3e_kit::{
    csg::Expr,
    surface::TriangleSurface,
    vec::{Mat3, Transform},
    volume::SdfVolume,
    Vec3,
};
use mm3e_orchestrator::{meshing::BooleanField, Combine, Object, Prim, Scene};

fn transform(position: Vec3, scale: f32) -> Transform {
    Transform::new(position, Mat3::from_euler(0.37, -0.23, 0.14), scale)
}
fn sphere(r: f32) -> Box<Expr> {
    Box::new(Expr::Sphere { r })
}
fn compare(scene: &Scene, points: &[Vec3]) {
    let program = BooleanField::from_scene(scene).unwrap();
    for &p in points {
        let actual = program.scalar(p).unwrap();
        let expected = scene.sample_authored(p).dist;
        assert_eq!(actual, expected, "lowering changed the authored scalar at {p:?}: {actual} versus {expected}");
    }
}
fn points() -> Vec<Vec3> {
    let mut state = 76531_u64;
    (0..2500)
        .map(|_| {
            let mut next = || {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                ((state >> 32) as u32 as f64 / u32::MAX as f64 * 6.0 - 3.0) as f32
            };
            Vec3::new(next(), next(), next())
        })
        .collect()
}

#[test]
fn global_csg_modifiers_and_spatial_warps_retain_exact_authored_scalars() {
    let mut scene = Scene::new(8, 8);
    let mut a = Object::new(Prim::Sphere { r: 0.8 }, transform(Vec3::new(0.1, -0.2, 0.3), 1.7), 0);
    a.mods.mirror = [true, false, true];
    a.mods.elongate = Vec3::new(0.12, 0.0, 0.03);
    a.mods.round = 0.04;
    a.mods.onion = 0.12;
    scene.add(a);
    let mut b = Object::new(Prim::Torus { major: 0.8, minor: 0.21 }, transform(Vec3::new(-0.2, 0.3, 0.1), 0.7), 1);
    b.mods.repeat = Vec3::new(1.1, 0.0, 1.7);
    b.mods.twist = 0.18;
    b.mods.bend = -0.22;
    b.combine = Combine::Subtract;
    scene.add(b);
    scene.add(Object::new(Prim::Box { half: Vec3::splat(0.3) }, transform(Vec3::new(0.1, 0.2, -0.3), 0.9), 2));
    let program = BooleanField::from_scene(&scene).unwrap();
    assert_eq!(program.channel_count(), 4);
    compare(&scene, &points());
}

#[test]
fn nested_transforms_offset_shell_intersection_and_subtraction_preserve_operation_order() {
    let mut scene = Scene::new(8, 8);
    let nested = Expr::Transform {
        xform: transform(Vec3::new(0.2, 0.1, -0.5), 1.9),
        shape: Box::new(Expr::Offset {
            distance: 0.071,
            shape: Box::new(Expr::Shell {
                thickness: 0.091,
                shape: Box::new(Expr::Intersect {
                    a: Box::new(Expr::Subtract {
                        a: sphere(0.8),
                        b: Box::new(Expr::Transform {
                            xform: transform(Vec3::new(0.6, 0.0, 0.0), 0.81),
                            shape: sphere(0.6),
                        }),
                    }),
                    b: Box::new(Expr::Box { half: Vec3::new(0.4, 0.7, 0.6) }),
                }),
            }),
        }),
    };
    let id = scene.csg(nested).unwrap();
    let mut object = Object::new(Prim::Csg { id }, transform(Vec3::new(-0.3, 0.5, 0.9), 0.73), 0);
    object.mods.round = 0.013;
    object.mods.onion = 0.03;
    scene.add(object);
    let program = BooleanField::from_scene(&scene).unwrap();
    assert_eq!(program.channel_count(), 12);
    compare(&scene, &points());
}

#[test]
fn smoothing_stays_an_exact_opaque_channel_and_later_cut_remains_separate() {
    let mut scene = Scene::new(8, 8);
    let id = scene
        .csg(Expr::SmoothUnion {
            a: Box::new(Expr::Shell { shape: sphere(0.5), thickness: 0.07 }),
            b: sphere(0.3),
            k: 0.08,
        })
        .unwrap();
    scene.add(Object::new(Prim::Csg { id }, Transform::IDENTITY, 0));
    scene.add(Object::new(Prim::Sphere { r: 0.4 }, transform(Vec3::new(0.7, 0.0, 0.0), 0.9), 1).smooth(0.17));
    scene.add(Object::new(Prim::Sphere { r: 0.2 }, transform(Vec3::new(0.4, 0.0, 0.0), 1.0), 2).subtract());
    let program = BooleanField::from_scene(&scene).unwrap();
    assert_eq!(program.channel_count(), 2);
    assert_eq!(program.opaque_smooth_channels(), 1);
    compare(&scene, &points());
}

#[test]
fn native_triangles_and_volumes_remain_actual_registered_geometry() {
    let mut scene = Scene::new(8, 8);
    let volume =
        SdfVolume::new((2, 2, 2), Vec3::splat(-1.0), Vec3::splat(2.0), vec![-0.7, 0.4, 0.2, 0.8, -0.2, 0.9, 0.7, 1.4])
            .unwrap();
    let id = scene.volume(volume);
    scene.add(Object::new(Prim::Volume { id }, transform(Vec3::new(0.0, 0.2, 0.0), 0.8), 0));
    let id = scene
        .surface(
            TriangleSurface::new(
                vec![Vec3::new(-0.7, 0.0, 0.0), Vec3::new(0.7, 0.0, 0.0), Vec3::new(0.0, 0.8, 0.2)],
                vec![[0, 1, 2]],
                0.04,
            )
            .unwrap(),
        )
        .unwrap();
    scene.add(Object::new(Prim::Surface { id }, transform(Vec3::new(0.2, 0.1, 0.0), 1.3), 1).subtract());
    compare(&scene, &points());
}

#[test]
fn large_union_is_balanced_and_limits_reject_expansion_without_fallback() {
    let mut scene = Scene::new(8, 8);
    for i in 0..128 {
        scene.add(Object::new(Prim::Sphere { r: 0.1 }, Transform::at(Vec3::new(i as f32 * 0.2, 0.0, 0.0)), 0));
    }
    let program = BooleanField::from_scene(&scene).unwrap();
    assert_eq!(program.channel_count(), 128);
    assert_eq!(program.scalar(Vec3::ZERO).unwrap(), scene.sample_authored(Vec3::ZERO).dist);
    scene.add(Object::new(Prim::Sphere { r: 0.1 }, Transform::IDENTITY, 0));
    assert!(BooleanField::from_scene(&scene).err().unwrap().contains("128"));
    let mut scene = Scene::new(8, 8);
    let mut expr = Expr::Sphere { r: 0.7 };
    for _ in 0..40 {
        expr = Expr::Shell { shape: Box::new(expr), thickness: 0.01 };
    }
    let id = scene.csg(Expr::SmoothUnion { a: Box::new(expr), b: sphere(0.4), k: 0.1 }).unwrap();
    scene.add(Object::new(Prim::Csg { id }, Transform::IDENTITY, 0));
    let program = BooleanField::from_scene(&scene).unwrap();
    assert_eq!(program.channel_count(), 1);
    assert!(
        program.estimated_work_per_point() < 100,
        "opaque nested shells must not exponentially duplicate evaluation"
    );
    compare(&scene, &[Vec3::new(0.3, 0.2, 0.1)]);
}
