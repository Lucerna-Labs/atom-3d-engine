from __future__ import annotations

import csv
import json
import os
import re
import shutil
import subprocess
import time
import urllib.request
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
RUN_ROOT = ROOT / "rag_runs"
OLLAMA_CHAT_URL = "http://127.0.0.1:11434/api/chat"
MODEL = os.environ.get("ORDO_SUITE_MODEL", "qwen3-coder-25b-a3b-rust-q5-chat")
CONDITION_FILTER = {
    item.strip()
    for item in os.environ.get("ORDO_SUITE_CONDITIONS", "").split(",")
    if item.strip()
}


@dataclass(frozen=True)
class OrdoBuildTask:
    task_id: str
    description: str
    required_api: str
    cli_contract: str
    tests: str


COMMON_TEST_HELPER = r'''
use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_suite");
    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn app");
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"),
        String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"),
    )
}
'''


TASKS = [
    OrdoBuildTask(
        task_id="mini_runtime",
        description=(
            "Build a miniature Ordo runtime with a ground graph, ramp edges, emergent thread traces, "
            "backpressure vibration signals, preload targets, budget routing, and a stdin CLI."
        ),
        required_api="""
pub struct Message { pub topic: String, pub payload: String }
pub struct Trace { pub thread_id: usize, pub path: Vec<String>, pub promoted: bool, pub signals: Vec<String> }
pub struct OrdoRuntime
impl OrdoRuntime {
    pub fn new() -> Self
    pub fn add_node(&mut self, id: &str)
    pub fn connect(&mut self, from: &str, to: &str)
    pub fn add_ramp(&mut self, from: &str, to: &str)
    pub fn set_capacity(&mut self, node: &str, capacity: usize)
    pub fn set_cost(&mut self, node: &str, cost: u32)
    pub fn preload_targets(&self, node: &str) -> Vec<String>
    pub fn route_for_budget(&self, candidates: &[&str], max_cost: u32) -> Option<String>
    pub fn publish(&mut self, from: &str, topic: &str, payload: &str) -> Trace
}
""".strip(),
        cli_contract="""
Commands: node NAME, connect A B, ramp A B, capacity NAME N, cost NAME N, publish FROM TOPIC PAYLOAD...
On publish print exactly: thread=<id> path=a>b>c promoted=true signals=backpressure:c
""".strip(),
        tests=COMMON_TEST_HELPER
        + r'''
use rust_ordo_suite::{Message, OrdoRuntime, Trace};

#[test]
fn mini_runtime_routes_and_signals() {
    let mut rt = OrdoRuntime::new();
    rt.add_node("input");
    rt.add_node("filter");
    rt.add_node("model");
    rt.connect("input", "filter");
    rt.add_ramp("filter", "model");
    rt.set_capacity("model", 0);

    let trace = rt.publish("input", "task", "hello");
    assert_eq!(trace.thread_id, 1);
    assert_eq!(trace.path, vec!["input", "filter", "model"]);
    assert!(trace.promoted);
    assert_eq!(trace.signals, vec!["backpressure:model"]);
    assert_eq!(rt.preload_targets("input"), vec!["filter"]);
    assert_eq!(rt.route_for_budget(&["model"], 0), Some("model".to_string()));

    let msg = Message { topic: "task".to_string(), payload: "hello".to_string() };
    let manual = Trace { thread_id: 9, path: vec!["x".to_string()], promoted: false, signals: vec![] };
    assert_eq!(msg.topic, "task");
    assert_eq!(manual.thread_id, 9);
}

#[test]
fn mini_runtime_cli() {
    let script = "\
node input
node filter
node model
connect input filter
ramp filter model
capacity model 0
publish input task hello
";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "thread=1 path=input>filter>model promoted=true signals=backpressure:model\n");
}
''',
    ),
    OrdoBuildTask(
        task_id="intersection_fabric",
        description=(
            "Build an Ordo intersection fabric where threads form from flow routes, intersections emerge from "
            "shared nodes, and messages can transfer between threads at the first shared node."
        ),
        required_api="""
pub struct TransferTrace { pub from_thread: usize, pub to_thread: usize, pub at: String, pub payload: String }
pub struct Fabric
impl Fabric {
    pub fn new() -> Self
    pub fn flow(&mut self, route: &[&str]) -> usize
    pub fn intersections(&self) -> Vec<String>
    pub fn transfer(&self, from_thread: usize, to_thread: usize, payload: &str) -> Option<TransferTrace>
}
""".strip(),
        cli_contract="""
Commands: flow a>b>c, transfer FROM_THREAD TO_THREAD PAYLOAD...
flow prints: thread=<id> route=a>b>c
transfer prints: transfer=<from>><to> at=<node> payload=<payload>
""".strip(),
        tests=COMMON_TEST_HELPER
        + r'''
use rust_ordo_suite::{Fabric, TransferTrace};

#[test]
fn intersections_emerge_from_flowing_threads() {
    let mut fabric = Fabric::new();
    let first = fabric.flow(&["a", "b", "c"]);
    let second = fabric.flow(&["x", "b", "z"]);
    let third = fabric.flow(&["q", "r"]);

    assert_eq!(first, 1);
    assert_eq!(second, 2);
    assert_eq!(third, 3);
    assert_eq!(fabric.intersections(), vec!["b"]);

    let transfer = fabric.transfer(first, second, "hello").unwrap();
    assert_eq!(transfer, TransferTrace {
        from_thread: 1,
        to_thread: 2,
        at: "b".to_string(),
        payload: "hello".to_string(),
    });
    assert!(fabric.transfer(first, third, "nope").is_none());
}

#[test]
fn intersection_cli() {
    let script = "\
flow a>b>c
flow x>b>z
transfer 1 2 hello
";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "thread=1 route=a>b>c\nthread=2 route=x>b>z\ntransfer=1>2 at=b payload=hello\n");
}
''',
    ),
    OrdoBuildTask(
        task_id="backpressure_scheduler",
        description=(
            "Build an Ordo orchestration seed that schedules tasks across candidate nodes using preference order, "
            "budget, capacity, ramp promotion, and backpressure vibration signals."
        ),
        required_api="""
pub struct Plan { pub task_id: String, pub node: String, pub promoted: bool, pub signals: Vec<String> }
pub struct Scheduler
impl Scheduler {
    pub fn new() -> Self
    pub fn add_node(&mut self, id: &str, cost: u32, capacity: usize)
    pub fn add_ramp(&mut self, from: &str, to: &str)
    pub fn schedule(&self, task_id: &str, candidates: &[&str], max_cost: u32) -> Option<Plan>
}
""".strip(),
        cli_contract="""
Commands: node NAME COST CAPACITY, ramp FROM TO, schedule TASK_ID BUDGET CANDIDATE...
schedule prints exactly: task=<id> node=<node> promoted=<bool> signals=<comma-list-or-none>
""".strip(),
        tests=COMMON_TEST_HELPER
        + r'''
use rust_ordo_suite::{Plan, Scheduler};

#[test]
fn scheduler_uses_budget_capacity_and_ramp_promotion() {
    let mut scheduler = Scheduler::new();
    scheduler.add_node("local", 0, 0);
    scheduler.add_node("cloud", 5, 2);
    scheduler.add_node("expensive", 10, 2);
    scheduler.add_ramp("local", "cloud");

    let plan = scheduler.schedule("job", &["local", "cloud", "expensive"], 5).unwrap();
    assert_eq!(plan, Plan {
        task_id: "job".to_string(),
        node: "cloud".to_string(),
        promoted: true,
        signals: vec!["backpressure:local".to_string()],
    });
    assert!(scheduler.schedule("job", &["expensive"], 5).is_none());
}

#[test]
fn scheduler_cli() {
    let script = "\
node local 0 0
node cloud 5 2
ramp local cloud
schedule job 5 local cloud
";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "task=job node=cloud promoted=true signals=backpressure:local\n");
}
''',
    ),
]


