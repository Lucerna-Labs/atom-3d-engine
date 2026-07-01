# Primitive Combination Report

## Run metadata

- Timestamp: `2026-06-24T13:38:28.749461+00:00`
- Seed: `42`
- Catalog: `D:\MINIMAX-PRIMITVE-SIMULATIONS\src\rag_primitives_catalog.json` (41 primitives)
- Seed pool: 40 primitives from 8 domains
- N-tuple size: 5
- Top recipes kept: 50
- Runtime: 10.83s

## Enumeration

- 5-tuple: 658,008

## Scoring summary

- Recipes scored: 658,008
- Max score: 413.0
- Mean score: 394.35
- Cross-domain: 657940 (100.0%)

### Wiring pattern distribution

| pattern | count | % |
|---------|-------|---|
| fusion ranking | 658008 | 100.0% |

### Domain appearance counts

| domain | appearances |
|--------|-------------|
| retrieval | 456632 |
| workflow | 379752 |
| decomposition | 379752 |
| synthesis | 281016 |
| gating | 281016 |
| reranking | 281016 |
| scoring | 281016 |
| filtering | 281016 |

### Root atom coverage

| atom | uses |
|------|------|
| combine | 657952 |
| compare | 649440 |
| order | 631674 |
| scale | 615504 |
| fold | 420672 |
| project | 333376 |
| sample | 281016 |
| hash | 82251 |
| scan | 82251 |

## Top 50 recipes

### Recipe #1 🌐 — score 413.0

- **Primitives**: bm25-retrieval, dedup-merge, dense-embedding-retrieval, parallel-fan-out, rag-fusion
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, compare, hash, project, scan, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #2 🌐 — score 413.0

- **Primitives**: bm25-retrieval, dedup-merge, dense-embedding-retrieval, parallel-fan-out, react-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, compare, hash, project, scan, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #3 🌐 — score 413.0

- **Primitives**: bm25-retrieval, dedup-merge, dense-embedding-retrieval, parallel-fan-out, self-ask-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, compare, hash, project, scan, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #4 🌐 — score 413.0

- **Primitives**: bm25-retrieval, dedup-merge, dense-embedding-retrieval, parallel-fan-out, tree-of-retrieval
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, compare, hash, project, scan, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #5 🌐 — score 413.0

- **Primitives**: chain-of-retrieval, dedup-merge, dense-embedding-retrieval, parallel-fan-out, rag-fusion
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, fold, compare, hash, project, scale, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
**Real walls (combined):**
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
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
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #6 🌐 — score 413.0

- **Primitives**: chain-of-retrieval, dedup-merge, dense-embedding-retrieval, parallel-fan-out, react-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, fold, compare, hash, project, scale, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
**Real walls (combined):**
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
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
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #7 🌐 — score 413.0

- **Primitives**: chain-of-retrieval, dedup-merge, dense-embedding-retrieval, parallel-fan-out, self-ask-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, fold, compare, hash, project, scale, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
**Real walls (combined):**
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
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
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #8 🌐 — score 413.0

- **Primitives**: chain-of-retrieval, dedup-merge, dense-embedding-retrieval, parallel-fan-out, tree-of-retrieval
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: order, combine, fold, compare, hash, project, scale, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
**Real walls (combined):**
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
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
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #9 🌐 — score 413.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, rag-fusion
- **Domains**: decomposition, gating, retrieval, synthesis, workflow
- **Categories**: decomposition, gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: compare, combine, hash, project, scale, order, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis → workflow
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
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
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #10 🌐 — score 413.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, react-workflow
- **Domains**: decomposition, gating, retrieval, synthesis, workflow
- **Categories**: decomposition, gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: compare, combine, hash, project, scale, order, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis → workflow
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
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
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #11 🌐 — score 413.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, self-ask-workflow
- **Domains**: decomposition, gating, retrieval, synthesis, workflow
- **Categories**: decomposition, gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: compare, combine, hash, project, scale, order, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis → workflow
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
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
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #12 🌐 — score 413.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, tree-of-retrieval
- **Domains**: decomposition, gating, retrieval, synthesis, workflow
- **Categories**: decomposition, gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: compare, combine, hash, project, scale, order, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis → workflow
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
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
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #13 🌐 — score 413.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, rag-fusion
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: compare, combine, hash, project, scale, fold, scan, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #14 🌐 — score 413.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, react-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: compare, combine, hash, project, scale, fold, scan, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #15 🌐 — score 413.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, self-ask-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: compare, combine, hash, project, scale, fold, scan, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #16 🌐 — score 413.0

