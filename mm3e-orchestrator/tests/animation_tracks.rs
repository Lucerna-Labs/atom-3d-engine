//! Exact sampling is essential for discrete poses and authored key boundaries.
use mm3e_orchestrator::anim::{Easing, Lerp, Track};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Discrete(u8);

impl Lerp for Discrete {
    fn lerp(self, _other: Self, _t: f32) -> Self {
        panic!("discrete sampling must select an authored value without interpolation")
    }
}

#[test]
fn step_holds_exact_values_and_never_calls_interpolation() {
    let track = Track::new(Easing::Step).key(0.0, Discrete(3)).key(1.0, Discrete(7)).key(2.0, Discrete(11));
    for (time, value) in [(-1.0, 3), (0.0, 3), (0.25, 3), (0.999_999, 3), (1.0, 7), (1.5, 7), (2.0, 11), (3.0, 11)] {
        assert_eq!(track.sample(time), Discrete(value), "time={time}");
    }
}

#[test]
fn all_easings_return_authored_keys_without_arithmetic() {
    for easing in [Easing::Step, Easing::Linear, Easing::SmoothStep, Easing::EaseIn, Easing::EaseOut] {
        let track = Track::new(easing).key(0.0, Discrete(3)).key(1.0, Discrete(7)).key(2.0, Discrete(11));
        for (time, value) in [(0.0, 3), (1.0, 7), (2.0, 11)] {
            assert_eq!(track.sample(time), Discrete(value), "easing={easing:?}, time={time}");
        }
    }
}

#[test]
fn disparate_scale_keys_do_not_lose_precision_at_an_interior_key() {
    // 1000 + (0.001 - 1000) evaluates to 0.0009765625 in f32: a 2.34% error.
    // Selecting the exact key preserves its authored value for every easing mode.
    for easing in [Easing::Step, Easing::Linear, Easing::SmoothStep, Easing::EaseIn, Easing::EaseOut] {
        let track = Track::new(easing).key(0.0, 1000.0_f32).key(1.0, 0.001_f32).key(2.0, 1.0_f32);
        assert_eq!(track.sample(1.0).to_bits(), 0.001_f32.to_bits(), "easing={easing:?}");
    }
    let step = Track::new(Easing::Step).key(0.0, 1000.0_f32).key(1.0, 0.001_f32).key(2.0, 1.0_f32);
    assert_eq!(step.sample(1.000_001).to_bits(), 0.001_f32.to_bits());
}

#[test]
fn continuous_easing_between_keys_is_unchanged() {
    for (easing, expected) in
        [(Easing::Linear, 12.0), (Easing::SmoothStep, 11.25), (Easing::EaseIn, 10.5), (Easing::EaseOut, 13.5)]
    {
        let track = Track::new(easing).key(1.0, 10.0_f32).key(3.0, 18.0_f32);
        assert_eq!(track.sample(1.5), expected, "easing={easing:?}");
        assert_eq!(track.sample(-1.0), 10.0);
        assert_eq!(track.sample(4.0), 18.0);
    }
}
