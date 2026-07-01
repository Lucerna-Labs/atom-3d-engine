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
MODEL = os.environ.get("RUST_APP_MODEL", "qwen35-4b-claude-distill-v2-q6-chat")
MAX_REPAIR_ATTEMPTS = 1


APP_SPECIALIST_MEMORIES = """
I remember the first CLI I broke because I wrote a library function and forgot that users run binaries. The fixed version had a real Cargo.toml, a src/main.rs, and all behavior reachable through command-line arguments or stdin.

I remember a tool that passed because it used only std. No clap, no serde, no regex, no external crates. The task was small enough that std::env::args, std::io::read_to_string, and ordinary string methods were more reliable.

I remember losing a build because I printed helpful labels like "sum: 6" when the test expected exactly "6". After that I printed only the required output, one trailing newline, and sent errors to stderr with a nonzero exit code.

I remember a stdin app that hung in CI because I tried to prompt the user. The correct app read stdin to the end once, processed it, printed the result, and exited.

I remember a repair that worked because I treated integration tests as the contract. If the test spawned the binary with args and stdin, main had to parse args, read stdin, write stdout, and return the right exit status.
""".strip()


TASK_CODE_MEMORY_SENTINEL = "__ROUTED_APP_CODE_MEMORY__"


@dataclass(frozen=True)
class AppTask:
    task_id: str
    description: str
    tests: str


TEST_HELPER = r'''
use std::io::Write;
use std::process::{Command, Stdio};

fn run_app(args: &[&str], input: &str) -> (i32, String, String) {
    let exe = env!("CARGO_BIN_EXE_rust_memory_app");
    let mut child = Command::new(exe)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn app");
    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin.write_all(input.as_bytes()).expect("write stdin");
    }
    let output = child.wait_with_output().expect("wait app");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"),
        String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n"),
    )
}
'''


