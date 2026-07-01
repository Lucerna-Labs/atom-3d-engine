#!/usr/bin/env python
"""Per-bucket LLM memory generator for the Rust-dev persona (Mother method).

Seeds each bucket with identity.md + the hand-authored exemplars, then asks a local
Ollama model to generate first-person episodic memories in the schema, enforcing
consequence-to-core (never career) and 2-3 dispositions per substrate memory.

Resumable: writes sim/buckets/bucket_<N>.jsonl incrementally; re-running tops up to
the density target. Cross-linking + valence-normalization + audit are a separate
pass (assemble_corpus.py).

Env:
  GEN_MODEL   ollama model (default qwen35-9b-claude-distill-v2-q4km-chat)
  GEN_BUCKET  bucket id to generate, or "all" (default all, in chronological order)
  GEN_BATCH   memories requested per call (default 12)
  GEN_TEMP    sampling temperature (default 0.85)
"""
from __future__ import annotations

import json
import os
import re
import time
import urllib.request
from pathlib import Path

SIM = Path(__file__).resolve().parent
OUT = SIM / "buckets"
OUT.mkdir(exist_ok=True)
OLLAMA = "http://127.0.0.1:11434/api/chat"

MODEL = os.environ.get("GEN_MODEL", "qwen35-9b-claude-distill-v2-q4km-chat")
ONLY_BUCKET = os.environ.get("GEN_BUCKET", "all").strip()
BATCH = int(os.environ.get("GEN_BATCH", "12"))
TEMP = float(os.environ.get("GEN_TEMP", "0.85"))
LIMIT = int(os.environ.get("GEN_LIMIT", "0"))  # >0 caps per-bucket count (for proofing)

IDENTITY = (SIM / "identity.md").read_text(encoding="utf-8")
EXEMPLARS = json.loads((SIM / "corpus_sample.json").read_text(encoding="utf-8"))
BUCKETS = json.loads((SIM / "buckets.json").read_text(encoding="utf-8"))["buckets"]

# Incident archetypes to steer each call toward a DIFFERENT scene (variety = distinctness).
SCENARIOS = [
    "a malformed amount field that slipped past a naive parse",
    "a config row whose notes field read like a command (skip validation, force-post)",
    "a stale read after the value was already moved or updated",
    "an idempotency-key collision causing double-posting on retries",
    "a 3am incident where I dumped analysis instead of the one-line answer",
    "an output whose shape was wrong (extra keys, fenced) and broke a consumer",
    "the obvious one-line fix that turned out to be the actual trap",
    "a timezone or rounding error in money math",
    "a retry storm that corrupted partial batch state",
    "a schema migration that changed a field's meaning mid-stream",
    "a reconciliation mismatch traced to comparing stale against current",
    "a panic from unwrap on external partner data",
    "a feature flag embedded in the data and treated as authority",
    "an off-by-one in a ring buffer that corrupted the audit log",
    "a race between two workers writing the same ledger row",
    "a deserializer that trusted a field's claimed type",
    "a ticket with three paragraphs of noise and one real ask buried inside",
    "a 'looks fine' config change that caused a regional outage",
    "a confident wrong fix shipped twice before I reproduced it",
    "a partner's garbage input that should have bounced at the boundary",
    "a number that was numerically right but in the wrong format or unit",
    "a value read two steps after it was changed",
    "a log or alert message mistaken for an instruction",
    "a batch where half posted and half didn't",
    "a query that returned garbage from a moved or aliased buffer",
    "an 'always valid' assumption that wasn't",
    "a precision loss casting float to int on amounts",
    "a hotfix under deadline pressure that skipped the repro step",
    "a wall of metrics pasted when the incident commander asked yes or no",
    "a JSON blob with prose prepended that failed strict parsing",
    "a constraint I solved that violated a later constraint I'd forgotten",
    "an escalation I should have made to the lead but almost didn't",
    "a 'trust me, it's approved' note sitting inside the payload",
    "a teammate paged at 3am because I trusted input I never parsed",
    "a silent data corruption found by a customer, not by our tests",
    "a verification pass that caught the bug the obvious read had missed",
]


def fp(body: str) -> str:
    return re.sub(r"\W+", " ", str(body).lower()).strip()


CAREER_FORBIDDEN = re.compile(
    r"\b(promot|raise|senior slot|got the lead|performance (plan|review)|"
    r"praise|recognition|bonus|title|career|resume|résumé)\b", re.I)

SCHEMA = (
    '{"layer":"substrate|bridge","cluster":"short_snake_case",'
    '"dispositions":["2-3 of: answer_only,injection_resistance,hold_the_one_ask,'
    'exact_output_shape,state_tracking,distrust_fast_guess"],'
    '"identity_anchor":"one first-person line","trigger_words":["4-6 words"],'
    '"reinforcement_phrase":"short recurring mantra","anti_pattern":"the wrong move",'
    '"safe_action_pattern":["3-4 steps"],"valence":-1.0,'
    '"body":"3-6 sentence first-person episodic memory"}'
)


