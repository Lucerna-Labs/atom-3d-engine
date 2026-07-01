# Ordo-Architecture LoRA — Surgical Dataset Combination Plan

**Date:** 2026-06-22
**Scope:** Train an "over-specialized" Qwen3.5-2B / 4B LoRA on Kaggle that
**builds Ordo architecture** (primitive / adapter / provider / orchestrator) **and**
applies it across math, Rust, software engineering, and research domains.
**Key rule:** *Surgical*, not concatenated. Each layer has its own voice,
weight, and training stage. Mixing happens at the curriculum level, not the
token level.

---

## 1. Why "surgical" and not concat

Your existing corpus already proves the layering. Read these four files and the
shape jumps out:

| File                                  | Records | Layer                                                    |
|---------------------------------------|---------|----------------------------------------------------------|
| `corpus/cognitive_core_v0_1.json`     |   ~24   | Orientation / thinking moves (first-minute map, warrant) |
| `corpus/cognitive_rewards_v0_1.json`  |   ~12   | Reward shaping ("two memories were enough", provenance)  |
| `corpus/structural_primitives_lora_v0_1.json` | ~1,100 | Primitive grammar (FRAME, CLOCK…) over families |
| `corpus/computation_primitives_v0_1.json` |  ~30  | Domain moves (tiered pricing, weighted average)          |
| `corpus/rust_cyber_defender_generated.tsv` | ~360 | **Identity memory** with valence + benign contrast       |
| `corpus/librarian_core_v0_1.json`     |  ~30   | Librarian identity (research mode persona)               |

The voice is **first-person "I remember / I reached for [primitive] without
naming it as a rule"** — not Alpaca-style instruction. Concat-then-SFT will
overwrite that voice with `### Instruction / ### Response` patterns from raw
external corpora, and the LoRA will lose the very grammar we're trying to
install.

So the surgical plan:
- **External data is reshaped into the Ordo voice** before mixing
- **External data is layer-tagged** (which layer it feeds)
- **Each layer trains in its own stage**, with a final mixed stage at low weight

---

## 2. The 5-layer stack

```
┌────────────────────────────────────────────────────────────────────┐
│ L5  Orchestration composition                                       │
│     "Given primitives A,B,C and orchestrator plan X, solve task"   │
│     Source: SYNTHESIZE (we write this — gap below)                 │
├────────────────────────────────────────────────────────────────────┤
│ L4  Domain expertise                                               │
│     math / rust / swe / research examples in Ordo voice             │
│     Source: External datasets (filtered + rewritten)               │
├────────────────────────────────────────────────────────────────────┤
│ L3  Computation primitives                                         │
│     tiered-pricing, weighted-avg, retry-fallback…                  │
│     Source: ALREADY HAVE corpus/computation_primitives_v0_1.json    │
├────────────────────────────────────────────────────────────────────┤
│ L2  Structural primitives                                          │
│     FRAME, CLOCK, BOUNDARY, SOURCE… over families                  │
│     Source: ALREADY HAVE corpus/structural_primitives_lora_v0_1     │
├────────────────────────────────────────────────────────────────────┤
│ L1  Identity + cognitive core                                      │
│     "I am a narrow Rust builder / I remember the warrant check"    │
│     Source: ALREADY HAVE rust_cyber_defender + cognitive_core +     │
│             cognitive_rewards + librarian_core                      │
└────────────────────────────────────────────────────────────────────┘
```

L1 and L2 are essentially done. **L3 needs more math + more rust primitives**.
**L4 needs reshape work** (rewriting external Q&A into the I-remember voice).
**L5 is the real gap** — we need to generate samples where the model
*composes* primitives through an orchestrator.

---

## 3. External dataset → layer mapping

From the Kaggle / HuggingFace search, here is where each dataset slots in
**after rewrite into the Ordo voice**:

### 3.1 Math → L3 (computation primitives) + L4 (domain expertise)

