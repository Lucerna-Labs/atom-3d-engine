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

use mm3e_kit::{
    atoms,
    camera::Camera,
    color::{Material, Rgba},
    framebuffer::Framebuffer,
    march::{Marcher, Ray},
    sdf::{self, Field},
    shade,
    vec::{Transform, Vec3},
};

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
            Prim::Plane { n, h } => sdf::plane(local, n, h),
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

/// One placed object: a shape, where/how big it is, its material, and how it joins the world.
#[derive(Clone, Copy, Debug)]
pub struct Object {
    pub prim: Prim,
    pub xform: Transform,
    pub mat: u32,
    pub combine: Combine,
}

impl Object {
    pub fn new(prim: Prim, xform: Transform, mat: u32) -> Object {
        Object { prim, xform, mat, combine: Combine::Union }
    }
    pub fn smooth(mut self, k: f32) -> Object {
        self.combine = Combine::Smooth(k);
        self
    }
    pub fn subtract(mut self) -> Object {
        self.combine = Combine::Subtract;
        self
    }

    /// This object's contribution to the world field at world point `p`.
    fn field(&self, p: Vec3) -> Field {
        let local = self.xform.to_local(p);
        // Rotation+translation is an isometry, so the world distance is `scale · sdf(local)`.
        Field::new(self.prim.distance(local) * self.xform.scale, self.mat)
    }
}

/// A light. Directional lights model the sun; point lights model local fixtures.
#[derive(Clone, Copy, Debug)]
pub struct Light {
    /// For directional: the direction *toward* the light. For point: the light's position.
    pub vec: Vec3,
    pub color: Vec3,
    pub directional: bool,
}

impl Light {
    pub fn directional(toward: Vec3, color: Vec3) -> Light {
        Light { vec: toward.normalize(), color, directional: true }
    }
    pub fn point(pos: Vec3, color: Vec3) -> Light {
        Light { vec: pos, color, directional: false }
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
    fn luminance(&self) -> f32 {
        self.color.x * 0.2126 + self.color.y * 0.7152 + self.color.z * 0.0722
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
    pub ambient: Vec3,
    pub fog: Vec3,
    pub fog_density: f32,
    pub aa: u32,
    pub bounces: u32,
    pub shadows: bool,
    pub ao: bool,
    pub marcher: Marcher,
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
            ambient: Vec3::splat(0.06),
            fog: Vec3::new(0.62, 0.72, 0.86),
            fog_density: 0.012,
            aa: 2,
            bounces: 1,
            shadows: true,
            ao: true,
            marcher: Marcher::default(),
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

    /// The world field: fold every object's contribution into one distance + material.
    /// This is the closure the kit's sphere tracer marches through. Built once per frame.
    fn world(&self) -> impl Fn(Vec3) -> Field + '_ {
        move |p: Vec3| {
            let mut first = true;
            atoms::fold(&self.objects, Field::FAR, |acc, obj| {
                let f = obj.field(p);
                // The first placed object seeds the field; a leading Subtract has nothing to
                // carve yet, so it too just seeds. After that, each object's combine mode rules.
                if first {
                    first = false;
                    return f;
                }
                match obj.combine {
                    Combine::Union => sdf::union(acc, f),
                    Combine::Smooth(k) => sdf::smooth_union(acc, f, k),
                    Combine::Subtract => sdf::subtract(acc, f),
                }
            })
        }
    }
}

// ----------------------------------------------------------------------------
// Rendering (policy: how the world becomes pixels)
// ----------------------------------------------------------------------------

/// Render `scene` from `camera` to a framebuffer, parallelized across CPU cores with scoped
/// std threads (no external crate). Each thread sphere-traces a contiguous band of rows; the
/// bands are then assembled into the framebuffer. Deterministic regardless of thread count.
pub fn render(scene: &Scene, camera: &Camera) -> Framebuffer {
    let (w, h) = (scene.width, scene.height);
    let clear = Rgba::from_vec3(shade::gamma(shade::aces(scene.fog)));
    let mut fb = Framebuffer::new(w, h, clear);

    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).max(1);
    let band = (h as usize).div_ceil(threads);

