from __future__ import annotations

import csv
import json
import math
import random
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "primitive-signal-sim"


@dataclass(frozen=True)
class Task:
    task_id: str
    family: str
    required: tuple[str, ...]
    distractors: tuple[str, ...]
    pressure: float


@dataclass(frozen=True)
class Mode:
    name: str
    classifier_boost: float
    primitive_boost: float
    noise_resistance: float
    distractor_resistance: float
    overhead: float


PRIMITIVES = (
    "EXTRACT_GIVENS",
    "SOURCE_BOUNDARY",
    "MAP_RELATION",
    "UNIT_TRACK",
    "STEP_COMPUTE",
    "VERIFY_RESULT",
    "COMPARE_AXES",
    "UNCERTAINTY_BOUND",
    "ORIGINAL_TASK_RETURN",
    "DANGER_CHECK",
)


TASKS = (
    Task(
        "rate_printer",
        "math",
        ("EXTRACT_GIVENS", "MAP_RELATION", "UNIT_TRACK", "STEP_COMPUTE", "VERIFY_RESULT"),
        ("COMPARE_AXES", "SOURCE_BOUNDARY"),
        0.58,
    ),
    Task(
        "ratio_mixture",
        "math",
        ("EXTRACT_GIVENS", "MAP_RELATION", "STEP_COMPUTE", "VERIFY_RESULT"),
        ("DANGER_CHECK", "ORIGINAL_TASK_RETURN"),
        0.62,
    ),
    Task(
        "probability_without_replacement",
        "math",
        ("EXTRACT_GIVENS", "MAP_RELATION", "STEP_COMPUTE", "VERIFY_RESULT", "UNCERTAINTY_BOUND"),
        ("COMPARE_AXES", "SOURCE_BOUNDARY"),
        0.78,
    ),
    Task(
        "source_injection_summary",
        "boundary",
        ("SOURCE_BOUNDARY", "ORIGINAL_TASK_RETURN", "DANGER_CHECK", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "UNIT_TRACK"),
        0.84,
    ),
    Task(
        "loaded_argument",
        "rhetoric",
        ("EXTRACT_GIVENS", "COMPARE_AXES", "UNCERTAINTY_BOUND", "DANGER_CHECK"),
        ("UNIT_TRACK", "STEP_COMPUTE"),
        0.71,
    ),
    Task(
        "spatial_exit",
        "spatial",
        ("EXTRACT_GIVENS", "MAP_RELATION", "DANGER_CHECK", "VERIFY_RESULT"),
        ("COMPARE_AXES", "UNIT_TRACK"),
        0.67,
    ),
    Task(
        "ambiguous_report",
        "generalist",
        ("EXTRACT_GIVENS", "SOURCE_BOUNDARY", "UNCERTAINTY_BOUND", "VERIFY_RESULT"),
        ("STEP_COMPUTE", "UNIT_TRACK"),
        0.69,
    ),
    Task(
        "multi_step_schedule",
        "math",
        ("EXTRACT_GIVENS", "MAP_RELATION", "UNIT_TRACK", "STEP_COMPUTE", "VERIFY_RESULT"),
        ("SOURCE_BOUNDARY", "COMPARE_AXES"),
        0.74,
    ),
)


MODES = (
    Mode("baseline", 0.00, 0.00, 0.00, 0.00, 0.00),
    Mode("memories_only", 0.16, 0.08, 0.10, 0.05, 0.03),
    Mode("primitives_only", 0.07, 0.24, 0.22, 0.18, 0.02),
    Mode("memories_plus_primitives", 0.21, 0.30, 0.28, 0.22, 0.06),
)


QUANTS = {
    "healthy_q6": {
        "base_classifier": 0.86,
        "base_activation": 0.88,
        "noise": 0.10,
        "cascade": 0.08,
        "distractor_pull": 0.09,
    },
    "damaged_q3": {
        "base_classifier": 0.72,
        "base_activation": 0.70,
        "noise": 0.24,
        "cascade": 0.19,
        "distractor_pull": 0.19,
    },
    "damaged_q2": {
        "base_classifier": 0.58,
        "base_activation": 0.54,
        "noise": 0.38,
        "cascade": 0.31,
        "distractor_pull": 0.31,
    },
}


def clamp(value: float, low: float = 0.0, high: float = 0.99) -> float:
    return max(low, min(high, value))


