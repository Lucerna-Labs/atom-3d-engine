//! Mesh ingestion end to end: generate a torus-knot tube mesh, round-trip it through the OBJ
//! parser, bake it into a signed-distance volume, and render it — smooth-unioned with an
//! analytic sphere to show baked meshes participate in CSG like any other field.
//!
//! Run: cargo run -p mm3e-orchestrator --example mesh_demo --release

use mm3e_kit::vec::{Transform, Vec3};
use mm3e_kit::Material;
use mm3e_orchestrator::mesh::{bake_sdf, parse_obj, Mesh};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};

/// A watertight (p, q) torus-knot tube: parallel-transported ring frames along the closed
/// curve, closed by index wraparound.
fn torus_knot(p: f32, q: f32, segments: u32, ring_verts: u32, tube_r: f32) -> Mesh {
    let curve = |t: f32| {
        let r = 1.2 + 0.5 * (q * t).cos();
        Vec3::new(r * (p * t).cos(), 0.5 * (q * t).sin(), r * (p * t).sin())
    };
    let mut mesh = Mesh::default();
    // Parallel-transport a normal along the curve for stable, twist-minimal ring frames.
    let mut normal = Vec3::new(0.0, 1.0, 0.0);
    for s in 0..segments {
        let t = std::f32::consts::TAU * s as f32 / segments as f32;
        let dt = 1e-3;
        let tangent = (curve(t + dt) - curve(t - dt)).normalize();
        normal = (normal - tangent.scale(normal.dot(tangent))).normalize();
        let binormal = tangent.cross(normal);
        let center = curve(t);
        for v in 0..ring_verts {
            let a = std::f32::consts::TAU * v as f32 / ring_verts as f32;
            mesh.positions.push(center + normal.scale(tube_r * a.cos()) + binormal.scale(tube_r * a.sin()));
        }
    }
    let idx = |s: u32, v: u32| (s % segments) * ring_verts + (v % ring_verts);
    for s in 0..segments {
        for v in 0..ring_verts {
            let (a, b, c, d) = (idx(s, v), idx(s + 1, v), idx(s + 1, v + 1), idx(s, v + 1));
            mesh.triangles.push([a, b, c]);
            mesh.triangles.push([a, c, d]);
        }
    }
    mesh
}

/// Serialize a mesh as OBJ text (the writer half of the round trip).
fn to_obj(mesh: &Mesh) -> String {
    let mut s = String::from("# MM3E torus knot\n");
    for p in &mesh.positions {
        s.push_str(&format!("v {} {} {}\n", p.x, p.y, p.z));
    }
    for t in &mesh.triangles {
        s.push_str(&format!("f {} {} {}\n", t[0] + 1, t[1] + 1, t[2] + 1));
    }
    s
}

fn main() {
    // Generate → OBJ → parse → bake: the full ingestion path, exercised end to end. The tube is
    // tessellated finely enough (facet sagitta ~0.0004) that grazing soft shadows do not read
    // the facet ridges as surface mottling — a faceted input mesh bakes faceted, faithfully.
    let knot = torus_knot(2.0, 3.0, 360, 48, 0.22);
    let obj_text = to_obj(&knot);
    std::fs::write("torus_knot.obj", &obj_text).expect("write obj");
    let loaded = parse_obj(&obj_text).expect("re-parse generated OBJ");
    println!("torus_knot.obj: {} vertices, {} triangles", loaded.positions.len(), loaded.triangles.len());

    let t0 = std::time::Instant::now();
    let vol = bake_sdf(&loaded, 192, 0.35).expect("bake");
    let (nx, ny, nz) = vol.dims;
    println!("baked {nx}x{ny}x{nz} SDF volume in {:.2}s", t0.elapsed().as_secs_f64());

    // Bake sanity: a true SDF is 1-Lipschitz, so adjacent samples can differ by at most one
    // cell spacing. A sign-parity error (non-watertight input, tangent-row miscount) shows up
    // as a violation of roughly 2·|d|.
    let mut violations = 0u32;
    let mut worst = 0.0f32;
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx.saturating_sub(1) {
                let step = (vol.at(i + 1, j, k) - vol.at(i, j, k)).abs();
                if step > vol.cell.x * 1.02 {
                    violations += 1;
                    worst = worst.max(step / vol.cell.x);
                }
            }
        }
    }
    println!("Lipschitz check: {violations} adjacent-pair violations (worst {worst:.2}x cell)");

    let mut scene = Scene::new(960, 540);
    scene.aa = 2;
    scene.bounces = 1;
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.55));
    // A brushed (not mirror) metal: a sampled surface has cell-scale waviness, and a mirror
    // finish would render every ripple of it in the self-reflections.
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.25)).metallic(1.0).roughness(0.32).reflective(0.18));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.18, 0.2)).roughness(0.3));
    // A sampled field's gradient is per-cell trilinear: widen the normal stencil to cell scale
    // or the stencil reads that faceting as speckle (shadow/AO acne on the knot's surface).
    let cell = vol.cell.x.max(vol.cell.y).max(vol.cell.z);
    scene.marcher.normal_h = cell * 0.6;
    let vid = scene.volume(vol);
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Volume { id: vid }, Transform::at(Vec3::new(0.0, 1.15, 0.0)), gold));
    // A baked mesh is a field like any other: smooth-union an analytic sphere straight into it.
    scene.add(Object::new(Prim::Sphere { r: 0.5 }, Transform::at(Vec3::new(1.1, 0.75, 0.9)), red).smooth(0.25));
    scene.sun_dir = Vec3::new(0.5, 0.75, 0.35).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.3, 1.18, 1.0).scale(1.5)).soft(0.04));
    scene.light(Light::directional(Vec3::new(-0.5, 0.4, -0.6), Vec3::new(0.3, 0.38, 0.55)));

    let camera = orbit_camera(Vec3::new(0.0, 0.9, 0.0), 6.2, 0.6, 0.42, 50f32.to_radians());
    let t0 = std::time::Instant::now();
    let fb = render(&scene, &camera);
    println!("rendered in {:.2}s", t0.elapsed().as_secs_f64());
    let path = std::path::Path::new("mesh_demo.bmp");
    std::fs::write(path, fb.to_bmp(mm3e_kit::color::Rgba::rgb8(0, 0, 0))).expect("write bmp");
    println!("wrote {} ({}x{})", std::fs::canonicalize(path).unwrap().display(), scene.width, scene.height);
}
