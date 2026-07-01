from __future__ import annotations

import csv
import json
import os
import sys
import time
import urllib.request
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SCRIPT_DIR = ROOT / "scripts"
RUN_ROOT = ROOT / "rag_runs"
OLLAMA_CHAT_URL = "http://127.0.0.1:11434/api/chat"

MODEL = os.environ.get("ROUTED_LORA_MODEL", "qwen35-4b-q4km-chat")
SUITE_FILTER = {
    item.strip().lower()
    for item in os.environ.get("ROUTED_LORA_SUITES", "").split(",")
    if item.strip()
}
TASK_FILTER = {
    item.strip()
    for item in os.environ.get("ROUTED_LORA_TASKS", "").split(",")
    if item.strip()
}
MAX_REPAIR_ATTEMPTS = int(os.environ.get("ROUTED_LORA_MAX_REPAIRS", "2"))

sys.path.insert(0, str(SCRIPT_DIR))
import run_ordo_extended_suite_benchmark as ordo_bench  # noqa: E402
import run_python_domain_scar_benchmark as py_bench  # noqa: E402


GENERAL_TASKS = [
    {
        "task_id": "story_lighthouse_letter",
        "prompt": "Write a short story about a lighthouse keeper who finds a lost letter.",
        "must_contain_any": ["lighthouse", "letter"],
    },
    {
        "task_id": "cookie_recipe_multiplier",
        "prompt": "If a recipe makes 18 cookies and I need 54 cookies, how many times should I multiply the recipe?",
        "must_contain_any": ["3", "three"],
    },
    {
        "task_id": "vanilla_cake_steps",
        "prompt": "Give simple steps for baking a vanilla cake.",
        "must_contain_any": ["cake", "bake"],
    },
    {
        "task_id": "wet_houseplant_advice",
        "prompt": "My houseplant has yellow leaves and wet soil. What should I do first?",
        "must_contain_any": ["water", "soil", "dry"],
    },
]

CONTAMINATION_TERMS = [
    "solution.py",
    "rust_ordo_ext",
    "Cargo.toml",
    "python_domain_scar",
    "ordo_artifact_scar",
    "Routed memories",
    "Active corpus",
]


