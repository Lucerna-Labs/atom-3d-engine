use mm3e_kit::{
    csg::Expr,
    sdf::Field,
    surface::TriangleSurface,
    vec::{Mat3, Transform, Vec3},
    volume::SdfVolume,
};
use mm3e_orchestrator::{sampling::CountedSceneField, Combine, Object, Prim, Scene};

fn triangle() -> TriangleSurface {
    TriangleSurface::new(
        vec![Vec3::new(-1.0, -1.0, 0.0), Vec3::new(1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
        vec![[0, 1, 2]],
        0.01,
    )
    .unwrap()
}
fn same(got: Field, want: Field) {
    assert_eq!(got.dist.to_bits(), want.dist.to_bits(), "{got:?} != {want:?}");
    assert_eq!(got.mat, want.mat);
}
fn field_scene() -> Scene {
    let mut scene = Scene::new(8, 8);
    let id = scene.surface(triangle()).unwrap();
    scene.add(
        Object::new(
            Prim::Surface { id },
            Transform::new(Vec3::new(0.4, -0.2, 0.3), Mat3::from_euler(0.2, 0.4, -0.1), 1.7),
            7,
        )
        .mirror(true, false, true)
        .elongate(Vec3::new(0.02, 0.01, 0.0))
        .repeat(Vec3::new(4.0, 0.0, 0.0))
        .twist(0.15)
        .bend(-0.08)
        .round(0.01)
        .onion(0.003),
    );
    let shape = Expr::Subtract {
        a: Box::new(Expr::Sphere { r: 1.2 }),
        b: Box::new(Expr::Transform {
            shape: Box::new(Expr::RoundBox { half: Vec3::new(0.3, 0.7, 0.7), radius: 0.05 }),
            xform: Transform::new(Vec3::new(0.35, 0.0, 0.6), Mat3::from_euler(0.0, 0.3, 0.1), 0.8),
        }),
    };
    let id = scene.csg(shape).unwrap();
    scene.add(Object::new(Prim::Csg { id }, Transform::at(Vec3::new(-0.4, 0.15, -0.2)), 11).smooth(0.06));
    scene.add(Object::new(Prim::Box { half: Vec3::new(0.3, 0.4, 0.5) }, Transform::IDENTITY, 13).subtract());
    let volume = SdfVolume::new(
        (2, 2, 2),
        Vec3::new(-0.2, -0.2, -0.2),
        Vec3::splat(0.4),
        vec![-0.2, 0.1, 0.3, 0.2, 0.4, 0.2, 0.5, 0.7],
    )
    .unwrap();
    let id = scene.volume(volume);
    scene.add(Object::new(Prim::Volume { id }, Transform::at(Vec3::new(2.0, 0.3, 0.0)), 19));
    scene
}

#[test]
fn exact_authored_distance_and_material_with_surface_domain_modifiers_csg_and_volume() {
    let scene = field_scene();
    let sampler = CountedSceneField::new(&scene).unwrap();
    let mut materials = std::collections::BTreeSet::new();
    for z in -6..=6 {
        for y in -7..=7 {
            for x in -9..=12 {
                let point = Vec3::new(x as f32 * 0.17, y as f32 * 0.16, z as f32 * 0.19);
                let result = sampler.sample(point, usize::MAX).unwrap();
                same(result.field, scene.sample_authored(point));
                assert!(result.work >= sampler.minimum_work_per_point());
                same(sampler.sample(point, result.work).unwrap().field, result.field);
                assert!(sampler.sample(point, result.work - 1).is_err());
                materials.insert(result.field.mat);
            }
        }
    }
    assert!(materials.len() >= 3, "field checks must exercise multiple real material owners: {materials:?}");
}

#[test]
fn single_triangle_charges_actual_node_triangle_domain_scale_and_seed() {
    let mut scene = Scene::new(1, 1);
    let id = scene.surface(triangle()).unwrap();
    scene.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, 9));
    let sampler = CountedSceneField::new(&scene).unwrap();
    // One domain, one root AABB, one triangle, one world scale, one authored seed.
    assert_eq!(sampler.minimum_work_per_point(), 5);
    let result = sampler.sample(Vec3::new(0.0, 0.0, 0.25), 5).unwrap();
    assert_eq!(result.work, 5);
    same(result.field, scene.sample_authored(Vec3::new(0.0, 0.0, 0.25)));
    assert!(sampler.sample(Vec3::new(0.0, 0.0, 0.25), 4).is_err());
}

