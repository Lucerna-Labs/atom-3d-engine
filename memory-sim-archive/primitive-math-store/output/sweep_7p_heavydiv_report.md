# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T02:03:31.489631+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog_full.json` (939 primitives)
- Seed pool: 28 primitives from 7 domains
- N-tuple size: 7
- Top recipes kept: 30
- Runtime: 20.79s

## Enumeration

- 7-tuple: 1,184,040

## Scoring summary

- Recipes scored: 1,184,040
- Max score: 1150.0
- Mean score: 938.73
- Cross-domain: 1184040 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 1184040 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| agentic-reasoning | 837936 |
| cryptography-advanced | 837936 |
| formal-verification | 837936 |
| combinatorial-optimization | 837936 |
| networking | 837936 |
| statistics-probability | 837936 |
| causal-inference | 837936 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 1184040 |
| sample | 837936 |
| compare | 296010 |
| hash | 296010 |
| order | 296010 |

## Top 30 recipes

### Recipe #1 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, chromatic-dispersion, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #2 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, chromatic-dispersion, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #3 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, computation-tree-logic, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #4 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, computation-tree-logic, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #5 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, graph-of-thought, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #6 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, graph-of-thought, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #7 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, linear-temporal-logic, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #8 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, linear-temporal-logic, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #9 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, maximum-a-posteriori, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - maximum-a-posteriori (statistics-probability): MAP: θ_MAP = argmax_θ p(θ | x) = argmax_θ [p(x | θ)·p(θ)]. Equivalent to MLE with log-prior as regularizer. Not a proper Bayesian estimate (no uncertainty quantification).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [maximum-a-posteriori] Ignores posterior variance (misleading confidence)
  - [maximum-a-posteriori] not invariant to reparameterization
  - [maximum-a-posteriori] for proper uncertainty quantification, use full posterior
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #10 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, maximum-a-posteriori, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - maximum-a-posteriori (statistics-probability): MAP: θ_MAP = argmax_θ p(θ | x) = argmax_θ [p(x | θ)·p(θ)]. Equivalent to MLE with log-prior as regularizer. Not a proper Bayesian estimate (no uncertainty quantification).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [maximum-a-posteriori] Ignores posterior variance (misleading confidence)
  - [maximum-a-posteriori] not invariant to reparameterization
  - [maximum-a-posteriori] for proper uncertainty quantification, use full posterior
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #11 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, maximum-likelihood-estimation, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #12 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #13 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, merkle-commitment, method-of-moments, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #14 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, merkle-commitment, method-of-moments, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #15 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, bulm-model-checking, merkle-commitment, pedersen-commitment, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #16 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, computation-tree-logic, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #17 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, computation-tree-logic, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #18 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, graph-of-thought, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #19 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, graph-of-thought, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #20 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, linear-temporal-logic, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #21 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, linear-temporal-logic, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #22 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, maximum-a-posteriori, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - maximum-a-posteriori (statistics-probability): MAP: θ_MAP = argmax_θ p(θ | x) = argmax_θ [p(x | θ)·p(θ)]. Equivalent to MLE with log-prior as regularizer. Not a proper Bayesian estimate (no uncertainty quantification).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [maximum-a-posteriori] Ignores posterior variance (misleading confidence)
  - [maximum-a-posteriori] not invariant to reparameterization
  - [maximum-a-posteriori] for proper uncertainty quantification, use full posterior
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #23 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, maximum-a-posteriori, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - maximum-a-posteriori (statistics-probability): MAP: θ_MAP = argmax_θ p(θ | x) = argmax_θ [p(x | θ)·p(θ)]. Equivalent to MLE with log-prior as regularizer. Not a proper Bayesian estimate (no uncertainty quantification).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [maximum-a-posteriori] Ignores posterior variance (misleading confidence)
  - [maximum-a-posteriori] not invariant to reparameterization
  - [maximum-a-posteriori] for proper uncertainty quantification, use full posterior
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #24 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, maximum-likelihood-estimation, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #25 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, maximum-likelihood-estimation, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #26 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, merkle-commitment, method-of-moments, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #27 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, merkle-commitment, method-of-moments, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #28 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, chromatic-dispersion, merkle-commitment, pedersen-commitment, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, compare, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #29 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, computation-tree-logic, graph-of-thought, merkle-commitment, method-of-moments, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - method-of-moments (statistics-probability): Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [method-of-moments] Can be inconsistent (moments don't converge to true values)
  - [method-of-moments] fails for distributions without moments
  - [method-of-moments] less efficient than MLE
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #30 🌐 — score 1150.0

- **Primitives**: bayesian-posterior-inference, branch-and-cut, computation-tree-logic, graph-of-thought, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, hash, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```
