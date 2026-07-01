//! Exhaustive, customizable typography-system experiments for Ordo UX.
//!
//! This crate defines renderer-neutral typography roles, font stacks, type
//! scales, variable-font axes, and text primitive builders. It does not shape
//! glyphs, load fonts, render text, or bind to a platform text system.

use kurbo::Point;
use ordo_ux_primitives::{Bounds, Fill, Primitive, TextRun, ThemeTokens};
use peniko::Color;

/// Semantic typography roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypeRole {
    /// Hero-sized display text.
    DisplayLarge,
    /// Medium display text.
    DisplayMedium,
    /// Small display text.
    DisplaySmall,
    /// Large page headline.
    HeadlineLarge,
    /// Medium section headline.
    HeadlineMedium,
    /// Small section headline.
    HeadlineSmall,
    /// Large title.
    TitleLarge,
    /// Medium title.
    TitleMedium,
    /// Small title.
    TitleSmall,
    /// Large body copy.
    BodyLarge,
    /// Default body copy.
    BodyMedium,
    /// Small body copy.
    BodySmall,
    /// Large label.
    LabelLarge,
    /// Default label.
    LabelMedium,
    /// Small label.
    LabelSmall,
    /// Caption text.
    Caption,
    /// Legal or fine-print text.
    FinePrint,
    /// Button text.
    Button,
    /// Tab text.
    Tab,
    /// Navigation item text.
    NavItem,
    /// Input field text.
    Input,
    /// Placeholder text.
    Placeholder,
    /// Tooltip text.
    Tooltip,
    /// Badge text.
    Badge,
    /// Data table cell.
    DataCell,
    /// Data table header.
    DataHeader,
    /// Numeric metric.
    Metric,
    /// Code body text.
    Code,
    /// Inline code text.
    CodeInline,
    /// Brand wordmark.
    BrandWordmark,
    /// Quote text.
    Quote,
    /// Keyboard shortcut text.
    KeyboardShortcut,
}

impl TypeRole {
    /// All built-in typography roles.
    pub const ALL: [Self; 32] = [
        Self::DisplayLarge,
        Self::DisplayMedium,
        Self::DisplaySmall,
        Self::HeadlineLarge,
        Self::HeadlineMedium,
        Self::HeadlineSmall,
        Self::TitleLarge,
        Self::TitleMedium,
        Self::TitleSmall,
        Self::BodyLarge,
        Self::BodyMedium,
        Self::BodySmall,
        Self::LabelLarge,
        Self::LabelMedium,
        Self::LabelSmall,
        Self::Caption,
        Self::FinePrint,
        Self::Button,
        Self::Tab,
        Self::NavItem,
        Self::Input,
        Self::Placeholder,
        Self::Tooltip,
        Self::Badge,
        Self::DataCell,
        Self::DataHeader,
        Self::Metric,
        Self::Code,
        Self::CodeInline,
        Self::BrandWordmark,
        Self::Quote,
        Self::KeyboardShortcut,
    ];
}

/// Font family category.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FontFamilyRole {
    /// Primary interface font.
    #[default]
    Sans,
    /// Serif font.
    Serif,
    /// Monospace font.
    Mono,
    /// Display or brand font.
    Display,
    /// Numeric/data font.
    Numeric,
}

/// A named font stack.
#[derive(Clone, Debug, PartialEq)]
pub struct FontStack {
    /// Stack name.
    pub name: String,
    /// Ordered font families.
    pub families: Vec<String>,
}

impl FontStack {
    /// Creates a font stack.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        families: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            families: families.into_iter().map(Into::into).collect(),
        }
    }

    /// Returns the preferred family, falling back to `System`.
    #[must_use]
    pub fn preferred(&self) -> &str {
        self.families
            .first()
            .map(String::as_str)
            .unwrap_or("System")
    }
}

/// Variable-font axis value.
#[derive(Clone, Debug, PartialEq)]
pub struct FontAxis {
    /// Four-character OpenType axis tag, such as `wght`, `wdth`, `opsz`, `slnt`, or `GRAD`.
    pub tag: String,
    /// Axis value.
    pub value: f32,
}

