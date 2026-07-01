# Database & Streaming / Sketching — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Streaming Aggregation Atoms

### window (cross-domain alias: `partition`, `slice`, `tumble`)
**Domain:** Database & Streaming / Sketching
**Definition:** Divide a stream into fixed-size time intervals (tumbling window), sliding intervals (sliding window), or session-based gaps. Each window is processed independently.
**Atom or composite:** Composite: emit when window closes (tumbling) or continuously (sliding). Tumbling = scan + boundary-check. Sliding = scan + queue management.
**Cost model:** Tumbling is O(1) per event. Sliding requires maintaining a queue and expiring old elements.
**Real wall?** No.
**Cross-domain wiring:** In signal processing: a gate function = selecting a time window. In graphics: viewport clipping = windowing in pixel space. In retrieval: query time range filter = window on timestamp.
**Notes:** Session windows (defined by gaps > threshold) are especially useful for user behavior streams — they define sessions without fixed boundaries.

### fold-window (cross-domain alias: `aggregate-window`, `reduce`, `summarize`)
**Domain:** Database & Streaming / Sketching
**Definition:** Apply a fold function (sum, count, min, max, mean, variance) over a window. Streaming folds maintain running state — O(1) update per element.
**Atom or composite:** Composite: for each event: update(state, event). State is the fold accumulator.
**Cost model:** O(1) per element for most aggregates. Variance requires maintaining sum and sum-of-squares (two accumulators).
**Real wall?** No. But for order-sensitive folds (top-k, percentile), maintaining exact state requires unbounded memory. Approximate quantile sketches are the solution.
**Cross-domain wiring:** Streaming fold = signal integration over a time window. In linear algebra: running mean = exponential moving average. In retrieval: corpus statistics = streaming fold over the document stream.
**Notes:** Exponential moving average (EMA): new_mean = α·x + (1−α)·old_mean. This is a fold with exponential forgetting — useful for non-stationary streams.

### watermark (cross-domain alias: `garbage-collect`, `event-time-boundary`, `staleness-marker`)
**Domain:** Database & Streaming / Sketching
**Definition:** A watermark is a timestamp T such that all events with event_time < T have been observed. Allows the system to emit windows that are probably complete (up to the watermark).
**Atom or composite:** Composite: track min(unprocessed_event_time) → advance watermark when all older events are processed → emit completed windows.
**Cost model:** Watermark generation requires knowing the stream's properties (out-of-order bound). Punctuation in stream = explicit watermark.
**Real wall?** Yes — late events (arriving after the watermark) must be handled. The watermark delay vs completeness trade-off is the real design constraint. You can choose low latency (watermark tracks processing time) or high completeness (watermark tracks event time with a large bound).
**Cross-domain wiring:** In distributed systems: vector clocks and causality tracking = watermark-like markers. In hardware: the ready signal = watermark for a computation pipeline.
**Notes:** The watermark defines the "completeness horizon" — events with event_time < watermark are guaranteed complete; events after may still arrive late.

---

## Sketch Primitives

### count-min (cross-domain alias: `frequency-sketch`, `heavy-hitter`, `count-accumulate`)
**Domain:** Database & Streaming / Sketching
**Definition:** Probabilistic frequency counter. d hash functions × w-wide counter array. Update: increment counters at h_i(item). Query: return min_i C[i, h_i(item)]. Overestimates — never underestimates.
**Atom or composite:** Composite: scan(item) → hash × d → fold(min of increments).
**Cost model:** d hash computations + d counter increments per update. d·w space. With w=ε⁻¹, d=log(δ⁻¹), error ≤ ε·N with probability δ.
**Real wall?** Yes — the accuracy/space trade-off is a real wall. Count-Min has a specific accuracy guarantee: estimate ≤ true frequency + ε·N. The space required is O(1/ε · log(1/δ)).
**Cross-domain wiring:** The min operation over hash buckets = taking the conservative estimate = taking the minimum of biased estimators. In signal: taking the minimum of multiple noisy measurements = the conservative estimate in sensor fusion.
**Notes:** Conservative update (only increment when the item is in that bucket) improves accuracy significantly for cash-register streams (non-negative frequencies).

