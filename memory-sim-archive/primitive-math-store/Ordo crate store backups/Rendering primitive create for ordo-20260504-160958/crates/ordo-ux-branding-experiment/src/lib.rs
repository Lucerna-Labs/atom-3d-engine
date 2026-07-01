//! Branding and logo primitive experiments for Ordo UX.
//!
//! This crate describes brand assets as renderer-neutral primitive trees. It
//! does not generate bitmap images, render SVGs, or bind to a graphics backend.

use kurbo::{BezPath, Point, Size};
use ordo_ux_primitives::{
    Bounds, Fill, Layer, Primitive, Shape, Stroke, TextAlign, TextRun, ThemeTokens, Transform,
};
use peniko::Color;

/// A compact brand palette used by primitive logo builders.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BrandPalette {
    /// Primary brand color.
    pub primary: Color,
    /// Secondary brand color.
    pub secondary: Color,
    /// Accent brand color.
    pub accent: Color,
    /// Text color used beside marks.
    pub text: Color,
    /// Light surface color.
    pub surface: Color,
    /// Dark surface color.
    pub inverse_surface: Color,
}

impl BrandPalette {
    /// Ordo's current experimental default palette.
    #[must_use]
    pub fn ordo() -> Self {
        Self {
            primary: Color::new([0.0, 0.42, 0.84, 1.0]),
            secondary: Color::new([0.0, 0.66, 0.52, 1.0]),
            accent: Color::new([0.94, 0.62, 0.16, 1.0]),
            text: Color::new([0.08, 0.09, 0.11, 1.0]),
            surface: Color::new([1.0, 1.0, 1.0, 1.0]),
            inverse_surface: Color::new([0.06, 0.07, 0.09, 1.0]),
        }
    }

    /// Returns a palette adjusted for use on a dark background.
    #[must_use]
    pub fn on_dark(mut self) -> Self {
        self.text = Color::new([0.96, 0.97, 0.98, 1.0]);
        self.surface = self.inverse_surface;
        self
    }
}

impl Default for BrandPalette {
    fn default() -> Self {
        Self::ordo()
    }
}

/// Logo mark construction styles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LogoMarkKind {
    /// Circular orbital mark.
    #[default]
    Orbital,
    /// Folded diamond mark.
    Fold,
    /// Simple monogram badge.
    Monogram,
}

/// Logo output variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LogoVariant {
    /// Full-color brand mark.
    #[default]
    FullColor,
    /// Single-color mark.
    Mono,
    /// Reversed mark for dark surfaces.
    Reversed,
}

/// A primitive logo mark.
#[derive(Clone, Debug, PartialEq)]
pub struct LogoMark {
    /// Stable identifier.
    pub id: String,
    /// Mark bounds.
    pub bounds: Bounds,
    /// Mark style.
    pub kind: LogoMarkKind,
    /// Color variant.
    pub variant: LogoVariant,
    /// Brand palette.
    pub palette: BrandPalette,
}

impl LogoMark {
    /// Creates a logo mark.
    #[must_use]
    pub fn new(id: impl Into<String>, bounds: Bounds) -> Self {
        Self {
            id: id.into(),
            bounds,
            kind: LogoMarkKind::default(),
            variant: LogoVariant::default(),
            palette: BrandPalette::default(),
        }
    }

