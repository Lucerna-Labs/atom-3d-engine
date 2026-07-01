from __future__ import annotations

import csv
import json
import random
from dataclasses import dataclass
from pathlib import Path

from simulate_primitive_signal_hardening import QUANTS, TASKS, Task, clamp


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "hybrid-lora-geometry-sim"


@dataclass(frozen=True)
class Geometry:
    name: str
    additive: float
    gate: float
    web: float
    fourier: float
    anti_echo: float
    overhead: float
    train_complexity: float


GEOMETRIES = (
    Geometry("base_no_adapter", 0.0, 0.0, 0.0, 0.0, 0.00, 0.00, 0.00),
    Geometry("standard_lora_additive", 0.78, 0.0, 0.0, 0.0, 0.06, 0.05, 0.18),
    Geometry("gated_lora", 0.58, 0.62, 0.0, 0.0, 0.38, 0.07, 0.28),
    Geometry("kronecker_web", 0.44, 0.0, 0.72, 0.0, 0.12, 0.10, 0.42),
    Geometry("fourier_carrier", 0.42, 0.0, 0.0, 0.74, 0.08, 0.09, 0.40),
    Geometry("additive_plus_web", 0.70, 0.0, 0.58, 0.0, 0.10, 0.13, 0.52),
    Geometry("additive_plus_fourier", 0.70, 0.0, 0.0, 0.58, 0.08, 0.12, 0.50),
    Geometry("gated_plus_web", 0.54, 0.58, 0.62, 0.0, 0.46, 0.14, 0.58),
    Geometry("gated_plus_fourier", 0.54, 0.58, 0.0, 0.62, 0.48, 0.13, 0.58),
    Geometry("web_plus_fourier_no_gate", 0.48, 0.0, 0.62, 0.62, 0.10, 0.17, 0.62),
    Geometry("hybrid_all_three_light", 0.56, 0.54, 0.54, 0.54, 0.52, 0.18, 0.70),
    Geometry("hybrid_all_three_balanced", 0.60, 0.68, 0.62, 0.60, 0.68, 0.20, 0.78),
    Geometry("hybrid_all_three_overbuilt", 0.74, 0.74, 0.72, 0.72, 0.72, 0.32, 0.92),
    Geometry("hybrid_no_add_payload", 0.20, 0.70, 0.66, 0.64, 0.70, 0.18, 0.74),
)


TASK_PRESSURE_MODES = (
    ("normal", 1.00),
    ("high_pressure", 1.18),
    ("heldout_shift", 1.08),
    ("echo_trap", 1.12),
)


def geometry_synergy(geometry: Geometry) -> float:
    synergy = 0.0
    synergy += 0.18 * min(geometry.additive, geometry.gate)
    synergy += 0.14 * min(geometry.gate, geometry.fourier)
    synergy += 0.12 * min(geometry.web, geometry.fourier)
    synergy += 0.10 * min(geometry.gate, geometry.web)
    return synergy