    // Each scoped thread renders rows [y0, y1) into its own buffer, then we stitch them in order.
    let bands: Vec<(u32, Vec<Rgba>)> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..threads)
            .map(|ti| {
                let y0 = (ti * band) as u32;
                let y1 = (((ti + 1) * band) as u32).min(h);
                s.spawn(move || {
                    // Build this thread's own world-field closure (cheap; borrows the scene).
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

    for (y0, rows) in bands {
        for (i, px) in rows.into_iter().enumerate() {
            let x = i as u32 % w;
            let y = y0 + i as u32 / w;
            fb.put(x, y, px);
        }
    }
    fb
}

/// Shade one pixel: supersample on an n×n sub-pixel grid (the 3-D analog of MMPE's analytic
/// AA band), then tone-map the accumulated linear radiance once.
fn shade_pixel(scene: &Scene, field: &dyn Fn(Vec3) -> Field, camera: &Camera, x: u32, y: u32) -> Rgba {
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
    let display = shade::gamma(shade::aces(acc.scale(inv_samples)));
    Rgba::from_vec3(display)
}

/// Trace one ray and return its linear HDR radiance. Recurses for mirror reflections.
fn trace(scene: &Scene, field: &dyn Fn(Vec3) -> Field, ray: &Ray, depth: u32) -> Vec3 {
    let hit = scene.marcher.march(field, ray);
    if !hit.hit {
        return shade::sky(ray.dir, scene.sun_dir);
    }

    let m = scene.materials[hit.mat as usize % scene.materials.len()];
    let albedo = surface_albedo(&m, hit.pos);
    let normal = hit.normal;
    let view = ray.dir.scale(-1.0);

    // Ambient term, attenuated by ambient occlusion.
    let occ = if scene.ao {
        scene.marcher.ambient_occlusion(field, hit.pos, normal)
    } else {
        1.0
    };
    let mut radiance = albedo.cmul(scene.ambient).scale(occ);

    // Direct lighting: `order` the lights brightest-first, then `combine` (fold) them in.
    for &li in atoms::order(&scene.lights, |l| l.luminance()).iter() {
        let light = scene.lights[li];
        let (l_dir, l_dist) = light.toward(hit.pos);
        let ndl = shade::lambert(normal, l_dir);
        if ndl <= 0.0 {
            continue;
        }
        // Lift the shadow ray off the surface to avoid self-intersection acne.
        let shadow = if scene.shadows {
            let max_t = l_dist.min(scene.marcher.max_dist);
            scene.marcher.soft_shadow(field, hit.pos + normal.scale(0.01), l_dir, max_t, 16.0)
        } else {
            1.0
        };
        if shadow <= 0.0 {
            continue;
        }
        let diffuse = albedo.cmul(light.color).scale(ndl);
        let spec = shade::specular(normal, l_dir, view, m.roughness) * m.specular;
        let specular = light.color.scale(spec);
        radiance = radiance + (diffuse + specular).scale(shadow);
    }

    radiance = radiance + m.emissive;

    // One-bounce mirror reflection, weighted by a Fresnel-modulated reflectivity.
    if depth < scene.bounces && m.reflectivity > 0.0 {
        let cos = normal.dot(view).max(0.0);
        let fr = shade::fresnel_schlick(cos, m.reflectivity);
        let rdir = ray.dir.reflect(normal).normalize();
        let rorigin = hit.pos + normal.scale(0.02);
        let refl = trace(scene, field, &Ray { origin: rorigin, dir: rdir }, depth + 1);
        radiance = radiance.mix(refl, fr);
    }

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

// ----------------------------------------------------------------------------
// Convenience: an orbiting camera for turntables and quick scene framing.
// ----------------------------------------------------------------------------

/// A camera orbiting `target` at `radius`, `yaw`/`pitch` in radians, FOV `fov_y` in radians.
pub fn orbit_camera(target: Vec3, radius: f32, yaw: f32, pitch: f32, fov_y: f32) -> Camera {
    let eye = target
        + Vec3::new(
            radius * yaw.cos() * pitch.cos(),
            radius * pitch.sin(),
            radius * yaw.sin() * pitch.cos(),
        );
    Camera::look_at(eye, target, Vec3::new(0.0, 1.0, 0.0), fov_y)
}
