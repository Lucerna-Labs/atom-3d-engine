# Dataset Mixture Candidates v0.2

This pass keeps adjusting the datasets before training. It adds tighter Q3/Q4 candidates and includes `2b_reference`, `4b`, `9b`, `27b`, and `35b_a3b_moe` model profiles in simulation.

Candidate datasets:

```text
data/dataset_mixture_candidates_v0_2
```

Builder:

```text
scripts/build_dataset_mixture_candidates_v0_2.py
```

Simulation:

```text
scripts/simulate_dataset_mixture_candidates.py --candidate-dir data/dataset_mixture_candidates_v0_2 --report-dir runs/dataset-mixture-candidate-sim-v0-2
```

Outputs:

```text
runs/dataset-mixture-candidate-sim-v0-2
```

## Model Profiles

| Profile | Meaning |
|---|---|
| `2b_reference` | current small-model reference profile |
| `4b` | mid-small dense profile; stronger than 2B but still sensitive to damaged quantization |
| `9b` | stronger base signal, lower quantization-like noise, lower cascade pressure |
| `27b` | large dense profile with stronger classifier/activation signal and reduced cascade |
| `35b_a3b_moe` | MoE profile with 35B total / A3B-active behavior; resilient routing and lower cascade under compression |

These are not real model runs. They are simulation profiles added so dataset mixtures can be tested across likely model-size behavior before training.

Quant states now include:

| Quant State | Meaning |
|---|---|
| `base_f16` | near full-precision reference |
| `q8_k_m` | high-precision quantized reference, modeled between f16 and regular healthy quant |
| `healthy_q8` | generic healthy Q8-like state |
| `healthy_q6` | healthy Q6-like state |
| `damaged_q4` | moderate compression damage |
| `damaged_q3` | heavy compression damage |
| `damaged_q2` | severe compression damage |
| `collapsed_q2_edge` | computation-collapse edge case |

## Candidate Set

| Candidate | Structural | Cognitive | Cyber | Purpose |
|---|---:|---:|---:|---|
| `q3_stability_tri_72` | 72% | 18% | 10% | Q3 stability, more clean-stop |
| `q3_routed_tri_68` | 68% | 20% | 12% | Q3 routing with more breadth |
| `q4_generalist_pair_72_28` | 72% | 28% | 0% | Q4 general capability |
| `q4_boundary_pair_72_28` | 72% | 0% | 28% | Q4 source-boundary behavior |
| `q2_amplifier_low_echo` | 72% | 14% | 14% | lower-echo Q2 amplifier |

## 2B Reference Results

Best candidate per base substrate context:

| Quant State | Best Candidate | Stability | Clean Pass | Pass | Echo | Cascade |
|---|---|---:|---:|---:|---:|---:|
| collapsed Q2 edge | `q2_amplifier_low_echo` | 0.026 | 0.235 | 0.457 | 0.280 | 0.245 |
| damaged Q2 | `q2_amplifier_low_echo` | 0.208 | 0.356 | 0.718 | 0.280 | 0.151 |
| damaged Q3 | `q3_routed_tri_68` | 0.554 | 0.621 | 0.859 | 0.039 | 0.106 |
| damaged Q4 | `q3_stability_tri_72` | 0.645 | 0.696 | 0.904 | 0.034 | 0.079 |
| healthy Q6 | `q4_generalist_pair_72_28` | 0.722 | 0.760 | 0.926 | 0.020 | 0.057 |
| healthy Q8 | `q4_generalist_pair_72_28` | 0.764 | 0.794 | 0.946 | 0.022 | 0.036 |

Compared with v0.1, v0.2 improves the Q2 candidate slightly and gives Q3 a cleaner winner.

## 4B Results

Best candidate per base substrate context:

| Quant State | Best Candidate | Stability | Clean Pass | Pass | Echo | Cascade |
|---|---|---:|---:|---:|---:|---:|
| collapsed Q2 edge | `q2_amplifier_low_echo` | 0.104 | 0.282 | 0.566 | 0.288 | 0.187 |
| damaged Q2 | `q3_stability_tri_72` | 0.302 | 0.426 | 0.613 | 0.036 | 0.179 |
| damaged Q3 | `q3_stability_tri_72` | 0.590 | 0.652 | 0.879 | 0.038 | 0.100 |
| damaged Q4 | `q3_routed_tri_68` | 0.658 | 0.707 | 0.911 | 0.039 | 0.070 |
| healthy Q6 | `q4_generalist_pair_72_28` | 0.747 | 0.780 | 0.937 | 0.021 | 0.046 |
| healthy Q8 | `q4_boundary_pair_72_28` | 0.782 | 0.811 | 0.953 | 0.031 | 0.032 |

The 4B profile already shifts away from the Q2 amplifier for ordinary damaged Q2. It only keeps the amplifier for collapsed Q2 edge.

## 9B Results

Best candidate per base substrate context:

| Quant State | Best Candidate | Stability | Clean Pass | Pass | Echo | Cascade |
|---|---|---:|---:|---:|---:|---:|
| base f16 | `q4_generalist_pair_72_28` | 0.837 | 0.858 | 0.965 | 0.020 | 0.020 |
| Q8_K_M | `q4_boundary_pair_72_28` | 0.806 | 0.831 | 0.960 | 0.033 | 0.024 |
| collapsed Q2 edge | `q2_amplifier_low_echo` | 0.179 | 0.335 | 0.675 | 0.288 | 0.155 |
| damaged Q2 | `q3_stability_tri_72` | 0.417 | 0.514 | 0.730 | 0.036 | 0.144 |
| damaged Q3 | `q3_stability_tri_72` | 0.622 | 0.678 | 0.893 | 0.038 | 0.089 |
| damaged Q4 | `q4_generalist_pair_72_28` | 0.684 | 0.728 | 0.916 | 0.019 | 0.068 |
| healthy Q6 | `q4_generalist_pair_72_28` | 0.766 | 0.796 | 0.944 | 0.021 | 0.039 |
| healthy Q8 | `q4_boundary_pair_72_28` | 0.797 | 0.824 | 0.958 | 0.031 | 0.027 |

