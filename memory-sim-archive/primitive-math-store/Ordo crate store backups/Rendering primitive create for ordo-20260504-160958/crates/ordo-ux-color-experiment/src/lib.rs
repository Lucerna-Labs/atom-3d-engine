//! Exhaustive, adjustable, living color palette experiments for Ordo UX.
//!
//! This crate explores color as a renderer-neutral design system. It does not
//! render anything or own animation loops. Instead, it defines semantic color
//! roles, rich presets, palette adjustments, ramps, and time-sampled color
//! motion that can make UI colors feel alive.

use std::f64::consts::TAU;

use ordo_ux_primitives::ThemeTokens;
use peniko::Color;

/// A semantic color role in the Ordo UX color system.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorRole {
    /// App background.
    Background,
    /// Default surface.
    Surface,
    /// Elevated surface.
    SurfaceRaised,
    /// Pressed surface.
    SurfacePressed,
    /// Primary text.
    TextPrimary,
    /// Secondary text.
    TextSecondary,
    /// Muted text.
    TextMuted,
    /// Border or separator.
    Border,
    /// Focus ring.
    Focus,
    /// Primary brand color.
    BrandPrimary,
    /// Secondary brand color.
    BrandSecondary,
    /// Cool accent.
    AccentCool,
    /// Warm accent.
    AccentWarm,
    /// Soft accent.
    AccentSoft,
    /// Success state.
    Success,
    /// Warning state.
    Warning,
    /// Danger state.
    Danger,
    /// Info state.
    Info,
    /// Online or live state.
    Live,
    /// Offline state.
    Offline,
    /// Glow or bloom color.
    Glow,
    /// Selection highlight.
    Selection,
    /// Data visualization color 1.
    Data01,
    /// Data visualization color 2.
    Data02,
    /// Data visualization color 3.
    Data03,
    /// Data visualization color 4.
    Data04,
    /// Data visualization color 5.
    Data05,
    /// Data visualization color 6.
    Data06,
    /// Data visualization color 7.
    Data07,
    /// Data visualization color 8.
    Data08,
    /// Data visualization color 9.
    Data09,
    /// Data visualization color 10.
    Data10,
    /// Data visualization color 11.
    Data11,
    /// Data visualization color 12.
    Data12,
}

impl ColorRole {
    /// All built-in color roles.
    pub const ALL: [Self; 34] = [
        Self::Background,
        Self::Surface,
        Self::SurfaceRaised,
        Self::SurfacePressed,
        Self::TextPrimary,
        Self::TextSecondary,
        Self::TextMuted,
        Self::Border,
        Self::Focus,
        Self::BrandPrimary,
        Self::BrandSecondary,
        Self::AccentCool,
        Self::AccentWarm,
        Self::AccentSoft,
        Self::Success,
        Self::Warning,
        Self::Danger,
        Self::Info,
        Self::Live,
        Self::Offline,
        Self::Glow,
        Self::Selection,
        Self::Data01,
        Self::Data02,
        Self::Data03,
        Self::Data04,
        Self::Data05,
        Self::Data06,
        Self::Data07,
        Self::Data08,
        Self::Data09,
        Self::Data10,
        Self::Data11,
        Self::Data12,
    ];
}

/// A named semantic color value.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorToken {
    /// Semantic role.
    pub role: ColorRole,
    /// Human-readable token name.
    pub name: String,
    /// Color value.
    pub color: Color,
}

impl ColorToken {
    /// Creates a color token.
    #[must_use]
    pub fn new(role: ColorRole, name: impl Into<String>, color: Color) -> Self {
        Self {
            role,
            name: name.into(),
            color,
        }
    }
}

/// A set of ordered colors, useful for scales, charts, glow families, and ramps.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorRamp {
    /// Ramp name.
    pub name: String,
    /// Ordered colors from low to high intensity.
    pub stops: Vec<Color>,
}

impl ColorRamp {
    /// Creates a ramp.
    #[must_use]
    pub fn new(name: impl Into<String>, stops: Vec<Color>) -> Self {
        Self {
            name: name.into(),
            stops,
        }
    }

