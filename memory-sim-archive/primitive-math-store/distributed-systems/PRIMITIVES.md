# Distributed Systems — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Replication

### repl-sync (cross-domain alias: `synchronous-replication`, `eager-replication`)
**Domain:** Distributed Systems
**Definition:** Write to all replicas before acknowledging the client. Guarantees consistency but latency = RTT to all replicas.
**Atom or composite:** Composite: write → wait for ACK from all replicas → respond to client.
**Real wall?** Yes — latency is bounded by the slowest replica. Adding replicas linearly increases write latency.
**Cross-domain wiring:** Synchronous replication = blocking I/O in a distributed setting. In signal: synchronous bus = wait for all responses before proceeding.

### repl-async (cross-domain alias: `asynchronous-replication`, `lazy-replication`)
**Domain:** Distributed Systems
**Definition:** Acknowledge immediately after primary write, replicate in background. Latency = single write, but data may be lost on primary failure.
**Atom or composite:** Composite: write to primary → acknowledge immediately → replicate in background.
**Real wall?** Yes — RPO (recovery point objective) > 0. Some data is always at risk of loss.
**Cross-domain wiring:** Async replication = write-ahead log + async apply. In signal: non-blocking I/O.

### repl-quorum-write (cross-domain alias: `quorum-replication`, `NWR`)
**Domain:** Distributed Systems
**Definition:** Write to W replicas, read from R replicas, where W+R > N (strict quorum) or W+R ≤ N ( sloppy quorum + hints). Cassandra/Dynamo style.
**Atom or composite:** Composite: write to W replicas → read from R replicas → compare versions → resolve conflicts.
**Real wall?** No. But weak quorum increases the chance of inconsistency — the trade-off is latency vs consistency.
**Cross-domain wiring:** Quorum = majority voting = threshold consensus. In control: N-modular redundancy.

### repl-chain (cross-domain alias: `chain-replication`, `replication-chain`)
**Domain:** Distributed Systems
**Definition:** Replicas arranged in a chain. Head accepts writes → forwards to next → tail confirms → head acknowledges. Throughput is limited by the tail but latency is lower than quorum.
**Atom or composite:** Composite: write traverses chain → confirmation propagates back.
**Real wall:** No. But failure of any node in chain requires reconfiguration.

### repl-primary-backup (cross-domain alias: `leader-follower`, `primary-backup-replication`)
**Domain:** Distributed Systems
**Definition:** All writes go to primary; primary replicates to backups synchronously or asynchronously. Reads can go to any replica.
**Atom or composite:** Composite: primary receives write → replicates to backups → confirms to client.
**Real wall:** No.
**Cross-domain wiring:** Primary-backup = master-slave in signal processing. In control: leader-follower consensus.

### repl-multipaxos (cross-domain alias: `multi-Paxos`, `consensus-replication`)
**Domain:** Distributed Systems
**Definition:** Paxos for a sequence of values (log entries). After first round elects a leader, subsequent entries skip prepare phase.
**Atom or composite:** Composite: first round: Paxos prepare/promise → leader elected → subsequent: accept directly → learn.
**Real wall:** No.
**Cross-domain wiring:** Multi-Paxos = optimized consensus for a sequence. In signal: burst mode after initial synchronization.

---

## Consistency Models

### cons-linearizable (cross-domain alias: `linearizability`, `strong-consistency`)
**Domain:** Distributed Systems
**Definition:** Each operation appears to happen atomically at some point between its invocation and response. All nodes agree on the global order of operations.
**Atom or composite:** Composite: write must be acknowledged by quorum before next read can proceed.
**Real wall?** Yes — linearizability requires a full round trip to a majority of nodes. The latency is a real wall.
**Cross-domain wiring:** Linearizability = real-time ordering = causal consistency + global clock. In concurrency: sequential consistency + real-time constraint.
**Notes:** Linearizability + durability (write to disk) = serializability.

