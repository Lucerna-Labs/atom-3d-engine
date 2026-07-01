//! Hit-test metadata carried beside visual primitives.

use crate::{Bounds, Shape};

/// Describes an interactive area associated with a primitive.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HitRegion {
    /// Stable application-defined identifier for the region.
    pub id: String,
    /// Coarse axis-aligned bounds for fast hit-test rejection.
    pub bounds: Bounds,
    /// Optional exact shape used after the coarse bounds check.
    pub shape: Option<Shape>,
    /// Optional cursor hint for shells that support pointer cursors.
    pub cursor: Option<CursorHint>,
    /// Whether the region should currently receive hit-test results.
    pub enabled: bool,
}

impl HitRegion {
    /// Creates an enabled hit region with bounds.
    #[must_use]
    pub fn new(id: impl Into<String>, bounds: Bounds) -> Self {
        Self {
            id: id.into(),
            bounds,
            shape: None,
            cursor: None,
            enabled: true,
        }
    }

    /// Attaches a precise hit-test shape.
    #[must_use]
    pub fn with_shape(mut self, shape: Shape) -> Self {
        self.shape = Some(shape);
        self
    }

    /// Attaches a cursor hint.
    #[must_use]
    pub fn with_cursor(mut self, cursor: CursorHint) -> Self {
        self.cursor = Some(cursor);
        self
    }

    /// Marks the region as disabled.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// Backend-neutral pointer cursor intent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CursorHint {
    /// The platform default cursor.
    Default,
    /// A pointing-hand cursor for clickable content.
    Pointer,
    /// A text insertion cursor.
    Text,
    /// A grab cursor for draggable content.
    Grab,
    /// A resize cursor on the horizontal axis.
    ResizeX,
    /// A resize cursor on the vertical axis.
    ResizeY,
}
