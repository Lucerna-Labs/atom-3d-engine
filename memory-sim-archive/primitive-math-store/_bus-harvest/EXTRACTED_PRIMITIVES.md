# Spiderweb Bus — Extracted Primitive Reserve (cross-domain harvest)

> Primitives harvested FOR the spiderweb-bus trilogy (kernel/lane-bus · P2P transport · SCG · spider orchestrator + the vibration/fabric layer).
> Root atoms: **scan · hash · fold · project · scale · compare · combine · order** — see `../_taxonomy-root/ROOT_ATOMS.md`.
> Produced 2026-06-21 by a 3-pass sweep. Non-destructive staging: each entry names its **native domain** so it can later be grafted into the canonical per-domain `PRIMITIVES.md`. Proposed NEW domains are flagged for blessing before files are scattered into the taxonomy.

## How this was produced
- **Harvest pass** — surveyed all 13 catalogued domains for cross-domain transfers into the trilogy (131 harvests → synthesized).
- **Gold-mine pass** — citation-grounded research mining (HF paper-search + WebSearch), biased to the 2021+ frontier (44 finds → assayed). Citations below are agent-sourced; verify before depending.
- **Discovery pass** — textbook localization of where each named primitive lives. *Pending — to be appended.*

The strongest signal: the harvest (mechanism) and the gold-mine (frontier metric) **independently converged on the same slots** (A1–A5 below). Two blind passes landing on identical slots is evidence these are the *right* primitives, not merely *some*.

---

## A. Convergence slots (existing-kit mechanism × frontier research)