    /// Sets mark style.
    #[must_use]
    pub fn with_kind(mut self, kind: LogoMarkKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets color variant.
    #[must_use]
    pub fn with_variant(mut self, variant: LogoVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets palette.
    #[must_use]
    pub fn with_palette(mut self, palette: BrandPalette) -> Self {
        self.palette = palette;
        self
    }

    /// Returns minimum clearspace around the mark.
    #[must_use]
    pub fn clearspace(&self) -> f64 {
        self.bounds.rect.width().min(self.bounds.rect.height()) * 0.2
    }

    /// Returns bounds including required clearspace.
    #[must_use]
    pub fn protected_bounds(&self) -> Bounds {
        let clearspace = self.clearspace();
        self.bounds.inflate(clearspace, clearspace)
    }

    /// Returns renderer-neutral primitives for this mark.
    #[must_use]
    pub fn primitives(&self) -> Vec<Primitive> {
        match self.kind {
            LogoMarkKind::Orbital => self.orbital_primitives(),
            LogoMarkKind::Fold => self.fold_primitives(),
            LogoMarkKind::Monogram => self.monogram_primitives(),
        }
    }

    fn color_pair(&self) -> (Color, Color) {
        match self.variant {
            LogoVariant::FullColor => (self.palette.primary, self.palette.secondary),
            LogoVariant::Mono => (self.palette.text, self.palette.text),
            LogoVariant::Reversed => (
                Color::new([1.0, 1.0, 1.0, 1.0]),
                self.palette.accent.with_alpha(0.9),
            ),
        }
    }

    fn orbital_primitives(&self) -> Vec<Primitive> {
        let rect = self.bounds.rect;
        let center = Point::new(rect.x0 + rect.width() / 2.0, rect.y0 + rect.height() / 2.0);
        let radius = rect.width().min(rect.height()) / 2.0;
        let (primary, secondary) = self.color_pair();

        vec![
            Primitive::fill(Shape::circle(center, radius), Fill::new(primary)),
            Primitive::stroke(
                Shape::circle(center, radius * 0.64),
                Stroke::new(secondary, radius * 0.16),
            )
            .with_transform(Transform::rotate(0.28)),
            Primitive::fill(
                Shape::circle(
                    Point::new(center.x + radius * 0.38, center.y - radius * 0.2),
                    radius * 0.18,
                ),
                Fill::new(self.palette.accent),
            ),
        ]
    }

    fn fold_primitives(&self) -> Vec<Primitive> {
        let rect = self.bounds.rect;
        let (primary, secondary) = self.color_pair();
        let mut left = BezPath::new();
        left.move_to((rect.x0 + rect.width() * 0.5, rect.y0));
        left.line_to((rect.x0, rect.y0 + rect.height() * 0.5));
        left.line_to((rect.x0 + rect.width() * 0.5, rect.y1));
        left.close_path();

        let mut right = BezPath::new();
        right.move_to((rect.x0 + rect.width() * 0.5, rect.y0));
        right.line_to((rect.x1, rect.y0 + rect.height() * 0.5));
        right.line_to((rect.x0 + rect.width() * 0.5, rect.y1));
        right.close_path();

        vec![
            Primitive::fill(Shape::path(left), Fill::new(primary)),
            Primitive::fill(Shape::path(right), Fill::new(secondary)),
        ]
    }

    fn monogram_primitives(&self) -> Vec<Primitive> {
        let rect = self.bounds.rect;
        let radius = rect.width().min(rect.height()) * 0.16;
        let (primary, _) = self.color_pair();
        let mut primitives = vec![Primitive::fill(
            Shape::rounded_rect(rect.x0, rect.y0, rect.width(), rect.height(), radius),
            Fill::new(primary),
        )];

        let text_origin = Point::new(rect.x0 + rect.width() / 2.0, rect.y0 + rect.height() * 0.64);
        primitives.push(Primitive::text(
            TextRun::new("O", text_origin)
                .with_font("System", rect.height() * 0.58)
                .with_align(TextAlign::Center)
                .with_fill(Fill::new(Color::new([1.0, 1.0, 1.0, 1.0])))
                .with_bounds(self.bounds),
        ));

        primitives
    }
}

/// Wordmark text beside or below a logo mark.
#[derive(Clone, Debug, PartialEq)]
pub struct Wordmark {
    /// Brand name.
    pub text: String,
    /// Baseline origin.
    pub origin: Point,
    /// Font family.
    pub font_family: String,
    /// Font size.
    pub font_size: f64,
    /// Text color.
    pub color: Color,
}

impl Wordmark {
    /// Creates a wordmark.
    #[must_use]
    pub fn new(text: impl Into<String>, origin: Point) -> Self {
        Self {
            text: text.into(),
            origin,
            font_family: "System".to_string(),
            font_size: 24.0,
            color: BrandPalette::default().text,
        }
    }

    /// Sets font style.
    #[must_use]
    pub fn with_font(mut self, family: impl Into<String>, size: f64) -> Self {
        self.font_family = family.into();
        self.font_size = size;
        self
    }

    /// Sets wordmark color.
    #[must_use]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Returns a primitive for this wordmark.
    #[must_use]
    pub fn primitive(&self, bounds: Bounds) -> Primitive {
        Primitive::text(
            TextRun::new(self.text.clone(), self.origin)
                .with_font(self.font_family.clone(), self.font_size)
                .with_fill(Fill::new(self.color))
                .with_bounds(bounds),
        )
    }
}

/// Arrangement of logo mark and wordmark.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LockupDirection {
    /// Mark followed by wordmark.
    #[default]
    Horizontal,
    /// Mark above wordmark.
    Vertical,
    /// Mark only.
    MarkOnly,
}

/// A complete brand lockup.
#[derive(Clone, Debug, PartialEq)]
pub struct BrandLockup {
    /// Logo mark.
    pub mark: LogoMark,
    /// Wordmark text.
    pub wordmark: Wordmark,
    /// Arrangement direction.
    pub direction: LockupDirection,
    /// Spacing between mark and wordmark.
    pub gap: f64,
}

impl BrandLockup {
    /// Creates a horizontal Ordo lockup.
    #[must_use]
    pub fn ordo(bounds: Bounds) -> Self {
        let mark_size = bounds.rect.height().min(bounds.rect.width() * 0.3);
        let mark_bounds = Bounds::new(bounds.origin(), Size::new(mark_size, mark_size));
        let text_origin = Point::new(
            bounds.rect.x0 + mark_size + mark_size * 0.32,
            bounds.rect.y0 + mark_size * 0.68,
        );

        Self {
            mark: LogoMark::new("ordo-mark", mark_bounds),
            wordmark: Wordmark::new("Ordo", text_origin).with_font("System", mark_size * 0.58),
            direction: LockupDirection::Horizontal,
            gap: mark_size * 0.32,
        }
    }

