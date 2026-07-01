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
OLLAMA_GENERATE_URL = "http://127.0.0.1:11434/api/generate"
MODEL = os.environ.get("ORDO_EXT_MODEL", "qwen3-coder-25b-a3b-rust-q5-chat")
API_MODE = os.environ.get("ORDO_EXT_API", "chat").strip().lower()
MAX_REPAIR_ATTEMPTS = int(os.environ.get("ORDO_EXT_MAX_REPAIRS", "2"))
CONDITION_FILTER = {
    item.strip()
    for item in os.environ.get("ORDO_EXT_CONDITIONS", "").split(",")
    if item.strip()
}
TASK_FILTER = {
    item.strip()
    for item in os.environ.get("ORDO_EXT_TASKS", "").split(",")
    if item.strip()
}
RECIPE_FILTER = {
    item.strip()
    for item in os.environ.get("ORDO_EXT_RECIPES", "").split(",")
    if item.strip()
}


HELPER = r'''
use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_ordo_ext");
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


@dataclass(frozen=True)
class Task:
    task_id: str
    description: str
    api: str
    cli: str
    tests: str


TASKS = [
    Task(
        "mini_runtime",
        "Build an Ordo mini runtime with graph edges, ramp promotion, traces, preload targets, budget routing, and backpressure signals.",
        "Message, Trace, OrdoRuntime with new/add_node/connect/add_ramp/set_capacity/set_cost/preload_targets/route_for_budget/publish.",
        "node/connect/ramp/capacity/cost/publish commands; publish prints thread=<id> path=a>b promoted=<bool> signals=<list-or-none>.",
        HELPER + r'''
use rust_ordo_ext::{Message, OrdoRuntime, Trace};

#[test]
fn mini_runtime_contract() {
    let mut rt = OrdoRuntime::new();
    rt.add_node("input"); rt.add_node("filter"); rt.add_node("model");
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
    let _m = Message { topic: "t".to_string(), payload: "p".to_string() };
    let _t = Trace { thread_id: 9, path: vec![], promoted: false, signals: vec![] };
}

#[test]
fn mini_runtime_cli() {
    let script = "node input\nnode filter\nnode model\nconnect input filter\nramp filter model\ncapacity model 0\npublish input task hello\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "thread=1 path=input>filter>model promoted=true signals=backpressure:model\n");
}
''',
    ),
    Task(
        "intersection_fabric",
        "Build a fabric where threads form from route flow, intersections emerge from shared nodes, and messages transfer at the first shared node.",
        "TransferTrace and Fabric with new/flow/intersections/transfer.",
        "flow a>b>c prints thread=<id> route=a>b>c; transfer FROM TO PAYLOAD prints transfer=<from>><to> at=<node> payload=<payload>.",
        HELPER + r'''
use rust_ordo_ext::{Fabric, TransferTrace};

#[test]
fn fabric_contract() {
    let mut fabric = Fabric::new();
    let a = fabric.flow(&["a", "b", "c"]);
    let x = fabric.flow(&["x", "b", "z"]);
    let q = fabric.flow(&["q", "r"]);
    assert_eq!((a, x, q), (1, 2, 3));
    assert_eq!(fabric.intersections(), vec!["b"]);
    assert_eq!(fabric.transfer(a, x, "hello"), Some(TransferTrace {
        from_thread: 1,
        to_thread: 2,
        at: "b".to_string(),
        payload: "hello".to_string(),
    }));
    assert_eq!(fabric.transfer(a, q, "none"), None);
}

#[test]
fn fabric_cli() {
    let (code, out, err) = run_app("flow a>b>c\nflow x>b>z\ntransfer 1 2 hello\n");
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "thread=1 route=a>b>c\nthread=2 route=x>b>z\ntransfer=1>2 at=b payload=hello\n");
}
''',
    ),
    Task(
        "backpressure_scheduler",
        "Build a scheduler seed with node cost, capacity, ramps, preference-order scheduling, and backpressure signals.",
        "Plan and Scheduler with new/add_node/add_ramp/schedule.",
        "node NAME COST CAPACITY, ramp FROM TO, schedule TASK BUDGET CANDIDATE... prints task=<id> node=<node> promoted=<bool> signals=<list-or-none>.",
        HELPER + r'''
