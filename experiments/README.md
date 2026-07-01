# experiments/ — cross-domain primitive discovery for the mm3e marcher

Python simulation + discovery code that feeds the Rust engine. These search for marcher optimizations
against the engine's real currency (SDF field-evals per frame at held image quality); anything that
survives gets reimplemented and wall-clock-validated in the real Rust marcher (see
`../mm3e-orchestrator/examples/`), which is the only number we trust.

> **Provenance note.** This work was developed in a session scratchpad (a temp dir) and consolidated
> here so it lives with the repo. The `jaxcpu` local venv and assorted one-off exploratory scripts were
> left behind intentionally (regenerable / not load-bearing). Live run results also exist on Kaggle and
> the webhook relay (see below).

## Contents

### `discovery-tpu/` — the TRUE discovery engine (current, publishable)
Genetic-programming synthesis of the marcher **step function** from scratch. Given only primitive
arithmetic atoms and the cost metric, it rediscovers hand-derived optimizations (over-relaxation) and
hunts for new ones. **A discovery engine, not a confirmation engine.**
- `discover-tpu.py` — the engine (JAX; runs on Kaggle TPU, also CPU for logic checks).
- `discover-tpu.ipynb` — the **publishable Kaggle notebook** (narrative + engine + live demo). Pushed
  **private** to https://www.kaggle.com/code/jessealicea/discover-tpu.
- `WRITEUP.md` — the full methodology paper. `README.md` — how to run/publish.
- `engine-mix-tpu.py` — the *confirmation* control (fixed-genome knob search) for contrast.
- `build_notebook.py` — regenerates the .ipynb from `discover-tpu.py` (paths were scratchpad-absolute;
  adjust if re-running).
- `kernel-metadata.json` — Kaggle push config (notebook kernel, TPU, private).

### `engine-mix-arc/` — the confirmation sims (Intel Arc, torch-XPU)
Evolve the parameters of a fixed operator genome (subitize, over-relax, secant, glue, symbols, …).
- `arc_engine_mix.py`, `arc_engine_mix2.py` (v2 = the 4-domain run).
- `arc_mix2.log` — **the v2 result**: −39.9%, and the per-domain ablation showing subitize is the only
  real driver (glue/symbols ~0%) — the evidence that a confirmation genome only re-finds what's seeded.

### `engine-mix-8h-kaggle/` — the confirmation sim scaled up (Kaggle P100, torch-CUDA)
`engine-mix-8h.py` + metadata. Higher-res confirmation run (webhook-streamed).

### `wide-net/` — the wide cross-domain bridge search
`wide_net.py` — tensorized chain-gating over the full ~11.7k-primitive library (Arc XPU);
`discoveries.txt`, `widesmoke*.txt` are result notes.

## Where the shipped results ended up (in the Rust engine, not here)
- `Marcher::subitize` (mm3e-kit/src/march.rs) — ANS "leap when clearly far", validated real
  (−16..23% field-evals & wall-clock), tested by `../mm3e-orchestrator/examples/stoch_subitize_real.rs`.
- Dual-number analytic normal (mm3e-kit/src/dual.rs) — validated by `.../examples/dual_normal_real.rs`.

## Compute policy (important)
Sims run on the **Intel Arc A380** (`torch-xpu`) or **Kaggle** (GPU/TPU) — **never** the RTX 5070 Ti,
and **not** on the CPU (local `jax[cpu]` validation spikes the CPU and is off-limits). See the repo's
compute-venue notes.

## Live result channels (external)
- Kaggle notebook: https://www.kaggle.com/code/jessealicea/discover-tpu (private)
- Webhook relay (progress + discovered formulas): `https://webhook.site/ff06312d-a70d-42ec-870f-fa6acfc717c4`
