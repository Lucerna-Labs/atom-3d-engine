# Dataset Mixture Candidates v0.1

This pass does not train anything. It creates and tests candidate dataset mixtures so the data can be adjusted before LoRA training.

Candidate datasets:

```text
data/dataset_mixture_candidates_v0_1
```

Builder:

```text
scripts/build_dataset_mixture_candidates.py
```

Candidate simulation:

```text
scripts/simulate_dataset_mixture_candidates.py
```

Simulation outputs:

```text
runs/dataset-mixture-candidate-sim
```

## What Changed

The candidate builder combines three active dataset families:

- structural primitive examples from `structural_primitive_lora_v0_2`
- cognitive/generalist memories from the cognitive and computation corpora
- cyber/source-boundary memories from the cyber defender corpus

It also generates two important bridging layers:

- routed association examples that connect one memory to one structural primitive
- anti-echo clean-stop examples that teach gate, carrier, gain clamp, checksum, and stop behavior

This is meant to test dataset shape, not just ratio.

## Candidate Datasets

| Candidate | Structural | Cognitive | Cyber | Extra Shape |
|---|---:|---:|---:|---|
| `routed_tri_structural_65` | 65% | 20% | 15% | primary routed three-pack |
| `routed_tri_structural_60` | 60% | 25% | 15% | more cognitive breadth |
| `cognitive_structural_25_75` | 75% | 25% | 0% | general capability candidate |
| `cyber_structural_25_75` | 75% | 0% | 25% | source-boundary candidate |
| `q2_structural_amplifier` | 70% | 15% | 15% | heavy signal amplification plus clean-stop |
| `stability_first_tri_70` | 70% | 20% | 10% | conservative, anti-echo-heavy three-pack |

## Best Candidate By Context

| Substrate | Quant State | Best Candidate | Stability | Clean Pass | Pass | Echo | Cascade |
|---|---|---|---:|---:|---:|---:|---:|
| base | collapsed Q2 edge | `q2_structural_amplifier` | 0.024 | 0.232 | 0.447 | 0.289 | 0.234 |
| base | damaged Q2 | `q2_structural_amplifier` | 0.199 | 0.350 | 0.706 | 0.278 | 0.163 |
| base | damaged Q3 | `stability_first_tri_70` | 0.547 | 0.615 | 0.856 | 0.035 | 0.111 |
| base | damaged Q4 | `routed_tri_structural_65` | 0.625 | 0.679 | 0.901 | 0.045 | 0.080 |
| base | healthy Q6 | `cyber_structural_25_75` | 0.721 | 0.760 | 0.927 | 0.032 | 0.056 |
| base | healthy Q8 | `cognitive_structural_25_75` | 0.766 | 0.796 | 0.946 | 0.021 | 0.036 |

For degraded instruct:

- damaged Q2 also prefers `q2_structural_amplifier`
- damaged Q3/Q4 prefer `stability_first_tri_70`
- healthy Q6/Q8 prefer `cognitive_structural_25_75`

For instruct:

- damaged Q2 still prefers `q2_structural_amplifier`, but stability remains low
- damaged Q3/Q4 prefer `stability_first_tri_70`
- healthy Q6/Q8 prefer `cyber_structural_25_75`

## Interpretation

There is not one best dataset.

The working split is:

```text
Q2:
    needs a separate amplifier-style dataset.
    It improves raw and clean pass, but echo remains high.

Q3:
    prefers stability-first routing.
    This is probably the best target for proving reliable improvement.

Q4:
    prefers routed structural-heavy, but the margin is small.
    This is where fine dataset adjustments may matter most.

Healthy Q6/Q8:
    does not need the full three-pack.
    lighter pairwise mixtures are cleaner.
```

## Candidate Ranking Notes

For base damaged Q2:

| Candidate | Stability | Clean | Pass | Echo | Cascade |
|---|---:|---:|---:|---:|---:|
| `q2_structural_amplifier` | 0.199 | 0.350 | 0.706 | 0.278 | 0.163 |
| `stability_first_tri_70` | 0.148 | 0.314 | 0.450 | 0.041 | 0.242 |
| `routed_tri_structural_65` | 0.107 | 0.278 | 0.398 | 0.044 | 0.240 |

The Q2 amplifier is clearly better on pass rate, but it pays for that with echo.

For base damaged Q3:

| Candidate | Stability | Clean | Pass | Echo | Cascade |
|---|---:|---:|---:|---:|---:|
| `stability_first_tri_70` | 0.547 | 0.615 | 0.856 | 0.035 | 0.111 |
| `routed_tri_structural_65` | 0.539 | 0.610 | 0.851 | 0.047 | 0.111 |
| `routed_tri_structural_60` | 0.522 | 0.597 | 0.836 | 0.046 | 0.120 |

The best Q3 candidates are very close. That suggests the next adjustment should focus on anti-echo density and bridge quality, not big ratio changes.

For base damaged Q4:

| Candidate | Stability | Clean | Pass | Echo | Cascade |
|---|---:|---:|---:|---:|---:|
| `routed_tri_structural_65` | 0.625 | 0.679 | 0.901 | 0.045 | 0.080 |
| `cognitive_structural_25_75` | 0.625 | 0.677 | 0.893 | 0.021 | 0.081 |
| `stability_first_tri_70` | 0.622 | 0.675 | 0.904 | 0.041 | 0.076 |

The top three are effectively tied. Q4 is where we should tune for the target domain rather than chase a single aggregate score.

## Next Dataset Adjustments

Do not jump to training yet. The next iteration should create v0.2 candidates:

1. `q3_stability_tri_72`
   - structural 72%
   - cognitive 18%
   - cyber 10%
   - anti-echo multiplier 5
   - more routed bridge examples

2. `q3_routed_tri_68`
   - structural 68%
   - cognitive 20%
   - cyber 12%
   - anti-echo multiplier 3
   - stronger association examples

3. `q4_generalist_pair`
   - structural 72%
   - cognitive 28%
   - no cyber
   - stronger math/rhetoric/spatial coverage

4. `q4_boundary_pair`
   - structural 72%
   - cyber 28%
   - no cognitive
   - stronger prompt-injection/source-boundary coverage

5. `q2_amplifier_low_echo`
   - keep amplifier structure
   - reduce all-fire pressure
   - increase clean-stop examples
   - add examples where the model refuses to continue after the structural packet fires

The most promising proof target is Q3, not Q2. Q2 is interesting, but Q3 is where the mechanism looks strong and clean enough to evaluate honestly.
