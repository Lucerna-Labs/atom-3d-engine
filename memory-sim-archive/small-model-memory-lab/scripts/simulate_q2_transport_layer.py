from __future__ import annotations

import csv
import json
import random
from pathlib import Path

from simulate_primitive_signal_hardening import QUANTS, TASKS, Task, clamp


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "q2-transport-layer"


FUNCTIONAL_PAYLOAD = (
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

STABILIZER_PAYLOAD = (
    "ROUTE_TASK",
    "CHAIN_LOCK",
    "CASCADE_GUARD",
    "DISTRACTOR_SUPPRESS",
    "FINAL_ANSWER_GATE",
) + FUNCTIONAL_PAYLOAD

TRANSPORT_FEATURES = (
    "FRAME_BOUNDARY",
    "SYNC_PREAMBLE",
    "ROUTE_HEADER",
    "REDUNDANT_ANCHOR",
    "DRIFT_DETECTOR",
    "FINAL_CHECKSUM",
    "REPAIR_REQUEST",
)

PACKETS = {
    "raw_functional": {
        "payload": FUNCTIONAL_PAYLOAD,
        "transport": (),
    },
    "raw_stabilized": {
        "payload": STABILIZER_PAYLOAD,
        "transport": (),
    },
    "framed_functional": {
        "payload": FUNCTIONAL_PAYLOAD,
        "transport": ("FRAME_BOUNDARY", "SYNC_PREAMBLE", "ROUTE_HEADER", "FINAL_CHECKSUM"),
    },
    "framed_stabilized": {
        "payload": STABILIZER_PAYLOAD,
        "transport": ("FRAME_BOUNDARY", "SYNC_PREAMBLE", "ROUTE_HEADER", "FINAL_CHECKSUM"),
    },
    "rf_transport_functional": {
        "payload": FUNCTIONAL_PAYLOAD,
        "transport": TRANSPORT_FEATURES,
    },
    "rf_transport_stabilized": {
        "payload": STABILIZER_PAYLOAD,
        "transport": TRANSPORT_FEATURES,
    },
}

SIGNAL_LEVELS = (
    ("normal", 0.46),
    ("strong", 0.70),
    ("saturated", 0.95),
)


def run_transport_trial(
    task: Task,
    payload: tuple[str, ...],
    transport: tuple[str, ...],
    signal_strength: float,
    rng: random.Random,
) -> dict[str, object]:
    quant = QUANTS["damaged_q2"]
    payload_set = set(payload)
    transport_set = set(transport)
    required_set = set(task.required)

    payload_coverage = len(required_set & payload_set) / len(required_set)
    transport_strength = len(transport_set) / len(TRANSPORT_FEATURES)

    framed = "FRAME_BOUNDARY" in transport_set
    synced = "SYNC_PREAMBLE" in transport_set
    routed = "ROUTE_HEADER" in transport_set
    redundant = "REDUNDANT_ANCHOR" in transport_set
    drift_detector = "DRIFT_DETECTOR" in transport_set
    checksum = "FINAL_CHECKSUM" in transport_set
    repair = "REPAIR_REQUEST" in transport_set

    route_task = "ROUTE_TASK" in payload_set
    chain_lock = "CHAIN_LOCK" in payload_set
    cascade_guard = "CASCADE_GUARD" in payload_set
    distractor_suppress = "DISTRACTOR_SUPPRESS" in payload_set
    final_answer_gate = "FINAL_ANSWER_GATE" in payload_set

    # A transport layer costs tokens, but its job is to reduce context noise
    # and make the primitive packet easier to lock onto.
    transport_overhead = 0.012 * len(transport_set)
    payload_overhead = 0.0035 * (len(payload) ** 1.15)
    overhead = transport_overhead + payload_overhead

    lock_quality = 0.0
    lock_quality += 0.16 if framed else 0.0
    lock_quality += 0.18 if synced else 0.0
    lock_quality += 0.20 if routed else 0.0
    lock_quality += 0.15 if redundant else 0.0
    lock_quality += 0.18 if drift_detector else 0.0
    lock_quality += 0.16 if checksum else 0.0
    lock_quality += 0.16 if repair else 0.0
    lock_quality = clamp(lock_quality, 0.0, 0.98)

    noise_resistance = 0.10 + 0.28 * payload_coverage + 0.18 * signal_strength * payload_coverage
    noise_resistance += 0.34 * lock_quality
    noise_resistance += 0.13 if chain_lock else 0.0
    noise_resistance += 0.17 if cascade_guard else 0.0
    noise_resistance += 0.06 if final_answer_gate else 0.0
    noise_resistance = clamp(noise_resistance, 0.0, 0.99)

    pressure_noise = quant["noise"] * task.pressure * (1.0 - noise_resistance)

    packet_received_p = 0.58 + 0.25 * signal_strength + 0.34 * lock_quality - overhead
    packet_received_p = clamp(packet_received_p, 0.0, 0.995)
    packet_received = rng.random() < packet_received_p

    classifier_p = quant["base_classifier"]
    classifier_p += 0.14 * payload_coverage
    classifier_p += 0.14 * signal_strength * payload_coverage
    classifier_p += 0.20 if route_task else 0.0
    classifier_p += 0.18 if routed else 0.0
    classifier_p += 0.11 if synced else 0.0
    classifier_p -= pressure_noise + overhead * 0.55
    if not packet_received:
        classifier_p -= 0.20
    classifier_p = clamp(classifier_p, 0.0, 0.995)
    classified = rng.random() < classifier_p

    active_required: list[str] = []
    missing_required: list[str] = []
    for primitive in task.required:
        activation_p = quant["base_activation"]
        activation_p += signal_strength if primitive in payload_set else 0.0
        activation_p += 0.08 * payload_coverage
        activation_p += 0.12 if chain_lock and primitive in payload_set else 0.0
        activation_p += 0.16 * lock_quality if packet_received else 0.0
        activation_p -= pressure_noise + overhead * 0.30
        if not classified:
            activation_p -= 0.09 if routed else 0.16
        if not packet_received:
            activation_p -= 0.18
        activation_p = clamp(activation_p, 0.0, 0.995)

        if rng.random() < activation_p:
            active_required.append(primitive)
        else:
            missing_required.append(primitive)

    if redundant and missing_required:
        recovered = []
        for primitive in missing_required:
            recover_p = 0.18 + 0.28 * signal_strength + 0.22 * lock_quality
            if rng.random() < clamp(recover_p, 0.0, 0.94):
                active_required.append(primitive)
                recovered.append(primitive)
        missing_required = [primitive for primitive in missing_required if primitive not in recovered]

    drifted = False
    drift_p = quant["distractor_pull"] * task.pressure
    drift_p -= 0.18 if framed else 0.0
    drift_p -= 0.26 if distractor_suppress else 0.0
    drift_p -= 0.25 if drift_detector else 0.0
    drift_p += 0.04 * max(0, len(payload) - 10)
    if rng.random() < clamp(drift_p, 0.0, 0.995):
        drifted = True

    if drifted and repair:
        repair_p = 0.26 + 0.34 * signal_strength + 0.22 * lock_quality
        if rng.random() < clamp(repair_p, 0.0, 0.96):
            drifted = False

    missing_ratio = len(missing_required) / len(task.required)
    cascade_p = quant["cascade"] * task.pressure
    cascade_p *= 1.0 + missing_ratio * 1.8
    cascade_p *= 1.0 - noise_resistance
    cascade_p += 0.14 if drifted else 0.0
    cascade_p -= 0.08 if cascade_guard else 0.0
    cascade_p -= 0.10 if checksum else 0.0
    cascade_p -= 0.08 if repair and missing_required else 0.0
    cascaded = rng.random() < clamp(cascade_p, 0.0, 0.995)

    if cascaded and repair:
        cascade_repair_p = 0.14 + 0.28 * signal_strength + 0.22 * lock_quality
        if rng.random() < clamp(cascade_repair_p, 0.0, 0.90):
            cascaded = False

    score = len(active_required) / len(task.required)
    score -= 0.16 if not classified else 0.0
    score -= 0.30 if drifted else 0.0
    score -= 0.34 if cascaded else 0.0
    score += 0.06 if checksum and not missing_required else 0.0
    score += 0.05 if final_answer_gate and not missing_required else 0.0
    score = clamp(score, 0.0, 1.0)

    passed = score >= 0.76 and not cascaded and not drifted and len(missing_required) <= 1
    return {
        "task_id": task.task_id,
        "passed": passed,
        "score": score,
        "packet_received": packet_received,
        "classified": classified,
        "drifted": drifted,
        "cascaded": cascaded,
        "missing_count": len(missing_required),
    }


def evaluate(packet_name: str, packet: dict[str, tuple[str, ...]], signal_name: str, signal_strength: float) -> dict[str, object]:
    rng = random.Random(20260604 + sum(ord(ch) for ch in packet_name + signal_name))
    iterations = 600
    rows = []
    for _ in range(iterations):
        for task in TASKS:
            rows.append(
                run_transport_trial(
                    task,
                    packet["payload"],
                    packet["transport"],
                    signal_strength,
                    rng,
                )
            )

    per_task = {}
    for task in TASKS:
        task_rows = [row for row in rows if row["task_id"] == task.task_id]
        per_task[task.task_id] = round(sum(1 for row in task_rows if row["passed"]) / len(task_rows), 4)

    return {
        "packet": packet_name,
        "signal": signal_name,
        "payload_size": len(packet["payload"]),
        "transport_size": len(packet["transport"]),
        "pass_rate": round(sum(1 for row in rows if row["passed"]) / len(rows), 4),
        "mean_score": round(sum(float(row["score"]) for row in rows) / len(rows), 4),
        "packet_received_rate": round(sum(1 for row in rows if row["packet_received"]) / len(rows), 4),
        "classified_rate": round(sum(1 for row in rows if row["classified"]) / len(rows), 4),
        "drift_rate": round(sum(1 for row in rows if row["drifted"]) / len(rows), 4),
        "cascade_rate": round(sum(1 for row in rows if row["cascaded"]) / len(rows), 4),
        "avg_missing": round(sum(int(row["missing_count"]) for row in rows) / len(rows), 4),
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
    for packet_name, packet in PACKETS.items():
        for signal_name, signal_strength in SIGNAL_LEVELS:
            rows.append(evaluate(packet_name, packet, signal_name, signal_strength))

    rows.sort(
        key=lambda row: (
            float(row["pass_rate"]),
            float(row["worst_task_rate"]),
            float(row["mean_score"]),
        ),
        reverse=True,
    )

    write_csv(REPORT_DIR / "transport_results.csv", rows)
    (REPORT_DIR / "transport_results.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")

    print("Q2 primitive transport-layer simulation")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Top results")
    for index, row in enumerate(rows[:12], start=1):
        print(
            f"{index:02d}. {row['packet']:<26} {row['signal']:<9} "
            f"pass={row['pass_rate']:.3f} worst={row['worst_task_rate']:.3f} "
            f"score={row['mean_score']:.3f} rx={row['packet_received_rate']:.3f} "
            f"drift={row['drift_rate']:.3f} cascade={row['cascade_rate']:.3f}"
        )


if __name__ == "__main__":
    main()
