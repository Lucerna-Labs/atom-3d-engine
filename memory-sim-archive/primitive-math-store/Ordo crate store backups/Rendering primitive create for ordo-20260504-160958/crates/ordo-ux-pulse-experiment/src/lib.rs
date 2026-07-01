//! Pulsing indicators and button primitive experiments for Ordo UX.
//!
//! This crate does not animate, render, or own a clock. It defines small
//! renderer-neutral component models that produce [`ordo_ux_primitives::Primitive`]
//! values from explicit state and time inputs.

use std::f64::consts::TAU;

use kurbo::{Point, Size};
use ordo_ux_primitives::{
    Bounds, CursorHint, Fill, HitRegion, Primitive, Shape, Stroke, TextAlign, TextRun, ThemeTokens,
};
use peniko::Color;

/// A snapshot of time supplied by the application or renderer shell.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PulseTime {
    /// Seconds elapsed from an arbitrary stable origin.
    pub seconds: f64,
}

impl PulseTime {
    /// Creates a time snapshot.
    #[must_use]
    pub fn seconds(seconds: f64) -> Self {
        Self { seconds }
    }
}

/// Pulse behavior for components that glow, breathe, or call attention.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pulse {
    /// Whether the pulse is active.
    pub enabled: bool,
    /// Full cycle duration in seconds.
    pub period_seconds: f64,
    /// Minimum alpha contribution.
    pub min_alpha: f32,
    /// Maximum alpha contribution.
    pub max_alpha: f32,
    /// How far the glow expands at peak intensity.
    pub radius_growth: f64,
}

impl Pulse {
    /// Creates a breathing pulse.
    #[must_use]
    pub fn breathing(period_seconds: f64) -> Self {
        Self {
            enabled: true,
            period_seconds,
            min_alpha: 0.18,
            max_alpha: 0.68,
            radius_growth: 6.0,
        }
    }

    /// Creates a disabled pulse.
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            period_seconds: 1.0,
            min_alpha: 0.0,
            max_alpha: 0.0,
            radius_growth: 0.0,
        }
    }

    /// Evaluates this pulse at a time snapshot.
    #[must_use]
    pub fn sample(&self, time: PulseTime) -> PulseSample {
        if !self.enabled || self.period_seconds <= 0.0 {
            return PulseSample {
                intensity: 0.0,
                alpha: 0.0,
                radius_growth: 0.0,
            };
        }

        let wave = ((time.seconds / self.period_seconds) * TAU).sin();
        let intensity = ((wave + 1.0) * 0.5) as f32;
        let alpha = self.min_alpha + (self.max_alpha - self.min_alpha) * intensity;

        PulseSample {
            intensity,
            alpha,
            radius_growth: self.radius_growth * f64::from(intensity),
        }
    }
}

impl Default for Pulse {
    fn default() -> Self {
        Self::breathing(1.4)
    }
}

/// Evaluated pulse values for one frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PulseSample {
    /// Normalized pulse intensity from `0.0` to `1.0`.
    pub intensity: f32,
    /// Alpha value to use for glow primitives.
    pub alpha: f32,
    /// Radius growth to apply to glow bounds.
    pub radius_growth: f64,
}

/// Semantic state for an indicator light.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndicatorTone {
    /// Neutral or idle.
    Neutral,
    /// Successful, available, or online.
    Success,
    /// Warning or degraded status.
    Warning,
    /// Error, blocked, or offline.
    Danger,
    /// Informational active state.
    Info,
}

impl IndicatorTone {
    /// Returns a default color for the tone.
    #[must_use]
    pub fn color(self) -> Color {
        match self {
            Self::Neutral => Color::new([0.52, 0.56, 0.62, 1.0]),
            Self::Success => Color::new([0.0, 0.68, 0.38, 1.0]),
            Self::Warning => Color::new([0.94, 0.58, 0.0, 1.0]),
            Self::Danger => Color::new([0.86, 0.12, 0.18, 1.0]),
            Self::Info => Color::new([0.0, 0.46, 0.92, 1.0]),
        }
    }
}

/// A small pulsing status indicator.
#[derive(Clone, Debug, PartialEq)]
pub struct LightIndicator {
    /// Stable application-defined identifier.
    pub id: String,
    /// Indicator center point.
    pub center: Point,
    /// Core dot radius.
    pub radius: f64,
    /// Semantic tone.
    pub tone: IndicatorTone,
    /// Pulse behavior.
    pub pulse: Pulse,
    /// Whether this indicator should expose a hit region.
    pub interactive: bool,
}

impl LightIndicator {
    /// Creates a new light indicator.
    #[must_use]
    pub fn new(id: impl Into<String>, center: Point, radius: f64, tone: IndicatorTone) -> Self {
        Self {
            id: id.into(),
            center,
            radius,
            tone,
            pulse: Pulse::default(),
            interactive: false,
        }
    }

