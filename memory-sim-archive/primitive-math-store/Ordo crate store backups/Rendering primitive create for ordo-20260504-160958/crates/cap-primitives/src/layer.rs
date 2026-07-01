//! Layer — an isolated rendering group.

use crate::Clip;
use cap_geometry::{Bounds, Pixels};

/// A rendering layer — groups primitives that share a clip and compositing context.
///
/// Layers are the building blocks of the scene tree. Each layer:
/// - Has its own clip region
/// - Can have an offset (scroll, transform)
/// - Is composited as a unit (opacity, blend mode)
///
/// The order of layers in the scene determines z-order (painter's algorithm).
#[derive(Clone, Debug, PartialEq)]
pub struct Layer {
    /// The clip region for this layer.
    pub clip: Clip,
    /// Offset applied to all content in this layer.
    pub offset: cap_geometry::Point<Pixels>,
    /// Opacity for the entire layer (0.0–1.0).
    pub opacity: f32,
    /// Blend mode for compositing this layer.
    pub blend_mode: BlendMode,
}

impl Layer {
    /// Create a layer with a rectangular clip and default settings.
    pub fn new(clip_bounds: Bounds<Pixels>) -> Self {
        Layer {
            clip: Clip::rect(clip_bounds),
            offset: cap_geometry::Point::default(),
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
        }
    }

    /// Create a layer with a rounded-rect clip.
    pub fn rounded(
        clip_bounds: Bounds<Pixels>,
        corner_radii: cap_geometry::Corners<Pixels>,
    ) -> Self {
        Layer {
            clip: Clip::rounded_rect(clip_bounds, corner_radii),
            offset: cap_geometry::Point::default(),
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
        }
    }

    /// Set the layer offset.
    pub fn with_offset(mut self, offset: cap_geometry::Point<Pixels>) -> Self {
        self.offset = offset;
        self
    }

    /// Set the layer opacity.
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Set the blend mode.
    pub fn with_blend_mode(mut self, mode: BlendMode) -> Self {
        self.blend_mode = mode;
        self
    }

    /// Get the bounding box from the clip.
    pub fn bounds(&self) -> &Bounds<Pixels> {
        self.clip.bounds()
    }
}

/// Compositing blend mode for layers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cap_geometry::{Corners, point, size};

    #[test]
    fn test_layer_creation() {
        let layer = Layer::new(Bounds::new(
            point(Pixels(0.0), Pixels(0.0)),
            size(Pixels(800.0), Pixels(600.0)),
        ));
        assert_eq!(layer.opacity, 1.0);
        assert_eq!(layer.blend_mode, BlendMode::Normal);
    }

    #[test]
    fn test_layer_builder() {
        let layer = Layer::new(Bounds::new(
            point(Pixels(0.0), Pixels(0.0)),
            size(Pixels(800.0), Pixels(600.0)),
        ))
        .with_opacity(0.5)
        .with_blend_mode(BlendMode::Multiply)
        .with_offset(point(Pixels(10.0), Pixels(20.0)));

        assert!((layer.opacity - 0.5).abs() < 0.01);
        assert_eq!(layer.blend_mode, BlendMode::Multiply);
        assert_eq!(layer.offset, point(Pixels(10.0), Pixels(20.0)));
    }

    #[test]
    fn test_rounded_layer() {
        let layer = Layer::rounded(
            Bounds::new(
                point(Pixels(0.0), Pixels(0.0)),
                size(Pixels(100.0), Pixels(100.0)),
            ),
            Corners::all(Pixels(12.0)),
        );
        assert!(matches!(layer.clip, Clip::RoundedRect { .. }));
    }
}
