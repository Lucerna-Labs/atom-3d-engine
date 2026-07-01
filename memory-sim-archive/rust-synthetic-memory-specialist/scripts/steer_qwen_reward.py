"""Reward-memory activation steering for Qwen3.5 base (NON-prompt injection).

Idea (user's): don't put the reward-shaped persona memories in the prompt. Distill
them into a steering direction and inject that into the residual stream, so the
disposition rides in the model's internal state (its cache/activations) while the
eval prompt stays BARE. This is the "via the KV cache, not prompt-based" route.

Contrast (response-token CAA, same machinery as steer_qwen2):
  POS = task WITH the reward-memory prefix   -> mean hidden over generated tokens
  NEG = the BARE task                        -> mean hidden over generated tokens
  direction = mean(POS) - mean(NEG), unit-normalized per layer.
At eval the hook adds `alpha * resid_norm * direction` at layer L, prompt is BARE
(no memory text anywhere). We sweep layers x alphas, report dev+holdout vs the
no-steer baseline.

Env: STEER_MODEL (default Qwen/Qwen3.5-4B-Base),
     REWARD_MEM_PATH (default data/reward_memories_full.txt),
     STEER_LAYERS (default 14,16), STEER_ALPHAS (default 0.3,0.6,0.9),
     STEER_SUITES (default dev,holdout), STEER_MAXNEW (default 64),
     STEER_CAPNEW (default 32), STEER_TIME_BUDGET_S (default 1500 = 25 min).
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
MEM_PATH = os.environ.get("REWARD_MEM_PATH", os.path.join(ROOT, "data", "reward_memories_full.txt"))
LAYERS = [int(x) for x in os.environ.get("STEER_LAYERS", "14,16").split(",") if x.strip()]
ALPHAS = [float(x) for x in os.environ.get("STEER_ALPHAS", "0.3,0.6,0.9").split(",") if x.strip()]
SUITES = [s for s in os.environ.get("STEER_SUITES", "dev,holdout").split(",") if s.strip()]
MAXNEW = int(os.environ.get("STEER_MAXNEW", "64"))
CAPNEW = int(os.environ.get("STEER_CAPNEW", "32"))
TIME_BUDGET_S = float(os.environ.get("STEER_TIME_BUDGET_S", "1500"))
T0 = time.time()


def load_reward_memory(path: str) -> str:
    """Join the bodies of all task= blocks (the reward memories are all task=common)."""
    blocks = []
    with open(path, encoding="utf-8") as fh:
        for block in fh.read().split("---"):
            lines = [ln.rstrip() for ln in block.strip().splitlines() if ln.strip()]
            if not lines or not lines[0].startswith("task="):
                continue
            body = "\n".join(lines[1:]).strip()
            if body:
                blocks.append(body)
    return "\n\n".join(blocks)


REWARD_MEM = load_reward_memory(MEM_PATH)
print(f"model={MODEL_ID} layers={LAYERS} alphas={ALPHAS} suites={SUITES}")
print(f"reward memory: {MEM_PATH} ({len(REWARD_MEM)} chars)")

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


def build_vectors():
    """POS = task with reward-memory prefix; NEG = bare task. diff over response tokens."""
    pos_sum = neg_sum = norm_sum = None
    n = 0
    for t in camp.DEV_TASKS:
        pm = gen_and_capture(camp.build_prompt(t, REWARD_MEM))
        nm = gen_and_capture(camp.build_prompt(t, ""))
        if pos_sum is None:
            pos_sum = [torch.zeros_like(x) for x in pm]
            neg_sum = [torch.zeros_like(x) for x in nm]
            norm_sum = [0.0 for _ in pm]
        for i in range(len(pm)):
            pos_sum[i] += pm[i]
            neg_sum[i] += nm[i]
            norm_sum[i] += float(nm[i].norm())
        n += 1
    vecs, lnorm = {}, {}
    for L in LAYERS:
        idx = L + 1
        diff = (pos_sum[idx] - neg_sum[idx]) / n
        nrm = float(diff.norm())
        vecs[L] = (diff / nrm) if nrm > 0 else diff
        lnorm[L] = norm_sum[idx] / n
        print(f"  layer {L}: ||reward-bare||={nrm:.3f}  mean_resid_norm={lnorm[L]:.2f}")
    return vecs, lnorm


print("extracting reward-memory vectors (response-token CAA: with-memory minus bare)...")
VECS, LAYER_NORM = build_vectors()

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
    layer_list[L].register_forward_hook(make_hook(L))


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
    # eval prompt is BARE (no memory text) -- the disposition comes only from the hook
    return sum(int(t.grader(clean_cut(generate(camp.build_prompt(t, ""))))[0]) for t in camp.SUITES[suite])


stamp = time.strftime("%Y%m%d-%H%M%S")
slug = MODEL_ID.split("/")[-1]
run_dir = os.path.join(ROOT, "rag_runs", f"steer-reward-{slug}-{stamp}")
os.makedirs(run_dir, exist_ok=True)
rows = []

_state["layer"] = None
for suite in SUITES:
    p = eval_suite(suite)
    rows.append({"layer": "none", "alpha": 0.0, "suite": suite, "passed": p, "tasks": len(camp.SUITES[suite])})
    print(f"baseline (no steer)  {suite:<8} {p}/{len(camp.SUITES[suite])}")

stopped = False
for L in LAYERS:
    if stopped:
        break
    for a in ALPHAS:
        if time.time() - T0 > TIME_BUDGET_S:
            print(f"[time guard] {int(time.time()-T0)}s > budget {int(TIME_BUDGET_S)}s -- stopping sweep early")
            stopped = True
            break
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
json.dump({"model": MODEL_ID, "mem_path": MEM_PATH, "layers": LAYERS, "alphas": ALPHAS,
           "contrast": "reward-memory-minus-bare", "rows": rows},
          open(os.path.join(run_dir, "report.json"), "w"), indent=2)
print(f"\nRun dir: {run_dir}")
print("STEER_REWARD_DONE")