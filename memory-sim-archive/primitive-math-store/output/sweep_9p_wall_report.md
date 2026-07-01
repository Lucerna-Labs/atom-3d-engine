# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T01:40:52.988098+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog_full.json` (939 primitives)
- Seed pool: 20 primitives from 7 domains
- N-tuple size: 9
- Top recipes kept: 30
- Runtime: 3.25s

## Enumeration

- 9-tuple: 167,960

## Scoring summary

- Recipes scored: 167,960
- Max score: 1125.0
- Mean score: 1085.30
- Cross-domain: 167960 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 167960 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| agentic-reasoning | 143650 |
| formal-verification | 143650 |
| networking | 143650 |
| cryptography-advanced | 143650 |
| combinatorial-optimization | 143650 |
| causal-inference | 143650 |
| statistics-probability | 119340 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 167960 |
| sample | 119340 |
| compare | 75582 |
| hash | 75582 |
| order | 75582 |

## Top 30 recipes

### Recipe #1 🌐 — score 1125.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #2 🌐 — score 1125.0

- **Primitives**: bayesian-posterior-inference, chromatic-dispersion, computation-tree-logic, graph-of-thought, integer-programming, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #3 🌐 — score 1125.0

- **Primitives**: bayesian-posterior-inference, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #4 🌐 — score 1125.0

- **Primitives**: bayesian-posterior-inference, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, state-transition-system
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #5 🌐 — score 1125.0

- **Primitives**: bayesian-posterior-inference, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, structural-causal-model
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #6 🌐 — score 1125.0

- **Primitives**: bayesian-posterior-inference, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #7 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #8 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, causal-identification, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - causal-identification (causal-inference): Causal identification: determine if P(Y | do(X)) is identifiable from observational distribution P(X, Y, Z). Backdoor and front-door criteria.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [causal-identification] Non-identifiable effects require auxiliary assumptions (instrumental variables, proxy variables) or bounds
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #9 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, integer-programming, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
```

### Recipe #10 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, integer-programming, linear-temporal-logic, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #11 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, integer-programming, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #12 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
```

### Recipe #13 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-relaxation, linear-temporal-logic, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #14 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-relaxation, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #15 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, state-transition-system
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #16 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, structural-causal-model
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #17 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #18 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, merkle-commitment, pedersen-commitment, state-transition-system
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #19 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, merkle-commitment, pedersen-commitment, structural-causal-model
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #20 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, linear-temporal-logic, merkle-commitment, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #21 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, state-transition-system
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #22 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, structural-causal-model
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #23 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #24 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, integer-programming, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #25 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #26 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, state-transition-system
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #27 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, structural-causal-model
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #28 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, computation-tree-logic, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #29 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, graph-of-thought, integer-programming, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #30 🌐 — score 1120.0

- **Primitives**: bayesian-posterior-inference, chain-of-thought, chromatic-dispersion, graph-of-thought, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 9 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```
