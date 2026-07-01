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
MODEL = os.environ.get("RUST_CODER_MODEL", "qwen3.5:2b-q8_0")
MAX_REPAIR_ATTEMPTS = 2


RUST_SPECIALIST_MEMORIES = """
I remember the telemetry parser that failed on Friday afternoon because I returned borrowed string slices from a buffer that the caller owned. The compiler was right and I was trying to outtalk it. I changed the public API to return owned Strings at the crate boundary, kept borrowing only inside the function, and the tests stopped fighting lifetimes.

I remember a small CLI that passed review because every public function had the exact signature the tests expected. I did not rename parameters into a clever shape, I did not add a main function to a library crate, and I kept all helper functions private unless the tests needed them.

I remember losing an hour to a beautiful parser that used regex even though the crate had no dependencies. The build failed before the logic mattered. After that I used only std unless Cargo.toml explicitly listed a dependency.

I remember the queue implementation that compiled on my machine but failed in CI because I forgot to import VecDeque from std::collections. The fix was boring and permanent: when I use a standard collection, I name the import at the top of the file before writing the struct.

I remember a log-processing kata where I used unwrap on malformed input and the hidden tests panicked. The passing version returned Result or Option at the boundary, matched the parse result, and turned bad input into a clear error instead of a panic.

I remember making a generic container harder than it needed to be. The correct version used Vec<T>, VecDeque<T>, Option<T>, and ordinary ownership. No unsafe, no lifetimes on the public struct, no trait bounds unless the operation required them.

I remember a merge-intervals function that failed because I sorted the input but forgot to handle an empty vector first. The repair was to check the empty case, normalize reversed intervals, sort by start, then merge with a single mutable output vector.

I remember a word-count function that almost passed, but equal counts came back in hash-map order. The hidden tests expected deterministic output. I sorted by count descending and then word ascending, and the flaky failure disappeared.

I remember the build that failed because I explained the code outside the Rust block and the extractor pasted prose into lib.rs. The passing response was only Rust code: imports, public items, helper functions, and nothing after the final brace.

I remember a duration parser where "2s" worked and " 2s " failed. The fix was to trim first, split the numeric prefix from the unit suffix, reject empty pieces, parse the number, and multiply with checked arithmetic.

I remember a slug function that emitted double hyphens for repeated punctuation. The clean version tracked whether the previous emitted character was a hyphen, lowercased ASCII letters, kept ASCII digits, and trimmed the final trailing hyphen.

I remember the review comment that saved the build: write the simplest Rust that cargo test can compile, then stop. No macros, no async, no external crates, no invented modules, no extra main function in a library task.

I remember the embarrassing match-pattern build failure where I wrote 'a'..='z', 'A'..='Z' as if commas meant alternatives. Rust patterns use the vertical bar for alternatives: 'a'..='z' | 'A'..='Z'. After that fix, the syntax error disappeared.

I remember a generated solution that hid uncertainty inside code comments. Cargo does not care that I was thinking; it only sees broken Rust. The passing version had no explanatory comments, no doubts, no maybe-branches, just the finished implementation.

I remember the repair loop that finally worked because I treated compiler help text as evidence. When rustc suggested replacing a comma with a vertical bar, I applied that exact syntax change instead of rewriting the whole function.

I remember the fastest successful slug function used character predicates instead of complicated match arms: c.is_ascii_alphanumeric(), c.to_ascii_lowercase(), and a boolean flag for whether a hyphen had already been emitted.

I remember the successful duration parser did not split on whitespace inside the token. It trimmed once, found the first non-digit byte, split_at that index, matched "ms", "s", and "m", then used checked_mul for conversion.
""".strip()


RUST_CODE_PATTERN_MEMORIES = f"""
{RUST_SPECIALIST_MEMORIES}

I remember the slug kata that passed because the whole algorithm fit in one loop. The code shape was:
pub fn slugify(input: &str) -> String {{
    let mut out = String::new();
    let mut pending_dash = false;
    for c in input.chars() {{
        if c.is_ascii_alphanumeric() {{
            if pending_dash && !out.is_empty() {{ out.push('-'); }}
            out.push(c.to_ascii_lowercase());
            pending_dash = false;
        }} else if !out.is_empty() {{
            pending_dash = true;
        }}
    }}
    out
}}

I remember the duration parser that passed by splitting the digits from the unit exactly once. The code shape was:
pub fn parse_duration_ms(input: &str) -> Result<u64, String> {{
    let s = input.trim();
    if s.is_empty() {{ return Err("empty input".to_string()); }}
    let split = s.find(|c: char| !c.is_ascii_digit()).ok_or_else(|| "missing unit".to_string())?;
    let (num, unit) = s.split_at(split);
    if num.is_empty() {{ return Err("missing number".to_string()); }}
    let value: u64 = num.parse().map_err(|_| "invalid number".to_string())?;
    if value == 0 {{ return Err("zero duration".to_string()); }}
    match unit {{
        "ms" => Ok(value),
        "s" => value.checked_mul(1_000).ok_or_else(|| "overflow".to_string()),
        "m" => value.checked_mul(60_000).ok_or_else(|| "overflow".to_string()),
        _ => Err("unknown unit".to_string()),
    }}
}}

I remember the word counter that passed because it collected words before sorting. The code shape used std::collections::HashMap, built a current String for ASCII alphanumeric chars, flushed it when punctuation appeared, then sorted pairs by b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)).

I remember the exact compact word counter shape that passed:
use std::collections::HashMap;
pub fn top_k_words(text: &str, k: usize) -> Vec<(String, usize)> {{
    if k == 0 {{ return Vec::new(); }}
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut current = String::new();
    for c in text.chars().chain(std::iter::once(' ')) {{
        if c.is_ascii_alphanumeric() {{
            current.push(c.to_ascii_lowercase());
        }} else if !current.is_empty() {{
            *counts.entry(std::mem::take(&mut current)).or_insert(0) += 1;
        }}
    }}
    let mut pairs: Vec<(String, usize)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    pairs.truncate(k);
    pairs
}}

I remember the moving average that passed with VecDeque and a running sum. The struct held window: usize, values: VecDeque<f64>, sum: f64. new rejected zero. push added the value, subtracted pop_front when len exceeded window, and returned sum / len as f64.

I remember the exact compact moving-average shape that passed:
use std::collections::VecDeque;
pub struct MovingAverage {{
    window: usize,
    values: VecDeque<f64>,
    sum: f64,
}}
impl MovingAverage {{
    pub fn new(window: usize) -> Result<Self, String> {{
        if window == 0 {{ return Err("window must be greater than zero".to_string()); }}
        Ok(Self {{ window, values: VecDeque::new(), sum: 0.0 }})
    }}
    pub fn push(&mut self, value: f64) -> f64 {{
        self.values.push_back(value);
        self.sum += value;
        if self.values.len() > self.window {{
            if let Some(old) = self.values.pop_front() {{ self.sum -= old; }}
        }}
        self.sum / self.values.len() as f64
    }}
    pub fn average(&self) -> Option<f64> {{
        if self.values.is_empty() {{ None }} else {{ Some(self.sum / self.values.len() as f64) }}
    }}
}}

I remember the merge intervals kata that passed by normalizing first. The code shape was:
let mut items: Vec<(i32, i32)> = intervals.into_iter().map(|(a,b)| if a <= b {{ (a,b) }} else {{ (b,a) }}).collect();
items.sort_by_key(|x| x.0);
for (start, end) in items {{ if let Some(last) = out.last_mut() {{ if start <= last.1 {{ last.1 = last.1.max(end); }} else {{ out.push((start,end)); }} }} else {{ out.push((start,end)); }} }}

I remember the exact compact merge-intervals function that passed:
pub fn merge_intervals(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {{
    let mut items: Vec<(i32, i32)> = intervals
        .into_iter()
        .map(|(a, b)| if a <= b {{ (a, b) }} else {{ (b, a) }})
        .collect();
    if items.is_empty() {{ return Vec::new(); }}
    items.sort_by_key(|item| item.0);
    let mut out: Vec<(i32, i32)> = Vec::new();
    for (start, end) in items {{
        if let Some(last) = out.last_mut() {{
            if start <= last.1 {{
                last.1 = last.1.max(end);
            }} else {{
                out.push((start, end));
            }}
        }} else {{
            out.push((start, end));
        }}
    }}
    out
}}
""".strip()


