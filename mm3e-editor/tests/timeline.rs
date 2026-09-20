use mm3e_editor::{
    animation::Clip,
    model::{Document, Pass},
    timeline::{self, AudioPlacement, AudioTiming, FrameRate, TimelineShot},
};
use serde_json::json;

fn clip(duration: f32) -> Clip {
    serde_json::from_value(json!({"id":"performance","duration":duration,"tracks":[
        {"target":{"type":"object","id":"marker"},"keys":[{"time":0},{"time":duration,"translation":[1,0,0]}]}
    ]}))
    .unwrap()
}
fn shot() -> TimelineShot {
    TimelineShot {
        id: "dialogue".into(),
        clip: "performance".into(),
        rate: FrameRate { numerator: 24, denominator: 1 },
        start_frame: 100,
        frame_count: 24,
        clip_frame_zero: 100,
        audio: None,
    }
}

#[test]
fn integer_shot_has_exact_count_and_drives_the_existing_native_animation_pose() {
    let shot = shot();
    let clip = clip(1.0);
    let state = timeline::schedule(&shot, &clip, None).unwrap();
    assert_eq!(state.frames.len(), 24);
    assert_eq!(state.frames[0].frame, 100);
    assert_eq!(state.frames[23].frame, 123);
    assert_eq!(state.frames[23].scheduled_time, 23.0 / 24.0);
    assert_eq!(state.end_frame_exclusive, 124);
    assert_eq!(state.end_clip_time_exclusive, 1.0);
    assert!(state.sample(124).is_err());
    let sample = state.sample(112).unwrap();
    assert_eq!(sample.time, 0.5);
    let document = Document {
        objects: serde_json::from_value(json!([
            {"id":"marker","shape":{"type":"sphere","radius":0.2}}
        ]))
        .unwrap(),
        clips: vec![clip],
        shots: vec![shot.clone()],
        ..Document::default()
    };
    let before = serde_json::to_value(&document).unwrap();
    let (scene, _) = document.compile_at(&Pass::Beauty, Some(&sample)).unwrap();
    assert_eq!(scene.objects[0].xform.pos.x, 0.5);
    assert!(scene.sample_object(0, mm3e_kit::Vec3::new(0.5, 0.0, 0.0)).unwrap().dist < 0.0);
    assert_eq!(serde_json::to_value(&document).unwrap(), before);
    assert!(state.frames.iter().all(|frame| frame.audio.is_none()));
    let mut missing_clip = document.clone();
    missing_clip.clips.clear();
    assert!(missing_clip.compile(&Pass::Beauty).is_err());
}

#[test]
fn fractional_rate_audio_windows_are_gapless_and_partial_selection_retains_floor_phase() {
    let mut shot = shot();
    shot.rate = FrameRate { numerator: 24_000, denominator: 1001 };
    shot.audio = Some(AudioPlacement { asset: "spoken".into(), start_sample: 37 });
    let state = timeline::schedule(&shot, &clip(2.0), Some(AudioTiming { sample_rate: 44_100, sample_frames: 50_000 }))
        .unwrap();
    let range = state.audio_range.unwrap();
    assert_eq!(range.start_sample, 37);
    assert_eq!(range.end_sample, 44_181);
    assert_eq!(state.audio_end_fraction_numerator, Some(2400));
    assert_eq!(state.frames[0].audio.unwrap().end_sample, 1876);
    assert_eq!(state.frames[1].audio.unwrap().end_sample, 3715);
    assert_eq!(state.frames[2].audio.unwrap().end_sample, 5555);
    for pair in state.frames.windows(2) {
        assert_eq!(pair[0].audio.unwrap().end_sample, pair[1].audio.unwrap().start_sample);
    }
    let selected = state.select(102, 2).unwrap();
    assert_eq!(selected[0].index, 2);
    assert_eq!(selected[0].audio.unwrap().start_sample, 3715);
    assert_eq!(selected[0].audio.unwrap().end_sample - selected[0].audio.unwrap().start_sample, 1840);
    assert_eq!(state.frames[0].audio.unwrap().end_sample - state.frames[0].audio.unwrap().start_sample, 1839);
    assert_eq!(selected, &state.frames[2..4]);
    assert!(state.select(99, 1).is_err());
    assert!(state.select(100, 0).is_err());
    assert!(state.select(123, 2).is_err());
    assert_eq!(state.frames[12].scheduled_time, 1001.0 / 2000.0);
    let encoded = serde_json::to_vec(&state).unwrap();
    let decoded: timeline::ShotState = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(state, decoded);
    assert_eq!(
        timeline::inspect(&shot, &clip(2.0), Some(AudioTiming { sample_rate: 44_100, sample_frames: 50_000 })).unwrap(),
        serde_json::to_value(state).unwrap()
    );
}

