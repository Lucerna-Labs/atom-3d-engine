//! Periodic-coordinate contracts tested against the actual multicolor sampler.
//! Pointwise phase equivalence does not certify a whole UV interpolation chart.
use mm3e_editor::delivery_uv::{transfer, TransferOptions};
use mm3e_kit::{
    surface::TriangleSurface,
    texture::{CornerUvs, Filter, Sampler, TextureImage, Wrap},
    Material, Transform, Vec3,
};
use mm3e_orchestrator::{
    appearance::{Channel, ScalarMap, SurfaceMaps, TextureMap},
    Object, Prim, Scene,
};
use serde_json::json;
use std::collections::BTreeMap;

fn pixels() -> Vec<[f32; 4]> {
    vec![
        [1., 0., 0., 1.],
        [0., 1., 0., 0.75],
        [0., 0., 1., 0.5],
        [1., 1., 0., 0.25],
        [0.2, 0.8, 0.4, 0.6],
        [0.8, 0.1, 0.9, 1.],
        [0.25, 0.5, 0.75, 0.8],
        [0.9, 0.3, 0.1, 0.4],
    ]
}
fn image(data: bool) -> TextureImage {
    if data {
        TextureImage::from_data_channels(4, 2, pixels()).unwrap()
    } else {
        TextureImage::from_linear_rgba(4, 2, pixels()).unwrap()
    }
}
fn sampler(u: Wrap, v: Wrap, filter: Filter) -> Sampler {
    Sampler { u, v, filter, lod_bias: 0.375 }
}

