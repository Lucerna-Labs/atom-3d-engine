# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T13:37:49.260860+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\rag_primitives_catalog.json` (41 primitives)
- Seed pool: 35 primitives from 8 domains
- N-tuple size: 4
- Top recipes kept: 30
- Runtime: 0.70s

## Enumeration

- 4-tuple: 52,360

## Scoring summary

- Recipes scored: 52,360
- Max score: 350.0
- Mean score: 330.95
- Cross-domain: 52270 (99.8%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 52360 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| retrieval | 34810 |
| decomposition | 28609 |
| synthesis | 20895 |
| gating | 20895 |
| reranking | 20895 |
| scoring | 20895 |
| filtering | 20895 |
| workflow | 5984 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 52290 |
| compare | 51645 |
| scale | 48484 |
| order | 45045 |
| fold | 28609 |
| project | 24955 |
| hash | 5984 |
| scan | 5984 |

## Top 30 recipes

### Recipe #1 🌐 — score 350.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out
- **Domains**: decomposition, gating, retrieval, synthesis
- **Categories**: decomposition, gating, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: compare, combine, hash, project, scale, order, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #2 🌐 — score 349.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, parallel-fan-out
- **Domains**: gating, retrieval, synthesis
- **Categories**: gating, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [bm25-retrieval] BM25F (fielded) needed for structured docs.
  - [bm25-retrieval] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25-retrieval] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25-retrieval] Fails on long-tail terms without IDF smoothing.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #3 🌐 — score 349.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, hierarchical-decomposition
- **Domains**: decomposition, retrieval, synthesis
- **Categories**: decomposition, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
```

### Recipe #4 🌐 — score 349.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out
- **Domains**: decomposition, gating, retrieval, synthesis
- **Categories**: decomposition, gating, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #5 🌐 — score 349.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out
- **Domains**: gating, retrieval, synthesis
- **Categories**: gating, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #6 🌐 — score 349.0

- **Primitives**: bm25-retrieval, dedup-merge, dense-embedding-retrieval, primitive-rerank
- **Domains**: reranking, retrieval, synthesis
- **Categories**: reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, scale, fold, compare, hash, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [bm25-retrieval] BM25F (fielded) needed for structured docs.
  - [bm25-retrieval] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25-retrieval] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25-retrieval] Fails on long-tail terms without IDF smoothing.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #7 🌐 — score 349.0

- **Primitives**: bm25-retrieval, dense-embedding-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval
- **Categories**: gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, scale, fold, project, compare, scan, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [bm25-retrieval] BM25F (fielded) needed for structured docs.
  - [bm25-retrieval] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25-retrieval] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25-retrieval] Fails on long-tail terms without IDF smoothing.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #8 🌐 — score 349.0

- **Primitives**: cross-encoder-rerank, dedup-merge, dense-embedding-retrieval, hierarchical-decomposition
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: compare, scale, combine, hash, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
**Real walls (combined):**
  - [cross-encoder-rerank] Slowest rerank option (O(K) model calls).
  - [cross-encoder-rerank] Requires GPU/endpoint.
  - [cross-encoder-rerank] Bounded by K (top-K from stage 1).
  - [cross-encoder-rerank] Model size matters: 0.6B is much faster than 4B, slight quality loss.
  - [cross-encoder-rerank] Tied to underlying LLM's pre-training distribution.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
```

### Recipe #9 🌐 — score 349.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [cross-encoder-rerank] Slowest rerank option (O(K) model calls).
  - [cross-encoder-rerank] Requires GPU/endpoint.
  - [cross-encoder-rerank] Bounded by K (top-K from stage 1).
  - [cross-encoder-rerank] Model size matters: 0.6B is much faster than 4B, slight quality loss.
  - [cross-encoder-rerank] Tied to underlying LLM's pre-training distribution.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #10 🌐 — score 349.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: compare, combine, hash, project, scale, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #11 🌐 — score 349.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, ncd-retrieval, primitive-rerank
- **Domains**: reranking, retrieval, synthesis
- **Categories**: reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: compare, combine, hash, project, scale, fold, order

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #12 🌐 — score 349.0

- **Primitives**: dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, compare, scale, order, fold, combine, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #13 🌐 — score 349.0

