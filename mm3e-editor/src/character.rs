//! A procedural, editable humanoid blockout. This is an authoring recipe, not a rig,
//! an anatomical model, or a learned character generator.
use crate::model::{identifier, range, vector, Combination, Entity, Modifiers, Shape, Surface, V3};

pub fn humanoid(id: &str, height: f32, build: f32, head_scale: f32, origin: V3) -> Result<Vec<Entity>, String> {
    identifier(id)?;
    range(height, 0.2, 10.0, "height")?;
    range(build, 0.6, 1.6, "build")?;
    range(head_scale, 0.7, 1.5, "head_scale")?;
    vector(origin, "origin")?;
    let point = |p: V3| std::array::from_fn(|i| origin[i] + p[i] * height);
    let extent = |p: V3| p.map(|v| v * height);
    let mut objects = vec![];
    let mut add = |role: &str, shape: Shape, position: V3, smooth: bool, material: Surface| {
        objects.push(Entity {
            id: format!("{id}/{role}"),
            label: role.replace('_', " "),
            role: role.into(),
            group: id.into(),
            shape,
            position,
            rotation_degrees: [0.0; 3],
            scale: 1.0,
            material,
            combine: if smooth { Combination::Smooth { radius: 0.016 * height } } else { Combination::Union },
            modifiers: Modifiers::default(),
        });
    };
    let clay = Surface::default();
    add(
        "pelvis",
        Shape::Ellipsoid { radii: extent([0.102 * build, 0.102, 0.073 * build]) },
        point([0.0, 0.49, 0.0]),
        false,
        clay.clone(),
    );
    add(
        "abdomen",
        Shape::Ellipsoid { radii: extent([0.08 * build, 0.10, 0.062 * build]) },
        point([0.0, 0.59, 0.0]),
        true,
        clay.clone(),
    );
    add(
        "chest",
        Shape::Ellipsoid { radii: extent([0.125 * build, 0.13, 0.078 * build]) },
        point([0.0, 0.715, 0.0]),
        true,
        clay.clone(),
    );
    add(
        "neck",
        Shape::Capsule { a: extent([0.0, -0.028, 0.0]), b: extent([0.0, 0.028, 0.0]), radius: 0.039 * height },
        point([0.0, 0.83, 0.0]),
        true,
        clay.clone(),
    );
    let head_center = 0.905;
    add(
        "head",
        Shape::Ellipsoid { radii: extent([0.067 * head_scale, 0.095 * head_scale, 0.072 * head_scale]) },
        point([0.0, head_center, 0.005]),
        true,
        clay.clone(),
    );
    for (side, sign) in [("left", 1.0f32), ("right", -1.0)] {
        let hip = [sign * 0.062 * build, 0.47, 0.0];
        let knee = [sign * 0.066 * build, 0.285, 0.01];
        let ankle = [sign * 0.069 * build, 0.075, 0.0];
        let shoulder = [sign * 0.115 * build, 0.755, 0.0];
        let elbow = [sign * (0.115 * build + 0.095), 0.635, 0.0];
        let wrist = [sign * (0.115 * build + 0.165), 0.50, 0.01];
        for (part, a, b, radius) in [
            ("thigh", hip, knee, 0.052 * build),
            ("shin", knee, ankle, 0.035 * build),
            ("upper_arm", shoulder, elbow, 0.033 * build),
            ("forearm", elbow, wrist, 0.025 * build),
        ] {
            add(
                &format!("{side}_{part}"),
                Shape::Capsule { a: extent(a), b: extent(b), radius: radius * height },
                origin,
                true,
                clay.clone(),
            );
        }
        add(
            &format!("{side}_foot"),
            Shape::Ellipsoid { radii: extent([0.042, 0.035, 0.079]) },
            point([ankle[0], 0.035, 0.034]),
            true,
            clay.clone(),
        );
        add(
            &format!("{side}_hand"),
            Shape::Ellipsoid { radii: extent([0.027, 0.045, 0.019]) },
            point([wrist[0] + sign * 0.011, 0.46, 0.01]),
            true,
            clay.clone(),
        );
        let dark = Surface { albedo: [0.015, 0.035, 0.045], roughness: 0.4, ..clay.clone() };
        add(
            &format!("{side}_eye"),
            Shape::Sphere { radius: 0.009 * height },
            point([sign * 0.026 * head_scale, head_center + 0.014 * head_scale, 0.072 * head_scale]),
            false,
            dark,
        );
    }
    Ok(objects)
}
