//! Object-local analytic CSG. Each expression contributes a single material-bearing object;
//! subtraction and intersection here never affect an unrelated scene object.
//!
//! Distances retain the approximations of the underlying kit primitives. A returned bound is
//! stronger than a visual enclosure: outside it, sphere distance is a lower bound on the
//! expression's positive field, so a renderer may safely use it for pruning. `None` deliberately
//! disables pruning for formulas for which that property has not been established.
use crate::{
    sdf,
    vec::{Transform, Vec3},
};

pub const MAX_DEPTH: usize = 64;
pub const MAX_NODES: usize = 1024;

#[derive(Clone, Debug)]
pub enum Expr {
    Sphere {
        r: f32,
    },
    Box {
        half: Vec3,
    },
    RoundBox {
        half: Vec3,
        radius: f32,
    },
    Torus {
        major: f32,
        minor: f32,
    },
    Cylinder {
        h: f32,
        r: f32,
    },
    Capsule {
        a: Vec3,
        b: Vec3,
        r: f32,
    },
    Cone {
        r1: f32,
        r2: f32,
        h: f32,
    },
    Ellipsoid {
        r: Vec3,
    },
    Octahedron {
        s: f32,
    },
    HexPrism {
        r: f32,
        h: f32,
    },
    Plane {
        n: Vec3,
        h: f32,
    },
    Transform {
        shape: Box<Expr>,
        xform: Transform,
    },
    Union {
        a: Box<Expr>,
        b: Box<Expr>,
    },
    Intersect {
        a: Box<Expr>,
        b: Box<Expr>,
    },
    Subtract {
        a: Box<Expr>,
        b: Box<Expr>,
    },
    SmoothUnion {
        a: Box<Expr>,
        b: Box<Expr>,
        k: f32,
    },
    /// Positive distances expand a shape; negative distances erode it.
    Offset {
        shape: Box<Expr>,
        distance: f32,
    },
    /// Shell thickness is the half thickness in this expression's coordinate system.
    Shell {
        shape: Box<Expr>,
        thickness: f32,
    },
}

impl Expr {
    /// Evaluate an already validated expression in its own coordinate system.
    pub fn distance(&self, p: Vec3) -> f32 {
        match self {
            Self::Sphere { r } => sdf::sphere(p, *r),
            Self::Box { half } => sdf::boxed(p, *half),
            Self::RoundBox { half, radius } => sdf::rounded_box(p, *half, *radius),
            Self::Torus { major, minor } => sdf::torus(p, *major, *minor),
            Self::Cylinder { h, r } => sdf::cylinder(p, *h, *r),
            Self::Capsule { a, b, r } => sdf::capsule(p, *a, *b, *r),
            Self::Cone { r1, r2, h } => sdf::round_cone(p, *r1, *r2, *h),
            Self::Ellipsoid { r } => sdf::ellipsoid(p, *r),
            Self::Octahedron { s } => sdf::octahedron(p, *s),
            Self::HexPrism { r, h } => sdf::hex_prism(p, *r, *h),
            Self::Plane { n, h } => sdf::plane(p, *n, *h),
            Self::Transform { shape, xform } => shape.distance(xform.to_local(p)) * xform.scale,
            Self::Union { a, b } => a.distance(p).min(b.distance(p)),
            Self::Intersect { a, b } => a.distance(p).max(b.distance(p)),
            Self::Subtract { a, b } => a.distance(p).max(-b.distance(p)),
            Self::SmoothUnion { a, b, k } => {
                sdf::smooth_union(sdf::Field::new(a.distance(p), 0), sdf::Field::new(b.distance(p), 0), *k).dist
            }
            Self::Offset { shape, distance } => shape.distance(p) - distance,
            Self::Shell { shape, thickness } => shape.distance(p).abs() - thickness,
        }
    }

    pub fn bound(&self) -> Option<(Vec3, f32)> {
        match self {
            Self::Sphere { r } => Some((Vec3::ZERO, *r)),
            Self::Box { half } | Self::RoundBox { half, .. } => Some((Vec3::ZERO, half.length())),
            Self::Torus { major, minor } => Some((Vec3::ZERO, major + minor)),
            Self::Cylinder { h, r } => Some((Vec3::ZERO, (h * h + r * r).sqrt())),
            Self::Capsule { a, b, r } => Some(((*a + *b).scale(0.5), (*a - *b).length() * 0.5 + r)),
            Self::Cone { r1, r2, h } => Some((Vec3::new(0.0, h * 0.5, 0.0), h * 0.5 + r1.max(*r2))),
            // The approximate ellipsoid and octahedron fields are not exact distances;
            // geometric containment alone does not prove a valid field underestimate.
            Self::Ellipsoid { .. } | Self::Octahedron { .. } | Self::Plane { .. } => None,
            Self::HexPrism { r, h } => Some((Vec3::ZERO, (4.0 * r * r / 3.0 + h * h).sqrt())),
            Self::Transform { shape, xform } => shape.bound().map(|(c, r)| (xform.to_world(c), r * xform.scale)),
            Self::Union { a, b } | Self::SmoothUnion { a, b, .. } => {
                let (ac, ar) = a.bound()?;
                let (bc, br) = b.bound()?;
                let (c, mut r) = enclose(ac, ar, bc, br);
                if let Self::SmoothUnion { k, .. } = self {
                    r += k * 0.25;
                }
                Some((c, r))
            }
            Self::Intersect { a, b } => match (a.bound(), b.bound()) {
                (Some(aa), Some(bb)) => Some(if aa.1 <= bb.1 { aa } else { bb }),
                (aa, bb) => aa.or(bb),
            },
            Self::Subtract { a, .. } => a.bound(),
            Self::Offset { shape, distance } => shape.bound().map(|(c, r)| (c, (r + distance).max(0.0))),
            Self::Shell { shape, thickness } => shape.bound().map(|(c, r)| (c, r + thickness)),
        }
    }

