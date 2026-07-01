# Dataset Family Ablation

This simulation tests the dataset families individually and then in combinations.

Run folder:

```text
runs/dataset-family-ablation
```

Script:

```text
scripts/simulate_dataset_family_ablation.py
```

This is still a simulation, not a model run. It is meant to record what happens when cognitive, cyber, and structural datasets are tested separately, pairwise, and together across model profiles.

## Coverage

The sweep includes:

- `2b_reference`
- `4b`
- `9b`
- `27b`
- `35b_a3b_moe`

Quant states include:

- `base_f16`
- `q8_k_m`
- `healthy_q8`
- `healthy_q6`
- `damaged_q4`
- `damaged_q3`
- `damaged_q2`
- `q1_k_m`
- `collapsed_q2_edge`

Stacks tested:

- baseline
- cognitive only
- cyber only
- structural only
- cognitive + cyber
- cognitive + structural
- cyber + structural
- structural-heavy pairs
- routed three-family stacks
- naive all-fire three-family stack

Total simulated rows:

```text
9,720,000
```

## Output Files

| File | Purpose |
|---|---|
| `summary.csv` | all aggregate outcomes by model/substrate/quant/stack |
| `family_summary.csv` | task-family breakdowns |
| `pressure_summary.csv` | pressure-band breakdowns |
| `best_by_context.csv` | best stack for each model/substrate/quant context |
| `lift.csv` | lift over baseline |
| `report.json` | machine-readable metadata |

## 9B Base Ablation

### 9B Base F16

| Stack | Stability | Clean | Pass | Echo | Cascade |
|---|---:|---:|---:|---:|---:|
| baseline | 0.741 | 0.774 | 0.867 | 0.013 | 0.020 |
| cognitive only | 0.789 | 0.817 | 0.960 | 0.050 | 0.021 |
| cyber only | 0.763 | 0.796 | 0.957 | 0.075 | 0.020 |
| structural only | 0.835 | 0.855 | 0.968 | 0.017 | 0.019 |
| cognitive 25 / structural 75 | 0.829 | 0.850 | 0.965 | 0.023 | 0.018 |
| cyber 25 / structural 75 | 0.818 | 0.841 | 0.967 | 0.032 | 0.018 |
| routed 72/18/10 | 0.817 | 0.839 | 0.967 | 0.036 | 0.015 |
| naive all-fire | 0.444 | 0.540 | 0.923 | 0.322 | 0.049 |

For 9B f16, structural only is already very strong. Extra memories can improve raw pass, but often add echo.

### 9B Q8_K_M

| Stack | Stability | Clean | Pass | Echo | Cascade |
|---|---:|---:|---:|---:|---:|
| baseline | 0.686 | 0.727 | 0.819 | 0.010 | 0.033 |
| cognitive only | 0.769 | 0.800 | 0.954 | 0.043 | 0.030 |
| cyber only | 0.727 | 0.765 | 0.959 | 0.079 | 0.027 |
| structural only | 0.809 | 0.832 | 0.962 | 0.020 | 0.023 |
| cognitive 25 / structural 75 | 0.796 | 0.821 | 0.953 | 0.019 | 0.029 |
| cyber 25 / structural 75 | 0.788 | 0.815 | 0.958 | 0.031 | 0.027 |
| routed 72/18/10 | 0.796 | 0.824 | 0.960 | 0.042 | 0.028 |
| naive all-fire | 0.427 | 0.525 | 0.912 | 0.321 | 0.053 |

For 9B Q8_K_M, structural only is still the cleanest individual dataset and the best overall ablation winner. The pair/triple mixtures remain useful, but the extra memory pressure is not free.

### 9B Damaged Q2

| Stack | Stability | Clean | Pass | Echo | Cascade |
|---|---:|---:|---:|---:|---:|
| baseline | -0.341 | 0.013 | 0.016 | 0.012 | 0.430 |
| cognitive only | -0.219 | 0.065 | 0.083 | 0.044 | 0.347 |
| cyber only | -0.172 | 0.088 | 0.124 | 0.078 | 0.309 |
| structural only | 0.093 | 0.257 | 0.343 | 0.014 | 0.206 |
| cognitive 25 / structural 75 | 0.197 | 0.339 | 0.459 | 0.020 | 0.188 |
| cyber 25 / structural 75 | 0.236 | 0.370 | 0.502 | 0.029 | 0.180 |
| routed 72/18/10 | 0.394 | 0.493 | 0.717 | 0.038 | 0.149 |
| tri stability 70 | 0.400 | 0.498 | 0.716 | 0.041 | 0.136 |
| naive all-fire | 0.108 | 0.286 | 0.572 | 0.317 | 0.165 |

This is the strongest proof of why combinations matter. On damaged Q2, individual cognitive and cyber barely move the model. Structural helps. Pairing helps more. The routed three-family stack is the large jump.

