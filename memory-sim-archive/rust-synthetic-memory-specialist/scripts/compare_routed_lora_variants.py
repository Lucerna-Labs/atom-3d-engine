"""Compare routed LoRA variants in one run.

This script executes `run_routed_lora_no_memory_eval.py` for multiple named
model checkpoints and aggregates suite-level pass rates into a single CSV/JSON
report for fast A/B selection.
"""
from __future__ import annotations

import argparse
import csv
import json
import os
import re
import sys
import subprocess
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
RUNS_ROOT = ROOT / "rag_runs"
SCRIPT = ROOT / "scripts" / "run_routed_lora_no_memory_eval.py"


def parse_variant(arg: str) -> tuple[str, str]:
    if ":" not in arg:
        raise ValueError(f"Variant must be label:model (got {arg!r})")
    label, model = arg.split(":", 1)
    label = label.strip()
    model = model.strip()
    if not label or not model:
        raise ValueError(f"Variant must be label:model (got {arg!r})")
    return label, model


def run_single_variant(
    *,
    label: str,
    model: str,
    suites: str,
    tasks: str,
    max_repairs: int,
    timeout_s: int | None = None,
) -> tuple[dict[str, Any], Path]:
    env = os.environ.copy()
    env["ROUTED_LORA_MODEL"] = model
    if suites is not None:
        env["ROUTED_LORA_SUITES"] = suites
    if tasks is not None:
        env["ROUTED_LORA_TASKS"] = tasks
    env["ROUTED_LORA_MAX_REPAIRS"] = str(max_repairs)

    cmd = [sys.executable, str(SCRIPT)]

    print(f"\n=== running variant: {label} ===")
    print(f"model={model}")
    proc = subprocess.run(
        cmd,
        env=env,
        capture_output=True,
        text=True,
        timeout=timeout_s,
    )
    output = proc.stdout + "\n" + proc.stderr
    if proc.returncode != 0:
        raise RuntimeError(f"variant {label} failed (model={model})\n{output}")

    match = re.search(r"Run dir:\s*(.+)$", output, re.MULTILINE)
    if not match:
        raise RuntimeError(f"could not find run directory in output for {label}\n{output}")
    run_dir = Path(match.group(1).strip())
    summary_path = run_dir / "summary.csv"
    if not summary_path.exists():
        raise RuntimeError(f"missing summary.csv for {label} at {summary_path}")

    rows = []
    with summary_path.open("r", encoding="utf-8", newline="") as handle:
        reader = csv.DictReader(handle)
        for row in reader:
            rows.append(row)

    total_tasks = sum(int(row["tasks"]) for row in rows if row.get("tasks"))
    total_passed = sum(int(row["passed"]) for row in rows if row.get("passed"))
    total_rate = total_passed / total_tasks if total_tasks else 0.0
    return (
        {
            "label": label,
            "model": model,
            "run_dir": str(run_dir),
            "suites": rows,
            "total_passed": total_passed,
            "total_tasks": total_tasks,
            "total_pass_rate": round(total_rate, 4),
            "suite_count": len(rows),
        },
        run_dir,
    )


def write_report(results: list[dict[str, Any]], out_dir: Path) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    report_json = out_dir / "compare_report.json"
    report_csv = out_dir / "compare_report.csv"
    with report_json.open("w", encoding="utf-8") as handle:
        json.dump(
            {
                "results": results,
                "generated_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            },
            handle,
            indent=2,
        )

    fieldnames = [
        "label",
        "model",
        "run_dir",
        "suite",
        "condition",
        "tasks",
        "passed",
        "pass_rate",
        "builder_or_direct_passes",
        "avg_attempts",
    ]
    with report_csv.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        for result in results:
            for suite_row in result["suites"]:
                out = {
                    "label": result["label"],
                    "model": result["model"],
                    "run_dir": result["run_dir"],
                    "suite": suite_row.get("suite", ""),
                    "condition": suite_row.get("condition", ""),
                    "tasks": suite_row.get("tasks", ""),
                    "passed": suite_row.get("passed", ""),
                    "pass_rate": suite_row.get("pass_rate", ""),
                    "builder_or_direct_passes": suite_row.get("builder_or_direct_passes", ""),
                    "avg_attempts": suite_row.get("avg_attempts", ""),
                }
                writer.writerow(out)

            out_total = {
                "label": result["label"],
                "model": result["model"],
                "run_dir": result["run_dir"],
                "suite": "TOTAL",
                "condition": "no_external_memory",
                "tasks": str(result["total_tasks"]),
                "passed": str(result["total_passed"]),
                "pass_rate": f"{result['total_pass_rate']:.4f}",
                "builder_or_direct_passes": "",
                "avg_attempts": "",
            }
            writer.writerow(out_total)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--variant",
        action="append",
        required=True,
        help='Repeatable. Format label:model_name (example: routed:data/routed:1)',
    )
    parser.add_argument(
        "--suites",
        default="python,ordo,general,router",
        help="Comma list for ROUTED_LORA_SUITES",
    )
    parser.add_argument(
        "--tasks",
        default="",
        help="Comma list for ROUTED_LORA_TASKS (empty means all)",
    )
    parser.add_argument("--max-repairs", type=int, default=2)
    parser.add_argument(
        "--timeout-seconds",
        type=int,
        default=None,
        help="Timeout per variant run (seconds)",
    )
    parser.add_argument(
        "--label",
        default="ab",
        help="Directory label under rag_runs/routed-lora-ab-<label>-<ts>",
    )
    args = parser.parse_args()

    variants = [parse_variant(v) for v in args.variant]
    if not variants:
        raise SystemExit("No variants specified")

    out_dir = RUNS_ROOT / f"routed-lora-ab-{args.label}-{time.strftime('%Y%m%d-%H%M%S')}"
    results: list[dict[str, Any]] = []

    run_dirs = []
    for label, model in variants:
        result, run_dir = run_single_variant(
            label=label,
            model=model,
            suites=args.suites,
            tasks=args.tasks,
            max_repairs=args.max_repairs,
            timeout_s=args.timeout_seconds,
        )
        results.append(result)
        run_dirs.append(run_dir)

    write_report(results, out_dir)

    print(f"\ncompare report -> {out_dir}")
    print("variants:")
    for result in results:
        total = result["total_pass_rate"]
        print(
            f"- {result['label']:<20} {result['model']:<40} "
            f"total={result['total_passed']}/{result['total_tasks']} ({total:.3f})"
        )
        for suite_row in result["suites"]:
            print(
                f"  {suite_row['suite']:<10} "
                f"{suite_row['passed']}/{suite_row['tasks']} "
                f"pass_rate={float(suite_row['pass_rate']):.3f}"
            )
        print(f"  run: {result['run_dir']}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