#[test]
fn full_half_open_interval_fits_at_native_clip_precision_and_extra_frames_are_rejected() {
    let mut shot = shot();
    for (duration, count) in [(0.7, 7), (0.9, 9), (1.3, 13)] {
        shot.rate = FrameRate { numerator: 10, denominator: 1 };
        shot.frame_count = count;
        let state = timeline::schedule(&shot, &clip(duration), None).unwrap();
        assert_eq!(state.end_clip_time_exclusive as f32, duration);
        shot.frame_count = count + 1;
        assert!(timeline::schedule(&shot, &clip(duration), None).unwrap_err().contains("half-open"));
    }
    shot.rate = FrameRate { numerator: 24, denominator: 1 };
    shot.start_frame = 123;
    shot.frame_count = 2;
    // Both sample instants 23/24 and 1 are inside the old inclusive clip contract;
    // the exclusive end 25/24 extends the new shot interval and must fail.
    assert!(timeline::schedule(&shot, &clip(1.0), None).is_err());
    shot.start_frame = 124;
    shot.frame_count = 1;
    assert!(timeline::schedule(&shot, &clip(1.0), None).is_err());
    shot.start_frame = 99;
    assert!(timeline::schedule(&shot, &clip(1.0), None).unwrap_err().contains("precedes"));
}

#[test]
fn large_absolute_and_negative_frame_numbers_keep_distinct_native_samples() {
    let mut shot = shot();
    for base in [-24, 1_i64 << 60, i64::MAX - 24] {
        shot.start_frame = base;
        shot.clip_frame_zero = base;
        let state = timeline::schedule(&shot, &clip(1.0), None).unwrap();
        assert_eq!(state.frames[0].sampled_time, 0.0);
        assert_eq!(state.frames[12].sampled_time, 0.5);
        assert!(state.frames.windows(2).all(|pair| pair[0].sampled_time < pair[1].sampled_time));
    }
    // Highest allowed in-clip times and frame rate still have distinct f32 instants.
    shot.rate = FrameRate { numerator: 240, denominator: 1 };
    shot.clip_frame_zero = 0;
    shot.start_frame = 861_600;
    shot.frame_count = 2400;
    let state = timeline::schedule(&shot, &clip(3600.0), None).unwrap();
    assert_eq!(state.end_clip_time_exclusive, 3600.0);
    assert!(state.frames.windows(2).all(|pair| pair[0].sampled_time < pair[1].sampled_time));
    // Beyond the accepted duration/rate envelope, precision-collapse candidates are
    // rejected before an indistinguishable sequence can be delivered.
    shot.start_frame = 1_i64 << 60;
    shot.frame_count = 2;
    assert!(timeline::schedule(&shot, &clip(3600.0), None).is_err());
}

#[test]
fn positive_rate_frame_count_and_integer_overflow_limits_fail_without_partial_schedule() {
    let mut shot = shot();
    for rate in [
        FrameRate { numerator: 0, denominator: 1 },
        FrameRate { numerator: 1, denominator: 0 },
        FrameRate { numerator: 1, denominator: 1001 },
        FrameRate { numerator: 241, denominator: 1 },
    ] {
        shot.rate = rate;
        assert!(timeline::schedule(&shot, &clip(1.0), None).is_err());
    }
    shot.rate = FrameRate { numerator: 1, denominator: 1000 };
    shot.frame_count = 3;
    assert_eq!(timeline::schedule(&shot, &clip(3600.0), None).unwrap().end_clip_time_exclusive, 3000.0);
    shot.rate = FrameRate { numerator: 240, denominator: 1 };
    shot.frame_count = 2400;
    timeline::schedule(&shot, &clip(10.0), None).unwrap();
    for count in [0, 2401, u32::MAX] {
        shot.frame_count = count;
        assert!(timeline::schedule(&shot, &clip(3600.0), None).is_err());
    }
    shot.frame_count = 1;
    shot.start_frame = i64::MAX;
    shot.clip_frame_zero = i64::MAX;
    assert!(timeline::schedule(&shot, &clip(1.0), None).unwrap_err().contains("overflow"));
    shot.start_frame = 0;
    shot.clip_frame_zero = i64::MIN;
    assert!(timeline::schedule(&shot, &clip(1.0), None).unwrap_err().contains("overflow"));
}