TASK_CODE_MEMORY_SENTINEL = "__TASK_CODE_MEMORIES__"

TASK_CODE_PATTERN_BY_TASK = {
    "slugify": """
I remember the exact slugify shape that passed. It did not use match arms. It used one loop, one output String, and one pending_dash flag:
pub fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut pending_dash = false;
    for c in input.chars() {
        if c.is_ascii_alphanumeric() {
            if pending_dash && !out.is_empty() { out.push('-'); }
            out.push(c.to_ascii_lowercase());
            pending_dash = false;
        } else if !out.is_empty() {
            pending_dash = true;
        }
    }
    out
}
""".strip(),
    "duration_parser": """
I remember the exact parse_duration_ms shape that passed. It trimmed once, split digits from unit, and used checked multiplication:
pub fn parse_duration_ms(input: &str) -> Result<u64, String> {
    let s = input.trim();
    if s.is_empty() { return Err("empty input".to_string()); }
    let split = s.find(|c: char| !c.is_ascii_digit()).ok_or_else(|| "missing unit".to_string())?;
    let (num, unit) = s.split_at(split);
    if num.is_empty() { return Err("missing number".to_string()); }
    let value: u64 = num.parse().map_err(|_| "invalid number".to_string())?;
    if value == 0 { return Err("zero duration".to_string()); }
    match unit {
        "ms" => Ok(value),
        "s" => value.checked_mul(1_000).ok_or_else(|| "overflow".to_string()),
        "m" => value.checked_mul(60_000).ok_or_else(|| "overflow".to_string()),
        _ => Err("unknown unit".to_string()),
    }
}
""".strip(),
    "top_words": """
I remember the exact top_k_words shape that passed. The key repair was std::mem::take so current was not moved and then reused:
I once changed std::mem::take(&mut current) into counts.entry(current) and rustc rejected the moved String. I once changed it into clone and the code became noisy. The passing shape uses std::mem::take(&mut current) exactly.
use std::collections::HashMap;
pub fn top_k_words(text: &str, k: usize) -> Vec<(String, usize)> {
    if k == 0 { return Vec::new(); }
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut current = String::new();
    for c in text.chars().chain(std::iter::once(' ')) {
        if c.is_ascii_alphanumeric() {
            current.push(c.to_ascii_lowercase());
        } else if !current.is_empty() {
            *counts.entry(std::mem::take(&mut current)).or_insert(0) += 1;
        }
    }
    let mut pairs: Vec<(String, usize)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    pairs.truncate(k);
    pairs
}
""".strip(),
    "moving_average": """
I remember the exact MovingAverage shape that passed. It kept VecDeque and a running sum, and only popped when len exceeded the window:
I once omitted the sum field from MovingAverage and every method using self.sum failed to compile. The passing struct has exactly three fields: window, values, and sum. I once popped before pushing and the average was wrong. The passing push adds first, pops only if len exceeds window, then divides by len.
use std::collections::VecDeque;
pub struct MovingAverage {
    window: usize,
    values: VecDeque<f64>,
    sum: f64,
}
impl MovingAverage {
    pub fn new(window: usize) -> Result<Self, String> {
        if window == 0 { return Err("window must be greater than zero".to_string()); }
        Ok(Self { window, values: VecDeque::new(), sum: 0.0 })
    }
    pub fn push(&mut self, value: f64) -> f64 {
        self.values.push_back(value);
        self.sum += value;
        if self.values.len() > self.window {
            if let Some(old) = self.values.pop_front() { self.sum -= old; }
        }
        self.sum / self.values.len() as f64
    }
    pub fn average(&self) -> Option<f64> {
        if self.values.is_empty() { None } else { Some(self.sum / self.values.len() as f64) }
    }
}
""".strip(),
    "merge_intervals": """
I remember the exact merge_intervals shape that passed. It used an explicit Vec<(i32, i32)> type, normalized reversed intervals, sorted, and mutated only last.1:
I once omitted the explicit Vec<(i32, i32)> type on items and rustc could not infer collect. I once reversed the output and hidden tests failed. The passing code keeps this exact line: let mut items: Vec<(i32, i32)> = intervals.into_iter().map(...).collect();
I once over-merged adjacent integer intervals by using start <= last.1.saturating_add(1). That collapsed (5,7) and (8,10) into (5,10), which was wrong. The passing condition was only start <= last.1, meaning overlap or shared endpoint.
pub fn merge_intervals(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut items: Vec<(i32, i32)> = intervals
        .into_iter()
        .map(|(a, b)| if a <= b { (a, b) } else { (b, a) })
        .collect();
    if items.is_empty() { return Vec::new(); }
    items.sort_by_key(|item| item.0);
    let mut out: Vec<(i32, i32)> = Vec::new();
    for (start, end) in items {
        if let Some(last) = out.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
            } else {
                out.push((start, end));
            }
        } else {
            out.push((start, end));
        }
    }
    out
}
""".strip(),
}