### hyperloglog (cross-domain alias: `cardinality-estimate`, `distinct-count`, `set-size`)
**Domain:** Database & Streaming / Sketching
**Definition:** Estimate the number of distinct elements in a stream using harmonic means of maximum leading zeros in hashed values. Registers store the max run of zeros seen; the harmonic mean of 2^register is the cardinality estimate.
**Atom or composite:** Composite: scan(item) → hash(item) → find leading zeros → fold(max into register) → compute harmonic mean.
**Cost model:** One hash per item + one comparison. Space: O(log log N) registers for N items. Standard: 2^p registers where p=10 (2^10 = 1024 registers).
**Real wall?** Yes — the accuracy is fundamentally limited by the number of registers. The standard error is 1.04/√m for m registers. More accuracy requires more registers (more space) — this is a real wall.
**Cross-domain wiring:** Leading zero count in HLL = measuring the scale of the item set via random hashing = information-theoretic lower bound on distinct counting. In signal: detecting the weakest signal in noise = same kind of max-of-noisy-measurements.
**Notes:** HyperLogLog++ adds linear counting for small cardinalities (below threshold) and a sparse representation. The two-phase approach (linear counting + HLL) removes the HLL bias for small sets.

### bloom-filter (cross-domain alias: `set-membership`, `probabilistic-set`, `fast-membership`)
**Domain:** Database & Streaming / Sketching
**Definition:** Probabilistic set membership test. m bits, k hash functions. Insert: set bits at h_i(item). Query: check all k bits — if any is 0, definitely not present; if all 1, probably present (false positive).
**Atom or composite:** Composite: scan(item) → hash × k → fold(set bit to 1). Query: hash × k → fold(logical AND) → return all_1.
**Cost model:** k hash computations + k bit operations per insert/query. Space: m bits. Optimal k = (m/n)·ln 2.
**Real wall?** Yes — false positives are unavoidable. The probability is p ≈ (1 − e^{−kn/m})^k. The space/accuracy trade-off is a real wall. With n items and m bits, p = (0.6185)^{m/n}.
**Cross-domain wiring:** In networking: CDN cache lookup = bloom filter (probably in cache = check; definitely not = skip). In compilers: register allocation = bloom filter for liveness analysis. In web search: early duplicate detection = bloom filter.
**Notes:** Counting Bloom Filter adds a counter per bit (4 bits) so items can be deleted. Cuckoo Filter uses cuckoo hashing for better space efficiency and deletion support.

### cuckoo-filter (cross-domain alias: `deletable-bloom`, `cuckoo-hash-set`)
**Domain:** Database & Streaming / Sketching
**Definition:** Probabilistic set with support for deletion. Uses cuckoo hashing — each item is stored in one of two candidate buckets. Fingerprints (short hashes) are stored instead of full items.
**Atom or composite:** Composite: insert: compute fingerprints → try bucket 1 → if full, try bucket 2 → if both full, cuckoo-evict and rehash → search = check both buckets.
**Cost model:** O(1) expected insert and lookup with good load factors. Delete requires knowing the fingerprint.
**Real wall?** Yes — load factor limit (typically ~95%). When the filter is full, insert fails or triggers rehashing.
**Cross-domain wiring:** Cuckoo hashing = two-choice hashing = load balancing with two hash functions = same as power-of-two choices in load balancers.
**Notes:** Cuckoo filters achieve better space efficiency than Bloom filters for the same false positive rate and support deletions natively.

### space-filling-curve (cross-domain alias: `z-order`, `hilbert-curve`, `morton-code`)
**Domain:** Database & Streaming / Sketching
**Definition:** Map N-dimensional points to 1D while preserving spatial locality. Z-order (Morton code): interleave the bits of the coordinates. Hilbert curve: a space-filling curve that also preserves locality better than Z-order for some access patterns.
**Atom or composite:** Composite: for Z-order: interleave bits of x and y (and z). For Hilbert: apply Hilbert index algorithm.
**Cost model:** One bit-interleave or Hilbert index computation per point. Very cheap.
**Real wall?** No. But Z-order has poor clustering for diagonal access patterns — Hilbert is better but more expensive to compute.
**Cross-domain wiring:** In databases: Z-order is the basis for Z-order indexing (similar to B-tree for multi-dimensional data). In graphics: Z-order = the order of tiles in a mipmap pyramid. In memory: Z-order determines cache line access patterns.
**Notes:** The key property: points close in N-dimensional space tend to be close in the 1D ordering. This is the locality preservation — it determines how well a 1D index (B-tree, sorted list) serves N-dimensional range queries.

