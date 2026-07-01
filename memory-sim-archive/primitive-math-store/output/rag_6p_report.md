# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T13:38:43.540664+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\rag_primitives_catalog.json` (41 primitives)
- Seed pool: 30 primitives from 7 domains
- N-tuple size: 6
- Top recipes kept: 50
- Runtime: 12.95s

## Enumeration

- 6-tuple: 593,775

## Scoring summary

- Recipes scored: 593,775
- Max score: 469.0
- Mean score: 451.06
- Cross-domain: 593773 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 593775 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| retrieval | 459179 |
| decomposition | 459179 |
| reranking | 363545 |
| gating | 363545 |
| scoring | 363545 |
| filtering | 363545 |
| synthesis | 217035 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 593747 |
| compare | 593313 |
| scale | 588770 |
| order | 555015 |
| fold | 416675 |
| project | 416675 |
| scan | 118755 |

## Top 50 recipes

### Recipe #1 🌐 — score 469.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #2 🌐 — score 469.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #3 🌐 — score 469.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #4 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [atomic-decomposition] Quality of decomposition is bounded by the LLM used.
  - [atomic-decomposition] Atomic is subjective: too granular = noise; too coarse = missing context.
  - [atomic-decomposition] Sub-Qs may overlap or be redundant, inflating retrieval cost.
  - [atomic-decomposition] Decomposition prompt must be tuned per domain (code vs policy vs medical).
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #5 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, bm25-retrieval, dense-embedding-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [atomic-decomposition] Quality of decomposition is bounded by the LLM used.
  - [atomic-decomposition] Atomic is subjective: too granular = noise; too coarse = missing context.
  - [atomic-decomposition] Sub-Qs may overlap or be redundant, inflating retrieval cost.
  - [atomic-decomposition] Decomposition prompt must be tuned per domain (code vs policy vs medical).
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

### Recipe #6 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [atomic-decomposition] Quality of decomposition is bounded by the LLM used.
  - [atomic-decomposition] Atomic is subjective: too granular = noise; too coarse = missing context.
  - [atomic-decomposition] Sub-Qs may overlap or be redundant, inflating retrieval cost.
  - [atomic-decomposition] Decomposition prompt must be tuned per domain (code vs policy vs medical).
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

### Recipe #7 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [atomic-decomposition] Quality of decomposition is bounded by the LLM used.
  - [atomic-decomposition] Atomic is subjective: too granular = noise; too coarse = missing context.
  - [atomic-decomposition] Sub-Qs may overlap or be redundant, inflating retrieval cost.
  - [atomic-decomposition] Decomposition prompt must be tuned per domain (code vs policy vs medical).
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #8 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [atomic-decomposition] Quality of decomposition is bounded by the LLM used.
  - [atomic-decomposition] Atomic is subjective: too granular = noise; too coarse = missing context.
  - [atomic-decomposition] Sub-Qs may overlap or be redundant, inflating retrieval cost.
  - [atomic-decomposition] Decomposition prompt must be tuned per domain (code vs policy vs medical).
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

### Recipe #9 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [atomic-decomposition] Quality of decomposition is bounded by the LLM used.
  - [atomic-decomposition] Atomic is subjective: too granular = noise; too coarse = missing context.
  - [atomic-decomposition] Sub-Qs may overlap or be redundant, inflating retrieval cost.
  - [atomic-decomposition] Decomposition prompt must be tuned per domain (code vs policy vs medical).
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

