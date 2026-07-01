# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T00:47:52.333195+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog_full.json` (939 primitives)
- Seed pool: 30 primitives from 6 domains
- N-tuple size: 6
- Top recipes kept: 100
- Runtime: 9.39s

## Enumeration

- 6-tuple: 593,775

## Scoring summary

- Recipes scored: 593,775
- Max score: 449.0
- Mean score: 419.73
- Cross-domain: 593775 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 593775 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| networking | 416675 |
| agentic-reasoning | 416675 |
| cryptography-advanced | 416675 |
| combinatorial-optimization | 416675 |
| formal-verification | 416675 |
| causal-inference | 416675 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 593775 |
| sample | 363545 |
| order | 217035 |
| scan | 118755 |
| compare | 118755 |
| project | 118755 |
| hash | 118755 |

## Top 100 recipes

### Recipe #1 🌐 — score 449.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #2 🌐 — score 449.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
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

### Recipe #3 🌐 — score 449.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #4 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
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
```

### Recipe #5 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
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
```

### Recipe #6 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, cutting-planes, merkle-commitment, sigma-protocol
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #7 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, bulm-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [bulm-model-checking] Completeness threshold: k must reach the diameter of the state graph
  - [bulm-model-checking] for liveness, need deeper unrolling
  - [bulm-model-checking] k-induction (inductive invariants for k > diameter)
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
```

### Recipe #8 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, bulm-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #9 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, bulm-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, sigma-protocol
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #10 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, computation-tree-logic, cutting-planes, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
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
```

### Recipe #11 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, graph-of-thought, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
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
```

### Recipe #12 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, linear-temporal-logic, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
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
```

### Recipe #13 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment, polynomial-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
```

### Recipe #14 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment, react-reasoning-acting
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
```

### Recipe #15 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, computation-tree-logic, cutting-planes, merkle-commitment, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #16 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, computation-tree-logic, cutting-planes, merkle-commitment, sigma-protocol
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #17 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, graph-of-thought, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #18 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, graph-of-thought, merkle-commitment, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #19 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, linear-temporal-logic, merkle-commitment, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #20 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, linear-temporal-logic, merkle-commitment, sigma-protocol
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #21 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment, polynomial-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
```

### Recipe #22 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment, react-reasoning-acting
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
```

### Recipe #23 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, polarization-mode-dispersion, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polarization-mode-dispersion (networking): Differential group delay between polarization modes: Δτ_PMD = D_PMD·√L. Stochastic (Maxwell-Boltzmann).
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
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [polarization-mode-dispersion] In older fibers, PMD can exceed 0.5 ps/√km and limit 40 Gbps systems
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #24 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, polynomial-commitment, sigma-protocol
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #25 🌐 — score 447.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, react-reasoning-acting, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #26 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
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
```

### Recipe #27 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
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
```

### Recipe #28 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, cutting-planes, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
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

### Recipe #29 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, cutting-planes, merkle-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
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

### Recipe #30 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, bulm-model-checking, chain-of-thought, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #31 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, bulm-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
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

### Recipe #32 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, bulm-model-checking, cutting-planes, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #33 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, bulm-model-checking, cutting-planes, merkle-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #34 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, computation-tree-logic, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #35 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, cutting-planes, graph-of-thought, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #36 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, cutting-planes, linear-temporal-logic, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #37 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, cutting-planes, merkle-commitment, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #38 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, cutting-planes, merkle-commitment, program-aided-language-model, react-reasoning-acting
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #39 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, computation-tree-logic, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
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

### Recipe #40 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, graph-of-thought, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
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

### Recipe #41 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, linear-temporal-logic, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
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

### Recipe #42 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
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

### Recipe #43 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, program-aided-language-model, react-reasoning-acting
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #44 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, computation-tree-logic, cutting-planes, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #45 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, computation-tree-logic, cutting-planes, merkle-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #46 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, cutting-planes, graph-of-thought, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #47 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, cutting-planes, graph-of-thought, merkle-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #48 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, cutting-planes, linear-temporal-logic, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
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

### Recipe #49 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, cutting-planes, linear-temporal-logic, merkle-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #50 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, cutting-planes, merkle-commitment, pedersen-commitment, polynomial-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
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

