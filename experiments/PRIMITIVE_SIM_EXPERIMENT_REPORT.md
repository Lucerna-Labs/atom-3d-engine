# Primitive Simulation Experiment Report

## Purpose

This document explains the current primitive-discovery experiment family: what it is testing, how the
simulations work, what has already been validated, which primitives actually applied, which did not,
and which findings are worth carrying into other systems such as the spiderweb bus, RF transport,
security, inference scheduling, kernels, false-memory systems, and model exoskeletons.

The central rule is:

```text
keep the mechanism, not the literal constant
```

Constants like `0.588`, `1.70`, `0.810`, or `m=28` are tied to a scene, channel model, or error
budget. The portable thing is the mechanism: confidence-gated jumps, overlap guards, authenticated
erasures, fountain repair, interleaving, adaptive redundancy, and explicit correctness gates.

## Current Progress

The project now has four connected tracks.

| Track | Status | What it proves |
|---|---|---|
| TPU/JAX discovery engine | Built; notebook/writeup exist; local logic validated | Search can synthesize readable step formulas from primitive atoms rather than only tune known knobs. |
| Real Rust renderer validation | Built for TPU signals; shipped `subitize`; dual normals integrated | Sim results are leads until they survive real MM3E field/shadow/AO/correctness checks. |
| Intel Arc engine-mix sims | Re-run locally on Arc A380 with `torch-xpu` | Fixed-genome confirmation repeatedly finds `subitize` as the main renderer survivor. |
| RF/crypto self-repair sims | Built and run locally on Arc A380; Rust RF examples pass | Per-shard auth before repair is the key for self-repair under loss plus corruption. |

Relevant files:

- `experiments/discovery-tpu/discover-tpu.py`
- `experiments/discovery-tpu/CROSS_DOMAIN_TRANSFER_NOTES.md`
- `mm3e-orchestrator/examples/tpu_signals_real.rs`
- `experiments/engine-mix-arc/arc_engine_mix2.py`
- `experiments/engine-mix-arc/RUN_NOTES.md`
- `experiments/rf-crypto-arc/rf_crypto_self_repair.py`
- `experiments/rf-crypto-arc/RUN_NOTES.md`
- `kernel Os-spiderwebBus/crates/spiderweb-rf`
- `kernel Os-spiderwebBus/crates/spiderweb-edge`

## How The Renderer Discovery Works

MM3E renders analytic signed-distance fields by sphere tracing. Each ray advances by a step length,
usually derived from the SDF distance `d`. The goal is to reduce expensive field evaluations without
tunneling through surfaces or changing the image.

The discovery engine treats the marcher as a policy search problem:

```text
signals in -> small formula/program -> proposed step length -> render-cost + correctness gate
```

Signals available to candidate formulas include:

- `d`: current SDF distance
- `d_prev`: previous distance
- `eps`: hit threshold
- `overlap`: how much successive safe regions overlap
- `approach`: whether the ray is moving smoothly toward a surface
- `curv`: local second-order trend
- `t`: current ray distance
- `i`: step index / progress age
- `radius`: local safety margin

The fitness/currency is not speed alone. A candidate only matters if it reduces field evaluations
while keeping hit agreement and depth error inside the quality gate.

This is one of the biggest meta-results of the whole project:

```text
optimization without a correctness currency produces tricks;
optimization with a correctness currency produces mechanisms.
```

## TPU Step Signals

The TPU/JAX discovery path produced several readable step rules. These were then tested in
`mm3e-orchestrator/examples/tpu_signals_real.rs` against the real MM3E scene field, shadows, AO, and
six deterministic cameras.

| Signal | Real-engine result | Verdict |
|---|---|---|
| shipped `omega=1.40` | baseline, `99.96%` hit agreement, `0.00064` depth error | current stable default |
| TPU `omega=1.70` | `+4.8%` total evals, `+11.6%` primary evals | reject for renderer default |
| `(radius+i*.02)/.588` | `-30.5%` total evals, but `0.02691` depth error | transfer lead, not renderer-safe |
| `overlap+t*.1` | `-47.8%` total evals, but `0.03353` depth error | strongest speed signal, too much drift |

The important read is not "these failed." It is more precise:

