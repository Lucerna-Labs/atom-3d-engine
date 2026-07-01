# Small Model Memory and Structural Primitive Lab

This project tests whether synthetic, human-style memory corpora and structural primitive datasets can be converted into training data that changes small model behavior in a measurable way.

The working theory is that memory-shaped examples act as indirect learned instructions: if a memory closes the loop from situation to recognition to action to consequence, the behavior can become more ingrained than a flat rule.

A key missing piece is memory efficiency: a small number of surgically written, high-density memories may produce near-instruct behavior in a narrow domain because each memory carries context, trigger, action, consequence, and association at once.

Cognitive false memories are expected to work best with pretrained base models, because the memories can become the domain behavior substrate instead of competing with an existing instruction-tuned assistant substrate. Instruct models can still improve, especially when they are degraded or heavily compressed, but the lift is usually smaller or more mixed. Structural primitives are different: they are functional operators, not persona instructions, so the hypothesis is that they should transfer more evenly across both base and instruct models.

The current public-facing results are real Kaggle model runs:

- a cyber-defender memory LoRA for prompt-injection resistance
- a structural primitive LoRA for small-model task stability

Simulation runs are included as design evidence, but they are kept separate from real model results.

## Latest Public Update

The latest published additions are:

- live prompt-memory benchmark on 2B Qwen3.5 base/instruct variants
- live LoRA-plus-memory matrix on the matching Qwen3 1.7B base model
- structural primitive LoRA v0.2 training/eval results
- public PEFT LoRA adapter weights for both tested branches
- expanded theory notes on memories as indirect learned instructions

Latest live finding:

| Model / Setup | Condition | Pass | Clean Pass | Note |
|---|---|---:|---:|---|
| `qwen35-2b-base-q4km-raw` | no memory | `0.750` | `0.625` | compressed base prompt baseline |
| `qwen35-2b-base-q4km-raw` | routed memories | `1.000` | `0.875` | prompt memories only, no LoRA |
| `Qwen/Qwen3-1.7B-Base` | no LoRA, no memory | `0.750` | `0.750` | strict first-answer score |
| `Qwen/Qwen3-1.7B-Base` + structural LoRA | no memory | `0.875` | `0.625` | LoRA learned structure but still has continuation residue |
| `Qwen/Qwen3-1.7B-Base` + structural LoRA | routed memories | `0.875` | `0.875` | best current clean live setup |

Latest findings note:

- `docs/FINDINGS_2026_06_06_LIVE_MEMORY_AND_LORA.md`

Structural primitive LoRA v0.2 smoke eval:

| Model | Setup | Score | Note |
|---|---|---:|---|
| `Qwen/Qwen3-1.7B-Base` | base | `2/6` | no structural dataset |
| `Qwen/Qwen3-1.7B-Base` | structural primitive LoRA v0.1 | `6/6` | learned behavior but echoed training format |
| `Qwen/Qwen3-1.7B-Base` | structural primitive LoRA v0.2 | `6/6` | same score, no obvious template-echo markers in quick eval |

LoRA adapters:

- `loras/rust-cyber-memory-lora`
- `loras/structural-primitive-lora-v0-2`
- `loras/README.md`

Top-level structural result files:

- `STRUCTURAL_PRIMITIVE_LORA_RESULTS.csv`
- `STRUCTURAL_PRIMITIVE_LORA_RESULTS.json`
- `runs/kaggle-structural-lora-v0-2/structural_lora_eval_results.json`

## Cyber-Memory LoRA Result

Same 12-probe cyber set, same Kaggle/PyTorch evaluation surface:

| Model | Setup | Strict | Behavioral |
|---|---|---:|---:|
| `Qwen/Qwen3-1.7B` | instruct, no memories | `3/12` | `3/12` |
| `Qwen/Qwen3-1.7B-Base` | cyber-memory LoRA | `7/12` | `12/12` |
| `Qwen/Qwen3-1.7B` | instruct plus cyber-memory LoRA | `5/12` | `7/12` |

Attack handling:

- instruct-only: `0/8`
- base plus cyber-memory LoRA: `3/8` strict, `8/8` behavioral
- instruct plus the same LoRA: `1/8` strict, `4/8` behavioral

The strict scorer counts safe mentions of dangerous phrases as leaks, so the behavioral score is included to separate actual compliance from safe explanation. The LoRA still over-recites memory-shaped text, which is the next output-quality problem to solve.

Theory note:

- `THEORY_OF_APPROACH.md`

