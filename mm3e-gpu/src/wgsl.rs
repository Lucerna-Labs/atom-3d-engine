//! WGSL codegen — the heart of the GPU backend. Walks an `mm3e_orchestrator::Scene` and emits a
//! WGSL compute shader whose `map(p) -> vec2(dist, matId)` is the GPU twin of the CPU `world()`
//! closure. This is the doctrine, re-targeted: the same 8 atoms (fold over objects, project the
//! camera ray, compare via SDFs, combine the CSG/lighting) now expressed as shader text the GPU
//! runs thousands of lanes at once. The camera + time ride in a uniform so a real-time loop never
//! recompiles; the scene (objects, materials, lights, sky) is baked into the source.

use mm3e_kit::{csg::Expr, vec::Vec3};
use mm3e_orchestrator::{Combine, Object, Prim, Scene};

fn f(x: f32) -> String {
    // Always emit a decimal point so WGSL treats it as f32, and avoid literals WGSL rejects.
    if x.is_finite() {
        let s = format!("{x:?}");
        if s.contains('.') || s.contains('e') {
            s
        } else {
            format!("{s}.0")
        }
    } else if x == f32::INFINITY {
        "1e30".to_string()
    } else if x == f32::NEG_INFINITY {
        "-1e30".to_string()
    } else {
        "0.0".to_string()
    }
}

fn v3(v: Vec3) -> String {
    format!("vec3<f32>({}, {}, {})", f(v.x), f(v.y), f(v.z))
}

/// Build the full compute shader for `scene`.
pub fn build_shader(scene: &Scene) -> String {
    build_shader_checked(scene).expect("GPU shader cannot represent this scene; use the CPU renderer")
}

/// Check native geometry support before generating shader source. Triangle shells currently
/// use the CPU nearest-triangle BVH; they must never disappear or become volume approximations
/// on the GPU path. The compatibility `build_shader` wrapper fails explicitly for these scenes.
pub fn build_shader_checked(scene: &Scene) -> Result<String, String> {
    if !scene.appearance.is_empty() {
        return Err("UV textures require the checked CPU renderer".into());
    }
    if scene.objects.iter().any(|object| matches!(object.prim, Prim::Surface { .. })) {
        return Err("native triangle surfaces are not supported by the GPU backend; use the CPU renderer".into());
    }
    let mut s = String::new();
    s.push_str(&format!("const MAX_DYN: u32 = {}u;\n", crate::MAX_DYN));
    s.push_str(KERNEL_HEADER);

    // Scene constants.
    s.push_str(&format!("const SUN: vec3<f32> = {};\n", v3(scene.sun_dir)));
    s.push_str(&format!("const SKY_AMBIENT: vec3<f32> = {};\n", v3(scene.sky_ambient)));
    s.push_str(&format!("const AMBIENT: vec3<f32> = {};\n", v3(scene.ambient)));
    s.push_str(&format!("const FOG: vec3<f32> = {};\n", v3(scene.fog)));
    s.push_str(&format!("const FOG_DENSITY: f32 = {};\n", f(scene.fog_density)));
    s.push_str(&format!("const EXPOSURE: f32 = {};\n", f(scene.post.exposure)));
    let mr = &scene.marcher;
    s.push_str(&format!("const MARCH_STEPS: i32 = {};\n", mr.max_steps.min(i32::MAX as u32)));
    s.push_str(&format!("const MAX_DIST: f32 = {};\n", f(mr.max_dist)));
    s.push_str(&format!("const EPS_BASE: f32 = {};\n", f(mr.eps)));
    s.push_str(&format!("const LOD_FOOTPRINT: f32 = {};\n", f(mr.lod_footprint)));
    s.push_str(&format!("const STEP_SCALE: f32 = {};\n", f(mr.step_scale.min(1.0))));
    s.push_str(&format!("const NORMAL_H: f32 = {};\n", f(mr.normal_h)));
    s.push_str(&format!("const REFL_LIFT: f32 = {};\n", f((mr.normal_h * 2.0).max(0.02))));
    s.push_str(&format!("const SHADOW_STEPS: i32 = {};\n", mr.shadow_steps.min(i32::MAX as u32)));
    let ao_samples = if scene.ao { mr.ao_samples } else { 0 };
    s.push_str(&format!("const AO_SAMPLES: i32 = {};\n", ao_samples.min(i32::MAX as u32)));
    s.push_str(&format!("const AO_SPAN: f32 = {};\n", f((ao_samples.max(2) - 1) as f32)));
    s.push_str(&format!("const AO_BASE: f32 = {};\n", f(mr.normal_h.max(0.01))));
    match &scene.gi {
        Some(vol) => {
            let (min, size, dims, _) = vol.raw();
            let axis_scale = |ext: f32, n: usize| if ext.abs() > 1e-6 { (n.max(2) - 1) as f32 / ext } else { 0.0 };
            s.push_str("const HAS_GI: bool = true;\n");
            s.push_str(&format!("const GI_MIN: vec3<f32> = {};\n", v3(min)));
            s.push_str(&format!(
                "const GI_SCALE: vec3<f32> = vec3<f32>({}, {}, {});\n",
                f(axis_scale(size.x, dims.0)),
                f(axis_scale(size.y, dims.1)),
                f(axis_scale(size.z, dims.2))
            ));
            s.push_str(&format!("const GI_NX: u32 = {}u;\n", dims.0));
            s.push_str(&format!("const GI_NY: u32 = {}u;\n", dims.1));
            s.push_str(&format!("const GI_NZ: u32 = {}u;\n\n", dims.2));
        }
        None => {
            s.push_str("const HAS_GI: bool = false;\n");
            s.push_str("const GI_MIN: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);\n");
            s.push_str("const GI_SCALE: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);\n");
            s.push_str("const GI_NX: u32 = 1u;\nconst GI_NY: u32 = 1u;\nconst GI_NZ: u32 = 1u;\n\n");
        }
    }

    s.push_str(&csg_functions(scene));
    s.push_str(&material_fn(scene));
    s.push_str(&map_fn(scene));
    s.push_str(&direct_lighting_fn(scene));
    s.push_str(KERNEL_BODY);
    Ok(s)
}

