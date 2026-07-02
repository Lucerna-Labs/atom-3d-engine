//! CPU↔GPU image parity, measured — not asserted. Renders the same scenes through the CPU
//! reference path (`mm3e_orchestrator::render`, bloom off so the post stacks match) and the
//! WGSL path, then reports per-channel differences. The scenes are chosen to expose the places
//! the two shading paths can drift: emissive + reflective materials, dense fog seen in mirrors,
//! and the standard PBR benchmark scene.
//!
//! Run: cargo run -p mm3e-gpu --example gpu_parity --release
//!      MM3E_GPU_ADAPTER=arc cargo run -p mm3e-gpu --example gpu_parity --release

use mm3e_gpu::GpuRenderer;
use mm3e_kit::color::Material;
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};

const W: u32 = 480;
const H: u32 = 270;

fn base_scene() -> Scene {
    let mut scene = Scene::new(W, H);
    scene.aa = 2;
    scene.bounces = 2;
    scene.post.bloom = false; // the GPU path has no bloom; compare the common pipeline
    scene
}

/// The cpu_vs_gpu benchmark scene: PBR metals, smooth union, soft sun.
fn pbr_scene() -> Scene {
    let mut scene = base_scene();
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().specular(0.15).roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.18, 0.20)).specular(0.7).roughness(0.25));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.25)).metallic(1.0).roughness(0.18).reflective(0.5));
    let mirror = scene.material(Material::solid(Vec3::splat(0.95)).metallic(1.0).roughness(0.05).reflective(0.85));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-2.4, 1.0, 0.2)), mirror));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.85), radius: 0.18 },
        Transform::at(Vec3::new(0.2, 0.95, -0.6)).rotated(Mat3::from_euler(0.0, 0.6, 0.0)),
        gold,
    ));
    scene.add(Object::new(Prim::Sphere { r: 0.7 }, Transform::at(Vec3::new(0.9, 0.7, 1.9)), red));
    scene.sun_dir = Vec3::new(0.55, 0.7, 0.35).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.25, 1.12, 0.95).scale(1.6)).soft(0.04));
    scene.light(Light::directional(Vec3::new(-0.4, 0.5, -0.7), Vec3::new(0.35, 0.42, 0.6)));
    scene
}

/// Emissive + reflective: a glowing sphere next to (and reflected in) a mirror. Exposes the
/// emissive-vs-Fresnel blend order — the CPU adds emissive after the reflection blend, so a
/// glowing reflective surface never dims at grazing angles.
fn emissive_scene() -> Scene {
    let mut scene = base_scene();
    let floor = scene.material(Material::solid(Vec3::splat(0.9)).checkered().roughness(0.5));
    let glow = scene.material(
        Material::solid(Vec3::new(0.9, 0.35, 0.1)).emissive(Vec3::new(4.0, 1.4, 0.3)).reflective(0.6).roughness(0.2),
    );
    let mirror = scene.material(Material::solid(Vec3::splat(0.95)).metallic(1.0).roughness(0.04).reflective(0.9));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 0.8 }, Transform::at(Vec3::new(-1.0, 0.8, 0.0)), glow));
    scene.add(Object::new(
        Prim::Box { half: Vec3::new(0.1, 1.4, 2.0) },
        Transform::at(Vec3::new(1.6, 1.4, 0.0)).rotated(Mat3::from_euler(0.0, -0.35, 0.0)),
        mirror,
    ));
    scene.sun_dir = Vec3::new(0.5, 0.75, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.2)));
    scene
}

/// Dense fog + mirrors: reflected legs must be fogged by BOTH path segments (the CPU recursion
/// fogs the reflected radiance along the primary leg too). Exposes under-fogged reflections.
fn foggy_scene() -> Scene {
    let mut scene = base_scene();
    scene.fog_density = 0.12;
    let floor = scene.material(Material::solid(Vec3::splat(0.85)).checkered().roughness(0.6));
    let mirror = scene.material(Material::solid(Vec3::splat(0.95)).metallic(1.0).roughness(0.05).reflective(0.85));
    let red = scene.material(Material::solid(Vec3::new(0.8, 0.15, 0.15)).roughness(0.3));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-1.4, 1.0, 0.0)), mirror));
    scene.add(Object::new(Prim::Sphere { r: 0.8 }, Transform::at(Vec3::new(1.4, 0.8, -2.5)), red));
    scene.add(Object::new(Prim::Torus { major: 0.8, minor: 0.25 }, Transform::at(Vec3::new(2.2, 0.9, 2.0)), red));
    scene.sun_dir = Vec3::new(0.5, 0.7, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.4)).soft(0.05));
    scene
}