def call_ollama(prompt: str, system: str | None = None, max_tokens: int = 900) -> str:
    messages: list[dict[str, str]] = []
    if system:
        messages.append({"role": "system", "content": system})
    messages.append({"role": "user", "content": prompt})
    payload = {
        "model": MODEL,
        "messages": messages,
        "stream": False,
        "think": False,
        "options": {
            "temperature": 0,
            "seed": 41,
            "num_predict": max_tokens,
            "num_ctx": 12000,
        },
    }
    request = urllib.request.Request(
        OLLAMA_CHAT_URL,
        data=json.dumps(payload).encode("utf-8"),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(request, timeout=600) as response:
        data = json.loads(response.read().decode("utf-8"))
    return data.get("message", {}).get("content", "")


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


def run_python_suite(run_dir: Path) -> list[dict[str, Any]]:
    py_bench.MODEL = MODEL
    py_bench.MAX_REPAIR_ATTEMPTS = MAX_REPAIR_ATTEMPTS
    tasks = [task for task in py_bench.TASKS if not TASK_FILTER or task.task_id in TASK_FILTER]
    rows: list[dict[str, Any]] = []
    suite_dir = run_dir / "python"
    suite_dir.mkdir(parents=True, exist_ok=True)
    for task in tasks:
        row = py_bench.run_task(suite_dir, "no_external_memory", task, "")
        row["suite"] = "python"
        rows.append(row)
    return rows


def run_ordo_suite(run_dir: Path) -> list[dict[str, Any]]:
    ordo_bench.MODEL = MODEL
    ordo_bench.API_MODE = "chat"
    ordo_bench.MAX_REPAIR_ATTEMPTS = MAX_REPAIR_ATTEMPTS
    tasks = [task for task in ordo_bench.TASKS if not TASK_FILTER or task.task_id in TASK_FILTER]
    rows: list[dict[str, Any]] = []
    suite_dir = run_dir / "ordo"
    suite_dir.mkdir(parents=True, exist_ok=True)
    for task in tasks:
        row = ordo_bench.run_task(suite_dir, "no_external_memory", task, "")
        row["suite"] = "ordo"
        rows.append(row)
    return rows


def run_general_suite(run_dir: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    out_dir = run_dir / "general"
    out_dir.mkdir(parents=True, exist_ok=True)
    for task in GENERAL_TASKS:
        raw = call_ollama(task["prompt"], max_tokens=700)
        lower = raw.lower()
        contains_required = any(term.lower() in lower for term in task["must_contain_any"])
        contamination = [term for term in CONTAMINATION_TERMS if term.lower() in lower]
        passed = contains_required and not contamination
        (out_dir / f"{task['task_id']}.txt").write_text(raw, encoding="utf-8")
        print(
            f"general            | {task['task_id']:<24} | "
            f"{'PASS' if passed else 'FAIL'} | contamination={','.join(contamination) or '-'}"
        )
        rows.append(
            {
                "suite": "general",
                "condition": "no_external_memory",
                "task_id": task["task_id"],
                "passed": passed,
                "passed_stage": "direct" if passed else "",
                "attempts": 1,
                "response_chars": len(raw),
                "contamination": ",".join(contamination),
            }
        )
    return rows


def run_router_suite(run_dir: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    out_dir = run_dir / "router"
    out_dir.mkdir(parents=True, exist_ok=True)
    system = "Select the smallest active memory corpus for the task. Return compact JSON only. Do not solve the task."
    probes = [
        {
            "task_id": "route_python_deep_merge",
            "expected": "python_domain_scar",
            "prompt": (
                "Available corpora: python_domain_scar, ordo_artifact_scar.\n\n"
                "Task: Build a Python module solution.py exposing deep_merge(base, override)."
            ),
        },
        {
            "task_id": "route_ordo_ramp",
            "expected": "ordo_artifact_scar",
            "prompt": (
                "Available corpora: python_domain_scar, ordo_artifact_scar.\n\n"
                "Task: Build an Ordo ramp runtime with on, off, and trace commands."
            ),
        },
        {
            "task_id": "route_general_story",
            "expected": "none",
            "prompt": (
                "Available corpora: python_domain_scar, ordo_artifact_scar.\n\n"
                "Task: Write a short story about a lighthouse keeper."
            ),
        },
    ]
    for probe in probes:
        raw = call_ollama(probe["prompt"], system=system, max_tokens=300)
        expected = probe["expected"]
        passed = expected.lower() in raw.lower() if expected != "none" else (
            "python_domain_scar" not in raw and "ordo_artifact_scar" not in raw
        )
        (out_dir / f"{probe['task_id']}.txt").write_text(raw, encoding="utf-8")
        print(f"router             | {probe['task_id']:<24} | {'PASS' if passed else 'FAIL'}")
        rows.append(
            {
                "suite": "router",
                "condition": "no_external_memory",
                "task_id": probe["task_id"],
                "passed": passed,
                "passed_stage": "direct" if passed else "",
                "attempts": 1,
                "expected": expected,
                "response_chars": len(raw),
            }
        )
    return rows


def summarize(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    summary: list[dict[str, Any]] = []
    for suite in sorted({str(row["suite"]) for row in rows}):
        group = [row for row in rows if row["suite"] == suite]
        passed = sum(1 for row in group if row["passed"])
        summary.append(
            {
                "suite": suite,
                "condition": "no_external_memory",
                "tasks": len(group),
                "passed": passed,
                "pass_rate": round(passed / len(group), 4) if group else 0.0,
                "builder_or_direct_passes": sum(
                    1 for row in group if row.get("passed_stage") in {"builder", "direct"}
                ),
                "avg_attempts": round(sum(int(row["attempts"]) for row in group) / len(group), 3)
                if group
                else 0.0,
            }
        )
    return summary


def main() -> int:
    stamp = time.strftime("%Y%m%d-%H%M%S")
    run_dir = RUN_ROOT / f"routed-lora-no-memory-eval-{stamp}"
    run_dir.mkdir(parents=True, exist_ok=True)
    suites = SUITE_FILTER or {"python", "ordo", "general", "router"}
    rows: list[dict[str, Any]] = []
    if "python" in suites:
        rows.extend(run_python_suite(run_dir))
    if "ordo" in suites:
        rows.extend(run_ordo_suite(run_dir))
    if "general" in suites:
        rows.extend(run_general_suite(run_dir))
    if "router" in suites:
        rows.extend(run_router_suite(run_dir))

    summary = summarize(rows)
    write_csv(run_dir / "trial_rows.csv", rows)
    write_csv(run_dir / "summary.csv", summary)
    (run_dir / "report.json").write_text(
        json.dumps(
            {
                "model": MODEL,
                "condition": "no_external_memory",
                "max_repair_attempts": MAX_REPAIR_ATTEMPTS,
                "summary": summary,
            },
            indent=2,
        ),
        encoding="utf-8",
    )
    print()
    print(f"Run dir: {run_dir}")
    for row in summary:
        print(f"{row['suite']:<10} {row['passed']}/{row['tasks']} pass_rate={row['pass_rate']:.3f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