impl FontAxis {
    /// Creates a variable-font axis value.
    #[must_use]
    pub fn new(tag: impl Into<String>, value: f32) -> Self {
        Self {
            tag: tag.into(),
            value,
        }
    }
}

/// Text case transform intent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextCase {
    /// Preserve original text.
    #[default]
    Preserve,
    /// Uppercase text.
    Uppercase,
    /// Lowercase text.
    Lowercase,
    /// Title-case words with a simple ASCII transform.
    Title,
}

/// OpenType feature toggle.
#[derive(Clone, Debug, PartialEq)]
pub struct OpenTypeFeature {
    /// Four-character OpenType feature tag, such as `kern`, `liga`, `tnum`, or `ss01`.
    pub tag: String,
    /// Whether the feature should be enabled.
    pub enabled: bool,
}

impl OpenTypeFeature {
    /// Creates an OpenType feature toggle.
    #[must_use]
    pub fn new(tag: impl Into<String>, enabled: bool) -> Self {
        Self {
            tag: tag.into(),
            enabled,
        }
    }
}

/// Renderer-neutral style for one typography role.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeStyle {
    /// Semantic role.
    pub role: TypeRole,
    /// Font stack category.
    pub family_role: FontFamilyRole,
    /// Font size in logical UI units.
    pub size: f64,
    /// Line height in logical UI units.
    pub line_height: f64,
    /// Font weight using CSS-like numeric values.
    pub weight: u16,
    /// Letter spacing in logical UI units.
    pub letter_spacing: f64,
    /// Optional optical-size value.
    pub optical_size: Option<f32>,
    /// Case transform intent.
    pub text_case: TextCase,
    /// OpenType feature toggles.
    pub features: Vec<OpenTypeFeature>,
    /// Variable-font axes.
    pub axes: Vec<FontAxis>,
}

impl TypeStyle {
    /// Creates a type style.
    #[must_use]
    pub fn new(
        role: TypeRole,
        family_role: FontFamilyRole,
        size: f64,
        line_height: f64,
        weight: u16,
    ) -> Self {
        Self {
            role,
            family_role,
            size,
            line_height,
            weight,
            letter_spacing: 0.0,
            optical_size: None,
            text_case: TextCase::Preserve,
            features: vec![
                OpenTypeFeature::new("kern", true),
                OpenTypeFeature::new("liga", true),
            ],
            axes: Vec::new(),
        }
    }

    /// Sets letter spacing.
    #[must_use]
    pub fn with_letter_spacing(mut self, letter_spacing: f64) -> Self {
        self.letter_spacing = letter_spacing;
        self
    }

    /// Sets case transform intent.
    #[must_use]
    pub fn with_case(mut self, text_case: TextCase) -> Self {
        self.text_case = text_case;
        self
    }

    /// Adds an OpenType feature toggle.
    #[must_use]
    pub fn with_feature(mut self, feature: OpenTypeFeature) -> Self {
        self.features.push(feature);
        self
    }

    /// Adds a variable-font axis.
    #[must_use]
    pub fn with_axis(mut self, axis: FontAxis) -> Self {
        self.axes.push(axis);
        self
    }

    /// Sets optical size axis intent.
    #[must_use]
    pub fn with_optical_size(mut self, optical_size: f32) -> Self {
        self.optical_size = Some(optical_size);
        self.axes.push(FontAxis::new("opsz", optical_size));
        self
    }

    /// Scales size and line height.
    #[must_use]
    pub fn scaled(mut self, factor: f64) -> Self {
        self.size *= factor;
        self.line_height *= factor;
        self
    }
}

/// Global customization knobs for a typography system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeAdjustment {
    /// Font-size multiplier.
    pub size_scale: f64,
    /// Line-height multiplier.
    pub line_height_scale: f64,
    /// Added weight.
    pub weight_delta: i16,
    /// Added letter spacing.
    pub letter_spacing_delta: f64,
    /// Whether metric/numeric roles should prefer tabular figures.
    pub tabular_numbers: bool,
}

impl TypeAdjustment {
    /// No adjustment.
    pub const IDENTITY: Self = Self {
        size_scale: 1.0,
        line_height_scale: 1.0,
        weight_delta: 0,
        letter_spacing_delta: 0.0,
        tabular_numbers: true,
    };