- **Primitives**: dedup-merge, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, tree-of-retrieval
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: compare, combine, hash, project, scale, fold, scan, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
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
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #17 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dedup-merge, dense-embedding-retrieval, rag-fusion
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, scale, fold, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
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
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #18 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dedup-merge, dense-embedding-retrieval, react-workflow
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, scale, fold, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
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
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #19 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dedup-merge, dense-embedding-retrieval, self-ask-workflow
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, scale, fold, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
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
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #20 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dedup-merge, dense-embedding-retrieval, tree-of-retrieval
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, scale, fold, hash, project, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
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
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #21 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, parallel-fan-out, rag-fusion
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
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
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #22 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, parallel-fan-out, react-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
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
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #23 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, parallel-fan-out, self-ask-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
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
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #24 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, bm25-retrieval, dense-embedding-retrieval, parallel-fan-out, tree-of-retrieval
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, scale, fold, project, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
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
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #25 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, chain-of-retrieval, dedup-merge, dense-embedding-retrieval, rag-fusion
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, fold, hash, project, scale, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #26 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, chain-of-retrieval, dedup-merge, dense-embedding-retrieval, react-workflow
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, fold, hash, project, scale, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #27 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, chain-of-retrieval, dedup-merge, dense-embedding-retrieval, self-ask-workflow
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, fold, hash, project, scale, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #28 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, chain-of-retrieval, dedup-merge, dense-embedding-retrieval, tree-of-retrieval
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, fold, hash, project, scale, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #29 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, chain-of-retrieval, dense-embedding-retrieval, parallel-fan-out, rag-fusion
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, fold, project, scale, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #30 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, chain-of-retrieval, dense-embedding-retrieval, parallel-fan-out, react-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, fold, project, scale, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #31 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, chain-of-retrieval, dense-embedding-retrieval, parallel-fan-out, self-ask-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, fold, project, scale, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #32 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, chain-of-retrieval, dense-embedding-retrieval, parallel-fan-out, tree-of-retrieval
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, fold, project, scale, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - chain-of-retrieval (workflow): Sequential retrieval: each step depends on previous answer. For multi-hop queries.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
**Real walls (combined):**
  - [abstractive-synthesis] Hallucination: LLM may invent info not in context.
  - [abstractive-synthesis] Context window: limited to model's input size.
  - [abstractive-synthesis] Latency: 100-500ms per generation.
  - [abstractive-synthesis] Cost: GPT-4-class models are $0.01-$0.10 per answer.
  - [abstractive-synthesis] Quality depends on retrieved context quality.
  - [chain-of-retrieval] Latency compounds (steps are sequential).
  - [chain-of-retrieval] Error compounds: bad first answer → bad second.
  - [chain-of-retrieval] Cost grows with hops.
  - [chain-of-retrieval] Stop condition (when to stop?) is non-trivial.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [parallel-fan-out] Cost = sum of all parallel runs.
  - [parallel-fan-out] Diminishing returns past 3-4 parallel retrievers.
  - [parallel-fan-out] Result combination is non-trivial (RRF, voting, etc.).
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #33 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, hierarchical-decomposition, rag-fusion
- **Domains**: decomposition, retrieval, synthesis, workflow
- **Categories**: decomposition, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
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
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #34 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, hierarchical-decomposition, react-workflow
- **Domains**: decomposition, retrieval, synthesis, workflow
- **Categories**: decomposition, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
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
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #35 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, hierarchical-decomposition, self-ask-workflow
- **Domains**: decomposition, retrieval, synthesis, workflow
- **Categories**: decomposition, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
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
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #36 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, hierarchical-decomposition, tree-of-retrieval
- **Domains**: decomposition, retrieval, synthesis, workflow
- **Categories**: decomposition, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
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
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #37 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, ncd-retrieval, rag-fusion
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
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
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #38 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, ncd-retrieval, react-workflow
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
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
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #39 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, ncd-retrieval, self-ask-workflow
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
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
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #40 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dedup-merge, dense-embedding-retrieval, ncd-retrieval, tree-of-retrieval
- **Domains**: retrieval, synthesis, workflow
- **Categories**: retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, hash, project, scale, fold, sample

```
**Wiring:** fusion ranking
**Cross-domain:** retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
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
  - [ncd-retrieval] Compression quality varies by algorithm (bzip2, lzma, zstd).
  - [ncd-retrieval] Computationally expensive vs cosine (10-100x).
  - [ncd-retrieval] Biased toward compressible text.
  - [ncd-retrieval] Effective cross-vocabulary but slow at scale.
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #41 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, rag-fusion
- **Domains**: decomposition, gating, retrieval, synthesis, workflow
- **Categories**: decomposition, gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
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
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #42 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, react-workflow
- **Domains**: decomposition, gating, retrieval, synthesis, workflow
- **Categories**: decomposition, gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
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
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #43 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, self-ask-workflow
- **Domains**: decomposition, gating, retrieval, synthesis, workflow
- **Categories**: decomposition, gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
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
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #44 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, hierarchical-decomposition, parallel-fan-out, tree-of-retrieval
- **Domains**: decomposition, gating, retrieval, synthesis, workflow
- **Categories**: decomposition, gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** decomposition → gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - hierarchical-decomposition (decomposition): Recursive decomposition: sub-Qs split further if they're still complex. Tree of questions.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
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
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #45 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, rag-fusion
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
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
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #46 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, react-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
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
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```

