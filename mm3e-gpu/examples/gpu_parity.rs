//! CPU/GPU image parity probe for the common beauty path.
//!
//! Run: cargo run -p mm3e-gpu --example gpu_parity --release
//!      MM3E_GPU_ADAPTER=arc cargo run -p mm3e-gpu --example gpu_parity --release

use mm3e_gpu::GpuRenderer;
use mm3e_kit::color::Material;
use mm3e_kit::vec::{Transform, Vec3};
use mm3e_orchestrator::{mesh, orbit_camera, render, Light, Object, Prim, Scene};

const W: u32 = 320;
const H: u32 = 180;

fn base_scene() -> Scene {
    let mut scene = Scene::new(W, H);
    scene.aa = 1;
    scene.bounces = 1;
    scene.post.bloom = false;
    scene
}

fn analytic_scene() -> Scene {
    let mut scene = base_scene();
    let floor = scene.material(Material::solid(Vec3::splat(0.9)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.2, 0.2)).roughness(0.3));
    let metal =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.25)).metallic(1.0).roughness(0.22).reflective(0.35));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 0.9 }, Transform::at(Vec3::new(-1.1, 0.9, 0.0)), red));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.55), radius: 0.15 },
        Transform::at(Vec3::new(1.0, 0.7, 0.0)),
        metal,
    ));
    scene.sun_dir = Vec3::new(0.55, 0.7, 0.35).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.5)).soft(0.04));
    scene
}

fn uv_sphere_mesh(rings: u32, segments: u32, radius: f32) -> mesh::Mesh {
    let mut m = mesh::Mesh::default();
    m.positions.push(Vec3::new(0.0, radius, 0.0));
    for r in 1..rings {
        let phi = std::f32::consts::PI * r as f32 / rings as f32;
        for s in 0..segments {
            let theta = std::f32::consts::TAU * s as f32 / segments as f32;
            m.positions.push(Vec3::new(
                radius * phi.sin() * theta.cos(),
                radius * phi.cos(),
                radius * phi.sin() * theta.sin(),
            ));
        }
    }
    m.positions.push(Vec3::new(0.0, -radius, 0.0));
    let ring = |r: u32, s: u32| 1 + (r - 1) * segments + (s % segments);
    for s in 0..segments {
        m.triangles.push([0, ring(1, s + 1), ring(1, s)]);
    }
    for r in 1..rings - 1 {
        for s in 0..segments {
            let (a, b, c, d) = (ring(r, s), ring(r, s + 1), ring(r + 1, s + 1), ring(r + 1, s));
            m.triangles.push([a, b, c]);
            m.triangles.push([a, c, d]);
        }
    }
    let south = (m.positions.len() - 1) as u32;
    for s in 0..segments {
        m.triangles.push([south, ring(rings - 1, s), ring(rings - 1, s + 1)]);
    }
    m
}

fn mesh_volume_scene() -> Scene {
    let vol = mesh::bake_sdf(&uv_sphere_mesh(18, 24, 0.85), 44, 0.3).expect("bake");
    let mut scene = base_scene();
    let floor = scene.material(Material::solid(Vec3::splat(0.9)).checkered().roughness(0.55));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.2, 0.2)).roughness(0.35));
    let vid = scene.volume(vol);
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Volume { id: vid }, Transform::at(Vec3::new(0.0, 0.9, 0.0)), red));
    scene.sun_dir = Vec3::new(0.5, 0.75, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.35)).soft(0.04));
    scene
}

fn compare(name: &str, scene: &Scene, r: &GpuRenderer) {
    let camera = orbit_camera(Vec3::new(0.0, 0.75, 0.0), 5.5, 0.5, 0.28, 50f32.to_radians());
    let cpu = render(scene, &camera).to_u32(mm3e_kit::color::Rgba::rgb8(0, 0, 0));
    let gpu = r.compile(scene, W, H).render_rgba(r, &camera);

    let mut sum = 0u64;
    let mut max = 0u32;
    for (i, c) in cpu.iter().enumerate() {
        let cr = (c >> 16 & 0xff) as i32;
        let cg = (c >> 8 & 0xff) as i32;
        let cb = (c & 0xff) as i32;
        for d in [
            (cr - gpu[i * 4] as i32).unsigned_abs(),
            (cg - gpu[i * 4 + 1] as i32).unsigned_abs(),
            (cb - gpu[i * 4 + 2] as i32).unsigned_abs(),
        ] {
            sum += d as u64;
            max = max.max(d);
        }
    }
    let channels = (cpu.len() * 3) as f64;
    println!("{name:<10} mean {:>6.3}/255   max {max:>3}", sum as f64 / channels);
}

fn main() {
    let r = match GpuRenderer::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("GPU init failed: {e}");
            std::process::exit(1);
        }
    };
    println!("CPU reference vs GPU ({}) at {W}x{H}", r.adapter_name());
    compare("analytic", &analytic_scene(), &r);
    compare("mesh-vol", &mesh_volume_scene(), &r);
}
