# Retrieval & Search — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## The 8 Foundational Atoms (Root Layer)

These are the canonical 8 atoms from The Painted Fence. All retrieval primitives are wiring of these.

### scan
**Domain:** Retrieval & Search
**Definition:** Stream a thing into units — characters → tokens, tokens → shingles, documents → posting entries, graph → edge list.
**Atom or composite:** Atom
**Cost model:** O(N) in the size of the thing being scanned. Sequential access is cache-friendly; random access is expensive.
**Real wall?** No. But scanning a corpus N times for N queries is O(N²) — index it once, query many times.
**Cross-domain wiring:** Sampling in signal processing. Edge listing in graph processing. Array iteration in linear algebra.
**Notes:** The first step of any retrieval system: how you segment the corpus determines everything downstream.

### hash
**Domain:** Retrieval & Search
**Definition:** A unit → a stable integer. Deterministic hash: same input → same integer. Randomized hash: uses randomness to spread items into pseudo-random buckets.
**Atom or composite:** Atom
**Cost model:** One hash function evaluation. Cryptographic hashes are expensive (designed to be slow). Non-cryptographic (MurmurHash, xxHash) are extremely cheap.
**Real wall?** No. But hash collisions are unavoidable — they introduce false positives in set membership, false negatives in frequency estimation.
**Cross-domain wiring:** Hash in cryptography. Feature hashing in ML. Hash-join in databases. Random projection in linear algebra (Johnson-Lindenstrauss = random hash into a lower-dimensional space).
**Notes:** The difference between a "cheap hash" and an "expensive hash" is exactly the generator/primitive split. The hash FUNCTION is the generator; the hash OPERATION is the primitive.

### fold
**Domain:** Retrieval & Search
**Definition:** Reduce a stream to an accumulator. Variants: fold(count) → term frequency, fold(min) → MinHash, fold(sum) → magnitude, fold(mean) → average.
**Atom or composite:** Atom
**Cost model:** O(N) in stream length. Parallelizable (map-reduce pattern).
**Real wall?** No. But information is lost — fold loses ordering and duplicates beyond the accumulator. This is the rate-distortion trade-off: compression loses detail.
**Cross-domain wiring:** Integrate in signal processing. Accumulate in streaming. Sum over kernel in convolution. Matrix-vector multiply = fold over (row·col) pairs.
**Notes:** This is the most composable primitive in the kit. Every retrieval "score" is some fold over a stream of evidence.

### project
**Domain:** Retrieval & Search
**Definition:** A vector through a matrix — a dot product of the query vector with a basis vector. Embedding = project text into a feature space. PageRank = project over the adjacency matrix.
**Atom or composite:** Atom
**Cost model:** One dot product = O(dim) operations. For sparse vectors, only non-zero dimensions cost. For dense, full dimension.
**Real wall?** No.
**Cross-domain wiring:** In linear algebra: matrix-vector multiply. In signal: mixing. In graphics: transform. In RF: matched filtering.
**Notes:** The key insight: projection is cheap; what fills the vector is the expensive generator. Neural embedder = project through learned weights. TF-IDF = project through term-frequency matrix. Random projection = project through random matrix.

### scale
**Domain:** Retrieval & Search
**Definition:** Divide by a norm or a max — normalize a vector, divide by document frequency, normalize a score to [0,1], clip to maximum.
**Atom or composite:** Atom
**Cost model:** One sqrt + N divisions for full L2 normalize. Or one division for max-normalize.
**Real wall?** No.
**Cross-domain wiring:** Unit normalization in graphics (normalize). L2-normalize in ML. Divide by IDF in retrieval. Divide by L2 norm in signal power normalization.
**Notes:** L2 normalization turns cosine similarity into dot product — same cost, same result. This is why normalized vectors are the preferred representation in retrieval.

### compare
**Domain:** Retrieval & Search
**Definition:** A distance or similarity over a pair — cosine similarity, Jaccard, Hamming, Euclidean, KL divergence. Given two vectors, produce a scalar score.
**Atom or composite:** Atom
**Cost model:** O(dim) for most metrics. Cosine = dot product + normalize. Jaccard = minhash + compare signatures. Hamming = XOR + popcount.
**Real wall?** No.
**Cross-domain wiring:** Correlation in signal processing. Dot product in linear algebra. Matched filter in RF. Cross-entropy loss = compare probability distributions.
**Notes:** Cosine is the workhorse — it measures the angle between two vectors, not their magnitude. For unit-normalized vectors, cosine = dot product = one compare operation.

### combine
**Domain:** Retrieval & Search
**Definition:** A weighted sum of signals — fuse lexical and semantic scores, combine multiple rankers, blend candidate lists. BM25 = tf-fold + idf-combine (weighted sum).
**Atom or composite:** Composite: sum over i of (w_i · signal_i). Weights can be learned, fixed, or data-driven.
**Cost model:** One multiply-add per signal being combined. Near-zero.
**Real wall?** No. But the weighting is the policy decision — wrong weights dilute strong signals. This is the "hybrid mechanism" bug: fusing signal computation with weighting in one function creates dilution.
**Cross-domain wiring:** Weighted sum = linear combination = signal mixing in RF. Alpha compositing in graphics = combine with alpha as weight. Bayesian update = combine prior and likelihood with weights.
**Notes:** The disciplined version: keep signal atoms dumb, pull the weighting policy into the orchestrator. The orchestrator decides how much weight to give each signal — not the signal itself.

