# Small Model Memory Lab Experiment Report

Date: 2026-06-04

## Scope

This lab studies whether small base models can be made more capable in narrow domains using external cognitive support instead of fine-tuning.

This project is intentionally separate from:

- the Mother behavioral memory experiment
- the Rust developer/synthetic memory specialist experiment
- prompt-injection-only memory robustness experiments

The active target here is small-model capability improvement, especially arithmetic/computation behavior.

## Core Thesis

Synthetic memories and primitives can act as an external capability substrate for small or damaged models, but the delivery mechanism matters. The same content can help, hurt, or collapse behavior depending on whether it arrives as title cues, episodic memory bodies, broad prompt instructions, KV/prefix cache, or residual-stream steering.

Current strongest finding:

> Human-style episodic computation memories plus a light output scaffold lifted Qwen3.5 2B Q4_K_M from about `50/67` to `58/67` on the computation suite, and repeated at `51/67` to `58/67`.

Current deeper-control finding:

> Residual-stream activation steering on Qwen3 1.7B Base improved the computation suite from `38/67` to `43/67` at late decoder layers.

## Environment

Primary workstation:

- Windows
- Intel i7-13700K
- 96GB RAM
- NVIDIA RTX 5070 Ti, 16GB VRAM, Blackwell / SM 12.0

Important runtime notes:

- LM Studio is useful for GGUF chat/completion testing.
- Ollama is useful for embedding and normal local inference, but not direct residual hooks.
- llama.cpp exposes prompt/prefix cache behavior, slots, and raw completion.
- PyTorch/Transformers is required for residual-stream activation steering.

Python stack verified:

- PyTorch `2.10.0+cu128`
- Transformers `5.3.0`
- Accelerate `1.12.0`
- Safetensors `0.7.0`

## Models Tested

### GGUF / LM Studio

- `qwen3.5-2b-base@q4_k_m`
- `qwen3.5-2b-base@q8_0`
- `qwen3.5-2b-base-i1` was present but not central to these documented runs

### GGUF / llama.cpp

- `Qwen3.5-2B-Base.Q4_K_M.gguf`
- Used through `llama-server` raw `/completion` endpoint on port `18081`.

### HuggingFace / PyTorch

- `Qwen/Qwen3-1.7B-Base`
- Used for residual-stream steering because it is a plain `Qwen3ForCausalLM` with 28 decoder layers.

Not used for residual steering yet:

- `Qwen/Qwen3.5-2B-Base`
- Reason: HF architecture is `Qwen3_5ForConditionalGeneration`, not a straightforward text-only causal model hook path.

## Benchmarks

### Capability Tasks

File: `benchmarks/capability_tasks.json`

8 tasks, 38 criteria.

Designed for generalist/librarian capability:

- unknown domain orientation
- loaded argument analysis
- word problem units
- spatial bottleneck
- debugging/reproduction
- option comparison
- source boundary
- uncertainty-aware summarization

### Computation Damage Tasks

File: `benchmarks/computation_damage_tasks.json`

12 tasks, 67 criteria.

Designed to expose arithmetic/reasoning damage:

- printer rate
- discount then tax
- weighted average
- inventory ledger
- schedule finish time
- ratio mixture
- average speed
- linear equation
- probability without replacement
- ticket table
- subscription total
- tiered pricing

## Corpora

### `cognitive_core_v0_1.json`

General cognitive memories:

- orientation
- question analysis
- evidence separation
- rhetoric
- analogy bounds
- decomposition
- math habits
- spatial reasoning
- debugging
- planning
- calibration

### `cognitive_rewards_v0_1.json`

Reward/consequence memories for successful behavior and calibration.

### `librarian_core_v0_1.json`

Kate, the librarian persona core.

Purpose:

- provide a generalist connective layer
- organize domain knowledge
- preserve provenance
- route between thin domains
- prevent overbroad memory activation

Finding:

- all-fire Kate overloads tiny models
- top/routed Kate memory works better than full activation

### `computation_primitives_v0_1.json`

Compute-only episodic memories.

Purpose:

- repair arithmetic execution
- preserve intermediate values
- carry units
- avoid rate, clock, ratio, credit, and tier errors

Important finding:

- body memories worked
- title-only memories hurt
- broad format-control memories hurt
- negative-outcome memories can contaminate outputs if they preserve the exact bad answer token

## Major Results

### Capability Suite: Qwen3.5 2B Q4 vs Q8

Runtime: LM Studio chat API.

Prompt shape:

