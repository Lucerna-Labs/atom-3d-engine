# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T01:37:52.111651+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog_full.json` (939 primitives)
- Seed pool: 50 primitives from 7 domains
- N-tuple size: 5
- Top recipes kept: 50
- Runtime: 31.33s

## Enumeration

- 5-tuple: 2,118,760

## Scoring summary

- Recipes scored: 2,118,760
- Max score: 500.0
- Mean score: 403.27
- Cross-domain: 2118424 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 2118760 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| cryptography-advanced | 1268092 |
| agentic-reasoning | 1268092 |
| formal-verification | 1268092 |
| networking | 1268092 |
| combinatorial-optimization | 1268092 |
| causal-inference | 1268092 |
| statistics-probability | 406456 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 2118634 |
| sample | 1032752 |
| project | 748006 |
| order | 584821 |
| compare | 406456 |
| hash | 406456 |
| scan | 211876 |

## Top 50 recipes

### Recipe #1 🌐 — score 500.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #2 🌐 — score 500.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, fiat-shamir-heuristic, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #3 🌐 — score 500.0

- **Primitives**: bayesian-posterior-inference, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #4 🌐 — score 500.0

- **Primitives**: bayesian-posterior-inference, bdd-symbolic-model-checking, fiat-shamir-heuristic, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #5 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, fiat-shamir-heuristic, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #6 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, fiat-shamir-heuristic, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #7 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, bulm-model-checking, chromatic-dispersion, fiat-shamir-heuristic, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #8 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, bulm-model-checking, fiat-shamir-heuristic, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #9 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, computation-tree-logic, fiat-shamir-heuristic, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #10 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, counterexample-guided-abstraction-refinement, fiat-shamir-heuristic, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - counterexample-guided-abstraction-refinement (formal-verification): CEGAR: iteratively (1) abstract model → (2) check abstract model → (3) if abstract counterexample is spurious, refine abstraction using Craig interpolation → repeat. Used for infinite-state or large finite-state systems.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [counterexample-guided-abstraction-refinement] Spurious counterexamples must be traceable to predicates
  - [counterexample-guided-abstraction-refinement] predicate discovery is the hard part
  - [counterexample-guided-abstraction-refinement] too many predicates → abstraction too concrete (BDD explosion)
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #11 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, fiat-shamir-heuristic, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #12 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, graph-of-thought, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #13 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, groth16-snark, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - groth16-snark (cryptography-advanced): Groth16: pairing-based SNARK with proof = (A, B, C) ∈ G₁ × G₂ × G₁; 3 pairings to verify; proof size = 3 elements (very short). Circuit-specific trusted setup (P = setup + preprocessing).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [groth16-snark] Trusted setup is circuit-specific (toxic waste → must destroy)
  - [groth16-snark] per-circuit ceremony
  - [groth16-snark] proofs not universal (cannot verify different circuits with same proving/verification keys)
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #14 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, linear-temporal-logic, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #15 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, maximum-likelihood-estimation, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #16 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #17 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, partial-order-reduction, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - partial-order-reduction (formal-verification): POR: reduce state space by exploring only one representative execution per equivalence class of concurrent events. Ample set, persistent set, sleep set algorithms. Independence → commutativity → reduced exploration.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [partial-order-reduction] POR must preserve properties being checked (safety vs. liveness require different reductions)
  - [partial-order-reduction] deadlocks may be missed without care
  - [partial-order-reduction] not all reductions are interchangeable
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #18 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #19 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, program-aided-language-model, react-reasoning-acting
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
```

### Recipe #20 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, program-aided-language-model, symmetry-reduction
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - symmetry-reduction (formal-verification): Exploit symmetries in the state space (e.g., interchangeable processes, identical components) to reduce the explored state graph. Quotient system = original / symmetry group.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [symmetry-reduction] Symmetric systems are a special case
  - [symmetry-reduction] detecting automorphism group of labeled transition system is expensive
  - [symmetry-reduction] orbit bisimulation
```

### Recipe #21 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, computation-tree-logic, fiat-shamir-heuristic, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #22 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, counterexample-guided-abstraction-refinement, fiat-shamir-heuristic, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - counterexample-guided-abstraction-refinement (formal-verification): CEGAR: iteratively (1) abstract model → (2) check abstract model → (3) if abstract counterexample is spurious, refine abstraction using Craig interpolation → repeat. Used for infinite-state or large finite-state systems.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [counterexample-guided-abstraction-refinement] Spurious counterexamples must be traceable to predicates
  - [counterexample-guided-abstraction-refinement] predicate discovery is the hard part
  - [counterexample-guided-abstraction-refinement] too many predicates → abstraction too concrete (BDD explosion)
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #23 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, cutting-planes, fiat-shamir-heuristic, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #24 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, cutting-planes, merkle-commitment, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, project, hash, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #25 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, fiat-shamir-heuristic, graph-of-thought, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #26 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, fiat-shamir-heuristic, groth16-snark, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - groth16-snark (cryptography-advanced): Groth16: pairing-based SNARK with proof = (A, B, C) ∈ G₁ × G₂ × G₁; 3 pairings to verify; proof size = 3 elements (very short). Circuit-specific trusted setup (P = setup + preprocessing).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [groth16-snark] Trusted setup is circuit-specific (toxic waste → must destroy)
  - [groth16-snark] per-circuit ceremony
  - [groth16-snark] proofs not universal (cannot verify different circuits with same proving/verification keys)
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #27 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, fiat-shamir-heuristic, linear-temporal-logic, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #28 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, fiat-shamir-heuristic, maximum-likelihood-estimation, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #29 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, fiat-shamir-heuristic, merkle-commitment, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #30 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, fiat-shamir-heuristic, partial-order-reduction, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - partial-order-reduction (formal-verification): POR: reduce state space by exploring only one representative execution per equivalence class of concurrent events. Ample set, persistent set, sleep set algorithms. Independence → commutativity → reduced exploration.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [partial-order-reduction] POR must preserve properties being checked (safety vs. liveness require different reductions)
  - [partial-order-reduction] deadlocks may be missed without care
  - [partial-order-reduction] not all reductions are interchangeable
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #31 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, fiat-shamir-heuristic, polynomial-commitment, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #32 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, fiat-shamir-heuristic, program-aided-language-model, react-reasoning-acting, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #33 🌐 — score 500.0