### Recipe #10 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, char-ngram-retrieval, cross-encoder-rerank, dense-embedding-retrieval, parallel-fan-out
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - char-ngram-retrieval (retrieval): Character n-gram (3-5) cosine similarity. Robust to typos, partial matches.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
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
  - [char-ngram-retrieval] Vector dimensionality grows with n-gram size × vocab.
  - [char-ngram-retrieval] High overlap between similar-language docs (false positives).
  - [char-ngram-retrieval] Doesn't capture word semantics.
  - [char-ngram-retrieval] Strong baseline for paraphrase-resistance.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #11 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, char-ngram-retrieval, dense-embedding-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - char-ngram-retrieval (retrieval): Character n-gram (3-5) cosine similarity. Robust to typos, partial matches.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [char-ngram-retrieval] Vector dimensionality grows with n-gram size × vocab.
  - [char-ngram-retrieval] High overlap between similar-language docs (false positives).
  - [char-ngram-retrieval] Doesn't capture word semantics.
  - [char-ngram-retrieval] Strong baseline for paraphrase-resistance.
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

### Recipe #12 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, parallel-fan-out
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
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
  - [extractive-synthesis] No abstraction or reasoning over multiple chunks.
  - [extractive-synthesis] Output length bounded by retrieved content.
  - [extractive-synthesis] Cannot resolve conflicting information.
  - [extractive-synthesis] Quality depends heavily on rerank precision.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #13 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, parallel-fan-out
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #14 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
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

### Recipe #15 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hyde-query-rewrite, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
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
  - [hyde-query-rewrite] Quality depends on LLM.
  - [hyde-query-rewrite] LLM may hallucinate.
  - [hyde-query-rewrite] Latency: extra LLM call before retrieval.
  - [hyde-query-rewrite] Doesn't help when query is already doc-like.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #16 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, keyword-extraction, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #17 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, llm-judge-score, parallel-fan-out
- **Domains**: gating, reranking, retrieval, scoring, synthesis
- **Categories**: gating, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
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
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #18 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, lsa-retrieval, parallel-fan-out
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
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
  - [lsa-retrieval] Topic-level, not fine-grained semantics.
  - [lsa-retrieval] k must be tuned per corpus.
  - [lsa-retrieval] Sensitive to preprocessing.
  - [lsa-retrieval] Folds synonyms close but can't bridge across topics.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #19 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #20 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, parallel-fan-out, primitive-composite-score
- **Domains**: gating, reranking, retrieval, scoring, synthesis
- **Categories**: gating, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-composite-score] Weight tuning per corpus.
  - [primitive-composite-score] Score scales differ; normalization needed.
  - [primitive-composite-score] Saturates when primitives agree.
  - [primitive-composite-score] Doesn't help when stage-1 retriever missed gold.
