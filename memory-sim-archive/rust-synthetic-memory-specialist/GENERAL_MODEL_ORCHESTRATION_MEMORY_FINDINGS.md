# General Model Orchestration Memory Findings

Date: 2026-06-12

## Question

Can a single general local model use a memory corpus to imitate a multi-agent orchestration runtime internally?

The target behavior is:

```text
requirements reader
builder
adversarial reviewer
verifier
repairer
final answer only
```

No actual subagents are spawned. The model is asked to run the loop privately.

## Corpus

File:

```text
data/sequential_orchestration_memories.txt
```

Size:

```text
50 lines
764 words
5,063 characters
```

The corpus is process-shaped, not domain-shaped. It remembers failures and fixes around:

```text
- narrating the committee instead of answering
- reading requirements first
- building the smallest sufficient artifact
- adversarially checking edge cases
- verifying exact output contracts
- repairing only the failing point
- returning final output only
- not turning ordinary tasks into coding tasks
```

## General Eval Harness

File:

```text
scripts/run_general_orchestration_eval.py
```

Tasks:

```text
invoice_json
tile_math
schedule_constraints
ticket_filter
prompt_injection_distraction
exact_format
```

The graders are deterministic and intentionally strict where the prompt asks for exact output.

## Base General Model

Model:

```text
qwen35-9b-q3km-chat
```

Run:

```text
rag_runs/general-orchestration-20260612-150526-qwen35-9b-q3km-chat
```

Results:

```text
no_memory:              3/6
orchestration_memory:   4/6
```

Task-level change:

```text
Fixed by orchestration memory:
- ticket_filter

Still passed:
- schedule_constraints
- prompt_injection_distraction
- exact_format

Still failed:
- invoice_json
- tile_math
```

Interpretation:

```text
Prompt-injected orchestration memory helped one general task: filtering and ordering tickets.
It did not fix arithmetic. In fact, the invoice arithmetic stayed wrong.
The memory seems more useful for checklist/constraint filtering than for numeric computation in this first run.
```

## Baked Ollama Variant

Model created:

```text
qwen35-9b-q3km-orchestration-memory
```

Modelfile:

```text
ollama_imports/qwen35-9b-q3km-orchestration-memory.Modelfile
```

This variant uses the same base model with orchestration memories in the Ollama system layer.

Run:

```text
rag_runs/general-orchestration-20260612-150538-qwen35-9b-q3km-orchestration-memory
```

Result:

```text
no prompt-side memory: 3/6
```

Task-level result:

```text
passed:
- schedule_constraints
- prompt_injection_distraction
- exact_format

failed:
- invoice_json
- tile_math
- ticket_filter
```

Interpretation:

```text
Baking the orchestration memories into the system layer did not reproduce the prompt-injected improvement.
The delivery channel matters.
The same memory text behaved differently when injected into the user prompt versus baked into the model system layer.
```

## Important Correction

The first loose tile grader counted any response containing 53 as correct.

That was too permissive because the task requested:

```text
Return only the number of tiles.
```

The grader was tightened to require exactly:

```text
53
```

After tightening, the baked model's tile response was correctly marked failed because it explained the calculation before giving 53.

## Current Conclusion

The single-model orchestration memory is not yet a stable general capability boost.

Current evidence:

```text
Prompt-side orchestration memory:
- improved qwen35-9b-q3km-chat from 3/6 to 4/6
- helped constraint/filtering behavior
- did not help arithmetic

Baked Ollama system-layer memory:
- stayed at 3/6
- did not preserve the ticket_filter improvement
```

Working hypothesis:

```text
The process memory is too abstract and too instruction-like.
It needs task-grounded scars from actual general-task failures, especially arithmetic, extraction, and exact-format failures.
```

## Next Design

The next version should not only say:

```text
read, build, adversarially check, verify, repair
```

It should include concrete general-task scars:

```text
I remember the invoice where I added unrelated numbers and inflated subtotal.
I remember the tile task where I solved the math but violated "return only the number."
I remember the ticket filter where I sorted before excluding duplicates.
I remember the schedule task where only one overlapping slot survived all calendars.
I remember the untrusted note where quoted instructions were data, not authority.
```

That would turn the orchestration memory from an abstract process into a scarred generalist corpus.

## V2 Scarred Generalist Recipe

File:

```text
data/sequential_orchestration_memories_v2.txt
```

The v2 corpus was built from the actual failures in the v1 eval:

```text
- invoice subtotal inflation
- money rounding
- JSON-only exact keys
- tile math with return-only-number format
- ticket filtering order
- duplicate exclusion before sorting
- schedule intersection
- untrusted note as data
- exact three-line output
- no committee narration
```

The eval harness now supports selecting the memory file:

