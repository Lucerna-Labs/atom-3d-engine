//! Image — a reference to image data for rendering.

use cap_geometry::{Bounds, Corners, Pixels};

/// A reference to image data for rendering.
///
/// The actual pixel data lives elsewhere (texture atlas, file, memory).
/// This is just a reference that tells the renderer which image to draw
/// and where.
#[derive(Clone, Debug, PartialEq)]
pub struct ImageRef {
    /// Unique identifier for this image source.
    pub source: ImageSource,
    /// Where to draw the image.
    pub bounds: Bounds<Pixels>,
    /// Optional corner radii for clipping the image.
    pub corner_radii: Corners<Pixels>,
    /// Opacity (0.0 = fully transparent, 1.0 = fully opaque).
    pub opacity: f32,
    /// Whether to render the image in grayscale.
    pub grayscale: bool,
}

impl ImageRef {
    /// Create an image reference with full opacity and no rounding.
    pub fn new(source: ImageSource, bounds: Bounds<Pixels>) -> Self {
        ImageRef {
            source,
            bounds,
            corner_radii: Corners::default(),
            opacity: 1.0,
            grayscale: false,
        }
    }

    /// Set corner radii.
    pub fn with_corner_radii(mut self, radii: Corners<Pixels>) -> Self {
        self.corner_radii = radii;
        self
    }

    /// Set opacity.
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Set grayscale.
    pub fn with_grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
}

/// Identifies where an image's pixel data comes from.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImageSource {
    /// Image identified by a string key (file path, URL, asset name).
    Key(String),
    /// Image identified by a numeric texture/atlas ID.
    AtlasId(u64),
}

#[cfg(test)]
mod tests {
    use super::*;
    use cap_geometry::{point, size};

    #[test]
    fn test_image_ref_creation() {
        let img = ImageRef::new(
            ImageSource::Key("icon.png".into()),
            Bounds::new(
                point(Pixels(0.0), Pixels(0.0)),
                size(Pixels(32.0), Pixels(32.0)),
            ),
        );
        assert_eq!(img.opacity, 1.0);
        assert!(!img.grayscale);
    }

    #[test]
    fn test_image_ref_builder() {
        let img = ImageRef::new(
            ImageSource::AtlasId(42),
            Bounds::new(
                point(Pixels(10.0), Pixels(10.0)),
                size(Pixels(100.0), Pixels(50.0)),
            ),
        )
        .with_opacity(0.5)
        .with_grayscale(true);

        assert!((img.opacity - 0.5).abs() < 0.01);
        assert!(img.grayscale);
    }
}
