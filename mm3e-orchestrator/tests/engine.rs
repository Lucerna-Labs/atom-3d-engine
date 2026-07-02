//! Integration tests for the orchestrator: scene serialization round-trips, render determinism,
//! a render smoke test, GI baking, and animation tracks. Run with `cargo test`.

use mm3e_kit::camera::Camera;
use mm3e_kit::color::Material;
use mm3e_kit::vec::{Quat, Transform, Vec3};
use mm3e_orchestrator::anim::{Easing, Track};
use mm3e_orchestrator::{orbit_camera, render, scene_io, Light, Object, Prim, RenderMode, Scene};

/// A small scene exercising a plane, several primitives, modifiers, and lights.
fn demo_scene() -> Scene {
    let mut scene = Scene::new(96, 64);
    scene.aa = 1;
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.2, 0.2)).roughness(0.3));
    let metal = scene.material(Material::solid(Vec3::new(0.9, 0.8, 0.4)).metallic(1.0).roughness(0.2).reflective(0.4));
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-1.2, 1.0, 0.0)), red).round(0.1));
    scene.add(Object::new(Prim::Torus { major: 0.7, minor: 0.25 }, Transform::at(Vec3::new(1.2, 1.0, 0.0)), metal));
    scene.add(
        Object::new(Prim::Box { half: Vec3::splat(0.6) }, Transform::at(Vec3::new(0.0, 0.6, 1.6)), red).onion(0.05),
    );
    scene.sun_dir = Vec3::new(0.5, 0.75, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::splat(2.0)).soft(0.04));
    scene.light(Light::sphere(Vec3::new(-3.0, 4.0, 3.0), Vec3::splat(20.0), 1.0));
    scene
}

fn cam() -> Camera {
    orbit_camera(Vec3::new(0.0, 0.8, 0.3), 7.0, 0.5, 0.3, 52f32.to_radians())
}

#[test]
fn scene_io_roundtrips_stably() {
    let mut scene = demo_scene();
    scene.mode = RenderMode::Steps; // a non-default mode must survive the round-trip
    let c = cam();
    let text = scene_io::serialize(&scene, &c);
    let (parsed, _) = scene_io::parse(&text).expect("parse");
    assert_eq!(parsed.objects.len(), scene.objects.len());
    assert_eq!(parsed.materials.len(), scene.materials.len());
    assert_eq!(parsed.lights.len(), scene.lights.len());
    assert_eq!(parsed.mode, RenderMode::Steps, "render mode lost on round-trip");
    // Re-serializing the parsed scene must reproduce the exact same text.
    let again = scene_io::serialize(&parsed, &c);
    assert_eq!(text, again, "serialization is not a stable round-trip");
}

#[test]
fn camera_roundtrips() {
    let scene = demo_scene();
    let c = cam();
    let text = scene_io::serialize(&scene, &c);
    let (_, parsed_cam) = scene_io::parse(&text).expect("parse");
    // The look-at basis + FOV must reconstruct from the serialized eye/target/up/fov.
    assert!((parsed_cam.eye - c.eye).length() < 1e-4);
    assert!((parsed_cam.forward - c.forward).length() < 1e-4);
    assert!((parsed_cam.up - c.up).length() < 1e-4);
    assert!((parsed_cam.fov_scale - c.fov_scale).abs() < 1e-4);
}

#[test]
fn scene_io_rejects_garbage() {
    assert!(scene_io::parse("size 10\n").is_err()); // missing height
    assert!(scene_io::parse("obj banana 1 2 3\n").is_err()); // unknown primitive
    assert!(scene_io::parse("wat 1 2 3\n").is_err()); // unknown directive
}

#[test]
fn render_is_deterministic() {
    let scene = demo_scene();
    let c = cam();
    let a = render(&scene, &c).to_bmp(mm3e_kit::color::Rgba::rgb8(0, 0, 0));
    let b = render(&scene, &c).to_bmp(mm3e_kit::color::Rgba::rgb8(0, 0, 0));
    assert_eq!(a, b, "render must be deterministic regardless of thread scheduling");
}

#[test]
fn render_smoke_produces_a_nontrivial_image() {
    let scene = demo_scene();
    let fb = render(&scene, &cam());
    let px = fb.to_u32(mm3e_kit::color::Rgba::rgb8(0, 0, 0));
    assert_eq!(px.len() as u32, scene.width * scene.height);
    // The image must not be a single flat color (geometry + floor are visible).
    let first = px[0];
    assert!(px.iter().any(|&p| p != first), "render is a flat color — nothing was drawn");
    // No NaN leaked through (all pixels are valid 0x00RRGGBB).
    assert!(px.iter().all(|&p| p <= 0x00FF_FFFF));
}