- no answer-shape cue
- memory retrieval enabled for memory mode

| Model | Memories | Score | Avg Rate |
|---|---:|---:|---:|
| Qwen3.5 2B Q4_K_M | No | `26/38` | `66.7%` |
| Qwen3.5 2B Q4_K_M | Yes | `31/38` | `80.0%` |
| Qwen3.5 2B Q8_0 | No | `23/38` | `60.2%` |
| Qwen3.5 2B Q8_0 | Yes | `28/38` | `73.5%` |

Interpretation:

- memories helped both quants by `+5` criteria
- Q4 outscored Q8 on this small suite, likely due to variance/runtime/prompt effects
- do not overclaim Q4 > Q8 from this suite alone

### Computation Suite: General Memories Hurt

Runtime: LM Studio chat API.

| Model | Memories | Score | Avg Rate |
|---|---:|---:|---:|
| Qwen3.5 2B Q4_K_M | No | `44/67` | `65.8%` |
| Qwen3.5 2B Q4_K_M | Yes, old all-corpus | `39/67` | `57.8%` |
| Qwen3.5 2B Q8_0 | No | `48/67` | `71.7%` |
| Qwen3.5 2B Q8_0 | Yes, old all-corpus | `46/67` | `68.3%` |

Interpretation:

- generalist/librarian memories helped broad tasks but hurt arithmetic
- compute tasks need a domain-specific memory substrate
- broad rhetorical/general memories add attention noise for numeric execution

### Computation Suite: Compute-Only Body Memories Worked

Runtime: LM Studio chat API.

Best recipe:

- model: `qwen3.5-2b-base@q4_k_m`
- corpus: `computation_primitives_v0_1.json`
- cue mode: `Body`
- TopK: `3`
- answer-shape scaffold enabled

| Setup | Baseline | Memory | Lift |
|---|---:|---:|---:|
| old all-corpus, title cues, no answer shape | `44/67` | `39/67` | `-5` |
| compute-only titles + answer shape | `50/67` | `47/67` | `-3` |
| compute-only bodies + answer shape, TopK 3 | `50/67` | `58/67` | `+8` |
| repeat same best setup | `51/67` | `58/67` | `+7` |

Q8 comparison:

| Model | Setup | Baseline | Memory | Lift |
|---|---|---:|---:|---:|
| Qwen3.5 2B Q8_0 | compute-only bodies + answer shape, TopK 3 | `55/67` | `54/67` | `-1` |

Interpretation:

- Q4 benefited more than Q8
- this supports the idea that memories help most when a capability is degraded but not fully collapsed
- Q8 may already carry more computation ability in weights, so the same memory payload adds less value or noise

### Memory Tuning Plateau

Best plateau:

- about `58-59/67` on Qwen3.5 2B Q4_K_M

Tried:

- more targeted correction memories
- negative-outcome memories
- TopK 1, 2, 3, 4, 5
- body, title, hybrid, compact cues
- stronger activation prompts
- stricter math answer prompts

Important failures:

- TopK 1 was too brittle
- TopK 5 added attention clutter
- hybrid title + body was worse
- strong instruction-like activation was worse
- compact primitive cues were worse
- over-tight math prompt was worse

Negative memory lesson:

If a memory says "I once answered X and it was wrong," the model may still emit `X`.

Better pattern:

- describe the failure class
- describe the correction
- avoid preserving the exact bad answer token unless the model is strong enough not to copy it

### Primitive Cache / KV Prefix Test

Runtime: llama.cpp raw `/completion`.

This is not the same runtime as LM Studio chat mode, so absolute scores are lower.

Goal:

- test whether compute primitives can sit in cached prefix/KV state

Clean run:

| Mode | Score | Avg Cached Tokens |
|---|---:|---:|
| baseline raw completion | `29/67` | `0.0` |
| universal primitive cache | `36/67` | `244.0` |
| routed primitive cache | `34/67` | `96.3` |

Interpretation:

- llama.cpp prefix cache worked mechanically
- cached primitive prefix improved raw completion by `+7`
- broad primitive packets can dominate or confuse raw base completion
- true invisible memory/KV continuation was not achieved with stock llama-server

Boundary lesson:

Primitive cache needs a transport layer:

- closed block boundaries
- short packets
- task routing
- maybe verifier/repair instead of one big universal prefix

### Residual-Stream Activation Steering

Runtime: HuggingFace/PyTorch.

Model:

- `Qwen/Qwen3-1.7B-Base`

Why this model:

- plain `Qwen3ForCausalLM`
- exposes decoder layers cleanly
- suitable for residual hooks

Steering vector:

- positive examples: careful arithmetic, visible intermediates, units, clock conversion, ratios, money signs, without-replacement counts
- negative examples: rushed arithmetic, skipped intermediates, dropped units, bad clock/rate/credit/probability habits
- operation: `hidden = hidden + strength * vector`

Wide sweep:

| Mode | Score |
|---|---:|
| baseline | `38/67` |
| layer 7, strength 0.5 | `12/67` |
| layer 14, strength 0.5 | `33/67` |
| layer 21, strength 0.5 | `43/67` |

Tight late-layer sweep:

| Layer | Strength | Score |
|---:|---:|---:|
| baseline | n/a | `38/67` |
| 20 | 0.25 | `38/67` |
| 20 | 0.5 | `39/67` |
| 20 | 0.75 | `40/67` |
| 21 | 0.25 | `42/67` |
| 21 | 0.5 | `43/67` |
| 21 | 0.75 | `36/67` |
| 22 | 0.25 | `41/67` |
| 22 | 0.5 | `40/67` |
| 22 | 0.75 | `43/67` |

Interpretation:

- activation steering is a real control surface
- late layers around 21-22 helped
- early steering was destructive
- strength matters; too high can collapse performance
- first vector is modest, but it proves the method

## Current Best Scores By Method

| Method | Runtime | Model | Best Score |
|---|---|---|---:|
| baseline, capability suite | LM Studio | Qwen3.5 2B Q4_K_M | `26/38` |
| memory, capability suite | LM Studio | Qwen3.5 2B Q4_K_M | `31/38` |
| baseline, computation suite | LM Studio | Qwen3.5 2B Q4_K_M | `51/67` repeat baseline near best run |
| compute body memories | LM Studio | Qwen3.5 2B Q4_K_M | `58/67` |
| primitive prefix cache | llama.cpp raw completion | Qwen3.5 2B Q4_K_M | `36/67` |
| activation steering | PyTorch/Transformers | Qwen3 1.7B Base | `43/67` |

Do not directly compare across runtimes as if they are identical. LM Studio chat mode, llama.cpp raw completion, and PyTorch generation have different templates and decoding behavior.

## Main Lessons

1. Corpus specificity matters.

Generalist memories can help broad reasoning and hurt arithmetic. Computation needs its own memories.

2. Human-style episodic bodies beat labels.

Title-only cues did not repair Q4 arithmetic. Episodic bodies did.

3. Memory delivery is fragile.

The same content can help or hurt depending on cue mode, TopK, prompt texture, and transport boundary.

4. Compressed models may benefit more.

Q4_K_M improved from compute memories where Q8_0 did not. This suggests external memories may compensate for quantization damage when the underlying circuit is degraded but still present.

5. Negative memories are double-edged.

Bad-answer tokens can become anchors. Consequence memories need careful wording.

6. KV/prefix cache works, but stock llama.cpp is not true hidden memory injection.

Prefix cache can reuse primitives, but the primitive text still exists as prefix context. True invisible KV memory needs a lower-level session/state or custom runtime path.

7. Residual steering is the deeper control surface.

Activation steering treats a primitive as a vector, not text. It worked modestly on Qwen3 1.7B Base and deserves the next serious iteration.

## Next Experimental Branches

### Branch A: Improve Compute Memory Plateau

Goal:

- move Qwen3.5 2B Q4_K_M from `58-59/67` toward `65+/67`

Approach:

- build task-family-specific memory chains
- add verifier/repair pass for missing artifacts
- avoid bad-answer contamination
- tune retrieval by family rather than generic TopK

### Branch B: KV / Cache Primitives

Goal:

- test whether cached primitives can become stable prior context

Approach:

- shorter primitive packets
- stronger boundaries
- separate slot/state handling
- compare visible prefix cache vs lower-level state continuation

### Branch C: Activation Steering

Goal:

- build better residual vectors and test composition

Approach:

- larger contrastive datasets
- separate vectors for rate, clock, ledger, ratio, money, probability
- sweep late layers only
- test vector composition
- compare steering-only vs steering + memory prompt

### Branch D: Hybrid Neural Exoskeleton

Goal:

- combine the methods

Candidate architecture:

- memory/RAG selects relevant episodic substrate
- primitive cache provides stable procedural prior
- residual steering nudges internal state
- verifier/repair loop catches missing artifacts

This is closest to the "neural exoskeleton" idea.
