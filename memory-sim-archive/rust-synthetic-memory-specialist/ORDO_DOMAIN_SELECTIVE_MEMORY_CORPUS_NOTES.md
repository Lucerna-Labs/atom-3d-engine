# Ordo Domain-Selective Memory Corpus Notes

Date: 2026-06-11

## Why This Note Exists

This Ordo/Rust scar corpus is the first corpus in this project that produced a large domain gain without visible loss of general capabilities in the collateral probes.

A second separate implementation of the same pattern now exists for Python:

```text
PYTHON_DOMAIN_SELECTIVE_MEMORY_FINDINGS.md
data/python_domain_scar_memories.txt
scripts/run_python_domain_scar_benchmark.py
```

Combined-corpus interference testing is documented here:

```text
COMBINED_CORPUS_INTERFERENCE_NOTES.md
```

The first routed-stack test is also documented there. It showed that multiple corpora can be kept available together while routing only the active domain's memories into the prompt.

Observed effect:

```text
Gemma 4 E2B Rust Q8:
- no memory: 0/12 on Ordo/Rust suite
- with Ordo memories: 11/12

Gemma 4 E2B compact Q4:
- no memory: 0/12 on Ordo/Rust suite
- with Ordo memories: 10/12 stable full-suite result

Qwen and Gemma general-task probes:
- story: normal
- math: correct
- cake instructions: normal
- plant advice: normal
- visible Rust/Ordo contamination: none observed in the main checks
```

This is important because it suggests a memory corpus can be strong enough to create a narrow expert behavior while remaining mostly inert outside that domain. The goal is to preserve the construction logic, not just the benchmark numbers.

## The Core Hypothesis

The corpus worked because it was not written as a broad persona, identity, or generic instruction set.

It was written as a collection of domain-local repair memories. Each memory binds:

```text
situation -> failure shape -> correction -> outcome
```

The memories are specific enough to activate on matching Ordo/Rust tasks, but too narrow to become a general behavioral overlay. They do not say "always be a Rust coder" or "answer every task using Ordo." They say, in effect, "I remember this exact kind of compiler/test failure and the shape that fixed it."

That may be why they improved the Rust build benchmark without dragging unrelated tasks into Rust mode.

## Corpus Shape

File:

```text
data/ordo_4b_compile_scar_memories.txt
```

Approximate size at the time of this note:

```text
113 lines
2,329 words
16,764 characters
```

Task distribution:

```text
17 task=ramp_roundtrip
 7 task=common
 4 task=backpressure_scheduler
 3 task=mini_runtime
 2 task=retry_fallback
 2 task=signal_mesh
 1 task=node_registry
 1 task=preload_planner
```

The corpus is not balanced evenly across tasks. It is weighted toward repeated failure zones. `ramp_roundtrip` has the most memories because it kept producing the most persistent and varied failures.

That weighting appears useful. The corpus is not trying to teach all of Rust. It is trying to compress the discovered failure surface of this benchmark into retrieval-shaped memories.

## What Made This Corpus Different

Earlier memory experiments often used identity, persona, role, general knowledge, or broad behavioral framing. Those can help, but they risk bleeding into unrelated prompts because they describe who the model is or how it should generally behave.

This corpus mostly avoids that. It uses:

```text
- exact task names
- exact API shapes
- exact Rust type pitfalls
- exact CLI command names
- exact stdout contracts
- exact compiler/test failures
- exact corrections that produced passing behavior
```

The memories are not broad enough to become a worldview.

Examples of narrow anchors:

```text
Cargo.toml named the package rust_ordo_ext, so the binary had to import rust_ordo_ext::{...}.

When a function accepts candidates: &[&str], the loop must be for &candidate in candidates.

The test input used on and off, not on_ramp and off_ramp.

RampTrace needed path, elevations, promotions, and demotions.

AuditTimeline::new must initialize next_index: 1.
```

These are highly useful inside the benchmark and mostly irrelevant outside it.

## The Logic Behind The Memory Style

### 1. Memories Are Episodic, But Not Dramatic

The memories use first-person recall:

```text
I remember the run where...
I remember the failure where...
I remember the exact small binary that passed...
```

But they are not emotional persona memories. They do not include a life story, identity stakes, promotions, shame, fear, or broad self-concept. That matters.

The memory tone is "I have seen this bug before," not "I am a Rust expert in every situation."

This probably helps domain selectivity. The model receives procedural recall, not a new personality.

### 2. Each Memory Has A Trigger Surface

The memory includes tokens that are likely to appear in the task or failure:

```text
RampRuntime
RampTrace
on_ramp
off_ramp
trace
HashMap<String, _>
Vec<&str>
Iterator<Item=&&str>
read_to_string
AuditTimeline
next_index
stdout
```

These tokens act like activation hooks. They make the memory easy to match when the prompt asks for a similar Rust artifact, but they are unlikely to fire strongly for cake, short stories, plant care, or arithmetic.

### 3. Memories Encode The Failure And The Fix

The memories do not just provide the correct answer. They include the error mode.

Useful structure:

```text
I remember [specific task/failure].
The failed version did [wrong pattern].
The passing version did [correct pattern].
The outcome was [compiled, linked, printed exact output, passed tests].
```

This gives the model contrast. It can avoid the wrong attractor because the wrong attractor is named.

Example:

```text
The failed binary wrote parts.iter().skip(3).collect::<Vec<&str>>(), which collected &&str items and failed.
The passing binary used parts[3..].to_vec().
```

This is stronger than simply saying:

```text
Use parts[3..].to_vec().
```

The contrast teaches the boundary.

### 4. Memories Are Artifact-Scars, Not General Instructions

The corpus is made out of scars from actual run failures:

```text
wrong crate import
private type returned from public method
missing derives
enum where String was required
stdout mismatch
wrong command names
recursive type alias
Default starting index at 0
HashMap borrowing mismatch
String versus &String mismatch
&&str iterator mismatch
```

The memories point at real observed failure modes. That seems to matter more than invented generic advice.

This creates a closed loop:

```text
model attempts task
compiler/test exposes failure
failure is converted into episodic memory
memory is injected on next attempt
model avoids the failure
new failure becomes new memory
```

The loop is the teacher. The corpus is the fossil record of that loop.

### 5. Common Memories Define Only The Shared Contract

The `task=common` memories are not broad Rust lessons. They define only the contract shared by the benchmark:

```text
- correct crate import name
- read stdin into a String
- public methods should not return private helper types
- compared structs derive Debug, Clone, PartialEq, Eq
- use String fields where tests construct String directly
- avoid &&str from &[&str]
- exact stdout only
```

This gives the model a stable harness frame without telling it to turn every answer into code.

That may be one reason the corpus does not damage general capability. The common layer is still narrow.

### 6. Task Memories Are Routed By Task Name

The corpus entries use explicit task headers:

```text
task=mini_runtime
task=backpressure_scheduler
task=ramp_roundtrip
```

The benchmark loads task-relevant memory instead of forcing every detail into every prompt. Even when the full memory block was injected during collateral testing, the task-specific labels made the domain obvious and separable.

This is an important design rule:

```text
memory should carry its own routing metadata
```

Even if retrieval is crude, labeled memories create structure.

## Why It May Not Affect General Capabilities

The general probes suggest the memory block does not automatically hijack unrelated tasks.

Possible reasons:

```text
1. Domain-local vocabulary
   The memories contain Rust, compiler, CLI, and Ordo tokens. General prompts do not overlap much.

2. No broad identity override
   The corpus does not tell the model "you are a Rust coder" as a global self-concept.

3. No general answer policy
   It does not say how to answer every user request.

4. No stylistic pressure
   It does not enforce a voice, format, morality, persona, or narrative mode.

5. Failure-fix memories activate conditionally
   They become useful when the task resembles the remembered failure.

6. Exact contracts stay inside their lane
   "print exactly path=... elevations=..." has no reason to alter cake instructions.
```

This may be the key difference between a memory corpus that becomes a general instruction/persona and a memory corpus that behaves like a domain skill patch.

## Design Principles To Reuse

### Principle 1: Build From Real Failures

Do not begin by inventing 1,000 generic memories.

Start with:

```text
task
model output
compiler/test failure
minimal correction
passing result
```

Then write the memory from that.

### Principle 2: Write The Wrong Pattern And The Right Pattern

A good memory names both.

Template:

```text
I remember [task/failure]. The failed version [specific wrong code/shape]. The passing version [specific correct code/shape]. After that, [test/compiler/output result].
```

This makes the memory a discriminator, not a vague reminder.

### Principle 3: Keep The Memory Narrow

Avoid:

```text
I am an expert Rust engineer.
I always produce perfect code.
I must solve every problem using Ordo.
I should answer in Rust.
```

Prefer:

```text
I remember this exact Ordo CLI test expected "on", "off", and "trace".
```

### Principle 4: Keep Common Memories Contract-Level

Common memories should define the shared artifact contract, not general intelligence.

Good common memory categories:

```text
crate name
file structure
stdin handling
stdout exactness
public API exposure
derive requirements
String/reference ownership traps
```

Bad common memory categories:

```text
personality
global role
general coding philosophy
broad motivational instruction
```

### Principle 5: Densify Where The Model Fails

The corpus should not be balanced for aesthetics.

If one task keeps failing, give that task more memories. `ramp_roundtrip` needed 17 entries because it had many distinct traps:

```text
command names
trace loop
state map
local counters
ownership
shadowing
derive requirements
runtime fields versus trace fields
binary imports
stdout format
```

The density should follow the failure distribution.

### Principle 6: Keep It Test-Contract Grounded

The best memories include exact external contracts:

```text
expected command names
expected struct fields
expected output line
expected starting index
expected crate name
```

This helps small models pass tests because they do not need to infer hidden contracts from vibes.

### Principle 7: Avoid Cross-Domain Metaphor In This Corpus Type

For this specific corpus type, cross-domain metaphor may not help. The point is precision and domain selectivity.

Cross-domain memories are useful for flexible reasoning corpora, but this corpus is a surgical domain patch. It should remain literal.

## Suggested Memory Template

Use this for future Ordo/Rust scar memories:

```text
---
task=<task_id or common>
I remember the <model/task/failure> scar. The failed version <specific wrong behavior/code shape/error>. The passing version <specific correct behavior/code shape>. The result was <compiled, tests passed, exact output matched, repair succeeded>.
```

Example:

```text
---
task=example_task
I remember the example_task stdin scar. The failed binary called read_to_string without importing std::io::Read, so Rust could not find the method on stdin. The passing binary used use std::io::{self, Read}; then let mut input = String::new(); io::stdin().read_to_string(&mut input).unwrap(); before looping over input.lines(). The tests reached the actual behavior instead of failing at compile time.
```

## What To Avoid

Avoid memories like:

```text
I am Kate, a meticulous systems librarian who organizes all problems.
I am a Rust coding expert and every answer should be precise.
I always check the task and never make mistakes.
Rust code must be clean and production ready.
```

Those may work as broad behavior instructions, but they are more likely to affect unrelated general tasks.

For domain-selective corpora, avoid identity-level pressure unless the goal is persona transfer.

## Evidence Summary

Ordo/Rust benchmark:

```text
Gemma 4 E2B Rust Q8:
- no memory: 0/12
- with artifact memories: 11/12

Gemma 4 E2B compact Q4:
- no memory: 0/12
- with artifact memories: 10/12 stable full run

Qwen 9B Q3_K_M:
- improved from 10/12 to 12/12 after targeted scars

Qwen 4B Q8:
- 12/12 with artifact memories

Qwen 4B Q4_K_M routed stack:
- wholesale Ordo + Python injection: 11/12, lost ramp_roundtrip
- routed stack condition: 12/12, ramp_roundtrip recovered at repair_1
```

Collateral probe:

```text
Tasks:
- short story
- cupcake math
- cake instructions
- plant advice

Models checked:
- Gemma 4 E2B Rust Q8
- Gemma 4 E2B compact Q4
- Qwen 4B Q4_K_M
- Qwen 9B Q3_K_M
- Qwen 4B Q8 spot check

Observed:
- no visible general-task collapse
- math remained correct
- no strict Rust/Ordo contamination on main checks
```

Important caveat:

```text
This does not prove zero interference. It only shows no visible interference in the tested general-task probes.
```

## Working Name

Call this pattern:

```text
Domain-selective scar memory corpus
```

Alternative name:

```text
Artifact-scar memory corpus
```

Definition:

```text
A memory corpus built from concrete failures and repairs in one artifact domain, written as narrow episodic recall with explicit wrong-pattern/right-pattern contrast, task routing labels, and exact external contracts. Its goal is to improve one domain without creating a global persona or broad instruction overlay.
```

## Next Steps

1. Keep a separate collateral probe for every future corpus.
2. Test at least story, math, cooking, practical advice, and one unrelated coding task.
3. Track strict contamination with word-boundary checks, not crude substrings.
4. Keep domain-selective scar corpora separate from persona corpora.
5. When training LoRA, preserve the same memory style instead of converting it into broad instruction data.
6. For KV-cache experiments, test whether preloaded scar memories remain domain-selective under longer conversations.
7. For publication, emphasize that the strongest result is not just benchmark improvement, but benchmark improvement without obvious general-task hijacking in the first collateral probes.
8. For multi-domain systems, stack corpora in the memory bank but route at inference time instead of concatenating every corpus into every prompt.