def load_jsonl_memories(path: Path, task_id: str) -> str:
    memories: list[str] = []
    if path.exists():
        with path.open("r", encoding="utf-8") as handle:
            for line in handle:
                line = line.strip()
                if not line:
                    continue
                item = json.loads(line)
                task = str(item.get("task", "common"))
                if task in {"common", task_id}:
                    memory = str(item.get("memory", "")).strip()
                    if memory:
                        memories.append(memory)
    shared = ROOT / "data" / "ordo_runtime_memory_build_corpus.jsonl"
    if shared.exists():
        with shared.open("r", encoding="utf-8") as handle:
            for line in handle:
                item = json.loads(line)
                memory = str(item.get("memory", "")).strip()
                if memory:
                    memories.append(memory)
    return "\n\n".join(memories)


def call_ollama(prompt: str) -> str:
    payload = {
        "model": MODEL,
        "messages": [{"role": "user", "content": prompt}],
        "stream": False,
        "think": False,
        "options": {
            "temperature": 0,
            "seed": 31,
            "num_predict": 4200,
            "num_ctx": 16384,
            "stop": ["\n### User", "\nUser:"],
        },
    }
    data = json.dumps(payload).encode("utf-8")
    request = urllib.request.Request(OLLAMA_CHAT_URL, data=data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(request, timeout=480) as response:
        body = json.loads(response.read().decode("utf-8"))
    return body["message"]["content"]


def strip_fence(text: str) -> str:
    text = text.strip()
    fence = re.search(r"```(?:rust|toml|text)?\s*(.*?)```", text, flags=re.DOTALL | re.IGNORECASE)
    if fence:
        return fence.group(1).strip()
    return text


def extract_file(raw: str, filename: str) -> str:
    escaped = re.escape(filename)
    marker = re.search(
        rf"(?:^|\n)\s*(?:===\s*)?{escaped}(?:\s*===)?\s*\n(.*?)(?=\n\s*(?:===\s*)?(?:Cargo\.toml|src/lib\.rs|src/main\.rs)(?:\s*===)?\s*\n|\Z)",
        raw,
        flags=re.DOTALL | re.IGNORECASE,
    )
    if marker:
        return strip_fence(marker.group(1))
    if filename.endswith(".rs"):
        fences = re.findall(r"```rust\s*(.*?)```", raw, flags=re.DOTALL | re.IGNORECASE)
        if filename == "src/lib.rs" and fences:
            return fences[0].strip()
        if filename == "src/main.rs" and len(fences) > 1:
            return fences[1].strip()
    if filename == "Cargo.toml":
        fence = re.search(r"```toml\s*(.*?)```", raw, flags=re.DOTALL | re.IGNORECASE)
        if fence:
            return fence.group(1).strip()
    return ""


def extract_project(raw: str) -> dict[str, str]:
    return {
        "Cargo.toml": extract_file(raw, "Cargo.toml"),
        "src/lib.rs": extract_file(raw, "src/lib.rs"),
        "src/main.rs": extract_file(raw, "src/main.rs"),
    }


def default_cargo(cargo: str) -> str:
    if "[package]" in cargo and "rust_ordo_suite" in cargo:
        return cargo
    return '[package]\nname = "rust_ordo_suite"\nversion = "0.1.0"\nedition = "2021"\n'


def write_project(crate_dir: Path, project: dict[str, str], tests: str) -> None:
    if crate_dir.exists():
        shutil.rmtree(crate_dir)
    (crate_dir / "src").mkdir(parents=True)
    (crate_dir / "tests").mkdir(parents=True)
    (crate_dir / "Cargo.toml").write_text(default_cargo(project.get("Cargo.toml", "")) + "\n", encoding="utf-8")
    (crate_dir / "src" / "lib.rs").write_text(project.get("src/lib.rs", "") + "\n", encoding="utf-8")
    (crate_dir / "src" / "main.rs").write_text(project.get("src/main.rs", "") + "\n", encoding="utf-8")
    (crate_dir / "tests" / "ordo_suite.rs").write_text(tests + "\n", encoding="utf-8")


def run_cargo_test(crate_dir: Path) -> tuple[bool, str]:
    try:
        completed = subprocess.run(
            ["cargo", "test", "--quiet"],
            cwd=crate_dir,
            text=True,
            capture_output=True,
            timeout=120,
        )
    except subprocess.TimeoutExpired as exc:
        output = (exc.stdout or "") + "\n" + (exc.stderr or "") + "\nTIMEOUT"
        return False, output
    output = (completed.stdout or "") + (completed.stderr or "")
    return completed.returncode == 0, output


def build_prompt(task: OrdoBuildTask, memory: str) -> str:
    memory_section = ""
    if memory:
        memory_section = f"""
Relevant lived memories:
{memory}
"""
    return f"""
Build this Ordo-shaped Rust app.

Task:
{task.description}

Required public API:
{task.required_api}

CLI contract:
{task.cli_contract}

Requirements:
- Output exactly three files: Cargo.toml, src/lib.rs, src/main.rs.
- Use only the Rust standard library.
- Package name must be rust_ordo_suite.
- No explanations outside file blocks.
- Keep the Ordo/spiderweb shape: flowing threads, intersections or ramps where requested, and vibration/backpressure signals where requested.
{memory_section}
Use this exact response shape:
=== Cargo.toml ===
```toml
[package]
name = "rust_ordo_suite"
version = "0.1.0"
edition = "2021"
```
=== src/lib.rs ===
```rust
// library code here
```
=== src/main.rs ===
```rust
// binary adapter here
```
""".strip()


def repair_prompt(task: OrdoBuildTask, memory: str, raw: str, cargo_output: str) -> str:
    return f"""
Repair this Ordo-shaped Rust app so it compiles and passes the tests.

Task:
{task.description}

Required public API:
{task.required_api}

Relevant lived memories:
{memory}

Cargo/test output:
```text
{cargo_output[-5000:]}
```

Previous response:
```text
{raw[-7000:]}
```

Return exactly:
=== Cargo.toml ===
```toml
...
```
=== src/lib.rs ===
```rust
...
```
=== src/main.rs ===
```rust
...
```
""".strip()


def run_task(run_dir: Path, condition: str, task: OrdoBuildTask, memory: str) -> dict[str, object]:
    task_dir = run_dir / condition / task.task_id
    task_dir.mkdir(parents=True, exist_ok=True)
    crate_dir = task_dir / "crate"

    prompt = build_prompt(task, memory)
    start = time.time()
    raw = call_ollama(prompt)
    builder_elapsed = time.time() - start
    project = extract_project(raw)
    write_project(crate_dir, project, task.tests)
    passed, cargo_output = run_cargo_test(crate_dir)
    stage = "builder" if passed else ""
    attempts = 1

    repair_elapsed = 0.0
    if not passed:
        start = time.time()
        raw = call_ollama(repair_prompt(task, memory, raw, cargo_output))
        repair_elapsed = time.time() - start
        attempts += 1
        project = extract_project(raw)
        write_project(crate_dir, project, task.tests)
        passed, cargo_output = run_cargo_test(crate_dir)
        stage = "repair_1" if passed else ""

    (task_dir / "prompt.txt").write_text(prompt, encoding="utf-8")
    (task_dir / "final_raw_response.txt").write_text(raw, encoding="utf-8")
    (task_dir / "Cargo.toml").write_text(project.get("Cargo.toml", ""), encoding="utf-8")
    (task_dir / "lib.rs").write_text(project.get("src/lib.rs", ""), encoding="utf-8")
    (task_dir / "main.rs").write_text(project.get("src/main.rs", ""), encoding="utf-8")
    (task_dir / "cargo_output.txt").write_text(cargo_output, encoding="utf-8")

    row = {
        "condition": condition,
        "task_id": task.task_id,
        "passed": passed,
        "passed_stage": stage,
        "attempts": attempts,
        "builder_elapsed_sec": round(builder_elapsed, 3),
        "repair_elapsed_sec": round(repair_elapsed, 3),
        "lib_chars": len(project.get("src/lib.rs", "")),
        "main_chars": len(project.get("src/main.rs", "")),
        "cargo_output_chars": len(cargo_output),
    }
    print(
        f"{condition:<18} | {task.task_id:<24} | {'PASS' if passed else 'FAIL'} | "
        f"stage={stage or '-'} attempts={attempts} builder={builder_elapsed:.1f}s repair={repair_elapsed:.1f}s"
    )
    return row


def summarize(rows: list[dict[str, object]]) -> list[dict[str, object]]:
    summary = []
    for condition in sorted({str(row["condition"]) for row in rows}):
        group = [row for row in rows if row["condition"] == condition]
        summary.append(
            {
                "condition": condition,
                "tasks": len(group),
                "passed": sum(1 for row in group if row["passed"]),
                "pass_rate": round(sum(1 for row in group if row["passed"]) / len(group), 4),
                "builder_passes": sum(1 for row in group if row["passed_stage"] == "builder"),
                "avg_attempts": round(sum(int(row["attempts"]) for row in group) / len(group), 3),
            }
        )
    return summary


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    fields: list[str] = []
    for row in rows:
        for key in row:
            if key not in fields:
                fields.append(key)
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)