### order
**Domain:** Retrieval & Search
**Definition:** Sort by score. Top-k retrieval = partial sort (maintain a heap of size k). Full sort = sort all candidates.
**Atom or composite:** Atom (sort/heapify). Composite for partial sort: heap operations maintain top-k while scanning.
**Cost model:** O(N log k) for partial sort with min-heap of size k. O(N log N) for full sort. O(N) for counting sort / bucket sort if scores are integer.
**Real wall?** No. But the optimal pack under a token budget is knapsack — NP-hard. Greedy top-k is the practical heuristic. This is the computability wall.
**Cross-domain wiring:** Ordering by score = priority queue operations. Sorting by distance = nearest-neighbor search. In signal: ordering by frequency content = spectral ordering.
**Notes:** The heap (priority queue) is the canonical data structure for top-k. Binary heap = O(log k) per insert. Fused filter+sort (WAND) is the standard optimization for early termination.

---

## Inverted Index Atoms

### invert (cross-domain alias: `postings-list-build`, `index`)
**Domain:** Retrieval & Search
**Definition:** Build a mapping from term → list of documents (postings). Each posting = (doc_id, term_frequency, optional payload like position).
**Atom or composite:** Composite: scan(corpus) → for each token in each doc: append(doc_id, tf) to term's posting list.
**Cost model:** O(total_tokens) to build. Postings are sorted by doc_id for efficient merge during retrieval.
**Real wall?** No.
**Cross-domain wiring:** Inverted index = transposition of the term-document matrix. Forward index = column scan; inverted index = row scan. This is matrix transpose.
**Notes:** The difference between a forward index (doc → terms) and inverted index (term → docs) is the transpose. Retrieval uses the inverted form because queries are term lookups.

### posting-merge (cross-domain alias: `intersect`, `AND`, `conjunctive-retrieval`)
**Domain:** Retrieval & Search
**Definition:** Given two sorted posting lists, compute their intersection (AND). Standard algorithm: two pointers, advance the smaller doc_id.
**Atom or composite:** Composite: initialize two pointers → while both not at end: compare doc_ids → advance smaller → on match, emit.
**Cost model:** O(len_A + len_B) worst case. Can skip large gaps using skip pointers (binary search into the larger list).
**Cost model:** No.
**Cross-domain wiring:** Set intersection. In signal: AND of two binary signals. In linear algebra: logical AND as element-wise multiply on binary vectors.
**Notes:** Skip pointers are the B-tree index of the posting list — they allow O(1) skips over large gaps instead of O(gap_size) linear scans.

### score-postings (cross-domain alias: `accumulate-score`, `accumulate`)
**Domain:** Retrieval & Search
**Definition:** For each document in the posting list, accumulate its BM25 / TF-IDF / or other score contribution from the current query term.
**Atom or composite:** Composite: for each (doc_id, tf) in postings: score += weight(term) · bm25(tf, dl, avgdl).
**Cost model:** O(total_matching_postings). The dominant cost in retrieval.
**Real wall?** No.
**Cross-domain wiring:** Score accumulation = fold over posting entries = dot product of query weight vector with document's term frequency vector.
**Notes:** This is the retrieval equivalent of accumulating signal energy — each term contributes to the document's relevance score.

---

## Vector Search Atoms

### ANN-search (cross-domain alias: `approximate-nearest-neighbor`, `ANN`, `graph-traverse`)
**Domain:** Retrieval & Search
**Definition:** Find the approximate k nearest neighbors in a high-dimensional vector space. Methods: HNSW (graph-based), IVF-PQ (inverted index + product quantization), ScaNN ( anisotropic splitting).
**Atom or composite:** Composite: each method has different wiring. HNSW = navigable small world graph + greedy layer traversal + beam search.
**Cost model:** Sub-linear in corpus size. HNSW: O(log N) per query at high recall. PQ: O(centroids_to_search × codebook_size) per query.
**Real wall?** Yes — the curse of dimensionality means exact NN search is O(N). Approximate methods trade recall for speed. The recall/speed trade-off is the conserved quantity.
**Cross-domain wiring:** HNSW traversal = graph walk with probabilistic early stopping = same as PageRank walk with restart. PQ = vector quantization = k-means in the embedding space.
**Notes:** HNSW builds a multi-layer graph — top layer is coarse (long edges), bottom layer is fine (short edges). This is the multi-resolution hierarchy — exactly like mipmaps.

### quantize-vector (cross-domain alias: `product-quantize`, `codebook-project`, `compress`)
**Domain:** Retrieval & Search
**Definition:** Split a high-dimensional vector into subvectors, cluster each subspace independently (k-means), replace each subvector with its nearest centroid ID. The centroid IDs are the compressed representation.
**Atom or composite:** Composite: split(vector) → for each subvector: find-nearest-centroid(subvector) → return centroid-ids.
**Cost model:** Training: k-means over each subspace (expensive). Inference: one distance-to-centroid per subspace (cheap).
**Real wall?** Yes — the codebook size grows as k^d_per_subvector. There's a fundamental bit-rate vs distortion trade-off (rate-distortion theory). More bits = better reconstruction = less distortion.
**Cross-domain wiring:** PQ = vector quantization = k-means clustering. The centroid IDs are analogous to index terms in a vocabulary. In signal: mu-law quantization = non-uniform PQ of audio samples.
**Notes:** The subvector independence assumption is a real limitation — PQ works best when subspaces are relatively decorrelated. For correlated dimensions, residual PQ (RPQ) and Cartesian product codes add a second stage.

