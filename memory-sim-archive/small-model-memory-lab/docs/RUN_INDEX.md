# Canonical Run Index

This index lists the runs worth preserving as evidence. There are many exploratory runs; this file focuses on runs that changed the hypothesis or established a current result.

## Capability Suite

Benchmark:

- `benchmarks/capability_tasks.json`
- 8 tasks, 38 criteria

| Run | Runtime | Model | Setup | Result |
|---|---|---|---|---:|
| `runs\tiny-memory-20260604-102754-893` | LM Studio chat | Qwen3.5 2B Q4_K_M | baseline vs all-corpus memory | `26/38` to `31/38` |
| `runs\tiny-memory-20260604-102842-929` | LM Studio chat | Qwen3.5 2B Q8_0 | baseline vs all-corpus memory | `23/38` to `28/38` |

Finding:

- memory helped both Q4 and Q8 by `+5`
- broad/general memories worked better on capability tasks than on arithmetic

## Computation Suite: General Memories Hurt

Benchmark:

- `benchmarks/computation_damage_tasks.json`
- 12 tasks, 67 criteria

| Run | Runtime | Model | Setup | Result |
|---|---|---|---|---:|
| `runs\tiny-memory-20260604-103441-668` | LM Studio chat | Qwen3.5 2B Q4_K_M | old all-corpus, no answer shape | `44/67` to `39/67` |
| `runs\tiny-memory-20260604-103517-417` | LM Studio chat | Qwen3.5 2B Q8_0 | old all-corpus, no answer shape | `48/67` to `46/67` |

Finding:

- generalist/librarian memory added arithmetic noise
- computation needed its own corpus

## Computation Suite: Compute Corpus

| Run | Runtime | Model | Setup | Result |
|---|---|---|---|---:|
| `runs\tiny-memory-20260604-103954-532` | LM Studio chat | Qwen3.5 2B Q4_K_M | compute-only title cues, no answer shape | `44/67` to `39/67` |
| `runs\tiny-memory-20260604-104020-564` | LM Studio chat | Qwen3.5 2B Q8_0 | compute-only title cues, no answer shape | `48/67` to `47/67` |
| `runs\tiny-memory-20260604-104117-679` | LM Studio chat | Qwen3.5 2B Q4_K_M | compute bodies, TopK 3, no answer shape | `44/67` to `41/67` |
| `runs\tiny-memory-20260604-104217-527` | LM Studio chat | Qwen3.5 2B Q4_K_M | compute bodies, TopK 3, answer shape | `50/67` to `58/67` |
| `runs\tiny-memory-20260604-104243-433` | LM Studio chat | Qwen3.5 2B Q8_0 | compute bodies, TopK 3, answer shape | `55/67` to `54/67` |
| `runs\tiny-memory-20260604-104317-307` | LM Studio chat | Qwen3.5 2B Q4_K_M | compute titles, TopK 3, answer shape | `50/67` to `47/67` |
| `runs\tiny-memory-20260604-104425-388` | LM Studio chat | Qwen3.5 2B Q4_K_M | repeat best compute-body setup | `51/67` to `58/67` |

Finding:

- body memories plus light answer-shape scaffold are the best prompt/RAG method so far
- title-only memories hurt
- Q4 benefited; Q8 did not

## Computation Memory Tuning Attempts

| Run | Runtime | Model | Setup | Result |
|---|---|---|---|---:|
| `runs\tiny-memory-20260604-105017-378` | LM Studio chat | Qwen3.5 2B Q4_K_M | first correction-memory edits | `49/67` to `58/67` |
| `runs\tiny-memory-20260604-105155-055` | LM Studio chat | Qwen3.5 2B Q4_K_M | added exact correction memories | `52/67` to `59/67` |
| `runs\tiny-memory-20260604-105533-200` | LM Studio chat | Qwen3.5 2B Q4_K_M | negative-outcome memories with bad-answer tokens | `50/67` to `58/67` |
| `runs\tiny-memory-20260604-105733-792` | LM Studio chat | Qwen3.5 2B Q4_K_M | strong memory activation | `51/67` to `49/67` |
| `runs\tiny-memory-20260604-105859-350` | LM Studio chat | Qwen3.5 2B Q4_K_M | compact compute cues | `52/67` to `55/67` |
| `runs\tiny-memory-20260604-105940-796` | LM Studio chat | Qwen3.5 2B Q4_K_M | overly strict math prompt | `49/67` to `48/67` |

Finding:

- plateau is around `58-59/67`
- strong activation and stricter prompts can reduce performance
- bad-answer tokens in negative memories can contaminate outputs

## Primitive Cache Runs

Runtime:

- llama.cpp raw `/completion`
- Qwen3.5 2B Q4_K_M GGUF

