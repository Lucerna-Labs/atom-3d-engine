//! xdsim — Cross-Domain Primitive Simulator
//!
//! Tests the game engine's primitives (the 8 root atoms as realized by the SDF sphere-tracer)
//! against a cross-domain primitive library, under the Painted Fence doctrine. Zero dependencies.
//!
//! What it actually does, each iteration, streaming JSONL the whole time:
//!   1. Parses the library (`<domain>/PRIMITIVES.md`) into structured primitives and computes
//!      cross-domain *convergence*: how many domains independently realize each root atom. The
//!      doctrine's own validation test — convergence across 4-6 domains certifies a real primitive.
//!   2. Runs an embedded CPU sphere-tracer (the engine's marcher) with the optimization levers
//!      exposed as parameters, validated against a high-budget ground-truth render every candidate.
//!   3. Harvests cross-domain *generators* (over-relaxation from signal/RF matched-filter,
//!      rate-distortion eps from information theory, empty-space seed from comp-geometry +
//!      delta-compress, adaptive shadow cap from control-opt) and tests each empirically on the
//!      engine — measuring the speedup and naming the conserved currency it charges.
//!   4. Runs a (mu+lambda) evolution strategy over the joint lever space to find the validated
//!      Pareto-best marcher, and emits the four-interruption verdict per discovery.
//!
//! Run: xdsim --library <path> --budget-seconds <n> --out <results.jsonl> [--res WxH]

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// ============================================================================
// Small zero-dep utilities: PRNG, Gaussian, JSON escape, wall-clock timestamp.
// ============================================================================

/// SplitMix64 — a tiny deterministic PRNG (no rand crate).
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_add(0x9E37_79B9_7F4A_7C15))
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform f32 in [0, 1).
    fn unit(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }
    /// Standard normal via Box-Muller.
    fn gauss(&mut self) -> f32 {
        let u1 = self.unit().max(1e-7);
        let u2 = self.unit();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos()
    }
}

/// Escape a string for embedding in a JSON value.
fn jesc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o
}

/// Seconds since the UNIX epoch (wall clock, for stream timestamps).
fn now_unix() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

// ============================================================================
// The engine primitives: a compact CPU sphere-tracer (the marcher = `fold` over
// the SDF = `compare`), with the optimization levers exposed as parameters and
// every field evaluation counted. This is the engine under test.
// ============================================================================

