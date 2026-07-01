"""Aggregate general-corpus-campaign run dirs into a comparison table.

Usage:
  python scripts/aggregate_campaign.py [substring-filter]

Scans rag_runs/general-corpus-campaign-* (optionally filtered by a substring of the
dir name, e.g. "screenA" or a date), and prints one row per (model, suite, condition,
profile) with mean/min/max passed across repeats. Reads report.json's summary[].
"""

from __future__ import annotations

import glob
import json
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RUN_ROOT = ROOT / "rag_runs"


def main() -> int:
    flt = sys.argv[1] if len(sys.argv) > 1 else ""
    rows: list[tuple] = []
    for d in sorted(glob.glob(str(RUN_ROOT / "general-corpus-campaign-*"))):
        base = os.path.basename(d)
        if flt and flt not in base:
            continue
        rj = os.path.join(d, "report.json")
        if not os.path.exists(rj):
            continue
        try:
            r = json.load(open(rj, encoding="utf-8"))
        except Exception:
            continue
        model = r.get("model", "?")
        profile = r.get("primitive_profile", "?")
        routed = r.get("routed_memory", "?")
        mem = os.path.basename(str(r.get("memory_path", "")))
        for s in r.get("summary", []):
            rows.append(
                (
                    model,
                    s.get("suite", "?"),
                    s.get("condition", "?"),
                    profile,
                    f"{s.get('mean_passed', '?')}",
                    f"{s.get('min_passed', '?')}-{s.get('max_passed', '?')}",
                    s.get("tasks", "?"),
                    routed,
                    mem,
                    base[24:],
                )
            )

    if not rows:
        print(f"No campaign runs found (filter={flt!r}).")
        return 0

    header = ("model", "suite", "condition", "profile", "mean", "min-max", "n", "routed", "memory", "run")
    widths = [26, 8, 16, 14, 6, 9, 4, 7, 26, 20]
    print("".join(str(h).ljust(w) for h, w in zip(header, widths)))
    print("-" * sum(widths))
    # sort by model, suite, then mean desc
    for row in sorted(rows, key=lambda x: (x[0], x[1], -float(x[4]) if x[4] not in ("?", "") else 0)):
        print("".join(str(c).ljust(w) for c, w in zip(row, widths)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
