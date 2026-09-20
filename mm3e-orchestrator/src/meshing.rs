//! Preserve hard Boolean boundaries for extraction without changing scene evaluation.
//!
//! Positive scales and offsets distribute through finite min/max. Negation exchanges
//! min and max; a shell is max(f,-f)-thickness. Leaf evaluation keeps the original
//! sequence of f32 transforms and distance operations. Smooth unions remain opaque
//! scalar channels, with their original complete subtree evaluated at each sample.
use crate::{Combine, Object, Prim, Scene};
use mm3e_kit::{csg::Expr, meshing_boolean::BooleanExpr, sdf, vec::Transform, Vec3};
use std::sync::Arc;

const MAX_SOURCE_NODES: usize = 65_536;

fn validate_object(scene: &Scene, object: Object) -> Result<(), String> {
    let m = object.mods;
    let finite = |p: Vec3| p.x.is_finite() && p.y.is_finite() && p.z.is_finite();
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
        return Err("Boolean field source requires finite domain modifiers and nonnegative elongation".into());
    }
    let shape = match object.prim {
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
        Prim::Volume { id } => {
            scene.volumes.get(id as usize).ok_or("unregistered volume in Boolean field")?.validate()?;
            Expr::Sphere { r: 1.0 }
        }
        Prim::Surface { id } => {
            scene.surfaces.get(id as usize).ok_or("unregistered surface in Boolean field")?;
            Expr::Sphere { r: 1.0 }
        }
        Prim::Csg { id } => {
            scene.csgs.get(id as usize).ok_or("unregistered CSG in Boolean field")?.validate()?;
            Expr::Sphere { r: 1.0 }
        }
    };
    // Reuse the kit's actual primitive and proper rigid-transform contract. The
    // dummy leaf for registered geometry only validates the outer transform.
    Expr::Transform { shape: Box::new(shape), xform: object.xform }
        .validate()
        .map_err(|error| format!("invalid Boolean field source: {error}"))?;
    if matches!(object.combine,Combine::Smooth(k) if !k.is_finite()||k<0.0) {
        return Err("Boolean field source requires a finite nonnegative smooth radius".into());
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum Geometry<'a> {
    Primitive(Prim),
    Expression(&'a Expr),
}

struct Source<'a> {
    scene: &'a Scene,
    object: Object,
    geometry: Geometry<'a>,
    transforms: Vec<Transform>,
}
impl Source<'_> {
    fn sample(&self, point: Vec3) -> f32 {
        let mut local = self.object.local_point(point);
        let finite = |p: Vec3| p.x.is_finite() && p.y.is_finite() && p.z.is_finite();
        if !finite(local) {
            return f32::NAN;
        }
        for transform in &self.transforms {
            local = transform.to_local(local);
            if !finite(local) {
                return f32::NAN;
            }
        }
        match self.geometry {
            Geometry::Primitive(primitive) => {
                primitive.distance(local, &self.scene.volumes, &self.scene.csgs, &self.scene.surfaces)
            }
            Geometry::Expression(expr) => expr.distance(local),
        }
    }
    fn work(&self) -> u64 {
        let base = match self.geometry {
            Geometry::Primitive(Prim::Surface { id }) => {
                self.scene.surfaces[id as usize].triangles().len().max(1) as u64
            }
            Geometry::Primitive(Prim::Volume { .. }) => 8,
            _ => 1,
        };
        base + 1 + self.transforms.len() as u64
    }
}

