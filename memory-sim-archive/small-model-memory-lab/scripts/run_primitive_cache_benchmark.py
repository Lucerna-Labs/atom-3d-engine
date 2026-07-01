import argparse
import csv
import json
import re
import time
from pathlib import Path
from urllib import request


PRIMITIVE_PREFIX = """### Cached arithmetic memory, closed block. Do not print this block.
I remember that arithmetic answers become reliable when intermediate values stay visible and task quantities do not change.
For printer rates, I divide pages by printers and by minutes to get pages per printer per minute, then multiply by the new printer count for the combined rate.
For clock time, I convert total minutes into hours plus leftover minutes before adding to a clock, or I convert the start time to minutes since midnight.
For ratio mixtures, I add total parts, divide total amount by total parts, then multiply each side and check that both amounts add back to the total.
For ticket tables, I solve each team separately, total them, then state which team has more and the exact ticket difference.
For subscription and money totals, I multiply monthly charge by months, add positive fees, subtract a credit exactly once, and keep cents visible.
For probability without replacement, I multiply the first probability by the changed second probability, simplify, then give a decimal or percent.
For tiered pricing, I calculate the first tier, calculate only the extra units in the second tier, then add the fixed fee.
### End cached arithmetic memory.
"""


ROUTED_PRIMITIVES = {
    "compute_printer_rate": "Printer rate memory: pages divided by printers divided by minutes gives pages per printer per minute. Combined rate is new printer count times per-printer rate.",
    "compute_discount_tax": "Discount and tax memory: compute discounted price first. Sales tax applies to the discounted price, then final price is discounted price plus tax.",
    "compute_weighted_average": "Weighted average memory: old average times old count recovers the old total. Add new scores, then divide by the full new count.",
    "compute_inventory_available": "Inventory ledger memory: start, subtract sold, add received, subtract reserved. Keep each event sign visible.",
    "compute_schedule_finish": "Clock memory: add durations, convert total minutes to hours plus leftover minutes, then add to start time. 160 minutes is 2 hours 40 minutes.",
    "compute_ratio_mixture": "Ratio memory: add total parts, divide total liters by total parts to get liters per part, multiply each ratio side, then check the total.",
    "compute_average_speed": "Average speed memory: compute time for each trip leg, add total distance and total time, then divide total distance by total time.",
    "compute_linear_equation": "Equation memory: expand, combine like terms, move constants, divide, then state x.",
    "compute_probability_without_replacement": "Without replacement memory: first probability times changed second probability. Simplify and give a decimal or percent.",
    "compute_ticket_table": "Ticket table memory: solve Team A, solve Team B, total them, then state which team has more and by how many tickets.",
    "compute_subscription_total": "Subscription memory: monthly charge times months, plus setup fee, minus credit exactly once.",
    "compute_tiered_pricing": "Tiered pricing memory: first tier amount plus extra units above the cutoff at second rate plus fixed fee.",
}


