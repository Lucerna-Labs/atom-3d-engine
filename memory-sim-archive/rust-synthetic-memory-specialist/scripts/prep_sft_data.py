"""Assemble the Ordo SFT dataset (bare spec -> solution), accumulating across batches.

Maintains data/ordo_gen_apps.json (accumulator of generated apps). Ingests any
workflow output JSONs listed in GEN_OUTPUTS (comma-separated paths), upserting green
non-held-out apps by task_id. Then writes data/ordo_sft.jsonl from accumulator + the
12 dev apps' verified artifacts. The 5 held-out apps are always EXCLUDED.
"""
from __future__ import annotations
import os, sys, json
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
os.environ.setdefault("ORDO_EXT_MODEL", "x")
import run_ordo_extended_suite_benchmark as base  # noqa: E402
from run_ordo_extended_suite_benchmark import Task  # noqa: E402

ROOT = base.ROOT
ACC = ROOT / "data" / "ordo_gen_apps.json"
HELDOUT = {"deadletter_sink", "hop_budget", "fanin_merge", "priority_lane", "checkpoint_replay"}


def completion(lib: str, main: str) -> str:
    return (
        "=== Cargo.toml ===\n```toml\n[package]\nname = \"rust_ordo_ext\"\nversion = \"0.1.0\"\nedition = \"2021\"\n```\n"
        "=== src/lib.rs ===\n```rust\n" + lib.strip() + "\n```\n"
        "=== src/main.rs ===\n```rust\n" + main.strip() + "\n```"
    )


# load accumulator (dict by task_id)
acc: dict[str, dict] = {}
if ACC.exists():
    for a in json.loads(ACC.read_text(encoding="utf-8")):
        acc[a["task_id"]] = a

# ingest any new workflow outputs
for path in [p.strip() for p in os.environ.get("GEN_OUTPUTS", "").split(",") if p.strip()]:
    try:
        apps = json.load(open(path, encoding="utf-8"))["result"]["apps"]
    except Exception as e:  # noqa: BLE001
        print(f"  skip {path}: {e!r}")
        continue
    for a in apps:
        if not a.get("cargo_passed") or a["task_id"] in HELDOUT:
            continue
        if a.get("lib_rs", "").strip() and a.get("main_rs", "").strip():
            acc[a["task_id"]] = {k: a[k] for k in ("task_id", "description", "api", "cli", "lib_rs", "main_rs")}

ACC.write_text(json.dumps(list(acc.values()), indent=1), encoding="utf-8")

# build SFT pairs
pairs = []
for a in acc.values():
    t = Task(a["task_id"], a["description"], a["api"], a["cli"], "")
    pairs.append({"task_id": a["task_id"], "src": "gen",
                  "prompt": base.build_prompt(t, ""), "completion": completion(a["lib_rs"], a["main_rs"])})

art_dirs = [
    ROOT / "rag_runs" / "ordo-extended-suite-20260608-185020" / "ordo_memory",
    ROOT / "rag_runs" / "ordo-extended-suite-20260608-185527" / "ordo_memory",
]
for t in base.TASKS:
    if t.task_id in HELDOUT or t.task_id in acc:
        continue
    for ad in art_dirs:
        lib, main = ad / t.task_id / "lib.rs", ad / t.task_id / "main.rs"
        if lib.exists() and main.exists():
            L, M = lib.read_text(encoding="utf-8"), main.read_text(encoding="utf-8")
            if L.strip() and M.strip():
                pairs.append({"task_id": t.task_id, "src": "dev",
                              "prompt": base.build_prompt(t, ""), "completion": completion(L, M)})
                break

out = ROOT / "data" / "ordo_sft.jsonl"
with open(out, "w", encoding="utf-8") as f:
    for p in pairs:
        f.write(json.dumps(p) + "\n")
ng = sum(1 for p in pairs if p["src"] == "gen")
nd = sum(1 for p in pairs if p["src"] == "dev")
print(f"accumulator={len(acc)} apps | wrote {len(pairs)} SFT pairs ({ng} gen + {nd} dev) -> {out}")