### Recipe #51 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, cutting-planes, merkle-commitment, pedersen-commitment, program-aided-language-model, react-reasoning-acting
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
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
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
```

### Recipe #52 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, cutting-planes, merkle-commitment, polynomial-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
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
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #53 🌐 — score 446.0

- **Primitives**: bdd-symbolic-model-checking, cutting-planes, merkle-commitment, program-aided-language-model, react-reasoning-acting, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
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
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #54 🌐 — score 445.0

- **Primitives**: backdoor-criterion, bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - backdoor-criterion (causal-inference): Backdoor criterion: set Z satisfies backdoor if Z blocks all backdoor paths from X to Y and contains no descendants of X.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [backdoor-criterion] Minimum sufficient adjustment set = smallest set satisfying backdoor
  - [backdoor-criterion] different adjustment sets give same estimate
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
```

### Recipe #55 🌐 — score 445.0

- **Primitives**: backdoor-criterion, bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - backdoor-criterion (causal-inference): Backdoor criterion: set Z satisfies backdoor if Z blocks all backdoor paths from X to Y and contains no descendants of X.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [backdoor-criterion] Minimum sufficient adjustment set = smallest set satisfying backdoor
  - [backdoor-criterion] different adjustment sets give same estimate
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
```

### Recipe #56 🌐 — score 445.0

- **Primitives**: backdoor-criterion, bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, sigma-protocol
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - backdoor-criterion (causal-inference): Backdoor criterion: set Z satisfies backdoor if Z blocks all backdoor paths from X to Y and contains no descendants of X.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [backdoor-criterion] Minimum sufficient adjustment set = smallest set satisfying backdoor
  - [backdoor-criterion] different adjustment sets give same estimate
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #57 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chromatic-dispersion, cutting-planes, merkle-commitment, polarization-mode-dispersion
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polarization-mode-dispersion (networking): Differential group delay between polarization modes: Δτ_PMD = D_PMD·√L. Stochastic (Maxwell-Boltzmann).
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
  - [polarization-mode-dispersion] In older fibers, PMD can exceed 0.5 ps/√km and limit 40 Gbps systems
```

### Recipe #58 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, bulm-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, polarization-mode-dispersion
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polarization-mode-dispersion (networking): Differential group delay between polarization modes: Δτ_PMD = D_PMD·√L. Stochastic (Maxwell-Boltzmann).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [polarization-mode-dispersion] In older fibers, PMD can exceed 0.5 ps/√km and limit 40 Gbps systems
```

### Recipe #59 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, front-door-criterion, merkle-commitment
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - front-door-criterion (causal-inference): Front-door criterion: set Z mediates all causal effect of X on Y, with no unblocked backdoor path from X to Z and no confounder of Z and Y.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [front-door-criterion] Front-door is useful when X and Y share an unmeasured confounder
  - [front-door-criterion] captures mediation through Z
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
```

### Recipe #60 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, integer-programming, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
```

### Recipe #61 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, linear-relaxation, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chain-of-thought] CoT effectiveness plateaus for >5–7 steps
  - [chain-of-thought] self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k)
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
```

### Recipe #62 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment, optical-snr-measurement
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - optical-snr-measurement (networking): OSNR = P_signal / (N_ASE · Δf_ref). Reference bandwidth = 0.1 nm (12.5 GHz at 1550 nm).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [optical-snr-measurement] ASE noise from EDFAs
  - [optical-snr-measurement] OSNR < ~15 dB for 100 Gbps PM-QPSK causes BER floor
```

### Recipe #63 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #64 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment, sigma-protocol
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #65 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment, state-transition-system
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #66 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment, structural-causal-model
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #67 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chain-of-thought, chromatic-dispersion, cutting-planes, merkle-commitment, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #68 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, computation-tree-logic, cutting-planes, merkle-commitment, polarization-mode-dispersion
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - computation-tree-logic (formal-verification): CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polarization-mode-dispersion (networking): Differential group delay between polarization modes: Δτ_PMD = D_PMD·√L. Stochastic (Maxwell-Boltzmann).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [polarization-mode-dispersion] In older fibers, PMD can exceed 0.5 ps/√km and limit 40 Gbps systems
```