### graph-walk (cross-domain alias: `random-walk`, `PageRank`, `hitting-time`)
**Domain:** Retrieval & Search
**Definition:** Traverse a graph starting from seed nodes, following edges according to a transition rule (uniform, weighted, teleport). The visit frequency of each node is the score.
**Atom or composite:** Composite: initialize node scores → repeat: distribute scores to neighbors per transition matrix → until convergence.
**Cost model:** O(edges × iterations). PageRank is the most-studied graph walk — converges in ~50 iterations for typical web graphs.
**Real wall?** No. But the convergence time depends on the spectral gap of the transition matrix — graphs with small spectral gap converge slowly.
**Cross-domain wiring:** PageRank = eigenvector of transition matrix = stationary distribution of random walk = power iteration. In physics: diffusion on a graph. In signal: spreading activation.
**Notes:** Random walk with restart (RWR) is the PageRank variant for relevance propagation — the restart probability controls how much the walk stays near the seed vs explores.

### rerank (cross-domain alias: `re-score`, `cross-encoder-score`, `late-interact`)
**Domain:** Retrieval & Search
**Definition:** Take the candidate list from the first-stage retriever and re-score with a more expensive, higher-quality model. Types: cross-encoder (joint query-doc encoding), ColBERT (late interaction), monoBERT.
**Atom or composite:** The reranker is the generator. The rerank primitive itself is: for each (query, candidate) pair: score = reranker(query, candidate).
**Cost model:** Expensive per candidate (full cross-encoder forward pass). But only applied to top-k candidates from the first stage, so total cost is bounded.
**Real wall?** No.
**Cross-domain wiring:** Cross-encoder = joint projection of query and document = compare in joint space. ColBERT = max-similarity over token-level interactions = fine-grained compare. In signal: a narrowband matched filter re-running on the output of a coarse scan.
**Notes:** The key principle from "The Painted Fence": always do the cheap thing first, escalate to the expensive generator only for the residual the cheap one can't separate.

---

## Term Weighting Atoms

### tf-weight (cross-domain alias: `frequency-weight`, `count-fold`)
**Domain:** Retrieval & Search
**Definition:** Weight a term by its frequency in the document: raw TF, log TF (1 + log(tf)), or BM25's sublinear TF.
**Atom or composite:** Composite: fold(count) over term occurrences in document.
**Cost model:** Counting is free — it's done during indexing.
**Real wall?** No.
**Cross-domain wiring:** Frequency counting = energy measurement in signal processing. Term frequency vector = histogram = probability mass function.
**Notes:** Sublinear TF (log TF) reduces the penalty for repeated terms — a term appearing 10 times shouldn't score 10× a term appearing once.

### idf-weight (cross-domain alias: `rarity-weight`, `document-frequency-scale`)
**Domain:** Retrieval & Search
**Definition:** Weight by inverse document frequency: IDF = log(N / df). Rare terms get high weights; common terms (stop words) get low weights.
**Atom or composite:** Composite: precompute idf(doc) during indexing → multiply tf-weight by idf at query time.
**Cost model:** Near-zero at query time — IDF is precomputed. df counting requires one pass over the corpus.
**Real wall?** No. But IDF is corpus-specific — a term's IDF in one corpus is different from another. This is the "generality" cost of corpus-specific weighting.
**Cross-domain wiring:** IDF ≈ pre-whitening in signal processing. A term's IDF is its signal-to-background ratio in the corpus. Rarest terms carry the most discriminative power = matched filter "whiten the noise" principle.
**Notes:** The IDF-whitened project is the second rung in the SCG's project reinforcement ladder.

### bm25-score (cross-domain alias: `okapi-bm25`, `probabilistic-retrieval-score`)
**Domain:** Retrieval & Search
**Definition:** Robertson-Sparck-Jones probabilistic retrieval model: score = IDF · (tf · (k₁+1)) / (tf + k₁ · (1−b + b·dl/avgdl)). Combines term frequency with document length normalization.
**Atom or composite:** Composite: idf-fold · (tf-fold · (k₁+1)) / (tf + k₁·(1−b + b·dl/avgdl)).
**Cost model:** Near-zero at query time (all components are precomputed). O(number_of_query_terms × avg_posting_length).
**Real wall?** No.
**Cross-domain wiring:** BM25 = fold(tf) with non-linear saturation + fold(document-length) normalization. The saturation function is like a soft step — diminishing returns for repeated terms.
**Notes:** BM25 with k₁=1.2, b=0.75 is the standard baseline. The parameters are tunable — learned from relevance judgments via grid search or LTR.

---

## Hash-Based Retrieval Atoms

### minhash (cross-domain alias: `jaccard-signature`, `set-similarity-hash`)
**Domain:** Retrieval & Search
**Definition:** Estimate Jaccard similarity between two sets using the minimum hash value over permuted sets: h_min(S) = min(π(S)). Jaccard(S₁,S₂) ≈ P(h_min(S₁) = h_min(S₂)).
**Atom or composite:** Composite: scan(set) → hash(element) → fold(min) → repeat k times → signature of k min-hash values.
**Cost model:** k hash computations per set element. k=100-400 is typical for good accuracy. Cheap per element.
**Real wall?** No. But accuracy degrades below k=100 for small sets. The variance of the estimate is ~1/√k.
**Cross-domain wiring:** In cryptography: MinHash ≈ the set reconciliation problem. In signal: measuring overlap between two binary signals. In databases: approximate join detection.
**Notes:** LSH banding on MinHash signatures = bucketing similar sets into the same bucket with probability proportional to their Jaccard similarity.

