# Combined Corpus Interference Notes

Date: 2026-06-11

## Question

Were the Python domain-selective scar memories included with the Rust/Ordo memories?

Answer:

```text
No.
```

The Rust/Ordo results and the first Python results were intentionally run separately. The Python corpus was not included in the Rust runs until this combined-corpus test.

## Why This Test Matters

The core question is whether multiple separate domain-selective scar corpora can coexist.

There are three possible outcomes:

```text
1. Clean coexistence
   Each corpus activates only for its own domain.

2. General-task contamination
   The combined corpus starts hijacking stories, math, cooking, advice, etc.

3. Domain interference
   The corpora do not damage general tasks, but one domain's memories add enough noise to weaken another domain's benchmark.
```

The observed result is closest to outcome 3.

## Harness Changes

Python harness now supports:

```text
no_memory
python_scar_memories
ordo_only_memories
python_plus_ordo_memories
```

File:

```text
scripts/run_python_domain_scar_benchmark.py
```

Ordo harness now supports:

```text
python_only_memories
ordo_artifact_plus_python
routed_stack_memories
```

File:

```text
scripts/run_ordo_extended_suite_benchmark.py
```

Python harness now also supports:

```text
routed_stack_memories
```

In this first router baseline, the corpora remain stacked on disk, but the prompt receives only the active domain's memories:

```text
Python task -> Python scar memories
Ordo/Rust task -> Ordo artifact memories
```

This is not yet fuzzy top-k retrieval. It is a clean domain router to test whether routing alone removes the interference caused by wholesale concatenation.

## Corpus Sizes

At the time of this test:

```text
Python scar corpus:
- 80 lines
- 1,500 words
- 9,781 characters

Ordo/Rust scar corpus:
- 113 lines
- 2,329 words
- 16,764 characters

Combined prompt memory mass:
- about 26.5k characters before task text
```

This is a real interference stress test. The combined memory block is large enough to create attention competition.

## Python Suite With Mixed Memories

Model:

```text
qwen35-4b-q4km-chat
```

Run:

```text
rag_runs/python-domain-scar-20260611-135436
```

Results:

```text
python_scar_memories:
- 4/5
- failed deep_merge in this run

ordo_only_memories:
- 4/5
- failed flatten_json

python_plus_ordo_memories:
- 4/5
- failed deep_merge
```

Interpretation:

```text
Adding Ordo memories did not collapse the Python suite.
The aggregate result stayed 4/5.
However, Python-only was also 4/5 in this run, despite having previously reached 5/5 builder-stage.
This indicates some stochasticity in the Python suite, especially around deep_merge.
```

Important earlier Python result:

```text
rag_runs/python-domain-scar-20260611-124515
python_scar_memories:
- 5/5
- 5/5 builder-stage
```

So the current read is:

```text
Python corpus can reach 5/5.
Combined Python+Ordo did not obviously make Python worse than the stochastic Python-only rerun.
Deep merge remains the brittle Python task.
```

## Ordo/Rust Suite With Mixed Memories

Model:

```text
qwen35-4b-q4km-chat
```

Run:

```text
rag_runs/ordo-extended-suite-20260611-135710
```

Results:

```text
ordo_artifact_scars:
- 12/12
- ramp_roundtrip passed at repair_1

ordo_artifact_plus_python:
- 11/12
- failed ramp_roundtrip
```

This is the clearest interference signal.

The combined corpus did not broadly destroy Rust behavior. It passed 11 of 12 tasks. But it did weaken the most brittle Ordo task, `ramp_roundtrip`.

Failure shape:

```text
The generated ramp library was mostly correct.
The binary got confused and invented a global/static runtime strategy.
It mentioned static mut / unsafe reasoning and failed with lifetime/scope errors.
```

Relevant observation:

```text
The Python memories include general mutation/copy/reuse language.
When appended after the Rust memories, they may have added noise around state ownership and mutation.
The model still knew the Rust library shape, but repair-time reasoning drifted away from the simple CLI-memory shape.
```

