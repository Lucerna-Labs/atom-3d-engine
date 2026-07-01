"""De-risk probe for activation-steering on Qwen3.5 base (HF + forward hooks).

Loads the model, locates the text-decoder layer stack, runs a few dev tasks with
greedy decoding through the campaign's prompt format + graders, and reports the
no-steering baseline. No steering yet -- just proves load/generate/grade + finds
the layer path we will hook.
"""
from __future__ import annotations
import os, sys, torch
import torch.nn as nn

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import run_general_corpus_campaign as camp  # noqa: E402

MODEL_ID = os.environ.get("STEER_MODEL", "Qwen/Qwen3.5-2B-Base")
N_TASKS = int(os.environ.get("STEER_PROBE_N", "6"))

from transformers import AutoTokenizer  # noqa: E402

print(f"loading tokenizer: {MODEL_ID}")
tok = AutoTokenizer.from_pretrained(MODEL_ID)
if tok.pad_token_id is None:
    tok.pad_token = tok.eos_token


def load_model():
    last = None
    for loader_name in ("AutoModelForCausalLM", "AutoModelForImageTextToText", "AutoModel"):
        try:
            import transformers
            loader = getattr(transformers, loader_name)
            m = loader.from_pretrained(MODEL_ID, dtype=torch.bfloat16, device_map="cuda")
            print(f"loaded via {loader_name}")
            return m
        except Exception as e:  # noqa: BLE001
            last = e
            print(f"  {loader_name} failed: {repr(e)[:160]}")
    raise last


model = load_model()
model.eval()

# Find the text decoder layer stack: the longest nn.ModuleList of transformer blocks.
candidates = []
for name, mod in model.named_modules():
    if isinstance(mod, nn.ModuleList) and len(mod) >= 8:
        candidates.append((name, len(mod)))
candidates.sort(key=lambda x: -x[1])
print("decoder-layer ModuleList candidates (name, count):", candidates[:5])
LAYERS_PATH = candidates[0][0] if candidates else None
print("USING LAYERS_PATH:", LAYERS_PATH)


@torch.no_grad()
def generate(prompt_text: str, max_new: int = 200) -> str:
    ids = tok(prompt_text, return_tensors="pt").to(model.device)
    out = model.generate(
        **ids, max_new_tokens=max_new, do_sample=False,
        pad_token_id=tok.pad_token_id, use_cache=True,
    )
    cont = tok.decode(out[0][ids["input_ids"].shape[1]:], skip_special_tokens=True)
    return cont


def clean_cut(cont: str) -> str:
    # emulate the Ollama stop sequences: cut at the next section marker
    for marker in ("\n###", "\n### ", "\nTask:", "\n\n###"):
        i = cont.find(marker)
        if i != -1:
            cont = cont[:i]
    return camp.as_clean(cont)


tasks = camp.DEV_TASKS[:N_TASKS]
passed = 0
for t in tasks:
    prompt = camp.build_prompt(t, "")  # generate-format, no memory
    raw = generate(prompt)
    ans = clean_cut(raw)
    ok, _ = t.grader(ans)
    passed += int(ok)
    show = ans[:60].replace("\n", " / ")
    print(f"[{ 'PASS' if ok else 'FAIL'}] {t.task_id:<30} -> {show!r}")
print(f"\nBASELINE (no steering): {passed}/{len(tasks)} on first {len(tasks)} dev tasks")
