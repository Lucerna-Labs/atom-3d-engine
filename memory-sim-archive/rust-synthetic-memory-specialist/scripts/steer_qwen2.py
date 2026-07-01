"""Activation steering v2 for Qwen3.5 base: ANSWER-MODE direction (thinking off).

Key insight (confirmed empirically): for these small base models, "thinking"
out loud is the failure mode -- answer-mode wins. So we steer toward answer-mode.

v2 improvements over steer_qwen.py:
  - Contrast = answer-mode framing vs think-mode framing.
  - Direction = diff-of-means averaged over the RESPONSE tokens the model actually
    generates under each framing (proper CAA), not just the last prompt token.
  - Finer, smaller alpha (fraction of per-layer residual norm) around L14.

Env: STEER_MODEL, STEER_LAYERS (default 12,14,16), STEER_ALPHAS (default 0.1,0.2,0.3,0.5),
     STEER_SUITES (dev,holdout), STEER_MAXNEW (eval gen cap, default 96),
     STEER_CAPNEW (capture gen length, default 40).
"""
from __future__ import annotations
import os, sys, time, json, csv
import torch
import torch.nn as nn

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import run_general_corpus_campaign as camp  # noqa: E402
from transformers import AutoTokenizer  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MODEL_ID = os.environ.get("STEER_MODEL", "Qwen/Qwen3.5-4B-Base")
LAYERS = [int(x) for x in os.environ.get("STEER_LAYERS", "12,14,16").split(",") if x.strip()]
ALPHAS = [float(x) for x in os.environ.get("STEER_ALPHAS", "0.1,0.2,0.3,0.5").split(",") if x.strip()]
SUITES = [s for s in os.environ.get("STEER_SUITES", "dev,holdout").split(",") if s.strip()]
MAXNEW = int(os.environ.get("STEER_MAXNEW", "96"))
CAPNEW = int(os.environ.get("STEER_CAPNEW", "40"))

print(f"model={MODEL_ID} layers={LAYERS} alphas={ALPHAS} suites={SUITES} (answer-mode CAA)")
tok = AutoTokenizer.from_pretrained(MODEL_ID)
if tok.pad_token_id is None:
    tok.pad_token = tok.eos_token


def load_model():
    import transformers
    last = None
    for name in ("AutoModelForCausalLM", "AutoModelForImageTextToText", "AutoModel"):
        try:
            m = getattr(transformers, name).from_pretrained(MODEL_ID, dtype=torch.bfloat16, device_map="cuda")
            print(f"loaded via {name}")
            return m
        except Exception as e:  # noqa: BLE001
            last = e
    raise last


model = load_model()
model.eval()
LAYERS_PATH = None
for nm, mod in model.named_modules():
    if isinstance(mod, nn.ModuleList) and len(mod) >= 8:
        LAYERS_PATH = nm
        break
layer_list = model.get_submodule(LAYERS_PATH)
print(f"decoder layers at '{LAYERS_PATH}' (n={len(layer_list)})")

ANSWER_SUFFIX = "\n\nAnswer with only the final value. Do not think out loud or explain."
THINK_SUFFIX = "\n\nThink through it step by step out loud, explaining your reasoning in detail, before you answer."


@torch.no_grad()
def gen_and_capture(prompt_text: str):
    """Generate a short continuation, then return mean hidden state over the
    response tokens for every layer (list len n_layers+1, each [hidden] float32)."""
    ids = tok(prompt_text, return_tensors="pt").to(model.device)
    plen = ids["input_ids"].shape[1]
    out = model.generate(**ids, max_new_tokens=CAPNEW, do_sample=False, pad_token_id=tok.pad_token_id, use_cache=True)
    full = out[0].unsqueeze(0)
    if full.shape[1] <= plen:  # no tokens generated
        full = torch.cat([full, torch.tensor([[tok.eos_token_id]], device=full.device)], dim=1)
    hs = model(full, output_hidden_states=True, use_cache=False).hidden_states
    return [h[0, plen:, :].float().mean(0) for h in hs]


