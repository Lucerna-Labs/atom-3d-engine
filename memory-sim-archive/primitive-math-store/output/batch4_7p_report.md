# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T00:08:28.563182+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog.json` (35 primitives)
- Seed pool: 30 primitives from 10 domains
- N-tuple size: 7
- Top recipes kept: 100
- Runtime: 50.79s

## Enumeration

- 7-tuple: 2,035,800

## Scoring summary

- Recipes scored: 2,035,800
- Max score: 531.0
- Mean score: 513.95
- Cross-domain: 2035799 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| nearest neighbor search | 1054024 | 51.8% |
| full sketch pipeline | 669138 | 32.9% |
| fusion ranking | 100804 | 5.0% |
| ranking by projection | 76476 | 3.8% |
| embedding ranking pipeline | 72794 | 3.6% |
| normalized fusion ranking | 40982 | 2.0% |
| normalized embedding pipeline | 20196 | 1.0% |
| sketch-based similarity | 1386 | 0.1% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| combinatorial-optimization | 1555100 |
| information-retrieval | 1378000 |
| causal-inference | 1378000 |
| cryptography-advanced | 1147770 |
| hashing | 1147770 |
| agentic-reasoning | 1147770 |
| machine-learning | 851760 |
| streaming | 851760 |
| networking | 851760 |
| formal-verification | 851760 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 2035800 |
| compare | 2034084 |
| fold | 2029365 |
| scale | 1958280 |
| hash | 1790643 |
| order | 1790643 |
| sample | 1555100 |
| project | 1147770 |
| scan | 851760 |

## Top 100 recipes

### Recipe #1 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
```

### Recipe #2 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, gradient-descent, k-median, lin-kernighan, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #3 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, set-packing
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #4 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #5 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #6 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, gradient-descent, lin-kernighan, set-packing, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #7 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, gradient-descent, lin-kernighan, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #8 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, k-median, lin-kernighan, lsa, set-packing
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #9 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, k-median, lin-kernighan, lsa, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #10 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, k-median, lin-kernighan, lsa, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #11 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, k-median, lin-kernighan, set-packing, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #12 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, k-median, lin-kernighan, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #13 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, lin-kernighan, lsa, set-packing, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #14 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, lin-kernighan, lsa, set-packing, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #15 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, lin-kernighan, lsa, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #16 🌐 — score 531.0

- **Primitives**: aggregate-signature, bm25, christofides-algorithm, lin-kernighan, set-packing, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #17 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa, set-packing
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #18 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #19 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #20 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, gradient-descent, k-median, lin-kernighan, set-packing, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #21 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, gradient-descent, k-median, lin-kernighan, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #22 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, gradient-descent, lin-kernighan, lsa, set-packing, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #23 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, gradient-descent, lin-kernighan, lsa, set-packing, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #24 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, gradient-descent, lin-kernighan, lsa, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #25 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, gradient-descent, lin-kernighan, set-packing, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval, machine-learning
- **Categories**: crypto, learning, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → learning → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #26 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, k-median, lin-kernighan, lsa, set-packing, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #27 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, k-median, lin-kernighan, lsa, set-packing, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #28 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, k-median, lin-kernighan, lsa, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #29 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, k-median, lin-kernighan, set-packing, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #30 🌐 — score 531.0

- **Primitives**: aggregate-signature, christofides-algorithm, lin-kernighan, lsa, set-packing, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #31 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
```

### Recipe #32 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, gradient-descent, k-median, lin-kernighan, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #33 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, set-packing
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #34 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #35 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #36 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, gradient-descent, lin-kernighan, set-packing, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #37 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, gradient-descent, lin-kernighan, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #38 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, k-median, lin-kernighan, lsa, set-packing
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #39 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, k-median, lin-kernighan, lsa, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #40 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, k-median, lin-kernighan, lsa, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #41 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, k-median, lin-kernighan, set-packing, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #42 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, k-median, lin-kernighan, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #43 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, lin-kernighan, lsa, set-packing, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #44 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, lin-kernighan, lsa, set-packing, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #45 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, lin-kernighan, lsa, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #46 🌐 — score 531.0

- **Primitives**: bloom-filter, bm25, christofides-algorithm, lin-kernighan, set-packing, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, scale, fold, order, scan, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #47 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa, set-packing
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: hash, combine, order, scan, fold, scale, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #48 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, order, scan, fold, scale, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #49 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, order, scan, fold, scale, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #50 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, gradient-descent, k-median, lin-kernighan, set-packing, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: hash, combine, order, scan, fold, scale, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #51 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, gradient-descent, k-median, lin-kernighan, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, order, scan, fold, scale, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #52 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, gradient-descent, lin-kernighan, lsa, set-packing, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, order, scan, fold, scale, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #53 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, gradient-descent, lin-kernighan, lsa, set-packing, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, order, scan, fold, scale, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #54 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, gradient-descent, lin-kernighan, lsa, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: hash, combine, order, scan, fold, scale, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #55 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, gradient-descent, lin-kernighan, set-packing, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, order, scan, fold, scale, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #56 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, k-median, lin-kernighan, lsa, set-packing, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: hash, combine, order, scan, fold, compare, sample, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #57 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, k-median, lin-kernighan, lsa, set-packing, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: hash, combine, order, scan, fold, compare, sample, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #58 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, k-median, lin-kernighan, lsa, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, order, scan, fold, compare, sample, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #59 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, k-median, lin-kernighan, set-packing, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: hash, combine, order, scan, fold, compare, sample, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #60 🌐 — score 531.0