    /// A more compact UI density.
    #[must_use]
    pub fn compact() -> Self {
        Self {
            size_scale: 0.94,
            line_height_scale: 0.96,
            weight_delta: 0,
            letter_spacing_delta: 0.0,
            tabular_numbers: true,
        }
    }

    /// A more spacious, editorial density.
    #[must_use]
    pub fn spacious() -> Self {
        Self {
            size_scale: 1.08,
            line_height_scale: 1.08,
            weight_delta: 0,
            letter_spacing_delta: 0.0,
            tabular_numbers: true,
        }
    }

    /// Applies adjustment to a style.
    #[must_use]
    pub fn apply(&self, mut style: TypeStyle) -> TypeStyle {
        style.size *= self.size_scale;
        style.line_height *= self.line_height_scale;
        style.letter_spacing += self.letter_spacing_delta;
        style.weight = clamp_weight(i32::from(style.weight) + i32::from(self.weight_delta));

        if self.tabular_numbers
            && matches!(
                style.role,
                TypeRole::DataCell | TypeRole::DataHeader | TypeRole::Metric
            )
        {
            style.features.push(OpenTypeFeature::new("tnum", true));
        }

        style
    }
}

impl Default for TypeAdjustment {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Typography preset families.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TypographyPreset {
    /// Modern product UI scale.
    #[default]
    Interface,
    /// Editorial/content-forward scale.
    Editorial,
    /// Dense operational dashboard scale.
    Dashboard,
    /// Brand-forward display scale.
    Brand,
}

/// A complete renderer-neutral typography system.
#[derive(Clone, Debug, PartialEq)]
pub struct TypographySystem {
    /// System name.
    pub name: String,
    /// Font stacks by category.
    pub sans: FontStack,
    /// Serif font stack.
    pub serif: FontStack,
    /// Monospace font stack.
    pub mono: FontStack,
    /// Display font stack.
    pub display: FontStack,
    /// Numeric font stack.
    pub numeric: FontStack,
    /// Role styles.
    pub styles: Vec<TypeStyle>,
    /// Global adjustment.
    pub adjustment: TypeAdjustment,
}

impl TypographySystem {
    /// Creates a preset typography system.
    #[must_use]
    pub fn preset(preset: TypographyPreset) -> Self {
        match preset {
            TypographyPreset::Interface => interface_preset(),
            TypographyPreset::Editorial => editorial_preset(),
            TypographyPreset::Dashboard => dashboard_preset(),
            TypographyPreset::Brand => brand_preset(),
        }
    }

    /// Returns true when every built-in role has a style.
    #[must_use]
    pub fn is_exhaustive(&self) -> bool {
        TypeRole::ALL
            .iter()
            .all(|role| self.styles.iter().any(|style| style.role == *role))
    }

    /// Returns a style for a role after global adjustment.
    #[must_use]
    pub fn style(&self, role: TypeRole) -> Option<TypeStyle> {
        self.styles
            .iter()
            .find(|style| style.role == role)
            .cloned()
            .map(|style| self.adjustment.apply(style))
    }

    /// Replaces or inserts a role style.
    pub fn set_style(&mut self, style: TypeStyle) {
        if let Some(existing) = self
            .styles
            .iter_mut()
            .find(|existing| existing.role == style.role)
        {
            *existing = style;
        } else {
            self.styles.push(style);
        }
    }

    /// Sets global adjustment.
    #[must_use]
    pub fn with_adjustment(mut self, adjustment: TypeAdjustment) -> Self {
        self.adjustment = adjustment;
        self
    }

    /// Returns the stack for a family role.
    #[must_use]
    pub fn stack(&self, role: FontFamilyRole) -> &FontStack {
        match role {
            FontFamilyRole::Sans => &self.sans,
            FontFamilyRole::Serif => &self.serif,
            FontFamilyRole::Mono => &self.mono,
            FontFamilyRole::Display => &self.display,
            FontFamilyRole::Numeric => &self.numeric,
        }
    }

