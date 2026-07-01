from __future__ import annotations

import csv
import json
import os
import re
import shutil
import subprocess
import textwrap
import time
import urllib.request
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
RUN_ROOT = ROOT / "rag_runs"
OLLAMA_CHAT_URL = "http://127.0.0.1:11434/api/chat"
MODEL = os.environ.get("PYSCAR_MODEL", "qwen35-4b-q4km-chat")
MAX_REPAIR_ATTEMPTS = int(os.environ.get("PYSCAR_MAX_REPAIRS", "2"))
CONDITION_FILTER = {
    item.strip()
    for item in os.environ.get("PYSCAR_CONDITIONS", "").split(",")
    if item.strip()
}
TASK_FILTER = {
    item.strip()
    for item in os.environ.get("PYSCAR_TASKS", "").split(",")
    if item.strip()
}


@dataclass(frozen=True)
class Task:
    task_id: str
    prompt: str
    tests: str


TASKS = [
    Task(
        task_id="csv_rollup",
        prompt="""
Build a Python module solution.py.

Expose this function:

    def rollup_sales(csv_text: str) -> list[tuple[str, int]]:

The input is CSV text with columns region,item,quantity.
Return a list of (region, total_quantity) tuples sorted alphabetically by region.
Ignore blank rows. Empty input or header-only input returns [].
Use only the Python standard library.
""",
        tests=r'''
import unittest
from solution import rollup_sales


class CsvRollupTests(unittest.TestCase):
    def test_rolls_up_and_sorts_regions(self):
        text = """region,item,quantity
west,bolt,3
east,nut,5
west,nut,7
north,gear,2
"""
        self.assertEqual(rollup_sales(text), [("east", 5), ("north", 2), ("west", 10)])

    def test_empty_and_header_only(self):
        self.assertEqual(rollup_sales(""), [])
        self.assertEqual(rollup_sales("region,item,quantity\n"), [])


if __name__ == "__main__":
    unittest.main()
''',
    ),
    Task(
        task_id="sliding_window",
        prompt="""
Build a Python module solution.py.

Expose this function:

    def detect_spikes(values: list[float], width: int, threshold: float) -> list[tuple[int, float]]:

For each index i starting at width, compare values[i] to the average of the previous width values only.
If values[i] > average * threshold, include (i, round(average, 2)).
The current value is not part of its own baseline.
Invalid width, invalid threshold, or too few values returns [].
Use only the Python standard library.
""",
        tests=r'''
import unittest
from solution import detect_spikes


class SlidingWindowTests(unittest.TestCase):
    def test_detects_using_previous_window_only(self):
        values = [10, 10, 10, 31, 12, 50]
        self.assertEqual(detect_spikes(values, 3, 2.0), [(3, 10.0), (5, 17.67)])

    def test_invalid_inputs(self):
        self.assertEqual(detect_spikes([1, 2], 3, 2.0), [])
        self.assertEqual(detect_spikes([1, 2, 3], 0, 2.0), [])
        self.assertEqual(detect_spikes([1, 2, 3], 2, 0), [])


if __name__ == "__main__":
    unittest.main()
''',
    ),
    Task(
        task_id="flatten_json",
        prompt="""
Build a Python module solution.py.

Expose this function:

    def flatten_json(value) -> dict[str, object]:

Flatten nested dictionaries and lists into path keys.
Dictionary keys are joined with dots. List indexes use square brackets.
Examples: user.name, user.tags[0], orders[1].total.
Only scalar leaves are stored, but empty dictionaries and empty lists are leaves too.
If the root value is scalar, return {"": value}.
Process dictionary keys in sorted order for deterministic output.
Use only the Python standard library.
""",
        tests=r'''
import unittest
from solution import flatten_json


class FlattenJsonTests(unittest.TestCase):
    def test_flattens_dicts_and_lists(self):
        data = {
            "user": {"name": "Ada", "tags": ["math", "code"]},
            "orders": [{"total": 5}, {"total": 8}],
        }
        self.assertEqual(
            flatten_json(data),
            {
                "orders[0].total": 5,
                "orders[1].total": 8,
                "user.name": "Ada",
                "user.tags[0]": "math",
                "user.tags[1]": "code",
            },
        )

    def test_empty_containers_and_root_scalar(self):
        self.assertEqual(flatten_json({"a": [], "b": {}}), {"a": [], "b": {}})
        self.assertEqual(flatten_json(7), {"": 7})


if __name__ == "__main__":
    unittest.main()
''',
    ),
    Task(
        task_id="topo_batches",
        prompt="""
Build a Python module solution.py.

Expose this function:

    def topo_batches(edges: list[tuple[str, str]]) -> list[list[str]]:

Each edge is (before, after), meaning before must appear in an earlier batch than after.
Return a list of batches. Each batch is a sorted list of all currently available nodes.
Include nodes that appear only as dependents.
If there is a cycle, return [].
Use only the Python standard library.
""",
        tests=r'''
import unittest
from solution import topo_batches


class TopoBatchTests(unittest.TestCase):
    def test_batches_are_stable_and_oriented(self):
        edges = [("parse", "plan"), ("fetch", "plan"), ("plan", "run"), ("lint", "run")]
        self.assertEqual(topo_batches(edges), [["fetch", "lint", "parse"], ["plan"], ["run"]])

    def test_cycle_returns_empty(self):
        self.assertEqual(topo_batches([("a", "b"), ("b", "a")]), [])


if __name__ == "__main__":
    unittest.main()
''',
    ),
    Task(
        task_id="deep_merge",
        prompt="""
Build a Python module solution.py.

Expose this function:

    def deep_merge(base: dict, override: dict) -> dict:

Return a new dictionary.
Merge dictionaries recursively only when both values are dictionaries.
Lists are replaced, not extended.
Scalars in override replace values from base.
An override value of None deletes that key from the returned dictionary.
Do not mutate base or override.
Use only the Python standard library.
""",
        tests=r'''
import copy
import unittest
from solution import deep_merge


class DeepMergeTests(unittest.TestCase):
    def test_merge_replace_delete_and_no_mutation(self):
        base = {"a": {"x": 1, "y": 2}, "b": [1, 2], "c": 3, "keep": {"z": 9}}
        override = {"a": {"y": 20}, "b": [9], "c": None, "d": 4}
        original_base = copy.deepcopy(base)
        original_override = copy.deepcopy(override)
        self.assertEqual(deep_merge(base, override), {"a": {"x": 1, "y": 20}, "b": [9], "d": 4, "keep": {"z": 9}})
        self.assertEqual(base, original_base)
        self.assertEqual(override, original_override)

    def test_delete_missing_key_is_ignored(self):
        self.assertEqual(deep_merge({"a": 1}, {"b": None}), {"a": 1})


if __name__ == "__main__":
    unittest.main()
''',
    ),
]