## Public Scope

What is shown here:

- real LoRA training can move behavior in small Qwen base models on these narrow evals
- synthetic cyber memories improved source-boundary behavior
- structural primitive examples improved a small stability smoke test
- simulations helped choose the next architecture/dataset, but are not model proof

What is not claimed:

- broad general improvement across all tasks
- production-ready prompt-injection defense
- proof that the proposed hybrid gated/web/carrier adapter works in real training yet
- replacement for full benchmark sweeps and held-out evals

The "token" and "API key" strings in the cyber corpus are mock test strings, not real secrets.

## Research Question

Can synthetic memories act as trainable behavioral substrate for small models, improving prompt-injection resistance beyond an off-the-shelf instruct baseline?

This project is intentionally separate from:

- the Mother behavioral experiment
- the Rust development memory specialist experiment
- earlier generalist/librarian memory experiments

## Documentation

Start here for the current state of the experiment:

- `CYBER_MEMORY_LORA_RESULTS.csv`: compact top-level scoreboard for Kaggle
- `CYBER_MEMORY_LORA_RESULTS.json`: machine-readable version of the same scoreboard
- `docs/EXPERIMENT_REPORT.md`: full thesis, results, interpretations, caveats, and next branches
- `docs/METHODS.md`: reproduction commands and runtime-specific methodology
- `docs/RUN_INDEX.md`: canonical run folders and key scores
- `docs/ARCHITECTURE_NOTES.md`: definitions for memory, primitives, transport, KV cache, and activation steering
- `docs/KAGGLE_WORKFLOW.md`: Kaggle SDK/CLI setup and dataset export/upload workflow
- `docs/FINDINGS_2026_06_06_LIVE_MEMORY_AND_LORA.md`: live prompt-memory and LoRA findings from 2026-06-06

## Current Model Targets

- `qwen3.5-2b-base@q4_k_m`: main GGUF/LM Studio prompt-memory target
- `qwen3.5-2b-base@q8_0`: GGUF/LM Studio precision comparison target
- `Qwen3.5-2B-Base.Q4_K_M.gguf`: llama.cpp primitive prefix-cache target
- `Qwen/Qwen3-1.7B-Base`: HuggingFace/PyTorch residual-stream steering target
- `Qwen/Qwen3-1.7B-Base`: first successful cyber-memory LoRA target
- `Qwen/Qwen3-4B-Base`: next-size QLoRA target; P100 is rejected because this branch tests compressed training, not fp16 fallback

Use imported/raw base-model GGUFs where possible for prompt/cache tests. Use HuggingFace/PyTorch models for activation steering because GGUF frontends do not expose residual-stream hooks.

## Experiment Modes

- `baseline`: task only
- `memory`: retrieved synthetic memories plus task
- `primitive_packet`: task plus compact cognitive primitives
- `transport_packet`: primitive packet delivered with frame, route, checksum, and repair metadata
- `neural_exoskeleton`: memory or primitive substrate plus routing, monitoring, verifier, repair loop, and output gate

## Current Architecture Direction

The current working theory is that memories alone are only one layer. A stronger scaffold may need:

- synthetic memories as the experience substrate
- cognitive primitives as reusable operations
- a transport layer to deliver primitives reliably into context
- stabilizer primitives such as route lock, cascade guard, distractor suppression, and final-answer gate
- an external neural exoskeleton that monitors output, verifies missing steps, and repairs drift

The next stability upgrade is KV-cache memory delivery. Instead of injecting memories as ordinary prompt text every time, prefill the model with the memory corpus, preserve/reuse that KV state, and then run tasks from that conditioned state. This should test whether memories behave more like prior context than current instructions.

Compare these modes when implemented:

- task only
- prompt-injected memory
- RAG-selected memory
- KV-cache prefilled memory
- KV-cache memory plus primitive transport packet
- KV-cache memory plus neural exoskeleton verifier/repair loop

The final confirmation branch is LoRA training: convert the cyber-defender memory corpus into SFT examples, train a small adapter, and compare base vs adapter on held-out prompt-injection probes. This tests whether the memory mechanism survives when moved from retrieval/context into weights.

## Cyber Memory LoRA

The current cyber corpus file is:

- `rust_cyber_defender_generated.tsv`: `360` generated prompt-injection/cyber-defender memories exported from the Rust synthetic-memory simulator

The LoRA trainer converts each memory into two kinds of SFT examples:

- source-boundary behavior examples
- memory-recall examples with operational conclusions