use rust_ordo_ext::{Plan, Scheduler};

#[test]
fn scheduler_contract() {
    let mut s = Scheduler::new();
    s.add_node("local", 0, 0);
    s.add_node("cloud", 5, 2);
    s.add_node("expensive", 10, 2);
    s.add_ramp("local", "cloud");
    assert_eq!(s.schedule("job", &["local", "cloud", "expensive"], 5), Some(Plan {
        task_id: "job".to_string(),
        node: "cloud".to_string(),
        promoted: true,
        signals: vec!["backpressure:local".to_string()],
    }));
    assert_eq!(s.schedule("job", &["expensive"], 5), None);
}

#[test]
fn scheduler_cli() {
    let script = "node local 0 0\nnode cloud 5 2\nramp local cloud\nschedule job 5 local cloud\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "task=job node=cloud promoted=true signals=backpressure:local\n");
}
''',
    ),
    Task(
        "signal_mesh",
        "Build a signal mesh that propagates vertical and horizontal vibration signals through nodes and layers.",
        "SignalTrace and SignalMesh with new/add_node/vertical/horizontal/propagate.",
        "node NAME LAYER, vertical ORIGIN, horizontal ORIGIN; print signal=<kind> origin=<node> affected=<csv>.",
        HELPER + r'''
use rust_ordo_ext::{SignalMesh, SignalTrace};

#[test]
fn signal_mesh_contract() {
    let mut mesh = SignalMesh::new();
    mesh.add_node("transport", 0);
    mesh.add_node("message", 1);
    mesh.add_node("flow", 2);
    mesh.add_node("peer", 2);
    mesh.add_node("orch", 3);
    assert_eq!(mesh.vertical("message"), SignalTrace {
        kind: "vertical".to_string(),
        origin: "message".to_string(),
        affected: vec!["message".to_string(), "flow".to_string(), "peer".to_string(), "orch".to_string()],
    });
    assert_eq!(mesh.horizontal("flow").affected, vec!["peer"]);
}

#[test]
fn signal_mesh_cli() {
    let script = "node transport 0\nnode message 1\nnode flow 2\nnode peer 2\nnode orch 3\nvertical message\nhorizontal flow\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "signal=vertical origin=message affected=message,flow,peer,orch\nsignal=horizontal origin=flow affected=peer\n");
}
''',
    ),
    Task(
        "preload_planner",
        "Build a preload planner that walks the graph breadth-first and returns upcoming nodes in discovery order.",
        "PreloadPlanner with new/connect/preload_from.",
        "connect A B, preload NODE DEPTH prints preload=a,b,c.",
        HELPER + r'''
use rust_ordo_ext::PreloadPlanner;

#[test]
fn preload_contract() {
    let mut p = PreloadPlanner::new();
    p.connect("input", "a");
    p.connect("input", "b");
    p.connect("a", "c");
    p.connect("b", "d");
    assert_eq!(p.preload_from("input", 1), vec!["a", "b"]);
    assert_eq!(p.preload_from("input", 2), vec!["a", "b", "c", "d"]);
}

#[test]
fn preload_cli() {
    let script = "connect input a\nconnect input b\nconnect a c\nconnect b d\npreload input 2\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "preload=a,b,c,d\n");
}
''',
    ),
    Task(
        "ramp_roundtrip",
        "Build a ramp trace that records elevation changes through ground and parallel layers.",
        "RampTrace and RampRuntime with new/connect/on_ramp/off_ramp/trace.",
        "connect/on/off/trace commands; trace prints path=<nodes> elevations=<levels> promoted=<n> demoted=<n>.",
        HELPER + r'''
use rust_ordo_ext::{RampRuntime, RampTrace};

