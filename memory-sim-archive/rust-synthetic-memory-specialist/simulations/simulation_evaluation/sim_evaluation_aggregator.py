#!/usr/bin/env python
"""
============================================================
RESULTS AGGREGATOR AND VISUALIZER
============================================================
Combines Tier 0-4 outputs into:
  - simulation_report.json  (machine-readable)
  - simulation_plots.pdf   (matplotlib charts)

Outputs: simulation_report.json, simulation_plots.pdf
to both /kaggle/working/simulation_results/ and
C:\\Projects\\small-model-memory-lab\\runs\\simulation_results\\
"""

from __future__ import annotations

import json
import os
import sys
from pathlib import Path
from typing import Any

# ── env guard ──────────────────────────────────────────────────────────────
os.environ.setdefault("TRANSFORMERS_NO_TORCHVISION", "1")
os.environ.setdefault("TRANSFORMERS_NO_TF", "1")
os.environ.setdefault("TRANSFORMERS_NO_FLAX", "1")

# ── paths ──────────────────────────────────────────────────────────────────
KAGGLE_INPUT = Path("/kaggle/input/small-model-memory-lab")
LOCAL_ROOT   = Path(__file__).parent.parent.parent   # D:\minimax-lab
MEMORY_CORPUS_DIR = LOCAL_ROOT / "memory_corpus"
WORKING_DIR  = Path("/kaggle/working/simulation_results")
LOCAL_RUNS   = LOCAL_ROOT / "runs" / "simulation_results"
OUT_DIRS    = [WORKING_DIR, LOCAL_RUNS]

# ── helpers ──────────────────────────────────────────────────────────────────

def find_root() -> Path | None:
    if KAGGLE_INPUT.exists():
        return KAGGLE_INPUT
    for p in KAGGLE_INPUT.parent.iterdir():
        if (p / "benchmarks").exists() and (p / "memory_corpus").exists():
            return p
    if (LOCAL_ROOT / "memory_corpus").exists():
        return LOCAL_ROOT
    return None


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def ensure_dirs(*paths: Path) -> None:
    for p in paths:
        p.mkdir(parents=True, exist_ok=True)


# =============================================================================
# MATPLOTLIB VISUALIZER
# =============================================================================

def try_import_matplotlib() -> bool:
    try:
        __import__("matplotlib")
        return True
    except ImportError:
        return False


def try_install_matplotlib() -> None:
    import subprocess
    subprocess.check_call([sys.executable, "-m", "pip", "install", "-q", "matplotlib"])