#[test]
fn analytic_csg_nodes_and_volume_corners_have_precomputed_constant_costs() {
    let mut scene = Scene::new(1, 1);
    scene.add(Object::new(Prim::Sphere { r: 0.5 }, Transform::IDENTITY, 1));
    let id = scene
        .csg(Expr::Union {
            a: Box::new(Expr::Sphere { r: 0.3 }),
            b: Box::new(Expr::Transform {
                shape: Box::new(Expr::Box { half: Vec3::splat(0.2) }),
                xform: Transform::at(Vec3::new(0.5, 0.0, 0.0)),
            }),
        })
        .unwrap();
    scene.add(Object::new(Prim::Csg { id }, Transform::IDENTITY, 2));
    // Reuse the same registered CSG object. Setup validates/counts its four nodes once;
    // each query still charges all four nodes for both actual evaluations.
    scene.add(Object::new(Prim::Csg { id }, Transform::at(Vec3::new(0.0, 0.4, 0.0)), 3));
    let id = scene.volume(SdfVolume::new((2, 2, 2), Vec3::ZERO, Vec3::splat(1.0), vec![0.2; 8]).unwrap());
    scene.add(Object::new(Prim::Volume { id }, Transform::at(Vec3::new(2.0, 0.0, 0.0)), 4));
    let sampler = CountedSceneField::new(&scene).unwrap();
    // Domain+scale+fold add three to primitive(1), CSG(4), CSG(4), and volume(8).
    assert_eq!(sampler.minimum_work_per_point(), 29);
    for point in [Vec3::ZERO, Vec3::new(0.3, 0.4, 0.5), Vec3::new(4.0, -2.0, 8.0)] {
        let result = sampler.sample(point, 29).unwrap();
        assert_eq!(result.work, 29);
        same(result.field, scene.sample_authored(point));
        assert!(sampler.sample(point, 28).is_err());
    }
}

#[test]
fn material_ties_and_first_object_fold_semantics_match_authored_scene() {
    let mut scene = Scene::new(1, 1);
    let mut first = Object::new(Prim::Sphere { r: 1.0 }, Transform::IDENTITY, 3);
    // The first object seeds the authored fold regardless of its combination flag.
    first.combine = Combine::Subtract;
    scene.add(first);
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::IDENTITY, 7));
    let sampler = CountedSceneField::new(&scene).unwrap();
    for point in [Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.1, 0.4, 0.3)] {
        same(sampler.sample(point, 100).unwrap().field, scene.sample_authored(point));
    }
}

#[test]
fn empty_field_cost_is_nonzero_and_nonfinite_queries_fail_explicitly() {
    let empty = Scene::new(1, 1);
    let sampler = CountedSceneField::new(&empty).unwrap();
    assert_eq!(sampler.minimum_work_per_point(), 1);
    let result = sampler.sample(Vec3::ZERO, 1).unwrap();
    assert_eq!(result.work, 1);
    same(result.field, Field::FAR);
    assert!(sampler.sample(Vec3::ZERO, 0).is_err());
    assert!(sampler.sample(Vec3::new(f32::NAN, 0.0, 0.0), 100).is_err());
    assert!(sampler.sample(Vec3::new(0.0, f32::INFINITY, 0.0), 100).is_err());
    let scene = field_scene();
    let sampler = CountedSceneField::new(&scene).unwrap();
    assert!(sampler.sample(Vec3::new(0.0, 0.0, f32::NEG_INFINITY), usize::MAX).is_err());
    // Finite input whose domain arithmetic overflows must not pass a fabricated value.
    assert!(sampler.sample(Vec3::splat(f32::MAX), usize::MAX).is_err());
}

#[test]
fn invalid_registered_sources_transforms_and_geometry_are_rejected_before_sampling() {
    for primitive in [
        Prim::Surface { id: 0 },
        Prim::Csg { id: 0 },
        Prim::Volume { id: 0 },
        Prim::Sphere { r: f32::NAN },
        Prim::Sphere { r: 0.0 },
    ] {
        let mut scene = Scene::new(1, 1);
        scene.add(Object::new(primitive, Transform::IDENTITY, 0));
        assert!(CountedSceneField::new(&scene).is_err(), "{primitive:?}");
    }
    let mut scene = Scene::new(1, 1);
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::IDENTITY, 0));
    for transform in [
        Transform::new(Vec3::ZERO, Mat3::IDENTITY, 0.0),
        Transform::new(Vec3::ZERO, Mat3::IDENTITY, -1.0),
        Transform::new(Vec3::new(f32::NAN, 0.0, 0.0), Mat3::IDENTITY, 1.0),
        Transform::new(
            Vec3::ZERO,
            Mat3::from_cols(Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
            1.0,
        ),
    ] {
        scene.objects[0].xform = transform;
        assert!(CountedSceneField::new(&scene).is_err());
    }
    scene.objects[0].xform = Transform::IDENTITY;
    scene.objects[0].mods.bend = f32::NAN;
    assert!(CountedSceneField::new(&scene).is_err());
    scene.objects[0].mods.bend = 0.0;
    scene.objects[0].combine = Combine::Smooth(f32::NAN);
    assert!(CountedSceneField::new(&scene).is_err());
    let mut scene = Scene::new(1, 1);
    scene.volumes.push(SdfVolume { dims: (2, 2, 2), min: Vec3::ZERO, cell: Vec3::splat(1.0), data: vec![f32::NAN; 8] });
    scene.add(Object::new(Prim::Volume { id: 0 }, Transform::IDENTITY, 0));
    assert!(CountedSceneField::new(&scene).is_err());
    let mut scene = Scene::new(1, 1);
    scene.csgs.push(Expr::Sphere { r: -1.0 });
    scene.add(Object::new(Prim::Csg { id: 0 }, Transform::IDENTITY, 0));
    assert!(CountedSceneField::new(&scene).is_err());
}
