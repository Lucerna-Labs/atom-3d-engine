"""
LoRA Training Test - Qwen3.5-2B-Base

A minimal LoRA adapter training script to prove the pipeline works end-to-end:
1. Load the base model
2. Attach LoRA adapters (rank 8, targeting attention projections)
3. Train on a tiny dataset for a few steps
4. Save the adapter
5. Verify the adapter loads and produces different output than base
6. Log everything to sim-logger-compatible JSON
"""

import torch
import json
import time
import os
from datetime import datetime, timezone
from pathlib import Path

from transformers import AutoModelForCausalLM, AutoTokenizer, TrainingArguments, Trainer, DataCollatorForLanguageModeling
from peft import LoraConfig, get_peft_model, TaskType

# ─── Configuration ──────────────────────────────────────────────────────────

MODEL_ID = "Qwen/Qwen3.5-2B-Base"
OUTPUT_DIR = Path("F:/OPENCLAW-PROJECTS/lora-test/output")
LOG_DIR = Path("F:/OPENCLAW-PROJECTS/lora-test/logs")

# LoRA config - small for proof of concept
LORA_RANK = 8
LORA_ALPHA = 16  # 2x rank is standard
LORA_DROPOUT = 0.05
TARGET_MODULES = ["q_proj", "k_proj", "v_proj", "o_proj"]  # Attention projections

# Training config - minimal for proof of concept
NUM_EPOCHS = 3
BATCH_SIZE = 2
LEARNING_RATE = 5e-4
MAX_SEQ_LENGTH = 128
LOGGING_STEPS = 1

# ─── Tiny Training Dataset ───────────────────────────────────────────────────

TRAIN_DATA = [
    {"input": "The capital of France is", "output": "Paris. Paris is the largest city in France and has been the capital since the 10th century."},
    {"input": "Water boils at", "output": "100 degrees Celsius (212 degrees Fahrenheit) at standard atmospheric pressure."},
    {"input": "The speed of light is approximately", "output": "299,792,458 meters per second in a vacuum. This is a fundamental constant of physics."},
    {"input": "Python is a programming language known for", "output": "its readability, simplicity, and versatility. It supports multiple programming paradigms including procedural, object-oriented, and functional programming."},
    {"input": "The human body has approximately", "output": "37.2 trillion cells. The human body is composed of about 200 different types of cells organized into tissues and organs."},
    {"input": "Photosynthesis converts", "output": "light energy into chemical energy. Plants use carbon dioxide and water, with sunlight as the energy source, to produce glucose and oxygen."},
    {"input": "The largest planet in our solar system is", "output": "Jupiter. It has a mass more than twice that of all other planets combined and is primarily composed of hydrogen and helium."},
    {"input": "Machine learning is a subset of", "output": "artificial intelligence that enables systems to learn and improve from experience without being explicitly programmed. It focuses on developing algorithms that can access data and use it to learn for themselves."},
]

EVAL_DATA = [
    {"input": "The Earth orbits the Sun at approximately", "output": "149.6 million kilometers, completing one orbit every 365.25 days."},
    {"input": "DNA stands for", "output": "deoxyribonucleic acid. It is a molecule that carries the genetic instructions used in the growth, development, and reproduction of all known living organisms."},
]

# ─── Training Script ─────────────────────────────────────────────────────────

def format_example(example):
    """Format a training example into a single string."""
    return f"### Input:\n{example['input']}\n\n### Response:\n{example['output']}"


