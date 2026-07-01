import json, glob, os

# prompt-side bar (held-out, repeats=3) for reference
PROMPT_BAR = {
    "Qwen3.5-2B-Base": {"bare": 0, "answer-only(best prompt)": 7},
    "Qwen3.5-4B-Base": {"bare": 10, "answer-only(best prompt)": 8},
}

dirs = sorted(glob.glob("rag_runs/steering-*"))
# keep latest run dir per model slug
latest = {}
for d in dirs:
    base = os.path.basename(d)
    # steering-<slug>-<stamp>
    slug = base[len("steering-"):].rsplit("-", 2)[0]
    latest[slug] = d  # sorted asc -> last wins = latest

for slug, d in latest.items():
    rj = os.path.join(d, "report.json")
    if not os.path.exists(rj):
        continue
    r = json.load(open(rj))
    rows = r["rows"]
    print(f"\n===== {slug}  ({os.path.basename(d)}) =====")
    for suite in ["dev", "holdout"]:
        srows = [x for x in rows if x["suite"] == suite]
        if not srows:
            continue
        base = next((x["passed"] for x in srows if x["layer"] == "none"), None)
        steered = [x for x in srows if x["layer"] != "none"]
        steered.sort(key=lambda x: -x["passed"])
        n = srows[0]["tasks"]
        print(f"  {suite}: baseline(no-steer)={base}/{n}", end="")
        if slug in PROMPT_BAR:
            print(f"   [prompt-side bar: bare={PROMPT_BAR[slug]['bare']}, best-prompt={PROMPT_BAR[slug]['answer-only(best prompt)']}]", end="")
        print()
        for x in steered[:6]:
            flag = "  <-- best" if x is steered[0] else ""
            print(f"      L={x['layer']:<3} alpha={x['alpha']:<4} -> {x['passed']}/{n}{flag}")
