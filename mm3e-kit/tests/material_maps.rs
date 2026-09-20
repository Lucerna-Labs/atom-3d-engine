use mm3e_kit::{
    tangent::TangentFrame,
    texture::{Sampler, TextureImage},
    Vec3,
};
fn triangle() -> [Vec3; 3] {
    [Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)]
}
fn uv() -> [[f64; 2]; 3] {
    [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]
}
fn near(a: Vec3, b: Vec3) {
    assert!((a - b).length() < 2e-6, "{a:?} {b:?}");
}
#[test]
fn data_channels_filter_independently_of_alpha() {
    let data = TextureImage::from_data_channels(2, 1, vec![[1.0, 0.25, 0.75, 0.0], [0.0, 0.75, 0.25, 1.0]]).unwrap();
    assert!(data.is_data());
    assert_eq!(data.sample([0.5, 0.5], 10.0, Sampler::default()).unwrap(), [0.5; 4]);
    let color = TextureImage::from_linear_rgba(2, 1, vec![[1.0, 0.25, 0.75, 0.0], [0.0, 0.75, 0.25, 1.0]]).unwrap();
    assert!(!color.is_data());
    assert_eq!(color.sample([0.5, 0.5], 10.0, Sampler::default()).unwrap(), [0.0, 0.375, 0.125, 0.5]);
}
#[test]
fn neutral_strength_and_axis_convention_have_expected_normals() {
    let frame = TangentFrame::from_triangle(triangle(), uv(), Vec3::new(0.0, 0.0, 1.0)).unwrap();
    near(frame.perturb([0.5, 0.5, 1.0], 1.0, false).unwrap(), Vec3::new(0.0, 0.0, 1.0));
    near(frame.perturb([1.0, 1.0, 0.5], 0.0, false).unwrap(), Vec3::new(0.0, 0.0, 1.0));
    near(frame.perturb([0.8, 0.5, 0.9], 1.0, false).unwrap(), Vec3::new(0.6, 0.0, 0.8));
    near(frame.perturb([0.5, 0.8, 0.9], 1.0, true).unwrap(), Vec3::new(0.0, -0.6, 0.8));
}
#[test]
fn mirrored_charts_and_mirrored_geometry_preserve_uv_orientation() {
    let mirrored_uv = [[0.0, 0.0], [-1.0, 0.0], [0.0, 1.0]];
    let frame = TangentFrame::from_triangle(triangle(), mirrored_uv, Vec3::new(0.0, 0.0, 1.0)).unwrap();
    near(frame.perturb([0.8, 0.5, 0.9], 1.0, false).unwrap(), Vec3::new(-0.6, 0.0, 0.8));
    let geometry = [Vec3::ZERO, Vec3::new(-1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
    let frame =
        TangentFrame::from_triangle_with_reference(geometry, uv(), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0))
            .unwrap();
    near(frame.perturb([0.8, 0.5, 0.9], 1.0, false).unwrap(), Vec3::new(-0.6, 0.0, 0.8));
}
#[test]
fn backside_is_opposite_and_rim_fallback_stays_in_geometry_hemisphere() {
    let front = TangentFrame::from_triangle(triangle(), uv(), Vec3::new(0.0, 0.0, 1.0))
        .unwrap()
        .perturb([0.8, 0.6, 0.9], 1.0, false)
        .unwrap();
    let back = TangentFrame::from_triangle(triangle(), uv(), Vec3::new(0.0, 0.0, -1.0))
        .unwrap()
        .perturb([0.8, 0.6, 0.9], 1.0, false)
        .unwrap();
    near(back, front * -1.0);
    for normal in [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.3, 0.4, 0.5).normalize()] {
        let frame = TangentFrame::from_triangle(triangle(), uv(), normal).unwrap();
        let shaded = frame.perturb([0.8, 0.6, 0.9], 1.0, false).unwrap();
        assert!(shaded.dot(normal) > 0.0);
        near(frame.normal, normal);
        assert!(frame.tangent.dot(frame.bitangent).abs() < 1e-6);
    }
}
#[test]
fn invalid_uv_frames_and_lower_hemisphere_maps_are_errors() {
    assert!(TangentFrame::from_triangle(triangle(), [[0.0; 2]; 3], Vec3::new(0.0, 0.0, 1.0)).is_err());
    let frame = TangentFrame::from_triangle(triangle(), uv(), Vec3::new(0.0, 0.0, 1.0)).unwrap();
    assert!(frame.perturb([0.5, 0.5, 0.2], 1.0, false).is_err());
    assert!(frame.perturb([0.5; 3], 1.0, false).is_err());
}
