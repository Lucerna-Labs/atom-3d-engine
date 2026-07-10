# discover-tpu — a raymarching-optimization discovery engine (JAX / Kaggle TPU)

**Publishable experiment.** A genetic-programming search that evolves the SDF marcher's step function
from scratch and — given only primitive arithmetic atoms and a rendering-cost metric — rediscovers
hand-derived optimizations (over-relaxation) and hunts for new ones. A *discovery* engine, not a
*confirmation* engine.

## Files
| File | Role |
|---|---|
| **`CROSS_DOMAIN_TRANSFER_NOTES.md`** | Tracks TPU/MM3E signals, including failed and partial results, as reusable mechanisms for the spiderweb bus, Redos kernel, inference engine, RF/signal processing, networking, security, false-memory systems, Kaggle/OpenAI competition algorithms, and LLM exoskeletons. |
| **`discover-tpu.ipynb`** | **The publishable Kaggle notebook** — full narrative + runnable engine + a live short demo. This is what you push/publish. |
| **`WRITEUP.md`** | The methodology "paper" — background, method, validation, limitations, related work. Read this for the full detail. |
| `discover-tpu.py` | The same engine as a headless **script** (for a long unattended run). |
| `kernel-metadata.json` | Kaggle push config (notebook kernel, TPU, internet on). |
| `engine-mix-tpu.py` | The *confirmation* control (evolves a fixed knob genome — re-finds subitize). |
| `push/` | Clean staging dir (`discover-tpu.ipynb` + `kernel-metadata.json`) ready for `kaggle kernels push`. |

## The idea in one paragraph
SDF sphere tracing steps a ray by `d = sdf(p)` (a safe radius). Faster hand-derived rules exist
(over-relaxation `ω·d`, secant, LOD, our shipped `subitize`). We evolve the step rule as a **linear
genetic program** — `L` instructions `(op,a,b,dst)` over registers seeded with march signals, ops = the
primitive library's root atoms (`add sub mul div min max abs neg sqrt sin hash gt glue half`), output
register = the step. A no-op program is naive sphere tracing, so the search *starts at baseline*.
**Nothing is pre-loaded** — subitize is just one reachable program among an astronomical space.
**Safety = the currency**: fitness is field-eval reduction gated by image error (tunnelers fail the gate
and die), so no hand-coded guard biases the search. The winner is **decoded to a readable formula**, so a
discovery is shippable (as `subitize` was, into the real Rust renderer).

## Validated before any TPU run
Ran locally on `jax[cpu]`. From **random programs** it independently rediscovered **over-relaxation**
three times across seeds/configs — `step ≈ d·1.70`, `d·2.03`, `(radius+i·0.02)/0.588` — i.e. Keinert's
2014 enhanced sphere tracing — and, on other seeds, *different* mechanisms (`overlap + t·0.1`). Reinventing
a hand-derived optimization from primitive atoms is the pass condition for a discovery engine.

## Publish / run on Kaggle TPU
```bash
kaggle kernels push -p push/          # pushes the notebook (see kernel-metadata.json)
kaggle kernels status jessealicea/discover-tpu
# to make it public/published: set "is_private": false in kernel-metadata.json (or flip it in the Kaggle UI).
```
- The notebook's **demo cell runs a short search (~0.5 min)** and prints discoveries live — perfect for a
  published, self-running notebook. For the real experiment, change that call to
  `run_search(minutes=450, pop=256, ncam=6, res=(80,45), steps=128, L=10)`.
- Confirm the accelerator attached: the setup cell must print **8 TPU devices**.
- Live progress also streams to the webhook relay (script mode):
  `https://webhook.site/ff06312d-a70d-42ec-870f-fa6acfc717c4`.

### Reproduce the local logic check (no TPU needed)
```bash
pip install "jax[cpu]" numpy
DISCOVER_NO_WEBHOOK=1 python discover-tpu.py --pop 32 --minutes 0.3 --res 24x14 --ncam 3 --steps 48 --L 8
```

## Status / honesty
- The engine + notebook + writeup are **built and validated on real JAX (CPU)**. The full TPU numbers are
  **not yet filled in** — the push is currently blocked on a fresh Kaggle API token (the stored one 401s).
- The CPU-smoke magnitudes (−52 %, −60 %) are **not** real results — tiny scene, naive baseline, no
  wall-clock. They prove the machine *discovers*. Real magnitudes come from the TPU run, and any discovery
  is a **lead** until it's reimplemented in the real Rust marcher and wall-clock-tested (the `subitize`
  pipeline: `mm3e-orchestrator/examples/stoch_subitize_real.rs`).

## Regenerate the notebook
`discover-tpu.ipynb` is generated from `discover-tpu.py` (engine reused verbatim) by
`../build_notebook.py`, so the notebook can't drift from the tested engine.
