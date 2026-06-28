//! mm3e-orchestrator — the single orchestrator. ALL policy, no mechanism.
//!
//! It owns the scene graph (which primitives exist, how they combine via CSG), the lights,
//! the camera, and every rendering decision: how many anti-aliasing samples, how many
//! reflection bounces, whether shadows and AO are on, the sky, the fog. It drives `mm3e-kit`;
//! it never computes a distance, a normal, or a tone-map itself.
//!
//! The pipeline mirrors MMPE exactly, one dimension up:
//!   scan pixels → project each into a camera ray → fold the ray down to a hit (sphere-trace)
//!   → combine lights into radiance → order/compose reflection + fog → tone-map → put pixel.

pub mod anim;
pub mod gi;
pub mod particles;
pub mod physics;
pub mod post;
pub mod reproject;
pub mod scene_io;

use mm3e_kit::{
    atoms,
    camera::Camera,
    color::{Material, Rgba},
    framebuffer::Framebuffer,
    march::{Hit, Marcher, Ray},
    sdf::{self, Field},
    shade,
    vec::{Transform, Vec3},
};

use gi::GiVolume;
use post::Post;

/// What the renderer outputs. `Beauty` is the lit image; the rest are arbitrary-output-variable
/// (AOV) debug passes — the tooling a real engine ships for inspecting a frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMode {
    Beauty,
    /// World-space normals mapped to RGB.
    Normal,
    /// Linear depth (distance along the ray), near = white.
    Depth,
    /// Ambient occlusion only.
    Ao,
    /// Sphere-tracing cost: a heatmap of march steps per pixel (the key perf diagnostic).
    Steps,
    /// Base color (albedo) with no lighting.
    Albedo,
}

/// A bundle of render-quality knobs for **adaptive rendering**: turn it down while the camera
/// moves, then progressively up (via [`Quality::lerp`]) as it sits still. `apply` writes the knobs
/// (resolution + AA + bounces + marcher budgets) into a [`Scene`]. This is what lets the CPU path
/// be a fast previewer that refines to a ground-truth still, instead of a brute-force real-timer.
#[derive(Clone, Copy, Debug)]
pub struct Quality {
    pub width: u32,
    pub height: u32,
    pub aa: u32,
    pub bounces: u32,
    pub max_steps: u32,
    pub shadow_steps: u32,
    pub ao_samples: u32,
}

impl Quality {
    /// Fast preview: low resolution, no AA, no reflections, lean shadows, AO off.
    pub fn fast(width: u32, height: u32) -> Quality {
        Quality { width, height, aa: 1, bounces: 0, max_steps: 64, shadow_steps: 12, ao_samples: 0 }
    }
    /// A middle ground.
    pub fn balanced(width: u32, height: u32) -> Quality {
        Quality { width, height, aa: 1, bounces: 1, max_steps: 110, shadow_steps: 28, ao_samples: 3 }
    }
    /// Full quality — converged still frames / offline.
    pub fn full(width: u32, height: u32) -> Quality {
        Quality { width, height, aa: 2, bounces: 2, max_steps: 160, shadow_steps: 64, ao_samples: 5 }
    }
    /// Interpolate the budgets from `a` toward `b` by `t ∈ [0, 1]` (resolution is `b`'s). Used to
    /// step quality up over successive still frames.
    pub fn lerp(a: Quality, b: Quality, t: f32) -> Quality {
        let t = t.clamp(0.0, 1.0);
        let mix = |x: u32, y: u32| (x as f32 + (y as f32 - x as f32) * t).round() as u32;
        Quality {
            width: b.width,
            height: b.height,
            aa: mix(a.aa, b.aa),
            bounces: mix(a.bounces, b.bounces),
            max_steps: mix(a.max_steps, b.max_steps),
            shadow_steps: mix(a.shadow_steps, b.shadow_steps),
            ao_samples: mix(a.ao_samples, b.ao_samples),
        }
    }
    /// Write these knobs into `scene`.
    pub fn apply(&self, scene: &mut Scene) {
        scene.width = self.width;
        scene.height = self.height;
        scene.aa = self.aa;
        scene.bounces = self.bounces;
        scene.marcher.max_steps = self.max_steps;
        scene.marcher.shadow_steps = self.shadow_steps;
        scene.marcher.ao_samples = self.ao_samples;
    }
}

// ----------------------------------------------------------------------------
// Scene description (policy: what exists in the world)
// ----------------------------------------------------------------------------

