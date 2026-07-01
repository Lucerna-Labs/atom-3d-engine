from __future__ import annotations

import csv
import json
import random
from dataclasses import dataclass
from pathlib import Path

from simulate_primitive_signal_hardening import QUANTS, TASKS, Task, clamp


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "q2-neural-exoskeleton"


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

TRANSPORT = (
    "FRAME_BOUNDARY",
    "SYNC_PREAMBLE",
    "ROUTE_HEADER",
    "REDUNDANT_ANCHOR",
    "DRIFT_DETECTOR",
    "FINAL_CHECKSUM",
    "REPAIR_REQUEST",
)


@dataclass(frozen=True)
class ExoConfig:
    name: str
    payload: tuple[str, ...]
    transport: tuple[str, ...]
    router: float
    state_buffer: float
    monitor: float
    verifier: float
    repair_loop: float
    output_gate: float
    max_repairs: int
    token_overhead: float


CONFIGS = (
    ExoConfig(
        "bare_q2",
        (),
        (),
        router=0.0,
        state_buffer=0.0,
        monitor=0.0,
        verifier=0.0,
        repair_loop=0.0,
        output_gate=0.0,
        max_repairs=0,
        token_overhead=0.0,
    ),
    ExoConfig(
        "primitive_packet",
        FUNCTIONAL_PAYLOAD,
        (),
        router=0.08,
        state_buffer=0.0,
        monitor=0.0,
        verifier=0.0,
        repair_loop=0.0,
        output_gate=0.0,
        max_repairs=0,
        token_overhead=0.03,
    ),
    ExoConfig(
        "transport_packet",
        FUNCTIONAL_PAYLOAD,
        TRANSPORT,
        router=0.18,
        state_buffer=0.08,
        monitor=0.12,
        verifier=0.18,
        repair_loop=0.0,
        output_gate=0.12,
        max_repairs=0,
        token_overhead=0.07,
    ),
    ExoConfig(
        "stabilized_transport",
        STABILIZER_PAYLOAD,
        TRANSPORT,
        router=0.25,
        state_buffer=0.10,
        monitor=0.20,
        verifier=0.24,
        repair_loop=0.0,
        output_gate=0.18,
        max_repairs=0,
        token_overhead=0.10,
    ),
    ExoConfig(
        "exo_light",
        FUNCTIONAL_PAYLOAD,
        TRANSPORT,
        router=0.28,
        state_buffer=0.26,
        monitor=0.42,
        verifier=0.48,
        repair_loop=0.44,
        output_gate=0.34,
        max_repairs=1,
        token_overhead=0.11,
    ),
    ExoConfig(
        "exo_full",
        STABILIZER_PAYLOAD,
        TRANSPORT,
        router=0.38,
        state_buffer=0.44,
        monitor=0.72,
        verifier=0.76,
        repair_loop=0.72,
        output_gate=0.64,
        max_repairs=2,
        token_overhead=0.16,
    ),
    ExoConfig(
        "exo_overbuilt",
        STABILIZER_PAYLOAD,
        TRANSPORT,
        router=0.44,
        state_buffer=0.58,
        monitor=0.88,
        verifier=0.88,
        repair_loop=0.86,
        output_gate=0.78,
        max_repairs=3,
        token_overhead=0.26,
    ),
)


