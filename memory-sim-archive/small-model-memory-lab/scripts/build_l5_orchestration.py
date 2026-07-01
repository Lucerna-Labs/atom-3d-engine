"""Build L5 orchestration-composition layer.

This is the layer the plan flagged as MISSING: the model needs to learn
not just primitive + adapter + provider individually, but the *composition*
of all four layers into a working Ordo capability. The 12 task families
from build_ordo_rust_lora_dataset.py each have a passed code build, so
we can produce per-family quadruplets:

  primitive   - the pure reusable logic
  adapter     - the thin runtime binding
  provider    - the capability surface with policy/review/events
  orchestrator- the route plan that uses the provider

For each family we generate 4 memories (one per role) plus 1 orchestrator
plan memory. We also generate a separate set of cross-cutting memories
covering lane discipline, review behavior, and event emission.

Output: data/l5_orchestration_v0_1/{train,validation}.jsonl + manifest.json + preview.json
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from ordo_common import (  # noqa: E402
    ORDO_PREFIX,
    RUST_PREFIX,
    memory_row,
    split_train_val,
    write_jsonl,
    write_manifest,
    write_preview,
)


# Each task family from TASK_TO_TITLE in build_ordo_rust_lora_dataset.py.
# Per family: (task_id, title, primitive_summary, adapter_summary, provider_summary, orchestrator_summary)
FAMILIES = [
    ("mini_runtime", "Ordo mini runtime",
     "The pure logic is a graph of nodes with capacity, cost, and ramp edges. Trace is the result of publish - thread_id, path, promoted, signals.",
     "The thin binding reads the rust lab's lib.rs and exposes the runtime to a CLI binary and a Tauri command. The CLI prints the exact line shape.",
     "The capability provider exposes mini_runtime.publish as the lane runtime.publish with input schema {from, topic, payload} and output schema Trace. Permission scope: local-only.",
     "The orchestrator discovers runtime.publish through the capability descriptor, plans a route through candidate senders, and emits orchestrator.route_selected and capability.completed events."),
    ("intersection_fabric", "Ordo intersection fabric",
     "Pure logic: take a set of streams, intersect on key, emit joined records. No I/O, no clock.",
     "Adapter: bind the pure function to a tokio task and a sync test harness. Each binding is a thin translate-and-call layer.",
     "Provider exposes fabric.join as lane fabric.join with input {streams: [Stream]} and output {joined: Vec<Record>}. Policy: read-only on streams, side-effect light.",
     "Orchestrator routes the join request through the fabric provider, recovers from a stream closing by emitting fallback_used, and logs orchestrator.plan_created."),
    ("backpressure_scheduler", "Ordo backpressure scheduler",
     "Pure logic: given candidates with cost and capacity, pick the first affordable with capacity; full nodes produce backpressure signals.",
     "Adapter: bind to the CLI scheduler binary. The CLI prints the exact Plan line shape.",
     "Provider exposes scheduler.schedule as lane scheduler.schedule. Permission: read graph state only. Review: required when over-budget candidates appear.",
     "Orchestrator composes scheduler.schedule with provider.fallback_used to recover when no candidate fits, and emits orchestrator.fallback_used."),
    ("signal_mesh", "Ordo signal mesh",
     "Pure logic: vertical(origin) returns origin plus every higher-layer node in insertion order. horizontal(origin) returns same-layer peers.",
     "Adapter: bind to the CLI signal-mesh binary. CLI prints signal=vertical/horizontal origin=... affected=...",
     "Provider exposes signal_mesh.vertical/horizontal as lane signal.mesh. Read-only. Side-effect light.",
     "Orchestrator uses signal.mesh to debug a plan failure: when a route fails, the orchestrator asks for vertical/horizontal impact before re-planning."),
    ("preload_planner", "Ordo preload planner",
     "Pure logic: BFS from start, depth limit, exclude start, preserve discovery order, use VecDeque.",
     "Adapter: bind to a CLI preload binary and a Tauri background pre-warm command.",
     "Provider exposes planner.preload as lane planner.preload. Read-only. Side-effect: cache write (in adapter, not primitive).",
     "Orchestrator composes preload with budget: only preload nodes that the active plan reaches."),
    ("ramp_roundtrip", "Ordo ramp roundtrip",
     "Pure logic: a ramp crossing sets promoted=true; backpressure still propagates if a downstream is full.",
     "Adapter: bind to the CLI ramp binary. CLI prints promoted and signals.",
     "Provider exposes ramp.traverse as lane ramp.traverse. Write-class capability (changes state: trace promoted flag).",
     "Orchestrator plans a route that uses a ramp and exposes the promoted outcome in the returned trace."),
    ("retry_fallback", "Ordo retry fallback",
     "Pure logic: false = failed, true = healthy. plan() returns first healthy candidate; records retry:<node> for failures; None if all fail.",
     "Adapter: bind to a CLI retry binary and a Tauri watchdog.",
     "Provider exposes retry.plan as lane retry.plan. Read-only. Review: required after N consecutive None returns.",
     "Orchestrator composes retry.plan with provider.fallback_used to switch engines after a node is unhealthy."),
    ("cost_ledger", "Ordo cost ledger",
     "Pure logic: append-only ledger keyed by task_id; total cost = sum across rows; queryable by task and window.",
     "Adapter: bind to SQLite via the shared Ordo storage layer; bind to a CLI read binary.",
     "Provider exposes ledger.append and ledger.query as lanes ledger.*. Write-class for append; read-class for query.",
     "Orchestrator attaches cost accounting to every route plan via a ledger.append in the same transaction as the work."),
    ("workflow_codec", "Ordo workflow codec",
     "Pure logic: encode a workflow as a stable byte string, decode back without loss. No version bumps without codec versioning.",
     "Adapter: bind to a CLI encode/decode binary and a Tauri IPC channel.",
     "Provider exposes codec.encode/codec.decode. Version field mandatory in the encoded form.",
     "Orchestrator uses codec to checkpoint long workflows; the checkpoint is storage-owned, the codec is primitive-owned."),
    ("node_registry", "Ordo node registry",
     "Pure logic: register nodes by id, look up by id, list by lane. Id uniqueness is the contract.",
     "Adapter: bind to shared Ordo storage and to a CLI registry binary.",
     "Provider exposes registry.register / registry.lookup / registry.list. Write-class for register, read-class for lookup.",
     "Orchestrator queries registry.list before planning a route, then composes the result with capability inventory."),
    ("fanout_join", "Ordo fanout/join helper",
     "Pure logic: given N branches, collect their results into a single joined result with timeout policy per branch.",
     "Adapter: bind to a tokio::join! and a CLI runner. The CLI prints the joined record.",
     "Provider exposes fanout.run as lane fanout.run. Timeout per branch is required schema.",
     "Orchestrator uses fanout.run to parallelize independent probes (e.g., inventory + health + cost) before re-planning."),
    ("audit_timeline", "Ordo audit timeline",
     "Pure logic: monotonic index from 1, events_for_thread returns owned clones in original order, CLI show prints one prefix only.",
     "Adapter: bind to a CLI show binary and a Tauri event log view.",
     "Provider exposes audit.record / audit.query. Write-class for record; read-class for query. Event logger name: audit.*",
     "Orchestrator attaches audit.record to every primitive invocation; the timeline is the operator-visible history."),
]


# Cross-cutting memories covering lane discipline, review, events.

CROSS_CUTTING = [
    ("lane-discipline", "FRAME",
     "I needed to expose a memory primitive to a Tauri command and a CLI binary. The live task was: 'let the same memory reader be called from two runtimes.' I reached for frame without naming it as a rule. The operation was to separate the primitive from the engine adapter. I defined the primitive first (no UI, no network), then a thin adapter per engine, then a provider that exposed both adapters as one capability lane. The orchestrator discovered the lane through capability descriptors, not by name. I ended by checking the useful path: primitive -> adapter -> provider -> orchestrator -> UXI. The lesson stayed because the duplication was prevented.",
     0.86),
    ("review-required", "CHECKSUM",
     "A destructive capability (delete node, publish to external) was added without review. The live task was: 'expose a delete node capability to the orchestrator.' I reached for checksum without naming it as a rule. The operation was to classify the capability safety class. The capability was destructive, so review was required. I added provider.review_required and the capability refused to run without an explicit operator approval. I ended by checking the useful path: classify safety class, set review behavior, emit review_required event. The lesson stayed because the failure was prevented at the boundary.",
     0.86),
    ("event-emission", "STATE_BUFFER",
     "A primitive ran but no events were emitted. Operators could not see the failure. The live task was: 'expose a primitive that retries on transient failure.' I reached for state_buffer without naming it as a rule. The operation was to add event-logger coverage for every transition. I added capability.invoked, capability.failed, adapter.selected, orchestrator.route_selected, and provider.permission_blocked. I ended by checking the useful path: event names follow the documented families, payloads include IDs and status, never private content. The lesson stayed because the operator log finally told the story.",
     0.86),
    ("primitive-engine-neutrality", "FRAME",
     "A primitive started calling the cloud provider directly. The live task was: 'add a routing primitive that picks the cheapest model.' I reached for frame without naming it as a rule. The operation was to keep the primitive engine-neutral. I refactored the primitive to take a price lookup function as a parameter, then the cloud call lives in the adapter, not the primitive. I ended by checking the useful path: primitive is a pure function, adapter holds the SDK call, provider holds the credentials. The lesson stayed because the primitive became testable in isolation.",
     0.86),
    ("adapter-thinness", "CHECKSUM",
     "An adapter started inventing policy and orchestration. The live task was: 'add a Tauri adapter for the search primitive.' I reached for checksum without naming it as a rule. The operation was to keep adapters thin. I moved the policy and orchestration back to the provider and the orchestrator. The adapter only translates and executes. I ended by checking the useful path: if the adapter is doing orchestration, the boundary is wrong. The lesson stayed because the layering stayed clean.",
     0.7),
    ("capability-discovery", "STATE_BUFFER",
     "An orchestrator hardcoded a provider name. The live task was: 'use the cheapest model in a plan.' I reached for state_buffer without naming it as a rule. The operation was to discover via capability descriptors, not by name. I rewrote the orchestrator to query capability inventory and pick by lane + descriptor, not by id. I ended by checking the useful path: orchestrator sees a descriptor, picks a provider, calls through the lane. The lesson stayed because the orchestrator became pluggable.",
     0.86),
    ("review-event-coverage", "PARITY",
     "A capability that talks to a paired device was exposed without review. The live task was: 'expose the device handshake capability to the orchestrator.' I reached for parity without naming it as a rule. The operation was to classify safety class and require review for device-class. I added provider.review_required and emitted handshake.* events for every transition. I ended by checking the useful path: device-class capabilities always carry review + event coverage. The lesson stayed because the operator log finally reflected the device boundary.",
     0.7),
    ("lane-prefix", "FRAME",
     "A new capability was registered under a generic prefix. The live task was: 'expose a research-source-scoring capability.' I reached for frame without naming it as a rule. The operation was to pick the lane prefix that reveals ownership. I used research.* because the capability is owned by the research domain. I avoided the plugin.* prefix because the capability is core, not a plugin. I ended by checking the useful path: prefix matches the existing domain or marks a new owner boundary. The lesson stayed because the inventory stayed readable.",
     0.7),
    ("orchestrator-fallback", "CHECKSUM",
     "A planned route hit a missing capability. The orchestrator crashed. The live task was: 'handle a route that needs a capability the runtime does not have.' I reached for checksum without naming it as a rule. The operation was to support fallback when a capability is missing. The orchestrator now emits orchestrator.fallback_used and offers the next-best route or a missing-capability message. I ended by checking the useful path: capability.inventory -> plan -> route -> fallback -> message. The lesson stayed because the runtime became resilient.",
     0.7),
    ("provider-gate", "FRAME",
     "A primitive was called directly from a UI button, bypassing the provider gate. The live task was: 'let the operator trigger a search from a button.' I reached for frame without naming it as a rule. The operation was to require every capability to pass through a provider. I refactored the UI to call the provider, not the primitive. The provider enforced permissions and review. I ended by checking the useful path: UI -> provider -> adapter -> primitive. The lesson stayed because the gate stopped being bypassed.",
     0.86),
]


def build_rows() -> list[dict]:
    rows: list[dict] = []
    counter = 0

    for task_id, title, prim, adp, prv, orch in FAMILIES:
        # Primitive memory
        counter += 1
        prim_text = (
            f"I remember designing the primitive layer of {title}. The live task was: "
            f"define the pure reusable logic for {task_id} without engine, runtime, or "
            f"UI dependencies. I reached for frame without naming it as a rule. The "
            f"operation was to keep the primitive engine-neutral, side-effect-light, and "
            f"testable in isolation. I wrote: {prim} I ended by checking the useful "
            f"path: primitive first, then adapter, then provider, then orchestrator. The "
            f"lesson stayed because the layering stayed clean."
        )
        rows.append(memory_row(
            row_id=f"L5-PRIM-{counter:04d}",
            bucket="l5-orchestration",
            family=f"{task_id}:primitive",
            primitive="FRAME",
            valence=0.86,
            repeat_weight=2,
            text=prim_text,
        ))

        # Adapter memory
        counter += 1
        adp_text = (
            f"I remember building the adapter layer of {title}. The live task was: "
            f"bind the {task_id} primitive to a concrete runtime without inventing "
            f"policy or orchestration. I reached for checksum without naming it as a "
            f"rule. The operation was to keep the adapter thin - it translates and "
            f"executes, nothing else. I wrote: {adp} I ended by checking the useful "
            f"path: if the adapter is doing orchestration, the boundary is wrong. The "
            f"lesson stayed because the runtime binding stayed testable."
        )
        rows.append(memory_row(
            row_id=f"L5-ADP-{counter:04d}",
            bucket="l5-orchestration",
            family=f"{task_id}:adapter",
            primitive="CHECKSUM",
            valence=0.7,
            repeat_weight=2,
            text=adp_text,
        ))

        # Provider memory
        counter += 1
        prv_text = (
            f"I remember exposing the capability for {title} through a provider. "
            f"The live task was: advertise {task_id} to the orchestrator with "
            f"permission, review, and event-logger coverage. I reached for "
            f"state_buffer without naming it as a rule. The operation was to "
            f"classify the safety class and set review behavior. I wrote: {prv} "
            f"I ended by checking the useful path: provider always classifies "
            f"safety class, sets review behavior, and emits documented event "
            f"names. The lesson stayed because the orchestrator could discover "
            f"the capability safely."
        )
        rows.append(memory_row(
            row_id=f"L5-PRV-{counter:04d}",
            bucket="l5-orchestration",
            family=f"{task_id}:provider",
            primitive="STATE_BUFFER",
            valence=0.86,
            repeat_weight=2,
            text=prv_text,
        ))

        # Orchestrator memory
        counter += 1
        orch_text = (
            f"I remember wiring the orchestrator to use {title}. The live task "
            f"was: route work to the {task_id} provider through capability "
            f"descriptors, with fallback when the capability is missing. I "
            f"reached for parity without naming it as a rule. The operation "
            f"was to discover via descriptors, not by id. I wrote: {orch} "
            f"I ended by checking the useful path: capability.inventory -> "
            f"plan -> route -> fallback -> message. The lesson stayed because "
            f"the runtime stayed resilient."
        )
        rows.append(memory_row(
            row_id=f"L5-ORCH-{counter:04d}",
            bucket="l5-orchestration",
            family=f"{task_id}:orchestrator",
            primitive="PARITY",
            valence=0.86,
            repeat_weight=2,
            text=orch_text,
        ))

        # SFT example for the orchestrator-plan shape, in Ordo voice
        counter += 1
        sft_user = (
            f"{ORDO_PREFIX}\n\n"
            f"Ordo architecture task: write the orchestrator plan for {title}.\n\n"
            f"Use the four-layer build pattern: primitive -> adapter -> provider -> orchestrator."
        )
        sft_assistant = (
            f"I remember the exact {title} plan.\n\n"
            f"Primitive: {prim}\n\n"
            f"Adapter: {adp}\n\n"
            f"Provider: {prv}\n\n"
            f"Orchestrator: {orch}\n\n"
            f"The lesson stayed because the layering stayed clean."
        )
        rows.append({
            "id": f"L5-ORCH-SFT-{counter:04d}",
            "kind": "l5_orchestration_sft",
            "task": task_id,
            "text": f"### User\n{sft_user}\n\n### Assistant\n{sft_assistant}",
        })

    # Cross-cutting memories
    for family, primitive, text, valence in CROSS_CUTTING:
        counter += 1
        rows.append(memory_row(
            row_id=f"L5-CROSS-{counter:04d}",
            bucket="l5-orchestration",
            family=family,
            primitive=primitive,
            valence=valence,
            repeat_weight=2,
            text=text,
        ))

    return rows


def main() -> None:
    out_dir = ROOT / "data" / "l5_orchestration_v0_1"
    rows = build_rows()
    train, val = split_train_val(rows, val_ratio=0.1, seed=23)
    n_train = write_jsonl(out_dir / "train.jsonl", train)
    n_val = write_jsonl(out_dir / "validation.jsonl", val)
    write_preview(out_dir / "preview.json", train)
    manifest = {
        "dataset": "l5_orchestration_v0_1",
        "source": "synthesized from TASK_TO_TITLE families + cross-cutting capability architecture",
        "row_count": len(rows),
        "by_family_kind": {
            "primitive_memories": len(FAMILIES),
            "adapter_memories": len(FAMILIES),
            "provider_memories": len(FAMILIES),
            "orchestrator_memories": len(FAMILIES),
            "orchestrator_sft": len(FAMILIES),
            "cross_cutting": len(CROSS_CUTTING),
        },
        "train_count": n_train,
        "validation_count": n_val,
        "notes": [
            "Generated without external downloads.",
            "Each of the 12 Ordo task families gets a primitive/adapter/provider/orchestrator quadruplet plus a SFT plan example.",
            "Cross-cutting memories cover lane discipline, review, event emission, discovery, fallback, and provider gate.",
        ],
    }
    write_manifest(out_dir / "manifest.json", manifest)
    print(json.dumps(manifest, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()