/// A primitive's local-space shape. The orchestrator picks these; the kit only evaluates them.
#[derive(Clone, Copy, Debug)]
pub enum Prim {
    Sphere { r: f32 },
    Box { half: Vec3 },
    RoundBox { half: Vec3, radius: f32 },
    Torus { major: f32, minor: f32 },
    Cylinder { h: f32, r: f32 },
    Capsule { a: Vec3, b: Vec3, r: f32 },
    Cone { r1: f32, r2: f32, h: f32 },
    Ellipsoid { r: Vec3 },
    Octahedron { s: f32 },
    HexPrism { r: f32, h: f32 },
    Plane { n: Vec3, h: f32 },
}

impl Prim {
    fn distance(&self, local: Vec3) -> f32 {
        match *self {
            Prim::Sphere { r } => sdf::sphere(local, r),
            Prim::Box { half } => sdf::boxed(local, half),
            Prim::RoundBox { half, radius } => sdf::rounded_box(local, half, radius),
            Prim::Torus { major, minor } => sdf::torus(local, major, minor),
            Prim::Cylinder { h, r } => sdf::cylinder(local, h, r),
            Prim::Capsule { a, b, r } => sdf::capsule(local, a, b, r),
            Prim::Cone { r1, r2, h } => sdf::round_cone(local, r1, r2, h),
            Prim::Ellipsoid { r } => sdf::ellipsoid(local, r),
            Prim::Octahedron { s } => sdf::octahedron(local, s),
            Prim::HexPrism { r, h } => sdf::hex_prism(local, r, h),
            Prim::Plane { n, h } => sdf::plane(local, n, h),
        }
    }

    /// A conservative local-space bounding sphere `(center, radius)`, or `None` if unbounded.
    fn local_bound(&self) -> Option<(Vec3, f32)> {
        match *self {
            Prim::Sphere { r } => Some((Vec3::ZERO, r)),
            Prim::Box { half } => Some((Vec3::ZERO, half.length())),
            Prim::RoundBox { half, .. } => Some((Vec3::ZERO, half.length())),
            Prim::Torus { major, minor } => Some((Vec3::ZERO, major + minor)),
            Prim::Cylinder { h, r } => Some((Vec3::ZERO, (h * h + r * r).sqrt())),
            Prim::Capsule { a, b, r } => Some(((a + b).scale(0.5), (a - b).scale(0.5).length() + r)),
            Prim::Cone { r1, r2, h } => Some((Vec3::new(0.0, h * 0.5, 0.0), (h * 0.5 + r1.max(r2)).max(r1).max(r2))),
            Prim::Ellipsoid { r } => Some((Vec3::ZERO, r.x.max(r.y).max(r.z))),
            Prim::Octahedron { s } => Some((Vec3::ZERO, s)),
            Prim::HexPrism { r, h } => Some((Vec3::ZERO, (r * r + h * h).sqrt())),
            Prim::Plane { .. } => None,
        }
    }
}

/// How an object folds into the accumulating world field.
#[derive(Clone, Copy, Debug)]
pub enum Combine {
    /// Hard boolean union with everything placed so far.
    Union,
    /// Organic smooth-min union with blend radius `k`.
    Smooth(f32),
    /// Carve this object *out of* the field built so far.
    Subtract,
}

/// Domain modifiers applied to a primitive — the SDF-modeler vocabulary. Point ops (mirror →
/// elongate → repeat → twist → bend) reshape the query domain before the primitive; distance
/// ops (round, onion) reshape its result. `repeat`/`bend` make a shape unbounded or non-convex
/// enough that the bounding-sphere prune is disabled for it.
#[derive(Clone, Copy, Debug, Default)]
pub struct Modifiers {
    pub mirror: [bool; 3],
    pub repeat: Vec3,
    pub twist: f32,
    pub bend: f32,
    pub elongate: Vec3,
    pub round: f32,
    pub onion: f32,
}

impl Modifiers {
    fn any_mirror(&self) -> bool {
        self.mirror[0] || self.mirror[1] || self.mirror[2]
    }
}

/// One placed object: a shape, where/how big it is, its material, how it joins the world, and
/// any domain modifiers.
#[derive(Clone, Copy, Debug)]
pub struct Object {
    pub prim: Prim,
    pub xform: Transform,
    pub mat: u32,
    pub combine: Combine,
    pub mods: Modifiers,
}

impl Object {
    pub fn new(prim: Prim, xform: Transform, mat: u32) -> Object {
        Object { prim, xform, mat, combine: Combine::Union, mods: Modifiers::default() }
    }
    pub fn smooth(mut self, k: f32) -> Object {
        self.combine = Combine::Smooth(k);
        self
    }
    pub fn subtract(mut self) -> Object {
        self.combine = Combine::Subtract;
        self
    }
    pub fn mirror(mut self, x: bool, y: bool, z: bool) -> Object {
        self.mods.mirror = [x, y, z];
        self
    }
    pub fn repeat(mut self, period: Vec3) -> Object {
        self.mods.repeat = period;
        self
    }
    pub fn twist(mut self, k: f32) -> Object {
        self.mods.twist = k;
        self
    }
    pub fn bend(mut self, k: f32) -> Object {
        self.mods.bend = k;
        self
    }
    pub fn elongate(mut self, h: Vec3) -> Object {
        self.mods.elongate = h;
        self
    }
    pub fn round(mut self, r: f32) -> Object {
        self.mods.round = r;
        self
    }
    pub fn onion(mut self, thickness: f32) -> Object {
        self.mods.onion = thickness;
        self
    }

