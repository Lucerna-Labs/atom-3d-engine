# Rust Coder Memory Benchmark Findings

Date: 2026-06-08

This branch tests whether synthetic memories can turn a small instruct model into a narrow Rust coder that produces library code passing real `cargo test`.

## Models

Initial 2B run:

```text
qwen3.5:2b-q8_0
```

4B follow-up runs:

```text
qwen35-4b-instruct-q8-chat:latest
```

9B comparison run:

```text
qwen3.5:9b-q8_0
```

Lower-quant 4B probes:

```text
qwen35-4b-base-q6-raw:latest
qwen35-4b-claude-distill-v2-q6-chat
```

The harness uses Ollama `/api/chat` with:

```json
{ "think": false }
```

This matters because these Qwen chat models can otherwise spend output budget on thinking-style continuations that clash with the memory-conditioned code task.

## Harness

Script:

```text
scripts/run_rust_coder_memory_benchmark.py
```

Current latest run:

```text
rag_runs/rust-coder-memory-20260608-141130
```

The benchmark creates throwaway Rust library crates for each task and runs real:

```text
cargo test --quiet
```

The verifier is real, not simulated. Each run writes a `summary.csv`, `trial_rows.csv`, generated crates, prompts, outputs, and cargo logs under the run directory.

## Subagent-Style Loop

The harness uses a simple subagent-style workflow:

1. Builder writes `src/lib.rs`.
2. Verifier immediately runs `cargo test`.
3. If builder passes, stop.
4. If builder fails, adversarial reviewer rewrites the draft.
5. Verifier tests the adversarial rewrite.
6. Repair agent gets compiler/test output for up to two repair attempts.

This order matters. Earlier, the adversarial reviewer ran before verification and damaged good drafts. Builder-first verification fixed that controller error.

## Conditions Tested

| Condition | Description |
|---|---|
| `no_memory` | 4B instruct model with task only |
| `rust_memories` | episodic failed/success Rust memories plus logic memories |
| `rust_code_memories` | routed task-specific code-shaped memories plus Rust scar memories |

The final version routes only the relevant code memory for each task instead of feeding all code memories at once. Feeding all memories caused snippet blending and malformed hybrids.

## Exhaustive 20-Task Suite

The expanded suite covers 20 small Rust library tasks:

```text
slugify
duration_parser
top_words
moving_average
merge_intervals
dedup_preserve_order
parse_bool_flag
median_i32
group_by_first_letter
checksum_xor
csv_line
redact_digits
retry_schedule
query_string
clamp_all
histogram_u8
nonblank_lines
first_duplicate
parse_csv_numbers
min_max
```

Each task has hidden local tests in `tests/generated_tests.rs` inside the temporary crate.

## 4B Exhaustive Results

Run:

```text
rag_runs/rust-coder-memory-20260608-141130/summary.csv
```

| Condition | Passed | Total | Pass Rate | Builder Passes | Repair Passes | Avg Attempts |
|---|---:|---:|---:|---:|---:|---:|
| `no_memory` | 17 | 20 | 0.850 | 12 | 5 | 2.05 |
| `rust_memories` | 16 | 20 | 0.800 | 10 | 6 | 2.30 |
| `rust_code_memories` | 20 | 20 | 1.000 | 20 | 0 | 1.00 |

Failures:

| Condition | Failed Tasks |
|---|---|
| `no_memory` | `slugify`, `duration_parser`, `moving_average` |
| `rust_memories` | `slugify`, `duration_parser`, `moving_average`, `query_string` |
| `rust_code_memories` | none |

The strongest result so far:

```text
4B instruct + routed code-shaped memories: 20/20 clean cargo-test builds
```

Every `rust_code_memories` task passed at builder stage. The adversarial and repair stages did not need to run for that condition.

## Earlier 2B Results

Earlier 2B run:

```text
rag_runs/rust-coder-memory-20260608-133240
```

| Condition | Passed | Total | Pass Rate |
|---|---:|---:|---:|
| `no_memory` | 0 | 5 | 0.000 |
| `rust_memories` | 0 | 5 | 0.000 |
| `rust_code_memories` | 2 | 5 | 0.400 |

