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

mod accel;
pub mod anim;
pub mod appearance;
pub mod film;
pub mod gi;
pub mod mesh;
pub mod meshing;
pub mod particles;
pub mod physics;
pub mod post;
pub mod reproject;
pub mod rigid;
pub mod sampling;
pub mod scene_io;

use mm3e_kit::{
    atoms,
    camera::Camera,
    color::{Material, Rgba},
    dual::{self, Dual},
    framebuffer::Framebuffer,
    march::{Hit, Marcher, Ray},
    sdf::{self, Field},
    shade,
    surface::TriangleSurface,
    vec::{Transform, Vec3},
    volume::SdfVolume,
};

use gi::GiVolume;
use post::Post;

pub use film::{render_film, render_film_checked, render_film_with_threads, FilmFrame, Lens};

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
    /// Screen-footprint LOD handed to the marcher (a validated graphics→marcher transfer): up while
    /// moving (≈−12% field-evals, imperceptible in motion), 0 for converged stills (exact).
    pub lod_footprint: f32,
}

impl Quality {
    /// Fast preview: low resolution, no AA, no reflections, lean shadows, AO off, LOD on.
    pub fn fast(width: u32, height: u32) -> Quality {
        Quality {
            width,
            height,
            aa: 1,
            bounces: 0,
            max_steps: 64,
            shadow_steps: 12,
            ao_samples: 0,
            lod_footprint: 0.006,
        }
    }
    /// A middle ground.
    pub fn balanced(width: u32, height: u32) -> Quality {
        Quality {
            width,
            height,
            aa: 1,
            bounces: 1,
            max_steps: 110,
            shadow_steps: 28,
            ao_samples: 3,
            lod_footprint: 0.002,
        }
    }
    /// Full quality — converged still frames / offline. LOD off (exact).
    pub fn full(width: u32, height: u32) -> Quality {
        Quality {
            width,
            height,
            aa: 2,
            bounces: 2,
            max_steps: 160,
            shadow_steps: 64,
            ao_samples: 5,
            lod_footprint: 0.0,
        }
    }
    /// Interpolate the budgets from `a` toward `b` by `t ∈ [0, 1]` (resolution is `b`'s). Used to
    /// step quality up over successive still frames.
    pub fn lerp(a: Quality, b: Quality, t: f32) -> Quality {
        let t = t.clamp(0.0, 1.0);
        let mix = |x: u32, y: u32| (x as f32 + (y as f32 - x as f32) * t).round() as u32;
        let mixf = |x: f32, y: f32| x + (y - x) * t;
        Quality {
            width: b.width,
            height: b.height,
            aa: mix(a.aa, b.aa),
            bounces: mix(a.bounces, b.bounces),
            max_steps: mix(a.max_steps, b.max_steps),
            shadow_steps: mix(a.shadow_steps, b.shadow_steps),
            ao_samples: mix(a.ao_samples, b.ao_samples),
            lod_footprint: mixf(a.lod_footprint, b.lod_footprint),
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
        scene.marcher.lod_footprint = self.lod_footprint;
    }
}

// ----------------------------------------------------------------------------
// Scene description (policy: what exists in the world)
// ----------------------------------------------------------------------------

/// A primitive's local-space shape. The orchestrator picks these; the kit only evaluates them.
#[derive(Clone, Copy, Debug)]
pub enum Prim {
    Sphere {
        r: f32,
    },
    Box {
        half: Vec3,
    },
    RoundBox {
        half: Vec3,
        radius: f32,
    },
    Torus {
        major: f32,
        minor: f32,
    },
    Cylinder {
        h: f32,
        r: f32,
    },
    Capsule {
        a: Vec3,
        b: Vec3,
        r: f32,
    },
    Cone {
        r1: f32,
        r2: f32,
        h: f32,
    },
    Ellipsoid {
        r: Vec3,
    },
    Octahedron {
        s: f32,
    },
    HexPrism {
        r: f32,
        h: f32,
    },
    Plane {
        n: Vec3,
        h: f32,
    },
    /// An isolated analytic CSG expression registered in `Scene::csgs`.
    Csg {
        id: u32,
    },
    /// A baked signed-distance volume, usually produced by `mesh::bake_sdf`.
    Volume {
        id: u32,
    },
    /// A native finite triangle shell registered in `Scene::surfaces`.
    Surface {
        id: u32,
    },
}

impl Prim {
    fn distance(
        &self,
        local: Vec3,
        volumes: &[SdfVolume],
        csgs: &[mm3e_kit::csg::Expr],
        surfaces: &[TriangleSurface],
    ) -> f32 {
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
            Prim::Csg { id } => csgs.get(id as usize).expect("unregistered CSG expression").distance(local),
            Prim::Surface { id } => surfaces.get(id as usize).expect("unregistered triangle surface").distance(local),
            Prim::Volume { id } => match volumes.is_empty() {
                true => f32::INFINITY,
                false => volumes[id as usize % volumes.len()].sample(local),
            },
        }
    }

    /// Dual-number twin of [`Prim::distance`] — same formulas, carrying the exact gradient.
    fn distance_dual(&self, lx: Dual, ly: Dual, lz: Dual) -> Dual {
        match *self {
            Prim::Sphere { r } => dual::sphere(lx, ly, lz, r),
            Prim::Box { half } => dual::boxed(lx, ly, lz, half),
            Prim::RoundBox { half, radius } => dual::rounded_box(lx, ly, lz, half, radius),
            Prim::Torus { major, minor } => dual::torus(lx, ly, lz, major, minor),
            Prim::Cylinder { h, r } => dual::cylinder(lx, ly, lz, h, r),
            Prim::Capsule { a, b, r } => dual::capsule(lx, ly, lz, a, b, r),
            Prim::Cone { r1, r2, h } => dual::round_cone(lx, ly, lz, r1, r2, h),
            Prim::Ellipsoid { r } => dual::ellipsoid(lx, ly, lz, r),
            Prim::Octahedron { s } => dual::octahedron(lx, ly, lz, s),
            Prim::HexPrism { r, h } => dual::hex_prism(lx, ly, lz, r, h),
            Prim::Plane { n, h } => dual::plane(lx, ly, lz, n, h),
            Prim::Volume { .. } | Prim::Csg { .. } | Prim::Surface { .. } => {
                unreachable!("volume/CSG/surface prims are excluded by Object::is_dual_safe")
            }
        }
    }

