"""Per-task drill-down across campaign runs (pinpoints which tasks fail to transfer).

Usage:
  python scripts/campaign_per_task.py <run-name-substring> <suite>
  e.g. python scripts/campaign_per_task.py stageB holdout

Reads per_task.csv from each matching general-corpus-campaign run dir (one model per
dir) and prints a task_id x (model, condition) matrix of pass_count/repeats for the
chosen suite. A cell like "3/3" = passed every repeat; "0/3" = never passed.
"""

from __future__ import annotations

import csv
import glob
import json
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RUN_ROOT = ROOT / "rag_runs"

SHORT_MODEL = {
    "qwen35-2b-base-q4km-raw": "2b-q4km",
    "qwen35-2b-base-q8-raw": "2b-q8",
    "qwen35-4b-base-q6-raw": "4b-q6",
    "qwen35-4b-base-q8-raw": "4b-q8",
}
COND_ORDER = ["no_memory", "general_memory", "primitives", "combo"]
MODEL_ORDER = ["2b-q4km", "2b-q8", "4b-q6", "4b-q8"]


def main() -> int:
    if len(sys.argv) < 3:
        print("usage: python scripts/campaign_per_task.py <run-substring> <suite>")
        return 1
    flt, suite = sys.argv[1], sys.argv[2]

    # cell[(task, model_short, condition)] = "pass/repeats"
    cell: dict[tuple[str, str, str], str] = {}
    tasks: list[str] = []
    cols: list[tuple[str, str]] = []

    for d in sorted(glob.glob(str(RUN_ROOT / "general-corpus-campaign-*"))):
        base = os.path.basename(d)
        if flt not in base:
            continue
        ptc = os.path.join(d, "per_task.csv")
        rj = os.path.join(d, "report.json")
        if not (os.path.exists(ptc) and os.path.exists(rj)):
            continue
        model = json.load(open(rj, encoding="utf-8")).get("model", "?")
        ms = SHORT_MODEL.get(model, model)
        with open(ptc, encoding="utf-8") as fh:
            for row in csv.DictReader(fh):
                if row["suite"] != suite:
                    continue
                t = row["task_id"]
                c = row["condition"]
                if t not in tasks:
                    tasks.append(t)
                if (ms, c) not in cols:
                    cols.append((ms, c))
                cell[(t, ms, c)] = f"{row['pass_count']}/{row['repeats']}"

    if not cell:
        print(f"No per-task data (filter={flt!r}, suite={suite!r}).")
        return 0

    cols.sort(key=lambda x: (MODEL_ORDER.index(x[0]) if x[0] in MODEL_ORDER else 99,
                             COND_ORDER.index(x[1]) if x[1] in COND_ORDER else 99))
    tasks.sort()

    tw = max(len(t) for t in tasks) + 2
    print(f"SUITE={suite}  (cell = repeats passed / total repeats)\n")
    head = "task".ljust(tw) + "".join(f"{m}:{c}".replace("general_memory", "genmem").replace("no_memory", "nomem")[:16].ljust(17) for m, c in cols)
    print(head)
    print("-" * len(head))
    for t in tasks:
        line = t.ljust(tw) + "".join((cell.get((t, m, c), "-")).ljust(17) for m, c in cols)
        print(line)

    # category transfer summary: for each condition, count tasks that pass >=2/3 repeats, per model
    print("\nTasks passing >=2 of 3 repeats (by model x condition):")
    for m in MODEL_ORDER:
        present = [(mm, c) for (mm, c) in cols if mm == m]
        if not present:
            continue
        parts = []
        for _, c in present:
            n = 0
            for t in tasks:
                v = cell.get((t, m, c))
                if v:
                    p, r = v.split("/")
                    if int(p) * 3 >= int(r) * 2:
                        n += 1
            parts.append(f"{c}={n}")
        print(f"  {m:<8} " + "  ".join(parts))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
