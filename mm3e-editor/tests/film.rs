use exr::{math::Vec2, prelude::*};
use mm3e_editor::film::encode_exr;
use mm3e_kit::vec::Vec3;
use std::io::Cursor;

#[test]
fn exr_reader_recovers_float_rgb_hdr_and_chromaticities_without_display_conversion() {
    let input = [
        Vec3::new(0.0, 0.125, 64.0),
        Vec3::new(3.5, -0.25, 0.001),
        Vec3::new(1000.0, 1.0, 2.0),
        Vec3::new(-1.0, 0.5, 9.0),
        Vec3::new(7.0, 11.0, 13.0),
        Vec3::new(17.0, 19.0, 23.0),
    ];
    let bytes = encode_exr(3, 2, &input).unwrap();
    let decoded = read()
        .no_deep_data()
        .largest_resolution_level()
        .all_channels()
        .all_layers()
        .all_attributes()
        .non_parallel()
        .from_buffered(Cursor::new(bytes))
        .unwrap();
    assert_eq!(decoded.layer_data.len(), 1);
    let layer = &decoded.layer_data[0];
    assert_eq!(layer.size, Vec2(3, 2));
    assert_eq!(layer.channel_data.list.len(), 3);
    for channel in &layer.channel_data.list {
        let expected: Vec<f32> = input
            .iter()
            .map(|p| match channel.name.to_string().as_str() {
                "R" => p.x,
                "G" => p.y,
                "B" => p.z,
                name => panic!("unexpected channel {name}"),
            })
            .collect();
        assert!(matches!(channel.sample_data, FlatSamples::F32(_)));
        assert_eq!(channel.sample_data.values_as_f32().collect::<Vec<_>>(), expected);
    }
    let chroma = decoded.attributes.chromaticities.unwrap();
    assert_eq!(chroma.red, Vec2(0.64, 0.33));
    assert_eq!(chroma.green, Vec2(0.30, 0.60));
    assert_eq!(chroma.blue, Vec2(0.15, 0.06));
    assert_eq!(chroma.white, Vec2(0.3127, 0.3290));
    assert_eq!(
        layer.attributes.other.get(&Text::from("transferFunction")),
        Some(&AttributeValue::Text(Text::from("linear")))
    );
}

#[test]
fn exr_rejects_invalid_dimensions_pixel_counts_and_nonfinite_channels() {
    let p = [Vec3::new(1.0, 2.0, 3.0)];
    for (width, height) in [(0, 1), (1, 0), (u32::MAX, 1), (1, u32::MAX), (2, 1)] {
        assert!(encode_exr(width, height, &p).is_err());
    }
    assert!(encode_exr(1, 1, &[]).is_err());
    assert!(encode_exr(1, 1, &[p[0], p[0]]).is_err());
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        for point in [Vec3::new(invalid, 0.0, 0.0), Vec3::new(0.0, invalid, 0.0), Vec3::new(0.0, 0.0, invalid)] {
            assert!(encode_exr(1, 1, &[point]).is_err());
        }
    }
}