    /// Returns a color sampled across the ramp.
    #[must_use]
    pub fn sample(&self, t: f32) -> Option<Color> {
        if self.stops.is_empty() {
            return None;
        }
        if self.stops.len() == 1 {
            return self.stops.first().copied();
        }

        let t = t.clamp(0.0, 1.0);
        let scaled = t * (self.stops.len() - 1) as f32;
        let left_index = scaled.floor() as usize;
        let right_index = (left_index + 1).min(self.stops.len() - 1);
        let local_t = scaled - left_index as f32;

        Some(mix(
            self.stops[left_index],
            self.stops[right_index],
            local_t,
        ))
    }
}

/// Global palette adjustment knobs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PaletteAdjustment {
    /// Saturation multiplier. `1.0` keeps saturation unchanged.
    pub saturation: f32,
    /// Lightness offset. `0.0` keeps lightness unchanged.
    pub lightness: f32,
    /// Contrast multiplier around midpoint `0.5`.
    pub contrast: f32,
    /// Hue shift in degrees.
    pub hue_shift_degrees: f32,
    /// Alpha multiplier.
    pub alpha: f32,
}

impl PaletteAdjustment {
    /// No adjustment.
    pub const IDENTITY: Self = Self {
        saturation: 1.0,
        lightness: 0.0,
        contrast: 1.0,
        hue_shift_degrees: 0.0,
        alpha: 1.0,
    };

    /// Creates a stronger, more luminous adjustment.
    #[must_use]
    pub fn vivid() -> Self {
        Self {
            saturation: 1.18,
            lightness: 0.03,
            contrast: 1.08,
            hue_shift_degrees: 0.0,
            alpha: 1.0,
        }
    }

    /// Creates a calmer, softer adjustment.
    #[must_use]
    pub fn soft() -> Self {
        Self {
            saturation: 0.82,
            lightness: 0.02,
            contrast: 0.94,
            hue_shift_degrees: 0.0,
            alpha: 1.0,
        }
    }

    /// Applies this adjustment to a color.
    #[must_use]
    pub fn apply(&self, color: Color) -> Color {
        let rgba = Rgba::from(color);
        let mut hsla = Hsla::from(rgba);
        hsla.h = wrap_unit(hsla.h + self.hue_shift_degrees / 360.0);
        hsla.s = clamp01(hsla.s * self.saturation);
        hsla.l = clamp01(((hsla.l - 0.5) * self.contrast) + 0.5 + self.lightness);
        hsla.a = clamp01(hsla.a * self.alpha);
        Color::from(Rgba::from(hsla))
    }
}

impl Default for PaletteAdjustment {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Motion applied to a color over time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LivingColorMotion {
    /// Full cycle duration in seconds.
    pub period_seconds: f64,
    /// Offset into the cycle in normalized turns.
    pub phase: f64,
    /// Hue drift amplitude in degrees.
    pub hue_drift_degrees: f32,
    /// Saturation pulse amplitude.
    pub saturation_pulse: f32,
    /// Lightness pulse amplitude.
    pub lightness_pulse: f32,
    /// Alpha pulse amplitude.
    pub alpha_pulse: f32,
}

impl LivingColorMotion {
    /// Subtle motion suitable for production UI.
    #[must_use]
    pub fn subtle() -> Self {
        Self {
            period_seconds: 4.0,
            phase: 0.0,
            hue_drift_degrees: 3.0,
            saturation_pulse: 0.04,
            lightness_pulse: 0.025,
            alpha_pulse: 0.0,
        }
    }

    /// Stronger motion for experimental or attention-heavy surfaces.
    #[must_use]
    pub fn expressive() -> Self {
        Self {
            period_seconds: 2.6,
            phase: 0.0,
            hue_drift_degrees: 12.0,
            saturation_pulse: 0.12,
            lightness_pulse: 0.06,
            alpha_pulse: 0.04,
        }
    }