    /// This object's contribution to the world field at world point `p`, with modifiers applied.
    fn field(&self, p: Vec3) -> Field {
        let mut local = self.xform.to_local(p);
        let m = &self.mods;
        if m.any_mirror() {
            local = sdf::op_mirror(local, m.mirror[0], m.mirror[1], m.mirror[2]);
        }
        if m.elongate != Vec3::ZERO {
            local = sdf::op_elongate(local, m.elongate);
        }
        if m.repeat != Vec3::ZERO {
            local = sdf::op_repeat(local, m.repeat);
        }
        if m.twist != 0.0 {
            local = sdf::op_twist(local, m.twist);
        }
        if m.bend != 0.0 {
            local = sdf::op_bend(local, m.bend);
        }
        // Rotation+translation is an isometry, so the world distance is `scale · sdf(local)`.
        let mut d = self.prim.distance(local) * self.xform.scale;
        if m.round != 0.0 {
            d = sdf::op_round(d, m.round);
        }
        if m.onion != 0.0 {
            d = sdf::op_onion(d, m.onion);
        }
        Field::new(d, self.mat)
    }

    /// A conservative world-space bounding sphere `(center, radius)`, or `None` for unbounded
    /// shapes (an infinite plane, or an infinitely repeated/bent domain). Used to prune the
    /// field fold: outside the sphere the cheap lower bound `|p − center| − radius` is a valid
    /// distance underestimate, so the expensive exact SDF can be skipped without ever letting
    /// the sphere tracer overshoot a surface.
    fn world_bound(&self) -> Option<(Vec3, f32)> {
        let (mut center, mut radius) = self.prim.local_bound()?;
        let m = &self.mods;
        if m.repeat != Vec3::ZERO || m.bend != 0.0 {
            return None; // unbounded / non-convex in a way the sphere can't conservatively cover
        }
        if m.any_mirror() {
            radius += center.length(); // reflected copies straddle the origin
            center = Vec3::ZERO;
        }
        if m.twist != 0.0 {
            // Twist preserves cylindrical radius and y, so recenter the bound onto the Y axis to
            // cover every twisted image of an off-axis shape (else it could be wrongly pruned).
            radius += (center.x * center.x + center.z * center.z).sqrt();
            center.x = 0.0;
            center.z = 0.0;
        }
        radius += m.elongate.length();
        let world_center = self.xform.pos + self.xform.rot.mul_vec(center.scale(self.xform.scale));
        // `round` and `onion` grow the outer surface in world units (applied after the scale).
        Some((world_center, radius * self.xform.scale + m.round.max(0.0) + m.onion.abs()))
    }
}

/// A light. Directional lights model the sun (parallel rays, no falloff); point/sphere lights
/// model local fixtures (inverse-square falloff). A non-zero `radius` makes the source an area
/// light: it softens the shadow penumbra (sun angular size, or sphere-light radius).
#[derive(Clone, Copy, Debug)]
pub struct Light {
    /// For directional: the unit direction *toward* the light. For point/sphere: its position.
    pub vec: Vec3,
    pub color: Vec3,
    pub directional: bool,
    /// Angular radius (directional) or world radius (point) of the source; 0 = a hard point.
    pub radius: f32,
}

impl Light {
    pub fn directional(toward: Vec3, color: Vec3) -> Light {
        Light { vec: toward.normalize(), color, directional: true, radius: 0.0 }
    }
    pub fn point(pos: Vec3, color: Vec3) -> Light {
        Light { vec: pos, color, directional: false, radius: 0.0 }
    }
    /// A spherical area light of world `radius` at `pos` — gives soft, distance-widened shadows.
    pub fn sphere(pos: Vec3, color: Vec3, radius: f32) -> Light {
        Light { vec: pos, color, directional: false, radius }
    }
    /// Soften a directional light's shadows by giving the sun an angular `radius` (radians).
    pub fn soft(mut self, radius: f32) -> Light {
        self.radius = radius;
        self
    }
    /// Unit direction from surface point `p` toward this light, and distance to it.
    fn toward(&self, p: Vec3) -> (Vec3, f32) {
        if self.directional {
            (self.vec, f32::INFINITY)
        } else {
            let d = self.vec - p;
            let len = d.length();
            (d.scale(1.0 / len.max(1e-6)), len)
        }
    }
    /// Inverse-square falloff for local lights; directional lights do not attenuate.
    fn attenuation(&self, dist: f32) -> f32 {
        if self.directional {
            1.0
        } else {
            1.0 / dist.max(0.05).powi(2)
        }
    }
    /// Penumbra hardness `k` for the soft-shadow trace (larger = sharper).
    fn shadow_k(&self, dist: f32) -> f32 {
        if self.radius <= 0.0 {
            return 24.0;
        }
        if self.directional {
            (1.0 / self.radius).clamp(2.0, 64.0)
        } else {
            (dist / self.radius).clamp(2.0, 64.0)
        }
    }
}

