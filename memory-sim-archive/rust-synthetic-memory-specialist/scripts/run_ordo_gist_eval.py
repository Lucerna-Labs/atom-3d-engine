"""Ordo gist-transfer eval: does a GIST corpus let a model build HELD-OUT Ordo apps
it has never seen, where the recall-based artifact_scars recipe cannot?

Reuses the extended-suite machinery (build/verify/repair, memory routing) from
run_ordo_extended_suite_benchmark. Adds:
  - a `gist` recipe pointing at data/ordo_gist_memories.txt (transferable principles)
  - a held-out suite of 5 NEW Ordo apps (cargo-verified reference solutions live in
    _holdout_verify/<id>/; their frozen tests are loaded here)
  - a suite loop: dev (the canonical 12) vs holdout (the 5 new apps)

Conditions: no_memory / ordo_gist / ordo_artifact_scars.
  dev: artifact_scars recalls (per-task scar + literal passing source) -> should ace.
  holdout: artifact_scars has nothing to recall (degenerates to common scars);
           gist supplies transferable principles -> the real transfer test.

Env: GIST_MODEL (default qwen35-4b-base-q6-raw), GIST_API (default generate),
     GIST_MAX_REPAIRS (default 2), GIST_SUITES (dev,holdout), GIST_CONDITIONS,
     GIST_TASKS (filter).
"""
from __future__ import annotations
import os, re, sys, time, csv, json
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

# Configure the base module's globals BEFORE importing it (it reads env at import).
os.environ["ORDO_EXT_MODEL"] = os.environ.get("GIST_MODEL", "qwen35-4b-base-q6-raw")
os.environ["ORDO_EXT_API"] = os.environ.get("GIST_API", "generate")
os.environ["ORDO_EXT_MAX_REPAIRS"] = os.environ.get("GIST_MAX_REPAIRS", "2")

import run_ordo_extended_suite_benchmark as base  # noqa: E402
from run_ordo_extended_suite_benchmark import Task  # noqa: E402

ROOT = base.ROOT
RUN_ROOT = base.RUN_ROOT
HV = ROOT / "_holdout_verify"

# Register the gist recipe (transferable principles, all task=common -> injected everywhere).
base.RECIPE_PATHS["gist"] = [ROOT / "data" / "ordo_gist_memories.txt"]
# gist + a complete compiling skeleton exemplar to adapt (the "stable-skeleton" middle ground).
base.RECIPE_PATHS["gist_skel"] = [
    ROOT / "data" / "ordo_gist_memories.txt",
    ROOT / "data" / "ordo_skeleton_exemplar.txt",
]


# --- hardened model call: capped generation, keep model loaded, timeout + retry, never crash the run ---
import urllib.request  # noqa: E402

_NUM_PREDICT = int(os.environ.get("GIST_NUM_PREDICT", "3000"))
_NUM_CTX = int(os.environ.get("GIST_NUM_CTX", "8192"))
_CALL_TIMEOUT = int(os.environ.get("GIST_CALL_TIMEOUT", "300"))


def _hardened_call_ollama(prompt: str) -> str:
    options = {
        "temperature": 0, "seed": 41,
        "num_predict": _NUM_PREDICT, "num_ctx": _NUM_CTX,
        "stop": ["\n### User", "\nUser:"],
    }
    if base.API_MODE == "generate":
        payload = {"model": base.MODEL, "prompt": prompt, "stream": False, "options": options, "keep_alive": "15m"}
        url = base.OLLAMA_GENERATE_URL
    else:
        payload = {"model": base.MODEL, "messages": [{"role": "user", "content": prompt}],
                   "stream": False, "think": False, "options": options, "keep_alive": "15m"}
        url = base.OLLAMA_CHAT_URL
    data = json.dumps(payload).encode("utf-8")
    last = None
    for _ in range(2):
        try:
            req = urllib.request.Request(url, data=data, headers={"Content-Type": "application/json"})
            with urllib.request.urlopen(req, timeout=_CALL_TIMEOUT) as resp:
                body = json.loads(resp.read().decode("utf-8"))
            return body["response"] if base.API_MODE == "generate" else body["message"]["content"]
        except Exception as exc:  # noqa: BLE001
            last = exc
            time.sleep(3)
    print(f"  [call_ollama failed after retries: {last!r}] -> empty (task will fail-build, run continues)")
    return ""


base.call_ollama = _hardened_call_ollama


def _tests(dirname: str) -> str:
    return (HV / dirname / "tests" / "runtime.rs").read_text(encoding="utf-8")


