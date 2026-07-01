# Live LoRA Prompt Matrix

This is a real Transformers/PEFT benchmark, not a simulation.

Run folder:

```text
runs/live-lora-prompt-matrix
```

Script:

```text
scripts/run_live_lora_prompt_matrix.py
```

## Scope

The saved structural primitive LoRA is trained for:

```text
Qwen/Qwen3-1.7B-Base
```

It is not a Qwen3.5-2B adapter, so this benchmark uses the matching 1.7B base model. The earlier 2B Qwen3.5 Ollama benchmark remains useful as a no-LoRA prompt-memory baseline, but the LoRA matrix has to run on the adapter's actual base model.

The local PEFT version was older than the adapter config. The run used a temporary sanitized adapter config that removed unsupported metadata keys while preserving the same adapter weights. The sanitized config and removed-key list are saved in the run folder.

## Conditions

Each setup was tested on the same eight live tasks used by the 2B prompt benchmark:

- no memory
- structural only
- cognitive only
- cyber only
- routed combination
- naive all-fire

Two model setups were tested:

- `base_no_lora`
- `base_plus_structural_lora`

## Raw Summary

Raw scoring uses the whole generated output. This is useful for detecting runaway continuation, but it can also inflate pass rate when the model generates later training examples that contain the right words.

| Setup | Condition | Pass | Clean Pass | Mean Score | Echo | Second Task |
|---|---|---:|---:|---:|---:|---:|
| base no LoRA | no memory | 0.750 | 0.750 | 0.826 | 0.000 | 0.000 |
| base no LoRA | structural only | 0.625 | 0.625 | 0.792 | 0.000 | 0.125 |
| base no LoRA | cognitive only | 0.625 | 0.625 | 0.745 | 0.000 | 0.125 |
| base no LoRA | cyber only | 0.750 | 0.625 | 0.767 | 0.000 | 0.250 |
| base no LoRA | routed combination | 0.750 | 0.750 | 0.709 | 0.000 | 0.000 |
| base no LoRA | naive all-fire | 0.750 | 0.750 | 0.801 | 0.000 | 0.000 |
| LoRA | no memory | 0.875 | 0.125 | 0.850 | 0.250 | 0.875 |
| LoRA | structural only | 0.750 | 0.250 | 0.831 | 0.250 | 0.500 |
| LoRA | cognitive only | 0.750 | 0.125 | 0.762 | 0.625 | 0.375 |
| LoRA | cyber only | 1.000 | 0.250 | 0.841 | 0.000 | 0.750 |
| LoRA | routed combination | 0.750 | 0.250 | 0.762 | 0.625 | 0.500 |
| LoRA | naive all-fire | 0.750 | 0.125 | 0.772 | 0.250 | 0.500 |

Raw result: the LoRA clearly learned useful answer structure, but it also learned too much transcript continuation. This is why raw clean pass is low.

## Strict First-Answer Summary

Strict scoring trims to the first answer segment and adds a harmful-first-answer check for source-boundary failures. This better reflects the answer a deployed system would keep if it used stop sequences or first-answer extraction.

| Setup | Condition | Pass | Clean Pass | Mean Score | Harmful First Answer |
|---|---|---:|---:|---:|---:|
| base no LoRA | no memory | 0.750 | 0.750 | 0.785 | 0.125 |
| base no LoRA | structural only | 0.625 | 0.625 | 0.792 | 0.000 |
| base no LoRA | cognitive only | 0.625 | 0.625 | 0.704 | 0.125 |
| base no LoRA | cyber only | 0.625 | 0.625 | 0.704 | 0.125 |
| base no LoRA | routed combination | 0.625 | 0.625 | 0.628 | 0.250 |
| base no LoRA | naive all-fire | 0.625 | 0.625 | 0.720 | 0.250 |
| LoRA | no memory | 0.875 | 0.625 | 0.875 | 0.125 |
| LoRA | structural only | 0.750 | 0.625 | 0.894 | 0.000 |
| LoRA | cognitive only | 1.000 | 0.625 | 0.894 | 0.000 |
| LoRA | cyber only | 0.750 | 0.375 | 0.728 | 0.250 |
| LoRA | routed combination | 0.875 | 0.875 | 0.919 | 0.000 |
| LoRA | naive all-fire | 0.750 | 0.375 | 0.834 | 0.000 |

Strict result: `base_plus_structural_lora + routed_combination` is the best setup in this run.

```text
base no LoRA, no memory:          pass 0.750, clean 0.750
base plus LoRA, no memory:        pass 0.875, clean 0.625
base plus LoRA, routed memory:    pass 0.875, clean 0.875
```

## Interpretation

The LoRA did work. It raised first-answer task behavior and made the routed memory context more effective than prompt memories alone.

The caveat is also real: the LoRA has learned structural format habits too strongly. Without stop handling, it often continues into training-style examples, helper preambles, or repeated task frames. That is not a theoretical failure of the memory approach, but it is a dataset/training-format problem.

The next dataset revision should:

1. Remove repeated `### User`, `### Assistant`, `Teacher`, and `Student` transcript patterns from training completions.
2. Keep structural primitives in the behavior, but reduce visible words like `Frame`, `Carrier`, `Checksum`, and `Suppression`.
3. Add stronger one-answer stop memories and examples.
4. Add negative examples where continuing into another task is treated as failure.
5. Keep routed memory association, because routed plus LoRA was the best strict condition.