def initial_pass(task: Task, config: ExoConfig, signal_strength: float, rng: random.Random) -> dict[str, object]:
    quant = QUANTS["damaged_q2"]
    payload_set = set(config.payload)
    required_set = set(task.required)

    coverage = len(required_set & payload_set) / len(required_set)
    transport_strength = len(config.transport) / len(TRANSPORT)
    stabilizer_count = sum(
        primitive in payload_set
        for primitive in (
            "ROUTE_TASK",
            "CHAIN_LOCK",
            "CASCADE_GUARD",
            "DISTRACTOR_SUPPRESS",
            "FINAL_ANSWER_GATE",
        )
    )
    stabilizer_strength = stabilizer_count / 5

    overhead = config.token_overhead
    overhead += 0.002 * len(config.payload)
    overhead += 0.003 * len(config.transport)

    noise_resistance = 0.06
    noise_resistance += 0.27 * coverage
    noise_resistance += 0.18 * signal_strength * coverage
    noise_resistance += 0.26 * transport_strength
    noise_resistance += 0.16 * stabilizer_strength
    noise_resistance += 0.18 * config.state_buffer
    noise_resistance = clamp(noise_resistance, 0.0, 0.995)

    pressure_noise = quant["noise"] * task.pressure * (1.0 - noise_resistance)

    packet_received_p = 0.38
    packet_received_p += 0.22 * signal_strength
    packet_received_p += 0.32 * transport_strength
    packet_received_p += 0.18 * config.router
    packet_received_p -= overhead * 0.35
    packet_received = rng.random() < clamp(packet_received_p, 0.0, 0.995)

    classifier_p = quant["base_classifier"]
    classifier_p += 0.13 * coverage
    classifier_p += 0.18 * config.router
    classifier_p += 0.14 * transport_strength
    classifier_p += 0.12 * stabilizer_strength
    classifier_p -= pressure_noise + overhead * 0.45
    if not packet_received:
        classifier_p -= 0.18
    classified = rng.random() < clamp(classifier_p, 0.0, 0.995)

    active_required: list[str] = []
    missing_required: list[str] = []
    for primitive in task.required:
        activation_p = quant["base_activation"]
        activation_p += signal_strength if primitive in payload_set else 0.0
        activation_p += 0.08 * coverage
        activation_p += 0.12 * transport_strength if packet_received else 0.0
        activation_p += 0.12 if "CHAIN_LOCK" in payload_set and primitive in payload_set else 0.0
        activation_p -= pressure_noise + overhead * 0.25
        if not classified:
            activation_p -= 0.10
        if not packet_received:
            activation_p -= 0.16

        if rng.random() < clamp(activation_p, 0.0, 0.995):
            active_required.append(primitive)
        else:
            missing_required.append(primitive)

    drift_p = quant["distractor_pull"] * task.pressure
    drift_p -= 0.20 * transport_strength
    drift_p -= 0.22 if "DISTRACTOR_SUPPRESS" in payload_set else 0.0
    drift_p -= 0.10 * config.monitor
    drifted = rng.random() < clamp(drift_p, 0.0, 0.995)

    missing_ratio = len(missing_required) / len(task.required)
    cascade_p = quant["cascade"] * task.pressure
    cascade_p *= 1.0 + missing_ratio * 1.8
    cascade_p *= 1.0 - noise_resistance
    cascade_p += 0.12 if drifted else 0.0
    cascade_p -= 0.08 if "CASCADE_GUARD" in payload_set else 0.0
    cascade_p -= 0.08 * config.verifier
    cascaded = rng.random() < clamp(cascade_p, 0.0, 0.995)

    state = {
        "classified": classified,
        "packet_received": packet_received,
        "active_required": active_required,
        "missing_required": missing_required,
        "drifted": drifted,
        "cascaded": cascaded,
        "repairs_used": 0,
    }
    recompute_score(task, config, state)
    return state


def recompute_score(task: Task, config: ExoConfig, state: dict[str, object]) -> None:
    active_required = state["active_required"]
    missing_required = state["missing_required"]
    score = len(active_required) / len(task.required)
    score -= 0.14 if not state["classified"] else 0.0
    score -= 0.28 if state["drifted"] else 0.0
    score -= 0.32 if state["cascaded"] else 0.0
    score += 0.06 * config.output_gate if not missing_required else 0.0
    score = clamp(score, 0.0, 1.0)
    state["score"] = score
    state["passed"] = score >= 0.76 and not state["drifted"] and not state["cascaded"] and len(missing_required) <= 1