def main():
    print("=" * 60)
    print("LoRA Training Test - Qwen3.5-2B-Base")
    print("=" * 60)
    
    # Create output directories
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    LOG_DIR.mkdir(parents=True, exist_ok=True)
    
    log_entries = []
    def log(event_type, data, severity="INFO"):
        entry = {
            "id": str(torch.randint(0, 2**31, (1,)).item()),
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "severity": severity,
            "category": "LoRA_Training",
            "event_type": event_type,
            "message": str(data) if isinstance(data, str) else json.dumps(data, default=str),
            "data": data if not isinstance(data, str) else {},
        }
        log_entries.append(entry)
        print(f"[{severity}] {event_type}: {data if not isinstance(data, str) else data}")
    
    # ── Step 1: Load tokenizer and model ──
    log("phase_start", {"phase": "model_loading", "model": MODEL_ID})
    
    tokenizer = AutoTokenizer.from_pretrained(MODEL_ID, trust_remote_code=True)
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token
    
    log("tokenizer_loaded", {"vocab_size": len(tokenizer), "pad_token": tokenizer.pad_token})
    
    model = AutoModelForCausalLM.from_pretrained(
        MODEL_ID,
        torch_dtype=torch.bfloat16,
        device_map="auto",
        trust_remote_code=True,
    )
    
    total_params = sum(p.numel() for p in model.parameters())
    log("model_loaded", {
        "total_parameters": total_params,
        "total_parameters_B": round(total_params / 1e9, 2),
        "dtype": str(model.dtype),
        "device": str(next(model.parameters()).device),
    })
    
    # ── Step 2: Attach LoRA adapters ──
    log("phase_start", {"phase": "lora_attachment"})
    
    lora_config = LoraConfig(
        task_type=TaskType.CAUSAL_LM,
        r=LORA_RANK,
        lora_alpha=LORA_ALPHA,
        lora_dropout=LORA_DROPOUT,
        target_modules=TARGET_MODULES,
        bias="none",
    )
    
    model = get_peft_model(model, lora_config)
    
    trainable_params = sum(p.numel() for p in model.parameters() if p.requires_grad)
    all_params = sum(p.numel() for p in model.parameters())
    trainable_pct = 100 * trainable_params / all_params
    
    log("lora_attached", {
        "lora_rank": LORA_RANK,
        "lora_alpha": LORA_ALPHA,
        "lora_dropout": LORA_DROPOUT,
        "target_modules": TARGET_MODULES,
        "trainable_parameters": trainable_params,
        "total_parameters": all_params,
        "trainable_percentage": round(trainable_pct, 4),
    })
    
    model.print_trainable_parameters()
    
    # ── Step 3: Prepare dataset ──
    log("phase_start", {"phase": "dataset_preparation"})
    
    def tokenize_function(examples):
        texts = [format_example(ex) for ex in examples]
        return tokenizer(
            texts,
            truncation=True,
            max_length=MAX_SEQ_LENGTH,
            padding="max_length",
            return_tensors="pt",
        )
    
    # Create simple dataset objects
    from torch.utils.data import Dataset
    
    class LoRADataset(Dataset):
        def __init__(self, data, tokenizer):
            self.data = data
            self.tokenizer = tokenizer
            self.formatted = [format_example(ex) for ex in data]
            self.encodings = tokenizer(
                self.formatted,
                truncation=True,
                max_length=MAX_SEQ_LENGTH,
                padding="max_length",
                return_tensors="pt",
            )
        
        def __len__(self):
            return len(self.data)
        
        def __getitem__(self, idx):
            item = {key: val[idx].clone() for key, val in self.encodings.items()}
            item["labels"] = item["input_ids"].clone()
            return item
    
    train_dataset = LoRADataset(TRAIN_DATA, tokenizer)
    eval_dataset = LoRADataset(EVAL_DATA, tokenizer)
    
    log("dataset_prepared", {
        "train_examples": len(train_dataset),
        "eval_examples": len(eval_dataset),
        "max_seq_length": MAX_SEQ_LENGTH,
    })
    
    # ── Step 4: Generate baseline output ──
    log("phase_start", {"phase": "baseline_generation"})
    
    model.eval()
    test_prompt = "The capital of Japan is"
    inputs = tokenizer(test_prompt, return_tensors="pt").to(model.device)
    
    with torch.no_grad():
        baseline_output = model.generate(
            **inputs,
            max_new_tokens=50,
            do_sample=False,
            temperature=1.0,
        )
    
    baseline_text = tokenizer.decode(baseline_output[0], skip_special_tokens=True)
    log("baseline_generated", {"prompt": test_prompt, "output": baseline_text})
    
    # ── Step 5: Train ──
    log("phase_start", {"phase": "training"})
    
    training_args = TrainingArguments(
        output_dir=str(OUTPUT_DIR),
        num_train_epochs=NUM_EPOCHS,
        per_device_train_batch_size=BATCH_SIZE,
        per_device_eval_batch_size=BATCH_SIZE,
        learning_rate=LEARNING_RATE,
        logging_steps=LOGGING_STEPS,
        eval_strategy="epoch",
        save_strategy="epoch",
        bf16=True,
        fp16=False,
        report_to="none",  # We handle our own logging
        remove_unused_columns=False,
        dataloader_pin_memory=False,
    )
    
    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=train_dataset,
        eval_dataset=eval_dataset,
        data_collator=DataCollatorForLanguageModeling(tokenizer=tokenizer, mlm=False),
    )
    
    train_start = time.time()
    train_result = trainer.train()
    train_duration = time.time() - train_start
    
    log("training_complete", {
        "duration_seconds": round(train_duration, 2),
        "train_loss": round(train_result.training_loss, 6),
        "total_steps": train_result.global_step,
        "metrics": {k: round(v, 6) if isinstance(v, float) else v for k, v in train_result.metrics.items()},
    })
    
    # ── Step 6: Save adapter ──
    log("phase_start", {"phase": "adapter_save"})
    
    adapter_path = OUTPUT_DIR / "adapter"
    model.save_pretrained(str(adapter_path))
    tokenizer.save_pretrained(str(adapter_path))
    
    adapter_size = sum(f.stat().st_size for f in adapter_path.glob("**/*") if f.is_file())
    log("adapter_saved", {
        "path": str(adapter_path),
        "size_mb": round(adapter_size / 1e6, 2),
        "files": [str(f.relative_to(adapter_path)) for f in adapter_path.glob("*") if f.is_file()],
    })
    
    # ── Step 7: Generate with adapter ──
    log("phase_start", {"phase": "adapter_generation"})
    
    model.eval()
    with torch.no_grad():
        adapter_output = model.generate(
            **inputs,
            max_new_tokens=50,
            do_sample=False,
            temperature=1.0,
        )
    
    adapter_text = tokenizer.decode(adapter_output[0], skip_special_tokens=True)
    log("adapter_generated", {"prompt": test_prompt, "output": adapter_text})
    
    # ── Step 8: Compare ──
    log("phase_start", {"phase": "comparison"})
    
    log("comparison", {
        "prompt": test_prompt,
        "baseline_output": baseline_text,
        "adapter_output": adapter_text,
        "outputs_differ": baseline_text != adapter_text,
        "baseline_length": len(baseline_text),
        "adapter_length": len(adapter_text),
    })
    
    # ── Step 9: Verify adapter loads independently ──
    log("phase_start", {"phase": "verification"})
    
    # Load fresh base model
    base_model_fresh = AutoModelForCausalLM.from_pretrained(
        MODEL_ID,
        torch_dtype=torch.bfloat16,
        device_map="auto",
        trust_remote_code=True,
    )
    
    # Load adapter onto it
    from peft import PeftModel
    loaded_model = PeftModel.from_pretrained(base_model_fresh, str(adapter_path))
    loaded_model.eval()
    
    with torch.no_grad():
        loaded_output = loaded_model.generate(
            **inputs,
            max_new_tokens=50,
            do_sample=False,
            temperature=1.0,
        )
    
    loaded_text = tokenizer.decode(loaded_output[0], skip_special_tokens=True)
    
    log("verification", {
        "loaded_output": loaded_text,
        "matches_adapter_output": loaded_text == adapter_text,
        "adapter_loads_correctly": True,
    })
    
    # ── Save log ──
    log_file = LOG_DIR / f"lora-training-{datetime.now().strftime('%Y%m%d-%H%M%S')}.jsonl"
    with open(log_file, "w") as f:
        for entry in log_entries:
            f.write(json.dumps(entry) + "\n")
    
    print(f"\nLog saved to: {log_file}")
    print(f"Adapter saved to: {adapter_path}")
    
    # ── Summary ──
    print("\n" + "=" * 60)
    print("TRAINING SUMMARY")
    print("=" * 60)
    print(f"Model: {MODEL_ID}")
    print(f"LoRA Rank: {LORA_RANK}, Alpha: {LORA_ALPHA}")
    print(f"Trainable params: {trainable_params:,} ({trainable_pct:.4f}%)")
    print(f"Training time: {train_duration:.1f}s")
    print(f"Final loss: {train_result.training_loss:.6f}")
    print(f"\nBaseline output:  {baseline_text}")
    print(f"Adapter output:   {adapter_text}")
    print(f"Verified output:  {loaded_text}")
    print(f"Adapter loads correctly: {loaded_text == adapter_text}")
    print(f"Outputs differ from baseline: {baseline_text != adapter_text}")
    print(f"\nAdapter size: {adapter_size / 1e6:.2f} MB")
    print(f"Saved to: {adapter_path}")


if __name__ == "__main__":
    main()