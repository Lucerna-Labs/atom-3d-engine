from __future__ import annotations

import csv
import json
import os
import re
import shutil
import subprocess
import time
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
RUN_ROOT = ROOT / "rag_runs"
OLLAMA_CHAT_URL = "http://127.0.0.1:11434/api/chat"
MODEL = os.environ.get("ORDO_RUNTIME_MODEL", "qwen35-4b-claude-distill-v2-q6-chat")
MAX_REPAIR_ATTEMPTS = 1
CONDITION_FILTER = {
    item.strip()
    for item in os.environ.get("ORDO_RUNTIME_CONDITIONS", "").split(",")
    if item.strip()
}


APP_MEMORIES = """
I remember the CLI app that passed only after I stopped treating it like a code snippet. The correct version had Cargo.toml, src/lib.rs, src/main.rs, and integration tests could import the library and spawn the binary.

I remember a runtime test that failed because stdout had friendly labels instead of the exact machine-readable line. The passing app printed one compact summary line, with fields in the order the tests expected.

I remember the Rust app that passed because every public struct and function used the exact names the tests imported. I did not rename the API into something clever.

I remember the crate that became stable after I used only std collections: VecDeque for queued work, HashMap for node state, BTreeSet when deterministic ordering mattered.
""".strip()


ORDO_RUNTIME_MEMORIES = """
I remember building a tiny Ordo runtime prototype after flattening it into a generic pub-sub system, and it failed review immediately. The fix was to preserve Jesse's shape: a ground bus, graph flow, on-ramps, off-ramps, emergent thread traces, and vibration signals when a downstream node could not accept work.

I remember the passing prototype exposed a small public API instead of hiding everything behind main. The tests imported OrdoRuntime, Message, and Trace from the crate. Main was just a script adapter around the same runtime, not a second implementation.

I remember the thread trace bug. I had declared threads before messages moved, which was backwards. The passing runtime made a new thread id when publish was called, then recorded the path as the message followed graph edges. Threads formed from flow.

I remember the ramp bug. A normal connect kept the message on the ground bus. A ramp edge meant the message was promoted to the highway layer, so the final Trace had promoted=true when any ramp edge was crossed.

I remember the backpressure vibration bug. When a node had capacity 0, the runtime still recorded the node in the path, but also emitted a signal shaped exactly backpressure:<node>. That signal was the web feeling pressure at an intersection.

I remember the preload test that passed because preload_targets returned only the immediate outgoing ground or ramp neighbors for a node, sorted by insertion route rather than invented priority.

I remember the cost-routing test. route_for_budget accepted candidate node ids in preference order and returned the first candidate whose configured cost was less than or equal to the budget. It did not sort them; preference order mattered.

I remember the CLI adapter that passed. It read stdin to the end, accepted commands like node, connect, ramp, capacity, cost, and publish, then printed exactly one line for publish: thread=<id> path=a>b>c promoted=true signals=backpressure:c.

I remember the failed preload implementation where I collected targets into VecDeque and then tried to call sort. VecDeque does not have sort. The passing version used Vec<String> for preload targets and returned the outgoing edge targets in insertion order without sorting.

I remember the CLI output bug where I used {:?} on the path and signals. The tests did not want debug arrays or quotes. The passing version used trace.path.join(">") and trace.signals.join(",") so the line was exactly thread=1 path=input>filter>model promoted=true signals=backpressure:model.

I remember the lifetime failure where publish stored current as &str and then assigned current = &e.to from a cloned edge. The edge died at the end of the match arm, so Rust was right to reject it. The passing version kept current as an owned String and assigned current = edge.to.clone().

I remember the false backpressure failure where add_node defaulted capacity to 0. That made every node look overloaded. The passing runtime defaulted capacity to usize::MAX and only emitted backpressure after set_capacity(node, 0).

I remember the broken main adapter where I called read_to_string like it accepted a closure. The passing version imported std::io::Read, created let mut input = String::new(), then called std::io::stdin().read_to_string(&mut input).unwrap().

I remember the payload collection failure where parts.iter().skip(2).collect::<String>() tried to collect &&str into a String. The passing version used parts[3..].join(" ") for the publish payload, because parts[0] was publish, parts[1] was from, and parts[2] was topic.

I remember the thread id failure where publish returned self.next_thread_id but never incremented it. The passing version copied let thread_id = self.next_thread_id; then incremented self.next_thread_id += 1; then returned that copied thread_id.

I remember the almost-passing comparison bug where I wrote edge.from == &current. Rust would not compare String with &String there. The passing line was edge.from == current.

I remember the first-thread bug. The tests expected the first flowing message to create thread_id 1. The passing runtime initialized next_thread_id to 1 in new(), not 0.
""".strip()


