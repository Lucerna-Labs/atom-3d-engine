use mm3e_editor::{
    delivery_uv::{transfer, TransferOptions},
    surface_refine::{refine, Options},
};
use mm3e_kit::{
    meshing::{extract_isosurface, Mesh},
    meshing_local::{
        extract_local_convex_union_isosurface_with_representation, ConvexRepresentationPolicy, LocalExtractionOptions,
    },
    surface::TriangleSurface,
    surface_features::SurfaceFeatures,
    texture::{CornerUvs, Sampler, TextureImage},
    Material, Transform, Vec3,
};
use mm3e_orchestrator::{
    appearance::{SurfaceMaps, TextureMap},
    Object, Prim, Scene,
};
use std::collections::BTreeMap;

const R: f32 = 0.1;
fn options() -> Options {
    Options {
        max_residual: R * 0.02,
        normal_step_m: f64::from(R) * 0.01,
        max_work: 8_000_000,
        max_vertices: 60_000,
        max_triangles: 120_000,
        max_passes: 6,
    }
}
fn scene(vertices: Vec<Vec3>, triangles: Vec<[u32; 3]>) -> Scene {
    let mut scene = Scene::new(8, 8);
    let id = scene.surface(TriangleSurface::new(vertices, triangles, R).unwrap()).unwrap();
    let material = scene.material(Material::default());
    scene.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, material));
    scene
}
fn flat() -> Scene {
    scene(vec![Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.)], vec![[0, 1, 2]])
}
fn scalar_sampled_prism() -> Mesh {
    extract_isosurface(
        |p| (-p.x).max(-p.y).max((p.x + p.y - 1.) / 2.0_f32.sqrt()).max(p.z.abs() - R),
        Vec3::new(-0.3, -0.3, -0.3),
        Vec3::new(1.3, 1.3, 0.3),
        [8, 8, 8],
    )
    .unwrap()
}
fn prism() -> Mesh {
    let mut mesh = scalar_sampled_prism();
    // Use the exact closed prism, not a scalar-grid approximation of its max
    // expression, which can introduce unrelated microscopic corner slivers.
    mesh.positions = vec![
        Vec3::new(0., 0., -R),
        Vec3::new(1., 0., -R),
        Vec3::new(0., 1., -R),
        Vec3::new(0., 0., R),
        Vec3::new(1., 0., R),
        Vec3::new(0., 1., R),
    ];
    mesh.triangles = vec![[0, 2, 1], [3, 4, 5], [0, 1, 4], [0, 4, 3], [1, 2, 5], [1, 5, 4], [2, 0, 3], [2, 3, 5]];
    mesh.metadata.surface_area = 1.0 + (2.0 + 2.0_f64.sqrt()) * f64::from(2.0 * R);
    mesh.metadata.signed_volume = f64::from(R);
    mesh
}

#[test]
fn scalar_sampled_prism_with_irregular_fans_is_not_accepted_with_missing_caps() {
    let scene = flat();
    let mesh = scalar_sampled_prism();
    let before = scene.sample_authored(Vec3::new(-0.05, -0.05, 0.)).dist.to_bits();
    let error = refine(&scene, mesh, options()).err().expect("irregular scalar-grid fan was falsely accepted");
    // The captured collapsed-child failure is repaired. This unchanged legacy
    // input now reaches the same fixed aggregate work cap; no mesh is certified.
    assert_budget_rejection(&error);
    assert_eq!(scene.sample_authored(Vec3::new(-0.05, -0.05, 0.)).dist.to_bits(), before);
}
fn assert_budget_rejection(error: &str) {
    assert!(
        [
            "native surface refinement exhausted work budget",
            "counted field work budget exhausted before query",
            "counted field work budget exhausted before operation",
        ]
        .iter()
        .any(|prefix| error.starts_with(prefix)),
        "expected the measured aggregate-work rejection, got {error}"
    );
}
fn residual(scene: &Scene, mesh: &Mesh) -> f32 {
    let mut maximum = 0.0_f32;
    for &point in &mesh.positions {
        maximum = maximum.max(scene.sample_authored(point).dist.abs());
    }
    for triangle in &mesh.triangles {
        let [a, b, c] = triangle.map(|i| mesh.positions[i as usize]);
        for point in [(a + b) * 0.5, (b + c) * 0.5, (c + a) * 0.5, (a + b + c) * (1. / 3.)] {
            maximum = maximum.max(scene.sample_authored(point).dist.abs());
        }
    }
    maximum
}
fn closed(mesh: &Mesh) {
    let mut edges = BTreeMap::<(u32, u32), (usize, i32)>::new();
    for &[a, b, c] in &mesh.triangles {
        // These are stored f32 coordinates, but differences/cross products
        // need f64, as in the USD writer. A captured valid one-ULP edge lost
        // its area entirely when the subtraction was performed in f32.
        let points = [a, b, c].map(|i| {
            let p = mesh.positions[i as usize];
            [f64::from(p.x), f64::from(p.y), f64::from(p.z)]
        });
        let u: [f64; 3] = std::array::from_fn(|i| points[1][i] - points[0][i]);
        let v: [f64; 3] = std::array::from_fn(|i| points[2][i] - points[0][i]);
        let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        assert!(n[0].hypot(n[1]).hypot(n[2]) > 0., "collapsed triangle {:?} at {:?}", [a, b, c], points);
        for (a, b) in [(a, b), (b, c), (c, a)] {
            let entry = edges.entry((a.min(b), a.max(b))).or_default();
            entry.0 += 1;
            entry.1 += if a < b { 1 } else { -1 };
        }
    }
    assert!(edges.values().all(|entry| *entry == (2, 0)));
}