enum Value<'a> {
    Source(Source<'a>),
    Min(Arc<Self>, Arc<Self>),
    Max(Arc<Self>, Arc<Self>),
    Neg(Arc<Self>),
    Scale(Arc<Self>, f32),
    Add(Arc<Self>, f32),
    Shell(Arc<Self>, f32),
    Smooth(Arc<Self>, Arc<Self>, f32),
}
impl Value<'_> {
    fn sample(&self, p: Vec3) -> f32 {
        match self {
            Self::Source(source) => source.sample(p),
            Self::Min(a, b) | Self::Max(a, b) | Self::Smooth(a, b, _) => {
                let a = a.sample(p);
                let b = b.sample(p);
                if !a.is_finite() || !b.is_finite() {
                    return f32::NAN;
                }
                match self {
                    Self::Min(..) => a.min(b),
                    Self::Max(..) => a.max(b),
                    Self::Smooth(_, _, k) => sdf::smooth_union(sdf::Field::new(a, 0), sdf::Field::new(b, 0), *k).dist,
                    _ => unreachable!(),
                }
            }
            Self::Neg(a) => -a.sample(p),
            Self::Scale(a, s) => a.sample(p) * s,
            Self::Add(a, x) => a.sample(p) + x,
            Self::Shell(a, t) => a.sample(p).abs() - t,
        }
    }
    fn work(&self) -> u64 {
        match self {
            Self::Source(source) => source.work(),
            Self::Min(a, b) | Self::Max(a, b) | Self::Smooth(a, b, _) => a.work() + b.work() + 1,
            Self::Neg(a) | Self::Scale(a, _) | Self::Add(a, _) | Self::Shell(a, _) => a.work() + 1,
        }
    }
}

#[derive(Clone, Copy)]
enum Post {
    Neg,
    Scale(f32),
    Add(f32),
}
struct Channel<'a> {
    source: Arc<Value<'a>>,
    post: Vec<Post>,
}
impl Channel<'_> {
    fn sample(&self, p: Vec3) -> f32 {
        let mut value = self.source.sample(p);
        for op in &self.post {
            value = match op {
                Post::Neg => -value,
                Post::Scale(s) => value * s,
                Post::Add(x) => value + x,
            };
        }
        value
    }
}

/// Immutable evaluated-scene adapter. Material ownership remains in `Scene::sample_authored`;
/// these scalar channels carry geometric boundaries only.
pub struct BooleanField<'a> {
    expression: BooleanExpr,
    channels: Vec<Channel<'a>>,
    work: u64,
    opaque_smooth_channels: usize,
}
impl<'a> BooleanField<'a> {
    pub fn from_scene(scene: &'a Scene) -> Result<Self, String> {
        if scene.objects.is_empty() || scene.objects.len() > 512 {
            return Err("Boolean field lowering requires 1..512 scene objects".into());
        }
        let mut builder = Builder { nodes: 0 };
        let mut value = None;
        for &object in &scene.objects {
            validate_object(scene, object)?;
            if !object.xform.scale.is_finite()
                || object.xform.scale <= 0.0
                || !object.mods.round.is_finite()
                || !object.mods.onion.is_finite()
            {
                return Err(
                    "Boolean field lowering requires positive finite scale and finite distance modifiers".into()
                );
            }
            let mut current = match object.prim {
                Prim::Csg { id } => {
                    let expression =
                        scene.csgs.get(id as usize).ok_or("unregistered CSG expression in Boolean field")?;
                    expression.validate()?;
                    builder.expression(scene, object, expression, &[])?
                }
                Prim::Surface { id } => {
                    scene.surfaces.get(id as usize).ok_or("unregistered surface in Boolean field")?;
                    builder.node(Value::Source(Source {
                        scene,
                        object,
                        geometry: Geometry::Primitive(object.prim),
                        transforms: vec![],
                    }))?
                }
                Prim::Volume { id } => {
                    scene.volumes.get(id as usize).ok_or("unregistered volume in Boolean field")?;
                    builder.node(Value::Source(Source {
                        scene,
                        object,
                        geometry: Geometry::Primitive(object.prim),
                        transforms: vec![],
                    }))?
                }
                _ => builder.node(Value::Source(Source {
                    scene,
                    object,
                    geometry: Geometry::Primitive(object.prim),
                    transforms: vec![],
                }))?,
            };
            current = builder.node(Value::Scale(current, object.xform.scale))?;
            if object.mods.round != 0.0 {
                current = builder.node(Value::Add(current, -object.mods.round))?;
            }
            if object.mods.onion != 0.0 {
                current = builder.shell(current, object.mods.onion)?;
            }
            value = Some(match value {
                None => current,
                Some(previous) => match object.combine {
                    Combine::Union => builder.node(Value::Min(previous, current))?,
                    Combine::Subtract => {
                        let negative = builder.node(Value::Neg(current))?;
                        builder.node(Value::Max(previous, negative))?
                    }
                    Combine::Smooth(k) => {
                        if !k.is_finite() || k < 0.0 {
                            return Err("Boolean field lowering requires finite nonnegative smooth radius".into());
                        }
                        if k == 0.0 {
                            builder.node(Value::Min(previous, current))?
                        } else {
                            builder.node(Value::Smooth(previous, current, k))?
                        }
                    }
                },
            });
        }
        let mut channels = vec![];
        let expression = balance(lower(value.as_ref().unwrap(), &[], &mut channels)?);
        let work = channels.iter().map(|channel| channel.source.work() + channel.post.len() as u64).sum();
        let opaque_smooth_channels =
            channels.iter().filter(|channel| matches!(channel.source.as_ref(), Value::Smooth(..))).count();
        Ok(Self { expression, channels, work, opaque_smooth_channels })
    }
    pub fn expression(&self) -> &BooleanExpr {
        &self.expression
    }
    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }
    /// Conservative primitive/transform/scalar operation count for one full channel sample.
    pub fn estimated_work_per_point(&self) -> u64 {
        self.work
    }
    pub fn opaque_smooth_channels(&self) -> usize {
        self.opaque_smooth_channels
    }
    pub fn sample(&self, point: Vec3, values: &mut [f32]) {
        if values.len() != self.channels.len() || !point.x.is_finite() || !point.y.is_finite() || !point.z.is_finite() {
            values.fill(f32::NAN);
            return;
        }
        for (value, channel) in values.iter_mut().zip(&self.channels) {
            *value = channel.sample(point);
        }
    }
    pub fn scalar(&self, point: Vec3) -> Result<f32, String> {
        let mut values = vec![0.0; self.channel_count()];
        self.sample(point, &mut values);
        self.expression.evaluate(&values)
    }
}

