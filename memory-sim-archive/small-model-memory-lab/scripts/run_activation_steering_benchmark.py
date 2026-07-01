import argparse
import csv
import json
import re
import time
from pathlib import Path

import torch
from transformers import AutoModelForCausalLM, AutoTokenizer


POSITIVE_EXAMPLES = [
    "I solve arithmetic by writing the setup first, preserving every intermediate value, carrying units, and checking the final answer against the story.",
    "When a problem has rates, I compute the unit rate carefully, keep the denominator visible, then rebuild the combined rate before dividing.",
    "When a problem has clock time, I convert total minutes into hours plus leftover minutes before adding to the start time.",
    "When a problem has ratios, I count total parts, compute the unit part, multiply each side, and check that the parts add to the total.",
    "When a problem has money, I keep cents visible, add fees, subtract credits exactly once, and state the final dollars.",
    "When a problem has probability without replacement, I update the counts after the first event, multiply, simplify, and give the decimal size.",
]

NEGATIVE_EXAMPLES = [
    "I answer arithmetic from the first plausible number, skip intermediate values, and do not check units.",
    "When a problem has rates, I divide by only one denominator and forget time or group size.",
    "When a problem has clock time, I treat minutes like decimal digits and name the clock time too early.",
    "When a problem has ratios, I skip the unit part and guess the two amounts directly.",
    "When a problem has money, I apply credits twice or lose the sign of a fee.",
    "When a problem has probability, I keep the same counts after an event that happened without replacement.",
]


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

    return torch.cat(captured, dim=0).mean(dim=0)


def make_prompt(task):
    return (
        f"Task:\n{task['task']}\n\n"
        "Answer with the setup, arithmetic, units, and final answer:\n"
    )


def generate_new_text(model, tokenizer, prompt, device, max_new_tokens):
    inputs = tokenizer(prompt, return_tensors="pt")
    inputs = {k: v.to(device) for k, v in inputs.items()}
    input_len = inputs["input_ids"].shape[-1]
    with torch.no_grad():
        out = model.generate(
            **inputs,
            max_new_tokens=max_new_tokens,
            do_sample=False,
            pad_token_id=tokenizer.eos_token_id,
            repetition_penalty=1.08,
        )
    return tokenizer.decode(out[0][input_len:], skip_special_tokens=True)


def generate_steered(model, tokenizer, layers, layer_index, vector, strength, prompt, device, max_new_tokens):
    steer = (vector.to(device=device, dtype=model.dtype) * strength).view(1, 1, -1)

    def hook(_module, _inputs, output):
        hidden, original = unpack_layer_output(output)
        return repack_layer_output(hidden + steer, original)

    handle = layers[layer_index].register_forward_hook(hook)
    try:
        return generate_new_text(model, tokenizer, prompt, device, max_new_tokens)
    finally:
        handle.remove()


def measure(task, response):
    hits = []
    for criterion in task.get("criteria", []):
        matched = False
        for pattern in criterion.get("patterns", []):
            if re.search(pattern, response, flags=re.IGNORECASE | re.MULTILINE):
                matched = True
                break
        if matched:
            for anti in criterion.get("anti_patterns", []):
                if re.search(anti, response, flags=re.IGNORECASE | re.MULTILINE):
                    matched = False
                    break
        if matched:
            hits.append(criterion["id"])
    total = len(task.get("criteria", []))
    return hits, total


def parse_int_list(value):
    return [int(x.strip()) for x in value.split(",") if x.strip()]


def parse_float_list(value):
    return [float(x.strip()) for x in value.split(",") if x.strip()]


