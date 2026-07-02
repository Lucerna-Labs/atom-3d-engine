//! WGSL codegen — the heart of the GPU backend. Walks an `mm3e_orchestrator::Scene` and emits a
//! WGSL compute shader whose `map(p) -> vec2(dist, matId)` is the GPU twin of the CPU `world()`
//! closure. This is the doctrine, re-targeted: the same 8 atoms (fold over objects, project the
//! camera ray, compare via SDFs, combine the CSG/lighting) now expressed as shader text the GPU
//! runs thousands of lanes at once. The camera + time ride in a uniform so a real-time loop never
//! recompiles; the scene (objects, materials, lights, sky) is baked into the source.

use mm3e_kit::vec::Vec3;
use mm3e_orchestrator::{Combine, Object, Prim, Scene};

fn f(x: f32) -> String {
    // Always emit a decimal point so WGSL treats it as f32, and avoid `inf`.
    if x.is_finite() {
        let s = format!("{x:?}");
        if s.contains('.') || s.contains('e') {
            s
        } else {
            format!("{s}.0")
        }
    } else {
        "1e30".to_string()
    }
}

fn v3(v: Vec3) -> String {
    format!("vec3<f32>({}, {}, {})", f(v.x), f(v.y), f(v.z))
}

/// Build the full compute shader for `scene`.
pub fn build_shader(scene: &Scene) -> String {
    let mut s = String::new();
    s.push_str(KERNEL_HEADER);

    // Scene constants.
    s.push_str(&format!("const SUN: vec3<f32> = {};\n", v3(scene.sun_dir)));
    s.push_str(&format!("const SKY_AMBIENT: vec3<f32> = {};\n", v3(scene.sky_ambient)));
    s.push_str(&format!("const AMBIENT: vec3<f32> = {};\n", v3(scene.ambient)));
    s.push_str(&format!("const FOG: vec3<f32> = {};\n", v3(scene.fog)));
    s.push_str(&format!("const FOG_DENSITY: f32 = {};\n", f(scene.fog_density)));
    s.push_str(&format!("const EXPOSURE: f32 = {};\n", f(scene.post.exposure)));
    // Marcher budgets, baked from `scene.marcher` so the shader honors the same policy the CPU
    // path does (the GPU previously hardcoded its own numbers and ignored the Scene entirely).
    // `step_scale` is clamped to 1 from above: the GPU marches conservatively (no CPU-style
    // over-relaxation/leaps — per-lane divergence makes them a poor GPU fit), but a scene that
    // under-relaxes for non-Lipschitz warps (twist/bend) must under-relax here too.
    let mr = &scene.marcher;
    s.push_str(&format!("const MARCH_STEPS: i32 = {};\n", mr.max_steps.min(i32::MAX as u32)));
    s.push_str(&format!("const MAX_DIST: f32 = {};\n", f(mr.max_dist)));
    s.push_str(&format!("const EPS_BASE: f32 = {};\n", f(mr.eps)));
    s.push_str(&format!("const LOD_FOOTPRINT: f32 = {};\n", f(mr.lod_footprint)));
    s.push_str(&format!("const STEP_SCALE: f32 = {};\n", f(mr.step_scale.min(1.0))));
    s.push_str(&format!("const SHADOW_STEPS: i32 = {};\n", mr.shadow_steps.min(i32::MAX as u32)));
    s.push_str(&format!("const AO_SAMPLES: i32 = {};\n", mr.ao_samples.min(i32::MAX as u32)));
    s.push_str(&format!("const AO_SPAN: f32 = {};\n\n", f((mr.ao_samples.max(2) - 1) as f32)));

    s.push_str(&material_fn(scene));
    s.push_str(&map_fn(scene));
    s.push_str(&direct_lighting_fn(scene));
    s.push_str(KERNEL_BODY);
    s
}