### simhash (cross-domain alias: `hamming-distance-similarity`, `fingerprint`)
**Domain:** Retrieval & Search
**Definition:** Produce a fingerprint for a set by hashing each element and accumulating into a bit vector using directional bit voting. Two sets with high overlap produce fingerprints with low Hamming distance.
**Atom or composite:** Composite: for each element: compute hash → for each bit: if bit=1, increment counter; if bit=0, decrement → sign of counter = fingerprint bit.
**Cost model:** One hash per element + N bit accumulations. Very cheap.
**Real wall?** No. But sensitive to the hash quality (collision resistance matters for security, not for retrieval).
**Cross-domain wiring:** SimHash ≈ locality-sensitive hashing for Hamming distance. In signal: sign-bit accumulation = 1-bit quantization of a random projection.
**Notes:** The key property: Hamming distance between two fingerprints can be computed in a single XOR + popcount operation. Finding near-duplicates = find all fingerprints within Hamming distance ≤ 3 of the query.

---

## Graph-Based Retrieval Atoms

### centrality (cross-domain alias: `pagerank`, `eigenvector-centrality`, `degree-weight`)
**Domain:** Retrieval & Search
**Definition:** Score nodes by their structural importance in a graph. PageRank: PR(node) = (1−d)/N + d·Σ PR(neighbor)/outdegree(neighbor). Degree centrality: score = in-degree. Betweenness: score = fraction of shortest paths through node.
**Atom or composite:** Composite: build transition matrix → fold(adjacency) → project (iterated) → scale. PageRank = power iteration on the transition matrix.
**Cost model:** Power iteration: O(edges × iterations). Typically converges in ~50 iterations.
**Real wall?** No. But PageRank is defined by the graph structure — changing the graph changes all scores. The "generality" cost: a PageRank score trained on one graph doesn't transfer to another.
**Cross-domain wiring:** PageRank = stationary distribution of random walk = eigenvector of transition matrix. In signal: resonant frequencies of a network = eigenvectors of the adjacency matrix. In physics: eigenvector centrality = consensus dynamics.
**Notes:** For dense import graphs in codebases, a 2-hop walk lights up all nodes — this is the hairball problem. Diffusion with decay is the fix (see physics-diffusion domain).

### label-propagate (cross-domain alias: `spread`, `transductive-learn`, `semi-supervised`)
**Domain:** Retrieval & Search
**Definition:** Propagate labels from seed nodes to unlabeled nodes via graph edges, iterating until convergence. Nodes inherit the majority label (or weighted label) from their neighbors.
**Atom or composite:** Composite: initialize labels → repeat: for each node: aggregate(neighbor_labels) → update(node_label) → until convergence.
**Cost model:** O(edges × iterations). Very fast on sparse graphs.
**Real wall?** No.
**Cross-domain wiring:** Label propagation = spreading activation = diffusion equation on a graph. The update rule is identical to the heat equation: new_heat[node] = avg(heat[neighbors]).
**Notes:** This is the graph equivalent of the diffusion primitive in physics-diffusion.

### graph-expand (cross-domain alias: `neighborhood-grow`, `BFS`, `frontier-expand`)
**Domain:** Retrieval & Search
**Definition:** Expand a set of seed nodes by one or more hops. Return all nodes within k hops of any seed.
**Atom or composite:** Composite: maintain frontier → for each node in frontier: add neighbors to new_frontier → deduplicate → repeat k times.
**Cost model:** O(N_k) where N_k is the k-hop neighborhood size. Can explode in high-degree graphs (the "fan-out" problem).
**Real wall?** Yes — fan-out is exponential in graph degree. A node with degree 1000, 3 hops from seed, adds 1000³ = 1 billion nodes. This is the real wall for graph walks on dense graphs.
**Cross-domain wiring:** BFS = level-set propagation = wavefront expansion = diffusion at discrete time steps. In signal: impulse response of a linear system.
**Notes:** Percolation theory (see physics-diffusion) gives the threshold where the expansion tips from isolated clusters to a giant connected component — the critical point for a diffusion front.

---

## Learning-to-Rank Atoms

### pointwise-score (cross-domain alias: `regression-score`, `CTR-prediction`, `relevance-estimate`)
**Domain:** Retrieval & Search
**Definition:** Predict a relevance score for a (query, document) pair. Treat as regression or classification over relevance grades. Features: BM25, TF-IDF, text overlap, query length, document length, etc.
**Atom or composite:** The model is the generator. The primitive: score = model(features(query, doc)).
**Cost model:** One model inference per (query, doc) pair. Linear models are cheap; gradient boosted trees add cost; neural rerankers are expensive.
**Real wall?** No. But training requires labeled relevance data — collecting relevance judgments is expensive.
**Cross-domain wiring:** Pointwise LTR = regression on (query, doc) pairs. The features are the basis; the model learns the optimal combine weights.
**Notes:** The key limitation: pointwise models don't learn relative ordering — they learn absolute relevance. Pairwise and listwise LTR address this.

### pairwise-compare (cross-domain alias: `margin-learn`, `ranking-pair-loss`)
**Domain:** Retrieval & Search
**Definition:** Learn to rank by comparing pairs: given doc_a and doc_b, predict which is more relevant. Loss = hinge(margin - score(diff)). LambdaRank computes gradients directly on NDCG.
**Atom or composite:** Composite: for each query: generate all pairs → for each pair: compute loss → aggregate gradients → update model.
**Cost model:** O(Q · D²) pairs per query. Impractical for large D — use sampling or listwise methods instead.
**Real wall?** No.
**Cross-domain wiring:** Pairwise loss = SVM ranking margin. The hinge loss is identical to the SVM classifier margin. LambdaRank = gradient descent on the NDCG metric.
**Notes:** LambdaMART (gradient boosted trees + LambdaRank) is the workhorse of industrial LTR — it combines the expressiveness of gradient boosting with the pairwise gradient signal of LambdaRank.

