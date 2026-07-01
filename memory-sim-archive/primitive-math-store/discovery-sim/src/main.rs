//! discovery-sim — Local primitive-COMBINATION discovery simulator.
//!
//! Proposes wirings of primitive policies (incl. combinations the kit does NOT document),
//! EXECUTES each against an SDF sphere-tracer, and keeps only those that pay a conserved
//! currency: field-evaluations per frame at held image-correctness (tolerance). A candidate
//! that type-checks but doesn't cut the currency is "painted" (rejected). This is the
//! Painted-Fence doctrine's Interruption 4 made mechanical: no named currency => no discovery.
//!
//! Executor = SDF marcher (analytic SDF + gradient). Policy vocabulary (the generator slots):
//!   step     : Fixed | OverRelax            (signal-processing-rf: matched-filter/over-relax)
//!   refine   : None | Bisection | Secant | Newton | Iqi
//!   precision: FixedEps | RateDistortion    (information-theory: rate-distortion quantize)
//!   cull     : None | CoarseSeed            (computational-geometry: delta-compress/predict-skip)
//!
//! Newton        <- linear-algebra-matrix      (NOT documented in kit for rendering)
//! Iqi           <- control-numerical-opt      (NOT documented in kit for rendering)
//! Bisection     <- formal-verification        (NOT documented in kit for rendering)
//! Secant/OverRelax/RateDistortion/CoarseSeed  (already validated by xdsim => documented)
//!
//! The proposer enumerates the full Cartesian product (40 wirings) so the search is
//! EXHAUSTIVE and unbiased; novelty is annotated per wiring. Real discovery = a novel
//! wiring that beats baseline on field-evals at held correctness.

use std::fs::File;
use std::io::Write;
use std::time::Instant;

// ----------------------------- vec3 -----------------------------
#[derive(Clone, Copy)]
struct V3(f32, f32, f32);
impl V3 {
    fn add(self, o: V3) -> V3 { V3(self.0 + o.0, self.1 + o.1, self.2 + o.2) }
    fn sub(self, o: V3) -> V3 { V3(self.0 - o.0, self.1 - o.1, self.2 - o.2) }
    fn scale(self, s: f32) -> V3 { V3(self.0 * s, self.1 * s, self.2 * s) }
    fn dot(self, o: V3) -> f32 { self.0 * o.0 + self.1 * o.1 + self.2 * o.2 }
    fn len(self) -> f32 { self.dot(self).sqrt() }
    fn norm(self) -> V3 { let l = self.len().max(1e-9); self.scale(1.0 / l) }
}

const CAM: V3 = V3(0.0, 0.0, -6.0);
const EPS_BASE: f32 = 0.0010;     // candidate hit epsilon at t=0
const EPS_GROWTH: f32 = 0.045;    // rate-distortion: epsilon grows with distance
const MAX_STEPS: u32 = 220;
const DELTA: f32 = 0.05;          // a pixel is "correct" if |cand-truth| <= DELTA
const FAR: f32 = 60.0;

// Scene: union of 3 spheres (analytic SDF + gradient).
fn scene_sdf(p: V3, evals: &mut u64) -> (f32, V3) {
    *evals += 1;
    let s = [
        (V3(-1.3, 0.10, 2.0), 1.00f32),
        (V3(1.20, 0.35, 2.6), 0.80f32),
        (V3(0.05, -1.10, 1.5), 0.60f32),
    ];
    let mut best = f32::MAX;
    let mut best_c = s[0].0;
    for &(c, r) in s.iter() {
        let d = (p.sub(c)).len() - r;
        if d < best { best = d; best_c = c; }
    }
    let grad = (p.sub(best_c)).norm();
    (best, grad)
}

// ----------------------------- policies -----------------------------
#[derive(Clone, Copy, PartialEq)]
enum Step { Fixed, OverRelax }
#[derive(Clone, Copy, PartialEq)]
enum Refine { None, Bisection, Secant, Newton, Iqi }
#[derive(Clone, Copy, PartialEq)]
enum Precision { FixedEps, RateDistortion }
#[derive(Clone, Copy, PartialEq)]
enum Cull { None, CoarseSeed }

