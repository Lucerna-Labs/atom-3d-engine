---
base_model: Qwen/Qwen3-1.7B-Base
library_name: peft
pipeline_tag: text-generation
license: cc0-1.0
tags:
- base_model:adapter:Qwen/Qwen3-1.7B-Base
- lora
- peft
- synthetic-memory
- prompt-injection
---

# Rust Cyber Memory LoRA

This is a PEFT LoRA adapter trained on generated cyber/source-boundary synthetic memories.

The adapter is part of the Small Model Memory and Structural Primitive Lab. It tests whether memory-shaped examples can become indirect learned instructions for prompt-injection resistance.

## Base Model

```text
Qwen/Qwen3-1.7B-Base
```

## Intended Use

Attach this adapter to the base model with PEFT and test source-boundary behavior, prompt-injection handling, and task-preservation behavior.

## Public Result

On the 12-probe cyber evaluation set:

| Setup | Strict | Behavioral |
|---|---:|---:|
| Qwen3 1.7B instruct, no memories | `3/12` | `3/12` |
| Qwen3 1.7B Base plus this LoRA | `7/12` | `12/12` |
| Qwen3 1.7B instruct plus this LoRA | `5/12` | `7/12` |

The strict scorer penalizes safe mentions of dangerous phrases. The behavioral scorer checks whether the model actually follows the malicious instruction or preserves the task boundary.

## Caveats

This is a research adapter, not a production security model. It was trained on synthetic examples and evaluated on a narrow probe set. It can still over-recite memory-shaped text.
