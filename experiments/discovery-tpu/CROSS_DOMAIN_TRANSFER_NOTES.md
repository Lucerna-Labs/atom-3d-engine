# Cross-Domain Transfer Notes for TPU Step Signals

These notes keep renderer-discovery results from being thrown away just because they did not ship as
the default MM3E marcher. A failed renderer candidate can still be a useful mechanism for the
spiderweb bus, Redos kernel work, inference engines, RF/signal-processing systems, networking,
security, false-memory systems, or LLM model exoskeletons.

The rule for this file: keep the mechanism, not the literal constant. Renderer constants such as
`0.588`, `1.70`, or `t*0.1` are scene/currency artifacts until another domain revalidates them.

## Source Evidence

Source experiment:

- `discover-tpu.py` / `discover-tpu.ipynb`: linear genetic-programming search over step functions.
- Inputs available to each program: `d`, `d_prev`, `eps`, `overlap`, `approach`, `curv`, `t`, step
  index, `radius`, constants.
- Core idea: a small decoded formula can be treated as a transferable mechanism.

Real-engine validation harness:

- `mm3e-orchestrator/examples/tpu_signals_real.rs`
- Resolution: `480x270`, `AA1`
- Views: 6 deterministic cameras
- Truth: 2x-step exact sphere tracing
- Candidate surface: real MM3E scene field, shadows, and AO

Renderer validation results:

| Signal | Total eval delta vs shipped | Primary eval delta vs shipped | Wall time | Hit agreement | Depth error | Renderer verdict |
|---|---:|---:|---:|---:|---:|---|
| shipped `omega=1.40` | baseline | baseline | `1104.10 ms` | `99.96%` | `0.00064` | current default |
| TPU `omega=1.70` | `+4.8%` | `+11.6%` | `1143.29 ms` | `99.93%` | `0.00064` | reject for renderer default |
| TPU raw `(radius+i*.02)/.588` | `-30.5%` | `-50.6%` | `817.14 ms` | `99.98%` | `0.02691` | speed signal, too much depth drift |
| TPU raw `overlap+t*.1` | `-47.8%` | `-47.4%` | `617.75 ms` | `99.89%` | `0.03353` | strongest speed signal, too much drift |

Important interpretation:

- The renderer has a hard geometric correctness gate. Depth drift of `0.02..0.03` is too expensive
  for exact/still rendering.
- Bus routing, kernels, inference scheduling, and exoskeleton control often tolerate bounded
  approximation if the system can detect drift and repair it.
- Therefore, the raw formulas should be tagged as `transfer candidates`, not `renderer failures`.

## Mechanism Inventory

### 1. Guarded Over-Relaxation

Renderer form:

```text
step = omega * d
if successive safe regions stop overlapping, roll back / become conservative
```

Renderer result:

- `omega=1.70` was worse than shipped `omega=1.40` on MM3E.
- The mechanism still matters: optimistic promotion with an overlap guard.

General mechanism:

- Move faster than the conservative safe step.
- Carry a cheap local consistency check.
- If the consistency check fails, undo the optimistic part and fall back to conservative motion.

Transfer value:

- Good for systems where rollback is cheap and waiting is expensive.
- Poor for systems where a bad jump corrupts state irreversibly.

### 2. Age-Indexed Aggression

Renderer form:

```text
step = (radius + i * 0.02) / 0.588
```

Renderer result:

- Big eval savings, but depth drift too high for exact rendering.

General mechanism:

- The longer a process has been moving without failing, the more aggressive it becomes.
- `i` is not magic. It means progress age, dwell time, number of confirmations, or number of hops.
- `radius` is a local safety margin or confidence margin.

Transfer value:

- Useful for schedulers, buses, and inference pipelines where confidence should grow with repeated
  non-failure.
- Needs a cap and a correction budget.

### 3. Overlap Plus Progress

Renderer form:

```text
step = overlap + t * 0.1
```

Renderer result:

- Fastest wall-clock result in the real-engine harness.
- Too much depth drift for exact rendering.

General mechanism:

- Combine local compatibility (`overlap`) with global progress (`t`).
- If local regions keep agreeing and the process has already advanced, take a larger jump.
- This is not just "go faster"; it is "go faster when the local continuity signal says the path is
  coherent."

Transfer value:

- Very relevant to the spiderweb bus because it resembles vibration-aware routing: when adjacent
  thread segments resonate/overlap and the message has made progress, promote or route more
  aggressively.
- Also relevant to cache reuse, speculative decode, prefetch, and lock/contention handling.

### 4. Currency-as-Safety

Renderer form:

```text
fitness = cost reduction gated by silhouette/depth error
```

General mechanism:

- The optimization is not allowed to define success by speed alone.
- Every domain needs a conserved currency and a correctness gate.

Transfer value:

- This may be the most important discovery-process result.
- For every future search, define:
  - the currency to reduce,
  - the drift/error metric,
  - the maximum acceptable repair cost,
  - and whether the result is exact, preview-only, or transfer-only.

### 5. Decoded Formula Over Black Box

Renderer form:

```text
linear GP -> decoded formula -> real-engine harness
```

General mechanism:

- Search can be broad, but the result must be inspectable.
- A usable result should become a small policy formula, not an opaque model.

Transfer value:

