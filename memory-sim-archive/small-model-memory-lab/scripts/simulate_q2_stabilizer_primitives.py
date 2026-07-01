from __future__ import annotations

import csv
import json
import random
from pathlib import Path

from simulate_primitive_signal_hardening import QUANTS, TASKS, Task, clamp


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "q2-stabilizer-primitives"


SIGNAL_LEVELS = (
    ("strong", 0.46),
    ("very_strong", 0.70),
    ("saturated", 0.95),
    ("overdrive", 1.18),
)


RECIPES = {
    "functional_10": (
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
    ),
    "functional_plus_route": (
        "ROUTE_TASK",
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
    ),
    "functional_plus_guards": (
        "ROUTE_TASK",
        "CHAIN_LOCK",
        "CASCADE_GUARD",
        "DISTRACTOR_SUPPRESS",
        "FINAL_ANSWER_GATE",
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
    ),
    "compact_math_guard": (
        "ROUTE_TASK",
        "CHAIN_LOCK",
        "CASCADE_GUARD",
        "FINAL_ANSWER_GATE",
        "EXTRACT_GIVENS",
        "MAP_RELATION",
        "UNIT_TRACK",
        "STEP_COMPUTE",
        "VERIFY_RESULT",
        "ORIGINAL_TASK_RETURN",
    ),
    "compact_general_guard": (
        "ROUTE_TASK",
        "CHAIN_LOCK",
        "CASCADE_GUARD",
        "DISTRACTOR_SUPPRESS",
        "FINAL_ANSWER_GATE",
        "EXTRACT_GIVENS",
        "SOURCE_BOUNDARY",
        "VERIFY_RESULT",
        "UNCERTAINTY_BOUND",
        "ORIGINAL_TASK_RETURN",
        "DANGER_CHECK",
    ),
    "minimal_stabilized_core": (
        "ROUTE_TASK",
        "CHAIN_LOCK",
        "CASCADE_GUARD",
        "FINAL_ANSWER_GATE",
        "EXTRACT_GIVENS",
        "MAP_RELATION",
        "VERIFY_RESULT",
        "ORIGINAL_TASK_RETURN",
    ),
}


def run_stabilized_trial(task: Task, bundle: tuple[str, ...], signal_strength: float, rng: random.Random) -> dict[str, object]:
    quant = QUANTS["damaged_q2"]
    bundle_set = set(bundle)
    required_set = set(task.required)
    distractor_set = set(task.distractors)

    coverage = len(required_set & bundle_set) / len(required_set)
    route_lock = "ROUTE_TASK" in bundle_set
    chain_lock = "CHAIN_LOCK" in bundle_set
    cascade_guard = "CASCADE_GUARD" in bundle_set
    distractor_suppress = "DISTRACTOR_SUPPRESS" in bundle_set
    final_gate = "FINAL_ANSWER_GATE" in bundle_set

    stabilizer_count = sum([route_lock, chain_lock, cascade_guard, distractor_suppress, final_gate])
    irrelevant = len(bundle_set - required_set) - stabilizer_count
    overhead = 0.004 * (len(bundle) ** 1.18) + 0.010 * max(0, irrelevant - 3)

    noise_resistance = 0.12 + 0.32 * coverage + 0.22 * signal_strength * coverage
    noise_resistance += 0.10 if route_lock else 0.0
    noise_resistance += 0.16 if chain_lock else 0.0
    noise_resistance += 0.20 if cascade_guard else 0.0
    noise_resistance += 0.07 if final_gate else 0.0
    noise_resistance = clamp(noise_resistance, 0.0, 0.985)

    pressure_noise = quant["noise"] * task.pressure * (1.0 - noise_resistance)

    classifier_p = quant["base_classifier"]
    classifier_p += 0.15 * coverage
    classifier_p += 0.18 * signal_strength * coverage
    classifier_p += 0.18 if route_lock else 0.0
    classifier_p += 0.08 if chain_lock else 0.0
    classifier_p += 0.05 if "EXTRACT_GIVENS" in bundle_set else 0.0
    classifier_p -= pressure_noise + overhead
    classifier_p = clamp(classifier_p, 0.0, 0.995)
    classified = rng.random() < classifier_p

    active_required: list[str] = []
    missing_required: list[str] = []
    for primitive in task.required:
        activation_p = quant["base_activation"]
        activation_p += signal_strength if primitive in bundle_set else 0.0
        activation_p += 0.10 * coverage
        activation_p += 0.10 if chain_lock and primitive in bundle_set else 0.0
        activation_p += 0.04 if final_gate and primitive == "VERIFY_RESULT" else 0.0
        activation_p -= pressure_noise + overhead * 0.25
        if not classified:
            activation_p -= 0.11 if route_lock else 0.17
        activation_p = clamp(activation_p, 0.0, 0.995)

        if rng.random() < activation_p:
            active_required.append(primitive)
        else:
            missing_required.append(primitive)

    if chain_lock and missing_required:
        recovered = []
        for primitive in missing_required:
            recover_p = clamp(0.18 + 0.34 * signal_strength * coverage, 0.0, 0.92)
            if rng.random() < recover_p:
                active_required.append(primitive)
                recovered.append(primitive)
        missing_required = [primitive for primitive in missing_required if primitive not in recovered]

    active_distractors = []
    for primitive in task.distractors:
        distractor_p = quant["distractor_pull"] * task.pressure
        distractor_p += 0.16 * signal_strength if primitive in bundle_set else 0.0
        distractor_p -= 0.18 * coverage
        distractor_p -= 0.26 if distractor_suppress else 0.0
        if rng.random() < clamp(distractor_p, 0.0, 0.995):
            active_distractors.append(primitive)

    missing_ratio = len(missing_required) / len(task.required)
    cascade_p = quant["cascade"] * task.pressure
    cascade_p *= 1.0 + missing_ratio * 1.8
    cascade_p *= 1.0 - noise_resistance
    cascade_p -= 0.08 if cascade_guard else 0.0
    cascade_p -= 0.04 if chain_lock and not missing_required else 0.0
    cascaded = rng.random() < clamp(cascade_p, 0.0, 0.995)

    distractor_penalty = min(0.26, 0.07 * len(active_distractors))
    score = len(active_required) / len(task.required)
    score -= 0.16 if not classified else 0.0
    score -= 0.34 if cascaded else 0.0
    score -= distractor_penalty
    score += 0.06 if final_gate and not missing_required else 0.0
    score = clamp(score, 0.0, 1.0)

    passed = score >= 0.76 and not cascaded and len(missing_required) <= 1
    return {
        "task_id": task.task_id,
        "passed": passed,
        "score": score,
        "classified": classified,
        "cascaded": cascaded,
        "missing_count": len(missing_required),
        "distractors": len(active_distractors),
    }