fn compare(name: &str, scene: &Scene, r: &GpuRenderer) {
    let camera = orbit_camera(Vec3::new(0.2, 0.85, 0.2), 7.5, 0.5, 0.3, 50f32.to_radians());

    let cpu_fb = render(scene, &camera);
    let cpu = cpu_fb.to_u32(mm3e_kit::color::Rgba::rgb8(0, 0, 0));

    let gpu_scene = r.compile(scene, W, H);
    let gpu = gpu_scene.render_rgba(r, &camera);

    let n = (W * H) as usize;
    let (mut sum, mut max, mut over4) = (0u64, 0u32, 0u32);
    for i in 0..n {
        let c = cpu[i];
        let (cr, cg, cb) = ((c >> 16 & 0xFF) as i32, (c >> 8 & 0xFF) as i32, (c & 0xFF) as i32);
        let (gr, gg, gb) = (gpu[i * 4] as i32, gpu[i * 4 + 1] as i32, gpu[i * 4 + 2] as i32);
        for d in [(cr - gr).unsigned_abs(), (cg - gg).unsigned_abs(), (cb - gb).unsigned_abs()] {
            sum += d as u64;
            max = max.max(d);
            if d > 4 {
                over4 += 1;
            }
        }
    }
    let channels = (n * 3) as f64;
    println!(
        "{name:<10}  mean {:>6.3}/255   max {max:>3}   >4/255: {:>6.3}% of channels",
        sum as f64 / channels,
        over4 as f64 / channels * 100.0
    );
}

/// The pbr scene with a plain (non-checkered) floor — the parity floor with the checker's
/// hash-tint (now also ported to WGSL) out of the picture entirely.
fn pbr_plain_scene() -> Scene {
    let mut scene = pbr_scene();
    scene.materials[1] = Material::solid(Vec3::splat(0.75)).specular(0.15).roughness(0.6);
    scene
}

/// A Cornell-style room with a baked GI volume: real one-bounce color bleed. The GPU samples
/// the same uploaded probe cubes the CPU interpolates — exposes the (previously missing) GI
/// term in the shader ambient.
fn gi_scene() -> Scene {
    let mut scene = base_scene();
    scene.fog_density = 0.0;
    let white = scene.material(Material::solid(Vec3::splat(0.85)).roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.75, 0.08, 0.08)).roughness(0.6));
    let blue = scene.material(Material::solid(Vec3::new(0.08, 0.10, 0.75)).roughness(0.6));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, white));
    scene.add(Object::new(
        Prim::Box { half: Vec3::new(0.15, 2.0, 3.0) },
        Transform::at(Vec3::new(-3.0, 2.0, 0.0)),
        red,
    ));
    scene.add(Object::new(
        Prim::Box { half: Vec3::new(0.15, 2.0, 3.0) },
        Transform::at(Vec3::new(3.0, 2.0, 0.0)),
        blue,
    ));
    scene.add(Object::new(Prim::Sphere { r: 0.9 }, Transform::at(Vec3::new(0.0, 0.9, 0.0)), white));
    scene.sun_dir = Vec3::new(0.2, 0.9, 0.3).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.6)).soft(0.05));
    scene.bake_gi((10, 6, 10), 5);
    scene
}

/// Shadows and AO switched OFF: the GPU previously ignored both toggles and rendered fully
/// shadowed + occluded regardless.
fn toggles_off_scene() -> Scene {
    let mut scene = pbr_scene();
    scene.shadows = false;
    scene.ao = false;
    scene
}

fn main() {
    let r = match GpuRenderer::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("GPU init failed: {e}");
            std::process::exit(1);
        }
    };
    println!("CPU (reference, bloom off) vs GPU ({}) at {W}x{H}, aa=2:", r.adapter_name());
    compare("pbr", &pbr_scene(), &r);
    compare("pbr-plain", &pbr_plain_scene(), &r);
    compare("emissive", &emissive_scene(), &r);
    compare("foggy", &foggy_scene(), &r);
    compare("gi-bake", &gi_scene(), &r);
    compare("no-shdw-ao", &toggles_off_scene(), &r);
}