- Especially important for kernel, bus, and inference runtime work where observability and failure
  analysis matter.

## Spiderweb Bus Notes

The spiderweb bus is layered and elevation-based: ground-level local bus, highway-level parallel
promotion, on-ramps/off-ramps, emergent threads, intersections, and vibration propagation.

### Candidate Transfer: Overlap as Vibration

Use `overlap` as a bus-level compatibility/resonance signal:

```text
overlap_score = shared downstream consumers
              + cache/key compatibility
              + matching priority class
              + low conflict/backpressure
```

A message/thread with high overlap is more likely to be on a coherent path through the fabric.

Possible bus policy:

```text
if overlap_score is high and progress_age is increasing:
    promote to highway / parallel layer
else:
    stay on ground bus or route conservatively
```

Why this maps well:

- `overlap+t*.1` was bad for exact ray depth but excellent at reducing work.
- In the bus, some drift is acceptable if off-ramps validate and repair.
- This directly matches vibration propagation: local continuity plus global progress.

Research readout:

- p95/p99 latency improvement must not increase:
  - retry rate,
  - duplicate work,
  - lost message rate,
  - downstream correction cost,
  - or backpressure propagation time.

### Candidate Transfer: Age-Indexed Ramp Promotion

Use the `i*.02` idea as route age:

```text
ramp_aggression = base_margin + hop_count_or_dwell_time * small_gain
```

Meaning:

- A message that survives several local hops without conflict can be promoted more aggressively.
- A thread that repeatedly intersects the same high-throughput path can become a stronger highway
  candidate.

Do not predeclare the fabric. Let this policy strengthen paths that form through actual traffic.

Research readout:

- Cap promotion.
- Decay the score when backpressure rises.
- Demote through off-ramps when validation fails.

### Candidate Transfer: Guarded Optimistic Routing

Map over-relaxation rollback to routing:

```text
try fast route
if overlap/backpressure guard fails:
    undo optimistic route
    demote to conservative local bus path
```

This is useful for:

- speculative parallel execution,
- cache preloading,
- batching,
- multi-GPU / cloud fallback decisions,
- or on-ramp admission control.

Research readout:

- Rollback must be cheaper than waiting.
- Failed promotions must teach the fabric, not just retry blindly.

## Redos Kernel Notes

Assumption: "Redos kernel" here means Jesse's kernel/runtime work, not the renderer's convolution
kernel code.

### Candidate Transfer: Scheduler Quantum Growth

Age-indexed aggression can become a scheduler rule:

```text
if a task makes forward progress without faults/conflicts:
    gradually increase its quantum, batch size, or prefetch window
```

Useful for:

- I/O scheduler windows,
- task time slices,
- readahead depth,
- memory compaction passes,
- page-cache promotion.

Risk:

- Starvation and unfairness.

Research readout:

- Track tail latency, starvation count, missed deadlines, and rollback/fault rate.
- Add decay when contention appears.

### Candidate Transfer: Overlap-Aware Locking / I/O

`overlap+t*.1` suggests a policy where local compatibility plus progress age lets the kernel be more
aggressive.

Examples:

- If adjacent I/O requests overlap in region and direction, merge or prefetch farther.
- If lock acquisition patterns show stable overlap without conflict, allow optimistic batching.
- If page access streams overlap across tasks, promote pages or keep them warm longer.

Risk:

- Kernel-level drift is expensive. Do not let approximate policy touch correctness-critical state
  without validation.

Research readout:

- Use this first for performance hints, not correctness decisions.
- Every aggressive path needs a conservative fallback.

### Candidate Transfer: Currency-as-Safety for Kernel Experiments

Candidate kernel currency:

- context switches avoided,
- cache misses avoided,
- I/O wait reduced,
- page faults reduced,
- lock wait reduced.

Candidate correctness gates:

- no data corruption,
- no lost wakeups,
- no missed deadlines,
- no starvation,
- bounded rollback cost.

## Inference Engine Notes

Hardware context:

- Primary local inference target is the Windows workstation with RTX 5070 Ti 16GB VRAM.
- Fleet is heterogeneous; multi-GPU work should assume tiered/cache-style routing rather than one
  uniform tensor-parallel pool.
- Aedes-style VRAM tiering is relevant: active GPU VRAM, secondary GPU VRAM, system RAM, NVMe.

### Candidate Transfer: Confidence Margin as Radius

Map renderer `radius` / `d` to inference confidence margin:

```text
margin = top1_logit - top2_logit
entropy = token_distribution_uncertainty
cache_overlap = similarity with reusable KV / prefix / route
```

If the margin is wide and cache overlap is high, the engine may safely:

- prefetch farther,
- batch more aggressively,
- skip some repair checks,
- keep a KV segment hot,
- or promote a request to a faster path.

Research readout:

- Track token-level correction rate, perplexity drift, acceptance rate, and latency.
- Use draft/verify or rollback where possible.

### Candidate Transfer: Overlap Plus Decode Progress

Map `overlap+t*.1` to decode:

```text
aggression = cache_overlap + decode_position * small_gain
```

Possible uses:

- Increase speculative decode length as a generation stabilizes.
- Raise prefetch window for long contexts with stable attention locality.
- Promote repeated prefixes into a hotter KV/cache tier.
- Route stable batches through GPU while uncertain/branchy requests stay conservative.