| Run | Setup | Result |
|---|---|---:|
| `runs\primitive-cache-20260604-112911` | first universal primitive cache | `50/67` baseline to `30/67` primitive cache |
| `runs\primitive-cache-20260604-113125` | closed block, isolated slots | `32/67` baseline to `23/67` primitive cache |
| `runs\primitive-cache-20260604-113426` | fixed seed, cache enabled | `33/67` baseline to `40/67` primitive cache |
| `runs\primitive-cache-20260604-113609` | clean baseline cache disabled | `29/67` baseline to `36/67` universal cache, `34/67` routed cache |

Finding:

- cache mechanics work
- clean primitive cache gave `+7`
- broad primitive packets can still cause list continuation and boundary problems
- this is prefix cache, not arbitrary hidden KV injection

## Activation Steering Runs

Runtime:

- HuggingFace / PyTorch
- Qwen/Qwen3-1.7B-Base

| Run | Setup | Result |
|---|---|---:|
| `runs\activation-steering-smoke` | tiny GPT-2 hook smoke test | hook mechanics only |
| `runs\activation-steering-20260604-115247` | layers 7,14,21; strengths 0.5,1.0,2.0 | baseline `38/67`, best `43/67` at layer 21 strength 0.5 |
| `runs\activation-steering-20260604-120833` | layers 20,21,22; strengths 0.25,0.5,0.75 | baseline `38/67`, best `43/67` at layer 21 strength 0.5 and layer 22 strength 0.75 |

Finding:

- residual-stream steering is a real control surface
- early layer steering was destructive
- late layer steering around 21-22 helped modestly

## Training-Ready Datasets

| Dataset | Purpose | Counts | Status |
|---|---|---:|---|
| `data\structural_primitive_lora_v0_1` | structural primitive LoRA for compressed small models | `537` train / `47` validation | dry-run ready |

Finding:

- dataset follows the repeat-reinforced packet from `runs\q2-primitive-self-reinforcement`
- `FRAME`, `CLOCK`, `ECC`, and `GAIN_CLAMP` are overweighted
- `ROUTER` is intentionally excluded from the first structural LoRA dataset

## Structural Primitive LoRA Runs

Runtime:

- Kaggle GPU
- assigned GPU: P100 path required P100-safe PyTorch and no bitsandbytes
- base model: `Qwen/Qwen3-1.7B-Base`

| Run | Setup | Result |
|---|---|---:|
| `runs\kaggle-structural-lora-v1` | structural primitive LoRA, 537 train / 47 validation, 180 steps | base `2/6` to LoRA `6/6` |
| `runs\kaggle-structural-lora-v0-2` | cleaned structural primitive LoRA v0.2, 320 train / 40 validation, 180 steps | base `2/6` to LoRA `6/6`; no obvious template-echo markers in quick eval |

Finding:

- first structural primitive LoRA completed successfully
- quick structural probe lift was `+4`
- outputs show strong primitive activation, but also some continuation/echo behavior that needs cleanup in the next dataset or generation stop rules
- v0.2 cleanup preserved the score and removed the obvious `### User` / `### Assistant` continuation markers in the quick eval

## Hybrid LoRA Geometry Simulation

These are toy/simulation runs, not real model proof.

| Run | Purpose | Finding |
|---|---|---|
| `runs\hybrid-lora-geometry-sim` | compare additive, gated, Kronecker/web, Fourier/carrier, and hybrid adapter geometry | refined all-three balanced clean pass `78.97%`; overbuilt `85.99%`; standard additive clean pass `6.58%` with high echo |

Finding:

- simulation supports a hybrid adapter, not standard additive LoRA alone
- best first real target is a balanced gated + web + carrier adapter
- overbuilt hybrid wins the toy score but should be second-stage because it has higher training complexity and nonzero echo under echo-trap pressure

## Simulation Runs

These are toy/simulation runs, not real model proof.

| Run | Purpose | Finding |
|---|---|---|
| `runs\primitive-signal-sim` | primitive signal hardening toy sim | raw primitives helped but did not reach perfect |
| `runs\q2-primitive-combo-sweep` | sweep primitive combinations | best raw combinations around mid/high 70s in toy scoring |
| `runs\q2-stabilizer-primitives` | add stabilizers | toy results approached near-perfect |
| `runs\q2-transport-layer` | RF-style transport framing | toy results approached near-perfect |
| `runs\q2-neural-exoskeleton` | full runtime wrapper simulation | toy results approached `99-100%` |
| `runs\q2-primitive-self-reinforcement` | structural primitives that keep the primitive layer alive | refined binary best `52.0%` to `87.1%`; repeat-reinforced best `89.3%`; cascade `22.2%` to `0.02%` |

Finding:

- simulations suggest primitives need transport/stabilizers/verifier
- structural primitives should be trained as their own layer, not only used as wrapper logic
- real model tests are much more fragile
- do not cite simulation results as model performance
