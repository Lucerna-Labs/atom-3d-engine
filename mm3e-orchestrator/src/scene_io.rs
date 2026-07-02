//! A hand-rolled, dependency-free text scene format — the data layer that de-Rusts content
//! authoring. `serialize` writes a `Scene` + `Camera` to a `.mm3e` document; `parse` reads one
//! back. The format is line-oriented: a directive keyword followed by whitespace-separated
//! values, `#` starts a comment. Parsing is the `scan` atom over tokens; the writer is its
//! inverse. Round-trips are covered by the test suite.

use mm3e_kit::camera::Camera;
use mm3e_kit::vec::{Mat3, Transform, Vec3};

use crate::{Combine, Light, Material, Object, Prim, RenderMode, Scene};

fn mode_name(m: RenderMode) -> &'static str {
    match m {
        RenderMode::Beauty => "beauty",
        RenderMode::Normal => "normal",
        RenderMode::Depth => "depth",
        RenderMode::Ao => "ao",
        RenderMode::Steps => "steps",
        RenderMode::Albedo => "albedo",
    }
}

// ----------------------------------------------------------------------------
// Writer
// ----------------------------------------------------------------------------

/// Serialize a scene and its camera to the `.mm3e` text format.
pub fn serialize(scene: &Scene, camera: &Camera) -> String {
    let mut s = String::new();
    s.push_str("# MM3E scene\n");
    s.push_str(&format!("size {} {}\n", scene.width, scene.height));
    s.push_str(&format!("aa {}\n", scene.aa));
    s.push_str(&format!("bounces {}\n", scene.bounces));
    s.push_str(&format!("shadows {}\n", scene.shadows as u32));
    s.push_str(&format!("ao {}\n", scene.ao as u32));
    s.push_str(&format!("sun {}\n", v3(scene.sun_dir)));
    s.push_str(&format!("ambient {}\n", v3(scene.ambient)));
    s.push_str(&format!("sky_ambient {}\n", v3(scene.sky_ambient)));
    s.push_str(&format!("fog {} {}\n", v3(scene.fog), scene.fog_density));
    let mr = &scene.marcher;
    s.push_str(&format!("marcher {} {} {} {}\n", mr.max_steps, mr.max_dist, mr.eps, mr.step_scale));
    let p = &scene.post;
    s.push_str(&format!(
        "post {} {} {} {} {}\n",
        p.exposure, p.bloom as u32, p.bloom_threshold, p.bloom_intensity, p.bloom_radius
    ));
    s.push_str(&format!("mode {}\n", mode_name(scene.mode)));

    // Camera: eye, target (= eye + forward), up, vertical FOV in degrees.
    let target = camera.eye + camera.forward;
    let fov_deg = (camera.fov_scale.atan() * 2.0).to_degrees();
    s.push_str(&format!("cam {} {} {} {}\n", v3(camera.eye), v3(target), v3(camera.up), fov_deg));

    for m in &scene.materials {
        s.push_str(&format!(
            "mat {} {} {} {} {} {} {}\n",
            v3(m.albedo),
            m.metallic,
            m.roughness,
            m.reflectivity,
            m.specular,
            v3(m.emissive),
            m.checker as u32
        ));
    }
    for l in &scene.lights {
        let kind = if l.directional { "dir" } else { "point" };
        s.push_str(&format!("light {} {} {} {}\n", kind, v3(l.vec), v3(l.color), l.radius));
    }
    for o in &scene.objects {
        s.push_str(&serialize_object(o));
        s.push('\n');
    }
    s
}

fn v3(v: Vec3) -> String {
    format!("{} {} {}", v.x, v.y, v.z)
}