/// Everything the renderer needs: geometry, materials, lights, atmosphere, and quality knobs.
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub objects: Vec<Object>,
    pub materials: Vec<Material>,
    pub lights: Vec<Light>,
    pub sun_dir: Vec3,
    /// Constant ambient floor (a small base term added under the sky/GI ambient).
    pub ambient: Vec3,
    /// Strength of the diffuse image-based ambient gathered from the procedural sky.
    pub sky_ambient: Vec3,
    pub fog: Vec3,
    pub fog_density: f32,
    pub aa: u32,
    pub bounces: u32,
    pub shadows: bool,
    pub ao: bool,
    pub marcher: Marcher,
    /// Post-processing (exposure, bloom, tone-map) applied to the linear-HDR frame.
    pub post: Post,
    /// Which output to render (beauty or a debug AOV).
    pub mode: RenderMode,
    /// Optional baked global-illumination volume (see [`Scene::bake_gi`]).
    pub gi: Option<GiVolume>,
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Scene {
        Scene {
            width,
            height,
            objects: Vec::new(),
            materials: vec![Material::default()],
            lights: Vec::new(),
            sun_dir: Vec3::new(0.6, 0.7, 0.4).normalize(),
            ambient: Vec3::splat(0.02),
            sky_ambient: Vec3::splat(0.4),
            fog: Vec3::new(0.62, 0.72, 0.86),
            fog_density: 0.012,
            aa: 2,
            bounces: 1,
            shadows: true,
            ao: true,
            marcher: Marcher::default(),
            post: Post::default(),
            mode: RenderMode::Beauty,
            gi: None,
        }
    }

    /// Register a material and return its id, for objects to reference.
    pub fn material(&mut self, m: Material) -> u32 {
        self.materials.push(m);
        (self.materials.len() - 1) as u32
    }
    pub fn add(&mut self, obj: Object) {
        self.objects.push(obj);
    }
    pub fn light(&mut self, l: Light) {
        self.lights.push(l);
    }

    /// Bake a global-illumination probe volume over the scene, replacing flat ambient with real
    /// one-bounce color bleed gathered from the world field. Expensive (offline); call once after
    /// the scene is assembled. `dims` is the probe grid resolution, `samples` the rays per cube
    /// face (1 = axis only, ≥5 = a cone). No-op for an empty / unbounded scene.
    pub fn bake_gi(&mut self, dims: (usize, usize, usize), samples: u32) {
        let spheres: Vec<(Vec3, f32)> = self.objects.iter().filter_map(Object::world_bound).collect();
        let Some((lo, hi)) = gi::bounds_of(&spheres, 1.5) else { return };
        // Build the volume in a block so the immutable borrows of `self` (the field + gather
        // closures) end before the mutable assignment to `self.gi`.
        let vol = {
            let field = self.world();
            let gather = |o: Vec3, d: Vec3| self.gather(&field, o, d);
            GiVolume::bake(lo, hi, dims, samples, &gather)
        };
        self.gi = Some(vol);
    }

    /// One-bounce incoming radiance along a ray, used while baking GI: march, then shade the hit
    /// with direct lighting only (no shadows, for speed) or return the sky on a miss.
    fn gather(&self, field: &dyn Fn(Vec3) -> Field, origin: Vec3, dir: Vec3) -> Vec3 {
        let hit = self.marcher.march(field, &Ray { origin, dir });
        if !hit.hit {
            return shade::sky_diffuse(dir, self.sun_dir);
        }
        let m = self.materials[hit.mat as usize % self.materials.len()];
        let albedo = surface_albedo(&m, hit.pos);
        let n = hit.normal;
        let mut c = m.emissive;
        for light in &self.lights {
            let (l_dir, l_dist) = light.toward(hit.pos);
            let ndl = shade::lambert(n, l_dir);
            if ndl > 0.0 {
                let w = ndl * light.attenuation(l_dist) / std::f32::consts::PI;
                c = c + albedo.cmul(light.color).scale(w);
            }
        }
        c
    }

    /// The world field closure (static geometry) — public so physics and custom tools can query
    /// the same `Fn(Vec3) -> Field` the renderer marches (e.g. distance/normal for collision).
    pub fn field(&self) -> impl Fn(Vec3) -> Field + '_ {
        self.world()
    }

    /// The world field: a bounded `fold` of every object's contribution into one distance +
    /// material — the closure the kit's sphere tracer marches through. Built once per frame.
    ///
    /// Each object carries a conservative bounding sphere; when the sample point is well outside
    /// it (`lower_bound > slack`), the cheap lower bound replaces the exact SDF. Because the
    /// lower bound never exceeds the true distance, every CSG op stays a safe underestimate and
    /// the tracer never overshoots — this is the O(1) early-out that makes scenes scale and the
    /// substrate a full BVH would later sit on. The `slack` covers the widest smooth-blend so
    /// near-surface blends always use the exact field.
    fn world(&self) -> impl Fn(Vec3) -> Field + '_ {
        let bounds: Vec<Option<(Vec3, f32)>> = self.objects.iter().map(Object::world_bound).collect();
        let slack = self
            .objects
            .iter()
            .map(|o| match o.combine {
                Combine::Smooth(k) => k,
                _ => 0.0,
            })
            .fold(0.0_f32, f32::max)
            + 0.1;
        move |p: Vec3| {
            let mut acc = Field::FAR;
            let mut first = true;
            for (i, obj) in self.objects.iter().enumerate() {
                let f = match bounds[i] {
                    Some((c, r)) => {
                        let lower = (p - c).length() - r;
                        if lower > slack {
                            Field::new(lower, obj.mat)
                        } else {
                            obj.field(p)
                        }
                    }
                    None => obj.field(p),
                };
                // The first placed object seeds the field; after that, its combine mode rules.
                if first {
                    first = false;
                    acc = f;
                    continue;
                }
                acc = match obj.combine {
                    Combine::Union => sdf::union(acc, f),
                    Combine::Smooth(k) => sdf::smooth_union(acc, f, k),
                    Combine::Subtract => sdf::subtract(acc, f),
                };
            }
            acc
        }
    }
}