def load_memories(task_id: str) -> str:
    path = ROOT / "data" / "python_domain_scar_memories.txt"
    raw = path.read_text(encoding="utf-8")
    memories: list[str] = []
    for block in raw.split("---"):
        lines = [line.rstrip() for line in block.strip().splitlines() if line.strip()]
        if not lines or not lines[0].startswith("task="):
            continue
        task = lines[0].split("=", 1)[1].strip()
        if task in {"common", task_id}:
            memories.append("\n".join(lines[1:]).strip())
    return "\n\n".join(memory for memory in memories if memory)


def load_all_ordo_memories() -> str:
    path = ROOT / "data" / "ordo_4b_compile_scar_memories.txt"
    if not path.exists():
        return ""
    blocks: list[str] = []
    for block in path.read_text(encoding="utf-8").split("---"):
        lines = [line.rstrip() for line in block.strip().splitlines() if line.strip()]
        if not lines:
            continue
        if lines[0].startswith("task="):
            lines = lines[1:]
        text = "\n".join(lines).strip()
        if text:
            blocks.append(text)
    return "\n\n".join(blocks)


def memory_for_condition(condition: str, task_id: str) -> str:
    python_memory = load_memories(task_id)
    ordo_memory = load_all_ordo_memories()
    if condition in {"python_scar_memories", "routed_stack_memories"}:
        return python_memory
    if condition == "ordo_only_memories":
        return ordo_memory
    if condition == "python_plus_ordo_memories":
        return "\n\n".join(part for part in [python_memory, ordo_memory] if part)
    return ""


