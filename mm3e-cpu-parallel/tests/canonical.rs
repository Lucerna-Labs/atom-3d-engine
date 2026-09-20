use mm3e_cpu_parallel::{CpuParallelRenderer, DynSphere};
use mm3e_kit::{
    camera::Camera,
    color::{Material, Rgba},
    csg::Expr,
    framebuffer::Framebuffer,
    vec::{Transform, Vec3},
    volume::SdfVolume,
};
use mm3e_orchestrator::{
    gi::GiVolume, render, render_linear_with_threads, render_with_threads, Light, Object, Prim, RenderMode, Scene,
};
use std::sync::Arc;

fn camera() -> Camera {
    Camera::look_at(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 45.0f32.to_radians())
}
fn scene() -> Scene {
    let mut scene = Scene::new(65, 41);
    scene.aa = 1;
    scene.bounces = 0;
    scene.ambient = Vec3::ZERO;
    scene.sky_ambient = Vec3::ZERO;
    scene.fog_density = 0.0;
    scene.ao = false;
    scene.shadows = false;
    scene.post.bloom = false;
    scene
}
fn pixels(frame: &Framebuffer) -> Vec<u8> {
    frame.to_rgba8(Rgba::rgb8(0, 0, 0))
}
fn parity(scene: &Scene, dynamic: &[DynSphere]) -> Framebuffer {
    let mut reference = scene.clone();
    for sphere in dynamic {
        let material =
            reference.material(Material { albedo: sphere.albedo, metallic: sphere.metallic, ..Material::default() });
        reference.add(Object::new(Prim::Sphere { r: sphere.radius }, Transform::at(sphere.pos), material));
    }
    let camera = camera();
    let expected = render(&reference, &camera);
    for threads in [1, 4] {
        let renderer = CpuParallelRenderer::with_threads(threads);
        let compiled = renderer.compile(Arc::new(scene.clone()));
        assert_eq!(renderer.num_threads(), threads);
        assert_eq!(compiled.num_threads(), threads);
        let actual = compiled.try_render(&camera, dynamic).unwrap();
        assert_eq!(pixels(&actual), pixels(&expected), "canonical parity failed with {threads} workers");
    }
    expected
}

#[test]
fn static_beauty_and_all_debug_modes_match_canonical_pixels() {
    let mut scene = scene();
    scene.aa = 2;
    scene.post.exposure = 1.7;
    scene.post.bloom = true;
    scene.post.bloom_radius = 2;
    let material = scene.material(Material {
        albedo: Vec3::new(0.05, 0.25, 0.8),
        emissive: Vec3::new(0.01, 0.2, 0.7),
        ..Material::default()
    });
    scene.add(Object::new(Prim::Sphere { r: 0.9 }, Transform::IDENTITY, material));
    scene.light(Light::directional(Vec3::new(0.4, 0.7, 1.0), Vec3::splat(3.0)));
    for mode in [
        RenderMode::Beauty,
        RenderMode::Albedo,
        RenderMode::Normal,
        RenderMode::Depth,
        RenderMode::Ao,
        RenderMode::Steps,
    ] {
        scene.mode = mode;
        parity(&scene, &[]);
    }
}

#[test]
fn canonical_lights_and_reflected_geometry_contribute_visible_radiance() {
    let mut scene = scene();
    let material = scene.material(Material::solid(Vec3::ONE));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::IDENTITY, material));
    let unlit = parity(&scene, &[]).pixel(32, 20);
    scene.light(Light::directional(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(2.0)));
    let lit = parity(&scene, &[]).pixel(32, 20);
    assert!(lit.r > unlit.r + 0.3, "the real scene light must illuminate the surface");

    scene.lights.clear();
    scene.materials[material as usize].reflectivity = 1.0;
    let card =
        scene.material(Material { albedo: Vec3::ZERO, emissive: Vec3::new(4.0, 0.0, 0.0), ..Material::default() });
    scene.add(Object::new(Prim::Box { half: Vec3::new(2.0, 2.0, 0.1) }, Transform::at(Vec3::new(0.0, 0.0, 6.0)), card));
    let no_bounce = parity(&scene, &[]).pixel(32, 20);
    scene.bounces = 1;
    let reflected = parity(&scene, &[]).pixel(32, 20);
    assert!(reflected.r > no_bounce.r + 0.7, "the sphere must reflect the emissive card behind the camera");
    assert!(reflected.r > reflected.g + 0.7);
}