    /// A conservative local-space bounding sphere `(center, radius)`, or `None` if unbounded.
    fn local_bound(
        &self,
        volumes: &[SdfVolume],
        csgs: &[mm3e_kit::csg::Expr],
        surfaces: &[TriangleSurface],
    ) -> Option<(Vec3, f32)> {
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
            Prim::Csg { id } => csgs.get(id as usize).expect("unregistered CSG expression").bound(),
            Prim::Surface { id } => Some(surfaces.get(id as usize).expect("unregistered triangle surface").bound()),
            Prim::Volume { id } => match volumes.is_empty() {
                true => None,
                false => {
                    let (lo, hi) = volumes[id as usize % volumes.len()].bounds();
                    Some(((lo + hi).scale(0.5), (hi - lo).scale(0.5).length()))
                }
            },
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
    pub(crate) fn field(
        &self,
        p: Vec3,
        volumes: &[SdfVolume],
        csgs: &[mm3e_kit::csg::Expr],
        surfaces: &[TriangleSurface],
    ) -> Field {
        let local = self.local_point(p);
        let m = &self.mods;
        let mut d = self.prim.distance(local, volumes, csgs, surfaces) * self.xform.scale;
        if m.round != 0.0 {
            d = sdf::op_round(d, m.round);
        }
        if m.onion != 0.0 {
            d = sdf::op_onion(d, m.onion);
        }
        Field::new(d, self.mat)
    }

    /// The shared object-domain mapping used by rendering and boundary-preserving meshing.
    pub(crate) fn local_point(&self, p: Vec3) -> Vec3 {
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
        local
    }

    /// True iff `field_dual` can represent this object exactly. `twist`/`bend` rotate the query
    /// point by a position-dependent angle (needing dual `sin`/`cos`, which `mm3e_kit::dual` does
    /// not implement — used by exactly one example scene in the whole codebase). Any object using
    /// either falls the WHOLE scene back to the tetrahedron normal (see `Scene::is_dual_safe`),
    /// never a silently-wrong gradient.
    fn is_dual_safe(&self) -> bool {
        self.mods.twist == 0.0
            && self.mods.bend == 0.0
            && !matches!(self.prim, Prim::Volume { .. } | Prim::Csg { .. } | Prim::Surface { .. })
    }

    /// Dual-number twin of [`Object::field`] — same pipeline (transform → modifiers → primitive →
    /// world scale → distance-space modifiers), carrying the exact gradient throughout. Callers
    /// must check `is_dual_safe` first; this does not itself guard against twist/bend.
    fn field_dual(&self, wx: Dual, wy: Dual, wz: Dual) -> Dual {
        let xf = &self.xform;
        let inv = if xf.scale.abs() > 1e-12 { 1.0 / xf.scale } else { 0.0 };
        let tx = wx.adds(-xf.pos.x);
        let ty = wy.adds(-xf.pos.y);
        let tz = wz.adds(-xf.pos.z);
        let rt = xf.rot.transpose();
        let mut lx = (tx.scale(rt.cols[0].x) + ty.scale(rt.cols[1].x) + tz.scale(rt.cols[2].x)).scale(inv);
        let mut ly = (tx.scale(rt.cols[0].y) + ty.scale(rt.cols[1].y) + tz.scale(rt.cols[2].y)).scale(inv);
        let mut lz = (tx.scale(rt.cols[0].z) + ty.scale(rt.cols[1].z) + tz.scale(rt.cols[2].z)).scale(inv);
        let m = &self.mods;
        if m.mirror[0] || m.mirror[1] || m.mirror[2] {
            (lx, ly, lz) = dual::op_mirror(lx, ly, lz, m.mirror[0], m.mirror[1], m.mirror[2]);
        }
        if m.elongate != Vec3::ZERO {
            (lx, ly, lz) = dual::op_elongate(lx, ly, lz, m.elongate);
        }
        if m.repeat != Vec3::ZERO {
            (lx, ly, lz) = dual::op_repeat(lx, ly, lz, m.repeat);
        }
        let mut d = self.prim.distance_dual(lx, ly, lz).scale(xf.scale);
        if m.round != 0.0 {
            d = dual::op_round(d, m.round);
        }
        if m.onion != 0.0 {
            d = dual::op_onion(d, m.onion);
        }
        d
    }

    /// A conservative world-space bounding sphere `(center, radius)`, or `None` for unbounded
    /// shapes (an infinite plane, or an infinitely repeated/bent domain). Used to prune the
    /// field fold: outside the sphere the cheap lower bound `|p − center| − radius` is a valid
    /// distance underestimate, so the expensive exact SDF can be skipped without ever letting
    /// the sphere tracer overshoot a surface.
    pub(crate) fn world_bound(
        &self,
        volumes: &[SdfVolume],
        csgs: &[mm3e_kit::csg::Expr],
        surfaces: &[TriangleSurface],
    ) -> Option<(Vec3, f32)> {
        let (mut center, mut radius) = self.prim.local_bound(volumes, csgs, surfaces)?;
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
#[derive(Clone)]
pub struct Scene {
    pub appearance: appearance::Appearance,
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
    /// Baked signed-distance volumes, referenced by `Prim::Volume` ids like materials.
    pub volumes: Vec<SdfVolume>,
    /// Isolated analytic expressions; one expression supplies one top-level object.
    pub csgs: Vec<mm3e_kit::csg::Expr>,
    /// Immutable native triangle shells with source topology and exact nearest-surface BVHs.
    pub surfaces: Vec<TriangleSurface>,
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Scene {
        Scene {
            appearance: appearance::Appearance::default(),
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
            volumes: Vec::new(),
            csgs: Vec::new(),
            surfaces: Vec::new(),
        }
    }

    /// Register a material and return its id, for objects to reference.
    pub fn material(&mut self, m: Material) -> u32 {
        self.materials.push(m);
        (self.materials.len() - 1) as u32
    }
    /// Register a baked SDF volume and return its id for `Prim::Volume`.
    ///
    /// This also raises the normal/AO/shadow sample radius when needed so sampled fields do not
    /// render with analytic-scene surface acne by default.
    pub fn volume(&mut self, v: SdfVolume) -> u32 {
        let suggested = v.recommended_normal_h();
        if suggested.is_finite() {
            self.marcher.normal_h = self.marcher.normal_h.max(suggested);
        }
        self.volumes.push(v);
        (self.volumes.len() - 1) as u32
    }
    /// Validate and register an object-local expression. Invalid expressions leave the scene unchanged.
    pub fn csg(&mut self, expr: mm3e_kit::csg::Expr) -> Result<u32, String> {
        expr.validate()?;
        let id = u32::try_from(self.csgs.len()).map_err(|_| "too many CSG expressions")?;
        self.csgs.push(expr);
        Ok(id)
    }
    /// Register an already validated triangle shell, returning its `Prim::Surface` id.
    /// Materials and placement belong to `Object`, as for all other primitives. The immutable
    /// `TriangleSurface` constructor guarantees valid geometry before the scene changes.
    pub fn surface(&mut self, surface: TriangleSurface) -> Result<u32, String> {
        let id = u32::try_from(self.surfaces.len()).map_err(|_| "too many triangle surfaces")?;
        self.surfaces.push(surface);
        Ok(id)
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
        self.bake_gi_checked(dims, samples).expect("GI appearance failed; use bake_gi_checked for a recoverable error");
    }
    pub(crate) fn bake_gi_unchecked(&mut self, dims: (usize, usize, usize), samples: u32) {
        let spheres: Vec<(Vec3, f32)> =
            self.objects.iter().filter_map(|o| o.world_bound(&self.volumes, &self.csgs, &self.surfaces)).collect();
        let Some((lo, hi)) = gi::bounds_of(&spheres, 1.5) else { return };
        // Build the volume in a block so the immutable borrows of `self` (the field + gather
        // closures) end before the mutable assignment to `self.gi`.
        let vol = {
            let field = self.world();
            let dual_safe = self.is_dual_safe();
            let gather =
                |o: Vec3, d: Vec3| self.gather(&field, dual_safe, o, d, 2.0 / f64::from(samples.max(1)).sqrt());
            GiVolume::bake(lo, hi, dims, samples, &gather)
        };
        self.gi = Some(vol);
    }

    /// One-bounce incoming radiance along a ray, used while baking GI: march, then shade the hit
    /// with direct lighting only (no shadows, for speed) or return the sky on a miss.
    fn gather(&self, field: &dyn Fn(Vec3) -> Field, dual_safe: bool, origin: Vec3, dir: Vec3, spread: f64) -> Vec3 {
        let ray = Ray { origin, dir };
        let hit = if dual_safe {
            self.marcher.march_with(field, |p| self.normal_dual(p), &ray)
        } else {
            self.marcher.march(field, &ray)
        };
        if !hit.hit {
            return shade::sky_diffuse(dir, self.sun_dir);
        }
        let resolved =
            self.material_for_hit(&hit, f64::from(hit.t) * spread / f64::from(hit.normal.dot(dir).abs().max(0.001)));
        let m = resolved.material;
        let albedo = m.albedo;
        let n = resolved.normal;
        let mut c = m.emissive;
        for light in &self.lights {
            let (l_dir, l_dist) = light.toward(hit.pos);
            let ndl = shade::lambert(n, l_dir);
            if ndl > 0.0 && (!resolved.normal_mapped || hit.normal.dot(l_dir) > 0.0) {
                let outgoing = shade::brdf(n, l_dir, dir * -1.0, albedo, m.metallic, m.roughness, m.specular);
                c = c + outgoing.cmul(light.color).scale(light.attenuation(l_dist));
            }
        }
        c
    }

    /// The world field closure (static geometry) — public so physics and custom tools can query
    /// the same `Fn(Vec3) -> Field` the renderer marches (e.g. distance/normal for collision).
    pub fn field(&self) -> impl Fn(Vec3) -> Field + '_ {
        self.world()
    }

    /// Evaluate the authored field without substituting acceleration bounds. Editing tools
    /// should use this for scalar observations: `field()` may return a bounding-sphere
    /// underestimate far from geometry. This preserves authored CSG order and material rules.
    ///
    /// This is the authored scalar value, not a promise of exact Euclidean distance: smooth
    /// CSG, ellipsoids, sampled volumes and domain warps retain their existing approximations.
    /// An empty scene returns `Field::FAR`.
    pub fn sample_authored(&self, p: Vec3) -> Field {
        let mut acc = None;
        for object in &self.objects {
            let field = object.field(p, &self.volumes, &self.csgs, &self.surfaces);
            acc = Some(match acc {
                None => field,
                Some(previous) => match object.combine {
                    Combine::Union => sdf::union(previous, field),
                    Combine::Smooth(k) => sdf::smooth_union(previous, field, k),
                    Combine::Subtract => sdf::subtract(previous, field),
                },
            });
        }
        acc.unwrap_or(Field::FAR)
    }

    /// Sample one object's authored field in world coordinates, before scene-wide CSG.
    /// Useful when another part overlaps the geometry being measured by an editing tool.
    /// The index refers to this scene snapshot; editors should resolve their stable IDs first.
    pub fn sample_object(&self, index: usize, p: Vec3) -> Option<Field> {
        self.objects.get(index).map(|object| object.field(p, &self.volumes, &self.csgs, &self.surfaces))
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
        let plan = accel::WorldPlan::build(&self.objects, &self.volumes, &self.csgs, &self.surfaces);
        move |p: Vec3| plan.eval(p, &self.objects, &self.volumes, &self.csgs, &self.surfaces)
    }

    /// Reference linear fold. Kept hidden for validation and for comparing the accelerated world
    /// against the exact authored order when changing the BVH planner.
    #[doc(hidden)]
    pub fn world_linear(&self) -> impl Fn(Vec3) -> Field + '_ {
        let bounds: Vec<Option<(Vec3, f32)>> =
            self.objects.iter().map(|o| o.world_bound(&self.volumes, &self.csgs, &self.surfaces)).collect();
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
                            obj.field(p, &self.volumes, &self.csgs, &self.surfaces)
                        }
                    }
                    None => obj.field(p, &self.volumes, &self.csgs, &self.surfaces),
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