#[test]
fn ramp_contract() {
    let mut rt = RampRuntime::new();
    rt.on_ramp("ground", "highway");
    rt.off_ramp("highway", "exit");
    assert_eq!(rt.trace("ground"), RampTrace {
        path: vec!["ground".to_string(), "highway".to_string(), "exit".to_string()],
        elevations: vec!["local".to_string(), "parallel".to_string(), "local".to_string()],
        promotions: 1,
        demotions: 1,
    });
}

#[test]
fn ramp_cli() {
    let (code, out, err) = run_app("on ground highway\noff highway exit\ntrace ground\n");
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "path=ground>highway>exit elevations=local,parallel,local promoted=1 demoted=1\n");
}
''',
    ),
    Task(
        "retry_fallback",
        "Build a retry/fallback planner that skips failed nodes, records retry signals, and selects the first healthy candidate.",
        "RetryPlan and RetryRuntime with new/add_node/plan.",
        "node NAME healthy|failed, plan TASK CANDIDATE... prints task=<id> node=<node> signals=<list-or-none>.",
        HELPER + r'''
use rust_ordo_ext::{RetryPlan, RetryRuntime};

#[test]
fn retry_contract() {
    let mut rt = RetryRuntime::new();
    rt.add_node("local", false);
    rt.add_node("backup", true);
    assert_eq!(rt.plan("job", &["local", "backup"]), Some(RetryPlan {
        task_id: "job".to_string(),
        node: "backup".to_string(),
        signals: vec!["retry:local".to_string()],
    }));
    assert_eq!(rt.plan("job", &["local"]), None);
}

#[test]
fn retry_cli() {
    let (code, out, err) = run_app("node local failed\nnode backup healthy\nplan job local backup\n");
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "task=job node=backup signals=retry:local\n");
}
''',
    ),
    Task(
        "cost_ledger",
        "Build a cost ledger that records route charges and reports budget acceptance.",
        "LedgerReport and CostLedger with new/charge/report.",
        "charge NODE COST, report BUDGET prints total=<n> accepted=<bool> route=<nodes>.",
        HELPER + r'''
use rust_ordo_ext::{CostLedger, LedgerReport};

#[test]
fn ledger_contract() {
    let mut ledger = CostLedger::new();
    ledger.charge("local", 0);
    ledger.charge("cloud", 5);
    assert_eq!(ledger.report(6), LedgerReport {
        total: 5,
        accepted: true,
        route: vec!["local".to_string(), "cloud".to_string()],
    });
    assert!(!ledger.report(4).accepted);
}

#[test]
fn ledger_cli() {
    let (code, out, err) = run_app("charge local 0\ncharge cloud 5\nreport 4\n");
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "total=5 accepted=false route=local>cloud\n");
}
''',
    ),
    Task(
        "workflow_codec",
        "Build a tiny workflow codec for Ordo graph edges using a semicolon DSL.",
        "Workflow with new/connect/serialize/parse/edges.",
        "connect A B, serialize, parse DSL; serialize prints workflow=a>b;b>c.",
        HELPER + r'''
use rust_ordo_ext::Workflow;

#[test]
fn codec_contract() {
    let mut wf = Workflow::new();
    wf.connect("input", "filter");
    wf.connect("filter", "model");
    assert_eq!(wf.serialize(), "input>filter;filter>model");
    let parsed = Workflow::parse("a>b;bad;c>d");
    assert_eq!(parsed.edges(), vec![("a".to_string(), "b".to_string()), ("c".to_string(), "d".to_string())]);
}

#[test]
fn codec_cli() {
    let (code, out, err) = run_app("connect input filter\nconnect filter model\nserialize\n");
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "workflow=input>filter;filter>model\n");
}
''',
    ),
    Task(
        "node_registry",
        "Build a node registry that maps nodes to capabilities and finds matching nodes deterministically.",
        "NodeRegistry with new/register/nodes_with.",
        "register NODE cap1,cap2; find CAP prints nodes=a,b.",
        HELPER + r'''
