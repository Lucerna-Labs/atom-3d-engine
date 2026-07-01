# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T00:07:02.554259+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\primitive_catalog.json` (35 primitives)
- Seed pool: 30 primitives from 10 domains
- N-tuple size: 4
- Top recipes kept: 100
- Runtime: 0.37s

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

## Top 100 recipes

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

### Recipe #51 🌐 — score 355.0

- **Primitives**: christofides-algorithm, hyperloglog, lsa, monte-carlo-tree-search
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, project, scale, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
```

### Recipe #52 🌐 — score 355.0

- **Primitives**: christofides-algorithm, hyperloglog, lsa, tcp-timestamp
- **Domains**: combinatorial-optimization, information-retrieval, networking, streaming
- **Categories**: optimization, retrieval, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, project, scale, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
```

### Recipe #53 🌐 — score 355.0

- **Primitives**: christofides-algorithm, hyperloglog, lsa, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, project, scale, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #54 🌐 — score 355.0

- **Primitives**: christofides-algorithm, hyperloglog, monte-carlo-tree-search, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, sample, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
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
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #55 🌐 — score 355.0

- **Primitives**: christofides-algorithm, hyperloglog, tcp-timestamp, tf-idf
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
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
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
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #56 🌐 — score 355.0

- **Primitives**: christofides-algorithm, hyperloglog, tf-idf, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, scale, project, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [hyperloglog] Bias correction needed for small cardinalities.
  - [hyperloglog] Sparse representation helps for low values.
  - [hyperloglog] Merge requires same precision parameter.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #57 🌐 — score 355.0

- **Primitives**: christofides-algorithm, instrumental-variable, lin-kernighan, lsh
- **Domains**: causal-inference, combinatorial-optimization, hashing
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, compare, sample, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
**Real walls (combined):**
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
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
```

### Recipe #58 🌐 — score 355.0

- **Primitives**: christofides-algorithm, instrumental-variable, lin-kernighan, minhash
- **Domains**: causal-inference, combinatorial-optimization, hashing
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, compare, sample, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
**Real walls (combined):**
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
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
```

### Recipe #59 🌐 — score 355.0

- **Primitives**: christofides-algorithm, instrumental-variable, lin-kernighan, sdn-data-plane-programming
- **Domains**: causal-inference, combinatorial-optimization, networking
- **Categories**: optimization, reasoning, systems
- **Wiring**: fusion ranking
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, compare, sample, hash

```
**Wiring:** fusion ranking
**Cross-domain:** optimization → reasoning → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
**Real walls (combined):**
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
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
```

### Recipe #60 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsa, lsh, monte-carlo-tree-search
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, hash, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
```

### Recipe #61 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsa, lsh, tcp-timestamp
- **Domains**: combinatorial-optimization, hashing, information-retrieval, networking
- **Categories**: optimization, retrieval, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, hash, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
```

### Recipe #62 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsa, lsh, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, hash, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #63 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsa, minhash, monte-carlo-tree-search
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, hash, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
```

### Recipe #64 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsa, minhash, tcp-timestamp
- **Domains**: combinatorial-optimization, hashing, information-retrieval, networking
- **Categories**: optimization, retrieval, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, hash, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
```

### Recipe #65 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsa, minhash, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, hash, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #66 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsa, monte-carlo-tree-search, sdn-data-plane-programming
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, networking
- **Categories**: optimization, reasoning, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, sample, compare, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
**Real walls (combined):**
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
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
```

### Recipe #67 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsa, sdn-data-plane-programming, tcp-timestamp
- **Domains**: combinatorial-optimization, information-retrieval, networking
- **Categories**: optimization, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, compare, hash, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
```

### Recipe #68 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsa, sdn-data-plane-programming, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, networking
- **Categories**: optimization, reasoning, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, project, scale, compare, hash, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsa] Topic-level match, not fine-grained semantics.
  - [lsa] k must be tuned per corpus; SVD is O(min(M,N) * M * N).
  - [lsa] Sensitive to preprocessing (stemming, stopwords).
  - [lsa] Folds 'random' and 'mix' too close in some corpora.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #69 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsh, monte-carlo-tree-search, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, sample, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #70 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsh, tcp-timestamp, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, networking
- **Categories**: optimization, retrieval, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, scale, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #71 🌐 — score 355.0

