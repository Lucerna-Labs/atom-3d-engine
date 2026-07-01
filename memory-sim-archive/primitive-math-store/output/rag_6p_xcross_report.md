# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T13:39:11.075288+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\rag_primitives_catalog.json` (41 primitives)
- Seed pool: 24 primitives from 6 domains
- N-tuple size: 6
- Top recipes kept: 50
- Runtime: 2.28s

## Enumeration

- 6-tuple: 134,596

## Scoring summary

- Recipes scored: 134,596
- Max score: 915.0
- Mean score: 883.81
- Cross-domain: 134596 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 134596 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| gating | 95836 |
| reranking | 95836 |
| retrieval | 95836 |
| decomposition | 95836 |
| scoring | 95836 |
| filtering | 95836 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 134568 |
| compare | 134568 |
| scale | 134134 |
| order | 126588 |
| fold | 107464 |
| project | 80332 |
| scan | 33649 |

## Top 50 recipes

### Recipe #1 🌐 — score 915.0

- **Primitives**: atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #2 🌐 — score 915.0

- **Primitives**: atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #3 🌐 — score 915.0

- **Primitives**: atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, keyword-extraction, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #4 🌐 — score 915.0

- **Primitives**: atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, llm-judge-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #5 🌐 — score 915.0

- **Primitives**: atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #6 🌐 — score 915.0

- **Primitives**: atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, fold, project, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #7 🌐 — score 915.0

- **Primitives**: atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, primitive-composite-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #8 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
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

### Recipe #9 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, ncd-retrieval, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
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

### Recipe #10 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, keyword-extraction, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #11 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, llm-judge-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #12 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, ncd-retrieval, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #13 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #14 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, primitive-composite-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #15 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, keyword-extraction, ncd-retrieval, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
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

### Recipe #16 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, llm-judge-score, ncd-retrieval, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
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

### Recipe #17 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #18 🌐 — score 915.0

- **Primitives**: atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval, primitive-composite-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: order, combine, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #19 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #20 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, keyword-extraction, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #21 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, llm-judge-score, primitive-rerank
- **Domains**: gating, reranking, retrieval, scoring
- **Categories**: gating, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #22 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, ncd-retrieval, primitive-rerank
- **Domains**: gating, reranking, retrieval
- **Categories**: gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #23 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval
- **Categories**: gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #24 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, primitive-composite-score, primitive-rerank
- **Domains**: gating, reranking, retrieval, scoring
- **Categories**: gating, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #25 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, keyword-extraction, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #26 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, llm-judge-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #27 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, ncd-retrieval, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #28 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #29 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, primitive-composite-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #30 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, keyword-extraction, llm-judge-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #31 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, keyword-extraction, ncd-retrieval, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #32 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, keyword-extraction, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #33 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, keyword-extraction, primitive-composite-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #34 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, llm-judge-score, ncd-retrieval, primitive-rerank
- **Domains**: reranking, retrieval, scoring
- **Categories**: reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #35 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, llm-judge-score, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval, scoring
- **Categories**: gating, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, scan, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #36 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, llm-judge-score, primitive-composite-score, primitive-rerank
- **Domains**: reranking, retrieval, scoring
- **Categories**: reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #37 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval
- **Categories**: gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, scan, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #38 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, ncd-retrieval, primitive-composite-score, primitive-rerank
- **Domains**: reranking, retrieval, scoring
- **Categories**: reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, order

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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

### Recipe #39 🌐 — score 915.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, parallel-fan-out, primitive-composite-score, primitive-rerank
- **Domains**: gating, reranking, retrieval, scoring
- **Categories**: gating, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: combine, scale, fold, compare, project, scan, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #40 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, keyword-extraction, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #41 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, llm-judge-score, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, scoring
- **Categories**: decomposition, gating, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → scoring
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #42 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, ncd-retrieval, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
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

### Recipe #43 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, parallel-fan-out, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #44 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, primitive-composite-score, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, scoring
- **Categories**: decomposition, gating, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → scoring
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [hierarchical-decomposition] Recursion depth must be bounded (3-4 typical).
  - [hierarchical-decomposition] Leaves must be atomic; intermediate nodes need partial answers.
  - [hierarchical-decomposition] Cost multiplies per level: 1 → N → N^2 in worst case.
  - [hierarchical-decomposition] Aggregation at parent nodes can lose information.
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

### Recipe #45 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, keyword-extraction, ncd-retrieval, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval
- **Categories**: decomposition, gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
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

### Recipe #46 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, llm-judge-score, ncd-retrieval, primitive-rerank
- **Domains**: gating, reranking, retrieval, scoring
- **Categories**: gating, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
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

### Recipe #47 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, ncd-retrieval, parallel-fan-out, primitive-rerank
- **Domains**: gating, reranking, retrieval
- **Categories**: gating, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold, scan

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #48 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, ncd-retrieval, primitive-composite-score, primitive-rerank
- **Domains**: gating, reranking, retrieval, scoring
- **Categories**: gating, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
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
  - [fallback-chain] Ordering matters (cheap first, expensive last).
  - [fallback-chain] Each escalation step has its own latency cost.
  - [fallback-chain] Worst case: all stages fail, latency is sum.
  - [fallback-chain] Cascade thresholds per stage need tuning.
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
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

### Recipe #49 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, keyword-extraction, llm-judge-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring
- **Categories**: decomposition, reranking, retrieval, scoring
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
  - [llm-judge-score] Slow: 100-500ms per call.
  - [llm-judge-score] Expensive at scale.
  - [llm-judge-score] Tied to LLM's distribution.
  - [llm-judge-score] May disagree with downstream LLM.
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #50 🌐 — score 915.0

- **Primitives**: cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, keyword-extraction, ncd-retrieval, primitive-rerank
- **Domains**: decomposition, reranking, retrieval
- **Categories**: decomposition, reranking, retrieval
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 6 atomic
- **Root atoms used**: compare, scale, combine, project, order, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval
**Components:**
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - keyword-extraction (decomposition): Extract keywords/entities from query. Lightweight; no LLM call. Used for keyword-based retrieval.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
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
  - [keyword-extraction] Misses synonyms, paraphrases, semantic context.
  - [keyword-extraction] Stopword filtering is language-dependent.
  - [keyword-extraction] Multi-word entities need special handling (NER).
  - [keyword-extraction] Won't help on pure semantic queries.
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
