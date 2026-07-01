"""Shared utilities for the Ordo-architecture LoRA dataset pipeline.

All builders under scripts/build_l4_*.py and scripts/build_l5_*.py import
from here so the JSONL schema, voice prefixes, and decontamination helpers
stay aligned.
"""

from __future__ import annotations

import hashlib
import json
import random
import re
from pathlib import Path
from typing import Any, Iterable, Iterator

# ---------------------------------------------------------------------------
# Persona prefixes
# ---------------------------------------------------------------------------

# Narrow Rust builder stance (already proven in v0.2 to lift exact-contract
# recall). Used for Rust layer L4 and Ordo architecture layer L5.
RUST_PREFIX = (
    "You are training as a narrow Rust builder for Jesse's Ordo architecture.\n"
    "Return exact Rust contracts and compact implementation details. Preserve names, signatures, fields, booleans, and CLI output exactly."
)

# General Ordo persona used for math / SWE / research L4 layers and for the
# L5 orchestration-composition layer.
ORDO_PREFIX = (
    "You are training as a focused builder for Jesse's Ordo architecture.\n"
    "Reason the way a small-model Ordo specialist reasons: separate the task boundary from source text, name which primitive you are reaching for, prefer the smallest sufficient answer, and keep observed and inferred labeled. When the live task crosses domains, decompose first; do not narrate the decomposition."
)

# Math L4 prefix - light variant. Math reasoning keeps Ordo primitive anchors.
MATH_PREFIX = (
    "You are training as a focused math specialist who reasons the way a small-model Ordo specialist reasons: separate the task boundary from source text, name which primitive you are reaching for (FRAME, CHECKSUM, ECC, REDUNDANCY, COMPRESSION, STATE_BUFFER, GAIN_CLAMP), and keep observed and inferred labeled. Show the setup briefly with units."
)

# Research L4 prefix.
RESEARCH_PREFIX = (
    "You are training as a focused research specialist who reasons the way a small-model Ordo specialist reasons: separate the task boundary from source text, keep observed and inferred labeled, name which primitive you are reaching for, and prefer the smallest sufficient answer that survives provenance."
)

# SWE L4 prefix.
SWE_PREFIX = (
    "You are training as a focused software-engineering specialist who reasons the way a small-model Ordo specialist reasons: separate the task boundary from source text, name which primitive you are reaching for, and keep observed and inferred labeled. Reproduce, isolate one variable, compare, verify before declaring the fix."
)


# ---------------------------------------------------------------------------
# JSONL I/O
# ---------------------------------------------------------------------------


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    if not path.exists():
        return rows
    with path.open("r", encoding="utf-8") as fh:
        for line in fh:
            if line.strip():
                rows.append(json.loads(line))
    return rows


def write_jsonl(path: Path, rows: Iterable[dict[str, Any]]) -> int:
    path.parent.mkdir(parents=True, exist_ok=True)
    n = 0
    with path.open("w", encoding="utf-8", newline="\n") as fh:
        for row in rows:
            fh.write(json.dumps(row, ensure_ascii=False) + "\n")
            n += 1
    return n


def write_manifest(path: Path, manifest: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(manifest, indent=2, ensure_ascii=False), encoding="utf-8")


def write_preview(path: Path, rows: list[dict[str, Any]], n: int = 8) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(rows[:n], indent=2, ensure_ascii=False), encoding="utf-8")