    /// Samples the motion wave at `seconds`.
    #[must_use]
    pub fn wave(&self, seconds: f64) -> f32 {
        if self.period_seconds <= 0.0 {
            return 0.0;
        }

        (((seconds / self.period_seconds + self.phase) * TAU).sin()) as f32
    }
}

impl Default for LivingColorMotion {
    fn default() -> Self {
        Self::subtle()
    }
}

/// A color plus motion that can be sampled over time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LivingColor {
    /// Base color.
    pub base: Color,
    /// Motion applied to the base color.
    pub motion: LivingColorMotion,
}

impl LivingColor {
    /// Creates a living color.
    #[must_use]
    pub fn new(base: Color, motion: LivingColorMotion) -> Self {
        Self { base, motion }
    }

    /// Samples the living color at `seconds`.
    #[must_use]
    pub fn sample(&self, seconds: f64) -> Color {
        let wave = self.motion.wave(seconds);
        let adjustment = PaletteAdjustment {
            saturation: 1.0 + wave * self.motion.saturation_pulse,
            lightness: wave * self.motion.lightness_pulse,
            contrast: 1.0,
            hue_shift_degrees: wave * self.motion.hue_drift_degrees,
            alpha: 1.0 + wave * self.motion.alpha_pulse,
        };

        adjustment.apply(self.base)
    }
}

/// Modern palette preset families.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ModernPalettePreset {
    /// Cool glass surfaces with crisp luminous accents.
    #[default]
    GlassAurora,
    /// Dark graphite base with vivid bloom tones.
    GraphiteBloom,
    /// Clean near-white clinical UI with precise signal colors.
    ClinicalEdge,
    /// High-energy chromatic neon set.
    NeonFlux,
}

/// A broad semantic palette with ramps and living-color behavior.
#[derive(Clone, Debug, PartialEq)]
pub struct OrdoColorSystem {
    /// Palette name.
    pub name: String,
    /// Semantic tokens.
    pub tokens: Vec<ColorToken>,
    /// Useful ramps.
    pub ramps: Vec<ColorRamp>,
    /// Global adjustments applied at sampling time.
    pub adjustment: PaletteAdjustment,
    /// Time-based motion applied at sampling time.
    pub motion: LivingColorMotion,
}

impl OrdoColorSystem {
    /// Creates a modern preset.
    #[must_use]
    pub fn preset(preset: ModernPalettePreset) -> Self {
        match preset {
            ModernPalettePreset::GlassAurora => glass_aurora(),
            ModernPalettePreset::GraphiteBloom => graphite_bloom(),
            ModernPalettePreset::ClinicalEdge => clinical_edge(),
            ModernPalettePreset::NeonFlux => neon_flux(),
        }
    }

    /// Returns a token color by role.
    #[must_use]
    pub fn color(&self, role: ColorRole) -> Option<Color> {
        self.tokens
            .iter()
            .find(|token| token.role == role)
            .map(|token| token.color)
    }

    /// Samples a role after palette adjustments and living motion.
    #[must_use]
    pub fn sample(&self, role: ColorRole, seconds: f64) -> Option<Color> {
        let color = self.adjustment.apply(self.color(role)?);
        Some(LivingColor::new(color, self.motion).sample(seconds))
    }

    /// Replaces or inserts a token.
    pub fn set_color(&mut self, role: ColorRole, color: Color) {
        if let Some(token) = self.tokens.iter_mut().find(|token| token.role == role) {
            token.color = color;
        } else {
            self.tokens
                .push(ColorToken::new(role, format!("{role:?}"), color));
        }
    }

    /// Sets global adjustment knobs.
    #[must_use]
    pub fn with_adjustment(mut self, adjustment: PaletteAdjustment) -> Self {
        self.adjustment = adjustment;
        self
    }

    /// Sets living color motion.
    #[must_use]
    pub fn with_motion(mut self, motion: LivingColorMotion) -> Self {
        self.motion = motion;
        self
    }

    /// Returns a ramp by name.
    #[must_use]
    pub fn ramp(&self, name: &str) -> Option<&ColorRamp> {
        self.ramps.iter().find(|ramp| ramp.name == name)
    }

