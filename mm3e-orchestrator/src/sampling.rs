//! Bounded evaluation of the exact authored field, with actual native surface BVH work.
//!
//! This adapter does not replace source geometry with acceleration bounds. Arithmetic and
//! material ownership follow `Scene::sample_authored`; only the work accounting is added.
//! Work units are domain/scale/modifier/fold operations, analytic CSG nodes, eight volume
//! corner reads, and the native surface's actual AABB/triangle tests, not CPU instructions.
use crate::{Combine, Object, Prim, Scene};
use mm3e_kit::{
    csg::Expr,
    sdf::{self, Field},
    Vec3,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug)]
pub struct CountedField {
    pub field: Field,
    pub work: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct CountedOwner {
    pub field: Field,
    pub owner: Option<usize>,
    pub work: usize,
}

#[derive(Clone, Copy)]
struct Cost {
    /// Outer coordinate transform plus each enabled domain modifier.
    domain: usize,
    /// None means native surface traversal, charged by its bounded query itself.
    geometry: Option<usize>,
    /// World scale, enabled distance modifiers, and the authored fold/seed.
    post: usize,
}

/// Immutable snapshot adapter; source validation and constant CSG costs run once at setup.
pub struct CountedSceneField<'a> {
    scene: &'a Scene,
    costs: Vec<Cost>,
    minimum: usize,
}

impl<'a> CountedSceneField<'a> {
    pub fn new(scene: &'a Scene) -> Result<Self, String> {
        let mut costs = Vec::with_capacity(scene.objects.len());
        let mut csg_costs = BTreeMap::new();
        let mut validated_volumes = BTreeSet::new();
        let mut minimum = 0usize;
        for (index, object) in scene.objects.iter().enumerate() {
            validate_object(object).map_err(|error| format!("counted field object {index}: {error}"))?;
            let geometry = match object.prim {
                Prim::Surface { id } => {
                    scene
                        .surfaces
                        .get(id as usize)
                        .ok_or_else(|| format!("unregistered surface {id} in counted field"))?;
                    None
                }
                Prim::Csg { id } => {
                    let cost = match csg_costs.get(&id) {
                        Some(&cost) => cost,
                        None => {
                            let expr = scene
                                .csgs
                                .get(id as usize)
                                .ok_or_else(|| format!("unregistered CSG {id} in counted field"))?;
                            let count = expr.validate()?;
                            csg_costs.insert(id, count);
                            count
                        }
                    };
                    Some(cost)
                }
                Prim::Volume { id } => {
                    let volume = scene
                        .volumes
                        .get(id as usize)
                        .ok_or_else(|| format!("unregistered volume {id} in counted field"))?;
                    if validated_volumes.insert(id) {
                        volume.validate()?;
                    }
                    Some(8)
                }
                primitive => {
                    primitive_expression(primitive).validate()?;
                    Some(1)
                }
            };
            let m = &object.mods;
            let domain = 1
                + usize::from(m.mirror.iter().any(|v| *v))
                + usize::from(m.elongate != Vec3::ZERO)
                + usize::from(m.repeat != Vec3::ZERO)
                + usize::from(m.twist != 0.0)
                + usize::from(m.bend != 0.0);
            let post = 2 + usize::from(m.round != 0.0) + usize::from(m.onion != 0.0);
            let cost = Cost { domain, geometry, post };
            // A nonempty native surface must visit at least its root and one triangle.
            // This is a lower bound only; each actual traversal still has its own budget.
            minimum = minimum
                .checked_add(domain)
                .and_then(|n| n.checked_add(geometry.unwrap_or(2)))
                .and_then(|n| n.checked_add(post))
                .ok_or("counted field minimum work overflow")?;
            costs.push(cost);
        }
        Ok(Self { scene, costs, minimum: minimum.max(1) })
    }

    /// Guaranteed lower bound for one query, suitable for a cheap grid preflight.
    /// Native BVH traversal usually costs more; callers must use each returned actual count.
    /// An empty scene charges one unit to construct its authored `Field::FAR` result.
    pub fn minimum_work_per_point(&self) -> usize {
        self.minimum
    }

    /// Charge every operation before evaluation and fail without a partial field result.
    /// For finite accepted results the distance bits and material match `sample_authored`.
    /// Nonfinite queries, transformed points and evaluated nonempty fields are errors.
    pub fn sample(&self, point: Vec3, max_work: usize) -> Result<CountedField, String> {
        self.sample_owner(point, max_work).map(|sample| CountedField { field: sample.field, work: sample.work })
    }