// ----------------------------------------------------------------------------
// Rendering (policy: how the world becomes pixels)
// ----------------------------------------------------------------------------

/// Render `scene` from `camera` to a framebuffer, parallelized across CPU cores with scoped std
/// threads (no external crate). In `Beauty` mode each thread sphere-traces a band of rows into a
/// **linear-HDR** buffer that a post pass (bloom, exposure, ACES, gamma) then resolves; in a
/// debug AOV mode the band is shaded straight to display pixels. Deterministic in thread count.
pub fn render(scene: &Scene, camera: &Camera) -> Framebuffer {
    let (w, h) = (scene.width, scene.height);

    if scene.mode != RenderMode::Beauty {
        return render_bands(scene, camera, |sc, f, cam, x, y| aov_pixel(sc, f, cam, x, y));
    }

    // Beauty path: each thread builds the *concrete* world-field closure and shades its band with
    // it, so the marcher and the per-primitive loop monomorphize and inline (no `&dyn Fn` call in
    // the ~8M-evals-per-frame hot path). Results are linear HDR, resolved through the post pass.
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).max(1);
    let band = (h as usize).div_ceil(threads);
    let bands: Vec<(u32, Vec<Vec3>)> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..threads)
            .map(|ti| {
                let y0 = (ti * band) as u32;
                let y1 = (((ti + 1) * band) as u32).min(h);
                s.spawn(move || {
                    let field = scene.world();
                    let mut rows = Vec::with_capacity(((y1.saturating_sub(y0)) * w) as usize);
                    for y in y0..y1 {
                        for x in 0..w {
                            rows.push(shade_pixel(scene, &field, camera, x, y));
                        }
                    }
                    (y0, rows)
                })
            })
            .collect();
        handles.into_iter().map(|hd| hd.join().unwrap()).collect()
    });

    let mut hdr = vec![Vec3::ZERO; (w * h) as usize];
    for (y0, rows) in bands {
        for (i, px) in rows.into_iter().enumerate() {
            hdr[(y0 * w) as usize + i] = px;
        }
    }
    post::resolve(&hdr, w, h, &scene.post)
}