#[derive(Clone, Copy)]
struct V3 {
    x: f32,
    y: f32,
    z: f32,
}
impl V3 {
    fn new(x: f32, y: f32, z: f32) -> V3 {
        V3 { x, y, z }
    }
    fn add(self, o: V3) -> V3 {
        V3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
    fn sub(self, o: V3) -> V3 {
        V3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
    fn scale(self, s: f32) -> V3 {
        V3::new(self.x * s, self.y * s, self.z * s)
    }
    fn dot(self, o: V3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    fn len(self) -> f32 {
        self.dot(self).sqrt()
    }
    fn norm(self) -> V3 {
        let l = self.len();
        if l > 1e-9 {
            self.scale(1.0 / l)
        } else {
            self
        }
    }
    fn abs(self) -> V3 {
        V3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
    fn maxs(self, s: f32) -> V3 {
        V3::new(self.x.max(s), self.y.max(s), self.z.max(s))
    }
}

/// The optimization levers the marcher exposes — the search space.
#[derive(Clone, Copy)]
struct Levers {
    omega: f32,         // over-relaxation factor (1.0 = pure sphere tracing)
    eps_base: f32,      // hit epsilon at t=0
    eps_growth: f32,    // epsilon grows with distance (rate-distortion: coarser far away)
    max_steps: u32,     // primary march budget
    shadow_cap_a: f32,  // shadow step cap base
    shadow_cap_b: f32,  // shadow step cap growth with t
    coarse_seed: bool,  // empty-space pre-march seed (skip the empty foreground)
    coarse_factor: u32, // coarse pass downsample factor
    secant: bool,       // near-surface secant/regula-falsi root refinement (control-numerical-opt)
}
impl Levers {
    /// Ground-truth reference: conservative, fine, expensive — the correctness oracle.
    fn reference() -> Levers {
        Levers {
            omega: 1.0,
            eps_base: 0.0004,
            eps_growth: 0.25,
            max_steps: 256,
            shadow_cap_a: 0.02,
            shadow_cap_b: 0.0,
            coarse_seed: false,
            coarse_factor: 1,
            secant: false,
        }
    }
    /// Baseline pure sphere tracing (omega=1.0, no cross-domain generators) — the comparison point.
    fn baseline() -> Levers {
        Levers {
            omega: 1.0,
            eps_base: 0.0008,
            eps_growth: 0.0,
            max_steps: 160,
            shadow_cap_a: 0.02,
            shadow_cap_b: 0.0,
            coarse_seed: false,
            coarse_factor: 1,
            secant: false,
        }
    }
    fn clamp(&mut self) {
        self.omega = self.omega.clamp(1.0, 1.9);
        self.eps_base = self.eps_base.clamp(0.0002, 0.004);
        self.eps_growth = self.eps_growth.clamp(0.0, 1.2);
        self.shadow_cap_a = self.shadow_cap_a.clamp(0.01, 0.2);
        self.shadow_cap_b = self.shadow_cap_b.clamp(0.0, 0.8);
    }
}

/// The world SDF (the `compare` atom): ground plane + a few CSG'd primitives. Every call counts.
fn world_sdf(p: V3, count: &mut u64) -> f32 {
    *count += 1;
    // ground plane y = 0
    let mut d = p.y;
    // sphere
    d = d.min(p.sub(V3::new(-1.1, 1.0, 0.0)).len() - 1.0);
    // rounded box
    let q = p
        .sub(V3::new(1.2, 0.9, 0.3))
        .abs()
        .sub(V3::new(0.6, 0.6, 0.6));
    let bx = q.maxs(0.0).len() + q.x.max(q.y.max(q.z)).min(0.0) - 0.12;
    d = d.min(bx);
    // torus in XZ
    let c = p.sub(V3::new(0.1, 0.6, 1.8));
    let t = ((c.x * c.x + c.z * c.z).sqrt() - 0.7, c.y);
    let tor = (t.0 * t.0 + t.1 * t.1).sqrt() - 0.25;
    d = d.min(tor);
    d
}

/// Field gradient normal (four `compare` samples — the tetrahedron trick).
fn normal(p: V3, count: &mut u64) -> V3 {
    let h = 0.0009;
    let k0 = V3::new(1.0, -1.0, -1.0);
    let k1 = V3::new(-1.0, -1.0, 1.0);
    let k2 = V3::new(-1.0, 1.0, -1.0);
    let k3 = V3::new(1.0, 1.0, 1.0);
    let g = k0
        .scale(world_sdf(p.add(k0.scale(h)), count))
        .add(k1.scale(world_sdf(p.add(k1.scale(h)), count)))
        .add(k2.scale(world_sdf(p.add(k2.scale(h)), count)))
        .add(k3.scale(world_sdf(p.add(k3.scale(h)), count)));
    g.norm()
}

/// Soft shadow march (distance-adaptive cap is a cross-domain generator under test).
fn soft_shadow(origin: V3, dir: V3, lv: &Levers, count: &mut u64) -> f32 {
    let mut res = 1.0f32;
    let mut t = 0.02f32;
    for _ in 0..48 {
        let h = world_sdf(origin.add(dir.scale(t)), count);
        if h < 0.001 {
            return 0.0;
        }
        res = res.min(12.0 * h / t);
        let hi = (lv.shadow_cap_a + lv.shadow_cap_b * t).max(0.02); // ensure hi >= lo for clamp
        t += h.clamp(0.02, hi);
        if t > 12.0 {
            break;
        }
    }
    res.clamp(0.0, 1.0)
}

/// March one ray (the `fold` atom). Returns (hit, t, shade) and counts every field eval.
fn march(origin: V3, dir: V3, lv: &Levers, count: &mut u64) -> (bool, f32, f32) {
    let mut omega = lv.omega;
    let mut t = 0.0f32;
    // Empty-space seed (cross-domain: predict-then-encode / comp-geometry bound): a cheap coarse
    // step to skip the empty foreground before the fine march. Modeled as a constant safe advance
    // from a single coarse probe — conservative, so it never moves the hit point.
    if lv.coarse_seed {
        let probe = world_sdf(origin, count);
        if probe > 0.1 {
            t = (probe * 0.8).max(0.0);
        }
    }
    let mut prev_radius = 0.0f32;
    let mut step_len = 0.0f32;
    // Secant state: the previous (t, distance) sample, to estimate dd/dt along the ray.
    let mut t_prev = 0.0f32;
    let mut d_prev = f32::INFINITY;
    for _ in 0..lv.max_steps {
        let p = origin.add(dir.scale(t));
        let d = world_sdf(p, count);
        let radius = d.abs();
        let eps = lv.eps_base * (1.0 + t * lv.eps_growth);
        if omega > 1.0 && radius + prev_radius < step_len {
            step_len -= omega * step_len;
            omega = 1.0;
        } else {
            if d < eps {
                // Lambert shade with one light + soft shadow.
                let n = normal(p, count);
                let lpos = V3::new(4.0, 6.0, 3.0);
                let ldir = lpos.sub(p).norm();
                let lam = n.dot(ldir).max(0.0);
                let sh = if lam > 0.0 {
                    soft_shadow(p.add(n.scale(0.01)), ldir, lv, count)
                } else {
                    1.0
                };
                let shade = 0.12 + 0.88 * lam * sh;
                return (true, t, shade);
            }
            step_len = d * omega;
            // Secant / regula-falsi refinement (control-numerical-opt): near the surface, estimate
            // the slope dd/dt from the last two samples and step toward the predicted root. On
            // grazing rays the slope is shallow (|dd/dt| < 1), so the secant step is LARGER than the
            // safe sphere step d — exactly where sphere tracing crawls. Capped at 4*d so a wrong
            // slope can't wildly overshoot; the over-relaxation overlap test next iter is the net.
            if lv.secant && d < 0.08 && d_prev.is_finite() {
                let dt = t - t_prev;
                let dd = d - d_prev;
                if dt > 1e-6 && dd < -1e-6 {
                    let secant = -d * dt / dd;
                    step_len = secant.clamp(d, d * 4.0);
                }
            }
        }
        prev_radius = radius;
        d_prev = d;
        t_prev = t;
        t += step_len;
        if t > 30.0 {
            break;
        }
    }
    (false, t, 0.0)
}

/// Render the scene to a grayscale buffer; returns (pixels, total field evaluations).
fn render(w: u32, h: u32, lv: &Levers) -> (Vec<f32>, u64) {
    let mut buf = vec![0.0f32; (w * h) as usize];
    let mut count = 0u64;
    let eye = V3::new(0.0, 1.6, 5.2);
    let aspect = w as f32 / h as f32;
    for y in 0..h {
        for x in 0..w {
            let u = (x as f32 + 0.5) / w as f32 * 2.0 - 1.0;
            let v = 1.0 - (y as f32 + 0.5) / h as f32 * 2.0;
            let dir = V3::new(u * aspect * 0.6, v * 0.6, -1.0).norm();
            let (hit, _t, shade) = march(eye, dir, lv, &mut count);
            buf[(y * w + x) as usize] = if hit { shade } else { 0.04 };
        }
    }
    (buf, count)
}

/// Mean absolute pixel difference in [0,1] — the distortion of a candidate vs ground truth.
fn image_error(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len().max(1) as f32;
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum::<f32>() / n
}

// ============================================================================
// The cross-domain primitive library: parse `<domain>/PRIMITIVES.md` and compute
// the convergence of the 8 root atoms across domains (the doctrine's validation).
// ============================================================================

const ATOMS: [&str; 8] = [
    "scan", "hash", "fold", "project", "scale", "compare", "combine", "order",
];

struct Primitive {
    name: String,
    domain: String,
    atoms: Vec<usize>, // indices into ATOMS that this primitive wires from
    real_wall: bool,
    currency: String,
    blob: String, // lowercased definition + cross-domain wiring + alias text (for reach detection)
}

struct Library {
    primitives: Vec<Primitive>,
    domains: Vec<String>,
}

/// Parse one PRIMITIVES.md into primitives (header `### name`, fields `**Atom or composite:**`,
/// `**Real wall?**`, etc.).
#[allow(clippy::too_many_arguments)]
fn push_primitive(
    out: &mut Vec<Primitive>,
    domain: &str,
    name: &Option<String>,
    alias: &str,
    atom_text: &str,
    def_text: &str,
    wiring_text: &str,
    real_wall: bool,
    currency: &str,
) {
    if let Some(n) = name {
        let blob = format!("{} {} {} {}", alias, def_text, wiring_text, atom_text).to_lowercase();
        let mut atoms = Vec::new();
        for (i, a) in ATOMS.iter().enumerate() {
            if blob.contains(a) {
                atoms.push(i);
            }
        }
        out.push(Primitive {
            name: n.clone(),
            domain: domain.to_string(),
            atoms,
            real_wall,
            currency: currency.to_string(),
            blob,
        });
    }
}

fn parse_domain(domain: &str, text: &str) -> Vec<Primitive> {
    let mut out = Vec::new();
    let mut name: Option<String> = None;
    let mut alias = String::new();
    let mut atom_text = String::new();
    let mut def_text = String::new();
    let mut wiring_text = String::new();
    let mut real_wall = false;
    let mut currency = String::new();

    for line in text.lines() {
        let l = line.trim();
        if let Some(rest) = l.strip_prefix("### ") {
            push_primitive(
                &mut out,
                domain,
                &name,
                &alias,
                &atom_text,
                &def_text,
                &wiring_text,
                real_wall,
                &currency,
            );
            alias.clear();
            atom_text.clear();
            def_text.clear();
            wiring_text.clear();
            real_wall = false;
            currency.clear();
            let nm = rest.split('(').next().unwrap_or(rest).trim().to_string();
            if let (Some(a), Some(b)) = (rest.find('('), rest.find(')')) {
                if b > a {
                    alias = rest[a + 1..b].to_string();
                }
            }
            name = Some(nm);
        } else if let Some(rest) = l.strip_prefix("**Atom or composite:**") {
            atom_text = rest.to_string();
        } else if let Some(rest) = l.strip_prefix("**Definition:**") {
            def_text = rest.to_string();
        } else if let Some(rest) = l.strip_prefix("**Cross-domain wiring:**") {
            wiring_text = rest.to_string();
        } else if let Some(rest) = l.strip_prefix("**Real wall?**") {
            let r = rest.trim();
            real_wall = r.to_lowercase().starts_with("yes");
            for sep in ["\u{2014}", "\u{2013}", "-"] {
                if let Some(idx) = r.find(sep) {
                    currency = r[idx + sep.len()..].trim().to_string();
                    break;
                }
            }
        }
    }
    push_primitive(
        &mut out,
        domain,
        &name,
        &alias,
        &atom_text,
        &def_text,
        &wiring_text,
        real_wall,
        &currency,
    );
    out
}

/// Walk the library root: each top-level dir containing a PRIMITIVES.md is a domain.
fn load_library(root: &Path) -> Library {
    let mut primitives = Vec::new();
    let mut domains = Vec::new();
    if let Ok(rd) = fs::read_dir(root) {
        let mut entries: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path())).collect();
        entries.sort();
        for dir in entries {
            if !dir.is_dir() {
                continue;
            }
            let pm = dir.join("PRIMITIVES.md");
            if pm.exists() {
                let domain = dir
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("?")
                    .to_string();
                if let Ok(text) = fs::read_to_string(&pm) {
                    let ps = parse_domain(&domain, &text);
                    if !ps.is_empty() {
                        domains.push(domain.clone());
                        primitives.extend(ps);
                    }
                }
            }
        }
    }
    Library {
        primitives,
        domains,
    }
}

/// Signature keywords that indicate a primitive's text *reaches into* another domain — used to
/// detect cross-domain glue. A primitive that names many distant domains is a binding candidate.
fn domain_keywords() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        (
            "information-theory",
            vec![
                "information theor",
                "entropy",
                "shannon",
                "mutual information",
                "rate-distortion",
                "rate distortion",
                "kl diverg",
                "coding theorem",
            ],
        ),
        (
            "signal-rf",
            vec![
                "signal",
                "matched filter",
                "fourier",
                "spectrum",
                "beamform",
                "modulat",
                "wavelet",
                "sampling rate",
            ],
        ),
        (
            "retrieval",
            vec![
                "retrieval",
                "ranking",
                "bm25",
                "embedding",
                "posting",
                "tf-idf",
                "nearest neighbor",
            ],
        ),
        (
            "ml",
            vec![
                "neural",
                "gradient",
                "backprop",
                "attention",
                "machine learning",
                " sgd",
                "optimizer",
                "deep ",
            ],
        ),
        (
            "linear-algebra",
            vec![
                "matrix",
                "linear algebra",
                "svd",
                "eigen",
                "tensor",
                "frobenius",
                "spectral norm",
                "low-rank",
            ],
        ),
        (
            "physics",
            vec![
                "physics",
                "diffusion",
                "thermodynam",
                "free energy",
                "heat kernel",
                "boltzmann",
                "hamiltonian",
            ],
        ),
        (
            "cryptography",
            vec![
                "crypto",
                "collision-resist",
                "merkle",
                "cipher",
                "pre-image",
                "fingerprint",
            ],
        ),
        (
            "graphics",
            vec![
                "graphics",
                "render",
                "alpha compositing",
                "painter's",
                "texture",
                "level of detail",
                "rasteriz",
            ],
        ),
        (
            "database",
            vec![
                "database",
                "sketch",
                "bloom filter",
                "count-min",
                "streaming aggregat",
                "consistent hashing",
            ],
        ),
        (
            "statistics",
            vec![
                "bayes",
                "estimator",
                "variance",
                "fisher information",
                "likelihood",
                "hypothesis test",
                "confidence",
            ],
        ),
        (
            "quantum",
            vec![
                "quantum",
                "qubit",
                "entangle",
                "von neumann",
                "density matrix",
            ],
        ),
        (
            "networking",
            vec![
                "network",
                "packet",
                "routing",
                "congestion",
                "max-flow",
                "min-cut",
            ],
        ),
        (
            "control-opt",
            vec![
                "control",
                "newton",
                "trust region",
                "gradient desc",
                "convex",
                "lagrang",
                "lyapunov",
            ],
        ),
        (
            "logic",
            vec![
                "logic",
                "sat ",
                "theorem prov",
                "resolution",
                "inference rule",
                "unification",
            ],
        ),
        (
            "geometry",
            vec![
                "convex hull",
                "voronoi",
                "delaunay",
                "bounding volume",
                "simplex",
                "mesh",
            ],
        ),
        (
            "combinatorial",
            vec![
                "combinatorial",
                "matching",
                "knapsack",
                "branch and bound",
                "spanning tree",
            ],
        ),
        (
            "biology",
            vec!["genom", "sequence align", "protein", "phylogen", "biolog"],
        ),
        (
            "astrophysics",
            vec![
                "astrophys",
                "cosmolog",
                "black hole",
                "schwarzschild",
                "relativ",
            ],
        ),
        (
            "queueing",
            vec![
                "queue",
                "markov chain",
                "poisson",
                "birth-death",
                "stochastic process",
            ],
        ),
    ]
}

