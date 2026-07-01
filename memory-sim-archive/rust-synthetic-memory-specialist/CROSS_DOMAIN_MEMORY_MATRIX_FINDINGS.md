# Cross-Domain Memory Matrix Findings

Date: 2026-06-11

## Purpose

This is the broader local test we actually needed.

The question is not whether more memory is better. We already know it is not.

The question is:

```text
Can different domain scar-memory corpora coexist without interfering with each other?
```

This run tests that across full local suites instead of one or two cherry-picked tasks.

## Model

```text
qwen35-4b-q4km-chat
```

## Suites

Python suite:

```text
5 tasks
- csv_rollup
- sliding_window
- flatten_json
- topo_batches
- deep_merge
```

Ordo suite:

```text
12 tasks
- mini_runtime
- intersection_fabric
- backpressure_scheduler
- signal_mesh
- preload_planner
- ramp_roundtrip
- retry_fallback
- cost_ledger
- workflow_codec
- node_registry
- fanout_join
- audit_timeline
```

Total:

```text
17 executable domain tasks per comparable condition
```

## Conditions

Python conditions:

```text
no_memory
python_scar_memories
ordo_only_memories
python_plus_ordo_memories
routed_stack_memories
```

Ordo conditions:

```text
no_memory
ordo_artifact_scars
python_only_memories
ordo_artifact_plus_python
routed_stack_memories
```

Meaning:

```text
own-domain memories:
- Python task + Python memories
- Ordo task + Ordo memories

foreign-only memories:
- Python task + Ordo memories
- Ordo task + Python memories

wholesale combined:
- Python task + Python and Ordo memories
- Ordo task + Ordo and Python memories

routed stack:
- Both corpora exist, but only the selected active corpus is injected
```

## Run Paths

Python:

```text
rag_runs/python-domain-scar-20260611-200915
```

Ordo:

```text
rag_runs/ordo-extended-suite-20260611-201142
```

## Python Results

```text
no_memory:                 4/5, builder 2/5
python_scar_memories:      5/5, builder 5/5
ordo_only_memories:        4/5, builder 4/5
python_plus_ordo_memories: 5/5, builder 5/5
routed_stack_memories:     4/5, builder 4/5
```

Task-level failures:

```text
no_memory:
- deep_merge failed

ordo_only_memories:
- flatten_json failed

routed_stack_memories:
- deep_merge failed

python_scar_memories:
- no failures

python_plus_ordo_memories:
- no failures
```

Python interpretation:

```text
The Python corpus works across the full Python suite in this run.
The Ordo corpus alone does not hijack Python completely, but it does lose flatten_json.
Wholesale Python+Ordo did not interfere on Python in this run; it matched Python-only at 5/5.
Routed-current underperformed Python-only in this run because deep_merge failed, which means the routed subset for Python still needs calibration.
```

## Ordo Results

```text
no_memory:                  0/12, builder 0/12
ordo_artifact_scars:        12/12, builder 11/12
python_only_memories:       0/12, builder 0/12
ordo_artifact_plus_python:  11/12, builder 11/12
routed_stack_memories:      12/12, builder 11/12
```

Task-level failures:

```text
no_memory:
- all 12 failed

python_only_memories:
- all 12 failed

ordo_artifact_plus_python:
- ramp_roundtrip failed

ordo_artifact_scars:
- no failures

routed_stack_memories:
- no failures
```

Ordo interpretation:

```text
The Ordo corpus is doing nearly all the work on Ordo.
Python memories alone provide no Ordo capability.
Wholesale Ordo+Python is mostly compatible but causes one brittle-task regression: ramp_roundtrip.
Routed stack removes that Ordo-side interference and restores 12/12.
```

## Comparable Aggregate View

No-memory baseline:

```text
Python no_memory: 4/5
Ordo no_memory:   0/12
Total:            4/17
```

Own-domain memories:

```text
Python own-domain: 5/5
Ordo own-domain:   12/12
Total:             17/17
```

Foreign-only memories:

```text
Python with Ordo-only: 4/5
Ordo with Python-only: 0/12
Total:                 4/17
```

Wholesale combined:

```text
Python with Python+Ordo: 5/5
Ordo with Ordo+Python:   11/12
Total:                   16/17
```

Routed stack:

```text
Python routed: 4/5
Ordo routed:   12/12
Total:         16/17
```

## What This Says About Coexistence

The corpora can coexist, but not perfectly by default.

The important pattern:

```text
Own-domain memories:      17/17
Wholesale combined:       16/17
Routed stack:             16/17
Foreign-only memories:     4/17
No memory:                 4/17
```

That means:

```text
1. The domain corpora are strongly domain-specific.
   Foreign-only memories do not transfer enough to solve the other domain.

2. Wholesale stacking is mostly safe, but not lossless.
   It costs one brittle Ordo task: ramp_roundtrip.

3. Routing fixes the Ordo-side interference.
   Ordo goes back to 12/12 under routed stack.

4. Routing is not automatically better unless the selected routed subset is calibrated.
   Python routed stack lost deep_merge in this run even though Python-only and Python+Ordo passed it.
```

## Updated Local Lesson

The answer is not:

```text
More memory is bad.
```

The better answer is:

```text
Different domain memories can coexist with limited interference,
but the selected subset matters and brittle tasks expose the interference first.
```

Current evidence:

```text
Full cross-domain matrix:
- 17 executable tasks
- 5 condition families
- own-domain corpora reached 17/17
- combined/routed corpora reached 16/17
```

## Next Local Tests

We still need more than one run because some tasks show stochasticity, especially Python `deep_merge`.

Next local matrix should be:

```text
1. Repeat this full 17-task matrix at least 3 times.
2. Track pass rate and builder-stage pass rate separately.
3. Add a third domain corpus, probably Python plus Ordo plus general/Rust-coder, once it has its own executable suite.
4. Tune the routed Python subset until routed Python matches Python-only at 5/5 reliably.
5. Keep `ramp_roundtrip`, `deep_merge`, and `flatten_json` as sentinel interference tasks.
```

## Current Status

This is now a real cross-domain coexistence test, not a 1-2 task spot check.

The current best local read:

```text
Yes, separate domain scar corpora can mostly coexist.
No, wholesale stacking is not fully safe.
Yes, routing can remove some interference.
No, routing is not magic; the routed subset still has to be calibrated.
```
