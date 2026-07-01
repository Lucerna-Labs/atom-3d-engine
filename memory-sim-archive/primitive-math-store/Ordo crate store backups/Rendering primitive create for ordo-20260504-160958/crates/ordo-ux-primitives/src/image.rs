//! Image references without backend-specific texture or surface handles.

use crate::{Bounds, Clip, HitRegion, Transform};

/// A renderer-neutral reference to image content.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImageRef {
    /// Stable application-defined image identifier or asset key.
    pub id: String,
    /// Destination bounds in logical UI units.
    pub bounds: Bounds,
    /// How the image should fit inside its destination bounds.
    pub fit: ImageFit,
    /// Overall image opacity from `0.0` to `1.0`.
    pub opacity: f32,
    /// Local transform applied before backend translation.
    pub transform: Transform,
    /// Optional clipping applied to the image.
    pub clip: Option<Clip>,
    /// Optional interactive region associated with this image.
    pub hit_region: Option<HitRegion>,
}

impl ImageRef {
    /// Creates an image reference that covers its destination bounds.
    #[must_use]
    pub fn new(id: impl Into<String>, bounds: Bounds) -> Self {
        Self {
            id: id.into(),
            bounds,
            fit: ImageFit::Cover,
            opacity: 1.0,
            transform: Transform::default(),
            clip: None,
            hit_region: None,
        }
    }

    /// Sets the image fitting mode.
    #[must_use]
    pub fn with_fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    /// Sets image opacity.
    #[must_use]
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Sets a local transform.
    #[must_use]
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    /// Sets clipping for the image.
    #[must_use]
    pub fn with_clip(mut self, clip: Clip) -> Self {
        self.clip = Some(clip);
        self
    }

    /// Sets an interactive region for the image.
    #[must_use]
    pub fn with_hit_region(mut self, hit_region: HitRegion) -> Self {
        self.hit_region = Some(hit_region);
        self
    }
}

/// How image content maps into destination bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImageFit {
    /// Stretch to exactly fill the destination bounds.
    Stretch,
    /// Preserve aspect ratio and cover the destination bounds.
    Cover,
    /// Preserve aspect ratio and fit entirely inside the destination bounds.
    Contain,
    /// Draw at natural size, anchored at the destination origin.
    None,
}
