//! Shape vocabulary shared by renderers and hit testing.

use crate::{Bounds, Transform};
use kurbo::{BezPath, Circle, Line, Point, Rect, RoundedRect, Shape as KurboShape};

/// Renderer-neutral geometry for visual primitives.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Shape {
    /// Axis-aligned rectangle.
    Rect(Rect),
    /// Rounded rectangle.
    RoundedRect(RoundedRect),
    /// Circle.
    Circle(Circle),
    /// Ellipse represented by center and radii.
    Ellipse {
        /// Center point of the ellipse.
        center: Point,
        /// Horizontal radius.
        radius_x: f64,
        /// Vertical radius.
        radius_y: f64,
    },
    /// Line segment.
    Line(Line),
    /// Arbitrary path.
    Path(BezPath),
}

impl Shape {
    /// Creates a rectangle shape from minimum coordinates and dimensions.
    #[must_use]
    pub fn rect(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self::Rect(Rect::from_origin_size(
            Point::new(x, y),
            kurbo::Size::new(width, height),
        ))
    }

    /// Creates a rounded rectangle shape.
    #[must_use]
    pub fn rounded_rect(x: f64, y: f64, width: f64, height: f64, radius: f64) -> Self {
        Self::RoundedRect(RoundedRect::new(x, y, x + width, y + height, radius))
    }

    /// Creates a circle shape.
    #[must_use]
    pub fn circle(center: impl Into<Point>, radius: f64) -> Self {
        Self::Circle(Circle::new(center, radius))
    }

    /// Creates an ellipse shape from center and radii.
    #[must_use]
    pub fn ellipse(center: impl Into<Point>, radius_x: f64, radius_y: f64) -> Self {
        Self::Ellipse {
            center: center.into(),
            radius_x,
            radius_y,
        }
    }

    /// Creates a line shape.
    #[must_use]
    pub fn line(p0: impl Into<Point>, p1: impl Into<Point>) -> Self {
        Self::Line(Line::new(p0, p1))
    }

    /// Creates a path shape.
    #[must_use]
    pub fn path(path: BezPath) -> Self {
        Self::Path(path)
    }

    /// Returns the shape's axis-aligned bounds.
    #[must_use]
    pub fn bounds(&self) -> Bounds {
        let rect = match self {
            Self::Rect(rect) => *rect,
            Self::RoundedRect(rect) => rect.bounding_box(),
            Self::Circle(circle) => circle.bounding_box(),
            Self::Ellipse {
                center,
                radius_x,
                radius_y,
            } => Rect::new(
                center.x - radius_x,
                center.y - radius_y,
                center.x + radius_x,
                center.y + radius_y,
            ),
            Self::Line(line) => line.bounding_box(),
            Self::Path(path) => path.bounding_box(),
        };

        Bounds::from_rect(rect)
    }
}

/// Clip geometry applied to a primitive or layer.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Clip {
    /// Shape defining the clipped area.
    pub shape: Shape,
    /// Transform applied to the clip shape.
    pub transform: Transform,
}

impl Clip {
    /// Creates a clip from a shape.
    #[must_use]
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            transform: Transform::default(),
        }
    }

    /// Sets a transform for the clip shape.
    #[must_use]
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

impl From<Bounds> for Clip {
    fn from(bounds: Bounds) -> Self {
        Self::new(Shape::Rect(bounds.rect))
    }
}