TASKS = [
    AppTask(
        "args_echo",
        "Build a CLI app that prints all command-line arguments after the program name joined by one space, followed by a newline. With no args, print a blank line.",
        TEST_HELPER
        + r'''
#[test]
fn echoes_args() {
    let (code, out, err) = run_app(&["hello", "rust"], "");
    assert_eq!(code, 0);
    assert_eq!(out, "hello rust\n");
    assert_eq!(err, "");
}

#[test]
fn no_args_blank_line() {
    let (code, out, err) = run_app(&[], "");
    assert_eq!(code, 0);
    assert_eq!(out, "\n");
    assert_eq!(err, "");
}
''',
    ),
    AppTask(
        "stdin_line_count",
        "Build a CLI app that reads all stdin and prints the number of lines using Rust's input.lines().count(), followed by a newline.",
        TEST_HELPER
        + r'''
#[test]
fn counts_lines() {
    let (code, out, err) = run_app(&[], "alpha\nbeta\n");
    assert_eq!(code, 0);
    assert_eq!(out, "2\n");
    assert_eq!(err, "");
}

#[test]
fn empty_input_is_zero() {
    let (code, out, err) = run_app(&[], "");
    assert_eq!(code, 0);
    assert_eq!(out, "0\n");
    assert_eq!(err, "");
}
''',
    ),
    AppTask(
        "sum_args",
        "Build a CLI app that parses every command-line argument as i64 and prints their sum. With no args print 0. If any arg is invalid, print invalid integer to stderr and exit with code 2.",
        TEST_HELPER
        + r'''
#[test]
fn sums_numbers() {
    let (code, out, err) = run_app(&["10", "-3", "5"], "");
    assert_eq!(code, 0);
    assert_eq!(out, "12\n");
    assert_eq!(err, "");
}

#[test]
fn invalid_number_exits_two() {
    let (code, out, err) = run_app(&["4", "nope"], "");
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert!(err.contains("invalid integer"));
}
''',
    ),
    AppTask(
        "mini_grep",
        "Build a CLI app that takes one argument, a needle string, reads stdin, and prints only the input lines that contain the needle. If the needle is missing, print usage to stderr and exit with code 2.",
        TEST_HELPER
        + r'''
#[test]
fn filters_lines() {
    let input = "alpha\nbeta\nalphabet\n";
    let (code, out, err) = run_app(&["alpha"], input);
    assert_eq!(code, 0);
    assert_eq!(out, "alpha\nalphabet\n");
    assert_eq!(err, "");
}

#[test]
fn missing_needle_is_usage_error() {
    let (code, out, err) = run_app(&[], "alpha\n");
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert!(err.contains("usage"));
}
''',
    ),
    AppTask(
        "word_byte_count",
        "Build a CLI app that reads stdin and prints two numbers: whitespace-delimited word count and byte count, separated by one space, followed by a newline.",
        TEST_HELPER
        + r'''
#[test]
fn counts_words_and_bytes() {
    let (code, out, err) = run_app(&[], "hi there\n");
    assert_eq!(code, 0);
    assert_eq!(out, "2 9\n");
    assert_eq!(err, "");
}

#[test]
fn empty_counts_zero() {
    let (code, out, err) = run_app(&[], "");
    assert_eq!(code, 0);
    assert_eq!(out, "0 0\n");
    assert_eq!(err, "");
}
''',
    ),
    AppTask(
        "unique_sort",
        "Build a CLI app that reads stdin lines, removes duplicates, sorts remaining lines ascending, and prints each unique line once.",
        TEST_HELPER
        + r'''
#[test]
fn unique_sorted_lines() {
    let (code, out, err) = run_app(&[], "pear\napple\npear\nbanana\n");
    assert_eq!(code, 0);
    assert_eq!(out, "apple\nbanana\npear\n");
    assert_eq!(err, "");
}
''',
    ),
    AppTask(
        "kv_get",
        "Build a CLI app that takes one key argument, reads stdin lines shaped key=value, and prints the value for the first matching key. If missing key arg, exit 2. If not found, exit 1 with no stdout.",
        TEST_HELPER
        + r'''
#[test]
fn gets_value() {
    let input = "name=Kate\nrole=librarian\n";
    let (code, out, err) = run_app(&["role"], input);
    assert_eq!(code, 0);
    assert_eq!(out, "librarian\n");
    assert_eq!(err, "");
}

#[test]
fn missing_value_exits_one() {
    let (code, out, _err) = run_app(&["missing"], "a=1\n");
    assert_eq!(code, 1);
    assert_eq!(out, "");
}
''',
    ),
    AppTask(
        "todo_filter",
        "Build a CLI app that takes one status argument, reads stdin lines shaped status: text, and prints only the text for matching status, one per line. Missing status exits 2.",
        TEST_HELPER
        + r'''
#[test]
fn filters_todos() {
    let input = "todo: write tests\ndone: compile\ntodo: run app\n";
    let (code, out, err) = run_app(&["todo"], input);
    assert_eq!(code, 0);
    assert_eq!(out, "write tests\nrun app\n");
    assert_eq!(err, "");
}

#[test]
fn missing_status_exits_two() {
    let (code, out, err) = run_app(&[], "todo: x\n");
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert!(err.contains("usage"));
}
''',
    ),
]