/// Emit one bounded helper per node, keeping source size linear even when transforms are
/// nested. Helpers evaluate local scalar distances; the enclosing Object supplies material.
fn csg_functions(scene: &Scene) -> String {
    fn emit(expr: &Expr, id: usize, next: &mut usize, out: &mut String) -> String {
        let name = format!("csg_{id}_{}", *next);
        *next += 1;
        let mut prelude = String::new();
        let body = match expr {
            Expr::Sphere { r } => format!("sd_sphere(p, {})", f(*r)),
            Expr::Box { half } => format!("sd_box(p, {})", v3(*half)),
            Expr::RoundBox { half, radius } => format!("sd_round_box(p, {}, {})", v3(*half), f(*radius)),
            Expr::Torus { major, minor } => format!("sd_torus(p, {}, {})", f(*major), f(*minor)),
            Expr::Cylinder { h, r } => format!("sd_cylinder(p, {}, {})", f(*h), f(*r)),
            Expr::Capsule { a, b, r } => format!("sd_capsule(p, {}, {}, {})", v3(*a), v3(*b), f(*r)),
            Expr::Cone { r1, r2, h } => format!("sd_round_cone(p, {}, {}, {})", f(*r1), f(*r2), f(*h)),
            Expr::Ellipsoid { r } => format!("sd_ellipsoid(p, {})", v3(*r)),
            Expr::Octahedron { s } => format!("sd_octahedron(p, {})", f(*s)),
            Expr::HexPrism { r, h } => format!("sd_hex_prism(p, {}, {})", f(*r), f(*h)),
            Expr::Plane { n, h } => format!("sd_plane(p, {}, {})", v3(*n), f(*h)),
            Expr::Transform { shape, xform } => {
                let child = emit(shape, id, next, out);
                let r = xform.rot.cols;
                prelude.push_str(&format!("  let R = mat3x3<f32>({}, {}, {});\n", v3(r[0]), v3(r[1]), v3(r[2])));
                prelude.push_str(&format!(
                    "  let q = transpose(R) * (p - {}) * {};\n",
                    v3(xform.pos),
                    f(1.0 / xform.scale)
                ));
                format!("{child}(q) * {}", f(xform.scale))
            }
            Expr::Union { a, b }
            | Expr::Intersect { a, b }
            | Expr::Subtract { a, b }
            | Expr::SmoothUnion { a, b, .. } => {
                let aa = emit(a, id, next, out);
                let bb = emit(b, id, next, out);
                match expr {
                    Expr::Union { .. } => format!("min({aa}(p), {bb}(p))"),
                    Expr::Intersect { .. } => format!("max({aa}(p), {bb}(p))"),
                    Expr::Subtract { .. } => format!("max({aa}(p), -{bb}(p))"),
                    Expr::SmoothUnion { k, .. } if *k <= 0.0 => format!("min({aa}(p), {bb}(p))"),
                    Expr::SmoothUnion { k, .. } => {
                        prelude.push_str(&format!("  let a = {aa}(p);\n  let b = {bb}(p);\n"));
                        prelude.push_str(&format!("  let h = clamp(0.5 + 0.5 * (b-a)/{}, 0.0, 1.0);\n", f(*k)));
                        format!("b*(1.0-h) + a*h - {}*h*(1.0-h)", f(*k))
                    }
                    _ => unreachable!(),
                }
            }
            Expr::Offset { shape, distance } => {
                let child = emit(shape, id, next, out);
                format!("{child}(p) - {}", f(*distance))
            }
            Expr::Shell { shape, thickness } => {
                let child = emit(shape, id, next, out);
                format!("abs({child}(p)) - {}", f(*thickness))
            }
        };
        out.push_str(&format!("fn {name}(p: vec3<f32>) -> f32 {{\n{prelude}  return {body};\n}}\n"));
        name
    }
    let mut out = String::new();
    for (id, expr) in scene.csgs.iter().enumerate() {
        expr.validate().expect("invalid registered CSG expression");
        emit(expr, id, &mut 0, &mut out);
    }
    out
}

fn material_fn(scene: &Scene) -> String {
    assert!(scene.materials.len() < 1000, "mm3e-gpu supports at most 999 materials");
    let mut s = format!(
        "fn material(raw_id: u32) -> Mat {{\n  let id = raw_id % {}u;\n  switch (id) {{\n",
        scene.materials.len().max(1)
    );
    for (i, m) in scene.materials.iter().enumerate() {
        s.push_str(&format!(
            "    case {i}u: {{ return Mat({}, {}, {}, {}, {}, {}, {}); }}\n",
            v3(m.albedo),
            f(m.metallic),
            f(m.roughness),
            f(m.reflectivity),
            f(m.specular),
            v3(m.emissive),
            if m.checker { "1.0" } else { "0.0" }
        ));
    }
    s.push_str("    default: { return Mat(vec3<f32>(0.8), 0.0, 0.35, 0.0, 0.5, vec3<f32>(0.0), 0.0); }\n");
    s.push_str("  }\n}\n\n");
    s
}

