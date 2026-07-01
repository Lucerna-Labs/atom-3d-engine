//! Primitive — the top-level scene primitive enum.
//!
//! Every renderable element in the scene tree is a `Primitive`. This enum
//! is the union of all primitive types. Renderer backends match on these
//! to produce draw calls.

use crate::{Clip, ImageRef, Layer, Shape, TextRun, Transform, fill::Rgba};
use cap_geometry::Bounds;

/// A single renderable primitive in the scene tree.
///
/// This is the type that the scene builder produces and the renderer consumes.
/// Each variant carries enough information for any backend to produce
/// the correct visual output.
#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    /// A shape (rectangle, circle, path, etc.).
    Shape(ShapePrimitive),
    /// A run of styled text.
    Text(TextPrimitive),
    /// An image.
    Image(ImagePrimitive),
    /// A clip region (constrains subsequent primitives).
    Clip(ClipPrimitive),
    /// A layer boundary (start or end).
    Layer(LayerPrimitive),
    /// A transformation to apply to subsequent primitives.
    Transform(TransformPrimitive),
    /// A shadow (drop shadow, box shadow).
    Shadow(ShadowPrimitive),
}

impl Primitive {
    /// Get the bounding box of this primitive, if applicable.
    pub fn bounds(&self) -> Option<&Bounds<cap_geometry::Pixels>> {
        match self {
            Primitive::Shape(s) => Some(&s.bounds),
            Primitive::Text(t) => Some(&t.bounds),
            Primitive::Image(i) => Some(&i.bounds),
            Primitive::Shadow(s) => Some(&s.bounds),
            Primitive::Clip(c) => Some(c.clip.bounds()),
            Primitive::Layer(l) => Some(l.layer.bounds()),
            Primitive::Transform(_) => None,
        }
    }
}

/// A shape primitive.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapePrimitive {
    /// The shape to render.
    pub shape: Shape,
    /// Precomputed bounding box.
    pub bounds: Bounds<cap_geometry::Pixels>,
    /// Draw order (lower = behind, higher = in front).
    pub order: u32,
}

/// A text primitive.
#[derive(Clone, Debug, PartialEq)]
pub struct TextPrimitive {
    /// The text run to render.
    pub run: TextRun,
    /// Precomputed bounding box.
    pub bounds: Bounds<cap_geometry::Pixels>,
    /// Draw order.
    pub order: u32,
}

/// An image primitive.
#[derive(Clone, Debug, PartialEq)]
pub struct ImagePrimitive {
    /// The image reference.
    pub image: ImageRef,
    /// Precomputed bounding box.
    pub bounds: Bounds<cap_geometry::Pixels>,
    /// Draw order.
    pub order: u32,
}

/// A clip primitive — constrains rendering to a region.
#[derive(Clone, Debug, PartialEq)]
pub struct ClipPrimitive {
    /// The clip region.
    pub clip: Clip,
}

/// A layer primitive — begins or ends a rendering layer.
#[derive(Clone, Debug, PartialEq)]
pub struct LayerPrimitive {
    /// The layer.
    pub layer: Layer,
    /// Whether this is the start or end of the layer.
    pub kind: LayerKind,
}

/// Layer boundary type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayerKind {
    /// Start of a layer — subsequent primitives belong to this layer.
    Start,
    /// End of a layer — pop back to the parent layer.
    End,
}

/// A transform primitive — applies a 2D transform to subsequent primitives.
#[derive(Clone, Debug, PartialEq)]
pub struct TransformPrimitive {
    /// The transform to apply.
    pub transform: Transform,
}

/// A shadow primitive — rendered as a blurred, colored shape behind content.
#[derive(Clone, Debug, PartialEq)]
pub struct ShadowPrimitive {
    /// The shadow's bounding box.
    pub bounds: Bounds<cap_geometry::Pixels>,
    /// Blur radius in logical pixels.
    pub blur_radius: cap_geometry::Pixels,
    /// Shadow color.
    pub color: Rgba,
    /// Corner radii of the shadow shape.
    pub corner_radii: cap_geometry::Corners<cap_geometry::Pixels>,
    /// Draw order.
    pub order: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Fill, Stroke, fill::Rgba, shape::RectShape};
    use cap_geometry::{Corners, Pixels, point, size};

    #[test]
    fn test_shape_primitive() {
        let rect = Bounds::new(
            point(Pixels(0.0), Pixels(0.0)),
            size(Pixels(100.0), Pixels(50.0)),
        );
        let shape = ShapePrimitive {
            shape: Shape::Rect(RectShape {
                rect,
                fill: Fill::solid_hex(0xFF0000FF),
                stroke: Stroke::default(),
            }),
            bounds: rect,
            order: 0,
        };
        let prim = Primitive::Shape(shape);
        assert!(prim.bounds().is_some());
    }

    #[test]
    fn test_shadow_primitive() {
        let shadow = ShadowPrimitive {
            bounds: Bounds::new(
                point(Pixels(5.0), Pixels(5.0)),
                size(Pixels(90.0), Pixels(90.0)),
            ),
            blur_radius: Pixels(10.0),
            color: Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.3,
            },
            corner_radii: Corners::all(Pixels(8.0)),
            order: 0,
        };
        let prim = Primitive::Shadow(shadow);
        assert!(prim.bounds().is_some());
    }
}
