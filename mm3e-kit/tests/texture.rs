use mm3e_kit::texture::{CornerUvs, Filter, Sampler, TextureImage, Wrap};
fn sampler(filter: Filter) -> Sampler {
    Sampler { u: Wrap::Clamp, v: Wrap::Clamp, filter, lod_bias: 0.0 }
}
fn near(a: [f32; 4], b: [f32; 4]) {
    for (a, b) in a.into_iter().zip(b) {
        assert!((a - b).abs() < 2e-7, "{a} != {b}");
    }
}
fn image() -> TextureImage {
    TextureImage::from_linear_rgba(
        2,
        2,
        vec![[1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]],
    )
    .unwrap()
}
#[test]
fn lower_left_uvs_address_top_down_images_and_boundaries_consistently() {
    let image = image();
    for filter in [Filter::Nearest, Filter::Bilinear, Filter::Trilinear] {
        let s = sampler(filter);
        near(image.sample([0.25, 0.25], 0.0, s).unwrap(), [0.0, 0.0, 1.0, 1.0]);
        near(image.sample([0.75, 0.75], 0.0, s).unwrap(), [0.0, 1.0, 0.0, 1.0]);
    }
    let s = sampler(Filter::Nearest);
    near(image.sample([0.0, 0.0], 0.0, s).unwrap(), [0.0, 0.0, 1.0, 1.0]);
    near(image.sample([1.0, 1.0], 0.0, s).unwrap(), [0.0, 1.0, 0.0, 1.0]);
    near(image.sample([0.5, 0.5], 0.0, s).unwrap(), [0.0, 1.0, 0.0, 1.0]);
}
#[test]
fn repeat_and_mirror_cover_negative_coordinates_and_tile_edges() {
    let image = image();
    let repeat = Sampler { u: Wrap::Repeat, v: Wrap::Repeat, ..sampler(Filter::Nearest) };
    let mirror = Sampler { u: Wrap::Mirror, v: Wrap::Mirror, ..sampler(Filter::Nearest) };
    near(image.sample([0.25, 0.25], 0.0, repeat).unwrap(), image.sample([-1.75, 3.25], 0.0, repeat).unwrap());
    near(image.sample([0.0, 0.0], 0.0, repeat).unwrap(), image.sample([1.0, -1.0], 0.0, repeat).unwrap());
    near(image.sample([0.25, 0.25], 0.0, mirror).unwrap(), image.sample([-0.25, 1.75], 0.0, mirror).unwrap());
}
#[test]
fn filtering_is_linear_and_transparent_texels_do_not_leak_hidden_colors() {
    let image = TextureImage::from_linear_rgba(2, 1, vec![[1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 0.0]]).unwrap();
    near(image.sample([0.5, 0.5], 0.0, sampler(Filter::Bilinear)).unwrap(), [0.5, 0.0, 0.0, 0.5]);
    near(image.sample([0.5, 0.5], 10.0, sampler(Filter::Trilinear)).unwrap(), [0.5, 0.0, 0.0, 0.5]);
    let black_white = TextureImage::from_linear_rgba(2, 1, vec![[0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0, 1.0]]).unwrap();
    near(black_white.sample([0.5, 0.5], 0.0, sampler(Filter::Bilinear)).unwrap(), [0.5, 0.5, 0.5, 1.0]);
}
#[test]
fn odd_mipmap_sizes_preserve_area_mean_and_fractional_lod_is_continuous() {
    let pixels =
        (0..15).map(|i| [i as f32 / 14.0, (i % 3) as f32 / 2.0, (i % 5) as f32 / 4.0, 1.0]).collect::<Vec<_>>();
    let expected = std::array::from_fn(|c| pixels.iter().map(|p| f64::from(p[c])).sum::<f64>() as f32 / 15.0);
    let image = TextureImage::from_linear_rgba(3, 5, pixels).unwrap();
    assert_eq!(image.level_count(), 3);
    near(image.sample([0.2, 0.8], 100.0, sampler(Filter::Trilinear)).unwrap(), expected);
    let a = image.sample([0.2, 0.8], 0.0, sampler(Filter::Trilinear)).unwrap();
    let b = image.sample([0.2, 0.8], 1.0, sampler(Filter::Trilinear)).unwrap();
    near(
        image.sample([0.2, 0.8], 0.5, sampler(Filter::Trilinear)).unwrap(),
        std::array::from_fn(|c| (a[c] + b[c]) * 0.5),
    );
    assert_eq!(image.decoded_bytes(), (15 + 2 + 1) * 16);
}
#[test]
fn independent_corner_indices_preserve_discontinuous_uv_charts() {
    let uvs = CornerUvs::new(
        vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.2, 0.2], [0.8, 0.2], [0.8, 0.8]],
        vec![[0, 1, 2], [3, 4, 5]],
        2,
    )
    .unwrap();
    assert_eq!(uvs.interpolate(0, [0.5, 0.5, 0.0]).unwrap(), [0.5, 0.0]);
    assert_eq!(uvs.interpolate(1, [0.5, 0.5, 0.0]).unwrap(), [0.5, 0.2]);
    assert!(uvs.interpolate(0, [0.4, 0.5, 0.0]).is_err());
    assert!(uvs.interpolate(2, [1.0, 0.0, 0.0]).is_err());
}
#[test]
fn invalid_images_coordinates_and_unbounded_inputs_are_rejected() {
    assert!(TextureImage::from_linear_rgba(0, 0, vec![]).is_err());
    assert!(TextureImage::from_linear_rgba(8192, 8192, vec![]).is_err());
    assert!(TextureImage::from_linear_rgba(1, 1, vec![[0.0, 0.0, f32::NAN, 1.0]]).is_err());
    assert!(image().sample([f64::INFINITY, 0.0], 0.0, Sampler::default()).is_err());
    assert!(image().sample([0.0, 0.0], f64::NAN, Sampler::default()).is_err());
    assert!(CornerUvs::new(vec![[0.0, 0.0]], vec![[0, 1, 0]], 1).is_err());
    assert!(CornerUvs::new(vec![[f64::NAN, 0.0]], vec![[0; 3]], 1).is_err());
}
