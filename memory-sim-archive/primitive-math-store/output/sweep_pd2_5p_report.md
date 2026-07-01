# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T01:36:28.225914+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog_full.json` (939 primitives)
- Seed pool: 14 primitives from 7 domains
- N-tuple size: 5
- Top recipes kept: 50
- Runtime: 0.03s

## Enumeration

- 5-tuple: 2,002

## Scoring summary

- Recipes scored: 2,002
- Max score: 371.0
- Mean score: 359.09
- Cross-domain: 2002 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 2002 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| cryptography-advanced | 1210 |
| agentic-reasoning | 1210 |
| statistics-probability | 1210 |
| formal-verification | 1210 |
| combinatorial-optimization | 1210 |
| causal-inference | 1210 |
| networking | 1210 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 2002 |
| sample | 1210 |
| order | 715 |

## Top 50 recipes

### Recipe #1 🌐 — score 371.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, computation-tree-logic, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #2 🌐 — score 371.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, integer-programming, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #3 🌐 — score 371.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, linear-relaxation, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #4 🌐 — score 371.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, maximum-likelihood-estimation, pedersen-commitment, state-transition-system
- **Domains**: cryptography-advanced, formal-verification, statistics-probability
- **Categories**: cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #5 🌐 — score 371.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, maximum-likelihood-estimation, pedersen-commitment, structural-causal-model
- **Domains**: causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #6 🌐 — score 371.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, maximum-likelihood-estimation, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #7 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, computation-tree-logic, integer-programming, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #8 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, computation-tree-logic, linear-relaxation, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #9 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, computation-tree-logic, pedersen-commitment, state-transition-system
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #10 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, computation-tree-logic, pedersen-commitment, structural-causal-model
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #11 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, computation-tree-logic, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #12 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, integer-programming, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #13 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, linear-relaxation, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #14 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, maximum-likelihood-estimation, pedersen-commitment, state-transition-system
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #15 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, maximum-likelihood-estimation, pedersen-commitment, structural-causal-model
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #16 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, maximum-likelihood-estimation, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #17 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, commitment-scheme, computation-tree-logic, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: cryptography-advanced, formal-verification, statistics-probability
- **Categories**: cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - commitment-scheme (cryptography-advanced): A commitment scheme allows a committer to lock a value (binding) without revealing it (hiding), then open it later (verifiable). Two phases: commit(value) → (commitment, decommit), verify(commitment, value, decommit) → accept/reject.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [commitment-scheme] Binding property (cannot change committed value) vs. hiding property (cannot see committed value before opening) — some schemes trade one for the other
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #18 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, do-operator, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - do-operator (causal-inference): do(X = x): intervention that fixes variable X to value x, removing incoming arrows from its causal parents. Post-intervention distribution P(Y | do(X=x)).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [do-operator] do differs from conditioning: do(X=x) ≠ P(X=x | X=x) unless X has no confounders
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #19 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, integer-programming, linear-relaxation, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #20 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, integer-programming, pedersen-commitment, state-transition-system
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #21 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, integer-programming, pedersen-commitment, structural-causal-model
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #22 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, integer-programming, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #23 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, linear-relaxation, pedersen-commitment, state-transition-system
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #24 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, linear-relaxation, pedersen-commitment, structural-causal-model
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #25 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, linear-relaxation, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #26 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, maximum-likelihood-estimation, pedersen-commitment, photon-arrival
- **Domains**: cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - photon-arrival (networking): Detection of a single photon or optical quantum at a photodetector.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [photon-arrival] Dark count rate, APD gain drift, timing jitter (>10 ps degrades correlation resolution)
```

### Recipe #27 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, maximum-likelihood-estimation, pedersen-commitment, signal-attenuation
- **Domains**: cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - signal-attenuation (networking): Exponential decay of signal power: P(z) = P₀·e^(-αz).
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [signal-attenuation] α depends on wavelength (fiber: 0.2 dB/km at 1550 nm), medium (free-space: inverse-square + atmospheric extinction)
```

### Recipe #28 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, pedersen-commitment, state-transition-system, structural-causal-model
- **Domains**: causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #29 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, pedersen-commitment, state-transition-system, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #30 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, computation-tree-logic, pedersen-commitment, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #31 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, integer-programming, linear-relaxation, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #32 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, integer-programming, maximum-likelihood-estimation, pedersen-commitment, state-transition-system
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #33 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, integer-programming, maximum-likelihood-estimation, pedersen-commitment, structural-causal-model
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #34 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, integer-programming, maximum-likelihood-estimation, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #35 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, linear-relaxation, maximum-likelihood-estimation, pedersen-commitment, state-transition-system
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #36 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, linear-relaxation, maximum-likelihood-estimation, pedersen-commitment, structural-causal-model
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #37 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, linear-relaxation, maximum-likelihood-estimation, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #38 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, maximum-likelihood-estimation, pedersen-commitment, state-transition-system, structural-causal-model
- **Domains**: causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #39 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, maximum-likelihood-estimation, pedersen-commitment, state-transition-system, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #40 🌐 — score 369.0

- **Primitives**: bayesian-posterior-inference, maximum-likelihood-estimation, pedersen-commitment, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #41 🌐 — score 369.0

- **Primitives**: chain-of-thought, computation-tree-logic, integer-programming, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: sample, combine, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #42 🌐 — score 369.0

- **Primitives**: chain-of-thought, computation-tree-logic, linear-relaxation, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: sample, combine, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #43 🌐 — score 369.0

- **Primitives**: chain-of-thought, computation-tree-logic, maximum-likelihood-estimation, pedersen-commitment, state-transition-system
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: sample, combine, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #44 🌐 — score 369.0

- **Primitives**: chain-of-thought, computation-tree-logic, maximum-likelihood-estimation, pedersen-commitment, structural-causal-model
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: sample, combine, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #45 🌐 — score 369.0

- **Primitives**: chain-of-thought, computation-tree-logic, maximum-likelihood-estimation, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: sample, combine, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #46 🌐 — score 369.0

- **Primitives**: computation-tree-logic, integer-programming, linear-relaxation, maximum-likelihood-estimation, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #47 🌐 — score 369.0

- **Primitives**: computation-tree-logic, integer-programming, maximum-likelihood-estimation, pedersen-commitment, state-transition-system
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #48 🌐 — score 369.0

- **Primitives**: computation-tree-logic, integer-programming, maximum-likelihood-estimation, pedersen-commitment, structural-causal-model
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #49 🌐 — score 369.0

- **Primitives**: computation-tree-logic, integer-programming, maximum-likelihood-estimation, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #50 🌐 — score 369.0

- **Primitives**: computation-tree-logic, linear-relaxation, maximum-likelihood-estimation, pedersen-commitment, state-transition-system
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```