Why this may work outside the renderer:

- The renderer punishes geometric depth drift.
- Inference engines can often verify candidate tokens or drop bad speculative work.

Research readout:

- Accepted speculative tokens per second must improve.
- Rejected speculative work must stay below the saved latency.
- Quality drift must be measured on task outputs, not just token throughput.

### Candidate Transfer: Program Search for Runtime Policies

The TPU experiment's linear-GP method may be more valuable than any one formula.

Search target:

```text
policy(inputs) -> prefetch length / batch admission / KV tier / route decision
```

Inputs:

- queue depth,
- VRAM free,
- token position,
- prefix overlap,
- entropy,
- model size,
- estimated KV growth,
- GPU temperature/load,
- cache hit/miss history.

Currency:

- tokens/sec,
- p95 latency,
- VRAM pressure,
- accepted speculative tokens,
- cache hit rate.

Gates:

- output quality,
- memory OOM rate,
- request starvation,
- rollback cost,
- thermal throttling.

## RF / Signal Processing Notes

RF is a natural transfer domain for these signals because many RF systems already live on the same
ideas: correlation peaks, coherent integration, lock quality, overlap windows, dwell time, and
adaptive sweep width.

Relevant RF primitives from the local primitive reserve:

- `correlate` / `matched-filter`: compare a received signal against a known template.
- `integrate`: accumulate over a window to improve SNR until coherence time runs out.
- `whiten`: normalize the background/noise before matching.
- `FFT` / `spectral-analysis`: move into a basis where channel structure is visible.
- `overlap-save-convolution` / `overlap-add-convolution`: block processing where overlap is a
  correctness boundary, not decoration.
- `beam-tracking`, `channel-estimation`, and `cognitive-radio sensing`: all depend on adaptive
  confidence and reacquisition policies.

### Candidate Transfer: Correlation Margin as Radius

Map renderer `radius` / `d` to signal confidence:

```text
margin = correlation_peak - next_best_peak
snr_margin = estimated_snr - required_snr
lock_margin = phase/frequency error budget remaining
```

If the margin is large, the receiver/scanner can afford a bigger next step:

- wider frequency-bin jumps during spectrum search,
- longer skip between pilot/channel-estimation updates,
- larger beam-refinement step,
- deeper coherent integration before rechecking,
- or more aggressive decimation/downsampling.

Research readout:

- false alarm rate,
- missed detection rate,
- BER/BLER,
- EVM,
- lock-loss rate,
- reacquisition time.

### Candidate Transfer: Overlap Plus Dwell Progress

Map `overlap+t*.1` to RF acquisition/tracking:

```text
aggression = window_overlap_or_correlation_consistency + dwell_time * small_gain
```

Possible uses:

- Matched-filter acquisition: if adjacent windows keep producing compatible peaks, move faster
  through the search space.
- Spectrum sensing: if energy/cyclostationary features remain coherent across windows, widen the
  sweep stride.
- Beam tracking: if adjacent beams share stable channel estimates, jump farther in the predicted
  direction.
- Channel estimation: if pilot estimates overlap smoothly across time/frequency, reduce pilot
  density or interpolate more aggressively.
- Overlap-save DSP: tune block overlap/window size based on measured boundary error rather than a
  fixed conservative value.

Why this may transfer:

- The renderer punished depth drift because exact geometry is the product.
- RF often has explicit reacquisition, resynchronization, and error-correction loops.
- A bounded wrong guess can be cheaper than a permanently conservative scan.

Research readout:

- Never let an aggressive receive policy silently lower detection reliability.
- Never let an aggressive transmit policy violate spectral masks, adjacent-channel leakage limits,
  or power/thermal constraints.
- Require a conservative reacquisition path after lock loss.

### Candidate Transfer: Age-Indexed Coherent Integration

The `i*.02` signal maps to dwell/integration age:

```text
confidence = base_margin + coherent_windows_survived * gain
```

Use:

- allow longer coherent integration while phase/frequency lock stays stable,
- increase beam confidence after repeated compatible measurements,
- reduce redundant sensing when the channel is stationary,
- expand FFT block size while stationarity holds.

Risk:

- Coherence time is a real wall. Past it, more integration smears instead of helps.
- In mobile or multipath channels, age can become stale confidence.

Research readout:

- decay immediately on Doppler spread, phase slip, fading, blockage, or peak splitting.
- cap by estimated coherence time.
- compare against a fixed-window baseline.

### Candidate Transfer: Guarded RF Optimism

Map guarded over-relaxation to scan/tracking:

```text
try larger frequency/beam/time step
if correlation overlap, SNR, or lock guard fails:
    roll back to last reliable bin/beam/window
    narrow the search
```

This fits:

- PLL/FLL acquisition sweeps,
- radar range/Doppler search,
- mmWave beam refinement,
- cognitive-radio channel scans,
- pilot-spacing adaptation.

Research readout:

- rollback must be faster than a conservative scan.
- track how often the guard fires; frequent rollback means the policy is over-aggressive.

### Candidate Transfer: RF Currency-as-Safety

Candidate RF currencies:

- samples processed,
- FFTs avoided,
- sweep time reduced,
- pilots saved,
- reacquisition time reduced,
- energy per detected signal,
- receiver CPU/GPU/DSP load.

