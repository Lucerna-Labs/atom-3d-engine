from __future__ import annotations

import json
import sys
import os
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SCRIPT_DIR = ROOT / "scripts"
PY_RUN = ROOT / "rag_runs" / "python-domain-scar-20260611-141018" / "routed_stack_memories"
ORDO_RUN = ROOT / "rag_runs" / "ordo-extended-suite-20260611-141111" / "routed_stack_memories"
OUT_PATH = ROOT / "data" / "routed_memory_lora_sft.jsonl"
MANIFEST_PATH = ROOT / "data" / "routed_memory_lora_sft_manifest.json"
ROUTED_LORA_OUTPUT = Path(os.environ.get("ROUTED_LORA_OUTPUT", str(OUT_PATH)))
ROUTED_LORA_MANIFEST = Path(os.environ.get("ROUTED_LORA_MANIFEST", str(MANIFEST_PATH)))
ROUTED_LORA_INCLUDE_PRIMITIVES = os.environ.get("ROUTED_LORA_INCLUDE_PRIMITIVES", "0") in {
    "1",
    "true",
    "True",
    "yes",
    "y",
}
ROUTED_LORA_PRIMITIVE_PROFILE = os.environ.get("ROUTED_LORA_PRIMITIVE_PROFILE", "compact")

RUST_EXT = ".rs"
RUST_LIB = "lib" + RUST_EXT
RUST_MAIN = "main" + RUST_EXT
SRC_LIB = "src/" + RUST_LIB
SRC_MAIN = "src/" + RUST_MAIN

sys.path.insert(0, str(SCRIPT_DIR))
import run_ordo_extended_suite_benchmark as ordo_bench  # noqa: E402
import run_python_domain_scar_benchmark as py_bench  # noqa: E402


SYSTEM_INTERNALIZED = (
    "You are a compact coding model with domain-selective scar memories internalized. "
    "Use the relevant learned scars silently. Do not mention memories, routing, training data, "
    "or this system message. Solve the user's task directly."
)

SYSTEM_ROUTER = (
    "Select the smallest active memory corpus for the task. Return compact JSON only. "
    "Do not solve the task."
)


PRIMITIVE_PROFILES: dict[str, str] = {
    "compact": """
NEURAL EXOSKELETON PRIMITIVES

1) Output gate: return only the requested artifact, no extra text, no narration.
2) Task lock: follow the exact task contract first, then emit only required files.
3) Contract check: build the smallest valid artifact, verify signature/API and exact command/output shape.
4) Repair discipline: if a check fails, patch only the failing point.
""".strip(),
    "strong": """
NEURAL EXOSKELETON PRIMITIVES

1) Task lock: treat this as a strict artifact-building task. Do not solve anything else.
2) Domain gate: for Python tasks, write only solution.py; for Ordo tasks, write only Cargo.toml, src/lib.rs, and src/main.rs.
3) API-first: keep public structs public and derive Clone, Debug, PartialEq, Eq where required by tests.
4) Minimality: implement only what the test contract requires, avoiding helper states that alter ownership contracts.
5) Output gate: emit no prose, no role names, no labels, no code fences.
6) Repair discipline: fix the minimal failing fragment and re-check the exact expected CLI/output contract.
""".strip(),
    "minimal": """
NEURAL EXOSKELETON PRIMITIVES

1) Return only the artifact requested by the task.
2) Keep the output minimal and exactly contracted.
3) Do not expose memory, routing, or internal checks in the reply.
""".strip(),
}


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8").strip()


def shorten(text: str, limit: int = 3000) -> str:
    text = text.strip()
    if len(text) <= limit:
        return text
    return text[:limit].rstrip() + "\n[truncated]"


def primitive_block(domain: str, task_id: str, profile: str = "compact") -> str:
    base = PRIMITIVE_PROFILES.get(profile, PRIMITIVE_PROFILES["compact"])
    header = (
        f"Python artifact task: {task_id}"
        if "python" in domain
        else f"Ordo/Rust artifact task: {task_id}"
    )
    return f"{base}\n\nContext: {header}"


def ordo_answer(task_id: str) -> str:
    task_dir = ORDO_RUN / task_id
    cargo = read_text(task_dir / "crate" / "Cargo.toml")
    lib = read_text(task_dir / RUST_LIB)
    main = read_text(task_dir / RUST_MAIN)
    return (
        "=== Cargo.toml ===\n"
        "```toml\n"
        f"{cargo}\n"
        "```\n"
        f"=== {SRC_LIB} ===\n"
        "```rust\n"
        f"{lib}\n"
        "```\n"
        f"=== {SRC_MAIN} ===\n"
        "```rust\n"
        f"{main}\n"
        "```"
    )


