use std::collections::HashMap;

use mm3e_kit::meshing::{extract_isosurface, extract_isosurface_with_options, Mesh, MeshingOptions};
use mm3e_kit::{sdf, Vec3};

fn bounds(half: f32) -> (Vec3, Vec3) {
    (Vec3::splat(-half), Vec3::splat(half))
}

fn mesh(field: impl Fn(Vec3) -> f32, half: f32, resolution: u32) -> Mesh {
    let (lo, hi) = bounds(half);
    extract_isosurface(field, lo, hi, [resolution; 3]).unwrap()
}

/// Independent edge/volume checks, rather than trusting extractor metadata.
fn assert_closed(mesh: &Mesh) -> f64 {
    assert!(!mesh.positions.is_empty());
    assert!(!mesh.triangles.is_empty());
    let mut edges: HashMap<(u32, u32), Vec<(u32, u32)>> = HashMap::new();
    let mut volume = 0.0f64;
    for &[a, b, c] in &mesh.triangles {
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(c, a);
        let [pa, pb, pc] = [a, b, c].map(|i| mesh.positions[i as usize]);
        let normal = (pb - pa).cross(pc - pa);
        assert!(normal.length_sq() > 0.0, "degenerate face: {pa:?} {pb:?} {pc:?}");
        assert!([pa, pb, pc].iter().all(|p| p.x.is_finite() && p.y.is_finite() && p.z.is_finite()));
        volume += f64::from(pa.dot(pb.cross(pc))) / 6.0;
        for (i, j) in [(a, b), (b, c), (c, a)] {
            edges.entry((i.min(j), i.max(j))).or_default().push((i, j));
        }
    }
    for (edge, incidents) in edges {
        assert_eq!(incidents.len(), 2, "edge {edge:?} has incidents {incidents:?}");
        assert_eq!(incidents[0], (incidents[1].1, incidents[1].0), "inconsistent winding at {edge:?}");
    }
    assert!(volume > 0.0);
    assert!((volume - mesh.metadata.signed_volume).abs() < 1e-5);
    assert_eq!(mesh.metadata.boundary_edges, 0);
    volume
}

fn maximum_field_residual(mesh: &Mesh, field: impl Fn(Vec3) -> f32) -> f32 {
    // Samples include triangle interiors and edges, not only lattice-edge roots.
    let mut maximum = 0.0f32;
    for &[a, b, c] in &mesh.triangles {
        let [a, b, c] = [a, b, c].map(|i| mesh.positions[i as usize]);
        for p in [a, b, c, (a + b + c) * (1.0 / 3.0), a * 0.2 + b * 0.3 + c * 0.5, (a + b) * 0.5] {
            maximum = maximum.max(field(p).abs());
        }
    }
    maximum
}

fn assert_winding(mesh: &Mesh, field: impl Fn(Vec3) -> f32) {
    let epsilon = mesh.metadata.spacing.x.min(mesh.metadata.spacing.y).min(mesh.metadata.spacing.z) * 0.05;
    for &[a, b, c] in &mesh.triangles {
        let [a, b, c] = [a, b, c].map(|i| mesh.positions[i as usize]);
        let center = (a + b + c) * (1.0 / 3.0);
        let normal = (b - a).cross(c - a).normalize();
        assert!(
            field(center + normal * epsilon) > field(center - normal * epsilon),
            "inward triangle at {center:?}, normal {normal:?}, plus {} minus {}",
            field(center + normal * epsilon),
            field(center - normal * epsilon)
        );
    }
}

