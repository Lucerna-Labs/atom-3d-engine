//! Renderer-neutral transition intent experiments for Ordo UX.
//!
//! This crate describes how UI state moves between visual states. It samples
//! explicit time and emits transition values, not renderer commands.

use ordo_ux_motion_experiment::{MotionTime, MotionTokens, MotionTrack};

/// Transition property.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionProperty {
    /// Opacity.
    Opacity,
    /// X translation.
    TranslateX,
    /// Y translation.
    TranslateY,
    /// Uniform scale.
    Scale,
    /// Blur radius intent.
    Blur,
}

/// One transition channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransitionChannel {
    /// Property being animated.
    pub property: TransitionProperty,
    /// Scalar track.
    pub track: MotionTrack,
}

/// A transition recipe.
#[derive(Clone, Debug, PartialEq)]
pub struct Transition {
    /// Transition name.
    pub name: String,
    /// Channels.
    pub channels: Vec<TransitionChannel>,
}

impl Transition {
    /// Creates an empty transition.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            channels: Vec::new(),
        }
    }

    /// Fade in recipe.
    #[must_use]
    pub fn fade_in(tokens: MotionTokens) -> Self {
        Self::new("fade-in").with_channel(TransitionProperty::Opacity, 0.0, 1.0, tokens.standard)
    }

    /// Slide up and fade in recipe.
    #[must_use]
    pub fn slide_up(tokens: MotionTokens) -> Self {
        Self::new("slide-up")
            .with_channel(TransitionProperty::Opacity, 0.0, 1.0, tokens.standard)
            .with_channel(TransitionProperty::TranslateY, 12.0, 0.0, tokens.standard)
    }

    /// Adds a channel.
    #[must_use]
    pub fn with_channel(
        mut self,
        property: TransitionProperty,
        from: f64,
        to: f64,
        duration: f64,
    ) -> Self {
        self.channels.push(TransitionChannel {
            property,
            track: MotionTrack::new(from, to, duration),
        });
        self
    }

    /// Samples every channel.
    #[must_use]
    pub fn sample(&self, time: MotionTime) -> TransitionSample {
        TransitionSample {
            values: self
                .channels
                .iter()
                .map(|channel| (channel.property, channel.track.sample(time)))
                .collect(),
        }
    }
}

/// Sampled transition values.
#[derive(Clone, Debug, PartialEq)]
pub struct TransitionSample {
    /// Property values.
    pub values: Vec<(TransitionProperty, f64)>,
}

impl TransitionSample {
    /// Returns a sampled value by property.
    #[must_use]
    pub fn get(&self, property: TransitionProperty) -> Option<f64> {
        self.values
            .iter()
            .find(|(candidate, _)| *candidate == property)
            .map(|(_, value)| *value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fade_in_samples_opacity() {
        let transition = Transition::fade_in(MotionTokens::default());
        let sample = transition.sample(MotionTime::seconds(0.22));
        assert_eq!(sample.get(TransitionProperty::Opacity), Some(1.0));
    }

    #[test]
    fn slide_up_has_two_channels() {
        let transition = Transition::slide_up(MotionTokens::default());
        assert_eq!(transition.channels.len(), 2);
    }
}