fn material_fn(scene: &Scene) -> String {
    let mut s = String::from("fn material(id: u32) -> Mat {\n  switch (id) {\n");
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
    // Signed inverse, matching Transform::to_local: a negative scale must invert with its sign,
    // and only a degenerate |scale| collapses to 0 (finite point rather than Inf poisoning).
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
        s.push_str(&format!("    q = q - clamp(q, -{h}, {h});\n", h = v3(m.elongate)));
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
fn emit_prim(o: &Object) -> String {
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

fn map_fn(scene: &Scene) -> String {
    let mut s = String::from("fn map(p: vec3<f32>) -> vec2<f32> {\n  var d = vec2<f32>(1e30, 0.0);\n");
    for (i, o) in scene.objects.iter().enumerate() {
        let matf = f(o.mat as f32);
        s.push_str("  {\n");
        s.push_str(&emit_local(o));
        s.push_str(&emit_prim(o));
        if i == 0 {
            s.push_str(&format!("    d = vec2<f32>(od, {matf});\n"));
        } else {
            match o.combine {
                Combine::Union => {
                    s.push_str(&format!("    if (od < d.x) {{ d = vec2<f32>(od, {matf}); }}\n"));
                }
                // k → 0 recovers a hard union on the CPU (sdf::smooth_union guards it); emitting
                // the blend with k = 0 would divide by zero in the shader.
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
        s.push_str(&format!("      let sh = soft_shadow(p + n*0.01, ldir, min(ldist, MAX_DIST), {k});\n"));
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
const MAX_DYN: u32 = 24u;
struct U {
  eye: vec3<f32>, fov: f32,
  right: vec3<f32>, aa: f32,
  up: vec3<f32>, bounces: f32,
  fwd: vec3<f32>, n_dyn: f32,
  res: vec2<f32>, _pad2: vec2<f32>,
  dyn_pr: array<vec4<f32>, 24>,   // dynamic spheres: xyz = centre, w = radius
  dyn_col: array<vec4<f32>, 24>,  // xyz = albedo, w = metallic
};
@group(0) @binding(0) var<uniform> u: U;
@group(0) @binding(1) var outtex: texture_storage_2d<rgba8unorm, write>;

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
  let h = 0.0009;
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
// The CPU soft shadow, ported verbatim (mm3e-kit/src/march.rs soft_shadow): blue-noise start
// jitter (deterministic hash of the ray origin) dithers the coarser steps across pixels instead
// of banding, and the step cap grows with t — fine near the caster, long leaps far away. The
// hash won't match the CPU bit-for-bit (sin precision differs across vendors, amplified by the
// fract), but the dither family and the step schedule now do.
fn soft_shadow(ro: vec3<f32>, rd: vec3<f32>, maxt: f32, k: f32) -> f32 {
  var res = 1.0;
  let hb = ro.x * 127.1 + ro.y * 311.7 + ro.z * 74.7;
  let jitter = abs(fract(sin(hb) * 43758.547));
  var t = 0.02 + jitter * 0.16;
  for (var i = 0; i < SHADOW_STEPS; i = i + 1) {
    let h = map(ro + rd*t).x;
    if (h < 0.0008) { return 0.0; }
    res = min(res, k*h/t);
    t = t + clamp(h, 0.06, 0.5 + 0.7*t);
    if (t > maxt) { break; }
  }
  return clamp(res, 0.0, 1.0);
}
fn ao(p: vec3<f32>, n: vec3<f32>) -> f32 {
  if (AO_SAMPLES == 0) { return 1.0; }
  var occ = 0.0; var sca = 1.0;
  for (var i = 0; i < AO_SAMPLES; i = i + 1) { let hr = 0.01 + 0.12 * f32(i) / AO_SPAN; let d = map(p + n*hr).x; occ = occ + (hr - d)*sca; sca = sca * 0.92; }
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
// 64-bit FNV-1a over the 8 little-endian bytes of (ix, iz), on u32 pairs — the kit's
// `atoms::hash_cell`, ported bit-exactly so the checker's per-tile tint matches the CPU
// reference. The FNV prime 0x00000100000001B3 is 2^40 + 0x1B3, so the 64-bit multiply
// decomposes into a 40-bit shift plus a small-constant product built from 16-bit limbs.
fn fnv_mul_prime(h: vec2<u32>) -> vec2<u32> {
  let x = (h.y & 0xFFFFu) * 0x1B3u;          // low limb * 0x1B3   (< 2^25)
  let y = (h.y >> 16u) * 0x1B3u;             // high limb * 0x1B3  (< 2^25)
  let sum = x + ((y & 0xFFFFu) << 16u);      // low 32 bits of lo*0x1B3 (may wrap once)
  let carry = (y >> 16u) + select(0u, 1u, sum < x);
  let lo = sum;
  let hi = carry + h.x * 0x1B3u + ((h.y & 0x00FFFFFFu) << 8u); // + (h << 40) mod 2^64
  return vec2<u32>(hi, lo);
}
fn fnv_byte(h: vec2<u32>, b: u32) -> vec2<u32> { return fnv_mul_prime(vec2<u32>(h.x, h.y ^ (b & 0xFFu))); }
fn hash_cell(ix: i32, iz: i32) -> f32 {
  var h = vec2<u32>(0xcbf29ce4u, 0x84222325u);
  let ux = bitcast<u32>(ix); let uz = bitcast<u32>(iz);
  h = fnv_byte(h, ux); h = fnv_byte(h, ux >> 8u); h = fnv_byte(h, ux >> 16u); h = fnv_byte(h, ux >> 24u);
  h = fnv_byte(h, uz); h = fnv_byte(h, uz >> 8u); h = fnv_byte(h, uz >> 16u); h = fnv_byte(h, uz >> 24u);
  return f32(h.x >> 8u) / 16777216.0;        // bits 40..63 of the 64-bit hash, in [0, 1)
}
fn surface_albedo(m: Mat, p: vec3<f32>) -> vec3<f32> {
  if (m.checker < 0.5) { return m.albedo; }
  let ix = i32(floor(p.x)); let iz = i32(floor(p.z));
  var base = vec3<f32>(0.20); if (((ix + iz) & 1) == 0) { base = vec3<f32>(0.92); }
  let tint = hash_cell(ix, iz) * 0.06 - 0.03;
  return max((base + vec3<f32>(tint)) * m.albedo, vec3<f32>(0.0));
}
fn aces(x: vec3<f32>) -> vec3<f32> { let a = max(x, vec3<f32>(0.0)); return clamp((a*(2.51*a + 0.03)) / (a*(2.43*a + 0.59) + 0.14), vec3<f32>(0.0), vec3<f32>(1.0)); }
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
    var base = albedo * (ibl(n) + AMBIENT) * occ;
    base = base + direct_lighting(p, n, v, albedo, m);
    // The CPU recursion computes fog(base*(1-fr) + refl*fr + emissive) at every level: emissive
    // is added AFTER the Fresnel blend (a glowing reflective surface never dims at grazing
    // angles), and the reflected radiance is fogged along every segment it traveled. Unrolled
    // iteratively: emit this leg's non-reflected share now, and fold BOTH the Fresnel weight and
    // this leg's fog transmittance (1-fa) into the attenuation the next leg inherits.
    let fa = clamp(1.0 - exp(-t * FOG_DENSITY), 0.0, 1.0);
    let fr = select(0.0, fresnel(max(dot(n, v), 0.0), m.refl), m.refl > 0.0);
    if (b == maxb || m.refl <= 0.0) {
      col = col + atten * ((base + m.emissive) * (1.0 - fa) + FOG * fa);
      break;
    }
    col = col + atten * ((base * (1.0 - fr) + m.emissive) * (1.0 - fa) + FOG * fa);
    atten = atten * (fr * (1.0 - fa));
    ro = p + n*0.02; rd = reflect(rd, n);
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