#[test]
fn dynamic_spheres_use_their_actual_albedo_and_metallic_materials() {
    let mut scene = scene();
    scene.light(Light::directional(Vec3::new(0.4, 0.2, 1.0), Vec3::splat(1.0)));
    let mut sphere = DynSphere { pos: Vec3::ZERO, radius: 0.9, albedo: Vec3::new(0.9, 0.02, 0.01), metallic: 0.0 };
    let red_frame = parity(&scene, &[sphere]);
    let red = red_frame.pixel(32, 20);
    assert!(red.r > red.g + 0.3);
    sphere.albedo = Vec3::new(0.01, 0.8, 0.02);
    let green_frame = parity(&scene, &[sphere]);
    let green = green_frame.pixel(32, 20);
    assert!(green.g > green.r + 0.3);
    sphere.metallic = 1.0;
    let metal = parity(&scene, &[sphere]);
    assert_ne!(pixels(&green_frame), pixels(&metal), "metallicity must change canonical BRDF shading");
}

fn asset_scene() -> Scene {
    let mut scene = scene();
    let red = scene.material(Material::solid(Vec3::new(0.8, 0.05, 0.05)));
    let blue = scene.material(Material::solid(Vec3::new(0.05, 0.08, 0.8)));
    let dims = 9;
    let low = Vec3::splat(-1.25);
    let cell = Vec3::splat(2.5 / (dims - 1) as f32);
    let mut samples = vec![];
    for z in 0..dims {
        for y in 0..dims {
            for x in 0..dims {
                let p = low + Vec3::new(x as f32 * cell.x, y as f32 * cell.y, z as f32 * cell.z);
                samples.push(p.length() - 0.8);
            }
        }
    }
    let volume = scene.volume(SdfVolume::new((dims, dims, dims), low, cell, samples).unwrap());
    scene.add(Object::new(Prim::Volume { id: volume }, Transform::at(Vec3::new(-1.25, 0.0, 0.0)), red));
    let csg = scene
        .csg(Expr::Subtract {
            a: Box::new(Expr::Sphere { r: 0.85 }),
            b: Box::new(Expr::Capsule { a: Vec3::new(0.0, 0.0, -1.2), b: Vec3::new(0.0, 0.0, 1.2), r: 0.3 }),
        })
        .unwrap();
    scene.add(Object::new(Prim::Csg { id: csg }, Transform::at(Vec3::new(1.25, 0.0, 0.0)), blue));
    // Constant probes deliberately test preservation and GI shading, not a physical bake.
    scene.gi =
        Some(GiVolume::bake(Vec3::splat(-3.0), Vec3::splat(3.0), (2, 2, 2), 1, &|_, _| Vec3::new(0.1, 0.3, 0.1)));
    scene.post.exposure = 1.2;
    scene
}

