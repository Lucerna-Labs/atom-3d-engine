//! Renderer-neutral micro-interaction experiments for Ordo UX.
//!
//! This crate maps interaction state to small feedback values like press scale,
//! hover lift, focus ring opacity, and drag emphasis. It does not listen to
//! input events or render feedback.

use ordo_ux_motion_experiment::{MotionTime, MotionTokens, MotionTrack};

/// Interaction state for a component.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InteractionState {
    /// Resting.
    #[default]
    Idle,
    /// Pointer hover.
    Hovered,
    /// Pressed.
    Pressed,
    /// Focused.
    Focused,
    /// Dragging.
    Dragging,
    /// Disabled.
    Disabled,
}

/// Feedback values for a micro-interaction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MicroFeedback {
    /// Visual scale.
    pub scale: f64,
    /// Elevation/lift intent.
    pub elevation: f64,
    /// Focus ring opacity.
    pub focus_opacity: f64,
    /// Content opacity.
    pub opacity: f64,
}

/// Micro-interaction recipe.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MicroInteraction {
    /// Current state.
    pub state: InteractionState,
    /// Motion tokens.
    pub tokens: MotionTokens,
}

impl MicroInteraction {
    /// Creates a micro-interaction.
    #[must_use]
    pub fn new(state: InteractionState) -> Self {
        Self {
            state,
            tokens: MotionTokens::default(),
        }
    }

    /// Samples feedback.
    #[must_use]
    pub fn sample(&self, time: MotionTime) -> MicroFeedback {
        let target = match self.state {
            InteractionState::Idle => MicroFeedback {
                scale: 1.0,
                elevation: 0.0,
                focus_opacity: 0.0,
                opacity: 1.0,
            },
            InteractionState::Hovered => MicroFeedback {
                scale: 1.01,
                elevation: 2.0,
                focus_opacity: 0.0,
                opacity: 1.0,
            },
            InteractionState::Pressed => MicroFeedback {
                scale: 0.97,
                elevation: 0.0,
                focus_opacity: 0.0,
                opacity: 1.0,
            },
            InteractionState::Focused => MicroFeedback {
                scale: 1.0,
                elevation: 1.0,
                focus_opacity: 1.0,
                opacity: 1.0,
            },
            InteractionState::Dragging => MicroFeedback {
                scale: 1.03,
                elevation: 8.0,
                focus_opacity: 0.4,
                opacity: 0.92,
            },
            InteractionState::Disabled => MicroFeedback {
                scale: 1.0,
                elevation: 0.0,
                focus_opacity: 0.0,
                opacity: 0.45,
            },
        };
        let duration = self.tokens.fast;
        MicroFeedback {
            scale: MotionTrack::new(1.0, target.scale, duration).sample(time),
            elevation: MotionTrack::new(0.0, target.elevation, duration).sample(time),
            focus_opacity: MotionTrack::new(0.0, target.focus_opacity, duration).sample(time),
            opacity: MotionTrack::new(1.0, target.opacity, duration).sample(time),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressed_feedback_scales_down() {
        let feedback =
            MicroInteraction::new(InteractionState::Pressed).sample(MotionTime::seconds(1.0));
        assert!(feedback.scale < 1.0);
    }

    #[test]
    fn dragging_feedback_lifts() {
        let feedback =
            MicroInteraction::new(InteractionState::Dragging).sample(MotionTime::seconds(1.0));
        assert!(feedback.elevation > 0.0);
    }
}