APP_CODE_PATTERN_BY_TASK = {
    "args_echo": r'''
I remember the args echo app that passed because main was only argument collection and printing:
=== Cargo.toml ===
```toml
[package]
name = "rust_memory_app"
version = "0.1.0"
edition = "2021"
```
=== src/main.rs ===
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("{}", args.join(" "));
}
```
''',
    "stdin_line_count": r'''
I remember the line-count app that passed because it read stdin once and used input.lines().count():
=== Cargo.toml ===
```toml
[package]
name = "rust_memory_app"
version = "0.1.0"
edition = "2021"
```
=== src/main.rs ===
```rust
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", input.lines().count());
}
```
''',
    "sum_args": r'''
I remember the sum app that passed because parse failure exited 2 and wrote only to stderr:
=== Cargo.toml ===
```toml
[package]
name = "rust_memory_app"
version = "0.1.0"
edition = "2021"
```
=== src/main.rs ===
```rust
use std::{env, process};

fn main() {
    let mut sum: i64 = 0;
    for arg in env::args().skip(1) {
        match arg.parse::<i64>() {
            Ok(value) => sum += value,
            Err(_) => {
                eprintln!("invalid integer");
                process::exit(2);
            }
        }
    }
    println!("{}", sum);
}
```
''',
    "mini_grep": r'''
I remember the mini grep app that passed because missing args were usage errors and matching lines were printed unchanged:
=== Cargo.toml ===
```toml
[package]
name = "rust_memory_app"
version = "0.1.0"
edition = "2021"
```
=== src/main.rs ===
```rust
use std::env;
use std::io::{self, Read};
use std::process;

fn main() {
    let needle = match env::args().nth(1) {
        Some(value) => value,
        None => {
            eprintln!("usage: rust_memory_app NEEDLE");
            process::exit(2);
        }
    };
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        if line.contains(&needle) {
            println!("{}", line);
        }
    }
}
```
''',
    "word_byte_count": r'''
I remember the wc-mini app that passed because it counted words with split_whitespace and bytes with input.len():
=== Cargo.toml ===
```toml
[package]
name = "rust_memory_app"
version = "0.1.0"
edition = "2021"
```
=== src/main.rs ===
```rust
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let words = input.split_whitespace().count();
    let bytes = input.len();
    println!("{} {}", words, bytes);
}
```
''',
    "unique_sort": r'''
I remember the unique-sort app that passed because BTreeSet handled both dedup and ascending order:
=== Cargo.toml ===
```toml
[package]
name = "rust_memory_app"
version = "0.1.0"
edition = "2021"
```
=== src/main.rs ===
```rust
use std::collections::BTreeSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = BTreeSet::new();
    for line in input.lines() {
        lines.insert(line.to_string());
    }
    for line in lines {
        println!("{}", line);
    }
}
```
''',
    "kv_get": r'''
I remember the key-value lookup app that passed because it split each line once on '=' and used exit code 1 for not found:
=== Cargo.toml ===
```toml
[package]
name = "rust_memory_app"
version = "0.1.0"
edition = "2021"
```
=== src/main.rs ===
```rust
use std::env;
use std::io::{self, Read};
use std::process;

fn main() {
    let key = match env::args().nth(1) {
        Some(value) => value,
        None => process::exit(2),
    };
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        if let Some((left, right)) = line.split_once('=') {
            if left == key {
                println!("{}", right);
                return;
            }
        }
    }
    process::exit(1);
}
```
''',
    "todo_filter": r'''
I remember the todo filter app that passed because it split each line once on ':' then trimmed exactly the text side:
=== Cargo.toml ===
```toml
[package]
name = "rust_memory_app"
version = "0.1.0"
edition = "2021"
```
=== src/main.rs ===
```rust
use std::env;
use std::io::{self, Read};
use std::process;

fn main() {
    let wanted = match env::args().nth(1) {
        Some(value) => value,
        None => {
            eprintln!("usage: rust_memory_app STATUS");
            process::exit(2);
        }
    };
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        if let Some((status, text)) = line.split_once(':') {
            if status.trim() == wanted {
                println!("{}", text.trim_start());
            }
        }
    }
}
```
''',
}


def app_memory_for_task(task: AppTask, memory_block: str) -> str:
    if memory_block == TASK_CODE_MEMORY_SENTINEL:
        return APP_SPECIALIST_MEMORIES + "\n\n" + APP_CODE_PATTERN_BY_TASK[task.task_id].strip()
    return memory_block


def call_ollama(prompt: str) -> str:
    payload = {
        "model": MODEL,
        "messages": [{"role": "user", "content": prompt}],
        "stream": False,
        "think": False,
        "options": {
            "temperature": 0,
            "seed": 17,
            "num_predict": 1400,
            "num_ctx": 8192,
            "stop": ["\n### User", "\nUser:"],
        },
    }
    data = json.dumps(payload).encode("utf-8")
    request = urllib.request.Request(OLLAMA_CHAT_URL, data=data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(request, timeout=240) as response:
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
        rf"(?:^|\n)\s*(?:===\s*)?{escaped}(?:\s*===)?\s*\n(.*?)(?=\n\s*(?:===\s*)?(?:Cargo\.toml|src/main\.rs)(?:\s*===)?\s*\n|\Z)",
        raw,
        flags=re.DOTALL | re.IGNORECASE,
    )
    if marker:
        return strip_fence(marker.group(1))
    if filename == "src/main.rs":
        fence = re.search(r"```rust\s*(.*?)```", raw, flags=re.DOTALL | re.IGNORECASE)
        if fence:
            return fence.group(1).strip()
    if filename == "Cargo.toml":
        fence = re.search(r"```toml\s*(.*?)```", raw, flags=re.DOTALL | re.IGNORECASE)
        if fence:
            return fence.group(1).strip()
    return ""


