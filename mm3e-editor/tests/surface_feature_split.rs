//! Original thin-triangle shoulder regression. Historical extraction counters
//! are not input to refine; domain, geometry and all refinement limits are the
//! exact frozen values. Independent checks cover native error and rounded shape.
use mm3e_editor::surface_refine::{refine, Options};
use mm3e_kit::{
    meshing::{Mesh, MeshingMetadata},
    surface::TriangleSurface,
    Material, Transform, Vec3,
};
use mm3e_orchestrator::{Object, Prim, Scene};
type D = [f64; 3];
fn d(p: Vec3) -> D {
    [p.x, p.y, p.z].map(f64::from)
}
fn sub(a: D, b: D) -> D {
    std::array::from_fn(|i| a[i] - b[i])
}
fn add(a: D, b: D) -> D {
    std::array::from_fn(|i| a[i] + b[i])
}
fn scale(a: D, s: f64) -> D {
    a.map(|v| v * s)
}
fn dot(a: D, b: D) -> f64 {
    (0..3).map(|i| a[i] * b[i]).sum()
}
fn cross(a: D, b: D) -> D {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn norm(v: D) -> f64 {
    v[0].hypot(v[1]).hypot(v[2])
}
fn vec(v: &serde_json::Value) -> Vec3 {
    Vec3::new(v[0].as_f64().unwrap() as f32, v[1].as_f64().unwrap() as f32, v[2].as_f64().unwrap() as f32)
}
fn fixture() -> (Mesh, Scene, [D; 3], f64, Options) {
    let v: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/surface_feature_split/flat-alias.json")).unwrap();
    let source = v["source"]["vertices"].as_array().unwrap().iter().map(vec).collect::<Vec<_>>();
    let original: [D; 3] = source.iter().copied().map(d).collect::<Vec<_>>().try_into().unwrap();
    let radius = (v["source"]["thickness_m"].as_f64().unwrap() as f32) * 0.5;
    let mut scene = Scene::new(8, 8);
    let id = scene.surface(TriangleSurface::new(source, vec![[0, 1, 2]], radius).unwrap()).unwrap();
    let material = scene.material(Material::default());
    scene.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, material));
    let positions = v["positions"].as_array().unwrap().iter().map(vec).collect();
    let triangles = v["triangles"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| [t[0].as_u64().unwrap() as u32, t[1].as_u64().unwrap() as u32, t[2].as_u64().unwrap() as u32])
        .collect();
    let metadata = MeshingMetadata {
        min: vec(&v["bounds_min"]),
        max: vec(&v["bounds_max"]),
        resolution: [32, 32, 8],
        spacing: vec(&v["spacing"]),
        iso_level: 0.,
        field_evaluations: 0,
        active_cells: 0,
        field_range: [0.; 2],
        boundary_minimum: 0.,
        boundary_intersection: false,
        clipping_allowed: false,
        boundary_edges: 0,
        connected_components: 1,
        degenerate_triangles_removed: 0,
        coincident_vertices_merged: 0,
        collinear_faces_split: 0,
        boolean_channels: 0,
        boolean_arrangement_work: 0,
        boolean_extra_field_evaluations: 0,
        boolean_projected_vertices: 0,
        boolean_refinement_passes: 0,
        boolean_added_vertices: 0,
        boolean_added_triangles: 0,
        boolean_max_vertex_displacement: 0.,
        surface_area: 0.,
        signed_volume: 0.,
    };
    let options = Options {
        max_residual: v["max_residual"].as_f64().unwrap() as f32,
        normal_step_m: v["normal_step_m"].as_f64().unwrap(),
        max_work: v["max_work"].as_u64().unwrap() as usize,
        max_vertices: v["max_vertices"].as_u64().unwrap() as usize,
        max_triangles: v["max_triangles"].as_u64().unwrap() as usize,
        max_passes: v["max_passes"].as_u64().unwrap() as u32,
    };
    (Mesh { positions, triangles, metadata }, scene, original, f64::from(radius), options)
}
fn distance(p: D, source: [D; 3], radius: f64) -> f64 {
    let [a, b, c] = source;
    let u = (p[0] - a[0]) / (b[0] - a[0]);
    let v = (p[1] - a[1]) / (c[1] - a[1]);
    if u >= 0. && v >= 0. && u + v <= 1. {
        return (p[2] - a[2]).abs() - radius;
    }
    [(a, b), (b, c), (c, a)]
        .into_iter()
        .map(|(a, b)| {
            let edge = sub(b, a);
            let t = (dot(sub(p, a), edge) / dot(edge, edge)).clamp(0., 1.);
            norm(sub(p, add(a, scale(edge, t))))
        })
        .fold(f64::INFINITY, f64::min)
        - radius
}
fn ray(mesh: &Mesh, origin: D, direction: D) -> f64 {
    mesh.triangles
        .iter()
        .filter_map(|t| {
            let [a, b, c] = t.map(|i| d(mesh.positions[i as usize]));
            let e1 = sub(b, a);
            let e2 = sub(c, a);
            let h = cross(direction, e2);
            let det = dot(e1, h);
            if det == 0. {
                return None;
            }
            let s = sub(origin, a);
            let u = dot(s, h) / det;
            let q = cross(s, e1);
            let v = dot(direction, q) / det;
            let time = dot(e2, q) / det;
            (u >= 0. && v >= 0. && u + v <= 1. && time > 0.).then_some(time)
        })
        .fold(f64::INFINITY, f64::min)
}
#[test]
fn original_frozen_flat_shoulders_converge_within_unchanged_six_passes() {
    let (input, scene, source, radius, options) = fixture();
    let allowed = f64::from(options.max_residual);
    let work = options.max_work;
    assert_eq!((input.positions.len(), input.triangles.len()), (2312, 4620));
    assert_eq!(options.max_passes, 6);
    assert_eq!(work, 109_954_694);
    let output = refine(&scene, input, options).unwrap();
    println!("feature-local flat refinement {}", output.report);
    assert!(output.report["charged_work"].as_u64().unwrap() <= work as u64);
    assert!(output.report["passes"].as_u64().unwrap() <= 6);
    assert!(output.report["feature_directed_splits"].as_u64().unwrap() > 0);
    assert_eq!(output.face_source_indices.len(), output.mesh.triangles.len());
    assert!(output.face_source_indices.iter().all(|&i| i < 4620));
    let topology = mm3e_kit::mesh_topology::validate(&output.mesh.triangles, 200_000_000).unwrap();
    assert_eq!(topology.components, 1);
    assert_eq!(topology.vertices + topology.triangles, topology.edges + 2);
    mm3e_kit::surface_intersections::validate_counted(&output.mesh.positions, &output.mesh.triangles, 200_000_000)
        .unwrap();
    let mut maximum = 0.0_f64;
    let mut native = 0.0_f64;
    let mut volume = 0.;
    for &p in &output.mesh.positions {
        maximum = maximum.max(distance(d(p), source, radius).abs());
        native = native.max(f64::from(scene.sample_authored(p).dist.abs()));
    }
    for t in &output.mesh.triangles {
        let [a, b, c] = t.map(|i| d(output.mesh.positions[i as usize]));
        volume += dot(a, cross(b, c)) / 6.;
        for weights in [[0.5, 0.5, 0.], [0., 0.5, 0.5], [0.5, 0., 0.5], [1. / 3.; 3]] {
            let q = (0..3).fold([0.; 3], |sum, i| add(sum, scale([a, b, c][i], weights[i])));
            let p = Vec3::new(q[0] as f32, q[1] as f32, q[2] as f32);
            maximum = maximum.max(distance(d(p), source, radius).abs());
            native = native.max(f64::from(scene.sample_authored(p).dist.abs()));
        }
    }
    assert!(native <= allowed, "native residual {native} > {allowed}");
    assert!(maximum <= allowed, "analytic residual {maximum} > {allowed}");
    let [a, b, c] = source;
    let area = norm(cross(sub(b, a), sub(c, a))) * 0.5;
    let perimeter = norm(sub(b, a)) + norm(sub(c, b)) + norm(sub(a, c));
    let exact_volume = 2. * radius * area
        + 0.5 * std::f64::consts::PI * radius * radius * perimeter
        + 4. * std::f64::consts::PI / 3. * radius.powi(3);
    assert!((volume - exact_volume).abs() / exact_volume <= 0.001, "rounded volume {volume} vs {exact_volume}");
    let face = scale(add(add(a, b), c), 1. / 3.);
    let mut rays = vec![(face, [0., 0., 1.]), (face, [0., 0., -1.])];
    for (a, b) in [(a, b), (b, c), (c, a)] {
        let edge = sub(b, a);
        let n = [edge[1], -edge[0], 0.];
        let n = scale(n, 1. / norm(n));
        let origin = scale(add(a, b), 0.5);
        for degrees in [0.0_f64, 22.5, 45., 67.5, 90.] {
            let angle = degrees.to_radians();
            for sign in [-1., 1.] {
                rays.push((origin, add(scale(n, angle.sin()), [0., 0., sign * angle.cos()])));
            }
        }
    }
    for (origin, direction) in rays {
        let hit = ray(&output.mesh, origin, direction);
        assert!(
            hit.is_finite() && (hit - radius).abs() <= radius * 0.02,
            "shoulder/cross-section hit {hit} vs {radius}: {origin:?} {direction:?}"
        );
    }
}