#[test]
fn triangular_prism_is_projected_to_rounded_native_edge_and_corner_caps() {
    let scene = flat();
    let mesh = prism();
    let before = residual(&scene, &mesh);
    assert!(before > R * 0.5, "the input must actually lack its rounded caps");
    let original = mesh.clone();
    let accepted = options().max_residual;
    let out = refine(&scene, mesh, options()).unwrap();
    assert!(residual(&scene, &out.mesh) <= accepted + 2e-7);
    closed(&out.mesh);
    assert!(out.mesh.positions.iter().any(|p| p.x < -R * 0.9));
    assert!(
        out.mesh.positions.iter().any(|p| p.x < -R * 0.5 && p.y < -R * 0.5),
        "rounded source vertex cap is missing"
    );
    assert!(out.mesh.positions.iter().any(|p| p.z > R * 0.99) && out.mesh.positions.iter().any(|p| p.z < -R * 0.99));
    assert_eq!(out.face_source_indices.len(), out.mesh.triangles.len());
    assert!(out.face_source_indices.iter().all(|&index| index < original.triangles.len()));
    assert_eq!(
        out.report["charged_work"].as_u64().unwrap(),
        out.report["field_work"].as_u64().unwrap() + out.report["structural_work"].as_u64().unwrap()
    );
    assert!(out.report["charged_work"].as_u64().unwrap() <= options().max_work as u64);
    for (index, &point) in original.positions.iter().enumerate() {
        if scene.sample_authored(point).dist.abs() <= accepted {
            assert_eq!(out.mesh.positions[index], point, "already-valid original vertex moved");
        }
    }
    assert!(out.mesh.metadata.surface_area > original.metadata.surface_area);
    let work = out.report["charged_work"].as_u64().unwrap() as usize;
    let mut exact = options();
    exact.max_work = work;
    let replay = refine(&scene, original.clone(), exact).unwrap();
    assert_eq!(replay.mesh.positions, out.mesh.positions);
    assert_eq!(replay.mesh.triangles, out.mesh.triangles);
    assert_eq!(replay.face_source_indices, out.face_source_indices);
    let mut short = options();
    short.max_work = work - 1;
    assert!(refine(&scene, original, short).err().unwrap().contains("work budget"));
}

fn folded_vertices() -> Vec<Vec3> {
    vec![Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(1., 1., 0.), Vec3::new(0., 1., 0.5)]
}
fn folded_scene() -> Scene {
    scene(folded_vertices(), vec![[0, 1, 2], [0, 2, 3]])
}
#[test]
fn folded_scalar_grid_correspondence_exhaustion_is_a_bounded_rejection() {
    // Retain the unchanged original native sheet and scalar-grid input. This
    // input's fixed correspondence is outside the bounded refiner's guarantee;
    // accepting it would hide the missing edge region preserved in r19 evidence.
    let scene = folded_scene();
    let mesh = extract_isosurface(
        |p| scene.sample_authored(p).dist,
        Vec3::new(-0.3, -0.3, -0.3),
        Vec3::new(1.3, 1.3, 0.8),
        [12, 12, 12],
    )
    .unwrap();
    assert_eq!((mesh.positions.len(), mesh.triangles.len(), mesh.metadata.connected_components), (962, 1920, 1));
    let witness = Vec3::new(0.17794356, 1.0891156, 0.41172564);
    let before = scene.sample_authored(witness).dist.to_bits();
    let error = refine(&scene, mesh, options()).err().expect("unresolved folded correspondence was falsely accepted");
    // Native outward checks now spend the unchanged aggregate allowance before
    // the pass limit on this unsupported fixed correspondence. It still rejects
    // rather than accepting an incomplete native edge region.
    assert_budget_rejection(&error);
    assert_eq!(scene.sample_authored(witness).dist.to_bits(), before);
}