use rust_ordo_ext::NodeRegistry;

#[test]
fn registry_contract() {
    let mut r = NodeRegistry::new();
    r.register("model", &["llm", "gpu"]);
    r.register("embed", &["gpu", "vector"]);
    r.register("local", &["cpu"]);
    assert_eq!(r.nodes_with("gpu"), vec!["embed", "model"]);
    assert_eq!(r.nodes_with("missing"), Vec::<String>::new());
}

#[test]
fn registry_cli() {
    let script = "register model llm,gpu\nregister embed gpu,vector\nfind gpu\n";
    let (code, out, err) = run_app(script);
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "nodes=embed,model\n");
}
''',
    ),
    Task(
        "fanout_join",
        "Build a fanout/join helper that preserves target order and deterministic joined output.",
        "FanoutJoin with new/fanout/join.",
        "fanout SOURCE A B; join A=x B=y prints joined=A=x,B=y.",
        HELPER + r'''
use rust_ordo_ext::FanoutJoin;

#[test]
fn fanout_contract() {
    let fj = FanoutJoin::new();
    assert_eq!(fj.fanout("input", &["a", "b"]), vec!["input>a", "input>b"]);
    assert_eq!(fj.join(&[("a", "one"), ("b", "two")]), "a=one,b=two");
}

#[test]
fn fanout_cli() {
    let (code, out, err) = run_app("fanout input a b\njoin a=one b=two\n");
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "fanout=input>a,input>b\njoined=a=one,b=two\n");
}
''',
    ),
    Task(
        "audit_timeline",
        "Build an audit timeline for Ordo events with monotonic event ids and thread filtering.",
        "AuditEvent and AuditTimeline with new/record/events_for_thread.",
        "record THREAD KIND NODE; show THREAD prints audit=1:kind@node,...",
        HELPER + r'''
use rust_ordo_ext::{AuditEvent, AuditTimeline};

#[test]
fn audit_contract() {
    let mut a = AuditTimeline::new();
    a.record(1, "route", "input");
    a.record(2, "route", "other");
    a.record(1, "signal", "model");
    assert_eq!(a.events_for_thread(1), vec![
        AuditEvent { index: 1, thread_id: 1, kind: "route".to_string(), node: "input".to_string() },
        AuditEvent { index: 3, thread_id: 1, kind: "signal".to_string(), node: "model".to_string() },
    ]);
}

