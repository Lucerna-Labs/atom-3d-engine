use mm3e_kit::{
    camera::Camera,
    color::{Material, Rgba},
    surface::TriangleSurface,
    texture::{CornerUvs, Filter, Sampler, TextureImage, Wrap},
    Mat3, Transform, Vec3,
};
use mm3e_orchestrator::{Light, Object, Prim, RenderMode, Scene};
fn surface() -> TriangleSurface {
    TriangleSurface::new(
        vec![
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(-1.0, 1.0, 0.0),
        ],
        vec![[0, 1, 2], [0, 2, 3]],
        0.01,
    )
    .unwrap()
}
fn uv(scale: f64) -> CornerUvs {
    CornerUvs::new(vec![[0.0, 0.0], [scale, 0.0], [scale, scale], [0.0, scale]], vec![[0, 1, 2], [0, 2, 3]], 2).unwrap()
}
fn color(rgb: [f32; 3]) -> TextureImage {
    TextureImage::from_linear_rgba(1, 1, vec![[rgb[0], rgb[1], rgb[2], 1.0]]).unwrap()
}
fn scene() -> Scene {
    let mut s = Scene::new(32, 32);
    s.ambient = Vec3::ONE;
    s.sky_ambient = Vec3::ZERO;
    s.fog_density = 0.0;
    s.ao = false;
    s.shadows = false;
    s.aa = 1;
    s.bounces = 0;
    s
}
fn add(s: &mut Scene, xf: Transform, material: u32) -> usize {
    let p = Prim::Surface { id: s.surface(surface()).unwrap() };
    let i = s.objects.len();
    s.add(Object::new(p, xf, material));
    i
}
fn camera() -> Camera {
    Camera::look_at(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 40f32.to_radians())
}
fn near(a: Vec3, b: Vec3) {
    assert!((a - b).length() < 2e-6, "{a:?} != {b:?}");
}

