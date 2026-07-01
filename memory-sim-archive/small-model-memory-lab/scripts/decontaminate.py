"""Decontaminate the L4 and L5 datasets against in-repo benchmark probes.

Drop any L4/L5 row whose text has >= 2 distinct 5-gram overlaps with any
probe task in benchmarks/computation_damage_tasks.json or
benchmarks/capability_tasks.json.

Writes cleaned train.jsonl / validation.jsonl + a decontamination_report.json
back into each dataset directory.

Usage:
    python scripts/decontaminate.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from ordo_common import (  # noqa: E402
    decontaminate,
    read_jsonl,
    write_jsonl,
    write_manifest,
)


# Datasets we generated; each gets a decontam pass.
TARGETS = [
    "l4_math_v0_1",
    "l4_rust_v0_1",
    "l4_swe_v0_1",
    "l4_research_v0_1",
    "l5_orchestration_v0_1",
]

THRESHOLD = 2  # >= 2 distinct 5-gram overlaps -> drop


def main() -> None:
    report: dict = {"threshold_5gram": THRESHOLD, "datasets": {}}
    for name in TARGETS:
        d = ROOT / "data" / name
        train_path = d / "train.jsonl"
        val_path = d / "validation.jsonl"
        if not train_path.exists():
            print(f"skip {name} (no train.jsonl)")
            continue
        train = read_jsonl(train_path)
        val = read_jsonl(val_path) if val_path.exists() else []

        kept_train, dropped_train = decontaminate(train, ROOT, threshold=THRESHOLD)
        kept_val, dropped_val = decontaminate(val, ROOT, threshold=THRESHOLD)

        # Write cleaned versions. Save dropped rows to dropped.jsonl for audit.
        write_jsonl(train_path, kept_train)
        if val:
            write_jsonl(val_path, kept_val)
        write_jsonl(d / "dropped.jsonl", dropped_train + dropped_val)

        report["datasets"][name] = {
            "train_input": len(train),
            "train_kept": len(kept_train),
            "train_dropped": len(dropped_train),
            "val_input": len(val),
            "val_kept": len(kept_val),
            "val_dropped": len(dropped_val),
            "drop_rate": round(
                (len(dropped_train) + len(dropped_val)) / max(1, len(train) + len(val)),
                4,
            ),
        }
        print(
            f"{name}: train {len(kept_train)}/{len(train)} kept, "
            f"val {len(kept_val)}/{len(val)} kept, "
            f"drop_rate={report['datasets'][name]['drop_rate']}"
        )

    # Refresh manifests
    for name in TARGETS:
        d = ROOT / "data" / name
        manifest_path = d / "manifest.json"
        if not manifest_path.exists():
            continue
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        manifest["decontamination"] = report["datasets"].get(name, {})
        write_manifest(manifest_path, manifest)

    # Write the global report
    out = ROOT / "data" / "decontamination_report.json"
    out.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"wrote {out}")


if __name__ == "__main__":
    main()