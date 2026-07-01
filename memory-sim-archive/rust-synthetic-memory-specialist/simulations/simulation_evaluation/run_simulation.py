#!/usr/bin/env python
"""
===========================================================
SIMULATION EVALUATION - COMPLETE ORCHESTRATOR
Small Model Memory and Structural Primitive Lab
===========================================================
Combines Tier 0-4 into one runnable script for Kaggle or local.

Usage:
  python run_simulation.py              # Run all tiers
  python run_simulation.py --tier 0   # Run only Tier 0 (CPU, no GPU needed)
  python run_simulation.py --aggregate   # Aggregate existing results

Model matrix: Qwen3.5-2B x Qwen3.5-4B x Q4_K_M x BF16 x WITH/WITHOUT corpus
Total runs: 8 per tier
"""

from __future__ import annotations

import json, os, sys, gc, math, re
from pathlib import Path
from collections import Counter

# paths
KAGGLE_INPUT = Path("/kaggle/input/small-model-memory-lab")
LOCAL_ROOT = Path(__file__).parent.parent  # D:\minimax-lab
MEMORY_CORPUS_DIR = LOCAL_ROOT / "memory_corpus"
WORKING_DIR = Path("/kaggle/working/simulation_results")
LOCAL_RUNS = LOCAL_ROOT / "runs" / "simulation_results"
OUT_DIR = WORKING_DIR if Path("/kaggle/working").exists() else LOCAL_RUNS
OUT_DIR.mkdir(parents=True, exist_ok=True)

MODEL_CONFIGS = {
    "Qwen3.5-2B": "Qwen/Qwen2.5-2B",
    "Qwen3.5-4B": "Qwen/Qwen2.5-4B",
}
QUANT_CONFIGS = ["BF16", "Q4_K_M"]

# ══ TIER 0: CORPUS ANALYZER ══

def tokenize(text):
    return re.findall(r"[A-Za-z]+", text.lower())

def token_entropy(tokens):
    if not tokens:
        return 0.0
    freq = Counter(tokens)
    total = len(tokens)
    probs = [count / total for count in freq.values() if count > 0]
    return -sum(p * math.log2(p) for p in probs if p > 0)

def ngram_diversity(tokens, n=3):
    if len(tokens) < n:
        return 0.0
    ngrams = ["_".join(tokens[i:i+n]) for i in range(len(tokens)-n+1)]
    if not ngrams:
        return 0.0
    return len(set(ngrams)) / len(ngrams)

def memory_loop_score(text):
    tl = text.lower()
    situation = any(w in tl for w in ["i remember", "i saw", "i had", "i felt", "the situation", "when", "during"])
    recognition = any(w in tl for w in ["noticed", "recognized", "saw that", "identified", "understood", "realized"])
    action = any(w in tl for w in ["i did", "i chose", "i decided", "i kept", "i preserved"])
    consequence = any(w in tl for w in ["the outcome", "the result", "the reward", "the lesson", "saved", "paid for"])
    if situation and recognition and action and consequence:
        return 1.0
    elif sum([situation, recognition, action, consequence]) >= 3:
        return 0.75
    elif sum([situation, recognition, action, consequence]) >= 2:
        return 0.5
    return 0.25