### Recipe #69 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, front-door-criterion, merkle-commitment, pedersen-commitment
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - front-door-criterion (causal-inference): Front-door criterion: set Z mediates all causal effect of X on Y, with no unblocked backdoor path from X to Z and no confounder of Z and Y.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [front-door-criterion] Front-door is useful when X and Y share an unmeasured confounder
  - [front-door-criterion] captures mediation through Z
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #70 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, front-door-criterion, merkle-commitment, sigma-protocol
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - front-door-criterion (causal-inference): Front-door criterion: set Z mediates all causal effect of X on Y, with no unblocked backdoor path from X to Z and no confounder of Z and Y.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [front-door-criterion] Front-door is useful when X and Y share an unmeasured confounder
  - [front-door-criterion] captures mediation through Z
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #71 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, graph-of-thought, merkle-commitment, polarization-mode-dispersion
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - graph-of-thought (agentic-reasoning): Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polarization-mode-dispersion (networking): Differential group delay between polarization modes: Δτ_PMD = D_PMD·√L. Stochastic (Maxwell-Boltzmann).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [polarization-mode-dispersion] In older fibers, PMD can exceed 0.5 ps/√km and limit 40 Gbps systems
```

### Recipe #72 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, integer-programming, merkle-commitment, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #73 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, integer-programming, merkle-commitment, sigma-protocol
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - integer-programming (combinatorial-optimization): IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [integer-programming] Problem formulation (modeling) quality matters more than solver
  - [integer-programming] different formulations have vastly different solving times
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #74 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, linear-relaxation, merkle-commitment, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #75 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, linear-relaxation, merkle-commitment, sigma-protocol
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-relaxation (combinatorial-optimization): LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [linear-relaxation] LP relaxation gap (IP-LP)/IP = integrality gap
  - [linear-relaxation] large gap means branch-and-bound explores many nodes
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #76 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, linear-temporal-logic, merkle-commitment, polarization-mode-dispersion
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - linear-temporal-logic (formal-verification): LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polarization-mode-dispersion (networking): Differential group delay between polarization modes: Δτ_PMD = D_PMD·√L. Stochastic (Maxwell-Boltzmann).
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [polarization-mode-dispersion] In older fibers, PMD can exceed 0.5 ps/√km and limit 40 Gbps systems
```

### Recipe #77 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, optical-snr-measurement, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - optical-snr-measurement (networking): OSNR = P_signal / (N_ASE · Δf_ref). Reference bandwidth = 0.1 nm (12.5 GHz at 1550 nm).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [optical-snr-measurement] ASE noise from EDFAs
  - [optical-snr-measurement] OSNR < ~15 dB for 100 Gbps PM-QPSK causes BER floor
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #78 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, optical-snr-measurement, sigma-protocol
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - optical-snr-measurement (networking): OSNR = P_signal / (N_ASE · Δf_ref). Reference bandwidth = 0.1 nm (12.5 GHz at 1550 nm).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [optical-snr-measurement] ASE noise from EDFAs
  - [optical-snr-measurement] OSNR < ~15 dB for 100 Gbps PM-QPSK causes BER floor
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #79 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment, sigma-protocol
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #80 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment, state-transition-system
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #81 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment, structural-causal-model
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #82 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, pedersen-commitment, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #83 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, polarization-mode-dispersion, polynomial-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polarization-mode-dispersion (networking): Differential group delay between polarization modes: Δτ_PMD = D_PMD·√L. Stochastic (Maxwell-Boltzmann).
  - polynomial-commitment (cryptography-advanced): Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [polarization-mode-dispersion] In older fibers, PMD can exceed 0.5 ps/√km and limit 40 Gbps systems
  - [polynomial-commitment] KZG requires a trusted setup ceremony (toxic waste must be destroyed)
  - [polynomial-commitment] DARK (Diophantine) uses no trusted setup
  - [polynomial-commitment] IPA (inner product argument) is post-quantum
```

### Recipe #84 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, polarization-mode-dispersion, react-reasoning-acting
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - polarization-mode-dispersion (networking): Differential group delay between polarization modes: Δτ_PMD = D_PMD·√L. Stochastic (Maxwell-Boltzmann).
  - react-reasoning-acting (agentic-reasoning): ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [polarization-mode-dispersion] In older fibers, PMD can exceed 0.5 ps/√km and limit 40 Gbps systems
  - [react-reasoning-acting] Tool call errors cascade
  - [react-reasoning-acting] loop detection needed (max iterations)
  - [react-reasoning-acting] action space quality determines success rate
```

### Recipe #85 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, sigma-protocol, state-transition-system
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
  - state-transition-system (formal-verification): Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
  - [state-transition-system] State explosion: |S| grows exponentially with components
  - [state-transition-system] compositional methods mitigate but don't solve
```

### Recipe #86 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, sigma-protocol, structural-causal-model
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
  - structural-causal-model (causal-inference): SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
  - [structural-causal-model] SCMs assume acyclic graphs (except with cycles under equilibrium)
  - [structural-causal-model] cyclic SCMs require fixed-point solution
```

### Recipe #87 🌐 — score 445.0

- **Primitives**: bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, sigma-protocol, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
  - tree-of-thought (agentic-reasoning): Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
**Real walls (combined):**
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
  - [chromatic-dispersion] D (ps/nm/km) varies with fiber type
  - [chromatic-dispersion] uncompensated dispersion limits bit rate × distance product
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
  - [tree-of-thought] Evaluation function quality determines pruning accuracy
  - [tree-of-thought] noisy evaluation → wrong branches kept, correct branches pruned