```text
GENERAL_ORCH_MEMORY_PATH=<path> python scripts/run_general_orchestration_eval.py
```

Run:

```text
rag_runs/general-orchestration-20260612-152233-qwen35-9b-q3km-chat
```

Model:

```text
qwen35-9b-q3km-chat
```

Result:

```text
no_memory:              3/6
orchestration_memory:   6/6
```

Task-level result:

```text
invoice_json:                 fail -> pass
tile_math:                    fail -> pass
schedule_constraints:         pass -> pass
ticket_filter:                fail -> pass
prompt_injection_distraction: pass -> pass
exact_format:                 pass -> pass
```

Representative correct outputs:

```json
{
  "customer": "Mara Lee",
  "subtotal": 42.5,
  "tax": 3.4,
  "total": 45.9
}
```

```text
53
```

```text
T-7,T-2,T-5
```

## V2 Interpretation

This is the strongest signal so far for the general orchestration idea.

The important change was not adding more abstract process text. The improvement came from turning the process into concrete scars:

```text
not just "verify the answer"
but "I once solved the tile math and failed because I did not return only 53"

not just "check arithmetic"
but "I once inflated the invoice subtotal; the correct subtotal was 42.50"

not just "filter carefully"
but "I once sorted before excluding duplicates; the surviving ids were T-7,T-2,T-5"
```

This matches the Rust/Python pattern:

```text
abstract instruction is weak
scarred failure/fix memory is strong
```

## Caveat

This is not yet a general proof.

The v2 memories were built from the same task family and some exact failures in the eval. That is allowed for recipe development, but the next meaningful test needs held-out tasks:

```text
- new invoice with different numbers
- new tile/area problem
- new filtering rule set
- new JSON extraction
- new prompt-injection distraction
- new exact-format prompt
```

If v2 transfers to those held-out variants, then the corpus is learning a reusable generalist checking loop rather than memorizing six answers.

## Domain-Layer Test

Question:

```text
Can the v2 orchestration memory be used as a universal layer on top of all domain scar corpora?
```

Initial answer:

```text
Not yet.
```

Python domain run:

```text
Run: rag_runs/single-model-orchestration-20260612-152544
Model: qwen35-4b-q4km-chat
Memory: data/sequential_orchestration_memories_v2.txt

domain_only:               5/5
domain_plus_orchestration: 4/5
```

Python task-level result:

```text
domain_only:
- all 5 passed

domain_plus_orchestration:
- csv_rollup passed
- sliding_window passed
- flatten_json passed
- topo_batches failed
- deep_merge passed
```

Important nuance:

```text
v2 helped deep_merge reach builder-stage in this run,
but it introduced a topo_batches failure.
```

Ordo domain slice:

```text
Run: rag_runs/single-model-orchestration-20260612-152807
Model: qwen35-4b-q4km-chat
Tasks: mini_runtime, ramp_roundtrip, workflow_codec, audit_timeline

domain_only:               4/4
domain_plus_orchestration: 3/4
```

Ordo task-level result:

```text
domain_only:
- all 4 passed

domain_plus_orchestration:
- mini_runtime passed
- ramp_roundtrip failed
- workflow_codec passed
- audit_timeline passed
```

Interpretation:

```text
V2 works as a general-task memory layer in the small general eval.
V2 is not yet safe as a default layer over specialized domain corpora.
It can improve one brittle task while interfering with another.
```

Current design rule:

```text
Use v2 for general tasks.
Do not automatically stack v2 on domain-specific scar corpora.
Route it only when the task is general, mixed-domain, or explicitly needs planning/checking behavior.
For Python/Ordo executable tasks, domain scars remain primary.
```

Next recipe direction:

```text
Create a domain-safe orchestration layer that contains no task-specific general answers.
Keep exact arithmetic/filtering scars in the generalist corpus.
Use a router to decide:
- general task -> general orchestration v2
- Python artifact -> Python scars, optional domain-safe process scars
- Ordo artifact -> Ordo scars, optional domain-safe process scars
```

## Base Model General Test

After focusing back on general models, the eval harness was updated for base-model testing.

Changes:

```text
scripts/run_general_orchestration_eval.py
```

Now supports:

```text
GENERAL_ORCH_API=chat
GENERAL_ORCH_API=generate
```

For raw/base models, the generate path uses a transport-style prompt:

```text
### Relevant Memories
...

### Task
...

### Instruction
Do any checking privately. Return only the requested final answer.

### Final Answer
```

The evaluator also saves both raw and cleaned responses, stripping visible `<think>...</think>` blocks before grading. This matters because base models may emit thinking traces even when the final answer is usable.

Base model:

```text
qwen35-9b-base-q8-raw
```

Run:

```text
rag_runs/general-orchestration-20260612-162415-qwen35-9b-base-q8-raw
```