### reciprocal-rank-fusion (cross-domain alias: `RRF`, `rank-combine`, `score-fuse`)
**Domain:** Retrieval & Search
**Definition:** Combine rankings from multiple retrievers using reciprocal rank: score(doc) = Σ 1/(k + rank_r(doc)) for each retriever r. k is a constant (typically 60).
**Atom or composite:** Composite: for each retriever: order by score → assign ranks → combine with RRF formula.
**Cost model:** O(T · k) where T is the number of retrievers and k is the size of the candidate pool. Very cheap.
**Real wall?** No.
**Cross-domain wiring:** RRF = alpha-compositing (Porter-Duff "over") on ranked lists. The constant k acts as a smoothing factor — prevents division by zero and dampens the effect of high ranks.
**Notes:** RRF is robust and requires no training — it works by combining ordinal information (ranks) rather than scores. This is why it's the standard first fusion method before more complex score combination.

---

## Corpus-Level Atoms

### pmi-compute (cross-domain alias: `mutual-information`, `association-measure`)
**Domain:** Retrieval & Search
**Definition:** PMI(x,y) = log P(x,y) / (P(x)·P(y)). Positive PMI indicates association; negative indicates repulsion. NPMI normalizes to [−1, 1].
**Atom or composite:** Composite: fold(cooccurrence) + fold(marginals) → combine(log-ratio).
**Cost model:** Requires co-occurrence counting over the corpus — one pass to build co-occurrence matrix, then compute PMI per pair. Can be expensive for large vocabularies.
**Real wall?** No. But PMI has a rare-term bias — two words that co-occur once get infinite PMI. Normalized PMI or PPMI (positive PMI only) fixes this.
**Cross-domain wiring:** PMI = log-ratio of observed co-occurrence to expected (independent) co-occurrence = log Bayes factor. In information theory: PMI = log-odds of co-occurrence.
**Notes:** PMI-based synonym discovery: words with high mutual PMI are likely synonyms for this corpus. The "this corpus" caveat is important — PMI is corpus-specific, same as IDF.

### corpus-statistics (cross-domain alias: `term-stats`, `index-statistics`)
**Domain:** Retrieval & Search
**Definition:** Pre-computed corpus statistics: document frequency (df), average document length (avgdl), document count (N), term variance, collection term frequency (ctf).
**Atom or composite:** Composite: scan(corpus) → fold(counts) → store statistics.
**Cost model:** One pass over the corpus during indexing. Stored in the index metadata.
**Real wall?** No.
**Cross-domain wiring:** Corpus statistics = distributional properties of the signal = noise PSD estimation. IDF = document-frequency statistics = noise-whitening coefficients.
**Notes:** These statistics are the "generator" for the retrieval score. The score formula is fixed (BM25), but the statistics that parameterize it are corpus-specific.

---

## Summary: Retrieval Atom → Cross-Domain Wiring

| Retrieval Primitive | Signal Alias | Linear Algebra Alias | Graphics Alias |
|---|---|---|---|
| scan | sampling | iterate | tessellate |
| hash | spread spectrum | random projection | hash-into-bucket |
| fold | integrate | sum, mean, min | accumulate |
| project | mix, match | matrix-vector multiply | transform |
| scale | normalize power | L2-normalize | normalize |
| compare | correlate | dot product, distance | template match |
| combine | weighted sum | linear combination | alpha over |
| order | rank by amplitude | partial sort | priority queue |
| invert | transpose | transpose | transpose index |
| ANN-search | matched filter scan | nearest neighbor | range query |
| graph-walk | random walk | power iteration | traversal |
| centrality | spectral analysis | eigenvector | structural importance |
| pmi-compute | log-odds | log-ratio | association score |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## ANN / Vector Search Atoms

### hnsw-build (cross-domain alias: `layered-NSW`, `hierarchical-navigate`, `small-world-graph`)
**Domain:** Retrieval & Search
**Definition:** Hierarchical Navigable Small World — multi-layer graph where each layer is an NSW graph with decreasing average degree. Top layer: sparse long-range connections. Bottom layer: dense short-range connections.
**Atom or composite:** Composite: build NSW graphs at each layer → insert nodes greedily (search top-down, refine bottom-up) → connect to EN (enter node) at each layer.
**Cost model:** Build time: O(N log N) for layered construction. Search time: O(log N) per query at high recall. Memory: O(N·ef_construction·M).
**Real wall?** Yes — HNSW build is offline and expensive (cannot update incrementally). For high-update corpora, use IVF or dynamic graph alternatives.
**Cross-domain wiring:** HNSW layer hierarchy = multi-resolution search = same as the SCG tier hierarchy. In graphics: HNSW = the same multi-layer spatial index as a mipmap pyramid.
**Notes:** The two key parameters: ef_construction (size of the dynamic candidate list during insertion) controls build quality; M (number of connections per node) controls search speed vs recall.

### ivf-search (cross-domain alias: `inverted-file-index`, `cluster-centroid-search`, `IVF-PQ`)
**Domain:** Retrieval & Search
**Definition:** Cluster vectors into k clusters; during search, only search clusters whose centroids are closest to the query. IVF-PQ adds product quantization to the inverted lists for compressed storage.
**Atom or composite:** Composite: assign vectors to centroids (offline) → at query time: find k nearest centroids → scan inverted lists for selected centroids → collect candidates → re-rank.
**Cost model:** Search: O(k·c + k·n_c) where k = clusters searched, c = avg cluster size, n_c = PQ code distance. Much faster than linear scan for large corpora.
**Real wall?** Yes — the recall of IVF depends on the cluster coherence. Bad clustering (overlapping, imbalanced) means relevant vectors are missed.
**Cross-domain wiring:** IVF = inverted index for vectors = same structure as the inverted index for text. The cluster centroid = the "term" in the text index. In physics: clustering = phase space partitioning.
**Notes:** The nprobe parameter (number of clusters to search) trades recall for speed. The right nprobe depends on the query distribution and cluster quality.

