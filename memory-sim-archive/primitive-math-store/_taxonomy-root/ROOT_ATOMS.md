# Root Atom Taxonomy
> The 8 atoms that everything else wires from.
> These are the canonical atoms described in The Painted Fence.
> Every domain-specific primitive is a specialization, analog, or wiring of one or more of these.

---

## The 8 Root Atoms

### scan
**Definition:** Stream a thing into units — characters → tokens, tokens → shingles, documents → posting entries, graph → edge list, signal → samples.
**Mathematical form:** `units(x) = {u₁, u₂, ..., u_n}` where each u_i is the atomic unit for this domain.
**Canonical example:** text → tokens. graph → adjacency list. signal → samples.
**Cross-domain specializations:**
- Signal: sampling, A/D conversion
- Retrieval: tokenization, shingling, posting list generation
- Database: stream partitioning, windowing
- Physics: discretizing a continuous field into lattice sites
- Graphics: tessellating a surface into triangles

### hash
**Definition:** A unit → a stable integer. Deterministic: same input → same output. Randomized: uses randomness to spread items into pseudo-random buckets.
**Mathematical form:** `h(u) → z ∈ ℤ` where h is the hash function.
**Cross-domain specializations:**
- Cryptography: cryptographic hash (collision-resistant, pre-image resistant)
- Retrieval: MinHash, SimHash, LSH family
- Database: Bloom filter hash, consistent hashing
- Linear algebra: random projection (Johnson-Lindenstrauss), feature hashing
- Signal: spread spectrum (PN sequence)

### fold
**Definition:** Reduce a stream to an accumulator. Variants: `fold(sum)` → total, `fold(min)` → minimum, `fold(max)` → maximum, `fold(mean)` → average, `fold(collect)` → set.
**Mathematical form:** `fold(f, stream) = a_n` where `a_i = f(a_{i-1}, stream_i)` and `a_0 = init`.
**Cross-domain specializations:**
- Signal: integration, accumulation, energy detection
- Retrieval: term frequency (fold count), BM25 component
- Database: streaming aggregation (sum, count, min, max, mean)
- Linear algebra: dot product = fold(sum of products)
- Physics: integration over time, population count

### project
**Definition:** A vector through a matrix — dot product of query vector with basis vector. Embedding = project text into feature space. A matrix-vector multiply is a sequence of projections.
**Mathematical form:** `project(v, B) = Bᵀ·v` (projection onto columns of B) or `project(v, u) = (⟨v,u⟩/⟨u,u⟩)·u` (onto a direction u).
**Cross-domain specializations:**
- Signal: matched filter, mixing, modulation
- Retrieval: embedding lookup, vector search, BM25 scoring
- Linear algebra: all matrix-vector multiplication
- Graphics: transform a point through a matrix
- RF: correlate received signal with template

### scale
**Definition:** Divide by a norm, a max, or a reference. Normalize to unit norm, to [0,1], or to a fixed dynamic range.
**Mathematical form:** `scale(v, ref) = v / ref`. Common: `L2(v) = v / ||v||₂`, `max_scale(v) = v / max(v)`.
**Cross-domain specializations:**
- Signal: normalize signal power, divide by RMS, AGC
- Retrieval: L2-normalize embeddings, IDF scale
- Database: normalize sketch counts
- Graphics: normalize a direction vector, scale a color to [0,1]
- Linear algebra: unit-normalize vectors

### compare
**Definition:** A distance or similarity over a pair — cosine, Jaccard, Hamming, Euclidean, KL divergence. Given two vectors, produce a scalar.
**Mathematical form:** `compare(a, b) → s ∈ ℝ`. Examples: `cosine(a,b) = ⟨a,b⟩/(||a||·||b||)`, `euclidean(a,b) = ||a−b||₂`.
**Cross-domain specializations:**
- Signal: correlation, matched filter output, SNR
- Retrieval: cosine similarity, BM25 score, NDCG
- Cryptography: Hamming distance of fingerprints
- Database: similarity join distance threshold
- Graphics: template matching, feature comparison