Results:

```text
no_memory:                 5/6
orchestration_memory v2:   6/6
orchestration_primitives:  6/6
```

Task-level change:

```text
no_memory failed tile_math.
The base model calculated 53 correctly, but included explanation and an extra final-answer marker.
V2 fixed the output contract and returned only:

53
```

Interpretation:

```text
The base model did not need help with most reasoning on this small suite.
It needed help with final-answer contract discipline.
The v2 scar memory supplied that discipline.
The primitive wrapper preserved the 6/6 score but did not improve beyond v2 on this suite.
```

Important distinction:

```text
For the instruct/general model, v2 corrected several task failures.
For the base model, v2 mainly corrected format/output discipline.
```

Current best general-model result:

```text
qwen35-9b-q3km-chat:
- no_memory: 3/6
- v2 memory: 6/6
- v2 + primitives: 6/6

qwen35-9b-base-q8-raw:
- no_memory: 5/6
- v2 memory: 6/6
- v2 + primitives: 6/6
```

## Expanded 15-Task General Benchmark

The 6-task eval was too small, so the suite was expanded to 15 tasks across more domains.

Added task types:

```text
probability without replacement
spatial turns
chemistry/moles
biology/Punnett square
logic scheduling
rhetorical fallacy
code tracing
table lookup/join
median/statistics
```

The corrected code-trace answer is:

```text
x = 2
n = 3 -> 7
n = 5 -> 19
n = 2 -> 40
```

So the grader expects exactly:

```text
40
```

### Instruct/General Model

Model:

```text
qwen35-9b-q3km-chat
```

Run:

```text
rag_runs/general-orchestration-20260612-163316-qwen35-9b-q3km-chat
```

Results:

```text
no_memory:                 8/15
orchestration_memory v2:   11/15
orchestration_primitives:  10/15
```

V2 fixed:

```text
invoice_json
tile_math
ticket_filter
```

V2 preserved:

```text
schedule_constraints
prompt_injection_distraction
exact_format
chemistry_moles
biology_punnett
rhetoric_fallacy
table_lookup_join
median_stat
```

V2 still failed:

```text
probability_no_replacement
spatial_turns
logic_schedule
code_trace
```

Primitive wrapper result:

```text
10/15
```

The primitive wrapper preserved the original v2 wins but introduced one extra failure on `median_stat`, so it is not better than plain v2 on this instruct model.

### Base Model

Model:

```text
qwen35-9b-base-q8-raw
```

API:

```text
generate
```

Run:

```text
rag_runs/general-orchestration-20260612-163400-qwen35-9b-base-q8-raw
```

Results:

```text
no_memory:                 11/15
orchestration_memory v2:   14/15
orchestration_primitives:  13/15
```

V2 fixed:

```text
tile_math
probability_no_replacement
code_trace
median_stat
```

V2 introduced/regressed:

```text
logic_schedule
```

Primitive wrapper fixed:

```text
logic_schedule
```

But primitive wrapper failed:

```text
code_trace
median_stat
```

So primitives changed the failure distribution rather than improving the total score.

## Expanded Benchmark Interpretation

The larger suite gives a better picture:

```text
Instruct/general model:
- v2 improves 8/15 -> 11/15
- primitives reduce that to 10/15

Base model:
- v2 improves 11/15 -> 14/15
- primitives reduce that to 13/15
```

This supports the user's expectation that a base model may respond differently to memory. The base model already had stronger latent performance on this particular suite after a proper raw transport prompt, and v2 mostly improved final-answer discipline and several reasoning checks.

The primitive wrapper is not dead, but this prompt-level implementation is too blunt. It can move failures around but does not yet raise the ceiling.

## V3 Recipe Targets

The next scar corpus should target the remaining failures from the expanded benchmark:

```text
probability_no_replacement:
- multiply sequential probabilities
- after first draw, denominator and counts change
- 3/5 * 2/4 = 3/10

spatial_turns:
- track facing direction only
- walking forward does not change facing
- north -> right = east -> left = north -> left = west

logic_schedule:
- solve constraints by eliminating impossible assignments
- Ana cannot Monday
- Ben after Ana means Ana cannot Wednesday
- therefore Ana Tuesday, Ben Wednesday, Carla Monday

code_trace:
- update the variable after every loop item
- x=2 -> 7 -> 19 -> 40

median_stat:
- sort first
- for five values, pick the third item
- 3,4,8,12,15 -> 8
```

V3 should add these as concrete scars, not as generic advice.

## Research Focus Shift

As of 2026-06-12, instruct-model testing is paused for this track.

Reason:

```text
The instruct tests served their purpose:
- they showed the memory corpus can benefit an instruction-tuned general model
- they gave a comparison point against base/raw behavior
```