### cons-sequential (cross-domain alias: `sequential-consistency`, `SC`)
**Domain:** Distributed Systems
**Definition:** All nodes see operations in the same order, but that order doesn't need to respect real time.
**Atom or composite:** Composite: operations are totally ordered, all nodes agree on the order.
**Real wall:** No.

### cons-causal (cross-domain alias: `causal-consistency`, `CC`)
**Domain:** Distributed Systems
**Definition:** Operations that are causally related must be seen by all nodes in causal order. Concurrent operations may be seen in different orders.
**Atom or composite:** Composite: track causal dependencies → deliver operations only after their causal predecessors.
**Real wall:** No. But requires tracking causal dependencies (vector clocks or version vectors).
**Cross-domain wiring:** Causal consistency = maintaining happens-before ordering = Lamport clock constraints.

### cons-eventual (cross-domain alias: `eventual-consistency`, `EC`)
**Domain:** Distributed Systems
**Definition:** All replicas will eventually converge to the same value if no new writes are made. No guarantees during concurrent writes.
**Atom or composite:** Composite: write to any replica → async propagation → eventually all replicas agree.
**Real wall:** No. But convergence is "eventual" — undefined time bound.
**Cross-domain wiring:** Eventual consistency = damped dynamics = convergence to fixed point. In physics: systems with damping converge to equilibrium.

### cons-read-your-writes (cross-domain alias: `RYW`, `session-guarantee`)
**Domain:** Distributed Systems
**Definition:** A client always sees its own previous writes, even if reads go to a different replica.
**Atom or composite:** Composite: track client's session → route reads to replicas that have applied all writes from this session.
**Real wall:** No. But requires sticky sessions or tracking per-client write history.

### cons-monotonic-reads (cross-domain alias: `MR`, `monotonic-consistency`)
**Domain:** Distributed Systems
**Definition:** If a client reads value V, any subsequent read returns V or a more recent value (never an older one).
**Atom or composite:** Composite: track per-client read versions → only serve reads from replicas >= last-read version.

### cons-bounded-staleness (cross-domain alias: `tunable-consistency`, `staleness-bound`)
**Domain:** Distributed Systems
**Definition:** Reads may return values that are up to K versions or T seconds stale. Probabilistic consistency: Dynamo, Cassandra.
**Atom or composite:** Composite: read → check staleness → if within bound: return; if outside: wait or redirect.
**Real wall:** No.
**Cross-domain wiring:** Bounded staleness = approximate computation with error bounds. In signal: bounded delay tolerance.

---

## Transactions

### txn-2pc (cross-domain alias: `two-phase-commit`, `2PC`)
**Domain:** Distributed Systems
**Definition:** Phase 1 (vote): coordinator asks all participants to prepare. Phase 2 (commit): if all vote yes → commit; else abort. If coordinator fails in phase 2, participants are blocked.
**Atom or composite:** Composite: coordinator → send prepare → collect votes → send commit/abort → participants apply.
**Real wall?** Yes — 2PC is blocking: if coordinator fails after participants vote yes but before sending commit, participants are blocked until recovery.
**Cross-domain wiring:** 2PC = atomic broadcast = all-or-nothing commit. In signal: handshake protocol for reliable transmission.