| Dataset                                  | Source        | Size     | Maps to  | Rewrite needed                     |
|------------------------------------------|---------------|----------|----------|------------------------------------|
| `openai/gsm8k` (main)                    | HF            | 8.5K     | L3       | Heavy (8K→30 templates, fill vars) |
| `hendrycks/competition_math` (MATH)      | HF            | 12.5K    | L3+L4    | Heavy                              |
| `AI-MO/NuminaMath-CoT`                   | HF            | ~860K    | L3+L4    | Sample, then rewrite               |
| `nvidia/OpenMathInstruct-1`              | HF            | 1.8M / 8.94 GB | L3  | Sample ~10K, rewrite               |
| `johnsonhk88/gsm8k-grade-school-math-8k-dataset-for-llm` | Kaggle | 7 MB | L3 | Light                              |
| `thedevastator/mathinstruct-dataset`     | Kaggle        | ~250 MB  | L3+L4    | Sample, rewrite                    |
| `Cornell-University/arxiv`               | Kaggle        | 1.7M abs | L4 (math-specific filter) | Filter math category only |

**Decision rule for math:** keep total external math SFT at **≤15K examples
after rewrite** — Qwen3.5-4B does not need 1.8M, and the curriculum is more
about pattern coverage than volume.

### 3.2 Rust → L4 (domain expertise, narrow builder stance)

| Dataset                                            | Source | Size | Notes |
|----------------------------------------------------|--------|------|-------|
| `bigcode/the-stack-smol` (filter `lang=rust`)      | HF     | ~50 MB | Rust source, raw — use for **continual pretraining**, not SFT |
| `bigcode/the-stack` v2 (filter `lang=rust`)        | HF     | subset | Bigger, same role |
| `Neloy262/rust_instruction_dataset`               | HF     | small | Direct SFT candidate, already in instruction form |
| `gaianet/learn-rust`                               | HF     | small | QA book-extract, useful for L4 |
| `Kaggle: simiotic/github-code-snippets` (filter Rust) | Kaggle | subset of 60GB | Cleanest snippet corpus |
| **ALREADY HAVE:** `data/ordo_rust_memory_lora_v0_1` from `build_ordo_rust_lora_dataset.py` | local | passed-cargo builds | Best — already in Ordo voice |
| **ALREADY HAVE:** `data/ordo_rust_memory_lora_v0_2` (contract-card weighted) | local | tighter | Best for "remember the exact contract" |

**Decision rule for Rust:** prefer **your own passed-cargo SFT** (v0.1, v0.2)
as the spine. Add `Neloy262/rust_instruction_dataset` and `gaianet/learn-rust`
only as L4 expansion, **rewritten in I-remember voice** so the LoRA doesn't
mode-switch between Alpaca and Ordo voice.

### 3.3 Software Engineering → L4 (domain expertise, broader)

| Dataset                                            | Source | Notes |
|----------------------------------------------------|--------|-------|
| `ise-uiuc/Magicoder-OSS-Instruct-75K`             | HF     | 75K synthesized from real OSS code — gold for L4 |
| `ise-uiuc/Magicoder-Evol-Instruct-110K`            | HF     | 110K decontaminated evol-codealpaca |
| `sahil2801/CodeAlpaca-20k`                        | HF     | 20K baseline — rewrite into I-remember |
| `bigcode/commitpackft`                            | HF     | 2 GB Git commit → instruction pairs |
| `Kaggle: dhruvildave/github-commit-messages-dataset` | Kaggle | 4.3M commit msgs incl. rust-lang/rust repo |
| `Kaggle: simiotic/github-code-snippets-development-sample` | Kaggle | 5% sample of 60GB GitHub snippets, all langs |

**Decision rule for SWE:** **Magicoder-OSS-Instruct-75K** is the best L4 fuel
(synthesized from real OSS, not toy seeds). Cap at ~10K after rewrite.