/// Emit the local-space point transform (inverse transform + modifiers) for one object into `q`.
fn emit_local(o: &Object) -> String {
    let r = o.xform.rot.cols;
    let mut s = String::new();
    s.push_str(&format!("    let R = mat3x3<f32>({}, {}, {});\n", v3(r[0]), v3(r[1]), v3(r[2])));
    let inv = if o.xform.scale.abs() > 1e-12 { 1.0 / o.xform.scale } else { 0.0 };
    s.push_str(&format!("    var q = transpose(R) * (p - {}) * {};\n", v3(o.xform.pos), f(inv)));
    let m = &o.mods;
    if m.mirror.iter().any(|&b| b) {
        s.push_str(&format!(
            "    q = vec3<f32>(select(q.x, abs(q.x), {}), select(q.y, abs(q.y), {}), select(q.z, abs(q.z), {}));\n",
            m.mirror[0], m.mirror[1], m.mirror[2]
        ));
    }
    if m.elongate != Vec3::ZERO {
        s.push_str(&format!("    q = q - min(max(q, -{h}), {h});\n", h = v3(m.elongate)));
    }
    if m.repeat != Vec3::ZERO {
        s.push_str(&format!("    q = op_repeat(q, {});\n", v3(m.repeat)));
    }
    if m.twist != 0.0 {
        s.push_str(&format!("    q = op_twist(q, {});\n", f(m.twist)));
    }
    if m.bend != 0.0 {
        s.push_str(&format!("    q = op_bend(q, {});\n", f(m.bend)));
    }
    s
}

/// The primitive distance expression in WGSL, evaluated on `q` and scaled to world units.
fn emit_prim(o: &Object, vols: &(Vec<u32>, &[mm3e_kit::volume::SdfVolume]), csg_count: usize) -> String {
    let sc = f(o.xform.scale);
    let body = match o.prim {
        Prim::Sphere { r } => format!("sd_sphere(q, {})", f(r)),
        Prim::Box { half } => format!("sd_box(q, {})", v3(half)),
        Prim::RoundBox { half, radius } => format!("sd_round_box(q, {}, {})", v3(half), f(radius)),
        Prim::Torus { major, minor } => format!("sd_torus(q, {}, {})", f(major), f(minor)),
        Prim::Cylinder { h, r } => format!("sd_cylinder(q, {}, {})", f(h), f(r)),
        Prim::Capsule { a, b, r } => format!("sd_capsule(q, {}, {}, {})", v3(a), v3(b), f(r)),
        Prim::Cone { r1, r2, h } => format!("sd_round_cone(q, {}, {}, {})", f(r1), f(r2), f(h)),
        Prim::Ellipsoid { r } => format!("sd_ellipsoid(q, {})", v3(r)),
        Prim::Octahedron { s } => format!("sd_octahedron(q, {})", f(s)),
        Prim::HexPrism { r, h } => format!("sd_hex_prism(q, {}, {})", f(r), f(h)),
        Prim::Plane { n, h } => format!("sd_plane(q, {}, {})", v3(n), f(h)),
        Prim::Surface { .. } => unreachable!("triangle surfaces are rejected before shader generation"),
        Prim::Csg { id } => {
            assert!((id as usize) < csg_count, "unregistered CSG expression");
            format!("csg_{id}_0(q)")
        }
        Prim::Volume { id } => {
            let (offsets, volumes) = vols;
            if volumes.is_empty() {
                "1e30".to_string()
            } else {
                let vid = id as usize % volumes.len();
                let v = &volumes[vid];
                let (nx, ny, nz) = v.dims;
                format!("vol_sample({}u, {nx}u, {ny}u, {nz}u, {}, {}, q)", offsets[vid], v3(v.min), v3(v.cell))
            }
        }
    };
    let mut expr = format!("    var od = {body} * {sc};\n");
    if o.mods.round != 0.0 {
        expr.push_str(&format!("    od = od - {};\n", f(o.mods.round)));
    }
    if o.mods.onion != 0.0 {
        expr.push_str(&format!("    od = abs(od) - {};\n", f(o.mods.onion)));
    }
    expr
}

pub(crate) fn volume_offsets(volumes: &[mm3e_kit::volume::SdfVolume]) -> Vec<u32> {
    let mut offsets = Vec::with_capacity(volumes.len());
    let mut off = 0u32;
    for v in volumes {
        offsets.push(off);
        off += v.data.len() as u32;
    }
    offsets
}

fn map_fn(scene: &Scene) -> String {
    let vols = (volume_offsets(&scene.volumes), scene.volumes.as_slice());
    let mut s = String::from("fn map(p: vec3<f32>) -> vec2<f32> {\n  var d = vec2<f32>(1e30, 0.0);\n");
    for (i, o) in scene.objects.iter().enumerate() {
        let matf = f(o.mat as f32);
        s.push_str("  {\n");
        s.push_str(&emit_local(o));
        s.push_str(&emit_prim(o, &vols, scene.csgs.len()));
        if i == 0 {
            s.push_str(&format!("    d = vec2<f32>(od, {matf});\n"));
        } else {
            match o.combine {
                Combine::Union => {
                    s.push_str(&format!("    if (od < d.x) {{ d = vec2<f32>(od, {matf}); }}\n"));
                }
                Combine::Smooth(k) if k <= 0.0 => {
                    s.push_str(&format!("    if (od < d.x) {{ d = vec2<f32>(od, {matf}); }}\n"));
                }
                Combine::Smooth(k) => {
                    s.push_str(&format!("    let h = clamp(0.5 + 0.5*(od - d.x)/{k}, 0.0, 1.0);\n", k = f(k)));
                    s.push_str(&format!(
                        "    d = vec2<f32>(od*(1.0-h) + d.x*h - {k}*h*(1.0-h), select({matf}, d.y, h > 0.5));\n",
                        k = f(k)
                    ));
                }
                Combine::Subtract => {
                    s.push_str("    d.x = max(d.x, -od);\n");
                }
            }
        }
        s.push_str("  }\n");
    }
    // Dynamic spheres (player + physics bodies) unioned from the uniform — no recompile when they
    // move. Material id 1000+i selects the dynamic colour in the shader.
    s.push_str("  let nd = u32(u.n_dyn);\n");
    s.push_str("  for (var i = 0u; i < nd; i = i + 1u) {\n");
    s.push_str("    let sp = u.dyn_pr[i];\n");
    s.push_str("    let dd = length(p - sp.xyz) - sp.w;\n");
    s.push_str("    if (dd < d.x) { d = vec2<f32>(dd, 1000.0 + f32(i)); }\n");
    s.push_str("  }\n");
    s.push_str("  return d;\n}\n\n");
    s
}