ORDO_CODE_MEMORY = """
I remember the Ordo mini-runtime that passed because the implementation stayed boring and exact.

The public API shape was:
```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message { pub topic: String, pub payload: String }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    pub thread_id: usize,
    pub path: Vec<String>,
    pub promoted: bool,
    pub signals: Vec<String>,
}

pub struct OrdoRuntime { ... }

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
```

The internal shape was HashMap<String, NodeState>, Vec<Edge>, and next_thread_id. Edge had from, to, and ramp. publish did a simple forward walk: push the current node into path, check capacity 0 for backpressure, follow the first outgoing edge, mark promoted if edge.ramp, and stop when no outgoing edge exists or a repeated node would loop.

preload_targets did not use VecDeque and did not sort:
```rust
pub fn preload_targets(&self, node: &str) -> Vec<String> {
    self.edges.iter()
        .filter(|edge| edge.from == node)
        .map(|edge| edge.to.clone())
        .collect()
}
```

The CLI print did not use debug formatting:
```rust
let signals = if trace.signals.is_empty() { "none".to_string() } else { trace.signals.join(",") };
println!(
    "thread={} path={} promoted={} signals={}",
    trace.thread_id,
    trace.path.join(">"),
    trace.promoted,
    signals
);
```

The publish loop kept current owned:
```rust
let thread_id = self.next_thread_id;
self.next_thread_id += 1;
let mut current = from.to_string();
loop {
    path.push(current.clone());
    if let Some(state) = self.nodes.get(&current) {
        if state.capacity == 0 {
            signals.push(format!("backpressure:{}", current));
        }
    }
    let edge = self.edges.iter().find(|edge| edge.from == current).cloned();
    match edge {
        Some(edge) => {
            if edge.ramp { promoted = true; }
            current = edge.to.clone();
        }
        None => break,
    }
}
```

The binary adapter read stdin with Read and joined payload words:
```rust
use rust_ordo_runtime::OrdoRuntime;
use std::io::Read;

fn main() {
    let mut runtime = OrdoRuntime::new();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }
        match parts[0] {
            "node" => runtime.add_node(parts[1]),
            "connect" => runtime.connect(parts[1], parts[2]),
            "ramp" => runtime.add_ramp(parts[1], parts[2]),
            "capacity" => runtime.set_capacity(parts[1], parts[2].parse().unwrap()),
            "cost" => runtime.set_cost(parts[1], parts[2].parse().unwrap()),
            "publish" => {
                let payload = parts[3..].join(" ");
                let trace = runtime.publish(parts[1], parts[2], &payload);
                let signals = if trace.signals.is_empty() { "none".to_string() } else { trace.signals.join(",") };
                println!("thread={} path={} promoted={} signals={}", trace.thread_id, trace.path.join(">"), trace.promoted, signals);
            }
            _ => {}
        }
    }
}
```

The CLI shape was a small stdin script runner:
node NAME
connect A B
ramp A B
capacity NAME N
cost NAME N
publish FROM TOPIC PAYLOAD...

On publish it printed:
thread=1 path=input>filter>model promoted=true signals=backpressure:model
""".strip()


