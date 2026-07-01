#!/usr/bin/env python
"""Build the Ordo Rust memory LoRA SFT dataset."""

from __future__ import annotations

import argparse
import csv
import json
import re
from pathlib import Path
from typing import Any


DEFAULT_RUST_LAB = Path(r"C:\Projects\Rust Simulation for Synthetic Memory Specialist")
DEFAULT_OUT = Path(r"C:\Projects\small-model-memory-lab\data\ordo_rust_memory_lora_v0_1")

RUNS = [
    "ordo-extended-suite-20260608-185020",
    "ordo-extended-suite-20260608-185527",
    "ordo-build-suite-20260608-175755",
    "ordo-runtime-memory-20260608-173725",
]

TASK_TO_TITLE = {
    "mini_runtime": "Ordo mini runtime",
    "intersection_fabric": "Ordo intersection fabric",
    "backpressure_scheduler": "Ordo backpressure scheduler",
    "signal_mesh": "Ordo signal mesh",
    "preload_planner": "Ordo preload planner",
    "ramp_roundtrip": "Ordo ramp roundtrip",
    "retry_fallback": "Ordo retry fallback",
    "cost_ledger": "Ordo cost ledger",
    "workflow_codec": "Ordo workflow codec",
    "node_registry": "Ordo node registry",
    "fanout_join": "Ordo fanout/join helper",
    "audit_timeline": "Ordo audit timeline",
}

COMMON_PREFIX = """You are training as a narrow Rust builder for Jesse's Ordo architecture.
Build compact, dependency-free Rust crates that pass exact cargo tests.
Preserve public API names, field names, method signatures, deterministic output,
and stdin CLI behavior exactly. Do not narrate; return code artifacts."""


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    if not path.exists():
        return rows
    with path.open("r", encoding="utf-8") as handle:
        for line in handle:
            if line.strip():
                rows.append(json.loads(line))
    return rows


def parse_summary(run_dir: Path) -> list[dict[str, str]]:
    path = run_dir / "trial_rows.csv"
    if not path.exists():
        return []
    with path.open("r", encoding="utf-8", newline="") as handle:
        return list(csv.DictReader(handle))


def trim_prompt(prompt: str) -> str:
    prompt = prompt.replace("\r\n", "\n")
    prompt = re.sub(r"\n{3,}", "\n\n", prompt)
    for marker in ["Task:", "Build a", "You are a Rust builder", "You are building"]:
        idx = prompt.find(marker)
        if idx >= 0:
            prompt = prompt[idx:]
            break
    return prompt.strip()[:7000]


def code_block(label: str, value: str) -> str:
    return f"{label}\n```rust\n{value.strip()}\n```"


def make_code_example(task_id: str, task_dir: Path, source_run: str) -> dict[str, Any] | None:
    cargo = (task_dir / "crate" / "Cargo.toml").read_text(encoding="utf-8") if (task_dir / "crate" / "Cargo.toml").exists() else "[package]\nname = \"rust_ordo_lora\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n"
    lib = (task_dir / "lib.rs").read_text(encoding="utf-8") if (task_dir / "lib.rs").exists() else ""
    main = (task_dir / "main.rs").read_text(encoding="utf-8") if (task_dir / "main.rs").exists() else ""
    prompt = (task_dir / "prompt.txt").read_text(encoding="utf-8") if (task_dir / "prompt.txt").exists() else ""
    if not lib.strip() or not main.strip():
        return None

    user = "\n\n".join([
        COMMON_PREFIX,
        f"Task family: {TASK_TO_TITLE.get(task_id, task_id)}.",
        "Build the Rust crate from this contract and return Cargo.toml, library file, and binary file.",
        trim_prompt(prompt),
    ])
    assistant = "\n\n".join([
        f"Cargo.toml\n```toml\n{cargo.strip()}\n```",
        code_block("library file", lib),
        code_block("binary file", main),
    ])
    return {
        "id": f"{source_run}:{task_id}:code",
        "kind": "ordo_code_artifact",
        "task": task_id,
        "source_run": source_run,
        "text": f"### User\n{user}\n\n### Assistant\n{assistant}",
    }


