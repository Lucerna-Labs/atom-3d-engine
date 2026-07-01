# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T13:39:13.659175+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\rag_primitives_catalog.json` (41 primitives)
- Seed pool: 21 primitives from 7 domains
- N-tuple size: 7
- Top recipes kept: 30
- Runtime: 2.18s

## Enumeration

- 7-tuple: 116,280

## Scoring summary

- Recipes scored: 116,280
- Max score: 970.0
- Mean score: 936.39
- Cross-domain: 116280 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 116280 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| reranking | 84456 |
| synthesis | 84456 |
| decomposition | 84456 |
| retrieval | 84456 |
| gating | 84456 |
| scoring | 84456 |
| filtering | 84456 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 116280 |
| compare | 116279 |
| scale | 116160 |
| order | 115488 |
| fold | 84456 |
| project | 84456 |

## Top 30 recipes

### Recipe #1 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, bm25-retrieval, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
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
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #2 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #3 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
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

### Recipe #4 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
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

### Recipe #5 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, llm-judge-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring, synthesis
- **Categories**: decomposition, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
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

### Recipe #6 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, primitive-composite-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring, synthesis
- **Categories**: decomposition, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
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

### Recipe #7 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
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
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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

### Recipe #8 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #9 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
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

### Recipe #10 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, llm-judge-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring, synthesis
- **Categories**: decomposition, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
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

### Recipe #11 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, atomic-decomposition, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, primitive-composite-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring, synthesis
- **Categories**: decomposition, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - atomic-decomposition (decomposition): Decompose a complex query into atomic, answerable sub-questions. One query → many sub-Qs.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
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

### Recipe #12 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, primitive-rerank
- **Domains**: reranking, retrieval, synthesis
- **Categories**: reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
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
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #13 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
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
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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

### Recipe #14 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
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
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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

### Recipe #15 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, llm-judge-score, primitive-rerank
- **Domains**: reranking, retrieval, scoring, synthesis
- **Categories**: reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
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
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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

### Recipe #16 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, primitive-composite-score, primitive-rerank
- **Domains**: reranking, retrieval, scoring, synthesis
- **Categories**: reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
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
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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

### Recipe #17 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, fallback-chain, primitive-rerank
- **Domains**: gating, reranking, retrieval, synthesis
- **Categories**: gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
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
  - [extractive-synthesis] No abstraction or reasoning over multiple chunks.
  - [extractive-synthesis] Output length bounded by retrieved content.
  - [extractive-synthesis] Cannot resolve conflicting information.
  - [extractive-synthesis] Quality depends heavily on rerank precision.
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

### Recipe #18 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
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
  - [extractive-synthesis] No abstraction or reasoning over multiple chunks.
  - [extractive-synthesis] Output length bounded by retrieved content.
  - [extractive-synthesis] Cannot resolve conflicting information.
  - [extractive-synthesis] Quality depends heavily on rerank precision.
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

### Recipe #19 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, llm-judge-score, primitive-rerank
- **Domains**: reranking, retrieval, scoring, synthesis
- **Categories**: reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
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
  - [extractive-synthesis] No abstraction or reasoning over multiple chunks.
  - [extractive-synthesis] Output length bounded by retrieved content.
  - [extractive-synthesis] Cannot resolve conflicting information.
  - [extractive-synthesis] Quality depends heavily on rerank precision.
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

### Recipe #20 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, primitive-composite-score, primitive-rerank
- **Domains**: reranking, retrieval, scoring, synthesis
- **Categories**: reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
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

### Recipe #21 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
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

### Recipe #22 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, llm-judge-score, primitive-rerank
- **Domains**: gating, reranking, retrieval, scoring, synthesis
- **Categories**: gating, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
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

### Recipe #23 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, primitive-composite-score, primitive-rerank
- **Domains**: gating, reranking, retrieval, scoring, synthesis
- **Categories**: gating, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** gating → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
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

### Recipe #24 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, llm-judge-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring, synthesis
- **Categories**: decomposition, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
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

### Recipe #25 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, primitive-composite-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring, synthesis
- **Categories**: decomposition, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
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

### Recipe #26 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, cross-encoder-rerank, dense-embedding-retrieval, llm-judge-score, primitive-composite-score, primitive-rerank
- **Domains**: reranking, retrieval, scoring, synthesis
- **Categories**: reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
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

### Recipe #27 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, extractive-synthesis, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, synthesis
- **Categories**: decomposition, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - extractive-synthesis (synthesis): Concatenate or extract relevant spans from top-K retrieved chunks. No LLM generation.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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
  - [primitive-rerank] Quality depends on rerank primitive choice per corpus.
  - [primitive-rerank] Top-K candidates are bounded by stage-1 retriever.
  - [primitive-rerank] Cannot recover gold if stage-1 missed it (recall ceiling).
  - [primitive-rerank] Slower than no-rerank (linear in K).
  - [primitive-rerank] Doesn't help if primitive saturates on top candidates.
```

### Recipe #28 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, fallback-chain, hierarchical-decomposition, primitive-rerank
- **Domains**: decomposition, gating, reranking, retrieval, synthesis
- **Categories**: decomposition, gating, reranking, retrieval, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → reranking → retrieval → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - fallback-chain (gating): Try cheap primitive first; if not confident, escalate to expensive. E.g., BM25 → hybrid → cross-encoder.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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

### Recipe #29 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, llm-judge-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring, synthesis
- **Categories**: decomposition, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - llm-judge-score (scoring): LLM scores (query, doc) or (query, generated_answer) on relevance/quality. Strong but slow.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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

### Recipe #30 🌐 — score 970.0

- **Primitives**: abstractive-synthesis, citation-tracking, cross-encoder-rerank, dense-embedding-retrieval, hierarchical-decomposition, primitive-composite-score, primitive-rerank
- **Domains**: decomposition, reranking, retrieval, scoring, synthesis
- **Categories**: decomposition, reranking, retrieval, scoring, synthesis
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 7 atomic
- **Root atoms used**: combine, order, compare, scale, project, fold

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → reranking → retrieval → scoring → synthesis
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - citation-tracking (synthesis): Preserve source IDs through synthesis for verifiability. Required for trust.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - primitive-composite-score (scoring): Score = weighted sum of primitive scores (BM25, NCD, char-ngram, LSA). Model-free.
  - primitive-rerank (reranking): Re-score top-K candidates using a different primitive (NCD, char-ngram, LSA). Model-free rerank.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [citation-tracking] Citation precision depends on rerank precision.
  - [citation-tracking] LLM may drop or hallucinate citations.
  - [citation-tracking] Citation format (numeric, inline, footnote) is domain-specific.
  - [citation-tracking] Citation noise: too many = cluttered; too few = unverified.
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
