"""Evaluate the Ordo LoRA on the 5 HELD-OUT apps with NO memory in context.
Loads Qwen3.5-4B-Base (+ optional LoRA adapter), generates each held-out spec,
writes the crate, runs real cargo test. The honest test of a baked-in, transferable
Ordo-builder.

Env: LORA_BASE (default Qwen/Qwen3.5-4B-Base), LORA_OUT (adapter dir),
     EVAL_ADAPTER (1=use adapter [default], 0=base-only control),
     EVAL_MAXNEW (3000), EVAL_REPAIRS (0 = single-shot builder only).
"""
from __future__ import annotations
import os, sys, time
from pathlib import Path
import torch
from transformers import AutoTokenizer, AutoModelForCausalLM
from peft import PeftModel

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
os.environ.setdefault("ORDO_EXT_MODEL", "x")
import run_ordo_extended_suite_benchmark as base  # noqa: E402
import run_ordo_gist_eval as ge  # noqa: E402  (provides HOLDOUT tasks)

BASE = os.environ.get("LORA_BASE", "Qwen/Qwen3.5-4B-Base")
ADAPTER = os.environ.get("LORA_OUT", str(base.ROOT / "ordo_lora_adapter"))
USE_ADAPTER = os.environ.get("EVAL_ADAPTER", "1").strip().lower() not in {"0", "false", "no", ""}
MAXNEW = int(os.environ.get("EVAL_MAXNEW", "3000"))
REPAIRS = int(os.environ.get("EVAL_REPAIRS", "0"))

print(f"base={BASE} adapter={'ON ('+ADAPTER+')' if USE_ADAPTER else 'OFF (base-only control)'} repairs={REPAIRS}")
tok = AutoTokenizer.from_pretrained(BASE)
if tok.pad_token_id is None:
    tok.pad_token = tok.eos_token
model = AutoModelForCausalLM.from_pretrained(BASE, dtype=torch.bfloat16, device_map="cuda")
if USE_ADAPTER:
    model = PeftModel.from_pretrained(model, ADAPTER)
model.eval()


@torch.no_grad()
def gen(prompt: str) -> str:
    ids = tok(prompt, return_tensors="pt").to(model.device)
    out = model.generate(**ids, max_new_tokens=MAXNEW, do_sample=False, pad_token_id=tok.pad_token_id, use_cache=True)
    return tok.decode(out[0][ids["input_ids"].shape[1]:], skip_special_tokens=True)


stamp = time.strftime("%Y%m%d-%H%M%S")
tag = "adapter" if USE_ADAPTER else "baseonly"
run_dir = base.RUN_ROOT / f"ordo-lora-eval-{stamp}-{tag}"
rows = []
for task in ge.HOLDOUT:
    prompt = base.build_prompt(task, "")
    raw = gen(prompt)
    proj = base.extract_project(raw)
    crate = run_dir / task.task_id / "crate"
    base.write_project(crate, proj, task.tests)
    passed, cargo = base.run_cargo_test(crate)
    stage = "builder" if passed else "-"
    attempts = 1
    while not passed and attempts <= REPAIRS:
        raw = gen(base.repair_prompt(task, "", raw, cargo))
        proj = base.extract_project(raw)
        base.write_project(crate, proj, task.tests)
        passed, cargo = base.run_cargo_test(crate)
        attempts += 1
        if passed:
            stage = f"repair_{attempts - 1}"
    (run_dir / task.task_id).mkdir(parents=True, exist_ok=True)
    (run_dir / task.task_id / "raw.txt").write_text(raw, encoding="utf-8")
    (run_dir / task.task_id / "cargo.txt").write_text(cargo, encoding="utf-8")
    print(f"{task.task_id:<20} {'PASS' if passed else 'FAIL'}  stage={stage}")
    rows.append((task.task_id, passed, stage))

n = sum(1 for _, p, _ in rows if p)
b = sum(1 for _, p, s in rows if p and s == "builder")
print(f"\nHELD-OUT (no memory, adapter={'on' if USE_ADAPTER else 'off'}): {n}/{len(rows)} (builder {b})")
print(f"Run dir: {run_dir}")
