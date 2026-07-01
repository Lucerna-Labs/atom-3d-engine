import argparse
import json
from pathlib import Path

import torch
from transformers import AutoModelForCausalLM, AutoTokenizer


DEFAULT_POSITIVE = [
    "I solve the problem by preserving each intermediate value, carrying units, and checking the final answer against the task.",
    "For arithmetic, I compute the setup explicitly, keep signs and units visible, and verify the result before answering.",
    "A careful calculation names the known quantities, writes each operation, and states the final value with units.",
]

DEFAULT_NEGATIVE = [
    "I answer quickly from a vague impression and skip intermediate values.",
    "I blend quantities together, drop units, and trust the first number that sounds plausible.",
    "A rushed answer guesses the result without checking the arithmetic path.",
]


def load_json_or_text(path):
    data = Path(path).read_text(encoding="utf-8")
    if path.endswith(".json"):
        parsed = json.loads(data)
        if isinstance(parsed, list):
            return [str(x) for x in parsed]
        raise ValueError("JSON example file must be a list of strings.")
    return [line.strip() for line in data.splitlines() if line.strip()]


def pick_dtype(name):
    if name == "float16":
        return torch.float16
    if name == "bfloat16":
        return torch.bfloat16
    if name == "float32":
        return torch.float32
    return "auto"


def get_layers(model):
    candidates = [
        ("model.layers", lambda m: m.model.layers),
        ("model.decoder.layers", lambda m: m.model.decoder.layers),
        ("transformer.h", lambda m: m.transformer.h),
        ("gpt_neox.layers", lambda m: m.gpt_neox.layers),
    ]
    for name, getter in candidates:
        try:
            layers = getter(model)
            if layers is not None and len(layers) > 0:
                return name, layers
        except AttributeError:
            pass
    raise RuntimeError("Could not find decoder layers for this model.")


def unpack_layer_output(output):
    if isinstance(output, tuple):
        return output[0], output
    return output, None


def repack_layer_output(hidden, original):
    if original is None:
        return hidden
    return (hidden,) + tuple(original[1:])


def collect_layer_mean(model, tokenizer, layers, layer_index, texts, device):
    captured = []

    def hook(_module, _inputs, output):
        hidden, _original = unpack_layer_output(output)
        captured.append(hidden.detach().float().mean(dim=1).cpu())
        return output

    handle = layers[layer_index].register_forward_hook(hook)
    try:
        with torch.no_grad():
            for text in texts:
                inputs = tokenizer(text, return_tensors="pt", truncation=True)
                inputs = {k: v.to(device) for k, v in inputs.items()}
                model(**inputs)
    finally:
        handle.remove()

    if not captured:
        raise RuntimeError("No activations captured.")
    return torch.cat(captured, dim=0).mean(dim=0)


def generate(model, tokenizer, prompt, device, max_new_tokens):
    inputs = tokenizer(prompt, return_tensors="pt")
    inputs = {k: v.to(device) for k, v in inputs.items()}
    with torch.no_grad():
        out = model.generate(
            **inputs,
            max_new_tokens=max_new_tokens,
            do_sample=False,
            pad_token_id=tokenizer.eos_token_id,
        )
    return tokenizer.decode(out[0], skip_special_tokens=True)


def generate_with_steering(model, tokenizer, layers, layer_index, vector, strength, prompt, device, max_new_tokens):
    steer = (vector.to(device=device, dtype=model.dtype) * strength).view(1, 1, -1)

    def hook(_module, _inputs, output):
        hidden, original = unpack_layer_output(output)
        return repack_layer_output(hidden + steer, original)

    handle = layers[layer_index].register_forward_hook(hook)
    try:
        return generate(model, tokenizer, prompt, device, max_new_tokens)
    finally:
        handle.remove()


def main():
    parser = argparse.ArgumentParser(description="Compute and apply a residual-stream steering vector.")
    parser.add_argument("--model", required=True, help="HF model id or local HuggingFace model path.")
    parser.add_argument("--layer", type=int, default=-1, help="Layer index. Default: middle layer.")
    parser.add_argument("--strength", type=float, default=1.0, help="Steering multiplier.")
    parser.add_argument("--positive-file", default="", help="JSON list or newline text file of positive examples.")
    parser.add_argument("--negative-file", default="", help="JSON list or newline text file of negative examples.")
    parser.add_argument("--prompt", default="Solve: Four printers make 480 pages in 6 minutes. How long for 3 printers to make 720 pages?")
    parser.add_argument("--max-new-tokens", type=int, default=160)
    parser.add_argument("--dtype", choices=["auto", "float16", "bfloat16", "float32"], default="auto")
    parser.add_argument("--device-map", default="auto")
    parser.add_argument("--trust-remote-code", action="store_true")
    parser.add_argument("--out", default=r"C:\Projects\small-model-memory-lab\runs\activation-steering-probe")
    args = parser.parse_args()

    positives = load_json_or_text(args.positive_file) if args.positive_file else DEFAULT_POSITIVE
    negatives = load_json_or_text(args.negative_file) if args.negative_file else DEFAULT_NEGATIVE

    tokenizer = AutoTokenizer.from_pretrained(args.model, trust_remote_code=args.trust_remote_code)
    if tokenizer.pad_token_id is None and tokenizer.eos_token_id is not None:
        tokenizer.pad_token = tokenizer.eos_token

    model = AutoModelForCausalLM.from_pretrained(
        args.model,
        torch_dtype=pick_dtype(args.dtype),
        device_map=args.device_map,
        trust_remote_code=args.trust_remote_code,
    )
    model.eval()

    layer_name, layers = get_layers(model)
    layer_index = args.layer if args.layer >= 0 else len(layers) // 2
    device = next(model.parameters()).device

    pos = collect_layer_mean(model, tokenizer, layers, layer_index, positives, device)
    neg = collect_layer_mean(model, tokenizer, layers, layer_index, negatives, device)
    vector = pos - neg
    vector_norm = torch.linalg.vector_norm(vector).item()

    baseline = generate(model, tokenizer, args.prompt, device, args.max_new_tokens)
    steered = generate_with_steering(
        model,
        tokenizer,
        layers,
        layer_index,
        vector,
        args.strength,
        args.prompt,
        device,
        args.max_new_tokens,
    )

    out_dir = Path(args.out)
    out_dir.mkdir(parents=True, exist_ok=True)
    report = {
        "model": args.model,
        "layer_container": layer_name,
        "layer_index": layer_index,
        "num_layers": len(layers),
        "strength": args.strength,
        "vector_norm": vector_norm,
        "prompt": args.prompt,
        "positive_examples": positives,
        "negative_examples": negatives,
        "baseline": baseline,
        "steered": steered,
    }
    (out_dir / "report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    torch.save(vector, out_dir / "steering_vector.pt")

    print(f"Model: {args.model}")
    print(f"Layer: {layer_name}[{layer_index}] / {len(layers)}")
    print(f"Vector norm: {vector_norm:.4f}")
    print("\n=== Baseline ===")
    print(baseline)
    print("\n=== Steered ===")
    print(steered)
    print(f"\nSaved: {out_dir}")


if __name__ == "__main__":
    main()
