//! Frame-generation demo: render a real G-buffer frame every K display frames and reproject cheap
//! fake frames in between as the camera orbits. Prints real-vs-fake timing and the effective
//! displayed FPS, and saves a real frame and a reprojected frame to BMP for a side-by-side look.
//!
//! Run: cargo run -p mm3e-orchestrator --example reproject --release

use mm3e_kit::color::{Material, Rgba};
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::reproject::{reproject, GFrame};
use mm3e_orchestrator::{orbit_camera, render_gbuffer, Light, Object, Prim, Scene};
use std::time::Instant;

fn scene() -> Scene {
    let mut scene = Scene::new(640, 360);
    scene.aa = 1;
    scene.bounces = 1;
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.2, 0.22)).roughness(0.3).specular(0.8));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.28)).metallic(1.0).roughness(0.2).reflective(0.5));
    let chrome = scene.material(Material::solid(Vec3::splat(0.92)).metallic(1.0).roughness(0.06).reflective(0.85));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-2.2, 1.0, 0.0)), chrome));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.85), radius: 0.18 },
        Transform::at(Vec3::new(0.1, 0.95, -0.4)).rotated(Mat3::from_euler(0.0, 0.6, 0.0)),
        gold,
    ));
    scene.add(Object::new(Prim::Torus { major: 0.85, minor: 0.3 }, Transform::at(Vec3::new(2.5, 1.0, 0.4)), red));
    scene.sun_dir = Vec3::new(0.5, 0.75, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.2, 1.12, 0.98).scale(2.0)).soft(0.04));
    scene.light(Light::sphere(Vec3::new(-3.0, 4.0, 3.0), Vec3::splat(30.0), 1.0));
    scene
}

fn main() {
    let scene = scene();
    let (w, h) = (scene.width, scene.height);
    let frames = 60u32;
    let real_every = 3u32; // 1 real + 2 reprojected

    let mut prev: Option<GFrame> = None;
    let (mut real_ms, mut real_n, mut fake_ms, mut fake_n) = (0.0f32, 0u32, 0.0f32, 0u32);

    let t_all = Instant::now();
    for f in 0..frames {
        let cam = orbit_camera(Vec3::new(0.2, 0.85, 0.4), 8.5, 0.55 + f as f32 * 0.02, 0.32, 50f32.to_radians());
        let t0 = Instant::now();
        // Fake (reprojected) frame, if this isn't a real-render slot and we have a frame to warp.
        if f % real_every != 0 {
            if let Some(p) = prev.as_ref() {
                let col = reproject(p, &cam);
                if f == 1 {
                    let fb = GFrame::color_to_framebuffer(w, h, &col);
                    std::fs::write("reproject_fake.bmp", fb.to_bmp(Rgba::rgb8(0, 0, 0))).unwrap();
                }
                fake_ms += t0.elapsed().as_secs_f32() * 1000.0;
                fake_n += 1;
                continue;
            }
        }
        // Real (raymarched) frame: render the G-buffer and store it for the next fake frames.
        let g = render_gbuffer(&scene, &cam);
        if f == 0 {
            let fb = GFrame::color_to_framebuffer(w, h, &g.color);
            std::fs::write("reproject_real.bmp", fb.to_bmp(Rgba::rgb8(0, 0, 0))).unwrap();
        }
        prev = Some(g);
        real_ms += t0.elapsed().as_secs_f32() * 1000.0;
        real_n += 1;
    }
    let total = t_all.elapsed().as_secs_f32();

    println!("reprojection demo @ {w}x{h}, 1 real + {} fake per cycle", real_every - 1);
    println!(
        "  real  frames: {real_n}  avg {:.1} ms  ({:.1} fps if all real)",
        real_ms / real_n as f32,
        real_n as f32 * 1000.0 / real_ms
    );
    println!(
        "  fake  frames: {fake_n}  avg {:.1} ms  ({:.0}x cheaper than a real frame)",
        fake_ms / fake_n.max(1) as f32,
        (real_ms / real_n as f32) / (fake_ms / fake_n.max(1) as f32)
    );
    println!("  displayed:    {frames} frames in {total:.2}s = {:.1} fps", frames as f32 / total);
    println!("wrote reproject_real.bmp and reproject_fake.bmp");
}
