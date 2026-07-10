//! Mesh ingestion end to end: generate a tube mesh, round-trip it through OBJ text, bake it into
//! a signed-distance volume, cache that volume as `.sdfv`, and render it as `Prim::Volume`.
//!
//! Run: cargo run -p mm3e-orchestrator --example mesh_demo --release

use mm3e_kit::color::Material;
use mm3e_kit::vec::{Transform, Vec3};
use mm3e_kit::volume::SdfVolume;
use mm3e_orchestrator::mesh::{bake_sdf, parse_obj, Mesh};
use mm3e_orchestrator::{orbit_camera, render, Light, Object, Prim, Scene};

fn torus_knot(p: f32, q: f32, segments: u32, ring_verts: u32, tube_r: f32) -> Mesh {
    let curve = |t: f32| {
        let r = 1.2 + 0.5 * (q * t).cos();
        Vec3::new(r * (p * t).cos(), 0.5 * (q * t).sin(), r * (p * t).sin())
    };
    let mut mesh = Mesh::default();
    let mut normal = Vec3::new(0.0, 1.0, 0.0);
    for s in 0..segments {
        let t = std::f32::consts::TAU * s as f32 / segments as f32;
        let tangent = (curve(t + 1e-3) - curve(t - 1e-3)).normalize();
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

fn load_or_bake_sdf(mesh: &Mesh, cache_path: &std::path::Path) -> SdfVolume {
    match SdfVolume::load_sdfv(cache_path) {
        Ok(vol) => {
            let (nx, ny, nz) = vol.dims;
            println!("loaded cached {}x{}x{} SDF volume from {}", nx, ny, nz, cache_path.display());
            vol
        }
        Err(load_err) => {
            println!("cache miss for {} ({load_err}); baking", cache_path.display());
            let t0 = std::time::Instant::now();
            let vol = bake_sdf(mesh, 160, 0.35).expect("bake");
            let (nx, ny, nz) = vol.dims;
            println!("baked {nx}x{ny}x{nz} SDF volume in {:.2}s", t0.elapsed().as_secs_f64());
            vol.save_sdfv(cache_path).expect("write sdfv cache");
            println!("cached baked SDF volume at {}", std::fs::canonicalize(cache_path).unwrap().display());
            vol
        }
    }
}

fn report_lipschitz(vol: &SdfVolume) {
    let (nx, ny, nz) = vol.dims;
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
}

fn main() {
    let knot = torus_knot(2.0, 3.0, 240, 36, 0.22);
    let obj_text = to_obj(&knot);
    let obj_path = std::path::Path::new("torus_knot.obj");
    std::fs::write(obj_path, &obj_text).expect("write obj");
    let loaded = parse_obj(&obj_text).expect("parse generated OBJ");
    println!("torus_knot.obj: {} vertices, {} triangles", loaded.positions.len(), loaded.triangles.len());

    let cache_path = std::path::Path::new("torus_knot.sdfv");
    let vol = load_or_bake_sdf(&loaded, cache_path);
    report_lipschitz(&vol);

    let mut scene = Scene::new(960, 540);
    scene.aa = 2;
    scene.bounces = 1;
    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.55));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.25)).metallic(1.0).roughness(0.32).reflective(0.18));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.18, 0.2)).roughness(0.3));
    let vid = scene.volume(vol);
    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Volume { id: vid }, Transform::at(Vec3::new(0.0, 1.15, 0.0)), gold));
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