#[test]
fn dynamic_snapshot_preserves_volumes_csg_gi_and_the_immutable_source() {
    let source = Arc::new(asset_scene());
    let camera = camera();
    let before = pixels(&render(&source, &camera));
    let counts = (source.objects.len(), source.materials.len(), source.volumes.len(), source.csgs.len());
    let original_volume = source.volumes[0].data.clone();
    let original_gi = source.gi.as_ref().unwrap().raw().3.to_vec();
    assert!(source.sample_object(0, Vec3::new(-1.25, 0.0, 0.0)).unwrap().dist < -0.7);
    assert!(source.sample_object(1, Vec3::new(1.25, 0.0, 0.0)).unwrap().dist > 0.2);
    assert!(source.sample_object(1, Vec3::new(1.8, 0.0, 0.0)).unwrap().dist < -0.2);
    let dynamic =
        DynSphere { pos: Vec3::new(0.0, -0.7, 0.5), radius: 0.35, albedo: Vec3::new(0.8, 0.05, 0.7), metallic: 0.0 };
    let with_gi = parity(&source, &[dynamic]);
    let mut without_gi = source.as_ref().clone();
    without_gi.gi = None;
    let without_gi = parity(&without_gi, &[dynamic]);
    assert_ne!(pixels(&with_gi), pixels(&without_gi), "baked GI must affect visible geometry");
    let mut albedo_scene = source.as_ref().clone();
    albedo_scene.mode = RenderMode::Albedo;
    let albedo = parity(&albedo_scene, &[dynamic]);
    let mut red = 0;
    let mut blue = 0;
    let mut magenta = 0;
    for y in 0..albedo.height {
        for x in 0..albedo.width {
            let p = albedo.pixel(x, y);
            red += usize::from(p.r > 0.5 && p.b < 0.1);
            blue += usize::from(p.b > 0.5 && p.r < 0.1);
            magenta += usize::from(p.r > 0.5 && p.b > 0.5);
        }
    }
    assert!(red > 20 && blue > 20 && magenta > 10, "every volume, CSG and dynamic material must remain visible");
    let compiled = CpuParallelRenderer::with_threads(4).compile(Arc::clone(&source));
    assert_eq!(pixels(&compiled.render(&camera, &[dynamic])), pixels(&with_gi));
    assert_eq!(pixels(&compiled.render(&camera, &[])), before);
    assert_eq!(counts, (source.objects.len(), source.materials.len(), source.volumes.len(), source.csgs.len()));
    assert_eq!(source.volumes[0].data, original_volume);
    assert_eq!(source.gi.as_ref().unwrap().raw().3, original_gi.as_slice());
}

#[test]
fn explicit_worker_budgets_preserve_display_and_linear_pixels() {
    let scene = asset_scene();
    let camera = camera();
    assert_eq!(pixels(&render_with_threads(&scene, &camera, 1)), pixels(&render_with_threads(&scene, &camera, 4)));
    assert_eq!(render_linear_with_threads(&scene, &camera, 1), render_linear_with_threads(&scene, &camera, 4));
    let renderer = CpuParallelRenderer::with_threads(0);
    assert_eq!(renderer.num_threads(), 1);
    let compiled = renderer.compile(Arc::new(scene.clone()));
    assert_eq!(compiled.num_threads(), 1);
    assert_eq!(pixels(&compiled.render(&camera, &[])), pixels(&render(&scene, &camera)));
}

#[test]
fn invalid_dynamic_inputs_fail_without_mutating_or_poisoning_source() {
    let source = Arc::new(scene());
    let camera = camera();
    let compiled = CpuParallelRenderer::with_threads(2).compile(Arc::clone(&source));
    let valid = DynSphere { pos: Vec3::ZERO, radius: 0.5, albedo: Vec3::ONE, metallic: 0.0 };
    let invalid = [
        DynSphere { pos: Vec3::new(f32::NAN, 0.0, 0.0), ..valid },
        DynSphere { radius: 0.0, ..valid },
        DynSphere { radius: -0.5, ..valid },
        DynSphere { radius: f32::INFINITY, ..valid },
        DynSphere { albedo: Vec3::new(0.1, f32::NAN, 0.2), ..valid },
        DynSphere { albedo: Vec3::new(1.1, 0.0, 0.0), ..valid },
        DynSphere { metallic: -0.1, ..valid },
    ];
    for sphere in invalid {
        assert!(compiled.try_render(&camera, &[valid, sphere]).is_err());
    }
    assert!(source.objects.is_empty());
    assert_eq!(source.materials.len(), 1);
    assert_eq!(pixels(&compiled.try_render(&camera, &[]).unwrap()), pixels(&render(&source, &camera)));
}