TESTS = r'''
use std::io::Write;
use std::process::{Command, Stdio};
use rust_ordo_runtime::{Message, OrdoRuntime, Trace};

#[test]
fn library_routes_through_ground_and_ramp_layers() {
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
    assert_eq!(rt.preload_targets("filter"), vec!["model"]);
}

#[test]
fn thread_ids_are_created_as_messages_flow() {
    let mut rt = OrdoRuntime::new();
    rt.add_node("a");
    rt.add_node("b");
    rt.connect("a", "b");

    let first = rt.publish("a", "topic", "one");
    let second = rt.publish("a", "topic", "two");

    assert_eq!(first.thread_id, 1);
    assert_eq!(second.thread_id, 2);
    assert_eq!(first.path, vec!["a", "b"]);
    assert_eq!(second.path, vec!["a", "b"]);
}

#[test]
fn cost_routing_respects_candidate_preference_and_budget() {
    let mut rt = OrdoRuntime::new();
    rt.add_node("local");
    rt.add_node("cloud");
    rt.set_cost("local", 0);
    rt.set_cost("cloud", 5);

    assert_eq!(rt.route_for_budget(&["cloud", "local"], 0), Some("local".to_string()));
    assert_eq!(rt.route_for_budget(&["cloud", "local"], 5), Some("cloud".to_string()));
    assert_eq!(rt.route_for_budget(&["cloud"], 4), None);
}

#[test]
fn public_types_have_expected_shape() {
    let msg = Message { topic: "task".to_string(), payload: "hello".to_string() };
    let trace = Trace {
        thread_id: 7,
        path: vec!["a".to_string()],
        promoted: false,
        signals: vec![],
    };
    assert_eq!(msg.topic, "task");
    assert_eq!(msg.payload, "hello");
    assert_eq!(trace.thread_id, 7);
}

#[test]
fn cli_runs_a_tiny_runtime_script() {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_runtime");
    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn runtime");

    let script = "\
node input
node filter
node model
connect input filter
ramp filter model
capacity model 0
publish input task hello
";

    child.stdin.as_mut().unwrap().write_all(script.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
    let stderr = String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n");

    assert!(output.status.success(), "stderr={stderr}");
    assert_eq!(stdout, "thread=1 path=input>filter>model promoted=true signals=backpressure:model\n");
}
'''