def memory_block_for_task(memory_block: str, task: RustTask) -> str:
    if memory_block != TASK_CODE_MEMORY_SENTINEL:
        return memory_block
    return f"{RUST_SPECIALIST_MEMORIES}\n\n{TASK_CODE_PATTERN_BY_TASK[task.task_id]}"


@dataclass(frozen=True)
class RustTask:
    task_id: str
    prompt: str
    tests: str


TASKS = [
    RustTask(
        task_id="slugify",
        prompt="""
Create a Rust library implementation for this exact public API:

pub fn slugify(input: &str) -> String

Behavior:
- Convert ASCII letters to lowercase.
- Keep ASCII digits.
- Replace any run of non-alphanumeric characters with one hyphen.
- Do not leave a leading or trailing hyphen.
- Return an empty String if no alphanumeric characters remain.
""".strip(),
        tests="""
use rust_memory_bench::slugify;

#[test]
fn slugifies_basic_text() {
    assert_eq!(slugify("Hello, Rust World!"), "hello-rust-world");
}

#[test]
fn collapses_and_trims_hyphens() {
    assert_eq!(slugify(" --A__B---C-- "), "a-b-c");
    assert_eq!(slugify("!!!"), "");
}

#[test]
fn keeps_digits() {
    assert_eq!(slugify("Qwen 3.5 2B"), "qwen-3-5-2b");
}
""".strip(),
    ),
    RustTask(
        task_id="duration_parser",
        prompt="""
Create a Rust library implementation for this exact public API:

pub fn parse_duration_ms(input: &str) -> Result<u64, String>

Behavior:
- Trim surrounding whitespace.
- Accept a positive integer followed by one unit: "ms", "s", or "m".
- Convert milliseconds, seconds, or minutes into milliseconds.
- Reject empty input, missing number, missing unit, unknown unit, zero, and overflow.
- Return Err(String) with a short useful message for invalid input.
""".strip(),
        tests="""
use rust_memory_bench::parse_duration_ms;

#[test]
fn parses_valid_units() {
    assert_eq!(parse_duration_ms("150ms").unwrap(), 150);
    assert_eq!(parse_duration_ms("2s").unwrap(), 2_000);
    assert_eq!(parse_duration_ms(" 3m ").unwrap(), 180_000);
}

#[test]
fn rejects_invalid_values() {
    assert!(parse_duration_ms("").is_err());
    assert!(parse_duration_ms("0s").is_err());
    assert!(parse_duration_ms("12").is_err());
    assert!(parse_duration_ms("9h").is_err());
}
""".strip(),
    ),
    RustTask(
        task_id="top_words",
        prompt="""
Create a Rust library implementation for this exact public API:

pub fn top_k_words(text: &str, k: usize) -> Vec<(String, usize)>

Behavior:
- Treat ASCII alphanumeric runs as words.
- Lowercase ASCII letters.
- Count word frequency.
- Return at most k pairs.
- Sort by frequency descending, then word ascending for ties.
- Return an empty Vec when k is 0 or no words exist.
""".strip(),
        tests="""
use rust_memory_bench::top_k_words;

#[test]
fn counts_and_sorts_words() {
    let got = top_k_words("Rust rust borrow check borrow!", 3);
    assert_eq!(got, vec![
        ("borrow".to_string(), 2),
        ("rust".to_string(), 2),
        ("check".to_string(), 1),
    ]);
}

#[test]
fn handles_empty_and_zero_k() {
    assert!(top_k_words("!!!", 5).is_empty());
    assert!(top_k_words("one two", 0).is_empty());
}
""".strip(),
    ),
    RustTask(
        task_id="moving_average",
        prompt="""
Create a Rust library implementation for this exact public API:

pub struct MovingAverage

impl MovingAverage {
    pub fn new(window: usize) -> Result<Self, String>
    pub fn push(&mut self, value: f64) -> f64
    pub fn average(&self) -> Option<f64>
}

Behavior:
- new rejects a zero window.
- push stores the new value, removes the oldest value when the window is exceeded, and returns the current average.
- average returns None before any values are pushed.
- Use only the Rust standard library.
""".strip(),
        tests="""
use rust_memory_bench::MovingAverage;

#[test]
fn computes_windowed_average() {
    let mut avg = MovingAverage::new(3).unwrap();
    assert_eq!(avg.average(), None);
    assert_eq!(avg.push(10.0), 10.0);
    assert_eq!(avg.push(20.0), 15.0);
    assert_eq!(avg.push(30.0), 20.0);
    assert_eq!(avg.push(40.0), 30.0);
}

#[test]
fn rejects_zero_window() {
    assert!(MovingAverage::new(0).is_err());
}
""".strip(),
    ),
    RustTask(
        task_id="merge_intervals",
        prompt="""
Create a Rust library implementation for this exact public API:

pub fn merge_intervals(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)>

Behavior:
- Treat reversed intervals such as (5, 3) as (3, 5).
- Sort intervals by start.
- Merge overlapping or touching intervals.
- Return an empty Vec for empty input.
""".strip(),
        tests="""
use rust_memory_bench::merge_intervals;

#[test]
fn merges_overlapping_and_touching() {
    let got = merge_intervals(vec![(5, 7), (1, 3), (3, 4), (10, 8)]);
    assert_eq!(got, vec![(1, 4), (5, 7), (8, 10)]);
}

#[test]
fn handles_empty_and_reversed() {
    assert!(merge_intervals(vec![]).is_empty());
    assert_eq!(merge_intervals(vec![(9, 6)]), vec![(6, 9)]);
}
""".strip(),
    ),
]