    /// True iff every object in the scene can be evaluated by `field_dual` exactly (see
    /// `Object::is_dual_safe`). Checked once per render call, not per-pixel.
    pub fn is_dual_safe(&self) -> bool {
        self.objects.iter().all(Object::is_dual_safe)
    }

    /// The world field's exact gradient at a single point, via dual-number autodiff — one pass
    /// instead of the tetrahedron trick's four extra samples. Skips the bounding-sphere lower-bound
    /// optimization `world()` uses for marching (a per-step speed trick); this is only ever called
    /// once per hit, so evaluating every object exactly is simpler and just as fast in practice.
    /// Callers must check `is_dual_safe()` first — this does not itself guard against twist/bend.
    pub fn normal_dual(&self, p: Vec3) -> Vec3 {
        let (x, y, z) = dual::seed(p);
        let mut acc: Option<Dual> = None;
        for obj in &self.objects {
            let f = obj.field_dual(x, y, z);
            acc = Some(match acc {
                None => f,
                Some(a) => match obj.combine {
                    Combine::Union => dual::union(a, f),
                    Combine::Smooth(k) => dual::smooth_union(a, f, k),
                    Combine::Subtract => dual::subtract(a, f),
                },
            });
        }
        acc.map(|d| d.g.normalize()).unwrap_or(Vec3::ZERO)
    }
}

