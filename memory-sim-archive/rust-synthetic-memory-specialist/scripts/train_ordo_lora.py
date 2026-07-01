"""Train a LoRA on Qwen3.5-4B-Base to build Ordo apps from a bare spec (the gist
baked into weights, not retrieved in context).

Data: data/ordo_sft.jsonl, one JSON per line {"prompt": <bare Ordo spec>, "completion":
<=== Cargo.toml ===/=== src/lib.rs ===/=== src/main.rs === solution>}. The 5 held-out
apps are NOT in this file.

Manual training loop (robust to transformers/trl API churn). bf16 + gradient
checkpointing to fit the 4B on 16GB. Completion-only loss (prompt tokens masked).

Env: LORA_BASE (HF repo, default Qwen/Qwen3.5-4B-Base), LORA_DATA, LORA_OUT,
     LORA_EPOCHS (3), LORA_LR (2e-4), LORA_R (16), LORA_MAXLEN (2048),
     LORA_GRAD_ACCUM (8), LORA_SMOKE (if set, run only N steps).
"""
from __future__ import annotations
import os, sys, json, random
from pathlib import Path
import torch
import torch.nn as nn
from transformers import AutoTokenizer, AutoModelForCausalLM
from peft import LoraConfig, get_peft_model

ROOT = Path(__file__).resolve().parents[1]
MODEL = os.environ.get("LORA_BASE", "Qwen/Qwen3.5-4B-Base")
DATA = Path(os.environ.get("LORA_DATA", str(ROOT / "data" / "ordo_sft.jsonl")))
OUT = Path(os.environ.get("LORA_OUT", str(ROOT / "ordo_lora_adapter")))
EPOCHS = int(os.environ.get("LORA_EPOCHS", "3"))
LR = float(os.environ.get("LORA_LR", "2e-4"))
R = int(os.environ.get("LORA_R", "16"))
MAXLEN = int(os.environ.get("LORA_MAXLEN", "2048"))
GRAD_ACCUM = int(os.environ.get("LORA_GRAD_ACCUM", "8"))
SMOKE = int(os.environ.get("LORA_SMOKE", "0"))

print(f"base={MODEL} data={DATA} out={OUT} epochs={EPOCHS} lr={LR} r={R} maxlen={MAXLEN}")
tok = AutoTokenizer.from_pretrained(MODEL)
if tok.pad_token_id is None:
    tok.pad_token = tok.eos_token

model = AutoModelForCausalLM.from_pretrained(MODEL, dtype=torch.bfloat16, device_map="cuda")
model.config.use_cache = False
model.gradient_checkpointing_enable()
if hasattr(model, "enable_input_require_grads"):
    model.enable_input_require_grads()

# Target the text-decoder projection linears (match by suffix, arch-agnostic).
SUFFIXES = ("q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj")
present = sorted({n.split(".")[-1] for n, m in model.named_modules()
                  if isinstance(m, nn.Linear) and n.split(".")[-1] in SUFFIXES})
if not present:
    present = sorted({n.split(".")[-1] for n, m in model.named_modules() if isinstance(m, nn.Linear)})
print("LoRA target modules:", present)

cfg = LoraConfig(r=R, lora_alpha=2 * R, lora_dropout=0.05, bias="none",
                 target_modules=present, task_type="CAUSAL_LM")
model = get_peft_model(model, cfg)
model.print_trainable_parameters()

examples = []
for line in DATA.read_text(encoding="utf-8").splitlines():
    line = line.strip()
    if line:
        examples.append(json.loads(line))
print(f"loaded {len(examples)} SFT examples")


def encode(ex):
    p = tok(ex["prompt"], add_special_tokens=True)["input_ids"]
    c = tok(ex["completion"] + tok.eos_token, add_special_tokens=False)["input_ids"]
    ids = (p + c)[:MAXLEN]
    labels = ([-100] * len(p) + c)[:MAXLEN]
    return ids, labels


model.train()
optim = torch.optim.AdamW([p for p in model.parameters() if p.requires_grad], lr=LR)
step = 0
for epoch in range(EPOCHS):
    random.seed(41 + epoch)
    random.shuffle(examples)
    running = 0.0
    n = 0
    optim.zero_grad()
    for i, ex in enumerate(examples):
        ids, labels = encode(ex)
        input_ids = torch.tensor([ids], device=model.device)
        lab = torch.tensor([labels], device=model.device)
        out = model(input_ids=input_ids, labels=lab)
        loss = out.loss / GRAD_ACCUM
        loss.backward()
        running += out.loss.item()
        n += 1
        if (i + 1) % GRAD_ACCUM == 0:
            optim.step()
            optim.zero_grad()
        step += 1
        if SMOKE and step >= SMOKE:
            print(f"SMOKE ok: {step} steps, last_loss={out.loss.item():.4f}")
            sys.exit(0)
    optim.step()
    optim.zero_grad()
    print(f"epoch {epoch}: avg_loss={running / max(1, n):.4f}")

OUT.mkdir(parents=True, exist_ok=True)
model.save_pretrained(str(OUT))
tok.save_pretrained(str(OUT))
print("saved adapter ->", OUT)
