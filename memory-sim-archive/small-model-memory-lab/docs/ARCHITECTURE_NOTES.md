# Architecture Notes

## Vocabulary

### Synthetic Memory

A synthetic memory is an episodic text record written as if the model had lived through an experience.

Good memory shape:

- first person
- specific situation
- reasoning
- action
- outcome
- emotional or consequence texture when useful

Bad memory shape:

- generic instruction
- resume-like capability claim
- abstract rule without scenario
- broad advice that competes with the task

In this lab, body memories worked better than title labels for Q4 arithmetic repair.

### Primitive

A primitive is a reusable cognitive operation.

Examples:

- convert minutes to hours plus leftover minutes
- compute pages per printer per minute
- count total ratio parts before liters
- recover old total from old average times old count
- subtract credit exactly once

Primitives can be delivered as:

- prompt text
- episodic memory bodies
- compact cues
- cache prefix
- residual-stream vectors
- verifier checks

### Transport Layer

The transport layer is how the primitive reaches the model.

Observed transport modes:

- plain prompt/RAG
- answer-shape scaffold
- compact cue
- strong instruction-like activation
- llama.cpp prefix cache
- residual-stream activation steering

Main finding:

Transport can be as important as content. Stronger wording is not always stronger behavior. For Q4 arithmetic, soft body memories plus a light scaffold worked better than direct instruction-like activation.

### Structural Primitive Layer

The structural primitive layer is a proposed layer that reinforces the primitives themselves.

Examples:

- `FRAME`: keep the task/source/output boundary intact
- `CLOCK`: keep steps in order and resist cascade
- `CARRIER`: keep one task-intent active through the answer
- `PARITY` and `CHECKSUM`: detect missing or corrupted operations
- `ECC`: repair missing operations before final output
- `REDUNDANCY`: encode the same operation through multiple stable cues
- `SUPPRESSION`: keep distractors from becoming active operations
- `COMPRESSION`: keep the primitive packet short enough to avoid attention clutter
- `STATE_BUFFER`: preserve givens and intermediate values
- `GAIN_CLAMP`: prevent overactivation from becoming noise

Simulation result:

- `runs/q2-primitive-self-reinforcement`
- refined no-structure `lora_candidate` baseline: `52.0%`
- refined best structural layer: `87.1%`
- cascade dropped from `22.2%` to `0.1%`

Current interpretation:

- `FRAME`, `CLOCK`, `CARRIER`, and `STATE_BUFFER` are core stabilizers.
- `CHECKSUM`, `PARITY`, and `ECC` are most useful as a group.
- `ROUTER` was negative on average in this sim because wrong routing creates coherent failure.
- for LoRA, structural primitive memories should be trained compactly and tested against overbuilt variants.

### KV / Prefix Cache

In the llama.cpp test, primitives were cached as prompt-prefix tokens.

What worked:

- the server reused cached prefix tokens
- universal primitive cache improved raw completion from `29/67` to `36/67`

What did not work yet:

- invisible memory state where the task is appended to hidden KV without the prefix being part of the request
- arbitrary K/V tensor injection

Precise wording:

- "prefix-cache primitive injection" is accurate
- "direct KV tensor injection" is not yet achieved in this lab

### Activation Steering

Activation steering treats a primitive as a vector in activation space.

Workflow:

1. collect positive examples
2. collect negative examples
3. run examples through a HF/PyTorch model
4. capture activations at a layer
5. compute positive mean minus negative mean
6. add `strength * vector` to hidden states during generation

This is closer to the user's "hook into the model and manipulate signals" idea than prompt/RAG memory.

Current result:

- Qwen3 1.7B Base baseline: `38/67`
- best steered: `43/67`
- useful layers: 21-22
- destructive layers: 7 and often 14

## Layered System View

The current architecture can be thought of as a stack:

```text
Task
  ↓
Retriever / Router
  ↓
Synthetic Memory Substrate
  ↓
Cognitive Primitives
  ↓
Transport Layer
  ↓
Model Runtime
  ↓
Verifier / Repair Loop
  ↓
Answer
```

Current implemented pieces:

- retriever/router: cosine retrieval over corpus embeddings
- synthetic memory substrate: JSON corpora
- cognitive primitives: computation corpus and primitive packets
- transport layer: prompt/RAG, answer-shape cues, prefix cache, residual steering
- model runtime: LM Studio, llama.cpp, PyTorch/Transformers

Not fully implemented yet:

- verifier/repair loop
- true hidden KV continuation
- composed activation vectors
- adaptive per-task layer/strength selection

## Why Some Memories Helped And Some Hurt

The arithmetic benchmark exposed three distinct effects:

1. Helpful substrate

Compute-specific body memories gave the model a usable procedural path.

2. Attention clutter

Too many or too broad memories made the model attend to the wrong thing.

3. Token contamination

Negative memories containing bad answer strings could cause the model to emit the bad answer.

This suggests memory content should be optimized for:

- relevance
- specificity
- low contamination risk
- visible intermediate artifacts
- minimal cross-task distraction

## Current Best Hybrid Hypothesis

The likely path to large gains is not one mechanism alone.

Candidate hybrid:

1. Use memory retrieval for domain-specific episodic substrate.
2. Use short primitive cache packets for stable procedural prior.
3. Use residual steering to nudge careful-execution activations in late layers.
4. Use verifier/repair to catch missing artifacts.

Example:

```text
Computation task
  ↓
Retrieve only math/rate/clock memories
  ↓
Prefill or include one short primitive packet
  ↓
Apply late-layer careful-arithmetic steering vector
  ↓
Generate answer
  ↓
Verifier checks:
    - final answer present?
    - units present?
    - intermediate values present?
    - known failure class triggered?
  ↓
Repair only missing artifacts
```

## Research Questions To Preserve

1. Do memory bodies outperform compact cues because they activate richer latent associations?
2. Does Q4 benefit more than Q8 because memories compensate for quantization damage?
3. Can negative-outcome memories be written without contaminating the output with bad tokens?
4. Can activation steering and memory prompting compose constructively?
5. Can KV/prefix cache reduce prompt-format noise enough to improve stability?
6. Is a verifier/repair loop the missing piece for near-perfect scores?
7. Can task-family-specific steering vectors outperform one generic careful-arithmetic vector?

## Publication Cautions

Before making external claims:

- run held-out tasks
- repeat each condition multiple times
- separate simulation from real model results
- compare against instruct baselines
- report failures and regressions
- avoid claiming "training" when the method is inference-time steering/prompting
- distinguish prefix cache from true KV tensor manipulation