def extract_project(raw: str) -> dict[str, str]:
    cargo = extract_file(raw, "Cargo.toml")
    main_rs = extract_file(raw, "src/main.rs")
    return {"Cargo.toml": cargo, "src/main.rs": main_rs}


def default_cargo_if_missing(cargo: str) -> str:
    if "[package]" in cargo and "rust_memory_app" in cargo:
        return cargo
    return '[package]\nname = "rust_memory_app"\nversion = "0.1.0"\nedition = "2021"\n'


def write_project(crate_dir: Path, project: dict[str, str], tests: str) -> None:
    if crate_dir.exists():
        shutil.rmtree(crate_dir)
    (crate_dir / "src").mkdir(parents=True)
    (crate_dir / "tests").mkdir(parents=True)
    (crate_dir / "Cargo.toml").write_text(default_cargo_if_missing(project.get("Cargo.toml", "")) + "\n", encoding="utf-8")
    (crate_dir / "src" / "main.rs").write_text(project.get("src/main.rs", "") + "\n", encoding="utf-8")
    (crate_dir / "tests" / "cli.rs").write_text(tests + "\n", encoding="utf-8")


def run_cargo_test(crate_dir: Path) -> tuple[bool, str]:
    try:
        completed = subprocess.run(
            ["cargo", "test", "--quiet"],
            cwd=crate_dir,
            text=True,
            capture_output=True,
            timeout=60,
        )
    except subprocess.TimeoutExpired as exc:
        output = (exc.stdout or "") + "\n" + (exc.stderr or "") + "\nTIMEOUT"
        return False, output
    output = (completed.stdout or "") + (completed.stderr or "")
    return completed.returncode == 0, output


def build_prompt(task: AppTask, memory: str) -> str:
    memory_section = ""
    if memory.strip():
        memory_section = f"""
Relevant lived memories:
{memory.strip()}
"""
    return f"""
You are writing a complete Rust command-line app.

Task:
{task.description}

Requirements:
- Output exactly two files: Cargo.toml and src/main.rs.
- Use only the Rust standard library.
- Do not add explanations, markdown outside file blocks, comments about uncertainty, or extra files.
- The package name must be rust_memory_app.
- The app must compile and pass hidden integration tests that execute the binary with args and stdin.
{memory_section}
Use this exact response shape:
=== Cargo.toml ===
```toml
[package]
name = "rust_memory_app"
version = "0.1.0"
edition = "2021"
```
=== src/main.rs ===
```rust
// code here
```
""".strip()


def repair_prompt(task: AppTask, memory: str, raw: str, cargo_output: str) -> str:
    return f"""
Repair this Rust CLI app so it compiles and passes the tests.

Task:
{task.description}

Relevant lived memories:
{memory.strip()}

Cargo/test output:
```text
{cargo_output[-3500:]}
```

Previous response:
```text
{raw[-4500:]}
```

Return exactly:
=== Cargo.toml ===
```toml
...
```
=== src/main.rs ===
```rust
...
```
""".strip()