#[test]
fn original_cross_feature_chord_needs_a_local_shoulder_before_binary_refinement() {
    // Exact observed post-projection edge409→416 in the frozen flat fixture.
    // Coordinates are (signed outward distance from source edge, z-plane_z).
    let radius = 0.0010000000474974513;
    let left = [-0.023091466863072983, -0.0010000001639127731];
    let right = [0.0007071198159579811, -0.0007071038708090782];
    let error = |p: [f64; 2]| (if p[0] <= 0. { p[1].abs() } else { p[0].hypot(p[1]) } - radius).abs();
    let project = |p: [f64; 2]| {
        if p[0] <= 0. {
            [p[0], -radius]
        } else {
            let scale = radius / p[0].hypot(p[1]);
            [p[0] * scale, p[1] * scale]
        }
    };
    let maximum = |points: &[[f64; 2]]| {
        points.windows(2).map(|p| error([(p[0][0] + p[1][0]) * 0.5, (p[0][1] + p[1][1]) * 0.5])).fold(0., f64::max)
    };
    let subdivide = |points: Vec<[f64; 2]>| {
        let mut next = Vec::new();
        for edge in points.windows(2) {
            next.push(edge[0]);
            next.push(project([(edge[0][0] + edge[1][0]) * 0.5, (edge[0][1] + edge[1][1]) * 0.5]));
        }
        next.push(*points.last().unwrap());
        next
    };
    let mut ordinary = vec![left, right];
    for _ in 0..6 {
        ordinary = subdivide(ordinary);
    }
    assert!(maximum(&ordinary) > radius * 0.005);
    let parameter = -left[0] / (right[0] - left[0]);
    assert!(parameter > 0. && parameter < 1.);
    let on_original_edge = [left[0] + parameter * (right[0] - left[0]), left[1] + parameter * (right[1] - left[1])];
    assert!(on_original_edge[0].abs() < 1e-14);
    let mut localized = vec![left, project(on_original_edge), right];
    for _ in 0..2 {
        localized = subdivide(localized);
    }
    assert!(maximum(&localized) <= radius * 0.005);
}