fn serialize_object(o: &Object) -> String {
    // A baked volume's grid data is binary and belongs to the bake, not to a text scene file.
    // Serialize the omission explicitly (the parser skips comments), never a broken directive.
    if let Prim::Volume { id } = o.prim {
        return format!("# volume object omitted (baked SDF volume id {id} is not text-serializable)");
    }
    let mut s = String::from("obj ");
    s.push_str(&match o.prim {
        Prim::Sphere { r } => format!("sphere {r}"),
        Prim::Box { half } => format!("box {}", v3(half)),
        Prim::RoundBox { half, radius } => format!("roundbox {} {}", v3(half), radius),
        Prim::Torus { major, minor } => format!("torus {major} {minor}"),
        Prim::Cylinder { h, r } => format!("cylinder {h} {r}"),
        Prim::Capsule { a, b, r } => format!("capsule {} {} {}", v3(a), v3(b), r),
        Prim::Cone { r1, r2, h } => format!("cone {r1} {r2} {h}"),
        Prim::Ellipsoid { r } => format!("ellipsoid {}", v3(r)),
        Prim::Octahedron { s } => format!("octahedron {s}"),
        Prim::HexPrism { r, h } => format!("hexprism {r} {h}"),
        Prim::Plane { n, h } => format!("plane {} {}", v3(n), h),
        Prim::Volume { .. } => unreachable!("handled by the early return above"),
    });
    // Recover Euler-free placement: store rotation columns so any Mat3 round-trips exactly.
    let r = o.xform.rot;
    s.push_str(&format!(
        " pos {} basis {} {} {} scale {} mat {}",
        v3(o.xform.pos),
        v3(r.cols[0]),
        v3(r.cols[1]),
        v3(r.cols[2]),
        o.xform.scale,
        o.mat
    ));
    match o.combine {
        Combine::Union => s.push_str(" combine union"),
        Combine::Smooth(k) => s.push_str(&format!(" combine smooth {k}")),
        Combine::Subtract => s.push_str(" combine subtract"),
    }
    let m = &o.mods;
    if m.round != 0.0 {
        s.push_str(&format!(" round {}", m.round));
    }
    if m.onion != 0.0 {
        s.push_str(&format!(" onion {}", m.onion));
    }
    if m.twist != 0.0 {
        s.push_str(&format!(" twist {}", m.twist));
    }
    if m.bend != 0.0 {
        s.push_str(&format!(" bend {}", m.bend));
    }
    if m.mirror != [false; 3] {
        s.push_str(&format!(" mirror {} {} {}", m.mirror[0] as u32, m.mirror[1] as u32, m.mirror[2] as u32));
    }
    if m.repeat != Vec3::ZERO {
        s.push_str(&format!(" repeat {}", v3(m.repeat)));
    }
    if m.elongate != Vec3::ZERO {
        s.push_str(&format!(" elongate {}", v3(m.elongate)));
    }
    s
}

// ----------------------------------------------------------------------------
// Parser
// ----------------------------------------------------------------------------

/// A token cursor with line-numbered errors.
struct Cur<'a> {
    toks: std::str::SplitWhitespace<'a>,
    line: usize,
}

impl<'a> Cur<'a> {
    fn next(&mut self, what: &str) -> Result<&'a str, String> {
        self.toks.next().ok_or_else(|| format!("line {}: expected {what}", self.line))
    }
    fn f(&mut self, what: &str) -> Result<f32, String> {
        let t = self.next(what)?;
        t.parse::<f32>().map_err(|_| format!("line {}: bad number '{t}' for {what}", self.line))
    }
    fn u(&mut self, what: &str) -> Result<u32, String> {
        let t = self.next(what)?;
        t.parse::<u32>().map_err(|_| format!("line {}: bad integer '{t}' for {what}", self.line))
    }
    fn b(&mut self, what: &str) -> Result<bool, String> {
        Ok(self.u(what)? != 0)
    }
    fn v3(&mut self, what: &str) -> Result<Vec3, String> {
        Ok(Vec3::new(self.f(what)?, self.f(what)?, self.f(what)?))
    }
}