- **Primitives**: bloom-filter, christofides-algorithm, lin-kernighan, lsa, set-packing, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: hash, combine, order, scan, fold, compare, sample, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #61 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, gradient-descent, k-median, lin-kernighan, lsa
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
```

### Recipe #62 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, gradient-descent, k-median, lin-kernighan, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #63 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, gradient-descent, lin-kernighan, lsa, set-packing
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #64 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, gradient-descent, lin-kernighan, lsa, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #65 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, gradient-descent, lin-kernighan, lsa, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #66 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, gradient-descent, lin-kernighan, set-packing, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #67 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, gradient-descent, lin-kernighan, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #68 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, k-median, lin-kernighan, lsa, set-packing
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #69 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, k-median, lin-kernighan, lsa, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #70 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, k-median, lin-kernighan, lsa, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #71 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, k-median, lin-kernighan, set-packing, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 3 composite + 4 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #72 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, k-median, lin-kernighan, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #73 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, lin-kernighan, lsa, set-packing, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #74 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, lin-kernighan, lsa, set-packing, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #75 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, lin-kernighan, lsa, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #76 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, count-min-sketch, lin-kernighan, set-packing, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #77 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, hyperloglog, k-median, lin-kernighan, lsa
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
```

### Recipe #78 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, hyperloglog, k-median, lin-kernighan, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #79 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, hyperloglog, lin-kernighan, lsa, set-packing
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #80 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, hyperloglog, lin-kernighan, lsa, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #81 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, hyperloglog, lin-kernighan, lsa, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #82 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, hyperloglog, lin-kernighan, set-packing, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #83 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, hyperloglog, lin-kernighan, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval, machine-learning, streaming
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, hash, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #84 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa, lsh
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
```

### Recipe #85 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa, minhash
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
```

### Recipe #86 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsa, sdn-data-plane-programming
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, networking
- **Categories**: learning, optimization, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → systems
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
```

### Recipe #87 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, k-median, lin-kernighan, lsh, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, hash, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #88 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, k-median, lin-kernighan, minhash, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, hash, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #89 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, k-median, lin-kernighan, sdn-data-plane-programming, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, networking
- **Categories**: learning, optimization, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, hash, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → systems
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [k-median] NP-hard; LP relaxation gap can be large.
  - [k-median] Primal-dual + clustering gives practical results.
  - [k-median] Closely related to k-means; metric k-median has PTAS in geometric settings.
  - [k-median] Local search (1+epsilon)-approximation requires O(log n) swaps.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #90 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, lsh, set-packing
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #91 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, lsh, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #92 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, lsh, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #93 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, minhash, set-packing
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #94 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, minhash, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #95 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, minhash, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #96 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, sdn-data-plane-programming, set-packing
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, networking
- **Categories**: learning, optimization, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → systems
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
```

### Recipe #97 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, sdn-data-plane-programming, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval, machine-learning, networking
- **Categories**: learning, optimization, reasoning, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → systems
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #98 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsa, sdn-data-plane-programming, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning, networking
- **Categories**: learning, optimization, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, project, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → systems
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #99 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsh, set-packing, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 2 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, hash, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [set-packing] General set packing is NP-hard; 3D matching is NP-complete.
  - [set-packing] Maximum independent set in hypergraphs is harder than in graphs.
  - [set-packing] Solved via branch-and-price with column generation.
  - [set-packing] LP relaxation can have integrality gap arbitrarily close to sqrt(n).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #100 🌐 — score 531.0

- **Primitives**: bm25, christofides-algorithm, gradient-descent, lin-kernighan, lsh, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, hashing, information-retrieval, machine-learning
- **Categories**: learning, optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, order, scan, compare, sample, hash, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** learning → optimization → reasoning → retrieval → sketching
**Components:**
  - bm25 (information-retrieval): Probabilistic relevance ranking with term frequency saturation and document length normalization.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [bm25] BM25F (fielded) is needed for structured documents.
  - [bm25] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25] Fails on long-tail terms without IDF smoothing.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [gradient-descent] Convergence rate depends on condition number of Hessian.
  - [gradient-descent] Saddle points in non-convex landscapes slow progress.
  - [gradient-descent] Step size scheduling is problem-dependent.
  - [gradient-descent] Stochastic variant has variance; reduces with batch size.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```
