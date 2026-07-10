# Discovering Raymarching Optimizations from Scratch: A Genetic-Programming "Step-Function" Search on TPU

**A discovery engine, not a confirmation engine.**

---

## Abstract

Signed-distance-field (SDF) *sphere tracing* renders 3-D scenes by walking a ray forward in steps
equal to the distance to the nearest surface. Decades of graphics research have hand-derived faster
stepping rules — **over-relaxation** (Keinert et al. 2014, "enhanced sphere tracing"), **secant /
regula-falsi** root refinement, **screen-footprint LOD**, and (in our own prior work) a
numerical-cognition-inspired **subitize** leap. Each was a human insight.

We ask: *can a machine rediscover these — and maybe find new ones — given nothing but primitive
arithmetic atoms and a rendering-cost metric?* We built a **tensorized linear-genetic-programming**
search that evolves the marcher's per-step "step-length function" from scratch, scored on the real
rendering currency (SDF evaluations per frame at held image quality). It runs on a **Kaggle TPU v5e-8
(8 cores) via JAX**, evaluating a large population of candidate programs in parallel.

Validated on CPU before any TPU run, the engine — **starting from random programs, with no optimization
pre-loaded** — independently rediscovered **over-relaxation** (`step ≈ 1.70·d`, i.e. enhanced sphere
tracing) and, on a separate seed, an **overlap-signal-driven** stepping rule. The winning program is
decoded to a human-readable formula, so a discovery is *shippable* — as our prior `subitize` finding
was, into the real Rust renderer.

The control condition matters: a *confirmation* search that instead tunes the parameters of a genome
already containing the known operators simply re-finds them (subitize), and added operators contribute
~0%. Handing a search the answer guarantees it returns the answer. Withholding it is what makes the
result a discovery.

---

## 1. Background

### 1.1 SDF sphere tracing

A scene is defined by a function `d = sdf(p)` returning the signed distance from a point `p` to the
nearest surface. To find where a ray `o + t·r` first hits the surface, sphere tracing exploits the fact
that `d` is a *safe radius*: no surface is closer than `d`, so we may advance `t` by `d` and never
overshoot. Repeat until `d < eps` (hit) or `t` exceeds the far plane (miss).

```
t = 0
loop:
    d = sdf(o + t*r)
    if d < eps: return HIT at t
    t += d                       # the "step function": step = d
```

The inner `sdf` evaluation is by far the dominant cost of a raymarched renderer. **The number of `sdf`
calls per frame is the currency we optimize.** The naïve step `step = d` is provably safe but slow: on
grazing rays that skim near a surface, `d` stays small and the march crawls.

### 1.2 Hand-derived faster step functions

Graphics researchers improved on `step = d` with cleverer rules, all of which keep the hit correct
while taking bigger steps through empty space:

| Optimization | Step rule (sketch) | Insight |
|---|---|---|
| **Over-relaxation** (Keinert 2014) | `step = ω·d`, ω≈1.4, undo if two successive unbounding spheres fail to overlap | over-step, with a cheap overshoot guard |
| **Secant / regula-falsi** | near the surface, extrapolate the root from the last two samples | 2nd-order convergence near hits |
| **Screen-footprint LOD** | widen `eps` with distance | sub-pixel geometry needn't resolve exactly |
| **Subitize** (our prior work) | `step = d·(1 + k·[d > 6·eps])` | "leap when it's *clearly* far" — the Approximate Number System |

Our engine (`mm3e`, a dependency-free Rust SDF renderer) ships several of these as dial-able knobs. In
particular, `Marcher::subitize` was discovered by a cross-domain search, **validated on the real
renderer** (−16…23 % field-evals and wall-clock at ~1 % depth error), and shipped.

### 1.3 The cross-domain discovery program

`subitize` came out of a broader program: mixing "primitives" from many domains (numerical cognition,
quantum walks, queueing, signal processing, …) into the renderer and measuring what transfers, charging
a conserved currency rather than merely type-checking. That program produced two *real* engine wins so
far — the dual-number analytic normal (−3 %, free) and subitize (−16…23 %, a knob).

But those searches share a limitation this experiment is designed to expose and escape.

---

## 2. Motivation: confirmation vs discovery

The earlier searches evolve the **parameters of a fixed genome**: a vector of knobs
`(ω, lod, secant, subitize, momentum, stochastic-ω, glue, symbols, …)`. The search tunes those scalars
against the currency. Because the genome *already contains* subitize and over-relaxation, the search
can only ever **re-find** them. It is a **confirmation engine**.

