"""Activation steering for Qwen3.5 base models (HF + forward hooks).

Idea: instead of spending prompt tokens on a discipline nudge, extract the
"answer concisely / commit" DIRECTION in the residual stream (contrastive
activations: concise-framing minus verbose-framing, diff-of-means per layer),
then add alpha * unit_vector into the residual stream at a chosen layer during
generation. Zero prompt tokens -> no attention competition (the thing that hurt
the 4B models when we injected text).

Eval reuses the campaign's dev + held-out suites and exact-match graders, so the
numbers are directly comparable to the prompt-side results.

Env:
  STEER_MODEL    HF repo or path (default Qwen/Qwen3.5-2B-Base)
  STEER_LAYERS   comma list of decoder layer indices to try (default 8,12,16)
  STEER_ALPHAS   comma list of alpha = fraction of per-layer residual norm (default 0.5,1.0,2.0)
  STEER_SUITES   dev,holdout (default both)
  STEER_MAXNEW   max new tokens (default 200)
"""
from __future__ import annotations
import os, sys, time, json, csv
import torch
import torch.nn as nn

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import run_general_corpus_campaign as camp  # noqa: E402
from transformers import AutoTokenizer  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MODEL_ID = os.environ.get("STEER_MODEL", "Qwen/Qwen3.5-2B-Base")
LAYERS = [int(x) for x in os.environ.get("STEER_LAYERS", "8,12,16").split(",") if x.strip()]
ALPHAS = [float(x) for x in os.environ.get("STEER_ALPHAS", "0.5,1.0,2.0").split(",") if x.strip()]
SUITES = [s for s in os.environ.get("STEER_SUITES", "dev,holdout").split(",") if s.strip()]
MAXNEW = int(os.environ.get("STEER_MAXNEW", "200"))
DEVICE = "cuda"

print(f"model={MODEL_ID} layers={LAYERS} alphas={ALPHAS} suites={SUITES}")
tok = AutoTokenizer.from_pretrained(MODEL_ID)
if tok.pad_token_id is None:
    tok.pad_token = tok.eos_token


def load_model():
    import transformers
    last = None
    for name in ("AutoModelForCausalLM", "AutoModelForImageTextToText", "AutoModel"):
        try:
            m = getattr(transformers, name).from_pretrained(MODEL_ID, dtype=torch.bfloat16, device_map=DEVICE)
            print(f"loaded via {name}")
            return m
        except Exception as e:  # noqa: BLE001
            last = e
    raise last


model = load_model()
model.eval()

# locate decoder layer ModuleList
LAYERS_PATH = None
for nm, mod in model.named_modules():
    if isinstance(mod, nn.ModuleList) and len(mod) >= 8:
        LAYERS_PATH = nm
        break
layer_list = model.get_submodule(LAYERS_PATH)
n_layers = len(layer_list)
print(f"decoder layers at '{LAYERS_PATH}' (n={n_layers})")


# ---------------- steering-vector extraction (contrastive activations) ----------------
POS_SUFFIX = "\n\nGive only the final answer. Do not explain. Commit to the single correct value."
NEG_SUFFIX = "\n\nThink out loud and explain every step in detail before you answer."


@torch.no_grad()
def last_token_hidden(text: str):
    ids = tok(text, return_tensors="pt").to(model.device)
    out = model(**ids, output_hidden_states=True, use_cache=False)
    # hidden_states: tuple len n_layers+1; [L+1] = output of decoder layer L
    return [hs[0, -1, :].float() for hs in out.hidden_states]  # list len n_layers+1


