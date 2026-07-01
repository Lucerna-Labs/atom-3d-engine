# Small Model Memory and Structural Primitive Lab

This Kaggle project tests whether synthetic memory corpora and structural primitive datasets can be trained into small models as measurable behavioral capability.

The theory is that memories act as indirect learned instructions. A strong synthetic memory closes the loop from situation to recognition to action to consequence, so the trained behavior can become more instinct-like than a flat prompt rule.

The efficiency hypothesis is that it may only take a fraction of the examples used in broad instruction tuning to get instruction-like outcomes in a narrow behavior space, if the memories are dense, specific, and associated across similar cases.

The model-fit hypothesis is that cognitive false memories are strongest on pretrained base models because they provide domain behavior where no instruction-tuned assistant substrate is already competing. Instruct models can still benefit, especially if they are degraded or aggressively compressed, but the gains should usually be smaller or more mixed. Structural primitives are different: they are functional operators rather than instructions, so they should transfer more evenly across base and instruct models.

There are two real LoRA branches so far:

- cyber-defender memories for prompt-injection resistance
- structural primitive memories for small-model task stability

The simulation files are included because they guided what to train next, but the LoRA results listed below are real Kaggle model runs.

## Latest Public Update

The current dataset now includes the latest structural primitive LoRA v0.2 eval, both public PEFT LoRA adapters, and the expanded theory note.

Structural primitive LoRA on a 6-probe smoke eval:

| Model | Setup | Score |
|---|---|---:|
| `Qwen/Qwen3-1.7B-Base` | base | `2/6` |
| `Qwen/Qwen3-1.7B-Base` | structural primitive LoRA v0.1 | `6/6` |
| `Qwen/Qwen3-1.7B-Base` | cleaned structural primitive LoRA v0.2 | `6/6` |

The v0.2 cleanup preserved the score and removed obvious `### User` / `### Assistant` continuation markers from the quick eval outputs.

Public adapters:

- `loras/rust-cyber-memory-lora`
- `loras/structural-primitive-lora-v0-2`

## Cyber-Memory LoRA Finding

Cyber-memory LoRA on a 12-probe prompt-injection evaluation set:

| Model | Setup | Strict | Behavioral |
|---|---|---:|---:|
| `Qwen/Qwen3-1.7B` | instruct, no memories | `3/12` | `3/12` |
| `Qwen/Qwen3-1.7B-Base` | cyber-memory LoRA | `7/12` | `12/12` |
| `Qwen/Qwen3-1.7B` | instruct plus same LoRA | `5/12` | `7/12` |

The strict scorer penalizes safe mentions of dangerous phrases. The behavioral scorer checks whether the model actually follows the malicious instruction or preserves the source boundary.

## What To Inspect

- `CYBER_MEMORY_LORA_RESULTS.csv`: compact scoreboard
- `CYBER_MEMORY_LORA_RESULTS.json`: machine-readable scoreboard and caveats
- `STRUCTURAL_PRIMITIVE_LORA_RESULTS.csv`: compact scoreboard for the structural v0.1/v0.2 training and eval runs
- `STRUCTURAL_PRIMITIVE_LORA_RESULTS.json`: machine-readable summary of the latest structural eval
- `THEORY_OF_APPROACH.md`: plain-English theory behind memories, primitives, KV cache, steering, and hybrid LoRA geometry
- `loras/`: PEFT LoRA adapters for the cyber-memory and structural-primitive runs
- `corpus/rust_cyber_defender_generated.tsv`: generated cyber-memory corpus
- `data/structural_primitive_lora_v0_2/train.jsonl`: cleaned structural primitive SFT split
- `kaggle_notebooks/lora_train/train_rust_cyber_lora.py`: first successful LoRA trainer
- `kaggle_notebooks/lora_train_structural/train_structural_primitive_lora.py`: structural primitive LoRA trainer
- `kaggle_notebooks/instruct_eval/eval_instruct_baseline.py`: no-memory instruct baseline
- `kaggle_notebooks/instruct_lora_eval/eval_instruct_plus_lora.py`: instruct model with the trained LoRA attached
- `docs/KAGGLE_WORKFLOW.md`: upload workflow and run history

## Important Scope

What is proven here:

- synthetic-memory SFT can change real model behavior in these constrained evals
- cyber-memory LoRA improved prompt-injection behavior over the tested baselines
- structural primitive LoRA improved a small smoke eval from `2/6` to `6/6`
- dataset cleanup reduced obvious template echo in the structural branch

What is not proven yet:

- broad general capability improvement
- reliability across many model families and held-out benchmarks
- real training of the proposed hybrid gated/web/carrier adapter
- production safety

The prompt-injection corpus uses mock strings such as "token" and "API key" as test content. They are synthetic examples, not real secrets.
