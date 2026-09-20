use mm3e_kit::{
    surface::TriangleSurface,
    vec::{Transform, Vec3},
};
use mm3e_orchestrator::{Object, Prim, Scene};

#[test]
fn gpu_rejects_native_surface_before_shader_generation_without_silent_substitution() {
    let mut scene = Scene::new(8, 8);
    let surface = TriangleSurface::new(
        vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
        vec![[0, 1, 2]],
        0.01,
    )
    .unwrap();
    let id = scene.surface(surface).unwrap();
    scene.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, 0));
    let error = mm3e_gpu::wgsl::build_shader_checked(&scene).unwrap_err();
    assert!(error.contains("native triangle surfaces") && error.contains("CPU renderer"));
    assert!(std::panic::catch_unwind(|| mm3e_gpu::wgsl::build_shader(&scene)).is_err());
    scene.objects[0].prim = Prim::Sphere { r: 1.0 };
    assert!(
        mm3e_gpu::wgsl::build_shader_checked(&scene).is_ok(),
        "unused native geometry need not exclude analytic GPU rendering"
    );
}

#[test]
#[ignore = "requires an actual wgpu adapter; run with --ignored --nocapture"]
fn checked_gpu_compile_rejects_surface_and_retains_supported_renderer_operation() {
    let renderer = mm3e_gpu::GpuRenderer::new().expect("a real GPU adapter is required for checked compile validation");
    println!("Native surface rejection adapter: {}", renderer.adapter_name());
    let mut scene = Scene::new(8, 8);
    let surface = TriangleSurface::new(
        vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
        vec![[0, 1, 2]],
        0.01,
    )
    .unwrap();
    let id = scene.surface(surface).unwrap();
    scene.add(Object::new(Prim::Surface { id }, Transform::IDENTITY, 0));
    assert!(renderer.compile_checked(&scene, 8, 8).err().unwrap().contains("native triangle surfaces"));
    assert!(renderer.compile_checked(&scene, 0, 8).err().unwrap().contains("out of range"));
    scene.objects[0].prim = Prim::Sphere { r: 1.0 };
    let compiled = renderer.compile_checked(&scene, 8, 8).unwrap();
    let camera = mm3e_kit::Camera::look_at(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 0.8);
    let rgba = compiled.render_rgba(&renderer, &camera);
    assert_eq!(rgba.len(), 8 * 8 * 4);
    assert!(rgba.chunks_exact(4).all(|pixel| pixel[3] == 255));
    assert!(rgba.chunks_exact(4).any(|pixel| pixel[0] != rgba[0] || pixel[1] != rgba[1] || pixel[2] != rgba[2]));
}
