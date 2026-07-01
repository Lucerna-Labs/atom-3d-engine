from __future__ import annotations

import csv
import json
import random
from pathlib import Path

from simulate_primitive_signal_hardening import TASKS, clamp
from sweep_q2_primitive_combinations import run_bundle_trial


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "q2-primitive-chain-gating"


SIGNAL_LEVELS = (
    ("strong", 0.46),
    ("very_strong", 0.70),
    ("saturated", 0.95),
    ("overdrive", 1.18),
)


RECIPES = {
    "flat_best_10": {
        "default": (
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
    },
    "family_minimal": {
        "math": ("EXTRACT_GIVENS", "MAP_RELATION", "UNIT_TRACK", "STEP_COMPUTE", "VERIFY_RESULT"),
        "boundary": ("SOURCE_BOUNDARY", "ORIGINAL_TASK_RETURN", "DANGER_CHECK", "VERIFY_RESULT"),
        "rhetoric": ("EXTRACT_GIVENS", "COMPARE_AXES", "UNCERTAINTY_BOUND", "DANGER_CHECK"),
        "spatial": ("EXTRACT_GIVENS", "MAP_RELATION", "DANGER_CHECK", "VERIFY_RESULT"),
        "generalist": ("EXTRACT_GIVENS", "SOURCE_BOUNDARY", "UNCERTAINTY_BOUND", "VERIFY_RESULT"),
    },
    "family_with_anchor": {
        "math": (
            "EXTRACT_GIVENS",
            "MAP_RELATION",
            "UNIT_TRACK",
            "STEP_COMPUTE",
            "VERIFY_RESULT",
            "UNCERTAINTY_BOUND",
        ),
        "boundary": (
            "EXTRACT_GIVENS",
            "SOURCE_BOUNDARY",
            "ORIGINAL_TASK_RETURN",
            "DANGER_CHECK",
            "VERIFY_RESULT",
        ),
        "rhetoric": (
            "EXTRACT_GIVENS",
            "SOURCE_BOUNDARY",
            "COMPARE_AXES",
            "UNCERTAINTY_BOUND",
            "DANGER_CHECK",
            "VERIFY_RESULT",
        ),
        "spatial": (
            "EXTRACT_GIVENS",
            "MAP_RELATION",
            "DANGER_CHECK",
            "VERIFY_RESULT",
            "ORIGINAL_TASK_RETURN",
        ),
        "generalist": (
            "EXTRACT_GIVENS",
            "SOURCE_BOUNDARY",
            "UNCERTAINTY_BOUND",
            "VERIFY_RESULT",
            "ORIGINAL_TASK_RETURN",
        ),
    },
    "chain_locked": {
        "math": (
            "EXTRACT_GIVENS",
            "MAP_RELATION",
            "UNIT_TRACK",
            "STEP_COMPUTE",
            "VERIFY_RESULT",
            "ORIGINAL_TASK_RETURN",
        ),
        "boundary": (
            "SOURCE_BOUNDARY",
            "ORIGINAL_TASK_RETURN",
            "DANGER_CHECK",
            "VERIFY_RESULT",
            "EXTRACT_GIVENS",
        ),
        "rhetoric": (
            "EXTRACT_GIVENS",
            "COMPARE_AXES",
            "UNCERTAINTY_BOUND",
            "DANGER_CHECK",
            "ORIGINAL_TASK_RETURN",
        ),
        "spatial": (
            "EXTRACT_GIVENS",
            "MAP_RELATION",
            "DANGER_CHECK",
            "VERIFY_RESULT",
            "ORIGINAL_TASK_RETURN",
        ),
        "generalist": (
            "EXTRACT_GIVENS",
            "SOURCE_BOUNDARY",
            "UNCERTAINTY_BOUND",
            "VERIFY_RESULT",
            "ORIGINAL_TASK_RETURN",
        ),
    },
}


def bundle_for(recipe: dict[str, tuple[str, ...]], family: str) -> tuple[str, ...]:
    return recipe.get(family, recipe.get("default", ()))


def wrong_bundle(recipe: dict[str, tuple[str, ...]], family: str, rng: random.Random) -> tuple[str, ...]:
    choices = [key for key in recipe if key != family and key != "default"]
    if not choices:
        return recipe["default"]
    return recipe[rng.choice(choices)]


def evaluate(recipe_name: str, recipe: dict[str, tuple[str, ...]], signal_name: str, signal_strength: float) -> dict[str, object]:
    rng = random.Random(20260604 + sum(ord(ch) for ch in recipe_name + signal_name))
    iterations = 320
    rows = []

    for _ in range(iterations):
        for task in TASKS:
            if "default" in recipe:
                route_correct = True
            else:
                route_p = 0.52 + 0.34 * signal_strength
                route_p -= 0.16 * task.pressure
                route_p = clamp(route_p, 0.0, 0.995)
                route_correct = rng.random() < route_p

            bundle = bundle_for(recipe, task.family) if route_correct else wrong_bundle(recipe, task.family, rng)
            row = run_bundle_trial(task, bundle, signal_strength, rng)
            row["route_correct"] = route_correct
            rows.append(row)

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
        "route_rate": round(sum(1 for row in rows if row["route_correct"]) / len(rows), 4),
        "cascade_rate": round(sum(1 for row in rows if row["cascaded"]) / len(rows), 4),
        "worst_task_rate": min(per_task.values()),
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
    for recipe_name, recipe in RECIPES.items():
        for signal_name, signal_strength in SIGNAL_LEVELS:
            rows.append(evaluate(recipe_name, recipe, signal_name, signal_strength))

    rows.sort(
        key=lambda row: (
            float(row["pass_rate"]),
            float(row["worst_task_rate"]),
            float(row["mean_score"]),
        ),
        reverse=True,
    )

    write_csv(REPORT_DIR / "chain_gating_results.csv", rows)
    (REPORT_DIR / "chain_gating_results.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")

    print("Q2 primitive-only associated-chain simulation")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Top results")
    for index, row in enumerate(rows[:10], start=1):
        print(
            f"{index:02d}. {row['recipe']:<18} {row['signal']:<11} "
            f"pass={row['pass_rate']:.3f} worst={row['worst_task_rate']:.3f} "
            f"score={row['mean_score']:.3f} route={row['route_rate']:.3f} "
            f"cascade={row['cascade_rate']:.3f}"
        )


if __name__ == "__main__":
    main()