def main():
    parser = argparse.ArgumentParser(description="Benchmark residual-stream activation steering.")
    parser.add_argument("--model", default="Qwen/Qwen3-1.7B-Base")
    parser.add_argument("--lab-root", default=r"C:\Projects\small-model-memory-lab")
    parser.add_argument("--benchmark-file", default="computation_damage_tasks.json")
    parser.add_argument("--layers", default="7,14,21")
    parser.add_argument("--strengths", default="0.5,1.0,2.0")
    parser.add_argument("--max-new-tokens", type=int, default=180)
    parser.add_argument("--dtype", choices=["auto", "float16", "bfloat16", "float32"], default="bfloat16")
    parser.add_argument("--device-map", default="auto")
    parser.add_argument("--trust-remote-code", action="store_true")
    parser.add_argument("--out-root", default="")
    args = parser.parse_args()

    lab_root = Path(args.lab_root)
    out_root = Path(args.out_root) if args.out_root else lab_root / "runs"
    run_dir = out_root / f"activation-steering-{time.strftime('%Y%m%d-%H%M%S')}"
    run_dir.mkdir(parents=True, exist_ok=True)

    tasks = json.loads((lab_root / "benchmarks" / args.benchmark_file).read_text(encoding="utf-8"))
    if isinstance(tasks, dict):
        tasks = [tasks]

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
    device = next(model.parameters()).device

    requested_layers = parse_int_list(args.layers)
    strengths = parse_float_list(args.strengths)
    requested_layers = [idx for idx in requested_layers if 0 <= idx < len(layers)]

    vectors = {}
    vector_report = []
    for layer_index in requested_layers:
        pos = collect_layer_mean(model, tokenizer, layers, layer_index, POSITIVE_EXAMPLES, device)
        neg = collect_layer_mean(model, tokenizer, layers, layer_index, NEGATIVE_EXAMPLES, device)
        vector = pos - neg
        vectors[layer_index] = vector
        norm = torch.linalg.vector_norm(vector).item()
        torch.save(vector, run_dir / f"careful_math_layer_{layer_index}.pt")
        vector_report.append({"layer": layer_index, "norm": norm})
        print(f"Vector layer {layer_index}: norm={norm:.4f}")

    rows = []
    for task in tasks:
        task_id = task["id"]
        task_dir = run_dir / re.sub(r"[^a-z0-9]+", "-", task_id.lower()).strip("-")
        task_dir.mkdir(parents=True, exist_ok=True)
        prompt = make_prompt(task)
        (task_dir / "prompt.txt").write_text(prompt, encoding="utf-8")

        baseline = generate_new_text(model, tokenizer, prompt, device, args.max_new_tokens)
        (task_dir / "baseline.response.txt").write_text(baseline, encoding="utf-8")
        hits, total = measure(task, baseline)
        rows.append({
            "task_id": task_id,
            "mode": "baseline",
            "layer": "",
            "strength": "",
            "hits": len(hits),
            "total": total,
            "rate": round(len(hits) / total, 4) if total else 0,
            "hit_ids": ",".join(hits),
            "response_preview": re.sub(r"\s+", " ", baseline).strip()[:180],
        })
        print(f"{task_id} baseline {len(hits)}/{total}")

        for layer_index in requested_layers:
            for strength in strengths:
                mode = f"steered_l{layer_index}_s{strength:g}"
                response = generate_steered(
                    model,
                    tokenizer,
                    layers,
                    layer_index,
                    vectors[layer_index],
                    strength,
                    prompt,
                    device,
                    args.max_new_tokens,
                )
                (task_dir / f"{mode}.response.txt").write_text(response, encoding="utf-8")
                hits, total = measure(task, response)
                rows.append({
                    "task_id": task_id,
                    "mode": "steered",
                    "layer": layer_index,
                    "strength": strength,
                    "hits": len(hits),
                    "total": total,
                    "rate": round(len(hits) / total, 4) if total else 0,
                    "hit_ids": ",".join(hits),
                    "response_preview": re.sub(r"\s+", " ", response).strip()[:180],
                })
                print(f"{task_id} layer={layer_index} strength={strength:g} {len(hits)}/{total}")

    with (run_dir / "summary.csv").open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)

    aggregates = []
    base_rows = [r for r in rows if r["mode"] == "baseline"]
    base_hits = sum(int(r["hits"]) for r in base_rows)
    base_total = sum(int(r["total"]) for r in base_rows)
    aggregates.append({"mode": "baseline", "layer": "", "strength": "", "hits": base_hits, "total": base_total, "rate": base_hits / base_total})
    for layer_index in requested_layers:
        for strength in strengths:
            selected = [r for r in rows if str(r["layer"]) == str(layer_index) and str(r["strength"]) == str(strength)]
            hits = sum(int(r["hits"]) for r in selected)
            total = sum(int(r["total"]) for r in selected)
            aggregates.append({"mode": "steered", "layer": layer_index, "strength": strength, "hits": hits, "total": total, "rate": hits / total})

    with (run_dir / "aggregate.json").open("w", encoding="utf-8") as f:
        json.dump({"vectors": vector_report, "aggregates": aggregates}, f, indent=2)

    print("\nAggregates:")
    for row in aggregates:
        print(f"{row['mode']} layer={row['layer']} strength={row['strength']}: {row['hits']}/{row['total']} = {row['rate']:.4f}")
    print(f"\nRun: {run_dir}")


if __name__ == "__main__":
    main()