- They failed the exact-renderer correctness gate.
- They still exposed transferable mechanisms for systems where rollback, repair, or bounded
  approximation is acceptable.

## Intel Arc Engine-Mix Sims

The Arc engine-mix sims are confirmation sims, not pure discovery. They seed a fixed genome of
candidate operators and tune the knobs on the Intel Arc A380 through `torch-xpu`.

Fresh local run:

```powershell
& "C:\Projects\discovery-search-12h-gpu\.venv-arc\Scripts\python.exe" experiments\engine-mix-arc\arc_engine_mix2.py --minutes 5 --pop 128 --res 96x54
```

Runtime:

- `Intel(R) Arc(TM) A380 Graphics`
- `torch 2.12.1+xpu`
- `62208` rays: 12 cameras at `96x54`
- `4224` configs in `302s`
- baseline: `2248192` stock field-evals

Best fresh genome:

```text
-39.1% field-evals
omega=1.00 lod=0.0007 sec=0.150 fq=0 normal=dual
COG mom=0.023 subitize=0.810
QUANT stoch=0.006 brefine=0.278
GLUE glue=0.192
SYM sym2=0.130
```

Per-domain ablation:

| Primitive | Result when removed | Worth |
|---|---:|---:|
| cognitive subitize | `-25.3%` at err `0.0014` | `-13.9%` |
| cognitive momentum | `-38.9%` at err `0.0113` | `-0.2%` |
| stochastic omega | `-39.1%` at err `0.0116` | `-0.0%` |
| binary refine | `-39.1%` at err `0.0116` | `+0.0%` |
| glue regime blend | `-39.1%` at err `0.0116` | `+0.0%` |
| symbolic second-order | `-38.9%` at err `0.0112` | `-0.3%` |

The longer prior Arc run found the same shape:

```text
BEST OVERALL: -39.9%
omega=1.00 lod=0.0009 sec=0.150 fq=0 normal=dual
mom=0.046 subitize=0.730 stoch=0.500 brefine=0.226 glue=0.092 sym2=0.164
```

That run's ablation showed `subitize` was worth `-11.6%`, stochastic omega around `-1.4%`, and
everything else was tiny or scene-dependent.

## Renderer Primitives: Applied vs Not Applied

| Primitive | Applied? | Status |
|---|---|---|
| `subitize` / far-clearance leap | Yes | Shipped as `Marcher::subitize`; real engine docs report roughly `-11%` field-evals at low error up to about `-23%` near the higher error edge. |
| dual-number analytic normal | Yes | Shipped where the scene is dual-safe; avoids repeated finite-difference samples for normals. |
| guarded over-relaxation | Yes, older baseline | `omega=1.40` remains the stable baseline. `omega=1.70` did not improve the real renderer. |
| secant near-surface refinement | Yes, as part of tested marcher variants | Useful around surface convergence; must stay guarded. |
| age-indexed aggression | Not as renderer default | Fast but too much depth drift; keep as transfer lead for schedulers/routing. |
| overlap plus progress | Not as renderer default | Very fast but too much depth drift; strong transfer lead for bus/inference promotion. |
| stochastic omega | Not priority | Long Arc run showed a small contribution; fresh run showed none. Maybe a stabilizer, not a core primitive. |
| binary refine | Not priority | No meaningful contribution in the fresh Arc run. |
| glue regime blend | Not priority | Did not earn its keep in the Arc runs. |
| symbolic second-order / curvature trend | Transfer lead | Small positive contribution; probably a nudge rather than standalone policy. |
| momentum / predictive coding | Transfer lead | Small positive contribution near the quality gate; useful as a trend signal. |

## How The RF/Crypto Self-Repair Experiment Works

The RF experiment is not a renderer test. It asks a transport question:

```text
Can a spiderweb bus lane survive burst loss and corruption without asking the sender to retransmit?
```

There are two layers of evidence:

1. Rust `spiderweb-rf` examples run through the actual bus lane.
2. Arc/XPU Monte Carlo sim compares crypto and repair policies at much larger sample count.

The Rust RF crate models a deterministic fading channel:

- good/bad fade states
- packet loss
- delay/jitter
- duplicate delivery
- deterministic seed so failures replay exactly