/// Render a **G-buffer frame** — display colour + primary-ray depth + the camera — for reprojection
/// ([`reproject`]). One sample/pixel, no post pass: it is the base the cheap fake frames warp.
pub fn render_gbuffer(scene: &Scene, camera: &Camera, movers: &[(Vec3, f32)]) -> reproject::GFrame {
    type GBand = (u32, Vec<([u8; 4], f32, i32)>); // (first row, [color, depth, obj] per pixel)
    let (w, h) = (scene.width, scene.height);
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).max(1);
    let band = (h as usize).div_ceil(threads);
    let bands: Vec<GBand> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..threads)
            .map(|ti| {
                let y0 = (ti * band) as u32;
                let y1 = (((ti + 1) * band) as u32).min(h);
                s.spawn(move || {
                    let field = scene.world();
                    let mut rows = Vec::with_capacity(((y1.saturating_sub(y0)) * w) as usize);
                    for y in y0..y1 {
                        for x in 0..w {
                            let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
                            let hit = scene.marcher.march(&field, &ray);
                            let (hdr, depth, obj) = if hit.hit {
                                let o = movers
                                    .iter()
                                    .position(|(c, r)| (hit.pos - *c).length() <= r + 0.05)
                                    .map(|k| k as i32)
                                    .unwrap_or(-1);
                                (shade_hit(scene, &field, &hit, &ray, 0), hit.t, o)
                            } else {
                                (shade::sky(ray.dir, scene.sun_dir), f32::INFINITY, -1)
                            };
                            let d = shade::gamma(shade::aces(hdr.scale(scene.post.exposure))).clamp01();
                            let px =
                                [(d.x * 255.0 + 0.5) as u8, (d.y * 255.0 + 0.5) as u8, (d.z * 255.0 + 0.5) as u8, 255];
                            rows.push((px, depth, obj));
                        }
                    }
                    (y0, rows)
                })
            })
            .collect();
        handles.into_iter().map(|hd| hd.join().unwrap()).collect()
    });
    let mut color = vec![[0u8; 4]; (w * h) as usize];
    let mut depth = vec![f32::INFINITY; (w * h) as usize];
    let mut obj = vec![-1i32; (w * h) as usize];
    for (y0, rows) in bands {
        for (i, (c, dd, o)) in rows.into_iter().enumerate() {
            let idx = (y0 * w) as usize + i;
            color[idx] = c;
            depth[idx] = dd;
            obj[idx] = o;
        }
    }
    reproject::GFrame { width: w, height: h, color, depth, obj, camera: *camera }
}

/// Checkerboard (interlaced) render — ray-trace only the pixels where `(x + y)` is even and fill
/// the rest by averaging their four (always-rendered) neighbours. Halves the per-frame ray work
/// for a small softening, ideal for the camera-moving phase of an interactive previewer. Resolves
/// straight to display pixels (no HDR post pass), since it is a preview path.
pub fn render_checkerboard(scene: &Scene, camera: &Camera) -> Framebuffer {
    let (w, h) = (scene.width, scene.height);
    // Pass 1: shade the even pixels; mark the odd ones unfilled (alpha 0).
    let mut fb = render_bands(scene, camera, |sc, f, cam, x, y| {
        if (x + y) % 2 == 0 {
            let c = shade_pixel(sc, f, cam, x, y).scale(sc.post.exposure);
            Rgba::from_vec3(shade::gamma(shade::aces(c)))
        } else {
            Rgba::new(0.0, 0.0, 0.0, 0.0)
        }
    });
    // Pass 2: fill the odd pixels from their even neighbours (which are all already shaded).
    for y in 0..h {
        for x in 0..w {
            if (x + y) % 2 == 0 {
                continue;
            }
            let mut acc = Vec3::ZERO;
            let mut n = 0.0f32;
            for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && ny >= 0 && (nx as u32) < w && (ny as u32) < h {
                    let p = fb.pixel(nx as u32, ny as u32);
                    acc = acc + Vec3::new(p.r, p.g, p.b);
                    n += 1.0;
                }
            }
            if n > 0.0 {
                fb.put(x, y, Rgba::from_vec3(acc.scale(1.0 / n)));
            }
        }
    }
    fb
}

/// Shade every row-band in parallel (scoped std threads), one pixel per `shade` callback, and
/// stitch the bands into a framebuffer. Shared by the beauty and AOV render paths.
fn render_bands<F>(scene: &Scene, camera: &Camera, shade: F) -> Framebuffer
where
    F: Fn(&Scene, &dyn Fn(Vec3) -> Field, &Camera, u32, u32) -> Rgba + Sync,
{
    let (w, h) = (scene.width, scene.height);
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).max(1);
    let band = (h as usize).div_ceil(threads);
    let shade = &shade;

    let bands: Vec<(u32, Vec<Rgba>)> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..threads)
            .map(|ti| {
                let y0 = (ti * band) as u32;
                let y1 = (((ti + 1) * band) as u32).min(h);
                s.spawn(move || {
                    let field = scene.world();
                    let mut rows = Vec::with_capacity(((y1.saturating_sub(y0)) * w) as usize);
                    for y in y0..y1 {
                        for x in 0..w {
                            rows.push(shade(scene, &field, camera, x, y));
                        }
                    }
                    (y0, rows)
                })
            })
            .collect();
        handles.into_iter().map(|hd| hd.join().unwrap()).collect()
    });

    let mut fb = Framebuffer::new(w, h, Rgba::new(0.0, 0.0, 0.0, 1.0));
    for (y0, rows) in bands {
        for (i, px) in rows.into_iter().enumerate() {
            fb.put(i as u32 % w, y0 + i as u32 / w, px);
        }
    }
    fb
}

