# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T01:42:34.056258+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog_full.json` (939 primitives)
- Seed pool: 7 primitives from 7 domains
- N-tuple size: 7
- Top recipes kept: 20
- Runtime: 0.01s

## Enumeration

- 7-tuple: 1

## Scoring summary

- Recipes scored: 1
- Max score: 855.0
- Mean score: 855.00
- Cross-domain: 1 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 1 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| formal-verification | 1 |
| networking | 1 |
| causal-inference | 1 |
| combinatorial-optimization | 1 |
| agentic-reasoning | 1 |
| statistics-probability | 1 |
| cryptography-advanced | 1 |

### Root atom coverage

| atom | uses |
|------|------|
| sample | 1 |
| combine | 1 |

## Top 1 recipes

### Recipe #1 🌐 — score 855.0

- **Primitives**: chain-of-thought, commitment-scheme, integer-programming, maximum-likelihood-estimation, photon-arrival, state-transition-system, structural-causal-model
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: sample, combine

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - commitment-scheme (cryptography-advanced): A commitment scheme allows a committer to lock a value (binding) without revealing it (hiding), then open it later (verifiable). Two phases: commit(value) → (commitment, decommit), verify(commitment, value, decommit) → accept/reject.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - photon-arrival (networking): Detection of a single photon or optical quantum at a photodetector.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [commitment-scheme] Binding property (cannot change committed value) vs. hiding property (cannot see committed value before opening) — some schemes trade one for the other
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [photon-arrival] Dark count rate, APD gain drift, timing jitter (>10 ps degrades correlation resolution)
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```