```

### Recipe #88 🌐 — score 444.0

- **Primitives**: backdoor-criterion, bdd-symbolic-model-checking, chain-of-thought, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, scan, sample, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - backdoor-criterion (causal-inference): Backdoor criterion: set Z satisfies backdoor if Z blocks all backdoor paths from X to Y and contains no descendants of X.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [backdoor-criterion] Minimum sufficient adjustment set = smallest set satisfying backdoor
  - [backdoor-criterion] different adjustment sets give same estimate
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #89 🌐 — score 444.0

- **Primitives**: backdoor-criterion, bdd-symbolic-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - backdoor-criterion (causal-inference): Backdoor criterion: set Z satisfies backdoor if Z blocks all backdoor paths from X to Y and contains no descendants of X.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [backdoor-criterion] Minimum sufficient adjustment set = smallest set satisfying backdoor
  - [backdoor-criterion] different adjustment sets give same estimate
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #90 🌐 — score 444.0

- **Primitives**: backdoor-criterion, bdd-symbolic-model-checking, cutting-planes, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - backdoor-criterion (causal-inference): Backdoor criterion: set Z satisfies backdoor if Z blocks all backdoor paths from X to Y and contains no descendants of X.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
  - [backdoor-criterion] Minimum sufficient adjustment set = smallest set satisfying backdoor
  - [backdoor-criterion] different adjustment sets give same estimate
  - [bdd-symbolic-model-checking] BDD explosion for some functions (multipliers, cryptographic functions)
  - [bdd-symbolic-model-checking] variable ordering problem is NP-hard
  - [bdd-symbolic-model-checking] dynamic variable reordering helps
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

### Recipe #91 🌐 — score 444.0

- **Primitives**: backdoor-criterion, bdd-symbolic-model-checking, cutting-planes, merkle-commitment, program-aided-language-model, sigma-protocol
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - backdoor-criterion (causal-inference): Backdoor criterion: set Z satisfies backdoor if Z blocks all backdoor paths from X to Y and contains no descendants of X.
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - sigma-protocol (cryptography-advanced): Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
**Real walls (combined):**
  - [backdoor-criterion] Minimum sufficient adjustment set = smallest set satisfying backdoor
  - [backdoor-criterion] different adjustment sets give same estimate
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #92 🌐 — score 444.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, chain-of-thought, cutting-planes, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
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
```

### Recipe #93 🌐 — score 444.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, chromatic-dispersion, cutting-planes, merkle-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, compare, project, hash

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
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
```

### Recipe #94 🌐 — score 444.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, cutting-planes, merkle-commitment, pedersen-commitment
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
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
```

### Recipe #95 🌐 — score 444.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, bulm-model-checking, cutting-planes, merkle-commitment, sigma-protocol
- **Domains**: combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, project, hash, sample

```
**Wiring:** fusion ranking
**Cross-domain:** combinatorial-optimization → cryptography-advanced → formal-verification
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - bulm-model-checking (formal-verification): Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
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
  - [sigma-protocol] Challenge space must be large enough (≥ 2^128 for computational security)
  - [sigma-protocol] many protocols require Fiat-Shamir to remove interaction (non-interactive)
```

### Recipe #96 🌐 — score 444.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, chromatic-dispersion, cutting-planes, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, project

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
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
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #97 🌐 — score 444.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, chromatic-dispersion, merkle-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification, networking
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, scan, sample, compare, hash

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced → formal-verification → networking
**Components:**
  - bdd-symbolic-model-checking (formal-verification): Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
  - branch-and-cut (combinatorial-optimization): Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
  - chain-of-thought (agentic-reasoning): Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
  - chromatic-dispersion (networking): Wavelength-dependent propagation velocity causing pulse broadening: Δτ = D·L·Δλ.
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
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [program-aided-language-model] Code generation errors (syntax, logic)
  - [program-aided-language-model] execution sandboxing
  - [program-aided-language-model] Python vs
  - [program-aided-language-model] LLM arithmetic (PAL reduces LLM math errors)
```

### Recipe #98 🌐 — score 444.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, computation-tree-logic, cutting-planes, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
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
```

### Recipe #99 🌐 — score 444.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, cutting-planes, graph-of-thought, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
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
```

### Recipe #100 🌐 — score 444.0

- **Primitives**: bdd-symbolic-model-checking, branch-and-cut, chain-of-thought, cutting-planes, linear-temporal-logic, merkle-commitment
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, formal-verification
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
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
```