def build_steering_vectors():
    train_tasks = camp.DEV_TASKS  # extract on dev prompts; eval on dev+holdout
    pos_sum = None
    neg_sum = None
    norm_sum = None
    n = 0
    for t in train_tasks:
        base = t.prompt.strip()
        ph = last_token_hidden(base + POS_SUFFIX)
        nh = last_token_hidden(base + NEG_SUFFIX)
        if pos_sum is None:
            pos_sum = [torch.zeros_like(h) for h in ph]
            neg_sum = [torch.zeros_like(h) for h in nh]
            norm_sum = [0.0 for _ in ph]
        for i in range(len(ph)):
            pos_sum[i] += ph[i]
            neg_sum[i] += nh[i]
            norm_sum[i] += float(ph[i].norm())
        n += 1
    vecs = {}        # layer L -> unit direction (float32, hidden)
    layer_norm = {}  # layer L -> mean residual norm at that layer's output
    for L in LAYERS:
        idx = L + 1  # hidden_states index for output of layer L
        diff = (pos_sum[idx] - neg_sum[idx]) / n
        nrm = diff.norm()
        vecs[L] = (diff / nrm) if float(nrm) > 0 else diff
        layer_norm[L] = norm_sum[idx] / n
        print(f"  layer {L}: ||diff||={float(nrm):.3f}  mean_resid_norm={layer_norm[L]:.2f}")
    return vecs, layer_norm


print("extracting steering vectors (contrastive: concise minus verbose)...")
VECS, LAYER_NORM = build_steering_vectors()


# ---------------- steering hook (one per target layer, gated by the active layer) ----------------
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
    out = model.generate(
        **ids, max_new_tokens=MAXNEW, do_sample=False, pad_token_id=tok.pad_token_id,
        use_cache=True, stop_strings=["\n###", "\nTask:", "\n### Task"], tokenizer=tok,
    )
    return tok.decode(out[0][ids["input_ids"].shape[1]:], skip_special_tokens=True)


def clean_cut(cont: str) -> str:
    for marker in ("\n###", "\nTask:", "\n\n###"):
        i = cont.find(marker)
        if i != -1:
            cont = cont[:i]
    return camp.as_clean(cont)


def eval_suite(suite: str) -> int:
    tasks = camp.SUITES[suite]
    passed = 0
    for t in tasks:
        ans = clean_cut(generate(camp.build_prompt(t, "")))
        passed += int(t.grader(ans)[0])
    return passed


# ---------------- sweep ----------------
stamp = time.strftime("%Y%m%d-%H%M%S")
slug = MODEL_ID.split("/")[-1]
run_dir = os.path.join(ROOT, "rag_runs", f"steering-{slug}-{stamp}")
os.makedirs(run_dir, exist_ok=True)
rows = []

# baseline (no steering)
_state["layer"] = None
for suite in SUITES:
    p = eval_suite(suite)
    rows.append({"layer": "none", "alpha": 0.0, "suite": suite, "passed": p, "tasks": len(camp.SUITES[suite])})
    print(f"baseline           {suite:<8} {p}/{len(camp.SUITES[suite])}")

# steered
for L in LAYERS:
    for a in ALPHAS:
        _state["layer"] = L
        _state["vec"] = VECS[L].to(model.device)
        _state["add"] = a * LAYER_NORM[L]  # alpha as fraction of residual norm
        for suite in SUITES:
            p = eval_suite(suite)
            rows.append({"layer": L, "alpha": a, "suite": suite, "passed": p, "tasks": len(camp.SUITES[suite])})
            print(f"L={L:<3} alpha={a:<4} {suite:<8} {p}/{len(camp.SUITES[suite])}")
        _state["layer"] = None

with open(os.path.join(run_dir, "results.csv"), "w", newline="", encoding="utf-8") as fh:
    w = csv.DictWriter(fh, fieldnames=["layer", "alpha", "suite", "passed", "tasks"])
    w.writeheader()
    w.writerows(rows)
json.dump({"model": MODEL_ID, "layers": LAYERS, "alphas": ALPHAS, "rows": rows},
          open(os.path.join(run_dir, "report.json"), "w"), indent=2)
print(f"\nRun dir: {run_dir}")
