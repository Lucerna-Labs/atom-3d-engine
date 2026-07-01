//! Transform values shared across primitive types.

use crate::Bounds;
use kurbo::{Affine, Vec2};

/// A local affine transform in logical UI units.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transform {
    /// The affine transform matrix.
    pub affine: Affine,
}

impl Transform {
    /// Identity transform.
    pub const IDENTITY: Self = Self {
        affine: Affine::IDENTITY,
    };

    /// Creates a transform from a Kurbo affine matrix.
    #[must_use]
    pub fn new(affine: Affine) -> Self {
        Self { affine }
    }

    /// Creates a translation transform.
    #[must_use]
    pub fn translate(x: f64, y: f64) -> Self {
        Self::new(Affine::translate(Vec2::new(x, y)))
    }

    /// Creates a uniform scale transform.
    #[must_use]
    pub fn scale(scale: f64) -> Self {
        Self::new(Affine::scale(scale))
    }

    /// Creates a non-uniform scale transform.
    #[must_use]
    pub fn scale_non_uniform(scale_x: f64, scale_y: f64) -> Self {
        Self::new(Affine::scale_non_uniform(scale_x, scale_y))
    }

    /// Creates a rotation transform, in radians.
    #[must_use]
    pub fn rotate(radians: f64) -> Self {
        Self::new(Affine::rotate(radians))
    }

    /// Applies another transform after this one.
    #[must_use]
    pub fn then(self, next: Transform) -> Self {
        Self::new(self.affine * next.affine)
    }

    /// Returns transformed axis-aligned bounds.
    #[must_use]
    pub fn transform_bounds(self, bounds: Bounds) -> Bounds {
        bounds.transformed(self)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<Affine> for Transform {
    fn from(affine: Affine) -> Self {
        Self::new(affine)
    }
}