Candidate correctness gates:

- detection probability at fixed false alarm rate,
- BER/BLER,
- EVM,
- synchronization/lock stability,
- channel-estimation NMSE,
- spectral mask compliance,
- adjacent-channel leakage,
- regulatory/power limits.

RF tag:

```text
renderer failure != RF failure
```

The transfer question is whether a signal reduces RF search/tracking work under a detection or
link-quality gate.

## Networking Notes

Networking is one of the strongest transfer domains because it already has explicit notions of
progress, overlap, backpressure, rollback, and route repair. The renderer's "step" maps to a network
policy decision: route farther, batch more, widen a window, choose a faster path, or promote a flow.

Relevant local primitives:

- TCP congestion/window control: safe growth, loss/backpressure, retransmission.
- Routing lookup / path selection: longest-prefix match, path-vector policy, link-state repair.
- MPTCP / link aggregation: multiple paths with path quality and failover.
- QoS queues / traffic shaping: priority, token buckets, buffer pressure.
- Netlink / socket diagnostics: feedback surfaces for observing path health.
- Packet fragmentation/reassembly: boundary correctness and repair cost.

### Candidate Transfer: Path Margin as Radius

Map renderer `radius` / `d` to network safety margin:

```text
path_margin = bandwidth_headroom
            + congestion_window_headroom
            + rtt_stability
            + loss_budget_remaining
            + queue_slack
```

If margin is high, the network layer can safely:

- increase congestion window faster,
- batch or coalesce messages,
- promote a flow to a highway/parallel lane,
- raise prefetch depth,
- use a lower-latency but less stable path,
- or reduce redundant probes.

Research readout:

- p95/p99 latency,
- jitter,
- packet loss,
- retransmits,
- reordering,
- queue depth,
- fairness across flows.

### Candidate Transfer: Overlap Plus Flow Progress

Map `overlap+t*.1` to flow continuity:

```text
aggression = path_overlap_or_shared_prefix + flow_age_or_bytes_delivered * small_gain
```

Possible meanings of `overlap`:

- two flows share a stable route prefix,
- packets share a QoS class and destination region,
- MPTCP subflows have correlated success,
- cached route metrics agree with live diagnostics,
- application messages share the same downstream graph path.

Possible uses:

- Route aggregation: promote compatible flows to the same fast lane.
- MPTCP: increase use of a subflow after repeated stable delivery.
- QUIC-style migration: try a new path, then keep it if overlap/progress stays good.
- Spiderweb bus transport: convert repeated local node-to-node traffic into a highway thread.
- Adaptive probing: probe less often once route overlap and progress stabilize.

Research readout:

- Do not let aggregation hide failures.
- Keep per-flow fairness.
- Demote quickly on loss bursts, ECN marks, RTT inflation, or route flaps.

### Candidate Transfer: Age-Indexed Congestion Growth

The `i*.02` signal resembles congestion slow-start / additive increase, but as a generic primitive:

```text
send_aggression = base_window + stable_round_trips * gain
```

Use:

- grow send windows while ACKs arrive cleanly,
- expand batch size while queues stay shallow,
- increase prefetch while downstream accepts work,
- lower redundant diagnostics while path health stays steady.

Risk:

- Old success can become stale after congestion, mobility, or route changes.
- Aggressive growth can create bufferbloat or unfairness.

Research readout:

- reset or decay on loss, ECN, high jitter, RTT slope, queue growth, or path change.
- compare against TCP-friendly behavior.
- never optimize one flow by starving others.

### Candidate Transfer: Guarded Optimistic Routing

Map over-relaxation rollback to network routing:

```text
try fast path / larger window / highway promotion
if loss, jitter, backpressure, or route-health guard fails:
    roll back to conservative path/window
```

This fits:

- MPTCP subflow selection,
- VPN/mesh path selection,
- bus on-ramp admission,
- prefetch depth,
- congestion-control experiments,
- service mesh routing.

Research readout:

- rollback must happen before congestion collapse or cascading backpressure.
- failed optimism must feed route metrics, not repeat endlessly.

### Candidate Transfer: Networking Currency-as-Safety

Candidate networking currencies:

- packets/messages delivered per CPU unit,
- RTT or tail latency reduced,
- retransmits avoided,
- probes avoided,
- route convergence time reduced,
- useful throughput per watt.

Candidate correctness gates:

- delivery correctness,
- no message loss,
- bounded duplication,
- fairness,
- congestion stability,
- path repair time,
- no route loops,
- no bufferbloat.

Networking tag:

```text
renderer drift may equal network speculation, as long as delivery semantics and fairness are gated
```

## Security Notes

Security is not only defensive filtering. In research mode these signals should also guide adversarial
experiments: failed bypasses, successful bypasses, false positives, missed detections, brittle heuristics,
and detector blind spots are all valuable data. They can guide inspection depth, anomaly triage, trust
scoring, rate limiting, key/cache placement, and hardening work.

The boundary is promotion, not exploration. Keep every security failure in the research corpus, but do not
let a learned shortcut weaken cryptographic checks, authentication, authorization, or integrity
verification in a deployed path.

Relevant local primitives:

- `hash-cryptographic`, `HMAC`, `AEAD`, `HKDF`, `KDF`: authentication, integrity, key derivation.
- `rolling-hash`, `fuzzy-hash`, `LSH`: similarity and near-duplicate detection.
- commitments / Merkle proofs / ZK-style verification: prove or bind state without exposing all data.
- Linux AF_ALG / TLS / keyctl / PF_KEY surfaces: practical kernel/user security APIs.

### Candidate Transfer: Trust Margin as Radius

Map renderer `radius` / `d` to security confidence margin:

```text
trust_margin = authenticated_identity_strength
             + policy_match_confidence
             + behavior_baseline_fit
             + integrity_signal_strength
             - anomaly_score
```

Use:

- decide how deep to inspect a request,
- choose whether to require step-up authentication,
- prioritize alerts,
- select cheap vs. expensive malware/similarity scans,
- tune rate limits for known-good but currently unusual traffic.

Research readout:

- false negative rate,
- false positive rate,
- auth bypass rate,
- failed bypass attempts and why they failed,
- successful bypasses and which assumption they broke,
- replay detection,
- privilege escalation attempts caught,
- benign edge cases that look malicious,
- alert fatigue,
- incident response time.

Promotion boundary:

- Never skip HMAC/signature/AEAD/tag verification because trust margin is high.
- Never reduce key length, nonce requirements, randomness, or replay protection.
- The signal can route defensive work; it cannot redefine cryptographic validity.

### Candidate Transfer: Overlap Plus Session Progress

Map `overlap+t*.1` to session continuity:

```text
security_confidence = policy_overlap + session_age_or_verified_actions * small_gain
```

Possible meanings of `overlap`:

- behavior matches previous authenticated session,
- device posture and network path match known-good profile,
- request shape overlaps with expected workflow,
- file/hash similarity matches known benign family,
- alert graph overlaps with a known incident pattern.

Use:

- reduce redundant prompts for stable low-risk sessions,
- increase trust in repeated successful checks,
- prioritize anomalous branches away from normal workflow,
- cluster alerts by overlap and progress through a kill chain,
- choose when to deepen inspection rather than inspect every packet/request equally,
- generate adversarial session traces that look normal until a late privilege or behavior shift,
- mine false-positive clusters for better benign baselines.

Research readout:

- bounded time-to-detect movement,
- missed high-severity alerts,
- step-up bypasses,
- lateral-movement blind spots,
- false-positive clusters,
- adversarial drift tests,
- normal-looking attack paths that become regression fixtures.

### Candidate Transfer: Age-Indexed Security Decay, Not Blind Trust

The renderer's `i*.02` signal says "be more aggressive after repeated non-failure." In security this
must be paired with decay:

```text
session_confidence = base + successful_verified_steps * gain - time_decay - anomaly_penalty
```

Use:

- widen fast-path access after repeated verified low-risk actions,
- lower inspection cost for stable machine-to-machine traffic,
- cache authorization decisions briefly,
- warm expensive reputation/fuzzy-hash indexes for suspicious clusters.

Risk:

- Attackers try to build trust and then abuse it.
- Long-lived sessions become stale.
- Similarity hashes can be evaded by adversarial perturbation.
- A failed policy may still reveal a usable attacker strategy or a detector blind spot.

Research readout:

- decay by time, privilege boundary, location change, device change, and anomaly.
- reset on sensitive actions.
- require step-up auth for privilege elevation regardless of age.
- keep trust-building attacks, stale-session failures, and similarity-evasion misses as test cases.

### Candidate Transfer: Guarded Security Optimism

Map over-relaxation rollback to security controls:

```text
allow fast security path
if anomaly, policy mismatch, replay signal, or integrity guard fails:
    roll back to strict path
    quarantine / require step-up / inspect deeply
```

This fits:

- adaptive zero-trust policy,
- rate-limit tuning,
- IDS alert triage,
- malware similarity clustering,
- WAF rule escalation,
- service-to-service auth caching.

Research readout:

- strict fallback must be reliable and cheap enough to trigger often.
- guard failures must become durable learning signals.
- the system must fail closed for integrity/authentication failures.
- every bypass or near-miss should produce a regression case, feature idea, or rule about where optimism
  is unsafe.

### Candidate Transfer: Security Currency and Failure Value

Candidate security currencies:

- analyst time saved,
- expensive scans avoided,
- auth prompts avoided without raising risk,
- alerts clustered,
- mean time to detect/respond reduced,
- CPU spent per protected request.

Failure currencies:

- bypass class discovered,
- false-positive family named,
- detector blind spot mapped,
- brittle heuristic exposed,
- stale-trust pattern found,
- benign edge case turned into a baseline feature,
- missed attack path turned into a regression test,
- failed hardening idea converted into a narrower rule.

Research readouts:

- auth bypasses found or prevented,
- integrity bypasses found or prevented,
- false negative and false positive movement,
- replay/evasion resistance,
- least-privilege preservation,
- auditability,
- explainability of trust decisions,
- cryptographic checks always enforced.

Security tag:

```text
security research keeps failures;
signals may optimize routing and hardening, never cryptographic truth
```

## LLM Model Exoskeleton Notes

Here "model exoskeleton" means the outer structure around a model: retrieval, memory, tool use,
critique loops, routing policies, prompt scaffolding, small helper models, and repair passes.

### Candidate Transfer: Memory Resonance as Overlap