fn direct_lighting_fn(scene: &Scene) -> String {
    let mut s = String::from(
        "fn direct_lighting(p: vec3<f32>, n: vec3<f32>, v: vec3<f32>, albedo: vec3<f32>, m: Mat) -> vec3<f32> {\n  var c = vec3<f32>(0.0);\n",
    );
    for l in &scene.lights {
        let (dir_or_pos, color, directional, radius) = (l.vec, l.color, l.directional, l.radius);
        s.push_str("  {\n");
        if directional {
            s.push_str(&format!("    let ldir = {};\n", v3(dir_or_pos)));
            s.push_str("    let ldist = 1e30;\n");
            s.push_str("    let atten = 1.0;\n");
        } else {
            s.push_str(&format!("    let lp = {};\n", v3(dir_or_pos)));
            s.push_str("    let ldir = normalize(lp - p);\n");
            s.push_str("    let ldist = length(lp - p);\n");
            s.push_str("    let atten = 1.0 / max(ldist*ldist, 0.0025);\n");
        }
        let k = if radius <= 0.0 {
            "24.0".to_string()
        } else if directional {
            f((1.0 / radius).clamp(2.0, 64.0))
        } else {
            // distance-dependent; compute in-shader
            format!("clamp(ldist/{}, 2.0, 64.0)", f(radius))
        };
        s.push_str("    let ndl = max(dot(n, ldir), 0.0);\n");
        s.push_str("    if (ndl > 0.0) {\n");
        if scene.shadows {
            let lift = f((scene.marcher.normal_h * 2.0).max(0.01));
            s.push_str(&format!("      let sh = soft_shadow(p + n*{lift}, ldir, min(ldist, MAX_DIST), {k});\n"));
        } else {
            s.push_str("      let sh = 1.0;\n");
        }
        s.push_str(&format!(
            "      c = c + brdf(n, ldir, v, albedo, m.metallic, m.rough, m.spec) * {} * (sh * atten);\n",
            v3(color)
        ));
        s.push_str("    }\n  }\n");
    }
    s.push_str("  return c;\n}\n\n");
    s
}

// ----------------------------------------------------------------------------
// Static WGSL: uniforms, primitive SDFs, shading, the compute entry point.
// ----------------------------------------------------------------------------

const KERNEL_HEADER: &str = r#"
struct U {
  eye: vec3<f32>, fov: f32,
  right: vec3<f32>, aa: f32,
  up: vec3<f32>, bounces: f32,
  fwd: vec3<f32>, n_dyn: f32,
  res: vec2<f32>, _pad2: vec2<f32>,
  dyn_pr: array<vec4<f32>, MAX_DYN>,
  dyn_col: array<vec4<f32>, MAX_DYN>,
};
@group(0) @binding(0) var<uniform> u: U;
@group(0) @binding(1) var outtex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<storage, read> gi_cubes: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read> vol_data: array<f32>;

fn vol_at(off: u32, nx: u32, ny: u32, i: u32, j: u32, k: u32) -> f32 {
  return vol_data[off + (k * ny + j) * nx + i];
}
fn vol_sample(off: u32, nx: u32, ny: u32, nz: u32, vmin: vec3<f32>, cell: vec3<f32>, p: vec3<f32>) -> f32 {
  let size = vec3<f32>(f32(nx - 1u), f32(ny - 1u), f32(nz - 1u)) * cell;
  let c = clamp(p, vmin, vmin + size);
  let outside = length(p - c);
  let g = (c - vmin) / cell;
  let i0 = min(u32(max(floor(g.x), 0.0)), nx - 2u);
  let j0 = min(u32(max(floor(g.y), 0.0)), ny - 2u);
  let k0 = min(u32(max(floor(g.z), 0.0)), nz - 2u);
  let fx = clamp(g.x - f32(i0), 0.0, 1.0);
  let fy = clamp(g.y - f32(j0), 0.0, 1.0);
  let fz = clamp(g.z - f32(k0), 0.0, 1.0);
  let x00 = mix(vol_at(off, nx, ny, i0, j0, k0), vol_at(off, nx, ny, i0 + 1u, j0, k0), fx);
  let x10 = mix(vol_at(off, nx, ny, i0, j0 + 1u, k0), vol_at(off, nx, ny, i0 + 1u, j0 + 1u, k0), fx);
  let x01 = mix(vol_at(off, nx, ny, i0, j0, k0 + 1u), vol_at(off, nx, ny, i0 + 1u, j0, k0 + 1u), fx);
  let x11 = mix(vol_at(off, nx, ny, i0, j0 + 1u, k0 + 1u), vol_at(off, nx, ny, i0 + 1u, j0 + 1u, k0 + 1u), fx);
  let d = mix(mix(x00, x10, fy), mix(x01, x11, fy), fz);
  if (outside > 0.0) { return max(outside, d - outside); }
  return d;
}