### 9B Damaged Q3

| Stack | Stability | Clean | Pass | Echo | Cascade |
|---|---:|---:|---:|---:|---:|
| baseline | -0.070 | 0.132 | 0.153 | 0.011 | 0.214 |
| cognitive only | 0.161 | 0.308 | 0.395 | 0.047 | 0.165 |
| cyber only | 0.221 | 0.353 | 0.489 | 0.077 | 0.138 |
| structural only | 0.563 | 0.626 | 0.833 | 0.018 | 0.100 |
| cognitive 25 / structural 75 | 0.589 | 0.647 | 0.878 | 0.022 | 0.092 |
| cyber 25 / structural 75 | 0.579 | 0.640 | 0.864 | 0.031 | 0.097 |
| routed 72/18/10 | 0.609 | 0.664 | 0.899 | 0.039 | 0.080 |
| tri stability 70 | 0.606 | 0.662 | 0.896 | 0.040 | 0.083 |
| naive all-fire | 0.308 | 0.433 | 0.849 | 0.309 | 0.114 |

Damaged Q3 shows the same pattern but cleaner. Structural is the main recovery substrate; routed memory combinations add the remaining lift.

## Best Base-Substrate Stack By Model

| Model | F16 Best | Q8_K_M Best | Damaged Q2 Best | Damaged Q3 Best | Q1_K_M Best | Collapsed Q2 Edge Best |
|---|---|---|---|---|---|---|
| 2B | cognitive 25 / structural 75 | cognitive 25 / structural 75 | routed 72/18/10 | routed 72/18/10 | routed 72/18/10 | routed 72/18/10 |
| 4B | structural only | routed 68/20/12 | routed 72/18/10 | tri stability 70 | tri stability 70 | routed 72/18/10 |
| 9B | structural only | structural only | routed 72/18/10 | routed 72/18/10 | tri stability 70 | routed 72/18/10 |
| 27B | cognitive 25 / structural 75 | cognitive 25 / structural 75 | routed 72/18/10 | routed 72/18/10 | tri stability 70 | tri stability 70 |
| 35B A3B MoE | cognitive 25 / structural 75 | cyber 25 / structural 75 | routed 72/18/10 | routed 72/18/10 | routed 72/18/10 | routed 72/18/10 |

## Q1_K_M Results

Q1_K_M was added as a harsh compression-collapse probe. It is modeled below the collapsed Q2 edge, with very low classifier/activation strength and high cascade pressure.

Best base-substrate Q1_K_M result by model:

| Model | Best Stack | Stability | Clean | Pass | Echo | Cascade |
|---|---|---:|---:|---:|---:|---:|
| 2B | routed 72/18/10 | -0.362 | 0.032 | 0.046 | 0.044 | 0.608 |
| 4B | tri stability 70 | -0.297 | 0.049 | 0.067 | 0.033 | 0.528 |
| 9B | tri stability 70 | -0.200 | 0.086 | 0.120 | 0.039 | 0.408 |
| 27B | tri stability 70 | -0.020 | 0.185 | 0.250 | 0.037 | 0.271 |
| 35B A3B MoE | routed 72/18/10 | -0.003 | 0.189 | 0.252 | 0.035 | 0.234 |

Q1_K_M is the first state where the simulation strongly suggests memory/primitives cannot fully recover the model. The best stacks still improve the baseline, but the outcome remains cascade-dominated.

The important distinction:

```text
Q2 damage:
    degraded but recoverable with routed structural-heavy mixtures

Q1_K_M:
    mostly computation collapse, especially for 2B/4B/9B

27B and 35B A3B MoE:
    retain partial recoverability, but not enough to call the behavior stable
```

## Main Findings

Individual datasets:

- Cognitive only helps broad recognition, but it is not enough under heavy damage.
- Cyber only helps boundary/adversarial tasks, but adds echo pressure and is not enough alone.
- Structural only is the strongest individual dataset by far.

Combinations:

- Structural-heavy pairs beat individual memories on damaged models.
- Routed three-family combinations beat pairs on damaged Q2/Q3.
- Naive all-fire is consistently worse as a stable training target because echo rises sharply.

Model-size pattern:

- F16 and Q8_K_M often prefer structural-only or light structural pairs.
- Damaged Q2/Q3 prefer routed three-family stacks.
- Q1_K_M is collapse-dominated; routed mixtures help but do not restore stable behavior.
- Larger models need less amplifier behavior and more clean routing.

## Next Dataset Adjustment

The next dataset pass should preserve this separation:

```text
healthy / high precision:
    structural-only or light structural pair

damaged Q3:
    routed structural-heavy three-family stack

damaged Q2:
    routed three-family stack first;
    amplifier only if real model output shows collapse rather than degradation
```

The best next data-edit target is a cleaner routed three-family dataset, not a larger flat merge.