#[test]
fn checkerboard_render_fills_every_pixel() {
    use mm3e_orchestrator::render_checkerboard;
    let scene = demo_scene();
    let fb = render_checkerboard(&scene, &cam());
    let px = fb.to_u32(mm3e_kit::color::Rgba::rgb8(0, 0, 0));
    assert_eq!(px.len() as u32, scene.width * scene.height);
    assert!(px.iter().any(|&p| p != px[0]), "checkerboard render is flat");
    assert!(px.iter().all(|&p| p <= 0x00FF_FFFF));
}

#[test]
fn all_render_modes_run() {
    let mut scene = demo_scene();
    let c = cam();
    for mode in [
        RenderMode::Beauty,
        RenderMode::Normal,
        RenderMode::Depth,
        RenderMode::Ao,
        RenderMode::Steps,
        RenderMode::Albedo,
    ] {
        scene.mode = mode;
        let fb = render(&scene, &c);
        assert_eq!(fb.width * fb.height, scene.width * scene.height);
    }
}

#[test]
fn gi_bake_and_render() {
    let mut scene = demo_scene();
    scene.bake_gi((6, 5, 6), 3);
    assert!(scene.gi.is_some());
    let fb = render(&scene, &cam());
    let px = fb.to_u32(mm3e_kit::color::Rgba::rgb8(0, 0, 0));
    assert!(px.iter().any(|&p| p != px[0]));
}

#[test]
fn gi_volume_interpolates_a_known_field() {
    use mm3e_orchestrator::gi::GiVolume;
    // A radiance field that is exactly linear in x, independent of direction. Every cube face of
    // a probe then equals splat(probe.x), and trilinear interpolation should reproduce it exactly.
    let radiance = |o: Vec3, _d: Vec3| Vec3::splat(o.x);
    let vol = GiVolume::bake(Vec3::new(0.0, 0.0, 0.0), Vec3::new(4.0, 2.0, 4.0), (5, 3, 5), 1, &radiance);
    let n = Vec3::new(1.0, 0.0, 0.0);
    // At a probe center (x=2) and a cell midpoint (x=2.5), the ambient cube returns ~x.
    assert!((vol.sample(Vec3::new(2.0, 1.0, 2.0), n).x - 2.0).abs() < 1e-3);
    assert!((vol.sample(Vec3::new(2.5, 1.0, 2.0), n).x - 2.5).abs() < 1e-3);
}

#[test]
fn gi_degenerate_dims_dont_panic() {
    // A zero / one probe dimension must be clamped, not panic on an empty buffer or underflow.
    let radiance = |_o: Vec3, _d: Vec3| Vec3::splat(0.5);
    for dims in [(0, 5, 6), (1, 1, 1), (3, 0, 3)] {
        let vol = mm3e_orchestrator::gi::GiVolume::bake(Vec3::ZERO, Vec3::splat(4.0), dims, 1, &radiance);
        let _ = vol.sample(Vec3::splat(2.0), Vec3::new(0.0, 1.0, 0.0)); // must not panic
    }
}

#[test]
fn physics_body_rests_on_floor() {
    use mm3e_kit::sdf::Field;
    use mm3e_orchestrator::physics::{Body, PhysicsWorld};
    // Field of an infinite floor at y = 0 (distance = p.y, normal up).
    let field = |p: Vec3| Field::new(p.y, 0);
    let mut w = PhysicsWorld::new();
    let id = w.add(Body::new(Vec3::new(0.0, 5.0, 0.0), 0.5));
    for _ in 0..300 {
        w.step(1.0 / 60.0, &field); // 5 seconds — long enough to settle
    }
    let b = w.bodies[id];
    assert!((b.pos.y - 0.5).abs() < 0.06, "body should rest at radius height, got {}", b.pos.y);
    assert!(b.grounded);
    assert!(b.vel.length() < 0.5, "body should have settled, |v| = {}", b.vel.length());
}

#[test]
fn reprojection_identity_mostly_matches() {
    use mm3e_orchestrator::render_gbuffer;
    use mm3e_orchestrator::reproject::reproject;
    let scene = demo_scene();
    let c = cam();
    let g = render_gbuffer(&scene, &c, &[]);
    assert_eq!(g.color.len(), (scene.width * scene.height) as usize);
    // Reprojecting to the SAME camera should reproduce almost the whole frame (each pixel's world
    // point projects back to ~its own pixel).
    let out = reproject(&g, &c, &[]);
    let mut same = 0u32;
    for (i, px) in out.iter().enumerate() {
        if (px[0] as i32 - g.color[i][0] as i32).abs() < 24 {
            same += 1;
        }
    }
    let frac = same as f32 / out.len() as f32;
    assert!(frac > 0.85, "identity reprojection should mostly match, got {frac:.2}");
}