This is not general domain hijacking. It is attention/repair interference on a brittle task.

## General Capability Probe With Both Corpora

Model:

```text
qwen35-4b-q4km-chat
```

Conditions:

```text
Python scar corpus + Ordo/Rust scar corpus injected together
```

General tasks:

```text
short story
cupcake math
vanilla cake instructions
```

Observed:

```text
short story:
- normal
- no strict Python/Rust contamination observed

math:
- correct
- 12 cupcakes
- no strict Python/Rust contamination observed

cake:
- normal
- no strict Python/Rust contamination observed
```

Interpretation:

```text
Combined corpora did not visibly damage ordinary general capabilities in this small probe.
The main risk is not general-task collapse.
The main risk is domain-domain interference when both corpora are injected wholesale.
```

## Practical Lesson

Multiple domain-selective scar corpora can probably coexist, but they should not all be dumped into context at full strength.

The better architecture is routed retrieval:

```text
user task
  -> classify domain
  -> retrieve only matching corpus
  -> include small common artifact contract
  -> include task-specific scars
  -> avoid unrelated domain scars unless the task explicitly crosses domains
```

The combined test suggests:

```text
Good:
- corpora are still not hijacking general tasks
- combined memory did not collapse Python
- combined memory preserved most Ordo behavior

Bad:
- adding the Python corpus to Ordo cost one brittle task
- full-context injection creates attention competition
- repair prompts may be especially sensitive to unrelated scars
```

## Recommended Next Design Rule

Do not combine corpora by concatenating everything.

Use:

```text
1. Domain router
2. Top-k task memories
3. Small common contract memories
4. Optional cross-domain memories only when task evidence supports them
5. Memory budget per domain
```

Suggested memory budget:

```text
common domain contract: 3-7 memories
task-specific scars: 3-10 memories
unrelated domain memories: 0 by default
cross-domain memories: 1-3 only when useful
```

## Current Answer

Were the corpora combined in the original tests?

```text
No.
```

What happened when they were combined?

```text
General tasks:
- no visible damage

Python benchmark:
- combined Python+Ordo: 4/5
- similar to stochastic Python-only rerun

Ordo benchmark:
- Ordo-only: 12/12
- Ordo+Python: 11/12
- lost ramp_roundtrip
```

Conclusion:

```text
The corpora remain mostly domain-selective, but wholesale concatenation can create interference on brittle tasks.
The next architecture should route memories instead of injecting every corpus at once.
```

## Routed Stack Baseline

After the wholesale combined-corpus run showed a clear Ordo interference signal, the harnesses were updated with a simple routed-stack condition.

The key change:

```text
Keep multiple corpora available.
Do not inject all of them.
Route by task domain and inject only the selected corpus.
```

Python routed-stack run:

```text
Model: qwen35-4b-q4km-chat
Run: rag_runs/python-domain-scar-20260611-141018
Condition: routed_stack_memories

Passed:    5/5
Pass rate: 1.000

csv_rollup:      pass, builder
sliding_window:  pass, builder
flatten_json:    pass, builder
topo_batches:    pass, builder
deep_merge:      pass, repair_1
```

Ordo routed-stack run:

```text
Model: qwen35-4b-q4km-chat
Run: rag_runs/ordo-extended-suite-20260611-141111
Condition: routed_stack_memories

Passed:    12/12
Pass rate: 1.000

All tasks passed.
ramp_roundtrip passed at repair_1.
```

This matters because the earlier wholesale stack produced:

```text
Ordo-only:        12/12
Ordo + Python:    11/12
Routed stack:     12/12
```

The router recovered the brittle `ramp_roundtrip` task that was lost when Python memories were injected wholesale.

Interpretation:

```text
The problem was not that the corpora cannot coexist.
The problem was prompt-time attention competition.
Routing lets the corpora stack without forcing unrelated scars into every task.
```

Current architecture rule:

```text
Stack corpora in the memory bank.
Route memories at inference time.
Inject only the domain/task scars that are active.
Add cross-domain memories only when the task explicitly needs them.
```

## LoRA Training Implication

