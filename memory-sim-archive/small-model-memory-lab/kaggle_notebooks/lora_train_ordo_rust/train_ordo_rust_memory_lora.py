#!/usr/bin/env python
"""Train an Ordo Rust memory LoRA on real Cargo-tested build artifacts.

This notebook trains a PEFT adapter from the hardened Ordo memory corpus and
successful real Rust build outputs. It is separate from the cyber and structural
primitive adapters: the goal here is a narrow Ordo/Rust builder instinct.
"""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import zipfile
from pathlib import Path
from typing import Any


os.environ.setdefault("TRANSFORMERS_NO_TORCHVISION", "1")
os.environ.setdefault("TRANSFORMERS_NO_TF", "1")
os.environ.setdefault("TRANSFORMERS_NO_FLAX", "1")
os.environ.setdefault("PYTORCH_CUDA_ALLOC_CONF", "expandable_segments:True")

TRAINING_ENABLED = True
MODEL_ID = os.environ.get("BASE_MODEL_ID", "Qwen/Qwen3-1.7B-Base")
DATASET_NAME = "ordo_rust_memory_lora_v0_2"

WORKING_DIR = Path("/kaggle/working") if Path("/kaggle/working").exists() else Path.cwd() / "runs" / "ordo-rust-lora-local"
WORKING_DIR.mkdir(parents=True, exist_ok=True)

OUTPUT_DIR = WORKING_DIR / "ordo-rust-memory-lora-v0-3"
REPORT_PATH = WORKING_DIR / "ordo_rust_lora_eval_results.json"
PREVIEW_PATH = WORKING_DIR / "ordo_rust_training_preview.json"

MAX_SEQ_LENGTH = int(os.environ.get("MAX_SEQ_LENGTH", "1536"))
MAX_STEPS = int(os.environ.get("MAX_STEPS", "90"))
EVAL_MAX_NEW_TOKENS = int(os.environ.get("EVAL_MAX_NEW_TOKENS", "260"))

EVAL_PROBES = [
    {
        "label": "mini runtime api",
        "prompt": "Build the Ordo mini_runtime library API. Include the publish signature, Trace fields, preload_targets, route_for_budget, and backpressure signal shape.",
        "required": ["publish", "topic", "payload", "Trace", "thread_id", "preload_targets", "route_for_budget", "backpressure:model"],
    },
    {
        "label": "scheduler contract",
        "prompt": "Write the core Rust contract for the Ordo backpressure_scheduler. Include Plan fields and the schedule method signature.",
        "required": ["Plan", "task_id", "promoted", "signals", "schedule", "candidates", "max_cost", "backpressure"],
    },
    {
        "label": "signal mesh behavior",
        "prompt": "Describe and sketch the Rust behavior for SignalMesh vertical and horizontal propagation in Ordo.",
        "required": ["SignalTrace", "vertical", "horizontal", "origin", "affected", "message", "flow", "peer"],
    },
    {
        "label": "retry polarity",
        "prompt": "Implement the key logic for RetryRuntime where false means failed and true means healthy.",
        "required": ["RetryPlan", "task_id", "false", "failed", "true", "healthy", "retry:local", "None"],
    },
    {
        "label": "audit timeline fields",
        "prompt": "Build the AuditTimeline memory: exact AuditEvent fields, record signature, events_for_thread return, and CLI show output shape.",
        "required": ["AuditEvent", "index", "thread_id", "record", "events_for_thread", "Vec", "audit=", "route@input"],
    },
    {
        "label": "preload bfs",
        "prompt": "Write the PreloadPlanner instinct for breadth-first preload_from behavior and the borrow-safe neighbor loop.",
        "required": ["preload_from", "VecDeque", "depth", "a", "b", "c", "d", "clone"],
    },
]


def should_train() -> bool:
    value = os.environ.get("RUN_LORA_TRAINING")
    if value is not None:
        return value.strip().lower() in {"1", "true", "yes", "y"}
    return TRAINING_ENABLED


def install_if_needed() -> None:
    packages = ["torch", "transformers", "datasets", "accelerate", "peft"]
    missing = []
    for package in packages:
        module = package.replace("-", "_")
        try:
            __import__(module)
        except ImportError:
            missing.append(package)
    if missing:
        subprocess.check_call([sys.executable, "-m", "pip", "install", "-q", *missing])

    gpu_capability = ""
    try:
        gpu_capability = subprocess.check_output(
            ["nvidia-smi", "--query-gpu=compute_cap", "--format=csv,noheader"],
            text=True,
        ).strip().splitlines()[0]
    except Exception:
        gpu_capability = ""

    if gpu_capability.startswith("6.") and os.environ.get("P100_TORCH_RESTARTED") != "1":
        print("Detected P100-class GPU. Installing a PyTorch CUDA wheel that supports sm_60.")
        subprocess.check_call([
            sys.executable,
            "-m",
            "pip",
            "install",
            "-q",
            "--force-reinstall",
            "--index-url",
            "https://download.pytorch.org/whl/cu121",
            "torch==2.5.1",
        ])
        subprocess.call([sys.executable, "-m", "pip", "uninstall", "-y", "-q", "torchvision", "torchaudio", "torchtext"])
        os.environ["P100_TORCH_RESTARTED"] = "1"
        os.execv(sys.executable, [sys.executable, *sys.argv])