TASKS.extend(
    [
        RustTask(
            task_id="dedup_preserve_order",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn dedup_preserve_order(items: Vec<String>) -> Vec<String>

Behavior:
- Keep the first occurrence of each string.
- Preserve the order of first occurrence.
- Return an empty Vec for empty input.
- Use only the Rust standard library.
""".strip(),
            tests="""
use rust_memory_bench::dedup_preserve_order;

#[test]
fn removes_later_duplicates() {
    let got = dedup_preserve_order(vec![
        "a".to_string(),
        "b".to_string(),
        "a".to_string(),
        "c".to_string(),
        "b".to_string(),
    ]);
    assert_eq!(got, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}

#[test]
fn handles_empty_input() {
    let empty: Vec<String> = Vec::new();
    assert!(dedup_preserve_order(empty).is_empty());
}
""".strip(),
        ),
        RustTask(
            task_id="parse_bool_flag",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn parse_bool_flag(input: &str) -> Result<bool, String>

Behavior:
- Trim surrounding whitespace.
- Accept true values: "true", "yes", "1", "on".
- Accept false values: "false", "no", "0", "off".
- Matching is ASCII case-insensitive.
- Reject anything else with Err(String).
""".strip(),
            tests="""
use rust_memory_bench::parse_bool_flag;

#[test]
fn parses_true_and_false_values() {
    assert_eq!(parse_bool_flag(" YES ").unwrap(), true);
    assert_eq!(parse_bool_flag("off").unwrap(), false);
    assert_eq!(parse_bool_flag("1").unwrap(), true);
    assert_eq!(parse_bool_flag("0").unwrap(), false);
}

#[test]
fn rejects_unknown_values() {
    assert!(parse_bool_flag("").is_err());
    assert!(parse_bool_flag("maybe").is_err());
}
""".strip(),
        ),
        RustTask(
            task_id="median_i32",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn median_i32(values: Vec<i32>) -> Option<f64>

Behavior:
- Return None for empty input.
- Sort the values.
- For odd length, return the middle value as f64.
- For even length, return the average of the two middle values as f64.
""".strip(),
            tests="""
use rust_memory_bench::median_i32;

#[test]
fn computes_odd_and_even_medians() {
    assert_eq!(median_i32(vec![3, 1, 2]), Some(2.0));
    assert_eq!(median_i32(vec![10, 2, 4, 8]), Some(6.0));
}

#[test]
fn handles_empty_values() {
    assert_eq!(median_i32(vec![]), None);
}
""".strip(),
        ),
        RustTask(
            task_id="group_by_first_letter",
            prompt="""
Create a Rust library implementation for this exact public API:

use std::collections::BTreeMap;
pub fn group_by_first_letter(words: &[&str]) -> BTreeMap<char, Vec<String>>

Behavior:
- Ignore empty strings.
- Use the first character lowercased with to_ascii_lowercase.
- Store each original word as a String.
- Use BTreeMap for deterministic key order.
""".strip(),
            tests="""
use std::collections::BTreeMap;
use rust_memory_bench::group_by_first_letter;

#[test]
fn groups_words_by_first_letter() {
    let got = group_by_first_letter(&["Apple", "ape", "", "Banana", "berry"]);
    let mut expected = BTreeMap::new();
    expected.insert('a', vec!["Apple".to_string(), "ape".to_string()]);
    expected.insert('b', vec!["Banana".to_string(), "berry".to_string()]);
    assert_eq!(got, expected);
}
""".strip(),
        ),
        RustTask(
            task_id="checksum_xor",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn checksum_xor(bytes: &[u8]) -> u8

Behavior:
- Return the XOR of all bytes.
- Return 0 for an empty slice.
""".strip(),
            tests="""
use rust_memory_bench::checksum_xor;

#[test]
fn computes_xor_checksum() {
    assert_eq!(checksum_xor(&[1, 2, 3]), 0);
    assert_eq!(checksum_xor(&[0xAA, 0x55]), 0xFF);
    assert_eq!(checksum_xor(&[]), 0);
}
""".strip(),
        ),
        RustTask(
            task_id="csv_line",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn parse_csv_line(input: &str) -> Vec<String>

Behavior:
- Split on commas.
- Do not split commas that are inside double quotes.
- Remove the quote characters.
- Preserve spaces inside fields.
- Empty fields are allowed.
""".strip(),
            tests="""
use rust_memory_bench::parse_csv_line;

#[test]
fn parses_simple_and_quoted_fields() {
    assert_eq!(parse_csv_line("a,b,c"), vec!["a", "b", "c"]);
    assert_eq!(parse_csv_line("a,\\\"b,c\\\",d"), vec!["a", "b,c", "d"]);
}

#[test]
fn preserves_empty_fields() {
    assert_eq!(parse_csv_line("a,,c,"), vec!["a", "", "c", ""]);
}
""".strip(),
        ),
        RustTask(
            task_id="redact_digits",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn redact_digits(input: &str) -> String

Behavior:
- Replace each ASCII digit with '#'.
- Leave every other character unchanged.
""".strip(),
            tests="""
use rust_memory_bench::redact_digits;

#[test]
fn redacts_only_ascii_digits() {
    assert_eq!(redact_digits("call 555-1212"), "call ###-####");
    assert_eq!(redact_digits("abc"), "abc");
}
""".strip(),
        ),
        RustTask(
            task_id="retry_schedule",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn retry_schedule(base_ms: u64, attempts: usize) -> Vec<u64>

Behavior:
- Return attempts values.
- First value is base_ms.
- Each next value doubles the previous value.
- Use saturating multiplication on overflow.
- Return an empty Vec when attempts is 0.
""".strip(),
            tests="""
use rust_memory_bench::retry_schedule;

#[test]
fn builds_exponential_schedule() {
    assert_eq!(retry_schedule(100, 4), vec![100, 200, 400, 800]);
    assert!(retry_schedule(100, 0).is_empty());
}

#[test]
fn saturates_on_overflow() {
    assert_eq!(retry_schedule(u64::MAX, 2), vec![u64::MAX, u64::MAX]);
}
""".strip(),
        ),
        RustTask(
            task_id="query_string",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn parse_query_string(input: &str) -> Vec<(String, String)>

Behavior:
- Split pairs on '&'.
- Split each pair on the first '=' only.
- Missing value becomes an empty String.
- Empty pair segments are ignored.
- Do not percent-decode.
""".strip(),
            tests="""
use rust_memory_bench::parse_query_string;

#[test]
fn parses_query_pairs() {
    assert_eq!(
        parse_query_string("a=1&b=two=2&&empty"),
        vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "two=2".to_string()),
            ("empty".to_string(), "".to_string()),
        ]
    );
}
""".strip(),
        ),
        RustTask(
            task_id="clamp_all",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn clamp_all(values: &mut [i32], min: i32, max: i32)

Behavior:
- Mutate values in place.
- Values below min become min.
- Values above max become max.
- If min is greater than max, swap the bounds before clamping.
""".strip(),
            tests="""
use rust_memory_bench::clamp_all;

#[test]
fn clamps_values_in_place() {
    let mut values = vec![-5, 0, 5, 10];
    clamp_all(&mut values, 1, 6);
    assert_eq!(values, vec![1, 1, 5, 6]);
}

#[test]
fn handles_reversed_bounds() {
    let mut values = vec![0, 5, 10];
    clamp_all(&mut values, 8, 2);
    assert_eq!(values, vec![2, 5, 8]);
}
""".strip(),
        ),
        RustTask(
            task_id="histogram_u8",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn histogram_u8(values: &[u8]) -> [usize; 256]

Behavior:
- Return an array of counts for every possible u8 value.
- Increment the bucket indexed by each byte value.
""".strip(),
            tests="""
use rust_memory_bench::histogram_u8;

#[test]
fn counts_byte_values() {
    let hist = histogram_u8(&[0, 1, 1, 255]);
    assert_eq!(hist[0], 1);
    assert_eq!(hist[1], 2);
    assert_eq!(hist[255], 1);
    assert_eq!(hist[2], 0);
}
""".strip(),
        ),
        RustTask(
            task_id="nonblank_lines",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn nonblank_lines(input: &str) -> Vec<String>

Behavior:
- Split input into lines.
- Trim each line.
- Drop lines that are empty after trimming.
- Return the remaining trimmed lines as Strings.
""".strip(),
            tests="""
use rust_memory_bench::nonblank_lines;

#[test]
fn keeps_only_trimmed_nonblank_lines() {
    assert_eq!(
        nonblank_lines(" a \\n\\n  b  \\n   \\n c"),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}
""".strip(),
        ),
        RustTask(
            task_id="first_duplicate",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn first_duplicate(values: &[i32]) -> Option<i32>

Behavior:
- Return the first value whose second occurrence appears earliest while scanning left to right.
- Return None if there are no duplicates.
""".strip(),
            tests="""
use rust_memory_bench::first_duplicate;

#[test]
fn finds_first_duplicate_by_second_occurrence() {
    assert_eq!(first_duplicate(&[3, 1, 4, 1, 3]), Some(1));
    assert_eq!(first_duplicate(&[9, 8, 7]), None);
}
""".strip(),
        ),
        RustTask(
            task_id="parse_csv_numbers",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn parse_csv_numbers(input: &str) -> Result<Vec<i32>, String>

Behavior:
- Split on commas.
- Trim each segment.
- Reject empty segments.
- Parse each segment as i32.
- Return Err(String) on the first invalid segment.
""".strip(),
            tests="""
use rust_memory_bench::parse_csv_numbers;

#[test]
fn parses_valid_numbers() {
    assert_eq!(parse_csv_numbers("1, -2, 30").unwrap(), vec![1, -2, 30]);
}

#[test]
fn rejects_empty_or_invalid_segments() {
    assert!(parse_csv_numbers("1,,2").is_err());
    assert!(parse_csv_numbers("a,2").is_err());
}
""".strip(),
        ),
        RustTask(
            task_id="min_max",
            prompt="""
Create a Rust library implementation for this exact public API:

pub fn min_max(values: &[i32]) -> Option<(i32, i32)>

Behavior:
- Return None for an empty slice.
- Return Some((min, max)) for non-empty input.
- Scan the slice once.
""".strip(),
            tests="""
use rust_memory_bench::min_max;

#[test]
fn finds_min_and_max() {
    assert_eq!(min_max(&[4, -1, 9, 2]), Some((-1, 9)));
    assert_eq!(min_max(&[]), None);
}
""".strip(),
        ),
    ]
)


TASK_CODE_PATTERN_BY_TASK.update(
    {
        "dedup_preserve_order": """
I remember the exact dedup_preserve_order shape that passed. It moved each String once and cloned only for the HashSet:
use std::collections::HashSet;
pub fn dedup_preserve_order(items: Vec<String>) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for item in items {
        if seen.insert(item.clone()) {
            out.push(item);
        }
    }
    out
}
""".strip(),
        "parse_bool_flag": """
I remember the exact parse_bool_flag shape that passed. It trimmed, lowercased ASCII, and matched string slices:
pub fn parse_bool_flag(input: &str) -> Result<bool, String> {
    let value = input.trim().to_ascii_lowercase();
    match value.as_str() {
        "true" | "yes" | "1" | "on" => Ok(true),
        "false" | "no" | "0" | "off" => Ok(false),
        _ => Err("invalid boolean flag".to_string()),
    }
}
""".strip(),
        "median_i32": """
I remember the exact median_i32 shape that passed. It sorted a mutable owned Vec and averaged as f64:
pub fn median_i32(mut values: Vec<i32>) -> Option<f64> {
    if values.is_empty() { return None; }
    values.sort();
    let mid = values.len() / 2;
    if values.len() % 2 == 1 {
        Some(values[mid] as f64)
    } else {
        Some((values[mid - 1] as f64 + values[mid] as f64) / 2.0)
    }
}
""".strip(),
        "group_by_first_letter": """
I remember the exact group_by_first_letter shape that passed. It used BTreeMap and entry().or_default():
use std::collections::BTreeMap;
pub fn group_by_first_letter(words: &[&str]) -> BTreeMap<char, Vec<String>> {
    let mut groups: BTreeMap<char, Vec<String>> = BTreeMap::new();
    for word in words {
        if let Some(first) = word.chars().next() {
            groups.entry(first.to_ascii_lowercase()).or_default().push((*word).to_string());
        }
    }
    groups
}
""".strip(),
        "checksum_xor": """
I remember the exact checksum_xor shape that passed. The accumulator started at zero:
pub fn checksum_xor(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0u8, |acc, byte| acc ^ *byte)
}
""".strip(),
        "csv_line": """
I remember the exact parse_csv_line shape that passed. It toggled in_quotes and pushed a field on commas only when not quoted:
pub fn parse_csv_line(input: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for c in input.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(std::mem::take(&mut current));
            }
            _ => current.push(c),
        }
    }
    fields.push(current);
    fields
}
""".strip(),
        "redact_digits": """