    /// The same counted fold with exact material-owner provenance. Hard-union
    /// ties keep the previous owner; subtractors retain the base owner.
    pub fn sample_owner(&self, point: Vec3, max_work: usize) -> Result<CountedOwner, String> {
        if !finite(point) {
            return Err("counted field query point must be finite".into());
        }
        if max_work < self.minimum {
            return Err("counted field work budget exhausted before query".into());
        }
        if self.scene.objects.is_empty() {
            return Ok(CountedOwner { field: Field::FAR, owner: None, work: 1 });
        }
        let mut work = 0usize;
        let mut accumulated: Option<Field> = None;
        let mut owner = None;
        for (index, (object, cost)) in self.scene.objects.iter().zip(&self.costs).enumerate() {
            charge(&mut work, max_work, cost.domain)?;
            let local = object.local_point(point);
            if !finite(local) {
                return Err("counted field transformed query point is not finite".into());
            }
            let distance = match object.prim {
                Prim::Surface { id } => {
                    let result = self.scene.surfaces[id as usize].distance_bounded(local, max_work - work)?;
                    // The core has already precharged every test within this remaining cap.
                    work += result.work;
                    result.distance
                }
                _ => {
                    charge(&mut work, max_work, cost.geometry.ok_or("missing constant field cost")?)?;
                    object.prim.distance(local, &self.scene.volumes, &self.scene.csgs, &self.scene.surfaces)
                }
            };
            charge(&mut work, max_work, cost.post)?;
            let mut distance = distance * object.xform.scale;
            if object.mods.round != 0.0 {
                distance = sdf::op_round(distance, object.mods.round);
            }
            if object.mods.onion != 0.0 {
                distance = sdf::op_onion(distance, object.mods.onion);
            }
            if !distance.is_finite() {
                return Err("counted field evaluated object distance is not finite".into());
            }
            let field = Field::new(distance, object.mat);
            accumulated = Some(match accumulated {
                None => {
                    owner = Some(index);
                    field
                }
                Some(previous) => match object.combine {
                    Combine::Union => {
                        if previous.dist > field.dist {
                            owner = Some(index);
                        }
                        sdf::union(previous, field)
                    }
                    Combine::Smooth(k) => {
                        let keep = if k <= 0.0 {
                            previous.dist <= field.dist
                        } else {
                            (0.5 + 0.5 * (field.dist - previous.dist) / k).clamp(0.0, 1.0) > 0.5
                        };
                        if !keep {
                            owner = Some(index);
                        }
                        sdf::smooth_union(previous, field, k)
                    }
                    Combine::Subtract => sdf::subtract(previous, field),
                },
            });
            if !accumulated.is_some_and(|field| field.dist.is_finite()) {
                return Err("counted field authored fold produced a nonfinite distance".into());
            }
        }
        Ok(CountedOwner { field: accumulated.unwrap_or(Field::FAR), owner, work })
    }
}

fn charge(work: &mut usize, limit: usize, amount: usize) -> Result<(), String> {
    if amount > limit.saturating_sub(*work) {
        return Err("counted field work budget exhausted before operation".into());
    }
    *work += amount;
    Ok(())
}

fn finite(p: Vec3) -> bool {
    p.x.is_finite() && p.y.is_finite() && p.z.is_finite()
}

fn validate_object(object: &Object) -> Result<(), String> {
    let t = &object.xform;
    let columns = t.rot.cols;
    let rotation = columns.iter().all(|c| finite(*c) && (c.length() - 1.0).abs() <= 1e-5)
        && columns[0].dot(columns[1]).abs() <= 1e-5
        && columns[0].dot(columns[2]).abs() <= 1e-5
        && columns[1].dot(columns[2]).abs() <= 1e-5
        && (columns[0].cross(columns[1]).dot(columns[2]) - 1.0).abs() <= 1e-5;
    if !finite(t.pos) || !t.scale.is_finite() || t.scale <= 0.0 || !rotation {
        return Err("transform requires finite position, positive scale and proper orthonormal rotation".into());
    }
    let m = &object.mods;
    if !finite(m.elongate)
        || !finite(m.repeat)
        || !m.twist.is_finite()
        || !m.bend.is_finite()
        || !m.round.is_finite()
        || !m.onion.is_finite()
        || m.elongate.x < 0.0
        || m.elongate.y < 0.0
        || m.elongate.z < 0.0
    {
        return Err("domain modifiers must be finite, with nonnegative elongation".into());
    }
    if matches!(object.combine, Combine::Smooth(k) if !k.is_finite() || k < 0.0) {
        return Err("smooth radius must be finite and nonnegative".into());
    }
    Ok(())
}

fn primitive_expression(primitive: Prim) -> Expr {
    match primitive {
        Prim::Sphere { r } => Expr::Sphere { r },
        Prim::Box { half } => Expr::Box { half },
        Prim::RoundBox { half, radius } => Expr::RoundBox { half, radius },
        Prim::Torus { major, minor } => Expr::Torus { major, minor },
        Prim::Cylinder { h, r } => Expr::Cylinder { h, r },
        Prim::Capsule { a, b, r } => Expr::Capsule { a, b, r },
        Prim::Cone { r1, r2, h } => Expr::Cone { r1, r2, h },
        Prim::Ellipsoid { r } => Expr::Ellipsoid { r },
        Prim::Octahedron { s } => Expr::Octahedron { s },
        Prim::HexPrism { r, h } => Expr::HexPrism { r, h },
        Prim::Plane { n, h } => Expr::Plane { n, h },
        Prim::Surface { .. } | Prim::Csg { .. } | Prim::Volume { .. } => {
            unreachable!("registered geometry is validated in place")
        }
    }
}
