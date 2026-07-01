//! Primitive draw instructions consumed by renderer backends.

use crate::{Bounds, Clip, Fill, HitRegion, ImageRef, Layer, Shape, Stroke, TextRun, Transform};

/// A renderer-neutral Ordo UX primitive.
///
/// Backends translate these values into draw calls. This enum intentionally
/// avoids Vello, WGPU, Winit, and windowing concepts.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Primitive {
    /// Fill a shape.
    Fill {
        /// Shape to fill.
        shape: Shape,
        /// Fill style.
        fill: Fill,
        /// Local transform.
        transform: Transform,
        /// Optional clip.
        clip: Option<Clip>,
        /// Optional known bounds.
        bounds: Option<Bounds>,
        /// Optional interactive region.
        hit_region: Option<HitRegion>,
    },
    /// Stroke a shape.
    Stroke {
        /// Shape to stroke.
        shape: Shape,
        /// Stroke style.
        stroke: Stroke,
        /// Local transform.
        transform: Transform,
        /// Optional clip.
        clip: Option<Clip>,
        /// Optional known bounds.
        bounds: Option<Bounds>,
        /// Optional interactive region.
        hit_region: Option<HitRegion>,
    },
    /// Draw a run of text.
    Text(TextRun),
    /// Draw referenced image content.
    Image(ImageRef),
    /// Draw child primitives as a grouped layer.
    Layer(Layer),
}

impl Primitive {
    /// Creates a fill primitive.
    #[must_use]
    pub fn fill(shape: Shape, fill: Fill) -> Self {
        Self::Fill {
            shape,
            fill,
            transform: Transform::default(),
            clip: None,
            bounds: None,
            hit_region: None,
        }
    }

    /// Creates a stroke primitive.
    #[must_use]
    pub fn stroke(shape: Shape, stroke: Stroke) -> Self {
        Self::Stroke {
            shape,
            stroke,
            transform: Transform::default(),
            clip: None,
            bounds: None,
            hit_region: None,
        }
    }

    /// Creates a text primitive.
    #[must_use]
    pub fn text(text: TextRun) -> Self {
        Self::Text(text)
    }

    /// Creates an image primitive.
    #[must_use]
    pub fn image(image: ImageRef) -> Self {
        Self::Image(image)
    }

    /// Creates a layer primitive.
    #[must_use]
    pub fn layer(layer: Layer) -> Self {
        Self::Layer(layer)
    }

    /// Sets the local transform for primitives that carry one directly.
    #[must_use]
    pub fn with_transform(mut self, transform: Transform) -> Self {
        match &mut self {
            Self::Fill {
                transform: current, ..
            }
            | Self::Stroke {
                transform: current, ..
            } => *current = transform,
            Self::Text(text) => text.transform = transform,
            Self::Image(image) => image.transform = transform,
            Self::Layer(layer) => layer.transform = transform,
        }
        self
    }

    /// Sets clipping for primitives that carry clipping directly.
    #[must_use]
    pub fn with_clip(mut self, clip: Clip) -> Self {
        match &mut self {
            Self::Fill { clip: current, .. } | Self::Stroke { clip: current, .. } => {
                *current = Some(clip);
            }
            Self::Text(text) => text.clip = Some(clip),
            Self::Image(image) => image.clip = Some(clip),
            Self::Layer(layer) => layer.clip = Some(clip),
        }
        self
    }

    /// Sets known bounds for primitives that carry bounds directly.
    #[must_use]
    pub fn with_bounds(mut self, bounds: Bounds) -> Self {
        match &mut self {
            Self::Fill {
                bounds: current, ..
            }
            | Self::Stroke {
                bounds: current, ..
            } => *current = Some(bounds),
            Self::Text(text) => text.bounds = Some(bounds),
            Self::Image(image) => image.bounds = bounds,
            Self::Layer(layer) => layer.bounds = Some(bounds),
        }
        self
    }

    /// Sets an interactive region for primitives that carry one directly.
    #[must_use]
    pub fn with_hit_region(mut self, hit_region: HitRegion) -> Self {
        match &mut self {
            Self::Fill {
                hit_region: current,
                ..
            }
            | Self::Stroke {
                hit_region: current,
                ..
            } => *current = Some(hit_region),
            Self::Text(text) => text.hit_region = Some(hit_region),
            Self::Image(image) => image.hit_region = Some(hit_region),
            Self::Layer(layer) => layer.hit_region = Some(hit_region),
        }
        self
    }

    /// Returns the best-known axis-aligned bounds for this primitive.
    #[must_use]
    pub fn bounds(&self) -> Option<Bounds> {
        match self {
            Self::Fill {
                shape,
                transform,
                bounds,
                ..
            } => Some(
                bounds
                    .unwrap_or_else(|| shape.bounds())
                    .transformed(*transform),
            ),
            Self::Stroke {
                shape,
                stroke,
                transform,
                bounds,
                ..
            } => {
                let bounds = bounds.unwrap_or_else(|| {
                    shape
                        .bounds()
                        .inflate(stroke.width / 2.0, stroke.width / 2.0)
                });
                Some(bounds.transformed(*transform))
            }
            Self::Text(text) => text.bounds.map(|bounds| bounds.transformed(text.transform)),
            Self::Image(image) => Some(image.bounds.transformed(image.transform)),
            Self::Layer(layer) => layer
                .bounds
                .map(|bounds| bounds.transformed(layer.transform)),
        }
    }
}