HOLDOUT = [
    Task(
        "deadletter_sink",
        "Build a deadletter sink: flow a message along graph edges to a terminal node; if that terminal node is not a registered target, collect it as a deadletter signal.",
        "Delivery { path: Vec<String>, delivered: bool, signals: Vec<String> } and DeadletterRuntime with new/connect/add_target/route.",
        "connect A B, target NODE, route START; route prints path=<a>b>c> delivered=<bool> signals=<\"none\" or csv>.",
        _tests("deadletter"),
    ),
    Task(
        "hop_budget",
        "Build a hop-budget flow: walk the first outgoing edge per node, one hop per edge, until the budget is spent or no edge remains; emit budget_exhausted if it stops on budget while an edge still existed.",
        "HopTrace { path: Vec<String>, signals: Vec<String> } and HopRuntime with new/connect/flow(start, budget).",
        "connect A B, flow START BUDGET; prints path=<a>b>c> signals=<\"none\" or csv>.",
        _tests("hop_budget"),
    ),
    Task(
        "fanin_merge",
        "Build a fan-in merge: register flows as emergent threads (ids from 1); a merge query reports, in thread order, which threads' routes include a given node.",
        "MergeTrace { node: String, threads: Vec<usize> } and MergeFabric with new/flow(&[&str])->usize/merge(node).",
        "flow a>b>c prints thread=<id> route=<a>b>c>; merge NODE prints merge=<node> threads=<csv or \"none\">.",
        _tests("fanin_merge"),
    ),
    Task(
        "priority_lane",
        "Build a priority-lane scheduler: nodes carry an integer priority; scheduling candidates picks the highest-priority known candidate (ties by candidate order) and emits lane:<node>.",
        "LanePlan { node: String, priority: u32, signals: Vec<String> } and LaneScheduler with new/add_node(name, priority)/schedule(&[&str])->Option<LanePlan>.",
        "node NAME PRIORITY, schedule A B C; prints node=<node> priority=<n> signals=lane:<node> (nothing if None).",
        _tests("priority_lane"),
    ),
    Task(
        "checkpoint_replay",
        "Build a checkpoint/replay log: events recorded with monotonic ids from 1; checkpoint captures the latest id; replay returns only events recorded strictly after the checkpoint.",
        "ReplayEvent { index: usize, kind: String, node: String } and ReplayLog with new/record(kind, node)/checkpoint()->usize/replay()->Vec<ReplayEvent>.",
        "record KIND NODE, checkpoint, replay; prints replay=<index:kind@node csv or \"none\">.",
        _tests("checkpoint_replay"),
    ),
]

SUITES = {"dev": list(base.TASKS), "holdout": HOLDOUT}


def main() -> int:
    suites = [s for s in os.environ.get("GIST_SUITES", "dev,holdout").split(",") if s.strip()]
    conditions = [c for c in os.environ.get("GIST_CONDITIONS", "no_memory,ordo_gist,ordo_artifact_scars").split(",") if c.strip()]
    task_filter = {t.strip() for t in os.environ.get("GIST_TASKS", "").split(",") if t.strip()}

    stamp = time.strftime("%Y%m%d-%H%M%S")
    slug = re.sub(r"[^A-Za-z0-9_.-]+", "-", base.MODEL).strip("-")
    run_dir = RUN_ROOT / f"ordo-gist-eval-{stamp}-{slug}"
    run_dir.mkdir(parents=True, exist_ok=True)
    print(f"model={base.MODEL} api={base.API_MODE} suites={suites} conditions={conditions}")

    rows: list[dict[str, object]] = []
    for suite in suites:
        tasks = [t for t in SUITES[suite] if not task_filter or t.task_id in task_filter]
        for condition in conditions:
            for task in tasks:
                memory = base.memory_for_condition(condition, task.task_id)
                row = base.run_task(run_dir / suite, condition, task, memory)
                row["suite"] = suite
                row["memory_chars"] = len(memory)
                rows.append(row)

    # summarize per (suite, condition)
    summary = []
    for suite in suites:
        for condition in conditions:
            grp = [r for r in rows if r["suite"] == suite and r["condition"] == condition]
            if not grp:
                continue
            summary.append({
                "suite": suite,
                "condition": condition,
                "tasks": len(grp),
                "passed": sum(1 for r in grp if r["passed"]),
                "builder_passes": sum(1 for r in grp if r["passed_stage"] == "builder"),
                "pass_rate": round(sum(1 for r in grp if r["passed"]) / len(grp), 4),
            })

    base.write_csv(run_dir / "trial_rows.csv", rows)
    base.write_csv(run_dir / "summary.csv", summary)
    (run_dir / "report.json").write_text(
        json.dumps({"model": base.MODEL, "api": base.API_MODE, "summary": summary}, indent=2), encoding="utf-8")
    print(f"\nRun dir: {run_dir}")
    for r in summary:
        print(f"{r['suite']:<8} {r['condition']:<22} {r['passed']}/{r['tasks']} (builder {r['builder_passes']}) rate={r['pass_rate']:.3f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
