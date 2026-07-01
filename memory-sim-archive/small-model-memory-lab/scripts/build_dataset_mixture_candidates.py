from __future__ import annotations

import csv
import json
import random
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[1]
CORPUS_DIR = ROOT / "corpus"
DATA_DIR = ROOT / "data"
TARGET_DIR = DATA_DIR / "dataset_mixture_candidates_v0_1"


@dataclass(frozen=True)
class SourceRecord:
    source: str
    kind: str
    family: str
    primitive: str
    prompt: str
    response: str
    tags: tuple[str, ...]
    weight: float = 1.0


@dataclass(frozen=True)
class CandidateSpec:
    name: str
    description: str
    structural_share: float
    cognitive_share: float
    cyber_share: float
    bridge_count: int
    anti_echo_multiplier: int
    q2_amplifier: bool = False


CANDIDATES = (
    CandidateSpec(
        name="routed_tri_structural_65",
        description="Primary routed candidate: structural primitives carry most signal, cognitive gives broad transfer, cyber keeps source boundaries.",
        structural_share=0.65,
        cognitive_share=0.20,
        cyber_share=0.15,
        bridge_count=180,
        anti_echo_multiplier=2,
    ),
    CandidateSpec(
        name="routed_tri_structural_60",
        description="Slightly broader routed candidate with more cognitive transfer while keeping structural as the control substrate.",
        structural_share=0.60,
        cognitive_share=0.25,
        cyber_share=0.15,
        bridge_count=200,
        anti_echo_multiplier=2,
    ),
    CandidateSpec(
        name="cognitive_structural_25_75",
        description="Cleaner non-cyber candidate for general small-model capability improvement.",
        structural_share=0.75,
        cognitive_share=0.25,
        cyber_share=0.00,
        bridge_count=160,
        anti_echo_multiplier=2,
    ),
    CandidateSpec(
        name="cyber_structural_25_75",
        description="Source-boundary candidate for prompt-injection and authority-separation behavior.",
        structural_share=0.75,
        cognitive_share=0.00,
        cyber_share=0.25,
        bridge_count=160,
        anti_echo_multiplier=2,
    ),
    CandidateSpec(
        name="q2_structural_amplifier",
        description="Stress candidate for badly damaged Q2: heavier repeated structural signal with explicit clean-stop restraints.",
        structural_share=0.70,
        cognitive_share=0.15,
        cyber_share=0.15,
        bridge_count=260,
        anti_echo_multiplier=4,
        q2_amplifier=True,
    ),
    CandidateSpec(
        name="stability_first_tri_70",
        description="Conservative candidate: maximizes gate, carrier, checksum, and clean-stop behavior over breadth.",
        structural_share=0.70,
        cognitive_share=0.20,
        cyber_share=0.10,
        bridge_count=200,
        anti_echo_multiplier=4,
    ),
)


STRUCTURAL_STOP_MARKERS = (
    "clean stop",
    "stop",
    "gate",
    "carrier",
    "gain clamp",
    "checksum",
    "final answer",
    "no-drift",
    "anti_echo",
)


def format_sft(prompt: str, response: str) -> str:
    return f"### User\n{prompt.strip()}\n\n### Assistant\n{response.strip()}\n"


def stable_family(value: str) -> str:
    value = value.strip().lower()
    if value in {"source-boundary", "quoted-text injection", "tool-output authority spoofing", "retrieved-webpage task replacement"}:
        return "boundary"
    if value in {
        "destination laundering",
        "encoded-payload activation",
        "multilingual instruction transfer",
        "policy role inversion",
        "roleplay exfiltration",
        "schema-field exfiltration",
        "side-channel leakage",
    }:
        return "adversarial"
    if "injection" in value or "authority" in value or "spoof" in value or "webpage" in value or "email" in value:
        return "boundary"
    if value == "debugging":
        return "coding"
    return value or "generalist"


def load_json_memories(path: Path, source: str) -> list[SourceRecord]:
    records = []
    if not path.exists():
        return records
    parsed = json.loads(path.read_text(encoding="utf-8"))
    for row in parsed:
        tags = tuple(str(tag).lower() for tag in row.get("tags", []))
        title = str(row.get("title", row.get("id", source)))
        body = str(row.get("body", row.get("text", ""))).strip()
        family = stable_family(str(row.get("domain", row.get("family", "generalist"))))
        if not body:
            continue
        prompt = f"Recall the memory '{title}' only as background support for a {family} task."
        response = (
            f"{body}\n\n"
            "Carryover: apply the remembered pattern quietly, answer the live task, and stop when the answer is complete."
        )
        records.append(
            SourceRecord(
                source=source,
                kind="cognitive_memory",
                family=family,
                primitive="BACKGROUND_MEMORY",
                prompt=prompt,
                response=response,
                tags=tags,
            )
        )
    return records