#[test]
fn folded_support_poly_producer_refines_the_original_complete_native_sheet() {
    let mut scene = folded_scene();
    let corners = CornerUvs::new(vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]], vec![[0, 1, 2], [0, 2, 3]], 2).unwrap();
    let image = TextureImage::from_linear_rgba(
        16,
        16,
        (0..256).map(|i| [(i % 16) as f32 / 15., (i / 16) as f32 / 15., ((i * 7) % 16) as f32 / 15., 1.]).collect(),
    )
    .unwrap();
    scene
        .bind_surface_maps(
            0,
            corners,
            SurfaceMaps { albedo: Some(TextureMap { image, sampler: Sampler::default() }), ..SurfaceMaps::default() },
        )
        .unwrap();
    let surface = TriangleSurface::new(folded_vertices(), vec![[0, 1, 2], [0, 2, 3]], R).unwrap();
    let features = SurfaceFeatures::with_supporting_planes(surface, f64::from(R), 200_000_000).unwrap();
    // Match the actual delivery wrapper's explicit bounded representation
    // policy. Native source, grid, budgets and all downstream gates stay fixed.
    let max_representation_error_m = (f64::from(R) * 0.0005).min(f64::from(R * 0.005) * 0.1);
    let extracted = extract_local_convex_union_isosurface_with_representation(
        Vec3::new(-0.3, -0.3, -0.3),
        Vec3::new(1.3, 1.3, 0.8),
        [12, 12, 12],
        LocalExtractionOptions {
            max_work: 200_000_000 - features.construction_work,
            max_vertices: options().max_vertices,
            max_triangles: options().max_triangles,
        },
        ConvexRepresentationPolicy { max_representation_error_m },
        |cell, remaining| features.sample_axis_aligned_cell(cell.points[0], cell.points[7], remaining),
    )
    .unwrap();
    let representation = extracted.report.representation.as_ref().expect("actual delivery policy was not applied");
    assert!(representation.correspondence.max_displacement_m <= max_representation_error_m);
    eprintln!(
        "support producer: {} vertices/{} faces, {} work",
        extracted.mesh.positions.len(),
        extracted.mesh.triangles.len(),
        extracted.report.charged_work()
    );
    closed(&extracted.mesh);
    let mut bounded = options();
    bounded.max_residual = R * 0.005;
    bounded.normal_step_m = f64::from(R) * 0.05;
    let out = refine(&scene, extracted.mesh, bounded).unwrap();
    eprintln!("folded native refiner: {}", out.report);
    assert!(residual(&scene, &out.mesh) <= R * 0.005 + 2e-7);
    closed(&out.mesh);
    assert_eq!(out.mesh.metadata.connected_components, 1);
    assert!(out.mesh.positions.iter().any(|p| p.z < -R * 0.8));
    assert!(out.mesh.positions.iter().any(|p| p.z > 0.5 + R * 0.8));
    assert_eq!(out.face_source_indices.len(), out.mesh.triangles.len());
    assert!(out.face_source_indices.iter().all(|&i| i < extracted.face_feature_ids.len()));
    let transferred = transfer(
        &scene,
        &out.mesh.positions,
        &out.mesh.triangles,
        &BTreeMap::from([(scene.objects[0].mat, 0)]),
        TransferOptions {
            max_uv_error_texels: 0.2,
            max_refinement_passes: 5,
            max_work: 20_000_000,
            max_vertices: options().max_vertices,
            max_triangles: options().max_triangles,
        },
    )
    .unwrap();
    let mut delivered = out.mesh.clone();
    delivered.positions = transferred.positions;
    delivered.triangles = transferred.triangles;
    closed(&delivered);
    assert!(residual(&scene, &delivered) <= R * 0.005 + 2e-7);
    // Original authored interior points are analytic witnesses: both normal rays
    // must hit the full shell at radius R, and the interpolated delivered UV must
    // recover the original point's authored XY coordinates. This catches using
    // the midsurface or attaching either side to a nearby unrelated source face.
    let authored = folded_vertices();
    for triangle in [[0usize, 1, 2], [0, 2, 3]] {
        let [a, b, c] = triangle.map(|i| authored[i]);
        let normal = (b - a).cross(c - a).normalize();
        for weights in [[1. / 3.; 3], [0.25, 0.5, 0.25], [0.25, 0.25, 0.5]] {
            let p = a * weights[0] + b * weights[1] + c * weights[2];
            for side in [-1., 1.] {
                let (distance, face, barycentric) =
                    ray_hit(&delivered, p, normal * side).expect("native shell side is missing");
                assert!(
                    (distance - f64::from(R)).abs() <= f64::from(R) * 0.005 + 2e-7,
                    "wrong physical shell thickness: {distance}"
                );
                let uv = std::array::from_fn::<_, 2, _>(|axis| {
                    (0..3).map(|i| transferred.corner_uvs[face][i][axis] * barycentric[i]).sum::<f64>()
                });
                assert!(
                    (uv[0] - f64::from(p.x)).abs() * 16. <= 0.2 + 2e-5,
                    "U attached to wrong original location: {uv:?}, {p:?}"
                );
                assert!(
                    (uv[1] - f64::from(p.y)).abs() * 16. <= 0.2 + 2e-5,
                    "V attached to wrong original location: {uv:?}, {p:?}"
                );
            }
        }
    }
}

