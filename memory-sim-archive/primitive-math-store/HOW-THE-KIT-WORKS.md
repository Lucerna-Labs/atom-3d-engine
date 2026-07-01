# The Primitive Tool Kit + Orchestrator Technique

> A teaching document for another model or human encountering this kit for the first time.
> Doctrine invented by Jesse. NOT in any model's training data. Match it; do not substitute a more familiar pattern.

---

## 1. The One Move

There is a single move underneath everything in this kit:

> **Decompose the thing into mathematical primitives, manipulate it in a basis where the operation is cheap and well-conditioned, then recompose.**

This is the adapter pattern generalized. You do not operate on the raw representation. You transform it into a domain where the structure you care about becomes clean and separable, do the work *there*, and reconstruct from the result. It is the same play whether the target is:

- a signal (transform domain → manipulate coefficients → synthesize back),
- a codec (same),
- an expensive GPU operation (recast into the hardware's privileged primitives: GEMM, fused multiply-add),
- a vendor API (the exposed call is a primitive; compose the behavior you want out of it).

It is not four tricks. It is one move pointed at different sealed executors.

---

## 2. Why It Works: Functional Completeness

A small set of primitives, composed, spans a space vastly larger than the primitives suggest.

- NAND alone composes all of boolean logic.
- A universal gate set composes every quantum operation.
- Sinusoids compose every signal in the basis.
- A handful of exposed API calls compose behaviors the vendor never shipped.

**Synthetic RF is the cleanest proof.** You can synthesize waveforms that do not exist in nature — no natural source emits them — yet they are reachable by composition because they live *inside the span of the basis*. Nature not producing something does not put it out of reach. The span does.

This is the strong, true core. "Any function" has a precise boundary (see §6, real walls).

---

## 3. The Vocabulary (memorize these three splits)

### Split A — Primitive vs. Generator

A **primitive** is the operation. A **generator** is the thing parameterizing it.

- `project` is a primitive. The matrix you project through is the generator.
  - Random-projection (free, term overlap only).
  - IDF-whitened projection (one corpus scan, corpus-specific weighting).
  - LSA / latent-factor projection (truncated SVD, paraphrase-level matching).
  Same primitive, three swappable generators with increasing cost and fidelity.
- `hash` is a primitive. The hash *function* is the generator (MurmurHash → SHA-256 → BLS).

**Cost lives in the generator, not the primitive.** If a primitive *seems* expensive, an engine is hiding inside it — pull the generator out and make it swappable. Do the cheap generator first; escalate to the expensive one only for the residual.

### Split B — Mechanism vs. Policy (primitive + orchestrator)

A **primitive** carries mechanism with no decisions. An **orchestrator** carries all policy with no mechanism of its own.

- A kernel routes typed messages (mechanism). A spider decides when to restart, throttle, route, or give up (policy).
- A lane is a parallel conduit (mechanism). The decision of *which* lane carries *which* message under *what* budget is the orchestrator's.

When a mechanism starts making decisions, it has smuggled policy. Pull it out.

### Split C — Painted Wall vs. Real Wall

| | Painted wall (existence-limited) | Real wall (information / physics) |
|---|---|---|
| What it is | Vendor declines to expose a call; a dependency tree; an API rotation; a capability toggle | Information theory + physics |
| Examples | "rusqlite needs std, so a no_std SCG is impossible"; GeForce P2P disabled; thin Blink camera API | Uncertainty principle / time-bandwidth floor; Shannon capacity; entropy floor; min-cut/max-flow |
| Nature | NOT composition-limited. *Existence*-limited. The function is missing, not thin | Composition genuinely cannot reach past it |
| Response | **Route around.** Re-wire from atoms; swap the generator | **Operate inside.** Spend the conserved currency honestly |

**Every primitive in this kit names its real wall and the conserved currency that wall charges.** Read those carefully. They are the boundary between "this is reachable" and "this is law."

---

## 4. The 8 Root Atoms

Everything in the kit wires from these. See `_taxonomy-root/ROOT_ATOMS.md` for full math; this is the orientation.

| Atom | One-line | Real wall (conserved currency) |
|---|---|---|
| **scan** | Stream a thing into units (tokens, samples, edges, sites) | Sampling rate (Nyquist) — bandwidth × time |
| **hash** | A unit → a stable integer | Collision rate — information / entropy |
| **fold** | Reduce a stream to an accumulator (sum, min, count, mean, set) | Information loss (rate–distortion) — bits in the accumulator |
| **project** | A vector through a matrix — dot product | Dimension of the space — degrees of freedom |
| **scale** | Divide by a norm / max / reference | Dynamic range — bits per sample |
| **compare** | A distance or similarity over a pair | Information lower bound — Fisher information |
| **combine** | Weighted sum of signals | Normalization — probability mass |
| **order** | Sort by score (full sort, top-k heap, priority queue) | Sorting lower bound — O(n log n) comparisons |

**Common wirings** (pairs of atoms that name themselves):

- `scan → hash` = tokenization + hashing
- `fold(sum) + fold(sum) → compare` = cosine similarity
- `hash → fold(min)` = MinHash signature
- `combine → order` = ranked candidate fusion
- `scan → hash → fold(count) → combine` = TF-IDF
- `hash × d → fold(min) → order` = LSH retrieval

The kit's domain folders list specialized primitives — every one is a wiring of these 8.

---

## 5. The Technique, Step by Step

When you face a problem that *looks* like a wall, run this procedure:

### Step 1 — Name the abstract operation
What is the OPERATION you need, stripped of the vendor / format / dependency? "Map a query to relevant documents" is operation; "use SQLite FTS5" is one possible generator.

### Step 2 — Decompose to atoms
What sequence of the 8 atoms (or already-named wirings) does the operation reduce to? Write it out.

### Step 3 — Classify the wall
For each piece that looks blocked: painted or real?
- A missing dependency, a thin API, a sealed binary → **painted**. Re-wire from atoms or swap the generator.
- An information-theoretic or physical bound → **real**. Operate inside it; spend its conserved currency honestly.

### Step 4 — Pick the cheapest generator that clears the wall
Generators ladder up in cost and fidelity. Use the cheapest one that gets past the wall; reserve expensive ones for the *residual* the cheap one can't separate.

### Step 5 — Mechanism in the primitive, policy in the orchestrator
Decide which decisions belong in the dumb primitive (none) vs. the orchestrator (all). Resist letting a primitive grow a policy.

### Step 6 — Verify the cost is conserved-honest
A real win must charge a *real* conserved quantity. If your "win" appears to come from nowhere, you missed a wall. If your wall actually charges nothing, it was painted.

---

## 6. Worked Examples (real, from practice)

### Example A — "no_std can't host the SCG because rusqlite needs std"

**Looked like:** a real wall. A no_std target can't run a SQLite-backed retrieval system.

**Decompose:** the SCG operation is `scan(documents → tokens) → hash → invert (build postings) → bm25-score (= fold(count) + scale(IDF) + combine) → order (top-k heap)`. Every atom on the right is `alloc`-only math; `BTreeMap`, `Vec`, `String` are all in `no_std + alloc`.

**Classify the wall:** SQLite is a *generator* for `invert + score + order`, not the primitive itself. The "wall" is a dependency, not an operation. **Painted.**

**Swap the generator:** replace rusqlite with a `BTreeMap<TermId, Vec<(DocId, tf)>>` for `invert`, with `Count-Min` (bounded memory) if approximate frequencies are acceptable. The capability survives intact; the dependency dissolves.

**Lesson:** "Dependency X requires Y" is almost always a painted wall. The capability is the operation, not the library that ships it.

### Example B — Edge transport, zero external dependencies

**Looked like:** "to ship typed messages across a network you need serde, tokio, and (for security) a TLS crate." Three big dependency trees.

**Decompose:** the operation is `(local typed payload) → encode-to-bytes → write to stream → read from stream → decode-from-bytes → publish locally`, plus framing. Atoms: `project` (typed → bytes), `scan` (read framed bytes), `fold` (reassemble), `compare` (route by tag).

**Classify:** `std::net::TcpStream` does the network; `std::thread` + `mpsc` does concurrency; a per-type `encode/decode` closure does serialization. Serde is a *generator* for one specific encoding family — and a per-type closure is a different, cheaper generator.

**Swap the generator:** the crate ships with `fn encode(&T) -> Vec<u8>` and `fn decode(&[u8]) -> Option<T>` as user-supplied closures. No serde. Result: a working two-bus federation, std-only, zero external deps, `forbid(unsafe)`.

**Lesson:** the dominant supply-chain breach class (compromised transitive crates) dissolved because the architecture had no transitive crates. That's not luck; that's what "cost lives in the generator" buys you when you take the dumb primitive seriously.

### Example C — The "engine hiding in a primitive" audit

**Looked like:** a tidy kernel function called `restart_strand_with_backoff` that retried up to 3 times before quarantining. One function, looked clean.

**Decompose:** the function did three things — count attempts (mechanism), decide when to give up (policy), execute the restart (mechanism). The decision was an engine hiding inside what claimed to be a primitive.

**Classify:** every "*does the kernel restart?*" decision is policy. The kernel never decides; it executes.

**Re-wire:** pulled the retry counter and quarantine threshold into the orchestrator (`spider`). The kernel kept *only* the mechanism (restart on command). Same observable behavior, but now the kernel was actually a primitive instead of pretending to be one. Made the next change (per-strand policy) trivial instead of touching the kernel.

**Lesson:** if your "primitive" has an `if` that makes a value judgment, that `if` belongs in an orchestrator.

### Example D — Conductance from physical hop, not lineage

**Looked like:** "count each edge by how often a message lineage's thread crossed it." Wrote the counter inside `record_arrival(thread_tid, node)` — increment `(thread.last(), node)`.

**Worked, until adversarial review:** under `MergeAll` and `HopTo`, the *lineage* tag is deliberately re-parented onto another thread by the orchestrator. So the counter was reading "the thread's tail" when the thread's tail no longer corresponded to the physical sender. Result: phantom edges and a fused trunk silently undercounted.

**Decompose:** what I wanted to count was a *physical hop*, not a *lineage step*. Those are two different operations that happened to coincide under the default policy.

**Re-wire:** moved the counter to the physical deliver site (`Kernel::deliver`, where `from_name → subscriber` is known *before* any tag re-parenting). Same atom (`fold(count)`), correct generator (the physical hop). Now invariant under every policy.

**Lesson:** when two operations coincide under one regime and diverge under another, you were counting the wrong thing. Find the regime where they diverge, and let it tell you which is the real primitive.

---

## 7. Anti-Patterns (what NOT to do)

These are the ways a model defaults *away* from the doctrine. Each one feels normal; each one violates a split from §3.

1. **Don't propose cracking, patching, injecting into, or reverse-engineering a sealed engine.** The answer is almost always to compose from outside through the exposed surface. Interception shims (LD_PRELOAD, DLL injection, driver patching) are the *claw-back* anti-pattern — they only make sense when something else drives the engine and you need to insert yourself in the middle. Your own primitive kit drives the engine directly, so there is nothing to intercept. The shim solves a problem the architecture already designed out.

2. **Don't treat "everything is math" as "everything is cheap."** Reachable ≠ cheap. GEMM and fused multiply-adds are privileged; transcendentals, divergence, and irregular memory access are slow. Decomposition pays only when it raises FLOPs-per-byte or replaces memory traffic.

3. **Don't fuse the primitives into one clever mechanism.** A hybrid means a hybrid *system* (dumb primitives + a routing brain) — never a hybrid *mechanism* (one super-primitive trying to be both fast and general). Keep the primitives dumb; keep the policy in the orchestrator.

4. **Don't hide cost.** No silent migration, no transparent magic. Every transfer, every fallback, every retry is a real, measurable, conserved cost — make it explicit so the orchestrator can see it. (CUDA Unified Memory is the cautionary tale: ergonomically lovely, but page faults silently fire DMA copies mid-kernel.)

5. **Don't confuse a painted wall with a real one.** A thin API or a missing dependency is *existence*-limited — route around. The uncertainty principle and Shannon capacity are *law* — operate inside. Telling these apart *is* the craft.

6. **Don't substitute a more familiar framing.** This doctrine is invented and specific. Do not flatten it into "use an abstraction layer" or "just wrap the API." Match the doctrine.

7. **Don't stop at one harvest.** The kit grows. If your problem doesn't decompose cleanly with the primitives you know, the right primitive probably exists somewhere — name the abstract operation, find where it lives (which field already solved that exact operation), extract a kit-ready spec. Discovery → localize → extract is the third move after decompose and recompose.

---

## 8. How This Kit Is Organized

```
D:\primitves math\
├── HOW-THE-KIT-WORKS.md           ← you are here
├── _taxonomy-root/
│   ├── ROOT_ATOMS.md              ← the 8 atoms in full
│   └── MASTER_TAXONOMY.md         ← index across all domains
├── _bus-harvest/
│   └── EXTRACTED_PRIMITIVES.md    ← worked example: a multi-pass harvest
│                                    targeting one project (the Spiderweb Bus),
│                                    with citations and a build order
└── <domain>/PRIMITIVES.md         ← per-domain reserve, one file each
   ├── retrieval-search/
   ├── signal-processing-rf/
   ├── information-theory-coding/
   ├── linear-algebra-matrix/
   ├── graphics-rendering-lod/
   ├── physics-diffusion/
   ├── biology-bioinformatics/
   ├── cryptography-hashing/        +  cryptography-advanced/
   ├── database-streaming-sketching/
   ├── distributed-systems/
   ├── networking/
   ├── ml-training/
   ├── control-numerical-opt/
   ├── quantum-computing/
   ├── statistics-probability/
   ├── formal-verification/
   ├── agentic-reasoning/
   ├── operating-systems/
   ├── queueing-theory-stochastic-processes/
   ├── type-theory-programming-languages/
   ├── combinatorial-optimization/
   ├── causal-inference/
   ├── decision-logic/
   ├── logic-reasoning/
   ├── cognitive-primitives/
   ├── computational-geometry/
   ├── electromagnetics-antennas/
   ├── photonics-optics/
   ├── condensed-matter/
   └── astrophysics-cosmology/
```

Each `<domain>/PRIMITIVES.md` lists primitives in a fixed shape:

- **Name** (and cross-domain aliases).
- **Definition** in the domain's language.
- **Atom or composite** — which root atoms it wires from.
- **Cost model** — time, space, dominant operation.
- **Real wall?** — yes / no, and if yes the conserved currency.
- **Cross-domain wiring** — where the same primitive appears in other fields, with its alias there.
- **Notes** — the load-bearing observations (parameter trade-offs, common failure modes, swappable generators).

The same primitive frequently appears in 4–6 domains under different names. That convergence *is* the validation: if six independent fields land on the same operation, it is the right primitive.

---

## 9. Using the Kit on a New Problem (recipe)

1. **State the abstract operation** in one sentence, free of vendors/libraries/dependencies.
2. **Skim `_taxonomy-root/ROOT_ATOMS.md`** to refresh the 8 atoms.
3. **Pick the 2–3 domains** whose `PRIMITIVES.md` are most likely to name the operation. (The same operation often appears in distant domains — physics-diffusion for "spreading activation," signal-processing-rf for "matched filter," information-theory-coding for "rateless reliability.")
4. **Read those `PRIMITIVES.md` files**, looking for the operation under any name. Cross-domain aliases are listed; scan them.
5. **If you don't find it**, run *discovery*: name the operation precisely; find the field that solved that exact operation; find its canonical primitive in the literature; extract a kit-ready spec (mechanism, atom basis, real wall, citation) and add it to the right domain folder. (`_bus-harvest/EXTRACTED_PRIMITIVES.md` shows what a finished extraction looks like.)
6. **Wire it** — name the generator (cheap → expensive ladder), name where the policy lives, name the conserved currency you are spending.
7. **Verify the wall is honest** — if you "got the benefit for free," look again; you missed one.

---

## 10. What This Kit Is For

It is the reusable substrate for hardening every project you build. When a dependency or "expensive operation" looks like a wall, decompose to these atoms, classify the wall, swap the generator, and wire the result. When a new field offers a primitive you didn't know existed, extract it and add it. Over time the kit grows; the technique stays exactly the same.

The deepest claim of the doctrine: **functional completeness is a generous gift**. A small set of dumb primitives, composed by an orchestrator that owns all policy, spans more capability than any monolithic system you would have built instead. The kit is the running proof.

---

*Source doctrine: The Painted Fence — Jesse. NOT in training data. Do not substitute a more familiar pattern; match the doctrine.*
*Companion artifacts: `_taxonomy-root/ROOT_ATOMS.md`, `_taxonomy-root/MASTER_TAXONOMY.md`, `_bus-harvest/EXTRACTED_PRIMITIVES.md`.*