def find_dataset_root() -> Path:
    candidates: list[Path] = []
    extracted_roots: list[Path] = []
    for base in [Path("/kaggle/input"), Path.cwd()]:
        if not base.exists():
            continue
        candidates.extend(base.rglob(f"data/{DATASET_NAME}/train.jsonl"))
        candidates.extend(base.rglob(f"{DATASET_NAME}/train.jsonl"))
        for archive in base.rglob("data.zip"):
            extract_root = WORKING_DIR / "extracted_data_zip"
            marker = extract_root / DATASET_NAME / "train.jsonl"
            if not marker.exists():
                extract_root.mkdir(parents=True, exist_ok=True)
                with zipfile.ZipFile(archive, "r") as handle:
                    handle.extractall(extract_root)
            extracted_roots.append(extract_root)
    for root in extracted_roots:
        candidates.extend(root.rglob(f"{DATASET_NAME}/train.jsonl"))
    if not candidates:
        raise FileNotFoundError(f"Could not find {DATASET_NAME}/train.jsonl in Kaggle inputs.")
    candidates.sort(key=lambda path: len(path.as_posix()))
    return candidates[0].parent


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    rows = []
    with path.open("r", encoding="utf-8") as handle:
        for line in handle:
            if line.strip():
                rows.append(json.loads(line))
    return rows


def load_examples(dataset_root: Path) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    return read_jsonl(dataset_root / "train.jsonl"), read_jsonl(dataset_root / "validation.jsonl")


def write_preview(dataset_root: Path, train: list[dict[str, Any]], validation: list[dict[str, Any]]) -> None:
    preview = {
        "dataset_root": str(dataset_root),
        "model_id": MODEL_ID,
        "training_enabled": should_train(),
        "train_count": len(train),
        "validation_count": len(validation),
        "first_train_examples": train[:3],
        "output_dir": str(OUTPUT_DIR),
        "report_path": str(REPORT_PATH),
        "max_steps": MAX_STEPS,
        "max_seq_length": MAX_SEQ_LENGTH,
    }
    PREVIEW_PATH.write_text(json.dumps(preview, indent=2), encoding="utf-8")
    print(json.dumps(preview, indent=2)[:4000])
    print(f"Preview written to {PREVIEW_PATH}")


def tokenize_examples(train: list[dict[str, Any]], validation: list[dict[str, Any]], tokenizer: Any) -> Any:
    from datasets import Dataset, DatasetDict

    eos = tokenizer.eos_token or ""
    dataset = DatasetDict({
        "train": Dataset.from_dict({"text": [row["text"] + eos for row in train]}),
        "validation": Dataset.from_dict({"text": [row["text"] + eos for row in validation]}),
    })

    def tokenize(batch: dict[str, list[str]]) -> dict[str, Any]:
        return tokenizer(batch["text"], truncation=True, max_length=MAX_SEQ_LENGTH, padding=False)

    return dataset.map(tokenize, batched=True, remove_columns=["text"])


def load_model_and_tokenizer() -> tuple[Any, Any]:
    import torch
    from transformers import AutoModelForCausalLM, AutoTokenizer

    tokenizer = AutoTokenizer.from_pretrained(MODEL_ID, trust_remote_code=True)
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token

    quantization_config = None
    dtype = torch.float32
    if torch.cuda.is_available():
        major, _minor = torch.cuda.get_device_capability()
        dtype = torch.bfloat16 if torch.cuda.is_bf16_supported() else torch.float16
        if major >= 7:
            try:
                from transformers import BitsAndBytesConfig
                quantization_config = BitsAndBytesConfig(
                    load_in_4bit=True,
                    bnb_4bit_compute_dtype=dtype,
                    bnb_4bit_use_double_quant=True,
                    bnb_4bit_quant_type="nf4",
                )
            except Exception:
                print("bitsandbytes unavailable; loading without 4-bit quantization.")
                quantization_config = None
        else:
            print("GPU compute capability is below 7.0; loading fp16 without bitsandbytes.")

    model = AutoModelForCausalLM.from_pretrained(
        MODEL_ID,
        trust_remote_code=True,
        device_map="auto",
        torch_dtype=dtype,
        quantization_config=quantization_config,
    )
    return model, tokenizer


