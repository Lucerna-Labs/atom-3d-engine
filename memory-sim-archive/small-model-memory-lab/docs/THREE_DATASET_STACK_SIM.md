# Three-Dataset Stack Simulation

Run folder:

```text
runs/three-dataset-stack-sim
```

Script:

```text
scripts/simulate_three_dataset_stack.py
```

This is a toy design simulation, not a model result. It tests how three active dataset families might interact before we spend Kaggle GPU time on another LoRA.

- cognitive/generalist memories
- cyber/source-boundary memories
- structural primitive memories

The simulation compares single packs, pairwise stacks, a balanced all-three stack, and a naive all-fire all-three stack.

## Dataset Packs

| Pack | Source files | Role |
|---|---|---|
| cognitive | `cognitive_core_v0_1.json`, `cognitive_rewards_v0_1.json`, `computation_primitives_v0_1.json` | broad recognition, task orientation, math, rhetoric, spatial reasoning, rewards |
| cyber | `rust_cyber_defender_generated.tsv` | source-boundary specialization, prompt-injection recognition, danger checks |
| structural | `structural_primitives_lora_v0_1.json` | frame, gate, carrier, checksum, ECC, gain clamp, clean stop |

## Headline

The balanced all-three stack helps most when the model is damaged, especially Q2-style damage. It is not automatically best on healthier models.

The naive all-fire all-three stack is bad. It often raises pass rate over baseline, but it creates much more echo and overhead, crushing clean-pass rate. This matches the earlier memory/RAG finding: more memories are not automatically better. Routing and compression matter.

## Best Stack Per Context

| Substrate | Quant state | Best stack | Clean pass | Pass | Mean score | Echo | Cascade |
|---|---|---|---:|---:|---:|---:|---:|
| base | damaged Q2 | all three balanced | 0.460 | 0.683 | 0.850 | 0.133 | 0.142 |
| base | damaged Q3 | cognitive + structural | 0.650 | 0.871 | 0.941 | 0.063 | 0.100 |
| base | healthy Q6 | structural only | 0.793 | 0.928 | 0.966 | 0.044 | 0.049 |
| degraded instruct | damaged Q2 | all three balanced | 0.332 | 0.512 | 0.753 | 0.140 | 0.164 |
| degraded instruct | damaged Q3 | cognitive + structural | 0.632 | 0.849 | 0.929 | 0.067 | 0.107 |
| degraded instruct | healthy Q6 | structural only | 0.789 | 0.921 | 0.964 | 0.048 | 0.053 |
| instruct | damaged Q2 | all three balanced | 0.197 | 0.302 | 0.605 | 0.127 | 0.210 |
| instruct | damaged Q3 | all three balanced | 0.523 | 0.765 | 0.884 | 0.126 | 0.118 |
| instruct | healthy Q6 | structural only | 0.791 | 0.924 | 0.965 | 0.044 | 0.049 |

## All-Three Result

Balanced all-three:

| Context | Clean pass | Pass | Mean score | Echo | Cascade |
|---|---:|---:|---:|---:|---:|
| base / damaged Q2 | 0.460 | 0.683 | 0.850 | 0.133 | 0.142 |
| base / damaged Q3 | 0.630 | 0.880 | 0.946 | 0.131 | 0.091 |
| base / healthy Q6 | 0.713 | 0.924 | 0.964 | 0.131 | 0.048 |
| degraded instruct / damaged Q2 | 0.332 | 0.512 | 0.753 | 0.140 | 0.164 |
| degraded instruct / damaged Q3 | 0.595 | 0.876 | 0.942 | 0.141 | 0.094 |
| degraded instruct / healthy Q6 | 0.684 | 0.920 | 0.962 | 0.146 | 0.055 |
| instruct / damaged Q2 | 0.197 | 0.302 | 0.605 | 0.127 | 0.210 |
| instruct / damaged Q3 | 0.523 | 0.765 | 0.884 | 0.126 | 0.118 |
| instruct / healthy Q6 | 0.666 | 0.902 | 0.954 | 0.131 | 0.070 |

Naive all-fire all-three:

| Context | Clean pass | Pass | Mean score | Echo | Cascade |
|---|---:|---:|---:|---:|---:|
| base / damaged Q2 | 0.092 | 0.170 | 0.476 | 0.288 | 0.254 |
| base / damaged Q3 | 0.330 | 0.570 | 0.780 | 0.288 | 0.143 |
| base / healthy Q6 | 0.530 | 0.890 | 0.928 | 0.286 | 0.075 |
| degraded instruct / damaged Q2 | 0.055 | 0.105 | 0.377 | 0.303 | 0.286 |
| degraded instruct / damaged Q3 | 0.240 | 0.426 | 0.682 | 0.303 | 0.162 |
| degraded instruct / healthy Q6 | 0.499 | 0.867 | 0.917 | 0.305 | 0.092 |
| instruct / damaged Q2 | 0.023 | 0.040 | 0.247 | 0.289 | 0.352 |
| instruct / damaged Q3 | 0.151 | 0.259 | 0.556 | 0.292 | 0.186 |
| instruct / healthy Q6 | 0.458 | 0.785 | 0.871 | 0.288 | 0.098 |

## Interpretation

All three datasets should not be dumped into training with equal visibility.

The useful pattern is:

```text
structural primitives as the control substrate
cognitive memories as broad recognition and domain transfer
cyber memories as targeted source-boundary specialization
```

The risky pattern is:

```text
all memories fire all the time
```

That raises echo pressure and distractor load. It may improve raw pass rate on damaged models, but it harms clean behavior.

## Recommended Next Dataset

For the next real LoRA training run, build a routed mixed dataset instead of a flat merge.

Suggested starting mix:

| Component | Approximate share | Purpose |
|---|---:|---|
| structural primitives | 50-60% | keep task shape, frame, gate, checksum, repair, clean stop |
| cognitive/generalist memories | 20-30% | broaden recognition across math, rhetoric, spatial, source, evidence |
| cyber/source-boundary memories | 10-20% | preserve prompt-injection and authority-boundary specialization |

Training examples should avoid making all three corpora visible at once. Better forms:

- structural primitive task examples
- cognitive memory recall examples only when relevant
- cyber source-boundary examples only for adversarial/source tasks
- mixed examples where one cognitive memory is organized by one or two structural primitives
- mixed examples where one cyber memory is stabilized by gate, carrier, checksum, and clean stop

The next experiment should compare:

1. structural-only LoRA
2. balanced routed three-dataset LoRA
3. naive all-three flat LoRA

The prediction is that the routed three-dataset LoRA should help degraded/compressed models more than structural-only while avoiding the echo collapse of naive all-fire training.
