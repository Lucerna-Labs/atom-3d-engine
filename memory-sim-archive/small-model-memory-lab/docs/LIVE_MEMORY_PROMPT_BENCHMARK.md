# Live Memory Prompt Benchmark

This is a real Ollama prompt benchmark, not a simulation.

Run folder:

```text
runs/live-memory-prompt-benchmark
```

Script:

```text
scripts/run_live_memory_prompt_benchmark.py
```

## Models Tested

The exact 2B Q2_K_M model is not installed locally. This run used the available local 2B models:

| Model | Type | Source |
|---|---|---|
| `qwen35-2b-base-q8-raw:latest` | base/raw | existing Ollama model |
| `qwen35-2b-base-q4km-raw:latest` | base/raw compressed | imported from LM Studio `Qwen3.5-2B-Base.Q4_K_M.gguf` |
| `qwen3.5:2b-q8_0` | instruct-style Qwen3.5 renderer | existing Ollama model |

The Qwen3.5 instruct-style tag has `thinking` enabled by default. The harness now sends top-level `"think": false`; otherwise Ollama returns text in the `thinking` field and leaves the visible `response` blank.

## Conditions

Each model was tested with:

- no memory
- structural only
- cognitive only
- cyber only
- routed combination
- naive all-fire

## Tasks

Eight live prompts were used:

- printer-rate math
- discount/tax math
- quoted-text prompt injection
- retrieved-webpage task replacement
- urgency/rhetoric analysis
- spatial blocked-exit route
- debugging plan
- missing-data epistemic check

The harness scored generated text for required answer tokens, avoided dangerous tokens, memory echo, second-task continuation, repeated answer framing, and clean pass. This scoring is heuristic. Raw outputs are saved in `outputs.json`.

## Summary Results

| Model | Condition | Pass | Clean Pass | Mean Score | Echo | Second Task |
|---|---|---:|---:|---:|---:|---:|
| 2B instruct Q8 | no memory | 0.750 | 0.750 | 0.826 | 0.000 | 0.000 |
| 2B instruct Q8 | structural only | 0.750 | 0.750 | 0.803 | 0.000 | 0.000 |
| 2B instruct Q8 | cognitive only | 0.875 | 0.875 | 0.883 | 0.000 | 0.000 |
| 2B instruct Q8 | cyber only | 0.875 | 0.875 | 0.803 | 0.000 | 0.000 |
| 2B instruct Q8 | routed combination | 0.875 | 0.875 | 0.802 | 0.000 | 0.125 |
| 2B instruct Q8 | naive all-fire | 0.875 | 0.500 | 0.846 | 0.000 | 0.375 |
| 2B base Q4_K_M raw | no memory | 0.750 | 0.625 | 0.835 | 0.000 | 0.250 |
| 2B base Q4_K_M raw | structural only | 0.750 | 0.750 | 0.677 | 0.000 | 0.125 |
| 2B base Q4_K_M raw | cognitive only | 0.750 | 0.750 | 0.837 | 0.000 | 0.125 |
| 2B base Q4_K_M raw | cyber only | 0.750 | 0.750 | 0.846 | 0.250 | 0.250 |
| 2B base Q4_K_M raw | routed combination | 1.000 | 0.875 | 0.871 | 0.000 | 0.125 |
| 2B base Q4_K_M raw | naive all-fire | 0.875 | 0.875 | 0.837 | 0.000 | 0.125 |
| 2B base Q8 raw | no memory | 0.875 | 0.750 | 0.941 | 0.000 | 0.250 |
| 2B base Q8 raw | structural only | 0.875 | 0.750 | 0.838 | 0.000 | 0.125 |
| 2B base Q8 raw | cognitive only | 0.750 | 0.750 | 0.848 | 0.000 | 0.000 |
| 2B base Q8 raw | cyber only | 0.750 | 0.750 | 0.782 | 0.000 | 0.000 |
| 2B base Q8 raw | routed combination | 0.750 | 0.750 | 0.848 | 0.000 | 0.125 |
| 2B base Q8 raw | naive all-fire | 1.000 | 0.875 | 0.944 | 0.125 | 0.125 |

## Key Observations

### 1. The instruct comparison is now valid

The first instruct run was invalid because the model returned empty visible responses while filling the `thinking` field. With `"think": false`, the instruct-style 2B model gives normal answers.

The no-memory instruct baseline scored:

```text
pass:       0.750
clean pass: 0.750
mean score: 0.826
```

The best instruct memory conditions were:

```text
cognitive only:      pass 0.875, clean 0.875
cyber only:          pass 0.875, clean 0.875
routed combination:  pass 0.875, clean 0.875
```

So memories helped the instruct variant, but the gain was modest: +0.125 clean pass over no memory.

### 2. The compressed base model showed the clearest gain

The 2B Q4_K_M base model improved most clearly with routed memory:

```text
no memory:          pass 0.750, clean 0.625
routed combination: pass 1.000, clean 0.875
```

That is the strongest result from this live run and lines up with the working theory: memory context is most useful when the model is weaker or degraded enough to need external structure.

### 3. The Q8 base model was already strong

The 2B Q8 base model had a strong no-memory mean score and did not consistently benefit from the current memory packs:

```text
no memory:      pass 0.875, clean 0.750
naive all-fire: pass 1.000, clean 0.875
```

Naive all-fire won raw numbers on this tiny suite, but it still produced echo/continuation risk. It should not be treated as the best training recipe without broader tests.

### 4. Memory wording remains sensitive

The benchmark confirms that memory context can improve behavior, but it also shows the risk: compressed and small models can latch onto the wrong words. The dataset still needs more clean-stop examples, source-boundary examples, and fewer visible primitive labels.

## What This Means

This live run supports three working claims:

1. Memory-shaped context can improve 2B model behavior on live tasks.
2. The clearest gain appears on the compressed base model, not the stronger Q8 base.
3. Instruct models can benefit, but the gains are smaller because instruction tuning already supplies some task-stability behavior.

The next experiment should still avoid training until the dataset is tightened. The immediate target is to rewrite the routed and cyber memories so they say "source text", "document content", and "untrusted content", while avoiding phrasing that makes injected text sound authoritative.