/// The set of domains a primitive's text reaches (its home domain plus any referenced by keyword).
fn primitive_reach(p: &Primitive, kw: &[(&'static str, Vec<&'static str>)]) -> Vec<String> {
    let mut reach: Vec<String> = vec![p.domain.clone()];
    for (dom, keys) in kw {
        let d = dom.to_string();
        if reach.contains(&d) {
            continue;
        }
        if keys.iter().any(|k| p.blob.contains(k)) {
            reach.push(d);
        }
    }
    reach
}

fn order_pair(a: &str, b: &str) -> (String, String) {
    if a <= b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}

/// Glue-scoring tuple: (glue_score, base_score, primitive index, top bridges, adapter hits, engine-relevant).
type GlueScored = (f64, f64, usize, Vec<(String, String, u32)>, u32, bool);
/// Recipe-search hit: (score, glue bridges, steps as (name, domain), verdict note).
type RecipeHit = (f64, u32, Vec<(String, String)>, String);

/// Verbs that signal a primitive is an *adapter* — it converts between representations, which is
/// the actual mechanism of glue (not merely co-mentioning two domains).
const ADAPTER_VERBS: [&str; 15] = [
    "convert",
    "transform",
    "mapping",
    " maps ",
    "between ",
    "encode",
    "decode",
    "embed",
    "lift ",
    "bridge",
    "translate",
    "interpolat",
    "reduction",
    " project",
    "represent",
];
/// Domains the SDF marcher actually touches — glue reaching these can be tested on the engine.
const ENGINE_TOKENS: [&str; 12] = [
    "graphics",
    "geometry",
    "linear-algebra",
    "signal-rf",
    "control-opt",
    "information-theory",
    "graphics-rendering-lod",
    "computational-geometry",
    "linear-algebra-matrix",
    "signal-processing-rf",
    "control-numerical-opt",
    "information-theory-coding",
];

