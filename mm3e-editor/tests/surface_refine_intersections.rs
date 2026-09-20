//! Regression for projection of distinct closed components onto one native shell.
use mm3e_editor::surface_refine::{refine, Options};
use mm3e_kit::{
    meshing::{extract_isosurface, Mesh},
    surface::TriangleSurface,
    Material, Transform, Vec3,
};
use mm3e_orchestrator::{Object, Prim, Scene};
use std::collections::{BTreeMap, BTreeSet};

const R: f32 = 1.0 / 1024.0;

fn scene() -> Scene {
    let mut scene = Scene::new(8, 8);
    let source = TriangleSurface::new(
        vec![Vec3::ZERO, Vec3::new(R * 0.1, 0., 0.), Vec3::new(0., R * 0.1, 0.)],
        vec![[0, 1, 2]],
        R,
    )
    .unwrap();
    let id = scene.surface(source).unwrap();
    let material = scene.material(Material::default());
    scene.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, material));
    scene
}

fn options() -> Options {
    Options {
        max_residual: R * 0.005,
        normal_step_m: f64::from(R) * 0.05,
        max_work: 8_000_000,
        max_vertices: 60_000,
        max_triangles: 120_000,
        max_passes: 6,
    }
}

fn shells(scene: &Scene, radii: &[f32]) -> Mesh {
    // Only obtain extraction metadata here. The complete input geometry is the
    // explicit octahedral complex below, not the scalar-grid mesh.
    let mut mesh =
        extract_isosurface(|p| scene.sample_authored(p).dist, Vec3::splat(-R * 4.), Vec3::splat(R * 4.), [8; 3])
            .unwrap();
    mesh.positions.clear();
    mesh.triangles.clear();
    let axes = [
        Vec3::new(1., 0., 0.),
        Vec3::new(-1., 0., 0.),
        Vec3::new(0., 1., 0.),
        Vec3::new(0., -1., 0.),
        Vec3::new(0., 0., 1.),
        Vec3::new(0., 0., -1.),
    ];
    let faces = [[0, 2, 4], [2, 1, 4], [1, 3, 4], [3, 0, 4], [2, 0, 5], [1, 2, 5], [3, 1, 5], [0, 3, 5]];
    for &radius in radii {
        let base = mesh.positions.len() as u32;
        mesh.positions.extend(axes.map(|p| p * radius));
        mesh.triangles.extend(faces.map(|triangle| triangle.map(|i| i + base)));
    }
    mesh.metadata.connected_components = radii.len();
    mesh
}

#[test]
fn disjoint_closed_shells_cannot_be_accepted_as_coincident_projected_components() {
    let scene = scene();
    let source_vertices = scene.surfaces[0].vertices().to_vec();
    let mesh = shells(&scene, &[R * 2., R * 3.]);
    assert_eq!((mesh.positions.len(), mesh.triangles.len()), (12, 16));

    // Every face lies in one coordinate octant at its shell's constant L1
    // radius. The two input surfaces are strictly separated by R in that norm.
    for (component, triangles) in mesh.triangles.as_chunks::<8>().0.iter().enumerate() {
        let radius = R * (component as f32 + 2.);
        for triangle in triangles {
            let points = triangle.map(|i| mesh.positions[i as usize]);
            for point in points {
                assert_eq!(point.x.abs() + point.y.abs() + point.z.abs(), radius);
            }
            for axis in 0..3 {
                let values = points.map(|p| [p.x, p.y, p.z][axis]);
                assert!(values.iter().all(|&v| v >= 0.) || values.iter().all(|&v| v <= 0.));
            }
        }
    }

    let error = refine(&scene, mesh, options()).err().expect("coincident projected shells were accepted");
    assert!(error.contains("intersect or overlap beyond their shared indexed boundary"), "{error}");
    assert_eq!(scene.surfaces[0].vertices(), source_vertices);
}

#[test]
fn one_shell_still_refines_to_the_native_triangle_offset_with_replayable_work() {
    let scene = scene();
    let input = shells(&scene, &[R * 2.]);
    let output = refine(&scene, input.clone(), options()).unwrap();
    assert_eq!(output.report["connected_components"], 1);
    assert_eq!(output.report["embedding_validation"]["intersection_free"], true);
    assert_eq!(output.report["embedding_validation"]["method"], "degree_one_radial_projection");
    let radial = &output.report["embedding_validation"]["radial_certificate"];
    assert_eq!(radial["forward_face_hits"], 1);
    assert_eq!(radial["work"]["work"], output.report["embedding_validation"]["work"]);
    assert_eq!(output.report["embedding_validation"]["pair_validation_work"], 0);
    let mut edges = BTreeMap::<(u32, u32), (usize, i32)>::new();
    let mut geometric_faces = BTreeSet::new();
    for triangle in &output.mesh.triangles {
        for (a, b) in [(triangle[0], triangle[1]), (triangle[1], triangle[2]), (triangle[2], triangle[0])] {
            let entry = edges.entry((a.min(b), a.max(b))).or_default();
            entry.0 += 1;
            entry.1 += if a < b { 1 } else { -1 };
        }
        let points = triangle.map(|i| output.mesh.positions[i as usize]);
        let mut key = points.map(|p| [p.x, p.y, p.z].map(|v| if v == 0. { 0 } else { v.to_bits() }));
        key.sort_unstable();
        assert!(geometric_faces.insert(key), "projection duplicated a geometric face");
        for weights in
            [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [0.5, 0.5, 0.], [0., 0.5, 0.5], [0.5, 0., 0.5], [1. / 3.; 3]]
        {
            let p: [f64; 3] = std::array::from_fn(|axis| {
                (0..3).map(|i| f64::from([points[i].x, points[i].y, points[i].z][axis]) * weights[i]).sum()
            });
            let query = Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32);
            assert!(scene.sample_authored(query).dist.abs() <= options().max_residual);
        }
    }
    assert!(edges.values().all(|&edge| edge == (2, 0)));
    assert_eq!(output.mesh.positions.len() + output.mesh.triangles.len(), edges.len() + 2);
    let work = output.report["charged_work"].as_u64().unwrap() as usize;
    assert!(work <= options().max_work);
    let replay = refine(&scene, input.clone(), Options { max_work: work, ..options() }).unwrap();
    assert_eq!(replay.mesh, output.mesh);
    assert_eq!(replay.face_source_indices, output.face_source_indices);
    let error = refine(&scene, input, Options { max_work: work - 1, ..options() }).err().unwrap();
    assert!(error.contains("work") && error.contains("budget"), "{error}");
}
