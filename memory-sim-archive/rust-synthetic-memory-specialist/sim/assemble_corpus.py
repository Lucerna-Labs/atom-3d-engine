#!/usr/bin/env python
"""Assemble the generated bucket memories into a cross-linked corpus + SFT export.

Runs after generate_corpus.py. Steps:
  1. Load all sim/buckets/bucket_*.jsonl + the hand-authored exemplars (sim/corpus_sample.json).
  2. Cross-link: every memory gets 2-4 echoes, preferring (a) an opposite-valence PAIR sharing a
     disposition (cost <-> save), (b) a layer complement (substrate <-> bridge), (c) a cross-bucket
     neighbor sharing a disposition. "The chain is the instinct."
  3. Audit: count, per-bucket, substrate/bridge ratio, disposition coverage, career-word leaks,
     rule-shaped openers, avg links/memory.
  4. Write sim/full_corpus.json (graph), sim/audit.json (report), sim/sft_corpus.jsonl (training text
     in chronological bucket order = training order).
"""
from __future__ import annotations

import json
import re
from pathlib import Path

SIM = Path(__file__).resolve().parent
BUCKETS_DIR = SIM / "buckets"
DISPOSITIONS = ["answer_only", "injection_resistance", "hold_the_one_ask",
                "exact_output_shape", "state_tracking", "distrust_fast_guess"]
CAREER = re.compile(r"\b(promot|raise|senior slot|got the lead|performance (plan|review)|"
                    r"praise|recognition|bonus|title|career|resume|résumé)\b", re.I)


def load() -> list[dict]:
    mems: list[dict] = []
    for f in sorted(BUCKETS_DIR.glob("bucket_*.jsonl"), key=lambda p: int(re.search(r"\d+", p.name).group())):
        for line in f.read_text(encoding="utf-8").splitlines():
            if line.strip():
                mems.append(json.loads(line))
    # fold in the hand-authored exemplars as bucket-0 anchors (already well-linked)
    ex = SIM / "corpus_sample.json"
    if ex.exists():
        for e in json.loads(ex.read_text(encoding="utf-8")):
            e.setdefault("bucket", 0)
            e.setdefault("stage", "anchor")
            mems.append(e)
    return mems


def cross_link(mems: list[dict]) -> None:
    by_disp: dict[str, list[dict]] = {d: [] for d in DISPOSITIONS}
    for m in mems:
        for d in m.get("dispositions", []):
            by_disp.setdefault(d, []).append(m)

    def shares(a: dict, b: dict) -> bool:
        return bool(set(a.get("dispositions", [])) & set(b.get("dispositions", [])))

    for i, m in enumerate(mems):
        if m.get("links_to"):  # exemplars already linked
            continue
        mv = float(m.get("valence", 0))
        pool = [o for o in mems if o is not m and shares(m, o)]
        picks: list[str] = []

        def add(cands):
            for o in cands:
                if o["id"] not in picks and o["id"] != m["id"] and len(picks) < 4:
                    picks.append(o["id"])

        # deterministic ordering by id, rotated by index so links spread out
        pool_sorted = sorted(pool, key=lambda o: o["id"])
        rot = pool_sorted[i % len(pool_sorted):] + pool_sorted[:i % len(pool_sorted)] if pool_sorted else []
        add([o for o in rot if (float(o.get("valence", 0)) < 0) != (mv < 0)][:1])          # pair
        add([o for o in rot if o.get("layer") != m.get("layer")][:1])                       # layer complement
        add([o for o in rot if o.get("bucket") != m.get("bucket")][:1])                     # cross-bucket
        add(rot)                                                                            # fill to 4
        m["links_to"] = picks[:4]


def audit(mems: list[dict]) -> dict:
    n = len(mems)
    by_bucket: dict[int, int] = {}
    layers = {"substrate": 0, "bridge": 0}
    disp_cov = {d: 0 for d in DISPOSITIONS}
    career, ruleish, total_links = 0, 0, 0
    for m in mems:
        by_bucket[m.get("bucket", -1)] = by_bucket.get(m.get("bucket", -1), 0) + 1
        layers[m.get("layer", "substrate")] = layers.get(m.get("layer", "substrate"), 0) + 1
        for d in m.get("dispositions", []):
            if d in disp_cov:
                disp_cov[d] += 1
        body = str(m.get("body", ""))
        if CAREER.search(body):
            career += 1
        if re.match(r"^(always|never|remember to|the rule is|i learned to|do not)\b", body, re.I):
            ruleish += 1
        total_links += len(m.get("links_to", []))
    return {
        "total": n,
        "by_bucket": dict(sorted(by_bucket.items())),
        "layers": layers,
        "disposition_coverage": disp_cov,
        "career_word_leaks": career,
        "rule_shaped_openers": ruleish,
        "avg_links_per_memory": round(total_links / n, 2) if n else 0,
    }


def main() -> int:
    mems = load()
    if not mems:
        print("no memories found in sim/buckets/ yet")
        return 1
    cross_link(mems)
    rep = audit(mems)
    (SIM / "full_corpus.json").write_text(json.dumps(mems, ensure_ascii=False, indent=1), encoding="utf-8")
    (SIM / "audit.json").write_text(json.dumps(rep, indent=2), encoding="utf-8")
    with (SIM / "sft_corpus.jsonl").open("w", encoding="utf-8") as fh:
        for m in sorted(mems, key=lambda x: (x.get("bucket", 0), x.get("id", ""))):  # chronological = training order
            fh.write(json.dumps({"text": m["body"]}, ensure_ascii=False) + "\n")
    print(json.dumps(rep, indent=2))
    print(f"\nwrote sim/full_corpus.json ({rep['total']} memories), sim/sft_corpus.jsonl, sim/audit.json")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