#[test]
fn exhausted_work_geometry_pass_and_coordinate_budgets_fail_without_scene_mutation() {
    let scene = flat();
    let mesh = prism();
    let before = scene.sample_authored(Vec3::new(-0.05, -0.05, 0.)).dist;
    for kind in 0..5 {
        let mut options = options();
        match kind {
            0 => options.max_work = 1,
            1 => options.max_vertices = mesh.positions.len(),
            2 => options.max_triangles = mesh.triangles.len(),
            3 => options.max_passes = 0,
            _ => options.normal_step_m = f64::MIN_POSITIVE,
        }
        assert!(refine(&scene, mesh.clone(), options).is_err(), "case {kind}");
    }
    for spacing in [Vec3::ZERO, Vec3::new(0.1, f32::NAN, 0.1), Vec3::new(0.1, -0.1, 0.1)] {
        let mut invalid = mesh.clone();
        invalid.metadata.spacing = spacing;
        assert!(refine(&scene, invalid, options()).err().unwrap().contains("grid spacing"));
    }
    assert_eq!(scene.sample_authored(Vec3::new(-0.05, -0.05, 0.)).dist.to_bits(), before.to_bits());
}

#[test]
fn open_edges_reversed_winding_and_vertex_pinches_are_not_silently_repaired() {
    let scene = flat();
    let original = prism();
    let mut open = original.clone();
    open.triangles.pop();
    assert!(refine(&scene, open, options()).err().unwrap().contains("closed"));
    let mut reversed = original.clone();
    reversed.triangles[0].swap(0, 1);
    assert!(refine(&scene, reversed, options()).err().unwrap().contains("oriented"));
    let mut pinched = original.clone();
    let count = pinched.positions.len() as u32;
    pinched.positions.extend(original.positions.iter().map(|p| *p + Vec3::new(0., 0., 0.01)));
    pinched
        .triangles
        .extend(original.triangles.iter().map(|triangle| triangle.map(|i| if i == 0 { 0 } else { i + count })));
    assert!(refine(&scene, pinched, options()).err().unwrap().contains("vertex fans"));
}

fn ray_hit(mesh: &Mesh, origin: Vec3, direction: Vec3) -> Option<(f64, usize, [f64; 3])> {
    fn d(p: Vec3) -> [f64; 3] {
        [p.x, p.y, p.z].map(f64::from)
    }
    fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        std::array::from_fn(|i| a[i] - b[i])
    }
    fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
        (0..3).map(|i| a[i] * b[i]).sum()
    }
    fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
    }
    let origin = d(origin);
    let direction = d(direction);
    let mut closest = None;
    for (face, triangle) in mesh.triangles.iter().enumerate() {
        let [a, b, c] = triangle.map(|i| d(mesh.positions[i as usize]));
        let e1 = sub(b, a);
        let e2 = sub(c, a);
        let p = cross(direction, e2);
        let determinant = dot(e1, p);
        if determinant.abs() <= 1e-20 {
            continue;
        }
        let t = sub(origin, a);
        let u = dot(t, p) / determinant;
        let q = cross(t, e1);
        let v = dot(direction, q) / determinant;
        let distance = dot(e2, q) / determinant;
        if u >= -1e-10
            && v >= -1e-10
            && u + v <= 1. + 1e-10
            && distance >= 0.
            && closest.as_ref().is_none_or(|&(best, _, _)| distance < best)
        {
            closest = Some((distance, face, [1. - u - v, u, v]));
        }
    }
    closest
}