/// Find binding ("glue") primitives: those that uniquely join distant domains few others connect.
/// Deepened: base = bridge-rarity x reach x real-wall utility; then weighted up if the primitive is
/// a genuine *adapter* (converts between representations) and if it reaches the engine's domains
/// (so the top hits are glue we can actually validate on the marcher).
fn binding_analysis(lib: &Library, sink: &mut Sink, top_n: usize) {
    let kw = domain_keywords();
    let reaches: Vec<Vec<String>> = lib
        .primitives
        .iter()
        .map(|p| primitive_reach(p, &kw))
        .collect();

    // How many primitives bridge each unordered domain-pair (commonness of the connection).
    let mut pair_count: HashMap<(String, String), u32> = HashMap::new();
    for reach in &reaches {
        for i in 0..reach.len() {
            for j in (i + 1)..reach.len() {
                *pair_count
                    .entry(order_pair(&reach[i], &reach[j]))
                    .or_insert(0) += 1;
            }
        }
    }

    // (glue_score, base_score, idx, bridges, adapter_hits, engine_relevant)
    let mut scored: Vec<GlueScored> = Vec::new();
    for (idx, reach) in reaches.iter().enumerate() {
        if reach.len() < 2 {
            continue;
        }
        let mut base = 0.0f64;
        let mut bridges: Vec<(String, String, u32)> = Vec::new();
        for i in 0..reach.len() {
            for j in (i + 1)..reach.len() {
                let pair = order_pair(&reach[i], &reach[j]);
                let c = *pair_count.get(&pair).unwrap_or(&1);
                base += 1.0 / c as f64; // rarer glue weighs more
                bridges.push((pair.0, pair.1, c));
            }
        }
        if lib.primitives[idx].real_wall {
            base *= 1.5; // load-bearing glue (names a conserved currency) over free glue
        }
        base *= (reach.len() as f64).sqrt(); // reward breadth of reach

        let blob = &lib.primitives[idx].blob;
        let adapter_hits = ADAPTER_VERBS.iter().filter(|v| blob.contains(**v)).count() as u32;
        let engine_relevant = reach.iter().any(|d| ENGINE_TOKENS.contains(&d.as_str()));
        // adapters are real glue; engine-reaching glue is testable — weight both up.
        let glue =
            base * (1.0 + 0.4 * adapter_hits as f64) * (if engine_relevant { 1.5 } else { 1.0 });

        bridges.sort_by_key(|b| b.2); // rarest bridges first
        bridges.truncate(4);
        scored.push((glue, base, idx, bridges, adapter_hits, engine_relevant));
    }
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    sink.emit(
        "binding_start",
        format!(
            "\"primitives\":{},\"distinct_domain_pairs\":{},\"definition\":\"glue = rare cross-domain bridge x reach x real-wall, weighted by adapter-signature (converts representations) and engine-relevance (reaches the marcher's domains)\"",
            lib.primitives.len(),
            pair_count.len()
        ),
    );
    for (rank, (glue, base, idx, bridges, adapter_hits, engine_relevant)) in
        scored.iter().take(top_n).enumerate()
    {
        let p = &lib.primitives[*idx];
        let bs: Vec<String> = bridges
            .iter()
            .map(|(a, b, c)| format!("{}<->{} (only {} share it)", a, b, c))
            .collect();
        sink.emit(
            "binding",
            format!(
                "\"rank\":{},\"primitive\":\"{}\",\"home_domain\":\"{}\",\"glue_score\":{:.3},\"base_score\":{:.3},\"adapter_hits\":{},\"engine_relevant\":{},\"reach\":{},\"real_wall\":{},\"currency\":\"{}\",\"rare_bridges\":\"{}\"",
                rank + 1,
                jesc(&p.name),
                jesc(&p.domain),
                glue,
                base,
                adapter_hits,
                engine_relevant,
                reaches[*idx].len(),
                p.real_wall,
                jesc(&p.currency),
                jesc(&bs.join(" | "))
            ),
        );
    }
}

