"""Activation steering v3: CLAMP/override the answer<->think axis.

Insight: naive "add a bigger vector" over-steers (inverted-U; alpha>=1 degrades).
To truly OVERPOWER the native signal without wrecking the rest of the computation,
project OUT the model's native component along the answer-vs-think direction and
SET that coordinate to a value we choose:

    h -> h - (h . vhat) vhat + target * vhat

target is swept as p_ans + k * ||answer-think||, where p_ans is the natural
answer-mode projection. k=0 normalizes every token to answer-mode level; k>0
overpowers it. This dominates one axis, leaving the orthogonal subspace intact.

Env: STEER_MODEL, STEER_LAYERS (default 14,16), STEER_TARGETS (k values, default 0,1,2,3),
     STEER_SUITES, STEER_MAXNEW (96), STEER_CAPNEW (40).
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
LAYERS = [int(x) for x in os.environ.get("STEER_LAYERS", "14,16").split(",") if x.strip()]
TARGETS = [float(x) for x in os.environ.get("STEER_TARGETS", "0,1,2,3").split(",") if x.strip()]
SUITES = [s for s in os.environ.get("STEER_SUITES", "dev,holdout").split(",") if s.strip()]
MAXNEW = int(os.environ.get("STEER_MAXNEW", "96"))
CAPNEW = int(os.environ.get("STEER_CAPNEW", "40"))

print(f"model={MODEL_ID} layers={LAYERS} targets(k)={TARGETS} suites={SUITES} (CLAMP answer/think axis)")
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
    ids = tok(prompt_text, return_tensors="pt").to(model.device)
    plen = ids["input_ids"].shape[1]
    out = model.generate(**ids, max_new_tokens=CAPNEW, do_sample=False, pad_token_id=tok.pad_token_id, use_cache=True)
    full = out[0].unsqueeze(0)
    if full.shape[1] <= plen:
        full = torch.cat([full, torch.tensor([[tok.eos_token_id]], device=full.device)], dim=1)
    hs = model(full, output_hidden_states=True, use_cache=False).hidden_states
    return [h[0, plen:, :].float().mean(0) for h in hs]


def build_axis():
    pos_sum = neg_sum = None
    n = 0
    for t in camp.DEV_TASKS:
        base = t.prompt.strip()
        pm = gen_and_capture(base + ANSWER_SUFFIX)
        nm = gen_and_capture(base + THINK_SUFFIX)
        if pos_sum is None:
            pos_sum = [torch.zeros_like(x) for x in pm]
            neg_sum = [torch.zeros_like(x) for x in nm]
        for i in range(len(pm)):
            pos_sum[i] += pm[i]
            neg_sum[i] += nm[i]
        n += 1
    info = {}
    for L in LAYERS:
        idx = L + 1
        a_mean = pos_sum[idx] / n
        t_mean = neg_sum[idx] / n
        diff = a_mean - t_mean
        dn = float(diff.norm())
        vhat = diff / dn if dn > 0 else diff
        p_ans = float((a_mean * vhat).sum())
        p_think = float((t_mean * vhat).sum())
        info[L] = {"vhat": vhat, "p_ans": p_ans, "p_think": p_think, "diffnorm": dn}
        print(f"  layer {L}: ||diff||={dn:.3f}  p_ans={p_ans:.3f}  p_think={p_think:.3f}")
    return info


print("extracting answer/think axis (response CAA)...")
AXIS = build_axis()

HANDLES = {}
_state = {"layer": None, "vhat": None, "target": None}


def make_hook(layer_idx):
    def hook(module, inputs, output):
        if _state["layer"] != layer_idx or _state["vhat"] is None:
            return output
        vhat = _state["vhat"]
        tgt = _state["target"]
        h = output[0] if isinstance(output, tuple) else output
        proj = (h.float() * vhat).sum(-1, keepdim=True)        # current coord along vhat
        h2 = (h.float() - proj * vhat + tgt * vhat).to(h.dtype)  # clamp coord to tgt
        if isinstance(output, tuple):
            return (h2,) + tuple(output[1:])
        return h2
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
run_dir = os.path.join(ROOT, "rag_runs", f"steering3clamp-{slug}-{stamp}")
os.makedirs(run_dir, exist_ok=True)
rows = []

_state["layer"] = None
for suite in SUITES:
    p = eval_suite(suite)
    rows.append({"layer": "none", "k": "-", "target": "-", "suite": suite, "passed": p, "tasks": len(camp.SUITES[suite])})
    print(f"baseline           {suite:<8} {p}/{len(camp.SUITES[suite])}")

for L in LAYERS:
    a = AXIS[L]
    _state["vhat"] = a["vhat"].to(model.device)
    for k in TARGETS:
        target = a["p_ans"] + k * a["diffnorm"]
        _state["layer"] = L
        _state["target"] = target
        for suite in SUITES:
            p = eval_suite(suite)
            rows.append({"layer": L, "k": k, "target": round(target, 3), "suite": suite, "passed": p, "tasks": len(camp.SUITES[suite])})
            print(f"L={L:<3} k={k:<4} (tgt={target:.2f}) {suite:<8} {p}/{len(camp.SUITES[suite])}")
        _state["layer"] = None

with open(os.path.join(run_dir, "results.csv"), "w", newline="", encoding="utf-8") as fh:
    w = csv.DictWriter(fh, fieldnames=["layer", "k", "target", "suite", "passed", "tasks"])
    w.writeheader(); w.writerows(rows)
json.dump({"model": MODEL_ID, "layers": LAYERS, "targets_k": TARGETS, "mode": "clamp-answer-axis", "rows": rows},
          open(os.path.join(run_dir, "report.json"), "w"), indent=2)
print(f"\nRun dir: {run_dir}")
