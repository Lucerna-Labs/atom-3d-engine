//! Serializable object-local shape construction. Operations affect one entity only.
use crate::model::{vec, Shape, V3};
use mm3e_kit::{
    csg::Expr,
    vec::{Mat3, Transform},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

fn one() -> f32 {
    1.0
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum CsgExpr {
    Primitive {
        shape: Box<Shape>,
    },
    Transform {
        shape: Box<CsgExpr>,
        #[serde(default)]
        position: V3,
        #[serde(default)]
        rotation_degrees: V3,
        #[serde(default = "one")]
        scale: f32,
    },
    Union {
        a: Box<CsgExpr>,
        b: Box<CsgExpr>,
    },
    Intersect {
        a: Box<CsgExpr>,
        b: Box<CsgExpr>,
    },
    Subtract {
        a: Box<CsgExpr>,
        b: Box<CsgExpr>,
    },
    SmoothUnion {
        a: Box<CsgExpr>,
        b: Box<CsgExpr>,
        radius: f32,
    },
    Offset {
        shape: Box<CsgExpr>,
        distance: f32,
    },
    /// HALF-thickness: abs(child field) - thickness.
    Shell {
        shape: Box<CsgExpr>,
        thickness: f32,
    },
}

impl CsgExpr {
    pub fn primitive(shape: Shape) -> Self {
        Self::Primitive { shape: Box::new(shape) }
    }
    pub fn compile(&self) -> Result<Expr, String> {
        fn build(node: &CsgExpr, depth: usize, count: &mut usize) -> Result<Expr, String> {
            *count += 1;
            if depth > 64 || *count > 1024 {
                return Err("local CSG exceeds depth 64 or 1024 nodes".into());
            }
            let mut child = |node: &CsgExpr| build(node, depth + 1, count).map(Box::new);
            Ok(match node {
                CsgExpr::Primitive { shape } => {
                    if matches!(shape.as_ref(), Shape::Csg { .. }) {
                        return Err(
                            "CSG primitive leaf cannot contain another CSG wrapper; use its expression directly".into(),
                        );
                    }
                    from_shape(shape)?
                }
                CsgExpr::Transform { shape, position, rotation_degrees, scale } => {
                    crate::model::vector(*position, "CSG position")?;
                    crate::model::vector(*rotation_degrees, "CSG rotation")?;
                    crate::model::range(*scale, 0.001, 1000.0, "CSG scale")?;
                    let angles = rotation_degrees.map(f32::to_radians);
                    Expr::Transform {
                        shape: child(shape)?,
                        xform: Transform::new(
                            vec(*position),
                            Mat3::from_euler(angles[0], angles[1], angles[2]),
                            *scale,
                        ),
                    }
                }
                CsgExpr::Union { a, b } => Expr::Union { a: child(a)?, b: child(b)? },
                CsgExpr::Intersect { a, b } => Expr::Intersect { a: child(a)?, b: child(b)? },
                CsgExpr::Subtract { a, b } => Expr::Subtract { a: child(a)?, b: child(b)? },
                CsgExpr::SmoothUnion { a, b, radius } => Expr::SmoothUnion { a: child(a)?, b: child(b)?, k: *radius },
                CsgExpr::Offset { shape, distance } => Expr::Offset { shape: child(shape)?, distance: *distance },
                CsgExpr::Shell { shape, thickness } => Expr::Shell { shape: child(shape)?, thickness: *thickness },
            })
        }
        let expression = build(self, 1, &mut 0)?;
        expression.validate()?;
        Ok(expression)
    }
}

pub fn from_shape(shape: &Shape) -> Result<Expr, String> {
    let expression = match shape {
        Shape::Sphere { radius } => Expr::Sphere { r: *radius },
        Shape::Box { half_extents } => Expr::Box { half: vec(*half_extents) },
        Shape::RoundBox { half_extents, radius } => Expr::RoundBox { half: vec(*half_extents), radius: *radius },
        Shape::Torus { major, minor } => Expr::Torus { major: *major, minor: *minor },
        Shape::Cylinder { half_height, radius } => Expr::Cylinder { h: *half_height, r: *radius },
        Shape::Capsule { a, b, radius } => Expr::Capsule { a: vec(*a), b: vec(*b), r: *radius },
        Shape::Cone { bottom_radius, top_radius, height } => {
            Expr::Cone { r1: *bottom_radius, r2: *top_radius, h: *height }
        }
        Shape::Ellipsoid { radii } => Expr::Ellipsoid { r: vec(*radii) },
        Shape::Octahedron { size } => Expr::Octahedron { s: *size },
        Shape::HexPrism { radius, half_height } => Expr::HexPrism { r: *radius, h: *half_height },
        Shape::Plane { normal, offset } => Expr::Plane { n: vec(*normal), h: *offset },
        Shape::Csg { expression } => return expression.compile(),
        Shape::Volume { .. } | Shape::Surface { .. } => {
            return Err("sampled volumes and triangle surfaces are not supported inside analytic local CSG".into())
        }
    };
    expression.validate()?;
    Ok(expression)
}
