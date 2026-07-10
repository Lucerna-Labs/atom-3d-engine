# Engine-Mix Arc Run Notes

## 2026-07-02 local Arc sweep

Command:

```powershell
& "C:\Projects\discovery-search-12h-gpu\.venv-arc\Scripts\python.exe" experiments\engine-mix-arc\arc_engine_mix2.py --minutes 5 --pop 128 --res 96x54
```

Device/runtime:

- `torch 2.12.1+xpu`
- `Intel(R) Arc(TM) A380 Graphics`
- `cuda_available = False`
- `62208` rays: 12 cameras at `96x54`
- `4224` configs tested in `302s`
- Baseline: `2248192` stock field-evals

Best fresh result:

```text
-39.1% field-evals | omega=1.00 lod=0.0007 sec=0.150 fq=0 normal=dual
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

Interpretation:

- **Primary survivor:** subitize / "far means take a larger empty-space leap." It accounts for most of the useful gain in both the previous two-hour Arc run and this fresh five-minute run.
- **Small but plausible add-ons:** momentum and second-order structure. They are not major on their own, but repeatedly show small positive contribution near the quality gate.
- **Weak in this run:** stochastic omega, binary refine, and glue. They may still be useful as stabilizers in longer runs or different scenes, but they did not earn priority here.
- **Renderer action:** test a hand-written `subitize + secant + dual-normal` marcher in the real Rust harness before shipping any policy. Treat `subitize ~= 0.7..0.85`, `secant ~= 0.15`, `omega = 1.0`, `lod ~= 0.0007..0.0009` as the first candidate band.
- **Spiderweb/inference transfer:** subitize maps cleanly to confidence-gated promotion: when the local signal is clearly safe/far from a boundary, take a bigger scheduling/routing leap; if near a boundary, stay conservative. Momentum/sym2 map to small trend/curvature nudges, not standalone policies.