/// Shade one pixel for a debug AOV (single sample, written straight to display — no post pass).
fn aov_pixel(scene: &Scene, field: &dyn Fn(Vec3) -> Field, camera: &Camera, x: u32, y: u32) -> Rgba {
    let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, scene.width, scene.height);
    let hit = scene.marcher.march(field, &ray);
    let v = match scene.mode {
        RenderMode::Normal => {
            if hit.hit {
                hit.normal.scale(0.5) + Vec3::splat(0.5)
            } else {
                Vec3::ZERO
            }
        }
        RenderMode::Depth => {
            let d = (hit.t / scene.marcher.max_dist).clamp(0.0, 1.0);
            Vec3::splat(1.0 - d)
        }
        RenderMode::Ao => {
            if hit.hit {
                Vec3::splat(scene.marcher.ambient_occlusion(field, hit.pos, hit.normal))
            } else {
                Vec3::ONE
            }
        }
        RenderMode::Steps => heatmap(hit.steps as f32 / scene.marcher.max_steps as f32),
        RenderMode::Albedo => {
            if hit.hit {
                surface_albedo(&scene.materials[hit.mat as usize % scene.materials.len()], hit.pos)
            } else {
                Vec3::ZERO
            }
        }
        RenderMode::Beauty => Vec3::ZERO,
    };
    Rgba::from_vec3(v)
}

/// Shade one pixel into **linear HDR**: supersample on an n×n sub-pixel grid (the 3-D analog of
/// MMPE's analytic AA band) and return the averaged radiance. Tone-mapping happens later, in the
/// post pass, so the float frame stays available for bloom/exposure.
fn shade_pixel<F: Fn(Vec3) -> Field + ?Sized>(scene: &Scene, field: &F, camera: &Camera, x: u32, y: u32) -> Vec3 {
    let n = scene.aa.max(1);
    let inv_samples = 1.0 / (n * n) as f32;
    let mut acc = Vec3::ZERO;
    for sy in 0..n {
        for sx in 0..n {
            let ox = (sx as f32 + 0.5) / n as f32;
            let oy = (sy as f32 + 0.5) / n as f32;
            let ray = camera.ray(x as f32 + ox, y as f32 + oy, scene.width, scene.height);
            acc = acc + trace(scene, field, &ray, 0);
        }
    }
    acc.scale(inv_samples)
}

/// Trace one ray and return its linear HDR radiance. Recurses for mirror reflections. Generic over
/// the field type so the beauty path monomorphizes (the primitive loop inlines into the marcher).
fn trace<F: Fn(Vec3) -> Field + ?Sized>(scene: &Scene, field: &F, ray: &Ray, depth: u32) -> Vec3 {
    let hit = scene.marcher.march(field, ray);
    if !hit.hit {
        return shade::sky(ray.dir, scene.sun_dir);
    }
    shade_hit(scene, field, &hit, ray, depth)
}

