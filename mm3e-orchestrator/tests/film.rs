use mm3e_kit::{
    camera::Camera,
    color::Material,
    vec::{Transform, Vec3},
};
use mm3e_orchestrator::{render_film_with_threads, render_linear_with_threads, Lens, Light, Object, Prim, Scene};

fn camera() -> Camera {
    Camera::look_at(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 0.8)
}

fn emissive_scene(width: u32, height: u32, emission: Vec3) -> Scene {
    let mut scene = Scene::new(width, height);
    scene.aa = 4;
    scene.ao = false;
    scene.shadows = false;
    scene.bounces = 0;
    scene.fog_density = 0.0;
    scene.ambient = Vec3::ZERO;
    scene.sky_ambient = Vec3::ZERO;
    scene.marcher.eps = 0.00001;
    scene.marcher.max_steps = 512;
    let material = scene.material(Material { albedo: Vec3::ZERO, emissive: emission, ..Material::default() });
    scene.objects.push(Object::new(Prim::Sphere { r: 0.55 }, Transform::IDENTITY, material));
    scene
}

#[test]
fn pinhole_beauty_is_bitwise_canonical_with_lighting_fog_reflections_and_aa() {
    let mut scene = Scene::new(23, 19);
    scene.aa = 3;
    scene.bounces = 2;
    let material = scene.material(Material { reflectivity: 0.7, ..Material::solid(Vec3::new(0.9, 0.1, 0.06)) });
    scene.objects.push(Object::new(Prim::Sphere { r: 0.65 }, Transform::IDENTITY, material));
    scene.lights.push(Light::directional(Vec3::new(-1.0, 2.0, 2.0), Vec3::splat(3.0)));
    let expected = render_linear_with_threads(&scene, &camera(), 2);
    let one = render_film_with_threads(&scene, &camera(), &Lens::default(), 1);
    let many = render_film_with_threads(&scene, &camera(), &Lens::default(), 5);
    assert_eq!(one.beauty, expected);
    assert_eq!(one, many);
}

#[test]
fn premultiplied_foreground_composites_over_independent_physical_background() {
    let emission = Vec3::new(4.0, 0.6, 0.2);
    let scene = emissive_scene(37, 29, emission);
    let lens = Lens { aperture_radius: 0.18, focus_distance: 6.0 };
    let film = render_film_with_threads(&scene, &camera(), &lens, 3);
    assert!(film.alpha.iter().any(|&a| a > 0.0 && a < 1.0), "must exercise silhouette sample coverage");
    assert!(film.alpha.contains(&0.0));
    assert!(film.alpha.contains(&1.0));
    let background = Vec3::new(0.02, 0.08, 0.3);
    let mut reference = scene.clone();
    let material = reference.material(Material { albedo: Vec3::ZERO, emissive: background, ..Material::default() });
    reference.objects.push(Object::new(
        Prim::Plane { n: Vec3::new(0.0, 0.0, 1.0), h: 3.0 },
        Transform::IDENTITY,
        material,
    ));
    let physical = render_film_with_threads(&reference, &camera(), &lens, 1);
    for index in 0..film.alpha.len() {
        let a = film.alpha[index];
        assert!((film.foreground[index] - emission.scale(a)).length() < 0.000001);
        let composited = film.foreground[index] + background.scale(1.0 - a);
        assert!((composited - physical.beauty[index]).length() < 0.000003, "pixel {index}");
        if a == 0.0 {
            assert_eq!(film.foreground[index], Vec3::ZERO);
        }
    }
}

#[test]
fn black_geometry_is_opaque_and_primary_sky_is_uncovered() {
    let scene = emissive_scene(13, 13, Vec3::ZERO);
    let film = render_film_with_threads(&scene, &camera(), &Lens::default(), 2);
    let center = 6 * 13 + 6;
    assert_eq!(film.alpha[center], 1.0);
    assert_eq!(film.foreground[center], Vec3::ZERO);
    assert_eq!(film.beauty[center], Vec3::ZERO);
    assert_eq!(film.alpha[0], 0.0);
    assert!(film.beauty[0].length() > 0.1);
    assert_eq!(film.depth[0], f32::INFINITY);
    assert_eq!(film.normals[0], Vec3::ZERO);
    assert_eq!(film.material_ids[0], u32::MAX);
}

