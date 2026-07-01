from __future__ import annotations

import csv
import json
import random
from dataclasses import dataclass
from pathlib import Path

from simulate_primitive_signal_hardening import QUANTS, TASKS, Task, clamp


ROOT = Path(__file__).resolve().parents[1]
CORPUS_DIR = ROOT / "corpus"
REPORT_DIR = ROOT / "runs" / "three-dataset-stack-sim"


@dataclass(frozen=True)
class DatasetPack:
    name: str
    description: str
    source_files: tuple[str, ...]
    classifier_boost: float
    primitive_boost: float
    noise_resistance: float
    distractor_resistance: float
    cascade_resistance: float
    source_boundary_boost: float
    broad_transfer_boost: float
    overhead: float
    echo_pressure: float
    instruction_like: bool


@dataclass(frozen=True)
class StackMode:
    name: str
    packs: tuple[str, ...]
    description: str
    balance: float
    all_fire: bool = False


PACKS: dict[str, DatasetPack] = {
    "cognitive": DatasetPack(
        name="cognitive",
        description=(
            "Generalist cognitive memories: orientation, evidence, rhetoric, math, "
            "spatial reasoning, rewards, and computation memories."
        ),
        source_files=(
            "cognitive_core_v0_1.json",
            "cognitive_rewards_v0_1.json",
            "computation_primitives_v0_1.json",
        ),
        classifier_boost=0.14,
        primitive_boost=0.07,
        noise_resistance=0.10,
        distractor_resistance=0.055,
        cascade_resistance=0.08,
        source_boundary_boost=0.025,
        broad_transfer_boost=0.12,
        overhead=0.035,
        echo_pressure=0.035,
        instruction_like=True,
    ),
    "cyber": DatasetPack(
        name="cyber",
        description=(
            "Cyber/source-boundary memories: prompt-injection families, provenance, "
            "authority boundaries, and danger checks."
        ),
        source_files=("rust_cyber_defender_generated.tsv",),
        classifier_boost=0.12,
        primitive_boost=0.10,
        noise_resistance=0.10,
        distractor_resistance=0.18,
        cascade_resistance=0.14,
        source_boundary_boost=0.28,
        broad_transfer_boost=0.04,
        overhead=0.05,
        echo_pressure=0.07,
        instruction_like=True,
    ),
    "structural": DatasetPack(
        name="structural",
        description=(
            "Structural primitive memories: frame, gate, carrier, checksum, ECC, "
            "gain clamp, clean stop, and related functional operators."
        ),
        source_files=("structural_primitives_lora_v0_1.json",),
        classifier_boost=0.08,
        primitive_boost=0.27,
        noise_resistance=0.25,
        distractor_resistance=0.22,
        cascade_resistance=0.24,
        source_boundary_boost=0.12,
        broad_transfer_boost=0.10,
        overhead=0.05,
        echo_pressure=0.05,
        instruction_like=False,
    ),
}


STACKS = (
    StackMode("baseline", (), "No memory dataset.", balance=1.0),
    StackMode("cognitive_only", ("cognitive",), "Only the cognitive/generalist dataset.", balance=1.0),
    StackMode("cyber_only", ("cyber",), "Only the cyber/source-boundary dataset.", balance=1.0),
    StackMode("structural_only", ("structural",), "Only the structural primitive dataset.", balance=1.0),
    StackMode(
        "cognitive_plus_cyber",
        ("cognitive", "cyber"),
        "Cognitive recognition plus cyber source-boundary specialization.",
        balance=0.88,
    ),
    StackMode(
        "cognitive_plus_structural",
        ("cognitive", "structural"),
        "Cognitive memories organized by structural primitives.",
        balance=0.94,
    ),
    StackMode(
        "cyber_plus_structural",
        ("cyber", "structural"),
        "Cyber boundary memories stabilized by structural primitives.",
        balance=0.95,
    ),
    StackMode(
        "all_three_balanced",
        ("cognitive", "cyber", "structural"),
        "Balanced stack: cognitive breadth, cyber boundary specialization, and structural operators.",
        balance=0.90,
    ),
    StackMode(
        "all_three_naive_all_fire",
        ("cognitive", "cyber", "structural"),
        "Naive all-fire stack: all signals active without routing or compression.",
        balance=0.68,
        all_fire=True,
    ),
)


