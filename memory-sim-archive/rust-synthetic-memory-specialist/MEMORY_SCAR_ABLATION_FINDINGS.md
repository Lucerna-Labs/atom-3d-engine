# Memory Scar Ablation Findings

Date: 2026-06-11

## Purpose

This test corrects an important drift in the experiment.

Before training another LoRA, we need to keep testing locally:

```text
How many scar memories are needed?
Which scar types help?
Which scar types interfere?
What happens when multiple scar sets are stacked?
```

Runtime routing and LoRA dataset generation are useful, but they do not answer this scar-composition question by themselves.

## New Local Ablation Harness

File:

```text
scripts/run_memory_scar_ablation.py
```

The harness tests memory conditions locally against the existing executable verifiers.

Conditions:

```text
no_memory
common_one
common_all
task_one
task_three
task_all
common_one_task_one
common_all_task_one
common_all_task_three
common_all_task_all
routed_current
cross_domain_noise
```

Meaning:

```text
task_one / task_three / task_all
- only task-specific scars

common_one / common_all
- only shared artifact-contract scars

common_*_task_*
- mixed common + task-specific scars

routed_current
- current full routed memory pathway from the benchmark harness

cross_domain_noise
- active domain scars plus all scars from the other domain
```

## Python Ablation

Model:

```text
qwen35-4b-q4km-chat
```

Run:

```text
rag_runs/memory-scar-ablation-20260611-184738
```

Tasks:

```text
flatten_json
deep_merge
```

Summary:

```text
no_memory:               1/2
task_one:                0/2
task_three:              1/2
task_all:                2/2
common_all_task_one:     1/2
common_all_task_three:   0/2
common_all_task_all:     2/2
routed_current:          2/2
cross_domain_noise:      2/2
```

Detailed read:

```text
flatten_json:
- no_memory passed at builder in this run
- task_one failed
- task_three passed
- task_all passed
- common_all_task_one failed
- common_all_task_three failed
- common_all_task_all passed
- cross_domain_noise passed after repair_1

deep_merge:
- no_memory failed
- task_one failed
- task_three failed
- task_all passed
- common_all_task_one passed
- common_all_task_three failed
- common_all_task_all passed
- cross_domain_noise passed at builder
```

Interpretation:

```text
One scar is not enough.
Three scars are not reliably enough.
The full task-scar set worked.
The full routed set worked.
Partial common+task mixes can make behavior worse.
Cross-domain noise did not collapse Python, but it moved flatten_json from builder to repair_1.
```

Python lesson:

```text
The model seems to need the complete local failure surface for hard recursive tasks.
Stable skeleton scars matter more than isolated warning scars.
Common scars should be included carefully; partial common/task mixtures can create wrong emphasis.
```

## Ordo Ramp Ablation

Model:

```text
qwen35-4b-q4km-chat
```

Run:

```text
rag_runs/memory-scar-ablation-20260611-184943
```

Task:

```text
ramp_roundtrip
```

Summary:

```text
no_memory:               0/1
task_one:                0/1
task_three:              0/1
task_all:                1/1 builder
common_all_task_one:     0/1
common_all_task_three:   0/1
common_all_task_all:     0/1
routed_current:          1/1 repair_2
cross_domain_noise:      0/1
```

Memory size notes:

```text
task_one:                ~1 scar,    572 chars
task_three:              ~3 scars,   1,431 chars
task_all:                ~17 scars,  8,186 chars
common_all_task_all:     ~24 scars,  10,288 chars
routed_current:          ~36 blocks, 13,778 chars
cross_domain_noise:      ~51 blocks, 19,634 chars
```

Interpretation:

```text
For ramp_roundtrip, the pure task-specific scar set was best.
All 17 ramp scars passed at builder stage.
One scar and three scars did nothing.
Adding common scars back in broke the task, even with all ramp scars present.
The current routed memory passed, but only after repair_2, likely because it includes larger artifact memory beyond the scar text file.
Cross-domain noise failed.
```

Ordo lesson:

```text
More memory is not automatically better.
The right task-local scars can outperform a larger mixed prompt.
Common memories can interfere with a brittle task if they add competing abstractions.
For some domains, the router should select task scars more aggressively and include common scars only when the task has shown it needs them.
```

## Updated Working Rule

The earlier rule was:

```text
common contract + task scars
```

The ablation result makes that too simple.

Better rule:

```text
1. Start with task scars only.
2. Add common scars only if they improve the specific task.
3. Prefer full local failure-surface coverage over one or two generic reminders.
4. Treat cross-domain memories as opt-in, not default.
5. Measure builder-stage pass rate, not only final repair-stage pass rate.
```

## Why This Matters For LoRA

The LoRA dataset should not blindly include every common memory with every task.

For training, the next dataset should separate:

```text
task_scar_only examples
common_plus_task examples
interference_guard examples
router_selection examples
general_guard examples
```

The model should learn that domain routing is not only Python versus Ordo. It is also:

```text
which scar subset belongs to this task?
```

This is a smaller router inside the larger router.

## Current Local Conclusion

The local tests now show more than one scar condition, not just full-memory versus no-memory.

Concrete findings:

```text
Python hard tasks:
- no_memory: 1/2
- task_one: 0/2
- task_three: 1/2
- task_all: 2/2
- routed_current: 2/2

Ordo ramp:
- no_memory: 0/1
- task_one: 0/1
- task_three: 0/1
- task_all: 1/1 builder
- common_all_task_all: 0/1
- routed_current: 1/1 repair_2
```

The strongest local signal is:

```text
Scar sets have thresholds.
Below the threshold, they may do nothing or hurt.
At the right density, they can snap the model into the correct artifact shape.
Too much unrelated or poorly mixed memory can interfere again.
```
