# LoRA Memory Experiment

## Hypothesis
Episodic memories that encode cognitive skills (not textbook rules) can fine-tune a small model to reason more reliably. The same architecture from Mother AI — behavioral installation through lived experience rather than instruction — applied to capability instead of identity.

## Base Model
Llama 3.2 1B (trained on Colab T4)

## Approach
1. Define candidate domains (rhetoric, logic, pattern recognition, causal reasoning)
2. Generate episodic memories per domain — scenes, not explanations
3. Run simulations to test behavioral coverage, echo chains, and pressure response
4. Iterate through domain combinations to find what actually works
5. Generate final 250-memory corpus for the winning configuration
6. Train LoRA on Colab
7. Benchmark pre/post

## Key Principle
Memories are scenes where the skill was *applied*, not rules about how to apply it. The model should *recognize patterns* because it has experienced them, not because it was told about them.

## Directory Structure
```
simulation/
  engine.py          - Core simulation engine
  domains.py         - Domain definitions and behavioral specs
  memory_schema.py   - Memory object schema
  generator.py       - Memory generation templates
  run_experiment.py   - Main experiment runner
  analyze.py         - Results analysis and comparison
  reports/           - Simulation output reports
domains/
  rhetoric.py        - Rhetorical analysis domain spec
  logic.py           - Logical reasoning domain spec
  pattern.py         - Pattern recognition domain spec
  causal.py          - Causal reasoning domain spec
corpus/
  generated/         - Auto-generated memory files
  final/             - Winning configuration corpus
colab/
  train_lora.ipynb   - Colab training notebook
  benchmark.ipynb    - Pre/post benchmark notebook
```