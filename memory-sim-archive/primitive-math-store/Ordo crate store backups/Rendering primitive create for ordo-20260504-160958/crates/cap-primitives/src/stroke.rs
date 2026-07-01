//! Stroke — outline painting for shapes.

use crate::fill::Rgba;
use cap_geometry::Pixels;

/// A stroke style applied to a shape's outline.
#[derive(Clone, Debug, PartialEq)]
pub struct Stroke {
    /// Stroke width in logical pixels.
    pub width: Pixels,
    /// Stroke color.
    pub color: Rgba,
    /// Dash pattern. Empty = solid line.
    pub dash_pattern: Vec<f32>,
    /// Dash offset.
    pub dash_offset: f32,
}

impl Stroke {
    /// Create a solid stroke with the given width and color.
    pub fn solid(width: impl Into<Pixels>, color: Rgba) -> Self {
        Stroke {
            width: width.into(),
            color,
            dash_pattern: Vec::new(),
            dash_offset: 0.0,
        }
    }

    /// Create a dashed stroke.
    pub fn dashed(width: impl Into<Pixels>, color: Rgba, pattern: &[f32]) -> Self {
        Stroke {
            width: width.into(),
            color,
            dash_pattern: pattern.to_vec(),
            dash_offset: 0.0,
        }
    }

    /// Check if this stroke has zero width or transparent color.
    pub fn is_invisible(&self) -> bool {
        self.width.0 <= 0.0 || self.color.is_transparent()
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke::solid(Pixels::ZERO, Rgba::TRANSPARENT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_stroke() {
        let stroke = Stroke::solid(Pixels(2.0), Rgba::WHITE);
        assert_eq!(stroke.width, Pixels(2.0));
        assert!(stroke.dash_pattern.is_empty());
        assert!(!stroke.is_invisible());
    }

    #[test]
    fn test_dashed_stroke() {
        let stroke = Stroke::dashed(Pixels(1.0), Rgba::BLACK, &[4.0, 2.0]);
        assert_eq!(stroke.dash_pattern, vec![4.0, 2.0]);
    }

    #[test]
    fn test_invisible_stroke() {
        let zero_width = Stroke::solid(Pixels(0.0), Rgba::WHITE);
        let transparent = Stroke::solid(Pixels(2.0), Rgba::TRANSPARENT);
        assert!(zero_width.is_invisible());
        assert!(transparent.is_invisible());
    }
}