/// Parse a `.mm3e` document into a scene and camera.
pub fn parse(src: &str) -> Result<(Scene, Camera), String> {
    // Strictly positive and finite — written as a positive predicate so NaN (which fails every
    // comparison) is rejected too.
    let pos = |x: f32| x > 0.0;
    let mut scene = Scene::new(640, 360);
    scene.materials.clear(); // replaced by file contents
    let mut camera = Camera::look_at(Vec3::new(0.0, 1.0, 6.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 1.0);

    for (i, raw) in src.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut c = Cur { toks: line.split_whitespace(), line: i + 1 };
        let directive = c.next("directive")?;
        match directive {
            "size" => {
                scene.width = c.u("width")?;
                scene.height = c.u("height")?;
                // Reject degenerate/absurd sizes: 0 renders nothing meaningful, and huge values
                // would overflow `w * h` buffer math downstream (u32 wrap → misallocated frame).
                if scene.width == 0 || scene.height == 0 || scene.width > 16384 || scene.height > 16384 {
                    return Err(format!(
                        "line {}: size {}x{} out of range 1..=16384",
                        i + 1,
                        scene.width,
                        scene.height
                    ));
                }
            }
            "aa" => scene.aa = c.u("aa")?,
            "bounces" => scene.bounces = c.u("bounces")?,
            "shadows" => scene.shadows = c.b("shadows")?,
            "ao" => scene.ao = c.b("ao")?,
            "sun" => {
                scene.sun_dir = c.v3("sun")?.normalize();
                if scene.sun_dir == Vec3::ZERO {
                    return Err(format!("line {}: sun direction must be non-zero", i + 1));
                }
            }
            "ambient" => scene.ambient = c.v3("ambient")?,
            "sky_ambient" => scene.sky_ambient = c.v3("sky_ambient")?,
            "fog" => {
                scene.fog = c.v3("fog")?;
                scene.fog_density = c.f("fog density")?;
            }
            "marcher" => {
                scene.marcher.max_steps = c.u("max_steps")?;
                scene.marcher.max_dist = c.f("max_dist")?;
                scene.marcher.eps = c.f("eps")?;
                scene.marcher.step_scale = c.f("step_scale")?;
                // Sanity bounds for an untrusted file: a huge step budget is a per-ray hang, and
                // non-positive distances/tolerances degenerate the march (never crash, but never
                // finish usefully either).
                if scene.marcher.max_steps == 0 || scene.marcher.max_steps > 100_000 {
                    return Err(format!("line {}: marcher max_steps out of range 1..=100000", i + 1));
                }
                if !pos(scene.marcher.max_dist) || !pos(scene.marcher.eps) || !pos(scene.marcher.step_scale) {
                    return Err(format!("line {}: marcher max_dist/eps/step_scale must be positive", i + 1));
                }
            }
            "post" => {
                scene.post.exposure = c.f("exposure")?;
                scene.post.bloom = c.b("bloom")?;
                scene.post.bloom_threshold = c.f("bloom_threshold")?;
                scene.post.bloom_intensity = c.f("bloom_intensity")?;
                scene.post.bloom_radius = c.u("bloom_radius")?;
            }
            "mode" => {
                scene.mode = match c.next("mode")? {
                    "beauty" => RenderMode::Beauty,
                    "normal" => RenderMode::Normal,
                    "depth" => RenderMode::Depth,
                    "ao" => RenderMode::Ao,
                    "steps" => RenderMode::Steps,
                    "albedo" => RenderMode::Albedo,
                    other => return Err(format!("line {}: unknown render mode '{other}'", i + 1)),
                }
            }
            "cam" => {
                let eye = c.v3("eye")?;
                let target = c.v3("target")?;
                let up = c.v3("up")?;
                let fov_deg = c.f("fov")?;
                // A degenerate look-at basis (eye on target, or up parallel to the view direction)
                // normalizes to zero vectors and fills the frame with NaN rays; reject it here so
                // the kit's camera can stay dumb mechanism.
                let forward = (target - eye).normalize();
                if forward == Vec3::ZERO {
                    return Err(format!("line {}: cam eye and target must not coincide", i + 1));
                }
                if forward.cross(up).normalize() == Vec3::ZERO {
                    return Err(format!("line {}: cam up must not be parallel to the view direction", i + 1));
                }
                if !pos(fov_deg) || fov_deg >= 180.0 {
                    return Err(format!("line {}: cam fov must be in (0, 180) degrees", i + 1));
                }
                camera = Camera::look_at(eye, target, up, fov_deg.to_radians());
            }
            "mat" => {
                let albedo = c.v3("albedo")?;
                let metallic = c.f("metallic")?;
                let roughness = c.f("roughness")?;
                let reflectivity = c.f("reflectivity")?;
                let specular = c.f("specular")?;
                let emissive = c.v3("emissive")?;
                let checker = c.b("checker")?;
                scene.materials.push(Material {
                    albedo,
                    metallic,
                    specular,
                    roughness,
                    reflectivity,
                    emissive,
                    checker,
                });
            }
            "light" => {
                let kind = c.next("light kind")?;
                let vec = c.v3("light vec")?;
                let color = c.v3("light color")?;
                let radius = c.f("light radius")?;
                let directional = match kind {
                    "dir" => true,
                    "point" => false,
                    other => return Err(format!("line {}: unknown light kind '{other}'", i + 1)),
                };
                scene.lights.push(Light { vec, color, directional, radius });
            }
            "obj" => scene.objects.push(parse_object(&mut c)?),
            other => return Err(format!("line {}: unknown directive '{other}'", i + 1)),
        }
    }
    if scene.materials.is_empty() {
        scene.materials.push(Material::default());
    }
    // Validate material references once the whole file is read (materials may be declared
    // anywhere). The renderer's modulo lookup would not crash on a bad index, but it would
    // silently pick the wrong material — better to reject the file with a real message.
    let n_mats = scene.materials.len() as u32;
    for (idx, obj) in scene.objects.iter().enumerate() {
        if obj.mat >= n_mats {
            return Err(format!("object {}: material index {} out of range (have {})", idx + 1, obj.mat, n_mats));
        }
    }
    Ok((scene, camera))
}