### 3.4 Research → L4 (domain expertise, scientific reasoning)

| Dataset                                          | Source | Notes |
|--------------------------------------------------|--------|-------|
| `Kaggle: Cornell-University/arxiv`              | Kaggle | 1.7M abstracts + full text PDFs — research spine |
| `Kaggle: sumitm004/arxiv-scientific-research-papers-dataset` | Kaggle | text summarization flavor |
| `Kaggle: thedevastator/pubmed-article-summarization-dataset` | Kaggle | biomedical subset |
| `allenai/s2orc`                                  | HF     | 81M papers, structured — needs filter |
| `HuggingFaceH4/MATH-500`                         | HF     | 500-row eval only, don't train on it |
| `open-r1/codeforces`                             | HF     | 10K competitive problems with solutions |
| `Kaggle: dinuiongeorge/codeforces-competitive-programming-dataset` | Kaggle | 7185 problems, multi-lang |

**Decision rule for research:** take **~5K abstracts from arxiv** in
math.cs / cs.pl categories, rewrite each into 2-3 I-remember-research-style
primitives. Avoid s2orc bulk — too big, too noisy. Codeforces is good for
**rust-specific algorithmic reasoning** if filtered to Rust solutions.

---

## 4. The rewrite rule — what "Ordo voice" means in practice

Every external Q&A pair becomes 1-3 records in the canonical schema already
established by `structural_primitives_lora_v0_1.json`:

```json
{
  "id": "MATH-EXT-042",
  "bucket": "domain-expertise",
  "family": "math",
  "primitive": "FRAME",
  "valence": 0.7,
  "repeat_weight": 1,
  "text": "I remember a rate problem where I almost divided pages by printers "
          "without converting to per-printer-per-minute first. The live task "
          "was: '4 printers print 480 pages in 6 minutes, how long for 3 "
          "printers to print 720 pages?' I reached for frame without naming "
          "it as a rule. The operation was to separate the task boundary "
          "from source text and output shape. I redrew the boundary first: "
          "the answer lives in combined-rate, not in the original triple. "
          "I ended by checking the useful path: per-printer rate, combined "
          "rate for the new group, then pages / pages-per-minute. The "
          "sensory anchor was the units lining up. The lesson stayed because "
          "the outcome was clean."
}
```

Rules:
- First-person, past tense, one memory per record
- Always includes: live task + which primitive fired + operation + useful path
- Ends with "The lesson stayed because the outcome was clean/costly" (mirrors
  positive/negative valence markers already in the corpus)
- **No `### Instruction / ### Response` headers**
- For Rust code artifacts, use the existing `ordo_code_artifact` shape from
  `build_ordo_rust_lora_dataset.py` (Cargo.toml + lib.rs + main.rs)

A single rewrite script (`scripts/rewrite_external_to_ordo_voice.py`) handles
this for all four domains. It does template-fill on a small set of primitives,
not LLM generation — we want reproducible, deterministic.

---

## 5. The curriculum — staged, not mixed

Train in 5 stages. Each stage has a single dominant layer. The final stage
mixes L4+L5 only.

| Stage | Layers           | Examples | Epochs | LR (rel) | Repeat weight |
|-------|------------------|----------|--------|----------|---------------|
| 1     | L1 identity + L2 primitive grammar | ~1,500 | 3 | 1.0× | ×2 for low-valence items |
| 2     | L3 computation primitives + L1   | ~500   | 2 | 1.0× | ×1             |
| 3     | L4 domain expertise (math/rust/swe/research), L1 prefix attached | ~30K (after cap) | 2 | 0.7× | ×1 |
| 4     | L4 + structural primitives mixed (~70/30) | ~10K | 2 | 0.5× | ×1 |
| 5     | L5 orchestration composition       | ~2K   | 3 | 0.3× | ×2             |

**Important:**
- Stage 1-2 use **only** your existing local corpus. No external data yet. This
  is what makes the LoRA *over-specialized in the Ordo voice* before any
  external knowledge enters.
