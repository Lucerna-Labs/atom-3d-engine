# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T01:36:31.542671+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog_full.json` (939 primitives)
- Seed pool: 50 primitives from 4 domains
- N-tuple size: 4
- Top recipes kept: 50
- Runtime: 2.89s

## Enumeration

- 4-tuple: 230,300

## Scoring summary

- Recipes scored: 230,300
- Max score: 330.0
- Mean score: 295.66
- Cross-domain: 226200 (98.2%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 230300 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| causal-inference | 177940 |
| combinatorial-optimization | 177940 |
| agentic-reasoning | 177940 |
| cryptography-advanced | 81305 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 230230 |
| sample | 81305 |
| project | 51935 |
| order | 51935 |
| scale | 35720 |
| compare | 35720 |
| hash | 18424 |

## Top 50 recipes

### Recipe #1 🌐 — score 330.0

- **Primitives**: cutting-planes, doubly-robust-estimation, merkle-commitment, pedersen-commitment
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, order, combine, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #2 🌐 — score 330.0

- **Primitives**: cutting-planes, doubly-robust-estimation, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #3 🌐 — score 330.0

- **Primitives**: cutting-planes, doubly-robust-estimation, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #4 🌐 — score 330.0

- **Primitives**: cutting-planes, gomory-cuts, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, order, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #5 🌐 — score 330.0

- **Primitives**: cutting-planes, gomory-cuts, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, order, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #6 🌐 — score 330.0

- **Primitives**: cutting-planes, inverse-probability-weighting, merkle-commitment, pedersen-commitment
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, order, combine, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #7 🌐 — score 330.0

- **Primitives**: cutting-planes, inverse-probability-weighting, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #8 🌐 — score 330.0

- **Primitives**: cutting-planes, inverse-probability-weighting, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #9 🌐 — score 330.0

- **Primitives**: cutting-planes, merkle-commitment, pedersen-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, order, combine, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #10 🌐 — score 330.0

- **Primitives**: cutting-planes, merkle-commitment, pedersen-commitment, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, order, combine, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #11 🌐 — score 330.0

- **Primitives**: cutting-planes, merkle-commitment, preprocessing-ip, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, order, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - preprocessing-ip (combinatorial-optimization): IP preprocessing: presolve (bounds tightening, variable elimination, probing). Reduce problem size before branching.
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [preprocessing-ip] Aggressive presolve can identify infeasibility early
  - [preprocessing-ip] may change problem structure (SOS, special ordered sets)
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #12 🌐 — score 330.0

- **Primitives**: cutting-planes, merkle-commitment, preprocessing-ip, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, order, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - preprocessing-ip (combinatorial-optimization): IP preprocessing: presolve (bounds tightening, variable elimination, probing). Reduce problem size before branching.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [preprocessing-ip] Aggressive presolve can identify infeasibility early
  - [preprocessing-ip] may change problem structure (SOS, special ordered sets)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #13 🌐 — score 329.0

- **Primitives**: cutting-planes, merkle-commitment, pedersen-commitment, program-aided-language-model
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, order, combine, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
**Real walls (combined):**
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

### Recipe #14 🌐 — score 329.0

- **Primitives**: cutting-planes, merkle-commitment, program-aided-language-model, propensity-score-matching
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
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
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #15 🌐 — score 329.0

- **Primitives**: cutting-planes, merkle-commitment, program-aided-language-model, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - program-aided-language-model (agentic-reasoning): PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
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

### Recipe #16 🌐 — score 328.0

- **Primitives**: cutting-planes, doubly-robust-estimation, gomory-cuts, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, order, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #17 🌐 — score 328.0

- **Primitives**: cutting-planes, doubly-robust-estimation, pedersen-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, order, combine, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #18 🌐 — score 328.0

- **Primitives**: cutting-planes, doubly-robust-estimation, pedersen-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, order, combine, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #19 🌐 — score 328.0

- **Primitives**: cutting-planes, doubly-robust-estimation, preprocessing-ip, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, order, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - preprocessing-ip (combinatorial-optimization): IP preprocessing: presolve (bounds tightening, variable elimination, probing). Reduce problem size before branching.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [preprocessing-ip] Aggressive presolve can identify infeasibility early
  - [preprocessing-ip] may change problem structure (SOS, special ordered sets)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #20 🌐 — score 328.0

- **Primitives**: cutting-planes, gomory-cuts, inverse-probability-weighting, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, order, scale, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #21 🌐 — score 328.0

- **Primitives**: cutting-planes, inverse-probability-weighting, pedersen-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, order, combine, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #22 🌐 — score 328.0

- **Primitives**: cutting-planes, inverse-probability-weighting, pedersen-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, order, combine, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #23 🌐 — score 328.0

- **Primitives**: cutting-planes, inverse-probability-weighting, preprocessing-ip, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, order, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization
**Components:**
  - cutting-planes (combinatorial-optimization): Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - preprocessing-ip (combinatorial-optimization): IP preprocessing: presolve (bounds tightening, variable elimination, probing). Reduce problem size before branching.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [cutting-planes] Different problem classes have specialized cuts
  - [cutting-planes] TSP has subtour elimination
  - [cutting-planes] packing has cover inequalities
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [preprocessing-ip] Aggressive presolve can identify infeasibility early
  - [preprocessing-ip] may change problem structure (SOS, special ordered sets)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #24 🌐 — score 328.0

- **Primitives**: disjunctive-programming, doubly-robust-estimation, merkle-commitment, pedersen-commitment
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, order, combine, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #25 🌐 — score 328.0

- **Primitives**: disjunctive-programming, doubly-robust-estimation, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #26 🌐 — score 328.0

- **Primitives**: disjunctive-programming, doubly-robust-estimation, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #27 🌐 — score 328.0

- **Primitives**: disjunctive-programming, gomory-cuts, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, order, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #28 🌐 — score 328.0

- **Primitives**: disjunctive-programming, gomory-cuts, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, order, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #29 🌐 — score 328.0

- **Primitives**: disjunctive-programming, inverse-probability-weighting, merkle-commitment, pedersen-commitment
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, order, combine, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #30 🌐 — score 328.0

- **Primitives**: disjunctive-programming, inverse-probability-weighting, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #31 🌐 — score 328.0

- **Primitives**: disjunctive-programming, inverse-probability-weighting, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, scale, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #32 🌐 — score 328.0

- **Primitives**: disjunctive-programming, merkle-commitment, pedersen-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, order, combine, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #33 🌐 — score 328.0

- **Primitives**: disjunctive-programming, merkle-commitment, pedersen-commitment, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, order, combine, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #34 🌐 — score 328.0

- **Primitives**: disjunctive-programming, merkle-commitment, preprocessing-ip, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, order, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - preprocessing-ip (combinatorial-optimization): IP preprocessing: presolve (bounds tightening, variable elimination, probing). Reduce problem size before branching.
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [preprocessing-ip] Aggressive presolve can identify infeasibility early
  - [preprocessing-ip] may change problem structure (SOS, special ordered sets)
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #35 🌐 — score 328.0

- **Primitives**: disjunctive-programming, merkle-commitment, preprocessing-ip, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, hash, order, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced
**Components:**
  - disjunctive-programming (combinatorial-optimization): Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - preprocessing-ip (combinatorial-optimization): IP preprocessing: presolve (bounds tightening, variable elimination, probing). Reduce problem size before branching.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [disjunctive-programming] Disjunctive cuts generalize lift-and-project
  - [disjunctive-programming] extended formulation may be smaller than convex hull projection
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [preprocessing-ip] Aggressive presolve can identify infeasibility early
  - [preprocessing-ip] may change problem structure (SOS, special ordered sets)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #36 🌐 — score 328.0

- **Primitives**: doubly-robust-estimation, gomory-cuts, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, order, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #37 🌐 — score 328.0

- **Primitives**: doubly-robust-estimation, gomory-cuts, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, order, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #38 🌐 — score 328.0

- **Primitives**: doubly-robust-estimation, lift-and-project, merkle-commitment, pedersen-commitment
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, project, hash, order, combine, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - lift-and-project (combinatorial-optimization): Lift-and-project (Balas-Perduc): iteratively lift 0-1 variables to higher-dimensional spaces. Add binary constraint x_i + x_j ≥ 1 in projection.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [lift-and-project] Lift-and-project is effective for small 0-1 IPs
  - [lift-and-project] too expensive for large IPs without preprocessing
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #39 🌐 — score 328.0

- **Primitives**: doubly-robust-estimation, lift-and-project, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, project, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - lift-and-project (combinatorial-optimization): Lift-and-project (Balas-Perduc): iteratively lift 0-1 variables to higher-dimensional spaces. Add binary constraint x_i + x_j ≥ 1 in projection.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [lift-and-project] Lift-and-project is effective for small 0-1 IPs
  - [lift-and-project] too expensive for large IPs without preprocessing
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #40 🌐 — score 328.0

- **Primitives**: doubly-robust-estimation, lift-and-project, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, project, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - lift-and-project (combinatorial-optimization): Lift-and-project (Balas-Perduc): iteratively lift 0-1 variables to higher-dimensional spaces. Add binary constraint x_i + x_j ≥ 1 in projection.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [lift-and-project] Lift-and-project is effective for small 0-1 IPs
  - [lift-and-project] too expensive for large IPs without preprocessing
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #41 🌐 — score 328.0

- **Primitives**: doubly-robust-estimation, merkle-commitment, pedersen-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, hash, order, combine, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → cryptography-advanced
**Components:**
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #42 🌐 — score 328.0

- **Primitives**: doubly-robust-estimation, merkle-commitment, preprocessing-ip, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, hash, order, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - preprocessing-ip (combinatorial-optimization): IP preprocessing: presolve (bounds tightening, variable elimination, probing). Reduce problem size before branching.
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [preprocessing-ip] Aggressive presolve can identify infeasibility early
  - [preprocessing-ip] may change problem structure (SOS, special ordered sets)
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #43 🌐 — score 328.0

- **Primitives**: doubly-robust-estimation, merkle-commitment, preprocessing-ip, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, hash, order, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - doubly-robust-estimation (causal-inference): Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - preprocessing-ip (combinatorial-optimization): IP preprocessing: presolve (bounds tightening, variable elimination, probing). Reduce problem size before branching.
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [doubly-robust-estimation] DR estimators have lower variance than pure IPW
  - [doubly-robust-estimation] both models wrong → bias (not DR)
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [preprocessing-ip] Aggressive presolve can identify infeasibility early
  - [preprocessing-ip] may change problem structure (SOS, special ordered sets)
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #44 🌐 — score 328.0

- **Primitives**: gomory-cuts, inverse-probability-weighting, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: order, scale, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #45 🌐 — score 328.0

- **Primitives**: gomory-cuts, inverse-probability-weighting, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: order, scale, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #46 🌐 — score 328.0

- **Primitives**: gomory-cuts, lift-and-project, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: order, project, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - lift-and-project (combinatorial-optimization): Lift-and-project (Balas-Perduc): iteratively lift 0-1 variables to higher-dimensional spaces. Add binary constraint x_i + x_j ≥ 1 in projection.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [lift-and-project] Lift-and-project is effective for small 0-1 IPs
  - [lift-and-project] too expensive for large IPs without preprocessing
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #47 🌐 — score 328.0

- **Primitives**: gomory-cuts, lift-and-project, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: order, project, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → combinatorial-optimization → cryptography-advanced
**Components:**
  - gomory-cuts (combinatorial-optimization): Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
  - lift-and-project (combinatorial-optimization): Lift-and-project (Balas-Perduc): iteratively lift 0-1 variables to higher-dimensional spaces. Add binary constraint x_i + x_j ≥ 1 in projection.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [gomory-cuts] GMI cuts are universal (valid for all IPs)
  - [gomory-cuts] problem-specific cuts (clique, cover) can be stronger
  - [lift-and-project] Lift-and-project is effective for small 0-1 IPs
  - [lift-and-project] too expensive for large IPs without preprocessing
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```

### Recipe #48 🌐 — score 328.0

- **Primitives**: inverse-probability-weighting, lift-and-project, merkle-commitment, pedersen-commitment
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, project, hash, order, combine, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - lift-and-project (combinatorial-optimization): Lift-and-project (Balas-Perduc): iteratively lift 0-1 variables to higher-dimensional spaces. Add binary constraint x_i + x_j ≥ 1 in projection.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - pedersen-commitment (cryptography-advanced): Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
**Real walls (combined):**
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [lift-and-project] Lift-and-project is effective for small 0-1 IPs
  - [lift-and-project] too expensive for large IPs without preprocessing
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [pedersen-commitment] Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown
  - [pedersen-commitment] no known trapdoor
```

### Recipe #49 🌐 — score 328.0

- **Primitives**: inverse-probability-weighting, lift-and-project, merkle-commitment, propensity-score-matching
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, project, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - lift-and-project (combinatorial-optimization): Lift-and-project (Balas-Perduc): iteratively lift 0-1 variables to higher-dimensional spaces. Add binary constraint x_i + x_j ≥ 1 in projection.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - propensity-score-matching (causal-inference): Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
**Real walls (combined):**
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [lift-and-project] Lift-and-project is effective for small 0-1 IPs
  - [lift-and-project] too expensive for large IPs without preprocessing
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [propensity-score-matching] Matching without replacement can improve balance but reduces effective sample size
  - [propensity-score-matching] with replacement may reuse controls
```

### Recipe #50 🌐 — score 328.0

- **Primitives**: inverse-probability-weighting, lift-and-project, merkle-commitment, self-consistency
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: scale, project, hash, combine, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** agentic-reasoning → causal-inference → combinatorial-optimization → cryptography-advanced
**Components:**
  - inverse-probability-weighting (causal-inference): IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
  - lift-and-project (combinatorial-optimization): Lift-and-project (Balas-Perduc): iteratively lift 0-1 variables to higher-dimensional spaces. Add binary constraint x_i + x_j ≥ 1 in projection.
  - merkle-commitment (cryptography-advanced): Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
  - self-consistency (agentic-reasoning): Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
**Real walls (combined):**
  - [inverse-probability-weighting] IPW unstable when propensity scores near 0 or 1 (extreme weights)
  - [inverse-probability-weighting] truncate weights or use stabilized weights
  - [lift-and-project] Lift-and-project is effective for small 0-1 IPs
  - [lift-and-project] too expensive for large IPs without preprocessing
  - [merkle-commitment] Tree construction O(n) for n data items
  - [merkle-commitment] bandwidth of proofs
  - [merkle-commitment] not additive-homomorphic
  - [self-consistency] Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge)
  - [self-consistency] marginal gains after n=10–20
```