/// Shade a confirmed surface hit — the post-hit half of [`trace`], factored out so the G-buffer
/// renderer (reprojection) can capture the primary depth without marching the ray a second time.
fn shade_hit<F: Fn(Vec3) -> Field + ?Sized>(scene: &Scene, field: &F, hit: &Hit, ray: &Ray, depth: u32) -> Vec3 {
    let m = scene.materials[hit.mat as usize % scene.materials.len()];
    let albedo = surface_albedo(&m, hit.pos);
    let normal = hit.normal;
    let view = ray.dir.scale(-1.0);

    // Ambient term: diffuse image-based lighting from the sky + baked GI + a constant floor,
    // all gated by ambient occlusion. This replaces the old flat ambient with real environment
    // and bounce light.
    let occ = if scene.ao { scene.marcher.ambient_occlusion(field, hit.pos, normal) } else { 1.0 };
    let gi = scene.gi.as_ref().map(|g| g.sample(hit.pos, normal)).unwrap_or(Vec3::ZERO);
    let ambient_irr = ibl_irradiance(scene, normal) + gi + scene.ambient;
    let mut radiance = albedo.cmul(ambient_irr).scale(occ);

    // Direct lighting: `combine` (fold) each light through the Cook-Torrance GGX BRDF with
    // inverse-square falloff and soft shadows. Lighting is additive, so no per-pixel ordering is
    // needed (the old `order` call allocated a Vec for every shaded pixel — pure waste).
    for light in &scene.lights {
        let (l_dir, l_dist) = light.toward(hit.pos);
        if shade::lambert(normal, l_dir) <= 0.0 {
            continue;
        }
        // Lift the shadow ray off the surface to avoid self-intersection acne.
        let shadow = if scene.shadows {
            let max_t = l_dist.min(scene.marcher.max_dist);
            scene.marcher.soft_shadow(field, hit.pos + normal.scale(0.01), l_dir, max_t, light.shadow_k(l_dist))
        } else {
            1.0
        };
        if shadow <= 0.0 {
            continue;
        }
        let surface = shade::brdf(normal, l_dir, view, albedo, m.metallic, m.roughness, m.specular);
        radiance = radiance + surface.cmul(light.color).scale(shadow * light.attenuation(l_dist));
    }

    // One-bounce environment reflection, layered by a Fresnel-modulated reflectivity. Energy is
    // conserved (the diffuse/direct base is attenuated by `1 − fr` and the reflection adds `fr`),
    // and emissive is added *after* so a reflective light source is never erased at grazing angles.
    if depth < scene.bounces && m.reflectivity > 0.0 {
        let cos = normal.dot(view).max(0.0);
        let fr = shade::fresnel_schlick(cos, m.reflectivity);
        let rdir = ray.dir.reflect(normal).normalize();
        let rorigin = hit.pos + normal.scale(0.02);
        let refl = trace(scene, field, &Ray { origin: rorigin, dir: rdir }, depth + 1);
        radiance = radiance.scale(1.0 - fr) + refl.scale(fr);
    }

    radiance = radiance + m.emissive;

    // Distance fog blends far geometry into the atmosphere.
    shade::apply_fog(radiance, scene.fog, hit.t, scene.fog_density)
}

/// Resolve a material's albedo at a point, evaluating the procedural checker when flagged.
/// The checker cell parity is the `compare`/`order` idea on a lattice; a `hash` of the cell
/// adds a faint per-tile tint so the floor never looks mechanically flat.
fn surface_albedo(m: &Material, p: Vec3) -> Vec3 {
    if !m.checker {
        return m.albedo;
    }
    let ix = p.x.floor() as i32;
    let iz = p.z.floor() as i32;
    let parity = (ix + iz).rem_euclid(2) == 0;
    let base = if parity { Vec3::splat(0.92) } else { Vec3::splat(0.20) };
    let tint = atoms::hash_cell(ix, iz) * 0.06 - 0.03;
    (base + Vec3::splat(tint)).cmul(m.albedo).max_scalar(0.0)
}

/// Diffuse image-based irradiance from the procedural sky: a few cosine-weighted hemisphere
/// samples around the normal, scaled by `sky_ambient`. The `combine` atom over `sky_diffuse`.
fn ibl_irradiance(scene: &Scene, n: Vec3) -> Vec3 {
    let up = if n.y.abs() > 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    let t1 = n.cross(up).normalize();
    let t2 = n.cross(t1).normalize();
    let dirs = [
        n,
        (n.scale(0.6) + t1.scale(0.6)).normalize(),
        (n.scale(0.6) - t1.scale(0.6)).normalize(),
        (n.scale(0.6) + t2.scale(0.6)).normalize(),
        (n.scale(0.6) - t2.scale(0.6)).normalize(),
    ];
    let mut acc = Vec3::ZERO;
    let mut w = 0.0;
    for d in dirs {
        let cw = d.dot(n).max(0.0);
        acc = acc + shade::sky_diffuse(d, scene.sun_dir).scale(cw);
        w += cw;
    }
    acc.scale(1.0 / w.max(1e-4)).cmul(scene.sky_ambient)
}

/// A perceptual blue→green→yellow→red heatmap for `t` in [0, 1] (used by the step-count AOV).
fn heatmap(t: f32) -> Vec3 {
    let t = t.clamp(0.0, 1.0);
    let r = (t * 3.0 - 1.5).clamp(0.0, 1.0);
    let g = (1.0 - (t * 3.0 - 1.3).abs()).clamp(0.0, 1.0);
    let b = (1.0 - t * 2.2).clamp(0.0, 1.0);
    Vec3::new(r, g, b)
}

// ----------------------------------------------------------------------------
// Convenience: an orbiting camera for turntables and quick scene framing.
// ----------------------------------------------------------------------------

/// A camera orbiting `target` at `radius`, `yaw`/`pitch` in radians, FOV `fov_y` in radians.
pub fn orbit_camera(target: Vec3, radius: f32, yaw: f32, pitch: f32, fov_y: f32) -> Camera {
    let eye =
        target + Vec3::new(radius * yaw.cos() * pitch.cos(), radius * pitch.sin(), radius * yaw.sin() * pitch.cos());
    Camera::look_at(eye, target, Vec3::new(0.0, 1.0, 0.0), fov_y)
}