### A1 — Vibration / attenuation kernel
- **Mechanism (harvest):** graph-Laplacian heat diffusion, one dumb per-tick local fold: `p += dt·(D·Σ_neighbors(p_j − p_i) − λ·p)`. Six domains (forward-Euler, DDPM decay, T2-decoherence, damped eventual-consistency, heat-equation label-propagation, matrix-exp) collapse onto this. The `−λp` term *is* attenuation-with-distance, physically correct (conserves pressure mass, reaches a fixed point) vs the ad-hoc `1/(1+d)`. Atoms: fold + project + scale. Kernel owns the add-multiply; spider owns `D`, `λ`.
- **Metric (frontier):** the *right* decay distance on a graph is **effective resistance** R(i,j), not hop count — a true metric (triangle inequality, web-wide consistent), and **R drops when parallel paths are added** (resistors in parallel), which makes desire-path/emergent-thread reinforcement rigorous. Cite: *On Over-Squashing in MPNNs* (Di Giovanni et al., ICML 2023) + *Understanding Oversquashing through Effective Resistance* (Black et al., ICML 2023) — https://arxiv.org/abs/2302.02941
- **Swappable generators:** heat-kernel / Personalized-PageRank diffusion (GDC, https://arxiv.org/abs/1911.05485) = cheap closed form; TIDE (https://arxiv.org/abs/2212.02483) = learnable per-vibration-type upgrade once online-learning is wired.
- **Native domain:** physics-diffusion (mechanism) + a graph-metric slot in linear-algebra-matrix.
- **Real walls:** forward-Euler CFL (`D·dt < ~1/max_degree` or it amplifies); percolation threshold λc (can't get whole-web reach *and* local containment — pick a side); exact L⁺ is dense/O(n³) → use resistance sketches (JL) / random-walk commute-time on a live mutating bus.
- **Unblocks:** principled vibrations; emergent-thread reinforcement; a concrete spider rewiring/GC target (add the max-resistance edge). Composes with 3D node positions.

### A2 — P2P transport reliability
- **Mechanism (harvest):** Random Linear Network Coding (RLNC over GF(2⁸)) at intersection nodes — forward random linear combinations, each sink solves a small system. The butterfly result: a contended shared edge carries MORE than any store-and-forward routing. "Routing can't beat the cut" is the **painted** wall; the **min-cut/max-flow capacity** is the real one it then sits against. Native domain: information-theory-coding.
- **Frontier:** **BS-AC-RLNC ("Blank Space") adaptive-causal sliding-window coding** — every coded packet is fresh degrees-of-freedom for the whole unacked window (heals any single loss, no wait-for-specific-retransmit, **no head-of-line blocking**); two suspension rules silence redundancy the network can't drain. ~2× throughput / ~3× lower in-order delay vs Selective-Repeat ARQ. Cite: https://arxiv.org/abs/2502.11984 (2025). Pair **fountain/LT rateless** coding on feedback-free one-way legs.
- **Native domain:** information-theory-coding → **PROPOSED new sub-domain `reliability-coding`**.
- **Real walls:** min-cut/max-flow (coding reaches, can't exceed); erasure-channel overhead (collect (1+δ)k droplets, δ>0 irreducible); BS-AC needs a feedback channel + bounded-window CPU.
- **Unblocks:** lossy bus-to-bus-over-network. Rising erasure / growing window = a native backpressure vibration; Blank-Space suspension = an explicit attenuation event the spider propagates.

### A3 — Spider online-learning + cost-aware routing
- **Mechanism (harvest):** φ-accrual failure detector (continuous suspicion scalar, self-calibrates per-lane cadence) + leverage-score GC (protect the rare-but-load-bearing lane naive LRU would cull) + FTRL-L1 (the L1 regularizer drives dead lanes to exactly 0 — the regularizer *is* the garbage collector, no-regret).
- **Frontier (the parent primitive):** **Dual mirror descent for online allocation** — one per-resource multiplier μ_r = a per-lane **shadow price**; pick action maximizing `reward − μ·consumption` (greedy 1-D scan, no solver), update μ by mirror descent. Best-of-many-worlds (optimal under i.i.d./non-stationary AND adversarial) with **no regime detector**; recovers EXP3 / dual-subgradient as special cases. Cite: Balseiro/Lu/Mirrokni, https://arxiv.org/abs/2011.10124 (2021).
- **Frontier (with a certificate):** **Drift-plus-penalty constrained-OCO** — O(√T) regret AND O~(√T) cumulative violation, one simplex projection/round, no Slater; the virtual queue Q_t *is* a vibration signal (rises when a conduit is oversubscribed). Cite: Sinha & Vaze, https://arxiv.org/abs/2310.18955 (2024).
- **Frontier (cheapest rigor):** **Bias-feedback / auxiliary-loss-free load balancing** — `b_i += u·sign(mean_occupancy − c_i)` added only to selection scores, never the value path; provably log-regret primal-dual. Cite: DeepSeek, https://arxiv.org/abs/2408.15664 (2024).
- **Native domain:** control-numerical-opt + ml-training → **PROPOSED new slot `online-allocation`**.
- **Real walls:** O(√T) is the adversarial floor (converges at 1/√t, not instantly); guarantees are vs the best FIXED mix; leverage/concentration are sampled from the PAST row-space (a future regime change is unobservable); φ-accrual can't distinguish partition from death (FLP).
- **Unblocks:** cost-aware routing across the whole operating envelope + weight-morphing with regret/violation certificates; "attenuation under backpressure" becomes the formal Φ'(Q_t) term.

### A4 — Stability governor
- **Mechanism (harvest):** the dominant eigenpair via **power-iteration = iterated fan-out (the kernel's native op)**, so it amortizes into normal traffic. Dominant eigenvector = the emergent **threads** to promote; dominant eigenvalue **ρmax** = the stability scalar. Keep `β·ρmax < 1` for backpressure (a local spike can't avalanche) but `β·ρmax > 1` for gossip/control (self-advertisement provably reaches the whole fabric) — **one inequality, opposite sign per traffic class**. Plus a Nyquist/loop-gain phase-margin governor (constrain the product of gains around any lane cycle ≤ 1) so closed feedback loops can't self-oscillate.
- **Native domain:** linear-algebra-matrix + control-numerical-opt.
- **Real walls:** thread-revelation speed bounded by the spectral gap |λ₂/λ₁|; ρmax only movable by changing attenuation γ or topology; power-iteration is O(edges) → refresh periodically, never per-message; gain-vs-bandwidth trade (must spend responsiveness to buy margin).

### A5 — Type identity + wire format (the shippable floor)
- **Frontier (adopt now):** **postcard-schema** — derives a structural reflection (DataModelType) at COMPILE TIME, folds schema+path into an 8-byte FNV1a `const` **Key**; zero runtime cost, no allocator, no std; the digest is over the *structural schema* (not the Rust type name) so it's stable across binaries and the P2P boundary, and CI-snapshottable to fail the build the instant a wire type changes. Cite: https://docs.rs/postcard-schema (postcard-rpc, 2024).
- **Frontier (NIC on-ramp):** **Cornflakes** per-field copy-vs-zero-copy threshold — zero-copy is NOT always a win (NIC scatter-gather has per-segment overhead); treat "NIC-gather vs CPU-copy this byte range" as a per-field scheduling decision via a cheap size threshold (~512B, NIC-specific). Cite: SOSP 2023 (Distinguished Artifact), https://sing.stanford.edu/site/assets/publications/cornflakes-sosp23.pdf
- **Mechanism (harvest):** **fencing tokens** (monotonic epoch, compare-and-drop) so a stale in-flight message/command from before a restart can't corrupt state — doubles as the cross-binary versioning guard; **Zadoff-Chu preamble** (RF) whose root index encodes the type tag → frame-sync + stable type-demux in one matched-correlate, resyncs cleanly after a corrupt read.
- **Native domain:** **PROPOSED new domain `wire-format-serialization` / `type-identity`** (+ fencing tokens → distributed-systems; ZC → signal-processing-rf).
- **Real walls:** postcard is not self-describing (both ends must share the schema out-of-band; unknown tag = undecodable, not graceful); tag-bit collision budget; Cornflakes requires the source buffer pinned/alive until DMA completes (real ownership the bus must enforce) and the crossover is hardware-coupled.
- **Unblocks:** in-process → cross-process/P2P; self-advertising auto-connect on MCU-class peers; CI wire-compat build gate. **This is the highest leverage-to-risk item on the board.**

---

## B. Other keepers (by component)

**kernel-lanes**
- **CSR sparse-matvec as one shared structure** for the router AND the SCG index — a publish is one sparse matvec; `row_ptr` is the on-ramp→off-ramp adjacency; the identical CSR powers BM25 (values=tf-idf). (linear-algebra-matrix; painted wall.)
- **CFL condition as the max-credit-per-tick formula** — a lane may not drain/inject more than a fixed fraction of ring capacity per hop or the backpressure-diffusion field diverges; principled policy-free credit bound. (physics-diffusion; REAL wall.)
- **RCM (reverse Cuthill-McKee) cache-local ring layout** — permute the lane graph to cluster communicating nodes into contiguous memory; fan-out writes a tight span. (linear-algebra-matrix; painted.)
- **Clock / second-chance eviction** — O(1) LRU approximation, alloc-free (one ref-bit + sweeping hand); the ref-bit doubles as liveness the spider reads for free. (database-streaming-sketching; REAL: working-set wall.)
- **PACKS — PIFO scheduler approximated from K dumb strict-priority queues** driven by one scalar `rank()`: the textbook realization of "mechanism in kernel, policy in spider" (swap `rank()`, swap the whole discipline). Cite: NSDI 2025, https://arxiv.org/abs/2308.00797
- **BFC — per-flow, per-hop credit backpressure** — pause only the congesting flow at the immediate upstream hop; kills head-of-line blocking on shared lanes, queues stay near-empty. Cite: NSDI 2022, https://www.usenix.org/conference/nsdi22/presentation/goyal

**vibrations-fabric**
- **Reaction-diffusion Turing instability** — slow short-range "success" activator vs fast long-range "cost" inhibitor; cross the D_B/D_A threshold and uniform "all lanes equal" destabilizes into stable high-traffic stripes = threads with an intrinsic wavelength. The only harvest giving thread FORMATION a stability *criterion*, not a heuristic. (physics-diffusion; REAL: the diffusion-rate gap is the conserved resource.)
- **BLS-style associative path aggregation** — fold per-node congestion signals (sum/max generator) into one fixed-size aggregate vibration → O(depth) not O(nodes) backpressure traffic. (cryptography-hashing, the algebra not the crypto; REAL: aggregation is lossy.)
- **Hi-Z early-reject admission** — a pyramid of cost watermarks; reject an over-budget message with one compare at the on-ramp before any dispatch (O(log lanes)). (graphics-rendering-lod; painted.)

**transport-p2p**
- **SIR/gossip epidemic dissemination gated by the same ρmax** — random-peer digest exchange (live lane tags, 3D positions, load/φ); O(log N), self-healing, no SPOF; the merged state IS the vibration field + self-advertisement. (distributed-systems + biology; REAL: epidemic threshold.)
- **Kabsch/ICP rigid 3D-frame reconciliation** — when two buses peer (or one restarts), SVD-align their coordinate frames so distances/attenuation/diffusion stay coherent across the merged fabric; residual RMSD = a drift alarm. (biology-bioinformatics; painted for the exact solve.)

**scg**
- **Gram-Schmidt rejection = MMR diversity made literal** — keep an orthonormal basis Q of admitted chunks; rank candidates by `BM25 · ‖c⊥‖` (new information not already spanned); stops the budget paying for paraphrases; age-decay Q = staleness suppression. (linear-algebra-matrix; painted, cap basis with truncated SVD.)
- **BLAST seed-and-extend = WAND cascade = magic-state distillation** — one unified cheap-first/escalate-residual: seed via inverted index → expensive BM25 only on candidates → WAND running-threshold early-termination. Three distant domains, identical pipeline. (REAL: seeds miss sub-threshold matches.)
- **Personalized PageRank / random-walk-with-restart** — structurally-aware relevance BM25 misses; α = the locality dial mapping onto token-budget tension; same power-iteration the spider already runs; no_std-friendly. (physics-diffusion + retrieval.)
- **Seismic geometric inverted index** — geometrically-cohesive blocks each with a component-wise-max summary (provable upper bound) → skip whole blocks; 1–2 orders lower latency, sub-ms, model-free in the hot path. Cite: SIGIR 2024 Best Paper, https://arxiv.org/abs/2404.18812
- **Ada-KV adaptive budget split** — allocate a fixed retention budget non-uniformly, proportional to each consumer's score concentration, with an L1 output-error bound. Cite: https://arxiv.org/abs/2407.11550
- **LSM-tree + per-run Bloom skip** — append to an alloc memtable, flush to immutable sorted runs, probe a per-run Bloom to skip whole runs on term-absence. (distributed-systems; REAL: write amplification.)

**spider-orchestrator**
- **Langevin thermostat as the lane-weight control law** — `m·a = F − γ·v + √(2γkT)·R(t)`; the −γ·v term kills load-balancer flap, the √(2γkT)·R(t) noise de-synchronizes lanes (kills the TCP global-sync pathology) and escapes routing minima; T = congestion is the one explore/exploit dial. Subsumes SA/Boltzmann/softmax-T/UCB/Grover under one law with the damping term they lack. (biology; painted.)

---

## C. Proposed new kit domains (awaiting blessing)
- **`wire-format-serialization` / `type-identity`** — postcard-schema Key, Cornflakes threshold, fencing tokens, ZC preamble. (Or fold into a `networking-transport` domain.)
- **`reliability-coding`** — RLNC, BS-AC-RLNC Blank Space, fountain/LT. (Or a sub-section of information-theory-coding.)
- **`online-allocation`** — dual mirror descent, drift-plus-penalty OCO, bias-feedback, FTRL-L1, leverage scores. (Or fold into control-numerical-opt.)
- Networking/scheduling primitives (PACKS, BFC, φ-accrual) — a `networking-systems` domain, or scatter to existing.

## D. Discovery pass — canonical lineage (battle-tested, sourced)
The third pass returned the *named, canonical* home of each wall's primitive (the "where it lives" with academic provenance). Several reinforce A1–A5; two fix **live issues in the actual codebase**.

| Primitive | Closes | Canonical source |
|---|---|---|
| **Graph Heat Kernel** `H_t = exp(−t·L)` | vibration propagation across the graph (replaces `1/(1+d)`, which ignores the graph) | Kondor & Lafferty, *Diffusion Kernels on Graphs*, ICML 2002; Chung, *Spectral Graph Theory* 1997 |
| **Drift-Plus-Penalty backpressure** (max-weight / differential-backlog) | provably-stable cross-layer backpressure + cost-aware routing in one controller | Tassiulas & Ephremides, IEEE TAC 1992; Neely, *Stochastic Network Optimization* 2010 |
| **Pheromone evaporation + reinforcement** (stigmergic trails) | bounded desire-path formation (replaces O(dᵏ) thread fan-out + FIFO `THREAD_CAP`) | Dorigo & Di Caro, *AntNet*, JAIR 1998 |
| **Bandits with Knapsacks** (PD-BwK / LinCBwK) | cost-aware admit/shed/route under a hard $/latency/token budget | Badanidiyuru, Kleinberg, Slivkins, FOCS 2013 / JACM 2018; Agrawal & Devanur, NeurIPS 2016 |
| **Exp3 / Exp3.S** (adversarial bandit) | online lane-weight learning, regret `O(√(TK ln K))`, non-stationary-safe | Auer, Cesa-Bianchi, Freund, Schapire, SICOMP 2002 |
| **Rateless erasure / RaptorQ + sliding-window RLNC** | lossy P2P reliability, no head-of-line blocking | RFC 6330 (RaptorQ); RFC 9265 (FEC in transport); Luby LT 2002 |
| **Interval Tree Clock + Merkle-DAG** | churn-safe causal order + cross-machine thread provenance (vector clocks break under self-register/leave) | Almeida, Baquero, Fonte, OPODIS 2008; Merkle-CRDTs, arXiv:2004.00107 |
| **Credit-based flow control** (consumer credits / `request(n)`) | lossless self-clocked rate matching; hard-gate inner loop under drift-plus-penalty | Kung & Morris, IEEE Network 1995; Reactive Streams |
| **Balanced multilevel graph partitioning** (METIS + Fiduccia-Mattheyses; Fennel streaming) | distributed lane/peer placement (min-cut, keep buffer-sharing chains co-located) | Karypis & Kumar, SIAM JSC 1998; Tsourakakis, WSDM 2014 |
| **Aho-Corasick DPI/LPI** | model-free deterministic intent-from-bytes (in-process = painted wall: the typed envelope tag is the cheapest classifier) | Aho & Corasick, CACM 1975; nDPI/libprotoident |
| **Space-Saving / Stream-Summary** (+ t-digest) | bounded-memory `webctl top` heavy-hitters/quantiles | Metwally, Agrawal, El Abbadi, ICDT 2005 |
| **Schema Fingerprint** (CRC-64-AVRO Rabin over canonical schema) | stable cross-process type tag (companion to postcard-schema) | Avro 1.11; Rabin 1981 |

## E. THREE-PASS CONVERGENCE (the strongest signal)
Independent passes landing on the **same slot** is the evidence we found the *right* primitive:
- **Vibration decay/propagation** — harvest: graph-Laplacian *diffusion* · gold-mine: *effective resistance* (L⁺) · discovery: *heat kernel* exp(−t·L). These are the same spectral-graph object (dynamics / metric / operator). **Three for three.**
- **Backpressure · cost-routing · weight-learning** — harvest: φ-accrual + FTRL-L1 + spectral ρmax · gold-mine: dual-mirror-descent + drift-plus-penalty-OCO + bias-feedback · discovery: Drift-Plus-Penalty (Tassiulas) + Bandits-with-Knapsacks + Exp3. One online-allocation/control family.
- **P2P reliability** — harvest: RLNC butterfly · gold-mine: BS-AC-RLNC "Blank Space" · discovery: RaptorQ / sliding-window RLNC. Same coding-theory primitive (textbook → frontier).
- **Type identity** — gold-mine: postcard-schema const Key · discovery: CRC-64-AVRO Rabin. Same stable-wire-tag.
- **Thread formation** — harvest: Turing instability (the *why*) · discovery: pheromone/AntNet (the bounded *how*). Complementary.

## F. Consolidated build order (all three passes)

**Buildable NOW in the in-process bus (the Rust workspace, no new deps, doctrine-clean drop-ins):**
1. **Graph Heat Kernel as the `attenuation()` generator** — the kit already exposes `attenuation()` as a swappable generator slot (`lib.rs:250-251`); this is a *generator swap*, not new architecture. Fixes the concrete `1/(1+d)`-ignores-the-graph lie: vibrations follow real strand topology, multi-path recombination, one principled knob `t` that also retires the hard radius cutoff. **Three-pass validated, lowest risk, highest fidelity. Start here.**
2. **Pheromone trails replace per-path thread fan-out + `THREAD_CAP`** — fixes a *live* scaling bug: today threads mint O(dᵏ) paths, bounded only by a blunt age-FIFO `THREAD_CAP=1024` that evicts load-bearing threads arbitrarily. Pheromone (deposit-on-tagged-arrival, evaporate-per-tick, prune-by-significance) bounds by *significance* with near-zero kernel change (the kernel already records every tagged arrival).
3. **Drift-Plus-Penalty backpressure** — converts the hand-tuned `Vibration::Backpressure/Starved` counters into a stability-PROVED controller AND delivers cost-aware routing via the `V·penalty` term (two walls, one primitive). Pure kernel mechanism (`Q_a−Q_b`, serve max); spider owns `V` + penalty. Pair with **credit-based flow control** as the lossless hard-gate inner loop.

**v2 — gated on edge transport (cross-process/P2P; honest only once distance = real latency):**
4. **Type tag + wire format** — postcard-schema const Key (or CRC-64-AVRO Rabin) → the cross-process routing key (TypeId is build-unstable).
5. **Bandits-with-Knapsacks** cost-aware admission on API lanes (Exp3 as the no-budget fallback).
6. **RaptorQ / sliding-window RLNC** reliability + **Interval Tree Clock** causal order + **METIS** peer placement + **Space-Saving** for `webctl top`.

## G. New-domain harvest (5 domains added 2026-06-21, deduped)

Five domains were added to the kit after the first sweep — `networking`, `agentic-reasoning`, `cryptography-advanced`, `formal-verification`, `statistics-probability` — and harvested with the §A–§F set passed in as a do-not-repeat list. 31 genuinely-new finds. They open **five capability AXES the prior 13 domains structurally lacked** (the prior set *computes* quantities; these *verify, reason, estimate, authenticate, and rate-control*):

**Axis 1 — Prove the bus invariants** (machine-checked, not commented). `formal-verification` + `cryptography-advanced`:
- **TLA+/PlusCal + TLC** model-check the exactly-once deliver/subscribe protocol (the English comment block ~`lib.rs:1324`) on a bounded instance — brute-forces every interleaving `burst_at_startup` only samples.
- **Inductive invariants + k-induction** as per-transition runtime monitors: (I1) per-TypeId conservation with cap-eviction as an *accounted* transition; (I2) every live ThreadId ∈ `fabric.threads` (today pinned by one test); (I3) `evict_cursor ≤ next_thread`. Maps to `lib.rs:460-515`.
- **Partial-order reduction** — makes TLC tractable AND supplies the conservative `commutes(TypeId, lane)` predicate that *licenses* the highway's informal "unordered like fan-out."
- **Assume-guarantee contracts** on `Strand::inputs()/outputs()` — scales verification past state-explosion and becomes the discharge condition for self-advertising auto-connect.
- **STARK/FRI + recursive folding** over the kernel trace — a succinct hash-only runtime certificate that credit-conservation + exactly-once held on the *real* trace (conservation = an arithmetic constraint = the cryptographic form of conserved-cost honesty); FRI needs only a CRH, fitting the std-only/forbid-unsafe/no-deps kernel. Real wall: prover O(N log N) → prove sampled checkpoints, not every packet.
- **Refinement-typed phantom-typestate** — lifts the postcard-Key check to *compile time* in-process (mismatched `Lane<In,Out>` fails to compile); **abstract interpretation (octagons)** — a *sound* occupancy envelope that proves no queue overflows, wrapping the optimizers as a safety gate.

**Axis 2 — The spider as a genuine agent** (`agentic-reasoning`):
- **CFR / CFR+** — equilibrium routing under mutually-adaptive flows (Exp3 only credits the chosen arm vs an oblivious adversary; CFR credits unchosen off-ramps via counterfactual costs the kernel already meters).
- **Online calibration + chain-of-verification canary** — makes the spider's *own* confidence ECE-trustworthy; fire a single-credit canary below the calibrated threshold, verify before committing.
- **Reasoning-monitoring** — cycle detection (Floyd/Brent / Bloom of state hashes) on the realized action trace catches *live* routing limit-cycles the offline spectral ρmax can't see.
- **PRM step-credit + beam search** placement; **typed-DAG orchestrator** — derives the parallel lane schedule + barriers from the dependency graph and type-checks every edge end-to-end.

**Axis 3 — Principled estimation** (`statistics-probability`) — *stronger replacements* for extracted heuristics:
- **Thompson sampling (Beta/Dirichlet)** — strictly stronger drop-in for **Exp3** in the stochastic-but-drifting regime the bus actually inhabits: order-optimal stochastic regret, O(1) closed-form integer increments, no step-size, no_std; the posterior *is* the self-advertised confidence a strand publishes. (Keep Exp3 only where a route is believed adversarial.)
- **CUSUM / Wald SPRT** — delay-optimal (Lorden) change-point detection; upgrades **φ-accrual** on slow "alive-but-stuck" drift.
- **Recursive Bayesian (scalar Kalman / particle)** — denoised cost mean + variance for explore-where-uncertain routing (t-digest/Count-Min are memory-bounded but dynamics-free).
- **Benjamini-Hochberg FDR** — the missing decision-aggregation layer: bounds the family-wise false-restart rate across hundreds of monitors at fleet scale.
- **HMM/Viterbi** latent producer-phase intent; **GP-BO** auto-tunes the now-many meta-knobs in tens of trials.

**Axis 4 — Authenticity / authority / anti-DoS for P2P** (`cryptography-advanced`, v2):
- **BLS signature aggregation + threshold-BLS** — constant-size unforgeable provenance (which nodes touched a packet, none forged credit) + t-of-n gating on destructive control (GC/evict/rebalance). Distinct from §B's scalar BLS path-aggregation.
- **SYN-cookie stateless-state** admission token (zero-state under register-flood); **cryptographic accumulator** — constant-size revocation + non-membership proofs (stronger than Bloom *in the membership path*, not the SCG bulk index); **VRF** unbiasable sortition; **VDF** proof-of-elapsed-sequential-time; **PSI** leakage-controlled cross-trust reconciliation.

**Axis 5 — Model-based rate control + tail-latency** (`networking`, orthogonal to backlog feedback):
- **BBR** — estimates BtlBw + RTprop directly, targets the bandwidth-delay product, keeps the queue *empty* (every extracted Lyapunov/bandit controller fills a buffer to learn the rate). The right law for low-latency lanes.
- **Happy-Eyeballs hedged dual-dispatch** — per-request p99 cutter (min of two lanes) with a staggered start paying the duplicate only on the tail.
- **Rendezvous/HRW hashing** — stateless, churn-stable, order-preserving stream→lane map (placement partitioners + queue schedulers don't give this); **QUIC Connection ID + path validation** — session identity invariant under NAT-rebind/failover; **STP + PFC** — loop-free multi-hop overlay + a checkable no-cyclic-buffer-wait deadlock invariant.

**Top picks (this harvest):** STARK/FRI trace certificate; the verification triad (TLA+/TLC + POR + k-induction monitors); **Thompson sampling** (replaces Exp3); **CUSUM/SPRT** (upgrades φ-accrual); BLS aggregation + threshold-BLS.

**Where these slot:** Axes 1–3 are largely *in-process buildable* (verification harness, spider estimators) and several are no_std-friendly drop-ins; Axes 4–5 are v2/edge-transport. New proposed kit domains beyond §C: `verification` is now its own axis (model-checking + runtime monitors + proof certificates).

## Provenance / caveats
- 4-pass sweep + new-domain harvest, complete 2026-06-21 (harvest · gold-mine · discovery · new-domains). Citations agent-sourced via WebSearch + HF paper-search; verify before depending. `emerging`/`speculative` tags are honest — `battle-tested` ones have canonical sources above.
- The in-process builds (§F #1–#3, plus Axis-1/2/3 harnesses and estimators) touch the real Spiderweb Bus codebase (`crates/spiderweb/src/lib.rs`); the §F v2 stack and Axes 4–5 are the edge-transport milestone.

*Source doctrine: The Painted Fence — Jesse. Harvested for the spiderweb-bus trilogy.*
