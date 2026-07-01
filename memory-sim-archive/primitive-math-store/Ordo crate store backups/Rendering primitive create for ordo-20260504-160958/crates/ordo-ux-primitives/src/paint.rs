//! Paint styles shared by future renderer backends.

use kurbo::{Cap as KurboCap, Join as KurboJoin, Stroke as KurboStroke};
use peniko::{Brush, Color};

/// A renderer-neutral paint value.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Paint {
    /// A solid sRGB color.
    Solid(Color),
    /// A Peniko brush, useful for gradients or other backend-neutral styling.
    Brush(Brush),
}

impl Paint {
    /// Creates a solid color paint.
    #[must_use]
    pub fn solid(color: Color) -> Self {
        Self::Solid(color)
    }
}

impl Default for Paint {
    fn default() -> Self {
        Self::Solid(Color::TRANSPARENT)
    }
}

impl From<Color> for Paint {
    fn from(color: Color) -> Self {
        Self::solid(color)
    }
}

impl From<Brush> for Paint {
    fn from(brush: Brush) -> Self {
        Self::Brush(brush)
    }
}

/// Fill styling for closed shapes and text glyphs.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fill {
    /// Paint used for the fill.
    pub paint: Paint,
    /// Interior rule for paths with self intersections.
    pub rule: FillRule,
    /// Fill opacity from `0.0` to `1.0`.
    pub opacity: f32,
}

impl Fill {
    /// Creates a fill from paint.
    #[must_use]
    pub fn new(paint: impl Into<Paint>) -> Self {
        Self {
            paint: paint.into(),
            rule: FillRule::NonZero,
            opacity: 1.0,
        }
    }

    /// Sets the path fill rule.
    #[must_use]
    pub fn with_rule(mut self, rule: FillRule) -> Self {
        self.rule = rule;
        self
    }

    /// Sets fill opacity.
    #[must_use]
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self::new(Color::TRANSPARENT)
    }
}

/// Path fill rule.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FillRule {
    /// Non-zero winding fill rule.
    #[default]
    NonZero,
    /// Even-odd fill rule.
    EvenOdd,
}

/// Stroke styling for shape outlines.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stroke {
    /// Paint used for the stroke.
    pub paint: Paint,
    /// Stroke width in logical UI units.
    pub width: f64,
    /// Line cap style.
    pub line_cap: LineCap,
    /// Line join style.
    pub line_join: LineJoin,
    /// Miter limit used when `line_join` is [`LineJoin::Miter`].
    pub miter_limit: f64,
    /// Dash pattern lengths in logical UI units.
    pub dash_pattern: Vec<f64>,
    /// Offset into the dash pattern.
    pub dash_offset: f64,
    /// Stroke opacity from `0.0` to `1.0`.
    pub opacity: f32,
}

impl Stroke {
    /// Creates a stroke from paint and width.
    #[must_use]
    pub fn new(paint: impl Into<Paint>, width: f64) -> Self {
        Self {
            paint: paint.into(),
            width,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 4.0,
            dash_pattern: Vec::new(),
            dash_offset: 0.0,
            opacity: 1.0,
        }
    }

    /// Sets the line cap style.
    #[must_use]
    pub fn with_line_cap(mut self, line_cap: LineCap) -> Self {
        self.line_cap = line_cap;
        self
    }

    /// Sets the line join style.
    #[must_use]
    pub fn with_line_join(mut self, line_join: LineJoin) -> Self {
        self.line_join = line_join;
        self
    }

    /// Sets the miter limit.
    #[must_use]
    pub fn with_miter_limit(mut self, miter_limit: f64) -> Self {
        self.miter_limit = miter_limit;
        self
    }

    /// Sets dashing parameters.
    #[must_use]
    pub fn with_dashes(mut self, dash_offset: f64, dash_pattern: impl Into<Vec<f64>>) -> Self {
        self.dash_offset = dash_offset;
        self.dash_pattern = dash_pattern.into();
        self
    }

    /// Sets stroke opacity.
    #[must_use]
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Returns Kurbo's geometry-only stroke style for backends that use Kurbo paths.
    #[must_use]
    pub fn to_kurbo_stroke(&self) -> KurboStroke {
        KurboStroke::new(self.width)
            .with_caps(self.line_cap.into())
            .with_join(self.line_join.into())
            .with_miter_limit(self.miter_limit)
            .with_dashes(self.dash_offset, &self.dash_pattern)
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self::new(Color::BLACK, 1.0)
    }
}

/// Stroke line cap style.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LineCap {
    /// Flat cap at the end point.
    #[default]
    Butt,
    /// Rounded cap centered on the end point.
    Round,
    /// Square cap centered on the end point.
    Square,
}

impl From<LineCap> for KurboCap {
    fn from(line_cap: LineCap) -> Self {
        match line_cap {
            LineCap::Butt => Self::Butt,
            LineCap::Round => Self::Round,
            LineCap::Square => Self::Square,
        }
    }
}

/// Stroke line join style.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LineJoin {
    /// Sharp miter join, subject to the miter limit.
    #[default]
    Miter,
    /// Rounded join.
    Round,
    /// Beveled join.
    Bevel,
}

impl From<LineJoin> for KurboJoin {
    fn from(line_join: LineJoin) -> Self {
        match line_join {
            LineJoin::Miter => Self::Miter,
            LineJoin::Round => Self::Round,
            LineJoin::Bevel => Self::Bevel,
        }
    }
}
