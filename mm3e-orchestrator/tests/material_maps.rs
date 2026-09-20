use mm3e_kit::{
    surface::TriangleSurface,
    texture::{CornerUvs, Sampler, TextureImage},
    Camera, Mat3, Material, Transform, Vec3,
};
use mm3e_orchestrator::{
    appearance::{Channel, NormalMap, ScalarMap, SurfaceMaps, TextureMap},
    Light, Object, Prim, Scene,
};
fn uv() -> CornerUvs {
    CornerUvs::new(vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]], vec![[0, 1, 2]], 1).unwrap()
}
fn map(p: [f32; 4], data: bool) -> TextureMap {
    TextureMap {
        image: if data {
            TextureImage::from_data_channels(1, 1, vec![p]).unwrap()
        } else {
            TextureImage::from_linear_rgba(1, 1, vec![p]).unwrap()
        },
        sampler: Sampler::default(),
    }
}
fn scene() -> Scene {
    let mut s = Scene::new(32, 32);
    s.aa = 1;
    s.ambient = Vec3::ZERO;
    s.sky_ambient = Vec3::ZERO;
    s.fog_density = 0.0;
    s.shadows = false;
    s.ao = false;
    s.bounces = 0;
    let id = s
        .surface(
            TriangleSurface::new(
                vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
                vec![[0, 1, 2]],
                0.01,
            )
            .unwrap(),
        )
        .unwrap();
    let material = s.material(Material {
        albedo: Vec3::splat(0.4),
        roughness: 0.8,
        metallic: 0.7,
        emissive: Vec3::new(4.0, 8.0, 16.0),
        ..Material::default()
    });
    s.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, material));
    s
}
fn camera() -> Camera {
    Camera::look_at(Vec3::new(0.3, 0.3, 2.0), Vec3::new(0.3, 0.3, 0.0), Vec3::new(0.0, 1.0, 0.0), 30f32.to_radians())
}
fn normal(strength: f32) -> NormalMap {
    NormalMap { map: map([0.8, 0.5, 0.9, 0.0], true), strength, flip_y: false, variance_filter: true }
}
fn near(a: Vec3, b: Vec3) {
    assert!((a - b).length() < 2e-5, "{a:?} != {b:?}");
}
#[test]
fn independent_data_channels_and_emission_factors_reach_actual_material_and_hdr_render() {
    let mut s = scene();
    let point = Vec3::new(0.2, 0.2, 0.01);
    let before = s.sample_authored(point);
    s.bind_surface_maps(
        0,
        uv(),
        SurfaceMaps {
            roughness: Some(ScalarMap { map: map([0.25, 0.5, 0.75, 0.1], true), channel: Channel::R }),
            metallic: Some(ScalarMap { map: map([0.25, 0.5, 0.75, 0.1], true), channel: Channel::A }),
            emissive: Some(map([0.5, 0.25, 1.0, 0.5], false)),
            normal: Some(normal(1.0)),
            ..SurfaceMaps::default()
        },
    )
    .unwrap();
    let sampled = s.sample_material(point, 0.0).unwrap();
    assert!((sampled.material.roughness - 0.2).abs() < 1e-6);
    assert!((sampled.material.metallic - 0.07).abs() < 1e-6);
    near(sampled.material.emissive, Vec3::new(1.0, 1.0, 8.0));
    near(sampled.shading_normal.unwrap(), Vec3::new(0.6, 0.0, 0.8));
    assert_eq!(before.dist.to_bits(), s.sample_authored(point).dist.to_bits());
    let image = mm3e_orchestrator::render_linear_checked(&s, &camera()).unwrap();
    near(image[16 * 32 + 16], Vec3::new(1.0, 1.0, 8.0));
}
#[test]
fn mapped_normals_change_lighting_without_changing_geometry_or_raw_aovs() {
    let mut s = scene();
    s.materials[1].emissive = Vec3::ZERO;
    s.materials[1].metallic = 0.0;
    s.light(Light::directional(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(3.0)));
    let baseline = mm3e_orchestrator::render_film_checked(&s, &camera(), &Default::default()).unwrap();
    s.bind_surface_maps(0, uv(), SurfaceMaps { normal: Some(normal(1.0)), ..SurfaceMaps::default() }).unwrap();
    let mapped = mm3e_orchestrator::render_film_checked(&s, &camera(), &Default::default()).unwrap();
    assert_eq!(baseline.depth, mapped.depth);
    assert_eq!(baseline.normals, mapped.normals);
    assert_eq!(baseline.alpha, mapped.alpha);
    assert_eq!(baseline.material_ids, mapped.material_ids);
    assert!((baseline.beauty[16 * 32 + 16] - mapped.beauty[16 * 32 + 16]).length() > 0.05);
    s.bind_surface_maps(0, uv(), SurfaceMaps { normal: Some(normal(0.0)), ..SurfaceMaps::default() }).unwrap();
    let neutral = mm3e_orchestrator::render_film_checked(&s, &camera(), &Default::default()).unwrap();
    assert_eq!(baseline, neutral);
}
#[test]
fn shading_normal_cannot_light_the_back_of_opaque_geometry() {
    let mut s = scene();
    s.materials[1].emissive = Vec3::ZERO;
    s.materials[1].metallic = 0.0;
    s.light(Light::directional(Vec3::new(0.995, 0.0, -0.1).normalize(), Vec3::splat(10.0)));
    s.bind_surface_maps(
        0,
        uv(),
        SurfaceMaps {
            normal: Some(NormalMap {
                map: map([0.995, 0.5, 0.55, 1.0], true),
                strength: 1.0,
                flip_y: false,
                variance_filter: true,
            }),
            ..SurfaceMaps::default()
        },
    )
    .unwrap();
    let sampled = s.sample_material(Vec3::new(0.2, 0.2, 0.01), 0.0).unwrap();
    assert!(sampled.shading_normal.unwrap().dot(Vec3::new(0.995, 0.0, -0.1)) > 0.9);
    near(mm3e_orchestrator::render_linear_checked(&s, &camera()).unwrap()[16 * 32 + 16], Vec3::ZERO);
}
#[test]
fn actual_placement_mirroring_and_posed_triangle_changes_update_tangent_directions() {
    let mut s = scene();
    s.bind_surface_maps(0, uv(), SurfaceMaps { normal: Some(normal(1.0)), ..SurfaceMaps::default() }).unwrap();
    s.objects[0].mods.mirror[0] = true;
    let sample = s.sample_material(Vec3::new(-0.2, 0.2, 0.01), 0.0).unwrap();
    near(sample.shading_normal.unwrap(), Vec3::new(-0.6, 0.0, 0.8));
    s.objects[0].mods.mirror[0] = false;
    s.objects[0].xform =
        Transform::new(Vec3::new(2.0, 1.0, 0.0), Mat3::from_euler(0.0, 0.0, std::f32::consts::FRAC_PI_2), 2.0);
    let point = s.objects[0].xform.to_world(Vec3::new(0.2, 0.2, 0.01));
    near(s.sample_material(point, 0.0).unwrap().shading_normal.unwrap(), Vec3::new(0.0, 0.6, 0.8));
    let posed = vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0)];
    s.surfaces[0] = TriangleSurface::new(posed, vec![[0, 1, 2]], 0.01).unwrap();
    s.objects[0].xform = Transform::IDENTITY;
    let n = Vec3::new(-1.0, 0.0, 1.0).normalize();
    let t = Vec3::new(1.0, 0.0, 1.0).normalize();
    let p = Vec3::new(0.2, 0.2, 0.2) + n * 0.01;
    near(s.sample_material(p, 0.0).unwrap().shading_normal.unwrap(), t * 0.6 + n * 0.8);
}
#[test]
fn invalid_role_and_uv_inputs_leave_the_previous_binding_intact() {
    let mut s = scene();
    s.bind_surface_texture(
        0,
        uv(),
        TextureImage::from_linear_rgba(1, 1, vec![[1.0, 0.0, 0.0, 1.0]]).unwrap(),
        Sampler::default(),
    )
    .unwrap();
    let old = s.sample_material(Vec3::new(0.2, 0.2, 0.01), 0.0).unwrap().material.albedo;
    assert!(s
        .bind_surface_maps(
            0,
            uv(),
            SurfaceMaps {
                roughness: Some(ScalarMap { map: map([0.5; 4], false), channel: Channel::R }),
                ..SurfaceMaps::default()
            }
        )
        .is_err());
    assert!(s
        .bind_surface_maps(
            0,
            CornerUvs::new(vec![[0.0, 0.0]], vec![[0; 3]], 1).unwrap(),
            SurfaceMaps { normal: Some(normal(1.0)), ..SurfaceMaps::default() }
        )
        .is_err());
    assert!(s
        .bind_surface_maps(
            0,
            uv(),
            SurfaceMaps {
                normal: Some(NormalMap {
                    map: map([0.5, 0.5, 0.1, 1.0], true),
                    strength: 1.0,
                    flip_y: false,
                    variance_filter: true
                }),
                ..SurfaceMaps::default()
            }
        )
        .is_err());
    near(s.sample_material(Vec3::new(0.2, 0.2, 0.01), 0.0).unwrap().material.albedo, old);
}