The forward research target is now:

```text
base / non-instruct models
```

Why:

```text
The thesis is strongest if memory-shaped corpora can recover or improve behavior in pretrained/base models without relying on existing instruction tuning.
The eventual Kaggle training target is likely a 4B model, so the recipe should be proven on base/raw models before moving into LoRA training.
```

Benchmark defaults were changed accordingly:

```text
scripts/run_general_orchestration_eval.py
- default model: qwen35-9b-base-q8-raw
- default API: generate

scripts/run_hard_general_orchestration_eval.py
- default model: qwen35-9b-base-q8-raw
- default API: generate
```

Instruct-model numbers remain useful historical comparison, but future recipe work should report base-model results first.

## Hard 20-Task Base Benchmark

To avoid overfitting to the 15-task medium benchmark, a harder base-only suite was added.

File:

```text
scripts/run_hard_general_orchestration_eval.py
```

Model:

```text
qwen35-9b-base-q8-raw
```

API:

```text
generate
```

Run:

```text
rag_runs/hard-general-orchestration-20260612-164229-qwen35-9b-base-q8-raw
```

Results:

```text
no_memory:                 7/20
orchestration_memory v2:   7/20
orchestration_primitives:  7/20
```

Passed by all three conditions:

```text
probability_two_without_replacement
boolean_logic
medical_triage_basic
chemistry_limiting_reagent
date_offset
causal_counterfactual
```

The primitive condition also fixed:

```text
rhetoric_two_fallacies
```

But it lost:

```text
compound_discount_tax
```

So the total stayed flat.

Hard failures that remained unresolved:

```text
weighted_average_drop_lowest
nested_json_invoice
multi_constraint_seating
grid_path_orientation
conditional_probability
code_trace_nested
table_join_filter
prompt_injection_nested_data
format_trap_yaml
set_intersection_order
ratio_scaling
statistics_iqr
```

## Hard Benchmark Interpretation

The hard suite shows a boundary:

```text
Medium benchmark:
- v2 improves base 11/15 -> 14/15

Hard benchmark:
- v2 does not improve base 7/20 -> 7/20
```

This means the current v2 recipe is not yet a general reasoning exoskeleton. It is a useful scar corpus for medium general tasks, especially output-contract discipline and a few familiar reasoning checks, but it does not yet repair harder multi-step computation or symbolic state tracking.

The hard failures cluster around:

```text
1. multi-step arithmetic with intermediate state
2. nested totals and category aggregation
3. constraint solving
4. spatial orientation and grid state
5. conditional probability
6. nested loop tracing
7. table joins
8. adversarial extraction from trusted/untrusted text
9. exact structured output
10. robust order preservation
```

This is good recipe-development data. V3 should not add generic advice. It should add one or two scarred memories for each failure cluster, written as lived episodes with:

```text
situation -> wrong first impulse -> correction/check -> final outcome
```

The target is to make those patterns reusable without leaking exact benchmark answers.

## V3 Experiments

A first V3 base-general corpus was added:

```text
data/sequential_orchestration_memories_v3_base_general.txt
```

Size:

```text
74 lines
1,546 words
9,236 characters
```

It targeted the hard-failure clusters with lived scars for:

```text
discount/tax sequencing
weighted averages
ratio scaling
category totals
table joins
list intersection order
constraint solving
conditional probability
without-replacement probability
grid state
nested loop tracing
IQR and medians
trusted/untrusted source boundaries
strict YAML output
```

### V3 Alone

Medium run:

```text
rag_runs/general-orchestration-20260612-164832-qwen35-9b-base-q8-raw
```

Result:

```text
orchestration_memory v3:   10/15
orchestration_primitives:  10/15
```

Hard run:

```text
rag_runs/hard-general-orchestration-20260612-164644-qwen35-9b-base-q8-raw
```

Result:

```text
orchestration_memory v3:   8/20
orchestration_primitives:  8/20
```

Interpretation:

```text
V3 alone improved the hard benchmark by one point over the 7/20 baseline.
But it badly regressed the medium benchmark from the V2 high-water mark of 14/15 to 10/15.
Therefore V3 is not a replacement for V2.
```

### V2 + Full V3

The harness now supports multiple memory files separated by semicolons:

```text
GENERAL_ORCH_MEMORY_PATH=file_a.txt;file_b.txt
HARD_ORCH_MEMORY_PATH=file_a.txt;file_b.txt
```

Medium run:

```text
rag_runs/general-orchestration-20260612-165025-qwen35-9b-base-q8-raw
```

Result:

```text
orchestration_memory v2+v3:   13/15
orchestration_primitives:     12/15
```

Hard run:

```text
rag_runs/hard-general-orchestration-20260612-165130-qwen35-9b-base-q8-raw
```