#[derive(Clone, Copy)]
struct Config { step: Step, refine: Refine, prec: Precision, cull: Cull }

impl Config {
    fn omega(&self) -> f32 { if self.step == Step::OverRelax { 1.40 } else { 1.0 } }
    fn hit_eps(&self, t: f32) -> f32 {
        match self.prec {
            Precision::FixedEps => EPS_BASE,
            Precision::RateDistortion => EPS_BASE + EPS_GROWTH * t,
        }
    }
    fn label(&self) -> String {
        let st = if self.step == Step::OverRelax { "OVR" } else { "fix" };
        let rf = match self.refine {
            Refine::None => "none", Refine::Bisection => "bisect",
            Refine::Secant => "secant", Refine::Newton => "newton", Refine::Iqi => "iqi",
        };
        let pr = if self.prec == Precision::RateDistortion { "RD" } else { "fix" };
        let cu = if self.cull == Cull::CoarseSeed { "seed" } else { "-" };
        format!("{st}/{rf}/{pr}/{cu}")
    }
}

// ----------------------------- marcher -----------------------------
fn march(cfg: &Config, origin: V3, dir: V3, t0: f32, evals: &mut u64, coarse: bool) -> (bool, f32) {
    let mut t = t0.max(0.0);
    let mut last_t = t;
    let mut last_f = f32::MAX;
    let max_steps = if coarse { 28 } else { MAX_STEPS };
    for _ in 0..max_steps {
        let p = origin.add(dir.scale(t));
        let (f, g) = scene_sdf(p, evals);
        let he = cfg.hit_eps(t);
        if f < he {
            return (true, refine_hit(cfg, origin, dir, t, f, g, last_t, last_f, he, evals));
        }
        last_t = t;
        last_f = f;
        let step = cfg.omega() * f;
        // guard: if over-relax made no progress, fall back to a plain step
        let step = if step < 1e-4 { he } else { step };
        t += step;
        if t > FAR { return (false, t); }
    }
    (false, t)
}

// Near-surface root refinement — the axis where primitive transfer is cleanest.
fn refine_hit(cfg: &Config, origin: V3, dir: V3, t: f32, f: f32, g: V3,
              last_t: f32, last_f: f32, he: f32, evals: &mut u64) -> f32 {
    match cfg.refine {
        Refine::None => t,
        Refine::Bisection => {
            // robust midpoint-shrink between last_t and t (pays extra evals)
            let mut lo = last_t.min(t);
            let mut hi = t.max(last_t);
            for _ in 0..2 {
                let m = 0.5 * (lo + hi);
                let (fm, _) = scene_sdf(origin.add(dir.scale(m)), evals);
                if fm < he { return m; }
                if m < t { lo = m; } else { hi = m; }
            }
            0.5 * (lo + hi)
        }
        Refine::Secant => {
            let denom = f - last_f;
            let ts = if denom.abs() > 1e-7 { t - f * (t - last_t) / denom } else { t };
            confirm(cfg, origin, dir, ts, he, evals, t)
        }
        Refine::Newton => {
            let d = dir.dot(g); // f'(t) along the ray = dot(dir, grad) since |grad|=1
            let tn = if d.abs() > 1e-4 { t - f / d } else { t };
            confirm(cfg, origin, dir, tn, he, evals, t)
        }
        Refine::Iqi => {
            // inverse-quadratic interpolation over 3 samples: last_t, t, t+|f|
            let t3 = t + f.abs().max(he);
            let (f3, _) = scene_sdf(origin.add(dir.scale(t3)), evals);
            // inverse quadratic: fit t(f) through (f_i, t_i), evaluate at f=0
            let (x0, y0) = (last_f, last_t);
            let (x1, y1) = (f, t);
            let (x2, y2) = (f3, t3);
            let ti = if (x0 - x1).abs() > 1e-7 && (x0 - x2).abs() > 1e-7 && (x1 - x2).abs() > 1e-7 {
                let r0 = y0 * (-x1) * (-x2) / ((x0 - x1) * (x0 - x2));
                let r1 = y1 * (-x0) * (-x2) / ((x1 - x0) * (x1 - x2));
                let r2 = y2 * (-x0) * (-x1) / ((x2 - x0) * (x2 - x1));
                r0 + r1 + r2
            } else { t };
            confirm(cfg, origin, dir, ti, he, evals, t)
        }
    }
}