    /// Sets pulse behavior.
    #[must_use]
    pub fn with_pulse(mut self, pulse: Pulse) -> Self {
        self.pulse = pulse;
        self
    }

    /// Makes the indicator interactive.
    #[must_use]
    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    /// Returns renderer-neutral primitives for this indicator.
    #[must_use]
    pub fn primitives(&self, time: PulseTime) -> Vec<Primitive> {
        let sample = self.pulse.sample(time);
        let color = self.tone.color();
        let mut primitives = Vec::new();

        if sample.alpha > 0.0 {
            let glow_color = color.with_alpha(sample.alpha);
            primitives.push(Primitive::fill(
                Shape::circle(self.center, self.radius + sample.radius_growth),
                Fill::new(glow_color),
            ));
        }

        let hit_region = self.interactive.then(|| self.hit_region());
        primitives.push(
            Primitive::fill(Shape::circle(self.center, self.radius), Fill::new(color))
                .with_hit_region_option(hit_region),
        );

        primitives
    }

    /// Returns the indicator hit region.
    #[must_use]
    pub fn hit_region(&self) -> HitRegion {
        let diameter = self.radius * 2.0;
        HitRegion::new(
            self.id.clone(),
            Bounds::new(
                Point::new(self.center.x - self.radius, self.center.y - self.radius),
                Size::new(diameter, diameter),
            ),
        )
        .with_shape(Shape::circle(self.center, self.radius))
        .with_cursor(CursorHint::Pointer)
    }
}

/// Visual state for a button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonState {
    /// Resting state.
    #[default]
    Normal,
    /// Pointer hover.
    Hovered,
    /// Pressed by pointer or keyboard.
    Pressed,
    /// Focused by keyboard or accessibility navigation.
    Focused,
    /// Disabled and not interactive.
    Disabled,
}

/// A renderer-neutral button model.
#[derive(Clone, Debug, PartialEq)]
pub struct PulseButton {
    /// Stable application-defined identifier.
    pub id: String,
    /// Button label.
    pub label: String,
    /// Button bounds.
    pub bounds: Bounds,
    /// Current visual state.
    pub state: ButtonState,
    /// Optional pulse for attention or active status.
    pub pulse: Option<Pulse>,
}

impl PulseButton {
    /// Creates a button.
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>, bounds: Bounds) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            bounds,
            state: ButtonState::Normal,
            pulse: None,
        }
    }

    /// Sets button state.
    #[must_use]
    pub fn with_state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }

    /// Adds pulse behavior.
    #[must_use]
    pub fn with_pulse(mut self, pulse: Pulse) -> Self {
        self.pulse = Some(pulse);
        self
    }

    /// Returns renderer-neutral primitives for this button.
    #[must_use]
    pub fn primitives(&self, theme: &ThemeTokens, time: PulseTime) -> Vec<Primitive> {
        let mut primitives = Vec::new();
        let radius = theme.radius_md;

        if let Some(pulse) = self.pulse {
            let sample = pulse.sample(time);
            if sample.alpha > 0.0 {
                primitives.push(Primitive::fill(
                    Shape::rounded_rect(
                        self.bounds.rect.x0 - sample.radius_growth,
                        self.bounds.rect.y0 - sample.radius_growth,
                        self.bounds.rect.width() + sample.radius_growth * 2.0,
                        self.bounds.rect.height() + sample.radius_growth * 2.0,
                        radius + sample.radius_growth,
                    ),
                    Fill::new(theme.accent.with_alpha(sample.alpha)),
                ));
            }
        }

        primitives.push(Primitive::fill(
            Shape::rounded_rect(
                self.bounds.rect.x0,
                self.bounds.rect.y0,
                self.bounds.rect.width(),
                self.bounds.rect.height(),
                radius,
            ),
            Fill::new(self.background_color(theme)),
        ));

        primitives.push(Primitive::stroke(
            Shape::rounded_rect(
                self.bounds.rect.x0,
                self.bounds.rect.y0,
                self.bounds.rect.width(),
                self.bounds.rect.height(),
                radius,
            ),
            Stroke::new(self.border_color(theme), 1.0),
        ));

        if self.state == ButtonState::Focused {
            primitives.push(Primitive::stroke(
                Shape::rounded_rect(
                    self.bounds.rect.x0 - 2.0,
                    self.bounds.rect.y0 - 2.0,
                    self.bounds.rect.width() + 4.0,
                    self.bounds.rect.height() + 4.0,
                    radius + 2.0,
                ),
                Stroke::new(theme.focus_ring, 2.0),
            ));
        }

        let text_origin = Point::new(
            self.bounds.rect.x0 + self.bounds.rect.width() / 2.0,
            self.bounds.rect.y0 + self.bounds.rect.height() / 2.0 + theme.font_size * 0.35,
        );
        primitives.push(Primitive::text(
            TextRun::new(self.label.clone(), text_origin)
                .with_font(theme.font_family.clone(), theme.font_size)
                .with_align(TextAlign::Center)
                .with_fill(Fill::new(self.text_color(theme)))
                .with_bounds(self.bounds),
        ));

        if self.state != ButtonState::Disabled {
            let hit_region = HitRegion::new(self.id.clone(), self.bounds)
                .with_shape(Shape::rounded_rect(
                    self.bounds.rect.x0,
                    self.bounds.rect.y0,
                    self.bounds.rect.width(),
                    self.bounds.rect.height(),
                    radius,
                ))
                .with_cursor(CursorHint::Pointer);

            if let Some(last) = primitives.last_mut() {
                *last = last.clone().with_hit_region(hit_region);
            }
        }

        primitives
    }

    fn background_color(&self, theme: &ThemeTokens) -> Color {
        match self.state {
            ButtonState::Normal | ButtonState::Focused => theme.surface,
            ButtonState::Hovered => theme.accent.with_alpha(0.12),
            ButtonState::Pressed => theme.accent.with_alpha(0.2),
            ButtonState::Disabled => theme.border.with_alpha(0.28),
        }
    }

    fn border_color(&self, theme: &ThemeTokens) -> Color {
        match self.state {
            ButtonState::Focused => theme.focus_ring,
            ButtonState::Hovered | ButtonState::Pressed => theme.accent,
            ButtonState::Disabled => theme.border.with_alpha(0.4),
            ButtonState::Normal => theme.border,
        }
    }

    fn text_color(&self, theme: &ThemeTokens) -> Color {
        match self.state {
            ButtonState::Disabled => theme.text_secondary.with_alpha(0.55),
            _ => theme.text_primary,
        }
    }
}