---

## Approximate Query Processing Atoms

### sample-approx (cross-domain alias: `reservoir-sample`, `stream-sample`, `statistical-sample`)
**Domain:** Database & Streaming / Sketching
**Definition:** Maintain a statistically representative sample of N items from a stream of unknown length. Reservoir sampling: keep a reservoir of size k, each new item replaces a random existing item with probability k/i at stream position i.
**Atom or composite:** Composite: at position i: if i ≤ k: add to reservoir; else: j = rand(1,i) → if j ≤ k: replace reservoir[j] with new item.
**Cost model:** O(1) per element. Requires a random number generator.
**Real wall?** No. But the sample mean has variance ~σ²/k — more samples = less variance. This is the fundamental accuracy/space trade-off.
**Cross-domain wiring:** Reservoir sampling = importance sampling without a proposal distribution. In signal: random sampling below Nyquist = random undersampling. In ML: stochastic gradient descent = reservoir sampling over the training set.
**Notes:** The key guarantee: every item in the stream has probability k/N of being in the final reservoir, regardless of stream length.

### biased-sampler (cross-domain alias: `weight-sample`, `stratified-sample`, `priority-sample`)
**Domain:** Database & Streaming / Sketching
**Definition:** Sample with unequal probabilities — rare items are oversampled to ensure sufficient representation. Inverse probability weighting (Horvitz-Thompson) corrects the bias in estimates.
**Atom or composite:** Composite: compute inclusion probability p_i for each item → generate uniform random u → include if u < p_i → weight estimate by 1/p_i.
**Cost model:** O(1) per element plus the computation of p_i.
**Real wall?** No. But variance of the Horvitz-Thompson estimator can be very high for very rare items.
**Cross-domain wiring:** Inverse propensity weighting in causal inference = same correction. Importance sampling uses the same inverse-weighting. In retrieval: diversity sampling = oversample from long-tail documents.

### approximate-join (cross-domain alias: `similarity-join`, `near-join`, `approximate-equi-join`)
**Domain:** Database & Streaming / Sketching
**Definition:** Find pairs of records from two tables that are similar (above threshold) without comparing all pairs. Uses LSH or blocking keys to reduce the candidate space.
**Atom or composite:** Composite: block(items, blocking_key) → within each block: compare candidates → return pairs above threshold.
**Cost model:** O(B · s²) where B is number of blocks and s is average block size. LSH blocking reduces s dramatically.
**Real wall?** Yes — blocking trades recall for speed. Some true matches fall into different blocks and are missed. The recall/throughput trade-off is the real constraint.
**Cross-domain wiring:** Approximate join = similarity search in a database. In retrieval: candidate generation = approximate join of query against corpus. In ML: approximate nearest neighbor search.
**Notes:** The blocking key is a lossy summary — two records can match but have different blocking keys and never be compared. Adaptive blocking (learning the blocking key) is the modern approach.

---

## Change Data Capture Atoms

### change-capture (cross-domain alias: `audit-log`, `delta-stream`, `CDC`)
**Domain:** Database & Streaming / Sketching
**Definition:** Capture changes to a database (INSERT, UPDATE, DELETE) as a stream of events. Methods: log-based (MySQL binlog, PostgreSQL WAL), trigger-based, polling-based.
**Atom or composite:** Composite: intercept write operations → serialize change as (table, key, before_state, after_state, timestamp).
**Cost model:** Log-based is near-zero overhead on the write path (sequential writes). Trigger-based adds overhead per transaction. Polling is expensive.
**Real wall?** Yes — the CDC stream can lag behind the database under heavy write load. The lag is the conserved quantity: high throughput → high lag.
**Cross-domain wiring:** In event sourcing: the event log IS the CDC stream. In signal processing: differencing a signal to get the delta = CDC on a time series.
**Notes:** The CDC stream is the foundation for event-driven architectures and data pipelines — it's the "write-ahead log as data stream" primitive.