#[test]
fn shared_material_ids_keep_independent_object_textures_and_exact_csg_owner_policy() {
    let mut s = scene();
    let white = s.material(Material::solid(Vec3::ONE));
    let a = add(&mut s, Transform::at(Vec3::new(-2.0, 0.0, 0.0)), white);
    let b = add(&mut s, Transform::at(Vec3::new(2.0, 0.0, 0.0)), white);
    s.bind_surface_texture(a, uv(1.0), color([1.0, 0.0, 0.0]), Sampler::default()).unwrap();
    s.bind_surface_texture(b, uv(1.0), color([0.0, 0.0, 1.0]), Sampler::default()).unwrap();
    let left = s.sample_material(Vec3::new(-2.0, 0.0, 0.01), 0.0).unwrap();
    let right = s.sample_material(Vec3::new(2.0, 0.0, 0.01), 0.0).unwrap();
    assert_eq!(left.field.mat, right.field.mat);
    assert_eq!(left.object, Some(a));
    assert_eq!(right.object, Some(b));
    near(left.material.albedo, Vec3::new(1.0, 0.0, 0.0));
    near(right.material.albedo, Vec3::new(0.0, 0.0, 1.0));
    let cutter = s.material(Material::solid(Vec3::new(0.0, 1.0, 0.0)));
    s.add(Object::new(Prim::Sphere { r: 0.2 }, Transform::at(Vec3::new(-2.0, 0.0, 0.0)), cutter).subtract());
    let cut = s.sample_material(Vec3::new(-1.8, 0.0, 0.0), 0.0).unwrap();
    assert_eq!(cut.object, Some(a));
    near(cut.material.albedo, Vec3::new(1.0, 0.0, 0.0));
    for i in 0..80 {
        let p = Vec3::new(i as f32 / 13.0 - 3.0, 0.14, 0.12);
        let a = s.sample_authored(p);
        let b = s.sample_authored_owner(p).0;
        assert_eq!(a.dist.to_bits(), b.dist.to_bits());
        assert_eq!(a.mat, b.mat);
    }
    s.objects.pop();
    s.objects[b].xform = s.objects[a].xform;
    s.objects[b].combine = mm3e_orchestrator::Combine::Smooth(0.1);
    let sample = s.sample_material(Vec3::new(-2.0, 0.0, 0.02), 0.0).unwrap();
    assert_eq!(sample.object, Some(b));
    near(sample.material.albedo, Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn uv_coordinates_follow_placed_and_replaced_posed_surfaces() {
    let mut s = scene();
    let white = s.material(Material::solid(Vec3::ONE));
    let xf = Transform::new(Vec3::new(3.0, 2.0, 1.0), Mat3::from_euler(0.2, 0.6, 0.1), 1.7);
    let index = add(&mut s, xf, white);
    let image = TextureImage::from_linear_rgba(
        2,
        2,
        vec![[1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0], [1.0; 4]],
    )
    .unwrap();
    s.bind_surface_texture(
        index,
        uv(1.0),
        image,
        Sampler { u: Wrap::Clamp, v: Wrap::Clamp, filter: Filter::Nearest, lod_bias: 0.0 },
    )
    .unwrap();
    let point = xf.to_world(Vec3::new(-0.5, -0.5, 0.01));
    let first = s.sample_material(point, 0.0).unwrap();
    near(first.material.albedo, Vec3::new(0.0, 0.0, 1.0));
    let Prim::Surface { id } = s.objects[index].prim else { unreachable!() };
    let vertices = s.surfaces[id as usize].vertices().iter().map(|&p| xf.to_world(p)).collect();
    s.surfaces[id as usize] = TriangleSurface::new(vertices, vec![[0, 1, 2], [0, 2, 3]], 0.017).unwrap();
    s.objects[index].xform = Transform::IDENTITY;
    let posed = s.sample_material(point, 0.0).unwrap();
    near(posed.material.albedo, first.material.albedo);
    for (a, b) in first.uv.unwrap().into_iter().zip(posed.uv.unwrap()) {
        assert!((a - b).abs() < 1e-6);
    }
    let vertices = s.surfaces[id as usize].vertices().to_vec();
    s.surfaces[id as usize] = TriangleSurface::new(vertices, vec![[0, 2, 1], [0, 3, 2]], 0.017).unwrap();
    assert!(mm3e_orchestrator::render_checked(&s, &camera()).err().unwrap().contains("topology"));
}

#[test]
fn automatic_mips_converge_in_beauty_film_and_albedo_and_level_zero_is_a_negative_control() {
    let mut s = scene();
    let white = s.material(Material::solid(Vec3::ONE));
    let index = add(&mut s, Transform::IDENTITY, white);
    let pixels = (0..64 * 64)
        .map(|i| {
            let v = ((i / 64 + i % 64) % 2) as f32;
            [v, v, v, 1.0]
        })
        .collect();
    let image = TextureImage::from_linear_rgba(64, 64, pixels).unwrap();
    s.bind_surface_texture(index, uv(512.0), image.clone(), Sampler::default()).unwrap();
    let linear = mm3e_orchestrator::render_linear_checked(&s, &camera()).unwrap();
    let center = 16 * 32 + 16;
    near(linear[center], Vec3::splat(0.5));
    let film = mm3e_orchestrator::render_film_checked(&s, &camera(), &mm3e_kit::camera::Lens::default()).unwrap();
    near(film.beauty[center], Vec3::splat(0.5));
    s.mode = RenderMode::Albedo;
    let bytes = mm3e_orchestrator::render_checked(&s, &camera()).unwrap().to_rgba8(Rgba::rgb8(0, 0, 0));
    assert_eq!(&bytes[center * 4..center * 4 + 3], &[128; 3]);
    s.bind_surface_texture(index, uv(512.0), image, Sampler { filter: Filter::Bilinear, ..Sampler::default() })
        .unwrap();
    let alias = mm3e_orchestrator::render_linear_checked(&s, &camera()).unwrap();
    let differences = linear.iter().zip(alias).filter(|(a, b)| (**a - *b).length() > 0.04).count();
    assert!(differences > 100, "level-zero control must expose minification aliasing: {differences}");
}

#[test]
fn reflection_and_gi_resolve_the_same_attached_texture() {
    let mut s = scene();
    s.bounces = 1;
    let mirror = s.material(Material::solid(Vec3::ZERO).reflective(1.0));
    s.add(Object::new(Prim::Sphere { r: 0.8 }, Transform::IDENTITY, mirror));
    let white = s.material(Material::solid(Vec3::ONE));
    let card = add(&mut s, Transform::at(Vec3::new(0.0, 0.0, 4.0)).scaled(2.0), white);
    s.bind_surface_texture(card, uv(1.0), color([1.0, 0.0, 0.0]), Sampler::default()).unwrap();
    let red = mm3e_orchestrator::render_linear_checked(&s, &camera()).unwrap()[16 * 32 + 16];
    s.bind_surface_texture(card, uv(1.0), color([0.0, 0.0, 1.0]), Sampler::default()).unwrap();
    let blue = mm3e_orchestrator::render_linear_checked(&s, &camera()).unwrap()[16 * 32 + 16];
    assert!(red.x > 0.9 && blue.z > 0.9 && red.z < 0.01 && blue.x < 0.01, "{red:?} {blue:?}");
    let mut gi = scene();
    gi.light(Light::directional(Vec3::new(0.0, 0.0, 1.0), Vec3::ONE));
    let white = gi.material(Material::solid(Vec3::ONE));
    let card = add(&mut gi, Transform::IDENTITY, white);
    gi.bind_surface_texture(card, uv(1.0), color([1.0, 0.0, 0.0]), Sampler::default()).unwrap();
    gi.bake_gi((5, 5, 5), 5);
    gi.appearance.status().unwrap();
    let red = gi.gi.as_ref().unwrap().sample(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0));
    gi.bind_surface_texture(card, uv(1.0), color([0.0, 0.0, 1.0]), Sampler::default()).unwrap();
    gi.bake_gi((5, 5, 5), 5);
    gi.appearance.status().unwrap();
    let blue = gi.gi.as_ref().unwrap().sample(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0));
    assert!(red.x > blue.x + 0.01 && blue.z > red.z + 0.01, "GI texture contribution not observed: {red:?} {blue:?}");
}