trait PrimitiveOptionExt {
    fn with_hit_region_option(self, hit_region: Option<HitRegion>) -> Self;
}

impl PrimitiveOptionExt for Primitive {
    fn with_hit_region_option(self, hit_region: Option<HitRegion>) -> Self {
        match hit_region {
            Some(hit_region) => self.with_hit_region(hit_region),
            None => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_pulse_samples_to_zero() {
        let sample = Pulse::disabled().sample(PulseTime::seconds(0.25));

        assert_eq!(sample.intensity, 0.0);
        assert_eq!(sample.alpha, 0.0);
        assert_eq!(sample.radius_growth, 0.0);
    }

    #[test]
    fn breathing_pulse_changes_over_time() {
        let pulse = Pulse::breathing(1.0);
        let low = pulse.sample(PulseTime::seconds(0.75));
        let high = pulse.sample(PulseTime::seconds(0.25));

        assert!(high.alpha > low.alpha);
        assert!(high.radius_growth > low.radius_growth);
    }

    #[test]
    fn indicator_emits_glow_and_core() {
        let indicator = LightIndicator::new(
            "online",
            Point::new(24.0, 24.0),
            5.0,
            IndicatorTone::Success,
        )
        .interactive();

        let primitives = indicator.primitives(PulseTime::seconds(0.25));

        assert_eq!(primitives.len(), 2);
        assert!(
            indicator
                .hit_region()
                .bounds
                .contains(Point::new(24.0, 24.0))
        );
    }

    #[test]
    fn button_emits_body_border_and_text() {
        let theme = ThemeTokens::default();
        let button = PulseButton::new("save", "Save", Bounds::from_xywh(0.0, 0.0, 96.0, 36.0));

        let primitives = button.primitives(&theme, PulseTime::seconds(0.0));

        assert_eq!(primitives.len(), 3);
    }

    #[test]
    fn focused_pulsing_button_emits_extra_primitives() {
        let theme = ThemeTokens::default();
        let button = PulseButton::new("deploy", "Deploy", Bounds::from_xywh(0.0, 0.0, 112.0, 40.0))
            .with_state(ButtonState::Focused)
            .with_pulse(Pulse::breathing(1.0));

        let primitives = button.primitives(&theme, PulseTime::seconds(0.25));

        assert_eq!(primitives.len(), 5);
    }

    #[test]
    fn disabled_button_has_no_hit_region() {
        let theme = ThemeTokens::default();
        let button = PulseButton::new("delete", "Delete", Bounds::from_xywh(0.0, 0.0, 96.0, 36.0))
            .with_state(ButtonState::Disabled);

        let primitives = button.primitives(&theme, PulseTime::seconds(0.0));

        assert!(matches!(
            primitives.last(),
            Some(Primitive::Text(text)) if text.hit_region.is_none()
        ));
    }
}