def call_ollama(prompt: str) -> str:
    payload = {
        "model": MODEL,
        "messages": [{"role": "user", "content": prompt}],
        "stream": False,
        "think": False,
        "options": {
            "temperature": 0,
            "num_predict": 1800,
            "num_ctx": 24000,
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


def extract_solution(raw: str) -> str:
    patterns = [
        r"```python\s*(.*?)```",
        r"```\s*(.*?)```",
    ]
    for pattern in patterns:
        match = re.search(pattern, raw, re.DOTALL | re.IGNORECASE)
        if match:
            return match.group(1).strip()
    marker = re.search(r"(?im)^solution\.py\s*:?\s*$", raw)
    if marker:
        return raw[marker.end() :].strip()
    return raw.strip()


def build_prompt(task: Task, memory: str) -> str:
    memory_block = f"\nRelevant lived memories:\n{memory}\n" if memory else ""
    return textwrap.dedent(
        f"""
        You are writing Python code for an automated verifier.
        Return only one file: solution.py.
        Use only the Python standard library.
        Do not include markdown prose outside the code.

        Task:
        {task.prompt.strip()}
        {memory_block}
        """
    ).strip()


def repair_prompt(task: Task, memory: str, raw: str, output: str) -> str:
    return textwrap.dedent(
        f"""
        The verifier imported and tested your solution.py, but it failed.
        Return a corrected solution.py only.
        Use only the Python standard library.

        Task:
        {task.prompt.strip()}

        Relevant lived memories:
        {memory}

        Verifier output:
        {output[-4000:]}

        Previous response:
        {raw[-5000:]}
        """
    ).strip()


def run_python_tests(task_dir: Path, solution: str, tests: str) -> tuple[bool, str]:
    if task_dir.exists():
        shutil.rmtree(task_dir)
    task_dir.mkdir(parents=True, exist_ok=True)
    (task_dir / "solution.py").write_text(solution + "\n", encoding="utf-8")
    (task_dir / "test_solution.py").write_text(tests, encoding="utf-8")
    try:
        result = subprocess.run(
            ["python", "-m", "unittest", "-v", "test_solution.py"],
            cwd=task_dir,
            capture_output=True,
            text=True,
            timeout=30,
        )
    except subprocess.TimeoutExpired as exc:
        output = (exc.stdout or "") + "\n" + (exc.stderr or "") + "\nTIMEOUT"
        return False, output
    output = (result.stdout or "") + "\n" + (result.stderr or "")
    return result.returncode == 0, output


def run_task(run_dir: Path, condition: str, task: Task, memory: str) -> dict[str, object]:
    task_dir = run_dir / condition / task.task_id
    prompt = build_prompt(task, memory)
    raw = call_ollama(prompt)
    solution = extract_solution(raw)
    passed, output = run_python_tests(task_dir / "attempt_0", solution, task.tests)
    attempts = 1
    stage = "builder" if passed else ""
    final_raw = raw
    final_solution = solution
    final_output = output
    while not passed and attempts <= MAX_REPAIR_ATTEMPTS:
        raw = call_ollama(repair_prompt(task, memory, raw, output))
        solution = extract_solution(raw)
        passed, output = run_python_tests(task_dir / f"attempt_{attempts}", solution, task.tests)
        attempts += 1
        final_raw = raw
        final_solution = solution
        final_output = output
        if passed:
            stage = f"repair_{attempts - 1}"
            break
    task_dir.mkdir(parents=True, exist_ok=True)
    (task_dir / "prompt.txt").write_text(prompt, encoding="utf-8")
    (task_dir / "final_raw_response.txt").write_text(final_raw, encoding="utf-8")
    (task_dir / "solution.py").write_text(final_solution + "\n", encoding="utf-8")
    (task_dir / "verifier_output.txt").write_text(final_output, encoding="utf-8")
    print(f"{condition:<22} | {task.task_id:<16} | {'PASS' if passed else 'FAIL'} | stage={stage or '-'} attempts={attempts}")
    return {
        "condition": condition,
        "task_id": task.task_id,
        "passed": passed,
        "passed_stage": stage,
        "attempts": attempts,
        "solution_chars": len(final_solution),
        "verifier_output_chars": len(final_output),
    }


def summarize(rows: list[dict[str, object]]) -> list[dict[str, object]]:
    summary: list[dict[str, object]] = []
    for condition in sorted({str(row["condition"]) for row in rows}):
        group = [row for row in rows if row["condition"] == condition]
        passed = [row for row in group if row["passed"]]
        summary.append(
            {
                "condition": condition,
                "tasks": len(group),
                "passed": len(passed),
                "pass_rate": round(len(passed) / len(group), 3) if group else 0.0,
                "builder_passes": sum(1 for row in group if row["passed_stage"] == "builder"),
                "avg_attempts": round(sum(int(row["attempts"]) for row in group) / len(group), 3) if group else 0.0,
            }
        )
    return summary


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        return
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)


def main() -> int:
    stamp = time.strftime("%Y%m%d-%H%M%S")
    run_dir = RUN_ROOT / f"python-domain-scar-{stamp}"
    run_dir.mkdir(parents=True, exist_ok=True)
    conditions = [
        "no_memory",
        "python_scar_memories",
        "ordo_only_memories",
        "python_plus_ordo_memories",
        "routed_stack_memories",
    ]
    if CONDITION_FILTER:
        conditions = [condition for condition in conditions if condition in CONDITION_FILTER]
    tasks = [task for task in TASKS if not TASK_FILTER or task.task_id in TASK_FILTER]
    rows: list[dict[str, object]] = []
    for condition in conditions:
        for task in tasks:
            memory = memory_for_condition(condition, task.task_id)
            rows.append(run_task(run_dir, condition, task, memory))
    summary = summarize(rows)
    write_csv(run_dir / "trial_rows.csv", rows)
    write_csv(run_dir / "summary.csv", summary)
    (run_dir / "report.json").write_text(
        json.dumps({"model": MODEL, "tasks": [task.task_id for task in tasks], "summary": summary}, indent=2),
        encoding="utf-8",
    )
    print()
    print(f"Run dir: {run_dir}")
    for row in summary:
        print(f"{row['condition']:<22} {row['passed']}/{row['tasks']} pass_rate={row['pass_rate']:.3f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