    /// Converts key roles into primitive theme tokens.
    #[must_use]
    pub fn to_theme_tokens(&self) -> ThemeTokens {
        let base = ThemeTokens::default();
        ThemeTokens {
            background: self.color(ColorRole::Background).unwrap_or(base.background),
            surface: self.color(ColorRole::Surface).unwrap_or(base.surface),
            text_primary: self
                .color(ColorRole::TextPrimary)
                .unwrap_or(base.text_primary),
            text_secondary: self
                .color(ColorRole::TextSecondary)
                .unwrap_or(base.text_secondary),
            accent: self.color(ColorRole::BrandPrimary).unwrap_or(base.accent),
            border: self.color(ColorRole::Border).unwrap_or(base.border),
            focus_ring: self.color(ColorRole::Focus).unwrap_or(base.focus_ring),
            ..base
        }
    }

    /// Returns true if every built-in role has a token.
    #[must_use]
    pub fn is_exhaustive(&self) -> bool {
        ColorRole::ALL
            .iter()
            .all(|role| self.tokens.iter().any(|token| token.role == *role))
    }
}

fn glass_aurora() -> OrdoColorSystem {
    system(
        "Glass Aurora",
        [
            (
                ColorRole::Background,
                "background",
                c(0.955, 0.975, 0.99, 1.0),
            ),
            (ColorRole::Surface, "surface", c(1.0, 1.0, 1.0, 0.86)),
            (
                ColorRole::SurfaceRaised,
                "surface-raised",
                c(0.98, 0.995, 1.0, 0.94),
            ),
            (
                ColorRole::SurfacePressed,
                "surface-pressed",
                c(0.88, 0.94, 0.98, 0.9),
            ),
            (
                ColorRole::TextPrimary,
                "text-primary",
                c(0.055, 0.07, 0.095, 1.0),
            ),
            (
                ColorRole::TextSecondary,
                "text-secondary",
                c(0.29, 0.35, 0.43, 1.0),
            ),
            (ColorRole::TextMuted, "text-muted", c(0.5, 0.56, 0.64, 1.0)),
            (ColorRole::Border, "border", c(0.72, 0.82, 0.9, 0.72)),
            (ColorRole::Focus, "focus", c(0.0, 0.48, 1.0, 1.0)),
            (
                ColorRole::BrandPrimary,
                "brand-primary",
                c(0.0, 0.42, 0.9, 1.0),
            ),
            (
                ColorRole::BrandSecondary,
                "brand-secondary",
                c(0.0, 0.7, 0.58, 1.0),
            ),
            (
                ColorRole::AccentCool,
                "accent-cool",
                c(0.16, 0.76, 1.0, 1.0),
            ),
            (
                ColorRole::AccentWarm,
                "accent-warm",
                c(1.0, 0.62, 0.18, 1.0),
            ),
            (
                ColorRole::AccentSoft,
                "accent-soft",
                c(0.66, 0.55, 1.0, 1.0),
            ),
            (ColorRole::Success, "success", c(0.0, 0.68, 0.38, 1.0)),
            (ColorRole::Warning, "warning", c(0.94, 0.62, 0.0, 1.0)),
            (ColorRole::Danger, "danger", c(0.86, 0.12, 0.18, 1.0)),
            (ColorRole::Info, "info", c(0.0, 0.48, 0.92, 1.0)),
            (ColorRole::Live, "live", c(0.04, 0.92, 0.66, 1.0)),
            (ColorRole::Offline, "offline", c(0.56, 0.58, 0.62, 1.0)),
            (ColorRole::Glow, "glow", c(0.15, 0.72, 1.0, 0.52)),
            (ColorRole::Selection, "selection", c(0.0, 0.48, 1.0, 0.22)),
            (ColorRole::Data01, "data-01", c(0.0, 0.48, 1.0, 1.0)),
            (ColorRole::Data02, "data-02", c(0.0, 0.72, 0.58, 1.0)),
            (ColorRole::Data03, "data-03", c(1.0, 0.62, 0.18, 1.0)),
            (ColorRole::Data04, "data-04", c(0.72, 0.42, 1.0, 1.0)),
            (ColorRole::Data05, "data-05", c(1.0, 0.26, 0.48, 1.0)),
            (ColorRole::Data06, "data-06", c(0.1, 0.82, 1.0, 1.0)),
            (ColorRole::Data07, "data-07", c(0.42, 0.84, 0.26, 1.0)),
            (ColorRole::Data08, "data-08", c(0.98, 0.82, 0.18, 1.0)),
            (ColorRole::Data09, "data-09", c(0.36, 0.4, 0.94, 1.0)),
            (ColorRole::Data10, "data-10", c(0.9, 0.3, 0.86, 1.0)),
            (ColorRole::Data11, "data-11", c(0.16, 0.62, 0.72, 1.0)),
            (ColorRole::Data12, "data-12", c(0.78, 0.48, 0.18, 1.0)),
        ],
        PaletteAdjustment::IDENTITY,
        LivingColorMotion::subtle(),
    )
}

