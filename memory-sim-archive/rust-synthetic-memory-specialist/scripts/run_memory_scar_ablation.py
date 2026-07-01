from __future__ import annotations

import csv
import json
import os
import sys
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SCRIPT_DIR = ROOT / "scripts"
RUN_ROOT = ROOT / "rag_runs"

sys.path.insert(0, str(SCRIPT_DIR))
import run_ordo_extended_suite_benchmark as ordo_bench  # noqa: E402
import run_python_domain_scar_benchmark as py_bench  # noqa: E402


PYTHON_MODEL = os.environ.get("SCAR_ABLATION_PY_MODEL", os.environ.get("PYSCAR_MODEL", "qwen35-4b-q4km-chat"))
ORDO_MODEL = os.environ.get("SCAR_ABLATION_ORDO_MODEL", os.environ.get("ORDO_EXT_MODEL", "qwen35-4b-q4km-chat"))
MAX_REPAIR_ATTEMPTS = int(os.environ.get("SCAR_ABLATION_MAX_REPAIRS", "2"))
SUITE_FILTER = {
    item.strip().lower()
    for item in os.environ.get("SCAR_ABLATION_SUITES", "python,ordo").split(",")
    if item.strip()
}
CONDITION_FILTER = {
    item.strip()
    for item in os.environ.get("SCAR_ABLATION_CONDITIONS", "").split(",")
    if item.strip()
}
TASK_FILTER = {
    item.strip()
    for item in os.environ.get("SCAR_ABLATION_TASKS", "").split(",")
    if item.strip()
}

DEFAULT_PYTHON_TASKS = {"deep_merge", "flatten_json"}
DEFAULT_ORDO_TASKS = {"ramp_roundtrip", "audit_timeline"}

CONDITIONS = [
    "no_memory",
    "common_one",
    "common_all",
    "task_one",
    "task_three",
    "task_all",
    "common_one_task_one",
    "common_all_task_one",
    "common_all_task_three",
    "common_all_task_all",
    "routed_current",
    "cross_domain_noise",
]


def parse_task_blocks(path: Path) -> dict[str, list[str]]:
    blocks_by_task: dict[str, list[str]] = {}
    if not path.exists():
        return blocks_by_task
    for block in path.read_text(encoding="utf-8").split("---"):
        lines = [line.rstrip() for line in block.strip().splitlines() if line.strip()]
        if not lines or not lines[0].startswith("task="):
            continue
        task = lines[0].split("=", 1)[1].strip()
        body = "\n".join(lines[1:]).strip()
        if body:
            blocks_by_task.setdefault(task, []).append(body)
    return blocks_by_task


PY_BLOCKS = parse_task_blocks(ROOT / "data" / "python_domain_scar_memories.txt")
ORDO_BLOCKS = parse_task_blocks(ROOT / "data" / "ordo_4b_compile_scar_memories.txt")


def join_blocks(blocks: list[str]) -> str:
    return "\n\n".join(block.strip() for block in blocks if block.strip())


def all_blocks(blocks_by_task: dict[str, list[str]]) -> list[str]:
    merged: list[str] = []
    for task in sorted(blocks_by_task):
        merged.extend(blocks_by_task[task])
    return merged


def ablated_memory(
    *,
    suite: str,
    task_id: str,
    condition: str,
) -> str:
    if condition == "no_memory":
        return ""
    active_blocks = PY_BLOCKS if suite == "python" else ORDO_BLOCKS
    other_blocks = ORDO_BLOCKS if suite == "python" else PY_BLOCKS
    common = active_blocks.get("common", [])
    task = active_blocks.get(task_id, [])
    selected: list[str] = []
    if condition == "common_one":
        selected = common[:1]
    elif condition == "common_all":
        selected = common
    elif condition == "task_one":
        selected = task[:1]
    elif condition == "task_three":
        selected = task[:3]
    elif condition == "task_all":
        selected = task
    elif condition == "common_one_task_one":
        selected = common[:1] + task[:1]
    elif condition == "common_all_task_one":
        selected = common + task[:1]
    elif condition == "common_all_task_three":
        selected = common + task[:3]
    elif condition == "common_all_task_all":
        selected = common + task
    elif condition == "routed_current":
        if suite == "python":
            return py_bench.load_memories(task_id)
        return ordo_bench.memory_for_condition("routed_stack_memories", task_id)
    elif condition == "cross_domain_noise":
        selected = common + task + all_blocks(other_blocks)
    return join_blocks(selected)