#[test]
fn audit_cli() {
    let (code, out, err) = run_app("record 1 route input\nrecord 2 route other\nrecord 1 signal model\nshow 1\n");
    assert_eq!(code, 0, "stderr={err}");
    assert_eq!(out, "audit=1:route@input,3:signal@model\n");
}
''',
    ),
]


RECIPE_PATHS = {
    "build_only": [
        ROOT / "data" / "ordo_build_suite_memory_corpus.jsonl",
    ],
    "extended_only": [
        ROOT / "data" / "ordo_extended_suite_memory_corpus.jsonl",
    ],
    "runtime_only": [
        ROOT / "data" / "ordo_runtime_memory_build_corpus.jsonl",
    ],
    "build_extended": [
        ROOT / "data" / "ordo_build_suite_memory_corpus.jsonl",
        ROOT / "data" / "ordo_extended_suite_memory_corpus.jsonl",
    ],
    "all_memory": [
        ROOT / "data" / "ordo_runtime_memory_build_corpus.jsonl",
        ROOT / "data" / "ordo_build_suite_memory_corpus.jsonl",
        ROOT / "data" / "ordo_extended_suite_memory_corpus.jsonl",
    ],
    "four_b_scars": [
        ROOT / "data" / "ordo_4b_compile_scar_memories.txt",
        ROOT / "data" / "ordo_build_suite_memory_corpus.jsonl",
        ROOT / "data" / "ordo_extended_suite_memory_corpus.jsonl",
    ],
    "four_b_all": [
        ROOT / "data" / "ordo_4b_compile_scar_memories.txt",
        ROOT / "data" / "ordo_runtime_memory_build_corpus.jsonl",
        ROOT / "data" / "ordo_build_suite_memory_corpus.jsonl",
        ROOT / "data" / "ordo_extended_suite_memory_corpus.jsonl",
    ],
    "artifact_scars": [
        ROOT / "data" / "ordo_4b_compile_scar_memories.txt",
        ROOT / "data" / "ordo_build_suite_memory_corpus.jsonl",
        ROOT / "data" / "ordo_extended_suite_memory_corpus.jsonl",
    ],
    "q4_artifact_scars": [
        ROOT / "data" / "ordo_4b_compile_scar_memories.txt",
        ROOT / "data" / "ordo_build_suite_memory_corpus.jsonl",
        ROOT / "data" / "ordo_extended_suite_memory_corpus.jsonl",
    ],
}


def load_memories(task_id: str, recipe: str = "build_extended") -> str:
    memories: list[str] = []
    if recipe == "q4_artifact_scars" and task_id == "ramp_roundtrip":
        compact = ROOT / "data" / "ordo_q4_ramp_compact_memory.txt"
        if compact.exists():
            return compact.read_text(encoding="utf-8").strip()
    routed_paths = list(RECIPE_PATHS.get(recipe, RECIPE_PATHS["build_extended"]))
    if recipe == "build_extended" and task_id == "mini_runtime":
        routed_paths.insert(0, ROOT / "data" / "ordo_runtime_memory_build_corpus.jsonl")
    for path in routed_paths:
        if not path.exists():
            continue
        if path.suffix == ".txt":
            for block in path.read_text(encoding="utf-8").split("\n---\n"):
                lines = [line for line in block.strip().splitlines() if line.strip()]
                if not lines:
                    continue
                header = lines[0].strip()
                if header.startswith("task="):
                    item_task = header.removeprefix("task=").strip()
                    if item_task not in {"common", task_id}:
                        continue
                    memory = "\n".join(lines[1:]).strip()
                    if memory:
                        memories.append(memory)
            continue
        with path.open("r", encoding="utf-8") as handle:
            for line in handle:
                if not line.strip():
                    continue
                item = json.loads(line)
                item_task = str(item.get("task", "common"))
                if item_task in {"common", task_id} or (
                    task_id == "mini_runtime" and path.name == "ordo_runtime_memory_build_corpus.jsonl"
                ):
                    memory = str(item.get("memory", "")).strip()
                    if memory:
                        memories.append(memory)
    if recipe in {"artifact_scars", "q4_artifact_scars"}:
        artifact_memory = load_artifact_memory(task_id)
        if artifact_memory:
            memories.append(artifact_memory)
    return "\n\n".join(memories)


def load_artifact_memory(task_id: str) -> str:
    roots = [
        RUN_ROOT / "ordo-extended-suite-20260608-185020" / "ordo_memory" / task_id,
        RUN_ROOT / "ordo-extended-suite-20260608-185527" / "ordo_memory" / task_id,
    ]
    library_name = "lib" + ".rs"
    binary_name = "main" + ".rs"
    for root in roots:
        trial_rows = root.parents[1] / "trial_rows.csv"
        if not trial_rows.exists():
            continue
        with trial_rows.open("r", encoding="utf-8", newline="") as handle:
            for row in csv.DictReader(handle):
                if row.get("task_id") == task_id and row.get("passed") == "True":
                    library_path = root / library_name
                    binary_path = root / binary_name
                    library_code = library_path.read_text(encoding="utf-8") if library_path.exists() else ""
                    binary_code = binary_path.read_text(encoding="utf-8") if binary_path.exists() else ""
                    if library_code.strip() and binary_code.strip():
                        return (
                            f"I remember the Ordo {task_id} build that passed cargo test. "
                            "The shape that worked was small, direct, and standard-library only.\n"
                            "The library shape I remembered:\n"
                            "```rust\n"
                            f"{library_code.strip()}\n"
                            "```\n"
                            "The binary CLI shape I remembered:\n"
                            "```rust\n"
                            f"{binary_code.strip()}\n"
                            "```"
                        )
    return ""


def load_all_python_scar_memories() -> str:
    path = ROOT / "data" / "python_domain_scar_memories.txt"
    if not path.exists():
        return ""
    memories: list[str] = []
    for block in path.read_text(encoding="utf-8").split("---"):
        lines = [line for line in block.strip().splitlines() if line.strip()]
        if not lines:
            continue
        if lines[0].strip().startswith("task="):
            lines = lines[1:]
        memory = "\n".join(lines).strip()
        if memory:
            memories.append(memory)
    return "\n\n".join(memories)


def memory_for_condition(condition: str, task_id: str) -> str:
    if condition == "python_only_memories":
        return load_all_python_scar_memories()
    if condition == "routed_stack_memories":
        return load_memories(task_id, "artifact_scars")
    if condition == "ordo_artifact_plus_python":
        parts = [load_memories(task_id, "artifact_scars"), load_all_python_scar_memories()]
        return "\n\n".join(part for part in parts if part)
    recipe = condition.removeprefix("ordo_")
    return load_memories(task_id, recipe) if condition.startswith("ordo_") else ""


def call_ollama(prompt: str) -> str:
    options = {
        "temperature": 0,
        "seed": 41,
        "num_predict": 5000,
        "num_ctx": 20000,
        "stop": ["\n### User", "\nUser:"],
    }
    if API_MODE == "generate":
        payload = {
            "model": MODEL,
            "prompt": prompt,
            "stream": False,
            "options": options,
        }
        url = OLLAMA_GENERATE_URL
    else:
        payload = {
            "model": MODEL,
            "messages": [{"role": "user", "content": prompt}],
            "stream": False,
            "think": False,
            "options": options,
        }
        url = OLLAMA_CHAT_URL
    data = json.dumps(payload).encode("utf-8")
    request = urllib.request.Request(url, data=data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(request, timeout=600) as response:
        body = json.loads(response.read().decode("utf-8"))
    if API_MODE == "generate":
        return body["response"]
    return body["message"]["content"]


def strip_fence(text: str) -> str:
    fence = re.search(r"```(?:rust|toml|text)?\s*(.*?)```", text.strip(), flags=re.DOTALL | re.IGNORECASE)
    return fence.group(1).strip() if fence else text.strip()


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
    if "[package]" in cargo and "rust_ordo_ext" in cargo:
        return cargo
    return '[package]\nname = "rust_ordo_ext"\nversion = "0.1.0"\nedition = "2021"\n'


def write_project(crate_dir: Path, project: dict[str, str], tests: str) -> None:
    if crate_dir.exists():
        shutil.rmtree(crate_dir)
    (crate_dir / "src").mkdir(parents=True)
    (crate_dir / "tests").mkdir(parents=True)
    (crate_dir / "Cargo.toml").write_text(default_cargo(project.get("Cargo.toml", "")) + "\n", encoding="utf-8")
    (crate_dir / "src" / "lib.rs").write_text(project.get("src/lib.rs", "") + "\n", encoding="utf-8")
    (crate_dir / "src" / "main.rs").write_text(project.get("src/main.rs", "") + "\n", encoding="utf-8")
    (crate_dir / "tests" / "ordo_ext.rs").write_text(tests + "\n", encoding="utf-8")


def run_cargo_test(crate_dir: Path) -> tuple[bool, str]:
    try:
        completed = subprocess.run(
            ["cargo", "test", "--quiet"],
            cwd=crate_dir,
            text=True,
            encoding="utf-8",
            errors="replace",
            capture_output=True,
            timeout=120,
        )
    except subprocess.TimeoutExpired as exc:
        return False, (exc.stdout or "") + "\n" + (exc.stderr or "") + "\nTIMEOUT"
    return completed.returncode == 0, (completed.stdout or "") + (completed.stderr or "")


def build_prompt(task: Task, memory: str) -> str:
    memory_block = f"\nRelevant lived memories:\n{memory}\n" if memory else ""
    return f"""
Build this harder Ordo-shaped Rust app.

Task:
{task.description}

Required API:
{task.api}

CLI:
{task.cli}

Requirements:
- Output exactly Cargo.toml, src/lib.rs, and src/main.rs.
- Use only the Rust standard library.
- Package name must be rust_ordo_ext.
- No explanations outside file blocks.
- Public structs used in tests should derive Clone, Debug, PartialEq, Eq.
- Stdout must match the CLI contract exactly.
{memory_block}
Use this exact response shape:
=== Cargo.toml ===
```toml
[package]
name = "rust_ordo_ext"
version = "0.1.0"
edition = "2021"
```
=== src/lib.rs ===
```rust
// library code
```
=== src/main.rs ===
```rust
// CLI adapter
```
""".strip()


def repair_prompt(task: Task, memory: str, raw: str, cargo_output: str) -> str:
    return f"""
Repair this Ordo-shaped Rust app.

Task:
{task.description}

Required API:
{task.api}

Relevant lived memories:
{memory}

Cargo/test output:
```text
{cargo_output[-6000:]}
```

Previous response:
```text
{raw[-8000:]}
```

Return exactly Cargo.toml, src/lib.rs, and src/main.rs using the same file-block format.
""".strip()


def run_task(run_dir: Path, condition: str, task: Task, memory: str) -> dict[str, object]:
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
    while not passed and attempts <= MAX_REPAIR_ATTEMPTS:
        start = time.time()
        raw = call_ollama(repair_prompt(task, memory, raw, cargo_output))
        repair_elapsed += time.time() - start
        attempts += 1
        project = extract_project(raw)
        write_project(crate_dir, project, task.tests)
        passed, cargo_output = run_cargo_test(crate_dir)
        stage = f"repair_{attempts - 1}" if passed else ""
    (task_dir / "prompt.txt").write_text(prompt, encoding="utf-8")
    (task_dir / "final_raw_response.txt").write_text(raw, encoding="utf-8")
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
    print(f"{condition:<18} | {task.task_id:<24} | {'PASS' if passed else 'FAIL'} | stage={stage or '-'} attempts={attempts}")
    return row


def summarize(rows: list[dict[str, object]]) -> list[dict[str, object]]:
    summary = []
    for condition in sorted({str(row["condition"]) for row in rows}):
        group = [row for row in rows if row["condition"] == condition]
        summary.append({
            "condition": condition,
            "tasks": len(group),
            "passed": sum(1 for row in group if row["passed"]),
            "pass_rate": round(sum(1 for row in group if row["passed"]) / len(group), 4),
            "builder_passes": sum(1 for row in group if row["passed_stage"] == "builder"),
            "avg_attempts": round(sum(int(row["attempts"]) for row in group) / len(group), 3),
        })
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
    run_dir = RUN_ROOT / f"ordo-extended-suite-{stamp}"
    run_dir.mkdir(parents=True, exist_ok=True)
    recipes = ["build_extended"] if not RECIPE_FILTER else sorted(recipe for recipe in RECIPE_FILTER if recipe in RECIPE_PATHS)
    conditions = ["no_memory"] + [f"ordo_{recipe}" for recipe in recipes]
    extra_conditions = ["python_only_memories", "ordo_artifact_plus_python", "routed_stack_memories"]
    if CONDITION_FILTER:
        conditions.extend(condition for condition in extra_conditions if condition in CONDITION_FILTER)
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
    (run_dir / "report.json").write_text(json.dumps({"model": MODEL, "tasks": [t.task_id for t in tasks], "summary": summary}, indent=2), encoding="utf-8")
    print()
    print(f"Run dir: {run_dir}")
    for row in summary:
        print(f"{row['condition']:<18} {row['passed']}/{row['tasks']} pass_rate={row['pass_rate']:.3f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