fn graphite_bloom() -> OrdoColorSystem {
    system(
        "Graphite Bloom",
        [
            (
                ColorRole::Background,
                "background",
                c(0.035, 0.04, 0.05, 1.0),
            ),
            (ColorRole::Surface, "surface", c(0.075, 0.085, 0.105, 1.0)),
            (
                ColorRole::SurfaceRaised,
                "surface-raised",
                c(0.11, 0.125, 0.15, 1.0),
            ),
            (
                ColorRole::SurfacePressed,
                "surface-pressed",
                c(0.055, 0.065, 0.08, 1.0),
            ),
            (
                ColorRole::TextPrimary,
                "text-primary",
                c(0.94, 0.96, 0.98, 1.0),
            ),
            (
                ColorRole::TextSecondary,
                "text-secondary",
                c(0.66, 0.7, 0.76, 1.0),
            ),
            (ColorRole::TextMuted, "text-muted", c(0.44, 0.49, 0.56, 1.0)),
            (ColorRole::Border, "border", c(0.22, 0.25, 0.3, 1.0)),
            (ColorRole::Focus, "focus", c(0.28, 0.72, 1.0, 1.0)),
            (
                ColorRole::BrandPrimary,
                "brand-primary",
                c(0.22, 0.58, 1.0, 1.0),
            ),
            (
                ColorRole::BrandSecondary,
                "brand-secondary",
                c(0.1, 0.9, 0.66, 1.0),
            ),
            (
                ColorRole::AccentCool,
                "accent-cool",
                c(0.22, 0.82, 1.0, 1.0),
            ),
            (ColorRole::AccentWarm, "accent-warm", c(1.0, 0.54, 0.2, 1.0)),
            (
                ColorRole::AccentSoft,
                "accent-soft",
                c(0.76, 0.46, 1.0, 1.0),
            ),
            (ColorRole::Success, "success", c(0.1, 0.86, 0.48, 1.0)),
            (ColorRole::Warning, "warning", c(1.0, 0.68, 0.12, 1.0)),
            (ColorRole::Danger, "danger", c(1.0, 0.2, 0.3, 1.0)),
            (ColorRole::Info, "info", c(0.3, 0.7, 1.0, 1.0)),
            (ColorRole::Live, "live", c(0.18, 1.0, 0.72, 1.0)),
            (ColorRole::Offline, "offline", c(0.42, 0.45, 0.5, 1.0)),
            (ColorRole::Glow, "glow", c(0.24, 0.68, 1.0, 0.62)),
            (ColorRole::Selection, "selection", c(0.22, 0.58, 1.0, 0.26)),
            (ColorRole::Data01, "data-01", c(0.22, 0.58, 1.0, 1.0)),
            (ColorRole::Data02, "data-02", c(0.1, 0.9, 0.66, 1.0)),
            (ColorRole::Data03, "data-03", c(1.0, 0.54, 0.2, 1.0)),
            (ColorRole::Data04, "data-04", c(0.76, 0.46, 1.0, 1.0)),
            (ColorRole::Data05, "data-05", c(1.0, 0.24, 0.52, 1.0)),
            (ColorRole::Data06, "data-06", c(0.28, 0.88, 1.0, 1.0)),
            (ColorRole::Data07, "data-07", c(0.56, 0.98, 0.36, 1.0)),
            (ColorRole::Data08, "data-08", c(1.0, 0.82, 0.2, 1.0)),
            (ColorRole::Data09, "data-09", c(0.48, 0.52, 1.0, 1.0)),
            (ColorRole::Data10, "data-10", c(1.0, 0.36, 0.92, 1.0)),
            (ColorRole::Data11, "data-11", c(0.24, 0.72, 0.8, 1.0)),
            (ColorRole::Data12, "data-12", c(0.9, 0.52, 0.22, 1.0)),
        ],
        PaletteAdjustment::vivid(),
        LivingColorMotion::subtle(),
    )
}