const PI: f32 = 3.14159265359;

struct Mat { albedo: vec3<f32>, metallic: f32, rough: f32, refl: f32, spec: f32, emissive: vec3<f32>, checker: f32 };

fn sd_sphere(p: vec3<f32>, r: f32) -> f32 { return length(p) - r; }
fn sd_box(p: vec3<f32>, b: vec3<f32>) -> f32 { let q = abs(p) - b; return length(max(q, vec3<f32>(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0); }
fn sd_round_box(p: vec3<f32>, b: vec3<f32>, r: f32) -> f32 { return sd_box(p, b - vec3<f32>(r)) - r; }
fn sd_plane(p: vec3<f32>, n: vec3<f32>, h: f32) -> f32 { return dot(p, n) + h; }
fn sd_torus(p: vec3<f32>, ma: f32, mi: f32) -> f32 { let q = length(p.xz) - ma; return length(vec2<f32>(q, p.y)) - mi; }
fn sd_cylinder(p: vec3<f32>, h: f32, r: f32) -> f32 { let dx = length(p.xz) - r; let dy = abs(p.y) - h; return min(max(dx, dy), 0.0) + length(max(vec2<f32>(dx, dy), vec2<f32>(0.0))); }
fn sd_capsule(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, r: f32) -> f32 { let pa = p - a; let ba = b - a; let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0); return length(pa - ba * h) - r; }
fn sd_round_cone(p: vec3<f32>, r1: f32, r2: f32, h: f32) -> f32 {
  let qx = length(p.xz); let qy = p.y; let b = (r1 - r2) / max(h, 1e-6); let a = sqrt(max(1.0 - b*b, 0.0)); let k = qx*(-b) + qy*a;
  if (k < 0.0) { return length(vec2<f32>(qx, qy)) - r1; }
  if (k > a*h) { return length(vec2<f32>(qx, qy - h)) - r2; }
  return qx*a + qy*b - r1;
}
fn sd_ellipsoid(p: vec3<f32>, r: vec3<f32>) -> f32 { let k0 = length(p / r); if (k0 < 1e-6) { return -min(r.x, min(r.y, r.z)); } let k1 = length(p / (r*r)); return k0 * (k0 - 1.0) / max(k1, 1e-9); }
fn sd_octahedron(p: vec3<f32>, s: f32) -> f32 { let a = abs(p); return (a.x + a.y + a.z - s) * 0.5773503; }
fn sd_hex_prism(p: vec3<f32>, r: f32, h: f32) -> f32 {
  let k = vec3<f32>(-0.8660254, 0.5, 0.57735); var a = abs(p); let t = 2.0 * min(k.x*a.x + k.y*a.z, 0.0);
  a.x = a.x - t*k.x; a.z = a.z - t*k.y; let cl = clamp(a.x, -k.z*r, k.z*r);
  let dx = length(vec2<f32>(a.x - cl, a.z - r)) * sign(a.z - r); let dy = a.y - h;
  return min(max(dx, dy), 0.0) + length(max(vec2<f32>(dx, dy), vec2<f32>(0.0)));
}
fn op_repeat(p: vec3<f32>, period: vec3<f32>) -> vec3<f32> {
  var q = p;
  if (period.x > 0.0) { q.x = p.x - period.x * round(p.x / period.x); }
  if (period.y > 0.0) { q.y = p.y - period.y * round(p.y / period.y); }
  if (period.z > 0.0) { q.z = p.z - period.z * round(p.z / period.z); }
  return q;
}
fn op_twist(p: vec3<f32>, k: f32) -> vec3<f32> { let a = k * p.y; let s = sin(a); let c = cos(a); return vec3<f32>(c*p.x - s*p.z, p.y, s*p.x + c*p.z); }
fn op_bend(p: vec3<f32>, k: f32) -> vec3<f32> { let a = k * p.x; let s = sin(a); let c = cos(a); return vec3<f32>(c*p.x - s*p.y, s*p.x + c*p.y, p.z); }

fn calc_normal(p: vec3<f32>) -> vec3<f32> {
  let h = NORMAL_H;
  let k0 = vec3<f32>(1.0, -1.0, -1.0); let k1 = vec3<f32>(-1.0, -1.0, 1.0);
  let k2 = vec3<f32>(-1.0, 1.0, -1.0); let k3 = vec3<f32>(1.0, 1.0, 1.0);
  return normalize(k0*map(p + k0*h).x + k1*map(p + k1*h).x + k2*map(p + k2*h).x + k3*map(p + k3*h).x);
}
fn raymarch(ro: vec3<f32>, rd: vec3<f32>) -> vec3<f32> {
  var t = 0.0;
  for (var i = 0; i < MARCH_STEPS; i = i + 1) {
    let p = ro + rd*t; let dm = map(p);
    if (dm.x < EPS_BASE * (1.0 + t*0.5) + LOD_FOOTPRINT * t) { return vec3<f32>(t, dm.y, 1.0); }
    t = t + dm.x * STEP_SCALE;
    if (t > MAX_DIST) { break; }
  }
  return vec3<f32>(t, 0.0, 0.0);
}
fn soft_shadow(ro: vec3<f32>, rd: vec3<f32>, maxt: f32, k: f32) -> f32 {
  var res = 1.0; var t = 0.02;
  for (var i = 0; i < SHADOW_STEPS; i = i + 1) {
    let h = map(ro + rd*t).x;
    if (h < 0.0008) { return 0.0; }
    res = min(res, k*h/t); t = t + clamp(h, 0.06, 0.5 + 0.7*t);
    if (t > maxt) { break; }
  }
  return clamp(res, 0.0, 1.0);
}
fn ao(p: vec3<f32>, n: vec3<f32>) -> f32 {
  if (AO_SAMPLES == 0) { return 1.0; }
  var occ = 0.0; var sca = 1.0;
  for (var i = 0; i < AO_SAMPLES; i = i + 1) { let hr = AO_BASE + 0.12 * f32(i) / AO_SPAN; let d = map(p + n*hr).x; occ = occ + (hr - d)*sca; sca = sca * 0.92; }
  return clamp(1.0 - 2.6*occ, 0.0, 1.0);
}
fn sky(d: vec3<f32>) -> vec3<f32> { let t = clamp(0.5*(d.y + 1.0), 0.0, 1.0); let base = mix(vec3<f32>(0.78,0.86,0.96), vec3<f32>(0.20,0.38,0.72), t); let s = max(dot(d, SUN), 0.0); return base + vec3<f32>(pow(s, 8.0)*0.3 + pow(s, 220.0)*6.0); }
fn sky_diffuse(d: vec3<f32>) -> vec3<f32> { let t = clamp(0.5*(d.y + 1.0), 0.0, 1.0); let base = mix(vec3<f32>(0.78,0.86,0.96), vec3<f32>(0.20,0.38,0.72), t); return base + vec3<f32>(pow(max(dot(d, SUN), 0.0), 8.0)*0.3); }
fn fresnel(c: f32, f0: f32) -> f32 { return f0 + (1.0 - f0) * pow(clamp(1.0 - c, 0.0, 1.0), 5.0); }
fn brdf(n: vec3<f32>, l: vec3<f32>, v: vec3<f32>, albedo: vec3<f32>, metallic: f32, rough: f32, reflectance: f32) -> vec3<f32> {
  let nol = max(dot(n, l), 0.0);
  if (nol <= 0.0) { return vec3<f32>(0.0); }
  let nov = max(dot(n, v), 1e-4); let h = normalize(l + v); let noh = max(dot(n, h), 0.0); let voh = max(dot(v, h), 0.0);
  let a = max(rough*rough, 1e-3); let a2 = a*a;
  let dd = noh*noh*(a2 - 1.0) + 1.0; let D = a2 / max(PI*dd*dd, 1e-12);
  let gv = nol * sqrt(nov*nov*(1.0 - a2) + a2); let gl = nov * sqrt(nol*nol*(1.0 - a2) + a2); let V = 0.5 / max(gv + gl, 1e-5);
  let f0 = mix(vec3<f32>(0.08 * clamp(reflectance, 0.0, 1.0)), albedo, metallic);
  let F = f0 + (vec3<f32>(1.0) - f0) * pow(clamp(1.0 - voh, 0.0, 1.0), 5.0);
  let spec = F * (D * V); let kd = (vec3<f32>(1.0) - F) * (1.0 - metallic); let diff = kd * albedo / PI;
  return (diff + spec) * nol;
}
fn ibl(n: vec3<f32>) -> vec3<f32> {
  var up = vec3<f32>(0.0, 1.0, 0.0); if (abs(n.y) > 0.9) { up = vec3<f32>(1.0, 0.0, 0.0); }
  let t1 = normalize(cross(n, up)); let t2 = normalize(cross(n, t1));
  var acc = vec3<f32>(0.0); var w = 0.0;
  let dirs = array<vec3<f32>, 5>(n, normalize(n*0.6 + t1*0.6), normalize(n*0.6 - t1*0.6), normalize(n*0.6 + t2*0.6), normalize(n*0.6 - t2*0.6));
  for (var i = 0; i < 5; i = i + 1) { let cw = max(dot(dirs[i], n), 0.0); acc = acc + sky_diffuse(dirs[i]) * cw; w = w + cw; }
  return acc / max(w, 1e-4) * SKY_AMBIENT;
}
fn gi_face(i: u32, j: u32, k: u32, face: u32) -> vec3<f32> {
  return gi_cubes[((k * GI_NY + j) * GI_NX + i) * 6u + face].xyz;
}
fn gi_axis_lo(g: f32, n: u32) -> u32 { return min(u32(max(floor(g), 0.0)), n - 1u); }
fn gi_sample(pos: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
  if (!HAS_GI) { return vec3<f32>(0.0); }
  let g = (pos - GI_MIN) * GI_SCALE;
  let i0 = gi_axis_lo(g.x, GI_NX); let j0 = gi_axis_lo(g.y, GI_NY); let k0 = gi_axis_lo(g.z, GI_NZ);
  let i1 = min(i0 + 1u, GI_NX - 1u); let j1 = min(j0 + 1u, GI_NY - 1u); let k1 = min(k0 + 1u, GI_NZ - 1u);
  let fx = clamp(g.x - f32(i0), 0.0, 1.0);
  let fy = clamp(g.y - f32(j0), 0.0, 1.0);
  let fz = clamp(g.z - f32(k0), 0.0, 1.0);
  var face: array<vec3<f32>, 6>;
  for (var fc = 0u; fc < 6u; fc = fc + 1u) {
    let x00 = mix(gi_face(i0, j0, k0, fc), gi_face(i1, j0, k0, fc), fx);
    let x10 = mix(gi_face(i0, j1, k0, fc), gi_face(i1, j1, k0, fc), fx);
    let x01 = mix(gi_face(i0, j0, k1, fc), gi_face(i1, j0, k1, fc), fx);
    let x11 = mix(gi_face(i0, j1, k1, fc), gi_face(i1, j1, k1, fc), fx);
    face[fc] = mix(mix(x00, x10, fy), mix(x01, x11, fy), fz);
  }
  let nn = n * n;
  var fx_face = face[1]; if (n.x >= 0.0) { fx_face = face[0]; }
  var fy_face = face[3]; if (n.y >= 0.0) { fy_face = face[2]; }
  var fz_face = face[5]; if (n.z >= 0.0) { fz_face = face[4]; }
  return fx_face * nn.x + fy_face * nn.y + fz_face * nn.z;
}
fn surface_albedo(m: Mat, p: vec3<f32>) -> vec3<f32> {
  if (m.checker < 0.5) { return m.albedo; }
  let ix = i32(floor(p.x)); let iz = i32(floor(p.z));
  var base = vec3<f32>(0.20); if (((ix + iz) & 1) == 0) { base = vec3<f32>(0.92); }
  return base * m.albedo;
}
fn aces(x: vec3<f32>) -> vec3<f32> { let a = max(x, vec3<f32>(0.0)); return clamp((a*(2.51*a + 0.03)) / (a*(2.43*a + 0.59) + 0.14), vec3<f32>(0.0), vec3<f32>(1.0)); }
fn apply_fog(c: vec3<f32>, t: f32) -> vec3<f32> { let fa = 1.0 - exp(-t * FOG_DENSITY); return mix(c, FOG, clamp(fa, 0.0, 1.0)); }
"#;

const KERNEL_BODY: &str = r#"
fn shade(ro0: vec3<f32>, rd0: vec3<f32>) -> vec3<f32> {
  var ro = ro0; var rd = rd0; var col = vec3<f32>(0.0); var atten = vec3<f32>(1.0);
  let maxb = i32(u.bounces);
  for (var b = 0; b <= maxb; b = b + 1) {
    let hit = raymarch(ro, rd);
    if (hit.z < 0.5) { col = col + atten * sky(rd); break; }
    let t = hit.x; let p = ro + rd*t; let n = calc_normal(p); let v = -rd;
    var m: Mat;
    if (hit.y > 999.5) {
      let di = u32(hit.y - 1000.0); let c = u.dyn_col[di];
      m = Mat(c.xyz, c.w, 0.32, 0.18, 0.5, vec3<f32>(0.0), 0.0);
    } else {
      m = material(u32(hit.y + 0.5));
    }
    let albedo = surface_albedo(m, p);
    let occ = ao(p, n);
    var rad = albedo * (ibl(n) + gi_sample(p, n) + AMBIENT) * occ;
    rad = rad + direct_lighting(p, n, v, albedo, m);
    rad = rad + m.emissive;
    rad = apply_fog(rad, t);
    let fr = select(0.0, fresnel(max(dot(n, v), 0.0), m.refl), m.refl > 0.0);
    if (b == maxb || m.refl <= 0.0) { col = col + atten * rad; break; }
    col = col + atten * rad * (1.0 - fr);
    atten = atten * fr;
    ro = p + n*REFL_LIFT; rd = reflect(rd, n);
  }
  return col;
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
  if (gid.x >= u32(u.res.x) || gid.y >= u32(u.res.y)) { return; }
  let aspect = u.res.x / u.res.y;
  let n = max(i32(u.aa), 1);
  var col = vec3<f32>(0.0);
  for (var sy = 0; sy < n; sy = sy + 1) {
    for (var sx = 0; sx < n; sx = sx + 1) {
      let ox = (f32(sx) + 0.5) / f32(n); let oy = (f32(sy) + 0.5) / f32(n);
      let px = f32(gid.x) + ox; let py = f32(gid.y) + oy;
      let ndcx = (2.0*px/u.res.x - 1.0) * aspect * u.fov;
      let ndcy = (1.0 - 2.0*py/u.res.y) * u.fov;
      let rd = normalize(u.fwd + u.right*ndcx + u.up*ndcy);
      col = col + shade(u.eye, rd);
    }
  }
  col = col / f32(n*n);
  col = pow(aces(col * EXPOSURE), vec3<f32>(1.0/2.2));
  textureStore(outtex, vec2<i32>(gid.xy), vec4<f32>(col, 1.0));
}
"#;

#[cfg(test)]
mod csg_parity_tests {
    use super::*;
    use crate::GpuRenderer;
    use mm3e_kit::{
        vec::{Mat3, Transform},
        Camera, Rgba,
    };
    use wgpu::util::DeviceExt;

    /// This test must run on an actual wgpu adapter; failure to initialize is a failure, not a
    /// passing substitute. It is opt-in so CPU-only library consumers can run normal tests.
    #[test]
    #[ignore = "requires an actual wgpu adapter; run cargo test -p mm3e-gpu --lib -- --ignored --nocapture"]
    fn csg_gpu_field_and_render_parity() {
        let gpu = GpuRenderer::new().expect("a wgpu adapter is required for CSG parity validation");
        println!("CSG parity adapter: {}", gpu.adapter_name());
        let sphere = || Box::new(Expr::Sphere { r: 0.8 });
        let moved = || Box::new(Expr::Transform { shape: sphere(), xform: Transform::at(Vec3::new(0.0, 0.0, 0.6)) });
        let expressions = vec![
            Expr::Sphere { r: 0.8 },
            Expr::Box { half: Vec3::new(0.8, 0.5, 0.3) },
            Expr::RoundBox { half: Vec3::new(0.8, 0.5, 0.3), radius: 0.1 },
            Expr::Torus { major: 0.7, minor: 0.2 },
            Expr::Cylinder { h: 0.7, r: 0.4 },
            Expr::Capsule { a: Vec3::new(-0.2, -0.7, 0.0), b: Vec3::new(0.2, 0.7, 0.0), r: 0.3 },
            Expr::Cone { r1: 0.5, r2: 0.2, h: 0.8 },
            Expr::Ellipsoid { r: Vec3::new(0.8, 0.5, 0.3) },
            Expr::Octahedron { s: 0.8 },
            Expr::HexPrism { r: 0.6, h: 0.8 },
            Expr::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.2 },
            Expr::Transform {
                shape: Box::new(Expr::Box { half: Vec3::new(0.8, 0.5, 0.3) }),
                xform: Transform::new(Vec3::new(0.2, 0.1, 0.3), Mat3::from_euler(0.1, 0.6, 0.3), 1.4),
            },
            Expr::Union { a: sphere(), b: moved() },
            Expr::Intersect { a: sphere(), b: moved() },
            Expr::Subtract { a: sphere(), b: moved() },
            Expr::SmoothUnion { a: sphere(), b: moved(), k: 0.2 },
            Expr::Offset { shape: sphere(), distance: 0.12 },
            Expr::Shell { shape: sphere(), thickness: 0.07 },
        ];
        let mut scene = Scene::new(32, 32);
        for expr in expressions {
            scene.csg(expr).unwrap();
        }
        let mut points = vec![];
        for x in -4..=4 {
            for y in -4..=4 {
                for z in -4..=4 {
                    points.push([x as f32 * 0.31, y as f32 * 0.37, z as f32 * 0.41, 0.0]);
                }
            }
        }
        let n = points.len();
        let total = n * scene.csgs.len();
        let mut source = build_shader(&scene);
        source.push_str("\n@group(0) @binding(4) var<storage,read> points: array<vec4<f32>>;\n@group(0) @binding(5) var<storage,read_write> results: array<f32>;\n");
        source.push_str(&format!("@compute @workgroup_size(64) fn probe(@builtin(global_invocation_id) gid: vec3<u32>) {{ if(gid.x >= {total}u) {{ return; }} let p = points[gid.x % {n}u].xyz; switch(gid.x/{n}u) {{\n"));
        for i in 0..scene.csgs.len() {
            source.push_str(&format!("case {i}u: {{ results[gid.x] = csg_{i}_0(p); }}\n"));
        }
        source.push_str("default: { results[gid.x]=0.0; } } }\n");
        let device = &gpu.device;
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("csg-parity"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("csg-parity"),
            layout: None,
            module: &module,
            entry_point: Some("probe"),
            compilation_options: Default::default(),
            cache: None,
        });
        let input = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("csg-points"),
            contents: bytemuck::cast_slice(&points),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let size = (total * 4) as u64;
        let output = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("csg-output"),
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("csg-readback"),
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("csg-bindings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry { binding: 4, resource: input.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 5, resource: output.as_entire_binding() },
            ],
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("csg-parity") });
        {
            let mut pass = encoder
                .begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("csg-parity"), timestamp_writes: None });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bindings, &[]);
            pass.dispatch_workgroups((total as u32).div_ceil(64), 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, size);
        gpu.queue.submit(Some(encoder.finish()));
        let slice = readback.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| tx.send(r).unwrap());
        device.poll(wgpu::PollType::Wait).unwrap();
        rx.recv().unwrap().unwrap();
        let mapped = slice.get_mapped_range();
        let values: &[f32] = bytemuck::cast_slice(&mapped);
        let mut max_error = 0.0f32;
        for (i, expr) in scene.csgs.iter().enumerate() {
            for (j, p) in points.iter().enumerate() {
                let expected = expr.distance(Vec3::new(p[0], p[1], p[2]));
                let actual = values[i * n + j];
                let error = (expected - actual).abs();
                max_error = max_error.max(error);
                assert!(
                    actual.is_finite() && error <= 5e-5 * expected.abs().max(1.0),
                    "expression {i} at {p:?}: CPU {expected}, GPU {actual}"
                );
            }
        }
        println!("{total} GPU scalar samples match CPU, max error={max_error}");
        drop(mapped);
        readback.unmap();
        scene.csgs.clear();
        let id = scene.csg(Expr::Sphere { r: 1.0 }).unwrap();
        scene.add(Object::new(Prim::Csg { id }, Transform::IDENTITY, 0));
        scene.aa = 1;
        scene.ao = false;
        scene.shadows = false;
        scene.bounces = 0;
        scene.post.bloom = false;
        let camera = Camera::look_at(Vec3::new(0.0, 0.0, 4.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 0.7);
        let csg_gpu = gpu.compile(&scene, 32, 32).render_rgba(&gpu, &camera);
        scene.objects[0].prim = Prim::Sphere { r: 1.0 };
        let primitive_gpu = gpu.compile(&scene, 32, 32).render_rgba(&gpu, &camera);
        assert_eq!(csg_gpu, primitive_gpu, "GPU analytic CSG sphere must retain primitive pixels");
        let cpu = mm3e_orchestrator::render(&scene, &camera).to_rgba8(Rgba::new(0.0, 0.0, 0.0, 1.0));
        let mean = cpu.iter().zip(&csg_gpu).map(|(a, b)| (*a as f32 - *b as f32).abs()).sum::<f32>() / cpu.len() as f32;
        println!("CPU/GPU sphere display mean byte error={mean}");
        assert!(mean < 2.0, "CPU/GPU render diverges");
    }
}