def main() -> int:
    stamp = time.strftime("%Y%m%d-%H%M%S")
    run_dir = RUN_ROOT / f"ordo-build-suite-{stamp}"
    run_dir.mkdir(parents=True, exist_ok=True)

    condition_names = ["no_memory", "ordo_build_corpus"]
    if CONDITION_FILTER:
        condition_names = [condition for condition in condition_names if condition in CONDITION_FILTER]

    rows: list[dict[str, object]] = []
    for condition in condition_names:
        for task in TASKS:
            memory = ""
            if condition == "ordo_build_corpus":
                memory = load_jsonl_memories(ROOT / "data" / "ordo_build_suite_memory_corpus.jsonl", task.task_id)
            rows.append(run_task(run_dir, condition, task, memory))

    summary = summarize(rows)
    write_csv(run_dir / "trial_rows.csv", rows)
    write_csv(run_dir / "summary.csv", summary)
    (run_dir / "report.json").write_text(
        json.dumps(
            {
                "model": MODEL,
                "description": "Multi-task Ordo build suite for Rust MoE coder.",
                "tasks": [task.task_id for task in TASKS],
                "summary": summary,
            },
            indent=2,
        ),
        encoding="utf-8",
    )

    print()
    print(f"Run dir: {run_dir}")
    for row in summary:
        print(f"{row['condition']:<18} {row['passed']}/{row['tasks']} pass_rate={row['pass_rate']:.3f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