The 9B profile changes the Q2 conclusion. A larger model no longer needs the noisy amplifier for ordinary damaged Q2. It prefers the cleaner Q3 stability dataset. The amplifier remains useful only in the collapsed Q2 edge case.

The 9B Q8_K_M comparison behaves as expected: it lands below f16 but above the generic healthy Q8 state.

| 9B Base Quant | Best Candidate | Stability | Clean Pass | Pass |
|---|---|---:|---:|---:|
| base f16 | `q4_generalist_pair_72_28` | 0.837 | 0.858 | 0.965 |
| Q8_K_M | `q4_boundary_pair_72_28` | 0.806 | 0.831 | 0.960 |
| healthy Q8 | `q4_generalist_pair_72_28` | 0.785 | 0.811 | 0.957 |

## 27B Results

Best candidate per base substrate context:

| Quant State | Best Candidate | Stability | Clean Pass | Pass | Echo | Cascade |
|---|---|---:|---:|---:|---:|---:|
| collapsed Q2 edge | `q3_stability_tri_72` | 0.296 | 0.417 | 0.594 | 0.042 | 0.168 |
| damaged Q2 | `q3_stability_tri_72` | 0.535 | 0.604 | 0.854 | 0.038 | 0.102 |
| damaged Q3 | `q3_stability_tri_72` | 0.669 | 0.715 | 0.919 | 0.035 | 0.063 |
| damaged Q4 | `q3_stability_tri_72` | 0.721 | 0.760 | 0.937 | 0.038 | 0.049 |
| healthy Q6 | `q4_boundary_pair_72_28` | 0.771 | 0.802 | 0.948 | 0.032 | 0.037 |
| healthy Q8 | `q3_routed_tri_68` | 0.806 | 0.832 | 0.963 | 0.037 | 0.023 |

The 27B dense profile is where the Q3 stability candidate becomes dominant across almost every damaged state, including collapsed Q2 edge.

## 35B A3B MoE Results

Best candidate per base substrate context:

| Quant State | Best Candidate | Stability | Clean Pass | Pass | Echo | Cascade |
|---|---|---:|---:|---:|---:|---:|
| collapsed Q2 edge | `q3_stability_tri_72` | 0.309 | 0.424 | 0.591 | 0.043 | 0.141 |
| damaged Q2 | `q3_stability_tri_72` | 0.546 | 0.613 | 0.858 | 0.044 | 0.094 |
| damaged Q3 | `q3_stability_tri_72` | 0.702 | 0.741 | 0.936 | 0.035 | 0.050 |
| damaged Q4 | `q4_generalist_pair_72_28` | 0.740 | 0.774 | 0.939 | 0.023 | 0.043 |
| healthy Q6 | `q3_routed_tri_68` | 0.791 | 0.819 | 0.960 | 0.039 | 0.026 |
| healthy Q8 | `q4_generalist_pair_72_28` | 0.818 | 0.841 | 0.960 | 0.019 | 0.026 |

The MoE profile behaves closer to a compression-resilient large model than to a tiny active-parameter model. It strongly prefers clean routing over amplifier pressure.

## Current Read

The dataset strategy should now split by model size:

```text
2B / damaged Q2:
    q2_amplifier_low_echo is still best, but echo remains high.

4B / damaged Q2-Q4:
    q3_stability_tri_72 or q3_routed_tri_68 becomes cleaner than the amplifier.

2B / damaged Q3-Q4:
    q3_routed_tri_68 and q3_stability_tri_72 are strongest.

9B / damaged Q2-Q3:
    q3_stability_tri_72 is strongest and much cleaner than the amplifier.

27B / damaged states:
    q3_stability_tri_72 dominates; amplifier is no longer the right default.

35B A3B MoE:
    q3_stability_tri_72 dominates damaged states; pairwise Q4 datasets win healthier states.

9B / Q4 and healthier:
    pairwise Q4 datasets win; the full three-pack is probably unnecessary.
```

## Next Dataset Adjustment

Do not train yet. The next data edit should make a v0.3 set:

1. `q3_stability_tri_72_b`
   - keep current ratio
   - increase bridge examples that connect cognitive memory to structural primitive without listing the memory
   - reduce generic anti-echo rows slightly

2. `q3_stability_tri_72_boundary`
   - structural 72%
   - cognitive 14%
   - cyber 14%
   - target prompt-injection and adversarial tasks

3. `q4_generalist_pair_76_24`
   - structural 76%
   - cognitive 24%
   - test whether slightly less cognitive improves healthy-model cleanliness

4. `q2_amplifier_low_echo_b`
   - keep amplifier behavior
   - lower echo by reducing all-fire pressure in the simulator
   - add closure examples where the model explicitly refuses to continue after checksum

The strongest proof targets are now:

```text
4B damaged Q2/Q3 with q3_stability_tri_72
9B damaged Q2/Q3 with q3_stability_tri_72
27B damaged Q2/Q3 with q3_stability_tri_72
35B A3B MoE damaged Q2/Q3 with q3_stability_tri_72
```

Those targets have meaningful lift without the ugly echo profile of the Q2 amplifier.
