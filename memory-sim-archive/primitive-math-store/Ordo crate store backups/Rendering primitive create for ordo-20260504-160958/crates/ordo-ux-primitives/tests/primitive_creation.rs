use kurbo::{Point, Rect, Size};
use ordo_ux_primitives::{
    Bounds, CursorHint, Fill, HitRegion, ImageFit, ImageRef, Layer, LineCap, LineJoin, Primitive,
    Shape, Stroke, TextAlign, TextRun, ThemeTokens, Transform,
};
use peniko::Color;

#[test]
fn create_fill_primitive() {
    let primitive = Primitive::fill(
        Shape::Rect(Rect::new(0.0, 0.0, 100.0, 48.0)),
        Fill::new(Color::new([0.2, 0.4, 0.8, 1.0])),
    );

    assert!(matches!(primitive, Primitive::Fill { .. }));
}

#[test]
fn create_stroke_primitive() {
    let primitive = Primitive::stroke(
        Shape::Rect(Rect::new(0.0, 0.0, 100.0, 48.0)),
        Stroke::new(Color::BLACK, 2.0),
    );

    match primitive {
        Primitive::Stroke { stroke, .. } => assert_eq!(stroke.width, 2.0),
        _ => panic!("expected stroke primitive"),
    }
}

#[test]
fn create_text_primitive() {
    let text = TextRun::new("Hello Ordo", Point::new(8.0, 16.0));
    let primitive = Primitive::Text(text);

    match primitive {
        Primitive::Text(text) => assert_eq!(text.text, "Hello Ordo"),
        _ => panic!("expected text primitive"),
    }
}

#[test]
fn create_hit_region() {
    let bounds = Bounds::new(Point::ORIGIN, Size::new(120.0, 32.0));
    let mut hit_region = HitRegion::new("primary-button", bounds);
    hit_region.cursor = Some(CursorHint::Pointer);

    assert!(hit_region.enabled);
    assert_eq!(hit_region.cursor, Some(CursorHint::Pointer));
}

#[test]
fn create_default_theme() {
    let theme = ThemeTokens::default();

    assert_eq!(theme.spacing_unit, 8.0);
    assert_eq!(theme.font_family, "System");
}

#[test]
fn infer_shape_and_stroke_bounds() {
    let primitive = Primitive::stroke(
        Shape::rect(10.0, 20.0, 40.0, 30.0),
        Stroke::new(Color::BLACK, 4.0),
    );

    let bounds = primitive.bounds().expect("stroke should have bounds");
    assert_eq!(bounds.rect, Rect::new(8.0, 18.0, 52.0, 52.0));
}

#[test]
fn builder_helpers_attach_metadata() {
    let bounds = Bounds::from_xywh(0.0, 0.0, 64.0, 64.0);
    let hit_region = HitRegion::new("tile", bounds)
        .with_cursor(CursorHint::Pointer)
        .with_shape(Shape::rect(0.0, 0.0, 64.0, 64.0));

    let primitive = Primitive::fill(
        Shape::rounded_rect(0.0, 0.0, 64.0, 64.0, 6.0),
        Fill::new(Color::new([1.0, 1.0, 1.0, 1.0])),
    )
    .with_bounds(bounds)
    .with_hit_region(hit_region)
    .with_transform(Transform::translate(4.0, 8.0));

    let transformed = primitive.bounds().expect("fill should have bounds");
    assert_eq!(transformed.rect, Rect::new(4.0, 8.0, 68.0, 72.0));
}

#[test]
fn stroke_builders_convert_to_kurbo_style() {
    let stroke = Stroke::new(Color::BLACK, 3.0)
        .with_line_cap(LineCap::Round)
        .with_line_join(LineJoin::Bevel)
        .with_dashes(1.0, vec![2.0, 4.0]);

    let kurbo_stroke = stroke.to_kurbo_stroke();
    assert_eq!(kurbo_stroke.width, 3.0);
    assert_eq!(kurbo_stroke.dash_offset, 1.0);
}

#[test]
fn text_image_and_layer_builders_work() {
    let text = TextRun::new("Better primitives", Point::new(12.0, 24.0))
        .with_font("Inter", 16.0)
        .with_align(TextAlign::Center)
        .with_bounds(Bounds::from_xywh(0.0, 0.0, 180.0, 32.0));

    let image = ImageRef::new("hero", Bounds::from_xywh(0.0, 0.0, 320.0, 180.0))
        .with_fit(ImageFit::Contain)
        .with_opacity(0.85);

    let mut layer = Layer::new().named("content").with_opacity(0.9);
    layer.push(Primitive::text(text));
    layer.push(Primitive::image(image));

    assert_eq!(layer.name.as_deref(), Some("content"));
    assert_eq!(layer.primitives.len(), 2);
}

#[test]
fn theme_creates_common_paint_styles() {
    let theme = ThemeTokens::default();

    assert_eq!(theme.border_stroke().width, 1.0);
    assert_eq!(theme.text_fill().opacity, 1.0);
    assert_eq!(theme.accent_fill().opacity, 1.0);
}
