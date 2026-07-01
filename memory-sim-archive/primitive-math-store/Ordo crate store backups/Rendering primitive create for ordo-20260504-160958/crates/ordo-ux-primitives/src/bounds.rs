//! Bounds used by layout, clipping, rendering backends, and hit testing.

use crate::Transform;
use kurbo::{Point, Rect, Size};

/// Axis-aligned bounds in logical UI units.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bounds {
    /// The rectangle occupied by the primitive or interaction region.
    pub rect: Rect,
}

impl Bounds {
    /// Empty bounds at the origin.
    pub const ZERO: Self = Self { rect: Rect::ZERO };

    /// Creates bounds from an origin and size.
    #[must_use]
    pub fn new(origin: Point, size: Size) -> Self {
        Self {
            rect: Rect::from_origin_size(origin, size),
        }
    }

    /// Creates bounds directly from a rectangle.
    #[must_use]
    pub fn from_rect(rect: Rect) -> Self {
        Self { rect }
    }

    /// Creates bounds from minimum coordinates and dimensions.
    #[must_use]
    pub fn from_xywh(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self::from_rect(Rect::from_origin_size(
            Point::new(x, y),
            Size::new(width, height),
        ))
    }

    /// Returns the origin of the bounds.
    #[must_use]
    pub fn origin(&self) -> Point {
        self.rect.origin()
    }

    /// Returns the size of the bounds.
    #[must_use]
    pub fn size(&self) -> Size {
        self.rect.size()
    }

    /// Returns `true` when `point` is inside the bounds.
    #[must_use]
    pub fn contains(&self, point: impl Into<Point>) -> bool {
        self.rect.contains(point)
    }

    /// Returns bounds containing both rectangles.
    #[must_use]
    pub fn union(&self, other: Bounds) -> Self {
        Self::from_rect(self.rect.union(other.rect))
    }

    /// Returns bounds inflated on each axis.
    #[must_use]
    pub fn inflate(&self, width: f64, height: f64) -> Self {
        Self::from_rect(self.rect.inflate(width, height))
    }

    /// Returns the axis-aligned bounding box after applying a transform.
    #[must_use]
    pub fn transformed(&self, transform: Transform) -> Self {
        Self::from_rect(transform.affine.transform_rect_bbox(self.rect))
    }
}

impl From<Rect> for Bounds {
    fn from(rect: Rect) -> Self {
        Self::from_rect(rect)
    }
}