    /// Check primitive domains, rigid transforms, and traversal budgets before storing an
    /// externally authored expression. Returns its node count on success. The root is depth 1.
    pub fn validate(&self) -> Result<usize, String> {
        let mut count = 0;
        self.validate_at(1, &mut count, Transform::IDENTITY)?;
        Ok(count)
    }

    fn validate_at(&self, depth: usize, count: &mut usize, parent: Transform) -> Result<(), String> {
        *count += 1;
        if depth > MAX_DEPTH || *count > MAX_NODES {
            return Err("CSG expression exceeds depth/node budget".into());
        }
        let positive = |x: f32| x.is_finite() && (1e-6..=1e6).contains(&x);
        let scalar = |x: f32| x.is_finite() && x.abs() <= 1e6;
        let finite = |v: Vec3| scalar(v.x) && scalar(v.y) && scalar(v.z);
        let extents = |v: Vec3| positive(v.x) && positive(v.y) && positive(v.z);
        let good = match self {
            Self::Sphere { r } => positive(*r),
            Self::Box { half } => extents(*half),
            Self::RoundBox { half, radius } => {
                extents(*half) && scalar(*radius) && *radius >= 0.0 && *radius <= half.x.min(half.y).min(half.z)
            }
            Self::Torus { major, minor } => positive(*major) && positive(*minor) && minor <= major,
            Self::Cylinder { h, r } => positive(*h) && positive(*r),
            Self::Capsule { a, b, r } => finite(*a) && finite(*b) && positive(*r) && (*a - *b).length() >= 1e-6,
            Self::Cone { r1, r2, h } => {
                positive(*h)
                    && scalar(*r1)
                    && scalar(*r2)
                    && *r1 >= 0.0
                    && *r2 >= 0.0
                    && r1.max(*r2) > 0.0
                    && (r1 - r2).abs() < *h
            }
            Self::Ellipsoid { r } => extents(*r),
            Self::Octahedron { s } => positive(*s),
            Self::HexPrism { r, h } => positive(*r) && positive(*h),
            Self::Plane { n, h } => finite(*n) && scalar(*h) && (n.length() - 1.0).abs() <= 1e-5,
            Self::Transform { shape, xform } => {
                let cols = xform.rot.cols;
                let rotation = cols.iter().all(|c| finite(*c) && (c.length() - 1.0).abs() <= 1e-5)
                    && cols[0].dot(cols[1]).abs() <= 1e-5
                    && cols[0].dot(cols[2]).abs() <= 1e-5
                    && cols[1].dot(cols[2]).abs() <= 1e-5
                    && (cols[0].cross(cols[1]).dot(cols[2]) - 1.0).abs() <= 1e-5;
                if !finite(xform.pos) || !positive(xform.scale) || !rotation {
                    return Err(
                        "CSG transform requires finite position, positive scale and a proper orthonormal rotation"
                            .into(),
                    );
                }
                let combined = parent.compose(*xform);
                if !positive(combined.scale) || !finite(combined.pos) {
                    return Err("CSG cumulative transform exceeds finite coordinate/scale limits".into());
                }
                shape.validate_at(depth + 1, count, combined)?;
                true
            }
            Self::Union { a, b }
            | Self::Intersect { a, b }
            | Self::Subtract { a, b }
            | Self::SmoothUnion { a, b, .. } => {
                if let Self::SmoothUnion { k, .. } = self {
                    if !scalar(*k) || *k < 0.0 {
                        return Err("CSG smooth radius must be finite and nonnegative".into());
                    }
                }
                a.validate_at(depth + 1, count, parent)?;
                b.validate_at(depth + 1, count, parent)?;
                true
            }
            Self::Offset { shape, distance } => {
                shape.validate_at(depth + 1, count, parent)?;
                scalar(*distance)
            }
            Self::Shell { shape, thickness } => {
                shape.validate_at(depth + 1, count, parent)?;
                positive(*thickness)
            }
        };
        if !good {
            return Err("invalid CSG primitive or modifier parameters".into());
        }
        if let Some((center, radius)) = self.bound() {
            if !finite(center) || !radius.is_finite() || !(0.0..=1e12).contains(&radius) {
                return Err("CSG expression has an invalid or excessive evaluated bound".into());
            }
        }
        Ok(())
    }
}