We demonstrated this directly. A 2-hour run over the full knob genome (89,856 configs) that *added*
new "glue" and "symbols" operators still landed on subitize as the sole driver:

```
BEST: -39.9%  (omega=1.00, subitize=0.730, stoch=0.500, glue=0.092, sym2=0.164, ...)
per-domain ablation (contribution when removed):
    subitize      : worth  -11.6%   <- the only real driver
    stoch-omega   : worth  -1.4%
    momentum      : worth  -0.3%
    2nd-order     : worth  -0.3%
    binary-refine : worth  +0.0%
    regime-blend  : worth  +0.0%    <- "glue" contributed nothing
```

Adding operators to a confirmation genome does not make it discover. To *discover*, the search must be
able to **compose mechanisms it was never given** — which means searching over **programs**, not
parameters.

---

## 3. Method: evolving the step function as a program

### 3.1 The search space

Each individual is a small **program** that computes the step length from the signals available at the
current march step. Concretely, a **linear genetic program**: a sequence of `L` instructions
`(op, a, b, dst)` operating on a bank of `R` registers, plus a handful of evolved floating-point
constants.

- **Registers** are seeded, every march step, with the raw signals:
  `d` (safe distance), `d_prev`, `eps`, `overlap = radius + prev_radius − last_step`,
  `approach = d_prev − d`, `curvature = d − 2·d_prev + d_prev2`, `t`, step-index `i`, the constant `1`,
  `radius = |d|`, and the evolved constants `c0..c3`.
- **Instructions** read two register operands, apply an op, and write the result to a destination
  register. After `L` instructions the **output register (`reg0`) is the step length**.
- Because `reg0` is *seeded to `d`*, **a program that does nothing is exactly naïve sphere tracing.**
  The search therefore starts at the naïve baseline and must *earn* every improvement.

### 3.2 The op-set = the primitive library's root atoms

The instruction op-set is the set of **root atoms** that the cross-domain primitive library is built
from — the same atoms in the framework's "atoms × glue × symbols" decomposition:

```
add  sub  mul  safe-div  min  max  abs  neg  sqrt  sin  hash  gt(indicator)  glue(cond)  half
```

- `hash` is the crypto-hashing atom, `frac(sin(a·k₁ + b·k₂)·k₃)` — the same blue-noise primitive our
  renderer uses for stochastic soft shadows.
- `gt` is the compare→indicator atom (`a > b ? 1 : 0`) — the building block of a subitize-style
  threshold.
- `glue` is a conditional (`a > 0 ? a : b`) — data-dependent routing.

In the framework's terms: **atoms = these ops (primitives), the operand/destination wiring = glue, the
register discipline and the safe-distance invariant = symbols.** The search explores *compositions* of
primitives under glue and symbol constraints — which is exactly "let it find what it can, randomly."

Crucially, **subitize is not privileged.** It is *one reachable program* —
`gt(d, mul(c, eps)) → mul` with an evolved `c` — out of an astronomical space (`(K·R·R·R)^L` structural
programs times a continuum of constants). The search must invent it, beat it, or find something else.

### 3.3 The currency *is* the safety mechanism

Faster stepping is worthless if it tunnels through surfaces. Rather than bolt on a hand-designed
overshoot guard (which would bias the search toward the human solution), **we let the cost metric
enforce correctness**:

- **Fitness** = field-evals reduction vs the naïve baseline, **gated** by image error. Each program is
  rendered over a fixed multi-camera set; we compare its hit mask (silhouette) and hit depths against a
  high-step ground-truth sphere trace. A program is only *eligible* if
  `silhouette_error + 0.3·depth_error ≤ 0.012`.
- A program that over-steps and tunnels produces wrong silhouettes/depths, **fails the gate, and dies.**
- The only priors are numerical: the step is clamped finite, non-negative, and bounded (to prevent
  `inf`/`NaN` runaway), and a hit is registered when `d < eps`.

This is the key design choice that makes it a *discovery* engine: safety emerges from the currency, so
the search is free to invent safe-aggressive strategies (like subitize's "leap only when far", which is
inherently safe *without* a guard) that a guard-biased search might never explore.

### 3.4 Evolutionary search

A simple, robust evolution strategy over the population:

