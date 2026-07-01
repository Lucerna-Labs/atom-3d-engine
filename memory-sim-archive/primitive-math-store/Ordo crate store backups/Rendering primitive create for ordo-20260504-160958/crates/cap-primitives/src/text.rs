//! Text — a run of styled text.

use crate::fill::Rgba;
use cap_geometry::{Bounds, Pixels, Point, Size};

/// A run of text with uniform style.
#[derive(Clone, Debug, PartialEq)]
pub struct TextRun {
    /// The text content.
    pub text: String,
    /// Font family name.
    pub font_family: String,
    /// Font size in logical pixels.
    pub font_size: Pixels,
    /// Font weight (100–900, 400 = normal, 700 = bold).
    pub font_weight: u16,
    /// Font style.
    pub font_style: FontStyle,
    /// Text color.
    pub color: Rgba,
    /// Position of the text baseline origin.
    pub origin: Point<Pixels>,
    /// Precomputed bounds (if available).
    pub bounds: Option<Bounds<Pixels>>,
    /// Line height override (None = use font metrics).
    pub line_height: Option<Pixels>,
    /// Letter spacing override.
    pub letter_spacing: Option<Pixels>,
    /// Underline style.
    pub underline: UnderlineStyle,
    /// Strikethrough style.
    pub strikethrough: StrikethroughStyle,
}

impl TextRun {
    /// Create a simple text run with defaults.
    pub fn new(
        text: impl Into<String>,
        font_family: impl Into<String>,
        font_size: impl Into<Pixels>,
        color: Rgba,
        origin: Point<Pixels>,
    ) -> Self {
        TextRun {
            text: text.into(),
            font_family: font_family.into(),
            font_size: font_size.into(),
            font_weight: 400,
            font_style: FontStyle::Normal,
            color,
            origin,
            bounds: None,
            line_height: None,
            letter_spacing: None,
            underline: UnderlineStyle::None,
            strikethrough: StrikethroughStyle::None,
        }
    }

    /// Set font weight.
    pub fn with_weight(mut self, weight: u16) -> Self {
        self.font_weight = weight;
        self
    }

    /// Set font style.
    pub fn with_style(mut self, style: FontStyle) -> Self {
        self.font_style = style;
        self
    }

    /// Set line height.
    pub fn with_line_height(mut self, height: impl Into<Pixels>) -> Self {
        self.line_height = Some(height.into());
        self
    }

    /// Add underline.
    pub fn with_underline(mut self, style: UnderlineStyle) -> Self {
        self.underline = style;
        self
    }

    /// Add strikethrough.
    pub fn with_strikethrough(mut self, style: StrikethroughStyle) -> Self {
        self.strikethrough = style;
        self
    }

    /// Estimate bounds from text length and font size (no shaping).
    pub fn estimated_bounds(&self) -> Bounds<Pixels> {
        let char_width = self.font_size * 0.6; // rough estimate
        let width = char_width * self.text.chars().count() as f32;
        let height = self.line_height.unwrap_or(self.font_size * 1.2);
        Bounds::new(self.origin, Size::new(width, height))
    }
}

/// Font style.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}

/// Underline decoration style.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderlineStyle {
    None,
    Solid,
    Wavy,
    Dotted,
    Dashed,
}

impl Default for UnderlineStyle {
    fn default() -> Self {
        UnderlineStyle::None
    }
}

/// Strikethrough decoration style.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrikethroughStyle {
    None,
    Solid,
}

impl Default for StrikethroughStyle {
    fn default() -> Self {
        StrikethroughStyle::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fill::Rgba;

    #[test]
    fn test_text_run_creation() {
        let run = TextRun::new(
            "Hello",
            "Inter",
            Pixels(16.0),
            Rgba::WHITE,
            Point::new(Pixels(0.0), Pixels(0.0)),
        );
        assert_eq!(run.text, "Hello");
        assert_eq!(run.font_weight, 400);
        assert_eq!(run.font_style, FontStyle::Normal);
    }

    #[test]
    fn test_text_run_builder() {
        let run = TextRun::new(
            "Bold italic",
            "Inter",
            Pixels(14.0),
            Rgba::BLACK,
            Point::new(Pixels(10.0), Pixels(20.0)),
        )
        .with_weight(700)
        .with_style(FontStyle::Italic)
        .with_underline(UnderlineStyle::Wavy);

        assert_eq!(run.font_weight, 700);
        assert_eq!(run.font_style, FontStyle::Italic);
        assert_eq!(run.underline, UnderlineStyle::Wavy);
    }

    #[test]
    fn test_text_run_estimated_bounds() {
        let run = TextRun::new(
            "Test",
            "Inter",
            Pixels(16.0),
            Rgba::BLACK,
            Point::new(Pixels(0.0), Pixels(0.0)),
        );
        let bounds = run.estimated_bounds();
        assert!(bounds.size.width.0 > 0.0);
        assert!(bounds.size.height.0 > 0.0);
    }
}
