from __future__ import annotations

import csv
import itertools
import json
import random
from pathlib import Path

from simulate_primitive_signal_hardening import PRIMITIVES, QUANTS, TASKS, Task, clamp


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "q2-primitive-combo-sweep"


SIGNAL_LEVELS = (
    ("standard", 0.24),
    ("strong", 0.46),
    ("very_strong", 0.70),
    ("saturated", 0.95),
)


def run_bundle_trial(
    task: Task,
    bundle: tuple[str, ...],
    signal_strength: float,
    rng: random.Random,
) -> dict[str, object]:
    quant = QUANTS["damaged_q2"]
    bundle_set = set(bundle)
    required_set = set(task.required)
    distractor_set = set(task.distractors)

    coverage = len(required_set & bundle_set) / len(required_set)
    irrelevant = len(bundle_set - required_set)
    distractor_overlap = len(bundle_set & distractor_set)

    extract_bonus = 0.05 if "EXTRACT_GIVENS" in bundle_set else 0.0
    verify_bonus = 0.04 if "VERIFY_RESULT" in bundle_set else 0.0
    return_bonus = 0.04 if "ORIGINAL_TASK_RETURN" in bundle_set else 0.0
    danger_bonus = 0.04 if "DANGER_CHECK" in bundle_set else 0.0
    boundary_bonus = 0.05 if task.family == "boundary" and "SOURCE_BOUNDARY" in bundle_set else 0.0

    size = len(bundle)
    overhead = 0.006 * (size**1.25) + 0.018 * max(0, irrelevant - 2)

    noise_resistance = 0.10
    noise_resistance += 0.30 * coverage
    noise_resistance += 0.22 * signal_strength * coverage
    noise_resistance += verify_bonus + return_bonus + danger_bonus
    noise_resistance -= 0.035 * distractor_overlap
    noise_resistance = clamp(noise_resistance, 0.0, 0.92)

    pressure_noise = quant["noise"] * task.pressure * (1.0 - noise_resistance)
    classifier_p = quant["base_classifier"]
    classifier_p += 0.16 * coverage
    classifier_p += 0.18 * signal_strength * coverage
    classifier_p += extract_bonus + verify_bonus + return_bonus + danger_bonus + boundary_bonus
    classifier_p -= pressure_noise + overhead
    classifier_p = clamp(classifier_p, 0.0, 0.995)
    classified = rng.random() < classifier_p

    active_required: list[str] = []
    missing_required: list[str] = []
    for primitive in task.required:
        in_bundle = primitive in bundle_set
        activation_p = quant["base_activation"]
        activation_p += signal_strength if in_bundle else 0.0
        activation_p += 0.08 * coverage
        activation_p += 0.04 if primitive == "VERIFY_RESULT" and verify_bonus else 0.0
        activation_p -= pressure_noise
        activation_p -= overhead * 0.35
        if not classified:
            activation_p -= 0.16
        activation_p = clamp(activation_p, 0.0, 0.995)

        if rng.random() < activation_p:
            active_required.append(primitive)
        else:
            missing_required.append(primitive)

    active_distractors: list[str] = []
    for primitive in task.distractors:
        distractor_p = quant["distractor_pull"] * task.pressure
        distractor_p += 0.22 * signal_strength if primitive in bundle_set else 0.0
        distractor_p -= 0.14 * coverage
        distractor_p += 0.025 * irrelevant
        if rng.random() < clamp(distractor_p):
            active_distractors.append(primitive)

    missing_ratio = len(missing_required) / len(task.required)
    cascade_p = quant["cascade"] * task.pressure
    cascade_p *= 1.0 + missing_ratio * 1.8
    cascade_p *= 1.0 - noise_resistance
    cascade_p += 0.018 * distractor_overlap
    cascaded = rng.random() < clamp(cascade_p, 0.0, 0.995)

    distractor_penalty = min(0.30, 0.08 * len(active_distractors))
    score = len(active_required) / len(task.required)
    score -= 0.22 if not classified else 0.0
    score -= 0.36 if cascaded else 0.0
    score -= distractor_penalty
    score = clamp(score, 0.0, 1.0)

    passed = score >= 0.76 and not cascaded and len(missing_required) <= 1
    return {
        "task_id": task.task_id,
        "passed": passed,
        "score": score,
        "classified": classified,
        "cascaded": cascaded,
        "active_required": len(active_required),
        "required_total": len(task.required),
        "active_distractors": len(active_distractors),
        "missing": ",".join(missing_required),
    }