fn enclose(a: Vec3, ar: f32, b: Vec3, br: f32) -> (Vec3, f32) {
    let delta = b - a;
    let d = delta.length();
    if d + br <= ar {
        return (a, ar);
    }
    if d + ar <= br {
        return (b, br);
    }
    let r = (d + ar + br) * 0.5;
    (a + delta.scale((r - ar) / d.max(1e-12)), r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec::Mat3;

    #[test]
    fn eyelid_shell_clip_has_an_actual_opening() {
        let cap = Expr::Intersect {
            a: Box::new(Expr::Shell { shape: Box::new(Expr::Sphere { r: 1.0 }), thickness: 0.05 }),
            b: Box::new(Expr::Plane { n: Vec3::new(0.0, -1.0, 0.0), h: 0.3 }),
        };
        assert!(cap.distance(Vec3::new(0.0, 0.5, 0.8660254)) < 0.0);
        assert!(cap.distance(Vec3::new(0.0, 0.0, 1.0)) > 0.0);
        assert!(cap.distance(Vec3::ZERO) > 0.0);
        assert_eq!(cap.validate().unwrap(), 4);
    }

    #[test]
    fn transformed_subtraction_and_bounds_match_field() {
        let base = Expr::Subtract {
            a: Box::new(Expr::Sphere { r: 1.0 }),
            b: Box::new(Expr::Transform {
                shape: Box::new(Expr::Sphere { r: 0.6 }),
                xform: Transform::at(Vec3::new(0.0, 0.0, 0.8)),
            }),
        };
        let xf = Transform::new(Vec3::new(2.0, 0.0, 1.0), Mat3::from_euler(0.2, 0.7, -0.1), 1.7);
        let expr = Expr::Transform { shape: Box::new(base.clone()), xform: xf };
        expr.validate().unwrap();
        let (c, r) = expr.bound().unwrap();
        for ix in -6..=6 {
            for iy in -6..=6 {
                for iz in -6..=6 {
                    let p = Vec3::new(ix as f32 * 0.45, iy as f32 * 0.45, iz as f32 * 0.45);
                    let world = xf.to_world(p);
                    assert!((expr.distance(world) - base.distance(p) * xf.scale).abs() < 3e-6);
                    assert!((world - c).length() - r <= expr.distance(world) + 3e-6);
                }
            }
        }
    }

    #[test]
    fn clipping_subtraction_smoothing_and_shell_bounds_are_safe() {
        let a = Box::new(Expr::Sphere { r: 1.0 });
        let b = Box::new(Expr::Transform {
            shape: Box::new(Expr::Sphere { r: 0.7 }),
            xform: Transform::at(Vec3::new(0.8, 0.2, 0.0)),
        });
        for expr in [
            Expr::Union { a: a.clone(), b: b.clone() },
            Expr::Intersect { a: a.clone(), b: b.clone() },
            Expr::Subtract { a: a.clone(), b: b.clone() },
            Expr::SmoothUnion { a: a.clone(), b: b.clone(), k: 0.3 },
            Expr::Shell { shape: a.clone(), thickness: 0.2 },
            Expr::Offset { shape: a.clone(), distance: -0.3 },
            Expr::HexPrism { r: 0.8, h: 1.2 },
        ] {
            expr.validate().unwrap();
            let (c, r) = expr.bound().unwrap();
            for ix in -10..=10 {
                for iy in -10..=10 {
                    for iz in -10..=10 {
                        let p = Vec3::new(ix as f32 * 0.7, iy as f32 * 0.7, iz as f32 * 0.7);
                        // Bounds are used outside their sphere, not as exact interior fields.
                        let lower = (p - c).length() - r;
                        if lower > 0.0 {
                            assert!(lower <= expr.distance(p) + 3e-6, "{expr:?}: {lower} > {}", expr.distance(p));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn malformed_geometry_and_excessive_expression_depth_are_rejected() {
        assert!(Expr::Sphere { r: f32::NAN }.validate().is_err());
        assert!(Expr::Plane { n: Vec3::new(0.0, 2.0, 0.0), h: 0.0 }.validate().is_err());
        assert!(Expr::Capsule { a: Vec3::ZERO, b: Vec3::ZERO, r: 1.0 }.validate().is_err());
        let mut deep = Expr::Sphere { r: 1.0 };
        for _ in 0..MAX_DEPTH {
            deep = Expr::Offset { shape: Box::new(deep), distance: 0.01 };
        }
        assert!(deep.validate().is_err());
        assert!(Expr::Transform {
            shape: Box::new(Expr::Sphere { r: 1.0 }),
            xform: Transform::new(Vec3::ZERO, Mat3::IDENTITY, 0.0)
        }
        .validate()
        .is_err());
        let mut huge = Expr::Ellipsoid { r: Vec3::splat(1.0) };
        for _ in 0..8 {
            huge = Expr::Transform { shape: Box::new(huge), xform: Transform::new(Vec3::ZERO, Mat3::IDENTITY, 1000.0) };
        }
        assert!(huge.validate().is_err(), "unbounded/approximate leaves must not bypass cumulative scale limits");
    }
}
