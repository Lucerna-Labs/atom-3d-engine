# Kaggle Workflow

This lab can be packaged and uploaded as a Kaggle Dataset.

Current dataset:

```text
https://www.kaggle.com/datasets/jessealicea/small-model-memory-lab
```

The first upload was created as a private dataset.

Current private starter kernel:

```text
https://www.kaggle.com/code/jessealicea/small-model-memory-lab-home-base
```

Current private LoRA training kernel:

```text
https://www.kaggle.com/code/jessealicea/rust-cyber-memory-lora-train
```

Current private instruct baseline kernel:

```text
https://www.kaggle.com/code/jessealicea/rust-cyber-instruct-baseline-eval
```

Current private instruct plus LoRA kernel:

```text
https://www.kaggle.com/code/jessealicea/rust-cyber-instruct-plus-lora-eval
```

Verified status:

```text
READY
```

Verified uploaded payload includes root docs plus expanded `benchmarks/`, `corpus/`, `docs/`, `runs/`, `scripts/`, and `kaggle_notebooks/` dataset files.

## Installed Tooling

Installed in the local Python environment:

- `kaggle` CLI/API: `1.7.4.5`
- `kagglehub`: pinned to `0.3.13`

Reason for the pin:

- `kagglehub` `1.0.1` installed but failed to import because of a `kagglesdk` import mismatch.
- `0.3.13` imports cleanly in this environment.

## Credential Setup

Credential file:

```text
C:\Users\jgali\.kaggle\kaggle.json
```

Important auth detail:

- The classic `kaggle` CLI expects `username` plus classic API `key`.
- The provided `KGAT` token works through the newer bearer-token path exposed by `kagglesdk` as `KAGGLE_API_TOKEN`.
- Direct CLI upload returned `401 Unauthorized` against the blob upload endpoint with the `KGAT` token.
- The SDK uploader below succeeds because it sets `KAGGLE_API_TOKEN` and calls `kagglesdk` directly.

Basic read/list verification:

```powershell
kaggle datasets list -s synthetic-memory -p 1
```

Bearer-token SDK verification:

```powershell
python .\scripts\upload_kaggle_dataset_sdk.py --dry-run
```

## Build Dataset Export

Script:

```powershell
python .\scripts\package_kaggle_dataset.py --owner YOUR_KAGGLE_USERNAME
```

Default export path:

```text
C:\Projects\small-model-memory-lab\kaggle_export\small-model-memory-lab
```

The export includes:

- docs
- benchmarks
- corpora
- scripts
- Kaggle kernel scaffolds
- README/TODO
- selected run summaries and response files

It does not include model weights.

## Create Or Update Kaggle Dataset

The reliable path for the current token is the SDK uploader:

```powershell
python .\scripts\upload_kaggle_dataset_sdk.py --owner jessealicea
```

Default behavior:

- Uploads root files individually.
- Uploads top-level folders as zip files.
- Creates the dataset if missing.
- Creates a new version if the dataset already exists.
- Creates private datasets by default unless `--public` is passed. The current public dataset should be refreshed with `--public`.

## Push Starter Kaggle Kernel

The starter kernel lives at:

```text
C:\Projects\small-model-memory-lab\kaggle_notebooks\starter
```

Push or update it with:

```powershell
python .\scripts\upload_kaggle_kernel_sdk.py
```

Dry run:

```powershell
python .\scripts\upload_kaggle_kernel_sdk.py --dry-run
```

The kernel currently validates the uploaded dataset, summarizes benchmark and corpus files, and runs a lightweight retrieval sanity check without model weights.

## Push Rust Cyber Memory LoRA Kernel

The LoRA training kernel lives at:

```text
C:\Projects\small-model-memory-lab\kaggle_notebooks\lora_train
```

Push or update it with:

```powershell
python .\scripts\upload_kaggle_kernel_sdk.py --kernel-dir .\kaggle_notebooks\lora_train
```

The first version is intentionally safe by default:

- `TRAINING_ENABLED = False`
- `RUN_LORA_TRAINING=1` can override that constant
- setup mode validates the dataset, loads `corpus/rust_cyber_defender_generated.tsv`, builds train examples, writes `training_examples_preview.json`, and exits

When ready to spend GPU time, edit the Kaggle script and set:

```python
TRAINING_ENABLED = True
```

Default LoRA target:

```text
Qwen/Qwen3-1.7B-Base
```

Override with:

```text
BASE_MODEL_ID=<huggingface model id>
```

Training outputs:

- `/kaggle/working/rust-cyber-memory-lora`
- `/kaggle/working/lora_eval_results.json`
- `/kaggle/working/training_examples_preview.json`

The built-in eval compares base vs LoRA on prompt-injection probes covering quoted text, tool output, webpage task replacement, README/env bait, HTML comments, markdown titles, diff comments, transcript role impersonation, and benign safety explanations.

First completed run:

- Kernel version: `4`
- Base model: `Qwen/Qwen3-1.7B-Base`
- Memories: `360`
- SFT examples: `720`
- Steps: `160`
- Base score: `1/12`, attacks `0/8`, benign `1/4`
- LoRA score: `7/12`, attacks `3/8`, benign `4/4`
- Lift over base: `+6` total, `+3` attacks, `+3` benign
- Local report: `C:\Projects\small-model-memory-lab\runs\kaggle-lora-v4\lora_eval_results.json`
- Local adapter: `C:\Projects\small-model-memory-lab\runs\kaggle-lora-v4\rust-cyber-memory-lora`

## Push Instruct Baseline Kernel

The instruct baseline kernel lives at:

```text
C:\Projects\small-model-memory-lab\kaggle_notebooks\instruct_eval
```

Push or update it with:

```powershell
python .\scripts\upload_kaggle_kernel_sdk.py --kernel-dir .\kaggle_notebooks\instruct_eval
```

Default target:

```text
Qwen/Qwen3-1.7B
```

Override with:

```text
INSTRUCT_MODEL_ID=<huggingface instruct model id>
```

First completed run:

- Kernel version: `1`
- Instruct model: `Qwen/Qwen3-1.7B`
- Memory corpus: none
- Strict score: `3/12`, attacks `0/8`, benign `3/4`
- Behavioral score: `3/12`, attacks `0/8`, benign `3/4`
- Local report: `C:\Projects\small-model-memory-lab\runs\kaggle-instruct-baseline-v1\instruct_baseline_eval_results.json`

Current apples-to-apples strict comparison on the 12-probe cyber set:

| Model | Memory/Training | Total | Attacks | Benign |
|---|---|---:|---:|---:|
| `Qwen/Qwen3-1.7B` | instruct, no memories | `3/12` | `0/8` | `3/4` |
| `Qwen/Qwen3-1.7B-Base` | cyber-memory LoRA | `7/12` | `3/8` | `4/4` |

Important caveat: the LoRA outputs still over-recite memory-shaped text. The initial strict scorer also counts safe mentions of dangerous terms as leaks, so it can undercount safe refusals. Treat this as a first confirmation signal, not a final benchmark.

## Push Instruct Plus LoRA Kernel

The instruct-plus-LoRA kernel lives at:

```text
C:\Projects\small-model-memory-lab\kaggle_notebooks\instruct_lora_eval
```

Push or update it with:

```powershell
python .\scripts\upload_kaggle_kernel_sdk.py --kernel-dir .\kaggle_notebooks\instruct_lora_eval
```

Default target:

```text
Qwen/Qwen3-1.7B
```

Adapter source:

```text
jessealicea/rust-cyber-memory-lora-train
```

First completed run:

- Kernel version: `1`
- Instruct model: `Qwen/Qwen3-1.7B`
- Adapter: cyber-memory LoRA trained from `Qwen/Qwen3-1.7B-Base`
- Strict score: `5/12`, attacks `1/8`, benign `4/4`
- Behavioral score: `7/12`, attacks `4/8`, benign `3/4`
- Local report: `C:\Projects\small-model-memory-lab\runs\kaggle-instruct-plus-lora-v1\instruct_plus_lora_eval_results.json`

Three-way comparison on the 12-probe cyber set:

| Model | Setup | Strict | Behavioral |
|---|---|---:|---:|
| `Qwen/Qwen3-1.7B` | instruct, no memories | `3/12` | `3/12` |
| `Qwen/Qwen3-1.7B-Base` | base plus cyber-memory LoRA | `7/12` | `12/12` |
| `Qwen/Qwen3-1.7B` | instruct plus cyber-memory LoRA | `5/12` | `7/12` |

Interpretation: attaching the base-trained adapter to the instruct model improves over instruct-only, especially on attacks, but it underperforms the adapter on the base model. The likely reason is distribution mismatch: the adapter was trained on base-model continuations, then applied to an instruct chat model. The next fair test is to train a second LoRA directly on `Qwen/Qwen3-1.7B` instruct with the same corpus and compare instruct-only vs instruct-trained-LoRA.

## Refresh Existing Kaggle Dataset

After changing docs, scripts, corpora, or run summaries:

```powershell
python .\scripts\package_kaggle_dataset.py --owner jessealicea
python .\scripts\upload_kaggle_dataset_sdk.py --owner jessealicea --version-notes "Update small model memory lab results"
```

## Include All Runs

The default export includes only canonical evidence runs.

To include every run folder summary/response file:

```powershell
python .\scripts\package_kaggle_dataset.py --owner YOUR_KAGGLE_USERNAME --include-all-runs
```

This may create a larger dataset.

## Current Kaggle Dataset Framing

Title:

```text
Small Model Memory and Structural Primitive Lab
```

Subtitle:

```text
Synthetic memory and LoRA tests for compressed small models
```

Description:

```text
Small Model Memory Lab packages synthetic-memory corpora, structural primitive LoRA datasets, primitive-cache tests, activation-steering runs, training kernels, baselines, result summaries, and reproduction notes for testing whether generated memories and primitive structures can become trainable behavior in compressed small language models.
```

Current documented results:

- Qwen3 1.7B instruct, no memories: `3/12` strict and behavioral on the cyber set, attacks `0/8`
- Qwen3 1.7B Base plus cyber-memory LoRA: `7/12` strict, `12/12` behavioral, attacks `3/8` strict and `8/8` behavioral
- Qwen3 1.7B instruct plus the base-trained LoRA: `5/12` strict, `7/12` behavioral, attacks `1/8` strict and `4/8` behavioral
- Qwen3 4B Base QLoRA branch: P100 is intentionally rejected; run on T4/A100/L4-class cloud GPU because the goal is compressed-model training, not fp16 fallback

Important caveat:

The strict scorer counts safe mentions of dangerous phrases as leaks, so behavioral scores are included to distinguish safe explanation from actual compliance. The first LoRA also over-recites memory-shaped text; it is evidence of a trainable signal, not a finished model.

Earlier archived results:

- Qwen3.5 2B Q4_K_M prompt-memory computation lift: `50/67` to `58/67`, repeated at `51/67` to `58/67`
- llama.cpp primitive prefix-cache raw-completion lift: `29/67` to `36/67`
- Qwen3 1.7B residual activation steering lift: `38/67` to `43/67`