SUBSTRATES = {
    "base": {
        "cognitive_scale": 1.00,
        "cyber_scale": 1.00,
        "structural_scale": 1.00,
        "instruction_conflict": 0.00,
        "echo_scale": 1.00,
    },
    "instruct": {
        "cognitive_scale": 0.62,
        "cyber_scale": 0.72,
        "structural_scale": 0.96,
        "instruction_conflict": 0.08,
        "echo_scale": 0.88,
    },
    "degraded_instruct": {
        "cognitive_scale": 0.82,
        "cyber_scale": 0.86,
        "structural_scale": 1.02,
        "instruction_conflict": 0.04,
        "echo_scale": 1.06,
    },
}


FAMILY_BOOSTS = {
    "cognitive": {
        "math": 0.10,
        "rhetoric": 0.17,
        "spatial": 0.13,
        "generalist": 0.18,
        "boundary": 0.06,
    },
    "cyber": {
        "boundary": 0.30,
        "rhetoric": 0.08,
        "generalist": 0.06,
        "math": 0.00,
        "spatial": 0.02,
    },
    "structural": {
        "math": 0.13,
        "boundary": 0.16,
        "rhetoric": 0.12,
        "spatial": 0.13,
        "generalist": 0.12,
    },
}


def read_json_count(path: Path) -> int:
    if not path.exists():
        return 0
    parsed = json.loads(path.read_text(encoding="utf-8"))
    if isinstance(parsed, list):
        return len(parsed)
    return 1


def read_tsv_count(path: Path) -> int:
    if not path.exists():
        return 0
    with path.open("r", encoding="utf-8", newline="") as handle:
        return max(0, sum(1 for _ in handle) - 1)


def corpus_inventory() -> dict[str, object]:
    packs = {}
    for pack_name, pack in PACKS.items():
        files = []
        total = 0
        for file_name in pack.source_files:
            path = CORPUS_DIR / file_name
            count = read_tsv_count(path) if path.suffix.lower() == ".tsv" else read_json_count(path)
            total += count
            files.append({"file": file_name, "records": count})
        packs[pack_name] = {
            "description": pack.description,
            "total_records": total,
            "files": files,
        }
    return packs