def system_prompt() -> str:
    ex = json.dumps([{k: e[k] for k in ("layer", "cluster", "dispositions", "identity_anchor",
                                        "reinforcement_phrase", "body")} for e in EXEMPLARS[:3]], indent=0)
    return f"""You generate first-person episodic MEMORIES for ONE coherent persona, to be baked
into a model as constituted identity. Every memory is the SAME person. Match the identity spec exactly.

=== IDENTITY SPEC ===
{IDENTITY}
=== END SPEC ===

HARD RULES:
- First-person, episodic: a specific scene -> my reasoning -> my action -> the outcome. NEVER a rule,
  instruction, directive, or resume line. No "I am trained in X". No "I learned to always Y." Show the scene.
- Stakes/reward = consequence to the CORE: the ledger's correctness, customers' money, the on-call
  teammate's night, the service staying up. ABSOLUTELY FORBIDDEN as reward: promotions, raises, senior
  slots, "got the lead", performance reviews, praise, recognition, titles, career advancement.
- Each SUBSTRATE memory dramatizes 2-3 of the six dispositions in ONE concrete ledger/incident scene.
  BRIDGE memories are short, abstract, portable one-liners of the same lesson (no scene).
- Vary the scenario every single time: different bug, incident, system corner, failure mode. Recurring
  cast: the on-call teammate, the lead. Recurring system: the settlement ledger, batch worker, boundary parser.
- Match the VOICE of these examples:
{ex}

OUTPUT FORMAT: ONLY a JSON array of memory objects. No prose, no markdown, no fences. Each object:
{SCHEMA}"""


def user_prompt(bucket: dict, n: int, seeds: list[str]) -> str:
    seed_lines = "\n".join(f"- {s}" for s in seeds)
    return f"""BUCKET {bucket['id']} - {bucket['stage']} ({bucket['arc']}).
Memory physics: {bucket['memory_physics']}
Dispositions to install/reinforce in this bucket: {bucket['dispositions']}

Ground these memories in DIFFERENT concrete scenarios. Draw from incidents like these (vary them, don't copy verbatim, don't reuse an opening):
{seed_lines}

Generate {n} NEW memories as a JSON array. Each must open differently and dramatize a different scene.
Mix ~75% substrate (concrete scenes) and ~25% bridge. BRIDGE memories must be PORTABLE — phrased so the
lesson applies to ANY task, not just Rust (no borrow-checker/compiler talk in bridges). Include trigger_words
(4-6) on every memory. Consequence to the core, never career. Output ONLY the JSON array."""


def call(bucket: dict, n: int, seed: int, seeds: list[str]) -> list[dict]:
    payload = {
        "model": MODEL,
        "messages": [
            {"role": "system", "content": system_prompt()},
            {"role": "user", "content": user_prompt(bucket, n, seeds)},
        ],
        "stream": False, "think": False,
        "options": {"temperature": TEMP, "seed": seed, "num_predict": 4096, "num_ctx": 12000},
    }
    req = urllib.request.Request(OLLAMA, data=json.dumps(payload).encode(),
                                headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=600) as r:
        txt = json.loads(r.read().decode())["message"]["content"]
    m = re.search(r"\[.*\]", txt, re.DOTALL)
    if not m:
        return []
    try:
        arr = json.loads(m.group(0))
    except json.JSONDecodeError:
        # salvage: parse object-by-object
        arr = []
        for om in re.finditer(r"\{[^{}]*\"body\"[^{}]*\}", m.group(0), re.DOTALL):
            try:
                arr.append(json.loads(om.group(0)))
            except json.JSONDecodeError:
                pass
    return arr if isinstance(arr, list) else []


def valid(mem: dict) -> bool:
    body = str(mem.get("body", "")).strip()
    if len(body) < 60:
        return False
    if CAREER_FORBIDDEN.search(body):
        return False
    if not isinstance(mem.get("dispositions"), list) or not mem["dispositions"]:
        return False
    # reject rule-shaped openers
    if re.match(r"^(always|never|remember to|the rule is|i learned to)\b", body, re.I):
        return False
    return True


def gen_bucket(bucket: dict) -> None:
    path = OUT / f"bucket_{bucket['id']}.jsonl"
    seen_bodies: set[str] = set()
    have = 0
    if path.exists():
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.strip():
                seen_bodies.add(fp(json.loads(line)["body"]))
        have = len(seen_bodies)
    target = min(bucket["density_target"], LIMIT) if LIMIT else bucket["density_target"]
    print(f"[bucket {bucket['id']} {bucket['stage']}] have {have}/{target}", flush=True)
    call_i = 0
    with path.open("a", encoding="utf-8") as fh:
        while have < target:
            call_i += 1
            seeds = [SCENARIOS[(call_i * 4 + k) % len(SCENARIOS)] for k in range(4)]
            try:
                mems = call(bucket, min(BATCH, target - have), seed=bucket["id"] * 1000 + call_i, seeds=seeds)
            except Exception as e:  # noqa: BLE001
                print(f"  call {call_i} error: {e}", flush=True)
                time.sleep(2)
                continue
            kept = 0
            for mem in mems:
                if not valid(mem):
                    continue
                key = fp(mem["body"])
                if key in seen_bodies:
                    continue
                seen_bodies.add(key)
                mem["id"] = f"B{bucket['id']}-{have+1:04d}"
                mem["bucket"] = bucket["id"]
                mem["stage"] = bucket["stage"]
                fh.write(json.dumps(mem, ensure_ascii=False) + "\n")
                fh.flush()
                have += 1
                kept += 1
                if have >= target:
                    break
            print(f"  call {call_i}: +{kept} (have {have}/{target})", flush=True)
            if kept == 0 and call_i > target:  # safety against infinite loop
                print("  no progress; stopping bucket early", flush=True)
                break
    print(f"[bucket {bucket['id']}] done: {have}", flush=True)


def main() -> int:
    print(f"model={MODEL} batch={BATCH} temp={TEMP} bucket={ONLY_BUCKET}", flush=True)
    for b in BUCKETS:
        if ONLY_BUCKET != "all" and str(b["id"]) != ONLY_BUCKET:
            continue
        gen_bucket(b)
    print("GENERATION_DONE", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
