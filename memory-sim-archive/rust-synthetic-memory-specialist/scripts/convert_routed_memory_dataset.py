"""Convert routed-memory chat-format rows to train_ordo_lora prompt/completion JSONL.

This keeps the LoRA training loop (SFT-style prompt/completion) compatible with
chat-built dataset rows used by the routed-LoRA corpus builder.
"""
from __future__ import annotations

import json
import os
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = Path(
    os.environ.get(
        "ROUTED_LORA_CONVERT_SOURCE",
        str(ROOT / "data" / "routed_memory_lora_sft.jsonl"),
    )
)
TARGET = Path(
    os.environ.get(
        "ROUTED_LORA_CONVERT_TARGET",
        str(ROOT / "data" / "routed_memory_lora_sft_prompt_completion.jsonl"),
    )
)
KIND_FILTER = {
    item.strip()
    for item in os.environ.get("ROUTED_LORA_CONVERT_KINDS", "").split(",")
    if item.strip()
}


def join_chat_messages(messages: list[dict[str, str]]) -> tuple[str, str]:
    prompt_parts: list[str] = []
    assistant_parts: list[str] = []
    for message in messages:
        role = message.get("role", "").strip().lower()
        content = str(message.get("content", "")).strip()
        if role in {"system", "user"}:
            prompt_parts.append(f"{role}: {content}")
        elif role == "assistant":
            assistant_parts.append(content)
    return "\n\n".join(prompt_parts), "\n\n".join(assistant_parts)


def main() -> int:
    if not SOURCE.exists():
        raise SystemExit(f"Missing source file: {SOURCE}")

    converted: list[dict[str, str]] = []
    with SOURCE.open("r", encoding="utf-8") as handle:
        for line_number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            try:
                row = json.loads(line)
            except json.JSONDecodeError as exc:
                raise SystemExit(f"Invalid JSON on line {line_number}: {exc}") from exc

            if KIND_FILTER:
                kind = str(row.get("metadata", {}).get("kind", ""))
                if kind not in KIND_FILTER:
                    continue

            if "prompt" in row and "completion" in row:
                prompt = str(row["prompt"])
                completion = str(row["completion"])
            else:
                messages = row.get("messages", [])
                if not isinstance(messages, list):
                    raise SystemExit(f"Row {line_number} has no prompt/completion or messages list")
                prompt, completion = join_chat_messages(messages)
                if not prompt:
                    raise SystemExit(f"Row {line_number} produced empty prompt")
                if not completion:
                    raise SystemExit(f"Row {line_number} produced empty completion")

            converted.append(
                {
                    "prompt": prompt,
                    "completion": completion,
                    "metadata": row.get("metadata", {}),
                }
            )

    TARGET.parent.mkdir(parents=True, exist_ok=True)
    with TARGET.open("w", encoding="utf-8") as handle:
        for row in converted:
            handle.write(json.dumps(row, ensure_ascii=False) + "\n")

    kinds = {}
    for row in converted:
        kind = str(row.get("metadata", {}).get("kind", ""))
        kinds[kind] = kinds.get(kind, 0) + 1

    print(f"Converted {len(converted)} rows")
    print(f"Source: {SOURCE}")
    print(f"Target: {TARGET}")
    print(f"Kinds: {kinds}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