### scann-split (cross-domain alias: `anisotropic-split`, `score-aware-loss`, `axial-search`)
**Domain:** Retrieval & Search
**Definition:** ScaNN's main contribution: split the scoring space using an anisotropic partitioning (not spherical like k-means). The split maximizes the margin between near and far vectors — better use of the vector space.
**Atom or composite:** Composite: build anisotropic partition tree → at query time: traverse using an axial search that prunes based on the maximum possible score in each subtree.
**Cost model:** Tree build is expensive but offline. Search: highly efficient due to anisotropic pruning.
**Real wall?** No. But the partition must be rebuilt when the corpus changes significantly.
**Cross-domain wiring:** ScaNN's anisotropic split = decision tree with data-dependent splits = same as the optimal retrieval boundary.
**Notes:** ScaNN won the ann-benchmark for recall vs latency in 2019-2020 before being overtaken by new HNSW variants and disk-based indexes.

### disk-ANN (cross-domain alias: `PQ-append`, `DiskANN`, `SSD-index`)
**Domain:** Retrieval & Search
**Definition:** ANN indexes that store vectors on disk/SSD and stream them during search, enabling billion-scale indexes that don't fit in RAM. Uses PQ for compression and graph traversal to minimize disk I/O.
**Atom or composite:** Composite: PQ-encode all vectors → build disk-resident graph → during search: navigate graph, streaming PQ codes from disk → decode on-the-fly.
**Cost model:** I/O bound — the number of disk reads per query is the critical metric. Disk bandwidth (~3 GB/s) vs memory bandwidth (~300 GB/s) means each disk access is ~100× more expensive.
**Real wall?** Yes — the disk bandwidth is a real wall. Disk-ANN is limited by I/O, not compute. Optimizing for disk locality (sequential reads) is critical.
**Cross-domain wiring:** Disk-ANN = memory hierarchy management = same as cache-oblivious algorithms. In operating systems: virtual memory page eviction.
**Notes:** SPTAG (Microsoft) and DiskANN (Microsoft Research) are the main disk-ANN systems. The key insight: the graph traversal pattern must be sequential enough to exploit OS prefetching.

---

## Query Understanding Atoms

### query-expand (cross-domain alias: `pseudo-RF`, ` Relevance-feedback`, `doc-terms-add`)
**Domain:** Retrieval & Search
**Definition:** Add terms from top-ranked documents to the original query. Rocchio expansion: q_new = α·q + β/|D_r|·Σ d∈D_r·d − γ/|D_nr|·Σ d∈D_nr·d. Adds discriminative terms from relevant documents.
**Atom or composite:** Composite: retrieve top-k docs → extract top-weight terms → add to query vector with weights → re-retrieve.
**Cost model:** One extra retrieval pass + term extraction. Expensive but can significantly improve recall.
**Real wall?** Yes — query expansion can hurt precision by adding terms that are discriminative in the top results but not generally relevant.
**Cross-domain wiring:** Query expansion = spreading activation from seed documents = the same as relevance propagation in a graph. In ML: pseudo-labeling = using model predictions as supervision.
**Notes:** Pseudo-relevance feedback (PRF) assumes top-k are relevant — if they're not, expansion degrades. It is the blind RF version of query expansion.

### query-rewrite (cross-domain alias: `query-normalize`, `spell-correct`, `normalize-query`)
**Domain:** Retrieval & Search
**Definition:** Transform the query to canonical form before retrieval. Steps: lowercase → strip accents → correct spelling → normalize whitespace → detect language → handle special characters.
**Atom or composite:** Composite: normalize(text) → spell-check → detect-language → apply language-specific rules.
**Cost model:** One pass over the query. Cheap — but must be fast because this is in the hot path.
**Real wall?** No. But normalization rules are language-specific and must be curated for each language in the corpus.
**Cross-domain wiring:** Query normalization = data cleaning = feature normalization in ML. In signal: DC removal = removing the constant component.
**Notes:** The biggest precision gains from query rewriting come from handling typos and synonyms — these are the most common causes of retrieval failure.

### query-classify (cross-domain alias: `intent-detect`, `question-type`, `nav-vs-nav`)
**Domain:** Retrieval & Search
**Definition:** Classify the query type: navigational (find a specific site), informational (find information), transactional (find a service). Different types need different retrieval strategies.
**Atom or composite:** The classifier is the generator. The primitive: given query → return class label.
**Cost model:** One model inference (fast classifier) per query. Usually a lightweight BERT variant or even a keyword-based classifier.
**Real wall?** No. But query classification errors cascade into poor retrieval — a navigational query classified as informational will return broad content instead of the target site.
**Cross-domain wiring:** Query type = the "mode" of the retrieval — analogous to the modality (text, image, video) in multi-modal retrieval.
**Notes:** For e-commerce: the taxonomy is often navigational (brand search), informational (product comparison), and transactional (add-to-cart intent). Each needs different retrieval.