The fountain primitive then sends:

```text
k data chunks + m XOR repair shards
```

The receiver solves the surviving equations over GF(2). If enough equations survive, dropped chunks
are reconstructed without retransmission.

Rust sanity check:

| Check | Result |
|---|---|
| RF chaos/fading lane | `393/500` delivered, about `21%` loss, deterministic by seed |
| RF fortify/no repair | `38/48` raw delivered |
| RF fortify/fountain repair | `48/48` recovered with `28` repair shards |

## Arc RF/Crypto Results

The Arc simulation added corruption/tamper on top of burst loss, then compared integrity and repair
strategies.

Command:

```powershell
& "C:\Projects\discovery-search-12h-gpu\.venv-arc\Scripts\python.exe" experiments\rf-crypto-arc\rf_crypto_self_repair.py --trials 131072 --k 48 --m 28 --extra 24
```

Runtime:

- `Intel(R) Arc(TM) A380 Graphics`
- `torch 2.12.1+xpu`
- `131072` simulated RF blocks
- `k=48` data chunks
- default repair `m=28`
- adaptive extra repair `24`

Results:

| Strategy | Tx/data | Clean block | Avg delivered | Silent corrupt |
|---|---:|---:|---:|---:|
| raw/no crypto | `1.00x` | `2.90%` | `87.29%` | `17.90%` |
| raw + per-frame MAC | `1.00x` | `2.90%` | `86.87%` | `0.00%` |
| rep2 adjacent + MAC | `2.00x` | `12.97%` | `94.53%` | `0.00%` |
| rep2 interleaved + MAC | `2.00x` | `52.80%` | `98.22%` | `0.00%` |
| FEC/RLNC `m=28` + per-shard MAC | `1.58x` | `99.82%` | `99.93%` | `0.00%` |
| FEC/RLNC `m=28` + block hash only | `1.58x` | `72.94%` | `96.28%` | `0.00%` |
| FEC/RLNC `m=28` no crypto | `1.58x` | `72.94%` | `99.95%` | `26.93%` |
| FEC/RLNC `m=28` + MAC + interleave | `1.58x` | `100.00%` | `100.00%` | `0.00%` |
| adaptive MAC+FEC base=16 extra=24 | `1.43x` | `100.00%` | `100.00%` | `0.00%` |

## RF/Crypto Primitives: Applied vs Not Applied

| Primitive | Applied? | Status |
|---|---|---|
| per-frame/per-shard MAC | Yes in `spiderweb-edge`; RF integration still next | Critical. Turns corruption into erasure before repair. |
| HMAC authenticated edge frames | Yes | Implemented in `spiderweb-edge` as `Edge::authenticated(key_id, key)`. |
| fountain/XOR repair | Yes in `spiderweb-rf` | Rust example recovers `48/48` chunks over fading loss. |
| FEC/RLNC-style repair | Sim-proven; full RLNC still future | Monte Carlo shows near/full recovery at moderate overhead. |
| interleaving | Sim-proven; not yet wired into RF lane | Very strong against burst fades, but costs latency. |
| adaptive redundancy | Sim-proven; not yet wired into RF lane | Best overhead/result tradeoff in the sim. |
| repetition | Not priority | Helps, especially interleaved, but is inefficient versus FEC/RLNC. |
| block hash only | Not sufficient | Detects a poisoned block late but cannot identify which shard caused it. |
| FEC without crypto | Rejected | Appears to deliver, but silently accepts corrupted blocks. |

The key transport insight:

```text
crypto before repair converts corruption into erasure;
erasure is the failure mode FEC knows how to repair.
```

Without that ordering, corrupted equations poison the repair decoder. A final block hash is still
useful as a commitment/check, but it cannot replace per-shard authentication.

## Unique Or Fascinating Findings

### 1. Confirmation sims re-find seeded truth

The Arc engine-mix run is useful but also humbling. Because its genome already contains `subitize`,
it mostly re-finds `subitize`. That is why the TPU/JAX program search matters: it searches formula
space rather than only tuning seeded knobs.

### 2. Sim magnitudes oversell

The same mechanism can look huge in a tensor sim and shrink in the real engine. This is already seen
with renderer work:

- sim-style search can show `-39%` to `-47%`
- real renderer validation may turn that into `-11%` to `-23%`
- some fast formulas fail exact rendering because depth drift is too high

This is not a failure. It is the reason the project has a validation ladder.

### 3. "Failed" renderer formulas are still useful elsewhere

`overlap+t*.1` and `(radius+i*.02)/.588` are not safe exact-renderer defaults. But as mechanisms,
they map cleanly to:

- spiderweb bus promotion
- scheduler quantum growth
- inference prefetch windows
- cache admission
- RF redundancy scaling
- speculative execution with rollback

The renderer rejects depth drift; the bus can tolerate bounded drift if off-ramps validate and repair.

### 4. Interleaving is a latency-for-survival trade

For RF, repetition with no interleaving only reached `12.97%` clean blocks. Interleaved repetition
reached `52.80%`. Same overhead, very different survival. The mechanism is not "send twice"; it is
"send diversity across independent fade windows."

### 5. Adaptive redundancy beats fixed redundancy

The RF sim reached full recovery at about `1.43x` average overhead with adaptive repair, compared to
`1.58x` fixed overhead for `m=28`. That suggests a bus policy:

```text
start with moderate repair
measure erasures/MAC failures
emit vibration
raise redundancy only when the lane proves it needs it
```

### 6. Security and reliability are not separate here

Authentication is not only "security." In a lossy/corrupt RF lane, it is part of reliability because
it prevents bad shards from poisoning the repair system. Security becomes the filter that makes
self-repair mathematically possible.

## Cross-System Transfer Map

| Mechanism | Renderer | Spiderweb bus | RF/network | Inference/model exoskeleton |
|---|---|---|---|---|
| subitize / clear-margin leap | shipped speed knob | promote when far from conflict | raise rate when SNR margin is high | prefetch/decode more when confidence margin is high |
| overlap + progress | rejected for exact depth | vibration-aware promotion | route when link continuity is coherent | speculative cache reuse |
| age-indexed aggression | rejected for exact depth | hop/dwell-based ramp promotion | adaptive redundancy after stable success | expand batch/prefetch after repeated non-failure |
| guarded optimism | baseline pattern | try fast path, demote on guard failure | adaptive modulation/redundancy | speculative decode with rollback |
| per-shard MAC | not renderer | authenticated lanes | corruption-to-erasure | provenance/trust gate |
| FEC/RLNC/fountain | not renderer | lossy-lane wrapper | self-repair without ARQ | reconstruct/cache missing context shards |
| interleaving | not renderer | latency-tier choice | burst fade survival | distribute work across failure domains |
| currency-as-safety | core validation rule | p95/correction-cost gate | clean-block/silent-corrupt gate | accuracy/cost/regret gate |

## Recommended Next Steps

1. **RF lane integration**
   - Wrap `spiderweb-rf` cells with sequence id, block id, shard id, and per-shard MAC.
   - Verify before decode.
   - Treat failed MACs as erasures.
   - Add dedup/reorder at the receiver.

2. **Adaptive redundancy**
   - Start with a modest repair budget.
   - Emit vibrations for erasure rate, MAC failure rate, and repair overhead.
   - Let the spider raise/lower redundancy or demote traffic to a safer lane.

3. **Interleaving policy**
   - Add an interleaving depth knob.
   - Track latency cost separately from recovery rate.
   - Use it only when burst fades dominate.

4. **Renderer follow-up**
   - Keep `subitize` as the applied primitive.
   - Treat `overlap+progress` and `age-indexed aggression` as transfer leads, not renderer defaults.
   - Use the real Rust validation harness for every candidate before changing shipped marcher policy.

5. **Discovery hygiene**
   - Continue writing decoded formulas and small policy mechanisms.
   - Avoid opaque model-only discoveries for kernel/bus/security work.
   - Record negative results because they explain boundaries.

## Bottom Line

The current experiments have produced two genuinely strong applied primitives:

```text
renderer: subitize / clear-margin leap
transport: authenticate-then-repair
```

They rhyme. Both say:

```text
when the local safety signal is clear, move aggressively;
when the safety signal is unclear, become conservative and repair before committing.
```

That is the shared primitive worth carrying forward.