def generate(model: Any, tokenizer: Any, prompt: str) -> str:
    import torch

    text = f"### User\n{prompt}\n\n### Assistant\n"
    inputs = tokenizer(text, return_tensors="pt").to(model.device)
    with torch.no_grad():
        generated = model.generate(
            **inputs,
            max_new_tokens=EVAL_MAX_NEW_TOKENS,
            do_sample=False,
            pad_token_id=tokenizer.eos_token_id,
        )
    return tokenizer.decode(generated[0][inputs["input_ids"].shape[-1]:], skip_special_tokens=True).strip()


def score_probe(probe: dict[str, Any], output: str) -> dict[str, Any]:
    text = output.lower()
    hits = [term for term in probe["required"] if re.search(re.escape(term.lower()), text)]
    passed = len(hits) >= max(5, len(probe["required"]) - 1)
    return {
        "label": probe["label"],
        "passed": bool(passed),
        "required_hits": len(hits),
        "required_total": len(probe["required"]),
        "hits": hits,
        "output": output,
    }


def evaluate(model: Any, tokenizer: Any, label: str) -> dict[str, Any]:
    rows = []
    for probe in EVAL_PROBES:
        rows.append(score_probe(probe, generate(model, tokenizer, probe["prompt"])))
    return {"label": label, "passed": sum(1 for row in rows if row["passed"]), "total": len(rows), "rows": rows}


def train_lora(train: list[dict[str, Any]], validation: list[dict[str, Any]]) -> None:
    import inspect
    import torch
    from peft import LoraConfig, TaskType, get_peft_model, prepare_model_for_kbit_training
    from transformers import DataCollatorForLanguageModeling, Trainer, TrainingArguments

    model, tokenizer = load_model_and_tokenizer()
    base_eval = evaluate(model, tokenizer, "base")

    model = prepare_model_for_kbit_training(model)
    try:
        model.gradient_checkpointing_enable()
    except Exception:
        pass

    config = LoraConfig(
        task_type=TaskType.CAUSAL_LM,
        r=8,
        lora_alpha=16,
        lora_dropout=0.05,
        target_modules=["q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj"],
    )
    model = get_peft_model(model, config)
    model.print_trainable_parameters()

    tokenized = tokenize_examples(train, validation, tokenizer)
    collator = DataCollatorForLanguageModeling(tokenizer=tokenizer, mlm=False)

    training_kwargs = {
        "output_dir": str(OUTPUT_DIR),
        "max_steps": MAX_STEPS,
        "per_device_train_batch_size": 1,
        "per_device_eval_batch_size": 1,
        "gradient_accumulation_steps": 8,
        "learning_rate": 1e-4,
        "warmup_steps": 10,
        "logging_steps": 10,
        "eval_steps": 35,
        "save_steps": 70,
        "save_total_limit": 2,
        "bf16": torch.cuda.is_available() and torch.cuda.is_bf16_supported(),
        "fp16": torch.cuda.is_available() and not torch.cuda.is_bf16_supported(),
        "report_to": [],
    }
    if "eval_strategy" in inspect.signature(TrainingArguments.__init__).parameters:
        training_kwargs["eval_strategy"] = "steps"
    else:
        training_kwargs["evaluation_strategy"] = "steps"
    args = TrainingArguments(**training_kwargs)

    trainer = Trainer(
        model=model,
        args=args,
        train_dataset=tokenized["train"],
        eval_dataset=tokenized["validation"],
        data_collator=collator,
    )
    trainer.train()
    trainer.save_model(str(OUTPUT_DIR))
    tokenizer.save_pretrained(str(OUTPUT_DIR))

    lora_eval = evaluate(model, tokenizer, "lora")
    report = {
        "model_id": MODEL_ID,
        "dataset": DATASET_NAME,
        "train_count": len(train),
        "validation_count": len(validation),
        "max_steps": MAX_STEPS,
        "max_seq_length": MAX_SEQ_LENGTH,
        "output_dir": str(OUTPUT_DIR),
        "base_eval": base_eval,
        "lora_eval": lora_eval,
        "lift_total": lora_eval["passed"] - base_eval["passed"],
    }
    REPORT_PATH.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))
    print(f"LoRA adapter saved to {OUTPUT_DIR}")
    print(f"Evaluation report saved to {REPORT_PATH}")


def main() -> int:
    dataset_root = find_dataset_root()
    train, validation = load_examples(dataset_root)
    write_preview(dataset_root, train, validation)
    if not should_train():
        print("Training disabled. Set RUN_LORA_TRAINING=1 to train.")
        return 0
    install_if_needed()
    train_lora(train, validation)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