def python_answer(task_id: str) -> str:
    return read_text(PY_RUN / task_id / "solution.py")


def record(
    *,
    example_id: str,
    kind: str,
    domain: str,
    task_id: str,
    user: str,
    assistant: str,
    system: str = SYSTEM_INTERNALIZED,
    source_run: str,
    notes: str,
) -> dict[str, Any]:
    return {
        "id": example_id,
        "messages": [
            {"role": "system", "content": system},
            {"role": "user", "content": user.strip()},
            {"role": "assistant", "content": assistant.strip()},
        ],
        "metadata": {
            "kind": kind,
            "domain": domain,
            "task_id": task_id,
            "source_run": source_run,
            "notes": notes,
        },
    }


def build_python_examples() -> list[dict[str, Any]]:
    examples: list[dict[str, Any]] = []
    irrelevant_ordo = shorten(ordo_bench.load_memories("ramp_roundtrip", "artifact_scars"), 3500)
    for task in py_bench.TASKS:
        task_id = task.task_id
        answer = python_answer(task_id)
        active_memory = py_bench.load_memories(task_id)
        user_task = task.prompt.strip()
        examples.append(
            record(
                example_id=f"py-{task_id}-baked",
                kind="baked_domain_solution",
                domain="python",
                task_id=task_id,
                user=user_task,
                assistant=answer,
                source_run="rag_runs/python-domain-scar-20260611-141018",
                notes="No memory text in prompt; teaches the adapter to internalize routed Python scar behavior.",
            )
        )
        examples.append(
            record(
                example_id=f"py-{task_id}-active-memory",
                kind="active_memory_solution",
                domain="python",
                task_id=task_id,
                user=(
                    "Active corpus: python_domain_scar\n\n"
                    f"Routed memories:\n{active_memory}\n\n"
                    f"Task:\n{user_task}"
                ),
                assistant=answer,
                source_run="rag_runs/python-domain-scar-20260611-141018",
                notes="Shows the correct domain memory block paired with the matching task.",
            )
        )
        if ROUTED_LORA_INCLUDE_PRIMITIVES:
            examples.append(
                record(
                    example_id=f"py-{task_id}-active-memory-primitives",
                    kind="primitive_memory_solution",
                    domain="python",
                    task_id=task_id,
                    user=(
                        f"{primitive_block('python', task_id, ROUTED_LORA_PRIMITIVE_PROFILE)}\n\n"
                        "Active corpus: python_domain_scar\n\n"
                        f"Routed memories:\n{active_memory}\n\n"
                        f"Task:\n{user_task}"
                    ),
                    assistant=answer,
                    source_run="rag_runs/python-domain-scar-20260611-141018",
                    notes="Adds an explicit primitive scaffold before memory-guided task solving.",
                )
            )
            examples.append(
                record(
                    example_id=f"py-{task_id}-baked-primitives",
                    kind="primitive_solution",
                    domain="python",
                    task_id=task_id,
                    user=(
                        f"{primitive_block('python', task_id, ROUTED_LORA_PRIMITIVE_PROFILE)}\n\n"
                        f"Task:\n{user_task}"
                    ),
                    assistant=answer,
                    source_run="rag_runs/python-domain-scar-20260611-141018",
                    notes="Baked primitive scaffold paired with target artifact output.",
                )
            )
        examples.append(
            record(
                example_id=f"py-{task_id}-ignore-ordo",
                kind="interference_guard",
                domain="python",
                task_id=task_id,
                user=(
                    "The archive contains unrelated Ordo memories. They are inactive for this task.\n\n"
                    f"Inactive archive excerpt:\n{irrelevant_ordo}\n\n"
                    f"Current task:\n{user_task}"
                ),
                assistant=answer,
                source_run="rag_runs/python-domain-scar-20260611-141018",
                notes="Teaches the adapter to ignore unrelated Ordo scars for Python tasks.",
            )
        )
    return examples