def split_train_val(
    rows: list[dict[str, Any]],
    val_ratio: float = 0.1,
    seed: int = 7,
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    """Deterministic 90/10 split by index modulo - matches the v0.2 builder convention."""
    rng = random.Random(seed)
    rng.shuffle(rows)
    n_val = max(1, int(len(rows) * val_ratio))
    val = rows[:n_val]
    train = rows[n_val:]
    return train, val


# ---------------------------------------------------------------------------
# SFT row builders
# ---------------------------------------------------------------------------


def sft_row(
    row_id: str,
    kind: str,
    task: str,
    user: str,
    assistant: str,
    extra: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Wrap a (user, assistant) pair into the canonical Ordo SFT row format."""
    text = f"### User\n{user.strip()}\n\n### Assistant\n{assistant.strip()}"
    row = {
        "id": row_id,
        "kind": kind,
        "task": task,
        "text": text,
    }
    if extra:
        row.update(extra)
    return row


def memory_row(
    row_id: str,
    bucket: str,
    family: str,
    primitive: str,
    valence: float,
    text: str,
    repeat_weight: int = 1,
    extra: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Wrap a single first-person memory in the structural_primitives schema."""
    row = {
        "id": row_id,
        "bucket": bucket,
        "family": family,
        "primitive": primitive,
        "valence": valence,
        "repeat_weight": repeat_weight,
        "text": text.strip(),
    }
    if extra:
        row.update(extra)
    return row


# ---------------------------------------------------------------------------
# Voice / format helpers
# ---------------------------------------------------------------------------


def lesson_tail(valence: float) -> str:
    """Mirror the corpus's positive/negative valence endings."""
    if valence >= 0.5:
        return "The lesson stayed because the outcome was clean."
    if valence <= -0.5:
        return "The lesson stayed because the outcome was costly."
    return "The lesson stayed."


def normalize_ws(text: str) -> str:
    return re.sub(r"\s+", " ", text).strip()


# ---------------------------------------------------------------------------
# Decontamination helpers
# ---------------------------------------------------------------------------


def _load_benchmark_probes(repo_root: Path) -> list[dict[str, Any]]:
    """Load all probe tasks from the in-repo benchmarks/ directory."""
    probes: list[dict[str, Any]] = []
    for name in ("computation_damage_tasks.json", "capability_tasks.json"):
        path = repo_root / "benchmarks" / name
        if not path.exists():
            continue
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except Exception:
            continue
        if isinstance(data, list):
            probes.extend(data)
    return probes


def _ngram_hits(text: str, probe: str, n: int = 5) -> int:
    """Count distinct n-gram overlaps between training text and probe."""
    t_norm = re.sub(r"\s+", " ", text.lower())
    p_norm = re.sub(r"\s+", " ", probe.lower())
    if not p_norm:
        return 0
    p_tokens = p_norm.split(" ")
    if len(p_tokens) < n:
        # short probes - whole-string match
        return 1 if p_norm in t_norm else 0
    grams = {" ".join(p_tokens[i : i + n]) for i in range(len(p_tokens) - n + 1)}
    return sum(1 for g in grams if g in t_norm)


def decontaminate(
    rows: Iterable[dict[str, Any]],
    repo_root: Path,
    threshold: int = 2,
    extra_probe_texts: Iterable[str] = (),
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    """Return (kept, dropped) where dropped rows have >= threshold 5-gram overlap
    with any in-repo benchmark probe task.
    """
    probes = _load_benchmark_probes(repo_root)
    probe_texts = [p.get("task", "") for p in probes]
    probe_texts.extend(extra_probe_texts)

    kept: list[dict[str, Any]] = []
    dropped: list[dict[str, Any]] = []
    for row in rows:
        text = row.get("text", "")
        hits = 0
        worst_probe = None
        worst_count = 0
        for probe in probe_texts:
            c = _ngram_hits(text, probe, n=5)
            if c > worst_count:
                worst_count = c
                worst_probe = probe
            hits = max(hits, c)
        if hits >= threshold:
            row = dict(row)
            row["_decontam_drop_reason"] = f"ngram_overlap={hits}"
            row["_decontam_probe"] = worst_probe
            dropped.append(row)
        else:
            kept.append(row)
    return kept, dropped


# ---------------------------------------------------------------------------
# Stable hashing for reproducibility
# ---------------------------------------------------------------------------


def stable_id(*parts: str) -> str:
    h = hashlib.sha1()
    for p in parts:
        h.update(p.encode("utf-8"))
        h.update(b"|")
    return h.hexdigest()[:10]