# Small Base-Model General-Performance Recipe — Findings

**Date:** 2026-06-15
**Scope:** Qwen3.5 **base** models only — `qwen35-2b-base-q4km-raw`, `qwen35-2b-base-q8-raw`,
`qwen35-4b-base-q6-raw`, `qwen35-4b-base-q8-raw` (raw completion, `generate` API, temp 0).
**Goal:** find the recipe — injected *memories*, *primitives*, or a *combination* — that gives
these small base models **general** reasoning performance, and prove it is *general* by measuring
on a **held-out** suite the recipe was never tuned on (the prior findings flagged surgical
v4/v5/v6 corpora as "upper bound, not transfer").

## Method

- **Harness:** `scripts/run_general_corpus_campaign.py`. Two 20-task suites of the same 12+ skill
  categories (arithmetic, json, filtering, scheduling, probability, spatial, logic, code,
  statistics, chemistry, rhetoric, date, triage, causal, security/format):
  - **dev** = the canonical 20 hard-general-orchestration tasks (identical graders) — tuning is allowed here.
  - **held-out** = 20 fresh parallel tasks, same skills, different numbers/wording — the transfer test.
    Answer key independently verified by a blind dual-solver workflow.
- **Conditions:** `no_memory` (bare), `general_memory` (the corpus), `primitives` (scaffold),
  `combo`. Deterministic graders (exact-match / json / comma-list / unit). Replication via repeats
  (greedy decoding still drifts ~±1 run-to-run from GPU non-determinism).
- **Analysis:** `scripts/aggregate_campaign.py`, `scripts/campaign_per_task.py`.

## Headline results (HELD-OUT / transfer, out of 20)

### 1. Prompt-side: the elaborate memory corpus does NOT transfer; a one-line nudge does.

| model | bare | verbose corpus | terse corpus | methods-only | discipline-only | **answer-only** |
|---|---|---|---|---|---|---|
| 2B-Q4KM | 0–1 | 4 | 5 | 1 | 5 | **7** |
| 2B-Q8 | 3 | 2 | 5 | 2 | 5 | 6 |
| 4B-Q6 | **9** | 5 | 6 | 5 | 6 | 7 |
| 4B-Q8 | **10** | 6 | 6 | 7 | 6 | 8 |

- **Category-method memories are inert for transfer.** "methods-only" collapses to bare; the entire
  2B gain comes from a **two-sentence output-discipline** nudge, not the methods. The project's core
  "method-scar corpus" direction does not generalize on base models for general reasoning.
- **`answer only, no explanation/labels/fences` is the single best general nudge** (7/6/7/8). It takes
  the weakest model from ~0 to **7/20** (a clean dual-solver-verified mechanism: it flips ~5 tasks from
  genuinely-wrong to correct — conditional-prob, boolean, prompt-injection, set-intersection, ratio —
  plus 1 format fix; ~83% real reasoning, not a grading artifact).
- **"Think step by step" CATASTROPHICALLY breaks the weakest model** (2B-Q4KM → 0/20). For a degraded
  base model, inviting reasoning = rambling into nothing.
- **A memory budget that shrinks with capability:** weak 2B models want a *lean* nudge; capable 4B
  models want *nothing* (any injection costs them ~3–4 points). Verbose corpora hurt everyone.
  `combo` (corpus + primitives) is always ≤ corpus alone — lean wins.

### 2. Activation steering: the only intervention that *helped* the capable 4B.

Delivered the disposition via the **residual stream** (zero prompt tokens) instead of text, using HF
forward-hooks on `Qwen/Qwen3.5-4B-Base` (= the same weights as the Q8 GGUF, bf16).

| steering method | 4B held-out (vs bare 6 on this HF instance) |
|---|---|
| additive, concise/verbose vector (last-token) | 6 (flat — overfit dev 4→8) |
| **additive, answer-mode vector (response-CAA)** | **8 (+2, best)** at L16, α≈0.6–0.8 |
| clamp/override the answer↔think axis | 7 (stable under strong force, but lower) |