fn clinical_edge() -> OrdoColorSystem {
    let mut palette = glass_aurora();
    palette.name = "Clinical Edge".to_string();
    palette.adjustment = PaletteAdjustment::soft();
    palette.motion = LivingColorMotion {
        hue_drift_degrees: 1.0,
        saturation_pulse: 0.015,
        lightness_pulse: 0.012,
        ..LivingColorMotion::subtle()
    };
    palette.set_color(ColorRole::Background, c(0.985, 0.99, 0.995, 1.0));
    palette.set_color(ColorRole::BrandPrimary, c(0.0, 0.36, 0.72, 1.0));
    palette
}

fn neon_flux() -> OrdoColorSystem {
    let mut palette = graphite_bloom();
    palette.name = "Neon Flux".to_string();
    palette.adjustment = PaletteAdjustment {
        saturation: 1.28,
        lightness: 0.04,
        contrast: 1.12,
        hue_shift_degrees: 0.0,
        alpha: 1.0,
    };
    palette.motion = LivingColorMotion::expressive();
    palette
}

fn system(
    name: &str,
    tokens: [(ColorRole, &str, Color); 34],
    adjustment: PaletteAdjustment,
    motion: LivingColorMotion,
) -> OrdoColorSystem {
    let tokens: Vec<_> = tokens
        .into_iter()
        .map(|(role, name, color)| ColorToken::new(role, name, color))
        .collect();
    let data = ColorRamp::new(
        "data",
        tokens
            .iter()
            .filter(|token| {
                matches!(
                    token.role,
                    ColorRole::Data01
                        | ColorRole::Data02
                        | ColorRole::Data03
                        | ColorRole::Data04
                        | ColorRole::Data05
                        | ColorRole::Data06
                        | ColorRole::Data07
                        | ColorRole::Data08
                        | ColorRole::Data09
                        | ColorRole::Data10
                        | ColorRole::Data11
                        | ColorRole::Data12
                )
            })
            .map(|token| token.color)
            .collect(),
    );
    let signal = ColorRamp::new(
        "signal",
        [
            ColorRole::Danger,
            ColorRole::Warning,
            ColorRole::Info,
            ColorRole::Success,
            ColorRole::Live,
        ]
        .into_iter()
        .filter_map(|role| {
            tokens
                .iter()
                .find(|token| token.role == role)
                .map(|token| token.color)
        })
        .collect(),
    );

    OrdoColorSystem {
        name: name.to_string(),
        tokens,
        ramps: vec![data, signal],
        adjustment,
        motion,
    }
}

fn c(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::new([r, g, b, a])
}

