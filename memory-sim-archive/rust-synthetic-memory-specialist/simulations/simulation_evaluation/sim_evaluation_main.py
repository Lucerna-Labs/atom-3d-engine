#!/usr/bin/env python
"""
============================================================
SIMULATION EVALUATION NOTEBOOK - MAIN ENTRY POINT
============================================================
Tier 3-4 + Results Aggregator for Small Model Memory Lab.

This module orchestrates:
  1. Tier 3: Domain-specific probes (cyber, structural, association, echo)
  2. Tier 4: Deep simulation scenarios (10 adversarial tests)
  3. Aggregator: Combines all results into simulation_report.json + plots

Run order:
  python sim_evaluation_main.py           → run all tiers + aggregate
  python sim_evaluation_aggregator.py    → aggregate only (after tiers complete)

Kaggle-compatible: detect /kaggle/working, fall back to local runs/ dir.
"""

from __future__ import annotations

import json
import os
import sys
from pathlib import Path

# ── Paths ────────────────────────────────────────────────────────────────────
KAGGLE_INPUT  = Path("/kaggle/input/small-model-memory-lab")
LOCAL_ROOT    = Path(__file__).parent.parent.parent
WORKING_DIR    = Path("/kaggle/working/simulation_results")
LOCAL_RUNS     = LOCAL_ROOT / "runs" / "simulation_results"
OUT_DIR        = WORKING_DIR if Path("/kaggle/working").exists() else LOCAL_RUNS

OUT_DIR.mkdir(parents=True, exist_ok=True)


def find_root() -> Path | None:
    if KAGGLE_INPUT.exists():
        return KAGGLE_INPUT
    for p in KAGGLE_INPUT.parent.iterdir():
        if (p / "benchmarks").exists() and (p / "corpus").exists():
            return p
    if (LOCAL_ROOT / "benchmarks").exists():
        return LOCAL_ROOT
    return None


def run_module(name: str, path: Path) -> bool:
    print(f"\n{'=' * 60}")
    print(f"Running: {name}")
    print(f"{'=' * 60}")
    if not path.exists():
        print(f"  [SKIP] {path} not found")
        return False
    try:
        import importlib.util
        spec = importlib.util.spec_from_file_location(name, path)
        if spec and spec.loader:
            mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(mod)
            if hasattr(mod, "main"):
                mod.main()
            elif hasattr(mod, "run_full_tier3"):
                mod.run_full_tier3()
            elif hasattr(mod, "run_full_tier4"):
                mod.run_full_tier4()
        return True
    except Exception as e:
        print(f"  [ERROR] {e}")
        return False


def run_aggregator() -> None:
    print(f"\n{'=' * 60}")
    print("Running: Aggregator + Visualizer")
    print(f"{'=' * 60}")
    agg_path = Path(__file__).parent / "sim_evaluation_aggregator.py"
    run_module("Aggregator", agg_path)


def main() -> int:
    print("=" * 70)
    print("SMALL MODEL MEMORY LAB - SIMULATION EVALUATION")
    print("Tiers 3-4 + Aggregator")
    print("=" * 70)
    print(f"Output directory: {OUT_DIR}")
    print(f"Root: {find_root()}")

    root = find_root()
    print(f"\nDataset check: {'OK' if root else 'NOT FOUND'}")

    # Tier 3
    tier3_path = Path(__file__).parent / "sim_evaluation_tiers_3_4.py"
    tier3_ok = run_module("Tier 3: Domain-Specific Probes", tier3_path)

    # Tier 4
    tier4_path = Path(__file__).parent / "sim_evaluation_tier4_deep_simulations.py"
    tier4_ok = run_module("Tier 4: Deep Simulation Scenarios", tier4_path)

    # Aggregator
    run_aggregator()

    print("\n" + "=" * 70)
    print("ALL DONE")
    print("=" * 70)
    print("Outputs:")
    print(f"  - {OUT_DIR / 'tier3_domain_probes.json'}")
    print(f"  - {OUT_DIR / 'tier4_deep_simulations.json'}")
    print(f"  - {OUT_DIR / 'simulation_report.json'}")
    print(f"  - {OUT_DIR / 'simulation_plots.pdf'}")
    print()
    print("To push to Kaggle:")
    print(f"  python scripts/upload_kaggle_kernel_sdk.py --kernel-dir kaggle_notebooks/simulation_evaluation")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())