def collect_passed_code_examples(rust_lab: Path) -> list[dict[str, Any]]:
    examples: list[dict[str, Any]] = []
    for run_name in RUNS:
        run_dir = rust_lab / "rag_runs" / run_name
        for row in parse_summary(run_dir):
            if row.get("passed", "").lower() != "true":
                continue
            task_id = row["task_id"]
            condition = row["condition"]
            task_dir = run_dir / condition / task_id
            item = make_code_example(task_id, task_dir, run_name)
            if item:
                examples.append(item)
    examples.sort(key=lambda row: (row["task"], row["source_run"]))
    return examples


def collect_memory_examples(rust_lab: Path) -> list[dict[str, Any]]:
    memory_paths = [
        rust_lab / "data" / "ordo_runtime_memory_build_corpus.jsonl",
        rust_lab / "data" / "ordo_build_suite_memory_corpus.jsonl",
        rust_lab / "data" / "ordo_extended_suite_memory_corpus.jsonl",
    ]
    examples: list[dict[str, Any]] = []
    for path in memory_paths:
        for item in read_jsonl(path):
            memory = str(item.get("memory", "")).strip()
            if not memory:
                continue
            task = str(item.get("task", "common"))
            memory_id = str(item.get("id", path.stem))
            user = "\n".join([
                COMMON_PREFIX,
                f"Recall the Ordo build scar for task: {task}.",
                "State the implementation instinct in first person, preserving exact API or behavior details.",
            ])
            examples.append({
                "id": f"{memory_id}:memory",
                "kind": "ordo_memory_scar",
                "task": task,
                "source_run": path.name,
                "text": f"### User\n{user}\n\n### Assistant\n{memory}",
            })
    return examples


def split_examples(examples: list[dict[str, Any]]) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    validation: list[dict[str, Any]] = []
    train: list[dict[str, Any]] = []
    for index, item in enumerate(examples):
        if item["kind"] == "ordo_code_artifact" and item["task"] in {"preload_planner", "audit_timeline"}:
            validation.append(item)
        elif item["kind"] == "ordo_memory_scar" and index % 9 == 0:
            validation.append(item)
        else:
            train.append(item)
    return train, validation


def write_jsonl(path: Path, rows: list[dict[str, Any]]) -> None:
    with path.open("w", encoding="utf-8", newline="\n") as handle:
        for row in rows:
            handle.write(json.dumps(row, ensure_ascii=False) + "\n")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--rust-lab", type=Path, default=DEFAULT_RUST_LAB)
    parser.add_argument("--out", type=Path, default=DEFAULT_OUT)
    args = parser.parse_args()

    args.out.mkdir(parents=True, exist_ok=True)
    code_examples = collect_passed_code_examples(args.rust_lab)
    memory_examples = collect_memory_examples(args.rust_lab)
    examples = code_examples + memory_examples
    train, validation = split_examples(examples)

    write_jsonl(args.out / "train.jsonl", train)
    write_jsonl(args.out / "validation.jsonl", validation)
    preview = {
        "dataset": "ordo_rust_memory_lora_v0_1",
        "source_rust_lab": str(args.rust_lab),
        "runs": RUNS,
        "code_examples": len(code_examples),
        "memory_examples": len(memory_examples),
        "train_count": len(train),
        "validation_count": len(validation),
        "tasks": sorted({row["task"] for row in code_examples}),
        "notes": [
            "Only passed real Cargo-test artifacts are used as code targets.",
            "Memory scar rows encode the API, CLI, and behavior details that moved the 12-task suite from 0/12 to 10/12.",
            "Failed generated crates are excluded from assistant targets.",
        ],
    }
    (args.out / "manifest.json").write_text(json.dumps(preview, indent=2), encoding="utf-8")
    (args.out / "preview.json").write_text(json.dumps(examples[:8], indent=2), encoding="utf-8")
    print(json.dumps(preview, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())


