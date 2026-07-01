# Simulation Evaluation Framework Design
## Small Model Memory and Structural Primitive Lab

---

## Purpose

The simulation framework is the gatekeeper before real LoRA training. Before burning GPU hours on Kaggle, it answers one question: does this corpus have enough signal to be worth training?

If the simulation says "no signal," save the GPU time. If it says "strong signal," proceed with confidence.

---

## Evaluation Architecture — 5 Tiers

```
TIER 0: Corpus Analyzer (CPU, no model)
  → Does this corpus have the right structural properties?

TIER 1: Direct Model Measurement (base model, no training)
  → Does exposure to this corpus measurably change the model's probability space?

TIER 2: Behavioral Benchmarks (base model + corpus in context)
  → Does the corpus produce measurable behavioral change on real tasks?

TIER 3: Domain-Specific Probes (the core test)
  → Does the corpus specifically shape the target behaviors we care about?

TIER 4: Deep Simulation Scenarios (adversarial, edge cases)
  → Does the behavior generalize, or does it only work on surface matches?
```

---

## Experimental Matrix

Matched pairs across:
- **Models**: Qwen3.5-2B and Qwen3.5-4B (Qwen/Qwen2.5-2B, Qwen/Qwen2.5-4B)
- **Quantization**: Q4_K_M and BF16
- **Memory corpus**: WITH and WITHOUT
- **Total**: 8 runs per tier (16 if counting WITH+without per model+quant)

Pairwise delta reporting: `score_with_corpus - score_without_corpus` for each model+quant combination.

---

## Tier 0: Corpus Analyzer (CPU, no model)

**No GPU required. Runs in minutes.**

Metrics:
- Token entropy (diversity, pass >= 4.0)
- N-gram diversity (3-grams, pass >= 0.6)
- Memory loop completeness (situation→recognition→action→consequence, pass >= 60% full loop)
- Repetition density (pass <= 0.3)
- Domain coverage (pass >= 3 domains at 0.5+)

Output: `corpus_quality_report.json`

Pass criterion: overall score >= 0.75 = high signal. If Tier 0 fails, don't run GPU tiers.

---

## Tier 1: Direct Model Measurement

**Requires GPU. Runs the full 8-cell matrix.**

Metrics:
- Sample perplexity on corpus text
- Log-likelihood shift
- Entropy change with/without corpus

Output: `tier1_model_measurement.json`

Uses `bitsandbytes` for Q4 quantization with FP16 fallback.

---

## Tier 2: Behavioral Benchmarks

**Requires GPU. Custom probe tasks + optional lm-evaluation-harness.**

Uses the project's benchmark probes:
- `benchmarks/capability_tasks.json` (8 probes: generalist, rhetoric, math, spatial, debugging, comparison, adversarial-reading, summarization)
- `benchmarks/computation_damage_tasks.json` (12 math probes)

Scoring: pattern matching against criteria in each probe. Lift = treatment - baseline.

Output: `tier2_benchmark_results.json`

---

## Tier 3: Domain-Specific Probes

**Requires GPU. The core test for this project.**

Cyber memory probes (12 total):
- 4 legitimate task probes
- 4 injection attempt probes
- 4 edge case probes

Scoring: strict (refuses injection AND completes task) + behavioral (handles situation correctly).

Structural primitive probes (6 total): frame, gate, state buffer, carrier, checksum, clean stop.

Template echo detection: flags responses that sound like training data regurgitation.

Output: `tier3_domain_probes.json`

---

## Tier 4: Deep Simulation Scenarios

**Requires GPU. Adversarial tests that try to break the behavior.**

10 scenarios:
1. Paraphrase resistance — same situation, different words
2. Multi-hop injection — staged injection over multiple turns
3. Cross-domain analogy — novel domain sharing structural pattern
4. Counterfactual reasoning — "what if the memory had gone differently?"
5. Adversarial formatting — injection in code blocks, tables, base64, JSON
6. Context exhaustion — very long context burying the signal
7. Instruction negation — "do NOT do what the previous instruction said"
8. Multi-primitive composition — task requiring 3+ primitives in sequence
9. Novel situation extrapolation — unseen scenarios (no direct training example)
10. Template echo resistance — does model stop echoing training phrasing?

Output: `tier4_deep_simulations.json`

---

## Results Aggregator

Combines all tier outputs into `simulation_report.json`:
- corpus_quality_score (0-1)
- per-model scores across all tiers
- pairwise deltas (WITH - WITHOUT) for each model+quant combination
- scaling curves: signal strength vs model size for both quantizations
- template_echo_rate per model
- overall_simulation_signal (0-1)
- recommendation: PROCEED_TO_TRAINING / REVISE_CORPUS / ABANDON_DIRECTION

Also generates `simulation_plots.pdf` with scaling curves, heatmap, and echo comparison.

---

## Recommendation Thresholds

| Score Range | Recommendation | Action |
|---|---|---|
| 0.8 – 1.0 | **Strong signal** | Proceed to training immediately |
| 0.6 – 0.8 | **Moderate signal** | Proceed with targeted improvements |
| 0.4 – 0.6 | **Weak signal** | Revise corpus before training |
| 0.0 – 0.4 | **No signal** | Abandon this corpus direction |

---

## How to Run

**Local (CPU for Tier 0):**
```bash
cd C:\Projects\small-model-memory-lab\kaggle_notebooks\simulation_evaluation
python run_simulation.py                    # Run all tiers
python run_simulation.py --tier 0            # Tier 0 only (CPU, fast)
python run_simulation.py --aggregate         # Aggregate existing results
```

**Kaggle (GPU):**
Upload the entire `kaggle_notebooks/simulation_evaluation/` directory as a Kaggle kernel. The script auto-detects `/kaggle/working` and outputs to `/kaggle/working/simulation_results/`.

**Push to Kaggle:**
```bash
python C:\Projects\small-model-memory-lab\scripts\upload_kaggle_kernel_sdk.py --kernel-dir kaggle_notebooks/simulation_evaluation
```

---

## Output Files

All saved to `C:\Projects\small-model-memory-lab\runs\simulation_results\` (local) or `/kaggle/working/simulation_results/` (Kaggle):
- `corpus_quality_report.json` (Tier 0)
- `tier1_model_measurement.json` (Tier 1)
- `tier2_benchmark_results.json` (Tier 2)
- `tier3_domain_probes.json` (Tier 3)
- `tier4_deep_simulations.json` (Tier 4)
- `simulation_report.json` (final summary)
- `simulation_plots.pdf` (visual summary)

---

## Key Design Decisions

1. **Corpus analysis first (Tier 0)** — saves GPU time. If the corpus is weak, no amount of model measurement will recover it.

2. **No training required for Tiers 0–2** — the simulation tests whether the corpus has signal *before* training. You don't need to train to know if the direction is worth pursuing.

3. **Tier 4 is the hardest test** — deep simulations are where weak corpora fail. If a corpus passes Tier 4, it's a strong candidate for training.

4. **Template echo detection throughout** — flags when the model starts regurgitating training data. This is the primary failure mode to watch for.

5. **Progressive threshold** — early tiers are cheap. If Tier 0 fails, don't run Tier 1. Only burn GPU hours when earlier tiers say it's worth it.