1. Evaluate all `P` programs → `(redux, error)`.
2. Gate: eligible if `error ≤ 0.012`; ineligible programs sort last.
3. Select the top-`ELITE` by reduction (more negative = better).
4. Produce offspring: copy an elite parent; **mutate** each instruction slot with probability ~0.15
   (resample its op / operands / destination) and perturb the constants with Gaussian noise.
5. Inject ~6 % **random immigrants** each generation to resist premature convergence — important for
   *discovery*, where the tail of the search space matters.

### 3.5 Interpretability: decode the winner

Unlike neuroevolution (which would yield an opaque weight vector), a linear GP yields a **readable
program**. A host-side decoder traces the instructions into an algebraic formula, e.g.
`step = clip( (radius / 0.588), 0, 20 )`. This is what makes a discovery *shippable*: we can read the
mechanism, reimplement it in the real Rust marcher, and wall-clock-test it — exactly the pipeline that
shipped `subitize`.

---

## 4. Implementation: JAX on TPU

### 4.1 Why JAX (not PyTorch/XLA)

A TPU is a native-XLA device. JAX compiles the entire batched marcher into a single XLA program and
shards it across the 8 cores with near-zero friction. The alternative confirmation sims run on PyTorch
(Intel Arc via XPU, NVIDIA via CUDA); for the TPU we re-expressed the marcher in JAX, porting the
operator math line-for-line to guard against drift.

### 4.2 The batched interpreter

The performance-critical piece is evaluating `P` *different* programs over `N` rays in parallel. We
tensorize the interpreter:

- Registers are a `(P, N, R)` float32 array.
- For each of the `L` instructions: gather the two operand registers per program
  (`jnp.take_along_axis` over the register axis), compute **all `K` candidate ops** on the operands
  (branchless), select each program's op (a second gather over the op axis), and write the result to
  the destination register via a one-hot mask.
- This is dense arithmetic + gathers — well suited to TPU.

### 4.3 The march

The outer march is a `lax.fori_loop` of fixed length (no data-dependent break — rays are **masked** as
they hit, which is what keeps the whole batch vectorized under XLA). Field-evals are counted per config
as the number of still-active rays each step. The population is sharded across cores with `jax.pmap`
(embarrassingly parallel over programs, zero cross-ray communication).

### 4.4 Precision

Sphere tracing needs true float32 (`eps ≈ 6e-4`). JAX elementwise ops run in the array dtype (the
bf16-ish TPU default applies only to matmul/conv, which this kernel does not use), so float32 arrays
stay float32. We pin `jax_default_matmul_precision='float32'` defensively and do **not** enable x64.

### 4.5 Robustness for an unattended cloud run

- **Self-healing TPU bootstrap.** JAX may or may not be preinstalled on Kaggle's TPU image; the script
  checks for a visible TPU and, only if absent *and only on Kaggle*, `pip install`s `jax[tpu]` and
  re-execs once. It never disturbs a working environment and never fires during local CPU testing.
- **Live streaming.** Every improved program (decoded formula), plus periodic heartbeats and the final
  result, is POSTed to a webhook — so a discovery is captured even if the kernel's own logs scroll.

---

## 5. Validation (before any TPU run)

We installed `jax[cpu]` locally and smoke-tested the full pipeline at tiny scale (a few thousand rays,
a few dozen generations, seconds of compute). This validates the *logic*, not the numbers. From random
programs, the engine climbed and produced readable discoveries:

**Run A** (L=8): converged toward **over-relaxation**.
```
[g0]  step = sqrt(radius)                       -26.7%   (an oddball valid stepper)
[g8]  step = radius + 0.227                      -45.0%
[g30] step = (radius + i·0.02) / 0.734           -52.1%
[g41] step = (radius + i·0.02) / 0.588  ≈ 1.70·d -60.1%   <- ENHANCED SPHERE TRACING, rediscovered
```

**Run B** (L=10, different seed): converged toward a **different** mechanism.
```
[g1]  step = d + i·0.02                           -40.7%
[g19] step = overlap + t·0.1                       -51.8%   <- overlap-signal-driven stepping
```

Two things stand out:
1. It **rediscovered over-relaxation** — a hand-derived 2014 result — from primitive atoms, with no
   knowledge that it exists. That is the pass condition for a discovery engine.
2. Different seeds found **different** mechanisms, i.e. the search is genuinely open-ended, not a funnel
   to a single answer.