def repair_pass(task: Task, config: ExoConfig, state: dict[str, object], signal_strength: float, rng: random.Random) -> None:
    if state["passed"]:
        return

    detect_p = 0.22
    detect_p += 0.34 * config.monitor
    detect_p += 0.30 * config.verifier
    detect_p += 0.12 * config.output_gate
    detected = rng.random() < clamp(detect_p, 0.0, 0.995)
    if not detected:
        return

    repair_power = 0.18
    repair_power += 0.44 * config.repair_loop
    repair_power += 0.24 * config.state_buffer
    repair_power += 0.16 * signal_strength
    repair_power += 0.10 * config.verifier
    repair_power = clamp(repair_power, 0.0, 0.98)

    if not state["classified"] and rng.random() < clamp(repair_power + config.router * 0.20, 0.0, 0.99):
        state["classified"] = True

    if not state["packet_received"] and rng.random() < clamp(repair_power + 0.12, 0.0, 0.99):
        state["packet_received"] = True

    missing = list(state["missing_required"])
    recovered = []
    for primitive in missing:
        targeted_p = repair_power
        targeted_p += 0.12 if primitive == "VERIFY_RESULT" else 0.0
        targeted_p += 0.08 if primitive in ("SOURCE_BOUNDARY", "ORIGINAL_TASK_RETURN") else 0.0
        if rng.random() < clamp(targeted_p, 0.0, 0.99):
            recovered.append(primitive)
            state["active_required"].append(primitive)
    state["missing_required"] = [primitive for primitive in missing if primitive not in recovered]

    if state["drifted"] and rng.random() < clamp(repair_power + config.monitor * 0.20, 0.0, 0.99):
        state["drifted"] = False

    if state["cascaded"] and rng.random() < clamp(repair_power + config.verifier * 0.16, 0.0, 0.96):
        state["cascaded"] = False

    state["repairs_used"] += 1
    recompute_score(task, config, state)


def run_trial(task: Task, config: ExoConfig, signal_strength: float, rng: random.Random) -> dict[str, object]:
    state = initial_pass(task, config, signal_strength, rng)
    for _ in range(config.max_repairs):
        repair_pass(task, config, state, signal_strength, rng)
        if state["passed"]:
            break

    return {
        "task_id": task.task_id,
        "passed": state["passed"],
        "score": state["score"],
        "classified": state["classified"],
        "packet_received": state["packet_received"],
        "drifted": state["drifted"],
        "cascaded": state["cascaded"],
        "missing_count": len(state["missing_required"]),
        "repairs_used": state["repairs_used"],
    }


def evaluate(config: ExoConfig, signal_name: str, signal_strength: float) -> dict[str, object]:
    rng = random.Random(20260604 + sum(ord(ch) for ch in config.name + signal_name))
    iterations = 800
    rows = []
    for _ in range(iterations):
        for task in TASKS:
            rows.append(run_trial(task, config, signal_strength, rng))

    per_task = {}
    for task in TASKS:
        task_rows = [row for row in rows if row["task_id"] == task.task_id]
        per_task[task.task_id] = round(sum(1 for row in task_rows if row["passed"]) / len(task_rows), 4)

    return {
        "config": config.name,
        "signal": signal_name,
        "pass_rate": round(sum(1 for row in rows if row["passed"]) / len(rows), 4),
        "mean_score": round(sum(float(row["score"]) for row in rows) / len(rows), 4),
        "packet_received_rate": round(sum(1 for row in rows if row["packet_received"]) / len(rows), 4),
        "classified_rate": round(sum(1 for row in rows if row["classified"]) / len(rows), 4),
        "drift_rate": round(sum(1 for row in rows if row["drifted"]) / len(rows), 4),
        "cascade_rate": round(sum(1 for row in rows if row["cascaded"]) / len(rows), 4),
        "avg_missing": round(sum(int(row["missing_count"]) for row in rows) / len(rows), 4),
        "avg_repairs": round(sum(int(row["repairs_used"]) for row in rows) / len(rows), 4),
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
    signal_levels = (("normal", 0.46), ("strong", 0.70), ("saturated", 0.95))
    rows = []
    for config in CONFIGS:
        for signal_name, signal_strength in signal_levels:
            rows.append(evaluate(config, signal_name, signal_strength))

    rows.sort(
        key=lambda row: (
            float(row["pass_rate"]),
            float(row["worst_task_rate"]),
            float(row["mean_score"]),
        ),
        reverse=True,
    )

    write_csv(REPORT_DIR / "exoskeleton_results.csv", rows)
    (REPORT_DIR / "exoskeleton_results.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")

    print("Q2 neural exoskeleton simulation")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Top results")
    for index, row in enumerate(rows[:12], start=1):
        print(
            f"{index:02d}. {row['config']:<22} {row['signal']:<9} "
            f"pass={row['pass_rate']:.3f} worst={row['worst_task_rate']:.3f} "
            f"score={row['mean_score']:.3f} rx={row['packet_received_rate']:.3f} "
            f"drift={row['drift_rate']:.3f} cascade={row['cascade_rate']:.3f} "
            f"repairs={row['avg_repairs']:.3f}"
        )


if __name__ == "__main__":
    main()
