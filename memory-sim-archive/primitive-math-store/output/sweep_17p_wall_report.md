# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T01:41:17.291025+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog_full.json` (939 primitives)
- Seed pool: 20 primitives from 7 domains
- N-tuple size: 17
- Top recipes kept: 10
- Runtime: 0.03s

## Enumeration

- 17-tuple: 1,140

## Scoring summary

- Recipes scored: 1,140
- Max score: 1990.0
- Mean score: 1972.67
- Cross-domain: 1140 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 1140 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| cryptography-advanced | 1139 |
| agentic-reasoning | 1139 |
| causal-inference | 1139 |
| formal-verification | 1139 |
| combinatorial-optimization | 1139 |
| networking | 1139 |
| statistics-probability | 1122 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 1140 |
| sample | 1122 |
| compare | 969 |
| hash | 969 |
| order | 969 |

## Top 10 recipes

### Recipe #1 🌐 — score 1990.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, causal-identification, chain-of-thought, chromatic-dispersion, commitment-scheme, computation-tree-logic, graph-of-thought, integer-programming, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, state-transition-system, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 17 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - causal-identification (causal-inference): Causal identification: determine if P(Y | do(X)) is identifiable from observational distribution P(X, Y, Z). Backdoor and front-door criteria.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - commitment-scheme (cryptography-advanced): A commitment scheme allows a committer to lock a value (binding) without revealing it (hiding), then open it later (verifiable). Two phases: commit(value) → (commitment, decommit), verify(commitment, value, decommit) → accept/reject.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
  - [causal-identification] Non-identifiable effects require auxiliary assumptions (instrumental variables, proxy variables) or bounds
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [commitment-scheme] Binding property (cannot change committed value) vs. hiding property (cannot see committed value before opening) — some schemes trade one for the other
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
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
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #2 🌐 — score 1990.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, causal-identification, chain-of-thought, chromatic-dispersion, computation-tree-logic, do-operator, graph-of-thought, integer-programming, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, state-transition-system, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 17 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - causal-identification (causal-inference): Causal identification: determine if P(Y | do(X)) is identifiable from observational distribution P(X, Y, Z). Backdoor and front-door criteria.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - do-operator (causal-inference): do(X = x): intervention that fixes variable X to value x, removing incoming arrows from its causal parents. Post-intervention distribution P(Y | do(X=x)).
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
  - [causal-identification] Non-identifiable effects require auxiliary assumptions (instrumental variables, proxy variables) or bounds
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [do-operator] do differs from conditioning: do(X=x) ≠ P(X=x | X=x) unless X has no confounders
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
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
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #3 🌐 — score 1990.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, causal-identification, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, integer-programming, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, photon-arrival, state-transition-system, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 17 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - causal-identification (causal-inference): Causal identification: determine if P(Y | do(X)) is identifiable from observational distribution P(X, Y, Z). Backdoor and front-door criteria.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - photon-arrival (networking): Detection of a single photon or optical quantum at a photodetector.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
  - [causal-identification] Non-identifiable effects require auxiliary assumptions (instrumental variables, proxy variables) or bounds
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
  - [photon-arrival] Dark count rate, APD gain drift, timing jitter (>10 ps degrades correlation resolution)
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #4 🌐 — score 1990.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, causal-identification, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, integer-programming, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, signal-attenuation, state-transition-system, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 17 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - causal-identification (causal-inference): Causal identification: determine if P(Y | do(X)) is identifiable from observational distribution P(X, Y, Z). Backdoor and front-door criteria.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - signal-attenuation (networking): Exponential decay of signal power: P(z) = P₀·e^(-αz).
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
  - [causal-identification] Non-identifiable effects require auxiliary assumptions (instrumental variables, proxy variables) or bounds
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
  - [signal-attenuation] α depends on wavelength (fiber: 0.2 dB/km at 1550 nm), medium (free-space: inverse-square + atmospheric extinction)
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #5 🌐 — score 1990.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, chain-of-thought, chromatic-dispersion, commitment-scheme, computation-tree-logic, do-operator, graph-of-thought, integer-programming, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, state-transition-system, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 17 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - commitment-scheme (cryptography-advanced): A commitment scheme allows a committer to lock a value (binding) without revealing it (hiding), then open it later (verifiable). Two phases: commit(value) → (commitment, decommit), verify(commitment, value, decommit) → accept/reject.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - do-operator (causal-inference): do(X = x): intervention that fixes variable X to value x, removing incoming arrows from its causal parents. Post-intervention distribution P(Y | do(X=x)).
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [commitment-scheme] Binding property (cannot change committed value) vs. hiding property (cannot see committed value before opening) — some schemes trade one for the other
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [do-operator] do differs from conditioning: do(X=x) ≠ P(X=x | X=x) unless X has no confounders
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
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
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #6 🌐 — score 1990.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, chain-of-thought, chromatic-dispersion, commitment-scheme, computation-tree-logic, graph-of-thought, integer-programming, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, photon-arrival, state-transition-system, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 17 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - commitment-scheme (cryptography-advanced): A commitment scheme allows a committer to lock a value (binding) without revealing it (hiding), then open it later (verifiable). Two phases: commit(value) → (commitment, decommit), verify(commitment, value, decommit) → accept/reject.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - photon-arrival (networking): Detection of a single photon or optical quantum at a photodetector.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [commitment-scheme] Binding property (cannot change committed value) vs. hiding property (cannot see committed value before opening) — some schemes trade one for the other
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
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
  - [photon-arrival] Dark count rate, APD gain drift, timing jitter (>10 ps degrades correlation resolution)
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #7 🌐 — score 1990.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, chain-of-thought, chromatic-dispersion, commitment-scheme, computation-tree-logic, graph-of-thought, integer-programming, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, signal-attenuation, state-transition-system, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 17 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - commitment-scheme (cryptography-advanced): A commitment scheme allows a committer to lock a value (binding) without revealing it (hiding), then open it later (verifiable). Two phases: commit(value) → (commitment, decommit), verify(commitment, value, decommit) → accept/reject.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - signal-attenuation (networking): Exponential decay of signal power: P(z) = P₀·e^(-αz).
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [commitment-scheme] Binding property (cannot change committed value) vs. hiding property (cannot see committed value before opening) — some schemes trade one for the other
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
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
  - [signal-attenuation] α depends on wavelength (fiber: 0.2 dB/km at 1550 nm), medium (free-space: inverse-square + atmospheric extinction)
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #8 🌐 — score 1990.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, chain-of-thought, chromatic-dispersion, computation-tree-logic, do-operator, graph-of-thought, integer-programming, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, photon-arrival, state-transition-system, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 17 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - do-operator (causal-inference): do(X = x): intervention that fixes variable X to value x, removing incoming arrows from its causal parents. Post-intervention distribution P(Y | do(X=x)).
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - photon-arrival (networking): Detection of a single photon or optical quantum at a photodetector.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [do-operator] do differs from conditioning: do(X=x) ≠ P(X=x | X=x) unless X has no confounders
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
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
  - [photon-arrival] Dark count rate, APD gain drift, timing jitter (>10 ps degrades correlation resolution)
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #9 🌐 — score 1990.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, chain-of-thought, chromatic-dispersion, computation-tree-logic, do-operator, graph-of-thought, integer-programming, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, signal-attenuation, state-transition-system, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 17 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - do-operator (causal-inference): do(X = x): intervention that fixes variable X to value x, removing incoming arrows from its causal parents. Post-intervention distribution P(Y | do(X=x)).
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - signal-attenuation (networking): Exponential decay of signal power: P(z) = P₀·e^(-αz).
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [do-operator] do differs from conditioning: do(X=x) ≠ P(X=x | X=x) unless X has no confounders
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
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
  - [signal-attenuation] α depends on wavelength (fiber: 0.2 dB/km at 1550 nm), medium (free-space: inverse-square + atmospheric extinction)
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #10 🌐 — score 1990.0

- **Primitives**: bayesian-posterior-inference, branch-and-bound, chain-of-thought, chromatic-dispersion, computation-tree-logic, graph-of-thought, integer-programming, linear-relaxation, linear-temporal-logic, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, photon-arrival, signal-attenuation, state-transition-system, structural-causal-model, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 17 atomic
- **Root atoms used**: combine, sample, compare, hash, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-bound (combinatorial-optimization): Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - photon-arrival (networking): Detection of a single photon or optical quantum at a photodetector.
  - signal-attenuation (networking): Exponential decay of signal power: P(z) = P₀·e^(-αz).
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-bound] Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance
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
  - [photon-arrival] Dark count rate, APD gain drift, timing jitter (>10 ps degrades correlation resolution)
  - [signal-attenuation] α depends on wavelength (fiber: 0.2 dB/km at 1550 nm), medium (free-space: inverse-square + atmospheric extinction)
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```