def stack_features(stack: StackMode, substrate_name: str) -> dict[str, float]:
    if not stack.packs:
        return {
            "classifier_boost": 0.0,
            "primitive_boost": 0.0,
            "noise_resistance": 0.0,
            "distractor_resistance": 0.0,
            "cascade_resistance": 0.0,
            "source_boundary_boost": 0.0,
            "broad_transfer_boost": 0.0,
            "overhead": 0.0,
            "echo_pressure": 0.0,
        }

    substrate = SUBSTRATES[substrate_name]
    values = {
        "classifier_boost": 0.0,
        "primitive_boost": 0.0,
        "noise_resistance": 0.0,
        "distractor_resistance": 0.0,
        "cascade_resistance": 0.0,
        "source_boundary_boost": 0.0,
        "broad_transfer_boost": 0.0,
        "overhead": 0.0,
        "echo_pressure": 0.0,
    }

    for pack_name in stack.packs:
        pack = PACKS[pack_name]
        scale = substrate[f"{pack_name}_scale"]
        values["classifier_boost"] += pack.classifier_boost * scale
        values["primitive_boost"] += pack.primitive_boost * scale
        values["noise_resistance"] += pack.noise_resistance * scale
        values["distractor_resistance"] += pack.distractor_resistance * scale
        values["cascade_resistance"] += pack.cascade_resistance * scale
        values["source_boundary_boost"] += pack.source_boundary_boost * scale
        values["broad_transfer_boost"] += pack.broad_transfer_boost * scale
        values["overhead"] += pack.overhead
        values["echo_pressure"] += pack.echo_pressure * substrate["echo_scale"]
        if pack.instruction_like:
            values["overhead"] += substrate["instruction_conflict"]

    # Associations help when cognitive memories and structural operators are both present.
    has_cognitive = "cognitive" in stack.packs
    has_cyber = "cyber" in stack.packs
    has_structural = "structural" in stack.packs
    if has_cognitive and has_structural:
        values["classifier_boost"] += 0.05
        values["cascade_resistance"] += 0.06
        values["echo_pressure"] -= 0.02
    if has_cyber and has_structural:
        values["source_boundary_boost"] += 0.08
        values["distractor_resistance"] += 0.05
        values["cascade_resistance"] += 0.04
    if has_cognitive and has_cyber:
        values["broad_transfer_boost"] += 0.05
        values["source_boundary_boost"] += 0.03

    if len(stack.packs) == 3:
        values["classifier_boost"] += 0.03
        values["primitive_boost"] += 0.04
        values["cascade_resistance"] += 0.04
        values["overhead"] += 0.05

    if stack.all_fire:
        values["overhead"] += 0.18
        values["echo_pressure"] += 0.18
        values["distractor_resistance"] -= 0.08
        values["noise_resistance"] -= 0.06

    for key in ("classifier_boost", "primitive_boost", "noise_resistance", "distractor_resistance", "cascade_resistance"):
        values[key] *= stack.balance

    values["source_boundary_boost"] *= stack.balance
    values["broad_transfer_boost"] *= stack.balance

    values["noise_resistance"] = clamp(values["noise_resistance"], 0.0, 0.62)
    values["distractor_resistance"] = clamp(values["distractor_resistance"], 0.0, 0.62)
    values["cascade_resistance"] = clamp(values["cascade_resistance"], 0.0, 0.66)
    values["overhead"] = max(0.0, values["overhead"])
    values["echo_pressure"] = max(0.0, values["echo_pressure"])
    return values


def family_stack_boost(task: Task, stack: StackMode, substrate_name: str) -> float:
    substrate = SUBSTRATES[substrate_name]
    boost = 0.0
    for pack_name in stack.packs:
        scale = substrate[f"{pack_name}_scale"]
        boost += FAMILY_BOOSTS[pack_name].get(task.family, 0.0) * scale
    return boost * stack.balance


def run_trial(
    task: Task,
    stack: StackMode,
    substrate_name: str,
    quant_name: str,
    rng: random.Random,
) -> dict[str, object]:
    quant = QUANTS[quant_name]
    features = stack_features(stack, substrate_name)
    family_boost = family_stack_boost(task, stack, substrate_name)

    pressure_noise = quant["noise"] * task.pressure * (1.0 - features["noise_resistance"])
    classifier_p = quant["base_classifier"] + features["classifier_boost"] + family_boost
    classifier_p += features["source_boundary_boost"] if task.family == "boundary" else 0.0
    classifier_p += features["broad_transfer_boost"] * 0.35
    classifier_p -= pressure_noise
    classifier_p -= features["overhead"] * 0.72
    classified = rng.random() < clamp(classifier_p)

    active_required: list[str] = []
    missing_required: list[str] = []
    for primitive in task.required:
        activation_p = quant["base_activation"] + features["primitive_boost"]
        activation_p += family_boost * 0.45
        activation_p += features["source_boundary_boost"] * 0.65 if primitive in ("SOURCE_BOUNDARY", "ORIGINAL_TASK_RETURN", "DANGER_CHECK") else 0.0
        activation_p += features["broad_transfer_boost"] * 0.20
        activation_p -= pressure_noise
        activation_p -= features["overhead"] * 0.42
        if not classified:
            activation_p -= 0.16
        if rng.random() < clamp(activation_p):
            active_required.append(primitive)
        else:
            missing_required.append(primitive)

    active_distractors: list[str] = []
    for primitive in task.distractors:
        distractor_p = quant["distractor_pull"] * task.pressure * (1.0 - features["distractor_resistance"])
        distractor_p += features["overhead"] * 0.12
        if rng.random() < clamp(distractor_p):
            active_distractors.append(primitive)

    missing_ratio = len(missing_required) / len(task.required)
    cascade_p = quant["cascade"] * task.pressure * (1.0 + missing_ratio * 1.8)
    cascade_p *= 1.0 - features["cascade_resistance"]
    cascade_p += features["overhead"] * 0.12
    cascaded = rng.random() < clamp(cascade_p)

    echo_p = features["echo_pressure"] * (0.35 + task.pressure * 0.65)
    echo_p += features["overhead"] * 0.10
    echo = rng.random() < clamp(echo_p, 0.0, 0.7)

    coverage = len(active_required) / len(task.required)
    score = coverage
    score -= 0.22 if not classified else 0.0
    score -= 0.31 if cascaded else 0.0
    score -= min(0.22, 0.07 * len(active_distractors))
    score -= 0.08 if echo and stack.all_fire else 0.0
    score = clamp(score, 0.0, 1.0)

    passed = score >= 0.76 and not cascaded and len(missing_required) <= 1
    clean_passed = passed and not echo and len(active_distractors) == 0
    return {
        "substrate": substrate_name,
        "quant": quant_name,
        "stack": stack.name,
        "task_id": task.task_id,
        "family": task.family,
        "score": round(score, 4),
        "passed": passed,
        "clean_passed": clean_passed,
        "classified": classified,
        "cascaded": cascaded,
        "echo": echo,
        "required_active": len(active_required),
        "required_total": len(task.required),
        "distractors_active": len(active_distractors),
        "missing": ",".join(missing_required),
        "distractors": ",".join(active_distractors),
    }


