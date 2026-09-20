use mm3e_kit::{
    surface::TriangleSurface,
    vec::{Mat3, Transform, Vec3},
    Camera, Material,
};
use mm3e_orchestrator::{render_film_with_threads, Lens, Object, Prim, Scene};

fn sheet(bend: bool) -> TriangleSurface {
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    let n = 12;
    for y in 0..=n {
        for x in 0..=n {
            let (x, y) = (x as f32 / n as f32 * 1.6 - 0.8, y as f32 / n as f32 * 1.6 - 0.8);
            let z = if bend { 0.2 * (4.0 * x).cos() + 0.15 * (3.0 * y).sin() } else { 0.0 };
            vertices.push(Vec3::new(x, y, z));
        }
    }
    for y in 0..n {
        for x in 0..n {
            let a = y * (n + 1) + x;
            triangles.extend([[a, a + 1, a + n + 2], [a, a + n + 2, a + n + 1]]);
        }
    }
    TriangleSurface::new(vertices, triangles, 0.006).unwrap()
}

#[test]
fn surface_registration_preserves_topology_material_placement_and_failure_atomicity() {
    let mut scene = Scene::new(24, 24);
    let invalid = TriangleSurface::new(vec![Vec3::ZERO; 3], vec![[0, 1, 2]], 0.01).and_then(|s| scene.surface(s));
    assert!(invalid.is_err());
    assert!(scene.surfaces.is_empty() && scene.objects.is_empty());
    let surface = sheet(false);
    let expected_vertices = surface.vertices().to_vec();
    let expected_triangles = surface.triangles().to_vec();
    let id = scene.surface(surface).unwrap();
    let xform = Transform::new(Vec3::new(2.0, 0.5, -1.0), Mat3::from_euler(0.3, 0.6, -0.2), 1.5);
    scene.add(Object::new(Prim::Surface { id }, xform, 17));
    assert!(!scene.is_dual_safe(), "triangle surfaces require the explicit finite-difference normal path");
    let clone = scene.clone();
    assert_eq!(clone.surfaces[0].vertices(), expected_vertices);
    assert_eq!(clone.surfaces[0].triangles(), expected_triangles);
    for (local, expected) in
        [(Vec3::ZERO, -0.009), (Vec3::new(0.2, -0.3, 0.106), 0.15), (Vec3::new(0.2, -0.3, -0.106), 0.15)]
    {
        let point = xform.to_world(local);
        let authored = scene.sample_authored(point);
        assert!((authored.dist - expected).abs() < 1e-6);
        assert_eq!(authored.mat, 17);
        assert_eq!(scene.sample_object(0, point).unwrap().dist, authored.dist);
        assert_eq!(clone.sample_authored(point).dist, authored.dist);
    }
    let camera = Camera::look_at(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 0.8);
    assert!(mm3e_orchestrator::scene_io::try_serialize(&scene, &camera).unwrap_err().contains("triangle surfaces"));
}

#[test]
fn surface_world_bvh_preserves_union_order_and_scalar_reference() {
    let mut scene = Scene::new(8, 8);
    let id = scene.surface(sheet(true)).unwrap();
    for index in 0..9 {
        scene.add(Object::new(Prim::Surface { id }, Transform::at(Vec3::new(index as f32 * 1.9, 0.0, 0.0)), index));
    }
    scene.add(Object::new(Prim::Sphere { r: 0.08 }, Transform::at(Vec3::new(2.0, 0.0, 0.1)), 12).smooth(0.02));
    scene.add(Object::new(Prim::Sphere { r: 0.1 }, Transform::at(Vec3::new(4.0, 0.2, 0.1)), 12).subtract());
    let accelerated = scene.field();
    let reference = scene.world_linear();
    for x in -10..180 {
        for z in -5..=5 {
            let point = Vec3::new(x as f32 * 0.1, 0.123, z as f32 * 0.1);
            let (a, b) = (accelerated(point), reference(point));
            assert!((a.dist - b.dist).abs() < 1e-6, "{point:?}: {} != {}", a.dist, b.dist);
            assert_eq!(a.mat, b.mat);
        }
    }
}

#[test]
fn actual_bent_cloth_shell_renders_without_holes_and_changes_depth_and_normals() {
    let mut scene = Scene::new(41, 41);
    scene.aa = 2;
    scene.ao = false;
    scene.shadows = false;
    scene.bounces = 0;
    scene.fog_density = 0.0;
    scene.ambient = Vec3::ZERO;
    scene.sky_ambient = Vec3::ZERO;
    scene.marcher.eps = 0.00001;
    scene.marcher.normal_h = 0.0002;
    scene.marcher.max_steps = 512;
    let emission = Vec3::new(0.25, 1.5, 0.08);
    let mat = scene.material(Material { albedo: Vec3::ZERO, emissive: emission, ..Material::default() });
    let id = scene.surface(sheet(false)).unwrap();
    scene.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, mat));
    let camera = Camera::look_at(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 0.8);
    let flat = render_film_with_threads(&scene, &camera, &Lens::default(), 1);
    scene.surfaces[0] = sheet(true);
    let bent = render_film_with_threads(&scene, &camera, &Lens::default(), 3);
    let cloned = render_film_with_threads(&scene.clone(), &camera, &Lens::default(), 1);
    assert_eq!(bent, cloned);
    assert!(bent.alpha.contains(&0.0) && bent.alpha.contains(&1.0));
    assert!(bent.alpha.iter().any(|a| *a > 0.0 && *a < 1.0));
    assert!(
        bent.depth
            .iter()
            .zip(&flat.depth)
            .filter(|(a, b)| a.is_finite() && b.is_finite() && (*a - *b).abs() > 0.02)
            .count()
            > 300
    );
    assert!(bent.normals.iter().zip(&flat.normals).filter(|(a, b)| (**a - **b).length() > 0.1).count() > 300);
    for y in 10..31 {
        for x in 10..31 {
            let index = y * 41 + x;
            assert_eq!(bent.alpha[index], 1.0, "continuous garment interior has a hole at ({x},{y})");
            assert_eq!(bent.material_ids[index], mat);
            assert_eq!(bent.foreground[index], emission);
            assert!(bent.depth[index].is_finite());
            assert!((bent.normals[index].length() - 1.0).abs() < 1e-4);
        }
    }
    // A back view hits the second physical side; no parity-sign voxel bake is involved.
    let back_camera = Camera::look_at(Vec3::new(0.0, 0.0, -3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 0.8);
    let back = render_film_with_threads(&scene, &back_camera, &Lens::default(), 2);
    let center = 20 * 41 + 20;
    assert_eq!(back.alpha[center], 1.0);
    assert!(back.normals[center].z < -0.5 && bent.normals[center].z > 0.5);
}