def post_json(url, payload, timeout=240):
    body = json.dumps(payload).encode("utf-8")
    req = request.Request(
        url,
        data=body,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with request.urlopen(req, timeout=timeout) as resp:
        return json.loads(resp.read().decode("utf-8"))


def post_optional_json(url, payload=None, timeout=60):
    data = None if payload is None else json.dumps(payload).encode("utf-8")
    req = request.Request(
        url,
        data=data,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with request.urlopen(req, timeout=timeout) as resp:
        raw = resp.read().decode("utf-8")
        return json.loads(raw) if raw else {}


def safe_name(text):
    return re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")


def build_prompt(task, prefix_mode):
    if prefix_mode == "fixed":
        prefix = PRIMITIVE_PREFIX + "\n"
    elif prefix_mode == "routed":
        primitive = ROUTED_PRIMITIVES.get(task["id"], "Arithmetic memory: keep intermediate values visible and preserve task facts.")
        prefix = f"### Cached task memory, closed block. Do not print this block.\n{primitive}\n### End cached task memory.\n\n"
    else:
        prefix = ""
    return (
        f"{prefix}"
        f"Task:\n{task['task']}\n\n"
        "Answer with the needed setup, arithmetic, units, and final answer:\n"
    )


def completion(server_url, prompt, slot_id, n_predict, cache_prompt=True):
    return post_json(
        f"{server_url.rstrip('/')}/completion",
        {
            "prompt": prompt,
            "n_predict": n_predict,
            "seed": 17,
            "temperature": 0,
            "repeat_penalty": 1.12,
            "repeat_last_n": 128,
            "cache_prompt": cache_prompt,
            "slot_id": slot_id,
            "stop": ["<|endoftext|>", "\nTask:", "\nQuestion:", "\n### Cached"],
        },
    )


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


def write_text(path, text):
    path.write_text(text, encoding="utf-8")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--lab-root", default=r"C:\Projects\small-model-memory-lab")
    parser.add_argument("--benchmark-file", default="computation_damage_tasks.json")
    parser.add_argument("--server-url", default="http://127.0.0.1:18081")
    parser.add_argument("--n-predict", type=int, default=220)
    parser.add_argument("--baseline-slot-id", type=int, default=0)
    parser.add_argument("--primitive-slot-id", type=int, default=1)
    parser.add_argument("--routed-slot-id", type=int, default=0)
    args = parser.parse_args()

    lab_root = Path(args.lab_root)
    benchmark_path = lab_root / "benchmarks" / args.benchmark_file
    tasks = json.loads(benchmark_path.read_text(encoding="utf-8"))
    if isinstance(tasks, dict):
        tasks = [tasks]

    stamp = time.strftime("%Y%m%d-%H%M%S")
    run_dir = lab_root / "runs" / f"primitive-cache-{stamp}"
    run_dir.mkdir(parents=True, exist_ok=True)
    write_text(run_dir / "primitive_prefix.txt", PRIMITIVE_PREFIX)

    for slot_id in (args.baseline_slot_id, args.primitive_slot_id, args.routed_slot_id):
        try:
            erased = post_optional_json(f"{args.server_url.rstrip('/')}/slots/{slot_id}?action=erase")
            write_text(run_dir / f"slot_{slot_id}_erase.json", json.dumps(erased, indent=2))
        except Exception as exc:
            write_text(run_dir / f"slot_{slot_id}_erase_error.txt", str(exc))

    warm = completion(
        args.server_url,
        PRIMITIVE_PREFIX + "\n",
        args.primitive_slot_id,
        1,
    )
    write_text(run_dir / "prefill.response.json", json.dumps(warm, indent=2))

    rows = []
    for task in tasks:
        task_dir = run_dir / safe_name(task["id"])
        task_dir.mkdir(parents=True, exist_ok=True)
        for mode, prefix_mode, slot_id in (
            ("baseline", "none", args.baseline_slot_id),
            ("primitive_cache", "fixed", args.primitive_slot_id),
            ("routed_primitive_cache", "routed", args.routed_slot_id),
        ):
            prompt = build_prompt(task, prefix_mode)
            result = completion(args.server_url, prompt, slot_id, args.n_predict, cache_prompt=(mode != "baseline"))
            response = result.get("content", "")
            hits, total = measure(task, response)
            timings = result.get("timings", {})
            row = {
                "task_id": task["id"],
                "family": task.get("family", ""),
                "mode": mode,
                "hits": len(hits),
                "total": total,
                "rate": round(len(hits) / total, 4) if total else 0,
                "hit_ids": ",".join(hits),
                "tokens_evaluated": result.get("tokens_evaluated", ""),
                "tokens_cached": result.get("tokens_cached", ""),
                "cache_n": timings.get("cache_n", ""),
                "prompt_n": timings.get("prompt_n", ""),
                "prompt_ms": timings.get("prompt_ms", ""),
                "predicted_n": timings.get("predicted_n", ""),
                "response_preview": re.sub(r"\s+", " ", response).strip()[:180],
            }
            rows.append(row)
            write_text(task_dir / f"{mode}.prompt.txt", prompt)
            write_text(task_dir / f"{mode}.response.txt", response)
            write_text(task_dir / f"{mode}.response.json", json.dumps(result, indent=2))
            print(f"{task['id']} {mode}: {len(hits)}/{total} cache_n={row['cache_n']} prompt_n={row['prompt_n']}")

    summary_path = run_dir / "summary.csv"
    with summary_path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)

    for mode in ("baseline", "primitive_cache", "routed_primitive_cache"):
        mode_rows = [r for r in rows if r["mode"] == mode]
        hits = sum(int(r["hits"]) for r in mode_rows)
        total = sum(int(r["total"]) for r in mode_rows)
        avg_cache = sum(float(r["cache_n"] or 0) for r in mode_rows) / len(mode_rows)
        avg_prompt = sum(float(r["prompt_n"] or 0) for r in mode_rows) / len(mode_rows)
        print(f"{mode}: {hits}/{total} = {hits / total:.4f}; avg cache_n={avg_cache:.1f}; avg prompt_n={avg_prompt:.1f}")
    print(f"Run: {run_dir}")
    print(f"Summary: {summary_path}")


if __name__ == "__main__":
    main()