fn mix(left: Color, right: Color, t: f32) -> Color {
    let left = Rgba::from(left);
    let right = Rgba::from(right);
    Color::from(Rgba {
        r: left.r + (right.r - left.r) * t,
        g: left.g + (right.g - left.g) * t,
        b: left.b + (right.b - left.b) * t,
        a: left.a + (right.a - left.a) * t,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rgba {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl From<Color> for Rgba {
    fn from(color: Color) -> Self {
        let rgba = color.to_rgba8();
        Self {
            r: f32::from(rgba.r) / 255.0,
            g: f32::from(rgba.g) / 255.0,
            b: f32::from(rgba.b) / 255.0,
            a: f32::from(rgba.a) / 255.0,
        }
    }
}

impl From<Rgba> for Color {
    fn from(rgba: Rgba) -> Self {
        c(
            clamp01(rgba.r),
            clamp01(rgba.g),
            clamp01(rgba.b),
            clamp01(rgba.a),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Hsla {
    h: f32,
    s: f32,
    l: f32,
    a: f32,
}

impl From<Rgba> for Hsla {
    fn from(rgba: Rgba) -> Self {
        let max = rgba.r.max(rgba.g).max(rgba.b);
        let min = rgba.r.min(rgba.g).min(rgba.b);
        let l = (max + min) * 0.5;
        let delta = max - min;

        if delta == 0.0 {
            return Self {
                h: 0.0,
                s: 0.0,
                l,
                a: rgba.a,
            };
        }

        let s = if l > 0.5 {
            delta / (2.0 - max - min)
        } else {
            delta / (max + min)
        };
        let h = if max == rgba.r {
            ((rgba.g - rgba.b) / delta + if rgba.g < rgba.b { 6.0 } else { 0.0 }) / 6.0
        } else if max == rgba.g {
            ((rgba.b - rgba.r) / delta + 2.0) / 6.0
        } else {
            ((rgba.r - rgba.g) / delta + 4.0) / 6.0
        };

        Self { h, s, l, a: rgba.a }
    }
}

impl From<Hsla> for Rgba {
    fn from(hsla: Hsla) -> Self {
        if hsla.s == 0.0 {
            return Self {
                r: hsla.l,
                g: hsla.l,
                b: hsla.l,
                a: hsla.a,
            };
        }

        let q = if hsla.l < 0.5 {
            hsla.l * (1.0 + hsla.s)
        } else {
            hsla.l + hsla.s - hsla.l * hsla.s
        };
        let p = 2.0 * hsla.l - q;

        Self {
            r: hue_to_rgb(p, q, hsla.h + 1.0 / 3.0),
            g: hue_to_rgb(p, q, hsla.h),
            b: hue_to_rgb(p, q, hsla.h - 1.0 / 3.0),
            a: hsla.a,
        }
    }
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    t = wrap_unit(t);
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

fn wrap_unit(value: f32) -> f32 {
    value.rem_euclid(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_are_exhaustive() {
        for preset in [
            ModernPalettePreset::GlassAurora,
            ModernPalettePreset::GraphiteBloom,
            ModernPalettePreset::ClinicalEdge,
            ModernPalettePreset::NeonFlux,
        ] {
            let system = OrdoColorSystem::preset(preset);
            assert!(
                system.is_exhaustive(),
                "{preset:?} should define every role"
            );
        }
    }

    #[test]
    fn palette_can_be_customized_by_role() {
        let mut system = OrdoColorSystem::preset(ModernPalettePreset::GlassAurora);
        let custom = c(0.9, 0.1, 0.7, 1.0);
        system.set_color(ColorRole::BrandPrimary, custom);

        assert_eq!(system.color(ColorRole::BrandPrimary), Some(custom));
    }

    #[test]
    fn adjustment_changes_color() {
        let color = c(0.2, 0.45, 0.8, 1.0);
        let adjusted = PaletteAdjustment::vivid().apply(color);

        assert_ne!(adjusted.to_rgba8(), color.to_rgba8());
    }

    #[test]
    fn living_color_samples_differ_over_time() {
        let living = LivingColor::new(c(0.2, 0.45, 0.8, 1.0), LivingColorMotion::expressive());

        assert_ne!(
            living.sample(0.25).to_rgba8(),
            living.sample(1.0).to_rgba8()
        );
    }

    #[test]
    fn ramp_samples_between_stops() {
        let ramp = ColorRamp::new("test", vec![c(0.0, 0.0, 0.0, 1.0), c(1.0, 1.0, 1.0, 1.0)]);
        let middle = ramp.sample(0.5).expect("ramp should sample");
        let rgba = Rgba::from(middle);

        assert!((rgba.r - 0.5).abs() < 0.01);
    }

    #[test]
    fn system_samples_roles_and_converts_to_theme() {
        let system = OrdoColorSystem::preset(ModernPalettePreset::NeonFlux);
        let sampled = system.sample(ColorRole::Live, 0.5);
        let theme = system.to_theme_tokens();

        assert!(sampled.is_some());
        assert_eq!(theme.accent, system.color(ColorRole::BrandPrimary).unwrap());
    }
}
