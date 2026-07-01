//! Shared theme tokens for primitive creation.

use crate::{Fill, Stroke};
use peniko::Color;

/// Minimal theme tokens used to create Ordo UX primitives.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThemeTokens {
    /// Base page or app background.
    pub background: Color,
    /// Elevated or grouped surface color.
    pub surface: Color,
    /// Primary text color.
    pub text_primary: Color,
    /// Secondary text color.
    pub text_secondary: Color,
    /// Primary accent color.
    pub accent: Color,
    /// Hairline and separator color.
    pub border: Color,
    /// Focus ring color.
    pub focus_ring: Color,
    /// Small corner radius in logical UI units.
    pub radius_sm: f64,
    /// Medium corner radius in logical UI units.
    pub radius_md: f64,
    /// Large corner radius in logical UI units.
    pub radius_lg: f64,
    /// Base spacing unit in logical UI units.
    pub spacing_unit: f64,
    /// Default font family.
    pub font_family: String,
    /// Default body text size.
    pub font_size: f64,
}

impl ThemeTokens {
    /// Creates a fill using the primary text color.
    #[must_use]
    pub fn text_fill(&self) -> Fill {
        Fill::new(self.text_primary)
    }

    /// Creates a fill using the accent color.
    #[must_use]
    pub fn accent_fill(&self) -> Fill {
        Fill::new(self.accent)
    }

    /// Creates a one logical-pixel border stroke.
    #[must_use]
    pub fn border_stroke(&self) -> Stroke {
        Stroke::new(self.border, 1.0)
    }
}

impl Default for ThemeTokens {
    fn default() -> Self {
        Self {
            background: Color::new([0.98, 0.98, 0.98, 1.0]),
            surface: Color::new([1.0, 1.0, 1.0, 1.0]),
            text_primary: Color::new([0.08, 0.08, 0.08, 1.0]),
            text_secondary: Color::new([0.36, 0.36, 0.36, 1.0]),
            accent: Color::new([0.0, 0.42, 0.84, 1.0]),
            border: Color::new([0.84, 0.84, 0.84, 1.0]),
            focus_ring: Color::new([0.0, 0.48, 1.0, 1.0]),
            radius_sm: 4.0,
            radius_md: 8.0,
            radius_lg: 12.0,
            spacing_unit: 8.0,
            font_family: "System".to_string(),
            font_size: 14.0,
        }
    }
}
