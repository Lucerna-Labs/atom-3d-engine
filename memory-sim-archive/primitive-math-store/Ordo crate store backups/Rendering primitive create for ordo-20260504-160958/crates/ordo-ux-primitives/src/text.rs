//! Text primitives without font rasterization or layout backend bindings.

use crate::{Bounds, Clip, Fill, HitRegion, Transform};
use kurbo::Point;

/// A run of text with common styling and placement.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextRun {
    /// Text content.
    pub text: String,
    /// Baseline origin in logical UI units.
    pub origin: Point,
    /// Font family requested by the application.
    pub font_family: String,
    /// Font size in logical UI units.
    pub font_size: f64,
    /// Font weight using CSS-like numeric values.
    pub font_weight: u16,
    /// Font style.
    pub font_style: FontStyle,
    /// Text alignment hint for renderers or layout systems.
    pub align: TextAlign,
    /// Fill used to paint glyphs.
    pub fill: Fill,
    /// Optional layout bounds.
    pub bounds: Option<Bounds>,
    /// Local transform.
    pub transform: Transform,
    /// Optional clipping applied to this text run.
    pub clip: Option<Clip>,
    /// Optional interactive region associated with this text.
    pub hit_region: Option<HitRegion>,
}

impl TextRun {
    /// Creates a text run at an origin with default styling.
    #[must_use]
    pub fn new(text: impl Into<String>, origin: Point) -> Self {
        Self {
            text: text.into(),
            origin,
            font_family: "System".to_string(),
            font_size: 14.0,
            font_weight: 400,
            font_style: FontStyle::Normal,
            align: TextAlign::Start,
            fill: Fill::default(),
            bounds: None,
            transform: Transform::default(),
            clip: None,
            hit_region: None,
        }
    }

    /// Sets the font family and size.
    #[must_use]
    pub fn with_font(mut self, family: impl Into<String>, size: f64) -> Self {
        self.font_family = family.into();
        self.font_size = size;
        self
    }

    /// Sets numeric font weight.
    #[must_use]
    pub fn with_weight(mut self, weight: u16) -> Self {
        self.font_weight = weight;
        self
    }

    /// Sets font style.
    #[must_use]
    pub fn with_style(mut self, style: FontStyle) -> Self {
        self.font_style = style;
        self
    }

    /// Sets text alignment.
    #[must_use]
    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets glyph fill.
    #[must_use]
    pub fn with_fill(mut self, fill: Fill) -> Self {
        self.fill = fill;
        self
    }

    /// Sets layout bounds.
    #[must_use]
    pub fn with_bounds(mut self, bounds: Bounds) -> Self {
        self.bounds = Some(bounds);
        self
    }
}

/// Font style intent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStyle {
    /// Normal upright text.
    #[default]
    Normal,
    /// Italic text.
    Italic,
}

/// Text alignment intent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextAlign {
    /// Leading edge in the current writing direction.
    #[default]
    Start,
    /// Centered text.
    Center,
    /// Trailing edge in the current writing direction.
    End,
}