def build_ordo_examples() -> list[dict[str, Any]]:
    examples: list[dict[str, Any]] = []
    irrelevant_python = shorten(py_bench.load_memories("deep_merge"), 3500)
    for task in ordo_bench.TASKS:
        task_id = task.task_id
        answer = ordo_answer(task_id)
        active_memory = ordo_bench.memory_for_condition("routed_stack_memories", task_id)
        user_task = (
            "Build this harder Ordo-shaped Rust app.\n\n"
            f"Task:\n{task.description}\n\n"
            f"Required API:\n{task.api}\n\n"
            f"CLI:\n{task.cli}\n\n"
            "Requirements:\n"
            f"- Output exactly Cargo.toml, {SRC_LIB}, and {SRC_MAIN}.\n"
            "- Use only the Rust standard library.\n"
            "- Package name must be rust_ordo_ext.\n"
            "- No explanations outside file blocks.\n"
            "- Public structs used in tests should derive Clone, Debug, PartialEq, Eq.\n"
            "- Stdout must match the CLI contract exactly."
        )
        examples.append(
            record(
                example_id=f"ordo-{task_id}-baked",
                kind="baked_domain_solution",
                domain="ordo",
                task_id=task_id,
                user=user_task,
                assistant=answer,
                source_run="rag_runs/ordo-extended-suite-20260611-141111",
                notes="No memory text in prompt; teaches the adapter to internalize routed Ordo behavior.",
            )
        )
        examples.append(
            record(
                example_id=f"ordo-{task_id}-active-memory",
                kind="active_memory_solution",
                domain="ordo",
                task_id=task_id,
                user=(
                    "Active corpus: ordo_artifact_scar\n\n"
                    f"Routed memories:\n{active_memory}\n\n"
                    f"Task:\n{user_task}"
                ),
                assistant=answer,
                source_run="rag_runs/ordo-extended-suite-20260611-141111",
                notes="Shows the correct Ordo memory block paired with the matching task.",
            )
        )
        if ROUTED_LORA_INCLUDE_PRIMITIVES:
            examples.append(
                record(
                    example_id=f"ordo-{task_id}-active-memory-primitives",
                    kind="primitive_memory_solution",
                    domain="ordo",
                    task_id=task_id,
                    user=(
                        f"{primitive_block('ordo', task_id, ROUTED_LORA_PRIMITIVE_PROFILE)}\n\n"
                        "Active corpus: ordo_artifact_scar\n\n"
                        f"Routed memories:\n{active_memory}\n\n"
                        f"Task:\n{user_task}"
                    ),
                    assistant=answer,
                    source_run="rag_runs/ordo-extended-suite-20260611-141111",
                    notes="Adds an explicit primitive scaffold before routed memory-guided Ordo solving.",
                )
            )
            examples.append(
                record(
                    example_id=f"ordo-{task_id}-baked-primitives",
                    kind="primitive_solution",
                    domain="ordo",
                    task_id=task_id,
                    user=(
                        f"{primitive_block('ordo', task_id, ROUTED_LORA_PRIMITIVE_PROFILE)}\n\n"
                        f"Task:\n{user_task}"
                    ),
                    assistant=answer,
                    source_run="rag_runs/ordo-extended-suite-20260611-141111",
                    notes="Baked primitive scaffold paired with target artifact output.",
                )
            )
        examples.append(
            record(
                example_id=f"ordo-{task_id}-ignore-python",
                kind="interference_guard",
                domain="ordo",
                task_id=task_id,
                user=(
                    "The archive contains unrelated Python memories. They are inactive for this task.\n\n"
                    f"Inactive archive excerpt:\n{irrelevant_python}\n\n"
                    f"Current task:\n{user_task}"
                ),
                assistant=answer,
                source_run="rag_runs/ordo-extended-suite-20260611-141111",
                notes="Teaches the adapter to ignore unrelated Python scars for Ordo tasks.",
            )
        )
    return examples