def generate_plots(report: dict[str, Any], out_dir: Path) -> Path | None:
    """Generate simulation_plots.pdf with 3 subplots."""
    if not try_import_matplotlib():
        try_install_matplotlib()

    import matplotlib
    matplotlib.use("Agg")  # non-interactive
    import matplotlib.pyplot as plt
    import numpy as np

    fig, axes = plt.subplots(1, 3, figsize=(18, 5))
    fig.suptitle("Small Model Memory Lab - Simulation Evaluation", fontsize=14, fontweight="bold")

    cells = report.get("matrix_cells", [])
    model_labels = [c.get("label", "?") for c in cells]
    model_sizes = [2 if "2B" in l else 4 for l in model_labels]
    quants = [c.get("quant", "?") for c in cells]

    # ── Plot 1: Scaling curves (WITH vs WITHOUT per quant) ─────────────────
    ax1 = axes[0]
    with_scores_2b = [c.get("tier3_cyber_behavioral", 0) for c in cells if "2B" in c.get("label", "")]
    with_scores_4b = [c.get("tier3_cyber_behavioral", 0) for c in cells if "4B" in c.get("label", "")]
    without_scores_2b = [c.get("tier3_cyber_behavioral_no_corpus", 0) for c in cells if "2B" in c.get("label", "")]

    x_2b = [2] * len(with_scores_2b)
    x_4b = [4] * len(with_scores_4b)
    ax1.scatter(x_2b, with_scores_2b, color="green", label="WITH corpus (2B)", s=80, alpha=0.8, zorder=3)
    ax1.scatter(x_4b, with_scores_4b, color="green", marker="^", s=80, alpha=0.8, zorder=3)
    ax1.scatter(x_2b, without_scores_2b, color="red", label="WITHOUT corpus (2B)", s=80, alpha=0.6, zorder=3)
    ax1.set_xlabel("Model Size (B params)")
    ax1.set_ylabel("Cyber Behavioral Score")
    ax1.set_title("Scaling Curves: Corpus Signal vs Model Size")
    ax1.set_xticks([2, 4])
    ax1.set_xticklabels(["2B", "4B"])
    ax1.legend(fontsize=8)
    ax1.grid(True, alpha=0.3)
    ax1.set_ylim(0, 1.05)

    # ── Plot 2: Heatmap of all 8 runs ───────────────────────────────────────
    ax2 = axes[1]
    n_cells = len(cells)
    heat_data = np.array([
        [c.get("tier3_cyber_strict", 0), c.get("tier3_cyber_behavioral", 0),
         c.get("tier4_deep_pass_rate", 0)]
        for c in cells
    ]).T  # 3 rows (strict, behavioral, deep sim), n_cells cols

    cell_labels = [c.get("label", f"Cell {i}") for i in range(n_cells)]
    row_labels = ["Cyber Strict", "Cyber Behavioral", "Tier4 Deep Sim"]

    im = ax2.imshow(heat_data, aspect="auto", cmap="RdYlGn", vmin=0, vmax=1)
    ax2.set_xticks(range(n_cells))
    ax2.set_xticklabels(cell_labels, rotation=45, ha="right", fontsize=7)
    ax2.set_yticks(range(3))
    ax2.set_yticklabels(row_labels)
    ax2.set_title("All 8 Runs: Score Heatmap")
    for i in range(3):
        for j in range(n_cells):
            ax2.text(j, i, f"{heat_data[i, j]:.2f}",
                     ha="center", va="center", fontsize=7,
                     color="white" if heat_data[i, j] < 0.3 else "black")
    fig.colorbar(im, ax=ax2, label="Score", shrink=0.8)

    # ── Plot 3: Template echo rate comparison ─────────────────────────────
    ax3 = axes[2]
    echo_rates = [c.get("template_echo_rate", 0) for c in cells]
    colors_echo = ["green" if "WITH" in c.get("label", "") else "red" for c in cells]
    bars = ax3.bar(range(n_cells), echo_rates, color=colors_echo, alpha=0.7, edgecolor="black")
    ax3.set_xticks(range(n_cells))
    ax3.set_xticklabels(cell_labels, rotation=45, ha="right", fontsize=7)
    ax3.set_ylabel("Template Echo Rate")
    ax3.set_title("Template Echo Rate per Model+Corpus")
    ax3.set_ylim(0, max(0.5, max(echo_rates) * 1.2) if echo_rates else 0.5)
    ax3.axhline(y=0.2, color="orange", linestyle="--", linewidth=1, label="Warning threshold")
    ax3.legend(fontsize=8)
    ax3.grid(True, alpha=0.3, axis="y")

    plt.tight_layout()
    out_path = out_dir / "simulation_plots.pdf"
    plt.savefig(out_path, format="pdf", bbox_inches="tight", dpi=150)
    plt.close(fig)
    print(f"Plots saved to {out_path}")
    return out_path


# =============================================================================
# REPORT AGGREGATOR
# =============================================================================

def compute_delta(a: float, b: float) -> float:
    return round(a - b, 4)