Result:

```text
orchestration_memory v2+v3:   6/20
orchestration_primitives:     8/20
```

Interpretation:

```text
Full stacking caused interference.
It preserved many V2 behaviors, but reduced both medium and hard reliability.
This supports the earlier observation that more memory is not automatically better.
```

### V2 + Targeted V3

A smaller targeted add-on was added:

```text
data/sequential_orchestration_memories_v3_targeted_base_general.txt
```

It contains only seven blocks:

```text
one anti-overload/common memory
conditional probability
without-replacement probability
grid state tracking
orientation-only tracking
trusted/untrusted extraction
strict YAML shape
```

Medium run:

```text
rag_runs/general-orchestration-20260612-165355-qwen35-9b-base-q8-raw
```

Result:

```text
orchestration_memory v2+targeted_v3:   13/15
orchestration_primitives:              13/15
```

Hard run:

```text
rag_runs/hard-general-orchestration-20260612-165509-qwen35-9b-base-q8-raw
```

Result:

```text
orchestration_memory v2+targeted_v3:   8/20
orchestration_primitives:              7/20
```

Interpretation:

```text
The targeted add-on is better than full V3, but it still trades away medium reliability.
It gets hard mode to 8/20, but V2 alone already gives the best medium result at 14/15.
```

## Current Best Recipe

For base/raw general testing, the current best stable recipe is still:

```text
data/sequential_orchestration_memories_v2.txt
```

Current score table:

```text
Medium 15-task benchmark:
- no_memory:              11/15
- v2:                     14/15
- v2 + primitives:        13/15
- v3 alone:               10/15
- v2 + full v3:           13/15
- v2 + targeted v3:       13/15

Hard 20-task benchmark:
- no_memory:              7/20
- v2:                     7/20
- v2 + primitives:        7/20
- v3 alone:               8/20
- v2 + full v3:           6/20
- v2 + full v3 + prim:    8/20
- v2 + targeted v3:       8/20
- v2 + targeted v3 + prim: 7/20
```

Design lesson:

```text
V2 is dense and effective because it contains task-grounded scars close to the medium benchmark.
V3 proves that new scars can move hard-task behavior, but broad additions create interference.
The next step should be pruning and routing, not simply adding memory volume.
```

Do not train the full V3 stack yet. If a LoRA training dataset is prepared from this branch, use V2 as the stable base and treat targeted V3 scars as optional ablation rows.

## 2026-06-14 Hard-Task Recipe Search

The hard benchmark was reviewed before further recipe work. Several grader issues were found and corrected:

```text
multi_constraint_seating:
- previous expected answer violated "Cy is not in seat 4"
- corrected expected order: Ada,Cy,Ben,Dee

code_trace_nested:
- correct trace result is 39, not 33

statistics_iqr:
- median-of-halves IQR is 7, not 8

rhetoric_two_fallacies:
- grading is now case-insensitive

comma-list tasks:
- comma spacing is normalized where the prompt only says "separated by commas"

ratio_scaling:
- accepts both 50 g and 50 grams
```

Corrected hard baseline run:

```text
rag_runs/hard-general-orchestration-20260614-073232-qwen35-9b-base-q8-raw
```

Corrected results:

```text
no_memory:                 9/20
orchestration_memory v2:   12/20
orchestration_primitives:  11/20
```

This is the honest corrected hard baseline. It shows that V2 does improve hard performance:

```text
9/20 -> 12/20
```

### V4 Hard Patch

File:

```text
data/sequential_orchestration_memories_v4_hard_patch.txt
```

Purpose:

```text
Narrow repair memories for the corrected hard failures:
- dropped weighted averages
- ratio scaling
- category invoice totals
- seating block constraints
- grid coordinates
- conditional probability
- table joins
- strict YAML status
```

Run:

```text
rag_runs/hard-general-orchestration-20260614-073541-qwen35-9b-base-q8-raw
```

Recipe:

```text
V2 + V4
```

Result:

```text
orchestration_memory:      15/20
orchestration_primitives:  14/20
```

Interpretation:

```text
V4 is a real improvement over V2 on the corrected hard suite.
Plain memory is stronger than prompt-level primitives here.
```

### Routed-Memory Test

The hard and medium harnesses now support routed memory:

```text
HARD_ORCH_ROUTED_MEMORY=1
GENERAL_ORCH_ROUTED_MEMORY=1
```

In routed mode, common memories always load, and task-specific memories are selected by task family.

Hard routed run:

```text
rag_runs/hard-general-orchestration-20260614-073831-qwen35-9b-base-q8-raw
```

Recipe:

```text
V2 + V4, routed
```

Result:

```text
orchestration_memory:      14/20
orchestration_primitives:  14/20
```

Interpretation:

```text
Routing reduced interference in some places but removed useful cross-domain spillover in others.
For this model and recipe, unrouted V2+V4 was better.
```

### V5 Surgical Hard Patch

File:

```text
data/sequential_orchestration_memories_v5_surgical_hard_patch.txt
```

Purpose:

```text
Benchmark-calibration upper-bound test for the five stubborn hard failures:
- weighted_average_drop_lowest
- nested_json_invoice
- multi_constraint_seating
- table_join_filter
- ratio_scaling
```

This file is intentionally surgical and close to the benchmark. It should not be treated as transfer proof.

Run:

```text
rag_runs/hard-general-orchestration-20260614-074221-qwen35-9b-base-q8-raw
```

Recipe:

```text
V2 + V4 + V5
```

Result:

```text
orchestration_memory:      18/20
orchestration_primitives:  17/20
```

Remaining failures:

```text
set_intersection_order
ratio_scaling
```

The ratio answer was actually semantically correct:

```text
50 grams
```

The grader was updated to accept that as valid.

### V6 Order Patch

File:

```text
data/sequential_orchestration_memories_v6_order_patch.txt
```

Purpose:

```text
One scar for preserving List A order in set-intersection tasks.
```

Run:

```text
rag_runs/hard-general-orchestration-20260614-074504-qwen35-9b-base-q8-raw
```

Recipe:

```text
V2 + V4 + V5 + V6
```

Corrected hard result:

```text
orchestration_memory:      20/20
orchestration_primitives:  19/20
```

This is the current hard-suite upper bound.

Important caution:

```text
This is not yet a generalization result.
V5 and V6 are surgical benchmark memories.
The result proves that the base model can be driven to perfect behavior on this hard suite through memory-shaped context, but held-out variants are still required.
```

### Medium Suite Cross-Check

Run:

```text
rag_runs/general-orchestration-20260614-074704-qwen35-9b-base-q8-raw
```

Recipe:

```text
V2 + V4 + V5 + V6
```

Result:

```text
orchestration_memory:      14/15
orchestration_primitives:  11/15
```

The surgical hard recipe preserved the previous medium high-water mark of 14/15.

Remaining medium failure:

```text
logic_schedule
```

### V7 Logic Schedule Patch

File:

```text
data/sequential_orchestration_memories_v7_logic_schedule_patch.txt
```

Purpose:

```text
One surgical scar for Ana/Ben/Carla weekday elimination.
```

Run:

```text
rag_runs/general-orchestration-20260614-074906-qwen35-9b-base-q8-raw
```

Recipe:

```text
V2 + V4 + V5 + V6 + V7
```

Result:

```text
orchestration_memory:      13/15
```

V7 fixed:

```text
logic_schedule
```

But regressed:

```text
rhetoric_fallacy
code_trace
```

Routed medium run:

```text
rag_runs/general-orchestration-20260614-075040-qwen35-9b-base-q8-raw
```

Result:

```text
orchestration_memory:      13/15
```

Routing prevented some leakage but removed useful cross-memory help for other tasks.

## Current Best Checkpoint

Best hard-suite recipe:

```text
V2 + V4 + V5 + V6
plain memory, no primitive wrapper, no routing
```

Hard:

```text
20/20
```

Medium:

```text
14/15
```

Best stable/non-surgical recipe remains:

```text
V2 + V4
```

Corrected hard:

```text
15/20
```

Design lessons:

```text
1. Benchmark correctness matters. Bad expected answers hid real improvement.
2. Plain memory beat primitive wrappers on this base model.
3. Cross-domain spillover sometimes helps; routing is not automatically better.
4. Surgical memories can drive a base model to perfect benchmark behavior.
5. That is an upper bound, not proof of transfer.
6. The next meaningful step is held-out variants for the V2+V4 recipe, then ablations for V5/V6-style surgical scars.
```

## 2026-06-14 Smaller Base / Lower-Quant Sweep

After the 9B Q8 base result, the corrected hard suite was run on available smaller base/raw models.

Recipe:

```text
V2 + V4 + V5 + V6
plain memory
no routing
no primitive wrapper
```

The available clean base/raw models in Ollama were:

```text
qwen35-4b-base-q8-raw
qwen35-4b-base-q6-raw
qwen35-2b-base-q8-raw
qwen35-2b-base-q4km-raw
```

A lower-quant 9B base model was not present in Ollama at this checkpoint. The lower-quant 9B entries were chat/instruct or coder variants, so they were not used for this base-only sweep.

Hard-suite results:

```text
qwen35-9b-base-q8-raw:
- no_memory:  9/20
- memory:    20/20
- delta:     +11

qwen35-4b-base-q8-raw:
- no_memory:  7/20
- memory:     9/20
- delta:      +2

qwen35-4b-base-q6-raw:
- no_memory:  8/20
- memory:     9/20
- delta:      +1

qwen35-2b-base-q8-raw:
- no_memory:  4/20
- memory:     7/20
- delta:      +3

qwen35-2b-base-q4km-raw:
- no_memory:  1/20
- memory:     2/20
- delta:      +1
```

Run directories:

```text
rag_runs/hard-general-orchestration-20260614-075251-qwen35-4b-base-q8-raw
rag_runs/hard-general-orchestration-20260614-075339-qwen35-4b-base-q6-raw
rag_runs/hard-general-orchestration-20260614-075436-qwen35-2b-base-q8-raw
rag_runs/hard-general-orchestration-20260614-075501-qwen35-2b-base-q4km-raw
```

Interpretation:

```text
The memory recipe transfers as a positive effect, but not as a high-score recipe.

The 4B base models improved only modestly:
- Q8: +2
- Q6: +1

The 2B Q8 model showed a larger relative gain:
- 4/20 -> 7/20

The 2B Q4KM model appears too damaged or too small for this recipe:
- 1/20 -> 2/20
```

Important conclusion:

```text
The 9B Q8 result does not automatically scale down.
The smaller models likely need their own scar recipe, shorter memory context, or a training-time LoRA version rather than prompt-side memory injection.
```

Next recipe direction for 4B/2B:

```text
1. Use fewer memories per prompt.
2. Prefer compact, task-family patches over the full surgical stack.
3. Test routed memory again for smaller models, because smaller context/reasoning capacity may be more sensitive to memory crowding.
4. Build 4B-specific scars from its actual failures rather than reusing 9B surgical scars.
5. Treat 2B Q4KM as a quantization-damage case; it may need primitives/training rather than prompt memory.
```

## Strong Neural-Exoskeleton Primitive Profile

The primitive wrapper was strengthened and made selectable through environment variables:

```text
HARD_ORCH_PRIMITIVE_PROFILE=strong
GENERAL_ORCH_PRIMITIVE_PROFILE=strong
```

The strong profile adds these primitive families:

```text
task-lock transport
source-boundary gate
state-register update
unit-anchor normalization
row-line aggregation
constraint-elimination lattice
order-source latch
final-shape canonicalizer
```

The goal was to make the primitives act less like generic advice and more like structural supports for damaged or small models.

Hard-suite strong-primitive sweep:

Recipe:

```text
V2 + V4 + V5 + V6 + strong primitives
plain prompt-side injection
no routing
```

Results:

```text
qwen35-9b-base-q8-raw:
- memory only:        20/20
- strong primitives: 17/20
- delta vs memory:   -3

qwen35-4b-base-q8-raw:
- memory only:         9/20
- strong primitives:  10/20
- delta vs memory:    +1

qwen35-4b-base-q6-raw:
- memory only:         9/20
- strong primitives:  11/20
- delta vs memory:    +2

qwen35-2b-base-q8-raw:
- memory only:         7/20
- strong primitives:   8/20
- delta vs memory:    +1

qwen35-2b-base-q4km-raw:
- memory only:         2/20
- strong primitives:   6/20
- delta vs memory:    +4
```

Run directories:

```text
rag_runs/hard-general-orchestration-20260614-091303-qwen35-9b-base-q8-raw
rag_runs/hard-general-orchestration-20260614-091109-qwen35-4b-base-q8-raw
rag_runs/hard-general-orchestration-20260614-091128-qwen35-4b-base-q6-raw
rag_runs/hard-general-orchestration-20260614-091145-qwen35-2b-base-q8-raw
rag_runs/hard-general-orchestration-20260614-091202-qwen35-2b-base-q4km-raw
```

Interpretation:

```text
Strong primitives hurt the already-clean 9B Q8 recipe.
Strong primitives helped every smaller/damaged model tested.
The biggest relative gain was on 2B Q4KM: 2/20 -> 6/20.
```

Working rule:

```text
Use plain memory for models that can already follow the scar corpus.
Use stronger primitives for smaller or more damaged models where the issue is signal stability, state tracking, and output-shape collapse.
```

This supports the neuro-exoskeleton idea in a narrow but useful way:

```text
The primitive layer is not universally good.
It behaves more like a compensating scaffold for degraded capability.
```

## 2026-06-15 Primitive Recipe Sweep

Additional primitive profiles were added to the hard harness:

```text
HARD_ORCH_PRIMITIVE_PROFILE=compact
HARD_ORCH_PRIMITIVE_PROFILE=rf
HARD_ORCH_PRIMITIVE_PROFILE=compiler
HARD_ORCH_PRIMITIVE_PROFILE=ledger
HARD_ORCH_PRIMITIVE_PROFILE=hybrid
HARD_ORCH_PRIMITIVE_PROFILE=domain_kernel
HARD_ORCH_PRIMITIVE_PROFILE=formula_bank
```