### synonym-handle (cross-domain alias: `synonym-expand`, `ABLN-detect`, `phrase-handle`)
**Domain:** Retrieval & Search
**Definition:** Handle synonyms and related terms: car ≈ automobile, ATM ≈ cash machine. Techniques: curated synonym dictionaries (ABLN rules), distributional synonyms from word embeddings, or learned synonym embeddings.
**Atom or composite:** Composite: for each term: look up synonyms → add to query with original weight or reduced weight.
**Cost model:** Dictionary lookup is O(1). Learned synonym models add latency.
**Real wall?** Yes — false synonyms (words that are similar in one context but not another) degrade precision. "bank" (river) vs "bank" (financial) is the classic case.
**Cross-domain wiring:** Synonym handling = query expansion with controlled vocabulary. In retrieval: controlled vocabulary = IDF-weighting of the controlled term list.
**Notes:** The WordNet synonym graph is the canonical curated synonym resource for English. Domain-specific ontologies (medical: UMLS, e-commerce: product taxonomy) are better for specialized corpora.

---

## Multi-Vector / Late Interaction Atoms

### colbert-score (cross-domain alias: `max-similarity`, `late-interaction`, `token-level-score`)
**Domain:** Retrieval & Search
**Definition:** ColBERT: encode query and document independently into token-level embeddings. Score = Σ max_i Σ_j (query_token_i · doc_token_j) over all query tokens. The max aggregates fine-grained similarity.
**Atom or composite:** Composite: encode query → encode document → for each query token: compute dot product with all document tokens → fold(max) → sum over query tokens.
**Cost model:** More expensive than single-vector dot product (requires N×M dot products for N query tokens, M doc tokens). But each is a small dot product (128-dim).
**Real wall?** Yes — ColBERT stores M token embeddings per document, not one. This increases storage by the average document token count. MaxSim is the cost.
**Cross-domain wiring:** ColBERT max-similarity = the same as the "max" operation in Count-Min (taking the maximum over hash buckets). It's a conservative estimate (upper bound on similarity).
**Notes:** Late interaction preserves fine-grained matching (query term "bank" can match "bank account" even if the document "bank" alone would have lower score). This is a key advantage over single-vector models.

### late-interact (cross-domain alias: `cross-encoder-light`, `decompose-score`, `bimodal-score`)
**Domain:** Retrieval & Search
**Definition:** Score a (query, document) pair by decomposing into independent sub-representations: ColBERT-style token interaction, or Decomposable Attention Model (DAM) using attention to compare sets.
**Atom or composite:** Composite: decompose query and doc → compare sub-units (tokens, spans) → aggregate comparisons.
**Cost model:** Lower than full cross-encoder (which encodes query and doc jointly). Higher than bi-encoder (which encodes independently).
**Real wall?** No. But late interaction still requires computing sub-representations for both query and doc — it is not free like bi-encoder scoring.
**Cross-domain wiring:** Late interaction = comparing two sets via all pairwise comparisons → aggregation via max or attention. In signal: cross-correlation at lag 0 = late interaction between two signals.
**Notes:** The original Decomposable Attention Model (Parikh et al., 2016) uses attention to compare two sequences — this is the linguistic equivalent of ColBERT.

### token-weight-learn (cross-domain alias: `learned-term-weight`, `term-importance`, `neural-IDF`)
**Domain:** Retrieval & Search
**Definition:** Learn per-term importance weights from data instead of using fixed IDF. A lightweight model predicts the importance of each token given the corpus statistics.
**Atom or composite:** The model is the generator. The primitive: weight = learned_importance(term, context).
**Cost model:** One look-up per token (the learned weight). Pre-computed and stored in the index.
**Real wall?** No. But the learned weights must be recomputed when the corpus changes significantly — they are corpus-specific, same as IDF.
**Cross-domain wiring:** Learned term weights = learned filter coefficients = adaptive filtering. In signal: adaptive filter = weights adapted to the signal statistics.
**Notes:** DeepCT (Subramaniam et al., 2019) uses a BERT-based model to predict per-passage term weights, then reweights the BM25 scores. This is a learned "IDF" that captures semantic importance.

---

## Learning to Rank Atoms

### lambdamart (cross-domain alias: `GBDT-rank`, `gradient-boosted-LTR`, `LambdaRank`)
**Domain:** Retrieval & Search
**Definition:** LambdaMART = LambdaRank + MART (gradient boosted regression trees). LambdaRank computes gradient of NDCG directly (the "Lambda" gradient); MART fits regression trees to the Lambda gradient.
**Atom or composite:** Composite: for each query: compute relevance grades → compute Lambda gradients → fit regression tree to gradients → update model → repeat.
**Cost model:** Training: O(Q·D·N·T) where Q = queries, D = tree depth, N = features, T = iterations. Inference: O(D) per query.
**Real wall?** No. But LambdaMART requires labeled relevance data (grade annotations per query-document pair) — collecting this is expensive.
**Cross-domain wiring:** LambdaMART gradient = gradient of a ranking metric = policy gradient in RL. In physics: the Lambda gradient is like the force that pushes documents toward their correct rank.
**Notes:** LightGBM's lambdarank objective implements LambdaMART efficiently. The key hyperparameter is the number of leaves — more leaves = more complex ranking function = more overfitting risk.