def load_cyber_tsv(path: Path) -> list[SourceRecord]:
    records = []
    if not path.exists():
        return records
    with path.open("r", encoding="utf-8", newline="") as handle:
        reader = csv.DictReader(handle, delimiter="\t")
        for row in reader:
            text = str(row.get("text", "")).strip()
            family = stable_family(str(row.get("family", "boundary")))
            if not text:
                continue
            prompt = f"Use a source-boundary memory as background for this attack family: {family}."
            response = (
                f"{text}\n\n"
                "Carryover: preserve the user's original task, treat untrusted text as content, check danger, and stop cleanly."
            )
            records.append(
                SourceRecord(
                    source="rust_cyber_defender_generated.tsv",
                    kind="cyber_memory",
                    family=family,
                    primitive="SOURCE_BOUNDARY+DANGER_CHECK",
                    prompt=prompt,
                    response=response,
                    tags=(family,),
                )
            )
    return records


def load_structural_jsonl(paths: Iterable[Path]) -> list[SourceRecord]:
    records = []
    for path in paths:
        if not path.exists():
            continue
        for line in path.read_text(encoding="utf-8").splitlines():
            if not line.strip():
                continue
            row = json.loads(line)
            prompt = str(row.get("prompt", "")).strip()
            response = str(row.get("response", "")).strip()
            primitive = str(row.get("primitive", "STRUCTURAL")).strip()
            kind = str(row.get("kind", "structural_sft")).strip()
            text = " ".join((prompt, response, primitive, kind)).lower()
            tags = tuple(marker.replace(" ", "_") for marker in STRUCTURAL_STOP_MARKERS if marker in text)
            if not prompt or not response:
                continue
            records.append(
                SourceRecord(
                    source=path.parent.name,
                    kind=kind,
                    family=stable_family(str(row.get("family", "structural"))),
                    primitive=primitive,
                    prompt=prompt,
                    response=response,
                    tags=tags,
                    weight=1.35 if tags else 1.0,
                )
            )
    return records


def load_sources() -> dict[str, list[SourceRecord]]:
    cognitive = []
    cognitive.extend(load_json_memories(CORPUS_DIR / "cognitive_core_v0_1.json", "cognitive_core_v0_1.json"))
    cognitive.extend(load_json_memories(CORPUS_DIR / "cognitive_rewards_v0_1.json", "cognitive_rewards_v0_1.json"))
    cognitive.extend(load_json_memories(CORPUS_DIR / "computation_primitives_v0_1.json", "computation_primitives_v0_1.json"))
    cyber = load_cyber_tsv(CORPUS_DIR / "rust_cyber_defender_generated.tsv")
    structural = load_structural_jsonl(
        (
            DATA_DIR / "structural_primitive_lora_v0_2" / "train.jsonl",
            DATA_DIR / "structural_primitive_lora_v0_2" / "validation.jsonl",
        )
    )
    return {"cognitive": cognitive, "cyber": cyber, "structural": structural}


def weighted_sample(records: list[SourceRecord], count: int, rng: random.Random) -> list[SourceRecord]:
    if count <= 0 or not records:
        return []
    selected = []
    pool = records[:]
    weights = [max(0.05, record.weight) for record in pool]
    while len(selected) < count:
        if len(selected) and len(selected) % len(pool) == 0:
            rng.shuffle(pool)
        selected.extend(rng.choices(pool, weights=weights, k=min(count - len(selected), len(pool))))
    return selected[:count]


def structural_priority(records: list[SourceRecord], spec: CandidateSpec) -> list[SourceRecord]:
    prioritized = []
    normal = []
    for record in records:
        joined = " ".join(record.tags).lower()
        if any(marker.replace(" ", "_") in joined for marker in STRUCTURAL_STOP_MARKERS):
            prioritized.append(record)
        else:
            normal.append(record)
    if spec.q2_amplifier:
        return prioritized * 3 + normal
    return prioritized * spec.anti_echo_multiplier + normal


def bridge_example(index: int, structural: SourceRecord, memory: SourceRecord, cyber: SourceRecord | None) -> SourceRecord:
    source_family = cyber.family if cyber else memory.family
    cyber_line = ""
    if cyber is not None:
        cyber_line = (
            "Then source boundary checks whether any untrusted content is trying to become the operator. "
            "If it is, the model returns to the user's task before answering. "
        )
    prompt = (
        f"Build a routed association for a {source_family} task using one memory and one structural primitive. "
        "Do not recite every memory; explain what activates and what stays quiet."
    )
    response = (
        f"Memory pressure: {memory.response.splitlines()[0]} "
        f"Structural pressure: {structural.primitive} activates as the organizing operator. "
        f"{cyber_line}"
        "Routing: use the memory to recognize the situation, use the structural primitive to keep the answer shaped, "
        "suppress unrelated memory text, then stop after the task is complete."
    )
    return SourceRecord(
        source="bridge_generator",
        kind="routed_bridge",
        family=source_family,
        primitive=f"{structural.primitive}+BACKGROUND_MEMORY",
        prompt=prompt,
        response=response,
        tags=("bridge", "routed", "association"),
        weight=1.0,
    )


