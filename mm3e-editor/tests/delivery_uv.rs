use mm3e_editor::delivery_uv::{transfer, TransferOptions, TransferredMesh};
use mm3e_kit::{
    meshing::{extract_isosurface, Mesh},
    surface::TriangleSurface,
    texture::{CornerUvs, Sampler, TextureImage},
    Material, Transform, Vec3,
};
use mm3e_orchestrator::{
    appearance::{SurfaceMaps, TextureMap},
    sampling::CountedSceneField,
    Combine, Object, Prim, Scene,
};
use std::collections::BTreeMap;

fn options() -> TransferOptions {
    TransferOptions {
        max_uv_error_texels: 0.2,
        max_refinement_passes: 5,
        max_work: 20_000_000,
        max_vertices: 200_000,
        max_triangles: 300_000,
    }
}
fn bind(scene: &mut Scene, object: usize, corners: Vec<[[f64; 2]; 3]>) {
    let count = corners.len();
    let values = corners.into_iter().flatten().collect();
    let indices = (0..count).map(|i| [3 * i as u32, 3 * i as u32 + 1, 3 * i as u32 + 2]).collect();
    let uv = CornerUvs::new(values, indices, count).unwrap();
    let image = TextureImage::from_linear_rgba(
        16,
        16,
        (0..256).map(|i| [(i % 16) as f32 / 15., (i / 16) as f32 / 15., ((i * 7) % 16) as f32 / 15., 1.]).collect(),
    )
    .unwrap();
    scene
        .bind_surface_maps(
            object,
            uv,
            SurfaceMaps { albedo: Some(TextureMap { image, sampler: Sampler::default() }), ..SurfaceMaps::default() },
        )
        .unwrap();
}
fn scene(vertices: Vec<Vec3>, triangles: Vec<[u32; 3]>, uv: Vec<[[f64; 2]; 3]>) -> Scene {
    let mut scene = Scene::new(8, 8);
    let surface = scene.surface(TriangleSurface::new(vertices, triangles, 0.08).unwrap()).unwrap();
    let material = scene.material(Material::default());
    scene.add(Object::new(Prim::Surface { id: surface }, Transform::IDENTITY, material));
    bind(&mut scene, 0, uv);
    scene
}
fn quad(seam: bool) -> Scene {
    scene(
        vec![Vec3::ZERO, Vec3::new(1., 0., 0.), Vec3::new(1., 1., 0.), Vec3::new(0., 1., 0.)],
        vec![[0, 1, 2], [0, 2, 3]],
        if seam {
            vec![[[0., 0.], [0.4, 0.], [0.4, 0.4]], [[0.6, 0.6], [1., 1.], [0.6, 1.]]]
        } else {
            vec![[[0., 0.], [1., 0.], [1., 1.]], [[0., 0.], [1., 1.], [0., 1.]]]
        },
    )
}
fn extract(scene: &Scene, bounds: (Vec3, Vec3), resolution: [u32; 3]) -> Mesh {
    extract_isosurface(|p| scene.sample_authored(p).dist, bounds.0, bounds.1, resolution).unwrap()
}
fn perform(scene: &Scene, mesh: &Mesh) -> TransferredMesh {
    let materials = scene.objects.iter().enumerate().map(|(i, o)| (o.mat, i)).collect();
    transfer(scene, &mesh.positions, &mesh.triangles, &materials, options()).unwrap()
}
fn geometry_preserved(original: &Mesh, out: &TransferredMesh) {
    assert_eq!(&out.positions[..original.positions.len()], original.positions);
    let original_surface = TriangleSurface::new(original.positions.clone(), original.triangles.clone(), 1e-8).unwrap();
    for p in &out.positions {
        assert!((original_surface.distance(*p) + 1e-8).abs() < 1e-6, "position left extracted geometry: {p:?}");
    }
    let mut edges = BTreeMap::<(u32, u32), usize>::new();
    for &[a, b, c] in &out.triangles {
        for (a, b) in [(a, b), (b, c), (c, a)] {
            *edges.entry((a.min(b), a.max(b))).or_default() += 1;
        }
    }
    let bad: Vec<_> = edges
        .iter()
        .filter(|(_, count)| **count != 2)
        .take(12)
        .map(|(&(a, b), count)| (a, b, count, out.positions[a as usize], out.positions[b as usize]))
        .collect();
    assert!(bad.is_empty(), "conforming closed extraction acquired boundary/nonmanifold edge: {bad:?}; {}", out.report);
    assert_eq!(out.corner_uvs.len(), out.triangles.len());
    assert!(out.report["max_sampled_uv_error_texels"].as_f64().unwrap() <= options().max_uv_error_texels);
    assert!(out.report["charged_work"].as_u64().unwrap() > 0);
}
#[test]
fn continuous_chart_covers_full_extracted_thick_shell_and_rims() {
    let scene = quad(false);
    let mesh = extract(&scene, (Vec3::new(-0.2, -0.2, -0.2), Vec3::new(1.2, 1.2, 0.2)), [10, 10, 8]);
    let out = perform(&scene, &mesh);
    geometry_preserved(&mesh, &out);
    assert!(out.positions.iter().any(|p| p.z > 0.07) && out.positions.iter().any(|p| p.z < -0.07));
    assert!(out.positions.iter().any(|p| p.x < 0.) && out.positions.iter().any(|p| p.x > 1.));
}
#[test]
fn planar_uv_seam_is_cut_on_composed_geometry_with_distinct_corner_values() {
    let scene = quad(true);
    let mesh = extract(&scene, (Vec3::new(-0.2, -0.2, -0.2), Vec3::new(1.2, 1.2, 0.2)), [10, 10, 8]);
    let out = perform(&scene, &mesh);
    geometry_preserved(&mesh, &out);
    assert_eq!(out.report["source_seam_edges"], 1);
    assert!(out.triangles.len() > mesh.triangles.len());
    let mut values = BTreeMap::<u32, Vec<[f64; 2]>>::new();
    for (triangle, uv) in out.triangles.iter().zip(&out.corner_uvs) {
        for i in 0..3 {
            values.entry(triangle[i]).or_default().push(uv[i]);
        }
    }
    assert!(
        values.values().any(|uv| uv.iter().any(|a| uv.iter().any(|b| (a[0] - b[0]).abs() > 0.4))),
        "UV seam was averaged away"
    );
}
#[test]
fn connected_cylindrical_unwrap_keeps_its_local_wrap_seam() {
    let count = 8usize;
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    let mut uv = Vec::new();
    for z in [0., 1.] {
        for i in 0..count {
            let theta = std::f64::consts::TAU * i as f64 / count as f64;
            vertices.push(Vec3::new(theta.cos() as f32, theta.sin() as f32, z));
        }
    }
    for i in 0..count {
        let j = (i + 1) % count;
        let (a, b, c, d) = (i as u32, j as u32, (j + count) as u32, (i + count) as u32);
        triangles.extend([[a, b, c], [a, c, d]]);
        let (u0, u1) = (i as f64 / count as f64, (i + 1) as f64 / count as f64);
        uv.extend([[[u0, 0.], [u1, 0.], [u1, 1.]], [[u0, 0.], [u1, 1.], [u0, 1.]]]);
    }
    let scene = scene(vertices, triangles, uv);
    let mesh = extract(&scene, (Vec3::new(-1.2, -1.2, -0.2), Vec3::new(1.2, 1.2, 1.2)), [14, 14, 10]);
    let out = perform(&scene, &mesh);
    geometry_preserved(&mesh, &out);
    assert_eq!(out.report["source_seam_edges"], 1);
    let mut values = BTreeMap::<u32, Vec<f64>>::new();
    for (triangle, uv) in out.triangles.iter().zip(&out.corner_uvs) {
        for i in 0..3 {
            values.entry(triangle[i]).or_default().push(uv[i][0]);
        }
    }
    assert!(
        values.values().any(|uv| uv.iter().copied().fold(f64::NEG_INFINITY, f64::max)
            - uv.iter().copied().fold(f64::INFINITY, f64::min)
            > 0.9),
        "connected wrap seam was averaged"
    );
}
#[test]
fn bounds_unrepresentable_uvs_and_unresolved_owner_changes_reject() {
    let scene = quad(false);
    let vertices = vec![Vec3::new(0.2, 0.2, 0.08), Vec3::new(0.8, 0.2, 0.08), Vec3::new(0.2, 0.8, 0.08)];
    let triangles = vec![[0, 1, 2]];
    let materials = BTreeMap::from([(scene.objects[0].mat, 0)]);
    let mut bounded = options();
    bounded.max_work = 1;
    assert!(transfer(&scene, &vertices, &triangles, &materials, bounded).err().unwrap().contains("work budget"));
    let mut huge = quad(false);
    bind(&mut huge, 0, vec![[[999999.97, 0.]; 3]; 2]);
    assert!(transfer(&huge, &vertices, &triangles, &materials, options())
        .err()
        .unwrap()
        .contains("represented in f32"));
    let mut mixed = quad(false);
    let material = mixed.material(Material::default());
    mixed.add(Object::new(Prim::Sphere { r: 0.2 }, Transform::at(Vec3::new(0.3, 0.3, 0.08)), material));
    let materials = mixed.objects.iter().enumerate().map(|(i, o)| (o.mat, i)).collect();
    let mut bounded = options();
    bounded.max_refinement_passes = 0;
    assert!(transfer(&mixed, &vertices, &triangles, &materials, bounded).err().unwrap().contains("material-owner"));
}
#[test]
fn exact_material_owner_ties_and_selection_remapping_match_the_field() {
    for combine in [Combine::Union, Combine::Smooth(0.0)] {
        let mut scene = quad(false);
        let other = scene.material(Material::default());
        let mut object = scene.objects[0];
        object.mat = other;
        object.combine = combine;
        scene.add(object);
        bind(&mut scene, 1, vec![[[0.8, 0.8]; 3]; 2]);
        let p = Vec3::new(0.2, 0.2, 0.08);
        let (field, owner) = scene.sample_authored_owner(p);
        assert_eq!(field.mat, scene.objects[0].mat);
        assert_eq!(owner, Some(0));
        let counted = CountedSceneField::new(&scene).unwrap().sample_owner(p, 1000).unwrap();
        assert_eq!(counted.owner, owner);
        assert_eq!(counted.field.mat, field.mat);
        let old = scene.objects[1];
        scene.objects = vec![old];
        scene.appearance.retain_objects(&[1]).unwrap();
        scene.validate_appearance().unwrap();
        assert_eq!(scene.sample_material(p, 0.0).unwrap().uv, Some([0.8, 0.8]));
        scene.appearance.remove_binding(0);
        assert!(scene.appearance.is_empty());
    }
}