Map `overlap` to how strongly a candidate memory/tool/scaffold resonates with the current task:

```text
overlap = semantic match
        + structural match
        + previous success on this thread
        - contradiction / stale context
```

Use:

- If overlap is high and the reasoning thread is progressing, retrieve deeper or invoke stronger tools.
- If overlap is low, keep the exoskeleton conservative and ask for/localize more evidence.

Research readout:

- Track hallucination rate, citation/evidence mismatch, tool-call waste, and correction loops.

### Candidate Transfer: Age-Indexed Scaffold Expansion

The `i*.02` signal can map to scaffold growth:

```text
as the task thread stabilizes:
    widen the memory horizon
    allow longer plans
    preload likely tools/docs
    promote repeated motifs into durable context
```

Use this for:

- long coding tasks,
- multi-file reviews,
- research synthesis,
- agent handoffs,
- recurring project memory.

Risk:

- The exoskeleton can overcommit to the wrong frame.

Research readout:

- Decay or reset on contradiction.
- Require stronger evidence before expanding context.
- Keep a cheap "what would falsify this route?" check.

### Candidate Transfer: Preview vs Exact Modes

Renderer lesson:

- raw TPU formulas are not exact-render defaults,
- but they may be excellent preview knobs.

Exoskeleton equivalent:

- fast approximate planning mode,
- exact verification mode,
- repair/finalization mode.

Do not use the same policy for all phases. Aggressive overlap/progress rules belong in preview,
brainstorming, cache warming, and candidate generation. Exact/final phases need conservative gates.

## False Memory / Memory Integrity Notes

This section is for synthetic memory, false-memory LoRA work, RAG memory, agent memory, project memory,
and any future exoskeleton layer that treats remembered material as reusable substrate.

Important distinction:

- False memories can be useful as behavioral training circuits when they encode
  `situation -> recognition -> action -> consequence -> updated instinct`.
- False memories can also be useful as constructive synthetic episodes for a Kaggle/OpenAI competition
  algorithm, provided they are treated as strategy memories or search priors rather than facts.
- False memories are dangerous when the system starts treating repeated, coherent, or familiar
  material as verified fact.
- Research failures are first-class outputs. A memory, formula, feature, or strategy that fails one
  task can still teach the search policy, expose a hidden constraint, or become useful in another domain.
- The restriction is on promotion and use, not on recording or exploring the result.

Relevant local primitives and findings:

- `source-memory`: item recognition can survive while source attribution fails.
- `source-monitoring`: source attribution failures produce false memories and confabulation.
- `false-memory-drm-paradigm`: semantically related items can produce high-confidence recall of a
  never-presented lure.
- `schema-memory`: coherent schemas improve recall but distort toward expectation.
- `processing-fluency`: ease of retrieval can feel like truth.
- Small-model memory-lab finding: negative-outcome memories can contaminate Q4 models if they preserve
  the exact wrong answer token. A memory that says "I once answered X and it was wrong" may cause the
  model to emit X anyway.

### Candidate Transfer: Provenance Margin as Radius

Map renderer `radius` / `d` to memory reliability:

```text
memory_margin = source_strength
              + evidence_count
              + timestamp/commit/provenance quality
              + independent corroboration
              - contradiction_score
              - generation_only_penalty
```

Use:

- decide whether a memory can be used as fact,
- decide whether it can only be used as a behavioral hint,
- decide whether to ask for evidence,
- decide whether to quarantine or decay a memory,
- decide whether to write it into durable project memory.

Research readout:

- source precision,
- citation/provenance accuracy,
- contradiction rate,
- hallucinated-detail rate,
- unsupported-claim rate,
- stale-memory usage,
- exact-wrong-token leakage.

### Candidate Transfer: Constructive Synthetic Memory for Competition Algorithms

The useful side is not only defensive. A competition algorithm can use intentionally synthetic memories as
training pressure for how an agent searches, tests, and corrects itself.

Constructive memory types:

- `strategy memory`: "When feature family A improves local validation but widens the public/private gap,
  run a leakage check before keeping it."
- `failure-class memory`: describe the class of mistake without preserving the exact wrong output.
- `scoring-loop memory`: hypothesis -> notebook change -> metric -> ablation -> keep/revert decision.
- `source-boundary memory`: keep competition rules, allowed data, external-data status, and submission
  constraints warm without treating guesses as rules.
- `algorithm-motif memory`: reusable patterns such as split design, calibration, retrieval, ensembling,
  threshold tuning, and public/private leaderboard gap checks.

How to use it for a Kaggle/OpenAI competition algorithm:

- prompt/RAG policy for an agent that writes and critiques notebooks,
- LoRA or fine-tune corpus for narrow behavior if the rules and compute budget allow it,
- search prior for which features, prompts, retrievers, ensembles, or validators to try first,
- critique memory for overfitting, leakage, stale assumptions, and public-leaderboard chasing,
- experiment-log scaffold that keeps every submission tied to a hypothesis and ablation.

Research readout:

- held-out/CV movement, including negative movement,
- public/private leaderboard gap,
- ablation against a rules-only prompt or no-memory agent,
- reproducibility across seeds or reruns,
- whether a failed idea reveals leakage, split mismatch, prompt mismatch, or feature brittleness,
- whether exact labels, solution-shaped artifacts, or disallowed resources were involved,
- whether apparent improvement comes from the idea itself or from parroting synthetic memory phrasing.