The 2B model only passed `slugify` and `parse_duration_ms`, both at builder stage. That result suggested the mechanism existed but was copy-fidelity limited at 2B.

## Earlier 4B Smoke Test

Earlier 4B five-task run:

```text
rag_runs/rust-coder-memory-20260608-135714
```

| Condition | Passed | Total | Pass Rate |
|---|---:|---:|---:|
| `no_memory` | 2 | 5 | 0.400 |
| `rust_memories` | 2 | 5 | 0.400 |
| `rust_code_memories` | 5 | 5 | 1.000 |

The 20-task run replaced this as the meaningful comparison.

## 9B Instruct Comparison

Run:

```text
rag_runs/rust-coder-memory-20260608-153009/summary.csv
```

This run used the same 20-task suite, same harness, same deterministic settings, and the 9B instruct model:

```text
qwen3.5:9b-q8_0
```

| Condition | Passed | Total | Pass Rate | Builder Passes | Repair Passes | Avg Attempts |
|---|---:|---:|---:|---:|---:|---:|
| `no_memory` | 14 | 20 | 0.700 | 11 | 3 | 2.25 |
| `rust_memories` | 14 | 20 | 0.700 | 11 | 3 | 2.20 |
| `rust_code_memories` | 19 | 20 | 0.950 | 16 | 3 | 1.45 |

Failures:

| Condition | Failed Tasks |
|---|---|
| `no_memory` | `slugify`, `duration_parser`, `top_words`, `moving_average`, `checksum_xor`, `min_max` |
| `rust_memories` | `duration_parser`, `top_words`, `moving_average`, `merge_intervals`, `csv_line`, `first_duplicate` |
| `rust_code_memories` | `top_words` |

The direct comparison against the 4B memory result is:

| Model / Condition | Passed | Pass Rate | Builder Passes |
|---|---:|---:|---:|
| 4B `no_memory` | 17/20 | 0.850 | 12/20 |
| 4B `rust_code_memories` | 20/20 | 1.000 | 20/20 |
| 9B `no_memory` | 14/20 | 0.700 | 11/20 |
| 9B `rust_code_memories` | 19/20 | 0.950 | 16/20 |

On this benchmark, the 4B model with routed code-shaped memories did not merely match the 9B instruct no-memory baseline. It exceeded it:

```text
4B + routed code memories: 20/20
9B + no memories:          14/20
```

The 9B model also improved strongly from routed memories:

```text
9B no_memory:          14/20
9B rust_code_memories: 19/20
```

The one 9B routed-memory failure was `top_words`. The generated code contained the correct deterministic sort, but also left a broken earlier `match` block above it. That failure is a cleanup/copy-fidelity issue more than a missing-algorithm issue.

## Lower-Quant 4B Probes

Two lower-quant 4B probes were run after the Q8 and 9B comparisons.

### 4B Q6 Base/Raw

Run:

```text
rag_runs/rust-coder-memory-20260608-155936/summary.csv
```

Model:

```text
qwen35-4b-base-q6-raw:latest
```

This model is Q6_K, but it is base/raw rather than the same instruct/chat model as the original 4B Q8 test. Treat it as a quantization/small-base sensitivity probe, not as a perfect apples-to-apples instruct comparison.

| Condition | Passed | Total | Pass Rate | Builder Passes | Repair Passes | Avg Attempts |
|---|---:|---:|---:|---:|---:|---:|
| `no_memory` | 15 | 20 | 0.750 | 12 | 3 | 2.15 |
| `rust_memories` | 13 | 20 | 0.650 | 10 | 3 | 2.35 |
| `rust_code_memories` | 20 | 20 | 1.000 | 20 | 0 | 1.00 |

### 4B Q6 Chat-Style Distill From NAS