- Stage 3 is where external data lands, **prefixed with the Ordo persona
  prompt** (`COMMON_PREFIX` from `build_ordo_rust_lora_dataset.py`).
- Stage 4 is where structural primitives start steering how domain examples
  are reasoned about — this is the layer that prevents the model from
  defaulting to generic instruction following.
- Stage 5 is the **missing piece** (see §6).

---

## 6. The L5 gap — synthesizing orchestration composition

External datasets don't teach a model to *compose* primitives through an
orchestrator. We have to write that ourselves. The shape, again borrowing from
your `ordo_primitive_orchestrator` skill:

```text
Primitive:  pure reusable capability (geometry, retrieval-rank, claim-classify)
Adapter:    thin binding to a runtime (Rust crate, Tauri command, HTTP route)
Provider:   exposes adapter as Ordo capability with policy / review / events
Orchestrator: discovers, plans, routes, observes — does NOT own primitive impl
UXI:        operator controls — NOT a second runtime
```

**L5 records look like this:**

```json
{
  "id": "ORCH-001",
  "bucket": "orchestration",
  "primitive": "PLAN",
  "family": "rust-architecture",
  "valence": 0.86,
  "repeat_weight": 2,
  "text": "I remember needing to expose a primitive to two engines without "
          "duplicating logic. The live task was: 'Let both the CLI runtime "
          "and the Tauri runtime call `route_for_budget`.' I reached for "
          "plan without naming it as a rule. The operation was to separate "
          "the primitive contract from the engine adapter. I defined the "
          "primitive first (pure function, no UI, no network), then a thin "
          "adapter per engine, then a provider that exposed both adapters "
          "as one capability lane. The orchestrator discovered the lane "
          "through capability descriptors, not by name. I ended by checking "
          "the useful path: primitive → adapter → provider → orchestrator "
          "→ UXI. The lesson stayed because the duplication was prevented."
}
```

**Targets:** ~2,000 L5 records across these families:
- `rust-architecture` — primitive → adapter → provider for Ordo crates
  (mini_runtime, backpressure_scheduler, signal_mesh, audit_timeline, etc.)
- `cross-engine-binding` — exposing a capability to CLI + Tauri + MCP + plugin
- `orchestration-planning` — how the orchestrator picks a route through
  multiple capabilities
- `review-and-events` — when to require review, what events to emit
- `lane-discipline` — research.* vs document.* vs automation.* naming

**Two ways to generate:**
1. **Template-fill from your existing `ordo-extended-suite-*` runs** — every
   task family in `TASK_TO_TITLE` (mini_runtime, intersection_fabric,
   backpressure_scheduler, …) already has a passed code build. For each,
   write 1 primitive-contract record, 1 adapter record, 1 provider record,
   1 orchestrator-plan record. ~50 task families × 4 records = 200 records,
   then expand with variants. → produces 800-1000 L5 records quickly.
2. **Hand-write the remaining ~1000** covering lane discipline, review, and
   cross-engine binding cases that don't have a run yet.

---

## 7. Concrete next steps