def run_trial(task: Task, geometry: Geometry, mode_name: str, pressure_multiplier: float, rng: random.Random) -> dict[str, object]:
    quant = QUANTS["damaged_q2"]
    pressure = clamp(task.pressure * pressure_multiplier, 0.0, 1.35)
    synergy = geometry_synergy(geometry)

    structural_signal = 0.18
    structural_signal += 0.34 * geometry.additive
    structural_signal += 0.18 * geometry.web
    structural_signal += 0.16 * geometry.fourier
    structural_signal += synergy
    structural_signal -= 0.38 * geometry.overhead
    structural_signal = clamp(structural_signal, 0.0, 1.0)

    gate_fit = 0.22 + 0.62 * geometry.gate + 0.16 * geometry.anti_echo
    if geometry.gate <= 0.05:
        gate_fit -= 0.18
    gate_open = rng.random() < clamp(gate_fit - 0.10 * pressure, 0.0, 0.995)
    if not gate_open and geometry.gate > 0:
        structural_signal *= 0.58

    packet_p = 0.42
    packet_p += 0.22 * geometry.additive
    packet_p += 0.25 * geometry.fourier
    packet_p += 0.14 * geometry.web
    packet_p += 0.12 * synergy
    packet_p -= 0.38 * geometry.overhead
    packet_received = rng.random() < clamp(packet_p, 0.0, 0.995)

    classification_p = quant["base_classifier"]
    classification_p += 0.20 * structural_signal
    classification_p += 0.16 * geometry.web
    classification_p += 0.10 * geometry.fourier
    classification_p += 0.08 * geometry.gate if gate_open else -0.06
    classification_p -= quant["noise"] * pressure * 0.42
    classification_p -= 0.20 * geometry.overhead
    if mode_name == "heldout_shift":
        classification_p -= 0.08 * (1.0 - geometry.web)
    classified = rng.random() < clamp(classification_p, 0.0, 0.995)

    required_active = 0
    missing_required = 0
    for primitive in task.required:
        activation_p = quant["base_activation"]
        activation_p += 0.26 * structural_signal
        activation_p += 0.13 * geometry.additive
        activation_p += 0.12 * geometry.web
        activation_p += 0.12 * geometry.fourier if primitive in ("VERIFY_RESULT", "ORIGINAL_TASK_RETURN") else 0.06 * geometry.fourier
        activation_p += 0.08 * geometry.gate if gate_open else -0.04
        activation_p -= quant["noise"] * pressure * (0.46 - 0.18 * geometry.gate)
        activation_p -= 0.16 * geometry.overhead
        if not classified:
            activation_p -= 0.13
        if not packet_received:
            activation_p -= 0.10
        if rng.random() < clamp(activation_p, 0.0, 0.995):
            required_active += 1
        else:
            missing_required += 1

    distractor_p = quant["distractor_pull"] * pressure
    distractor_p += 0.16 * geometry.additive
    distractor_p += 0.08 * geometry.fourier
    distractor_p += 0.06 * geometry.web
    distractor_p -= 0.18 * geometry.gate if gate_open else 0.0
    distractor_p -= 0.10 * geometry.anti_echo
    if mode_name == "echo_trap":
        distractor_p += 0.11
    active_distractors = sum(1 for _ in task.distractors if rng.random() < clamp(distractor_p, 0.0, 0.995))

    overdrive_p = 0.04
    overdrive_p += 0.28 * max(0.0, geometry.additive - 0.58)
    overdrive_p += 0.20 * max(0.0, geometry.fourier - 0.60)
    overdrive_p += 0.12 * max(0.0, geometry.web - 0.62)
    overdrive_p += 0.30 * geometry.overhead
    overdrive_p -= 0.26 * geometry.gate
    overdrive_p -= 0.20 * geometry.anti_echo
    overdrive = rng.random() < clamp(overdrive_p, 0.0, 0.98)

    echo_p = 0.03
    echo_p += 0.34 * geometry.additive
    echo_p += 0.22 * geometry.fourier
    echo_p += 0.08 * geometry.web
    echo_p += 0.10 if mode_name == "echo_trap" else 0.0
    echo_p += 0.20 if overdrive else 0.0
    echo_p -= 0.38 * geometry.gate
    echo_p -= 0.34 * geometry.anti_echo
    echo_p -= 0.08 * synergy
    echo = rng.random() < clamp(echo_p, 0.0, 0.98)

    missing_ratio = missing_required / len(task.required)
    cascade_p = quant["cascade"] * pressure
    cascade_p *= 1.0 + missing_ratio * 1.8
    cascade_p -= 0.20 * geometry.gate
    cascade_p -= 0.12 * geometry.web
    cascade_p -= 0.12 * geometry.fourier
    cascade_p -= 0.10 * synergy
    cascade_p += 0.12 if overdrive else 0.0
    cascade_p += 0.08 if echo else 0.0
    cascaded = rng.random() < clamp(cascade_p, 0.0, 0.98)

    repaired = 0
    if missing_required and geometry.web > 0 and geometry.gate > 0:
        repair_p = 0.14 + 0.28 * geometry.web + 0.24 * geometry.gate + 0.12 * geometry.fourier
        repair_p -= 0.18 * geometry.overhead
        for _ in range(missing_required):
            if rng.random() < clamp(repair_p, 0.0, 0.92):
                repaired += 1
        missing_required -= repaired
        required_active += repaired

    score = required_active / len(task.required)
    score += 0.08 * classified
    score += 0.05 * packet_received
    score -= 0.22 if cascaded else 0.0
    score -= 0.14 if echo else 0.0
    score -= 0.12 if overdrive else 0.0
    score -= min(0.22, 0.06 * active_distractors)
    score = clamp(score, 0.0, 1.0)

    passed = score >= 0.78 and not cascaded and missing_required <= 1
    clean_passed = passed and not echo and not overdrive
    return {
        "task_id": task.task_id,
        "family": task.family,
        "mode": mode_name,
        "passed": passed,
        "clean_passed": clean_passed,
        "score": score,
        "classified": classified,
        "packet_received": packet_received,
        "missing_count": missing_required,
        "repaired": repaired,
        "distractors": active_distractors,
        "cascade": cascaded,
        "echo": echo,
        "overdrive": overdrive,
        "gate_open": gate_open,
    }