/// For each atom, the set of domains that independently realize it (convergence).
fn atom_convergence(lib: &Library) -> [Vec<String>; 8] {
    let mut sets: [Vec<String>; 8] = Default::default();
    for p in &lib.primitives {
        for &a in &p.atoms {
            if !sets[a].contains(&p.domain) {
                sets[a].push(p.domain.clone());
            }
        }
    }
    sets
}

// ============================================================================
// JSONL streaming sink: append one record per discovery to a file and stdout.
// ============================================================================

struct Sink {
    file: fs::File,
    n: u64,
}
impl Sink {
    fn new(path: &str) -> Sink {
        let file = fs::File::create(path).expect("create results file");
        Sink { file, n: 0 }
    }
    /// Emit a pre-built JSON object string (without surrounding braces) with a common envelope.
    fn emit(&mut self, kind: &str, body: String) {
        self.n += 1;
        let rec = format!(
            "{{\"seq\":{},\"ts\":{:.3},\"kind\":\"{}\",{}}}",
            self.n,
            now_unix(),
            kind,
            body
        );
        let _ = writeln!(self.file, "{}", rec);
        let _ = self.file.flush();
        println!("{}", rec);
        let _ = std::io::stdout().flush();
    }
}

// ============================================================================
// Cross-domain experiments: each toggles one generator harvested from a domain,
// tested empirically against the baseline, with its conserved currency named and
// the four-interruption verdict (painted vs real wall, where cost/policy live).
// ============================================================================

struct Experiment {
    name: &'static str,
    atom: &'static str,
    domain_source: &'static str,
    library_primitive: &'static str,
    currency: &'static str,
    apply: fn(Levers) -> Levers,
}

fn experiments() -> Vec<Experiment> {
    vec![
        Experiment {
            name: "over_relaxation",
            atom: "fold",
            domain_source: "signal-processing-rf",
            library_primitive: "matched-filter (project/correlate)",
            currency: "degrees of freedom (over-step risk vs empty-space skipped)",
            apply: |mut l| {
                l.omega = 1.4;
                l
            },
        },
        Experiment {
            name: "rate_distortion_eps",
            atom: "scale",
            domain_source: "information-theory-coding",
            library_primitive: "rate-distort / quantize-info",
            currency: "bits per sample (distortion budget R(D))",
            apply: |mut l| {
                l.eps_growth = 0.6;
                l
            },
        },
        Experiment {
            name: "empty_space_seed",
            atom: "scan",
            domain_source: "computational-geometry",
            library_primitive: "delta-compress / predict-then-encode",
            currency: "bits of the accumulator (fold) — coarse prediction residual",
            apply: |mut l| {
                l.coarse_seed = true;
                l.coarse_factor = 4;
                l
            },
        },
        Experiment {
            name: "adaptive_shadow_cap",
            atom: "order",
            domain_source: "control-numerical-opt",
            library_primitive: "trust-region / adaptive step",
            currency: "step budget (O(n) shadow evals)",
            apply: |mut l| {
                l.shadow_cap_b = 0.4;
                l
            },
        },
        Experiment {
            name: "secant_root_refine",
            atom: "compare",
            domain_source: "control-numerical-opt",
            library_primitive: "secant / regula-falsi root-finding",
            currency: "near-surface field-evals (convergence order of the root-find)",
            apply: |mut l| {
                l.secant = true;
                l
            },
        },
    ]
}

/// One render measurement.
struct Meas {
    evals: u64,
    error: f32,
}
fn measure(w: u32, h: u32, lv: &Levers, reference: &[f32]) -> Meas {
    let (img, evals) = render(w, h, lv);
    Meas {
        evals,
        error: image_error(&img, reference),
    }
}

/// Four-interruption verdict for a measured generator vs baseline.
fn verdict(base: &Meas, cand: &Meas, tol: f32) -> (String, String) {
    let speedup = base.evals as f64 / cand.evals.max(1) as f64;
    let correct = cand.error <= tol;
    // Painted wall: a "this is as fast as it gets" wall that a cheap generator routes around (faster
    // AND still correct). Real wall: the win charges correctness (faster but distortion rose past tol).
    let painted_or_real = if speedup > 1.02 && correct {
        "painted (routed around: faster at equal correctness)"
    } else if speedup > 1.02 && !correct {
        "real (the speedup charged correctness — distortion exceeded tolerance)"
    } else if speedup <= 1.02 && correct {
        "neutral (no speedup; generator did not move the cost)"
    } else {
        "regressive (slower and/or incorrect)"
    };
    let note = format!(
        "speedup={:.3}x, error={:.5} (tol={:.5}), correct={}",
        speedup, cand.error, tol, correct
    );
    (painted_or_real.to_string(), note)
}