Competition currency:

- better score per submission,
- valid submissions per hour,
- fewer dead-end experiments,
- fewer leakage bugs,
- fewer rule/compliance mistakes,
- lower token/GPU/TPU cost per useful hypothesis.

Failure currency:

- a feature family eliminated or narrowed,
- a synthetic memory pattern shown to contaminate or distract,
- a validation split shown to be misleading,
- a public-leaderboard improvement shown to be non-generalizing,
- a prompt/retrieval pattern shown to bias the wrong behavior,
- an algorithm motif that fails here but becomes a transfer candidate for another subsystem.

### Candidate Transfer: Overlap Plus Rehearsal Is a Risk Signal

In the renderer, `overlap+t*.1` means local continuity plus progress. In memory systems it can mean
something darker:

```text
false_confidence_risk = semantic_overlap + rehearsal_count * small_gain
```

Why:

- Repetition makes a memory fluent.
- Similar memories can create gist.
- Gist can produce a plausible lure.
- The lure may be recalled with confidence even when it has no source.

Use this signal both ways:

- in exploration, deliberately rehearse or cluster related memories to see what lures the model invents,
- in candidate generation, treat strong but unverified overlap as a brainstorming prior,
- flag memories that are becoming strong through repetition but lack provenance,
- separate "behavioral memory" from "fact memory",
- require source checks before using repeated content as evidence,
- detect when multiple related memories are converging on an unsupported detail.

Mode rule:

- In research mode, repeated unsupported material is still useful as a lure, stress test, or search prior.
- In submission/deployment mode, repetition can increase behavioral weight but not factual truth value.

### Candidate Transfer: Age-Indexed Memory Consolidation With Decay

The `i*.02` signal maps to rehearsal/consolidation age:

```text
memory_strength = base + successful_retrievals * gain - time_decay - contradiction_penalty
```

Use:

- strengthen behavioral instincts that repeatedly help,
- keep high-value source-boundary memories warm,
- decay stale memories,
- lower confidence when evidence has not been refreshed.

Risk:

- Long-lived memories become trusted because they are familiar.
- Synthetic memories can overfit a small model into parroting phrasing.
- Negative memories can leak the exact bad answer/action they were meant to suppress.

Research handling:

- consolidate behavior, not unsupported factual content.
- keep paired variants when possible: one verbatim failure memory and one abstracted failure-class memory.
- measure which variant teaches recovery and which variant contaminates output.
- store correction memories as failure classes when the exact bad token is shown to leak.
- require external evidence before promoting a synthetic memory into a submitted/factual claim.

### Candidate Transfer: Source-Monitoring Guard

Map guarded over-relaxation to memory use:

```text
use memory aggressively in research
if source/provenance/contradiction guard fails:
    keep it as a hypothesis or failure signal
    do not promote it to factual/submission use without evidence
```

Possible memory states:

- `fact`: source-backed, current, externally checkable.
- `working assumption`: useful but not verified.
- `behavioral memory`: shapes action but is not factual evidence.
- `synthetic training memory`: intentionally fictional, only useful as a pattern.
- `contaminated`: known wrong, stale, or source-confused.

Promotion rule:

- The model should be able to say where a memory came from.
- If it cannot, the memory can still be used for exploration, failure analysis, or behavior shaping.
- Factual/submission use needs stronger source support than exploration use.
- Synthetic memory must be labeled or structurally separated from evidence memory.

### Candidate Transfer: False-Memory Currency and Failure Value

Candidate currencies:

- task success from memory-shaped examples,
- useful failures preserved as search guidance,
- fewer prompt-injection failures,
- fewer arithmetic/process slips,
- better source-boundary behavior,
- fewer repeated tool calls,
- faster recall of stable project facts.

Candidate readouts:

- unsupported factual claims found,
- false attribution found,
- exact-wrong-token leakage observed or avoided,
- source-boundary collapse observed or avoided,
- contamination between synthetic and evidence memory,
- hallucination confidence movement,
- held-out task lift or drop, not just memory phrasing imitation.

False-memory tag:

```text
failed memories are data;
coherence is not provenance; rehearsal is not truth
```

This is the memory-domain equivalent of the renderer depth gate. A memory can be useful and still not
be ready for evidence/submission use.

## Watchlist for Future Kaggle/TPU Results

For every new formula or sim result, capture this table before deciding where it belongs. Failed
results are not trash; they may be negative evidence, a transfer mechanism, or a clue about the next
search direction.