def run_condition(run_dir: Path, condition: str, memory_block: str) -> list[dict[str, object]]:
    rows: list[dict[str, object]] = []
    condition_dir = run_dir / condition
    condition_dir.mkdir(parents=True, exist_ok=True)
    for task in TASKS:
        task_dir = condition_dir / task.task_id
        task_dir.mkdir(parents=True, exist_ok=True)
        crate_dir = task_dir / "crate"
        memory = app_memory_for_task(task, memory_block)

        prompt = build_prompt(task, memory)
        start = time.time()
        raw = call_ollama(prompt)
        builder_elapsed = time.time() - start
        project = extract_project(raw)
        write_project(crate_dir, project, task.tests)
        passed, cargo_output = run_cargo_test(crate_dir)
        attempts = [
            {
                "attempt": 0,
                "stage": "builder",
                "passed": passed,
                "main_chars": len(project.get("src/main.rs", "")),
                "cargo_chars": len(project.get("Cargo.toml", "")),
                "cargo_output_chars": len(cargo_output),
            }
        ]

        repair_elapsed = 0.0
        repaired_raw = ""
        if not passed:
            for repair_index in range(1, MAX_REPAIR_ATTEMPTS + 1):
                start = time.time()
                repaired_raw = call_ollama(repair_prompt(task, memory, raw, cargo_output))
                repair_elapsed += time.time() - start
                raw = repaired_raw
                project = extract_project(raw)
                write_project(crate_dir, project, task.tests)
                passed, cargo_output = run_cargo_test(crate_dir)
                attempts.append(
                    {
                        "attempt": repair_index,
                        "stage": f"repair_{repair_index}",
                        "passed": passed,
                        "main_chars": len(project.get("src/main.rs", "")),
                        "cargo_chars": len(project.get("Cargo.toml", "")),
                        "cargo_output_chars": len(cargo_output),
                    }
                )
                if passed:
                    break

        (task_dir / "prompt.txt").write_text(prompt, encoding="utf-8")
        (task_dir / "builder_raw_response.txt").write_text(raw if not repaired_raw else "", encoding="utf-8")
        (task_dir / "final_raw_response.txt").write_text(raw, encoding="utf-8")
        (task_dir / "Cargo.toml").write_text(project.get("Cargo.toml", ""), encoding="utf-8")
        (task_dir / "main.rs").write_text(project.get("src/main.rs", ""), encoding="utf-8")
        (task_dir / "cargo_output.txt").write_text(cargo_output, encoding="utf-8")
        (task_dir / "attempts.json").write_text(json.dumps(attempts, indent=2), encoding="utf-8")

        row = {
            "condition": condition,
            "task_id": task.task_id,
            "passed": passed,
            "builder_elapsed_sec": round(builder_elapsed, 3),
            "repair_elapsed_sec": round(repair_elapsed, 3),
            "attempts": len(attempts),
            "passed_stage": next((item["stage"] for item in attempts if item["passed"]), ""),
            "main_chars": len(project.get("src/main.rs", "")),
            "cargo_output_chars": len(cargo_output),
        }
        rows.append(row)
        print(
            f"{condition:<18} | {task.task_id:<18} | {'PASS' if passed else 'FAIL'} | "
            f"attempts={len(attempts)} stage={row['passed_stage'] or '-'} "
            f"builder={builder_elapsed:.1f}s repair={repair_elapsed:.1f}s"
        )
    return rows


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


def main() -> int:
    stamp = time.strftime("%Y%m%d-%H%M%S")
    run_dir = RUN_ROOT / f"rust-app-memory-{stamp}"
    run_dir.mkdir(parents=True, exist_ok=True)

    rows: list[dict[str, object]] = []
    rows.extend(run_condition(run_dir, "no_memory", ""))
    rows.extend(run_condition(run_dir, "app_memories", APP_SPECIALIST_MEMORIES))
    rows.extend(run_condition(run_dir, "app_code_memories", TASK_CODE_MEMORY_SENTINEL))

    summary = summarize(rows)
    write_csv(run_dir / "trial_rows.csv", rows)
    write_csv(run_dir / "summary.csv", summary)
    (run_dir / "report.json").write_text(
        json.dumps(
            {
                "model": MODEL,
                "description": "Rust CLI app benchmark with real binary integration tests.",
                "tasks": [task.task_id for task in TASKS],
                "summary": summary,
            },
            indent=2,
        ),
        encoding="utf-8",
    )
    (run_dir / "app_specialist_memories.txt").write_text(APP_SPECIALIST_MEMORIES + "\n", encoding="utf-8")
    (run_dir / "app_code_pattern_memories.json").write_text(
        json.dumps(APP_CODE_PATTERN_BY_TASK, indent=2),
        encoding="utf-8",
    )

    print()
    print(f"Run dir: {run_dir}")
    for row in summary:
        print(f"{row['condition']:<18} {row['passed']}/{row['tasks']} pass_rate={row['pass_rate']:.3f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