#[test]
fn checked_render_rejects_runtime_attribute_precision_failures_instead_of_delivering_fallback_pixels() {
    let tiny = 1e-38_f32;
    let mut s = Scene::new(1, 1);
    s.shadows = false;
    s.ao = false;
    s.bounces = 0;
    s.aa = 1;
    let surface = TriangleSurface::new(
        vec![Vec3::new(tiny, 0.0, 0.0), Vec3::new(tiny, tiny, 0.0), Vec3::new(tiny, 0.0, tiny)],
        vec![[0, 1, 2]],
        0.01,
    )
    .unwrap();
    let id = s.surface(surface).unwrap();
    let white = s.material(Material::solid(Vec3::ONE));
    s.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, white));
    s.bind_surface_texture(
        0,
        CornerUvs::new(vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], vec![[0, 1, 2]], 1).unwrap(),
        color([1.0, 0.0, 0.0]),
        Sampler::default(),
    )
    .unwrap();
    let eye = Vec3::new(0.1, tiny * 0.25, tiny * 0.25);
    let camera =
        Camera::look_at(eye, Vec3::new(tiny, tiny * 0.25, tiny * 0.25), Vec3::new(0.0, 1.0, 0.0), 40f32.to_radians());
    assert!(mm3e_orchestrator::render_checked(&s, &camera).is_err());
    assert!(s.appearance.status().is_err());
}