- **Primitives**: bdd-symbolic-model-checking, fiat-shamir-heuristic, program-aided-language-model, self-consistency, symmetry-reduction
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
  - symmetry-reduction (formal-verification): Exploit symmetries in the state space (e.g., interchangeable processes, identical components) to reduce the explored state graph. Quotient system = original / symmetry group.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
  - [symmetry-reduction] Symmetric systems are a special case
  - [symmetry-reduction] detecting automorphism group of labeled transition system is expensive
  - [symmetry-reduction] orbit bisimulation
```

### Recipe #34 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bayesian-posterior-inference, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
```

### Recipe #35 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bayesian-posterior-inference, bdd-symbolic-model-checking, fiat-shamir-heuristic, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → statistics-probability
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bayesian-posterior-inference (statistics-probability): Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bayesian-posterior-inference] Prior sensitivity: posterior can be prior-dominated if data is sparse
  - [bayesian-posterior-inference] Jeffreys prior = uninformative
  - [bayesian-posterior-inference] conjugate priors simplify
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #36 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, fiat-shamir-heuristic
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
```

### Recipe #37 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, branch-and-cut, fiat-shamir-heuristic, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #38 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, bulm-model-checking, chromatic-dispersion, fiat-shamir-heuristic
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
```

### Recipe #39 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, bulm-model-checking, fiat-shamir-heuristic, self-consistency
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, hash, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #40 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, computation-tree-logic, fiat-shamir-heuristic
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
```

### Recipe #41 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, counterexample-guided-abstraction-refinement, fiat-shamir-heuristic
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - counterexample-guided-abstraction-refinement (formal-verification): CEGAR: iteratively (1) abstract model → (2) check abstract model → (3) if abstract counterexample is spurious, refine abstraction using Craig interpolation → repeat. Used for infinite-state or large finite-state systems.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [counterexample-guided-abstraction-refinement] Spurious counterexamples must be traceable to predicates
  - [counterexample-guided-abstraction-refinement] predicate discovery is the hard part
  - [counterexample-guided-abstraction-refinement] too many predicates → abstraction too concrete (BDD explosion)
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
```

### Recipe #42 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, fiat-shamir-heuristic
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
```

### Recipe #43 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, graph-of-thought
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
```

### Recipe #44 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, groth16-snark
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - groth16-snark (cryptography-advanced): Groth16: pairing-based SNARK with proof = (A, B, C) ∈ G₁ × G₂ × G₁; 3 pairings to verify; proof size = 3 elements (very short). Circuit-specific trusted setup (P = setup + preprocessing).
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [groth16-snark] Trusted setup is circuit-specific (toxic waste → must destroy)
  - [groth16-snark] per-circuit ceremony
  - [groth16-snark] proofs not universal (cannot verify different circuits with same proving/verification keys)
```

### Recipe #45 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, linear-temporal-logic
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
```

### Recipe #46 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, maximum-likelihood-estimation
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking, statistics-probability
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking → statistics-probability
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - maximum-likelihood-estimation (statistics-probability): MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [maximum-likelihood-estimation] Non-convex likelihoods → multiple local maxima
  - [maximum-likelihood-estimation] flat likelihood → high variance
  - [maximum-likelihood-estimation] regularization may be needed
```

### Recipe #47 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, merkle-commitment
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
```

### Recipe #48 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, partial-order-reduction
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - partial-order-reduction (formal-verification): POR: reduce state space by exploring only one representative execution per equivalence class of concurrent events. Ample set, persistent set, sleep set algorithms. Independence → commutativity → reduced exploration.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [partial-order-reduction] POR must preserve properties being checked (safety vs. liveness require different reductions)
  - [partial-order-reduction] deadlocks may be missed without care
  - [partial-order-reduction] not all reductions are interchangeable
```

### Recipe #49 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, polynomial-commitment
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
```

### Recipe #50 🌐 — score 498.0

- **Primitives**: algorithm-of-thoughts, bdd-symbolic-model-checking, chromatic-dispersion, fiat-shamir-heuristic, react-reasoning-acting
- **Domains**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, scan, compare, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → cryptography-advanced → formal-verification → networking
**Components:**
  - algorithm-of-thoughts (agentic-reasoning): AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - fiat-shamir-heuristic (cryptography-advanced): Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [algorithm-of-thoughts] LLM as state evaluator may misjudge search frontier
  - [algorithm-of-thoughts] algorithm must fit problem structure
  - [algorithm-of-thoughts] not general-purpose
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [fiat-shamir-heuristic] Random oracle model required
  - [fiat-shamir-heuristic] Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack)
  - [fiat-shamir-heuristic] not all protocols safely transform
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
```