struct Builder {
    nodes: usize,
}
impl Builder {
    fn node<'a>(&mut self, value: Value<'a>) -> Result<Arc<Value<'a>>, String> {
        self.nodes += 1;
        if self.nodes > MAX_SOURCE_NODES {
            return Err("Boolean source lowering exceeds node budget".into());
        }
        Ok(Arc::new(value))
    }
    fn shell<'a>(&mut self, source: Arc<Value<'a>>, thickness: f32) -> Result<Arc<Value<'a>>, String> {
        self.node(Value::Shell(source, thickness))
    }
    fn expression<'a>(
        &mut self,
        scene: &'a Scene,
        object: Object,
        expr: &'a Expr,
        transforms: &[Transform],
    ) -> Result<Arc<Value<'a>>, String> {
        Ok(match expr {
            Expr::Transform { shape, xform } => {
                let mut nested = transforms.to_vec();
                nested.push(*xform);
                let child = self.expression(scene, object, shape, &nested)?;
                self.node(Value::Scale(child, xform.scale))?
            }
            Expr::Union { a, b }
            | Expr::Intersect { a, b }
            | Expr::Subtract { a, b }
            | Expr::SmoothUnion { a, b, .. } => {
                let left = self.expression(scene, object, a, transforms)?;
                let right = self.expression(scene, object, b, transforms)?;
                match expr {
                    Expr::Union { .. } => self.node(Value::Min(left, right))?,
                    Expr::Intersect { .. } => self.node(Value::Max(left, right))?,
                    Expr::Subtract { .. } => {
                        let negative = self.node(Value::Neg(right))?;
                        self.node(Value::Max(left, negative))?
                    }
                    Expr::SmoothUnion { k, .. } => {
                        if *k == 0.0 {
                            self.node(Value::Min(left, right))?
                        } else {
                            self.node(Value::Smooth(left, right, *k))?
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Expr::Offset { shape, distance } => {
                let child = self.expression(scene, object, shape, transforms)?;
                self.node(Value::Add(child, -distance))?
            }
            Expr::Shell { shape, thickness } => {
                let child = self.expression(scene, object, shape, transforms)?;
                self.shell(child, *thickness)?
            }
            _ => self.node(Value::Source(Source {
                scene,
                object,
                geometry: Geometry::Expression(expr),
                transforms: transforms.to_vec(),
            }))?,
        })
    }
}

fn lower<'a>(value: &Arc<Value<'a>>, post: &[Post], channels: &mut Vec<Channel<'a>>) -> Result<BooleanExpr, String> {
    match value.as_ref() {
        Value::Shell(child, thickness) => {
            let mut positive = vec![Post::Add(-thickness)];
            positive.extend_from_slice(post);
            let mut negative = vec![Post::Neg, Post::Add(-thickness)];
            negative.extend_from_slice(post);
            let a = Box::new(lower(child, &positive, channels)?);
            let b = Box::new(lower(child, &negative, channels)?);
            let reversed = post.iter().filter(|op| matches!(op, Post::Neg)).count() % 2 != 0;
            Ok(if reversed { BooleanExpr::Union(a, b) } else { BooleanExpr::Intersection(a, b) })
        }
        Value::Min(a, b) | Value::Max(a, b) => {
            let negative = post.iter().filter(|op| matches!(op, Post::Neg)).count() % 2 != 0;
            let intersection = matches!(value.as_ref(), Value::Max(..)) != negative;
            let a = Box::new(lower(a, post, channels)?);
            let b = Box::new(lower(b, post, channels)?);
            Ok(if intersection { BooleanExpr::Intersection(a, b) } else { BooleanExpr::Union(a, b) })
        }
        Value::Neg(child) | Value::Scale(child, _) | Value::Add(child, _) => {
            let op = match value.as_ref() {
                Value::Neg(_) => Post::Neg,
                Value::Scale(_, s) => Post::Scale(*s),
                Value::Add(_, x) => Post::Add(*x),
                _ => unreachable!(),
            };
            let mut combined = Vec::with_capacity(post.len() + 1);
            combined.push(op);
            combined.extend_from_slice(post);
            lower(child, &combined, channels)
        }
        Value::Source(_) | Value::Smooth(..) => {
            if channels.len() >= mm3e_kit::meshing_boolean::MAX_BOOLEAN_CHANNELS {
                return Err("Boolean field exceeds 128 scalar channels after shell/offset expansion".into());
            }
            let index = channels.len();
            channels.push(Channel { source: value.clone(), post: post.to_vec() });
            Ok(BooleanExpr::Leaf(index))
        }
    }
}

/// Hard min/max are associative for finite inputs. Balancing consecutive operators
/// preserves authored Boolean meaning and avoids depth growth for ordinary large unions.
fn balance(expr: BooleanExpr) -> BooleanExpr {
    fn gather(expr: BooleanExpr, intersection: bool, out: &mut Vec<BooleanExpr>) {
        match expr {
            BooleanExpr::Union(a, b) if !intersection => {
                gather(*a, intersection, out);
                gather(*b, intersection, out);
            }
            BooleanExpr::Intersection(a, b) if intersection => {
                gather(*a, intersection, out);
                gather(*b, intersection, out);
            }
            other => out.push(balance(other)),
        }
    }
    fn join(mut items: Vec<BooleanExpr>, intersection: bool) -> BooleanExpr {
        if items.len() == 1 {
            return items.pop().unwrap();
        }
        let right = items.split_off(items.len() / 2);
        let a = Box::new(join(items, intersection));
        let b = Box::new(join(right, intersection));
        if intersection {
            BooleanExpr::Intersection(a, b)
        } else {
            BooleanExpr::Union(a, b)
        }
    }
    match expr {
        BooleanExpr::Union(a, b) => {
            let mut nodes = vec![];
            gather(*a, false, &mut nodes);
            gather(*b, false, &mut nodes);
            join(nodes, false)
        }
        BooleanExpr::Intersection(a, b) => {
            let mut nodes = vec![];
            gather(*a, true, &mut nodes);
            gather(*b, true, &mut nodes);
            join(nodes, true)
        }
        BooleanExpr::Subtract(a, b) => BooleanExpr::Subtract(Box::new(balance(*a)), Box::new(balance(*b))),
        leaf => leaf,
    }
}