def condition_rows() -> list[str]:
    if CONDITION_FILTER:
        return [condition for condition in CONDITIONS if condition in CONDITION_FILTER]
    return CONDITIONS


def selected_python_tasks() -> list[Any]:
    allowed = TASK_FILTER or DEFAULT_PYTHON_TASKS
    return [task for task in py_bench.TASKS if task.task_id in allowed]


def selected_ordo_tasks() -> list[Any]:
    allowed = TASK_FILTER or DEFAULT_ORDO_TASKS
    return [task for task in ordo_bench.TASKS if task.task_id in allowed]


def run_python(run_dir: Path) -> list[dict[str, Any]]:
    py_bench.MODEL = PYTHON_MODEL
    py_bench.MAX_REPAIR_ATTEMPTS = MAX_REPAIR_ATTEMPTS
    rows: list[dict[str, Any]] = []
    suite_dir = run_dir / "python"
    for condition in condition_rows():
        for task in selected_python_tasks():
            memory = ablated_memory(suite="python", task_id=task.task_id, condition=condition)
            row = py_bench.run_task(suite_dir, condition, task, memory)
            row["suite"] = "python"
            row["memory_chars"] = len(memory)
            row["scar_count_estimate"] = len([part for part in memory.split("\n\n") if part.strip()])
            rows.append(row)
    return rows


def run_ordo(run_dir: Path) -> list[dict[str, Any]]:
    ordo_bench.MODEL = ORDO_MODEL
    ordo_bench.API_MODE = "chat"
    ordo_bench.MAX_REPAIR_ATTEMPTS = MAX_REPAIR_ATTEMPTS
    rows: list[dict[str, Any]] = []
    suite_dir = run_dir / "ordo"
    for condition in condition_rows():
        for task in selected_ordo_tasks():
            memory = ablated_memory(suite="ordo", task_id=task.task_id, condition=condition)
            row = ordo_bench.run_task(suite_dir, condition, task, memory)
            row["suite"] = "ordo"
            row["memory_chars"] = len(memory)
            row["scar_count_estimate"] = len([part for part in memory.split("\n\n") if part.strip()])
            rows.append(row)
    return rows


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        return
    fields: list[str] = []
    for row in rows:
        for key in row:
            if key not in fields:
                fields.append(key)
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)


def summarize(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    summary: list[dict[str, Any]] = []
    keys = sorted({(str(row["suite"]), str(row["condition"])) for row in rows})
    for suite, condition in keys:
        group = [row for row in rows if row["suite"] == suite and row["condition"] == condition]
        passed = sum(1 for row in group if row["passed"])
        summary.append(
            {
                "suite": suite,
                "condition": condition,
                "tasks": len(group),
                "passed": passed,
                "pass_rate": round(passed / len(group), 4) if group else 0,
                "builder_passes": sum(1 for row in group if row.get("passed_stage") == "builder"),
                "avg_attempts": round(sum(int(row["attempts"]) for row in group) / len(group), 3) if group else 0,
                "avg_memory_chars": round(sum(int(row["memory_chars"]) for row in group) / len(group), 1) if group else 0,
                "avg_scar_count_estimate": round(
                    sum(int(row["scar_count_estimate"]) for row in group) / len(group), 1
                )
                if group
                else 0,
            }
        )
    return summary


def main() -> int:
    stamp = time.strftime("%Y%m%d-%H%M%S")
    run_dir = RUN_ROOT / f"memory-scar-ablation-{stamp}"
    run_dir.mkdir(parents=True, exist_ok=True)
    rows: list[dict[str, Any]] = []
    if "python" in SUITE_FILTER:
        rows.extend(run_python(run_dir))
    if "ordo" in SUITE_FILTER:
        rows.extend(run_ordo(run_dir))
    summary = summarize(rows)
    write_csv(run_dir / "trial_rows.csv", rows)
    write_csv(run_dir / "summary.csv", summary)
    (run_dir / "report.json").write_text(
        json.dumps(
            {
                "python_model": PYTHON_MODEL,
                "ordo_model": ORDO_MODEL,
                "max_repair_attempts": MAX_REPAIR_ATTEMPTS,
                "suites": sorted(SUITE_FILTER),
                "conditions": condition_rows(),
                "summary": summary,
            },
            indent=2,
        ),
        encoding="utf-8",
    )
    print()
    print(f"Run dir: {run_dir}")
    for row in summary:
        print(
            f"{row['suite']:<8} {row['condition']:<24} "
            f"{row['passed']}/{row['tasks']} pass_rate={row['pass_rate']:.3f} "
            f"scars~{row['avg_scar_count_estimate']}"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
