# Simulation Run Index

This index maps archived outputs to the claims they support. Treat `manifest.tsv` and `existing-output-inventory.tsv` as the checksum authorities; this file is the human-readable guide.

## Valid Captured Runs

| Run | Category | Claim supported |
| --- | --- | --- |
| `runs/20260701-214125-rf-crypto-self-repair-arc-xpu-32768.log` | RF transport | Arc/XPU Monte Carlo confirms auth-before-repair: adaptive MAC+FEC recovered `100.00%` clean blocks at about `1.43x` overhead with `0.00%` silent corruption. |
| `runs/20260701-214235-spiderweb-rf-fortify-release.log` | RF transport Rust | Concrete `spiderweb-rf` lane: bare fading delivered `38/48`, fountain repair recovered `48/48`. |
| `runs/20260701-214248-spiderweb-rf-chaos-release.log` | RF transport Rust | RF chaos lane is deterministic by seed; fading delivered `393/500`, about `21%` loss, with duplicates. |
| `runs/20260701-214304-mm3e-tpu-signals-real-release.log` | Renderer validation | Real engine rejects aggressive TPU step signals as renderer defaults because depth error rises even when field evaluations drop. |
| `runs/20260701-214319-mm3e-stoch-subitize-real-release.log` | Renderer validation | Subitize is the useful driver, but only in a guarded range; aggressive sim winner is fast with too much depth error. |
| `runs/20260701-214330-mm3e-dual-normal-real-release.log` | Renderer validation | Dual-number normals reduce wall-clock and normal-computation cost while staying close to shipped tetrahedral normals. |
| `runs/20260702-141214-pmre-cargo-test-workspace.log` | PMRE verification | PMRE workspace tests pass: 13 kit tests plus 1 orchestrator test. |
| `runs/20260702-141214-pmre-cargo-clippy-workspace.log` | PMRE verification | PMRE clippy passes with `-D warnings`. |
| `runs/20260702-141214-pmre-cargo-tree.log` | PMRE verification | Dependency proof: `pmre-kit` is zero-dependency, but `pmre-orchestrator` currently pulls `wgpu` and `pollster`. |

## Imported Prior Outputs

| Imported path | Source meaning |
| --- | --- |
| `imported/experiments/engine-mix-arc/arc_mix2.log` | Intel Arc fixed-genome confirmation sim output; shows the seeded genome mostly rediscovers subitize. |
| `imported/experiments/engine-mix-arc/arc_mix2.err` | Stderr sidecar for the Arc engine-mix run; zero length. |
| `imported/experiments/wide-net/discoveries.txt` | Wide cross-domain discovery notes. |
| `imported/experiments/wide-net/widesmoke.txt` | Large wide-net smoke output. |
| `imported/experiments/wide-net/widesmoke2.txt` | Second wide-net smoke output. |
| `imported/experiments/discovery-tpu/WRITEUP.md` | Discovery TPU methodology writeup. |
| `imported/experiments/discovery-tpu/CROSS_DOMAIN_TRANSFER_NOTES.md` | Cross-domain transfer notes for bus, kernel, renderer, inference, RF, security, and false-memory leads. |
| `imported/experiments/PRIMITIVE_SIM_EXPERIMENT_REPORT.md` | Main primitive simulation experiment report. |
| `imported/MM3E_3D_RENDERER_REPORT.md` | Renderer-focused experiment report. |

## Non-Evidence / Failed Capture Attempts

`manifest.tsv` includes `20260701-214215-spiderweb-rf-fortify-release.log` with a nonzero exit code from an early capture-helper quoting bug. Keep it for provenance, but do not cite it as simulation evidence.

The small unmanifested `20260701-214135-*` logs are partial pre-fix capture attempts. They are retained only so no output is silently discarded.