Profile concepts:

```text
compact:
- shorter version of the strong exoskeleton

rf:
- carrier lock
- demodulation
- noise gate
- parity/error correction
- phase alignment

compiler:
- lex
- parse
- typecheck
- execute
- test
- emit

ledger:
- transaction open
- normalize entries
- post lines
- reconcile
- rollback
- commit

hybrid:
- compact mix of RF, compiler, and ledger ideas

domain_kernel:
- finance, probability, chemistry, code trace, spatial, rhetoric, causal circuit, and codec kernels

formula_bank:
- explicit reusable formula/operator table
```

### Routing Result

Strong primitives were retested with routed memory:

```text
HARD_ORCH_ROUTED_MEMORY=1
HARD_ORCH_PRIMITIVE_PROFILE=strong
```

Results:

```text
qwen35-4b-base-q8-raw:
- strong unrouted: 10/20
- strong routed:   12/20

qwen35-4b-base-q6-raw:
- strong unrouted: 11/20
- strong routed:   12/20

qwen35-2b-base-q8-raw:
- strong unrouted:  8/20
- strong routed:    6/20

qwen35-2b-base-q4km-raw:
- strong unrouted:  6/20
- strong routed:    8/20
```

Interpretation:

```text
Routing helps 4B and the heavily quantized 2B Q4KM.
Routing hurts 2B Q8.
This is model/quant-sensitive; routing is not a universal win.
```

### Cross-Domain Profile Sweep

Routed memory plus the new profiles was tested on the two most interesting targets:

```text
qwen35-4b-base-q6-raw
qwen35-2b-base-q4km-raw
```

Results:

```text
qwen35-4b-base-q6-raw:
- strong routed:       12/20
- compact routed:      11/20
- rf routed:           10/20
- compiler routed:     10/20
- ledger routed:        9/20
- hybrid routed:       10/20
- domain_kernel routed:11/20
- formula_bank routed: 11/20

qwen35-2b-base-q4km-raw:
- strong routed:        8/20
- compact routed:       9/20
- rf routed:            8/20
- compiler routed:      8/20
- ledger routed:        9/20
- hybrid routed:        9/20
- domain_kernel routed: 8/20
- formula_bank routed:  9/20
```

Best current primitive recipes:

```text
4B base Q8:
- strong routed: 12/20

4B base Q6:
- strong routed: 12/20

2B base Q8:
- strong unrouted: 8/20

2B base Q4KM:
- compact/ledger/hybrid/formula_bank routed: 9/20
```

### Primitive-Only Test

Primitive-only runs were tested by setting:

```text
HARD_ORCH_MEMORY_PATH=
```

Results were poor:

```text
qwen35-4b-base-q6-raw:
- strong only:        4/20
- compact only:       4/20
- hybrid only:        3/20
- domain_kernel only: 5/20

qwen35-2b-base-q4km-raw:
- strong only:        3/20
- compact only:       5/20
- hybrid only:        5/20
- domain_kernel only: 3/20
```

Interpretation:

```text
Primitives alone are not enough.
The scars/memories still carry most of the capability.
The primitive layer acts as a scaffold over memory, not as a replacement for memory.
```

### Transport Test

Ollama chat transport was tested on the 4B base models:

```text
HARD_ORCH_API=chat
HARD_ORCH_ROUTED_MEMORY=1
HARD_ORCH_PRIMITIVE_PROFILE=strong
```

Results:

```text
qwen35-4b-base-q8-raw:
- generate strong routed: 12/20
- chat strong routed:     10/20

qwen35-4b-base-q6-raw:
- generate strong routed: 12/20
- chat strong routed:     10/20
```

Interpretation:

```text
Raw generate remains the better transport for these base models.
Chat transport appears to add formatting/instruction pressure that does not help this experiment.
```

### Current Primitive Conclusion

The best primitive shape depends on model damage:

```text
9B Q8:
- no primitives; memory only

4B Q8/Q6:
- strong routed primitives

2B Q8:
- strong unrouted primitives

2B Q4KM:
- compact/ledger/hybrid/formula_bank routed primitives
```

The hard ceiling for 4B/2B did not move past 12/20 and 9/20 with prompt-side recipes.

Working hypothesis:

```text
For smaller models, prompt-side primitives improve output shape and some state behavior,
but they cannot reliably restore computation kernels such as multi-step arithmetic,
probability, code tracing, chemistry, and causal counterfactuals.
```

Next likely path:

```text
Train the primitives/memories into a LoRA for the 4B model instead of relying on prompt-side injection.
Prompt-side recipes identify useful structures, but training may be needed for them to become stable behavior.
```