- **Primitives**: christofides-algorithm, lsh, tf-idf, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, scale, project, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lsh (hashing): Locality-Sensitive Hashing: amplify hash collisions to find approximate nearest neighbors in sublinear time.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #72 🌐 — score 355.0

- **Primitives**: christofides-algorithm, minhash, monte-carlo-tree-search, tf-idf
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, sample, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #73 🌐 — score 355.0

- **Primitives**: christofides-algorithm, minhash, tcp-timestamp, tf-idf
- **Domains**: combinatorial-optimization, hashing, information-retrieval, networking
- **Categories**: optimization, retrieval, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, scale, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #74 🌐 — score 355.0

- **Primitives**: christofides-algorithm, minhash, tf-idf, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, hashing, information-retrieval
- **Categories**: optimization, reasoning, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, compare, scale, project, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - minhash (hashing): Approximate Jaccard similarity via k random hash signatures; min across each row.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #75 🌐 — score 355.0

- **Primitives**: christofides-algorithm, monte-carlo-tree-search, sdn-data-plane-programming, tf-idf
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
  - monte-carlo-tree-search (agentic-reasoning): Best-first tree search with UCB selection; balance exploration and exploitation.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #76 🌐 — score 355.0

- **Primitives**: christofides-algorithm, sdn-data-plane-programming, tcp-timestamp, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, networking
- **Categories**: optimization, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, compare, hash, scale, sample, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - tcp-timestamp (networking): Per-packet timestamp option for RTT estimation and PAWS (Protect Against Wrapped Sequences).
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #77 🌐 — score 355.0

- **Primitives**: christofides-algorithm, sdn-data-plane-programming, tf-idf, tree-of-thought
- **Domains**: agentic-reasoning, combinatorial-optimization, information-retrieval, networking
- **Categories**: optimization, reasoning, retrieval, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, compare, hash, scale, project, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → retrieval → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - sdn-data-plane-programming (networking): P4-style programmable data plane for per-packet computation at line rate.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
  - tree-of-thought (agentic-reasoning): LLM reasoning with explicit search tree; expand/prune branches using self-evaluation.
**Real walls (combined):**
  - [christofides-algorithm] 1.5-approximation is best known deterministic for metric TSP; hardness gap is unresolved.
  - [christofides-algorithm] Requires triangle inequality; arbitrary edge weights are not handled.
  - [christofides-algorithm] MST step O(E log V); matching O(V^3); total O(V^3).
  - [christofides-algorithm] Random-MST (substituting random for matching) often improves in practice.
  - [christofides-algorithm] Dominated by maximum adjacency ordering in some real instances.
  - [sdn-data-plane-programming] P4-16 has bounded externs; stateful memory is finite.
  - [sdn-data-plane-programming] Compiler optimization is crucial for table utilization.
  - [sdn-data-plane-programming] Recompilation requires hardware support (Tofino, etc.).
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #78 🌐 — score 355.0

- **Primitives**: count-min-sketch, k-median, lin-kernighan, lsa
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, scan, fold, compare, order, sample, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
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
```

### Recipe #79 🌐 — score 355.0

- **Primitives**: count-min-sketch, k-median, lin-kernighan, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, scan, fold, compare, order, sample, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - k-median (combinatorial-optimization): Cluster points into k groups minimizing sum of distances to nearest center.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
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
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #80 🌐 — score 355.0

- **Primitives**: count-min-sketch, lin-kernighan, lsa, set-packing
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, scan, compare, sample, project, scale, fold, order

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
**Real walls (combined):**
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
```

### Recipe #81 🌐 — score 355.0

- **Primitives**: count-min-sketch, lin-kernighan, set-packing, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, streaming
- **Categories**: optimization, retrieval, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, scan, compare, sample, fold, order, scale, project

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → retrieval → sketching
**Components:**
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - set-packing (combinatorial-optimization): Find maximum collection of pairwise-disjoint subsets. Generalizes 3D matching and maximum independent set.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
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
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #82 🌐 — score 354.0

- **Primitives**: christofides-algorithm, gradient-descent, lin-kernighan, lsa
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval
- **Wiring**: embedding ranking pipeline
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, scale, compare, sample, project

```
**Wiring:** embedding ranking pipeline
**Cross-domain:** learning → optimization → retrieval
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
**Real walls (combined):**
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
```

### Recipe #83 🌐 — score 354.0

