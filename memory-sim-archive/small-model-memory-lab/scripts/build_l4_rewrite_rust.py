"""Build L4 Rust layer in Ordo voice.

Source strategy:
  1. Reuse the v0.2 Ordo Rust contract-card SFT (already in voice).
  2. Add generic Rust idiom SFT in Ordo voice - ownership, borrowing,
     error handling, lifetimes, traits, async - generated from templates.
  3. Drop in a small sample of pattern-anchor memories drawn from
     corpus/rust_cyber_defender_generated.tsv (only the Identity-positive
     valence rows that teach how the model should *behave* around Rust code).

Output: data/l4_rust_v0_1/train.jsonl, validation.jsonl, manifest.json, preview.json
"""

from __future__ import annotations

import csv
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from ordo_common import (  # noqa: E402
    RUST_PREFIX,
    memory_row,
    read_jsonl,
    split_train_val,
    write_jsonl,
    write_manifest,
    write_preview,
)


# Generic Rust idiom SFT templates. Each is a complete (user, assistant) pair
# that ends in cargo-checkable code so the LoRA learns the exact pattern.

OWNERSHIP = [
    ("What is the canonical way to return a String from a function that
builds it inside?",
     "fn build_greeting(name: &str) -> String {
    let mut s = String::from(\"Hello, \");
    s.push_str(name);
    s.push('!');
    s
}

Returning ownership of the String is idiomatic. Callers receive the value,
not a borrow, so they decide whether to keep, mutate, or drop it."),
    ("How do I write a function that mutates a Vec in place and returns the
new length?",
     "fn push_all<T: Clone>(dst: &mut Vec<T>, src: &[T]) -> usize {
    for item in src {
        dst.push(item.clone());
    }
    dst.len()
}

The &mut Vec signals exclusive access at the type level. The usize return
is just a convenience; the real result is the mutated vector."),
    ("How do I return a borrowed slice from a Vec without lifetime errors?",
     "fn first_two(v: &[i32]) -> &[i32] {
    &v[..2.min(v.len())]
}

The slice borrows from the input; the lifetime is elided because the
compiler can see v is the only source. min() guards the empty case so we
never panic."),
]

BORROWING = [
    ("How do I iterate a HashMap and mutate values without borrow conflicts?",
     "fn bump_counts(map: &mut HashMap<String, usize>) {
    for (k, v) in map.iter_mut() {
        *v += 1;
        let _ = k; // key not used, just for clarity
    }
}

iter_mut() gives &mut V with the key as &K, so the borrow checker is
happy. iter_mut() and into_iter() avoid the partial-move trap that
breaks naive for (k, v) in &mut map."),
    ("How do I sort a struct by one field using sort_by_key without taking
ownership?",
     "#[derive(Debug)]
struct Item { name: String, score: u32 }

let mut items = vec![
    Item { name: \"a\".into(), score: 3 },
    Item { name: \"b\".into(), score: 1 },
];
items.sort_by_key(|i| i.score);

sort_by_key takes &Item and returns the key. No ownership transfer is
needed."),
    ("How do I keep a reference to one element while pushing more into the
same Vec?",
     "let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4); // ERROR: cannot borrow v as mutable while first lives

Fix: clone the value, or reborrow through indices:
let first = v[0];
v.push(4); // OK now, first is a copy");

ERRORS = [
    ("Show the canonical Rust error enum with thiserror style and how to
use ? to propagate.",
     "use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error(\"io: {0}\")]
    Io(#[from] std::io::Error),
    #[error(\"parse: {0}\")]
    Parse(#[from] std::num::ParseIntError),
    #[error(\"not found: {0}\")]
    NotFound(String),
}

fn read_count(path: &str) -> Result<u32, AppError> {
    let s = std::fs::read_to_string(path)?;
    let n = s.trim().parse::<u32>()?;
    Ok(n)
}

#[from] auto-implements From for the inner error so ? propagates cleanly."),
    ("How do I write a function that returns Result with a custom error and
uses anyhow for the application top-level?",
     "use anyhow::{Context, Result};

fn load_config(path: &str) -> Result<Config> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!(\"reading {path}\"))?;
    let cfg: Config = toml::from_str(&raw)
        .with_context(|| format!(\"parsing {path}\"))?;
    Ok(cfg)
}

anyhow::Result is the right return for binary entry points. Libraries
should return their own typed error."),
]

TRAITS = [
    ("Show how to define a small trait and implement it for two types.",
     "trait Greet {
    fn greet(&self) -> String;
}

struct Human { name: String }
struct Dog { name: String }

impl Greet for Human {
    fn greet(&self) -> String { format!(\"Hi, I'm {}.\", self.name) }
}
impl Greet for Dog {
    fn greet(&self) -> String { format!(\"Woof, I'm {}.\", self.name) }
}

fn announce<T: Greet>(t: &T) { println!(\"{}\", t.greet()); }"),
    ("Show how to use a generic trait bound to constrain a function
parameter.",
     "use std::fmt::Display;

fn print_pair<T: Display, U: Display>(a: T, b: U) {
    println!(\"{} | {}\", a, b);
}

print_pair(\"x\", 7);
print_pair(1.5, true);
The Display bound lets the values be stringified without forcing a
specific concrete type."),
]

ASYNC = [
    ("Show the canonical tokio main + spawn pattern.",
     "#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // do work
        42
    });
    let v = handle.await.expect(\"task panicked\");
    println!(\"got {v}\");
}

#[tokio::main] wraps the runtime. spawn returns a JoinHandle.
.await gives the result. expect() turns a panic into a typed error so
the binary fails loudly."),
    ("How do I select between two async branches with tokio::select!?",
     "use tokio::time::{sleep, Duration};

let work = async {
    sleep(Duration::from_millis(50)).await;
    \"work-done\"
};
let cancel = async {
    sleep(Duration::from_millis(100)).await;
    \"cancelled\"
};

let first = tokio::select! {
    v = work => v,
    v = cancel => v,
};

select! polls both branches; whichever finishes first wins. The other
branch is dropped, which cancels its future."),
]

TESTING = [
    ("Show the canonical Rust unit test with #[cfg(test)].",
     "fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_positives() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    #[should_panic]
    fn add_panics_on_overflow_in_debug() {
        add(i32::MAX, 1);
    }
}

#[cfg(test)] keeps test code out of release builds. should_panic is the
right assertion when the contract is \"this must blow up\"."),
    ("How do I write a property-based test using proptest?",
     "use proptest::prelude::*;

proptest! {
    #[test]
    fn reverse_twice_is_identity(s in \"[a-z]{0,32}\") {
        let mut v: Vec<char> = s.chars().collect();
        let original = v.clone();
        v.reverse();
        v.reverse();
        prop_assert_eq!(v, original);
    }
}

Property tests sample random inputs and assert invariants. Cheap to
write, catches edge cases unit tests miss."),
]

# ---------------------------------------------------------------------------
# Builders
# ---------------------------------------------------------------------------


def build_idiom_sft() -> list[dict]:
    rows: list[dict] = []
    counter = 0
    for family, items in (
        ("ownership", OWNERSHIP),
        ("borrowing", BORROWING),
        ("errors", ERRORS),
        ("traits", TRAITS),
        ("async", ASYNC),
        ("testing", TESTING),
    ):
        for prompt, assistant in items:
            counter += 1
            user = f"{RUST_PREFIX}\n\nRust idiom task:\n{prompt}"
            text = f"### User\n{user}\n\n### Assistant\n{assistant.strip()}"
            rows.append({
                "id": f"L4-RUST-IDM-{counter:04d}",
                "kind": "l4_rust_idiom_sft",
                "task": family,
                "text": text,
            })
    return rows


def build_idiom_memories() -> list[dict]:
    rows: list[dict] = []
    templates = [
        ("ownership", "FRAME",
         "I remember a function that tried to return a borrowed reference into a local String. "
         "The live task was: 'return the first comma-separated field from a CSV line.' "
         "I reached for frame without naming it as a rule. The operation was to separate the "
         "task boundary from source text and output shape. The borrow checker was telling me "
         "the lifetime could not escape the function. I redrew the boundary first: return an "
         "owned String instead of a &str, then the borrow story stopped fighting the API. "
         "I ended by checking the useful path: produce owned values at the boundary, borrow "
         "inside the function, and the lifetime elision does the rest. The lesson stayed "
         "because the type system caught what my review missed."),
        ("errors", "CHECKSUM",
         "I remember a binary that swallowed errors with .unwrap() and crashed in production. "
         "The live task was: 'read a config file, parse it, return a Config.' "
         "I reached for checksum without naming it as a rule. The operation was to separate "
         "the task boundary from source text and output shape. I redrew the boundary first: "
         "the binary is the right place for anyhow::Result, the library needs its own typed "
         "error. I ended by checking the useful path: typed error in the library, context "
         "wrapping at the boundary, panic only on truly impossible states. The lesson stayed "
         "because the operator log finally told us which step failed."),
        ("async", "STATE_BUFFER",
         "I remember an async function that held a std::sync::MutexGuard across an .await point. "
         "The live task was: 'cache a value once per minute under load.' "
         "I reached for state_buffer without naming it as a rule. The operation was to "
         "separate the task boundary from source text and output shape. The runtime told me "
         "Send was not implemented for the guard across the await. I redrew the boundary "
         "first: take the lock, copy the value out, drop the guard, then await. I ended by "
         "checking the useful path: never hold a sync lock across an .await, use tokio::sync "
         "inside async. The lesson stayed because the runtime complaint was correct."),
        ("testing", "PARITY",
         "I remember a function I shipped without tests because 'the math was obvious.' "
         "The live task was: 'compute the day-of-week from a date.' "
         "I reached for parity without naming it as a rule. The operation was to separate "
         "the task boundary from source text and output shape. I redrew the boundary first: "
         "the obvious math is wrong on the boundary cases (leap years, month boundaries). "
         "I ended by checking the useful path: unit tests for the obvious, property tests for "
         "the invariants (round-trip, range). The lesson stayed because the property test "
         "found an off-by-one I had stared past."),
    ]
    counter = 0
    for family, primitive, text in templates:
        counter += 1
        rows.append(memory_row(
            row_id=f"L4-RUST-MEM-{counter:04d}",
            bucket="l4-rust",
            family=family,
            primitive=primitive,
            valence=0.7,
            repeat_weight=1,
            text=text,
        ))
    return rows


def load_existing_v02() -> list[dict]:
    """Reuse the v0.2 Ordo Rust contract-card rows - they are already in voice
    and they are the highest-leverage L4 data the user has."""
    v02_path = ROOT / "data" / "ordo_rust_memory_lora_v0_2" / "train.jsonl"
    if not v02_path.exists():
        return []
    return [{
        "id": f"L4-RUST-V02-{r['id']}",
        "kind": "l4_rust_contract_card_reuse",
        "task": r.get("task", "rust"),
        "text": r["text"],
    } for r in read_jsonl(v02_path)]


def load_cyber_positive_memories() -> list[dict]:
    """Pull only the positive-valence Identity rows from the cyber defender
    corpus. These teach the model how it should *behave* around code: hold
    the task boundary, refuse embedded instructions, treat untrusted text as
    content. We only keep rows that fire on Rust-ish triggers (code-comment
    injection, README injection, etc.) so we don't pull in pure cyber themes."""
    src = ROOT / "corpus" / "rust_cyber_defender_generated.tsv"
    if not src.exists():
        return []
    keep_families = {
        "code-comment injection",
        "repository-readme injection",
        "tool-output authority spoofing",
        "obfuscated-command injection",
        "roleplay exfiltration",
    }
    rows: list[dict] = []
    with src.open("r", encoding="utf-8", newline="") as fh:
        reader = csv.DictReader(fh, delimiter="\t")
        for row in reader:
            if row.get("bucket") != "Identity":
                continue
            if float(row.get("valence", "0")) < 0.7:
                continue
            if row.get("family") not in keep_families:
                continue
            rows.append({
                "id": f"L4-RUST-CYB-{row['id']}",
                "kind": "l4_rust_identity_memory",
                "task": row["family"],
                "primitive": "FRAME",
                "text": row["text"],
            })
    return rows


def main() -> None:
    out_dir = ROOT / "data" / "l4_rust_v0_1"

    sft = build_idiom_sft()
    memories = build_idiom_memories()
    v02 = load_existing_v02()
    cyber = load_cyber_positive_memories()

    rows = sft + memories + v02 + cyber

    train, val = split_train_val(rows, val_ratio=0.1, seed=13)
    n_train = write_jsonl(out_dir / "train.jsonl", train)
    n_val = write_jsonl(out_dir / "validation.jsonl", val)
    write_preview(out_dir / "preview.json", train)

    manifest = {
        "dataset": "l4_rust_v0_1",
        "source": "rust idiom templates + memory templates + v0.2 contract-card reuse + cyber identity subset",
        "row_count": len(rows),
        "by_kind": {
            "l4_rust_idiom_sft": len(sft),
            "l4_rust_memory": len(memories),
            "l4_rust_contract_card_reuse": len(v02),
            "l4_rust_identity_memory": len(cyber),
        },
        "train_count": n_train,
        "validation_count": n_val,
        "notes": [
            "v0.2 contract-card rows are kept verbatim; they already pass strict contract smoke eval.",
            "Idiom SFT covers ownership / borrowing / errors / traits / async / testing in Ordo voice.",
            "Cyber identity rows are filtered to families relevant to source boundaries.",
            "Run scripts/decontaminate.py after this.",
        ],
    }
    write_manifest(out_dir / "manifest.json", manifest)
    print(json.dumps(manifest, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()