### combine
**Definition:** Weighted sum of signals — fuse candidate lists, blend scores, mix modalities.
**Mathematical form:** `combine({(s_i, w_i)}) = Σ_i w_i·s_i`. The weights w_i are the policy decision of the orchestrator.
**Cross-domain specializations:**
- Signal: weighted sum of channels, mix in RF
- Retrieval: RRF, score fusion, hybrid retrieval
- Graphics: alpha compositing (w = alpha)
- Linear algebra: linear combination
- Physics: weighted force accumulation

### order
**Definition:** Sort by score. Top-k retrieval = partial sort (maintain a heap of size k). Full sort = sort all candidates.
**Mathematical form:** `order(candidates, score_fn) = sorted(candidates, key=score_fn, reverse=True)`.
**Cross-domain specializations:**
- Signal: rank by amplitude, sort frequency bins by power
- Retrieval: rank by relevance score, maintain top-k heap
- Database: ORDER BY in SQL, priority queue operations
- Graphics: painter's algorithm (sort by depth), sort-rendering
- Physics: rank by energy, temperature, etc.

---

## Atom Pairs — Common Wirings

These are the most common 2-atom combinations, expressed as primitives in their own right:

| Wirings | Result | Domain |
|---|---|---|
| scan → hash | tokenization + hashing | everywhere |
| scan → fold | stream aggregation | DB, signal |
| fold(sum) + fold(sum) → compare | cosine similarity | retrieval, signal |
| hash → fold(min) | MinHash signature | retrieval, crypto |
| hash → fold(count) | frequency counting | DB, retrieval |
| project → scale | normalized projection | retrieval, signal |
| project → compare | vector similarity | retrieval |
| combine → order | rank candidates | retrieval |
| scan → hash × k → fold(min) | MinHash | retrieval |
| scan → hash → fold(count) → fold(combine) | TF-IDF | retrieval |
| fold → project → scale | embedding pipeline | retrieval |
| hash × d → fold(min) → order | LSH retrieval | retrieval |

---

## The Reinforcement Ladder (per "The Painted Fence")

The same atom, reinforced with more structure:

### project — rung 1: random projection
- Generator: uniform random matrix
- Cost: free
- Fidelity: term overlap only

### project — rung 2: IDF-whitened projection
- Generator: corpus document-frequency statistics (IDF weights)
- Cost: one corpus scan
- Fidelity: corpus-specific semantic weighting

### project — rung 3: latent-factor projection (LSA)
- Generator: truncated SVD of the term-document matrix
- Cost: O(N·k) via randomized SVD
- Fidelity: paraphrase-level semantic matching

### fold — rung 1: raw frequency (tf)
- Cost: free (count during scan)
- Fidelity: term presence

### fold — rung 2: sublinear tf (1 + log(tf))
- Cost: one log per term
- Fidelity: diminishing returns for repeated terms

### fold — rung 3: BM25 saturation
- Cost: one division
- Fidelity: principled term saturation

---

## Real Walls by Atom

| Atom | Real Wall | Conserved Currency |
|---|---|---|
| scan | Sampling rate (Nyquist) | bandwidth × time |
| hash | Collision rate | information (entropy) |
| fold | Information loss (rate-distortion) | bits of the accumulator |
| project | Dimension of the space | degrees of freedom |
| scale | Dynamic range | bits per sample |
| compare | Information-theoretic lower bound | Fisher information |
| combine | Normalization cost | probability mass |
| order | Sorting lower bound | O(n log n) comparisons |

---

## Cross-Domain Primitive Map (Summary)

```
scan  ───→ tokenize, sample, discretize, tessellate, index
hash  ───→ spread, random-project, LSH, MinHash, code
fold  ───→ integrate, accumulate, count, sum, min, mean
project ─→ dot, mix, correlate, match, transform, compress
scale  ───→ normalize, whiten, IDF, unit-vec, divide-by-norm
compare ──→ correlate, cosine, distance, jaccard, hamming
combine ─→ weighted-sum, fuse, blend, mix, alpha-over
order  ───→ rank, sort, heap, top-k, priority-queue
```

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*