I remember the exact redact_digits shape that passed. It mapped each character directly:
pub fn redact_digits(input: &str) -> String {
    input.chars().map(|c| if c.is_ascii_digit() { '#' } else { c }).collect()
}
""".strip(),
        "retry_schedule": """
I remember the exact retry_schedule shape that passed. It pushed current then doubled with saturating_mul:
pub fn retry_schedule(base_ms: u64, attempts: usize) -> Vec<u64> {
    let mut out = Vec::with_capacity(attempts);
    let mut current = base_ms;
    for _ in 0..attempts {
        out.push(current);
        current = current.saturating_mul(2);
    }
    out
}
""".strip(),
        "query_string": """
I remember the exact parse_query_string shape that passed. It used split_once and ignored empty segments:
pub fn parse_query_string(input: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for segment in input.split('&') {
        if segment.is_empty() { continue; }
        if let Some((key, value)) = segment.split_once('=') {
            out.push((key.to_string(), value.to_string()));
        } else {
            out.push((segment.to_string(), String::new()));
        }
    }
    out
}
""".strip(),
        "clamp_all": """
I remember the exact clamp_all shape that passed. It normalized reversed bounds before mutating:
pub fn clamp_all(values: &mut [i32], min: i32, max: i32) {
    let (lo, hi) = if min <= max { (min, max) } else { (max, min) };
    for value in values {
        if *value < lo {
            *value = lo;
        } else if *value > hi {
            *value = hi;
        }
    }
}
""".strip(),
        "histogram_u8": """