    /// Sets lockup direction.
    #[must_use]
    pub fn with_direction(mut self, direction: LockupDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Returns bounds including clearspace.
    #[must_use]
    pub fn protected_bounds(&self) -> Bounds {
        self.mark.protected_bounds()
    }

    /// Returns renderer-neutral primitives for this lockup.
    #[must_use]
    pub fn primitives(&self) -> Vec<Primitive> {
        let mut layer = Layer::new().named("brand-lockup");
        for primitive in self.mark.primitives() {
            layer.push(primitive);
        }

        if self.direction != LockupDirection::MarkOnly {
            let word_bounds = match self.direction {
                LockupDirection::Horizontal => Bounds::from_xywh(
                    self.wordmark.origin.x,
                    self.mark.bounds.rect.y0,
                    self.mark.bounds.rect.width() * 4.0,
                    self.mark.bounds.rect.height(),
                ),
                LockupDirection::Vertical => Bounds::from_xywh(
                    self.mark.bounds.rect.x0 - self.mark.bounds.rect.width(),
                    self.mark.bounds.rect.y1 + self.gap,
                    self.mark.bounds.rect.width() * 3.0,
                    self.mark.bounds.rect.height(),
                ),
                LockupDirection::MarkOnly => self.mark.bounds,
            };
            layer.push(self.wordmark.primitive(word_bounds));
        }

        vec![Primitive::layer(layer)]
    }
}

/// Applies brand palette choices to the primitive theme tokens.
#[must_use]
pub fn branded_theme(base: ThemeTokens, palette: BrandPalette) -> ThemeTokens {
    ThemeTokens {
        accent: palette.primary,
        focus_ring: palette.secondary,
        text_primary: palette.text,
        surface: palette.surface,
        ..base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_palette_has_distinct_brand_colors() {
        let palette = BrandPalette::ordo();

        assert_ne!(palette.primary, palette.secondary);
        assert_ne!(palette.primary, palette.accent);
    }

    #[test]
    fn logo_mark_emits_orbital_primitives() {
        let mark = LogoMark::new("mark", Bounds::from_xywh(0.0, 0.0, 64.0, 64.0));
        let primitives = mark.primitives();

        assert_eq!(primitives.len(), 3);
        assert!(mark.protected_bounds().rect.width() > mark.bounds.rect.width());
    }

    #[test]
    fn logo_mark_variants_change_primitive_count_by_kind() {
        let fold = LogoMark::new("fold", Bounds::from_xywh(0.0, 0.0, 64.0, 64.0))
            .with_kind(LogoMarkKind::Fold);
        let monogram = LogoMark::new("mono", Bounds::from_xywh(0.0, 0.0, 64.0, 64.0))
            .with_kind(LogoMarkKind::Monogram)
            .with_variant(LogoVariant::Reversed);

        assert_eq!(fold.primitives().len(), 2);
        assert_eq!(monogram.primitives().len(), 2);
    }

    #[test]
    fn wordmark_emits_text_primitive() {
        let wordmark = Wordmark::new("Ordo", Point::new(20.0, 32.0));
        let primitive = wordmark.primitive(Bounds::from_xywh(0.0, 0.0, 120.0, 48.0));

        assert!(matches!(primitive, Primitive::Text(_)));
    }

    #[test]
    fn lockup_emits_layer_primitive() {
        let lockup = BrandLockup::ordo(Bounds::from_xywh(0.0, 0.0, 220.0, 56.0));
        let primitives = lockup.primitives();

        assert_eq!(primitives.len(), 1);
        assert!(matches!(primitives.first(), Some(Primitive::Layer(_))));
    }

    #[test]
    fn branded_theme_maps_palette_to_theme_tokens() {
        let palette = BrandPalette::ordo().on_dark();
        let theme = branded_theme(ThemeTokens::default(), palette);

        assert_eq!(theme.accent, palette.primary);
        assert_eq!(theme.focus_ring, palette.secondary);
        assert_eq!(theme.text_primary, palette.text);
    }
}