/// Point-to-triangle distance via projected barycentric coordinates and three
/// independently clamped segments. Used to test exact-surface -> mesh coverage.
fn triangle_distance(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> f32 {
    let ab = b - a;
    let ac = c - a;
    let n = ab.cross(ac);
    let n2 = n.length_sq();
    let projection = p - n * ((p - a).dot(n) / n2);
    let v = projection - a;
    let aa = ab.dot(ab);
    let bb = ab.dot(ac);
    let cc = ac.dot(ac);
    let denominator = aa * cc - bb * bb;
    if denominator > 0.0 {
        let u = (v.dot(ab) * cc - v.dot(ac) * bb) / denominator;
        let w = (v.dot(ac) * aa - v.dot(ab) * bb) / denominator;
        if u >= 0.0 && w >= 0.0 && u + w <= 1.0 {
            return (p - projection).length();
        }
    }
    [(a, b), (b, c), (c, a)]
        .into_iter()
        .map(|(a, b)| {
            let edge = b - a;
            let t = ((p - a).dot(edge) / edge.length_sq()).clamp(0.0, 1.0);
            (p - (a + edge * t)).length()
        })
        .fold(f32::INFINITY, f32::min)
}

fn surface_to_mesh_error(mesh: &Mesh, points: impl IntoIterator<Item = Vec3>) -> f32 {
    points
        .into_iter()
        .map(|p| {
            mesh.triangles
                .iter()
                .map(|t| {
                    let [a, b, c] = t.map(|i| mesh.positions[i as usize]);
                    triangle_distance(p, a, b, c)
                })
                .fold(f32::INFINITY, f32::min)
        })
        .fold(0.0, f32::max)
}

#[test]
fn sphere_is_indexed_closed_outward_and_converges_in_both_distance_directions() {
    let sphere = |p| sdf::sphere(p, 1.0);
    let coarse = mesh(sphere, 1.5, 16);
    let fine = mesh(sphere, 1.5, 32);
    let actual_volume = assert_closed(&fine);
    assert_winding(&fine, sphere);
    assert_eq!(fine.metadata.connected_components, 1);
    assert_eq!(fine.metadata.field_evaluations, 33usize.pow(3));
    assert!(fine.positions.len() < fine.triangles.len());
    assert!((actual_volume / (4.0 * std::f64::consts::PI / 3.0) - 1.0).abs() < 0.008);
    let coarse_residual = maximum_field_residual(&coarse, sphere);
    let fine_residual = maximum_field_residual(&fine, sphere);
    assert!(fine_residual < 0.006, "fine residual {fine_residual}");
    assert!(fine_residual < coarse_residual * 0.4, "{coarse_residual} -> {fine_residual}");
    let points = (0..96).map(|i| {
        let y = 1.0 - 2.0 * (i as f32 + 0.5) / 96.0;
        let theta = i as f32 * 2.399_963_1;
        Vec3::new((1.0 - y * y).sqrt() * theta.cos(), y, (1.0 - y * y).sqrt() * theta.sin())
    });
    let exact_to_mesh = surface_to_mesh_error(&fine, points);
    assert!(exact_to_mesh < 0.006, "exact sphere -> mesh error {exact_to_mesh}");
    assert_eq!(fine, mesh(sphere, 1.5, 32), "output must be deterministic including indices");
}

#[test]
fn box_retains_faces_and_exact_lattice_isovalues_do_not_crack() {
    let half = Vec3::new(0.75, 0.5, 0.625);
    let field = |p| sdf::boxed(p, half);
    let extracted = mesh(field, 1.0, 32);
    let volume = assert_closed(&extracted);
    assert_winding(&extracted, field);
    assert!((volume / f64::from(8.0 * half.x * half.y * half.z) - 1.0).abs() < 0.012);
    assert!(maximum_field_residual(&extracted, field) < 0.035);
    let mut minimum = Vec3::splat(f32::INFINITY);
    let mut maximum = Vec3::splat(f32::NEG_INFINITY);
    for &p in &extracted.positions {
        minimum = minimum.min(p);
        maximum = maximum.max(p);
    }
    assert_eq!(minimum, -half);
    assert_eq!(maximum, half);
    assert!(extracted.metadata.degenerate_triangles_removed > 0);
    assert!(surface_to_mesh_error(&extracted, [Vec3::new(half.x, 0.1, 0.2), Vec3::new(-0.2, half.y, 0.1)]) < 1e-6);
}

#[test]
fn torus_retains_its_hole_and_matches_analytic_surface_and_volume() {
    let field = |p| sdf::torus(p, 0.8, 0.27);
    let extracted = mesh(field, 1.3, 40);
    let volume = assert_closed(&extracted);
    assert_winding(&extracted, field);
    let expected_volume = 2.0 * std::f64::consts::PI.powi(2) * 0.8 * 0.27f64.powi(2);
    assert!((volume / expected_volume - 1.0).abs() < 0.025);
    assert!(maximum_field_residual(&extracted, field) < 0.01);
    assert!(extracted.positions.iter().all(|p| (p.x * p.x + p.z * p.z).sqrt() > 0.5));
    let exact_points = (0..12).flat_map(|i| {
        (0..8).map(move |j| {
            let u = (i as f32 + 0.3) * std::f32::consts::TAU / 12.0;
            let v = (j as f32 + 0.2) * std::f32::consts::TAU / 8.0;
            Vec3::new((0.8 + 0.27 * v.cos()) * u.cos(), 0.27 * v.sin(), (0.8 + 0.27 * v.cos()) * u.sin())
        })
    });
    assert!(surface_to_mesh_error(&extracted, exact_points) < 0.01);
}

#[test]
fn subtractive_cavity_has_inner_outward_winding_and_negative_volume_contribution() {
    let center = Vec3::new(0.13, -0.07, 0.11);
    let field = |p| sdf::sphere(p, 1.0).max(-sdf::sphere(p - center, 0.4));
    let extracted = mesh(field, 1.3, 38);
    let volume = assert_closed(&extracted);
    assert_winding(&extracted, field);
    assert_eq!(extracted.metadata.connected_components, 2, "both outer shell and enclosed cavity must be extracted");
    let expected = 4.0 * std::f64::consts::PI / 3.0 * (1.0 - 0.4f64.powi(3));
    assert!((volume / expected - 1.0).abs() < 0.01);
    assert!(maximum_field_residual(&extracted, field) < 0.009);
    let cavity_faces = extracted
        .triangles
        .iter()
        .filter(|tri| {
            let [a, b, c] = tri.map(|i| extracted.positions[i as usize]);
            let p = (a + b + c) * (1.0 / 3.0);
            if (p - center).length() < 0.5 {
                assert!((b - a).cross(c - a).dot(p - center) < 0.0);
                true
            } else {
                false
            }
        })
        .count();
    assert!(cavity_faces > 100);
}

#[test]
fn open_cut_and_disconnected_boolean_components_survive_sampling() {
    let cutter = Vec3::new(0.8, 0.0, 0.0);
    let cut = |p| sdf::sphere(p, 1.0).max(-sdf::sphere(p - cutter, 0.6));
    let extracted = mesh(cut, 1.3, 38);
    assert_closed(&extracted);
    // At the unresolved sharp intersection, a linear facet can span both CSG
    // branches while its center's exact-field gradient selects only one. Check
    // exact analytic normals off that crease; all edges are checked above.
    let mut smooth_faces = 0;
    for tri in &extracted.triangles {
        let [a, b, c] = tri.map(|i| extracted.positions[i as usize]);
        let p = (a + b + c) * (1.0 / 3.0);
        let outer = sdf::sphere(p, 1.0);
        let inner = -sdf::sphere(p - cutter, 0.6);
        if (outer - inner).abs() > extracted.metadata.spacing.length() {
            let outward = if outer > inner { p.normalize() } else { -(p - cutter).normalize() };
            assert!((b - a).cross(c - a).dot(outward) > 0.0);
            smooth_faces += 1;
        }
    }
    assert!(smooth_faces > extracted.triangles.len() / 2);
    assert_eq!(extracted.metadata.connected_components, 1);
    let cut_error = maximum_field_residual(&extracted, cut);
    let coarse_error = maximum_field_residual(&mesh(cut, 1.3, 19), cut);
    assert!(cut_error < extracted.metadata.spacing.length() * 0.5, "cut error {cut_error}");
    assert!(cut_error < coarse_error * 0.75, "cut residual {coarse_error} -> {cut_error}");

    let offset = Vec3::new(0.65, 0.0, 0.0);
    let separate = |p| sdf::sphere(p - offset, 0.32).min(sdf::sphere(p + offset, 0.32));
    let extracted = mesh(separate, 1.2, 36);
    assert_closed(&extracted);
    assert_eq!(extracted.metadata.connected_components, 2);
    assert!(extracted.positions.iter().any(|p| p.x < -0.9));
    assert!(extracted.positions.iter().any(|p| p.x > 0.9));
}

#[test]
fn a_resolved_thin_shell_preserves_both_surfaces() {
    let field = |p: Vec3| (p.length() - 0.85).abs() - 0.06;
    let extracted = mesh(field, 1.1, 48);
    let volume = assert_closed(&extracted);
    assert_eq!(extracted.metadata.connected_components, 2);
    let expected = 4.0 * std::f64::consts::PI / 3.0 * (0.91f64.powi(3) - 0.79f64.powi(3));
    assert!((volume / expected - 1.0).abs() < 0.04);
    assert!(maximum_field_residual(&extracted, field) < 0.012);
}

#[test]
fn clipping_requires_explicit_mode_and_never_adds_caps() {
    let (lo, hi) = bounds(1.0);
    let error = extract_isosurface(|p| p.x, lo, hi, [8; 3]).unwrap_err();
    assert!(error.contains("clips") && error.contains("bounds"));
    // A sphere only tangent to a sampled boundary is rejected as well.
    assert!(extract_isosurface(|p| sdf::sphere(p, 1.0), lo, hi, [8; 3]).unwrap_err().contains("bounds"));
    let clipped = extract_isosurface_with_options(
        |p| p.x,
        lo,
        hi,
        MeshingOptions { resolution: [8; 3], allow_clipping: true, ..MeshingOptions::default() },
    )
    .unwrap();
    assert!(clipped.metadata.boundary_intersection);
    assert!(clipped.metadata.boundary_edges > 0);
    assert!(clipped.positions.iter().all(|p| p.x == 0.0));
    assert!((clipped.metadata.surface_area - 4.0).abs() < 1e-8);
    assert_winding(&clipped, |p| p.x);
}

#[test]
fn finite_validation_empty_degenerate_fields_and_work_limits_are_explicit() {
    let (lo, hi) = bounds(1.0);
    for invalid in [f32::INFINITY, f32::NEG_INFINITY, f32::NAN] {
        assert!(extract_isosurface(|_| invalid, lo, hi, [2; 3]).unwrap_err().contains("non-finite field"));
    }
    assert!(extract_isosurface(|_| 1.0, lo, hi, [2; 3]).unwrap_err().contains("no nondegenerate isosurface"));
    assert!(extract_isosurface_with_options(
        |_| 0.0,
        lo,
        hi,
        MeshingOptions { resolution: [2; 3], allow_clipping: true, ..MeshingOptions::default() }
    )
    .unwrap_err()
    .contains("no nondegenerate isosurface"));
    assert!(extract_isosurface(|p| p.x, lo, hi, [0, 2, 2]).is_err());
    assert!(extract_isosurface(|_| panic!("over-budget grids must not evaluate"), lo, hi, [500; 3])
        .unwrap_err()
        .contains("budget"));
    assert!(extract_isosurface(|_| panic!("overflow grids must not evaluate"), lo, hi, [u32::MAX; 3]).is_err());
    assert!(extract_isosurface(|p| p.x, hi, lo, [2; 3]).is_err());
    assert!(extract_isosurface(|p| p.x, Vec3::splat(f32::NAN), hi, [2; 3]).is_err());
    assert!(extract_isosurface(|p| p.x, Vec3::splat(1.0), Vec3::splat(1.0 + f32::EPSILON), [2; 3])
        .unwrap_err()
        .contains("not representable"));
}

#[test]
fn nonzero_isovalue_and_anisotropic_grid_work_without_changing_the_field() {
    let options = MeshingOptions { resolution: [20, 24, 28], iso_level: 0.3, allow_clipping: false };
    let extracted =
        extract_isosurface_with_options(|p| sdf::sphere(p, 0.5), Vec3::splat(-1.2), Vec3::splat(1.2), options).unwrap();
    assert_closed(&extracted);
    assert_eq!(extracted.metadata.resolution, [20, 24, 28]);
    assert_eq!(extracted.metadata.field_evaluations, 21 * 25 * 29);
    assert!(maximum_field_residual(&extracted, |p| sdf::sphere(p, 0.8)) < 0.011);
}

#[test]
fn rounded_native_sheet_grid_vertex_rounding_retains_closed_edges() {
    let surface = mm3e_kit::surface::TriangleSurface::new(
        vec![
            Vec3::new(-0.5, 0.0, -0.5),
            Vec3::new(0.5, 0.0, -0.5),
            Vec3::new(0.5, 0.0, 0.5),
            Vec3::new(-0.5, 0.0, 0.5),
        ],
        vec![[0, 1, 2], [0, 2, 3]],
        0.05,
    )
    .unwrap();
    let field = |p| surface.distance(p);
    // This exact alignment previously returned 160 open edges: intersections
    // rounded onto a lattice vertex but retained separate lattice-edge IDs.
    let extracted =
        extract_isosurface(field, Vec3::new(-0.8, -0.3, -0.8), Vec3::new(0.8, 0.3, 0.8), [32, 16, 32]).unwrap();
    let volume = assert_closed(&extracted);
    assert!(volume > 0.095 && volume < 0.13);
    assert!(maximum_field_residual(&extracted, field) < 0.02);
    assert!(extracted.positions.iter().any(|p| p.y > 0.049));
    assert!(extracted.positions.iter().any(|p| p.y < -0.049));
    assert!(extracted.positions.iter().any(|p| p.x > 0.535));
}

#[test]
fn representably_distinct_roots_near_a_grid_vertex_are_never_epsilon_welded() {
    let center = Vec3::new(0.400_000_1, 0.0, 0.0);
    let half = Vec3::new(0.4, 0.4, 0.4);
    let field = |p| sdf::boxed(p - center, half).min(sdf::boxed(p + center, half));
    let extracted = mesh(field, 1.0, 20);
    assert_closed(&extracted);
    assert_eq!(extracted.metadata.connected_components, 2);
    assert!(extracted.positions.iter().any(|p| p.x > 0.0 && p.x < 1e-6));
    assert!(extracted.positions.iter().any(|p| p.x < 0.0 && p.x > -1e-6));
    assert!(extracted.positions.iter().all(|p| p.x != 0.0));
}

#[test]
fn translated_thick_sheet_retains_closed_edges_off_grid() {
    let surface = mm3e_kit::surface::TriangleSurface::new(
        vec![
            Vec3::new(-0.4, -0.3, 0.0),
            Vec3::new(0.4, -0.3, 0.0),
            Vec3::new(0.4, 0.3, 0.0),
            Vec3::new(-0.4, 0.3, 0.0),
        ],
        vec![[0, 1, 2], [0, 2, 3]],
        0.06,
    )
    .unwrap();
    for time in [0.0, 0.5, 1.0] {
        for resolution in [16, 32] {
            let offset = Vec3::new(0.12 * time, 0.06 * time, 0.0);
            let field = |p| surface.distance(p - offset);
            let extracted =
                extract_isosurface(field, Vec3::new(-0.7, -0.6, -0.3), Vec3::new(0.9, 0.7, 0.3), [resolution; 3])
                    .unwrap_or_else(|error| panic!("time {time}, resolution {resolution}: {error}"));
            let volume = assert_closed(&extracted);
            assert!(volume > 0.057 && volume < 0.08);
            let residual = maximum_field_residual(&extracted, field);
            // The half-resolution comparison mesh has its own sampling error;
            // delivery applies the authored residual limit to the fine mesh.
            assert!(residual < extracted.metadata.spacing.length() * 0.5);
            if resolution == 32 {
                assert!(residual < 0.02, "time {time}, fine residual {residual}");
            }
            if time == 0.5 && resolution == 32 {
                assert!(extracted.metadata.coincident_vertices_merged > 0);
                assert!(extracted.metadata.collinear_faces_split > 0);
            }
        }
    }
}