#[test]
fn reprojection_hybrid_beats_cheap_fill() {
    use mm3e_orchestrator::reproject::reproject;
    use mm3e_orchestrator::{render_gbuffer, reproject_hybrid};
    let scene = demo_scene();
    let c0 = orbit_camera(Vec3::new(0.0, 0.8, 0.3), 7.0, 0.5, 0.3, 52f32.to_radians());
    let c1 = orbit_camera(Vec3::new(0.0, 0.8, 0.3), 7.0, 0.62, 0.3, 52f32.to_radians()); // moved → holes
    let g = render_gbuffer(&scene, &c0, &[]);
    let truth = render_gbuffer(&scene, &c1, &[]).color; // ground truth at the new camera
    let basic = reproject(&g, &c1, &[]); // cheap row-fill
    let hybrid = reproject_hybrid(&g, &c1, &scene, &[]); // rerendered holes
    let err = |a: &[[u8; 4]], b: &[[u8; 4]]| -> u64 {
        a.iter().zip(b).map(|(p, q)| (p[0] as i32 - q[0] as i32).unsigned_abs() as u64).sum()
    };
    // Rerendering the holes must be at least as close to ground truth as smearing them.
    assert!(err(&hybrid, &truth) <= err(&basic, &truth), "hybrid reprojection should beat the cheap fill");
}

#[test]
fn reprojection_object_motion_has_effect() {
    use mm3e_orchestrator::render_gbuffer;
    use mm3e_orchestrator::reproject::reproject;
    // Tag the first sphere (centre -1.2,1,0, radius 1) as a moving object.
    let scene = demo_scene();
    let c = cam();
    let g = render_gbuffer(&scene, &c, &[(Vec3::new(-1.2, 1.0, 0.0), 1.3)]);
    assert!(g.obj.contains(&0), "the mover should tag some pixels");
    // Reprojecting with a non-zero object delta must differ from a zero delta (the object moves).
    let still = reproject(&g, &c, &[Vec3::ZERO]);
    let moved = reproject(&g, &c, &[Vec3::new(1.5, 0.0, 0.0)]);
    assert_ne!(still, moved, "object motion vectors should shift the moving object's pixels");
}

#[test]
fn quality_presets_and_lerp() {
    use mm3e_orchestrator::Quality;
    let fast = Quality::fast(320, 180);
    let full = Quality::full(960, 540);
    assert!(fast.aa <= full.aa && fast.bounces <= full.bounces && fast.shadow_steps < full.shadow_steps);
    // Lerp climbs the budgets and takes the target resolution.
    let mid = Quality::lerp(fast, full, 0.5);
    assert_eq!((mid.width, mid.height), (960, 540));
    assert!(mid.shadow_steps > fast.shadow_steps && mid.shadow_steps < full.shadow_steps);
    // apply() writes the knobs into a scene.
    let mut s = Scene::new(10, 10);
    full.apply(&mut s);
    assert_eq!((s.width, s.height, s.aa, s.bounces), (960, 540, 2, 2));
    assert_eq!((s.marcher.shadow_steps, s.marcher.ao_samples), (64, 5));
}

#[test]
fn particles_burst_move_and_expire() {
    use mm3e_orchestrator::particles::Particles;
    let mut p = Particles::new();
    p.burst(Vec3::new(0.0, 2.0, 0.0), 16, 4.0, Vec3::new(1.0, 0.8, 0.2), 7);
    assert_eq!(p.alive().len(), 16);
    let start = p.alive()[0].pos;
    p.update(0.1);
    assert_ne!(p.alive()[0].pos, start, "particles should move when updated");
    // After enough time every particle expires.
    for _ in 0..40 {
        p.update(0.1);
    }
    assert_eq!(p.alive().len(), 0, "particles should die after their lifetime");
}

#[test]
fn physics_resolves_penetration() {
    use mm3e_kit::sdf::Field;
    use mm3e_orchestrator::physics::{Body, PhysicsWorld};
    // A unit sphere obstacle at the origin; a body starts overlapping it.
    let field = |p: Vec3| Field::new(p.length() - 1.0, 0);
    let mut w = PhysicsWorld::new();
    w.gravity = Vec3::ZERO; // isolate the collision response
    let id = w.add(Body::new(Vec3::new(0.3, 0.0, 0.0), 0.5));
    w.step(1.0 / 60.0, &field);
    let b = w.bodies[id];
    let surface_dist = b.pos.length() - 1.0; // distance from the body centre to the obstacle surface
    assert!(surface_dist >= 0.5 - 0.05, "body should be pushed outside, got {surface_dist}");
}