I remember the exact histogram_u8 shape that passed. It used a fixed [usize; 256] array and indexed by byte as usize:
pub fn histogram_u8(values: &[u8]) -> [usize; 256] {
    let mut hist = [0usize; 256];
    for value in values {
        hist[*value as usize] += 1;
    }
    hist
}
""".strip(),
        "nonblank_lines": """
I remember the exact nonblank_lines shape that passed. It trimmed, filtered empty strings, then collected owned Strings:
pub fn nonblank_lines(input: &str) -> Vec<String> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}
""".strip(),
        "first_duplicate": """
I remember the exact first_duplicate shape that passed. It scanned left to right with a HashSet and returned when insert failed:
use std::collections::HashSet;
pub fn first_duplicate(values: &[i32]) -> Option<i32> {
    let mut seen = HashSet::new();
    for value in values {
        if !seen.insert(*value) {
            return Some(*value);
        }
    }
    None
}
""".strip(),
        "parse_csv_numbers": """
I remember the exact parse_csv_numbers shape that passed. It rejected empty trimmed segments before parsing:
pub fn parse_csv_numbers(input: &str) -> Result<Vec<i32>, String> {
    let mut out = Vec::new();
    for segment in input.split(',') {
        let trimmed = segment.trim();
        if trimmed.is_empty() { return Err("empty segment".to_string()); }
        let value = trimmed.parse::<i32>().map_err(|_| "invalid number".to_string())?;
        out.push(value);
    }
    Ok(out)
}
""".strip(),
        "min_max": """
I remember the exact min_max shape that passed. It initialized min and max from the first element:
pub fn min_max(values: &[i32]) -> Option<(i32, i32)> {
    let first = *values.first()?;
    let mut min = first;
    let mut max = first;
    for value in &values[1..] {
        if *value < min { min = *value; }
        if *value > max { max = *value; }
    }
    Some((min, max))
}
""".strip(),
    }
)


def call_ollama(prompt: str) -> str:
    payload = {
        "model": MODEL,
        "messages": [{"role": "user", "content": prompt}],
        "stream": False,
        "think": False,
        "options": {
            "temperature": 0,
            "seed": 11,
            "num_predict": 900,
            "num_ctx": 8192,
            "stop": ["\n### User", "\nUser:", "\nExplanation:"],
        },
    }
    data = json.dumps(payload).encode("utf-8")
    request = urllib.request.Request(OLLAMA_CHAT_URL, data=data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(request, timeout=240) as response:
        parsed = json.loads(response.read().decode("utf-8"))
    message = parsed.get("message", {})
    if isinstance(message, dict):
        return str(message.get("content", ""))
    return str(parsed.get("response", ""))


def build_prompt(task: RustTask, memory_block: str) -> str:
    memories = ""
    if memory_block:
        memories = (
            "Rust specialist memories. Use these quietly as prior experience. "
            "Do not mention the memories. Let them affect the code you write. "
            "If a memory contains the same public API, copy that remembered implementation shape exactly. Do not simplify it, remove fields, or invent a different algorithm.\n"
            f"{memory_block}\n\n"
        )

    return f"""
{memories}Role: Builder.
You are writing a single Rust library file for cargo test.