- **Primitives**: christofides-algorithm, gradient-descent, lin-kernighan, tf-idf
- **Domains**: combinatorial-optimization, information-retrieval, machine-learning
- **Categories**: learning, optimization, retrieval
- **Wiring**: embedding ranking pipeline
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, scale, compare, sample, project

```
**Wiring:** embedding ranking pipeline
**Cross-domain:** learning → optimization → retrieval
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - gradient-descent (machine-learning): First-order optimization; iterates x -= eta * grad f(x).
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - tf-idf (information-retrieval): Sparse vector space model; cosine similarity between query and document TF-IDF vectors.
**Real walls (combined):**
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
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #84 🌐 — score 354.0

- **Primitives**: christofides-algorithm, lin-kernighan, lsa, synthetic-control
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval
- **Categories**: optimization, reasoning, retrieval
- **Wiring**: ranking by projection
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, compare, sample, project, scale

```
**Wiring:** ranking by projection
**Cross-domain:** optimization → reasoning → retrieval
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - lsa (information-retrieval): Latent Semantic Analysis: truncated SVD on term-document matrix, cosine in k-dim latent space.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
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
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
```

### Recipe #85 🌐 — score 354.0

- **Primitives**: christofides-algorithm, lin-kernighan, synthetic-control, tf-idf
- **Domains**: causal-inference, combinatorial-optimization, information-retrieval
- **Categories**: optimization, reasoning, retrieval
- **Wiring**: ranking by projection
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, compare, sample, scale, project

```
**Wiring:** ranking by projection
**Cross-domain:** optimization → reasoning → retrieval
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - lin-kernighan (combinatorial-optimization): Variable k-opt local search for TSP. The most powerful general TSP heuristic.
  - synthetic-control (causal-inference): Construct a counterfactual from weighted combination of untreated units; difference-in-differences generalization.
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
  - [synthetic-control] Pre-treatment fit must be near-perfect or estimates are biased.
  - [synthetic-control] Inference via placebo tests is conservative.
  - [synthetic-control] Sensitive to donor pool selection.
  - [synthetic-control] Cannot handle multiple treated units without augmentation.
  - [tf-idf] Cannot handle synonyms or paraphrases.
  - [tf-idf] Sensitive to tokenizer and stopword choice.
  - [tf-idf] Sublinear TF scaling helps but not enough for long docs.
  - [tf-idf] Dominated by learned sparse (SPLADE) on modern benchmarks.
```

### Recipe #86 🌐 — score 353.0

- **Primitives**: aggregate-signature, christofides-algorithm, counterfactual-imagination, instrumental-variable
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: crypto, optimization, reasoning
- **Wiring**: fusion ranking
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, sample, compare, project

```
**Wiring:** fusion ranking
**Cross-domain:** crypto → optimization → reasoning
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
```

### Recipe #87 🌐 — score 353.0

- **Primitives**: aggregate-signature, christofides-algorithm, instrumental-variable, monte-carlo-tree-search
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: crypto, optimization, reasoning
- **Wiring**: fusion ranking
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** crypto → optimization → reasoning
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
```

### Recipe #88 🌐 — score 353.0

- **Primitives**: aggregate-signature, christofides-algorithm, instrumental-variable, tcp-timestamp
- **Domains**: causal-inference, combinatorial-optimization, cryptography-advanced, networking
- **Categories**: crypto, optimization, reasoning, systems
- **Wiring**: fusion ranking
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, project, compare, sample

```
**Wiring:** fusion ranking
**Cross-domain:** crypto → optimization → reasoning → systems
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
```

### Recipe #89 🌐 — score 353.0

- **Primitives**: aggregate-signature, christofides-algorithm, instrumental-variable, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, cryptography-advanced
- **Categories**: crypto, optimization, reasoning
- **Wiring**: fusion ranking
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: combine, scale, hash, order, scan, fold, project, sample, compare

```
**Wiring:** fusion ranking
**Cross-domain:** crypto → optimization → reasoning
**Components:**
  - aggregate-signature (cryptography-advanced): BLS or Schnorr-based signatures that compress n signatures into one.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #90 🌐 — score 353.0

- **Primitives**: bloom-filter, christofides-algorithm, counterfactual-imagination, instrumental-variable
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, hashing
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, sample, compare, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
```

### Recipe #91 🌐 — score 353.0