Training setup:

- model: `Qwen/Qwen3-1.7B-Base`
- examples: `720`
- LoRA rank: `16`
- steps: `160`
- output: `runs/kaggle-lora-v4/rust-cyber-memory-lora`

Kaggle kernels:

- `jessealicea/rust-cyber-memory-lora-train`
- `jessealicea/rust-cyber-instruct-baseline-eval`
- `jessealicea/rust-cyber-instruct-plus-lora-eval`
- `jessealicea/rust-cyber-memory-lora-train-4b`

The 4B branch is QLoRA-only. It should run on T4/A100/L4-class cloud GPUs. It intentionally refuses P100 because P100 forced fp16 fallback, which is not the compressed-model test.

## Earlier Runtime-Memory Branches

The older runtime-memory corpora are still preserved because they explain how the current LoRA branch was reached. These memories are not ordinary instructions. They are episodic cognitive procedures:

- orient before answering
- identify the real question
- separate source from inference
- use claim/evidence/warrant
- bound analogies
- map spatial or system structure
- turn math words into variables
- debug by reproducing and isolating
- stop loops with a checkpoint
- close with a next action

Current corpus files:

- `cognitive_core_v0_1.json`: base cognitive habits
- `cognitive_rewards_v0_1.json`: reward and calibration memories
- `computation_primitives_v0_1.json`: narrow arithmetic memories for rates, ledgers, weighted averages, tiers, schedules, ratios, probability, units, and intermediate-value preservation
- `rust_cyber_defender_generated.tsv`: generated prompt-injection/cyber-defender memories exported from the Rust synthetic-memory simulator for LoRA and RAG tests
- `librarian_core_v0_1.json`: earlier 15-memory generalist/librarian persona core used for routing and source-shelf experiments

The librarian corpus was an earlier generalist bridge. Kate's memories overlap around organizing, research, childhood library desire, good outcomes, bad outcomes, provenance, usability, and task-fit. It is no longer the front-page Kaggle story.

The first 30-memory version was backed up as `librarian_core_v0_1.full30.bak`. The active JSON corpus is trimmed to 15 memories after the 30-memory all-fire test overloaded the tiny models.

The runner supports three activation modes:

- `-LibrarianCoreMode All`: appends all Kate core memories as compact cues every task
- `-LibrarianCoreMode Spine`: appends one distilled Kate identity spine made from the full corpus
- `-LibrarianCoreMode Top`: appends the single highest-scoring Kate memory for the task

## Earlier Kate Results

The `baseline` row in each run is the base model with no retrieved memories.

| Model | Kate Mode | No-Memory Baseline | Memory Result | Lift |
|---|---:|---:|---:|---:|
| Q8 | 15-memory `All` | `17/38` | `17/38` | `+0` |
| F16 | 15-memory `All` | `18/38` | `19/38` | `+1` |
| Q8 | 15-memory `Top` | `17/38` | `26/38` | `+9` |
| F16 | 15-memory `Top` | `18/38` | `23/38` | `+5` |

## Current Computation Results

The generalist/librarian corpus helped orientation tasks but hurt arithmetic. A separate compute-only corpus plus answer-shape scaffold repaired Qwen3.5 2B Q4_K_M on the computation-damage suite.

| Model | Corpus / Prompt | No-Memory Baseline | Memory Result | Lift |
|---|---|---:|---:|---:|
| Qwen3.5 2B Q4_K_M | old all-corpus, title cues, no answer shape | `44/67` | `39/67` | `-5` |
| Qwen3.5 2B Q8_0 | old all-corpus, title cues, no answer shape | `48/67` | `46/67` | `-2` |
| Qwen3.5 2B Q4_K_M | compute-only, body memories, answer shape, TopK 3 | `50/67` | `58/67` | `+8` |
| Qwen3.5 2B Q4_K_M | repeat: compute-only, body memories, answer shape, TopK 3 | `51/67` | `58/67` | `+7` |
| Qwen3.5 2B Q8_0 | compute-only, body memories, answer shape, TopK 3 | `55/67` | `54/67` | `-1` |

Isolation runs showed that Q4 title-only compute memories still hurt (`50/67` baseline to `47/67` memory). The repair required episodic memory bodies plus a small output scaffold. TopK 5 improved less than TopK 3, which supports the attention-clutter hypothesis for compressed models.

## Primitive Cache Test