- **Primitives**: dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval
- **Categories**: gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: project, compare, scale, combine, fold, scan, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval
**Components:**
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #14 🌐 — score 348.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval
- **Domains**: reranking, retrieval, synthesis
- **Categories**: reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [bm25-retrieval] BM25F (fielded) needed for structured docs.
  - [bm25-retrieval] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25-retrieval] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25-retrieval] Fails on long-tail terms without IDF smoothing.
  - [cross-encoder-rerank] Slowest rerank option (O(K) model calls).
  - [cross-encoder-rerank] Requires GPU/endpoint.
  - [cross-encoder-rerank] Bounded by K (top-K from stage 1).
  - [cross-encoder-rerank] Model size matters: 0.6B is much faster than 4B, slight quality loss.
  - [cross-encoder-rerank] Tied to underlying LLM's pre-training distribution.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
```

### Recipe #15 🌐 — score 348.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, primitive-rerank
- **Domains**: reranking, retrieval, synthesis
- **Categories**: reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [bm25-retrieval] BM25F (fielded) needed for structured docs.
  - [bm25-retrieval] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25-retrieval] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25-retrieval] Fails on long-tail terms without IDF smoothing.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #16 🌐 — score 348.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [cross-encoder-rerank] Slowest rerank option (O(K) model calls).
  - [cross-encoder-rerank] Requires GPU/endpoint.
  - [cross-encoder-rerank] Bounded by K (top-K from stage 1).
  - [cross-encoder-rerank] Model size matters: 0.6B is much faster than 4B, slight quality loss.
  - [cross-encoder-rerank] Tied to underlying LLM's pre-training distribution.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
```

### Recipe #17 🌐 — score 348.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval
- **Domains**: reranking, retrieval, synthesis
- **Categories**: reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [cross-encoder-rerank] Slowest rerank option (O(K) model calls).
  - [cross-encoder-rerank] Requires GPU/endpoint.
  - [cross-encoder-rerank] Bounded by K (top-K from stage 1).
  - [cross-encoder-rerank] Model size matters: 0.6B is much faster than 4B, slight quality loss.
  - [cross-encoder-rerank] Tied to underlying LLM's pre-training distribution.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
```

### Recipe #18 🌐 — score 348.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #19 🌐 — score 348.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, ncd-retrieval, primitive-rerank
- **Domains**: reranking, retrieval, synthesis
- **Categories**: reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #20 🌐 — score 348.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [cross-encoder-rerank] Slowest rerank option (O(K) model calls).
  - [cross-encoder-rerank] Requires GPU/endpoint.
  - [cross-encoder-rerank] Bounded by K (top-K from stage 1).
  - [cross-encoder-rerank] Model size matters: 0.6B is much faster than 4B, slight quality loss.
  - [cross-encoder-rerank] Tied to underlying LLM's pre-training distribution.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #21 🌐 — score 348.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, map-reduce-workflow, parallel-fan-out
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: compare, combine, hash, project, scale, fold, order, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - map-reduce-workflow (workflow): Decompose → parallel retrieve per sub-Q → reduce/synthesize. Standard sub-Q RAG pattern.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [map-reduce-workflow] Decomposition quality bounds everything downstream.
  - [map-reduce-workflow] Parallel retrieval may return overlapping or conflicting chunks.
  - [map-reduce-workflow] Reduce step can lose sub-Q-specific nuance.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #22 🌐 — score 348.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, parallel-fan-out, reciprocal-rank-fusion
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: compare, combine, hash, project, scale, scan, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - reciprocal-rank-fusion (reranking): RRF: combine multiple retrieval rankings via 1/(60 + rank). Parameter-free, robust.
**Real walls (combined):**
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [reciprocal-rank-fusion] Per-rank weight equal; no way to favor primitive A over B.
  - [reciprocal-rank-fusion] Top-1 dominator can hide if ranking is poor.
  - [reciprocal-rank-fusion] Doesn't surface confidence scores.
```

### Recipe #23 🌐 — score 348.0

- **Primitives**: dedup-merge, hierarchical-decomposition, lsa-retrieval, parallel-fan-out
- **Domains**: decomposition, gating, retrieval, synthesis
- **Categories**: decomposition, gating, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: compare, combine, hash, order, fold, project, scale, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - lsa-retrieval (retrieval): Latent Semantic Analysis: truncated SVD on term-doc matrix. Topic-level match.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [lsa-retrieval] Topic-level, not fine-grained semantics.
  - [lsa-retrieval] k must be tuned per corpus.
  - [lsa-retrieval] Sensitive to preprocessing.
  - [lsa-retrieval] Folds synonyms close but can't bridge across topics.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #24 🌐 — score 347.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dedup-merge, hyde-query-rewrite
