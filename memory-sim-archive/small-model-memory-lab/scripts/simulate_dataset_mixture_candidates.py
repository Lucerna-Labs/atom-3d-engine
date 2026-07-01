from __future__ import annotations

import argparse
import csv
import json
import random
from pathlib import Path

from simulate_exhaustive_stack_sweep import (
    QUANTS,
    SUBSTRATES,
    TASKS,
    StackConfig,
    Stats,
    flatten_counter,
    pressure_band,
    run_trial,
)


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_CANDIDATE_DIR = ROOT / "data" / "dataset_mixture_candidates_v0_1"
DEFAULT_REPORT_DIR = ROOT / "runs" / "dataset-mixture-candidate-sim"


BASELINE_QUANTS = {
    "base_f16": {
        "base_classifier": 0.94,
        "base_activation": 0.94,
        "noise": 0.035,
        "cascade": 0.025,
        "distractor_pull": 0.045,
    },
    "q8_k_m": {
        "base_classifier": 0.91,
        "base_activation": 0.915,
        "noise": 0.055,
        "cascade": 0.040,
        "distractor_pull": 0.060,
    },
    "q1_k_m": {
        "base_classifier": 0.36,
        "base_activation": 0.30,
        "noise": 0.62,
        "cascade": 0.56,
        "distractor_pull": 0.46,
    },
}


MODEL_PROFILES = {
    "2b_reference": {
        "description": "Current small-model reference profile.",
        "classifier_delta": 0.00,
        "activation_delta": 0.00,
        "noise_scale": 1.00,
        "cascade_scale": 1.00,
        "distractor_scale": 1.00,
    },
    "4b": {
        "description": "Mid-small dense profile: stronger than 2B but still sensitive to damaged quantization.",
        "classifier_delta": 0.04,
        "activation_delta": 0.04,
        "noise_scale": 0.90,
        "cascade_scale": 0.88,
        "distractor_scale": 0.92,
    },
    "9b": {
        "description": "Larger 9B profile: stronger base signal, lower quantization-like noise, less cascade.",
        "classifier_delta": 0.08,
        "activation_delta": 0.07,
        "noise_scale": 0.78,
        "cascade_scale": 0.74,
        "distractor_scale": 0.82,
    },
    "27b": {
        "description": "Large dense profile: stronger classifier/activation signal and meaningfully reduced cascade.",
        "classifier_delta": 0.13,
        "activation_delta": 0.12,
        "noise_scale": 0.62,
        "cascade_scale": 0.58,
        "distractor_scale": 0.68,
    },
    "35b_a3b_moe": {
        "description": "MoE profile with 35B total / ~A3B active behavior: resilient routing and lower cascade under compression, but not as dense-active as a full 35B.",
        "classifier_delta": 0.11,
        "activation_delta": 0.09,
        "noise_scale": 0.55,
        "cascade_scale": 0.50,
        "distractor_scale": 0.62,
    },
}


def install_model_profile_quants() -> list[tuple[str, str, str]]:
    for quant_name, values in BASELINE_QUANTS.items():
        QUANTS[quant_name] = dict(values)
    original = {
        name: dict(values)
        for name, values in QUANTS.items()
        if not name.startswith(("2b_reference__", "4b__", "9b__", "27b__", "35b_a3b_moe__"))
    }
    contexts = []
    for model_name, profile in MODEL_PROFILES.items():
        for quant_name, values in original.items():
            expanded_name = f"{model_name}__{quant_name}"
            QUANTS[expanded_name] = {
                "base_classifier": min(0.97, values["base_classifier"] + profile["classifier_delta"]),
                "base_activation": min(0.97, values["base_activation"] + profile["activation_delta"]),
                "noise": values["noise"] * profile["noise_scale"],
                "cascade": values["cascade"] * profile["cascade_scale"],
                "distractor_pull": values["distractor_pull"] * profile["distractor_scale"],
            }
            contexts.append((model_name, quant_name, expanded_name))
    return contexts


def load_manifest(path: Path) -> dict[str, object]:
    return json.loads(path.read_text(encoding="utf-8"))


def score_candidate_shape(candidate: dict[str, object]) -> tuple[float, float, bool]:
    summary = candidate["summary"]
    by_kind = summary["by_kind"]
    count = float(summary["count"])
    bridge_rate = float(by_kind.get("routed_bridge", 0)) / count
    anti_echo_rate = float(by_kind.get("anti_echo_stop", 0)) / count
    gate_rate = (
        float(by_kind.get("gate_control", 0))
        + float(by_kind.get("gate_application", 0))
        + float(by_kind.get("carrier_control", 0))
    ) / count
    q2 = bool(candidate.get("q2_amplifier", False))

    routing_quality = 0.72 + bridge_rate * 1.15 + gate_rate * 0.55
    compression = 0.70 + anti_echo_rate * 0.80 + gate_rate * 0.45
    if q2:
        routing_quality -= 0.08
        compression -= 0.05
    routing_quality = max(0.45, min(0.96, routing_quality))
    compression = max(0.45, min(0.96, compression))
    return routing_quality, compression, q2


