"""Assemble discover-tpu.ipynb from the validated discover-tpu.py engine + narrative cells.
Reuses the engine code verbatim (extracted between the module-level `import jax` and `if __name__`)
so the published notebook can't drift from what was smoke-tested."""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "kaggle-engine-mix-tpu", "discover-tpu.py")
OUT = os.path.join(HERE, "kaggle-engine-mix-tpu", "discover-tpu.ipynb")

src = open(SRC, encoding="utf-8").read()
start = src.find("\nimport jax\n") + 1
end = src.find("if __name__")
engine = src[start:end].strip()

RUN_SEARCH = r'''
# The evolutionary search, as a reusable function (demo + full run both call it).
def run_search(minutes=0.5, pop=64, ncam=4, res=(32, 18), steps=96, L=10, seed=20260701, verbose=True):
    w, h = res
    devs = jax.devices(); ncore = len(devs); plat = devs[0].platform
    eye_np, dir_np = make_rays(ncam, w, h)
    eye = jnp.asarray(eye_np); dir = jnp.asarray(dir_np); N = dir.shape[0]
    P = max(ncore, (pop // ncore) * ncore); ELITE = max(8, P // 16); TOL = 0.012; REF_STEPS = steps * 2
    _, _, _, _, base_ev = jax.jit(lambda: march_plain(eye, dir, steps))()
    _, _, ref_hit, ref_t, _ = jax.jit(lambda: march_plain(eye, dir, REF_STEPS))()
    ref_hit = ref_hit[0]; ref_t = ref_t[0]; base = int(np.asarray(base_ev)[0])

    def evaluate(g, eye, dir, ref_hit, ref_t):
        hit, t_hit, evals = march(g, eye, dir, steps, L)
        sil = jnp.mean((hit != ref_hit[None]).astype(jnp.float32), axis=1)
        both = hit & ref_hit[None]
        depth = (jnp.sum(jnp.where(both, jnp.abs(t_hit - ref_t[None]) / (ref_t[None] + 1e-3), 0.0), axis=1)
                 / jnp.maximum(jnp.sum(both.astype(jnp.float32), axis=1), 1.0))
        redux = (evals.astype(jnp.float32) - base) / base * 100.0
        return redux, sil + 0.3 * depth

    peval = jax.pmap(evaluate, in_axes=(0, None, None, None, None))

    def eval_pop(g):
        chunk = tuple(jnp.asarray(x.reshape((ncore, P // ncore) + x.shape[1:])) for x in g)
        rr, ee = peval(chunk, eye, dir, ref_hit, ref_t)
        return np.asarray(rr).reshape(P), np.asarray(ee).reshape(P)

    rng = np.random.default_rng(seed)
    OP = rng.integers(0, K, (P, L)).astype(np.int32); A = rng.integers(0, R, (P, L)).astype(np.int32)
    B = rng.integers(0, R, (P, L)).astype(np.int32); DST = rng.integers(0, R, (P, L)).astype(np.int32)
    CN = (rng.standard_normal((P, C)) * 1.5).astype(np.float32)

    def mut_int(arr, hi, p, parents):
        c = arr[parents].copy(); m = rng.random(c.shape) < p
        c[m] = rng.integers(0, hi, int(m.sum())); return c.astype(np.int32)

    print(f"device = {ncore}x {plat.upper()} | {N} rays | pop={P} L={L} | naive-baseline {base} field-evals")
    best = None; history = []; t0 = time.time()
    while time.time() - t0 < minutes * 60:
        redux, err = eval_pop((OP, A, B, DST, CN)); ok = err <= TOL
        score = np.where(ok, redux, 1e6); order = np.argsort(score); bi = int(order[0])
        if ok[bi] and (best is None or redux[bi] < best[0]):
            g1 = (OP[bi].copy(), A[bi].copy(), B[bi].copy(), DST[bi].copy(), CN[bi].copy())
            best = (float(redux[bi]), float(err[bi]), g1); f = decode(*g1, L)
            history.append((float(redux[bi]), f))
            if verbose:
                print(f"  [{time.time()-t0:5.0f}s] DISCOVERED {redux[bi]:+5.1f}%  (err {err[bi]:.4f})   step = clip( {f} , 0, 20 )")
        elite = order[:ELITE]; parents = elite[rng.integers(0, ELITE, P)]
        OP = mut_int(OP, K, 0.18, parents); A = mut_int(A, R, 0.14, parents)
        B = mut_int(B, R, 0.14, parents); DST = mut_int(DST, R, 0.14, parents)
        CN = (CN[parents] + 0.15 * rng.standard_normal((P, C))).astype(np.float32)
        imm = rng.random(P) < 0.06
        if imm.any():
            m = int(imm.sum())
            OP[imm] = rng.integers(0, K, (m, L)); A[imm] = rng.integers(0, R, (m, L))
            B[imm] = rng.integers(0, R, (m, L)); DST[imm] = rng.integers(0, R, (m, L))
            CN[imm] = (rng.standard_normal((m, C)) * 1.5).astype(np.float32)
    if best:
        print(f"\nBEST DISCOVERED: {best[0]:+.1f}% field-evals vs naive (err {best[1]:.4f})")
        print(f"   step = clip( {decode(*best[2], L)} , 0, 20 )")
    return best, history
'''.strip()

