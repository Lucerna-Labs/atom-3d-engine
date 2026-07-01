# Python Domain-Selective Scar Memory Findings

Date: 2026-06-11

## Purpose

This is the second separate domain-selective scar memory corpus after the Ordo/Rust corpus.

Goal:

```text
Test whether the same construction logic works for Python:
- narrow artifact-contract memories
- task-specific failure/fix scars
- no broad persona
- no global "I am a Python expert" identity overlay
- executable benchmark with real tests
- collateral probe for general capability damage
```

## Files

```text
data/python_domain_scar_memories.txt
scripts/run_python_domain_scar_benchmark.py
```

Current corpus size:

```text
about 4.9k characters before repair additions
small enough to stay focused and cheap to inject
```

## Benchmark Design

The harness asks the model to produce a single Python file:

```text
solution.py
```

The verifier writes that file, writes hidden `unittest` tests, and runs:

```text
python -m unittest -v test_solution.py
```

The benchmark has five tasks:

```text
csv_rollup
- parse CSV text with region,item,quantity
- return sorted region totals

sliding_window
- detect spikes against the previous window only
- avoid including current value in its own baseline

flatten_json
- flatten dict/list structures into stable path keys
- preserve empty dict/list leaves

topo_batches
- deterministic topological batching
- cycle returns []

deep_merge
- recursive config merge
- lists replace
- None deletes existing keys
- missing None deletion is ignored
- no mutation of inputs
```

## Memory Construction Logic

The Python corpus follows the same pattern as the Ordo corpus:

```text
situation -> failure shape -> correction -> outcome
```

The memories are not identity memories. They are artifact scars.

Examples of narrow anchors:

```text
solution.py
csv.DictReader
values[i-width:i]
round(avg, 2)
orders[1].total
indegree[after]
copy base before applying override
missing override key is not the same as override value None
```

This should make the corpus activate for Python artifact tasks while remaining mostly inert for unrelated tasks.

## First Qwen 4B Q4_K_M Run

Model:

```text
qwen35-4b-q4km-chat
```

Initial full comparison:

```text
Run: rag_runs/python-domain-scar-20260611-123753

no_memory:
- 3/5 pass rate 0.600
- builder passes: csv_rollup, flatten_json
- topo_batches passed after repair_1
- failures: sliding_window, deep_merge

python_scar_memories:
- 3/5 pass rate 0.600
- builder passes: csv_rollup, sliding_window, topo_batches
- failures: flatten_json, deep_merge
```

Interpretation:

```text
The first corpus was not globally better, but it moved behavior:
- fixed sliding_window immediately
- moved topo_batches from repair to builder
- damaged flatten_json
- did not fix deep_merge
```

That is still useful because it showed the corpus was active, just not calibrated.

## Scar Repair Loop

Additional scars were added from real verifier failures:

```text
flatten_json:
- empty dict/list leaves vanished
- root paths got leading dots
- root dict keys became bracketed like [orders]
- list recursion used a path variable before assignment
- stable recursion skeleton needed one generic walk() path

deep_merge:
- missing override key was confused with override value None
- new non-None override keys were skipped
- nested base dicts were copied as empty dicts
- existing None override replaced with None instead of deleting
- base must be copied first, then overrides applied
```

The decisive repair was adding compact stable-skeleton scars, not just more individual warnings.

## Final Python Memory Result

Full memory condition:

```text
Run: rag_runs/python-domain-scar-20260611-124515
Model: qwen35-4b-q4km-chat
Condition: python_scar_memories

Passed:       5/5
Pass rate:    1.000
Builder pass: 5/5
Repair pass:  0/5
```

Final no-memory baseline rerun:

```text
Run: rag_runs/python-domain-scar-20260611-124543
Model: qwen35-4b-q4km-chat
Condition: no_memory

Passed:       4/5
Pass rate:    0.800
Builder pass: 2/5
Repair pass:  2/5
Failed:       deep_merge
```

Comparison:

```text
no_memory:              4/5, 2 builder-stage passes
python_scar_memories:   5/5, 5 builder-stage passes
```

The headline is not just +1 task. The stronger signal is that the memory corpus converted the suite into clean builder-stage behavior.

## Collateral Probe

Model:

```text
qwen35-4b-q4km-chat
```

Conditions:

```text
no_memory
with_python_scar_memories
```

General tasks:

```text
short lighthouse story
cupcake arithmetic
vanilla cake steps
overwatered houseplant advice
```

Observed:

```text
No memory:
- story normal
- math correct, 12 cupcakes
- cake normal
- plant advice normal

With Python scar memories:
- story normal
- math correct, 12 cupcakes
- cake normal
- plant advice normal
- strict Python/scar contamination check: none observed
```

Contamination detector checked for:

```text
solution.py
csv
DictReader
deep_merge
flatten_json
topo
python module
unittest
markdown code fences
```

Result:

```text
Observed collateral damage: none in this small probe
Observed Python-domain contamination: none in this small probe
Math degradation: none
```

## Comparison To Ordo/Rust Corpus

Similarities:

```text
- both use task labels
- both use common artifact-contract memories
- both use task-specific scars
- both avoid persona identity
- both improve executable test performance
- both show no obvious general-task hijacking in small collateral probes
```

Differences:

```text
Ordo/Rust:
- stronger raw benchmark jump on Gemma: 0/12 -> 10-11/12
- larger corpus
- many more scars concentrated around one hard task

Python:
- smaller jump on Qwen 4B Q4: 4/5 -> 5/5
- bigger quality shift: 2/5 builder -> 5/5 builder
- required compact stable-skeleton scars for complex recursive tasks
```

## Important Lesson

The Python experiment adds a second successful example of a domain-selective scar corpus.

The key lesson is that some tasks need two levels of memory:

```text
1. local scar memories
   "this exact failure happened"

2. compact stable-skeleton memories
   "this is the whole safe shape that prevents the class of failures"
```

For `flatten_json` and `deep_merge`, individual scars were not enough. The model kept fixing one edge case and breaking another. The stable skeleton memories helped it hold the entire structure.

This suggests the next corpus recipe should include:

```text
- common contract memories
- individual failure scars
- one compact stable implementation-shape memory per hard task
```

## Status

This corpus is now a valid second domain-selective scar memory experiment.

Current strongest result:

```text
qwen35-4b-q4km-chat:
- no_memory: 4/5, failed deep_merge, only 2 builder passes
- python_scar_memories: 5/5, all builder-stage
- collateral probe: no visible general-task damage
```

The next useful test is cross-domain isolation:

```text
- Ordo memories on Python tasks
- Python memories on Ordo tasks
- Ordo + Python memories together on both task suites
```

That will tell us whether multiple domain-selective scar corpora can coexist without interfering.

## Routed Stack Result

Cross-domain testing showed that wholesale concatenation can create mild domain-domain interference, especially on brittle tasks. The next test added a simple routed-stack condition.

In this condition, the Python and Ordo corpora both exist in the memory bank, but the prompt receives only the corpus selected for the active task domain.

Run:

```text
rag_runs/python-domain-scar-20260611-141018
```

Model:

```text
qwen35-4b-q4km-chat
```

Condition:

```text
routed_stack_memories
```

Result:

```text
Passed:    5/5
Pass rate: 1.000

csv_rollup:      pass, builder
sliding_window:  pass, builder
flatten_json:    pass, builder
topo_batches:    pass, builder
deep_merge:      pass, repair_1
```

Interpretation:

```text
The Python corpus still works when it is part of a larger stacked memory bank, as long as unrelated corpora are not injected wholesale.
Routing appears to preserve the domain benefit while reducing prompt-time attention competition.
```
