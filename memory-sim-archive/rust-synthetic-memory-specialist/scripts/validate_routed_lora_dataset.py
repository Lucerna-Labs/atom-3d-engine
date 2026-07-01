from __future__ import annotations

import json
from collections import Counter
from pathlib import Path
from typing import Any
import os


ROOT = Path(__file__).resolve().parents[1]
DATASET = Path(os.environ.get("ROUTED_LORA_DATASET", str(ROOT / "data" / "routed_memory_lora_sft.jsonl")))

REQUIRED_KINDS = {
    "baked_domain_solution",
    "active_memory_solution",
    "interference_guard",
    "router_selection",
    "general_capability_guard",
}

REQUIRED_DOMAINS = {"python", "ordo", "router", "general"}


def load_rows() -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    with DATASET.open("r", encoding="utf-8") as handle:
        for line_number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError as exc:
                raise SystemExit(f"Invalid JSON on line {line_number}: {exc}") from exc
    return rows


def validate_message_shape(row: dict[str, Any], index: int) -> list[str]:
    errors: list[str] = []
    messages = row.get("messages")
    if not isinstance(messages, list) or len(messages) < 2:
        return [f"row {index}: messages must be a list with at least system/user/assistant"]
    roles = [message.get("role") for message in messages if isinstance(message, dict)]
    if roles[:2] != ["system", "user"] or roles[-1] != "assistant":
        errors.append(f"row {index}: unexpected role order {roles}")
    for message_index, message in enumerate(messages):
        if not isinstance(message, dict):
            errors.append(f"row {index}: message {message_index} is not an object")
            continue
        content = message.get("content")
        if not isinstance(content, str) or not content.strip():
            errors.append(f"row {index}: message {message_index} has empty content")
    return errors


def main() -> int:
    if not DATASET.exists():
        raise SystemExit(f"Missing dataset: {DATASET}")
    rows = load_rows()
    errors: list[str] = []
    ids = [row.get("id") for row in rows]
    duplicates = [item for item, count in Counter(ids).items() if count > 1]
    if duplicates:
        errors.append(f"duplicate ids: {duplicates}")
    kinds = Counter(str(row.get("metadata", {}).get("kind", "")) for row in rows)
    domains = Counter(str(row.get("metadata", {}).get("domain", "")) for row in rows)
    missing_kinds = REQUIRED_KINDS - set(kinds)
    missing_domains = REQUIRED_DOMAINS - set(domains)
    if missing_kinds:
        errors.append(f"missing kinds: {sorted(missing_kinds)}")
    if missing_domains:
        errors.append(f"missing domains: {sorted(missing_domains)}")
    for index, row in enumerate(rows, start=1):
        if not isinstance(row.get("metadata"), dict):
            errors.append(f"row {index}: missing metadata object")
        errors.extend(validate_message_shape(row, index))
    if errors:
        print("Dataset validation failed:")
        for error in errors:
            print(f"- {error}")
        return 1
    print("Dataset validation passed")
    print(f"rows: {len(rows)}")
    print(f"kinds: {dict(kinds)}")
    print(f"domains: {dict(domains)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