The NAS drive `I:\` contained several Qwen 3.5 4B GGUF variants. The imported lower-quant chat-style candidate was:

```text
I:\llm\Small models\Qwen3.5-4B-Claude-4.6-Opus-Reasoning-Distilled-v2-GGUF\Qwen3.5-4B.Q6_K.gguf
```

It was imported as:

```text
qwen35-4b-claude-distill-v2-q6-chat
```

Run:

```text
rag_runs/rust-coder-memory-20260608-160725/summary.csv
```

| Condition | Passed | Total | Pass Rate | Builder Passes | Repair/Adversary Passes | Avg Attempts |
|---|---:|---:|---:|---:|---:|---:|
| `no_memory` | 16 | 20 | 0.800 | 10 | 6 | 2.30 |
| `rust_memories` | 15 | 20 | 0.750 | 6 | 9 | 2.45 |
| `rust_code_memories` | 20 | 20 | 1.000 | 19 | 1 | 1.05 |

The lower-quant pattern matches the earlier Q8 result:

```text
4B Q8 instruct/chat + routed code memories: 20/20
4B Q6 base/raw + routed code memories:      20/20
4B Q6 chat-style + routed code memories:   20/20
```

The lower-quant models were not better than Q8 without memories. The lift came from routed code-shaped memories. Abstract episodic Rust memories again did not help; they were neutral-to-negative.

## Rust CLI App-Build Benchmark

After the library-function benchmarks, a separate app benchmark tested whether the model could build complete Rust command-line apps.

Script:

```text
scripts/run_rust_app_memory_benchmark.py
```

Run:

```text
rag_runs/rust-app-memory-20260608-164539/summary.csv
```

Model:

```text
qwen35-4b-claude-distill-v2-q6-chat
```

This benchmark creates real binary crates with:

```text
Cargo.toml
src/main.rs
tests/cli.rs
```

The verifier runs:

```text
cargo test --quiet
```

The tests spawn the compiled binary with command-line args and stdin using `CARGO_BIN_EXE_rust_memory_app`. So this is a real app-build test, not a simulated app response.

Tasks:

```text
args_echo
stdin_line_count
sum_args
mini_grep
word_byte_count
unique_sort
kv_get
todo_filter
```

Results:

| Condition | Passed | Total | Pass Rate | Builder Passes | Avg Attempts |
|---|---:|---:|---:|---:|---:|
| `no_memory` | 6 | 8 | 0.750 | 4 | 1.50 |
| `app_memories` | 7 | 8 | 0.875 | 2 | 1.75 |
| `app_code_memories` | 8 | 8 | 1.000 | 8 | 1.00 |

Failures:

| Condition | Failed Tasks |
|---|---|
| `no_memory` | `mini_grep`, `todo_filter` |
| `app_memories` | `todo_filter` |
| `app_code_memories` | none |

The app result repeats the library-task pattern:

```text
no_memory:         6/8
app_memories:      7/8
app_code_memories: 8/8
```

The strongest detail is builder-stage success:

```text
app_code_memories: 8/8 builder-stage app builds
```

That means the memory-conditioned model generated complete compiling CLI apps on the first attempt for every app task in this suite.

## Hard Ordo Runtime Benchmark

After the CLI app benchmark, a harder benchmark asked the same 4B Q6 chat-style model to build a tiny Ordo-shaped runtime, not just a utility app.

Script:

```text
scripts/run_ordo_runtime_memory_benchmark.py
```

Latest run:

```text
rag_runs/ordo-runtime-memory-20260608-171154/summary.csv
```

Model:

```text
qwen35-4b-claude-distill-v2-q6-chat
```

The generated project had to include:

```text
Cargo.toml
src/lib.rs
src/main.rs
tests/runtime.rs
```

The verifier ran real `cargo test --quiet`. The hidden tests imported the library API and also spawned the binary with an Ordo runtime script.

Required runtime behavior:

```text
typed Message and Trace structs
OrdoRuntime public API
ground graph edges
ramp edges that mark promoted=true
emergent thread ids as messages flow
preload_targets
route_for_budget
backpressure:<node> vibration signals
stdin CLI commands
exact machine-readable publish output
```

Results:

| Condition | Passed | Total | Pass Rate |
|---|---:|---:|---:|
| `no_memory` | 0 | 1 | 0.000 |
| `app_memories` | 0 | 1 | 0.000 |
| `ordo_memories` | 0 | 1 | 0.000 |
| `ordo_code_memories` | 0 | 1 | 0.000 |

Three hardening passes were attempted. The best `ordo_code_memories` outputs got close structurally:

- produced `Message`, `Trace`, and `OrdoRuntime`
- implemented nodes, edges, ramps, preload targets, and cost routing
- implemented a stdin CLI adapter
- fixed the earlier `VecDeque.sort()` mistake after a scar memory was added
- fixed debug-style CLI output after a scar memory was added
- fixed broken stdin reading after a scar memory was added

But the model repeatedly failed on the publish loop ownership pattern:

```text
from = &edge.to
```

That borrows from a cloned edge that dies at the end of the match arm. The memory explicitly said to keep `current` as an owned `String`, but the model reverted to `mut from: &str` and repeated the lifetime error.

The latest failure also showed secondary likely test failures if compilation had succeeded:

- `next_thread_id` initialized to `0` instead of `1`
- `add_node` defaulted cost to `1` instead of `0`

This is the first clear hard-boundary result:

```text
Prompt-side memories can push small models through narrow Rust functions
and small CLI apps, but a multi-surface runtime requires either finer
decomposition, stronger repair, KV-prefill, or LoRA training.
```

The failure is not conceptual collapse. The model understood most of the Ordo runtime shape. The failure is exact Rust ownership/control-flow fidelity under a larger generated project.

## Ordo Memory-Build Corpus

After the hard-runtime failures, a dedicated Ordo memory-build corpus was created:

```text
data/ordo_runtime_memory_build_corpus.jsonl
```

This corpus contains Ordo-specific episodic memories across:

```text
architecture shape
ground bus versus ramp promotion
emergent thread traces
backpressure vibration
preload target behavior
cost-aware routing
Rust ownership scars
CLI adapter scars
public API fidelity
project shape
repair behavior
decomposition strategy
```

The benchmark script now supports a corpus-only condition:

```text
ORDO_RUNTIME_CONDITIONS=ordo_build_corpus
```

This lets expensive models run only the strongest memory condition instead of the full baseline sweep.

### 9B Instruct With Ordo Corpus

Run:

```text
rag_runs/ordo-runtime-memory-20260608-173456/summary.csv
```

Model:

```text
qwen3.5:9b-q8_0
```

| Condition | Passed | Total | Pass Rate |
|---|---:|---:|---:|
| `ordo_build_corpus` | 0 | 1 | 0.000 |

The 9B instruct model still failed the hard Ordo runtime with the expanded corpus.

### Python-Coder 4B With Ordo Corpus

Run:

```text
rag_runs/ordo-runtime-memory-20260608-173605/summary.csv
```

Model imported from the NAS:

```text
qwen35-4b-python-coder-q6-chat
```

Source GGUF:

```text
I:\llm\Coder Models\Qwen3.5-4B-Python-Coder-GGUF\Qwen3.5-4B.Q6_K.gguf
```

| Condition | Passed | Total | Pass Rate |
|---|---:|---:|---:|
| `ordo_build_corpus` | 0 | 1 | 0.000 |

The Python-coder 4B model also failed the hard Ordo runtime.

### Rust-Coder MoE With And Without Ordo Corpus

Model imported from the NAS:

```text
qwen3-coder-25b-a3b-rust-q5-chat
```

Source GGUF:

```text
I:\llm\Coder Models\Qwen3-coder-REAP-25B-A3B-Rust-GGUF\Qwen3-Coder-REAP-25B-A3B-Rust-Q5_K_M.gguf
```

Ollama metadata:

```text
architecture: qwen3moe
parameters:   24.9B
quantization: Q5_K_M
```

No-memory run:

```text
rag_runs/ordo-runtime-memory-20260608-173858/summary.csv
```

Ordo corpus run:

```text
rag_runs/ordo-runtime-memory-20260608-173725/summary.csv
```

| Model / Condition | Passed | Stage |
|---|---:|---|
| Rust-coder MoE `no_memory` | 0/1 | failed after repair |
| Rust-coder MoE `ordo_build_corpus` | 1/1 | builder |

This is the first successful hard-runtime result:

```text
Rust-specialized Qwen coder + Ordo memory-build corpus: PASS at builder stage
```

The generated runtime passed real `cargo test --quiet` with both library tests and a spawned CLI integration test.

The passing code included:

- public `Message`, `Trace`, and `OrdoRuntime`
- `next_thread_id` initialized to `1`
- owned `String` route cursor in `publish`
- ramp promotion via `Trace.promoted`
- backpressure signals shaped like `backpressure:model`
- `preload_targets` from outgoing edges
- preference-order `route_for_budget`
- stdin script adapter in `main`
- exact machine-readable publish output

The most important comparison is:

```text
Rust coder alone:              FAIL
Rust coder + Ordo build corpus: PASS
```

That suggests the hard-runtime win required both ingredients: a model with enough Rust/code capacity and a domain-specific Ordo memory corpus.

## Multi-Build Ordo Suite

After the single hard-runtime pass, the benchmark was expanded into a three-task Ordo build suite:

```text
scripts/run_ordo_build_suite_benchmark.py
```

Additional suite corpus:

```text
data/ordo_build_suite_memory_corpus.jsonl
```

Model:

```text
qwen3-coder-25b-a3b-rust-q5-chat
```

Tasks:

| Task | What It Tests |
|---|---|
| `mini_runtime` | ground graph, ramp promotion, trace path, backpressure, preload targets, CLI publish output |
| `intersection_fabric` | threads formed from flow routes, emergent intersections, transfer between threads, CLI flow/transfer output |
| `backpressure_scheduler` | budget routing, capacity checks, ramp promotion after backpressure, scheduler CLI output |

Initial expanded-suite run:

```text
rag_runs/ordo-build-suite-20260608-174825/summary.csv
```

| Condition | Passed | Total | Pass Rate | Builder Passes |
|---|---:|---:|---:|---:|
| `no_memory` | 0 | 3 | 0.000 | 0 |
| `ordo_build_corpus` | 1 | 3 | 0.333 | 1 |

The initial corpus passed `mini_runtime` but failed:

- `intersection_fabric`: extra CLI output and missing comparison derive
- `backpressure_scheduler`: missing `PartialEq/Eq` and ambiguous `Option` type

Targeted memories were then added for the observed failures:

- `TransferTrace` needs `Clone, Debug, PartialEq, Eq`
- CLI must not print an `intersections=...` summary line
- transfer output must be `transfer=1>2`, with no spaces around `>`
- `Plan` needs `Clone, Debug, PartialEq, Eq`
- `skipped_full_node` needs explicit `Option<String>`
- skipped-node ramp checks should compare with `skipped_node.as_str()`

Refinement curve:

| Run | Condition | Result |
|---|---|---:|
| `20260608-174825` | initial corpus | 1/3 |
| `20260608-175345` | after first scars | 1/3 |
| `20260608-175603` | after derive/type scars | 2/3 |
| `20260608-175755` | after exact CLI scars | 3/3 |

Final suite run:

```text
rag_runs/ordo-build-suite-20260608-175755/summary.csv
```

| Condition | Passed | Total | Pass Rate | Builder Passes | Avg Attempts |
|---|---:|---:|---:|---:|---:|
| `ordo_build_corpus` | 3 | 3 | 1.000 | 3 | 1.00 |

Final per-task result:

| Task | Result | Stage |
|---|---|---|
| `mini_runtime` | PASS | builder |
| `intersection_fabric` | PASS | builder |
| `backpressure_scheduler` | PASS | builder |

This is the strongest Ordo result so far:

```text
Rust-coder MoE + hardened Ordo memory corpus: 3/3 complex Ordo builds at builder stage
```

The key pattern was not one-shot perfection. It was closed-loop corpus hardening:

```text
test -> observe exact failure -> add lived scar memory -> rerun -> improve
```

That matches the larger thesis better than a single lucky pass. The memory corpus became more useful when it absorbed the specific failure outcomes from real compiler and integration-test pressure.

## Extended 12-Task Ordo Build Suite

The next suite expanded beyond the earlier 3-task Ordo benchmark because a Kaggle-facing result needs more than a few examples.

Script:

```text
scripts/run_ordo_extended_suite_benchmark.py
```

Additional memory corpus:

```text
data/ordo_extended_suite_memory_corpus.jsonl
```

Model:

```text
qwen3-coder-25b-a3b-rust-q5-chat
```

The suite creates 12 separate Rust binary/library crates and runs real:

```text
cargo test --quiet
```

Each task has both library API tests and CLI integration tests. The tasks are:

```text
mini_runtime
intersection_fabric
backpressure_scheduler
signal_mesh
preload_planner
ramp_roundtrip
retry_fallback
cost_ledger
workflow_codec
node_registry
fanout_join
audit_timeline
```

### Memory Refinement Curve

The first pass overfed the older full runtime corpus into every task and performed poorly. Routing the corpus more tightly and then adding memories from exact cargo/test failures produced the lift.

| Run | Change | Result | Builder Passes |
|---|---|---:|---:|
| `rag_runs/ordo-extended-suite-20260608-182417` | overfed broad memories | 2/12 | not final |
| `rag_runs/ordo-extended-suite-20260608-183351` | routed task-specific memories | 4/12 | 3 |
| `rag_runs/ordo-extended-suite-20260608-184240` | added API signature scars | 6/12 | 6 |
| `rag_runs/ordo-extended-suite-20260608-185020` | added CLI/behavior scars | 10/12 | 9 |
| `rag_runs/ordo-extended-suite-20260608-185527` | added preload/ramp scars | 10/12 | 9 |

Best memory result:

```text
rag_runs/ordo-extended-suite-20260608-185020/summary.csv
```

| Condition | Passed | Total | Pass Rate | Builder Passes | Avg Attempts |
|---|---:|---:|---:|---:|---:|
| `ordo_memory` | 10 | 12 | 0.833 | 9 | 1.25 |

No-memory baseline on the same 12 tasks:

```text
rag_runs/ordo-extended-suite-20260608-190030/summary.csv
```

| Condition | Passed | Total | Pass Rate | Builder Passes | Avg Attempts |
|---|---:|---:|---:|---:|---:|
| `no_memory` | 0 | 12 | 0.000 | 0 | 2.00 |

The direct comparison is:

```text
Rust-coder MoE no_memory:     0/12
Rust-coder MoE Ordo memories: 10/12
```

The best memory run passed 11 of the 12 task types across the two 10/12 runs, but not all in one run. The `20260608-185020` run failed `ramp_roundtrip` and `preload_planner`; the `20260608-185527` run fixed `preload_planner` but regressed `mini_runtime`. This suggests the corpus is near a context-interference boundary. The next improvement should route even more narrowly instead of simply adding more memories.

### What The 12-Task Suite Shows

The 12-task suite is the strongest current Ordo evidence:

```text
same model
same task suite
same deterministic harness
real Cargo verifier
0/12 without memories
10/12 with hardened Ordo memories
```

The important pattern is the closed loop:

```text
run real model -> compile/test -> observe exact failure -> write scar memory -> rerun
```

This is not a simulation score. The outputs are generated Rust crates, and the pass/fail signal comes from real compiler and integration-test execution.

## Benchmark Corrections

During the earlier 4B run, the `merge_intervals` test exposed an inconsistency in the benchmark.

The task said:

```text
Merge overlapping or touching intervals.
```

The original test expected `(1,4)` and `(5,7)` to merge while also expecting `(5,7)` and `(8,10)` not to merge. That was inconsistent. The test was corrected to the normal interval rule:

```text
touching = shared endpoint
```

So:

```text
(1,3) + (3,4) -> (1,4)
(1,4) and (5,7) remain separate
```

Corrected expected output:

```rust
vec![(1, 4), (5, 7), (8, 10)]
```

During the 20-task expansion, the `csv_line` test also exposed a Python string escaping issue. The generated Rust test briefly contained an invalid string literal. The test was corrected to emit:

```rust
parse_csv_line("a,\"b,c\",d")
```

That was a benchmark bug, not a model failure.

## Main Findings

### 1. Abstract Rust memories were not enough

The failed/success/logic memory mix did not improve the 4B model on the expanded suite:

```text
no_memory:     17/20
rust_memories: 16/20
```

For small coding models, abstract memories can become distracting context unless they are tightly connected to executable implementation shape.

### 2. Routed code-shaped memories were decisive

Routed code-shaped memories moved the 4B model from:

```text
17/20 -> 20/20
```

More importantly, they moved builder-stage success from:

```text
12/20 -> 20/20
```

That means the model did not merely become easier to repair. It wrote passing Rust immediately when the relevant implementation-shaped memory was supplied.

### 3. The subagent loop is useful as a harness, not the main source of the win

The verifier is essential because it prevents bad rewrites from being accepted. But in the best condition, the builder passed every task before adversary or repair ran.

The current best architecture is:

```text
routed implementation memory -> builder -> real cargo verifier
```

The adversary and repair stages remain useful for failure analysis and future harder tests.

### 4. Copy fidelity and routing are the core mechanism

Earlier failures showed exact copy-fidelity losses:

- omitted a required struct field
- missed an explicit `Vec<(i32, i32)>` type annotation
- moved a `String` instead of preserving it with `std::mem::take`
- blended unrelated snippets when too many code memories fired at once

The 4B model preserved the details when each task received only the relevant code-shaped memory.

### 5. The next proof is held-out generalization

The 20/20 result is meaningful, but it is not final proof that the model learned a general Rust-coding instinct. The strongest condition uses task-shaped memories, so the next test should include held-out tasks that were not directly represented by a code memory.

The next suite should separate:

```text
seen pattern transfer
near-neighbor transfer
held-out generalization
adversarial compiler-error repair
```

## Interpretation

The result supports this claim:

```text
Concrete, routed, code-shaped memories can improve a small instruct model's
ability to produce clean Rust builds on narrow tasks.
```

The result does not yet prove:

```text
Abstract episodic Rust memories alone can make a small instruct model a reliable Rust coder.
```

It also does not yet prove:

```text
The model can generalize the memory mechanism to unseen Rust tasks without direct code-shaped support.
```

The useful mechanism appears to be:

```text
routed code-shaped memory -> builder copies implementation shape -> cargo passes
```

The failure mechanism appears to be:

```text
broad or abstract memory -> attention diffusion or irrelevant narrative pull -> compiler errors remain
```

## Next Changes

1. Add held-out Rust tasks that do not have exact task-specific memories.
2. Add near-neighbor tasks that share only part of a remembered implementation pattern.
3. Add copy-fidelity memories:
   - preserve fields exactly
   - preserve type annotations exactly
   - preserve helper functions exactly
   - do not rewrite known-good code after cargo passes
4. Add compiler-error repair memories:
   - `E0382 moved value -> use std::mem::take or clone`
   - `E0282 type annotations needed -> add explicit Vec type`
   - `E0609 unknown field -> restore the missing struct field`
5. Try mixed-agent routing:
   - 4B builder with routed memories
   - larger reviewer only after real verifier failure
6. Prepare a LoRA dataset from the successful code-shaped memories:
   - prompt
   - remembered pattern
   - clean final `lib.rs`
   - cargo error
   - minimal repair

## Current Read

The 2B experiment showed partial signal but strong copy-fidelity limits.

The 4B exhaustive experiment is a clear narrow-domain win:

```text
no_memory:          17/20
rust_memories:      16/20
rust_code_memories: 20/20
```

The lower-quant 4B follow-ups repeated the same shape:

```text
Q6 base/raw no_memory:          15/20
Q6 base/raw rust_code_memories: 20/20

Q6 chat-style no_memory:          16/20
Q6 chat-style rust_code_memories: 20/20
```

The same model also passed the app-build benchmark:

```text
Q6 chat-style app no_memory:          6/8
Q6 chat-style app_code_memories:      8/8
```

The same model did not pass the hard Ordo runtime benchmark:

```text
Q6 chat-style Ordo runtime no_memory:          0/1
Q6 chat-style Ordo runtime ordo_code_memories: 0/1
```

A Rust-specialized coder model did pass after adding the dedicated Ordo memory-build corpus:

```text
Rust-coder MoE no_memory:          0/1
Rust-coder MoE ordo_build_corpus:  1/1
```

The expanded Ordo suite strengthened that result:

```text
Rust-coder MoE no_memory:                  0/3
Rust-coder MoE hardened Ordo build corpus: 3/3
```

The harder 12-task Ordo suite strengthened it again:

```text
Rust-coder MoE no_memory:                  0/12
Rust-coder MoE hardened Ordo build corpus: 10/12
```

The most important detail is not just the final pass rate. It is that `rust_code_memories` hit:

```text
20/20 builder-stage passes
```

That is exactly the behavior we want before moving to LoRA: the memory corpus can make the model act like a narrower, more reliable Rust builder under real compiler verification.