def summarize(rows: list[dict[str, object]]) -> list[dict[str, object]]:
    groups: dict[tuple[str, str, str], list[dict[str, object]]] = {}
    for row in rows:
        key = (str(row["substrate"]), str(row["quant"]), str(row["stack"]))
        groups.setdefault(key, []).append(row)

    output = []
    for (substrate, quant, stack), group in sorted(groups.items()):
        count = len(group)
        output.append(
            {
                "substrate": substrate,
                "quant": quant,
                "stack": stack,
                "trials": count,
                "pass_rate": round(sum(1 for row in group if row["passed"]) / count, 4),
                "clean_pass_rate": round(sum(1 for row in group if row["clean_passed"]) / count, 4),
                "mean_score": round(sum(float(row["score"]) for row in group) / count, 4),
                "classified_rate": round(sum(1 for row in group if row["classified"]) / count, 4),
                "cascade_rate": round(sum(1 for row in group if row["cascaded"]) / count, 4),
                "echo_rate": round(sum(1 for row in group if row["echo"]) / count, 4),
                "avg_distractors": round(sum(int(row["distractors_active"]) for row in group) / count, 4),
            }
        )
    return output


def summarize_by_family(rows: list[dict[str, object]]) -> list[dict[str, object]]:
    groups: dict[tuple[str, str, str, str], list[dict[str, object]]] = {}
    for row in rows:
        key = (str(row["substrate"]), str(row["quant"]), str(row["stack"]), str(row["family"]))
        groups.setdefault(key, []).append(row)

    output = []
    for (substrate, quant, stack, family), group in sorted(groups.items()):
        count = len(group)
        output.append(
            {
                "substrate": substrate,
                "quant": quant,
                "stack": stack,
                "family": family,
                "trials": count,
                "pass_rate": round(sum(1 for row in group if row["passed"]) / count, 4),
                "clean_pass_rate": round(sum(1 for row in group if row["clean_passed"]) / count, 4),
                "mean_score": round(sum(float(row["score"]) for row in group) / count, 4),
                "cascade_rate": round(sum(1 for row in group if row["cascaded"]) / count, 4),
                "echo_rate": round(sum(1 for row in group if row["echo"]) / count, 4),
            }
        )
    return output