### txn-saga (cross-domain alias: `saga-pattern`, `choreography`)
**Domain:** Distributed Systems
**Definition:** Distributed transaction as a sequence of local transactions. Each step has a compensating transaction that undoes it if a later step fails. No distributed lock — better performance but no all-or-nothing guarantee.
**Atom or composite:** Composite: execute steps in order → if step fails: execute compensating transactions in reverse order.
**Real wall?** Yes — compensating transactions may not fully undo the original action (e.g., sending an email can't be undone).
**Cross-domain wiring:** Saga = optimistic concurrency control with explicit compensation. In control: graceful degradation.

### txn-mvcc (cross-domain alias: `multiversion-concurrency-control`, `snapshot-isolation`)
**Domain:** Distributed Systems
**Definition:** Each transaction reads from a consistent snapshot (timestamp-based MVCC). Writes create new versions. Conflicts detected by checking for overlapping read/write sets.
**Atom or composite:** Composite: assign timestamp → read from snapshot → collect writes → validate → apply atomically.
**Real wall:** No. But MVCC accumulates versions — needs garbage collection.
**Cross-domain wiring:** MVCC = time-travel queries = branching timeline model. In version control: git's object model = MVCC with content-addressable storage.
**Notes:** Serializable snapshot isolation (SSI) extends MVCC to full serializability with O(N) overhead vs O(N²) for lock-based approaches.

### txn-optimistic (cross-domain alias: `optimistic-concurrency-control`, `OCC`)
**Domain:** Distributed Systems
**Definition:** Proceed without locking; at commit time, validate that no conflicts occurred. If conflict detected → abort and retry.
**Atom or composite:** Composite: read → validate on commit → if conflict: abort and retry.
**Real wall:** No. But abort rate increases with contention — high contention → high abort rate → wasted work.
**Cross-domain wiring:** Optimistic CC = try-and-retry = rejection sampling. In signal: CSMA/CD = carrier sense multiple access with collision detection.
**Notes:** OCC is ideal for low-contention scenarios — avoids locking overhead entirely.

---

## Coordination

### coord-lock (cross-domain alias: `distributed-lock`, `mutual-exclusion`)
**Domain:** Distributed Systems
**Definition:** Acquire a lock before accessing a shared resource. Implemented via: centralized lock server (ZooKeeper), quorum locks (Chubby), or lease-based locks.
**Atom or composite:** Composite: request lock → wait for grant → access resource → release lock.
**Real wall?** Yes — lock server is a single point of failure. Distributed locks require quorum or lease mechanism.
**Cross-domain wiring:** Distributed lock = mutual exclusion in a distributed setting. In control: resource reservation protocol.

### coord-fencing (cross-domain alias: `fencing-token`, `epoch-number`)
**Domain:** Distributed Systems
**Definition:** Every lock acquisition returns a monotonically increasing fencing token. Operations must include the fencing token; servers reject operations with stale tokens.
**Atom or composite:** Composite: acquire lock → get fencing token → include token in all operations → servers reject stale tokens.
**Real wall:** No.
**Cross-domain wiring:** Fencing token = sequence number in reliable protocols. In signal: sequence numbers in ARQ (automatic repeat request).

### coord-election (cross-domain alias: `leader-election`, `bully-algorithm`)
**Domain:** Distributed Systems
**Definition:** Elect a leader from a set of nodes. Bully: highest-ID node wins by sending "election" to higher IDs, declaring itself leader when no one responds.
**Atom or composite:** Composite: send election messages → collect acknowledgments → declare winner.
**Real wall:** No.
**Cross-domain wiring:** Leader election = consensus on a single value (who is leader). In physics: spontaneous symmetry breaking = electing a ground state.
**Notes:** Raft combines leader election with log replication in one protocol — simpler than Paxos for this reason.

### coord-barrier (cross-domain alias: `distributed-barrier`, `synchronization-barrier`)
**Domain:** Distributed Systems
**Definition:** All nodes must reach the barrier before any can proceed. Count-based (N nodes) or time-based (T seconds).
**Atom or composite:** Composite: nodes arrive at barrier → wait → when all present: release all.
**Real wall:** Yes — stragglers (slow nodes) determine barrier release time. One slow node blocks everyone.
**Cross-domain wiring:** Barrier = synchronization point = handshaking. In signal: frame synchronization = wait for all bits before processing.

### coord-consistent-hash (cross-domain alias: `rendezvous-hash`, `virtual-node-hash`)
**Domain:** Distributed Systems
**Definition:** Map keys to nodes with minimal remapping on add/remove. Ring hash: hash(key) and hash(node_id) onto a circle; key belongs to the nearest clockwise node.
**Atom or composite:** Composite: hash(key) → find position on ring → find next node clockwise.
**Real wall:** No.
**Cross-domain wiring:** Consistent hashing = minimal perturbation when the ring changes. In signal: frequency allocation with minimal reassignment. In retrieval: shard routing.

### coord-load-balance (cross-domain alias: `load-balancing`, `LB`)
**Domain:** Distributed Systems
**Definition:** Distribute requests across nodes. Strategies: random, round-robin, least-connections, weighted, IP-hash, least-latency.
**Atom or composite:** Composite: select node according to policy → route request → update load metrics.
**Real wall:** No. But load metrics may be stale — the "least" in "least connections" may not be current.
**Cross-domain wiring:** Load balancing = parallel queuing. In signal: power dividers and combiners. In physics: current distribution across parallel resistors.

---

## Failure Detection

### fd-heartbeat (cross-domain alias: `heartbeating`, `ping`)
**Domain:** Distributed Systems
**Definition:** Nodes periodically send heartbeat messages. If no heartbeat is received within the timeout, the node is considered failed.
**Atom or composite:** Composite: send heartbeat → on timeout: mark failed.
**Real wall:** Yes — network partitions are indistinguishable from node failures with heartbeats alone.
**Cross-domain wiring:** Heartbeat = keepalive signal. In signal: carrier detect. In biology: heartbeat = vital sign.

### fd-phi-accrual (cross-domain alias: `Phi-failure-detector`, `Akka`)
**Domain:** Distributed Systems
**Definition:** Compute a failure suspicion level (phi) based on the distribution of inter-arrival times of heartbeats. phi = −log₁₀(1 − CDF(Δt)). Higher phi → more suspicious.
**Atom or composite:** Composite: track inter-arrival times → compute CDF → compute phi → compare to threshold.
**Real wall:** No. But requires enough samples to estimate the distribution accurately.
**Cross-domain wiring:** Phi accrual = Bayesian failure detection based on observed data. In signal: SNR-based detection threshold.

### fd-chandy-lamport (cross-domain alias: `consistent-snapshot`, `distributed-snapshot`)
**Domain:** Distributed Systems
**Definition:** Chandy-Lamport algorithm for consistent global snapshots without stopping the system. Marker messages propagate through channels; a node records its state when it receives a marker on a channel for the first time.
**Atom or composite:** Composite: initiator sends markers → markers propagate through channels → nodes record state upon receiving first marker.
**Real wall:** No.
**Cross-domain wiring:** Chandy-Lamport = causal ordering of snapshot markers. In signal: trigger signal for simultaneous acquisition.

---

## Consensus

### consensus-paxos (cross-domain alias: `Paxos`, `classic-Paxos`)
**Domain:** Distributed Systems
**Definition:** Phase 1 (prepare): proposer sends prepare(N) → acceptors promise not to accept lower N → respond with highest accepted value. Phase 2 (accept): if majority promised → send accept(N, value) → majority → learn.
**Atom or composite:** Composite: prepare → promise → (if value found, use it) → accept → learn.
**Real wall:** Yes — Paxos requires multiple rounds of communication. Fast Paxos reduces rounds for common cases.
**Cross-domain wiring:** Paxos = consensus on a single value. In game theory: agreement protocol. In physics: synchronized oscillators reaching consensus.
**Notes:** Multi-Paxos optimizes for the common case where the same leader proposes multiple values.

### consensus-raft (cross-domain alias: `Raft`, `leader-based-consensus`)
**Domain:** Distributed Systems
**Definition:** Raft = leader election + log replication + safety. Leader: append entries → replicate to majority → apply. Followers: forward requests to leader. Term numbers enforce uniqueness.
**Atom or composite:** Composite: leader election (term) → log replication → commit check (majority) → apply.
**Real wall:** No. Raft is designed to be understandable — simpler than Paxos.
**Cross-domain wiring:** Raft = leader-follower consensus. In signal: master-slave synchronization. In physics: leader particle in a flock.

### consensus-zab (cross-domain alias: `ZooKeeper-Atomic-Broadcast`, `ZAB`)
**Domain:** Distributed Systems
**Definition:** ZooKeeper's atomic broadcast protocol. Similar to Raft but uses leader-driven broadcast with sequence numbers.
**Atom or composite:** Composite: leader broadcasts transactions → followers receive and apply → leader waits for acknowledgments.

### consensus-flexible (cross-domain alias: `Flexible-Paxos`, `quorum-overlap`)
**Domain:** Distributed Systems
**Definition:** Paxos with disjoint quorum requirements: prepare quorum ∩ accept quorum ≠ ∅, but prepare quorum can be smaller than accept quorum. Reduces coordination overhead.
**Atom or composite:** Composite: prepare with small quorum → accept with large quorum.
**Real wall:** No.
**Cross-domain wiring:** Flexible quorum = trading off coordination for speed. In signal: reduced constellation of control signals.

### consensus-epaxos (cross-domain alias: `Egalitarian-Paxos`, `EPaxos`)
**Domain:** Distributed Systems
**Definition:** Paxos variant where any node can propose (no leader bottleneck). Uses dependency graph to order conflicting commands; non-conflicting commands can be committed in one round.
**Atom or composite:** Composite: any node proposes → detect conflicts → commit if no conflicts or via Fast-Paxos if conflicts.
**Real wall:** Yes — EPaxos's complexity is high; implementation bugs are common.

---

## Distributed Storage

### store-lsm-tree (cross-domain alias: `LSM-tree`, `log-structured-merge`)
**Domain:** Distributed Systems
**Definition:** Write-ahead log → memtable (in-memory) → sorted string table (SSTable) → levels. Compaction merges from upper to lower levels.
**Atom or composite:** Composite: write to WAL + memtable → memtable full → flush to SSTable → compaction merges levels.
**Real wall:** Yes — write amplification (10-50×). One logical write causes multiple physical writes during compaction.
**Cross-domain wiring:** LSM tree = sorted run management = external merge sort. In retrieval: the posting list is stored in LSM format.
**Notes:** Bloom filters on each level are critical: they skip entire levels when the key is absent with high probability.

### store-column-family (cross-domain alias: `wide-column-store`, `Bigtable-model`)
**Domain:** Distributed Systems
**Definition:** Data organized as (row_key, column_family, column, timestamp). Wide rows: many columns per row. Column families are the physical storage unit.
**Atom or composite:** Composite: row_key → column_family → column → versioned values.
**Real wall:** No. But very wide rows can exceed node storage capacity — requires row key design.

### store-crdt-set (cross-domain alias: `CRDT`, `conflict-free-replicated-data-type`)
**Domain:** Distributed Systems
**Definition:** Data structure designed so concurrent updates from any node can be merged deterministically without coordination. G-Set (Grow-only Set): union merge. OR-Set (Observed-Remove Set): add with unique tag, remove by tag.
**Atom or composite:** Composite: concurrent operations → merge using commutative, associative, idempotent merge function.
**Real wall:** No. But CRDTs can only represent a subset of data types — not all data structures have CRDT equivalents.
**Cross-domain wiring:** CRDT merge = lattice merge = join operation in lattice theory. In signal: additive combining of signals (commutative, associative).
**Notes:** CRDT = state-based merge using a join-semilattice. The merge function is: max(state_A, state_B) for monotonically increasing state.

### store-vector-clock (cross-domain alias: `version-vector`, `VC`)
**Domain:** Distributed Systems
**Definition:** Each node maintains a vector of counters: VC[node_i] = number of updates known from node_i. Causality: A → B iff VC_A[i] ≤ VC_B[i] for all i and VC_A[j] < VC_B[j] for some j.
**Atom or composite:** Composite: on update: increment own counter → on sync: max per component.
**Real wall:** Yes — vector clock size grows with the number of nodes. O(N) storage per node.
**Cross-domain wiring:** Version vectors = Lamport timestamps with per-node counters. In physics: coordinate system for spacetime events.

---

## Recovery & Durability

### recovery-wal (cross-domain alias: `write-ahead-log`, `WAL`, `journal`)
**Domain:** Distributed Systems
**Definition:** Before modifying data, write the change to a persistent log. On crash recovery, replay the WAL to restore consistent state.
**Atom or composite:** Composite: write to WAL → update data → mark WAL entry as applied.
**Real wall:** Yes — WAL write latency is the lower bound on transaction latency (must be durable before proceeding).
**Cross-domain wiring:** WAL = journaling filesystem. In signal: memory-mapped I/O with write barriers.

### recovery-checkpoint (cross-domain alias: `checkpointing`, `snapshot`)
**Domain:** Distributed Systems
**Definition:** Periodically save the full system state to persistent storage. On failure, restore from checkpoint + replay WAL since checkpoint.
**Atom or composite:** Composite: global barrier → freeze state → write to persistent storage → resume.
**Real wall:** Yes — large state takes time to checkpoint; checkpoint frequency trades off recovery time vs overhead.
**Cross-domain wiring:** Checkpoint = periodic full backup. In signal: frame buffer snapshot.

### recovery-shadow-paging (cross-domain alias: `shadow-paging`, `copy-on-write-paging`)
**Domain:** Distributed Systems
**Definition:** Write to a shadow page; on commit, atomically update the parent pointer to point to the shadow page. Old page is never overwritten until new page is committed.
**Atom or composite:** Composite: write to shadow page → on commit: flip parent pointer.
**Real wall:** No. But two-level indirection and periodic garbage collection of old pages.

---

## Network & Routing

### net-gossip (cross-domain alias: `gossip-protocol`, `epidemic-broadcast`)
**Domain:** Distributed Systems
**Definition:** Nodes periodically share state with random peers. Information spreads exponentially (epidemic) — O(log N) rounds to reach all nodes.
**Atom or composite:** Composite: select random peer → send state → merge received state with local.
**Real wall:** No. But convergence time is probabilistic; eventual consistency, not guaranteed.
**Cross-domain wiring:** Gossip = infection model = Susceptible-Infected-Susceptible (SIS) epidemiology. In physics: diffusion on a random graph.

### net-circuit-switch (cross-domain alias: `circuit-relay`, `Tor-circuit`)
**Domain:** Distributed Systems
**Definition:** Establish a dedicated path (circuit) before communication. All traffic flows through the same nodes. Setup cost is high; per-packet cost is low.
**Atom or composite:** Composite: negotiate path → reserve resources at each node → transmit.
**Real wall:** Yes — circuit setup delay + resource reservation. Tor circuits have 3 hops minimum for anonymity.

### net-packet-switch (cross-domain alias: `packet-routing`, `IP-forwarding`)
**Domain:** Distributed Systems
**Definition:** Each packet is independently routed based on destination address. No setup cost; each packet makes routing decisions independently.
**Atom or composite:** Composite: look up route → forward packet. Route may change between packets.
**Real wall:** No.
**Cross-domain wiring:** Packet switching = statistical multiplexing = burst-mode communication. In signal: time-division multiplexing.

### net-sdn-controller (cross-domain alias: `SDN`, `OpenFlow`, `network-controller`)
**Domain:** Distributed Systems
**Definition:** Centralized network controller programs the forwarding tables on switches. Decisions made centrally; applied globally.
**Atom or composite:** Composite: controller receives network events → computes routing → programs switches.
**Real wall:** Yes — controller is a single point of failure and a potential bottleneck.
**Cross-domain wiring:** SDN = software-defined routing = programmable network. In signal: software-defined radio.

---

## Distributed Computing

### dist-map-reduce (cross-domain alias: `MapReduce`, ` embarrassingly-parallel`)
**Domain:** Distributed Systems
**Definition:** Map: (k1,v1) → list[(k2,v2)]. Shuffle: group by k2. Reduce: (k2, list[v2]) → list[(k3,v3)].
**Atom or composite:** Composite: map over data → shuffle to reducers → reduce.
**Real wall:** No. But shuffle is the expensive part — requires sorting and network transfer.
**Cross-domain wiring:** MapReduce = fork-join parallelism + group-by. In linear algebra: map = independent computation, reduce = aggregation across grouped results.

### dist-dataflow (cross-domain alias: `Apache-Flink`, `dataflow-engine`, `streaming-dataflow`)
**Domain:** Distributed Systems
**Definition:** Computation as a directed acyclic graph (DAG) of operators. Data flows along edges. Exactly-once semantics via checkpointing.
**Atom or composite:** Composite: build DAG → stream data → checkpoint state → recover from failures.
**Real wall:** No. But DAG topology determines the parallelism and latency.
**Cross-domain wiring:** Dataflow = pipeline processing = signal flow graph. In signal: DSP pipeline.

### dist-shuffle (cross-domain alias: `shuffle-phase`, `data-shuffle`)
**Domain:** Distributed Systems
**Definition:** Redistribute data across nodes based on a key. Each node sends records with key K to the node responsible for K.
**Atom or composite:** Composite: compute partition for each record → send to partition owner → receive and merge.
**Real wall:** Yes — shuffle is the network bottleneck in distributed computing. It requires O(total_data) network transfer.
**Cross-domain wiring:** Shuffle = network-heavy operation. In signal: inter-stage communication.

### dist-partition-prune (cross-domain alias: `partition-pruning`, `predicate-pushdown`)
**Domain:** Distributed Systems
**Definition:** Push predicates down to partitions; only scan partitions that can satisfy the predicate. Eliminates unnecessary data reads.
**Atom or composite:** Composite: analyze predicate → identify relevant partitions → scan only those.
**Real wall:** No.
**Cross-domain wiring:** Partition pruning = index seek. In signal: bandpass filtering = frequency-domain pruning.

### dist-federated (cross-domain alias: `federated-learning`, `FL`)
**Domain:** Distributed Systems
**Definition:** Train models on distributed data without centralizing data. Clients compute local gradients → server aggregates (FedAvg: weighted average) → update global model.
**Atom or composite:** Composite: server broadcasts model → clients train locally → clients send gradients → server aggregates.
**Real wall?** Yes — communication cost, privacy leakage (gradient inversion attacks), and non-IID data distributions are endemic problems.
**Cross-domain wiring:** Federated learning = distributed optimization with privacy constraints. In signal: distributed estimation with quantized updates.
**Notes:** Differential privacy (adding noise to gradients) is the main mitigation for privacy leakage.

---

## Summary: Distributed Systems Atom → Cross-Domain Wiring

| Distributed Primitive | Consensus Alias | Signal Alias | DB Alias |
|---|---|---|---|
| repl-quorum-write | majority voting | N-modular redundancy | multi-version commit |
| cons-linearizability | real-time ordering | synchronous bus | serializable |
| cons-causal | Lamport ordering | causal signal | causal DB |
| txn-2PC | atomic commit | handshake | distributed transaction |
| txn-saga | compensating-transaction | graceful degradation | workflow orchestration |
| coord-election | leader election | spontaneous symmetry breaking | coordinator election |
| coord-barrier | synchronization point | handshaking | global barrier |
| coord-consistent-hash | ring partitioning | frequency allocation | shard routing |
| fd-chandy-lamport | global snapshot | simultaneous trigger | consistent backup |
| consensus-paxos | distributed consensus | agreement protocol | distributed log |
| store-crdt-set | conflict-free merge | lattice merge | eventual-consistent store |
| store-vector-clock | causal ordering | spacetime coordinates | version tracking |
| recovery-wal | journaling | write barrier | persistent log |
| net-gossip | epidemic broadcast | SIS model | rumor spreading |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*