def run_tier0():
    print("\n" + "=" * 60)
    print("TIER 0: CORPUS ANALYZER (CPU)")
    print("=" * 60)

    corpus_files = [
        ("cognitive_core_v0_1", MEMORY_CORPUS_DIR / "cognitive_core_v0_1" / "cognitive_core_v0_1.json", "json"),
        ("cognitive_rewards_v0_1", MEMORY_CORPUS_DIR / "cognitive_rewards_v0_1" / "cognitive_rewards_v0_1.json", "json"),
        ("structural_primitives_v0_1", MEMORY_CORPUS_DIR / "structural_primitives_v0_1" / "structural_primitives_lora_v0_1.json", "json"),
        ("rust_cyber_defender", MEMORY_CORPUS_DIR / "rust_cyber_defender" / "rust_cyber_defender_generated.tsv", "tsv"),
    ]

    results = {}
    for name, path, ftype in corpus_files:
        print(f"\n  Analyzing: {name}")
        texts = []
        if ftype == "json" and path.exists():
            with open(path, encoding="utf-8") as f:
                data = json.load(f)
            for item in data:
                body = item.get("body") or item.get("text") or item.get("task")
                if body:
                    texts.append(str(body))
        elif ftype == "tsv" and path.exists():
            with open(path, encoding="utf-8") as f:
                f.readline()
                for line in f:
                    parts = line.strip().split("\t")
                    if len(parts) >= 5:
                        texts.append(parts[4])

        print(f"    Loaded {len(texts)} texts")
        if not texts:
            results[name] = {"error": "no texts loaded"}
            continue

        entropies = [token_entropy(tokenize(t)) for t in texts]
        diversities = [ngram_diversity(tokenize(t), 3) for t in texts]
        loop_scores = [memory_loop_score(t) for t in texts]
        n_pass = 0
        if sum(entropies)/len(entropies) >= 4.0: n_pass += 1
        if sum(diversities)/len(diversities) >= 0.6: n_pass += 1
        full_loop_pct = sum(1 for s in loop_scores if s >= 1.0) / len(loop_scores) * 100
        if full_loop_pct >= 60: n_pass += 1
        total_tokens = sum(len(tokenize(t)) for t in texts)

        results[name] = {
            "num_texts": len(texts),
            "total_tokens": total_tokens,
            "token_entropy_mean": round(sum(entropies)/len(entropies), 4),
            "ngram_diversity_mean": round(sum(diversities)/len(diversities), 4),
            "memory_loop_full_pct": round(full_loop_pct, 1),
            "pass_count": n_pass,
            "overall_score": round(n_pass / 3, 2),
        }
        print(f"    Entropy: {results[name]['token_entropy_mean']:.4f} | Loop: {full_loop_pct:.1f}% | Score: {results[name]['overall_score']:.2f}")

    report = {"tier": "0", "corpus_results": results}
    out_path = OUT_DIR / "corpus_quality_report.json"
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2)
    print(f"\n  [SAVED] {out_path}")
    return report


# ══ TIER 1: DIRECT MODEL MEASUREMENT ══

def run_tier1():
    print("\n" + "=" * 60)
    print("TIER 1: DIRECT MODEL MEASUREMENT")
    print("=" * 60)

    if not Path("/kaggle/working").exists():
        print("  [SKIP] No GPU environment")
        return {"tier": "1", "status": "skipped_no_gpu"}

    try:
        import torch
        from transformers import AutoTokenizer, AutoModelForCausalLM, BitsAndBytesConfig
    except ImportError as e:
        print(f"  [SKIP] Missing: {e}")
        return {"tier": "1", "status": "skipped"}

    results = {}
    for model_name, model_id in MODEL_CONFIGS.items():
        results[model_name] = {}
        for quant in QUANT_CONFIGS:
            print(f"\n  [{model_name} x {quant}]")
            try:
                if quant == "Q4_K_M":
                    try:
                        bnb = BitsAndBytesConfig(load_in_4bit=True, bnb_4bit_compute_dtype=torch.bfloat16, bnb_4bit_quant_type="nf4")
                        model = AutoModelForCausalLM.from_pretrained(model_id, quantization_config=bnb, device_map="auto", trust_remote_code=True)
                        mode = "Q4_K_M"
                    except Exception:
                        model = AutoModelForCausalLM.from_pretrained(model_id, torch_dtype=torch.float16, device_map="auto", trust_remote_code=True)
                        mode = "FP16"
                else:
                    model = AutoModelForCausalLM.from_pretrained(model_id, torch_dtype=torch.bfloat16, device_map="auto", trust_remote_code=True)
                    mode = "BF16"

                tokenizer = AutoTokenizer.from_pretrained(model_id, trust_remote_code=True)
                sample = "I remember the moment I recognized the pattern, chose to preserve the boundary, and the outcome confirmed the decision."
                inputs = tokenizer(sample, return_tensors="pt")
                input_ids = inputs["input_ids"].to(model.device)
                with torch.no_grad():
                    outputs = model(input_ids, labels=input_ids)
                ppl = round(math.exp(outputs.loss.item()), 4)
                results[model_name][quant] = {"status": "success", "mode": mode, "sample_perplexity": ppl}
                print(f"    Loaded: {mode}, perplexity: {ppl}")
            except Exception as e:
                print(f"    [ERROR] {e}")
                results[model_name][quant] = {"status": "error", "error": str(e)}
            finally:
                if "model" in dir():
                    del model
                torch.cuda.empty_cache()
                gc.collect()

    report = {"tier": "1", "results": results}
    out_path = OUT_DIR / "tier1_model_measurement.json"
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2)
    print(f"\n  [SAVED] {out_path}")
    return report