def call_ollama(prompt: str) -> str:
    payload = {
        "model": MODEL,
        "messages": [{"role": "user", "content": prompt}],
        "stream": False,
        "think": False,
        "options": {
            "temperature": 0,
            "seed": 23,
            "num_predict": 3600,
            "num_ctx": 12288,
            "stop": ["\n### User", "\nUser:"],
        },
    }
    data = json.dumps(payload).encode("utf-8")
    request = urllib.request.Request(OLLAMA_CHAT_URL, data=data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(request, timeout=360) as response:
        body = json.loads(response.read().decode("utf-8"))
    return body["message"]["content"]


def load_jsonl_corpus(path: Path) -> str:
    if not path.exists():
        return ""
    memories: list[str] = []
    with path.open("r", encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if not line:
                continue
            item = json.loads(line)
            memory = str(item.get("memory", "")).strip()
            if memory:
                memories.append(memory)
    return "\n\n".join(memories)


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
    if "[package]" in cargo and "rust_ordo_runtime" in cargo:
        return cargo
    return '[package]\nname = "rust_ordo_runtime"\nversion = "0.1.0"\nedition = "2021"\n'


def write_project(crate_dir: Path, project: dict[str, str]) -> None:
    if crate_dir.exists():
        shutil.rmtree(crate_dir)
    (crate_dir / "src").mkdir(parents=True)
    (crate_dir / "tests").mkdir(parents=True)
    (crate_dir / "Cargo.toml").write_text(default_cargo(project.get("Cargo.toml", "")) + "\n", encoding="utf-8")
    (crate_dir / "src" / "lib.rs").write_text(project.get("src/lib.rs", "") + "\n", encoding="utf-8")
    (crate_dir / "src" / "main.rs").write_text(project.get("src/main.rs", "") + "\n", encoding="utf-8")
    (crate_dir / "tests" / "runtime.rs").write_text(TESTS + "\n", encoding="utf-8")


def run_cargo_test(crate_dir: Path) -> tuple[bool, str]:
    try:
        completed = subprocess.run(
            ["cargo", "test", "--quiet"],
            cwd=crate_dir,
            text=True,
            capture_output=True,
            timeout=90,
        )
    except subprocess.TimeoutExpired as exc:
        output = (exc.stdout or "") + "\n" + (exc.stderr or "") + "\nTIMEOUT"
        return False, output
    output = (completed.stdout or "") + (completed.stderr or "")
    return completed.returncode == 0, output


def build_prompt(memory: str) -> str:
    memory_section = ""
    if memory.strip():
        memory_section = f"""
Relevant lived memories:
{memory.strip()}
"""
    return f"""
Build a small Ordo runtime as a complete Rust app.

Ordo shape:
- It is not a generic actor system.
- It has a ground bus, graph flow, ramp edges that promote work to a highway layer, emergent thread traces, backpressure vibration signals, preload targets, and cost-aware routing.

Required public API:
- Message {{ pub topic: String, pub payload: String }}
- Trace {{ pub thread_id: usize, pub path: Vec<String>, pub promoted: bool, pub signals: Vec<String> }}
- OrdoRuntime::new()
- add_node(&mut self, id: &str)
- connect(&mut self, from: &str, to: &str)
- add_ramp(&mut self, from: &str, to: &str)
- set_capacity(&mut self, node: &str, capacity: usize)
- set_cost(&mut self, node: &str, cost: u32)
- preload_targets(&self, node: &str) -> Vec<String>
- route_for_budget(&self, candidates: &[&str], max_cost: u32) -> Option<String>
- publish(&mut self, from: &str, topic: &str, payload: &str) -> Trace

CLI:
- Read stdin to the end.
- Commands: node NAME, connect A B, ramp A B, capacity NAME N, cost NAME N, publish FROM TOPIC PAYLOAD...
- On publish, print exactly: thread=<id> path=a>b>c promoted=true signals=backpressure:c

Requirements:
- Output exactly three files: Cargo.toml, src/lib.rs, src/main.rs.
- Use only the Rust standard library.
- Package name must be rust_ordo_runtime.
- No explanations outside file blocks.
{memory_section}
Use this exact response shape:
=== Cargo.toml ===
```toml
[package]
name = "rust_ordo_runtime"
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


def repair_prompt(memory: str, raw: str, cargo_output: str) -> str:
    return f"""
Repair the Ordo runtime app so it compiles and passes the tests.

Relevant lived memories:
{memory.strip()}

Cargo/test output:
```text
{cargo_output[-4500:]}
```

Previous response:
```text
{raw[-6500:]}
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


def run_condition(run_dir: Path, condition: str, memory: str) -> dict[str, object]:
    task_dir = run_dir / condition / "ordo_runtime"
    task_dir.mkdir(parents=True, exist_ok=True)
    crate_dir = task_dir / "crate"

    prompt = build_prompt(memory)
    start = time.time()
    raw = call_ollama(prompt)
    builder_elapsed = time.time() - start
    project = extract_project(raw)
    write_project(crate_dir, project)
    passed, cargo_output = run_cargo_test(crate_dir)
    attempts = [
        {
            "attempt": 0,
            "stage": "builder",
            "passed": passed,
            "lib_chars": len(project.get("src/lib.rs", "")),
            "main_chars": len(project.get("src/main.rs", "")),
            "cargo_output_chars": len(cargo_output),
        }
    ]

    repair_elapsed = 0.0
    if not passed:
        for repair_index in range(1, MAX_REPAIR_ATTEMPTS + 1):
            start = time.time()
            raw = call_ollama(repair_prompt(memory, raw, cargo_output))
            repair_elapsed += time.time() - start
            project = extract_project(raw)
            write_project(crate_dir, project)
            passed, cargo_output = run_cargo_test(crate_dir)
            attempts.append(
                {
                    "attempt": repair_index,
                    "stage": f"repair_{repair_index}",
                    "passed": passed,
                    "lib_chars": len(project.get("src/lib.rs", "")),
                    "main_chars": len(project.get("src/main.rs", "")),
                    "cargo_output_chars": len(cargo_output),
                }
            )
            if passed:
                break

    (task_dir / "prompt.txt").write_text(prompt, encoding="utf-8")
    (task_dir / "final_raw_response.txt").write_text(raw, encoding="utf-8")
    (task_dir / "Cargo.toml").write_text(project.get("Cargo.toml", ""), encoding="utf-8")
    (task_dir / "lib.rs").write_text(project.get("src/lib.rs", ""), encoding="utf-8")
    (task_dir / "main.rs").write_text(project.get("src/main.rs", ""), encoding="utf-8")
    (task_dir / "cargo_output.txt").write_text(cargo_output, encoding="utf-8")
    (task_dir / "attempts.json").write_text(json.dumps(attempts, indent=2), encoding="utf-8")

    row = {
        "condition": condition,
        "task_id": "ordo_runtime",
        "passed": passed,
        "passed_stage": next((item["stage"] for item in attempts if item["passed"]), ""),
        "attempts": len(attempts),
        "builder_elapsed_sec": round(builder_elapsed, 3),
        "repair_elapsed_sec": round(repair_elapsed, 3),
        "lib_chars": len(project.get("src/lib.rs", "")),
        "main_chars": len(project.get("src/main.rs", "")),
        "cargo_output_chars": len(cargo_output),
    }
    print(
        f"{condition:<22} | {'PASS' if passed else 'FAIL'} | "
        f"attempts={len(attempts)} stage={row['passed_stage'] or '-'} "
        f"builder={builder_elapsed:.1f}s repair={repair_elapsed:.1f}s"
    )
    return row


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
    run_dir = RUN_ROOT / f"ordo-runtime-memory-{stamp}"
    run_dir.mkdir(parents=True, exist_ok=True)

    corpus_memory = load_jsonl_corpus(ROOT / "data" / "ordo_runtime_memory_build_corpus.jsonl")
    conditions = [
        ("no_memory", ""),
        ("app_memories", APP_MEMORIES),
        ("ordo_memories", APP_MEMORIES + "\n\n" + ORDO_RUNTIME_MEMORIES),
        ("ordo_code_memories", APP_MEMORIES + "\n\n" + ORDO_RUNTIME_MEMORIES + "\n\n" + ORDO_CODE_MEMORY),
        (
            "ordo_build_corpus",
            APP_MEMORIES + "\n\n" + ORDO_RUNTIME_MEMORIES + "\n\n" + ORDO_CODE_MEMORY + "\n\n" + corpus_memory,
        ),
    ]
    if CONDITION_FILTER:
        conditions = [item for item in conditions if item[0] in CONDITION_FILTER]
    rows = [run_condition(run_dir, condition, memory) for condition, memory in conditions]
    summary = [
        {
            "condition": row["condition"],
            "tasks": 1,
            "passed": 1 if row["passed"] else 0,
            "pass_rate": 1.0 if row["passed"] else 0.0,
            "passed_stage": row["passed_stage"],
            "attempts": row["attempts"],
        }
        for row in rows
    ]
    write_csv(run_dir / "trial_rows.csv", rows)
    write_csv(run_dir / "summary.csv", summary)
    (run_dir / "report.json").write_text(
        json.dumps(
            {
                "model": MODEL,
                "description": "Hard Ordo runtime generation benchmark with library and CLI verification.",
                "summary": summary,
            },
            indent=2,
        ),
        encoding="utf-8",
    )
    (run_dir / "ordo_runtime_memories.txt").write_text(
        APP_MEMORIES + "\n\n" + ORDO_RUNTIME_MEMORIES + "\n\n" + ORDO_CODE_MEMORY + "\n\n" + corpus_memory + "\n",
        encoding="utf-8",
    )

    print()
    print(f"Run dir: {run_dir}")
    for row in summary:
        print(f"{row['condition']:<22} {row['passed']}/{row['tasks']} pass_rate={row['pass_rate']:.3f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