Runtime routing from disk is not enough for a LoRA.

If the route exists only in a Python harness or a local memory bank, the adapter will not learn it. The routing behavior has to appear inside the training distribution.

To convert the routed-stack result into LoRA-trainable form, a dataset builder was added:

```text
scripts/build_routed_lora_dataset.py
```

Generated files:

```text
data/routed_memory_lora_sft.jsonl
data/routed_memory_lora_sft_manifest.json
```

Current dataset shape:

```text
72 total examples

17 baked_domain_solution examples
- task prompt -> correct artifact
- no memory text in the prompt
- teaches the LoRA to internalize the routed behavior

17 active_memory_solution examples
- active routed memory block + matching task -> correct artifact
- preserves the memory-to-behavior pathway

17 interference_guard examples
- unrelated domain archive excerpt + current task -> correct artifact
- teaches the adapter to ignore inactive memories

17 router_selection examples
- task -> active corpus JSON
- teaches the explicit routing decision

4 general_capability_guard examples
- story, arithmetic, cake, plant advice
- keeps unrelated prompts from collapsing into artifact mode
```

This is the trainable form of the router:

```text
not: corpora sitting on disk
but: examples that bind task cues -> selected domain behavior -> correct artifact
```

The next LoRA should be trained on this routed dataset, then evaluated without injecting the memory files. That test will tell us whether the routing and scars have moved from external prompt context into the adapter.

Important correction:

```text
Do not jump straight from routed runtime success to LoRA training.
First keep testing scar composition locally.
```

A new local scar-ablation pass is documented here:

```text
MEMORY_SCAR_ABLATION_FINDINGS.md
```

A broader cross-domain coexistence matrix is documented here:

```text
CROSS_DOMAIN_MEMORY_MATRIX_FINDINGS.md
```

Core local finding:

```text
One scar and three scars were often not enough.
Full task-specific scar sets worked.
Common scars sometimes helped and sometimes interfered.
Cross-domain noise can preserve final pass rate while reducing builder-stage quality.
```

This means the LoRA dataset should not blindly include every common memory with every task. It should preserve the distinction between task-scar-only, common-plus-task, interference guard, and router-selection examples.

Cross-domain matrix finding:

```text
17 executable domain tasks were tested across Python and Ordo.
Own-domain corpora reached 17/17.
Wholesale combined corpora reached 16/17.
Routed stack reached 16/17.
Foreign-only corpora reached 4/17, same aggregate as no-memory, but with different Python task behavior.
```

The conclusion is:

```text
Different domain memories can mostly coexist, but brittle tasks expose interference first.
Routing fixes some interference, but the routed subset still has to be calibrated.
```

## Updated LoRA Evaluation Tests

The testing side was updated to match the LoRA goal.

Dataset validation:

```text
scripts/validate_routed_lora_dataset.py
```

This checks:

```text
- valid JSONL
- unique ids
- required chat message shape
- required example kinds
- required domains
```

Current validation result:

```text
passed
rows: 72
kinds:
- baked_domain_solution: 17
- active_memory_solution: 17
- interference_guard: 17
- router_selection: 17
- general_capability_guard: 4

domains:
- python: 15
- ordo: 36
- router: 17
- general: 4
```

Post-training no-memory evaluation:

```text
scripts/run_routed_lora_no_memory_eval.py
```

This is the important LoRA test. It runs the trained adapter with:

```text
condition: no_external_memory
```

Suites:

```text
python executable suite
ordo executable suite
general capability guards
router-selection probes
```

Use:

```text
ROUTED_LORA_MODEL=<trained-adapter-model> python scripts/run_routed_lora_no_memory_eval.py
```

Optional filters:

```text
ROUTED_LORA_SUITES=python,ordo,general,router
ROUTED_LORA_TASKS=deep_merge,ramp_roundtrip
ROUTED_LORA_MAX_REPAIRS=2
```

Pass condition:

```text
The adapter improves Python/Ordo behavior without any injected memory text.
General prompts stay general.
Router probes select the right active corpus or no corpus.
```