def evaluate(geometry: Geometry, iterations: int) -> dict[str, object]:
    seed = 20260605 + sum(ord(ch) for ch in geometry.name)
    rng = random.Random(seed)
    rows = []
    for mode_name, pressure_multiplier in TASK_PRESSURE_MODES:
        for _ in range(iterations):
            for task in TASKS:
                rows.append(run_trial(task, geometry, mode_name, pressure_multiplier, rng))

    per_mode = {}
    for mode_name, _ in TASK_PRESSURE_MODES:
        group = [row for row in rows if row["mode"] == mode_name]
        per_mode[mode_name] = {
            "pass_rate": round(sum(1 for row in group if row["passed"]) / len(group), 4),
            "clean_pass_rate": round(sum(1 for row in group if row["clean_passed"]) / len(group), 4),
            "echo_rate": round(sum(1 for row in group if row["echo"]) / len(group), 4),
            "cascade_rate": round(sum(1 for row in group if row["cascade"]) / len(group), 4),
        }

    return {
        "geometry": geometry.name,
        "pass_rate": round(sum(1 for row in rows if row["passed"]) / len(rows), 4),
        "clean_pass_rate": round(sum(1 for row in rows if row["clean_passed"]) / len(rows), 4),
        "mean_score": round(sum(float(row["score"]) for row in rows) / len(rows), 4),
        "classified_rate": round(sum(1 for row in rows if row["classified"]) / len(rows), 4),
        "packet_rate": round(sum(1 for row in rows if row["packet_received"]) / len(rows), 4),
        "gate_open_rate": round(sum(1 for row in rows if row["gate_open"]) / len(rows), 4),
        "repair_rate": round(sum(int(row["repaired"]) for row in rows) / len(rows), 4),
        "echo_rate": round(sum(1 for row in rows if row["echo"]) / len(rows), 4),
        "overdrive_rate": round(sum(1 for row in rows if row["overdrive"]) / len(rows), 4),
        "cascade_rate": round(sum(1 for row in rows if row["cascade"]) / len(rows), 4),
        "avg_missing": round(sum(int(row["missing_count"]) for row in rows) / len(rows), 4),
        "avg_distractors": round(sum(int(row["distractors"]) for row in rows) / len(rows), 4),
        "train_complexity": geometry.train_complexity,
        "per_mode": per_mode,
    }


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    visible = [{key: value for key, value in row.items() if key != "per_mode"} for row in rows]
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(visible[0].keys()))
        writer.writeheader()
        writer.writerows(visible)


def main() -> int:
    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    iterations = 900
    rows = [evaluate(geometry, iterations) for geometry in GEOMETRIES]
    rows.sort(
        key=lambda row: (
            float(row["clean_pass_rate"]),
            float(row["pass_rate"]),
            -float(row["echo_rate"]),
            -float(row["cascade_rate"]),
            -float(row["train_complexity"]),
        ),
        reverse=True,
    )

    write_csv(REPORT_DIR / "geometry_results.csv", rows)
    (REPORT_DIR / "geometry_results.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")
    (REPORT_DIR / "report.json").write_text(
        json.dumps(
            {
                "description": "Toy simulation comparing additive, gated, Kronecker-web, Fourier-carrier, and hybrid LoRA geometry.",
                "iterations_per_task_per_mode": iterations,
                "task_pressure_modes": TASK_PRESSURE_MODES,
                "geometries": [geometry.__dict__ for geometry in GEOMETRIES],
                "results": rows,
            },
            indent=2,
        ),
        encoding="utf-8",
    )

    print("Hybrid LoRA geometry simulation")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Top geometries by clean pass rate")
    for index, row in enumerate(rows[:12], start=1):
        print(
            f"{index:02d}. {row['geometry']:<30} "
            f"clean={row['clean_pass_rate']:.3f} pass={row['pass_rate']:.3f} "
            f"score={row['mean_score']:.3f} echo={row['echo_rate']:.3f} "
            f"overdrive={row['overdrive_rate']:.3f} cascade={row['cascade_rate']:.3f} "
            f"repair={row['repair_rate']:.3f}"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