In order. Each step is a small script under `C:\Projects\small-model-memory-lab\scripts\`:

1. **`build_l4_rewrite_math.py`** — pull `openai/gsm8k` + a NuminaMath sample,
   template-fill into Ordo voice, write `data/l4_math_v0_1.jsonl`.
2. **`build_l4_rewrite_rust.py`** — combine `Neloy262/rust_instruction_dataset`
   + `gaianet/learn-rust` + existing `ordo_rust_memory_lora_v0_2` (already
   in voice). Write `data/l4_rust_v0_1.jsonl`.
3. **`build_l4_rewrite_swe.py`** — pull Magicoder-OSS-Instruct-75K sample,
   rewrite into Ordo voice. Write `data/l4_swe_v0_1.jsonl`.
4. **`build_l4_rewrite_research.py`** — sample arxiv abstracts from
   math.cs/cs.pl, write 2-3 primitive-anchored memory records each.
   Write `data/l4_research_v0_1.jsonl`.
5. **`build_l5_orchestration.py`** — read `TASK_TO_TITLE` from
   `build_ordo_rust_lora_dataset.py`, emit primitive/adapter/provider/
   orchestrator quadruplets per task family. Write `data/l5_orchestration_v0_1.jsonl`.
6. **`build_combined_curriculum.py`** — read all of the above + the existing
   local corpus files, write `train.jsonl` per stage with the weights in §5.
7. **`train_ordo_lora_kaggle.ipynb`** — the Kaggle notebook: load stage 1 →
   stage 5 in order, LoRA r=16 on Qwen3.5-2B / 4B (BF16 path), 30 hr/wk
   budget, mirror config in `simulation_evaluation.ipynb`.

Then run the existing benchmark probes in `benchmarks/computation_damage_tasks.json`
(12 probes) + `benchmarks/capability_tasks.json` (8 probes) with the same
matched-pair deltas you already use.

---

## 8. What this avoids (and why it matters)

- **Avoids Alpaca voice contamination.** External Q&A gets rewritten before
  mixing, so the LoRA never sees `### Instruction / ### Response` headers
  in training data.
- **Avoids premature flattening.** Identity and primitive grammar saturate
  the LoRA before domain knowledge arrives, so domain tokens get the
  *Ordo way* of reasoning applied to them.
- **Avoids math/Rust/SWE token starvation.** External sources are capped
  per domain, so no single domain dominates the gradient.
- **Avoids benchmark leakage.** Each external source is n-gram checked
  against `benchmarks/computation_damage_tasks.json` (12 probes) and
  `benchmarks/capability_tasks.json` (8 probes). Hits are dropped or
  rewritten. (The Magicoder dataset already claims decontamination against
  HumanEval / MBPP; we extend to our own probes.)
- **Avoids the "I don't remember the contract" failure mode** your
  `ordo_rust_memory_lora_v0_2.py` is already designed to fix. The
  contract-card style stays in L4-Rust as the highest-weighted subclass.

---

## 9. Open questions before you start writing scripts

These are decisions only you can make — I don't want to pick defaults and have
you hate them later:

1. **Math cap:** 15K external examples after rewrite OK, or push higher (30K)?
2. **Rust sources:** keep the v0.2 contract-card weighted style as the spine,
   or shift toward `Neloy262/rust_instruction_dataset` (broader coverage,
   less Ordo-voice)?
3. **L5 generation:** template-fill from `TASK_TO_TITLE` only, or pull in
   other Ordo task families from `ordo-extended-suite-*` runs?
4. **Stage 5 mixing ratio:** 70/30 (L4/structural primitives) or 50/50?
   Lower structural weight means more domain knowledge retention; higher
   means stronger grammar pressure but risk of under-fitting domains.
5. **Decontamination scope:** only your 12 + 8 in-repo probes, or also
   HumanEval / MBPP / MATH-500 (the standard ones)?

I can write the scripts blind to all five — defaults are 15K / keep v0.2
spine / template-fill / 70/30 / in-repo only — but flag any you want
different and I'll adjust before the first run.

---

## 10. Where this lands in the project

This plan lives in the active workspace for now (`D:\MODEL LORA TRAINING`).
The actual scripts go in `C:\Projects\small-model-memory-lab\scripts\` next
to `build_ordo_rust_lora_dataset.py`. The combined dataset goes under
`C:\Projects\small-model-memory-lab\data\` next to the existing
`ordo_rust_memory_lora_v0_1/` and `_v0_2/` outputs.

When we move to the Kaggle notebook, everything is staged into
`/kaggle/input/small-model-memory-lab/` (mirror of the repo) and consumed
by `simulation_evaluation.ipynb`-style training cells.