```

### Recipe #21 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, hyde-query-rewrite, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - hyde-query-rewrite (decomposition): Hypothetical Document Embeddings: LLM generates a hypothetical answer, used as the retrieval query. Bridges question-to-doc gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [cross-encoder-rerank] Slowest rerank option (O(K) model calls).
  - [cross-encoder-rerank] Requires GPU/endpoint.
  - [cross-encoder-rerank] Bounded by K (top-K from stage 1).
  - [cross-encoder-rerank] Model size matters: 0.6B is much faster than 4B, slight quality loss.
  - [cross-encoder-rerank] Tied to underlying LLM's pre-training distribution.
  - [hyde-query-rewrite] Quality depends on LLM.
  - [hyde-query-rewrite] LLM may hallucinate.
  - [hyde-query-rewrite] Latency: extra LLM call before retrieval.
  - [hyde-query-rewrite] Doesn't help when query is already doc-like.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #22 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, lsa-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - lsa-retrieval (retrieval): Latent Semantic Analysis: truncated SVD on term-doc matrix. Topic-level match.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [cross-encoder-rerank] Slowest rerank option (O(K) model calls).
  - [cross-encoder-rerank] Requires GPU/endpoint.
  - [cross-encoder-rerank] Bounded by K (top-K from stage 1).
  - [cross-encoder-rerank] Model size matters: 0.6B is much faster than 4B, slight quality loss.
  - [cross-encoder-rerank] Tied to underlying LLM's pre-training distribution.
  - [lsa-retrieval] Topic-level, not fine-grained semantics.
  - [lsa-retrieval] k must be tuned per corpus.
  - [lsa-retrieval] Sensitive to preprocessing.
  - [lsa-retrieval] Folds synonyms close but can't bridge across topics.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #23 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, extractive-synthesis, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [extractive-synthesis] No abstraction or reasoning over multiple chunks.
  - [extractive-synthesis] Output length bounded by retrieved content.
  - [extractive-synthesis] Cannot resolve conflicting information.
  - [extractive-synthesis] Quality depends heavily on rerank precision.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #24 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, fallback-chain, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #25 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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

### Recipe #26 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, hyde-query-rewrite, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hyde-query-rewrite (decomposition): Hypothetical Document Embeddings: LLM generates a hypothetical answer, used as the retrieval query. Bridges question-to-doc gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [hyde-query-rewrite] Quality depends on LLM.
  - [hyde-query-rewrite] LLM may hallucinate.
  - [hyde-query-rewrite] Latency: extra LLM call before retrieval.
  - [hyde-query-rewrite] Doesn't help when query is already doc-like.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #27 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, keyword-extraction, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #28 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, llm-judge-score, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, scoring, synthesis
- **Categories**: gating, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #29 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, lsa-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - lsa-retrieval (retrieval): Latent Semantic Analysis: truncated SVD on term-doc matrix. Topic-level match.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [lsa-retrieval] Topic-level, not fine-grained semantics.
  - [lsa-retrieval] k must be tuned per corpus.
  - [lsa-retrieval] Sensitive to preprocessing.
  - [lsa-retrieval] Folds synonyms close but can't bridge across topics.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #30 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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

### Recipe #31 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, parallel-fan-out, primitive-composite-score, primitive-rerank
- **Domains**: gating, reranking, retrieval, scoring, synthesis
- **Categories**: gating, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-composite-score] Weight tuning per corpus.
  - [primitive-composite-score] Score scales differ; normalization needed.
  - [primitive-composite-score] Saturates when primitives agree.
  - [primitive-composite-score] Doesn't help when stage-1 retriever missed gold.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #32 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, char-ngram-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - char-ngram-retrieval (retrieval): Character n-gram (3-5) cosine similarity. Robust to typos, partial matches.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [char-ngram-retrieval] Vector dimensionality grows with n-gram size × vocab.
  - [char-ngram-retrieval] High overlap between similar-language docs (false positives).
  - [char-ngram-retrieval] Doesn't capture word semantics.
  - [char-ngram-retrieval] Strong baseline for paraphrase-resistance.
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

### Recipe #33 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, char-ngram-retrieval, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - char-ngram-retrieval (retrieval): Character n-gram (3-5) cosine similarity. Robust to typos, partial matches.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [char-ngram-retrieval] Vector dimensionality grows with n-gram size × vocab.
  - [char-ngram-retrieval] High overlap between similar-language docs (false positives).
  - [char-ngram-retrieval] Doesn't capture word semantics.
  - [char-ngram-retrieval] Strong baseline for paraphrase-resistance.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #34 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, char-ngram-retrieval, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - char-ngram-retrieval (retrieval): Character n-gram (3-5) cosine similarity. Robust to typos, partial matches.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [char-ngram-retrieval] Vector dimensionality grows with n-gram size × vocab.
  - [char-ngram-retrieval] High overlap between similar-language docs (false positives).
  - [char-ngram-retrieval] Doesn't capture word semantics.
  - [char-ngram-retrieval] Strong baseline for paraphrase-resistance.
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

### Recipe #35 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, char-ngram-retrieval, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - char-ngram-retrieval (retrieval): Character n-gram (3-5) cosine similarity. Robust to typos, partial matches.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [char-ngram-retrieval] Vector dimensionality grows with n-gram size × vocab.
  - [char-ngram-retrieval] High overlap between similar-language docs (false positives).
  - [char-ngram-retrieval] Doesn't capture word semantics.
  - [char-ngram-retrieval] Strong baseline for paraphrase-resistance.
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

### Recipe #36 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, hierarchical-decomposition, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [extractive-synthesis] No abstraction or reasoning over multiple chunks.
  - [extractive-synthesis] Output length bounded by retrieved content.
  - [extractive-synthesis] Cannot resolve conflicting information.
  - [extractive-synthesis] Quality depends heavily on rerank precision.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #37 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, ncd-retrieval, parallel-fan-out
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [extractive-synthesis] No abstraction or reasoning over multiple chunks.
  - [extractive-synthesis] Output length bounded by retrieved content.
  - [extractive-synthesis] Cannot resolve conflicting information.
  - [extractive-synthesis] Quality depends heavily on rerank precision.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #38 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #39 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, ncd-retrieval, parallel-fan-out
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #40 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, hyde-query-rewrite, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - hyde-query-rewrite (decomposition): Hypothetical Document Embeddings: LLM generates a hypothetical answer, used as the retrieval query. Bridges question-to-doc gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [hyde-query-rewrite] Quality depends on LLM.
  - [hyde-query-rewrite] LLM may hallucinate.
  - [hyde-query-rewrite] Latency: extra LLM call before retrieval.
  - [hyde-query-rewrite] Doesn't help when query is already doc-like.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #41 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, keyword-extraction, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #42 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, llm-judge-score, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, scoring, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #43 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, lsa-retrieval, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - lsa-retrieval (retrieval): Latent Semantic Analysis: truncated SVD on term-doc matrix. Topic-level match.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [lsa-retrieval] Topic-level, not fine-grained semantics.
  - [lsa-retrieval] k must be tuned per corpus.
  - [lsa-retrieval] Sensitive to preprocessing.
  - [lsa-retrieval] Folds synonyms close but can't bridge across topics.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #44 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, ncd-retrieval, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #45 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, primitive-composite-score
- **Domains**: decomposition, gating, reranking, retrieval, scoring, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-composite-score] Weight tuning per corpus.
  - [primitive-composite-score] Score scales differ; normalization needed.
  - [primitive-composite-score] Saturates when primitives agree.
  - [primitive-composite-score] Doesn't help when stage-1 retriever missed gold.
```