def aggregate_results(
    tier3_path: Path | None,
    tier4_path: Path | None,
    tier0_path: Path | None,
) -> dict[str, Any]:
    """Build simulation_report.json from all available tier outputs."""

    report: dict[str, Any] = {
        "corpus_quality_score": 0.0,
        "per_model_scores": {},
        "pairwise_deltas": [],
        "scaling_curves": {"2B": {}, "4B": {}},
        "cross_factor_interaction": {},
        "template_echo_rate": {},
        "overall_simulation_signal": 0.0,
        "recommendation": "INSUFFICIENT_DATA",
        "confidence_level": "low",
        "top_strengths": [],
        "top_weaknesses": [],
        "matrix_cells": [],
        "tiers_summary": {},
    }

    matrix_cells = []
    strengths = []
    weaknesses = []

    model_ids = ["Qwen/Qwen3.5-2B", "Qwen/Qwen3.5-4B"]
    quants    = ["Q4_K_M", "BF16"]

    # ── Load tier data ────────────────────────────────────────────────────
    tier3_data = load_json(tier3_path) if tier3_path and tier3_path.exists() else {}
    tier4_data = load_json(tier4_path) if tier4_path and tier4_path.exists() else {}

    # ── Build matrix cells ───────────────────────────────────────────────
    for model_id in model_ids:
        model_label = model_id.split("/")[-1].replace(".", "")
        for quant in quants:
            for with_mem in [False, True]:
                suffix = f"{model_label}_{quant}_{'WITH' if with_mem else 'WITHOUT'}"
                cell = {"label": suffix, "model": model_label, "quant": quant,
                        "with_corpus": with_mem}

                # Tier 3 data
                t3_cell = next(
                    (r for r in tier3_data.get("matrix", [])
                     if r.get("cell", "").endswith(suffix)), {}
                )
                if t3_cell:
                    cell["tier3_cyber_strict"]     = t3_cell.get("cyber", {}).get("strict_score", 0)
                    cell["tier3_cyber_behavioral"]  = t3_cell.get("cyber", {}).get("behavioral_score", 0)
                    cell["tier3_structural_score"]  = t3_cell.get("structural", {}).get("score_value", 0)
                    cell["template_echo_rate"]     = t3_cell.get("cyber", {}).get("template_echo_avg", 0)
                    cell["association_match_rate"]  = t3_cell.get("association", {}).get("structural_match_rate", 0)

                # Tier 4 data
                t4_cell = next(
                    (r for r in tier4_data.get("matrix", [])
                     if r.get("cell", "").endswith(suffix)), {}
                )
                if t4_cell:
                    cell["tier4_deep_pass_rate"]   = t4_cell.get("summary", {}).get("overall_pass_rate", 0)
                    cell["tier4_leaked_scenarios"] = t4_cell.get("summary", {}).get("leaked_scenarios", 0)
                    cell["tier4_avg_structural"]   = t4_cell.get("summary", {}).get("avg_structural_match", 0)

                matrix_cells.append(cell)

    report["matrix_cells"] = matrix_cells

    # ── Compute pairwise deltas (WITH - WITHOUT) ──────────────────────────
    deltas = []
    for model_id in model_ids:
        model_label = model_id.split("/")[-1].replace(".", "")
        for quant in quants:
            suffix = f"{model_label}_{quant}"
            with_cell    = next((c for c in matrix_cells
                                 if c["model"] == model_label and c["quant"] == quant and c["with_corpus"]), None)
            without_cell = next((c for c in matrix_cells
                                 if c["model"] == model_label and c["quant"] == quant and not c["with_corpus"]), None)
            if with_cell and without_cell:
                delta = {
                    "cell": suffix,
                    "model": model_label,
                    "quant": quant,
                    "cyber_strict_delta":    compute_delta(with_cell.get("tier3_cyber_strict", 0),
                                                            without_cell.get("tier3_cyber_strict", 0)),
                    "cyber_behav_delta":     compute_delta(with_cell.get("tier3_cyber_behavioral", 0),
                                                            without_cell.get("tier3_cyber_behavioral", 0)),
                    "structural_delta":      compute_delta(with_cell.get("tier3_structural_score", 0),
                                                            without_cell.get("tier3_structural_score", 0)),
                    "tier4_delta":           compute_delta(with_cell.get("tier4_deep_pass_rate", 0),
                                                            without_cell.get("tier4_deep_pass_rate", 0)),
                }
                deltas.append(delta)

                # Track for strengths/weaknesses
                if delta["cyber_strict_delta"] > 0.1:
                    strengths.append(f"{suffix}: cyber_strict +{delta['cyber_strict_delta']:.2f}")
                if delta["cyber_strict_delta"] < -0.05:
                    weaknesses.append(f"{suffix}: cyber_strict {delta['cyber_strict_delta']:.2f}")

    report["pairwise_deltas"] = deltas

    # ── Scaling curves ───────────────────────────────────────────────────
    for size in ["2B", "4B"]:
        size_cells = [c for c in matrix_cells if size in c.get("model", "")]
        with_cells    = [c for c in size_cells if c.get("with_corpus")]
        without_cells = [c for c in size_cells if not c.get("with_corpus")]
        report["scaling_curves"][size] = {
            "with_avg_cyber_behavioral":    round(sum(c.get("tier3_cyber_behavioral", 0) for c in with_cells) /
                                                   max(1, len(with_cells)), 4),
            "without_avg_cyber_behavioral": round(sum(c.get("tier3_cyber_behavioral", 0) for c in without_cells) /
                                                   max(1, len(without_cells)), 4),
            "with_avg_tier4":              round(sum(c.get("tier4_deep_pass_rate", 0) for c in with_cells) /
                                                   max(1, len(with_cells)), 4),
            "without_avg_tier4":           round(sum(c.get("tier4_deep_pass_rate", 0) for c in without_cells) /
                                                   max(1, len(without_cells)), 4),
        }

    # ── Cross-factor interaction: where does corpus matter most? ───────────
    for quant in quants:
        q_deltas = [d for d in deltas if d["quant"] == quant]
        if q_deltas:
            avg = round(sum(d["cyber_strict_delta"] for d in q_deltas) / len(q_deltas), 4)
            report["cross_factor_interaction"][quant] = {
                "avg_cyber_strict_delta": avg,
                "signal": "strong" if avg > 0.15 else "moderate" if avg > 0.05 else "weak",
            }

    # ── Template echo rates ──────────────────────────────────────────────
    for cell in matrix_cells:
        if cell.get("with_corpus"):
            model_key = cell.get("model", "?")
            if model_key not in report["template_echo_rate"]:
                report["template_echo_rate"][model_key] = []
            report["template_echo_rate"][model_key].append(cell.get("template_echo_rate", 0))

    for model_key in report["template_echo_rate"]:
        rates = report["template_echo_rate"][model_key]
        report["template_echo_rate"][model_key] = round(sum(rates) / max(1, len(rates)), 4)

    # ── Corpus quality score ─────────────────────────────────────────────
    positive_deltas = [d for d in deltas
                       if d["cyber_strict_delta"] > 0 or d["structural_delta"] > 0 or d["tier4_delta"] > 0]
    report["corpus_quality_score"] = round(
        len(positive_deltas) / max(1, len(deltas)), 4
    )

    # ── Overall simulation signal ────────────────────────────────────────
    signal_components = []
    if deltas:
        signal_components.append(
            sum(d["cyber_strict_delta"] for d in deltas) / len(deltas)
        )
        signal_components.append(
            sum(d["tier4_delta"] for d in deltas) / len(deltas)
        )
    avg_signal = sum(signal_components) / max(1, len(signal_components))
    report["overall_simulation_signal"] = round(max(0, min(1, avg_signal + 0.5)), 4)

    # ── Recommendation logic ─────────────────────────────────────────────
    if report["overall_simulation_signal"] >= 0.65 and report["corpus_quality_score"] >= 0.6:
        rec = "PROCEED_TO_TRAINING"
        conf = "high"
    elif report["corpus_quality_score"] >= 0.4:
        rec = "REVISE_CORPUS"
        conf = "medium"
    else:
        rec = "ABANDON_DIRECTION"
        conf = "low"
    report["recommendation"] = rec
    report["confidence_level"] = conf

    # ── Top 3 strengths / weaknesses ─────────────────────────────────────
    report["top_strengths"] = sorted(set(strengths))[:3]
    report["top_weaknesses"] = sorted(set(weaknesses))[:3]

    # ── Per-model scores ─────────────────────────────────────────────────
    for model_id in model_ids:
        model_label = model_id.split("/")[-1].replace(".", "")
        m_cells = [c for c in matrix_cells if c.get("model") == model_label]
        with_cells    = [c for c in m_cells if c.get("with_corpus")]
        without_cells = [c for c in m_cells if not c.get("with_corpus")]

        def avg(lst, key):
            return round(sum(c.get(key, 0) for c in lst) / max(1, len(lst)), 4)

        report["per_model_scores"][model_label] = {
            "with_corpus": {
                "cyber_strict": avg(with_cells, "tier3_cyber_strict"),
                "cyber_behavioral": avg(with_cells, "tier3_cyber_behavioral"),
                "structural": avg(with_cells, "tier3_structural_score"),
                "tier4_deep": avg(with_cells, "tier4_deep_pass_rate"),
                "template_echo": avg(with_cells, "template_echo_rate"),
            },
            "without_corpus": {
                "cyber_strict": avg(without_cells, "tier3_cyber_strict"),
                "cyber_behavioral": avg(without_cells, "tier3_cyber_behavioral"),
                "structural": avg(without_cells, "tier3_structural_score"),
                "tier4_deep": avg(without_cells, "tier4_deep_pass_rate"),
            },
            "delta_cyber_strict": round(
                avg(with_cells, "tier3_cyber_strict") - avg(without_cells, "tier3_cyber_strict"), 4),
            "delta_cyber_behavioral": round(
                avg(with_cells, "tier3_cyber_behavioral") - avg(without_cells, "tier3_cyber_behavioral"), 4),
            "delta_tier4": round(
                avg(with_cells, "tier4_deep_pass_rate") - avg(without_cells, "tier4_deep_pass_rate"), 4),
        }

    # ── Tiers summary ────────────────────────────────────────────────────
    report["tiers_summary"] = {
        "tier3": {
            "corpus_quality_score": report["corpus_quality_score"],
            "deltas": deltas,
            "cells_evaluated": len(matrix_cells),
        },
        "tier4": {
            "overall_signal": report["overall_simulation_signal"],
            "cross_factor_interaction": report["cross_factor_interaction"],
        },
    }

    return report


