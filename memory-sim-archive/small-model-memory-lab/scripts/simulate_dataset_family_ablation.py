from __future__ import annotations

import csv
import json
import random
from pathlib import Path

from simulate_dataset_mixture_candidates import MODEL_PROFILES, install_model_profile_quants
from simulate_exhaustive_stack_sweep import (
    SUBSTRATES,
    TASKS,
    StackConfig,
    Stats,
    flatten_counter,
    pressure_band,
    run_trial,
)


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "dataset-family-ablation"


STACKS = (
    StackConfig(
        "baseline",
        {},
        1.00,
        1.00,
        "No dataset active.",
    ),
    StackConfig(
        "cognitive_only",
        {"cognitive": 1.00},
        0.82,
        0.78,
        "Cognitive/generalist memories only.",
    ),
    StackConfig(
        "cyber_only",
        {"cyber": 1.00},
        0.84,
        0.78,
        "Cyber/source-boundary memories only.",
    ),
    StackConfig(
        "structural_only",
        {"structural": 1.00},
        0.94,
        0.94,
        "Structural primitive examples only.",
    ),
    StackConfig(
        "cognitive_cyber_equal",
        {"cognitive": 0.50, "cyber": 0.50},
        0.80,
        0.74,
        "Equal cognitive plus cyber memories without structural substrate.",
    ),
    StackConfig(
        "cognitive_structural_equal",
        {"cognitive": 0.50, "structural": 0.50},
        0.88,
        0.84,
        "Equal cognitive plus structural.",
    ),
    StackConfig(
        "cyber_structural_equal",
        {"cyber": 0.50, "structural": 0.50},
        0.88,
        0.84,
        "Equal cyber plus structural.",
    ),
    StackConfig(
        "cognitive_25_structural_75",
        {"cognitive": 0.25, "structural": 0.75},
        0.91,
        0.89,
        "Structural-heavy generalist pair.",
    ),
    StackConfig(
        "cyber_25_structural_75",
        {"cyber": 0.25, "structural": 0.75},
        0.91,
        0.89,
        "Structural-heavy source-boundary pair.",
    ),
    StackConfig(
        "tri_equal",
        {"cognitive": 1 / 3, "cyber": 1 / 3, "structural": 1 / 3},
        0.82,
        0.78,
        "Equal three-family mixture.",
    ),
    StackConfig(
        "tri_structural_65",
        {"cognitive": 0.20, "cyber": 0.15, "structural": 0.65},
        0.90,
        0.88,
        "Structural-heavy three-family mixture.",
    ),
    StackConfig(
        "tri_stability_70",
        {"cognitive": 0.20, "cyber": 0.10, "structural": 0.70},
        0.93,
        0.92,
        "Stability-oriented three-family mixture.",
    ),
    StackConfig(
        "tri_routed_72_18_10",
        {"cognitive": 0.18, "cyber": 0.10, "structural": 0.72},
        0.94,
        0.93,
        "v0.2-style Q3 stability ratio.",
    ),
    StackConfig(
        "tri_routed_68_20_12",
        {"cognitive": 0.20, "cyber": 0.12, "structural": 0.68},
        0.93,
        0.90,
        "v0.2-style Q3 routed ratio.",
    ),
    StackConfig(
        "tri_naive_all_fire",
        {"cognitive": 1 / 3, "cyber": 1 / 3, "structural": 1 / 3},
        0.48,
        0.46,
        "Naive three-family all-fire mixture.",
        all_fire=True,
    ),
)


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        return
    fieldnames: list[str] = []
    for row in rows:
        for key in row:
            if key not in fieldnames:
                fieldnames.append(key)
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def best_rows(summary: list[dict[str, object]]) -> list[dict[str, object]]:
    grouped: dict[tuple[str, str, str], list[dict[str, object]]] = {}
    for row in summary:
        grouped.setdefault((str(row["model"]), str(row["substrate"]), str(row["quant"])), []).append(row)

    output = []
    for (model, substrate, quant), rows in sorted(grouped.items()):
        candidates = [row for row in rows if row["stack"] != "baseline"]
        candidates.sort(
            key=lambda row: (
                float(row["stability_score"]),
                float(row["clean_pass_rate"]),
                float(row["pass_rate"]),
                float(row["mean_score"]),
            ),
            reverse=True,
        )
        output.append({"model": model, "substrate": substrate, "quant": quant, **candidates[0]})
    return output