def stack_from_candidate(candidate: dict[str, object]) -> StackConfig:
    shares = candidate["shares"]
    weights = {
        "cognitive": float(shares.get("cognitive", 0.0)),
        "cyber": float(shares.get("cyber", 0.0)),
        "structural": float(shares.get("structural", 0.0)),
    }
    weights = {key: value for key, value in weights.items() if value > 0.0}
    routing_quality, compression, q2 = score_candidate_shape(candidate)
    return StackConfig(
        name=str(candidate["name"]),
        weights=weights,
        routing_quality=routing_quality,
        compression=compression,
        description=str(candidate["description"]),
        all_fire=q2,
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
        rows.sort(
            key=lambda row: (
                float(row["stability_score"]),
                float(row["clean_pass_rate"]),
                float(row["pass_rate"]),
                float(row["mean_score"]),
            ),
            reverse=True,
        )
        output.append({"model": model, "substrate": substrate, "quant": quant, **rows[0]})
    return output


def candidate_lift_rows(summary: list[dict[str, object]]) -> list[dict[str, object]]:
    grouped: dict[tuple[str, str, str], dict[str, dict[str, object]]] = {}
    for row in summary:
        key = (str(row["model"]), str(row["substrate"]), str(row["quant"]))
        grouped.setdefault(key, {})[str(row["stack"])] = row

    output = []
    for (model, substrate, quant), stacks in sorted(grouped.items()):
        base = stacks.get("baseline")
        if base is None:
            continue
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
    parser = argparse.ArgumentParser(description="Simulate concrete dataset mixture candidates.")
    parser.add_argument("--candidate-dir", type=Path, default=DEFAULT_CANDIDATE_DIR)
    parser.add_argument("--report-dir", type=Path, default=DEFAULT_REPORT_DIR)
    args = parser.parse_args()

    candidate_dir = args.candidate_dir
    report_dir = args.report_dir
    report_dir.mkdir(parents=True, exist_ok=True)
    root_manifest = load_manifest(candidate_dir / "manifest.json")
    candidates = list(root_manifest["candidates"])
    stacks = [stack_from_candidate(candidate) for candidate in candidates]
    quant_contexts = install_model_profile_quants()
    iterations = 140
    rng = random.Random(20260605)

    summary_stats: dict[tuple[object, ...], Stats] = {}
    family_stats: dict[tuple[object, ...], Stats] = {}
    pressure_stats: dict[tuple[object, ...], Stats] = {}

    for substrate_name in SUBSTRATES:
        for model_name, quant_state, quant_name in quant_contexts:
            for stack in stacks:
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
    lift = candidate_lift_rows(
        [
            *summary,
            # Lift requires a baseline row. Use zero-ish explicit rows so the file remains meaningful for candidates.
            *[
                {
                    "model": model_name,
                    "substrate": substrate,
                    "quant": quant_state,
                    "stack": "baseline",
                    "pass_rate": 0.0,
                    "clean_pass_rate": 0.0,
                    "mean_score": 0.0,
                    "cascade_rate": 0.0,
                    "echo_rate": 0.0,
                    "avg_distractors": 0.0,
                }
                for substrate in SUBSTRATES
                for model_name, quant_state, _ in quant_contexts
            ],
        ]
    )

    write_csv(report_dir / "summary.csv", summary)
    write_csv(report_dir / "family_summary.csv", family_summary)
    write_csv(report_dir / "pressure_summary.csv", pressure_summary)
    write_csv(report_dir / "best_by_context.csv", best)
    write_csv(report_dir / "lift_from_zero_baseline.csv", lift)

    report = {
        "description": "Simulation of concrete dataset mixture candidates built in data/dataset_mixture_candidates_v0_1.",
        "iterations_per_task": iterations,
        "candidate_manifest": str(candidate_dir / "manifest.json"),
        "model_profiles": MODEL_PROFILES,
        "candidates": candidates,
        "stack_parameters": [
            {
                "name": stack.name,
                "weights": stack.weights,
                "routing_quality": stack.routing_quality,
                "compression": stack.compression,
                "all_fire": stack.all_fire,
            }
            for stack in stacks
        ],
        "best_by_context": best,
    }
    (report_dir / "report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")

    print("Dataset mixture candidate simulation")
    print(f"Candidates: {len(stacks)}")
    print(f"Model profiles: {len(MODEL_PROFILES)}")
    print(f"Rows simulated: {len(SUBSTRATES) * len(quant_contexts) * len(stacks) * len(TASKS) * iterations}")
    print(f"Reports: {report_dir}")
    print()
    print("Best candidate per context")
    for row in best:
        print(
            f"{row['model']:>12} | {row['substrate']:>17} | {row['quant']:>17} | {row['stack']:<32} "
            f"stable={row['stability_score']:.3f} clean={row['clean_pass_rate']:.3f} "
            f"pass={row['pass_rate']:.3f} echo={row['echo_rate']:.3f} cascade={row['cascade_rate']:.3f}"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
