from __future__ import annotations

import csv
import itertools
import json
import random
from pathlib import Path

from simulate_primitive_signal_hardening import PRIMITIVES, QUANTS, TASKS, Task, clamp


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "q2-primitive-self-reinforcement"


STRUCTURAL_PRIMITIVES = (
    "FRAME",
    "CLOCK",
    "CARRIER",
    "ROUTER",
    "PARITY",
    "CHECKSUM",
    "ECC",
    "REDUNDANCY",
    "SUPPRESSION",
    "COMPRESSION",
    "STATE_BUFFER",
    "GAIN_CLAMP",
)

BASE_PAYLOAD = tuple(PRIMITIVES)

SIGNAL_LEVELS = (
    ("rag_like", 0.48),
    ("kv_packet", 0.64),
    ("lora_candidate", 0.78),
)


def overhead_for(active_structural: set[str], payload_size: int) -> float:
    raw_size = payload_size + len(active_structural)
    overhead = 0.0045 * (raw_size**1.20)
    overhead += 0.018 if "REDUNDANCY" in active_structural else 0.0
    overhead += 0.012 if "ECC" in active_structural else 0.0
    overhead += 0.010 if "STATE_BUFFER" in active_structural else 0.0
    if "COMPRESSION" in active_structural:
        overhead *= 0.64
    return overhead


def surviving_structural_primitives(
    requested: tuple[str, ...],
    task: Task,
    signal_strength: float,
    rng: random.Random,
) -> tuple[set[str], int]:
    requested_set = set(requested)
    quant = QUANTS["damaged_q2"]
    overhead = 0.006 * (len(requested) ** 1.18)

    base_noise = quant["noise"] * task.pressure
    active: set[str] = set()
    failed: list[str] = []

    for primitive in requested:
        survival_p = 0.46
        survival_p += 0.16 * signal_strength
        survival_p += 0.08 if "FRAME" in requested_set else 0.0
        survival_p += 0.07 if "CLOCK" in requested_set else 0.0
        survival_p += 0.06 if "CARRIER" in requested_set else 0.0
        survival_p += 0.13 if "REDUNDANCY" in requested_set else 0.0
        survival_p += 0.04 if "GAIN_CLAMP" in requested_set else 0.0
        survival_p -= base_noise * 0.24
        survival_p -= overhead

        if rng.random() < clamp(survival_p, 0.0, 0.985):
            active.add(primitive)
        else:
            failed.append(primitive)

    repair_available = ("ECC" in active) and (("PARITY" in active) or ("CHECKSUM" in active))
    repaired = 0
    if repair_available and failed:
        repair_p = 0.20
        repair_p += 0.22 * signal_strength
        repair_p += 0.10 if "FRAME" in active else 0.0
        repair_p += 0.08 if "CLOCK" in active else 0.0
        repair_p -= overhead * 0.45
        for primitive in failed:
            if rng.random() < clamp(repair_p, 0.0, 0.94):
                active.add(primitive)
                repaired += 1

    return active, repaired


def route_payload(task: Task, active_structural: set[str], rng: random.Random) -> tuple[tuple[str, ...], bool]:
    if "ROUTER" not in active_structural:
        return BASE_PAYLOAD, False

    route_p = 0.58
    route_p += 0.12 if "FRAME" in active_structural else 0.0
    route_p += 0.10 if "CLOCK" in active_structural else 0.0
    route_p += 0.08 if "STATE_BUFFER" in active_structural else 0.0
    route_p -= 0.12 * task.pressure

    route_correct = rng.random() < clamp(route_p, 0.0, 0.98)
    if route_correct:
        routed = tuple(dict.fromkeys((*task.required, "VERIFY_RESULT", "ORIGINAL_TASK_RETURN")))
        return routed, True

    wrong_family = rng.choice([candidate for candidate in TASKS if candidate.family != task.family])
    routed = tuple(dict.fromkeys((*wrong_family.required, "VERIFY_RESULT")))
    return routed, False


