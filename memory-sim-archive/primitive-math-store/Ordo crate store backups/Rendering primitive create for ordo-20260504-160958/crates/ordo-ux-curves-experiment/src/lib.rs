//! Renderer-neutral animation curve experiments for Ordo UX.
//!
//! This crate owns easing and spring math only. It does not schedule frames,
//! own clocks, render, or mutate UI state.

/// A normalized animation curve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationCurve {
    /// Linear interpolation.
    Linear,
    /// Quadratic ease-in.
    EaseIn,
    /// Quadratic ease-out.
    EaseOut,
    /// Smoothstep ease-in-out.
    EaseInOut,
    /// Cubic Bezier with CSS-like control points.
    CubicBezier {
        /// First control point x.
        x1: f64,
        /// First control point y.
        y1: f64,
        /// Second control point x.
        x2: f64,
        /// Second control point y.
        y2: f64,
    },
    /// Damped spring approximation.
    Spring(SpringCurve),
    /// Stepped curve.
    Steps {
        /// Number of steps.
        steps: u32,
        /// Whether each step jumps at the start.
        jump_start: bool,
    },
}

impl AnimationCurve {
    /// Modern default curve for UI motion.
    #[must_use]
    pub fn standard() -> Self {
        Self::CubicBezier {
            x1: 0.2,
            y1: 0.0,
            x2: 0.0,
            y2: 1.0,
        }
    }

    /// Emphasized curve for larger transitions.
    #[must_use]
    pub fn emphasized() -> Self {
        Self::CubicBezier {
            x1: 0.2,
            y1: 0.0,
            x2: 0.0,
            y2: 1.0,
        }
    }

    /// Samples the curve at normalized `t`.
    #[must_use]
    pub fn sample(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match *self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseInOut => t * t * (3.0 - 2.0 * t),
            Self::CubicBezier { y1, y2, .. } => cubic_bezier_y(t, y1, y2),
            Self::Spring(spring) => spring.sample(t),
            Self::Steps { steps, jump_start } => {
                if steps == 0 {
                    t
                } else if jump_start {
                    ((t * f64::from(steps)).ceil() / f64::from(steps)).clamp(0.0, 1.0)
                } else {
                    ((t * f64::from(steps)).floor() / f64::from(steps)).clamp(0.0, 1.0)
                }
            }
        }
    }
}

impl Default for AnimationCurve {
    fn default() -> Self {
        Self::standard()
    }
}

/// Spring curve parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringCurve {
    /// Oscillation frequency.
    pub frequency: f64,
    /// Exponential damping.
    pub damping: f64,
    /// Initial overshoot amplitude.
    pub overshoot: f64,
}

impl SpringCurve {
    /// Creates a snappy spring.
    #[must_use]
    pub fn snappy() -> Self {
        Self {
            frequency: 12.0,
            damping: 7.0,
            overshoot: 0.18,
        }
    }

    /// Samples a normalized damped spring.
    #[must_use]
    pub fn sample(&self, t: f64) -> f64 {
        let decay = (-self.damping * t).exp();
        let wave = (self.frequency * t).cos();
        (1.0 - decay * wave * (1.0 + self.overshoot)).clamp(0.0, 1.2)
    }
}

/// Interpolates two values through a curve.
#[must_use]
pub fn lerp(start: f64, end: f64, t: f64, curve: AnimationCurve) -> f64 {
    start + (end - start) * curve.sample(t)
}

fn cubic_bezier_y(t: f64, y1: f64, y2: f64) -> f64 {
    let inv = 1.0 - t;
    3.0 * inv * inv * t * y1 + 3.0 * inv * t * t * y2 + t * t * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curves_sample_endpoints() {
        let curve = AnimationCurve::EaseInOut;
        assert_eq!(curve.sample(0.0), 0.0);
        assert_eq!(curve.sample(1.0), 1.0);
    }

    #[test]
    fn steps_quantize_motion() {
        let curve = AnimationCurve::Steps {
            steps: 4,
            jump_start: false,
        };
        assert_eq!(curve.sample(0.6), 0.5);
    }

    #[test]
    fn lerp_uses_curve() {
        assert_eq!(lerp(0.0, 10.0, 0.5, AnimationCurve::Linear), 5.0);
    }
}
