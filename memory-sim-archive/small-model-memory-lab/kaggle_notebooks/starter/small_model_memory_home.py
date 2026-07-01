"""Small Model Memory Lab home base on Kaggle.

This first Kaggle kernel validates that the uploaded dataset is readable,
summarizes the memory corpora and benchmark tasks, and runs a lightweight
retrieval sanity check without downloading model weights.
"""

from __future__ import annotations

import json
import math
import os
import re
from collections import Counter, defaultdict
from pathlib import Path


DATASET_REF = "jessealicea/small-model-memory-lab"
KAGGLE_INPUT = Path("/kaggle/input/small-model-memory-lab")
LOCAL_FALLBACK = Path(".")


def find_root() -> Path | None:
    if KAGGLE_INPUT.exists():
        return KAGGLE_INPUT
    kaggle_input_root = Path("/kaggle/input")
    if kaggle_input_root.exists():
        for candidate in kaggle_input_root.iterdir():
            if (candidate / "benchmarks").exists() and (candidate / "corpus").exists():
                return candidate
        for benchmark_dir in kaggle_input_root.rglob("benchmarks"):
            candidate = benchmark_dir.parent
            if (candidate / "corpus").exists():
                return candidate
    if (LOCAL_FALLBACK / "benchmarks").exists() and (LOCAL_FALLBACK / "corpus").exists():
        return LOCAL_FALLBACK
    return None


def load_json(path: Path):
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def tokenize(text: str) -> set[str]:
    return {
        token
        for token in re.findall(r"[a-z0-9][a-z0-9_-]{2,}", text.lower())
        if token
        not in {
            "the",
            "and",
            "for",
            "that",
            "with",
            "this",
            "you",
            "are",
            "from",
            "into",
            "what",
            "when",
            "how",
            "why",
        }
    }


def memory_text(memory: dict) -> str:
    return " ".join(
        str(memory.get(key, ""))
        for key in ["id", "domain", "title", "tags", "body"]
    )


def score_memory(task: dict, memory: dict) -> float:
    task_terms = tokenize(task.get("task", ""))
    memory_terms = tokenize(memory_text(memory))
    if not task_terms or not memory_terms:
        return 0.0
    overlap = len(task_terms & memory_terms)
    return overlap / math.sqrt(len(task_terms) * len(memory_terms))


def retrieve_memories(task: dict, memories: list[dict], limit: int = 5) -> list[tuple[float, dict]]:
    scored = [(score_memory(task, memory), memory) for memory in memories]
    scored.sort(key=lambda item: item[0], reverse=True)
    return [item for item in scored[:limit] if item[0] > 0]


def summarize_tasks(tasks: list[dict]) -> dict:
    families = Counter(task.get("family", "unknown") for task in tasks)
    criteria_count = sum(len(task.get("criteria", [])) for task in tasks)
    return {
        "tasks": len(tasks),
        "families": dict(sorted(families.items())),
        "criteria": criteria_count,
    }


def summarize_memories(memories: list[dict]) -> dict:
    domains = Counter(memory.get("domain", "unknown") for memory in memories)
    tags = Counter(tag for memory in memories for tag in memory.get("tags", []))
    return {
        "memories": len(memories),
        "domains": dict(sorted(domains.items())),
        "top_tags": tags.most_common(12),
    }


def corpus_entry_count(value) -> int:
    if isinstance(value, list):
        return len(value)
    if isinstance(value, dict):
        for key in ["memories", "items", "entries"]:
            if isinstance(value.get(key), list):
                return len(value[key])
        return len(value)
    return 0


def extract_memories(value) -> list[dict]:
    if isinstance(value, list):
        return [item for item in value if isinstance(item, dict)]
    if isinstance(value, dict):
        for key in ["memories", "items", "entries"]:
            if isinstance(value.get(key), list):
                return [item for item in value[key] if isinstance(item, dict)]
    return []


def main() -> None:
    root = find_root()
    if root is None:
        print(f"Dataset ref: {DATASET_REF}")
        print(f"Expected Kaggle input at: {KAGGLE_INPUT}")
        input_root = Path("/kaggle/input")
        if input_root.exists():
            visible = sorted(path.name for path in input_root.iterdir())
            print(f"Visible Kaggle inputs: {visible}")
        else:
            print("No /kaggle/input directory is visible in this run.")
        print(
            "The kernel source is valid, but Kaggle did not mount the dataset for "
            "this run. Reopen the kernel in Kaggle, verify the dataset source is "
            "attached, and run again."
        )
        return

    print(f"Dataset ref: {DATASET_REF}")
    print(f"Dataset root: {root}")
    print(f"Kaggle kernel: {bool(os.environ.get('KAGGLE_KERNEL_RUN_TYPE'))}")

    benchmark_paths = sorted((root / "benchmarks").glob("*.json"))
    corpus_paths = sorted((root / "corpus").glob("*.json"))

    benchmarks = {path.name: load_json(path) for path in benchmark_paths}
    corpora = {path.name: load_json(path) for path in corpus_paths}
    all_memories = []
    for value in corpora.values():
        all_memories.extend(extract_memories(value))

    print("\nBenchmark summary")
    for name, tasks in benchmarks.items():
        print(f"- {name}: {json.dumps(summarize_tasks(tasks), sort_keys=True)}")

    print("\nCorpus summary")
    for name, value in corpora.items():
        print(f"- {name}: {corpus_entry_count(value)} entries")

    if all_memories:
        print("\nCombined memory summary")
        print(json.dumps(summarize_memories(all_memories), indent=2, sort_keys=True))

    print("\nRetrieval sanity check")
    retrieval_rows = []
    for benchmark_name, tasks in benchmarks.items():
        for task in tasks:
            hits = retrieve_memories(task, all_memories, limit=5)
            retrieval_rows.append(
                {
                    "benchmark": benchmark_name,
                    "task_id": task.get("id"),
                    "family": task.get("family"),
                    "hits": len(hits),
                    "top_memory": hits[0][1].get("id") if hits else None,
                    "top_score": round(hits[0][0], 4) if hits else 0,
                }
            )

    families = defaultdict(list)
    for row in retrieval_rows:
        families[row["family"]].append(row["top_score"])

    for family, scores in sorted(families.items()):
        avg = sum(scores) / len(scores)
        print(f"- {family}: {len(scores)} tasks, average top-memory score {avg:.4f}")

    weakest = sorted(retrieval_rows, key=lambda row: row["top_score"])[:10]
    print("\nWeakest retrieval targets")
    for row in weakest:
        print(
            f"- {row['benchmark']} :: {row['task_id']} "
            f"({row['family']}): top={row['top_memory']} score={row['top_score']}"
        )

    print("\nNext Kaggle step")
    print(
        "Attach a small Hugging Face model or Kaggle Model source, then run the same "
        "benchmark tasks with prompt-memory, primitive-cache, and no-memory conditions."
    )


if __name__ == "__main__":
    main()