def run_trial(
    task: Task,
    requested_structural: tuple[str, ...],
    signal_strength: float,
    rng: random.Random,
) -> dict[str, object]:
    quant = QUANTS["damaged_q2"]
    active_structural, structural_repairs = surviving_structural_primitives(
        requested_structural,
        task,
        signal_strength,
        rng,
    )
    payload, route_correct = route_payload(task, active_structural, rng)
    payload_set = set(payload)
    required_set = set(task.required)
    distractor_set = set(task.distractors)

    coverage = len(required_set & payload_set) / len(required_set)
    distractor_overlap = len(payload_set & distractor_set)
    irrelevant = len(payload_set - required_set)
    overhead = overhead_for(active_structural, len(payload_set))

    noise_resistance = 0.08 + 0.28 * coverage
    noise_resistance += 0.20 * signal_strength * coverage
    noise_resistance += 0.12 if "FRAME" in active_structural else 0.0
    noise_resistance += 0.10 if "CLOCK" in active_structural else 0.0
    noise_resistance += 0.10 if "CARRIER" in active_structural else 0.0
    noise_resistance += 0.14 if "STATE_BUFFER" in active_structural else 0.0
    noise_resistance += 0.08 if "GAIN_CLAMP" in active_structural else 0.0
    noise_resistance -= 0.026 * max(0, irrelevant - 3)
    noise_resistance = clamp(noise_resistance, 0.0, 0.965)

    pressure_noise = quant["noise"] * task.pressure * (1.0 - noise_resistance)
    packet_p = 0.38
    packet_p += 0.16 * signal_strength
    packet_p += 0.14 if "FRAME" in active_structural else 0.0
    packet_p += 0.12 if "CLOCK" in active_structural else 0.0
    packet_p += 0.10 if "CARRIER" in active_structural else 0.0
    packet_p -= overhead * 0.42
    packet_received = rng.random() < clamp(packet_p, 0.0, 0.985)

    classifier_p = quant["base_classifier"]
    classifier_p += 0.16 * coverage
    classifier_p += 0.14 * signal_strength * coverage
    classifier_p += 0.11 if "ROUTER" in active_structural else 0.0
    classifier_p += 0.05 if "FRAME" in active_structural else 0.0
    classifier_p += 0.05 if "STATE_BUFFER" in active_structural else 0.0
    classifier_p -= pressure_noise + overhead * 0.55
    classifier_p -= 0.10 if not packet_received else 0.0
    classified = rng.random() < clamp(classifier_p, 0.0, 0.985)

    active_required: list[str] = []
    missing_required: list[str] = []
    for primitive in task.required:
        activation_p = quant["base_activation"]
        activation_p += signal_strength if primitive in payload_set else 0.0
        activation_p += 0.09 * coverage
        activation_p += 0.08 if "CARRIER" in active_structural else 0.0
        activation_p += 0.08 if "REDUNDANCY" in active_structural and primitive in payload_set else 0.0
        activation_p += 0.06 if "STATE_BUFFER" in active_structural else 0.0
        activation_p += 0.04 if "GAIN_CLAMP" in active_structural else 0.0
        activation_p -= pressure_noise
        activation_p -= overhead * 0.26
        activation_p -= 0.11 if not classified else 0.0
        activation_p -= 0.10 if not packet_received else 0.0
        if rng.random() < clamp(activation_p, 0.0, 0.985):
            active_required.append(primitive)
        else:
            missing_required.append(primitive)

    detected_missing = False
    if missing_required and (("PARITY" in active_structural) or ("CHECKSUM" in active_structural)):
        detect_p = 0.40
        detect_p += 0.12 if "FRAME" in active_structural else 0.0
        detect_p += 0.10 if "CLOCK" in active_structural else 0.0
        detect_p += 0.08 if "STATE_BUFFER" in active_structural else 0.0
        detect_p -= overhead * 0.35
        detected_missing = rng.random() < clamp(detect_p, 0.0, 0.96)

    functional_repairs = 0
    if detected_missing and "ECC" in active_structural:
        repaired = []
        repair_p = 0.24
        repair_p += 0.22 * signal_strength
        repair_p += 0.12 if "REDUNDANCY" in active_structural else 0.0
        repair_p += 0.10 if "STATE_BUFFER" in active_structural else 0.0
        repair_p -= overhead * 0.34
        for primitive in missing_required:
            if rng.random() < clamp(repair_p, 0.0, 0.96):
                active_required.append(primitive)
                repaired.append(primitive)
                functional_repairs += 1
        missing_required = [primitive for primitive in missing_required if primitive not in repaired]

    active_distractors: list[str] = []
    for primitive in task.distractors:
        distractor_p = quant["distractor_pull"] * task.pressure
        distractor_p += 0.14 * signal_strength if primitive in payload_set else 0.0
        distractor_p += 0.018 * irrelevant
        distractor_p += 0.03 * distractor_overlap
        distractor_p -= 0.18 if "SUPPRESSION" in active_structural else 0.0
        distractor_p -= 0.06 if "GAIN_CLAMP" in active_structural else 0.0
        if rng.random() < clamp(distractor_p, 0.0, 0.985):
            active_distractors.append(primitive)

    missing_ratio = len(missing_required) / len(task.required)
    cascade_p = quant["cascade"] * task.pressure
    cascade_p *= 1.0 + missing_ratio * 1.75
    cascade_p *= 1.0 - noise_resistance
    cascade_p += 0.10 if not packet_received else 0.0
    cascade_p += 0.08 if not classified else 0.0
    cascade_p -= 0.16 if "CHECKSUM" in active_structural else 0.0
    cascade_p -= 0.12 if "CLOCK" in active_structural else 0.0
    cascade_p -= 0.08 if "STATE_BUFFER" in active_structural else 0.0
    cascade_p -= 0.06 if "GAIN_CLAMP" in active_structural else 0.0
    cascaded = rng.random() < clamp(cascade_p, 0.0, 0.985)

    if cascaded and "CHECKSUM" in active_structural and "ECC" in active_structural:
        cascade_repair_p = 0.22
        cascade_repair_p += 0.18 * signal_strength
        cascade_repair_p += 0.10 if "CLOCK" in active_structural else 0.0
        cascade_repair_p -= overhead * 0.28
        if rng.random() < clamp(cascade_repair_p, 0.0, 0.86):
            cascaded = False
            functional_repairs += 1

    score = len(active_required) / len(task.required)
    score -= 0.15 if not classified else 0.0
    score -= 0.14 if not packet_received else 0.0
    score -= 0.32 if cascaded else 0.0
    score -= min(0.28, 0.07 * len(active_distractors))
    score += 0.04 if route_correct else 0.0
    score = clamp(score, 0.0, 1.0)

    passed = score >= 0.76 and not cascaded and len(missing_required) <= 1
    return {
        "task_id": task.task_id,
        "family": task.family,
        "passed": passed,
        "score": score,
        "classified": classified,
        "packet_received": packet_received,
        "route_correct": route_correct,
        "cascaded": cascaded,
        "missing_count": len(missing_required),
        "distractors": len(active_distractors),
        "structural_requested": len(requested_structural),
        "structural_active": len(active_structural),
        "structural_repairs": structural_repairs,
        "functional_repairs": functional_repairs,
        "active_structural": "+".join(sorted(active_structural)),
    }


