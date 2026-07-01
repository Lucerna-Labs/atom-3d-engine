//! Clip — clipping regions that constrain rendering.

use cap_geometry::{Bounds, Pixels};

/// A clipping region that constrains what's rendered inside it.
#[derive(Clone, Debug, PartialEq)]
pub enum Clip {
    /// Axis-aligned rectangular clip.
    Rect(Bounds<Pixels>),
    /// Rounded rectangle clip.
    RoundedRect {
        /// The bounding rectangle.
        rect: Bounds<Pixels>,
        /// Per-corner radii.
        corner_radii: cap_geometry::Corners<Pixels>,
    },
}

impl Clip {
    /// Create a rectangular clip.
    pub fn rect(bounds: Bounds<Pixels>) -> Self {
        Clip::Rect(bounds)
    }

    /// Create a rounded rectangle clip.
    pub fn rounded_rect(rect: Bounds<Pixels>, corner_radii: cap_geometry::Corners<Pixels>) -> Self {
        Clip::RoundedRect { rect, corner_radii }
    }

    /// Get the bounding box of this clip.
    pub fn bounds(&self) -> &Bounds<Pixels> {
        match self {
            Clip::Rect(b) => b,
            Clip::RoundedRect { rect, .. } => rect,
        }
    }

    /// Check if this clip would cull everything (empty bounds).
    pub fn is_empty(&self) -> bool {
        self.bounds().is_empty()
    }
}

impl Default for Clip {
    fn default() -> Self {
        Clip::Rect(Bounds::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cap_geometry::{point, size};

    #[test]
    fn test_rect_clip() {
        let clip = Clip::rect(Bounds::new(
            point(Pixels(0.0), Pixels(0.0)),
            size(Pixels(100.0), Pixels(100.0)),
        ));
        assert!(!clip.is_empty());
    }

    #[test]
    fn test_rounded_rect_clip() {
        let clip = Clip::rounded_rect(
            Bounds::new(
                point(Pixels(0.0), Pixels(0.0)),
                size(Pixels(50.0), Pixels(50.0)),
            ),
            cap_geometry::Corners::all(Pixels(8.0)),
        );
        assert!(!clip.is_empty());
        assert_eq!(clip.bounds().size.width, Pixels(50.0));
    }
}