// ----------------------------------------------------------------------------
// Rendering (policy: how the world becomes pixels)
// ----------------------------------------------------------------------------

fn frame_len(w: u32, h: u32) -> usize {
    (w as usize).checked_mul(h as usize).expect("render dimensions overflow addressable memory")
}

/// Render `scene` from `camera` to a framebuffer, parallelized across CPU cores with scoped std
/// threads (no external crate). In `Beauty` mode each thread sphere-traces a band of rows into a
/// **linear-HDR** buffer that a post pass (bloom, exposure, ACES, gamma) then resolves; in a
/// debug AOV mode the band is shaded straight to display pixels. Deterministic in thread count.
pub fn render(scene: &Scene, camera: &Camera) -> Framebuffer {
    render_checked(scene, camera).expect("scene appearance failed; use render_checked for a recoverable error")
}

/// Checked appearance boundary for texture-aware clients. Evaluation failures are
/// returned instead of allowing an incomplete textured image to be delivered.
pub fn render_checked(scene: &Scene, camera: &Camera) -> Result<Framebuffer, String> {
    render_with_threads_checked(scene, camera, available_render_threads())
}

/// The canonical renderer with an explicit worker budget. Zero selects one worker; the
/// number of workers is capped by image rows. Shading and post-processing match [`render`].
pub fn render_with_threads(scene: &Scene, camera: &Camera, threads: usize) -> Framebuffer {
    render_with_threads_checked(scene, camera, threads)
        .expect("scene appearance failed; use render_with_threads_checked for a recoverable error")
}
pub fn render_with_threads_checked(scene: &Scene, camera: &Camera, threads: usize) -> Result<Framebuffer, String> {
    scene.validate_appearance()?;
    let frame = render_unchecked(scene, camera, threads);
    scene.appearance.status()?;
    Ok(frame)
}
fn render_unchecked(scene: &Scene, camera: &Camera, threads: usize) -> Framebuffer {
    let (w, h) = (scene.width, scene.height);
    let dual_safe = scene.is_dual_safe();

    if scene.mode != RenderMode::Beauty {
        return render_bands_with_threads(scene, camera, threads, |sc, f, cam, x, y| {
            aov_pixel(sc, f, cam, x, y, dual_safe)
        });
    }

    post::resolve(&render_linear_unchecked(scene, camera, threads), w, h, &scene.post)
}

/// Render row-major scene-linear beauty radiance before exposure, bloom, tone mapping or
/// display transfer. Values above one remain intact. This always computes beauty regardless
/// of `scene.mode`, so callers cannot accidentally encode a debug AOV as linear radiance.
pub fn render_linear(scene: &Scene, camera: &Camera) -> Vec<Vec3> {
    render_linear_checked(scene, camera)
        .expect("scene appearance failed; use render_linear_checked for a recoverable error")
}
pub fn render_linear_checked(scene: &Scene, camera: &Camera) -> Result<Vec<Vec3>, String> {
    render_linear_with_threads_checked(scene, camera, available_render_threads())
}

/// Scene-linear beauty radiance with an explicit worker budget; deterministic in worker
/// count. Zero selects one worker, and no empty row workers are spawned.
pub fn render_linear_with_threads(scene: &Scene, camera: &Camera, threads: usize) -> Vec<Vec3> {
    render_linear_with_threads_checked(scene, camera, threads)
        .expect("scene appearance failed; use render_linear_with_threads_checked for a recoverable error")
}
pub fn render_linear_with_threads_checked(scene: &Scene, camera: &Camera, threads: usize) -> Result<Vec<Vec3>, String> {
    scene.validate_appearance()?;
    let frame = render_linear_unchecked(scene, camera, threads);
    scene.appearance.status()?;
    Ok(frame)
}
fn render_linear_unchecked(scene: &Scene, camera: &Camera, threads: usize) -> Vec<Vec3> {
    let (w, h) = (scene.width, scene.height);
    if w == 0 || h == 0 {
        return Vec::new();
    }
    let dual_safe = scene.is_dual_safe();
    // Beauty path: each thread builds the *concrete* world-field closure and shades its band with
    // it, so the marcher and the per-primitive loop monomorphize and inline (no `&dyn Fn` call in
    // the ~8M-evals-per-frame hot path). Results are linear HDR, resolved through the post pass.
    let threads = threads.max(1).min((h as usize).max(1));
    let band = h as usize / threads;
    let extra_rows = h as usize % threads;
    let bands: Vec<(u32, Vec<Vec3>)> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..threads)
            .map(|ti| {
                let y0 = (ti * band + ti.min(extra_rows)) as u32;
                let y1 = y0 + band as u32 + u32::from(ti < extra_rows);
                s.spawn(move || {
                    let field = scene.world();
                    let mut rows = Vec::with_capacity(((y1.saturating_sub(y0)) * w) as usize);
                    for y in y0..y1 {
                        for x in 0..w {
                            rows.push(shade_pixel(scene, &field, camera, x, y, dual_safe));
                        }
                    }
                    (y0, rows)
                })
            })
            .collect();
        handles.into_iter().map(|hd| hd.join().unwrap()).collect()
    });

    let mut hdr = vec![Vec3::ZERO; frame_len(w, h)];
    for (y0, rows) in bands {
        for (i, px) in rows.into_iter().enumerate() {
            hdr[y0 as usize * w as usize + i] = px;
        }
    }
    hdr
}