def evaluate(recipe_name: str, bundle: tuple[str, ...], signal_name: str, signal_strength: float) -> dict[str, object]:
    rng = random.Random(20260604 + sum(ord(ch) for ch in recipe_name + signal_name))
    iterations = 500
    rows = []
    for _ in range(iterations):
        for task in TASKS:
            rows.append(run_stabilized_trial(task, bundle, signal_strength, rng))

    per_task = {}
    for task in TASKS:
        task_rows = [row for row in rows if row["task_id"] == task.task_id]
        per_task[task.task_id] = round(sum(1 for row in task_rows if row["passed"]) / len(task_rows), 4)

    return {
        "recipe": recipe_name,
        "signal": signal_name,
        "signal_strength": signal_strength,
        "pass_rate": round(sum(1 for row in rows if row["passed"]) / len(rows), 4),
        "mean_score": round(sum(float(row["score"]) for row in rows) / len(rows), 4),
        "classified_rate": round(sum(1 for row in rows if row["classified"]) / len(rows), 4),
        "cascade_rate": round(sum(1 for row in rows if row["cascaded"]) / len(rows), 4),
        "avg_missing": round(sum(int(row["missing_count"]) for row in rows) / len(rows), 4),
        "avg_distractors": round(sum(int(row["distractors"]) for row in rows) / len(rows), 4),
        "worst_task_rate": min(per_task.values()),
        "bundle": "+".join(bundle),
        "per_task": per_task,
    }


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    visible_rows = [{k: v for k, v in row.items() if k != "per_task"} for row in rows]
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(visible_rows[0].keys()))
        writer.writeheader()
        writer.writerows(visible_rows)


def main() -> None:
    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    rows = []
    for recipe_name, bundle in RECIPES.items():
        for signal_name, signal_strength in SIGNAL_LEVELS:
            rows.append(evaluate(recipe_name, bundle, signal_name, signal_strength))

    rows.sort(
        key=lambda row: (
            float(row["pass_rate"]),
            float(row["worst_task_rate"]),
            float(row["mean_score"]),
        ),
        reverse=True,
    )

    write_csv(REPORT_DIR / "stabilizer_results.csv", rows)
    (REPORT_DIR / "stabilizer_results.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")

    print("Q2 stabilizer primitive simulation")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Top results")
    for index, row in enumerate(rows[:12], start=1):
        print(
            f"{index:02d}. {row['recipe']:<24} {row['signal']:<11} "
            f"pass={row['pass_rate']:.3f} worst={row['worst_task_rate']:.3f} "
            f"score={row['mean_score']:.3f} classified={row['classified_rate']:.3f} "
            f"cascade={row['cascade_rate']:.3f} missing={row['avg_missing']:.3f}"
        )


if __name__ == "__main__":
    main()