DEMO = r'''
# A short demo: watch it climb from random programs. On a Kaggle TPU this uses all 8 cores; on CPU, 1.
# Increase `minutes` / `pop` / `res` for a real search (see the "full experiment" cell below).
best, history = run_search(minutes=0.5, pop=64, ncam=4, res=(32, 18), steps=96, L=10)
'''.strip()

BASE_IMPORTS = "import os, time, urllib.request\nimport numpy as np"

SETUP = r'''
# On a Kaggle TPU notebook JAX + the 8 TPU cores are normally ready out of the box. If the line below
# does NOT list 8 TPU devices, set Settings -> Accelerator -> TPU, or uncomment the install:
# !pip install -q -U "jax[tpu]" -f https://storage.googleapis.com/libtpu-releases/index.html
import jax
print("JAX", jax.__version__, "| devices:", jax.devices())
'''.strip()


def md(s):
    return {"cell_type": "markdown", "metadata": {}, "source": s.strip("\n")}


def code(s):
    return {"cell_type": "code", "metadata": {}, "execution_count": None, "outputs": [], "source": s.strip("\n")}


cells = [
    md(r"""
# Discovering Raymarching Optimizations From Scratch
## A genetic-programming "step-function" search on TPU — a *discovery* engine, not a *confirmation* engine

Signed-distance-field **sphere tracing** renders 3-D scenes by stepping a ray forward by the distance to
the nearest surface. Decades of graphics research hand-derived *faster* stepping rules — **over-relaxation**
(Keinert 2014), **secant** root refinement, **screen-footprint LOD**, and a numerical-cognition-inspired
**subitize** leap (our prior work, shipped into a real Rust renderer).

**This notebook asks: can a machine rediscover these — and maybe find new ones — given nothing but
primitive arithmetic atoms and a rendering-cost metric?**

We evolve the marcher's per-step *step-length function* from scratch with **linear genetic programming**,
scored on the real rendering currency (SDF evaluations per frame at held image quality), running a large
population of candidate *programs* in parallel on a **Kaggle TPU v5e-8 (8 cores) via JAX**.

> **Result preview (validated on CPU below).** Starting from *random programs, with no optimization
> pre-loaded*, the search independently rediscovered **over-relaxation** — `step ≈ 1.70·d`, i.e. Keinert's
> enhanced sphere tracing — and, on a different seed, an **overlap-signal-driven** rule. The winner is
> decoded to a human-readable formula, so a discovery is *shippable*.
"""),
    md(r"""
## 1. Background: sphere tracing and the currency

A scene is a function `d = sdf(p)` = signed distance to the nearest surface. `d` is a **safe radius**: no
surface is closer, so a ray may advance by `d` and never overshoot. Repeat until `d < eps` (hit).

```
t = 0
loop:
    d = sdf(o + t*r)
    if d < eps: return HIT
    t += d                 # <-- the "step function". Naive rule: step = d.
```

The `sdf` call dominates render cost, so **field-evals per frame is the currency we minimize.** The naive
`step = d` is safe but slow on grazing rays. Faster hand-derived rules (all keeping the hit correct):

| Optimization | Step rule (sketch) |
|---|---|
| **Over-relaxation** (Keinert 2014) | `step = ω·d`, ω≈1.4, undo if successive spheres don't overlap |
| **Secant** | near the surface, extrapolate the root from the last two samples |
| **LOD** | widen `eps` with distance |
| **Subitize** (ours, shipped) | `step = d·(1 + k·[d > 6·eps])` — "leap when clearly far" |
"""),
    md(r"""
## 2. Confirmation vs discovery — the whole point

Our earlier searches evolve the **parameters of a fixed genome** of knobs
`(ω, lod, secant, subitize, momentum, glue, symbols, …)`. Because the genome *already contains* subitize
and over-relaxation, such a search can only ever **re-find** them. It is a **confirmation engine**.

We showed this directly: a 2-hour search (89,856 configs) that *added* new "glue" and "symbols" operators
**still** landed on subitize as the only driver —

```
BEST: -39.9%   (subitize=0.730, glue=0.092, sym2=0.164, ...)
ablation — contribution when each operator is removed:
    subitize      : -11.6%   <- the only real driver
    stoch-omega   :  -1.4%
    2nd-order     :  -0.3%
    regime-blend  :  +0.0%   <- "glue" contributed nothing
```

Adding operators to a confirmation genome doesn't make it discover. To **discover**, the search must
compose mechanisms it was never given — i.e. search over **programs**, not parameters. That is this
notebook.
"""),
    md(r"""
## 3. Method: evolving the step function as a program

Each individual is a small **linear genetic program**: `L` instructions `(op, a, b, dst)` over `R`
registers, plus a few evolved constants.

- **Registers** are seeded each march step with the raw signals: `d`, `d_prev`, `eps`,
  `overlap = radius + prev_radius − last_step`, `approach`, `curvature`, `t`, step-index, `1`, `radius`,
  and evolved constants `c0..c3`.
- After `L` instructions the **output register (`reg0`) is the step length**. Since `reg0` seeds to `d`,
  **a no-op program is exactly naive sphere tracing** — the search starts at the baseline and earns every gain.

**The op-set is the root atoms the cross-domain primitive library is built from:**
`add sub mul safe-div min max abs neg sqrt sin hash gt(indicator) glue(cond) half`. Atoms = primitives,
the operand/dst wiring = **glue**, the register discipline + safe-distance invariant = **symbols**.

**Subitize is *not* privileged** — it's just one reachable program (`gt(d, c·eps) → mul`) in an
astronomical space. The search must invent it, beat it, or find something else.

**The currency IS the safety mechanism.** Instead of a hand-coded overshoot guard (which would bias the
search toward the human answer), fitness = field-eval reduction **gated by image error**
(`silhouette + 0.3·depth ≤ 0.012` vs a high-step ground truth). A program that tunnels produces wrong
pixels, fails the gate, and dies. The only priors are numerical (finite, non-negative, bounded step).

**Search:** elitist evolution strategy — select the top programs, mutate instruction slots + constants,
inject ~6% random immigrants each generation to keep the search open-ended. **The winner is decoded to a
readable formula**, so a discovery is shippable (as `subitize` was, into the real Rust renderer).
"""),
    md(r"""
## 4. Implementation: JAX on TPU

A TPU is a native-XLA device, so JAX compiles the whole batched marcher into one XLA program and shards
the population across the 8 cores (`pmap`). The march is a fixed-length `lax.fori_loop` (no data-dependent
break — rays are **masked** as they hit, keeping the batch vectorized). The per-step interpreter evaluates
`P` different programs over `N` rays with `take_along_axis` gathers + one-hot writes, in float32
(TPU's bf16 default only affects matmul/conv, which this kernel avoids).

The code below is the exact engine (also runnable as `discover-tpu.py`). It runs unchanged on a single
CPU device, which is how the logic was validated before any TPU run.
"""),
    code(SETUP),
    code(BASE_IMPORTS),
    code("# === engine (verbatim from discover-tpu.py) ===\n" + engine),
    code(RUN_SEARCH),
    md(r"""
## 5. Watch it discover (live)

Run the cell below. From **random programs**, with nothing pre-loaded, it climbs — printing each new best
as a decoded step formula. Look for it converging toward `step ≈ (something)·d` (over-relaxation) or an
`overlap`/`curvature`-driven rule. This short demo is deliberately tiny; the real magnitudes come from the
full run.
"""),
    code(DEMO),
    md(r"""
## 6. The full experiment (Kaggle TPU, ~7.5 h)

For a real search, set the accelerator to **TPU** (Settings → Accelerator, or `enable_tpu: true` when
pushing via the API) and run:

```python
best, history = run_search(minutes=450, pop=256, ncam=6, res=(80, 45), steps=128, L=10)
```

Defaults: population 256, program length 10, 14 registers, 14 ops, 4 evolved constants; 6 cameras at
80×45 (~21k rays); 128 march steps; error gate 0.012; 7.5 h (under Kaggle's ~9 h TPU-session ceiling).
The first line printed reports the device — confirm it reads `8x TPU`.
"""),
    md(r"""
## 7. Results

*Populated from the full TPU run.* We report the best decoded formula, its field-eval reduction and image
error, the diversity of high-fitness mechanisms across seeds, and a comparison against the hand-derived
optimizations (over-relaxation, secant, subitize). Any discovery that beats or matches the shipped knobs is
then reimplemented in the real Rust marcher and **wall-clock-tested** — the same validation that shipped
`subitize`.

### Validation runs (CPU, logic check)
From random programs the engine independently produced:
```
Run A (L=8):  step = (radius + i*0.02) / 0.588  ≈  1.70 * d     <- ENHANCED SPHERE TRACING, rediscovered
Run B (L=10): step = overlap + t*0.1                            <- a different mechanism, different seed
```
Rediscovering a hand-derived 2014 optimization from primitive atoms — and finding *different* mechanisms
on different seeds — is the pass condition for a genuine discovery engine.
"""),
    md(r"""
## 8. Limitations & honesty

- **The currency is a proxy.** Field-evals at held image quality is a good cost proxy, but real wall-clock
  includes shading/shadows/AO/caches. Prior work: sim reductions deflate on the real engine (subitize
  −40%→−16..23%; a dual-number lead −45%→−3%). **A discovery is a lead, not a result, until it runs in the
  real renderer.**
- The CPU-smoke numbers (−60%, −52%) are **not** meaningful magnitudes — tiny scene, naive baseline, no
  wall-clock. They validate that the machine *discovers*.
- **Fixed scene** (may overfit geometry; multi-scene generalization is future work), **step-only** (normal
  and shadow strategies are held fixed), and **no overshoot guard by design** (the currency selects safe
  strategies; the `overlap` signal is exposed so guarded strategies remain discoverable).

## 9. Reproducibility & related work

Pure JAX + NumPy, no other deps. Local CPU logic check:
`pip install "jax[cpu]" numpy` → run the demo cell. Population seed fixed; mechanisms reproduce, exact
constants may drift with floating point.

Related: Keinert et al., *Enhanced Sphere Tracing* (2014); Hart, *Sphere Tracing* (1996); linear genetic
programming (Brameier & Banzhaf); symbolic regression (Koza; Schmidt & Lipson). This extends our
cross-domain primitive-discovery program that shipped `subitize` and the dual-number normal into `mm3e`.

## 10. Contribution

A **discovery-vs-confirmation** control experiment; a **tensorized linear-GP interpreter** for the SDF
marcher step function on TPU; **currency-as-safety** (image-gated cost as the sole correctness pressure);
and empirical **rediscovery of enhanced sphere tracing from scratch**, decoded to shippable formulas.
"""),
]

nb = {
    "cells": cells,
    "metadata": {
        "kernelspec": {"display_name": "Python 3", "language": "python", "name": "python3"},
        "language_info": {"name": "python"},
        "accelerator": "TPU",
    },
    "nbformat": 4,
    "nbformat_minor": 5,
}

with open(OUT, "w", encoding="utf-8") as f:
    json.dump(nb, f, indent=1)

# validate it loads + report structure
with open(OUT, encoding="utf-8") as f:
    back = json.load(f)
nmd = sum(1 for c in back["cells"] if c["cell_type"] == "markdown")
ncode = sum(1 for c in back["cells"] if c["cell_type"] == "code")
print(f"wrote {OUT}")
print(f"cells: {len(back['cells'])} ({nmd} markdown, {ncode} code)")