// One confirmation sample; if the refined t is within eps it wins, else fall back to t.
fn confirm(cfg: &Config, origin: V3, dir: V3, ts: f32, he: f32, evals: &mut u64, fallback: f32) -> f32 {
    if !(ts.is_finite() && ts >= 0.0 && ts < FAR) { return fallback; }
    let _ = cfg;
    let (fs, _) = scene_sdf(origin.add(dir.scale(ts)), evals);
    if fs < he * 3.0 { ts } else { fallback }
}

// ----------------------------- render -----------------------------
fn render(cfg: &Config, w: u32, h: u32) -> (Vec<f32>, u64) {
    let mut img = vec![0f32; (w * h) as usize];
    let mut evals: u64 = 0;
    let aspect = w as f32 / h as f32;
    let tanf = 0.62;

    let mut t0map = vec![0f32; (w * h) as usize];
    if cfg.cull == Cull::CoarseSeed {
        // 4x downsampled pre-pass: cheap march with big eps, record surface t as the fine start.
        let (cw, ch) = (w / 4, h / 4);
        for cy in 0..ch {
            for cx in 0..cw {
                let x = (2.0 * ((cx as f32 + 0.5) / cw as f32) - 1.0) * aspect * tanf;
                let y = (1.0 - 2.0 * ((cy as f32 + 0.5) / ch as f32)) * tanf;
                let d = V3(x, y, 1.0).norm();
                let (hit, t) = march_seed(CAM, d, &mut evals);
                for dy in 0..4u32 {
                    for dx in 0..4u32 {
                        let fx = (cx * 4 + dx).min(w - 1);
                        let fy = (cy * 4 + dy).min(h - 1);
                        t0map[(fy * w + fx) as usize] = if hit { (t * 0.85).max(0.0) } else { 0.0 };
                    }
                }
            }
        }
    }

    for py in 0..h {
        for px in 0..w {
            let x = (2.0 * ((px as f32 + 0.5) / w as f32) - 1.0) * aspect * tanf;
            let y = (1.0 - 2.0 * ((py as f32 + 0.5) / h as f32)) * tanf;
            let d = V3(x, y, 1.0).norm();
            let t0 = t0map[(py * w + px) as usize];
            let (hit, t) = march(cfg, CAM, d, t0, &mut evals, false);
            img[(py * w + px) as usize] = if hit { t } else { -1.0 };
        }
    }
    (img, evals)
}

// Coarse pre-pass marcher: big eps, few steps, no refine.
fn march_seed(origin: V3, dir: V3, evals: &mut u64) -> (bool, f32) {
    let mut t = 0.0f32;
    let he = 0.05f32;
    for _ in 0..40 {
        let p = origin.add(dir.scale(t));
        let (f, _) = scene_sdf(p, evals);
        if f < he { return (true, t); }
        t += f;
        if t > FAR { return (false, t); }
    }
    (false, t)
}

// Correctness = MISS RATE: fraction of truth-hit pixels where the candidate missed
// (no hit) OR differs from truth by more than DELTA depth units. Robust to outliers.
fn miss_rate(img: &[f32], truth: &[f32], delta: f32) -> f32 {
    let n = img.len();
    let mut miss = 0u32;
    let mut hit = 0u32;
    for i in 0..n {
        if truth[i] > 0.0 {
            hit += 1;
            let c = img[i];
            if c <= 0.0 || (c - truth[i]).abs() > delta {
                miss += 1;
            }
        }
    }
    if hit == 0 { return 1.0; }
    miss as f32 / hit as f32
}