### materialize (cross-domain alias: `precompute`, `cache`, `denormalize`)
**Domain:** Database & Streaming / Sketching
**Definition:** Pre-compute and store the result of an expensive query or aggregation so future reads are cheap. Types: materialized views (SQL), pre-computed indexes (key-value), denormalized tables.
**Atom or composite:** Composite: compute(query) → store(result) → on source data change: update(result).
**Cost model:** Write-time cost: recompute on every update. Benefit: read cost is O(1) vs O(query_cost).
**Real wall?** Yes — materialized views must be kept consistent with source data. Under high update rates, the maintenance cost can exceed the benefit. This is the write/read trade-off — materialized views are a cached answer.
**Cross-domain wiring:** In retrieval: pre-computed BM25 scores = materialized view over the corpus. In graphics: pre-baked lightmaps = materialized irradiance.
**Notes:** Incremental view maintenance (IVM) = recompute only the affected portion of the view. This is the key to making materialized views efficient under updates.

---

## Consistency / Indexing Atoms

### consistent-hash (cross-domain alias: `rendezvous-hash`, `virtual-nodes`, `distribution`)
**Domain:** Database & Streaming / Sketching
**Definition:** Distribute keys across N nodes with minimal remapping when nodes are added/removed. Ring hash: hash keys and node IDs onto a circle; each key goes to the nearest clockwise node.
**Atom or composite:** Composite: hash(key) → find position on ring → find next node clockwise.
**Cost model:** O(log N) with a balanced tree (Chord). O(1) with a skip list or hash of the ring positions.
**Real wall?** No.
**Cross-domain wiring:** In caches: consistent hashing minimizes cache invalidation on scale-out. In distributed systems: same as the rendezvous hashing used for load balancing. In retrieval: shard assignment = consistent hash over document IDs.
**Notes:** Virtual nodes (replicating each physical node multiple times on the ring) smooths the load distribution. The number of virtual nodes is the smoothing parameter.

### range-scan (cross-domain alias: `btree-search`, `interval-query`, `index-lookup`)
**Domain:** Database & Streaming / Sketching
**Definition:** Given a sorted index (B-tree, LSM-tree), find all entries in the range [lo, hi]. Start at lo's position, scan forward until past hi.
**Atom or composite:** Composite: locate(lo) via B-tree descent → while key ≤ hi: emit(entry), advance → stop.
**Cost model:** O(log N) to locate start + O(k) for k results. Sequential I/O for the scan — very cache-friendly.
**Real wall?** No.
**Cross-domain wiring:** B-tree traversal = binary search in sorted array = the decision tree of comparisons. In retrieval: range query on a sorted corpus.
**Notes:** LSM-trees (LevelDB, RocksDB) use a multi-level structure: memtable (in-memory) → L0 → L1 → ... → L_max. Each level is 10× larger. Compaction merges and sorts from upper to lower levels.

### lsm-compact (cross-domain alias: `merge-sort`, `level-merge`, `sorted-run-merge`)
**Domain:** Database & Streaming / Sketching
**Definition:** Periodically merge sorted runs from upper levels of an LSM tree into lower levels. The merge is a k-way merge sort — all keys end up sorted in the output.
**Atom or composite:** Composite: take top-of-stack from each level → k-way merge → write merged sorted run to next level.
**Cost model:** Full scan of all participating levels per compaction cycle. This is the "write amplification" cost of LSM trees — one write causes multiple I/Os.
**Real wall?** Yes — write amplification is a real cost. An LSM tree may write 10-50× for each logical write. The amplification factor is the cost of the compaction budget.
**Cross-domain wiring:** K-way merge sort = external sort merge. In retrieval: merging posting lists = k-way merge. In signal: polyphase merge filter = same structure.
**Notes:** The Bloom filter on each LSM level is what makes lookups fast: first check the smallest level (most recent), and skip entire levels whose Bloom filter says the key is not present.

---

## Summary: Database-Streaming Atom → Cross-Domain Wiring