// ============================================================================
// Evolution strategy: (mu+lambda) over the joint lever space, minimizing field
// evals subject to staying within the distortion tolerance vs ground truth.
// ============================================================================

fn fitness(m: &Meas, tol: f32) -> f64 {
    // minimize evals; heavy penalty for exceeding the distortion tolerance.
    m.evals as f64 + 5.0e6 * (m.error - tol).max(0.0) as f64
}

fn mutate(l: &Levers, rng: &mut Rng, sigma: f32) -> Levers {
    let mut n = *l;
    n.omega += rng.gauss() * 0.15 * sigma;
    n.eps_base += rng.gauss() * 0.0006 * sigma;
    n.eps_growth += rng.gauss() * 0.2 * sigma;
    n.shadow_cap_a += rng.gauss() * 0.02 * sigma;
    n.shadow_cap_b += rng.gauss() * 0.1 * sigma;
    if rng.unit() < 0.1 {
        n.coarse_seed = !n.coarse_seed;
    }
    if rng.unit() < 0.1 {
        n.secant = !n.secant;
    }
    n.clamp();
    n
}

// ============================================================================
// Shape / symbol layer: the lightest "symbol" — track tensor RANK through a
// recipe so a shape-aware composer rejects rank-collapse (the hollow-recipe
// class the type-only sim rewarded) at compose time, and rewards shape-coherent
// cross-domain (glue) recipes. Symbols COMBINE: each step's shape composes the
// running rank, exactly like the primitives compose the computation.
// ============================================================================

/// Output-rank sentinel meaning "same rank as the input" (rank-preserving op).
const PRESERVE: u8 = 255;

/// A primitive's shape signature: the minimum input rank it accepts, and the rank it outputs.
struct Shape {
    needs: u8,
    out_rank: u8,
}

/// Heuristic shape classifier from a primitive's name — a stand-in for a machine-readable
/// `Signature:` field in the store. Reductions collapse to a scalar (rank 0); decompositions /
/// contractions need a matrix (rank >= 2); everything else is treated as rank-preserving.
fn shape_of(name: &str) -> Shape {
    let n = name.to_lowercase();
    let has = |k: &str| n.contains(k);
    // scalar reductions: matrix/vector -> scalar (rank 0). NOTE: "normalize" is excluded ("normal").
    if (has("norm") && !has("normal"))
        || has("frobenius")
        || has("spectral")
        || has("trace")
        || has("determinant")
        || has("entropy")
        || has("perplexit")
        || has("divergence")
        || has("kl-")
        || has("-loss")
        || has("loss-")
    {
        return Shape {
            needs: 1,
            out_rank: 0,
        };
    }
    // matrix decompositions / contractions: need rank >= 2, produce a matrix.
    if has("svd")
        || has("eig")
        || has(" qr")
        || has("cholesky")
        || has("decomp")
        || has("factor")
        || has("pseudoinverse")
        || has("nmf")
        || has("matmul")
        || has("gemm")
        || has("contract")
        || has("tensor")
        || has("tensordot")
    {
        return Shape {
            needs: 2,
            out_rank: 2,
        };
    }
    if has("outer") || has("kron") || has("gram") {
        return Shape {
            needs: 1,
            out_rank: 2,
        };
    }
    Shape {
        needs: 0,
        out_rank: PRESERVE,
    }
}

/// Evaluate a recipe under the shape layer + glue. Returns
/// (shape_valid, break_index, glue_bridges, score, verdict).
fn eval_recipe(steps: &[(String, String)]) -> (bool, Option<usize>, u32, f64, String) {
    let mut rank: u8 = 2; // a matrix input
    let mut collapsed_at: Option<usize> = None;
    for (i, (name, _dom)) in steps.iter().enumerate() {
        let s = shape_of(name);
        if rank < s.needs {
            let note = format!(
                "INVALID at #{} `{}`: needs rank>={}, have rank {}",
                i + 1,
                name,
                s.needs,
                rank
            );
            return (false, Some(i), 0, 0.0, note);
        }
        if s.out_rank != PRESERVE {
            if s.out_rank == 0 && rank > 0 {
                collapsed_at = Some(i);
            }
            rank = s.out_rank;
        }
    }
    // glue = adjacent cross-domain transitions (the "bridges" the recipe makes).
    let mut glue = 0u32;
    for w in steps.windows(2) {
        if w[0].1 != w[1].1 {
            glue += 1;
        }
    }
    let dead = collapsed_at
        .map(|c| (steps.len() - 1 - c) as f64)
        .unwrap_or(0.0);
    let score = 10.0 + glue as f64 * 4.0 - dead * 3.0;
    let note = match collapsed_at {
        Some(c) => format!(
            "valid but collapses to scalar at #{} ({} vestigial step(s))",
            c + 1,
            steps.len() - 1 - c
        ),
        None => "shape-coherent end to end".to_string(),
    };
    (true, None, glue, score, note)
}

