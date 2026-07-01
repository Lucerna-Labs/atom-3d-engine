#!/usr/bin/env python
"""Build a tighter v0.2 Ordo Rust LoRA dataset with contract-card weighting."""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
V1 = ROOT / 'data' / 'ordo_rust_memory_lora_v0_1'
OUT = ROOT / 'data' / 'ordo_rust_memory_lora_v0_2'
OUT.mkdir(parents=True, exist_ok=True)

PREFIX = """You are training as a narrow Rust builder for Jesse's Ordo architecture.
Return exact Rust contracts and compact implementation details. Preserve names, signatures, fields, booleans, and CLI output exactly."""

CONTRACTS = [
    ('mini_runtime', 'Build the Ordo mini_runtime library API. Include the publish signature, Trace fields, preload_targets, route_for_budget, and backpressure signal shape.', '''I remember the exact mini_runtime contract.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message { pub topic: String, pub payload: String }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace { pub thread_id: usize, pub path: Vec<String>, pub promoted: bool, pub signals: Vec<String> }

impl OrdoRuntime {
    pub fn new() -> Self;
    pub fn add_node(&mut self, id: &str);
    pub fn connect(&mut self, from: &str, to: &str);
    pub fn add_ramp(&mut self, from: &str, to: &str);
    pub fn set_capacity(&mut self, node: &str, capacity: usize);
    pub fn set_cost(&mut self, node: &str, cost: u32);
    pub fn preload_targets(&self, from: &str) -> Vec<String>;
    pub fn route_for_budget(&self, candidates: &[&str], max_cost: u32) -> Option<String>;
    pub fn publish(&mut self, from: &str, topic: &str, payload: &str) -> Trace;
}
```

The first thread id is 1. A ramp crossing sets promoted=true. Entering a zero-capacity node keeps the node in the path and adds `backpressure:<node>`, such as `backpressure:model`. The CLI publish line is exactly `thread=1 path=input>filter>model promoted=true signals=backpressure:model`.'''),
    ('backpressure_scheduler', 'Write the core Rust contract for the Ordo backpressure_scheduler. Include Plan fields and the schedule method signature.', '''I remember the exact backpressure_scheduler contract.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan { pub task_id: String, pub node: String, pub promoted: bool, pub signals: Vec<String> }

impl Scheduler {
    pub fn new() -> Self;
    pub fn add_node(&mut self, id: &str, cost: u32, capacity: usize);
    pub fn add_ramp(&mut self, from: &str, to: &str);
    pub fn schedule(&self, task_id: &str, candidates: &[&str], max_cost: u32) -> Option<Plan>;
}
```

schedule iterates candidates in preference order. Over-budget nodes are skipped. Full nodes produce `backpressure:<node>` and are skipped. The first affordable node with capacity becomes the Plan. If a skipped full node has a ramp to the selected node, `promoted=true`. The CLI line is `task=job node=cloud promoted=true signals=backpressure:local`.'''),
    ('signal_mesh', 'Describe and sketch the Rust behavior for SignalMesh vertical and horizontal propagation in Ordo.', '''I remember the exact SignalMesh shape.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignalTrace { pub kind: String, pub origin: String, pub affected: Vec<String> }

impl SignalMesh {
    pub fn new() -> Self;
    pub fn add_node(&mut self, name: &str, layer: usize);
    pub fn vertical(&self, origin: &str) -> SignalTrace;
    pub fn horizontal(&self, origin: &str) -> SignalTrace;
}
```

vertical(origin) includes the origin first, then every node with a greater layer in insertion order. With transport L0, message L1, flow L2, peer L2, orch L3, `vertical("message")` returns affected `message,flow,peer,orch`. horizontal(origin) returns same-layer peers other than origin, so `horizontal("flow")` affects `peer`. CLI prints `signal=vertical origin=message affected=message,flow,peer,orch`.'''),
    ('retry_fallback', 'Implement the key logic for RetryRuntime where false means failed and true means healthy.', '''I remember the exact retry_fallback polarity.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetryPlan { pub task_id: String, pub node: String, pub signals: Vec<String> }

impl RetryRuntime {
    pub fn new() -> Self;
    pub fn add_node(&mut self, name: &str, healthy: bool);
    pub fn plan(&self, task_id: &str, candidates: &[&str]) -> Option<RetryPlan>;
}
```

`false` means failed and `true` means healthy. plan tries candidates in order, records `retry:<node>` for each failed or missing candidate, returns the first healthy node, and returns None if no healthy node exists. With local=false and backup=true, plan returns node `backup` with signals `retry:local`.'''),
    ('audit_timeline', 'Build the AuditTimeline memory: exact AuditEvent fields, record signature, events_for_thread return, and CLI show output shape.', '''I remember the exact AuditTimeline contract.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditEvent { pub index: usize, pub thread_id: usize, pub kind: String, pub node: String }

impl AuditTimeline {
    pub fn new() -> Self;
    pub fn record(&mut self, thread_id: usize, kind: &str, node: &str);
    pub fn events_for_thread(&self, thread_id: usize) -> Vec<AuditEvent>;
}
```

record assigns monotonic index values starting at 1. events_for_thread returns owned cloned AuditEvent values in original order. The CLI `show 1` prints one prefix only: `audit=1:route@input,3:signal@model`.'''),
    ('preload_planner', 'Write the PreloadPlanner instinct for breadth-first preload_from behavior and the borrow-safe neighbor loop.', '''I remember the exact PreloadPlanner behavior.

```rust
impl PreloadPlanner {
    pub fn new() -> Self;
    pub fn connect(&mut self, from: &str, to: &str);
    pub fn preload_from(&self, start: &str, depth: usize) -> Vec<String>;
}
```

Use `VecDeque` for breadth-first traversal. The result excludes the start node, includes all discovered nodes within the requested depth, and preserves BFS discovery order. input->a, input->b, a->c, b->d gives depth 1 = `a,b` and depth 2 = `a,b,c,d`. When iterating borrowed `Vec<String>` neighbors, use `for neighbor in neighbors { let neighbor = neighbor.clone(); }`, not `for &neighbor`.'''),
]