def evaluate_combo(
    combo: tuple[str, ...],
    signal_name: str,
    signal_strength: float,
    iterations: int,
) -> dict[str, object]:
    seed_text = "|".join((signal_name, *combo))
    rng = random.Random(20260604 + sum(ord(ch) for ch in seed_text))
    rows = []
    for _ in range(iterations):
        for task in TASKS:
            rows.append(run_trial(task, combo, signal_strength, rng))

    per_task = {}
    for task in TASKS:
        task_rows = [row for row in rows if row["task_id"] == task.task_id]
        per_task[task.task_id] = round(sum(1 for row in task_rows if row["passed"]) / len(task_rows), 4)

    return {
        "signal": signal_name,
        "signal_strength": signal_strength,
        "combo_size": len(combo),
        "combo": "+".join(combo) if combo else "NONE",
        "pass_rate": round(sum(1 for row in rows if row["passed"]) / len(rows), 4),
        "mean_score": round(sum(float(row["score"]) for row in rows) / len(rows), 4),
        "packet_received_rate": round(sum(1 for row in rows if row["packet_received"]) / len(rows), 4),
        "classified_rate": round(sum(1 for row in rows if row["classified"]) / len(rows), 4),
        "route_rate": round(sum(1 for row in rows if row["route_correct"]) / len(rows), 4),
        "cascade_rate": round(sum(1 for row in rows if row["cascaded"]) / len(rows), 4),
        "avg_missing": round(sum(int(row["missing_count"]) for row in rows) / len(rows), 4),
        "avg_distractors": round(sum(int(row["distractors"]) for row in rows) / len(rows), 4),
        "structural_survival": round(
            sum(int(row["structural_active"]) for row in rows)
            / max(1, sum(int(row["structural_requested"]) for row in rows)),
            4,
        ),
        "avg_structural_repairs": round(sum(int(row["structural_repairs"]) for row in rows) / len(rows), 4),
        "avg_functional_repairs": round(sum(int(row["functional_repairs"]) for row in rows) / len(rows), 4),
        "worst_task_rate": min(per_task.values()),
        "per_task": per_task,
    }


