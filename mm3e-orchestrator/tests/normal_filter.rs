use mm3e_kit::{
    shade,
    surface::TriangleSurface,
    texture::{CornerUvs, Sampler, TextureImage},
    Camera, Material, Transform, Vec3,
};
use mm3e_orchestrator::{
    appearance::{NormalMap, SurfaceMaps, TextureMap},
    Light, Object, Prim, Scene,
};
fn scene() -> Scene {
    mapped_scene(None, 1.0, true, Sampler::default())
}
fn mapped_scene(constant: Option<[f32; 4]>, strength: f32, variance_filter: bool, sampler: Sampler) -> Scene {
    let mut s = Scene::new(1, 1);
    s.aa = 1;
    s.shadows = false;
    s.ao = false;
    s.bounces = 0;
    s.ambient = Vec3::ZERO;
    s.sky_ambient = Vec3::ZERO;
    s.fog_density = 0.0;
    let id = s
        .surface(
            TriangleSurface::new(
                vec![Vec3::new(-1.0, -1.0, 0.0), Vec3::new(1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
                vec![[0, 1, 2]],
                0.01,
            )
            .unwrap(),
        )
        .unwrap();
    let mat = s.material(Material { albedo: Vec3::ZERO, roughness: 0.04, specular: 1.0, ..Material::default() });
    s.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, mat));
    s.light(Light::directional(Vec3::new(0.0, 0.0, 1.0), Vec3::ONE));
    let pixels = (0..64 * 64)
        .map(|i| constant.unwrap_or([if (i / 64 + i % 64) % 2 == 0 { 0.8 } else { 0.2 }, 0.5, 0.9, 1.0]))
        .collect();
    let image = TextureImage::from_data_channels(64, 64, pixels).unwrap();
    let uv = CornerUvs::new(vec![[0.0, 0.0], [64.0, 0.0], [32.0, 64.0]], vec![[0, 1, 2]], 1).unwrap();
    s.bind_surface_maps(
        0,
        uv,
        SurfaceMaps {
            normal: Some(NormalMap { map: TextureMap { image, sampler }, strength, flip_y: false, variance_filter }),
            ..SurfaceMaps::default()
        },
    )
    .unwrap();
    s
}
#[test]
fn unresolved_opposing_normals_do_not_invent_a_sharp_frontal_glint() {
    let s = scene();
    let camera = Camera::look_at(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 40f32.to_radians());
    let rendered = mm3e_orchestrator::render_linear_checked(&s, &camera).unwrap()[0].x;
    let reference = [Vec3::new(0.6, 0.0, 0.8), Vec3::new(-0.6, 0.0, 0.8)]
        .into_iter()
        .map(|n| shade::brdf(n, Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Vec3::ZERO, 0.0, 0.04, 1.0).x)
        .sum::<f32>()
        * 0.5;
    assert!((rendered-reference).abs()<0.1,"averaged-normal highlight differs from two-normal source-population reference: rendered={rendered}, reference={reference}");
}

#[test]
fn disable_strength_and_constant_controls_preserve_pixels_and_geometry() {
    let camera = Camera::look_at(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 40f32.to_radians());
    for (constant, strength) in [(None, 0.0), (Some([204.0 / 255.0, 128.0 / 255.0, 230.0 / 255.0, 0.0]), 1.0)] {
        let filtered = mapped_scene(constant, strength, true, Sampler::default());
        let control = mapped_scene(constant, strength, false, Sampler::default());
        assert_eq!(
            mm3e_orchestrator::render_linear_checked(&filtered, &camera).unwrap(),
            mm3e_orchestrator::render_linear_checked(&control, &camera).unwrap()
        );
        let sample = filtered.sample_material(Vec3::new(0.0, 0.0, 0.01), 1.0).unwrap();
        assert_eq!(sample.normal_variance, 0.0);
        assert_eq!(sample.material.roughness, sample.unfiltered_roughness);
    }
    let filtered = scene();
    let control = mapped_scene(None, 1.0, false, Sampler::default());
    let point = Vec3::new(0.0, 0.0, 0.01);
    let a = filtered.sample_material(point, 1.0).unwrap();
    let b = control.sample_material(point, 1.0).unwrap();
    assert_eq!(a.shading_normal, b.shading_normal);
    assert_eq!(a.geometric_normal, b.geometric_normal);
    assert_eq!(a.field.dist, b.field.dist);
    assert_eq!(a.material.albedo, b.material.albedo);
    assert!(a.material.roughness > 0.8);
    assert_eq!(b.material.roughness, 0.04);
}
#[test]
fn normal_strength_changes_variance_quadratically_and_roughness_stays_bounded() {
    let point = Vec3::new(0.0, 0.0, 0.01);
    let base = scene().sample_material(point, 1.0).unwrap();
    for strength in [0.0, 0.25, 0.5, 1.0, 8.0] {
        let sample = mapped_scene(None, strength, true, Sampler::default()).sample_material(point, 1.0).unwrap();
        assert!((sample.normal_variance - base.normal_variance * f64::from(strength).powi(2)).abs() < 1e-9);
        assert!((0.04..=1.0).contains(&sample.material.roughness));
    }
    let nearest =
        mapped_scene(None, 1.0, true, Sampler { filter: mm3e_kit::texture::Filter::Nearest, ..Sampler::default() });
    assert_eq!(nearest.sample_material(point, 1.0).unwrap().normal_variance, 0.0);
}
#[test]
fn moving_light_retains_reference_errors_and_suppresses_false_frontal_peak() {
    let camera = Camera::look_at(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 40f32.to_radians());
    let view = Vec3::new(0.0, 0.0, 1.0);
    let mut old_error = 0.0;
    let mut new_error = 0.0;
    for degrees in [-80.0_f32, -60.0, -40.0, -20.0, 0.0, 20.0, 40.0, 60.0, 80.0] {
        let angle = degrees.to_radians();
        let light = Vec3::new(angle.sin(), 0.0, angle.cos());
        let mut filtered = scene();
        let mut control = mapped_scene(None, 1.0, false, Sampler::default());
        for s in [&mut filtered, &mut control] {
            s.lights.clear();
            s.light(Light::directional(light, Vec3::ONE));
        }
        let a = mm3e_orchestrator::render_linear_checked(&filtered, &camera).unwrap()[0].x;
        let b = mm3e_orchestrator::render_linear_checked(&control, &camera).unwrap()[0].x;
        let reference = [Vec3::new(0.6, 0.0, 0.8), Vec3::new(-0.6, 0.0, 0.8)]
            .into_iter()
            .map(|n| shade::brdf(n, light, view, Vec3::ZERO, 0.0, 0.04, 1.0).x)
            .sum::<f32>()
            * 0.5;
        println!("degrees={degrees} filtered={a} unfiltered={b} source_reference={reference}");
        assert!(a.is_finite() && a >= 0.0);
        old_error += (b - reference).abs();
        new_error += (a - reference).abs();
    }
    assert!(new_error < old_error * 0.001, "filtered={new_error}, unfiltered={old_error}");
}