### listwise-LTR (cross-domain alias: `ListNet`, `ListMLE`, `permutation-likelihood`)
**Domain:** Retrieval & Search
**Definition:** Directly optimize a listwise loss over the full ranking: ListNet minimizes cross-entropy between the model's probability distribution over permutations and the ground-truth distribution. ListMLE uses the likelihood of the ground-truth permutation.
**Atom or composite:** Composite: compute permutation probability distribution → compute listwise loss → gradient descent.
**Cost model:** O(D·L·log L) per query where D = document count, L = number of candidate documents. Expensive for large candidate sets.
**Real wall?** Yes — the combinatorial explosion of permutations makes exact listwise optimization infeasible for large candidate lists. Approximations are needed.
**Cross-domain wiring:** Listwise LTR = policy gradient on the ranking policy. In ML: the ranking policy = the retrieval policy; the reward = NDCG.
**Notes:** ListNet's cross-entropy over permutations = softmax over scores → minimize KL(ground_truth || softmax(model_scores)). This is the same as the neural loss for neural rerankers.

### pointwise-LTR-advanced (cross-domain alias: `deep-pointwise`, `regression-rank`, `BERT-rerank`)
**Domain:** Retrieval & Search
**Definition:** Treat each (query, document, label) as an independent regression/classification sample. Use deep models (BERT, MonoBERT) to score each pair independently. Simpler than pairwise/listwise but ignores cross-document interactions.
**Atom or composite:** The model is the generator. The primitive: score = model(query, document).
**Cost model:** Expensive — one BERT forward pass per (query, document) pair. Only applied to top-k candidates from the first stage.
**Real wall?** No. But it misses the "document A should rank above document B" relational signal that pairwise/listwise methods capture.
**Cross-domain wiring:** Pointwise LTR = independent classification per document = the same as treating each document as an isolated case. In ML: per-sample loss vs relational loss.
**Notes:** The cascade: bi-encoder (cheap) → pointwise BERT reranker (expensive, applied to top-100) is the standard industrial pipeline.

---

## Diversity / Coverage Atoms

### MMR (cross-domain alias: `maximal-marginal-relevance`, `diversity-promote`, `novelty-select`)
**Domain:** Retrieval & Search
**Definition:** Balance relevance and diversity: MMR = argmax_{d∈C\D} [α·sim(q,d) − (1−α)·max_{d'∈D} sim(d,d')]. The second term penalizes documents similar to already-selected ones.
**Atom or composite:** Composite: sort by relevance → iteratively: select doc that maximizes relevance − diversity penalty → add to result → repeat.
**Cost model:** O(k·|C|) where k = result size, |C| = candidate set. The diversity term requires comparing each candidate to all selected docs.
**Real wall?** No. But the α parameter trades off relevance vs diversity — α=1 is pure relevance, α=0 is pure diversity.
**Cross-domain wiring:** MMR = repulsive potential in physics (diverse docs push each other away). In ML: it is similar to ensemble diversity — selecting models that disagree.
**Notes:** The similarity function for diversity can use a different embedding space than the relevance space — e.g., topic diversity vs semantic similarity.

### IA-Select (cross-domain alias: `intent-aware-select`, `subtopic-coverage`, `proportional-cover`)
**Domain:** Retrieval & Search
**Definition:** Ensure coverage of all query subtopics/intents: select documents proportional to the subtopic probability given the query. IA-Select: at rank i, select the document that covers the most uncovered intent mass.
**Atom or composite:** Composite: classify doc subtopic coverage → track covered intent mass → select doc that maximizes uncovered intent coverage.
**Cost model:** Requires subtopic classification per document. One extra classification per candidate.
**Real wall?** No. But subtopic taxonomy must be available and must be relevant to the query — bad taxonomy = bad diversity.
**Cross-domain wiring:** IA-Select = proportional coverage = stratified sampling. In information theory: it maximizes the coverage of the query's intent space.
**Notes:** xQuAD (explicit query aspect diversity) extends MMR by explicitly modeling subtopic coverage as a separate objective.

---

## Session / Conversational Atoms

### session-expand (cross-domain alias: `conversation-history`, `context-window`, `multi-turn-query`)
**Domain:** Retrieval & Search
**Definition:** Expand the current query using the conversation history. Approaches: concatenate previous queries, use attention over history, or maintain a running query vector.
**Atom or composite:** Composite: retrieve conversation history → combine with current query → run retrieval on combined query.
**Cost model:** One extra retrieval per turn (for history). Attention-based history costs more.
**Real wall?** Yes — conversation history can introduce topic drift (shifting from the original intent). Anaphora resolution ("that one," "it") must correctly link to the previous turn.
**Cross-domain wiring:** Session expansion = maintaining context in a stateful retrieval system. In physics: the conversation history is like the "initial conditions" of a diffusion process.
**Notes:** A key challenge: how to weight the contribution of each historical turn. Recent turns are usually more relevant — a recency-weighted expansion is standard.

### coreference-resolve (cross-domain alias: `pronoun-link`, `entity-tracking`, `anaphoric-reference`)
**Domain:** Retrieval & Search
**Definition:** Link pronouns and referring expressions in the current query to entities mentioned in the conversation history. "that" → "the red car from two turns ago."
**Atom or composite:** The coreference resolver is the generator. The primitive: given current query + history → resolve anaphora → expanded query.
**Cost model:** One coreference inference per query turn. Fast for simple rule-based; slower for neural coreference.
**Real wall?** Yes — coreference resolution errors cascade. Resolving "it" to the wrong entity produces a completely wrong retrieval.
**Cross-domain wiring:** Coreference = entity linking in the temporal dimension. In knowledge graphs: linking entities across time = linking across documents.
**Notes:** SpanBERT and other neural coreference models have largely replaced rule-based approaches for English. For specialized domains, a rule-based coreference system may be more reliable.

---

*Last updated: 2026-06-21 (expanded with ANN, query understanding, multi-vector, LTR, diversity, session)*
*Source doctrine: The Painted Fence — Jesse*