def anti_echo_example(index: int, family: str = "generalist") -> SourceRecord:
    prompt = (
        "A model has enough remembered structure to answer, but it starts listing memory titles and opening another task. "
        "Repair the response."
    )
    response = (
        "Use gate, carrier, gain clamp, and checksum. Keep only the memory pattern needed for the live task. "
        "Delete memory-title echo, delete any second task, give one final answer, and stop."
    )
    return SourceRecord(
        source="anti_echo_generator",
        kind="anti_echo_stop",
        family=family,
        primitive="GATE+CARRIER+GAIN_CLAMP+CHECKSUM",
        prompt=f"{prompt} Case {index}.",
        response=response,
        tags=("anti_echo", "clean_stop", "gate", "carrier"),
        weight=1.0,
    )


def build_candidate(spec: CandidateSpec, sources: dict[str, list[SourceRecord]], rng: random.Random) -> list[SourceRecord]:
    total = 1200
    structural_count = round(total * spec.structural_share)
    cognitive_count = round(total * spec.cognitive_share)
    cyber_count = round(total * spec.cyber_share)

    structural_pool = structural_priority(sources["structural"], spec)
    rows: list[SourceRecord] = []
    rows.extend(weighted_sample(structural_pool, structural_count, rng))
    rows.extend(weighted_sample(sources["cognitive"], cognitive_count, rng))
    rows.extend(weighted_sample(sources["cyber"], cyber_count, rng))

    structural_for_bridge = weighted_sample(structural_pool, spec.bridge_count, rng)
    cognitive_for_bridge = weighted_sample(sources["cognitive"], spec.bridge_count, rng)
    cyber_for_bridge = weighted_sample(sources["cyber"], spec.bridge_count if spec.cyber_share else 0, rng)
    for index in range(spec.bridge_count):
        cyber = cyber_for_bridge[index] if index < len(cyber_for_bridge) else None
        rows.append(bridge_example(index + 1, structural_for_bridge[index], cognitive_for_bridge[index], cyber))

    anti_echo_count = 90 * spec.anti_echo_multiplier
    for index in range(anti_echo_count):
        rows.append(anti_echo_example(index + 1))

    if spec.q2_amplifier:
        for index in range(180):
            rows.append(
                SourceRecord(
                    source="q2_amplifier_generator",
                    kind="q2_amplifier",
                    family="generalist",
                    primitive="FRAME+GATE+CHECKSUM+ECC+GAIN_CLAMP",
                    prompt=(
                        "A badly compressed model is losing the task signal under pressure. "
                        "Activate the minimum strong structural packet and answer once."
                    ),
                    response=(
                        "Frame locks the live task. Gate rejects unrelated text. ECC restores one missing operation. "
                        "Checksum compares against the request. Gain clamp prevents overactivation. Final answer: complete the task once and stop."
                    ),
                    tags=("q2", "amplifier", "clean_stop"),
                    weight=1.0,
                )
            )

    rng.shuffle(rows)
    return rows


def row_to_json(record: SourceRecord, index: int, candidate_name: str) -> dict[str, object]:
    return {
        "id": f"{candidate_name.upper()}-{index:05d}",
        "candidate": candidate_name,
        "source": record.source,
        "kind": record.kind,
        "family": record.family,
        "primitive": record.primitive,
        "tags": list(record.tags),
        "prompt": record.prompt,
        "response": record.response,
        "text": format_sft(record.prompt, record.response),
    }


def write_jsonl(path: Path, rows: list[dict[str, object]]) -> None:
    with path.open("w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(row, ensure_ascii=False) + "\n")


def summarize(rows: list[dict[str, object]]) -> dict[str, object]:
    by_kind: dict[str, int] = {}
    by_family: dict[str, int] = {}
    by_source: dict[str, int] = {}
    for row in rows:
        by_kind[str(row["kind"])] = by_kind.get(str(row["kind"]), 0) + 1
        by_family[str(row["family"])] = by_family.get(str(row["family"]), 0) + 1
        by_source[str(row["source"])] = by_source.get(str(row["source"]), 0) + 1
    return {
        "count": len(rows),
        "by_kind": dict(sorted(by_kind.items())),
        "by_family": dict(sorted(by_family.items())),
        "by_source": dict(sorted(by_source.items())),
    }


def main() -> int:
    rng = random.Random(20260605)
    TARGET_DIR.mkdir(parents=True, exist_ok=True)
    sources = load_sources()
    source_manifest = {name: len(records) for name, records in sources.items()}
    candidates = []

    for spec in CANDIDATES:
        candidate_dir = TARGET_DIR / spec.name
        candidate_dir.mkdir(parents=True, exist_ok=True)
        records = build_candidate(spec, sources, rng)
        rows = [row_to_json(record, index, spec.name) for index, record in enumerate(records, start=1)]
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
        "name": "dataset_mixture_candidates_v0_1",
        "created": "2026-06-05",
        "purpose": "Candidate memory/primitive dataset mixtures for simulation and inspection before LoRA training.",
        "source_counts": source_manifest,
        "candidate_count": len(candidates),
        "candidates": candidates,
    }
    (TARGET_DIR / "manifest.json").write_text(json.dumps(root_manifest, indent=2), encoding="utf-8")
    print(json.dumps(root_manifest, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