# =============================================================================
# MAIN
# =============================================================================

def main() -> None:
    print("=" * 70)
    print("RESULTS AGGREGATOR AND VISUALIZER")
    print("=" * 70)

    root = find_root()
    tier0_path = root / "benchmarks" / "capability_tasks.json" if root else None
    tier3_path = OUT_DIRS[0] / "tier3_domain_probes.json"
    tier4_path = OUT_DIRS[0] / "tier4_deep_simulations.json"

    # Try local runs fallback
    if not tier3_path.exists():
        tier3_path = LOCAL_RUNS / "tier3_domain_probes.json"
    if not tier4_path.exists():
        tier4_path = LOCAL_RUNS / "tier4_deep_simulations.json"

    print(f"Tier 3 path: {tier3_path} ({tier3_path.exists()})")
    print(f"Tier 4 path: {tier4_path} ({tier4_path.exists()})")
    print(f"Tier 0 path: {tier0_path} ({tier0_path.exists() if tier0_path else False})")

    report = aggregate_results(tier3_path, tier4_path, tier0_path)

    # Save to all output dirs
    for out_dir in OUT_DIRS:
        ensure_dirs(out_dir)
        report_path = out_dir / "simulation_report.json"
        report_path.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
        print(f"Report saved to {report_path}")

        # Generate plots if matplotlib available
        try:
            plot_path = generate_plots(report, out_dir)
            if plot_path:
                print(f"Plots saved to {plot_path}")
        except Exception as e:
            print(f"Plot generation skipped: {e}")

    # Print recommendation
    print()
    print("=" * 50)
    print("SIMULATION RECOMMENDATION")
    print("=" * 50)
    print(f"  Overall signal:    {report['overall_simulation_signal']}")
    print(f"  Corpus quality:      {report['corpus_quality_score']}")
    print(f"  Recommendation:      {report['recommendation']}")
    print(f"  Confidence:          {report['confidence_level']}")
    print()
    print("  Top strengths:")
    for s in report["top_strengths"]:
        print(f"    - {s}")
    print("  Top weaknesses:")
    for w in report["top_weaknesses"]:
        print(f"    - {w}")
    print()
    print("  Pairwise deltas (WITH - WITHOUT):")
    for d in report["pairwise_deltas"]:
        print(f"    {d['cell']}: cyber_strict={d['cyber_strict_delta']:+.2f} "
              f"tier4={d['tier4_delta']:+.2f}")
    print()
    print("Done.")


if __name__ == "__main__":
    raise SystemExit(main())