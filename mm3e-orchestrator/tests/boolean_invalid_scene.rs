//! Regression for invalid public Scene values that f32 min/max can hide.
use mm3e_kit::{vec::Transform, Vec3};
use mm3e_orchestrator::{meshing::BooleanField, Object, Prim, Scene};

#[test]
fn lowering_rejects_nonfinite_transforms_and_domain_modifiers_before_scalar_sampling() {
    type InvalidCase = (&'static str, fn(&mut Object));
    let invalid: [InvalidCase; 6] = [
        ("translation", |object| object.xform.pos.x = f32::NAN),
        ("rotation", |object| object.xform.rot.cols[0].x = f32::NAN),
        ("twist", |object| object.mods.twist = f32::NAN),
        ("bend", |object| object.mods.bend = f32::NAN),
        ("elongate", |object| object.mods.elongate.x = f32::NAN),
        ("repeat", |object| object.mods.repeat.x = f32::NAN),
    ];
    let mut accepted = Vec::new();
    for (name, corrupt) in invalid {
        let mut object = Object::new(Prim::Box { half: Vec3::ONE }, Transform::IDENTITY, 0);
        corrupt(&mut object);
        let mut scene = Scene::new(8, 8);
        scene.add(object);
        if let Ok(program) = BooleanField::from_scene(&scene) {
            accepted.push(format!("{name}: {:?}", program.scalar(Vec3::ZERO)));
        }
    }
    assert!(accepted.is_empty(), "invalid scenes accepted (some masked into finite geometry): {accepted:?}");
}

#[test]
fn malformed_primitives_and_registered_buffers_cannot_hide_inside_a_union() {
    let primitives = [
        Prim::Box { half: Vec3::new(f32::NAN, 1.0, 1.0) },
        Prim::Sphere { r: -1.0 },
        Prim::Plane { n: Vec3::new(0.0, 2.0, 0.0), h: 0.0 },
        Prim::Volume { id: 42 },
        Prim::Csg { id: 42 },
        Prim::Surface { id: 42 },
    ];
    for prim in primitives {
        let mut scene = Scene::new(8, 8);
        scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::IDENTITY, 0));
        scene.add(Object::new(prim, Transform::IDENTITY, 0));
        assert!(BooleanField::from_scene(&scene).is_err(), "accepted malformed primitive {prim:?}");
    }
    let mut scene = Scene::new(8, 8);
    scene.volumes.push(mm3e_kit::volume::SdfVolume {
        dims: (2, 2, 2),
        min: Vec3::ZERO,
        cell: Vec3::ONE,
        data: vec![0.0],
    });
    scene.add(Object::new(Prim::Volume { id: 0 }, Transform::IDENTITY, 0));
    assert!(BooleanField::from_scene(&scene).is_err());
}

#[test]
fn nonfinite_query_points_are_rejected_instead_of_masked_by_box_min_max() {
    let mut scene = Scene::new(8, 8);
    scene.add(Object::new(Prim::Box { half: Vec3::ONE }, Transform::IDENTITY, 0));
    let field = BooleanField::from_scene(&scene).unwrap();
    for x in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(field.scalar(Vec3::new(x, 0.0, 0.0)).is_err());
    }
    assert_eq!(field.scalar(Vec3::ZERO).unwrap(), -1.0);
}

#[test]
fn overflow_in_a_finite_domain_transform_cannot_be_hidden_in_an_opaque_smooth_subtree() {
    let mut scene = Scene::new(8, 8);
    scene.add(Object::new(Prim::Box { half: Vec3::ONE }, Transform::IDENTITY, 0).twist(f32::MAX));
    scene.add(Object::new(Prim::Sphere { r: 0.75 }, Transform::IDENTITY, 0));
    scene.add(Object::new(Prim::Sphere { r: 0.25 }, Transform::at(Vec3::new(2.0, 0.0, 0.0)), 0).smooth(0.1));
    let field = BooleanField::from_scene(&scene).unwrap();
    assert!(field.scalar(Vec3::ZERO).unwrap().is_finite());
    assert!(
        field.scalar(Vec3::new(0.2, 2.0, 0.1)).is_err(),
        "nonfinite mapped points must not become finite box/min geometry"
    );
}