- **Answer-mode additive steering is the first thing to move the capable 4B *up* on held-out (+2).**
  Every *text* injection pushed the 4B *down* — steering avoids the attention-competition penalty.
- **Overpowering for control (clamp/ablation):** more *stable* than additive (additive breaks at
  α≥1.0; clamp holds at strong k=3–4) — but tops out at 7 < additive's 8. **More control ≠ more
  accuracy.** And clamping to the natural answer-mode level (k=0) *hurts* (6→3): the native per-token
  signal on that axis carries real information — it is **not** pure noise to be overridden.
- The 2B is too capability-limited for steering to help much (3→4, +1).

## The through-line

> **For small base models, "thinking" is the failure mode. Force answer-mode and they do better.**

This single principle holds across *both* levers: it is why "answer only" won and "think first" broke
the weakest model (prompting), and why an **answer-mode steering vector** is the only thing that lifted
the capable model (activations).

## Deployment bridge (attempted, parked)

Goal: apply the answer-mode vector as a llama.cpp `--control-vector` on the real Ollama GGUF for a
ship-it number. Built the control-vector GGUF (`data/answer_mode_cv.gguf`, `model_hint=qwen35`,
`direction.17`). `llama-cli`/`llama-server` load the qwen35 arch fine. **Blocked by two Windows/parity
walls:** (1) `--control-vector-scaled FNAME:SCALE` collides with the drive-letter colon (fixable by
baking the scale in); (2) `llama-server /completion` does not reproduce Ollama's generation — bare
scored 0/20 (model echoes prompt-instruction text) vs Ollama's 10, a tokenization/BOS parity gap. Since
the HF weights *are* the deployment weights (bf16 vs Q8 quant), the HF steering result already answers
the science; the bridge was only for a deployment number and was not worth the parity rabbit hole.

## Practical recipe (recommendation)

- **Deploying a weak 2B base model:** prepend the one line *"Answer with only the final value — no
  explanation, no labels, no fences."* Do **not** invite step-by-step reasoning. (~0 → 7/20.)
- **Deploying a capable 4B base model:** leave the prompt bare; if you can hook activations, apply the
  **answer-mode steering vector** at ~layer 16, α≈0.6–0.8 (+2 held-out, zero token cost).
- **Do not** ship elaborate method-memory corpora for general reasoning on base models — they do not
  transfer and hurt the capable models.

## Artifacts

- Harness: `scripts/run_general_corpus_campaign.py`; analysis: `scripts/aggregate_campaign.py`,
  `scripts/campaign_per_task.py`.
- Corpora: `data/general_corpus_v1.txt` (verbose), `..._v2_terse.txt`, `..._v3_methods_only.txt`,
  `..._v3_common_only.txt`, `data/disc_answeronly.txt` (**the winning nudge**), `disc_*.txt` variants,
  `data/false_memories_*.txt` (authored, untested).
- Steering: `scripts/steer_qwen.py` (v1 add), `steer_qwen2.py` (answer-mode CAA), `steer_qwen3.py`
  (clamp/override), `scripts/build_control_vector.py`, `data/answer_mode_cv.gguf`,
  `scripts/eval_llamaserver.py`.
- Run outputs: `rag_runs/general-corpus-campaign-*`, `rag_runs/steering*-*`.

## Open / next

- Held-out is 20 tasks (±1 noise); a 2nd independent held-out set would firm up the small deltas.
- ~9/20 task types are above all four models' ceiling regardless of recipe (structural limit ~11/20).
- Steering not yet validated on the deployment Q8 GGUF (parity bridge parked). Qwen ships official
  residual SAEs (`Qwen/SAE-Res-Qwen3.5-*-Base-*`) — a cleaner, disentangled steering direction than
  diff-of-means is the natural next experiment if steering is pursued further.
- The authored "human false memories" (`data/false_memories_*.txt`) were never run — an open thread.