- **Primitives**: bloom-filter, christofides-algorithm, instrumental-variable, monte-carlo-tree-search
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, hashing
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, project, scale, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
```

### Recipe #92 🌐 — score 353.0

- **Primitives**: bloom-filter, christofides-algorithm, instrumental-variable, tcp-timestamp
- **Domains**: causal-inference, combinatorial-optimization, hashing, networking
- **Categories**: optimization, reasoning, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, project, scale, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching → systems
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
```

### Recipe #93 🌐 — score 353.0

- **Primitives**: bloom-filter, christofides-algorithm, instrumental-variable, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, hashing
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: hash, combine, order, scan, fold, project, scale, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - bloom-filter (hashing): Constant-space set membership; supports inserts and queries with no false negatives.
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #94 🌐 — score 353.0

- **Primitives**: christofides-algorithm, count-min-sketch, counterfactual-imagination, instrumental-variable
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, streaming
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, sample, compare, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
```

### Recipe #95 🌐 — score 353.0

- **Primitives**: christofides-algorithm, count-min-sketch, instrumental-variable, monte-carlo-tree-search
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, streaming
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, project, scale, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [monte-carlo-tree-search] UCB1 assumes stationary rewards; non-stationary requires decay.
  - [monte-carlo-tree-search] Simulation quality caps achievable strength.
  - [monte-carlo-tree-search] Memory cost: O(tree size) per game.
```

### Recipe #96 🌐 — score 353.0

- **Primitives**: christofides-algorithm, count-min-sketch, instrumental-variable, tcp-timestamp
- **Domains**: causal-inference, combinatorial-optimization, networking, streaming
- **Categories**: optimization, reasoning, sketching, systems
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, project, scale, compare, sample

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching → systems
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [tcp-timestamp] Clock skew between sender and receiver biases RTT.
  - [tcp-timestamp] Modern kernels disable by default (security/perf tradeoff).
  - [tcp-timestamp] Timestamps reveal clock granularity (fingerprinting).
```

### Recipe #97 🌐 — score 353.0

- **Primitives**: christofides-algorithm, count-min-sketch, instrumental-variable, tree-of-thought
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, streaming
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, hash, project, scale, sample, compare

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - count-min-sketch (streaming): Sublinear space for point queries and frequency estimation with epsilon error and 1-delta confidence.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [tree-of-thought] Quality of self-evaluation is the bottleneck.
  - [tree-of-thought] Tree width controls compute; deeper trees are not always better.
  - [tree-of-thought] Prompt engineering matters more than algorithmic choice.
```

### Recipe #98 🌐 — score 353.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, hyperloglog, instrumental-variable
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, streaming
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, hash, project, scale

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - hyperloglog (streaming): Cardinality estimation with O(1) memory; based on trailing zero counts across 2^B registers.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
```

### Recipe #99 🌐 — score 353.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, instrumental-variable, lsh
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, hashing
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, project, scale, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [lsh] Probability of collision depends on distance; calibration is tricky.
  - [lsh] Multi-probe LSH improves recall per probe count.
  - [lsh] Hash families are data-type specific (cosine, Jaccard, Euclidean).
```

### Recipe #100 🌐 — score 353.0

- **Primitives**: christofides-algorithm, counterfactual-imagination, instrumental-variable, minhash
- **Domains**: agentic-reasoning, causal-inference, combinatorial-optimization, hashing
- **Categories**: optimization, reasoning, sketching
- **Wiring**: nearest neighbor search
- **Architecture**: 1 composite + 3 atomic
- **Root atoms used**: order, combine, scan, fold, sample, compare, project, scale, hash

```
**Wiring:** nearest neighbor search
**Cross-domain:** optimization → reasoning → sketching
**Components:**
  - christofides-algorithm (combinatorial-optimization): 1.5-approximation algorithm for metric TSP. Combines MST, perfect matching, and Euler tour.
  - counterfactual-imagination (agentic-reasoning): What-if simulation: enumerate alternative worlds and rank by plausibility/utility.
  - instrumental-variable (causal-inference): Two-stage least squares using an instrument Z to identify treatment effect on Y through X.
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
  - [instrumental-variable] Exclusion restriction is untestable.
  - [instrumental-variable] Weak instruments bias 2SLS toward OLS.
  - [instrumental-variable] Heterogeneous treatment effects break LATE interpretation.
  - [minhash] Variance reduces as 1/sqrt(k); small k is noisy.
  - [minhash] One-permutation hashing is faster but loses precision.
  - [minhash] Cannot handle weighted Jaccard (use b-bit minhash for that).
```