#[test]
fn integer_repeat_and_even_mirror_shifts_preserve_actual_color_and_data_samples() {
    for data in [false, true] {
        let image = image(data);
        for filter in [Filter::Nearest, Filter::Bilinear, Filter::Trilinear] {
            for (wrap, period) in [(Wrap::Repeat, 1.), (Wrap::Mirror, 2.)] {
                let sampler = sampler(wrap, wrap, filter);
                for uv in [[0.125, 0.25], [-0.375, -0.5], [999_998.125, -999_998.5], [0., 1.]] {
                    for lod in [0., 0.5, 1.25] {
                        let original = image.sample(uv, lod, sampler).unwrap();
                        for shift in [-2., -1., 1., 2.] {
                            assert_eq!(
                                image.sample([uv[0] + period * shift, uv[1] - period * shift], lod, sampler).unwrap(),
                                original
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn odd_mirror_shifts_and_clamped_axis_shifts_are_not_repeat_equivalences() {
    let image = image(false);
    let uv = [0.125, 0.25];
    for filter in [Filter::Nearest, Filter::Bilinear] {
        let repeat = sampler(Wrap::Repeat, Wrap::Repeat, filter);
        let mirror = sampler(Wrap::Mirror, Wrap::Mirror, filter);
        let clamp = sampler(Wrap::Clamp, Wrap::Clamp, filter);
        for axis in 0..2 {
            let mut odd = uv;
            odd[axis] += 1.0;
            let mut even = uv;
            even[axis] += 2.0;
            assert_eq!(image.sample(uv, 0., repeat).unwrap(), image.sample(odd, 0., repeat).unwrap());
            assert_ne!(image.sample(uv, 0., mirror).unwrap(), image.sample(odd, 0., mirror).unwrap());
            assert_eq!(image.sample(uv, 0., mirror).unwrap(), image.sample(even, 0., mirror).unwrap());
            assert_ne!(image.sample(uv, 0., clamp).unwrap(), image.sample(odd, 0., clamp).unwrap());
        }
    }
}

fn scene(maps: SurfaceMaps, uv: [f64; 2]) -> Scene {
    let mut scene = Scene::new(8, 8);
    let surface = scene
        .surface(
            TriangleSurface::new(vec![Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.)], vec![[0, 1, 2]], 0.05)
                .unwrap(),
        )
        .unwrap();
    let material = scene.material(Material::default());
    scene.add(Object::new(Prim::Surface { id: surface }, Transform::IDENTITY, material));
    scene.bind_surface_maps(0, CornerUvs::new(vec![uv; 3], vec![[0, 1, 2]], 1).unwrap(), maps).unwrap();
    scene
}
fn run(scene: &Scene, tolerance: f64) -> Result<mm3e_editor::delivery_uv::TransferredMesh, String> {
    transfer(
        scene,
        &[Vec3::new(0.1, 0.1, 0.05), Vec3::new(0.6, 0.1, 0.05), Vec3::new(0.1, 0.6, 0.05)],
        &[[0, 1, 2]],
        &BTreeMap::from([(scene.objects[0].mat, 0)]),
        TransferOptions {
            max_uv_error_texels: tolerance,
            max_refinement_passes: 0,
            max_work: 100_000,
            max_vertices: 100,
            max_triangles: 100,
        },
    )
}
fn map(u: Wrap, v: Wrap) -> TextureMap {
    TextureMap { image: image(false), sampler: sampler(u, v, Filter::Bilinear) }
}

#[test]
fn transfer_reports_intersection_of_all_bound_sampler_periods_per_axis() {
    let cases = [
        (Wrap::Repeat, Wrap::Repeat, None, json!([1., 1.])),
        (Wrap::Mirror, Wrap::Repeat, None, json!([2., 1.])),
        (Wrap::Repeat, Wrap::Mirror, None, json!([1., 2.])),
        (Wrap::Mirror, Wrap::Mirror, Some((Wrap::Clamp, Wrap::Repeat)), json!([null, 2.])),
        (Wrap::Repeat, Wrap::Repeat, Some((Wrap::Repeat, Wrap::Clamp)), json!([1., null])),
    ];
    for (u, v, scalar, wanted) in cases {
        let maps = SurfaceMaps {
            albedo: Some(map(Wrap::Repeat, Wrap::Repeat)),
            emissive: Some(map(u, v)),
            roughness: scalar.map(|(u, v)| ScalarMap {
                map: TextureMap { image: image(true), sampler: sampler(u, v, Filter::Trilinear) },
                channel: Channel::G,
            }),
            ..SurfaceMaps::default()
        };
        let out = run(&scene(maps, [0.125, 0.375]), 0.01).unwrap();
        assert_eq!(out.report["sampling_periods"][0]["periods"], wanted);
        assert_eq!(out.corner_uvs, vec![[[0.125, 0.375]; 3]]);
    }
}

#[test]
fn f32_storage_must_preserve_phase_within_requested_texel_error() {
    let maps = || SurfaceMaps { albedo: Some(map(Wrap::Repeat, Wrap::Repeat)), ..SurfaceMaps::default() };
    let exact = scene(maps(), [999_999.875, 0.375]);
    let out = run(&exact, 0.001).unwrap();
    assert_eq!(out.corner_uvs[0][0], [999_999.875, 0.375]);
    let lost = scene(maps(), [999_999.97, 0.375]);
    assert!(run(&lost, 0.001).err().unwrap().contains("represented in f32"));
    let sampler = sampler(Wrap::Repeat, Wrap::Repeat, Filter::Bilinear);
    let source = image(false).sample([999_999.97, 0.375], 0., sampler).unwrap();
    let stored = image(false).sample([f64::from(999_999.97_f64 as f32), 0.375], 0., sampler).unwrap();
    assert_ne!(source, stored, "large coordinate casts can lose real texture phase");
}

#[test]
fn agreeing_periodic_probe_samples_do_not_authorize_a_cross_seam_interpolation() {
    let image = image(false);
    let sampler = sampler(Wrap::Repeat, Wrap::Clamp, Filter::Bilinear);
    let expected = image.sample([0.125, 0.5], 0., sampler).unwrap();
    // These are the finite transfer probe weights on the second triangle corner.
    // A 3000-period lift aliases every one of them but is not a constant lift.
    for weight in [0.0_f64, 1., 1. / 3., 0.6, 0.2, 0.499, 0.002, 0.5] {
        let u = 0.125 + 3000.0 * weight;
        let delta = (u - 0.125).rem_euclid(1.0_f64);
        assert!(delta.min(1. - delta) < 1e-9);
        let actual = image.sample([u, 0.5], 0., sampler).unwrap();
        assert!(actual.into_iter().zip(expected).all(|(a, b)| (a - b).abs() < 1e-6));
    }
    let between = image.sample([0.125 + 3000.0 * 0.1235, 0.5], 0., sampler).unwrap();
    assert!(between.into_iter().zip(expected).any(|(a, b)| (a - b).abs() > 0.4));
}