### Recipe #46 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, hyde-query-rewrite, ncd-retrieval, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hyde-query-rewrite (decomposition): Hypothetical Document Embeddings: LLM generates a hypothetical answer, used as the retrieval query. Bridges question-to-doc gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [hyde-query-rewrite] Quality depends on LLM.
  - [hyde-query-rewrite] LLM may hallucinate.
  - [hyde-query-rewrite] Latency: extra LLM call before retrieval.
  - [hyde-query-rewrite] Doesn't help when query is already doc-like.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #47 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, keyword-extraction, ncd-retrieval, parallel-fan-out
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #48 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, llm-judge-score, ncd-retrieval, parallel-fan-out
- **Domains**: gating, reranking, retrieval, scoring, synthesis
- **Categories**: gating, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #49 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, lsa-retrieval, ncd-retrieval, parallel-fan-out
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - lsa-retrieval (retrieval): Latent Semantic Analysis: truncated SVD on term-doc matrix. Topic-level match.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [lsa-retrieval] Topic-level, not fine-grained semantics.
  - [lsa-retrieval] k must be tuned per corpus.
  - [lsa-retrieval] Sensitive to preprocessing.
  - [lsa-retrieval] Folds synonyms close but can't bridge across topics.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
```

### Recipe #50 🌐 — score 467.0

- **Primitives**: abstractive-synthesis, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, primitive-composite-score
- **Domains**: gating, reranking, retrieval, scoring, synthesis
- **Categories**: gating, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [primitive-composite-score] Weight tuning per corpus.
  - [primitive-composite-score] Score scales differ; normalization needed.
  - [primitive-composite-score] Saturates when primitives agree.
  - [primitive-composite-score] Doesn't help when stage-1 retriever missed gold.
```
