# Track 1 Deliverable: Tier 0-2 Simulation Evaluation

## Summary

Built the first half of the simulation evaluation notebook for Jesse's Small Model Memory Lab. Created a unified orchestrator (`run_simulation.py`) that covers Tier 0 (corpus analyzer, CPU-only), Tier 1 (direct model measurement, 8-cell matrix), and Tier 2 (behavioral benchmarks, custom probe tasks). All code is self-contained and works on both Kaggle (with GPU) and locally (CPU for Tier 0).

## Changed Files

- `kaggle_notebooks/simulation_evaluation/run_simulation.py` — Complete orchestrator (520 lines): Tier 0 corpus analyzer, Tier 1 direct measurement, Tier 2 benchmarks, with imports for Track 2's Tier 3-4 modules. Supports `--tier 0/1/2/3/4` and `--aggregate` flags. Outputs to `/kaggle/working/simulation_results/` (Kaggle) or `runs/simulation_results/` (local).

## Notes

- Model matrix: Qwen3.5-2B x Qwen3.5-4B x Q4_K_M x BF16
- Tier 0 runs on CPU (no GPU needed) — good for quick corpus quality check
- Tiers 1-2 skip gracefully when no GPU is available
- Uses `bitsandbytes` for Q4 quantization with FP16 fallback
- Results saved as JSON to output directory on every tier
- Tier 3-4 modules are imported from the same directory (Track 2's files)