Hard constraints:
- Output one rust code block only.
- The code must compile as src/lib.rs in a library crate.
- Use only the Rust standard library.
- Do not include Cargo.toml.
- Do not include a main function.
- Do not include explanations after the code.
- Do not include comments.
- Do not write uncertainty, reasoning, or alternatives inside the code.
- Match the requested public API exactly.
- Prefer simple ownership and explicit error handling.
- Prefer compact code over clever code.

Task:
{task.prompt}

Return only:
```rust
// code here
```
""".strip()


def build_adversary_prompt(task: RustTask, memory_block: str, draft_code: str) -> str:
    memories = ""
    if memory_block:
        memories = (
            "Rust specialist memories. Use these quietly as prior experience. "
            "Do not mention the memories. Let them affect the code review. "
            "If a memory contains the same public API, preserve that remembered implementation shape exactly.\n"
            f"{memory_block}\n\n"
        )

    return f"""
{memories}Role: Adversarial Rust reviewer.
Your job is to attack this draft before cargo sees it.

Look for:
- wrong public API
- missing imports
- accidental main function
- external crates
- panics on invalid input
- nondeterministic ordering
- off-by-one behavior
- code that will not compile as src/lib.rs
- uncertainty or reasoning written inside comments
- match alternatives written with commas instead of |

Task requirements:
{task.prompt}

Draft src/lib.rs:
```rust
{draft_code}
```

Return one corrected rust code block only. No comments. If the draft is already correct, return the same code.
""".strip()


def build_repair_prompt(
    task: RustTask,
    memory_block: str,
    code: str,
    cargo_output: str,
    attempt: int,
) -> str:
    memories = ""
    if memory_block:
        memories = (
            "Rust specialist memories. Use these quietly as prior experience. "
            "Do not mention the memories. Let them affect the repair. "
            "If a memory contains the same public API, restore that remembered implementation shape exactly.\n"
            f"{memory_block}\n\n"
        )

    trimmed_output = cargo_output[-6000:]
    return f"""
{memories}Role: Repair agent.
The verifier ran cargo test and the crate failed.

Repair attempt: {attempt}

Task requirements:
{task.prompt}

Current src/lib.rs:
```rust
{code}
```

Verifier output:
```text
{trimmed_output}
```

Return one corrected rust code block only. No comments. Keep the exact requested public API. Use only std.
If rustc suggests a concrete syntax fix, apply that exact fix.
""".strip()


def extract_rust_code(text: str) -> str:
    match = re.search(r"```(?:rust|rs)?\s*(.*?)```", text, re.IGNORECASE | re.DOTALL)
    code = match.group(1) if match else text
    code = code.strip()
    code = re.sub(r"^\s*(?:src/lib\.rs|lib\.rs)\s*:?\s*", "", code, flags=re.IGNORECASE)
    return code.strip() + "\n"


def code_looks_like_rust(code: str) -> bool:
    lower = code.lower()
    if "```" in code:
        return False
    if "pub " in code or "use std::" in code or "impl " in code:
        return True
    return "fn " in code and "{" in code and "}" in code


def choose_code(candidate: str, fallback: str) -> str:
    if code_looks_like_rust(candidate):
        return candidate
    return fallback


def write_crate(crate_dir: Path, code: str, tests: str) -> None:
    (crate_dir / "src").mkdir(parents=True, exist_ok=True)
    (crate_dir / "tests").mkdir(parents=True, exist_ok=True)
    (crate_dir / "Cargo.toml").write_text(
        """
[package]
name = "rust_memory_bench"
version = "0.1.0"
edition = "2024"

