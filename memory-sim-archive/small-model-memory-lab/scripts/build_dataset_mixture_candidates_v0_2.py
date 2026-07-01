from __future__ import annotations

import json
import random
from pathlib import Path

from build_dataset_mixture_candidates import (
    DATA_DIR,
    CandidateSpec,
    anti_echo_example,
    build_candidate,
    load_sources,
    row_to_json,
    summarize,
    write_jsonl,
)


ROOT = Path(__file__).resolve().parents[1]
TARGET_DIR = DATA_DIR / "dataset_mixture_candidates_v0_2"


CANDIDATES = (
    CandidateSpec(
        name="q3_stability_tri_72",
        description="Q3-focused stability candidate with heavier structural substrate and more clean-stop pressure.",
        structural_share=0.72,
        cognitive_share=0.18,
        cyber_share=0.10,
        bridge_count=260,
        anti_echo_multiplier=5,
    ),
    CandidateSpec(
        name="q3_routed_tri_68",
        description="Q3 routed candidate: slightly more breadth than stability_tri_72 while keeping structural dominant.",
        structural_share=0.68,
        cognitive_share=0.20,
        cyber_share=0.12,
        bridge_count=280,
        anti_echo_multiplier=3,
    ),
    CandidateSpec(
        name="q4_generalist_pair_72_28",
        description="Q4 generalist pair: structural plus cognitive memories, no cyber specialization.",
        structural_share=0.72,
        cognitive_share=0.28,
        cyber_share=0.00,
        bridge_count=220,
        anti_echo_multiplier=3,
    ),
    CandidateSpec(
        name="q4_boundary_pair_72_28",
        description="Q4 boundary pair: structural plus cyber/source-boundary memories, no cognitive breadth.",
        structural_share=0.72,
        cognitive_share=0.00,
        cyber_share=0.28,
        bridge_count=220,
        anti_echo_multiplier=3,
    ),
    CandidateSpec(
        name="q2_amplifier_low_echo",
        description="Lower-echo Q2 amplifier: keeps strong structural signal but adds more explicit stop/closure pressure.",
        structural_share=0.72,
        cognitive_share=0.14,
        cyber_share=0.14,
        bridge_count=300,
        anti_echo_multiplier=6,
        q2_amplifier=True,
    ),
)


def add_extra_low_echo_rows(rows: list[dict[str, object]], candidate_name: str, count: int) -> None:
    start = len(rows) + 1
    for offset in range(count):
        record = anti_echo_example(offset + 1, "generalist")
        response = (
            "The structural packet has fired enough. Gate closes, carrier holds only the live task, "
            "gain clamp lowers repetition, checksum confirms the answer, and the model stops. "
            "Final answer: one completed answer, no memory list, no second task."
        )
        rows.append(
            {
                "id": f"{candidate_name.upper()}-{start + offset:05d}",
                "candidate": candidate_name,
                "source": "low_echo_generator",
                "kind": "low_echo_stop",
                "family": record.family,
                "primitive": record.primitive,
                "tags": ["anti_echo", "clean_stop", "closure"],
                "prompt": record.prompt,
                "response": response,
                "text": f"### User\n{record.prompt}\n\n### Assistant\n{response}\n",
            }
        )


def main() -> int:
    rng = random.Random(20260605)
    TARGET_DIR.mkdir(parents=True, exist_ok=True)
    sources = load_sources()
    candidates = []

    for spec in CANDIDATES:
        candidate_dir = TARGET_DIR / spec.name
        candidate_dir.mkdir(parents=True, exist_ok=True)
        records = build_candidate(spec, sources, rng)
        rows = [row_to_json(record, index, spec.name) for index, record in enumerate(records, start=1)]
        if spec.name == "q2_amplifier_low_echo":
            add_extra_low_echo_rows(rows, spec.name, 220)
        validation_count = max(80, round(len(rows) * 0.08))
        validation = rows[:validation_count]
        train = rows[validation_count:]
        write_jsonl(candidate_dir / "train.jsonl", train)
        write_jsonl(candidate_dir / "validation.jsonl", validation)
        (candidate_dir / "preview.json").write_text(json.dumps(rows[:25], indent=2), encoding="utf-8")
        manifest = {
            "name": spec.name,
            "description": spec.description,
            "shares": {
                "structural": spec.structural_share,
                "cognitive": spec.cognitive_share,
                "cyber": spec.cyber_share,
            },
            "bridge_count": spec.bridge_count,
            "anti_echo_multiplier": spec.anti_echo_multiplier,
            "q2_amplifier": spec.q2_amplifier,
            "train_count": len(train),
            "validation_count": len(validation),
            "summary": summarize(rows),
            "files": {
                "train_jsonl": str(candidate_dir / "train.jsonl"),
                "validation_jsonl": str(candidate_dir / "validation.jsonl"),
                "preview": str(candidate_dir / "preview.json"),
            },
        }
        (candidate_dir / "manifest.json").write_text(json.dumps(manifest, indent=2), encoding="utf-8")
        candidates.append(manifest)

    root_manifest = {
        "name": "dataset_mixture_candidates_v0_2",
        "created": "2026-06-05",
        "purpose": "Second candidate iteration after v0.1 simulation: tighter Q3/Q4 mixtures and lower-echo Q2 amplifier.",
        "source_counts": {name: len(records) for name, records in sources.items()},
        "candidate_count": len(candidates),
        "candidates": candidates,
    }
    (TARGET_DIR / "manifest.json").write_text(json.dumps(root_manifest, indent=2), encoding="utf-8")
    print(json.dumps(root_manifest, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