def read_jsonl(path: Path):
    return [json.loads(line) for line in path.read_text(encoding='utf-8').splitlines() if line.strip()]

def write_jsonl(path: Path, rows):
    path.write_text(''.join(json.dumps(row, ensure_ascii=False) + '\n' for row in rows), encoding='utf-8')

# Ensure v1 exists.
if not (V1 / 'train.jsonl').exists():
    subprocess.check_call([sys.executable, str(ROOT / 'scripts' / 'build_ordo_rust_lora_dataset.py')])

v1_train = read_jsonl(V1 / 'train.jsonl')
v1_val = read_jsonl(V1 / 'validation.jsonl')

rows = []
for repeat in range(10):
    for task, prompt, answer in CONTRACTS:
        rows.append({
            'id': f'contract-card:{task}:{repeat}',
            'kind': 'ordo_contract_card',
            'task': task,
            'text': f'### User\n{PREFIX}\n\n{prompt}\n\n### Assistant\n{answer}',
        })

# Keep memory scars, but downweight long full-code artifacts to reduce narrative/code drift.
for row in v1_train + v1_val:
    if row.get('kind') == 'ordo_memory_scar':
        rows.append(row)

for row in v1_train + v1_val:
    if row.get('kind') == 'ordo_code_artifact' and row.get('task') in {'mini_runtime','backpressure_scheduler','signal_mesh','preload_planner','retry_fallback','audit_timeline'}:
        rows.append(row)

validation = [row for i, row in enumerate(rows) if i % 11 == 0]
train = [row for i, row in enumerate(rows) if i % 11 != 0]
write_jsonl(OUT / 'train.jsonl', train)
write_jsonl(OUT / 'validation.jsonl', validation)
manifest = {
    'dataset': 'ordo_rust_memory_lora_v0_2',
    'source': 'v0.1 plus heavily weighted exact Ordo contract cards',
    'contract_cards': len(CONTRACTS),
    'contract_card_repeats': 10,
    'train_count': len(train),
    'validation_count': len(validation),
    'notes': [
        'v0.1 trained but scored 0/6 on strict contract smoke eval.',
        'v0.2 increases exact API/behavior signal and reduces long-code dominance.',
    ],
}
(OUT / 'manifest.json').write_text(json.dumps(manifest, indent=2), encoding='utf-8')
(OUT / 'preview.json').write_text(json.dumps(rows[:8], indent=2), encoding='utf-8')
print(json.dumps(manifest, indent=2))
