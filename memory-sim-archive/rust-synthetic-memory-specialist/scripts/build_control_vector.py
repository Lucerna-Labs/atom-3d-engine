"""Extract the answer-mode direction from HF Qwen3.5-4B-Base and export it as a
llama.cpp control-vector GGUF, so it can be applied to the actual Ollama GGUF.

Direction = answer-mode minus think-mode, response-token CAA (same as steer_qwen2),
unit-normalized per layer. We export directions for a band of layers (default just
the layer that won in HF, L16) so llama-cli can apply `strength * direction` to the
residual stream -- the deployment-real version of our additive steering.

Env: CV_MODEL (HF repo, default Qwen/Qwen3.5-4B-Base)
     CV_LAYERS (decoder layers to populate, default "16")
     CV_MODEL_HINT (architecture hint for the control vector; default "qwen3")
     CV_OUT (output gguf path, default data/answer_mode_cv.gguf)
     CV_CAPNEW (capture gen length, default 40)
"""
from __future__ import annotations
import os, sys
import numpy as np
import torch
import torch.nn as nn

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import run_general_corpus_campaign as camp  # noqa: E402
from transformers import AutoTokenizer  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MODEL_ID = os.environ.get("CV_MODEL", "Qwen/Qwen3.5-4B-Base")
CV_LAYERS = [int(x) for x in os.environ.get("CV_LAYERS", "16").split(",") if x.strip()]
MODEL_HINT = os.environ.get("CV_MODEL_HINT", "qwen3")
OUT = os.environ.get("CV_OUT", os.path.join(ROOT, "data", "answer_mode_cv.gguf"))
CAPNEW = int(os.environ.get("CV_CAPNEW", "40"))

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
n_layers = len(model.get_submodule(LAYERS_PATH))
hidden = model.config.text_config.hidden_size if hasattr(model.config, "text_config") else model.config.hidden_size
print(f"n_layers={n_layers} hidden={hidden}")

ANSWER_SUFFIX = "\n\nAnswer with only the final value. Do not think out loud or explain."
THINK_SUFFIX = "\n\nThink through it step by step out loud, explaining your reasoning in detail, before you answer."


@torch.no_grad()
def cap(prompt_text):
    ids = tok(prompt_text, return_tensors="pt").to(model.device)
    plen = ids["input_ids"].shape[1]
    out = model.generate(**ids, max_new_tokens=CAPNEW, do_sample=False, pad_token_id=tok.pad_token_id, use_cache=True)
    full = out[0].unsqueeze(0)
    if full.shape[1] <= plen:
        full = torch.cat([full, torch.tensor([[tok.eos_token_id]], device=full.device)], dim=1)
    hs = model(full, output_hidden_states=True, use_cache=False).hidden_states
    return [h[0, plen:, :].float().mean(0) for h in hs]


pos_sum = neg_sum = None
n = 0
for t in camp.DEV_TASKS:
    base = t.prompt.strip()
    pm = cap(base + ANSWER_SUFFIX)
    nm = cap(base + THINK_SUFFIX)
    if pos_sum is None:
        pos_sum = [torch.zeros_like(x) for x in pm]
        neg_sum = [torch.zeros_like(x) for x in nm]
    for i in range(len(pm)):
        pos_sum[i] += pm[i]
        neg_sum[i] += nm[i]
    n += 1

directions = {}
for L in CV_LAYERS:
    diff = (pos_sum[L + 1] - neg_sum[L + 1]) / n
    nrm = float(diff.norm())
    directions[L] = (diff / nrm).cpu().numpy().astype(np.float32)
    print(f"  layer {L}: ||diff||={nrm:.3f} -> unit direction[{directions[L].shape}]")

# ---- write llama.cpp control-vector GGUF ----
try:
    import gguf
except ImportError:
    print("gguf not installed; run: pip install gguf")
    raise

w = gguf.GGUFWriter(OUT, arch="controlvector")
w.add_string("controlvector.model_hint", MODEL_HINT)
w.add_uint32("controlvector.layer_count", n_layers)
for L, vec in directions.items():
    # llama.cpp expects 1-based layer index: direction.<layer>
    w.add_tensor(f"direction.{L + 1}", vec)
w.write_header_to_file()
w.write_kv_data_to_file()
w.write_tensors_to_file()
w.close()
print(f"wrote control vector -> {OUT}  (model_hint={MODEL_HINT}, layers={[l+1 for l in CV_LAYERS]})")