> **Honesty note.** These CPU-smoke numbers (−60 %, −52 %) are *not* meaningful magnitudes — the scene
> is tiny, the baseline is naïve sphere tracing (not the already-optimized shipped renderer), and there
> is no wall-clock. They validate that the machine *discovers*. The real magnitudes come from the full
> TPU run, and — as our `subitize` and LOD work repeatedly showed — **sim magnitudes oversell; only a
> test on the real Rust renderer is believable.**

---

## 6. The full experiment (TPU)

Configuration (defaults): population 256, program length 10, 14 registers, 14 ops, 4 evolved constants;
6 cameras at 80×45 (~21k rays); 128 march steps; naïve-sphere-trace baseline; error gate 0.012; 7.5 h
budget (under Kaggle's ~9 h TPU-session ceiling). The kernel self-reports `programs/s` so throughput is
visible early and tunable.

**Results: _to be populated from the TPU run._** We will report: the best discovered formula (decoded),
its field-eval reduction and image error, the diversity of high-fitness mechanisms found, and a
comparison against the hand-derived optimizations (over-relaxation, secant, subitize). Any discovery
that beats or matches the shipped knobs will then be reimplemented in the real Rust marcher and
wall-clock-tested — the same validation that shipped `subitize`.

---

## 7. Limitations & honesty

- **The currency is a proxy.** Field-evals at held silhouette/depth is a good proxy for cost, but the
  real renderer's wall-clock includes shading, shadows, AO, and cache effects. Prior work showed sim
  reductions deflate substantially on the real engine (subitize: sim −40 % → real −16…23 %; a
  dual-number lead: sim −45 % → real −3 %). **A discovery is a *lead*, not a result, until it runs in
  the real renderer.**
- **The scene is fixed.** Discoveries may overfit this scene's geometry; generalization needs multiple
  scenes (future work).
- **Step-only.** We evolve the step function, where the known wins live. The normal and shadow
  strategies are held fixed (future work could discover those too).
- **No overshoot guard by design.** This lets the currency select safe strategies, but it also means the
  search cannot discover "aggressive + guard" hybrids unless the guard signal (which we expose as the
  `overlap` register) is used by the program.
- **Reproducibility caveat:** floating-point reductions differ slightly across TPU/CPU; the *mechanisms*
  reproduce, exact constants may not.

---

## 8. Reproducibility

- **Code:** `discover-tpu.py` (script) / `discover-tpu.ipynb` (notebook). Pure JAX + NumPy; no other deps.
- **Local logic check (CPU):**
  `pip install "jax[cpu]" numpy` then
  `DISCOVER_NO_WEBHOOK=1 python discover-tpu.py --pop 32 --minutes 0.3 --res 24x14 --ncam 3 --steps 48 --L 8`
- **Full run (Kaggle TPU):** push with `enable_tpu: true`, `enable_internet: true`; the first log line
  must read `... | 8x TPU | ...`.
- **Seed:** fixed (`20260701`) for the population init; per-run mechanism reproduces, exact constants may
  drift with FP.

---

## 9. Related work

- Keinert, Schäfer, Korndörfer, Ganse, Stamminger, *Enhanced Sphere Tracing* (SCCG 2014) — over-relaxation.
- Hart, *Sphere Tracing* (1996) — the original.
- Genetic programming / linear GP (Banzhaf et al.; Brameier & Banzhaf) — evolving programs as
  instruction sequences.
- Symbolic regression (Koza; Schmidt & Lipson) — recovering interpretable laws from data.
- Our own `subitize` (numerical-cognition transfer) and dual-number-normal findings — the shipped
  results of the cross-domain program this experiment extends.

---

## 10. Contributions

1. A **discovery vs confirmation** framing with a direct control experiment showing a parameter-tuning
   search cannot escape its seeded genome.
2. A **tensorized linear-GP interpreter** for the SDF marcher step function that runs a large program
   population in parallel on TPU via JAX.
3. **Currency-as-safety**: using the rendering cost metric (gated by image error) as the sole
   correctness pressure, so the search discovers safe-aggressive stepping without a hand-coded guard.
4. Empirical **rediscovery of enhanced sphere tracing (over-relaxation) from scratch**, plus divergent
   novel mechanisms across seeds — with a decoder that makes each discovery a shippable formula.

*This document is living: results sections are populated as the TPU run completes, and any discovery
that survives to the real Rust renderer is linked back here.*