| Database Primitive | Retrieval Alias | Signal Alias | Physics Alias |
|---|---|---|---|
| window | query time filter | gate function | time slice |
| fold-window | running corpus stats | signal integration | temporal averaging |
| watermark | completeness boundary | event horizon | closure condition |
| count-min | heavy-hitter detection | frequency counter | population count |
| hyperloglog | distinct URL count | entropy estimator | cardinality measure |
| bloom-filter | duplicate detection | bandpass filter | probabilistic filter |
| space-filling-curve | Z-order index | interleaved sampling | Hilbert ordering |
| sample-approx | corpus sampling | random sampling | Monte Carlo sampling |
| approximate-join | similarity search | matched filter | particle collision |
| change-capture | delta indexing | differencing | state differential |
| materialize | pre-computed scores | pre-warp | pre-computed solution |
| consistent-hash | shard routing | ring oscillator | ring topology |
| range-scan | posting list traversal | sweep integration | interval search |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## Replication / Consistency Atoms

### replicate (cross-domain alias: `replica-manage`, `leader-follow`, `sync-copy`)
**Domain:** Database & Streaming / Sketching
**Definition:** Maintain N replicas of the data. Read from any replica; write to the leader and propagate to followers. Replica consistency: synchronous (write waits for all N) vs asynchronous (write returns after leader).
**Atom or composite:** Composite: write(leader) → propagate to followers → confirm.
**Cost model:** Synchronous: write latency = latency_to_all_followers. Asynchronous: write latency = latency_to_leader only. Throughput tradeoff.
**Real wall?** Yes — the CAP theorem: under network partition, you must choose between consistency and availability. No replication scheme can provide both simultaneously.
**Cross-domain wiring:** Replication = redundancy in fault tolerance = same as replicated computation in distributed ML.
**Notes:** Single leader replication is the simplest. Multi-leader (active-active) allows writes at multiple sites but requires conflict resolution.

### conflict-resolve (cross-domain alias: `CRDT-merge`, `last-write-wins`, `operational-transform`)
**Domain:** Database & Streaming / Sketching
**Definition:** Handle concurrent writes to the same record. Approaches: last-write-wins (LWW, simple but loses updates), multi-version concurrency control (MVCC, creates multiple versions), CRDT merge (conflict-free by construction).
**Atom or composite:** The resolution policy is the generator. CRDTs are the primitive that requires no coordination for merge.
**Cost model:** LWW: one timestamp comparison. MVCC: create new version + garbage collect old ones. CRDT: merge at read time.
**Real wall?** Yes — no conflict resolution strategy is universally correct. The choice depends on the data model and application semantics.
**Cross-domain wiring:** Conflict resolution = consensus = agreement in distributed systems. In ML: model averaging = CRDT-like merge of distributed model weights.
**Notes:** CRDTs (Conflict-free Replicated Data Types) are the most elegant solution: G-Counter (grow-only counter), PN-Counter (increment/decrement), LWW-Register, OR-Set (add/remove). Each is designed for a specific data type.

### consensus-protocol (cross-domain alias: `Paxos`, `Raft`, `leader-election`)
**Domain:** Database & Streaming / Sketching
**Definition:** Reach agreement on a value across distributed nodes despite failures. Paxos: prepare → promise → accept → learn. Raft: leader election (term) + log replication + safety.
**Atom or composite:** Composite: leader election (majority of votes) → log replication (majority acknowledgment) → commit.
**Cost model:** Two round trips for Raft (append entries): leader → followers (prepare) → majority → leader → followers (commit). Latency = 2 × RTT.
**Real wall?** Yes — consensus requires a majority (N/2+1 for Raft). With 3 nodes, one failure still allows progress. With 2 nodes, one failure stops progress.
**Cross-domain wiring:** Consensus = agreement on a value = consistent hashing requires consensus on the ring state. In distributed ML: gradient averaging requires consensus on the aggregated model.
**Notes:** Raft was designed to be more understandable than Paxos (the original consensus algorithm). The key insight: a leader with a complete log can make decisions alone once elected.