    /// Creates a text run for a role.
    #[must_use]
    pub fn text_run(
        &self,
        role: TypeRole,
        text: impl Into<String>,
        origin: Point,
        color: Color,
        bounds: Option<Bounds>,
    ) -> Option<TextRun> {
        let style = self.style(role)?;
        let family = self.stack(style.family_role).preferred().to_string();
        let text = apply_case(text.into(), style.text_case);
        let mut run = TextRun::new(text, origin)
            .with_font(family, style.size)
            .with_weight(style.weight)
            .with_fill(Fill::new(color));

        if let Some(bounds) = bounds {
            run = run.with_bounds(bounds);
        }

        Some(run)
    }

    /// Creates a text primitive for a role.
    #[must_use]
    pub fn primitive(
        &self,
        role: TypeRole,
        text: impl Into<String>,
        origin: Point,
        color: Color,
        bounds: Option<Bounds>,
    ) -> Option<Primitive> {
        self.text_run(role, text, origin, color, bounds)
            .map(Primitive::text)
    }

    /// Maps key typography choices into primitive theme tokens.
    #[must_use]
    pub fn apply_to_theme(&self, mut theme: ThemeTokens) -> ThemeTokens {
        if let Some(body) = self.style(TypeRole::BodyMedium) {
            theme.font_family = self.stack(body.family_role).preferred().to_string();
            theme.font_size = body.size;
        }
        theme
    }
}

fn interface_preset() -> TypographySystem {
    system(
        "Interface",
        TypeAdjustment::IDENTITY,
        FontStack::new("sans", ["Inter", "System"]),
        FontStack::new("serif", ["Charter", "Georgia"]),
        FontStack::new("mono", ["JetBrains Mono", "SF Mono", "Consolas"]),
        FontStack::new("display", ["Inter Display", "Inter", "System"]),
        FontStack::new("numeric", ["Inter", "System"]),
    )
}

fn editorial_preset() -> TypographySystem {
    let mut system = interface_preset();
    system.name = "Editorial".to_string();
    system.serif = FontStack::new("serif", ["Literata", "Charter", "Georgia"]);
    system.display = FontStack::new("display", ["Fraunces", "Literata", "Georgia"]);
    system.adjustment = TypeAdjustment::spacious();
    for style in &mut system.styles {
        if matches!(
            style.role,
            TypeRole::DisplayLarge
                | TypeRole::DisplayMedium
                | TypeRole::DisplaySmall
                | TypeRole::HeadlineLarge
                | TypeRole::HeadlineMedium
                | TypeRole::HeadlineSmall
                | TypeRole::Quote
        ) {
            style.family_role = FontFamilyRole::Serif;
        }
    }
    system
}

fn dashboard_preset() -> TypographySystem {
    let mut system = interface_preset();
    system.name = "Dashboard".to_string();
    system.adjustment = TypeAdjustment::compact();
    for style in &mut system.styles {
        if matches!(
            style.role,
            TypeRole::Metric | TypeRole::DataCell | TypeRole::DataHeader
        ) {
            style.family_role = FontFamilyRole::Numeric;
            style.features.push(OpenTypeFeature::new("tnum", true));
        }
    }
    system
}

fn brand_preset() -> TypographySystem {
    let mut system = interface_preset();
    system.name = "Brand".to_string();
    system.display = FontStack::new("display", ["Satoshi", "Inter Display", "System"]);
    for style in &mut system.styles {
        if matches!(
            style.role,
            TypeRole::DisplayLarge
                | TypeRole::DisplayMedium
                | TypeRole::DisplaySmall
                | TypeRole::BrandWordmark
        ) {
            style.family_role = FontFamilyRole::Display;
            style.weight = (style.weight + 50).min(900);
            style.axes.push(FontAxis::new("GRAD", 0.18));
        }
    }
    system
}

fn system(
    name: &str,
    adjustment: TypeAdjustment,
    sans: FontStack,
    serif: FontStack,
    mono: FontStack,
    display: FontStack,
    numeric: FontStack,
) -> TypographySystem {
    TypographySystem {
        name: name.to_string(),
        sans,
        serif,
        mono,
        display,
        numeric,
        styles: base_styles(),
        adjustment,
    }
}

fn base_styles() -> Vec<TypeStyle> {
    vec![
        TypeStyle::new(
            TypeRole::DisplayLarge,
            FontFamilyRole::Display,
            64.0,
            72.0,
            720,
        )
        .with_letter_spacing(-0.8)
        .with_optical_size(64.0),
        TypeStyle::new(
            TypeRole::DisplayMedium,
            FontFamilyRole::Display,
            52.0,
            60.0,
            700,
        )
        .with_letter_spacing(-0.6)
        .with_optical_size(52.0),
        TypeStyle::new(
            TypeRole::DisplaySmall,
            FontFamilyRole::Display,
            44.0,
            52.0,
            680,
        )
        .with_letter_spacing(-0.4)
        .with_optical_size(44.0),
        TypeStyle::new(
            TypeRole::HeadlineLarge,
            FontFamilyRole::Sans,
            36.0,
            44.0,
            680,
        )
        .with_letter_spacing(-0.2),
        TypeStyle::new(
            TypeRole::HeadlineMedium,
            FontFamilyRole::Sans,
            30.0,
            38.0,
            660,
        ),
        TypeStyle::new(
            TypeRole::HeadlineSmall,
            FontFamilyRole::Sans,
            24.0,
            32.0,
            640,
        ),
        TypeStyle::new(TypeRole::TitleLarge, FontFamilyRole::Sans, 22.0, 30.0, 620),
        TypeStyle::new(TypeRole::TitleMedium, FontFamilyRole::Sans, 18.0, 26.0, 600),
        TypeStyle::new(TypeRole::TitleSmall, FontFamilyRole::Sans, 16.0, 24.0, 600),
        TypeStyle::new(TypeRole::BodyLarge, FontFamilyRole::Sans, 16.0, 26.0, 400),
        TypeStyle::new(TypeRole::BodyMedium, FontFamilyRole::Sans, 14.0, 22.0, 400),
        TypeStyle::new(TypeRole::BodySmall, FontFamilyRole::Sans, 13.0, 20.0, 400),
        TypeStyle::new(TypeRole::LabelLarge, FontFamilyRole::Sans, 14.0, 20.0, 580),
        TypeStyle::new(TypeRole::LabelMedium, FontFamilyRole::Sans, 12.0, 18.0, 580),
        TypeStyle::new(TypeRole::LabelSmall, FontFamilyRole::Sans, 11.0, 16.0, 580),
        TypeStyle::new(TypeRole::Caption, FontFamilyRole::Sans, 12.0, 18.0, 420),
        TypeStyle::new(TypeRole::FinePrint, FontFamilyRole::Sans, 10.0, 14.0, 400),
        TypeStyle::new(TypeRole::Button, FontFamilyRole::Sans, 14.0, 18.0, 640)
            .with_case(TextCase::Preserve),
        TypeStyle::new(TypeRole::Tab, FontFamilyRole::Sans, 13.0, 18.0, 620),
        TypeStyle::new(TypeRole::NavItem, FontFamilyRole::Sans, 14.0, 20.0, 560),
        TypeStyle::new(TypeRole::Input, FontFamilyRole::Sans, 14.0, 22.0, 400),
        TypeStyle::new(TypeRole::Placeholder, FontFamilyRole::Sans, 14.0, 22.0, 400),
        TypeStyle::new(TypeRole::Tooltip, FontFamilyRole::Sans, 12.0, 17.0, 450),
        TypeStyle::new(TypeRole::Badge, FontFamilyRole::Sans, 11.0, 14.0, 680)
            .with_case(TextCase::Uppercase)
            .with_letter_spacing(0.4),
        TypeStyle::new(TypeRole::DataCell, FontFamilyRole::Numeric, 13.0, 20.0, 430),
        TypeStyle::new(
            TypeRole::DataHeader,
            FontFamilyRole::Numeric,
            12.0,
            18.0,
            680,
        )
        .with_case(TextCase::Uppercase)
        .with_letter_spacing(0.32),
        TypeStyle::new(TypeRole::Metric, FontFamilyRole::Numeric, 32.0, 38.0, 700)
            .with_feature(OpenTypeFeature::new("tnum", true)),
        TypeStyle::new(TypeRole::Code, FontFamilyRole::Mono, 13.0, 20.0, 450)
            .with_feature(OpenTypeFeature::new("liga", false)),
        TypeStyle::new(TypeRole::CodeInline, FontFamilyRole::Mono, 13.0, 18.0, 450)
            .with_feature(OpenTypeFeature::new("liga", false)),
        TypeStyle::new(
            TypeRole::BrandWordmark,
            FontFamilyRole::Display,
            28.0,
            34.0,
            720,
        )
        .with_letter_spacing(-0.2),
        TypeStyle::new(TypeRole::Quote, FontFamilyRole::Serif, 20.0, 32.0, 430),
        TypeStyle::new(
            TypeRole::KeyboardShortcut,
            FontFamilyRole::Mono,
            11.0,
            14.0,
            620,
        )
        .with_case(TextCase::Uppercase),
    ]
}

fn apply_case(text: String, text_case: TextCase) -> String {
    match text_case {
        TextCase::Preserve => text,
        TextCase::Uppercase => text.to_uppercase(),
        TextCase::Lowercase => text.to_lowercase(),
        TextCase::Title => title_case_ascii(&text),
    }
}

fn title_case_ascii(text: &str) -> String {
    text.split(' ')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    let mut out = first.to_uppercase().collect::<String>();
                    out.push_str(chars.as_str().to_lowercase().as_str());
                    out
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn clamp_weight(weight: i32) -> u16 {
    weight.clamp(1, 1000) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_are_exhaustive() {
        for preset in [
            TypographyPreset::Interface,
            TypographyPreset::Editorial,
            TypographyPreset::Dashboard,
            TypographyPreset::Brand,
        ] {
            let system = TypographySystem::preset(preset);
            assert!(
                system.is_exhaustive(),
                "{preset:?} should define every role"
            );
        }
    }

    #[test]
    fn styles_are_customizable_by_role() {
        let mut system = TypographySystem::preset(TypographyPreset::Interface);
        system.set_style(TypeStyle::new(
            TypeRole::BodyMedium,
            FontFamilyRole::Serif,
            17.0,
            28.0,
            500,
        ));

        let style = system
            .style(TypeRole::BodyMedium)
            .expect("style should exist");
        assert_eq!(style.family_role, FontFamilyRole::Serif);
        assert_eq!(style.size, 17.0);
    }

    #[test]
    fn adjustment_scales_type_and_adds_tabular_numbers() {
        let system =
            TypographySystem::preset(TypographyPreset::Dashboard).with_adjustment(TypeAdjustment {
                size_scale: 2.0,
                line_height_scale: 1.0,
                weight_delta: 25,
                letter_spacing_delta: 0.1,
                tabular_numbers: true,
            });
        let style = system
            .style(TypeRole::Metric)
            .expect("metric style should exist");

        assert!(style.size >= 60.0);
        assert!(
            style
                .features
                .iter()
                .any(|feature| feature.tag == "tnum" && feature.enabled)
        );
    }

    #[test]
    fn text_run_applies_case_font_and_bounds() {
        let system = TypographySystem::preset(TypographyPreset::Interface);
        let run = system
            .text_run(
                TypeRole::Badge,
                "new",
                Point::new(8.0, 18.0),
                Color::BLACK,
                Some(Bounds::from_xywh(0.0, 0.0, 64.0, 24.0)),
            )
            .expect("badge run should exist");

        assert_eq!(run.text, "NEW");
        assert_eq!(run.font_family, "Inter");
        assert!(run.bounds.is_some());
    }

    #[test]
    fn primitive_builder_returns_text_primitive() {
        let system = TypographySystem::preset(TypographyPreset::Brand);
        let primitive = system
            .primitive(
                TypeRole::BrandWordmark,
                "Ordo",
                Point::new(0.0, 30.0),
                Color::BLACK,
                None,
            )
            .expect("primitive should exist");

        assert!(matches!(primitive, Primitive::Text(_)));
    }

    #[test]
    fn system_updates_theme_tokens() {
        let system = TypographySystem::preset(TypographyPreset::Dashboard);
        let theme = system.apply_to_theme(ThemeTokens::default());

        assert_eq!(theme.font_family, "Inter");
        assert!(theme.font_size < 14.0);
    }

    #[test]
    fn font_stack_falls_back_to_system() {
        let stack = FontStack::new("empty", Vec::<String>::new());

        assert_eq!(stack.preferred(), "System");
    }
}