#[test]
fn subitize_knob_cuts_field_evals_via_shipped_marcher() {
    // Validates the SHIPPED `Marcher::subitize` knob (numerical-cognition / ANS transfer) through
    // the real `Marcher::march`, under the corrected (unconditional) overlap guard. The leap's
    // honest niche is empty space, so the eval cut is asserted on a miss-heavy sky framing. On
    // hit-dominated framings the leaps that reach geometry trip the guard, retreat, and decay —
    // a small bounded cost instead of a saving. That price is the fix for real tunneling: the
    // old version of this test demanded an eval cut on the hit-dominated framing, and that "win"
    // came from unguarded leaps tunneling into surfaces and terminating marches early with hits
    // buried inside the geometry (see mm3e-kit's buried-hit regression test). Both framings must
    // keep the silhouette (no tunneling), and the hit-heavy cost must stay bounded.
    use mm3e_kit::atoms;
    use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
    fn count(scene: &Scene, c: &Camera, w: u32, h: u32) -> (u64, Vec<bool>) {
        let counter = AtomicU64::new(0);
        let base = scene.field();
        let cf = |p: Vec3| {
            counter.fetch_add(1, Relaxed);
            base(p)
        };
        let m = scene.marcher;
        let mut hits = Vec::with_capacity((w * h) as usize);
        for (x, y) in atoms::scan(w, h) {
            let ray = c.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
            hits.push(m.march(&cf, &ray).hit);
        }
        (counter.load(Relaxed), hits)
    }
    let mut scene = demo_scene();
    let (w, h) = (scene.width, scene.height);
    let agreement = |a: &[bool], b: &[bool]| a.iter().zip(b).filter(|(x, y)| x == y).count() as f32 / a.len() as f32;

    // Miss-heavy framing (mostly sky): the leap's home turf — it must cut field-evals.
    let sky = Camera::look_at(
        Vec3::new(0.0, 1.5, 8.0),
        Vec3::new(0.0, 4.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        52f32.to_radians(),
    );
    scene.marcher.subitize = 0.0;
    let (sky_e0, sky_h0) = count(&scene, &sky, w, h);
    scene.marcher.subitize = 0.4;
    let (sky_e1, sky_h1) = count(&scene, &sky, w, h);
    assert!(sky_e1 < sky_e0, "subitize=0.4 should cut field-evals on a miss-heavy framing: {sky_e0} -> {sky_e1}");
    let sky_agree = agreement(&sky_h0, &sky_h1);
    assert!(sky_agree > 0.98, "subitize must not flip many hits (no tunneling): agree={sky_agree:.4}");

    // Hit-dominated framing (the demo down-look): cost must stay small and bounded, silhouette kept.
    let c = cam();
    scene.marcher.subitize = 0.0;
    let (e0, h0) = count(&scene, &c, w, h);
    scene.marcher.subitize = 0.4;
    let (e1, h1) = count(&scene, &c, w, h);
    assert!(
        (e1 as f64) < (e0 as f64) * 1.15,
        "subitize's guarded cost on a hit-heavy framing must stay bounded: {e0} -> {e1}"
    );
    let agree = agreement(&h0, &h1);
    assert!(agree > 0.98, "subitize must not flip many hits (no tunneling): agree={agree:.4}");
}

#[test]
fn animation_tracks_sample() {
    let pos = Track::new(Easing::Linear).key(0.0, Vec3::new(0.0, 0.0, 0.0)).key(2.0, Vec3::new(2.0, 4.0, 0.0));
    assert_eq!(pos.sample(-1.0), Vec3::new(0.0, 0.0, 0.0)); // clamps to start
    assert_eq!(pos.sample(3.0), Vec3::new(2.0, 4.0, 0.0)); // clamps to end
    let mid = pos.sample(1.0);
    assert!((mid - Vec3::new(1.0, 2.0, 0.0)).length() < 1e-5);

    // Use a 90° turn (a 180° turn has an ambiguous midpoint). Halfway is 45° about +Y, which
    // maps +x → (cos45, 0, -sin45) ≈ (0.707, 0, -0.707) at t=1, and ~22.5° at t=0.5.
    let spin = Track::new(Easing::Linear)
        .key(0.0, Quat::IDENTITY)
        .key(1.0, Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), std::f32::consts::FRAC_PI_2));
    let end = spin.sample(1.0).to_mat3().mul_vec(Vec3::new(1.0, 0.0, 0.0));
    assert!((end - Vec3::new(0.0, 0.0, -1.0)).length() < 1e-3);
    // The slerp midpoint is a 45° rotation (quaternion half-angles): +x → (0.707, 0, -0.707).
    let mid = spin.sample(0.5).to_mat3().mul_vec(Vec3::new(1.0, 0.0, 0.0));
    assert!(mid.x > 0.65 && mid.z < -0.65 && mid.y.abs() < 1e-3);
}