fn main() {
    // ---- args ----
    let args: Vec<String> = std::env::args().collect();
    let mut library = String::from(".");
    let mut budget_seconds = 30.0f64;
    let mut out = String::from("xdsim-results.jsonl");
    let mut res = (96u32, 54u32);
    let mut mode = String::from("optimize");
    let mut recipe_len = 6usize;
    let mut recipe_samples: u64 = 300_000;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--mode" => {
                i += 1;
                mode = args.get(i).cloned().unwrap_or(mode);
            }
            "--recipe-len" => {
                i += 1;
                recipe_len = args
                    .get(i)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(recipe_len);
            }
            "--recipe-samples" => {
                i += 1;
                recipe_samples = args
                    .get(i)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(recipe_samples);
            }
            "--library" => {
                i += 1;
                library = args.get(i).cloned().unwrap_or(library);
            }
            "--budget-seconds" => {
                i += 1;
                budget_seconds = args
                    .get(i)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(budget_seconds);
            }
            "--out" => {
                i += 1;
                out = args.get(i).cloned().unwrap_or(out);
            }
            "--res" => {
                i += 1;
                if let Some(s) = args.get(i) {
                    if let Some((a, b)) = s.split_once('x') {
                        if let (Ok(a), Ok(b)) = (a.parse(), b.parse()) {
                            res = (a, b);
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }

    let (w, h) = res;
    let mut sink = Sink::new(&out);
    let start = Instant::now();

    // ---- Phase 0: library + convergence ----
    let lib = load_library(Path::new(&library));
    sink.emit(
        "library_loaded",
        format!(
            "\"path\":\"{}\",\"domains\":{},\"primitives\":{}",
            jesc(&library),
            lib.domains.len(),
            lib.primitives.len()
        ),
    );

    // Binding mode: find the cross-domain glue primitives and stop (no sphere-tracer search).
    if mode == "binding" {
        binding_analysis(&lib, &mut sink, 25);
        return;
    }

    // Recipe mode: shape/symbol + glue aware composition search. It rejects the rank-collapse
    // "hollow recipe" class the type-only sim rewarded, and surfaces shape-coherent cross-domain
    // (glue) recipes instead. Small + targeted — runs locally in well under a second.
    if mode == "recipe" {
        let hollow = [
            "dot",
            "project-vec",
            "normalize-vec",
            "scale-vec",
            "random-project",
            "randomized-svd",
            "frobenius-norm",
            "spectral-norm",
            "tensor-contract",
        ];
        let hs: Vec<(String, String)> = hollow
            .iter()
            .map(|n| (n.to_string(), "linear-algebra-matrix".to_string()))
            .collect();
        let (hv, hbreak, hg, hsc, hn) = eval_recipe(&hs);
        sink.emit(
            "recipe_hollow",
            format!(
                "\"recipe\":\"{}\",\"shape_valid\":{},\"break_at\":{},\"glue\":{},\"score\":{:.1},\"verdict\":\"{}\"",
                jesc(&hollow.join(" -> ")),
                hv,
                hbreak.map(|b| (b + 1) as i64).unwrap_or(-1),
                hg,
                hsc,
                jesc(&hn)
            ),
        );

        let n = lib.primitives.len();
        if n == 0 {
            sink.emit(
                "recipe_search",
                "\"error\":\"no primitives parsed\"".to_string(),
            );
            return;
        }
        let mut rng = Rng::new(0x5EED_BEEF_1234_5678);
        let len = recipe_len.max(3);
        let mut found: Vec<RecipeHit> = Vec::new();
        for _ in 0..recipe_samples {
            let mut steps: Vec<(String, String)> = Vec::with_capacity(len);
            for _ in 0..len {
                let p = &lib.primitives[(rng.next_u64() as usize) % n];
                steps.push((p.name.clone(), p.domain.clone()));
            }
            let (v, _b, g, s, note) = eval_recipe(&steps);
            if v && g >= 2 {
                found.push((s, g, steps, note));
            }
        }
        found.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        sink.emit(
            "recipe_search",
            format!(
                "\"samples\":{},\"recipe_len\":{},\"shape_valid_glue_recipes\":{},\"note\":\"top = shape-coherent AND cross-domain (glue>=2)\"",
                recipe_samples, len, found.len()
            ),
        );
        let mut seen = std::collections::HashSet::new();
        let mut rank = 0u32;
        for (s, g, steps, note) in &found {
            let names: Vec<String> = steps.iter().map(|x| x.0.clone()).collect();
            if !seen.insert(names.join("|")) {
                continue;
            }
            let mut doms: Vec<String> = steps.iter().map(|x| x.1.clone()).collect();
            doms.sort();
            doms.dedup();
            rank += 1;
            sink.emit(
                "recipe",
                format!(
                    "\"rank\":{},\"score\":{:.1},\"glue\":{},\"domains\":{},\"recipe\":\"{}\",\"note\":\"{}\"",
                    rank,
                    s,
                    g,
                    doms.len(),
                    jesc(&names.join(" -> ")),
                    jesc(note)
                ),
            );
            if rank >= 15 {
                break;
            }
        }
        return;
    }

    let conv = atom_convergence(&lib);
    // Engine uses 7 of 8 atoms (hash is the procedural/spatial layer).
    let engine_atoms: [(&str, &str); 7] = [
        ("scan", "ray/pixel walk"),
        ("fold", "sphere-trace march"),
        ("project", "camera ray + dot"),
        ("scale", "normalize / step"),
        ("compare", "SDF distance"),
        ("combine", "CSG smooth-union"),
        ("order", "depth / painter"),
    ];
    for (atom, role) in engine_atoms {
        let ai = ATOMS.iter().position(|a| *a == atom).unwrap();
        let domains = &conv[ai];
        let validated = domains.len() >= 4; // doctrine: convergence across >=4 domains certifies it
        sink.emit(
            "convergence",
            format!(
                "\"atom\":\"{}\",\"engine_role\":\"{}\",\"cross_domain_count\":{},\"validated\":{},\"domains\":\"{}\"",
                atom,
                jesc(role),
                domains.len(),
                validated,
                jesc(&domains.join(", "))
            ),
        );
    }

    // Real-wall census: the doctrine's real-vs-painted boundary across the whole library.
    let real_walls = lib.primitives.iter().filter(|p| p.real_wall).count();
    let sample: Vec<String> = lib
        .primitives
        .iter()
        .filter(|p| p.real_wall && !p.currency.is_empty())
        .take(8)
        .map(|p| format!("{} [{}] charges {}", p.name, p.domain, p.currency))
        .collect();
    sink.emit(
        "library_census",
        format!(
            "\"real_wall_primitives\":{},\"painted_or_free\":{},\"total\":{},\"sample_real_walls\":\"{}\"",
            real_walls,
            lib.primitives.len() - real_walls,
            lib.primitives.len(),
            jesc(&sample.join(" | "))
        ),
    );

    // ---- Reference (ground truth) ----
    let (reference, ref_evals) = render(w, h, &Levers::reference());
    let base = measure(w, h, &Levers::baseline(), &reference);
    let tol = 0.012f32; // distortion tolerance vs ground truth (mean abs gray diff)
    sink.emit(
        "ground_truth",
        format!(
            "\"res\":\"{}x{}\",\"reference_evals\":{},\"baseline_evals\":{},\"baseline_error\":{:.5},\"tolerance\":{:.5}",
            w, h, ref_evals, base.evals, base.error, tol
        ),
    );

    // ---- Phase 1: cross-domain generator experiments (one-shot, attributed) ----
    for ex in experiments() {
        let lv = (ex.apply)(Levers::baseline());
        let m = measure(w, h, &lv, &reference);
        let (paint, note) = verdict(&base, &m, tol);
        sink.emit(
            "experiment",
            format!(
                "\"name\":\"{}\",\"atom\":\"{}\",\"domain_source\":\"{}\",\"library_primitive\":\"{}\",\"currency\":\"{}\",\"evals\":{},\"baseline_evals\":{},\"error\":{:.5},\"wall\":\"{}\",\"note\":\"{}\"",
                ex.name,
                ex.atom,
                jesc(ex.domain_source),
                jesc(ex.library_primitive),
                jesc(ex.currency),
                m.evals,
                base.evals,
                m.error,
                jesc(&paint),
                jesc(&note)
            ),
        );
    }

    // ---- Phase 2: evolution strategy until the wall-clock budget expires ----
    let mut rng = Rng::new(0x00C0_FFEE_1234_5678);
    let mu = 6usize;
    let lambda = 18usize;
    // seed population around the all-generators-on config
    let mut seed = Levers::baseline();
    seed.omega = 1.4;
    seed.eps_growth = 0.4;
    seed.coarse_seed = true;
    seed.secant = true;
    let mut pop: Vec<(Levers, f64, Meas)> = Vec::new();
    for _ in 0..mu {
        let l = mutate(&seed, &mut rng, 1.0);
        let m = measure(w, h, &l, &reference);
        let f = fitness(&m, tol);
        pop.push((l, f, m));
    }
    pop.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut gen = 0u64;
    let mut best_evals = pop[0].2.evals;
    sink.emit(
        "search_start",
        format!(
            "\"mu\":{},\"lambda\":{},\"seed_best_evals\":{}",
            mu, lambda, best_evals
        ),
    );

    while start.elapsed().as_secs_f64() < budget_seconds {
        gen += 1;
        let sigma = (1.0 - (gen as f32 / 400.0)).max(0.2); // anneal mutation
        let mut children: Vec<(Levers, f64, Meas)> = Vec::with_capacity(lambda);
        for _ in 0..lambda {
            let parent = &pop[(rng.next_u64() as usize) % pop.len()].0;
            let l = mutate(parent, &mut rng, sigma);
            let m = measure(w, h, &l, &reference);
            let f = fitness(&m, tol);
            children.push((l, f, m));
        }
        pop.append(&mut children);
        pop.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        pop.truncate(mu);

        let b = &pop[0];
        if b.2.evals < best_evals && b.2.error <= tol {
            best_evals = b.2.evals;
            let speedup = base.evals as f64 / best_evals.max(1) as f64;
            sink.emit(
                "improvement",
                format!(
                    "\"gen\":{},\"best_evals\":{},\"baseline_evals\":{},\"speedup_vs_baseline\":{:.3},\"error\":{:.5},\"omega\":{:.3},\"eps_base\":{:.5},\"eps_growth\":{:.3},\"shadow_cap_a\":{:.3},\"shadow_cap_b\":{:.3},\"coarse_seed\":{},\"secant\":{}",
                    gen, best_evals, base.evals, speedup, b.2.error, b.0.omega, b.0.eps_base, b.0.eps_growth, b.0.shadow_cap_a, b.0.shadow_cap_b, b.0.coarse_seed, b.0.secant
                ),
            );
        }
        if gen.is_multiple_of(25) {
            sink.emit(
                "heartbeat",
                format!(
                    "\"gen\":{},\"elapsed_s\":{:.1},\"budget_s\":{:.0},\"best_evals\":{},\"pop_best_fitness\":{:.0}",
                    gen,
                    start.elapsed().as_secs_f64(),
                    budget_seconds,
                    best_evals,
                    pop[0].1
                ),
            );
        }
    }

    // ---- Final report ----
    let b = &pop[0];
    let speedup = base.evals as f64 / b.2.evals.max(1) as f64;
    sink.emit(
        "final",
        format!(
            "\"generations\":{},\"elapsed_s\":{:.1},\"baseline_evals\":{},\"best_evals\":{},\"speedup_vs_baseline\":{:.3},\"reference_evals\":{},\"error\":{:.5},\"tolerance\":{:.5},\"best_omega\":{:.3},\"best_eps_base\":{:.5},\"best_eps_growth\":{:.3},\"best_shadow_cap_a\":{:.3},\"best_shadow_cap_b\":{:.3},\"best_coarse_seed\":{},\"best_secant\":{},\"conserved_currency\":\"field-evaluations per validated frame (correctness held at tol)\"",
            gen,
            start.elapsed().as_secs_f64(),
            base.evals,
            b.2.evals,
            speedup,
            ref_evals,
            b.2.error,
            tol,
            b.0.omega,
            b.0.eps_base,
            b.0.eps_growth,
            b.0.shadow_cap_a,
            b.0.shadow_cap_b,
            b.0.coarse_seed,
            b.0.secant
        ),
    );
}
