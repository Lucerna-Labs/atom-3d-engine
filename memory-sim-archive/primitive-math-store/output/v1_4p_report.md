# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-23T21:37:49.139197+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog.json` (35 primitives)
- Seed pool: 30 primitives from 10 domains
- N-tuple size: 4
- Top recipes kept: 50
- Runtime: 0.39s

## Enumeration

- 4-tuple: 27,405

## Scoring summary

- Recipes scored: 27,405
- Max score: 357.0
- Mean score: 335.06
- Cross-domain: 27359 (99.8%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| nearest neighbor search | 12771 | 46.6% |
| fusion ranking | 6321 | 23.1% |
| full sketch pipeline | 3199 | 11.7% |
| ranking by projection | 2405 | 8.8% |
| normalized fusion ranking | 1140 | 4.2% |
| embedding ranking pipeline | 859 | 3.1% |
| sketch-based similarity | 386 | 1.4% |
| normalized embedding pipeline | 324 | 1.2% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| combinatorial-optimization | 14755 |
| information-retrieval | 12455 |
| causal-inference | 12455 |
| cryptography-advanced | 9855 |
| hashing | 9855 |
| agentic-reasoning | 9855 |
| streaming | 6930 |
| networking | 6930 |
| machine-learning | 6930 |
| formal-verification | 6930 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 27404 |
| compare | 26690 |
| fold | 26040 |
| scale | 22560 |
| hash | 18550 |
| order | 18550 |
| sample | 14755 |
| project | 9855 |
| scan | 6930 |

## Top 50 recipes

### Recipe #1 🌐 — score 357.0

- **Primitives**: aggregate-signature, christofides-algorithm, lin-kernighan, lsa
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
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
```

### Recipe #2 🌐 — score 357.0

- **Primitives**: aggregate-signature, christofides-algorithm, lin-kernighan, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
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
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #3 🌐 — score 357.0

- **Primitives**: bloom-filter, christofides-algorithm, lin-kernighan, lsa
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, compare, sample, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
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
```

### Recipe #4 🌐 — score 357.0

- **Primitives**: bloom-filter, christofides-algorithm, lin-kernighan, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, compare, sample, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
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
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #5 🌐 — score 357.0

- **Primitives**: christofides-algorithm, count-min-sketch, lin-kernighan, lsa
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, sample, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
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
```

### Recipe #6 🌐 — score 357.0

- **Primitives**: christofides-algorithm, count-min-sketch, lin-kernighan, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, sample, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
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
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #7 🌐 — score 357.0

- **Primitives**: christofides-algorithm, hyperloglog, lin-kernighan, lsa
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, sample, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
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
```

### Recipe #8 🌐 — score 357.0

- **Primitives**: christofides-algorithm, hyperloglog, lin-kernighan, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, sample, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #9 🌐 — score 357.0

- **Primitives**: christofides-algorithm, lin-kernighan, lsa, lsh
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, compare, sample, project, scale, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
**Real walls (combined):**
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
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
```

### Recipe #10 🌐 — score 357.0

- **Primitives**: christofides-algorithm, lin-kernighan, lsa, minhash
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, compare, sample, project, scale, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
**Real walls (combined):**
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
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
```

### Recipe #11 🌐 — score 357.0

- **Primitives**: christofides-algorithm, lin-kernighan, lsa, sdn-data-plane-programming
- **Domains**: combinatorial-optimization, information-retrieval, networking
- **Categories**: optimization, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, compare, sample, project, scale, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
**Real walls (combined):**
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
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
```

### Recipe #12 🌐 — score 357.0

- **Primitives**: christofides-algorithm, lin-kernighan, lsh, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, compare, sample, hash, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
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

### Recipe #13 🌐 — score 357.0

- **Primitives**: christofides-algorithm, lin-kernighan, minhash, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, compare, sample, hash, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
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

### Recipe #14 🌐 — score 357.0

- **Primitives**: christofides-algorithm, lin-kernighan, sdn-data-plane-programming, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, networking
- **Categories**: optimization, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, compare, sample, hash, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
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

### Recipe #15 🌐 — score 355.0

- **Primitives**: aggregate-signature, christofides-algorithm, counterfactual-imagination, lsa
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, sample, compare, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
```

### Recipe #16 🌐 — score 355.0

- **Primitives**: aggregate-signature, christofides-algorithm, counterfactual-imagination, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, sample, compare, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
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
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #17 🌐 — score 355.0

- **Primitives**: aggregate-signature, christofides-algorithm, instrumental-variable, lin-kernighan
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: crypto, optimization, reasoning
- **Wiring**: fusion ranking
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, project, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** crypto → optimization → reasoning
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
```

### Recipe #18 🌐 — score 355.0

- **Primitives**: aggregate-signature, christofides-algorithm, lsa, monte-carlo-tree-search
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, project, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
```

### Recipe #19 🌐 — score 355.0

- **Primitives**: aggregate-signature, christofides-algorithm, lsa, tcp-timestamp
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, networking
- **Categories**: crypto, optimization, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, project, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → retrieval → systems
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
```

### Recipe #20 🌐 — score 355.0

- **Primitives**: aggregate-signature, christofides-algorithm, lsa, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, project, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #21 🌐 — score 355.0

- **Primitives**: aggregate-signature, christofides-algorithm, monte-carlo-tree-search, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, sample, compare, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
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
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #22 🌐 — score 355.0

- **Primitives**: aggregate-signature, christofides-algorithm, tcp-timestamp, tf-idf
- **Domains**: combinatorial-optimization, cryptography-advanced, information-retrieval, networking
- **Categories**: crypto, optimization, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, compare, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → retrieval → systems
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
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
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #23 🌐 — score 355.0

- **Primitives**: aggregate-signature, christofides-algorithm, tf-idf, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, cryptography-advanced, information-retrieval
- **Categories**: crypto, optimization, reasoning, retrieval
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, project, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** crypto → optimization → reasoning → retrieval
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [aggregate-signature] Pairing-based BLS requires structured curves; not quantum-safe.
  - [aggregate-signature] Rogue public key attacks need proof-of-possession.
  - [aggregate-signature] Verification is one pairing; faster than n individual verifies.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #24 🌐 — score 355.0

- **Primitives**: bloom-filter, christofides-algorithm, counterfactual-imagination, lsa
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, sample, compare, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
```

### Recipe #25 🌐 — score 355.0

- **Primitives**: bloom-filter, christofides-algorithm, counterfactual-imagination, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, sample, compare, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
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
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #26 🌐 — score 355.0

- **Primitives**: bloom-filter, christofides-algorithm, instrumental-variable, lin-kernighan
- **Domains**: causal-inference, combinatorial-optimization, hashing
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, project, scale, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
```

### Recipe #27 🌐 — score 355.0

- **Primitives**: bloom-filter, christofides-algorithm, lsa, monte-carlo-tree-search
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, project, scale, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
```

### Recipe #28 🌐 — score 355.0

- **Primitives**: bloom-filter, christofides-algorithm, lsa, tcp-timestamp
- **Domains**: combinatorial-optimization, hashing, information-retrieval, networking
- **Categories**: optimization, retrieval, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, project, scale, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching → systems
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
```

### Recipe #29 🌐 — score 355.0

- **Primitives**: bloom-filter, christofides-algorithm, lsa, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, project, scale, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #30 🌐 — score 355.0

- **Primitives**: bloom-filter, christofides-algorithm, monte-carlo-tree-search, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, sample, compare, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
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
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #31 🌐 — score 355.0

- **Primitives**: bloom-filter, christofides-algorithm, tcp-timestamp, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, networking
- **Categories**: optimization, retrieval, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, compare, scale, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching → systems
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
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
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #32 🌐 — score 355.0

- **Primitives**: bloom-filter, christofides-algorithm, tf-idf, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, scale, project, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [bloom-filter] False positive rate: (1 - e^{-kn/m})^k.
  - [bloom-filter] Cannot delete (counting Bloom filter for that, but bigger).
  - [bloom-filter] Sizing tradeoff: more bits = fewer FPs, more memory.
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #33 🌐 — score 355.0

- **Primitives**: christofides-algorithm, count-min-sketch, counterfactual-imagination, lsa
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, sample, compare, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
```

### Recipe #34 🌐 — score 355.0

- **Primitives**: christofides-algorithm, count-min-sketch, counterfactual-imagination, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, sample, compare, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #35 🌐 — score 355.0

- **Primitives**: christofides-algorithm, count-min-sketch, instrumental-variable, lin-kernighan
- **Domains**: causal-inference, combinatorial-optimization, streaming
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, project, scale, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
```

### Recipe #36 🌐 — score 355.0

- **Primitives**: christofides-algorithm, count-min-sketch, lsa, monte-carlo-tree-search
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, project, scale, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
```

### Recipe #37 🌐 — score 355.0

- **Primitives**: christofides-algorithm, count-min-sketch, lsa, tcp-timestamp
- **Domains**: combinatorial-optimization, information-retrieval, networking, streaming
- **Categories**: optimization, retrieval, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, project, scale, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
```

### Recipe #38 🌐 — score 355.0

- **Primitives**: christofides-algorithm, count-min-sketch, lsa, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, project, scale, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #39 🌐 — score 355.0

- **Primitives**: christofides-algorithm, count-min-sketch, monte-carlo-tree-search, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, sample, compare, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #40 🌐 — score 355.0

- **Primitives**: christofides-algorithm, count-min-sketch, tcp-timestamp, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, networking, streaming
- **Categories**: optimization, retrieval, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, scale, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #41 🌐 — score 355.0

- **Primitives**: christofides-algorithm, count-min-sketch, tf-idf, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, scale, project, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [count-min-sketch] Overestimates (never underestimates) frequencies.
  - [count-min-sketch] Heavy hitters need separate detection (Misra-Gries).
  - [count-min-sketch] Cannot answer negative queries; adversarial inputs can saturate.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #42 🌐 — score 355.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, hyperloglog, lsa
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, hash, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
```

### Recipe #43 🌐 — score 355.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, hyperloglog, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, hash, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #44 🌐 — score 355.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, lsa, lsh
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, project, scale, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
```

### Recipe #45 🌐 — score 355.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, lsa, minhash
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, project, scale, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
```

### Recipe #46 🌐 — score 355.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, lsa, sdn-data-plane-programming
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, networking
- **Categories**: optimization, reasoning, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, project, scale, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
```

### Recipe #47 🌐 — score 355.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, lsh, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, hash, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #48 🌐 — score 355.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, minhash, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, hash, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #49 🌐 — score 355.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, sdn-data-plane-programming, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, networking
- **Categories**: optimization, reasoning, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, hash, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [counterfactual-imagination] Combinatorial explosion: branching factor is the ceiling.
  - [counterfactual-imagination] Counterfactual reasoning is sensitive to graph topology.
  - [counterfactual-imagination] Pearl's do-calculus identifies identifiable queries; many are not.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #50 🌐 — score 355.0

- **Primitives**: christofides-algorithm, hyperloglog, instrumental-variable, lin-kernighan
- **Domains**: causal-inference, combinatorial-optimization, streaming
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, project, scale, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [lin-kernighan] Optimal k unknown a priori; algorithm adapts k at each step.
  - [lin-kernighan] Dominated by Helsgaun's LKH (alpha-nearness, richer moves) on TSPLIB.
  - [lin-kernighan] No polynomial worst-case guarantee; can cycle without tabu list.
  - [lin-kernighan] Sensitive to initial tour quality.
```