#[test]
fn seed_and_only_subtractors_have_the_same_exact_owner_without_repeated_field_queries() {
    let mut scene = quad(false);
    for position in [Vec3::new(0.3, 0.3, 0.), Vec3::new(0.7, 0.7, 0.)] {
        let material = scene.material(Material::default());
        scene.add(Object::new(Prim::Sphere { r: 0.16 }, Transform::at(position), material).subtract());
    }
    let field = CountedSceneField::new(&scene).unwrap();
    for p in [Vec3::ZERO, Vec3::new(0.3, 0.3, 0.), Vec3::new(0.5, 0.5, 0.08), Vec3::splat(10.)] {
        let owner = field.sample_owner(p, 1000).unwrap();
        assert_eq!(owner.owner, Some(0));
        assert_eq!(owner.field.mat, scene.objects[0].mat);
    }
    let mesh = extract(&scene, (Vec3::new(-0.2, -0.2, -0.2), Vec3::new(1.2, 1.2, 0.2)), [14, 14, 8]);
    let out = perform(&scene, &mesh);
    geometry_preserved(&mesh, &out);
    assert_eq!(out.report["owner_sampling"], "structural_seed_with_subtractors");
    assert!(out.material_ids.iter().all(|&material| material == 0));
}

#[test]
fn intentional_multi_tile_affine_uv_winding_is_not_shortest_unwrapped() {
    let mut scene = quad(false);
    bind(
        &mut scene,
        0,
        vec![
            [[0.125, 0.125], [3000.125, 0.125], [3000.125, 1.125]],
            [[0.125, 0.125], [3000.125, 1.125], [0.125, 1.125]],
        ],
    );
    let positions = vec![Vec3::new(0., 0., 0.08), Vec3::new(1., 0., 0.08), Vec3::new(0., 1., 0.08)];
    let out =
        transfer(&scene, &positions, &[[0, 1, 2]], &BTreeMap::from([(scene.objects[0].mat, 0)]), options()).unwrap();
    assert_eq!(out.corner_uvs, vec![[[0.125, 0.125], [3000.125, 0.125], [0.125, 1.125]]]);
    assert_eq!(out.report["periodic_rebased_corners"], 0);
    let image = &scene.surface_texture(0).unwrap().maps.albedo.as_ref().unwrap().image;
    let sampler = Sampler::default();
    assert_ne!(
        image.sample([0.125 + 3000.0 * 0.1234567, 0.4], 0., sampler).unwrap(),
        image.sample([0.125, 0.4], 0., sampler).unwrap()
    );
}