/// Render a **G-buffer frame** — display colour + primary-ray depth + the camera — for reprojection
/// ([`reproject`]). One sample/pixel, no post pass: it is the base the cheap fake frames warp.
pub fn render_gbuffer(scene: &Scene, camera: &Camera, movers: &[(Vec3, f32)]) -> reproject::GFrame {
    render_gbuffer_checked(scene, camera, movers)
        .expect("G-buffer appearance failed; use render_gbuffer_checked for a recoverable error")
}
pub fn render_gbuffer_checked(
    scene: &Scene,
    camera: &Camera,
    movers: &[(Vec3, f32)],
) -> Result<reproject::GFrame, String> {
    scene.validate_appearance()?;
    let frame = render_gbuffer_unchecked(scene, camera, movers);
    scene.appearance.status()?;
    Ok(frame)
}
fn render_gbuffer_unchecked(scene: &Scene, camera: &Camera, movers: &[(Vec3, f32)]) -> reproject::GFrame {
    type GBand = (u32, Vec<([u8; 4], f32, i32)>); // (first row, [color, depth, obj] per pixel)
    let (w, h) = (scene.width, scene.height);
    let dual_safe = scene.is_dual_safe();
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
                            let hit = if dual_safe {
                                scene.marcher.march_with(&field, |p| scene.normal_dual(p), &ray)
                            } else {
                                scene.marcher.march(&field, &ray)
                            };
                            let (hdr, depth, obj) = if hit.hit {
                                let o = movers
                                    .iter()
                                    .position(|(c, r)| (hit.pos - *c).length() <= r + 0.05)
                                    .map(|k| k as i32)
                                    .unwrap_or(-1);
                                (
                                    shade_hit(
                                        scene,
                                        &field,
                                        &hit,
                                        &ray,
                                        0,
                                        dual_safe,
                                        TextureFootprint::pixel(camera, scene, 1),
                                    ),
                                    hit.t,
                                    o,
                                )
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
    let n = frame_len(w, h);
    let mut color = vec![[0u8; 4]; n];
    let mut depth = vec![f32::INFINITY; n];
    let mut obj = vec![-1i32; n];
    for (y0, rows) in bands {
        for (i, (c, dd, o)) in rows.into_iter().enumerate() {
            let idx = y0 as usize * w as usize + i;
            color[idx] = c;
            depth[idx] = dd;
            obj[idx] = o;
        }
    }
    reproject::GFrame { width: w, height: h, color, depth, obj, camera: *camera }
}

/// **Level-3 hybrid reprojection**: forward-warp `prev` into `new_camera`, then *rerender the
/// disocclusion holes for real* (raymarch just those pixels) instead of smearing them. Newly
/// revealed surfaces come out correct, at a fraction of a full render's cost (holes are typically
/// a few percent of the frame). The static scene reprojects by camera; movers by their motion.
pub fn reproject_hybrid(
    prev: &reproject::GFrame,
    new_camera: &Camera,
    scene: &Scene,
    mover_deltas: &[Vec3],
) -> Vec<[u8; 4]> {
    if !scene.appearance.is_empty() {
        // GFrame has no UV/content/footprint provenance. Re-evaluate textured
        // scenes at the requested view instead of reusing stale color samples.
        let mut current = scene.clone();
        current.width = prev.width;
        current.height = prev.height;
        return render(&current, new_camera).to_rgba8(Rgba::rgb8(0, 0, 0)).as_chunks::<4>().0.to_vec();
    }
    let (w, h) = (prev.width, prev.height);
    let (mut out, filled) = reproject::warp(prev, new_camera, mover_deltas);
    let field = scene.world();
    let dual_safe = scene.is_dual_safe();
    for y in 0..h {
        for x in 0..w {
            let i = y as usize * w as usize + x as usize;
            if filled[i] {
                continue;
            }
            let ray = new_camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
            let hit = if dual_safe {
                scene.marcher.march_with(&field, |p| scene.normal_dual(p), &ray)
            } else {
                scene.marcher.march(&field, &ray)
            };
            let hdr = if hit.hit {
                shade_hit(scene, &field, &hit, &ray, 0, dual_safe, TextureFootprint::pixel(new_camera, scene, 1))
            } else {
                shade::sky(ray.dir, scene.sun_dir)
            };
            let d = shade::gamma(shade::aces(hdr.scale(scene.post.exposure))).clamp01();
            out[i] = [(d.x * 255.0 + 0.5) as u8, (d.y * 255.0 + 0.5) as u8, (d.z * 255.0 + 0.5) as u8, 255];
        }
    }
    out
}

/// Checkerboard (interlaced) render — ray-trace only the pixels where `(x + y)` is even and fill
/// the rest by averaging their four (always-rendered) neighbours. Halves the per-frame ray work
/// for a small softening, ideal for the camera-moving phase of an interactive previewer. Resolves
/// straight to display pixels (no HDR post pass), since it is a preview path.
pub fn render_checkerboard(scene: &Scene, camera: &Camera) -> Framebuffer {
    let (w, h) = (scene.width, scene.height);
    let dual_safe = scene.is_dual_safe();
    // Pass 1: shade the even pixels; mark the odd ones unfilled (alpha 0).
    let mut fb = render_bands(scene, camera, |sc, f, cam, x, y| {
        if (x + y) % 2 == 0 {
            let c = shade_pixel(sc, f, cam, x, y, dual_safe).scale(sc.post.exposure);
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
fn available_render_threads() -> usize {
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
}

fn render_bands<F>(scene: &Scene, camera: &Camera, shade: F) -> Framebuffer
where
    F: Fn(&Scene, &dyn Fn(Vec3) -> Field, &Camera, u32, u32) -> Rgba + Sync,
{
    render_bands_with_threads(scene, camera, available_render_threads(), shade)
}

fn render_bands_with_threads<F>(scene: &Scene, camera: &Camera, threads: usize, shade: F) -> Framebuffer
where
    F: Fn(&Scene, &dyn Fn(Vec3) -> Field, &Camera, u32, u32) -> Rgba + Sync,
{
    let (w, h) = (scene.width, scene.height);
    if w == 0 || h == 0 {
        return Framebuffer::new(w, h, Rgba::rgb8(0, 0, 0));
    }
    let threads = threads.max(1).min((h as usize).max(1));
    let band = h as usize / threads;
    let extra_rows = h as usize % threads;
    let shade = &shade;

    let bands: Vec<(u32, Vec<Rgba>)> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..threads)
            .map(|ti| {
                let y0 = (ti * band + ti.min(extra_rows)) as u32;
                let y1 = y0 + band as u32 + u32::from(ti < extra_rows);
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
fn aov_pixel(scene: &Scene, field: &dyn Fn(Vec3) -> Field, camera: &Camera, x: u32, y: u32, dual_safe: bool) -> Rgba {
    let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, scene.width, scene.height);
    let hit = if dual_safe {
        scene.marcher.march_with(field, |p| scene.normal_dual(p), &ray)
    } else {
        scene.marcher.march(field, &ray)
    };
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
                scene
                    .material_for_hit(&hit, TextureFootprint::pixel(camera, scene, 1).projected(&hit, &ray))
                    .material
                    .albedo
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
fn shade_pixel<F: Fn(Vec3) -> Field + ?Sized>(
    scene: &Scene,
    field: &F,
    camera: &Camera,
    x: u32,
    y: u32,
    dual_safe: bool,
) -> Vec3 {
    let n = scene.aa.max(1);
    let inv_samples = 1.0 / (n * n) as f32;
    let mut acc = Vec3::ZERO;
    for sy in 0..n {
        for sx in 0..n {
            let ox = (sx as f32 + 0.5) / n as f32;
            let oy = (sy as f32 + 0.5) / n as f32;
            let ray = camera.ray(x as f32 + ox, y as f32 + oy, scene.width, scene.height);
            acc = acc + trace(scene, field, &ray, 0, dual_safe, TextureFootprint::pixel(camera, scene, n));
        }
    }
    acc.scale(inv_samples)
}

/// Trace one ray and return its linear HDR radiance. Recurses for mirror reflections. Generic over
/// the field type so the beauty path monomorphizes (the primitive loop inlines into the marcher).
#[derive(Clone, Copy, Default)]
struct TextureFootprint {
    width: f64,
    spread: f64,
}
impl TextureFootprint {
    fn pixel(camera: &Camera, scene: &Scene, samples_per_axis: u32) -> Self {
        Self {
            width: 0.0,
            spread: 2.0 * f64::from(camera.fov_scale)
                / f64::from(scene.height.max(1))
                / f64::from(samples_per_axis.max(1)),
        }
    }
    fn projected(self, hit: &Hit, ray: &Ray) -> f64 {
        (self.width + f64::from(hit.t.max(0.0)) * self.spread) / f64::from(hit.normal.dot(ray.dir).abs().max(0.001))
    }
}
fn trace<F: Fn(Vec3) -> Field + ?Sized>(
    scene: &Scene,
    field: &F,
    ray: &Ray,
    depth: u32,
    dual_safe: bool,
    footprint: TextureFootprint,
) -> Vec3 {
    let hit = if dual_safe {
        scene.marcher.march_with(field, |p| scene.normal_dual(p), ray)
    } else {
        scene.marcher.march(field, ray)
    };
    if !hit.hit {
        return shade::sky(ray.dir, scene.sun_dir);
    }
    shade_hit(scene, field, &hit, ray, depth, dual_safe, footprint)
}

/// Shade a confirmed surface hit — the post-hit half of [`trace`], factored out so the G-buffer
/// renderer (reprojection) can capture the primary depth without marching the ray a second time.
fn shade_hit<F: Fn(Vec3) -> Field + ?Sized>(
    scene: &Scene,
    field: &F,
    hit: &Hit,
    ray: &Ray,
    depth: u32,
    dual_safe: bool,
    footprint: TextureFootprint,
) -> Vec3 {
    let projected = footprint.projected(hit, ray);
    let resolved = scene.material_for_hit(hit, projected);
    let m = resolved.material;
    let albedo = m.albedo;
    let normal = resolved.normal;
    let view = ray.dir.scale(-1.0);

    // Ambient term: diffuse image-based lighting from the sky + baked GI + a constant floor,
    // all gated by ambient occlusion. This replaces the old flat ambient with real environment
    // and bounce light.
    let occ = if scene.ao { scene.marcher.ambient_occlusion(field, hit.pos, hit.normal) } else { 1.0 };
    let gi = scene.gi.as_ref().map(|g| g.sample(hit.pos, normal)).unwrap_or(Vec3::ZERO);
    let ambient_irr = ibl_irradiance(scene, normal) + gi + scene.ambient;
    let mut radiance = albedo.cmul(ambient_irr).scale(occ);

    // Direct lighting: `combine` (fold) each light through the Cook-Torrance GGX BRDF with
    // inverse-square falloff and soft shadows. Lighting is additive, so no per-pixel ordering is
    // needed (the old `order` call allocated a Vec for every shaded pixel — pure waste).
    for light in &scene.lights {
        let (l_dir, l_dist) = light.toward(hit.pos);
        if shade::lambert(normal, l_dir) <= 0.0 || (resolved.normal_mapped && hit.normal.dot(l_dir) <= 0.0) {
            continue;
        }
        // Lift the shadow ray off the surface to avoid self-intersection acne.
        let shadow = if scene.shadows {
            let max_t = l_dist.min(scene.marcher.max_dist);
            let lift = (scene.marcher.normal_h * 2.0).max(0.01);
            scene.marcher.soft_shadow(field, hit.pos + hit.normal.scale(lift), l_dir, max_t, light.shadow_k(l_dist))
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
        let rorigin = hit.pos + hit.normal.scale((scene.marcher.normal_h * 2.0).max(0.02));
        let refl = if resolved.normal_mapped && rdir.dot(hit.normal) <= 0.0 {
            Vec3::ZERO
        } else {
            trace(
                scene,
                field,
                &Ray { origin: rorigin, dir: rdir },
                depth + 1,
                dual_safe,
                TextureFootprint { width: projected, spread: footprint.spread },
            )
        };
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

/// Canonicalize an orbit yaw to `[-pi, pi)`, preventing precision loss after long input sessions.
pub fn wrap_orbit_yaw(yaw: f32) -> f32 {
    if yaw.is_finite() {
        (yaw + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
    } else {
        0.0
    }
}

/// A camera orbiting `target` at `radius`, `yaw`/`pitch` in radians, FOV `fov_y` in radians.
///
/// Interactive callers may accumulate rotation indefinitely, so this constructor also
/// canonicalizes yaw and keeps pitch away from the singularity at either pole.
pub fn orbit_camera(target: Vec3, radius: f32, yaw: f32, pitch: f32, fov_y: f32) -> Camera {
    let yaw = wrap_orbit_yaw(yaw);
    let pitch_limit = std::f32::consts::FRAC_PI_2 - 1.0e-4;
    let pitch = if pitch.is_finite() { pitch.clamp(-pitch_limit, pitch_limit) } else { 0.0 };
    let radius = if radius.is_finite() { radius.abs().max(1.0e-4) } else { 1.0 };
    let fov_y = if fov_y.is_finite() { fov_y.clamp(1.0e-4, std::f32::consts::PI - 1.0e-4) } else { 45f32.to_radians() };
    let eye =
        target + Vec3::new(radius * yaw.cos() * pitch.cos(), radius * pitch.sin(), radius * yaw.sin() * pitch.cos());
    Camera::look_at(eye, target, Vec3::new(0.0, 1.0, 0.0), fov_y)
}

#[cfg(test)]
mod dual_wiring_tests {
    //! End-to-end verification that the dual-number normal wiring (see `Object::field_dual`,
    //! `Scene::normal_dual`, and every `march_with` call site above) never changes shaded output
    //! by more than the analytic-vs-finite-difference gap already measured in
    //! `dual_normal_real.rs`, across every `Combine` mode it supports, and that scenes it does NOT
    //! support (twist/bend) fall back to byte-identical old behavior rather than a silent wrong
    //! answer. Compares full shaded RADIANCE end to end (lighting + shadows + AO), not just raw
    //! normal vectors — a wrong gradient would cascade into visibly wrong shadows/AO here.
    use super::*;
    use mm3e_kit::vec::Mat3;

    fn small_scene() -> Scene {
        let mut scene = Scene::new(64, 48);
        scene.aa = 1;
        scene.bounces = 1;
        let red = scene.material(Material::solid(Vec3::new(0.8, 0.2, 0.2)).roughness(0.4));
        scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, red));
        scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-1.2, 1.0, 0.0)), red));
        scene.add(Object::new(
            Prim::RoundBox { half: Vec3::splat(0.6), radius: 0.15 },
            Transform::at(Vec3::new(1.3, 0.7, 0.2)).rotated(Mat3::from_euler(0.0, 0.5, 0.0)),
            red,
        ));
        scene.sun_dir = Vec3::new(0.5, 0.7, 0.4).normalize();
        scene.light(Light::directional(scene.sun_dir, Vec3::splat(1.5)));
        scene
    }

    fn cam() -> Camera {
        orbit_camera(Vec3::new(0.0, 0.7, 0.0), 6.0, 0.4, 0.3, 45f32.to_radians())
    }

    /// The pre-wiring behavior, reproduced exactly: march with the tetrahedron normal (bypassing
    /// the new `is_dual_safe` branch entirely), then shade — the "before" side of every comparison.
    fn old_path_radiance(scene: &Scene, field: &dyn Fn(Vec3) -> Field, ray: &Ray) -> Vec3 {
        let hit = scene.marcher.march(field, ray);
        if !hit.hit {
            return shade::sky(ray.dir, scene.sun_dir);
        }
        shade_hit(scene, field, &hit, ray, 0, false, TextureFootprint::default())
    }

    /// Render both paths over a small grid and return (max, mean) absolute radiance delta.
    fn compare_paths(scene: &Scene) -> (f32, f32) {
        let field = scene.world();
        let camera = cam();
        let dual_safe = scene.is_dual_safe();
        let (w, h) = (scene.width, scene.height);
        let (mut max_d, mut sum_d, mut n) = (0.0f32, 0.0f64, 0u32);
        for y in (0..h).step_by(3) {
            for x in (0..w).step_by(3) {
                let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
                let old = old_path_radiance(scene, &field, &ray);
                let new = shade_pixel(scene, &field, &camera, x, y, dual_safe);
                let d = (old - new).abs();
                let m = d.x.max(d.y).max(d.z);
                max_d = max_d.max(m);
                sum_d += m as f64;
                n += 1;
            }
        }
        (max_d, (sum_d / n.max(1) as f64) as f32)
    }

    #[test]
    fn union_only_scene_is_dual_safe_and_matches_old_radiance() {
        let scene = small_scene();
        assert!(scene.is_dual_safe(), "plain Union/no-modifier scene must be dual-safe");
        let (max_d, mean_d) = compare_paths(&scene);
        assert!(max_d < 0.01, "max radiance delta too large: {max_d}");
        assert!(mean_d < 0.002, "mean radiance delta too large: {mean_d}");
    }

    #[test]
    fn smooth_union_scene_is_dual_safe_and_matches_old_radiance() {
        let mut scene = small_scene();
        let red = scene.materials.len() as u32 - 1;
        scene.add(Object::new(Prim::Sphere { r: 0.5 }, Transform::at(Vec3::new(-0.9, 0.9, 0.0)), red).smooth(0.4));
        assert!(scene.is_dual_safe(), "smooth-union scene must be dual-safe");
        let (max_d, mean_d) = compare_paths(&scene);
        assert!(max_d < 0.01, "max radiance delta too large: {max_d}");
        assert!(mean_d < 0.002, "mean radiance delta too large: {mean_d}");
    }

    #[test]
    fn subtract_scene_is_dual_safe_and_matches_old_radiance() {
        let mut scene = small_scene();
        let red = scene.materials.len() as u32 - 1;
        scene.add(Object::new(Prim::Sphere { r: 0.4 }, Transform::at(Vec3::new(-1.2, 1.0, 0.0)), red).subtract());
        assert!(scene.is_dual_safe(), "subtract-combine scene must be dual-safe");
        let (max_d, mean_d) = compare_paths(&scene);
        assert!(max_d < 0.01, "max radiance delta too large: {max_d}");
        assert!(mean_d < 0.002, "mean radiance delta too large: {mean_d}");
    }

    #[test]
    fn twist_scene_is_not_dual_safe_and_output_is_unchanged() {
        let mut scene = small_scene();
        let red = scene.materials.len() as u32 - 1;
        scene.add(Object::new(Prim::Sphere { r: 0.4 }, Transform::at(Vec3::new(0.0, 1.5, 0.0)), red).twist(0.9));
        assert!(!scene.is_dual_safe(), "a twisted object must disable the dual path for the whole scene");
        // With is_dual_safe() == false, trace()/aov_pixel/etc. take the `else` branch, which is the
        // exact original code path — so old and new must be BIT-IDENTICAL, not just close.
        let (max_d, _mean_d) = compare_paths(&scene);
        assert_eq!(max_d, 0.0, "twist-fallback scene must render byte-identically to the old path");
    }

    /// Mirror/onion are piecewise exact, but non-smooth at mirror seams and shell mid-surfaces.
    /// The dual path should therefore stay globally stable without pretending every seam pixel
    /// matches the tetrahedron finite-difference normal byte-for-byte.
    #[test]
    fn mirror_and_round_and_onion_modifiers_are_dual_safe() {
        let mut scene = small_scene();
        let red = scene.materials.len() as u32 - 1;
        scene.add(
            Object::new(Prim::Sphere { r: 0.3 }, Transform::at(Vec3::new(0.0, 0.9, 0.6)), red)
                .mirror(true, false, false)
                .round(0.1)
                .onion(0.05),
        );
        assert!(scene.is_dual_safe(), "mirror/round/onion modifiers are dual-safe");
        let (max_d, mean_d) = compare_paths(&scene);
        assert!(max_d < 0.35, "seam-local radiance spike too large: {max_d}; mean={mean_d}");
        assert!(mean_d < 0.002, "mean radiance delta too large: {mean_d}");
    }

    /// Real wall-clock, on the SHIPPED, wired path (not a standalone harness): times the actual
    /// per-pixel `shade_pixel` (dual, automatic via `is_dual_safe`) against the old tetrahedron
    /// path on a larger, more representative scene. `cargo test --release -- --nocapture` to see
    /// the numbers; asserts only that dual is not slower (a real regression would be a bug).
    #[test]
    fn shipped_dual_path_is_not_slower_than_old_path_on_a_real_scene() {
        let mut scene = small_scene();
        scene.width = 240;
        scene.height = 160;
        let red = scene.materials.len() as u32 - 1;
        scene.add(Object::new(Prim::Sphere { r: 0.5 }, Transform::at(Vec3::new(-0.9, 0.9, 0.0)), red).smooth(0.4));
        scene.add(Object::new(Prim::Torus { major: 0.5, minor: 0.18 }, Transform::at(Vec3::new(0.0, 0.4, -1.2)), red));
        assert!(scene.is_dual_safe());
        let field = scene.world();
        let camera = cam();
        let dual_safe = scene.is_dual_safe();
        let (w, h) = (scene.width, scene.height);

        let old_t0 = std::time::Instant::now();
        for y in 0..h {
            for x in 0..w {
                let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, w, h);
                std::hint::black_box(old_path_radiance(&scene, &field, &ray));
            }
        }
        let old_ms = old_t0.elapsed().as_secs_f64() * 1000.0;

        let new_t0 = std::time::Instant::now();
        for y in 0..h {
            for x in 0..w {
                std::hint::black_box(shade_pixel(&scene, &field, &camera, x, y, dual_safe));
            }
        }
        let new_ms = new_t0.elapsed().as_secs_f64() * 1000.0;

        println!(
            "old (tetrahedron): {old_ms:.2} ms | new (shipped dual, wired): {new_ms:.2} ms | {:+.1}%",
            (new_ms - old_ms) / old_ms * 100.0
        );
        assert!(new_ms < old_ms * 1.5, "dual path unexpectedly much slower: old={old_ms:.2}ms new={new_ms:.2}ms");
    }

    #[test]
    fn full_render_produces_a_sane_image_on_a_dual_safe_scene() {
        let scene = small_scene();
        let camera = cam();
        let fb = render(&scene, &camera);
        let mut any_nonzero = false;
        for y in 0..scene.height {
            for x in 0..scene.width {
                let p = fb.pixel(x, y);
                assert!(p.r.is_finite() && p.g.is_finite() && p.b.is_finite(), "NaN/Inf pixel at ({x},{y})");
                if p.r > 0.0 || p.g > 0.0 || p.b > 0.0 {
                    any_nonzero = true;
                }
            }
        }
        assert!(any_nonzero, "rendered frame is entirely black");
    }
}

#[cfg(test)]
mod explicit_worker_tests {
    use super::*;
    use std::{collections::HashSet, sync::Mutex};

    #[test]
    fn explicit_aov_worker_budget_drives_the_actual_render_threads() {
        let scene = Scene::new(3, 17);
        let camera = Camera::look_at(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 1.0);
        for (budget, expected) in [(0, 1), (1, 1), (4, 4), (12, 12), (30, 17)] {
            let workers = Mutex::new(HashSet::new());
            render_bands_with_threads(&scene, &camera, budget, |_, _, _, _, _| {
                workers.lock().unwrap().insert(std::thread::current().id());
                Rgba::rgb8(0, 0, 0)
            });
            assert_eq!(workers.into_inner().unwrap().len(), expected);
        }
    }
}