fn parse_object(c: &mut Cur) -> Result<Object, String> {
    let name = c.next("primitive")?;
    let prim = match name {
        "sphere" => Prim::Sphere { r: c.f("r")? },
        "box" => Prim::Box { half: c.v3("half")? },
        "roundbox" => Prim::RoundBox { half: c.v3("half")?, radius: c.f("radius")? },
        "torus" => Prim::Torus { major: c.f("major")?, minor: c.f("minor")? },
        "cylinder" => Prim::Cylinder { h: c.f("h")?, r: c.f("r")? },
        "capsule" => Prim::Capsule { a: c.v3("a")?, b: c.v3("b")?, r: c.f("r")? },
        "cone" => Prim::Cone { r1: c.f("r1")?, r2: c.f("r2")?, h: c.f("h")? },
        "ellipsoid" => Prim::Ellipsoid { r: c.v3("r")? },
        "octahedron" => Prim::Octahedron { s: c.f("s")? },
        "hexprism" => Prim::HexPrism { r: c.f("r")?, h: c.f("h")? },
        "plane" => Prim::Plane { n: c.v3("n")?, h: c.f("h")? },
        other => return Err(format!("line {}: unknown primitive '{other}'", c.line)),
    };

    let mut obj = Object::new(prim, Transform::IDENTITY, 0);
    let mut pos = Vec3::ZERO;
    let mut basis = Mat3::IDENTITY;
    let mut scale = 1.0;
    while let Ok(kw) = c.next("modifier or end") {
        match kw {
            "pos" => pos = c.v3("pos")?,
            "basis" => basis = Mat3::from_cols(c.v3("col0")?, c.v3("col1")?, c.v3("col2")?),
            "scale" => scale = c.f("scale")?,
            "mat" => obj.mat = c.u("mat")?,
            "combine" => {
                obj.combine = match c.next("combine kind")? {
                    "union" => Combine::Union,
                    "smooth" => Combine::Smooth(c.f("k")?),
                    "subtract" => Combine::Subtract,
                    other => return Err(format!("line {}: unknown combine '{other}'", c.line)),
                }
            }
            "round" => obj.mods.round = c.f("round")?,
            "onion" => obj.mods.onion = c.f("onion")?,
            "twist" => obj.mods.twist = c.f("twist")?,
            "bend" => obj.mods.bend = c.f("bend")?,
            "mirror" => obj.mods.mirror = [c.b("mx")?, c.b("my")?, c.b("mz")?],
            "repeat" => obj.mods.repeat = c.v3("repeat")?,
            "elongate" => obj.mods.elongate = c.v3("elongate")?,
            other => return Err(format!("line {}: unknown object field '{other}'", c.line)),
        }
    }
    obj.xform = Transform::new(pos, basis, scale);
    Ok(obj)
}
