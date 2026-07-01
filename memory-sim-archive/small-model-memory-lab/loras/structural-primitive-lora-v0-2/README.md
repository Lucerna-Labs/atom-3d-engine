---
base_model: Qwen/Qwen3-1.7B-Base
library_name: peft
pipeline_tag: text-generation
license: cc0-1.0
tags:
- base_model:adapter:Qwen/Qwen3-1.7B-Base
- lora
- peft
- structural-primitives
- synthetic-memory
---

# Structural Primitive LoRA v0.2

This is the cleaned structural primitive PEFT LoRA adapter from the Small Model Memory and Structural Primitive Lab.

It tests whether small functional primitives, such as frame, gate, state buffer, carrier, checksum, and clean stop, can be trained into a small base model as reusable task-stability behavior.

## Base Model

```text
Qwen/Qwen3-1.7B-Base
```

## Intended Use

Attach this adapter to the base model with PEFT and test small reasoning, source-boundary, spatial, rhetoric, and debugging prompts.

## Public Result

On the 6-probe structural smoke eval:

| Setup | Score |
|---|---:|
| Qwen3 1.7B Base, no structural LoRA | `2/6` |
| Structural primitive LoRA v0.1 | `6/6` |
| Cleaned structural primitive LoRA v0.2 | `6/6` |

The v0.2 cleanup preserved the score while removing obvious `### User` / `### Assistant` template-continuation markers in the quick eval outputs.

## Caveats

This is a research adapter from a narrow smoke eval, not proof of broad general capability. Some repetition remains and should be tested carefully.