### two-phase-commit (cross-domain alias: `2PC`, `prepare-commit`, `atomic-commit`)
**Domain:** Database & Streaming / Sketching
**Definition:** Atomic commit across distributed participants: Phase 1 (prepare): coordinator asks all participants to vote "yes" to commit. Phase 2 (commit): if all voted yes, coordinator sends commit; else, sends abort.
**Atom or composite:** Composite: coordinator sends prepare → participants vote → coordinator sends commit/abort.
**Cost model:** Two round trips. Blocking: if the coordinator fails after phase 1, participants are blocked until recovery.
**Real wall?** Yes — 2PC is blocking. If the coordinator fails and participants cannot communicate, the transaction is stuck until coordinator recovery. This is why SAGA (compensating transactions) is preferred for long-lived transactions.
**Cross-domain wiring:** 2PC = atomic transaction on distributed state. In ML: model checkpointing = two-phase (write + commit).
**Notes:** Three-phase commit (3PC) removes the blocking problem but is more complex. In practice, 2PC with a reliable coordinator (ZooKeeper) is common.

---

## Storage Engine Atoms

### lsm-tree (cross-domain alias: `log-structured-merge`, `LSMT`, `tiered-storage`)
**Domain:** Database & Streaming / Sketching
**Definition:** Log-Structured Merge-tree: writes go to an in-memory buffer (memtable, typically a skip list). When it fills, it is flushed to disk as an immutable SSTable (sorted string table). Periodically, SSTables are merged (compaction).
**Atom or composite:** Composite: write → memtable → flush (when full) → SSTable → compaction (merge sorted runs).
**Cost model:** Write: O(1) (sequential write to WAL + insert into memtable). Read: O(log N) (bloom filter check + binary search in SSTables).
**Real wall?** Yes — read amplification (many SSTables to check) and write amplification (compaction rewrites data multiple times) are the two real costs of LSM trees.
**Cross-domain wiring:** LSM tree = B-tree + log-structured storage + periodic compaction. In streaming: LSM = the same as the buffered streaming aggregation with periodic flush.
**Notes:** RocksDB, LevelDB, Cassandra (LSM mode) use LSM trees. The key design choice: leveling compaction (每次 level is 10× larger) vs tiered compaction (same-size levels, newest to oldest).

### columnar-store (cross-domain alias: `column-oriented`, `vectorized`, `Apache-Parquet`)
**Domain:** Database & Streaming / Sketching
**Definition:** Store data column-by-column rather than row-by-row. Enables vectorized processing (SIMD on column batches), better compression (similar values are stored contiguously), and projection pushdown (read only needed columns).
**Atom or composite:** Composite: for each column: store values in compressed format → for queries: scan only needed columns.
**Cost model:** Write: more expensive (must write each column separately). Read: much cheaper for analytical queries (OLAP) that select few columns.
**Real wall?** No. But columnar storage is inefficient for point queries (single rows) and updates.
**Cross-domain wiring:** Columnar storage = array-oriented processing = vectorized computation. In ML: columnar = the same as storing model weights by layer rather than by sample.
**Notes:** Apache Parquet (columnar, on-disk) + Apache Arrow (columnar, in-memory) is the standard open-source columnar stack. DuckDB is the query engine optimized for columnar data.

### buffer-pool (cross-domain alias: `page-cache`, `LRU-buffer`, `hot-pages`)
**Domain:** Database & Streaming / Sketching
**Definition:** Keep frequently accessed pages in memory to avoid disk I/O. The buffer pool manager evicts pages using LRU, LRU-K, or clock approximations when full.
**Atom or composite:** Composite: on page request: if in pool → pin → return; if not → evict LRU → load from disk → pin → return.
**Cost model:** LRU is O(1) per eviction. LRU-K (evict based on Kth most recent access) is O(log N). Both must be crash-consistent (write-ahead log).
**Real wall?** Yes — the buffer pool hit rate depends on the working set fitting in memory. If the working set exceeds the buffer pool size, thrashing occurs.
**Cross-domain wiring:** Buffer pool = CPU cache + virtual memory page cache. In ML: gradient accumulation buffer = same idea (hold values until full before processing).
**Notes:** The clock (second-chance) algorithm approximates LRU with O(1) operations per page request — more scalable than LRU for large buffer pools.

---

## Time-Series / Streaming Atoms