def run_one(task: Task, mode: Mode, quant: dict[str, float], rng: random.Random) -> dict[str, object]:
    pressure_noise = quant["noise"] * task.pressure * (1.0 - mode.noise_resistance)
    classifier_p = clamp(quant["base_classifier"] + mode.classifier_boost - pressure_noise - mode.overhead)
    classified = rng.random() < classifier_p

    active_required: list[str] = []
    missing_required: list[str] = []
    active_distractors: list[str] = []

    for primitive in task.required:
        activation_p = quant["base_activation"] + mode.primitive_boost
        activation_p -= quant["noise"] * task.pressure * (1.0 - mode.noise_resistance)
        activation_p -= mode.overhead * 0.5
        if not classified:
            activation_p -= 0.18
        activation_p = clamp(activation_p)

        if rng.random() < activation_p:
            active_required.append(primitive)
        else:
            missing_required.append(primitive)

    for primitive in task.distractors:
        distractor_p = quant["distractor_pull"] * task.pressure * (1.0 - mode.distractor_resistance)
        if rng.random() < clamp(distractor_p):
            active_distractors.append(primitive)

    missing_ratio = len(missing_required) / len(task.required)
    cascade_p = quant["cascade"] * task.pressure * (1.0 + missing_ratio * 1.8)
    cascade_p *= 1.0 - mode.noise_resistance
    cascaded = rng.random() < clamp(cascade_p)

    distractor_penalty = min(0.24, 0.08 * len(active_distractors))
    coverage = len(active_required) / len(task.required)
    score = coverage
    score -= 0.25 if not classified else 0.0
    score -= 0.35 if cascaded else 0.0
    score -= distractor_penalty
    score = clamp(score, 0.0, 1.0)

    passed = score >= 0.76 and not cascaded and len(missing_required) <= 1
    return {
        "task_id": task.task_id,
        "family": task.family,
        "score": score,
        "passed": passed,
        "classified": classified,
        "cascaded": cascaded,
        "required_active": len(active_required),
        "required_total": len(task.required),
        "distractors_active": len(active_distractors),
        "missing": ",".join(missing_required),
        "distractors": ",".join(active_distractors),
    }


def summarize(rows: list[dict[str, object]]) -> list[dict[str, object]]:
    groups: dict[tuple[str, str], list[dict[str, object]]] = {}
    for row in rows:
        groups.setdefault((str(row["quant"]), str(row["mode"])), []).append(row)

    output = []
    for (quant, mode), group in sorted(groups.items()):
        pass_rate = sum(1 for row in group if row["passed"]) / len(group)
        mean_score = sum(float(row["score"]) for row in group) / len(group)
        cascade_rate = sum(1 for row in group if row["cascaded"]) / len(group)
        distractor_rate = sum(int(row["distractors_active"]) for row in group) / len(group)
        classified_rate = sum(1 for row in group if row["classified"]) / len(group)
        output.append(
            {
                "quant": quant,
                "mode": mode,
                "trials": len(group),
                "pass_rate": round(pass_rate, 4),
                "mean_score": round(mean_score, 4),
                "classified_rate": round(classified_rate, 4),
                "cascade_rate": round(cascade_rate, 4),
                "avg_distractors": round(distractor_rate, 4),
            }
        )
    return output


def lift(summary: list[dict[str, object]]) -> list[dict[str, object]]:
    by_quant: dict[str, dict[str, dict[str, object]]] = {}
    for row in summary:
        by_quant.setdefault(str(row["quant"]), {})[str(row["mode"])] = row

    output = []
    for quant, modes in sorted(by_quant.items()):
        base = modes["baseline"]
        base_pass = float(base["pass_rate"])
        base_score = float(base["mean_score"])
        for mode_name, row in sorted(modes.items()):
            if mode_name == "baseline":
                continue
            output.append(
                {
                    "quant": quant,
                    "mode": mode_name,
                    "pass_lift": round(float(row["pass_rate"]) - base_pass, 4),
                    "score_lift": round(float(row["mean_score"]) - base_score, 4),
                    "cascade_delta": round(float(row["cascade_rate"]) - float(base["cascade_rate"]), 4),
                }
            )
    return output


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        return
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    rng = random.Random(20260604)
    REPORT_DIR.mkdir(parents=True, exist_ok=True)

    rows: list[dict[str, object]] = []
    iterations = 500
    for quant_name, quant in QUANTS.items():
        for mode in MODES:
            for _ in range(iterations):
                for task in TASKS:
                    row = run_one(task, mode, quant, rng)
                    row["quant"] = quant_name
                    row["mode"] = mode.name
                    rows.append(row)

    summary = summarize(rows)
    lift_rows = lift(summary)

    write_csv(REPORT_DIR / "trial_rows.csv", rows)
    write_csv(REPORT_DIR / "summary.csv", summary)
    write_csv(REPORT_DIR / "lift.csv", lift_rows)

    report = {
        "description": "Simulation of primitive signal hardening under quantization-like activation noise.",
        "iterations_per_task": iterations,
        "tasks": [task.__dict__ for task in TASKS],
        "primitives": PRIMITIVES,
        "summary": summary,
        "lift": lift_rows,
    }
    (REPORT_DIR / "report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")

    print("Primitive signal hardening simulation")
    print(f"Rows: {len(rows)}")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Summary")
    for row in summary:
        print(
            f"{row['quant']:>11} | {row['mode']:<24} "
            f"pass={row['pass_rate']:.3f} score={row['mean_score']:.3f} "
            f"cascade={row['cascade_rate']:.3f} distractors={row['avg_distractors']:.3f}"
        )
    print()
    print("Lift over baseline")
    for row in lift_rows:
        print(
            f"{row['quant']:>11} | {row['mode']:<24} "
            f"pass_lift={row['pass_lift']:+.3f} score_lift={row['score_lift']:+.3f} "
            f"cascade_delta={row['cascade_delta']:+.3f}"
        )


if __name__ == "__main__":
    main()
