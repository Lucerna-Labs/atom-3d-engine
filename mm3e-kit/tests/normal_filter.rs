use mm3e_kit::texture::{Filter, Sampler, TextureImage, Wrap};

fn checker() -> TextureImage {
    TextureImage::from_data_channels(
        4,
        4,
        (0..16).map(|i| [if (i / 4 + i % 4) % 2 == 0 { 0.8 } else { 0.2 }, 0.5, 0.9, 0.0]).collect(),
    )
    .unwrap()
}

#[test]
fn constant_quantized_and_grazing_normals_have_exactly_zero_variance() {
    for rgb in [[204.0 / 255.0, 128.0 / 255.0, 230.0 / 255.0], [1.0, 0.0, 0.5]] {
        let image = TextureImage::from_data_channels(7, 5, vec![[rgb[0], rgb[1], rgb[2], 0.0]; 35]).unwrap();
        for uv in [[0.0, 0.0], [0.173, 0.897], [-4.13, 7.98]] {
            for lod in [0.0, 0.00001, 0.5, 1.0, 1.99, 999.0] {
                assert_eq!(image.normal_variance(uv, lod, Sampler::default()).unwrap(), 0.0);
            }
        }
    }
}

#[test]
fn checker_matches_source_slope_variance_and_continuous_lod_boundaries() {
    let image = checker();
    for uv in [[0.0, 0.0], [0.33, 0.71], [-0.3, 4.1]] {
        for lod in [1.0, 1.5, 2.0, 8.0] {
            let variance = image.normal_variance(uv, lod, Sampler::default()).unwrap();
            assert!((variance - 0.5625).abs() < 2e-7, "{variance}");
        }
        for boundary in [0.0, 1.0, 2.0] {
            let left = image.normal_variance(uv, boundary - 1e-7, Sampler::default()).unwrap();
            let right = image.normal_variance(uv, boundary + 1e-7, Sampler::default()).unwrap();
            assert!((left - right).abs() < 1e-6, "LOD {boundary}: {left}, {right}");
        }
    }
}

#[test]
fn odd_mips_retain_source_population_variance() {
    let pixels: Vec<[f32; 4]> =
        (0..35).map(|i| [0.1 + (i % 7) as f32 / 10.0, 0.2 + (i / 7) as f32 / 10.0, 0.8, 1.0]).collect();
    let slopes: Vec<[f64; 2]> = pixels
        .iter()
        .map(|p| std::array::from_fn(|i| (2.0 * f64::from(p[i]) - 1.0) / (2.0 * f64::from(p[2]) - 1.0)))
        .collect();
    let mean: [f64; 2] = std::array::from_fn(|i| slopes.iter().map(|s| s[i]).sum::<f64>() / 35.0);
    let reference = slopes.iter().map(|s| (s[0] - mean[0]).powi(2) + (s[1] - mean[1]).powi(2)).sum::<f64>() / 35.0;
    let image = TextureImage::from_data_channels(7, 5, pixels).unwrap();
    assert!((image.normal_variance([0.123, 0.876], 999.0, Sampler::default()).unwrap() - reference).abs() < 1e-7);
}

#[test]
fn unminified_and_unfiltered_queries_allocate_nothing_and_bias_is_applied_once() {
    let image = checker();
    let bytes = image.decoded_bytes();
    for filter in [Filter::Nearest, Filter::Bilinear, Filter::Trilinear] {
        let lod = if filter == Filter::Trilinear { 0.0 } else { 99.0 };
        assert_eq!(image.normal_variance([0.12, 0.34], lod, Sampler { filter, ..Sampler::default() }).unwrap(), 0.0);
    }
    assert_eq!(image.normal_moment_bytes(), 0);
    assert_eq!(image.decoded_bytes(), bytes);
    assert_eq!(
        image.normal_variance([0.12, 0.34], 1.0, Sampler { lod_bias: -1.0, ..Sampler::default() }).unwrap(),
        0.0
    );
    let a = image.normal_variance([0.12, 0.34], 0.0, Sampler { lod_bias: 1.0, ..Sampler::default() }).unwrap();
    let b = image.normal_variance([0.12, 0.34], 1.0, Sampler::default()).unwrap();
    assert_eq!(a, b);
    assert_eq!(image.normal_moment_bytes(), (16 + 4 + 1) * 12);
    assert_eq!(image.decoded_bytes(), bytes + (16 + 4 + 1) * 12);
    let clone = image.clone();
    assert_eq!(clone.normal_moment_bytes(), image.normal_moment_bytes());
}

#[test]
fn variance_uses_normal_sampler_wrap_and_rejects_invalid_inputs() {
    let pixels = (0..16)
        .map(|i| {
            [
                if i % 4 < 2 {
                    0.5
                } else if i % 2 == 0 {
                    0.8
                } else {
                    0.2
                },
                0.5,
                0.9,
                1.0,
            ]
        })
        .collect();
    let image = TextureImage::from_data_channels(4, 4, pixels).unwrap();
    let clamp = Sampler { u: Wrap::Clamp, v: Wrap::Clamp, ..Sampler::default() };
    assert_eq!(image.normal_variance([-1.75, 0.5], 1.0, clamp).unwrap(), 0.0);
    let repeated = image.normal_variance([1.75, 0.5], 1.0, Sampler::default()).unwrap();
    assert!(repeated > 0.5);
    assert_eq!(image.normal_variance([1.75, 0.5], 1.0, Sampler { u: Wrap::Mirror, ..clamp }).unwrap(), 0.0);
    assert!(image.normal_variance([f64::NAN, 0.0], 0.0, clamp).is_err());
    assert!(image.normal_variance([0.0, 0.0], f64::INFINITY, clamp).is_err());
    let color = TextureImage::from_linear_rgba(1, 1, vec![[1.0; 4]]).unwrap();
    assert!(color.normal_variance([0.0; 2], 0.0, clamp).is_err());
    let invalid = TextureImage::from_data_channels(2, 2, vec![[0.5, 0.5, 0.1, 1.0]; 4]).unwrap();
    assert!(invalid.normal_variance([0.0; 2], 1.0, clamp).is_err());
    assert_eq!(invalid.normal_moment_bytes(), 0);
}