### downsampling (cross-domain alias: `LTTB`, `min-max-downsample`, `temporal-reduce`)
**Domain:** Database & Streaming / Sketching
**Definition:** Reduce the number of data points while preserving the visual/analytical shape. LTTB (Largest Triangle Three Buckets): within each time bucket, select the point that forms the largest triangle with the selected points from adjacent buckets.
**Atom or composite:** Composite: segment into buckets → for each bucket: evaluate triangle area criterion → select representative point.
**Cost model:** O(N) single pass over N points. Produces M representative points.
**Real wall?** No. But the downsampling algorithm must preserve the features of the time series — LTTB is superior to simple min/max or average downsampling.
**Cross-domain wiring:** Downsampling = compression in time = decimation in signal processing. In ML: stride in a 1D convolution = temporal downsampling.
**Notes:** The raw aggregation (average, min, max per bucket) loses peaks and valleys. LTTB preserves visual fidelity while reducing the point count.

### anomaly-detect-stream (cross-domain alias: `streaming-SVM`, `ADWIN`, `drift-detect`)
**Domain:** Database & Streaming / Sketching
**Definition:** Detect anomalies in a data stream using sliding windows and adaptive thresholds. ADWIN (ADaptive WINdowing): maintains a window of recent items; if the mean shifts significantly, ADWIN shrinks the window.
**Atom or composite:** Composite: for each new item: update window → if mean shift > threshold → emit anomaly.
**Cost model:** O(1) per item with ADWIN (amortized). Streaming isolation forest and SVM are alternatives.
**Real wall?** Yes — the anomaly definition is domain-specific. The algorithm must adapt to the data distribution without being triggered by normal variations.
**Cross-domain wiring:** Streaming anomaly detection = outlier detection in a time series. In signal processing: change-point detection = anomaly detection in signal statistics.
**Notes:** The CUSUM (cumulative sum) algorithm detects shifts in the mean of a process: it accumulates deviations from the target mean and triggers when the cumulative sum exceeds a threshold.

###TTL-enforce (cross-domain alias: `expire`, `time-to-live`, `ephemeral-data`)
**Domain:** Database & Streaming / Sketching
**Definition:** Automatically delete data after a time-to-live period. Implementation: periodic background job (coarse-grained) or fine-grained per-record expiration.
**Atom or composite:** Composite: on read/write: check TTL → if expired, skip/delete. Periodic: scan and delete expired records.
**Cost model:** Fine-grained: O(1) check per record. Coarse-grained: O(N) periodic scan. Tombstones mark deleted records for replication.
**Real wall?** Yes — the expiration must be propagated to all replicas before the next write. Stale data can survive for up to the replication lag period.
**Cross-domain wiring:** TTL = exponential decay in time = relevance decay in retrieval. In physics: radioactive decay = TTL with exponential distribution.
**Notes:** In systems like Cassandra, TTL creates a tombstone (deletion marker) that is propagated to replicas. The tombstone is deleted during compaction.

---

## Query Processing Atoms

### pushdown-filter (cross-domain alias: `predicate-pushdown`, `early-filter`, `index-accelerate`)
**Domain:** Database & Streaming / Sketching
**Definition:** Apply filters as early as possible in the query plan — at the source, not after transporting data to a compute node. The filter reduces the data volume early.
**Atom or composite:** Composite: parse query → push predicate to scan operator → only qualifying rows are passed up the plan.
**Cost model:** Reduces downstream data movement and computation. The pushdown itself is cheap.
**Real wall?** No. But not all predicates are pushable (e.g., non-deterministic functions, cross-column predicates in columnar stores).
**Cross-domain wiring:** Predicate pushdown = early stopping in retrieval = discard non-candidates early. In signal: pre-filtering before analysis = band-pass filter before FFT.
**Notes:** In Spark, predicate pushdown is a key optimization: filters on partition columns are applied during partition reading, skipping entire files.

### vectorized-exec (cross-domain alias: `SIMD-query`, `batch-execute`, `columnar-process`)
**Domain:** Database & Streaming / Sketching
**Definition:** Process batches of rows (vectors) with SIMD instructions rather than one row at a time. Vectorization enables processing 100s-1000s of rows per CPU instruction.
**Atom or composite:** Composite: fetch batch of column values → process batch with SIMD → output batch.
**Cost model:** The batch processing amortizes the instruction fetch overhead. Typical batch sizes: 1024-4096 rows.
**Real wall?** No. But vectorization requires data in columnar format and cache-friendly memory layout.
**Cross-domain wiring:** Vectorized execution = SIMD processing = the same as GPU tensor operations. In ML: batched inference = vectorized execution.
**Notes:** Modern query engines (DuckDB, ClickHouse, Polars) are fully vectorized. The Volcano iterator model (one row at a time) is the alternative — slower but more composable.

