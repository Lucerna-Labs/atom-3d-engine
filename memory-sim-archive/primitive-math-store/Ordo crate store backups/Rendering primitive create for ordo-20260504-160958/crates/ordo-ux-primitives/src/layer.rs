//! Layer grouping for renderer-neutral primitive trees.

use crate::{Bounds, Clip, HitRegion, Primitive, Transform};

/// A group of primitives with shared opacity, transform, clipping, or metadata.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Layer {
    /// Optional application-defined layer name.
    pub name: Option<String>,
    /// Child primitives in painter's order.
    pub primitives: Vec<Primitive>,
    /// Optional bounds for culling or layout bookkeeping.
    pub bounds: Option<Bounds>,
    /// Transform applied to all children.
    pub transform: Transform,
    /// Optional clipping applied to all children.
    pub clip: Option<Clip>,
    /// Shared layer opacity from `0.0` to `1.0`.
    pub opacity: f32,
    /// Optional interactive region associated with the whole layer.
    pub hit_region: Option<HitRegion>,
}

impl Layer {
    /// Creates an empty layer.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a layer with child primitives.
    #[must_use]
    pub fn with_primitives(primitives: Vec<Primitive>) -> Self {
        Self {
            primitives,
            ..Self::default()
        }
    }

    /// Assigns an application-defined layer name.
    #[must_use]
    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets culling or layout bounds.
    #[must_use]
    pub fn with_bounds(mut self, bounds: Bounds) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// Sets a shared layer transform.
    #[must_use]
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    /// Sets clipping for all children.
    #[must_use]
    pub fn with_clip(mut self, clip: Clip) -> Self {
        self.clip = Some(clip);
        self
    }

    /// Sets shared layer opacity.
    #[must_use]
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Appends a primitive to the layer.
    pub fn push(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            name: None,
            primitives: Vec::new(),
            bounds: None,
            transform: Transform::default(),
            clip: None,
            opacity: 1.0,
            hit_region: None,
        }
    }
}