- **Domains**: decomposition, retrieval, synthesis
- **Categories**: decomposition, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, scale, fold, hash, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - hyde-query-rewrite (decomposition): Hypothetical Document Embeddings: LLM generates a hypothetical answer, used as the retrieval query. Bridges question-to-doc gap.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [bm25-retrieval] BM25F (fielded) needed for structured docs.
  - [bm25-retrieval] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25-retrieval] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25-retrieval] Fails on long-tail terms without IDF smoothing.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [hyde-query-rewrite] Quality depends on LLM.
  - [hyde-query-rewrite] LLM may hallucinate.
  - [hyde-query-rewrite] Latency: extra LLM call before retrieval.
  - [hyde-query-rewrite] Doesn't help when query is already doc-like.
```

### Recipe #25 🌐 — score 347.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, hyde-query-rewrite, parallel-fan-out
- **Domains**: decomposition, gating, retrieval, synthesis
- **Categories**: decomposition, gating, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - hyde-query-rewrite (decomposition): Hypothetical Document Embeddings: LLM generates a hypothetical answer, used as the retrieval query. Bridges question-to-doc gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [bm25-retrieval] BM25F (fielded) needed for structured docs.
  - [bm25-retrieval] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25-retrieval] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25-retrieval] Fails on long-tail terms without IDF smoothing.
  - [hyde-query-rewrite] Quality depends on LLM.
  - [hyde-query-rewrite] LLM may hallucinate.
  - [hyde-query-rewrite] Latency: extra LLM call before retrieval.
  - [hyde-query-rewrite] Doesn't help when query is already doc-like.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #26 🌐 — score 347.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, lsa-retrieval, parallel-fan-out
- **Domains**: gating, retrieval, synthesis
- **Categories**: gating, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - lsa-retrieval (retrieval): Latent Semantic Analysis: truncated SVD on term-doc matrix. Topic-level match.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [bm25-retrieval] BM25F (fielded) needed for structured docs.
  - [bm25-retrieval] k1=1.5, b=0.75 are defaults but rarely optimal.
  - [bm25-retrieval] Cannot bridge vocabulary gap (no synonym handling).
  - [bm25-retrieval] Fails on long-tail terms without IDF smoothing.
  - [lsa-retrieval] Topic-level, not fine-grained semantics.
  - [lsa-retrieval] k must be tuned per corpus.
  - [lsa-retrieval] Sensitive to preprocessing.
  - [lsa-retrieval] Folds synonyms close but can't bridge across topics.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #27 🌐 — score 347.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, map-reduce-workflow
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - map-reduce-workflow (workflow): Decompose → parallel retrieve per sub-Q → reduce/synthesize. Standard sub-Q RAG pattern.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [map-reduce-workflow] Decomposition quality bounds everything downstream.
  - [map-reduce-workflow] Parallel retrieval may return overlapping or conflicting chunks.
  - [map-reduce-workflow] Reduce step can lose sub-Q-specific nuance.
```

### Recipe #28 🌐 — score 347.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, parallel-fan-out
- **Domains**: gating, retrieval, synthesis
- **Categories**: gating, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #29 🌐 — score 347.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, reciprocal-rank-fusion
- **Domains**: reranking, retrieval, synthesis
- **Categories**: reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - reciprocal-rank-fusion (reranking): RRF: combine multiple retrieval rankings via 1/(60 + rank). Parameter-free, robust.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [reciprocal-rank-fusion] Per-rank weight equal; no way to favor primitive A over B.
  - [reciprocal-rank-fusion] Top-1 dominator can hide if ranking is poor.
  - [reciprocal-rank-fusion] Doesn't surface confidence scores.
```

### Recipe #30 🌐 — score 347.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, weighted-fusion-rerank
- **Domains**: reranking, retrieval, synthesis
- **Categories**: reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 4 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - weighted-fusion-rerank (reranking): Weighted combination of normalized scores from multiple primitives. Tunable weights per corpus.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [weighted-fusion-rerank] Weights need tuning per corpus; not transferable.
  - [weighted-fusion-rerank] Score scales differ (cosine vs BM25 vs NCD); normalization needed.
  - [weighted-fusion-rerank] Can overfit to one corpus.
```