### join-algorithm (cross-domain alias: `hash-join`, `sort-merge-join`, `nested-loop`)
**Domain:** Database & Streaming / Sketching
**Definition:** Join two tables. Hash join: build hash table on smaller table → probe with larger. Sort-merge join: sort both → merge. Nested loop: for each row in outer, scan inner (slow for large tables).
**Atom or composite:** Composite: for each algorithm: build phase + probe phase + output join results.
**Cost model:** Hash join: O(N+M) expected. Sort-merge: O(N log N + M log M + N+M). Nested loop: O(N·M) — avoid for large tables.
**Real wall?** Yes — hash join requires the hash table to fit in memory. Grace hash join spills to disk if the build side is too large.
**Cross-domain wiring:** Join = cartesian product filtered by a predicate. In retrieval: approximate join = LSH-based join without comparing all pairs.
**Notes:** Broadcast join (small table broadcast to all nodes) is the standard in distributed systems when one table fits in memory.

---

## Distributed Systems Atoms

### sharding (cross-domain alias: `partition-data`, `range-shard`, `hash-shard`)
**Domain:** Database & Streaming / Sketching
**Definition:** Split the data across multiple nodes. Range sharding: contiguous key ranges go to each node (good for range queries, bad for hot spots). Hash sharding: key → node via hash (good distribution, bad range queries).
**Atom or composite:** Composite: key → determine shard (range or hash) → route to shard owner.
**Cost model:** Routing overhead: O(1) for hash routing, O(log N) for range routing (if using a distributed B-tree).
**Real wall?** Yes — hot spots (frequently accessed keys) can overload a single shard even with good distribution overall. Adaptive re-sharding is the solution.
**Cross-domain wiring:** Sharding = data partitioning = the same as feature partitioning in distributed ML training.
**Notes:** MongoDB, Elasticsearch, and Cassandra all support sharding. Cassandra's partitioning (vnode) uses consistent hashing with virtual nodes to smooth the load distribution.

### paxos-state (cross-domain alias: `replicated-log`, `state-machine-replicate`, `log-replication`)
**Domain:** Database & Streaming / Sketching
**Definition:** Maintain a replicated state machine via a log of commands. All replicas execute commands in the same order → same state. Paxos ensures the log is consistent despite failures.
**Atom or composite:** Composite: propose command → majority write → learn → execute sequentially.
**Cost model:** Two round trips per log entry. Multi-Paxos (batching) amortizes the cost.
**Real wall?** Yes — the replicated state machine must be deterministic. Non-deterministic operations (e.g., NOW(), UUID()) require special handling (must be logged with the command).
**Cross-domain wiring:** Replicated state machine = fault-tolerant computation = the foundation of distributed consensus.
**Notes:** Raft is equivalent to Paxos but more understandable. The key insight: the log is the consensus primitive — everyone agrees on the same sequence of commands.

### vector-clock (cross-domain alias: `causal-history`, `happens-before`, `causal-consistency`)
**Domain:** Database & Streaming / Sketching
**Definition:** Track causality in a distributed system: each node has a vector clock VC[n]. VC[i] = number of events seen by node i. VC₁ < VC₂ iff VC₁[d] ≤ VC₂[d] for all d and strictly less for at least one.
**Atom or composite:** Composite: on each event: VC[i]++ → on receive: VC[j] = max(VC[j], VC_sender).
**Cost model:** O(N) space and comparison where N = number of nodes. Impractical for large systems — use compact alternatives (dotted version vectors).
**Real wall?** Yes — vector clocks grow linearly with the number of nodes. For 1000+ nodes, the storage and comparison cost is prohibitive.
**Cross-domain wiring:** Vector clocks = partial order tracking = same as the dependency graph in ML training pipelines.
**Notes:** Dynamo, Cassandra, and Riak use vector clocks (or CRDT-based variants) to track causality and resolve conflicts in eventually consistent databases.

---

*Last updated: 2026-06-21 (expanded with replication, storage engines, time-series, query processing, distributed)*
*Source doctrine: The Painted Fence — Jesse*
