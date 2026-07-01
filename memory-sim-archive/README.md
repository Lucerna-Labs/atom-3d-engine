# Memory And Primitive Simulation Archive

This folder gathers the memory-model projects and primitive simulation work into one Git-tracked parent archive.

It is intentionally a curated archive, not a raw disk mirror. The archive keeps project notes, scripts, corpora, datasets, notebooks, result summaries, and canonical adapter outputs. It skips base model weights, optimizer checkpoints, downloaded embedding weights, build output, and duplicate raw Kaggle checkpoint folders that are too large or too redundant for normal Git history.

## Contents

| Folder | Source | Purpose |
|---|---|---|
| `small-model-memory-lab/` | `C:\Projects\small-model-memory-lab` | Main structural primitive LoRA, cyber-memory LoRA, primitive cache, activation steering, live LoRA matrix, Kaggle notebooks, result notes, corpora, and canonical LoRA adapters. |
| `rust-synthetic-memory-specialist/` | `F:\all memory\Rust Simulation for Synthetic Memory Specialist` | Older routed-memory and Ordo/Rust memory specialist lineage, including routing-to-LoRA notes, Rust harnesses, memory corpora, scripts, simulations, and the canonical `ordo_lora_adapter`. |
| `primitive-math-store/` | `F:\all primitves\primitves math` | Primitive taxonomy, doctrine notes, primitive simulator, discovery simulator, recipe-inference outputs, and cross-domain primitive catalogs. |
| `lora-memory-experiment/` | `F:\all memory\lora-memory-experiment` | Early memory-LoRA experiment with corpus, Colab/notebook support, and simulation pieces. |
| `lora-test/` | `F:\all memory\lora-test` | Small LoRA smoke test, training script, logs, and final adapter output. Base HF model copy is skipped. |
| `memory-data-set/` | `F:\all memory\memory data set` | Memory-writing guide and seed cyber memory records. |
| `planning/` | `F:\all memory\ordo_lora_dataset_plan.md` | Surgical Ordo architecture LoRA dataset plan. |

## Index Files

- `SOURCE_MAP.csv`: source path, archive path, and notes for each imported project.
- `COPIED_FILES.csv`: per-file provenance for copied files.
- `SKIPPED_HEAVY_FILES.csv`: heavyweight artifacts intentionally left in place, with size and reason.

## Git And LFS Policy

The archive includes canonical adapter `.safetensors` files where they are part of the research result and under GitHub's single-file limit. These are tracked by Git LFS via the repo-level `.gitattributes`.

Skipped examples include:

- GGUF and HuggingFace base model weights.
- Optimizer, scheduler, RNG, and checkpoint internals.
- Downloaded embedding/reranker model weights.
- Duplicate Kaggle export/checkpoint copies.
- The unrelated `F:\all memory\Lora_data` image LoRA dataset.

## Relationship To The Renderer Experiments

The renderer `experiments/` folder validates primitive discoveries against the MM3E engine.

This archive preserves the adjacent model-memory lineage:

- structural primitives trained into small models with LoRA;
- routed synthetic memories converted into training distributions;
- primitive cache and activation steering tests;
- Rust primitive inference and type-validity simulations;
- the primitive taxonomy/store that informed both engine and model experiments.