def evaluate_bundle(
    bundle: tuple[str, ...],
    signal_name: str,
    signal_strength: float,
    iterations: int,
) -> dict[str, object]:
    seed_text = "|".join((signal_name, *bundle))
    rng = random.Random(20260604 + sum(ord(ch) for ch in seed_text))

    rows = []
    for _ in range(iterations):
        for task in TASKS:
            rows.append(run_bundle_trial(task, bundle, signal_strength, rng))

    pass_rate = sum(1 for row in rows if row["passed"]) / len(rows)
    mean_score = sum(float(row["score"]) for row in rows) / len(rows)
    cascade_rate = sum(1 for row in rows if row["cascaded"]) / len(rows)
    classified_rate = sum(1 for row in rows if row["classified"]) / len(rows)
    avg_distractors = sum(int(row["active_distractors"]) for row in rows) / len(rows)

    per_task = {}
    for task in TASKS:
        task_rows = [row for row in rows if row["task_id"] == task.task_id]
        per_task[task.task_id] = round(sum(1 for row in task_rows if row["passed"]) / len(task_rows), 4)

    return {
        "signal": signal_name,
        "signal_strength": signal_strength,
        "bundle_size": len(bundle),
        "bundle": "+".join(bundle),
        "pass_rate": round(pass_rate, 4),
        "mean_score": round(mean_score, 4),
        "classified_rate": round(classified_rate, 4),
        "cascade_rate": round(cascade_rate, 4),
        "avg_distractors": round(avg_distractors, 4),
        "worst_task_rate": min(per_task.values()),
        "per_task": per_task,
    }


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        return
    visible_rows = [{k: v for k, v in row.items() if k != "per_task"} for row in rows]
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(visible_rows[0].keys()))
        writer.writeheader()
        writer.writerows(visible_rows)


def main() -> None:
    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    iterations = 180

    results: list[dict[str, object]] = []
    for signal_name, signal_strength in SIGNAL_LEVELS:
        for size in range(2, len(PRIMITIVES) + 1):
            for bundle in itertools.combinations(PRIMITIVES, size):
                results.append(evaluate_bundle(bundle, signal_name, signal_strength, iterations))

    results.sort(
        key=lambda row: (
            float(row["pass_rate"]),
            float(row["worst_task_rate"]),
            float(row["mean_score"]),
            -int(row["bundle_size"]),
        ),
        reverse=True,
    )

    top = results[:40]
    write_csv(REPORT_DIR / "all_combo_results.csv", results)
    write_csv(REPORT_DIR / "top_40.csv", top)
    (REPORT_DIR / "top_40.json").write_text(json.dumps(top, indent=2), encoding="utf-8")

    by_signal = {}
    for signal_name, _ in SIGNAL_LEVELS:
        signal_rows = [row for row in results if row["signal"] == signal_name]
        by_signal[signal_name] = max(
            signal_rows,
            key=lambda row: (
                float(row["pass_rate"]),
                float(row["worst_task_rate"]),
                float(row["mean_score"]),
                -int(row["bundle_size"]),
            ),
        )
    (REPORT_DIR / "best_by_signal.json").write_text(json.dumps(by_signal, indent=2), encoding="utf-8")
    write_csv(REPORT_DIR / "best_by_signal.csv", list(by_signal.values()))

    print("Q2 primitive-only combo sweep")
    print(f"Combinations tested: {len(results)}")
    print(f"Iterations per task per combo: {iterations}")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Best by signal level")
    for signal_name, row in by_signal.items():
        print(
            f"{signal_name:<12} pass={row['pass_rate']:.3f} "
            f"worst_task={row['worst_task_rate']:.3f} "
            f"score={row['mean_score']:.3f} "
            f"cascade={row['cascade_rate']:.3f} "
            f"size={row['bundle_size']} "
            f"bundle={row['bundle']}"
        )
    print()
    print("Top 10 overall")
    for index, row in enumerate(top[:10], start=1):
        print(
            f"{index:02d}. {row['signal']:<12} pass={row['pass_rate']:.3f} "
            f"worst={row['worst_task_rate']:.3f} size={row['bundle_size']} "
            f"{row['bundle']}"
        )


if __name__ == "__main__":
    main()