#[test]
fn geometry_passes_are_raw_center_pinhole_depth_world_normal_and_material() {
    let mut scene = emissive_scene(3, 3, Vec3::ONE);
    scene.objects.clear();
    scene.objects.push(Object::new(Prim::Plane { n: Vec3::new(0.0, 0.0, 1.0), h: 0.0 }, Transform::IDENTITY, 1));
    let lens = Lens { aperture_radius: 0.9, focus_distance: 1.0 };
    let film = render_film_with_threads(&scene, &camera(), &lens, 2);
    let ray = camera().ray(0.5, 0.5, 3, 3);
    let ray_distance = 3.0 / -ray.dir.z;
    assert!(ray_distance > 3.1);
    for index in 0..9 {
        assert!((film.depth[index] - 3.0).abs() < 0.0001, "pixel {index}: {}", film.depth[index]);
        assert!((film.normals[index] - Vec3::new(0.0, 0.0, 1.0)).length() < 0.00001);
        assert_eq!(film.material_ids[index], 1);
    }
    let center = emissive_scene(3, 3, Vec3::ONE);
    let sphere = render_film_with_threads(&center, &camera(), &lens, 1);
    assert!((sphere.depth[4] - 2.45).abs() < 0.0001);
    assert!((sphere.normals[4] - Vec3::new(0.0, 0.0, 1.0)).length() < 0.00001);
}

#[test]
fn lens_rays_converge_on_camera_forward_focus_plane_even_off_axis() {
    let camera = Camera::look_at(Vec3::new(2.0, 3.0, 4.0), Vec3::new(-1.0, 0.3, 1.0), Vec3::new(0.0, 1.0, 0.0), 1.1);
    let lens = Lens { aperture_radius: 0.2, focus_distance: 5.0 };
    for image in [[100.5, 20.5], [31.0, 97.5], [80.0, 60.0]] {
        let pinhole = camera.ray(image[0], image[1], 160, 120);
        let expected = pinhole.at(lens.focus_distance / pinhole.dir.dot(camera.forward));
        for aperture in [[0.0, 0.0], [0.5, 0.5], [0.2, 0.7], [1.0, 0.8], [0.5, 1.0]] {
            let ray = camera.ray_with_lens(image, [160, 120], &lens, aperture);
            let offset = ray.origin - camera.eye;
            assert!(offset.dot(camera.forward).abs() < 0.000001);
            assert!(offset.length() <= lens.aperture_radius + 0.000001);
            let depth = (ray.origin - camera.eye).dot(camera.forward);
            let focus = ray.at((lens.focus_distance - depth) / ray.dir.dot(camera.forward));
            assert!((focus - expected).length() < 0.000003);
            let zero = camera.ray_with_lens(image, [160, 120], &Lens::default(), aperture);
            assert_eq!(zero.origin, pinhole.origin);
            assert_eq!(zero.dir, pinhole.dir);
        }
    }
}

#[test]
fn aperture_changes_defocused_coverage_deterministically_but_not_geometry_aovs() {
    let mut scene = emissive_scene(31, 25, Vec3::ONE);
    scene.aa = 6;
    let pinhole = render_film_with_threads(&scene, &camera(), &Lens::default(), 1);
    let lens = Lens { aperture_radius: 0.3, focus_distance: 7.0 };
    let aperture = render_film_with_threads(&scene, &camera(), &lens, 1);
    assert_eq!(aperture, render_film_with_threads(&scene, &camera(), &lens, 8));
    assert_ne!(pinhole.alpha, aperture.alpha);
    assert_eq!(pinhole.depth, aperture.depth);
    assert_eq!(pinhole.normals, aperture.normals);
    assert_eq!(pinhole.material_ids, aperture.material_ids);
    let pinhole_partial = pinhole.alpha.iter().filter(|&&a| a > 0.0 && a < 1.0).count();
    let aperture_partial = aperture.alpha.iter().filter(|&&a| a > 0.0 && a < 1.0).count();
    assert!(aperture_partial > pinhole_partial, "defocus must spread opaque coverage at the silhouette");
}

#[test]
fn lens_validation_and_empty_film_dimensions_are_explicit() {
    assert!(Lens::default().validate().is_ok());
    for radius in [-0.1, f32::NAN, f32::INFINITY] {
        assert!(Lens { aperture_radius: radius, focus_distance: 1.0 }.validate().is_err());
    }
    for distance in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(Lens { aperture_radius: 0.1, focus_distance: distance }.validate().is_err());
    }
    for (width, height) in [(0, 5), (5, 0), (0, 0)] {
        let frame = render_film_with_threads(&Scene::new(width, height), &camera(), &Lens::default(), 0);
        assert_eq!((frame.width, frame.height), (width, height));
        assert!(frame.beauty.is_empty() && frame.alpha.is_empty() && frame.depth.is_empty());
    }
}
