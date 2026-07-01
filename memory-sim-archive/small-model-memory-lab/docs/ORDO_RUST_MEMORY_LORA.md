# Ordo Rust Memory LoRA v0.1

This adapter branch trains a narrow Rust/Ordo builder from the strongest real-compiler memory results so far.

Source evidence:

- Rust-coder MoE without Ordo memories on the 12-task suite: 0/12.
- The same model with hardened Ordo memories: 10/12.
- Earlier 3-task Ordo suite after scar-memory hardening: 3/3 builder-stage passes.

Training set:

- `data/ordo_rust_memory_lora_v0_1/train.jsonl`
- `data/ordo_rust_memory_lora_v0_1/validation.jsonl`

The dataset uses only passed real Cargo-test artifacts as code targets. Failed generated crates are excluded. Memory-scar records encode exact API signatures, field names, CLI output forms, Rust ownership repairs, and behavior details learned from real compiler/integration-test failures.

Kaggle training notebook:

- `kaggle_notebooks/lora_train_ordo_rust/train_ordo_rust_memory_lora.py`
- Public kernel slug: `jessealicea/ordo-rust-memory-lora-train`

Default base model:

- `Qwen/Qwen3-1.7B-Base`

This is intentionally a small public proof adapter, not the local Rust MoE model. The local MoE result motivates the corpus; the Kaggle LoRA tests whether a compact public base model can internalize part of that Ordo/Rust behavior.