# ══ TIER 2: BEHAVIORAL BENCHMARKS ══

def run_tier2():
    print("\n" + "=" * 60)
    print("TIER 2: BEHAVIORAL BENCHMARKS")
    print("=" * 60)

    if not Path("/kaggle/working").exists():
        print("  [SKIP] No GPU environment")
        return {"tier": "2", "status": "skipped_no_gpu"}

    try:
        import torch
        from transformers import AutoTokenizer, AutoModelForCausalLM, BitsAndBytesConfig
    except ImportError as e:
        print(f"  [SKIP] Missing: {e}")
        return {"tier": "2", "status": "skipped"}

    cap_path = LOCAL_ROOT / "benchmarks" / "capability_tasks.json"
    comp_path = LOCAL_ROOT / "benchmarks" / "computation_damage_tasks.json"
    probes = []
    if cap_path.exists():
        with open(cap_path, encoding="utf-8") as f:
            probes.extend(json.load(f))
    if comp_path.exists():
        with open(comp_path, encoding="utf-8") as f:
            probes.extend(json.load(f))
    print(f"  Loaded {len(probes)} probe tasks")

    results = {}
    for model_name, model_id in MODEL_CONFIGS.items():
        results[model_name] = {}
        for quant in QUANT_CONFIGS:
            print(f"\n  [{model_name} x {quant}]")
            try:
                if quant == "Q4_K_M":
                    try:
                        bnb = BitsAndBytesConfig(load_in_4bit=True, bnb_4bit_compute_dtype=torch.bfloat16, bnb_4bit_quant_type="nf4")
                        model = AutoModelForCausalLM.from_pretrained(model_id, quantization_config=bnb, device_map="auto", trust_remote_code=True)
                    except Exception:
                        model = AutoModelForCausalLM.from_pretrained(model_id, torch_dtype=torch.float16, device_map="auto", trust_remote_code=True)
                else:
                    model = AutoModelForCausalLM.from_pretrained(model_id, torch_dtype=torch.bfloat16, device_map="auto", trust_remote_code=True)
                tokenizer = AutoTokenizer.from_pretrained(model_id, trust_remote_code=True)

                scores = []
                for item in probes[:8]:
                    task_text = item.get("task", "")[:400]
                    criteria = item.get("criteria", [])
                    prompt = f"Task: {task_text}\nAnswer:"
                    inputs = tokenizer(prompt, return_tensors="pt", truncation=True, max_length=512)
                    ids = inputs["input_ids"].to(model.device)
                    with torch.no_grad():
                        outputs = model.generate(ids, max_new_tokens=80, do_sample=False, temperature=0.0)
                    response = tokenizer.decode(outputs[0][len(ids[0]):], skip_special_tokens=True)
                    matched = 0
                    for crit in criteria:
                        for pat in crit.get("patterns", []):
                            if re.search(pat, response, re.IGNORECASE):
                                matched += 1
                                break
                    scores.append(matched / max(len(criteria), 1))

                avg = round(sum(scores)/len(scores), 4) if scores else 0
                results[model_name][quant] = {"status": "success", "avg_score": avg, "n_probes": len(scores)}
                print(f"    Avg score: {avg:.4f} on {len(scores)} probes")
            except Exception as e:
                print(f"    [ERROR] {e}")
                results[model_name][quant] = {"status": "error", "error": str(e)}
            finally:
                if "model" in dir():
                    del model
                torch.cuda.empty_cache()
                gc.collect()

    report = {"tier": "2", "results": results}
    out_path = OUT_DIR / "tier2_benchmark_results.json"
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2)
    print(f"\n  [SAVED] {out_path}")
    return report