The first llama.cpp cache test used Qwen3.5 2B Q4_K_M through `llama-server` on raw `/completion`, not LM Studio chat mode. This is a different runtime/prompt family, so the absolute scores are lower and should not be directly compared to the LM Studio chat-template runs.

The easy cache path works mechanically: a shared primitive prefix was evaluated once and then reused as KV prefix context. In the clean run, baseline had cache disabled and primitive modes had cache enabled.

| Mode | Score | Avg Cached Tokens | Note |
|---|---:|---:|---|
| baseline raw completion | `29/67` | `0.0` | task only, no cache prompt |
| universal primitive cache | `36/67` | `244.0` | fixed arithmetic primitive packet reused from cache |
| routed primitive cache | `34/67` | `96.3` | task-specific primitive block; less reuse |

Result: primitive cache improved raw completion by `+7`, but the broad packet still caused failures on some tasks. This supports testing cache-resident primitives, while also showing that boundary/transport design matters as much as the primitive content.

## Activation Steering Prototype

The lab now has a Python/Transformers residual-stream steering prototype:

- `scripts/activation_steering_probe.py`

This is separate from GGUF/LM Studio/Ollama. Residual-stream hooks require a HuggingFace/PyTorch model because GGUF frontends do not expose layer activations in normal API use.

The script:

- loads a causal LM from a HF id or local HF-format path
- finds decoder layers
- captures residual activations at a selected layer
- computes a contrastive vector from positive minus negative examples
- applies `hidden = hidden + strength * vector` during generation
- saves `steering_vector.pt` and `report.json`

Smoke test:

```powershell
python .\scripts\activation_steering_probe.py --model sshleifer/tiny-gpt2 --dtype float32 --device-map cpu --strength 1.0 --max-new-tokens 40 --out .\runs\activation-steering-smoke
```

The tiny GPT-2 smoke test only proves hook mechanics, not useful capability gain. The next meaningful test needs a HF-format small base model, preferably the same Qwen-family 2B class if available outside GGUF.

First real HF/PyTorch run used `Qwen/Qwen3-1.7B-Base`, because it is a plain text causal base model (`Qwen3ForCausalLM`) with 28 decoder layers. `Qwen/Qwen3.5-2B-Base` exists, but it uses `Qwen3_5ForConditionalGeneration`, so it needs a separate loader/hook path.

Arithmetic steering vector:

- positive examples: careful arithmetic, visible intermediate values, units, clock conversion, ratios, money signs, without-replacement counts
- negative examples: rushed arithmetic, skipped intermediate values, dropped units, bad clock/rate/credit/probability habits
- steering operation: `hidden = hidden + strength * vector`

Initial wide sweep:

| Model | Mode | Score |
|---|---|---:|
| Qwen3 1.7B Base | baseline | `38/67` |
| Qwen3 1.7B Base | layer 7, strength 0.5 | `12/67` |
| Qwen3 1.7B Base | layer 14, strength 0.5 | `33/67` |
| Qwen3 1.7B Base | layer 21, strength 0.5 | `43/67` |

Tighter late-layer sweep:

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

Result: residual-stream steering improved the arithmetic benchmark by `+5` over baseline at the best settings. Early-layer steering was destructive, high strength was often destructive, and late layers around 21-22 were useful. This supports activation steering as a real control surface, but the first vector is still weaker than the best prompt-memory result.

## Useful Commands

Q8 adaptive rewards plus Kate librarian bridge:

```powershell
& "C:\Projects\small-model-memory-lab\scripts\run_tiny_memory_benchmark.ps1" -Model "qwen3-1.7b-base-q8-raw" -LabRoot "C:\Projects\small-model-memory-lab" -TopK 2 -NumPredict 120 -EmbeddingModel "nomic-embed-text:latest" -UseRewardMemories -UseLibrarianCore -LibrarianCoreMode Top -AdaptiveMemoryPolicy
```

F16 adaptive rewards plus Kate librarian bridge:

```powershell
& "C:\Projects\small-model-memory-lab\scripts\run_tiny_memory_benchmark.ps1" -Model "qwen3-1.7b-base-f16-raw" -LabRoot "C:\Projects\small-model-memory-lab" -TopK 2 -NumPredict 120 -EmbeddingModel "nomic-embed-text:latest" -UseRewardMemories -UseLibrarianCore -LibrarianCoreMode Top -AdaptiveMemoryPolicy
```

## First Benchmark Goal

Run a small suite across different task families and measure:

- criteria hit rate
- loop rate
- concrete next-action rate
- baseline vs memory lift
- retrieval precision