def build_vectors():
    pos_sum = neg_sum = norm_sum = None
    n = 0
    for t in camp.DEV_TASKS:
        base = t.prompt.strip()
        pm = gen_and_capture(base + ANSWER_SUFFIX)
        nm = gen_and_capture(base + THINK_SUFFIX)
        if pos_sum is None:
            pos_sum = [torch.zeros_like(x) for x in pm]
            neg_sum = [torch.zeros_like(x) for x in nm]
            norm_sum = [0.0 for _ in pm]
        for i in range(len(pm)):
            pos_sum[i] += pm[i]
            neg_sum[i] += nm[i]
            norm_sum[i] += float(pm[i].norm())
        n += 1
    vecs, lnorm = {}, {}
    for L in LAYERS:
        idx = L + 1
        diff = (pos_sum[idx] - neg_sum[idx]) / n
        nrm = float(diff.norm())
        vecs[L] = (diff / nrm) if nrm > 0 else diff
        lnorm[L] = norm_sum[idx] / n
        print(f"  layer {L}: ||answer-think||={nrm:.3f}  mean_resid_norm={lnorm[L]:.2f}")
    return vecs, lnorm


print("extracting answer-mode vectors (response-token CAA: answer-mode minus think-mode)...")
VECS, LAYER_NORM = build_vectors()

HANDLES = {}
_state = {"layer": None, "vec": None, "add": 0.0}


def make_hook(layer_idx):
    def hook(module, inputs, output):
        if _state["layer"] != layer_idx or _state["vec"] is None:
            return output
        if isinstance(output, tuple):
            h = output[0] + (_state["add"] * _state["vec"]).to(output[0].dtype)
            return (h,) + tuple(output[1:])
        return output + (_state["add"] * _state["vec"]).to(output.dtype)
    return hook


for L in LAYERS:
    HANDLES[L] = layer_list[L].register_forward_hook(make_hook(L))


@torch.no_grad()
def generate(prompt_text: str) -> str:
    ids = tok(prompt_text, return_tensors="pt").to(model.device)
    out = model.generate(**ids, max_new_tokens=MAXNEW, do_sample=False, pad_token_id=tok.pad_token_id,
                         use_cache=True, stop_strings=["\n###", "\nTask:", "\n### Task"], tokenizer=tok)
    return tok.decode(out[0][ids["input_ids"].shape[1]:], skip_special_tokens=True)


def clean_cut(cont: str) -> str:
    for marker in ("\n###", "\nTask:", "\n\n###"):
        i = cont.find(marker)
        if i != -1:
            cont = cont[:i]
    return camp.as_clean(cont)


def eval_suite(suite: str) -> int:
    return sum(int(t.grader(clean_cut(generate(camp.build_prompt(t, ""))))[0]) for t in camp.SUITES[suite])


stamp = time.strftime("%Y%m%d-%H%M%S")
slug = MODEL_ID.split("/")[-1]
run_dir = os.path.join(ROOT, "rag_runs", f"steering2-{slug}-{stamp}")
os.makedirs(run_dir, exist_ok=True)
rows = []

_state["layer"] = None
for suite in SUITES:
    p = eval_suite(suite)
    rows.append({"layer": "none", "alpha": 0.0, "suite": suite, "passed": p, "tasks": len(camp.SUITES[suite])})
    print(f"baseline           {suite:<8} {p}/{len(camp.SUITES[suite])}")

for L in LAYERS:
    for a in ALPHAS:
        _state["layer"] = L
        _state["vec"] = VECS[L].to(model.device)
        _state["add"] = a * LAYER_NORM[L]
        for suite in SUITES:
            p = eval_suite(suite)
            rows.append({"layer": L, "alpha": a, "suite": suite, "passed": p, "tasks": len(camp.SUITES[suite])})
            print(f"L={L:<3} alpha={a:<4} {suite:<8} {p}/{len(camp.SUITES[suite])}")
        _state["layer"] = None

with open(os.path.join(run_dir, "results.csv"), "w", newline="", encoding="utf-8") as fh:
    w = csv.DictWriter(fh, fieldnames=["layer", "alpha", "suite", "passed", "tasks"])
    w.writeheader(); w.writerows(rows)
json.dump({"model": MODEL_ID, "layers": LAYERS, "alphas": ALPHAS, "contrast": "answer-mode-CAA", "rows": rows},
          open(os.path.join(run_dir, "report.json"), "w"), indent=2)
print(f"\nRun dir: {run_dir}")