#[test]
fn pcm_bounds_and_checked_wide_arithmetic_do_not_pad_resample_or_wrap() {
    let mut shot = shot();
    shot.audio = Some(AudioPlacement { asset: "spoken".into(), start_sample: 13 });
    let state = timeline::schedule(&shot, &clip(1.0), Some(AudioTiming { sample_rate: 48_000, sample_frames: 48_013 }))
        .unwrap();
    assert_eq!(state.audio_range.unwrap().end_sample, 48_013);
    assert!(timeline::schedule(&shot, &clip(1.0), Some(AudioTiming { sample_rate: 48_000, sample_frames: 48_012 }))
        .is_err());
    for timing in
        [AudioTiming { sample_rate: 0, sample_frames: 100 }, AudioTiming { sample_rate: 48_000, sample_frames: 0 }]
    {
        assert!(timeline::schedule(&shot, &clip(1.0), Some(timing)).is_err());
    }
    assert!(timeline::schedule(&shot, &clip(1.0), None).is_err());
    shot.audio.as_mut().unwrap().start_sample = u64::MAX - 1;
    assert!(timeline::schedule(&shot, &clip(1.0), Some(AudioTiming { sample_rate: 48_000, sample_frames: u64::MAX }))
        .unwrap_err()
        .contains("overflow"));
    shot.audio.as_mut().unwrap().start_sample = 0;
    shot.rate = FrameRate { numerator: u32::MAX, denominator: u32::MAX };
    shot.frame_count = 2400;
    let expected = 2400_u64 * u64::from(u32::MAX);
    let state =
        timeline::schedule(&shot, &clip(2400.0), Some(AudioTiming { sample_rate: u32::MAX, sample_frames: expected }))
            .unwrap();
    assert_eq!(state.audio_range.unwrap().end_sample, expected);
    assert_eq!(state.audio_end_fraction_numerator, Some(0));
    // Intermediate index*sample_rate*denominator exceeds u64 while the quotient fits.
    assert!(2400_u128 * u128::from(u32::MAX) * u128::from(u32::MAX) > u128::from(u64::MAX));
    shot.audio = None;
    assert!(timeline::schedule(&shot, &clip(2400.0), Some(AudioTiming { sample_rate: 48_000, sample_frames: 100 }))
        .is_err());
}

#[test]
fn low_sample_rate_empty_frame_windows_keep_exact_source_indices_without_silence() {
    let mut shot = shot();
    shot.audio = Some(AudioPlacement { asset: "slow-pcm".into(), start_sample: 0 });
    let state = timeline::schedule(&shot, &clip(1.0), Some(AudioTiming { sample_rate: 1, sample_frames: 1 })).unwrap();
    assert!(state.frames[..23].iter().all(|f| f.audio.unwrap().start_sample == 0 && f.audio.unwrap().end_sample == 0));
    assert_eq!(state.frames[23].audio.unwrap().end_sample, 1);
    assert_eq!(state.audio_range.unwrap().end_sample, 1);
}

#[test]
fn durable_validation_rejects_duplicate_missing_and_unbounded_references() {
    let shots = vec![shot()];
    let clips = vec![clip(1.0)];
    timeline::validate(&shots, &clips, |_| panic!("silent shot must not consult audio metadata")).unwrap();
    assert!(timeline::validate(&[shot(), shot()], &clips, |_| unreachable!()).is_err());
    assert!(timeline::validate(&vec![shot(); 65], &clips, |_| unreachable!()).is_err());
    assert!(timeline::validate(&shots, &[], |_| unreachable!()).is_err());
    let mut with_audio = shot();
    with_audio.audio = Some(AudioPlacement { asset: "spoken".into(), start_sample: 0 });
    let mut lookups = 0;
    timeline::validate(&[with_audio.clone()], &clips, |id| {
        lookups += 1;
        assert_eq!(id, "spoken");
        Ok(AudioTiming { sample_rate: 48_000, sample_frames: 48_000 })
    })
    .unwrap();
    assert_eq!(lookups, 1);
    assert!(timeline::validate(&[with_audio], &clips, |_| Err("missing audio".into())).is_err());
    let mut mismatch = shot();
    mismatch.clip = "other".into();
    assert!(timeline::schedule(&mismatch, &clips[0], None).is_err());
    let mut bad = shot();
    bad.id = "bad id".into();
    assert!(timeline::schedule(&bad, &clips[0], None).is_err());
}
