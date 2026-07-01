# Findings: Live Memory Prompting and Structural LoRA

Date: 2026-06-06

This note summarizes the live model findings from the prompt-memory benchmark and the structural primitive LoRA matrix. These were real model runs, not simulations.

## Runs Covered

Prompt-memory benchmark:

```text
runs/live-memory-prompt-benchmark
docs/LIVE_MEMORY_PROMPT_BENCHMARK.md
scripts/run_live_memory_prompt_benchmark.py
```

LoRA prompt matrix:

```text
runs/live-lora-prompt-matrix
docs/LIVE_LORA_PROMPT_MATRIX.md
scripts/run_live_lora_prompt_matrix.py
```

## Finding 1: Prompt Memories Can Move Live Model Behavior

The 2B prompt-memory benchmark tested:

- `qwen35-2b-base-q8-raw:latest`
- `qwen35-2b-base-q4km-raw:latest`
- `qwen3.5:2b-q8_0`

Each model was tested with no memory, individual memory families, routed memory, and naive all-fire memory.

The clearest prompt-memory gain appeared on the compressed base model:

| Model | Condition | Pass | Clean Pass |
|---|---|---:|---:|
| 2B base Q4_K_M raw | no memory | 0.750 | 0.625 |
| 2B base Q4_K_M raw | routed combination | 1.000 | 0.875 |

This supports the working theory that weaker or compressed models benefit more from externalized memory structure than stronger models.

## Finding 2: Instruct Models Still Benefit, But Less

The instruct-style Qwen3.5 2B model needed top-level `"think": false` in the Ollama request. Without that, it generated internal thinking but returned blank visible responses.

With thinking disabled:

| Model | Condition | Pass | Clean Pass |
|---|---|---:|---:|
| 2B instruct Q8 | no memory | 0.750 | 0.750 |
| 2B instruct Q8 | cognitive only | 0.875 | 0.875 |
| 2B instruct Q8 | cyber only | 0.875 | 0.875 |
| 2B instruct Q8 | routed combination | 0.875 | 0.875 |

The gain exists, but it is modest. This fits the theory: instruction tuning already supplies some task-stability behavior, so prompt memories add less than they do for a weaker base model.

## Finding 3: The Existing LoRA Is Real, But It Belongs To Qwen3 1.7B

The saved structural primitive LoRA is trained for:

```text
Qwen/Qwen3-1.7B-Base
```

It should not be evaluated as a Qwen3.5-2B adapter. The live LoRA matrix used the matching 1.7B base model.

The local PEFT package was older than the adapter config. A temporary sanitized adapter config was used so the same adapter weights could load locally. The sanitized files are saved in:

```text
runs/live-lora-prompt-matrix/sanitized_adapter_config.json
runs/live-lora-prompt-matrix/adapter_config_removed_keys.json
```

## Finding 4: LoRA Plus Routed Memories Was The Best Clean Setup

Strict first-answer scoring was used because raw LoRA outputs sometimes continued into training-format examples. Strict scoring trims to the first answer segment and checks whether the first answer itself is harmful.

Best comparison:

| Setup | Condition | Pass | Clean Pass | Mean Score |
|---|---|---:|---:|---:|
| base no LoRA | no memory | 0.750 | 0.750 | 0.785 |
| base plus structural LoRA | no memory | 0.875 | 0.625 | 0.875 |
| base plus structural LoRA | routed combination | 0.875 | 0.875 | 0.919 |

Interpretation:

- LoRA alone improved task behavior but hurt clean output because of continuation/template residue.
- Routed prompt memory helped stabilize the LoRA.
- The best overall setup was not LoRA-only or prompt-only; it was LoRA plus routed memory context.

## Finding 5: The Current LoRA Learned Too Much Format

The LoRA learned useful structural behavior, but it also learned visible training patterns. Raw outputs sometimes continued into:

- `### User`
- `### Assistant`
- `Teacher`
- `Student`
- repeated final-answer frames
- visible primitive labels such as `Frame`, `Carrier`, `Checksum`, and `Suppression`

This is not evidence that the LoRA failed. It is evidence that the dataset taught both the desired behavior and too much of the transcript wrapper.

The next training dataset should make the desired behavior internal and reduce visible scaffolding.

## Finding 6: Naive All-Fire Is Risky

Naive all-fire sometimes raises pass rate, but it also raises contamination risk. It can cause the model to mention memory labels, drift into second tasks, or keep producing examples.

Routed memory is more promising because it selectively activates memory families:

- cognitive memories for problem shape
- cyber memories for source-boundary/injection tasks
- structural primitives for task framing, checking, and stopping

## Working Conclusion

The approach is still alive.

The live runs support these claims:

1. Synthetic memory context can change live model behavior.
2. Compressed or weaker base models show the clearest prompt-memory gains.
3. Instruct models can benefit, but gains are smaller unless the instruct model is degraded or weak.
4. A structural primitive LoRA can improve task behavior.
5. LoRA plus routed memories performed better than either base no-memory or LoRA-only in the strict first-answer score.
6. The hard part is dataset surgery: memories must encode behavior without teaching noisy transcript continuation.

## Next Dataset Revision

The next revision should:

1. Remove or sharply reduce `### User`, `### Assistant`, `Teacher`, and `Student` training transcript markers.
2. Replace visible primitive labels with natural first-person or task-native language.
3. Add negative examples where continuing after the answer is treated as failure.
4. Add clean-stop memories: answer once, verify against the task, stop.
5. Keep routed association, because routed plus LoRA was the best strict condition.
6. Add more held-out tasks so the benchmark is broader than the current eight-probe smoke test.

## Current Best Result

The strongest documented live setup from these runs is:

```text
Qwen/Qwen3-1.7B-Base
+ structural-primitive-lora-v0-2
+ routed memory context
```

Strict first-answer result:

```text
pass:       0.875
clean pass: 0.875
mean score: 0.919
```

Baseline:

```text
Qwen/Qwen3-1.7B-Base
no LoRA
no memory

pass:       0.750
clean pass: 0.750
mean score: 0.785
```
