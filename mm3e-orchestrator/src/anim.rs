//! Animation: keyframe tracks sampled at a time `t`.
//!
//! The engine is stateless — `render()` rebuilds the world every frame — so animation needs no
//! new architecture: time is just one more parameter. You evaluate the tracks at `t`, build the
//! `Scene` from the sampled values, and render. A `Track` is `compare` (locate the segment) +
//! `combine` (interpolate); rotations interpolate with `Quat::slerp`.

use mm3e_kit::vec::{Quat, Vec3};

/// Values a track can interpolate.
pub trait Lerp: Copy {
    fn lerp(self, other: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }
}
impl Lerp for Vec3 {
    fn lerp(self, other: Self, t: f32) -> Self {
        self.mix(other, t)
    }
}
impl Lerp for Quat {
    fn lerp(self, other: Self, t: f32) -> Self {
        self.slerp(other, t)
    }
}

/// Per-segment easing applied to the normalized time before interpolation.
#[derive(Clone, Copy, Debug)]
pub enum Easing {
    Linear,
    SmoothStep,
    EaseIn,
    EaseOut,
}

impl Easing {
    fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::SmoothStep => t * t * (3.0 - 2.0 * t),
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
        }
    }
}

/// A keyframed value: `(time, value)` pairs interpolated with an easing. Times should be
/// ascending; sampling clamps to the endpoints outside the keyed range.
#[derive(Clone, Debug)]
pub struct Track<T> {
    keys: Vec<(f32, T)>,
    easing: Easing,
}

impl<T: Lerp> Track<T> {
    pub fn new(easing: Easing) -> Track<T> {
        Track { keys: Vec::new(), easing }
    }
    /// A track that holds a single constant value.
    pub fn constant(v: T) -> Track<T> {
        Track { keys: vec![(0.0, v)], easing: Easing::Linear }
    }
    /// Add a keyframe (builder form).
    pub fn key(mut self, time: f32, value: T) -> Track<T> {
        self.keys.push((time, value));
        self
    }
    /// Try to sample the track, returning `None` when it has no keyframes.
    pub fn try_sample(&self, time: f32) -> Option<T> {
        let keys = &self.keys;
        let first = keys.first()?;
        if time <= first.0 {
            return Some(first.1);
        }
        let last = keys.last().unwrap();
        if time >= last.0 {
            return Some(last.1);
        }
        for w in keys.windows(2) {
            let (t0, v0) = w[0];
            let (t1, v1) = w[1];
            if time >= t0 && time <= t1 {
                let local = if t1 > t0 { (time - t0) / (t1 - t0) } else { 0.0 };
                return Some(v0.lerp(v1, self.easing.apply(local)));
            }
        }
        Some(last.1)
    }
    /// Sample the track at `time`, clamping to the endpoints and easing within each segment.
    pub fn sample(&self, time: f32) -> T {
        self.try_sample(time).expect("Track::sample on a track with no keyframes")
    }
}

/// A `0 → 1` ping-pong wave (handy for back-and-forth motion without authoring return keys).
pub fn ping_pong(t: f32) -> f32 {
    let t = t.rem_euclid(2.0);
    if t <= 1.0 {
        t
    } else {
        2.0 - t
    }
}