def lift(summary: list[dict[str, object]]) -> list[dict[str, object]]:
    by_context: dict[tuple[str, str], dict[str, dict[str, object]]] = {}
    for row in summary:
        key = (str(row["substrate"]), str(row["quant"]))
        by_context.setdefault(key, {})[str(row["stack"])] = row

    rows = []
    for (substrate, quant), stacks in sorted(by_context.items()):
        base = stacks["baseline"]
        for name, row in sorted(stacks.items()):
            if name == "baseline":
                continue
            rows.append(
                {
                    "substrate": substrate,
                    "quant": quant,
                    "stack": name,
                    "pass_lift": round(float(row["pass_rate"]) - float(base["pass_rate"]), 4),
                    "clean_pass_lift": round(float(row["clean_pass_rate"]) - float(base["clean_pass_rate"]), 4),
                    "score_lift": round(float(row["mean_score"]) - float(base["mean_score"]), 4),
                    "cascade_delta": round(float(row["cascade_rate"]) - float(base["cascade_rate"]), 4),
                    "echo_delta": round(float(row["echo_rate"]) - float(base["echo_rate"]), 4),
                }
            )
    return rows


def best_rows(summary: list[dict[str, object]]) -> list[dict[str, object]]:
    best = []
    by_context: dict[tuple[str, str], list[dict[str, object]]] = {}
    for row in summary:
        by_context.setdefault((str(row["substrate"]), str(row["quant"])), []).append(row)

    for (substrate, quant), group in sorted(by_context.items()):
        candidates = [row for row in group if row["stack"] != "baseline"]
        candidates.sort(
            key=lambda row: (
                float(row["clean_pass_rate"]),
                float(row["pass_rate"]),
                float(row["mean_score"]),
                -float(row["echo_rate"]),
                -float(row["cascade_rate"]),
            ),
            reverse=True,
        )
        best.append({"substrate": substrate, "quant": quant, **candidates[0]})
    return best


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        return
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)


def main() -> int:
    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    rng = random.Random(20260605)
    iterations = 1200

    rows: list[dict[str, object]] = []
    for substrate_name in SUBSTRATES:
        for quant_name in QUANTS:
            for stack in STACKS:
                for _ in range(iterations):
                    for task in TASKS:
                        rows.append(run_trial(task, stack, substrate_name, quant_name, rng))

    summary = summarize(rows)
    family_summary = summarize_by_family(rows)
    lift_rows = lift(summary)
    best = best_rows(summary)

    write_csv(REPORT_DIR / "trial_rows.csv", rows)
    write_csv(REPORT_DIR / "summary.csv", summary)
    write_csv(REPORT_DIR / "family_summary.csv", family_summary)
    write_csv(REPORT_DIR / "lift.csv", lift_rows)
    write_csv(REPORT_DIR / "best_by_context.csv", best)

    report = {
        "description": (
            "Toy simulation for combining three active dataset families: cognitive/generalist "
            "memories, cyber/source-boundary memories, and structural primitive memories. "
            "This is not a model result; it is a design simulation for choosing the next training mixture."
        ),
        "iterations_per_task": iterations,
        "dataset_inventory": corpus_inventory(),
        "packs": {name: pack.__dict__ for name, pack in PACKS.items()},
        "stacks": [stack.__dict__ for stack in STACKS],
        "substrates": SUBSTRATES,
        "summary": summary,
        "family_summary": family_summary,
        "lift": lift_rows,
        "best_by_context": best,
    }
    (REPORT_DIR / "report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")

    print("Three-dataset stack simulation")
    print(f"Rows: {len(rows)}")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Best stack per substrate/quant context")
    for row in best:
        print(
            f"{row['substrate']:>17} | {row['quant']:>11} | {row['stack']:<24} "
            f"clean={row['clean_pass_rate']:.3f} pass={row['pass_rate']:.3f} "
            f"score={row['mean_score']:.3f} echo={row['echo_rate']:.3f} "
            f"cascade={row['cascade_rate']:.3f}"
        )

    print()
    print("All-three comparison")
    for row in summary:
        if row["stack"] in ("all_three_balanced", "all_three_naive_all_fire"):
            print(
                f"{row['substrate']:>17} | {row['quant']:>11} | {row['stack']:<24} "
                f"clean={row['clean_pass_rate']:.3f} pass={row['pass_rate']:.3f} "
                f"score={row['mean_score']:.3f} echo={row['echo_rate']:.3f} "
                f"cascade={row['cascade_rate']:.3f}"
            )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
