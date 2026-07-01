//! Shape — geometric primitives that can be filled and stroked.

use crate::{Fill, Stroke};
use cap_geometry::{Bounds, Corners, Pixels, Point, Size};

/// A renderable shape.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    /// A rectangle, optionally with rounded corners.
    Rect(RectShape),
    /// A circle (center + radius).
    Circle(CircleShape),
    /// An ellipse (center + radii).
    Ellipse(EllipseShape),
    /// A line segment.
    Line(LineShape),
    /// A rounded rectangle with per-corner radii.
    RoundedRect(RoundedRectShape),
    /// An arbitrary path of cubic bezier segments.
    Path(PathShape),
}

impl Shape {
    /// Get the bounding box of this shape.
    pub fn bounds(&self) -> Bounds<Pixels> {
        match self {
            Shape::Rect(s) => s.bounds(),
            Shape::Circle(s) => s.bounds(),
            Shape::Ellipse(s) => s.bounds(),
            Shape::Line(s) => s.bounds(),
            Shape::RoundedRect(s) => s.bounds(),
            Shape::Path(s) => s.bounds,
        }
    }
}

/// A filled and/or stroked rectangle.
#[derive(Clone, Debug, PartialEq)]
pub struct RectShape {
    /// Origin and size.
    pub rect: Bounds<Pixels>,
    /// Fill style.
    pub fill: Fill,
    /// Stroke style.
    pub stroke: Stroke,
}

impl RectShape {
    pub fn bounds(&self) -> Bounds<Pixels> {
        self.rect
    }
}

/// A filled and/or stroked circle.
#[derive(Clone, Debug, PartialEq)]
pub struct CircleShape {
    /// Center point.
    pub center: Point<Pixels>,
    /// Radius.
    pub radius: Pixels,
    /// Fill style.
    pub fill: Fill,
    /// Stroke style.
    pub stroke: Stroke,
}

impl CircleShape {
    pub fn bounds(&self) -> Bounds<Pixels> {
        Bounds::centered_at(self.center, Size::new(self.radius * 2.0, self.radius * 2.0))
    }
}

/// A filled and/or stroked ellipse.
#[derive(Clone, Debug, PartialEq)]
pub struct EllipseShape {
    /// Center point.
    pub center: Point<Pixels>,
    /// Horizontal radius.
    pub radius_x: Pixels,
    /// Vertical radius.
    pub radius_y: Pixels,
    /// Fill style.
    pub fill: Fill,
    /// Stroke style.
    pub stroke: Stroke,
}

impl EllipseShape {
    pub fn bounds(&self) -> Bounds<Pixels> {
        Bounds::centered_at(
            self.center,
            Size::new(self.radius_x * 2.0, self.radius_y * 2.0),
        )
    }
}

/// A stroked line segment.
#[derive(Clone, Debug, PartialEq)]
pub struct LineShape {
    /// Start point.
    pub start: Point<Pixels>,
    /// End point.
    pub end: Point<Pixels>,
    /// Stroke style.
    pub stroke: Stroke,
}

impl LineShape {
    pub fn bounds(&self) -> Bounds<Pixels> {
        let min = self.start.min(&self.end);
        let max = self.start.max(&self.end);
        let half_w = self.stroke.width * 0.5;
        Bounds::from_corners(
            Point::new(min.x - half_w, min.y - half_w),
            Point::new(max.x + half_w, max.y + half_w),
        )
    }
}

/// A rounded rectangle with per-corner radii.
#[derive(Clone, Debug, PartialEq)]
pub struct RoundedRectShape {
    /// Origin and size.
    pub rect: Bounds<Pixels>,
    /// Per-corner radii.
    pub corner_radii: Corners<Pixels>,
    /// Fill style.
    pub fill: Fill,
    /// Stroke style.
    pub stroke: Stroke,
}

impl RoundedRectShape {
    pub fn bounds(&self) -> Bounds<Pixels> {
        self.rect
    }
}

/// An arbitrary path of cubic bezier segments.
#[derive(Clone, Debug, PartialEq)]
pub struct PathShape {
    /// SVG-style path data string.
    pub d: String,
    /// Bounding box (precomputed or estimated).
    pub bounds: Bounds<Pixels>,
    /// Fill style.
    pub fill: Fill,
    /// Stroke style.
    pub stroke: Stroke,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fill::Rgba;

    #[test]
    fn test_rect_shape_bounds() {
        let rect = Bounds::new(
            Point::new(Pixels(10.0), Pixels(20.0)),
            Size::new(Pixels(100.0), Pixels(50.0)),
        );
        let shape = RectShape {
            rect,
            fill: Fill::solid_hex(0xFF0000FF),
            stroke: Stroke::default(),
        };
        assert_eq!(shape.bounds(), rect);
    }

    #[test]
    fn test_circle_shape_bounds() {
        let shape = CircleShape {
            center: Point::new(Pixels(50.0), Pixels(50.0)),
            radius: Pixels(25.0),
            fill: Fill::None,
            stroke: Stroke::default(),
        };
        let b = shape.bounds();
        assert!((b.origin.x.0 - 25.0).abs() < 0.01);
        assert!((b.size.width.0 - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_line_shape_bounds() {
        let shape = LineShape {
            start: Point::new(Pixels(0.0), Pixels(0.0)),
            end: Point::new(Pixels(100.0), Pixels(50.0)),
            stroke: Stroke::solid(Pixels(2.0), Rgba::WHITE),
        };
        let b = shape.bounds();
        assert!(b.origin.x.0 < 0.0); // includes stroke width
    }
}