def mechanism_lifts(rows: list[dict[str, object]]) -> list[dict[str, object]]:
    output = []
    for signal_name, _ in SIGNAL_LEVELS:
        signal_rows = [row for row in rows if row["signal"] == signal_name]
        for primitive in STRUCTURAL_PRIMITIVES:
            with_p = [row for row in signal_rows if primitive in str(row["combo"]).split("+")]
            without_p = [
                row
                for row in signal_rows
                if primitive not in str(row["combo"]).split("+") and str(row["combo"]) != "NONE"
            ]
            if not with_p or not without_p:
                continue
            with_pass = sum(float(row["pass_rate"]) for row in with_p) / len(with_p)
            without_pass = sum(float(row["pass_rate"]) for row in without_p) / len(without_p)
            output.append(
                {
                    "signal": signal_name,
                    "primitive": primitive,
                    "with_pass": round(with_pass, 4),
                    "without_pass": round(without_pass, 4),
                    "pass_lift": round(with_pass - without_pass, 4),
                }
            )
    output.sort(key=lambda row: (str(row["signal"]), float(row["pass_lift"])), reverse=True)
    return output


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
    iterations = 45

    combos: list[tuple[str, ...]] = [()]
    for size in range(1, len(STRUCTURAL_PRIMITIVES) + 1):
        combos.extend(itertools.combinations(STRUCTURAL_PRIMITIVES, size))

    rows = []
    for signal_name, signal_strength in SIGNAL_LEVELS:
        for combo in combos:
            rows.append(evaluate_combo(combo, signal_name, signal_strength, iterations))

    rows.sort(
        key=lambda row: (
            float(row["pass_rate"]),
            float(row["worst_task_rate"]),
            float(row["mean_score"]),
            -int(row["combo_size"]),
        ),
        reverse=True,
    )

    top = rows[:80]
    lifts = mechanism_lifts(rows)
    best_by_signal = {}
    for signal_name, _ in SIGNAL_LEVELS:
        signal_rows = [row for row in rows if row["signal"] == signal_name]
        best_by_signal[signal_name] = max(
            signal_rows,
            key=lambda row: (
                float(row["pass_rate"]),
                float(row["worst_task_rate"]),
                float(row["mean_score"]),
                -int(row["combo_size"]),
            ),
        )

    write_csv(REPORT_DIR / "all_combo_results.csv", rows)
    write_csv(REPORT_DIR / "top_80.csv", top)
    write_csv(REPORT_DIR / "mechanism_lifts.csv", lifts)
    write_csv(REPORT_DIR / "best_by_signal.csv", list(best_by_signal.values()))
    (REPORT_DIR / "top_80.json").write_text(json.dumps(top, indent=2), encoding="utf-8")
    (REPORT_DIR / "report.json").write_text(
        json.dumps(
            {
                "description": (
                    "Q2-only simulation of structural primitive self-reinforcement. "
                    "Functional primitives are fixed; structural primitives may fail, "
                    "repair each other, route the payload, and repair the functional layer."
                ),
                "iterations_per_task": iterations,
                "structural_primitives": STRUCTURAL_PRIMITIVES,
                "functional_payload": BASE_PAYLOAD,
                "best_by_signal": best_by_signal,
                "mechanism_lifts": lifts,
            },
            indent=2,
        ),
        encoding="utf-8",
    )

    print("Q2 primitive self-reinforcement simulation")
    print(f"Combinations tested: {len(rows)}")
    print(f"Iterations per task per combo: {iterations}")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Best by signal level")
    for signal_name, row in best_by_signal.items():
        print(
            f"{signal_name:<15} pass={row['pass_rate']:.3f} "
            f"worst={row['worst_task_rate']:.3f} score={row['mean_score']:.3f} "
            f"survive={row['structural_survival']:.3f} rx={row['packet_received_rate']:.3f} "
            f"cascade={row['cascade_rate']:.3f} size={row['combo_size']} combo={row['combo']}"
        )

    print()
    print("Top 12 overall")
    for index, row in enumerate(top[:12], start=1):
        print(
            f"{index:02d}. {row['signal']:<15} pass={row['pass_rate']:.3f} "
            f"worst={row['worst_task_rate']:.3f} score={row['mean_score']:.3f} "
            f"survive={row['structural_survival']:.3f} size={row['combo_size']} "
            f"{row['combo']}"
        )


if __name__ == "__main__":
    main()
