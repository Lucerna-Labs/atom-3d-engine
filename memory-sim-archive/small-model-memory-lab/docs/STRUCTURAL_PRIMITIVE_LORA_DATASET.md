# Structural Primitive LoRA Dataset

Date: 2026-06-04

Purpose:

Train a LoRA that reinforces structural primitives for compressed small models.

This dataset is separate from:

- Rust cyber-defense memories
- Mother behavioral memories
- Kate/librarian generalist memories

## Files

Memory corpus:

- `corpus/structural_primitives_lora_v0_1.tsv`
- `corpus/structural_primitives_lora_v0_1.json`

SFT dataset:

- `data/structural_primitive_lora_v0_1/train.jsonl`
- `data/structural_primitive_lora_v0_1/validation.jsonl`
- `data/structural_primitive_lora_v0_1/manifest.json`
- `data/structural_primitive_lora_v0_1/preview.json`

Kaggle training script:

- `kaggle_notebooks/lora_train_structural/train_structural_primitive_lora.py`

Generator:

- `scripts/build_structural_primitive_lora_dataset.py`

## Counts

- memories: `122`
- SFT examples: `584`
- train examples: `537`
- validation examples: `47`

## v0.2 Cleanup

The cleaned dataset is:

- `data/structural_primitive_lora_v0_2/train.jsonl`
- `data/structural_primitive_lora_v0_2/validation.jsonl`
- `data/structural_primitive_lora_v0_2/manifest.json`

Counts:

- SFT examples: `360`
- train examples: `320`
- validation examples: `40`

Changes:

- kept useful task, compact task, and repair examples from v0.1
- reduced long memory recall/application examples
- added gate open/close examples
- added web association repair examples
- added carrier/no-drift examples
- added anti-echo stop examples
- patched the trainer to prefer v0.2 and append EOS during tokenization

## Target Packet

The dataset follows the best repeat-reinforced simulation packet:

```text
FRAME
CLOCK
CARRIER
PARITY
CHECKSUM
ECC
REDUNDANCY
SUPPRESSION
COMPRESSION
STATE_BUFFER
GAIN_CLAMP
FRAME
CLOCK
ECC
GAIN_CLAMP
```

`FRAME`, `CLOCK`, `ECC`, and `GAIN_CLAMP` are intentionally overweighted.

`ROUTER` is intentionally absent because the Q2 self-reinforcement simulation showed negative average lift for router. Routing created a second failure surface when it pointed a coherent packet at the wrong task.

## Example Types

The JSONL records include:

- `memory_recall`
- `memory_application`
- `task_solution`
- `compact_task_solution`
- `repair_solution`

The goal is not only to teach the model the names of primitives. The goal is to train the behavior:

- frame the real task
- keep steps in order
- hold state
- suppress distractors
- keep scaffolding compact
- detect missing operations
- repair before final output
- final-check against the original request

## Dry Run

The structural training script was dry-run locally with training disabled:

```powershell
$env:RUN_LORA_TRAINING='0'
python .\kaggle_notebooks\lora_train_structural\train_structural_primitive_lora.py
```

Result:

- found dataset root
- loaded `537` train examples
- loaded `47` validation examples
- wrote preview to `runs/structural-primitive-lora-local/structural_training_preview.json`

## Kaggle Start

After the project dataset is versioned on Kaggle, run the notebook/script with:

```text
RUN_LORA_TRAINING=1
BASE_MODEL_ID=Qwen/Qwen3-1.7B-Base
```

For a larger base model, override `BASE_MODEL_ID`, but keep this first run on `Qwen/Qwen3-1.7B-Base` so it can be compared against the existing activation-steering and cyber-memory LoRA runs.
