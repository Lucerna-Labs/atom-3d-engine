from __future__ import annotations

import csv
import json
from dataclasses import replace
from pathlib import Path

from simulate_hybrid_lora_geometry import GEOMETRIES, Geometry, evaluate


ROOT = Path(__file__).resolve().parents[1]
REPORT_DIR = ROOT / "runs" / "hybrid-dataset-coupling-sim"


DATASET_PROFILES = (
    {
        "name": "current_structural_v0_1",
        "description": "The dataset just trained: structural primitive payload, light implicit gain clamp, weak explicit stop behavior.",
        "additive_boost": 0.06,
        "gate_boost": 0.02,
        "web_boost": 0.02,
        "fourier_boost": 0.00,
        "anti_echo_boost": 0.04,
        "overhead_delta": 0.02,
        "complexity_delta": 0.00,
    },
    {
        "name": "v0_1_plus_clean_stop",
        "description": "Current dataset plus explicit clean-stop and no-continuation examples.",
        "additive_boost": 0.06,
        "gate_boost": 0.08,
        "web_boost": 0.02,
        "fourier_boost": 0.00,
        "anti_echo_boost": 0.26,
        "overhead_delta": 0.03,
        "complexity_delta": 0.04,
    },
    {
        "name": "hybrid_tagged_v0_2",
        "description": "Current dataset plus examples that explicitly teach gate, web association, carrier rhythm, and clean stop.",
        "additive_boost": 0.08,
        "gate_boost": 0.18,
        "web_boost": 0.16,
        "fourier_boost": 0.14,
        "anti_echo_boost": 0.22,
        "overhead_delta": 0.05,
        "complexity_delta": 0.10,
    },
    {
        "name": "hybrid_overstuffed_v0_2",
        "description": "Too many hybrid examples and repeated primitive labels; strong signal but more clutter.",
        "additive_boost": 0.14,
        "gate_boost": 0.22,
        "web_boost": 0.22,
        "fourier_boost": 0.20,
        "anti_echo_boost": 0.18,
        "overhead_delta": 0.14,
        "complexity_delta": 0.18,
    },
    {
        "name": "gate_web_carrier_minimal",
        "description": "Compact dataset focused only on gate, web, carrier, and stop behavior with less payload repetition.",
        "additive_boost": -0.02,
        "gate_boost": 0.20,
        "web_boost": 0.18,
        "fourier_boost": 0.16,
        "anti_echo_boost": 0.28,
        "overhead_delta": -0.02,
        "complexity_delta": 0.08,
    },
)


TARGET_GEOMETRIES = (
    "standard_lora_additive",
    "gated_lora",
    "gated_plus_web",
    "gated_plus_fourier",
    "hybrid_all_three_balanced",
    "hybrid_all_three_overbuilt",
)


def clamp01(value: float) -> float:
    return max(0.0, min(0.98, value))


def apply_profile(geometry: Geometry, profile: dict[str, object]) -> Geometry:
    return replace(
        geometry,
        name=f"{geometry.name}__{profile['name']}",
        additive=clamp01(geometry.additive + float(profile["additive_boost"])),
        gate=clamp01(geometry.gate + float(profile["gate_boost"])),
        web=clamp01(geometry.web + float(profile["web_boost"])),
        fourier=clamp01(geometry.fourier + float(profile["fourier_boost"])),
        anti_echo=clamp01(geometry.anti_echo + float(profile["anti_echo_boost"])),
        overhead=max(0.0, geometry.overhead + float(profile["overhead_delta"])),
        train_complexity=max(0.0, geometry.train_complexity + float(profile["complexity_delta"])),
    )


def visible_name(row_name: str) -> tuple[str, str]:
    geometry, profile = row_name.split("__", 1)
    return geometry, profile


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    visible = [{key: value for key, value in row.items() if key != "per_mode"} for row in rows]
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(visible[0].keys()))
        writer.writeheader()
        writer.writerows(visible)


def main() -> int:
    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    iterations = 2400
    base_geometries = [geometry for geometry in GEOMETRIES if geometry.name in TARGET_GEOMETRIES]

    rows = []
    for profile in DATASET_PROFILES:
        for geometry in base_geometries:
            profiled = apply_profile(geometry, profile)
            result = evaluate(profiled, iterations)
            geometry_name, profile_name = visible_name(str(result["geometry"]))
            result["geometry"] = geometry_name
            result["dataset_profile"] = profile_name
            result["profile_description"] = profile["description"]
            rows.append(result)

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

    write_csv(REPORT_DIR / "dataset_coupling_results.csv", rows)
    (REPORT_DIR / "dataset_coupling_results.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")
    (REPORT_DIR / "report.json").write_text(
        json.dumps(
            {
                "description": "Toy simulation of how dataset shape couples with hybrid LoRA geometry.",
                "iterations_per_task_per_mode": iterations,
                "dataset_profiles": DATASET_PROFILES,
                "target_geometries": TARGET_GEOMETRIES,
                "results": rows,
            },
            indent=2,
        ),
        encoding="utf-8",
    )

    print("Hybrid dataset coupling simulation")
    print(f"Reports: {REPORT_DIR}")
    print()
    print("Top results")
    for index, row in enumerate(rows[:14], start=1):
        print(
            f"{index:02d}. {row['geometry']:<28} {row['dataset_profile']:<26} "
            f"clean={row['clean_pass_rate']:.3f} pass={row['pass_rate']:.3f} "
            f"echo={row['echo_rate']:.3f} cascade={row['cascade_rate']:.3f} "
            f"complexity={row['train_complexity']:.2f}"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