def lift_rows(summary: list[dict[str, object]]) -> list[dict[str, object]]:
    grouped: dict[tuple[str, str, str], dict[str, dict[str, object]]] = {}
    for row in summary:
        key = (str(row["model"]), str(row["substrate"]), str(row["quant"]))
        grouped.setdefault(key, {})[str(row["stack"])] = row

    output = []
    for (model, substrate, quant), stacks in sorted(grouped.items()):
        base = stacks["baseline"]
        for stack, row in sorted(stacks.items()):
            if stack == "baseline":
                continue
            output.append(
                {
                    "model": model,
                    "substrate": substrate,
                    "quant": quant,
                    "stack": stack,
                    "pass_lift": round(float(row["pass_rate"]) - float(base["pass_rate"]), 4),
                    "clean_pass_lift": round(float(row["clean_pass_rate"]) - float(base["clean_pass_rate"]), 4),
                    "score_lift": round(float(row["mean_score"]) - float(base["mean_score"]), 4),
                    "cascade_delta": round(float(row["cascade_rate"]) - float(base["cascade_rate"]), 4),
                    "echo_delta": round(float(row["echo_rate"]) - float(base["echo_rate"]), 4),
                    "distractor_delta": round(float(row["avg_distractors"]) - float(base["avg_distractors"]), 4),
                }
            )
    return output


def main() -> int:
    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    quant_contexts = install_model_profile_quants()
    iterations = 120
    rng = random.Random(20260605)

    summary_stats: dict[tuple[object, ...], Stats] = {}
    family_stats: dict[tuple[object, ...], Stats] = {}
    pressure_stats: dict[tuple[object, ...], Stats] = {}

    for model_name, quant_state, quant_name in quant_contexts:
        for substrate_name in SUBSTRATES:
            for stack in STACKS:
                for task in TASKS:
                    for _ in range(iterations):
                        row = run_trial(task, stack, substrate_name, quant_name, rng)
                        summary_stats.setdefault((model_name, substrate_name, quant_state, stack.name), Stats()).update(row)
                        family_stats.setdefault((model_name, substrate_name, quant_state, stack.name, task.family), Stats()).update(row)
                        pressure_stats.setdefault(
                            (model_name, substrate_name, quant_state, stack.name, pressure_band(task.pressure)),
                            Stats(),
                        ).update(row)

    summary = flatten_counter(summary_stats, ("model", "substrate", "quant", "stack"))
    family_summary = flatten_counter(family_stats, ("model", "substrate", "quant", "stack", "family"))
    pressure_summary = flatten_counter(pressure_stats, ("model", "substrate", "quant", "stack", "pressure_band"))
    best = best_rows(summary)
    lifts = lift_rows(summary)

    write_csv(REPORT_DIR / "summary.csv", summary)
    write_csv(REPORT_DIR / "family_summary.csv", family_summary)
    write_csv(REPORT_DIR / "pressure_summary.csv", pressure_summary)
    write_csv(REPORT_DIR / "best_by_context.csv", best)
    write_csv(REPORT_DIR / "lift.csv", lifts)

    report = {
        "description": "Dataset-family ablation simulation: individual cognitive/cyber/structural families, pairs, triples, and naive all-fire.",
        "iterations_per_task": iterations,
        "model_profiles": MODEL_PROFILES,
        "stacks": [
            {
                "name": stack.name,
                "weights": stack.weights,
                "routing_quality": stack.routing_quality,
                "compression": stack.compression,
                "description": stack.description,
                "all_fire": stack.all_fire,
            }
            for stack in STACKS
        ],
        "best_by_context": best,
    }
    (REPORT_DIR / "report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")

    print("Dataset family ablation simulation")
    print(f"Model profiles: {len(MODEL_PROFILES)}")
    print(f"Stacks: {len(STACKS)}")
    print(f"Rows simulated: {len(quant_contexts) * len(SUBSTRATES) * len(STACKS) * len(TASKS) * iterations}")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Best stack per context")
    for row in best:
        print(
            f"{row['model']:>12} | {row['substrate']:>17} | {row['quant']:>17} | {row['stack']:<28} "
            f"stable={row['stability_score']:.3f} clean={row['clean_pass_rate']:.3f} "
            f"pass={row['pass_rate']:.3f} echo={row['echo_rate']:.3f} cascade={row['cascade_rate']:.3f}"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