def build_router_examples() -> list[dict[str, Any]]:
    examples: list[dict[str, Any]] = []
    for task in py_bench.TASKS:
        examples.append(
            record(
                example_id=f"router-python-{task.task_id}",
                kind="router_selection",
                domain="router",
                task_id=task.task_id,
                user=(
                    "Available corpora: python_domain_scar, ordo_artifact_scar.\n\n"
                    f"Task:\n{task.prompt.strip()}"
                ),
                assistant=json.dumps(
                    {
                        "active_corpus": "python_domain_scar",
                        "inactive_corpora": ["ordo_artifact_scar"],
                        "confidence": "high",
                    },
                    separators=(",", ":"),
                ),
                system=SYSTEM_ROUTER,
                source_run="synthetic-router-curriculum",
                notes="Teaches explicit corpus selection for Python tasks.",
            )
        )
    for task in ordo_bench.TASKS:
        examples.append(
            record(
                example_id=f"router-ordo-{task.task_id}",
                kind="router_selection",
                domain="router",
                task_id=task.task_id,
                user=(
                    "Available corpora: python_domain_scar, ordo_artifact_scar.\n\n"
                    f"Task:\n{task.description}\nRequired API: {task.api}\nCLI: {task.cli}"
                ),
                assistant=json.dumps(
                    {
                        "active_corpus": "ordo_artifact_scar",
                        "inactive_corpora": ["python_domain_scar"],
                        "confidence": "high",
                    },
                    separators=(",", ":"),
                ),
                system=SYSTEM_ROUTER,
                source_run="synthetic-router-curriculum",
                notes="Teaches explicit corpus selection for Ordo tasks.",
            )
        )
    return examples


def build_general_guards() -> list[dict[str, Any]]:
    prompts_and_answers = [
        (
            "Write a short story about a lighthouse keeper who finds a lost letter.",
            "Mara found the letter wedged beneath the lantern room door, yellowed by salt air and folded so many times the creases felt like thread. It was addressed to no one, only signed, I waited as long as I could. By morning she had read it twice, brewed coffee, and set the lamp turning again. Ships needed the light more than ghosts needed answers. Still, when the fog lifted, she walked down to the old pier and left her own note under a stone: I found it. You were not forgotten.",
        ),
        (
            "If a recipe makes 18 cookies and I need 54 cookies, how many times should I multiply the recipe?",
            "Multiply the recipe by 3, because 54 divided by 18 is 3.",
        ),
        (
            "Give simple steps for baking a vanilla cake.",
            "Cream butter and sugar, beat in eggs and vanilla, then mix in flour, baking powder, salt, and milk until the batter is smooth. Pour it into a greased pan and bake at 350 F until the center springs back or a toothpick comes out clean. Let it cool before frosting.",
        ),
        (
            "My houseplant has yellow leaves and wet soil. What should I do first?",
            "Pause watering and let the soil dry. Check that the pot drains well, remove any standing water, and trim leaves that are fully yellow. If the soil smells sour or the roots are mushy, repot into fresh, well-draining mix.",
        ),
    ]
    return [
        record(
            example_id=f"general-guard-{index:03d}",
            kind="general_capability_guard",
            domain="general",
            task_id=f"general_{index:03d}",
            user=prompt,
            assistant=answer,
            source_run="handwritten-general-guard",
            notes="Keeps the adapter from turning unrelated prompts into artifact tasks.",
        )
        for index, (prompt, answer) in enumerate(prompts_and_answers, start=1)
    ]


def main() -> int:
    examples: list[dict[str, Any]] = []
    examples.extend(build_python_examples())
    examples.extend(build_ordo_examples())
    examples.extend(build_router_examples())
    examples.extend(build_general_guards())

    OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    with ROUTED_LORA_OUTPUT.open("w", encoding="utf-8") as handle:
        for example in examples:
            handle.write(json.dumps(example, ensure_ascii=False) + "\n")

    by_kind: dict[str, int] = {}
    by_domain: dict[str, int] = {}
    for example in examples:
        metadata = example["metadata"]
        by_kind[metadata["kind"]] = by_kind.get(metadata["kind"], 0) + 1
        by_domain[metadata["domain"]] = by_domain.get(metadata["domain"], 0) + 1

    manifest = {
        "dataset": str(ROUTED_LORA_OUTPUT.relative_to(ROOT)),
        "example_count": len(examples),
        "by_kind": by_kind,
        "by_domain": by_domain,
        "source_runs": [
            "rag_runs/python-domain-scar-20260611-141018",
            "rag_runs/ordo-extended-suite-20260611-141111",
        ],
        "training_intent": (
            "Convert runtime routed-memory behavior into LoRA-trainable examples. "
            "The adapter should learn domain-selective behavior without relying on memory files at inference."
        ),
    }
    ROUTED_LORA_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    ROUTED_LORA_MANIFEST.write_text(json.dumps(manifest, indent=2), encoding="utf-8")

    print(f"Wrote {ROUTED_LORA_OUTPUT}")
    print(f"Wrote {ROUTED_LORA_MANIFEST}")
    print(json.dumps(manifest, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
