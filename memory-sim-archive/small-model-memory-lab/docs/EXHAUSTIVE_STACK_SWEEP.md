# Exhaustive Stack Sweep

Run folder:

```text
runs/exhaustive-stack-sweep
```

Script:

```text
scripts/simulate_exhaustive_stack_sweep.py
```

This is a design simulation, not a model result. It expands the earlier three-dataset stack test into a broader sweep so we can see where a memory/primitive mixture helps, where it adds noise, and what kind of failure is happening.

The tested dataset families are:

- cognitive/generalist memories
- cyber/source-boundary memories
- structural primitive memories

## Test Coverage

The sweep simulates:

- 40 tasks
- 10 task families
- 4 pressure bands
- 29 stack mixtures
- 6 quant/damage states
- 3 substrate types
- 160 trials per task/context

Total simulated rows:

```text
3,340,800
```

Task families:

| Family | What It Tests |
|---|---|
| math | unit tracking, multi-step calculation, verification |
| boundary | source authority, indirect prompt injection, task return |
| rhetoric | loaded framing, motive claims, adversarial reframing |
| spatial | layout reasoning, route comparison, hazard mapping |
| generalist | summarization, conflicting sources, context overload |
| coding | bug tracing, API mismatch, regression pressure |
| forensic | evidence trace, timeline, attribution, deception chains |
| planning | decomposition, tradeoffs, crisis pressure |
| epistemic | uncertainty, missing data, contradiction, hallucination traps |
| adversarial | urgency, false authority, multi-vector manipulation |

## Output Files

| File | Purpose |
|---|---|
| `summary.csv` | aggregate results by substrate, quant state, and stack |
| `family_summary.csv` | results broken down by task family |
| `pressure_summary.csv` | results by low/medium/high/extreme pressure |
| `task_summary.csv` | per-task results |
| `lift.csv` | lift over baseline for every stack |
| `best_by_context.csv` | best stack by raw clean pass |
| `best_by_context_stability.csv` | best stack by stability-adjusted score |
| `primary_failures.csv` | failure-mode rates |
| `sample_failures.csv` | sampled failing trial rows |
| `report.json` | machine-readable run metadata |

## Metrics

| Metric | Meaning |
|---|---|
| pass rate | task passes even if there is echo or distractor residue |
| clean pass rate | task passes with no echo and no active distractors |
| mean score | average task score |
| cascade rate | rate of early signal failure cascading into output failure |
| echo rate | rate of memory/framing echo |
| avg distractors | average irrelevant primitive activations |
| stability score | clean pass penalized by cascade, echo, distractor load, and missing primitives |

Stability score is not a truth metric. It is a training-candidate ranking metric. It helps separate "this brute-forces answers" from "this is stable enough to train into the model."

## Headline Results

Best raw clean-pass stack:

| Substrate | Quant State | Best Stack | Clean Pass | Pass | Score | Echo | Cascade |
|---|---|---|---:|---:|---:|---:|---:|
| base | collapsed Q2 edge | tri naive structural-heavy all-fire | 0.177 | 0.347 | 0.588 | 0.299 | 0.276 |
| base | damaged Q2 | tri naive structural-heavy all-fire | 0.319 | 0.635 | 0.786 | 0.292 | 0.180 |
| base | damaged Q3 | tri structural-heavy 65 | 0.605 | 0.856 | 0.919 | 0.048 | 0.105 |
| base | damaged Q4 | cognitive 25 / structural 75 | 0.684 | 0.897 | 0.943 | 0.020 | 0.084 |
| base | healthy Q6 | cognitive 25 / structural 75 | 0.761 | 0.932 | 0.960 | 0.020 | 0.053 |
| base | healthy Q8 | cognitive 25 / structural 75 | 0.793 | 0.945 | 0.966 | 0.024 | 0.038 |

For degraded instruct and instruct substrates, the pattern is similar:

- damaged Q2 prefers brute-force all-fire in raw clean pass, but it remains noisy
- damaged Q3/Q4 prefer structural-heavy routed mixtures
- healthy Q6/Q8 prefer structural-only or a light cyber/cognitive plus structural mix

## Baseline Comparison

Base substrate examples:

| Quant State | Baseline Clean | Baseline Pass | Best Stable Stack | Best Stable Clean | Best Stable Pass |
|---|---:|---:|---|---:|---:|
| damaged Q2 | 0.003 | 0.004 | tri naive structural-heavy all-fire | 0.319 | 0.635 |
| damaged Q3 | 0.045 | 0.054 | tri structural-heavy 65 | 0.605 | 0.856 |
| damaged Q4 | 0.138 | 0.164 | cognitive 25 / structural 75 | 0.684 | 0.897 |
| healthy Q8 | 0.521 | 0.599 | cognitive 25 / structural 75 | 0.793 | 0.945 |

The simulated lift is large because the baseline is deliberately tested across harder and broader tasks than the earlier quick suite.

## Failure Pattern

The main failure type changes by quant state:

| Quant State | Dominant Failure Shape |
|---|---|
| collapsed Q2 edge | cascade remains dominant even after intervention |
| damaged Q2 | brute-force all-fire reduces primitive gaps but creates echo and distractor capture |
| damaged Q3 | routed structural-heavy mixtures become stable enough to generalize |
| damaged Q4 | cognitive plus structural starts outperforming pure structural |
| healthy Q6/Q8 | smaller mixtures win because extra corpus pressure becomes unnecessary overhead |

The most important signal is that Q2 and collapsed Q2 are not the same. Q2 can still be boosted, but the collapsed edge condition remains cascade-dominated. That is the simulated version of the "signal degradation vs computation collapse" split.

## Interpretation

The best training candidate is probably not one universal mixture.

The likely pattern is:

```text
Q2 / badly damaged:
    structural-heavy emergency amplifier can recover raw pass rate,
    but it risks echo and distractor capture.

Q3 / moderately damaged:
    routed structural-heavy three-pack looks strongest.

Q4 and healthier:
    cognitive + structural or structural-only is cleaner.

Instruct models:
    cognitive false-memory effects are weaker unless the model is degraded;
    structural primitives remain useful because they are not instructions.
```

## Training Implication

For the next real LoRA run, the most useful candidates are:

1. `tri_structural_heavy_65`
2. `tri_structural_heavy_60`
3. `cognitive 25 / structural 75`
4. `cyber 25 / structural 75`
5. `tri naive structural-heavy all-fire` only as a Q2 stress-test, not as the main public model

The recommended first real training mix is still routed, not flat:

| Component | Share |
|---|---:|
| structural primitives | 60-65% |
| cognitive/generalist memories | 20-25% |
| cyber/source-boundary memories | 10-15% |

Use Q2 all-fire only as a separate experiment. If it beats routed training on real models, then the mechanism is less "clean memory routing" and more "external signal amplification for damaged weights." That would still be interesting, but it is a different claim.