# ══ TIER 3: DOMAIN PROBES ══

def run_tier3():
    print("\n" + "=" * 60)
    print("TIER 3: DOMAIN-SPECIFIC PROBES")
    print("=" * 60)
    tier3_path = Path(__file__).parent / "sim_evaluation_tiers_3_4.py"
    if not tier3_path.exists():
        print("  [SKIP] Tier 3 module not found")
        return {"tier": "3", "status": "skipped"}
    try:
        import importlib.util
        spec = importlib.util.spec_from_file_location("tier3", tier3_path)
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)
        if hasattr(mod, "run_full_tier3"):
            mod.run_full_tier3()
        print("  Tier 3 done")
        return {"tier": "3", "status": "done"}
    except Exception as e:
        print(f"  [WARN] {e}")
        return {"tier": "3", "status": "skipped"}


# ══ TIER 4: DEEP SIMULATIONS ══

def run_tier4():
    print("\n" + "=" * 60)
    print("TIER 4: DEEP SIMULATION SCENARIOS")
    print("=" * 60)
    tier4_path = Path(__file__).parent / "sim_evaluation_tier4_deep_simulations.py"
    if not tier4_path.exists():
        print("  [SKIP] Tier 4 module not found")
        return {"tier": "4", "status": "skipped"}
    try:
        import importlib.util
        spec = importlib.util.spec_from_file_location("tier4", tier4_path)
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)
        if hasattr(mod, "run_full_tier4"):
            mod.run_full_tier4()
        print("  Tier 4 done")
        return {"tier": "4", "status": "done"}
    except Exception as e:
        print(f"  [WARN] {e}")
        return {"tier": "4", "status": "skipped"}


# ══ AGGREGATOR ══

def run_aggregator():
    print("\n" + "=" * 60)
    print("RESULTS AGGREGATOR")
    print("=" * 60)
    agg_path = Path(__file__).parent / "sim_evaluation_aggregator.py"
    if agg_path.exists():
        try:
            import importlib.util
            spec = importlib.util.spec_from_file_location("agg", agg_path)
            mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(mod)
            if hasattr(mod, "main"):
                mod.main()
            print("  Aggregator done")
            return {"status": "done"}
        except Exception as e:
            print(f"  [WARN] {e}")

    # Fallback
    report = {"tier": "aggregated", "files_found": []}
    for fname in ["corpus_quality_report.json", "tier1_model_measurement.json",
                  "tier2_benchmark_results.json", "tier3_domain_probes.json",
                  "tier4_deep_simulations.json"]:
        if (OUT_DIR / fname).exists():
            report["files_found"].append(fname)
    out_path = OUT_DIR / "simulation_report.json"
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2)
    print(f"  [SAVED] {out_path}")
    return report


# ══ MAIN ══

def main():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--tier", choices=["0","1","2","3","4"])
    parser.add_argument("--aggregate", action="store_true")
    args = parser.parse_args()

    print("=" * 70)
    print("SMALL MODEL MEMORY LAB - SIMULATION EVALUATION")
    print(f"Output: {OUT_DIR}")
    print("=" * 70)

    if args.aggregate:
        run_aggregator()
        return
    if args.tier == "0":
        run_tier0()
        return
    elif args.tier == "1":
        run_tier1()
        return
    elif args.tier == "2":
        run_tier2()
        return
    elif args.tier == "3":
        run_tier3()
        return
    elif args.tier == "4":
        run_tier4()
        return

    run_tier0()
    run_tier1()
    run_tier2()
    run_tier3()
    run_tier4()
    run_aggregator()

    print("\n" + "=" * 70)
    print("ALL DONE")
    print("=" * 70)
    for f in sorted(OUT_DIR.glob("*.json")):
        print(f"  {f.name}")
    for f in sorted(OUT_DIR.glob("*.pdf")):
        print(f"  {f.name}")


if __name__ == "__main__":
    raise SystemExit(main())