| Field | Notes to record |
|---|---|
| Formula / decoded rule | Exact text from the run. |
| Source run | CPU smoke, Kaggle TPU, GPU confirmation, real-engine harness, etc. |
| Currency optimized | Field evals, wall-clock, tokens/sec, latency, cache hits, etc. |
| Correctness/readout | Depth error, output quality, lost messages, OOM rate, starvation, etc. |
| Failure mode | Drift, tunneling, unfairness, stale context, wasted speculation, etc. |
| Failure value | What the failed or partial result teaches: bad prior, hidden constraint, transfer lead, split issue, or stress case. |
| Mechanism label | Optimistic rollback, overlap+progress, age-indexed aggression, threshold leap, etc. |
| Research status | Sandbox-only, failure-signal, transfer-only, preview knob, promotion candidate, or ship. |
| Spiderweb bus hypothesis | How it affects threads, ramps, intersections, or vibrations. |
| Kernel hypothesis | Scheduler, memory, I/O, lock, or cache use. |
| Inference hypothesis | KV/cache, batching, speculative decode, routing, or tiering use. |
| RF hypothesis | Correlation, sensing, beam tracking, channel estimation, scan stride, or DSP block use. |
| Networking hypothesis | Routing, congestion control, MPTCP, QoS, mesh/VPN pathing, or bus transport use. |
| Security hypothesis | Adversarial research value, bypass class, detector blind spot, false-positive family, defensive inspection, trust scoring, anomaly triage, auth caching, or integrity routing use. |
| Exoskeleton hypothesis | Retrieval, memory, tool routing, scaffold expansion, or verification use. |
| False-memory hypothesis | Constructive strategy memory, search priors, provenance, source-monitoring, contamination, rehearsal, competition-algorithm use, or memory-integrity use. |
| Required next probe | The smallest real test that could teach something either way. |

## Current Transfer Tags

| Signal | Renderer tag | Cross-domain tag |
|---|---|---|
| `omega=1.70` | not for MM3E default / transfer lead | guarded optimism / rollback template |
| `(radius+i*.02)/.588` | transfer-only / maybe preview | age-indexed aggression / congestion growth / session confidence with decay |
| `overlap+t*.1` | transfer-only / maybe preview | overlap plus progress / vibration routing / RF correlation-dwell routing / path/session continuity / strategy-memory reinforcement / false-confidence risk |
| `subitize` | shipped as dial | threshold leap / clear-margin fast path / SNR-margin fast path / trust-margin fast path / provenance-margin fast path |
| linear-GP decoded formula search | active method | policy discovery engine for bus/kernel/inference/network/security/memory |

## Immediate Next Experiments

1. Spiderweb bus simulator:
   - Implement a synthetic fabric with local bus, highway promotion, on/off ramps, and backpressure.
   - Compare conservative routing against `overlap + progress_age` promotion.
   - Measure p95 latency, duplicate work, missed deliveries, rollback cost, and any useful failure modes.

2. Inference scheduler harness:
   - Use a fake decode workload with KV-cache pressure and queue depth.
   - Compare static batching/prefetch against `cache_overlap + decode_position * k`.
   - Measure accepted work, p95 latency, VRAM pressure, quality/verification failures, and dead-end
     scheduling priors.

3. Exoskeleton memory-router harness:
   - Treat memories/tools as nodes and task state as a moving thread.
   - Compare static top-k retrieval against overlap-progress widening.
   - Measure evidence precision, task success, extra tool calls, correction loops, and failed routes that
     become useful anti-routes.

4. Redos kernel microbench:
   - Start with a non-correctness-critical policy such as readahead or cache promotion.
   - Compare fixed window against age-indexed growth with contention decay.
   - Measure throughput, tail latency, starvation, rollback/fault rate, and contention patterns learned
     from losing policies.

5. RF signal harness:
   - Build a synthetic acquisition/tracking workload with noise, Doppler drift, and multipath-like
     fading.
   - Compare fixed scan/dwell windows against `correlation_overlap + dwell_age * k`.
   - Measure false alarm rate, missed detection rate, lock-loss/reacquisition time, BER/EVM proxy,
     samples/FFTs avoided, and failure cases that reveal aliasing or tracking blind spots.

6. Networking harness:
   - Build a synthetic mesh/VPN or MPTCP-like path simulator with loss, jitter, route flaps, and
     backpressure.
   - Compare conservative routing/window growth against `path_overlap + flow_age * k`.
   - Measure delivery correctness, p95/p99 latency, fairness, retransmits, route repair time, congestion
     stability, and bad-routing cases that teach repair rules.

7. Security harness:
   - Build a security session/alert-routing simulator with benign drift, adversarial drift, failed
      bypasses, successful bypasses, false positives, replay attempts, and privilege-boundary events.
   - Compare fixed inspection depth against `trust_margin`, `policy_overlap + verified_steps * k`,
      and guarded fallback.
   - Measure false negatives, false positives, auth/integrity bypass rate, step-up correctness,
     alert fatigue, time to detect/respond, detector blind spots, failed hardening ideas, and attacks or
     false alarms that become training/regression cases.

8. False-memory harness:
   - Build a memory-router benchmark with true sourced facts, constructive competition-strategy memories,
      synthetic behavioral memories, source-confused lures, stale facts, and negative-outcome memories
      with/without exact bad tokens.
   - Compare static retrieval against `memory_margin`, `semantic_overlap + rehearsal_count * k`, and
      source-monitoring rollback.
   - Add a competition-algorithm lane that compares a rules-only notebook agent against a synthetic
      strategy-memory agent on held-out score, ablation quality, valid submissions/hour, and public/private
      gap.
   - Measure task lift or drop, source accuracy, unsupported claims, lure acceptance, hallucination
      confidence, exact-wrong-token leakage, rule compliance, score movement under ablation, and failed
      memories that teach better search.

The key posture: every result gets preserved as a mechanism until another domain falsifies it.