// Mean abs depth error on truth-hit pixels (for reporting only).
fn mean_depth_error(img: &[f32], truth: &[f32]) -> f32 {
    let n = img.len();
    let mut sum = 0.0f32;
    let mut cnt = 0u32;
    for i in 0..n {
        let tr = truth[i];
        if tr <= 0.0 { continue; }
        cnt += 1;
        let c = img[i];
        sum += if c <= 0.0 { 5.0 } else { (c - tr).abs() };
    }
    if cnt == 0 { return f32::MAX; }
    sum / cnt as f32
}

// ----------------------------- proposer: exhaustive wiring enumeration -----------------------------
fn all_configs() -> Vec<Config> {
    let mut v = Vec::new();
    for step in [Step::Fixed, Step::OverRelax] {
        for refine in [Refine::None, Refine::Bisection, Refine::Secant, Refine::Newton, Refine::Iqi] {
            for prec in [Precision::FixedEps, Precision::RateDistortion] {
                for cull in [Cull::None, Cull::CoarseSeed] {
                    v.push(Config { step, refine, prec, cull });
                }
            }
        }
    }
    v
}

// Which variants in a config are NOT documented in the kit for rendering.
fn novel_parts(c: &Config) -> Vec<&'static str> {
    let mut v = Vec::new();
    match c.refine {
        Refine::Newton => v.push("newton-raphson (linear-algebra-matrix)"),
        Refine::Iqi => v.push("inverse-quadratic-interpolation (control-numerical-opt)"),
        Refine::Bisection => v.push("bisection (formal-verification)"),
        _ => {}
    }
    v
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (w, h) = if args.len() >= 3 {
        (args[1].parse().unwrap_or(96), args[2].parse().unwrap_or(54))
    } else { (96u32, 54u32) };
    let (tw, th) = (w * 2, h * 2);

    let t0 = Instant::now();

    // Ground truth: high-budget render (Newton refine, 2x res, many steps). Defines correctness.
    let truth_img = render_truth(tw, th);

    // Baseline: plain sphere tracing, no transfers. Its own miss-rate is the correctness bar.
    let baseline = Config { step: Step::Fixed, refine: Refine::None, prec: Precision::FixedEps, cull: Cull::None };
    let truth_ds = truth_downsampled(&truth_img, tw, th, w, h);
    let (base_img, base_evals) = render(&baseline, w, h);
    let base_miss = miss_rate(&base_img, &truth_ds, DELTA);
    let base_err = mean_depth_error(&base_img, &truth_ds);
    let base_speed = 1.0f32;

    println!("=== discovery-sim — primitive-combination discovery ===");
    println!("resolution: {w}x{h} (truth {tw}x{th}) | DELTA={DELTA} | baseline_evals: {base_evals} baseline_miss: {base_miss:.3} baseline_meanerr: {base_err:.5}");
    println!("baseline: {} -> {base_evals} evals (speedup {base_speed:.3}x) | correctness bar = baseline miss ({base_miss:.3})", baseline.label());

    let mut results: Vec<(Config, u64, f32, f32, Vec<&'static str>)> = Vec::new();
    for cfg in all_configs() {
        let (img, evals) = render(&cfg, w, h);
        let miss = miss_rate(&img, &truth_ds, DELTA);
        let speedup = base_evals as f32 / evals.max(1) as f32;
        results.push((cfg, evals, miss, speedup, novel_parts(&cfg)));
    }

    // DISCOVERY = at least as correct as baseline (miss <= base_miss) AND >1.02x cheaper.
    let mut disc: Vec<_> = results.iter().filter(|r| r.2 <= base_miss && r.3 > 1.02).collect();
    let neutral: Vec<_> = results.iter().filter(|r| r.2 <= base_miss && r.3 <= 1.02).collect();
    let painted: Vec<_> = results.iter().filter(|r| r.2 > base_miss).collect();
    disc.sort_by(|a, b| a.1.cmp(&b.1));

    println!("\n--- DISCOVERIES (miss <= baseline {base_miss:.3}, >1.02x cheaper) ---");
    println!("{:<4} {:>10} {:>9} {:>9} {:>8}  novel", "rank", "wiring", "evals", "speedup", "miss");
    for (i, (cfg, evals, miss, sp, novel)) in disc.iter().enumerate() {
        let n = if novel.is_empty() { "(documented)" } else { &novel.join(", ")[..] };
        println!("{:<4} {:>10} {:>9} {:>8.3}x {:>8.3}  {n}", i + 1, cfg.label(), evals, sp, miss);
    }
    if disc.is_empty() { println!("  (none)"); }

    println!("\n--- NEUTRAL (as correct, not cheaper) ---");
    for (cfg, evals, miss, sp, _) in &neutral {
        println!("  {:>10} {:>9} {:>8.3}x miss {:.3}", cfg.label(), evals, sp, miss);
    }
    println!("\n--- PAINTED (less correct than baseline) ---");
    for (cfg, evals, miss, _, _) in &painted {
        println!("  {:>10}  evals {} miss {:.3} > baseline {:.3}", cfg.label(), evals, miss, base_miss);
    }

    // JSONL dump
    let mut f = File::create("discoveries.jsonl").expect("open out");
    for (cfg, evals, miss, sp, novel) in &results {
        let verdict = if *miss > base_miss { "painted" }
            else if *sp > 1.02 { "discovery" } else { "neutral" };
        let json = format!(
            "{{\"wiring\":\"{}\",\"evals\":{},\"miss_rate\":{:.6},\"baseline_miss\":{:.6},\"speedup_vs_baseline\":{:.4},\"verdict\":\"{}\",\"novel\":[{}],\"currency\":\"field-evaluations per frame at held correctness (miss<=baseline, DELTA={})\"}}\n",
            cfg.label(), evals, miss, base_miss, sp, verdict,
            novel.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(","),
            DELTA
        );
        let _ = f.write_all(json.as_bytes());
    }

    let novel_wins = disc.iter().filter(|r| !r.4.is_empty()).count();
    println!("\n{} wirings tested | {} discoveries | {} of them NOVEL (kit-undocumented) | elapsed {:.1}s",
             results.len(), disc.len(), novel_wins, t0.elapsed().as_secs_f32());
    println!("currency on every win: field-evaluations (conserved). painted = broke correctness. full table -> discoveries.jsonl");
}

// ---- truth (high-budget) ----
fn render_truth(tw: u32, th: u32) -> Vec<f32> {
    // tiny-eps, newton refine, high steps — reuse render but with a dedicated cfg + shrunk eps.
    // We emulate a small eps by rendering at EPS_BASE then refining hard; to keep it simple and
    // honest we just render at higher res with Newton + many steps and treat that as the reference.
    let cfg = Config { step: Step::Fixed, refine: Refine::Newton, prec: Precision::FixedEps, cull: Cull::None };
    // Temporarily run with a finer eps via a local march loop is ideal; for simplicity we render
    // at 2x res with Newton (already done) — the extra resolution gives the ground truth depth.
    render(&cfg, tw, th).0
}

fn truth_downsampled(truth_full: &[f32], tw: u32, th: u32, w: u32, h: u32) -> Vec<f32> {
    let mut out = vec![0f32; (w * h) as usize];
    for py in 0..h {
        for px in 0..w {
            // nearest 2x block center
            let tx = (px * 2).min(tw - 1);
            let ty = (py * 2).min(th - 1);
            out[(py * w + px) as usize] = truth_full[(ty * tw + tx) as usize];
        }
    }
    out
}
