//! Renderer-neutral motion timeline experiments for Ordo UX.
//!
//! This crate evaluates animation tracks from explicit time values. It does not
//! own clocks, frame loops, renderers, or component state.

use ordo_ux_curves_experiment::{AnimationCurve, lerp};

/// Explicit time snapshot for motion sampling.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MotionTime {
    /// Seconds elapsed from an arbitrary stable origin.
    pub seconds: f64,
}

impl MotionTime {
    /// Creates a time snapshot.
    #[must_use]
    pub fn seconds(seconds: f64) -> Self {
        Self { seconds }
    }
}

/// A scalar animation track.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTrack {
    /// Start value.
    pub from: f64,
    /// End value.
    pub to: f64,
    /// Delay before animation begins.
    pub delay_seconds: f64,
    /// Duration after delay.
    pub duration_seconds: f64,
    /// Curve used for interpolation.
    pub curve: AnimationCurve,
}

impl MotionTrack {
    /// Creates a track.
    #[must_use]
    pub fn new(from: f64, to: f64, duration_seconds: f64) -> Self {
        Self {
            from,
            to,
            delay_seconds: 0.0,
            duration_seconds,
            curve: AnimationCurve::default(),
        }
    }

    /// Sets delay.
    #[must_use]
    pub fn with_delay(mut self, delay_seconds: f64) -> Self {
        self.delay_seconds = delay_seconds;
        self
    }

    /// Sets curve.
    #[must_use]
    pub fn with_curve(mut self, curve: AnimationCurve) -> Self {
        self.curve = curve;
        self
    }

    /// Samples this track at explicit time.
    #[must_use]
    pub fn sample(&self, time: MotionTime) -> f64 {
        if self.duration_seconds <= 0.0 {
            return self.to;
        }
        let t = ((time.seconds - self.delay_seconds) / self.duration_seconds).clamp(0.0, 1.0);
        lerp(self.from, self.to, t, self.curve)
    }

    /// Returns true once the track has completed.
    #[must_use]
    pub fn is_finished(&self, time: MotionTime) -> bool {
        time.seconds >= self.delay_seconds + self.duration_seconds
    }
}

/// Common UI motion tokens.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTokens {
    /// Very fast micro feedback.
    pub instant: f64,
    /// Fast UI response.
    pub fast: f64,
    /// Standard transition duration.
    pub standard: f64,
    /// Slow emphasized motion.
    pub slow: f64,
    /// Default curve.
    pub curve: AnimationCurve,
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            instant: 0.08,
            fast: 0.14,
            standard: 0.22,
            slow: 0.36,
            curve: AnimationCurve::default(),
        }
    }
}

/// A named motion value.
#[derive(Clone, Debug, PartialEq)]
pub struct MotionValue {
    /// Property name.
    pub name: String,
    /// Track.
    pub track: MotionTrack,
}

impl MotionValue {
    /// Creates a named motion value.
    #[must_use]
    pub fn new(name: impl Into<String>, track: MotionTrack) -> Self {
        Self {
            name: name.into(),
            track,
        }
    }

    /// Samples the value.
    #[must_use]
    pub fn sample(&self, time: MotionTime) -> f64 {
        self.track.sample(time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_respects_delay() {
        let track = MotionTrack::new(0.0, 10.0, 1.0).with_delay(1.0);
        assert_eq!(track.sample(MotionTime::seconds(0.5)), 0.0);
    }

    #[test]
    fn track_samples_midpoint() {
        let track = MotionTrack::new(0.0, 10.0, 1.0).with_curve(AnimationCurve::Linear);
        assert_eq!(track.sample(MotionTime::seconds(0.5)), 5.0);
    }

    #[test]
    fn tokens_have_ordered_durations() {
        let tokens = MotionTokens::default();
        assert!(tokens.fast < tokens.standard);
        assert!(tokens.standard < tokens.slow);
    }
}