#[test]
fn mapped_metalness_changes_gathered_light_instead_of_being_ignored_by_gi() {
    let mut s = scene();
    s.materials[1].emissive = Vec3::ZERO;
    s.materials[1].metallic = 1.0;
    s.light(Light::directional(Vec3::new(0.0, 0.0, 1.0), Vec3::splat(4.0)));
    let binding = |value| SurfaceMaps {
        metallic: Some(ScalarMap { map: map([value, 0.0, 0.0, 1.0], true), channel: Channel::R }),
        ..SurfaceMaps::default()
    };
    s.bind_surface_maps(0, uv(), binding(0.0)).unwrap();
    s.bake_gi_checked((5, 5, 5), 5).unwrap();
    let dielectric = s.gi.as_ref().unwrap().sample(Vec3::new(0.25, 0.25, 0.4), Vec3::new(0.0, 0.0, -1.0));
    s.bind_surface_maps(0, uv(), binding(1.0)).unwrap();
    s.bake_gi_checked((5, 5, 5), 5).unwrap();
    let metallic = s.gi.as_ref().unwrap().sample(Vec3::new(0.25, 0.25, 0.4), Vec3::new(0.0, 0.0, -1.0));
    assert!((dielectric - metallic).length() > 0.001, "GI ignored mapped metallic factor: {dielectric:?} {metallic:?}");
}