### Recipe #47 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, self-ask-workflow
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - self-ask-workflow (workflow): LLM asks itself clarifying questions, retrieves, asks more, then answers. For complex multi-step queries.
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
  - [self-ask-workflow] LLM may get stuck in loops.
  - [self-ask-workflow] Question quality depends on LLM.
  - [self-ask-workflow] Cost is unpredictable.
  - [self-ask-workflow] Stop conditions are critical.
```

### Recipe #48 🌐 — score 412.0

- **Primitives**: abstractive-synthesis, dense-embedding-retrieval, ncd-retrieval, parallel-fan-out, tree-of-retrieval
- **Domains**: gating, retrieval, synthesis, workflow
- **Categories**: gating, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, order, compare, project, scale, fold, scan, sample

```
**Wiring:** fusion ranking
**Cross-domain:** gating → retrieval → synthesis → workflow
**Components:**
  - abstractive-synthesis (synthesis): LLM generates answer from retrieved context. Most flexible; can reason over multiple chunks.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - ncd-retrieval (retrieval): Normalized Compression Distance: similarity = (C(xy) - min(C(x),C(y))) / max(C(x),C(y)). Cross-vocabulary similarity.
  - parallel-fan-out (gating): Run multiple retrievers in parallel; combine. Higher quality but higher cost.
  - tree-of-retrieval (workflow): Branching retrieval: explore multiple paths, prune low-confidence branches.
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
  - [tree-of-retrieval] Tree width must be bounded (3-5 typical).
  - [tree-of-retrieval] Pruning threshold tuning.
  - [tree-of-retrieval] Combine step at root must reconcile branches.
  - [tree-of-retrieval] Cost grows exponentially without pruning.
```

### Recipe #49 🌐 — score 412.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dedup-merge, dense-embedding-retrieval, rag-fusion
- **Domains**: reranking, retrieval, synthesis, workflow
- **Categories**: reranking, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, compare, hash, project, sample, order

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis → workflow
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - rag-fusion (workflow): Generate N query variants via LLM, retrieve each, RRF-fuse results. Higher recall than single query.
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
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [rag-fusion] N variants: 3-5 typical.
  - [rag-fusion] Cost = N * retrieve.
  - [rag-fusion] Diminishing returns past N=5.
  - [rag-fusion] Quality depends on LLM-generated variants.
```

### Recipe #50 🌐 — score 412.0

- **Primitives**: bm25-retrieval, cross-encoder-rerank, dedup-merge, dense-embedding-retrieval, react-workflow
- **Domains**: reranking, retrieval, synthesis, workflow
- **Categories**: reranking, retrieval, synthesis, workflow
- **Wiring**: fusion ranking
- **Architecture**: 0 composite + 5 atomic
- **Root atoms used**: combine, scale, fold, compare, hash, project, order, sample

```
**Wiring:** fusion ranking
**Cross-domain:** reranking → retrieval → synthesis → workflow
**Components:**
  - bm25-retrieval (retrieval): Probabilistic relevance ranking with TF saturation and document length normalization. Industry standard for sparse retrieval.
  - cross-encoder-rerank (reranking): Cross-encoder model scores (query, doc) pair jointly. State-of-the-art rerank quality.
  - dedup-merge (synthesis): Remove near-duplicate chunks (cosine or NCD > threshold) and merge their content.
  - dense-embedding-retrieval (retrieval): Dense embedding via neural model (bge-m3, qwen3-embedding); cosine similarity in d-dim space. Bridges vocabulary gap.
  - react-workflow (workflow): Reasoning + Acting loop: LLM reasons about what to retrieve next, retrieves, observes, repeats.
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
  - [dedup-merge] Threshold tuning per corpus.
  - [dedup-merge] May merge complementary info from near-duplicates that look the same.
  - [dedup-merge] Computational cost: O(N^2) naive, O(N log N) with LSH.
  - [dense-embedding-retrieval] Requires GPU or fast inference endpoint.
  - [dense-embedding-retrieval] Latency: 10-50ms per query on CPU; 5-10ms on GPU.
  - [dense-embedding-retrieval] Quality varies by model size; bge-m3 (566M) often beats qwen3-4b on small fixtures.
  - [dense-embedding-retrieval] Cannot embed documents longer than model's context window.
  - [dense-embedding-retrieval] Non-deterministic across model versions.
  - [react-workflow] LLM hallucination in reasoning step.
  - [react-workflow] Action space must be well-defined.
  - [react-workflow] Termination conditions matter.
  - [react-workflow] Quality depends on LLM's instruction-following.
```
