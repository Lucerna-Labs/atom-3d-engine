# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T01:57:45.578837+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog_full.json` (939 primitives)
- Seed pool: 30 primitives from 6 domains
- N-tuple size: 7
- Top recipes kept: 30
- Runtime: 35.29s

## Enumeration

- 7-tuple: 2,035,800

## Scoring summary

- Recipes scored: 2,035,800
- Max score: 915.0
- Mean score: 870.95
- Cross-domain: 2035800 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 2035800 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| combinatorial-optimization | 1555100 |
| agentic-reasoning | 1555100 |
| cryptography-advanced | 1555100 |
| formal-verification | 1555100 |
| networking | 1555100 |
| causal-inference | 1555100 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 2035800 |
| sample | 1378000 |
| order | 851760 |
| scan | 475020 |
| project | 475020 |
| hash | 475020 |
| compare | 475020 |

## Top 30 recipes

### Recipe #1 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, chain-of-thought, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
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
```

### Recipe #2 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
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
```

### Recipe #3 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, computation-tree-logic, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
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
```

### Recipe #4 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, cutting-planes, graph-of-thought, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #5 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, cutting-planes, linear-temporal-logic, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #6 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, cutting-planes, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
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

### Recipe #7 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, cutting-planes, merkle-commitment, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #8 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, cutting-planes, merkle-commitment, program-aided-language-model, react-reasoning-acting
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
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
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
```

### Recipe #9 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, cutting-planes, merkle-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #10 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
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
```

### Recipe #11 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, computation-tree-logic, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
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
```

### Recipe #12 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, cutting-planes, graph-of-thought, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #13 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, cutting-planes, linear-temporal-logic, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #14 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, cutting-planes, merkle-commitment, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #15 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, cutting-planes, merkle-commitment, program-aided-language-model, react-reasoning-acting
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
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
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
```

### Recipe #16 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, computation-tree-logic, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
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
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
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
```

### Recipe #17 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, cutting-planes, graph-of-thought, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
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
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #18 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, cutting-planes, linear-temporal-logic, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
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
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #19 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
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
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
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

### Recipe #20 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, cutting-planes, merkle-commitment, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
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
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #21 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, cutting-planes, merkle-commitment, program-aided-language-model, react-reasoning-acting
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
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
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
```

### Recipe #22 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, cutting-planes, merkle-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #23 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, computation-tree-logic, cutting-planes, graph-of-thought, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #24 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, computation-tree-logic, cutting-planes, linear-temporal-logic, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #25 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, computation-tree-logic, cutting-planes, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
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

### Recipe #26 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, computation-tree-logic, cutting-planes, merkle-commitment, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #27 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, computation-tree-logic, cutting-planes, merkle-commitment, program-aided-language-model, react-reasoning-acting
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
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
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
```

### Recipe #28 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, computation-tree-logic, cutting-planes, merkle-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [computation-tree-logic] Expressive enough for many properties
  - [computation-tree-logic] CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p)
  - [computation-tree-logic] CTL* (full branching time) is more expressive
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #29 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, cutting-planes, graph-of-thought, linear-temporal-logic, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [graph-of-thought] Cycle detection and handling
  - [graph-of-thought] graph consistency maintenance
  - [graph-of-thought] no established benchmark for GoT quality
  - [linear-temporal-logic] LTL model checking is PSPACE-complete (in |TS|)
  - [linear-temporal-logic] explicit-state MC has complexity O(|TS| · 2^|φ|)
  - [linear-temporal-logic] symbolic is PSPACE in practice
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #30 🌐 — score 915.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, cutting-planes, graph-of-thought, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [branch-and-cut] Cut selection: too many cuts = expensive LP solves
  - [branch-and-cut] too few = weak bound
  - [branch-and-cut] GMI cuts are most effective
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
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