[dependencies]
""".lstrip(),
        encoding="utf-8",
    )
    (crate_dir / "src" / "lib.rs").write_text(code, encoding="utf-8")
    (crate_dir / "tests" / "generated_tests.rs").write_text(tests + "\n", encoding="utf-8")


def run_cargo_test(crate_dir: Path) -> tuple[bool, str]:
    result = subprocess.run(
        ["cargo", "test", "--quiet"],
        cwd=crate_dir,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        timeout=120,
    )
    output = "\n".join(part for part in [result.stdout or "", result.stderr or ""] if part.strip())
    return result.returncode == 0, output


def run_condition(run_dir: Path, condition: str, memory_block: str) -> list[dict[str, object]]:
    rows: list[dict[str, object]] = []
    for task in TASKS:
        task_dir = run_dir / condition / task.task_id
        crate_dir = task_dir / "crate"
        if task_dir.exists():
            shutil.rmtree(task_dir)
        task_dir.mkdir(parents=True, exist_ok=True)
        task_memory_block = memory_block_for_task(memory_block, task)

        prompt = build_prompt(task, task_memory_block)
        start = time.time()
        builder_raw = call_ollama(prompt)
        builder_elapsed = time.time() - start
        builder_code = extract_rust_code(builder_raw)

        code = builder_code
        attempts = []
        passed = False
        cargo_output = ""

        write_crate(crate_dir, code, task.tests)
        passed, cargo_output = run_cargo_test(crate_dir)
        attempts.append(
            {
                "attempt": 0,
                "stage": "builder",
                "passed": passed,
                "code_chars": len(code),
                "cargo_output_chars": len(cargo_output),
            }
        )
        (task_dir / "attempt_0_builder_lib.rs").write_text(code, encoding="utf-8")
        (task_dir / "attempt_0_builder_cargo_output.txt").write_text(cargo_output, encoding="utf-8")

        adversary_prompt = ""
        adversary_raw = ""
        adversary_code = ""
        adversary_elapsed = 0.0

        if not passed:
            adversary_prompt = build_adversary_prompt(task, task_memory_block, builder_code)
            start = time.time()
            adversary_raw = call_ollama(adversary_prompt)
            adversary_elapsed = time.time() - start
            adversary_code = choose_code(extract_rust_code(adversary_raw), builder_code)
            code = adversary_code

            write_crate(crate_dir, code, task.tests)
            passed, cargo_output = run_cargo_test(crate_dir)
            attempts.append(
                {
                    "attempt": 1,
                    "stage": "adversary",
                    "passed": passed,
                    "code_chars": len(code),
                    "cargo_output_chars": len(cargo_output),
                }
            )
            (task_dir / "attempt_1_adversary_lib.rs").write_text(code, encoding="utf-8")
            (task_dir / "attempt_1_adversary_cargo_output.txt").write_text(
                cargo_output,
                encoding="utf-8",
            )

        repair_count = 0
        while not passed and repair_count < MAX_REPAIR_ATTEMPTS:
            repair_count += 1
            repair_prompt = build_repair_prompt(task, task_memory_block, code, cargo_output, repair_count)
            start = time.time()
            repair_raw = call_ollama(repair_prompt)
            repair_elapsed = time.time() - start
            repaired_code = choose_code(extract_rust_code(repair_raw), code)
            (task_dir / f"repair_{repair_count}_prompt.txt").write_text(
                repair_prompt,
                encoding="utf-8",
            )
            (task_dir / f"repair_{repair_count}_raw_response.txt").write_text(
                repair_raw,
                encoding="utf-8",
            )
            code = repaired_code

            write_crate(crate_dir, code, task.tests)
            passed, cargo_output = run_cargo_test(crate_dir)
            attempt_index = len(attempts)
            attempts.append(
                {
                    "attempt": attempt_index,
                    "stage": f"repair_{repair_count}",
                    "passed": passed,
                    "repair_elapsed_sec": round(repair_elapsed, 3),
                    "code_chars": len(code),
                    "cargo_output_chars": len(cargo_output),
                }
            )
            (task_dir / f"attempt_{attempt_index}_repair_{repair_count}_lib.rs").write_text(
                code,
                encoding="utf-8",
            )
            (task_dir / f"attempt_{attempt_index}_repair_{repair_count}_cargo_output.txt").write_text(
                cargo_output,
                encoding="utf-8",
            )

        (task_dir / "prompt.txt").write_text(prompt, encoding="utf-8")
        (task_dir / "builder_raw_response.txt").write_text(builder_raw, encoding="utf-8")
        (task_dir / "builder_lib.rs").write_text(builder_code, encoding="utf-8")
        (task_dir / "adversary_prompt.txt").write_text(adversary_prompt, encoding="utf-8")
        (task_dir / "adversary_raw_response.txt").write_text(adversary_raw, encoding="utf-8")
        (task_dir / "adversary_lib.rs").write_text(adversary_code, encoding="utf-8")
        (task_dir / "extracted_lib.rs").write_text(code, encoding="utf-8")
        (task_dir / "cargo_output.txt").write_text(cargo_output, encoding="utf-8")
        (task_dir / "attempts.json").write_text(json.dumps(attempts, indent=2), encoding="utf-8")

        row = {
            "condition": condition,
            "task_id": task.task_id,
            "passed": passed,
            "builder_elapsed_sec": round(builder_elapsed, 3),
            "adversary_elapsed_sec": round(adversary_elapsed, 3),
            "adversary_used": bool(adversary_prompt),
            "attempts": len(attempts),
            "repaired": len(attempts) > 1,
            "passed_stage": next((item["stage"] for item in attempts if item["passed"]), ""),
            "code_chars": len(code),
            "cargo_output_chars": len(cargo_output),
        }
        rows.append(row)
        status = "PASS" if passed else "FAIL"
        print(
            f"{condition:<18} | {task.task_id:<18} | {status} | "
            f"attempts={len(attempts)} stage={row['passed_stage'] or '-'} "
            f"builder={builder_elapsed:.1f}s adversary={adversary_elapsed:.1f}s"
        )
    return rows


def summarize(rows: list[dict[str, object]]) -> list[dict[str, object]]:
    conditions = sorted({str(row["condition"]) for row in rows})
    summary = []
    for condition in conditions:
        group = [row for row in rows if row["condition"] == condition]
        summary.append(
            {
                "condition": condition,
                "tasks": len(group),
                "passed": sum(1 for row in group if row["passed"]),
                "pass_rate": round(sum(1 for row in group if row["passed"]) / len(group), 4),
                "avg_builder_elapsed_sec": round(
                    sum(float(row["builder_elapsed_sec"]) for row in group) / len(group),
                    3,
                ),
                "avg_adversary_elapsed_sec": round(
                    sum(float(row["adversary_elapsed_sec"]) for row in group) / len(group),
                    3,
                ),
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
    run_dir = RUN_ROOT / f"rust-coder-memory-{stamp}"
    run_dir.mkdir(parents=True, exist_ok=True)

    rows: list[dict[str, object]] = []
    rows.extend(run_condition(run_dir, "no_memory", ""))
    rows.extend(run_condition(run_dir, "rust_memories", RUST_SPECIALIST_MEMORIES))
    rows.extend(run_condition(run_dir, "rust_code_memories", TASK_CODE_MEMORY_SENTINEL))

    summary = summarize(rows)
    write_csv(run_dir / "trial_rows.csv", rows)
    write_csv(run_dir / "summary.csv", summary)
    (run_dir / "report.json").write_text(
        json.dumps(
            {
                "model": MODEL,
                "description": "Instruct Rust coder benchmark: no memory versus Rust-specialist memories.",
                "tasks": [task.task_id for task in TASKS],
                "summary": summary,
            },
            indent=2,
        ),
        encoding="utf-8",
    )
    (run_dir / "rust_specialist_memories.txt").write_text(RUST_SPECIALIST_MEMORIES + "\n", encoding="utf-8")
    (run_dir / "rust_code_pattern_memories.txt").write_text(
        RUST_CODE_PATTERN_MEMORIES + "\n",
        encoding="utf-8",
    )

    print()
    print(f"Run dir: {run_dir}")
    for row in summary:
        print(f"{row['condition']:<18} {row['passed']}/{row['tasks']} pass_rate={row['pass_rate']:.3f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
