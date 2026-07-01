# Cognitive Primitives — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.
>
> This catalog captures cognitive-science, psychology, and neuroscience-of-mind primitives drawn from classical cognitive architectures (ACT-R, SOAR, CLARION, LIDA), dual-process theory (Kahneman System 1/2), memory taxonomies (Baddeley, Tulving, Squire), attention research (Posner, Treisman), and predictive processing (Friston, Clark). Complements `agentic-reasoning/` (LLM-agent implementation), `decision-logic/` (runtime decisions), and `ml-training/` (learning algorithms).

---

## Section 1 — Attention & Salience

### salience-score (cross-domain alias: `priority`, `importance-weight`, `attention-weight`)
**Domain:** Cognitive Primitives
**Definition:** Scalar measure of how strongly a stimulus or item draws attention, computed from feature contrast and top-down relevance. Foundational to all attention-allocation models.
**Atom or composite:** Atom — a scalar `project` from features to priority.
**Cost model:** O(features) per item; cheap forward pass but updates with every context shift.
**Real wall?** Yes — only finite attentional capacity to spend on the highest-scoring items.
**Cross-domain wiring:** Maps to agentic-reasoning `context-prioritization` and `relevance-score`; decision-logic `priority-queue`; ml-training `attention-weights`.
**Notes:** Itti & Koch (2001) review; the canonical scalar at the base of every salience map.

### salience-map-itti-koch (cross-domain alias: `priority-map`, `attention-landscape`)
**Domain:** Cognitive Primitives
**Definition:** Topographic 2D map combining feature-channel conspicuity (color, intensity, orientation) into a master priority map that guides eye movements (Itti, Koch & Niebur 1998).
**Atom or composite:** Composite — `combine` of per-feature conspicuity maps via normalized summation.
**Cost model:** O(pixels × features); biological cortex computes in parallel, GPU mirrors closely.
**Real wall?** Yes — winner-take-all bottleneck limits one focus locus at a time.
**Cross-domain wiring:** Mirrors signal-processing-rf `power-spectrum-peak-picking`; agentic-reasoning `context-window-ranking`; ml-training `spatial-attention-map`.
**Notes:** Itti, Koch & Niebur (1998) IEEE PAMI; the seminal computational model of bottom-up attention.

### bottom-up-attention (cross-domain alias: `stimulus-driven-attention`, `exogenous-attention`)
**Domain:** Cognitive Primitives
**Definition:** Attention capture driven by intrinsic stimulus properties (sudden onset, high contrast, motion) without top-down goal influence; fast (~100ms) and reflexive.
**Atom or composite:** Atom — a `scan` over the sensory surface for outliers.
**Cost model:** Low-latency reflex (~100ms); cortical V1→pulvinar pathway.
**Real wall?** Yes — attentional capture is mandatory for high-salience transients (cannot suppress).
**Cross-domain wiring:** Decision-logic `interrupt-handler`; agentic-reasoning `event-trigger`; ordo-runtime `priority-preemption`.
**Notes:** Theeuwes (2010); contrast with endogenous attention; stimulus-driven capture even when task-irrelevant.

### top-down-attention (cross-domain alias: `goal-directed-attention`, `endogenous-attention`)
**Domain:** Cognitive Primitives
**Definition:** Attention modulated by current goals, expectations, and task set; slower (~300ms) and voluntary, originating in prefrontal and parietal cortex.
**Atom or composite:** Composite — `combine`(goal-template, sensory-map) via biased multiplication.
**Cost model:** Slower (~300ms) than bottom-up; requires sustained PFC activation.
**Real wall?** Yes — executive resources are limited; competes with stimulus capture.
**Cross-domain wiring:** Agentic-reasoning `goal-conditioned-retrieval`; decision-logic `policy-driven-focus`; ml-training `task-conditioning`.
**Notes:** Corbetta & Shulman (2002); dorsal frontoparietal network drives goal-directed selection.

### biased-competition-desimone-duncan (cross-domain alias: `competitive-selection`, `weighted-competition`)
**Domain:** Cognitive Primitives
**Definition:** Neurons compete for representation; top-down goals bias the competition in favor of task-relevant features, suppressing irrelevant ones (Desimone & Duncan 1995).
**Atom or composite:** Composite — `compare` + `combine` with multiplicative gain on goal-matched units.
**Cost model:** O(units²) lateral inhibition; biologically efficient, algorithmically softmax-like.
**Real wall?** Yes — only one winner per receptive-field overlap region.
**Cross-domain wiring:** ml-training `softmax-attention`; agentic-reasoning `tool-selection-competition`; decision-logic `arbitration`.
**Notes:** Desimone & Duncan (1995) Annu. Rev. Neurosci.; foundational for modern attention theory.

### feature-integration-theory-treisman (cross-domain alias: `FIT`, `binding-by-attention`)
**Domain:** Cognitive Primitives
**Definition:** Features (color, shape, orientation) are processed in parallel feature-maps; focal attention binds them into objects via a serial scan (Treisman & Gelade 1980).
**Atom or composite:** Composite — parallel `scan` of features + serial `combine` via spatial focus.
**Cost model:** Parallel feature stage O(1); serial binding O(N) in conjunction search.
**Real wall?** Yes — illusory conjunctions occur without attention, proving binding requires focus.
**Cross-domain wiring:** Agentic-reasoning `entity-resolution`; ml-training `binding-problem` (capsule nets); decision-logic `composite-key-construction`.
**Notes:** Treisman & Gelade (1980) Cognitive Psychology; the binding problem in vision.

### guided-search-wolfe (cross-domain alias: `top-down-guided-search`, `GS-model`)
**Domain:** Cognitive Primitives
**Definition:** Hybrid search where top-down feature templates bias a parallel activation map, guiding serial focal attention to high-priority candidates (Wolfe 1994, 2021 GS6).
**Atom or composite:** Composite — `project`(template) onto activation-map, then `order` by priority.
**Cost model:** Conjunction search slope reduced from pure serial by guidance strength.
**Real wall?** Yes — guidance is imperfect; distractor features still pull attention.
**Cross-domain wiring:** Agentic-reasoning `query-guided-retrieval`; ml-training `cross-attention`; decision-logic `prefilter-then-rank`.
**Notes:** Wolfe (1994, 2021) — Guided Search 6.0; integrates FIT with goal-driven guidance.

### visual-search (cross-domain alias: `target-detection`, `find-task`)
**Domain:** Cognitive Primitives
**Definition:** Task of locating a target among distractors; performance characterized by reaction-time slopes against set size, revealing parallel vs serial processing.
**Atom or composite:** Composite — `scan` + `compare` + `order` until target found.
**Cost model:** Flat slope ~0 ms/item for pop-out; 20–40 ms/item for conjunction search.
**Real wall?** Yes — set-size effect is the diagnostic signature of serial processing.
**Cross-domain wiring:** Agentic-reasoning `corpus-search`; decision-logic `lookup-with-scan`; ml-training `RetrieveAndRank`.
**Notes:** Treisman & Gelade (1980); the workhorse paradigm of attention research.

### pop-out (cross-domain alias: `feature-singleton`, `parallel-detection`)
**Domain:** Cognitive Primitives
**Definition:** Effortless detection of a unique-feature target (e.g., red among green) with reaction time independent of distractor count; indicates parallel preattentive processing.
**Atom or composite:** Atom — single-step `scan` returning the max-salience location.
**Cost model:** O(1) regardless of set size; fully parallel cortical processing.
**Real wall?** Yes — only single-feature differences pop out; conjunctions do not.
**Cross-domain wiring:** Signal-processing-rf `peak-detection`; agentic-reasoning `outlier-flag`; statistics-probability `extreme-value`.
**Notes:** Treisman & Gelade (1980); the defining empirical signature of preattentive vision.

### conjunction-search (cross-domain alias: `feature-conjunction`, `serial-search`)
**Domain:** Cognitive Primitives
**Definition:** Search for a target defined by a conjunction of features (e.g., red-vertical among red-horizontal and green-vertical); serial, attention-demanding with steep RT slopes.
**Atom or composite:** Composite — serial `combine` of feature maps under focal attention.
**Cost model:** ~20–40 ms/item; linear in set size.
**Real wall?** Yes — attention-binding bottleneck enforces seriality.
**Cross-domain wiring:** Decision-logic `composite-filter-scan`; agentic-reasoning `multi-constraint-retrieval`; ml-training `multi-head-attention`.
**Notes:** Treisman & Gelade (1980); diagnostic counterpart to pop-out.

### focal-attention (cross-domain alias: `attentional-focus`, `spotlight-attention`)
**Domain:** Cognitive Primitives
**Definition:** Concentrated allocation of attention to a small spatial or representational region; serves as the binding glue for features into objects.
**Atom or composite:** Atom — `project` of attention onto a single locus.
**Cost model:** One focus at a time; switching cost ~50–200 ms.
**Real wall?** Yes — only one focal locus per moment (with caveats for split-attention research).
**Cross-domain wiring:** Agentic-reasoning `current-context-window`; ml-training `query-token-attention`; decision-logic `active-cursor`.
**Notes:** Posner (1980); the spotlight metaphor of attention.

### ambient-attention (cross-domain alias: `diffuse-attention`, `global-mode`)
**Domain:** Cognitive Primitives
**Definition:** Broad, low-resolution attention spread over the full field; complementary to focal mode; supports gist extraction and rapid scene categorization.
**Atom or composite:** Composite — wide-scope `scan` with reduced per-location precision.
**Cost model:** Fast (~30 ms gist) but coarse; trades resolution for coverage.
**Real wall?** Yes — resolution/coverage tradeoff is fundamental.
**Cross-domain wiring:** Signal-processing-rf `wideband-survey`; agentic-reasoning `summary-pass`; decision-logic `coarse-routing`.
**Notes:** Trevarthen (1968), Oliva & Torralba (2006) on scene gist.

### focus-budget (cross-domain alias: `attention-budget`, `attentional-resource`)
**Domain:** Cognitive Primitives
**Definition:** Limited pool of attentional capacity that can be allocated across tasks; dual-task and divided-attention paradigms quantify the budget.
**Atom or composite:** Atom — a `scale` factor (0..1) summed across concurrent demands.
**Cost model:** Sum of allocations cannot exceed unity; performance degrades when overcommitted.
**Real wall?** Yes — Kahneman (1973) capacity theory; empirically measured ceiling.
**Cross-domain wiring:** Decision-logic `resource-budget`; agentic-reasoning `context-budget`; ordo-runtime `cpu-share-quota`.
**Notes:** Kahneman (1973) — Attention and Effort; the original capacity-budget framework.

### attentional-bottleneck (cross-domain alias: `selection-bottleneck`, `central-bottleneck`)
**Domain:** Cognitive Primitives
**Definition:** Stage in processing where parallel input streams must serialize through a limited-capacity channel; explains dual-task interference and PRP effects.
**Atom or composite:** Atom — a `fold` from many parallel inputs to one serial output.
**Cost model:** Forces serialization, producing characteristic queuing delays.
**Real wall?** Yes — Broadbent (1958) filter model; cannot process two responses simultaneously.
**Cross-domain wiring:** Decision-logic `mutex-serialize`; ordo-runtime `bus-arbiter`; agentic-reasoning `single-active-tool`.
**Notes:** Pashler (1994) review of PRP; classic Broadbent (1958) origin.

### attentional-spotlight (cross-domain alias: `spotlight-of-attention`, `Posner-spotlight`)
**Domain:** Cognitive Primitives
**Definition:** Metaphor for focal attention as a movable spotlight illuminating a region of the visual field, enhancing processing within and suppressing without.
**Atom or composite:** Atom — a `project` onto a region with gain enhancement.
**Cost model:** Movement velocity ~1°/8ms; gradient falloff at edges.
**Real wall?** Yes — only one spotlight (mostly); enhancement is graded.
**Cross-domain wiring:** Agentic-reasoning `cursor-position`; ml-training `local-attention-window`; decision-logic `region-of-interest`.
**Notes:** Posner, Snyder & Davidson (1980); the seminal spotlight paradigm.

### zoom-lens-model (cross-domain alias: `attentional-zoom`, `variable-aperture-attention`)
**Domain:** Cognitive Primitives
**Definition:** Refinement of the spotlight model in which the focus aperture is variable: tight focus = high resolution / small area, wide focus = low resolution / large area (Eriksen & St James 1986).
**Atom or composite:** Composite — `scale` parameter modulating spotlight radius and gain.
**Cost model:** Inverse area/resolution tradeoff; fixed total capacity.
**Real wall?** Yes — empirical tradeoff measured via flanker tasks.
**Cross-domain wiring:** Signal-processing-rf `time-bandwidth-product`; agentic-reasoning `context-window-tradeoff`; ml-training `attention-temperature`.
**Notes:** Eriksen & St James (1986) Perception & Psychophysics.

### attentional-blink (cross-domain alias: `AB`, `T2-deficit`)
**Domain:** Cognitive Primitives
**Definition:** Impaired detection of a second target (T2) when it appears 200–500 ms after a first target (T1) in a rapid serial visual presentation stream.
**Atom or composite:** Atom — a transient `gate-closed` interval following T1 capture.
**Cost model:** 200–500 ms refractory period; T2 must wait or be lost.
**Real wall?** Yes — central consolidation bottleneck during T1 encoding.
**Cross-domain wiring:** Ordo-runtime `post-event-refractory`; decision-logic `cooldown-window`; signal-processing-rf `receiver-dead-time`.
**Notes:** Raymond, Shapiro & Arnell (1992); a robust attention-temporal-dynamics phenomenon.

### inhibition-of-return (cross-domain alias: `IOR`, `return-suppression`)
**Domain:** Cognitive Primitives
**Definition:** Slowed response to a previously attended location after ~300 ms, biasing attention toward novel locations; facilitates foraging-efficient search (Posner & Cohen 1984).
**Atom or composite:** Atom — temporary negative `salience-score` tag on visited locations.
**Cost model:** Decay over seconds; tag list ~4–5 locations.
**Real wall?** Yes — limited tag capacity; tags decay.
**Cross-domain wiring:** Decision-logic `visited-set`; agentic-reasoning `no-repeat-search`; ml-training `exploration-bonus`.
**Notes:** Posner & Cohen (1984); foraging-theory rationale by Klein (2000).

### posner-cueing-paradigm (cross-domain alias: `spatial-cueing`, `valid-invalid-cue`)
**Domain:** Cognitive Primitives
**Definition:** Experimental task in which a precue indicates a target location (valid or invalid); RT advantage on valid trials quantifies attentional orienting.
**Atom or composite:** Composite — `project`(cue) → prior over locations → RT differential.
**Cost model:** Cue→target SOA 100–500 ms; magnitude of validity effect ~40–80 ms.
**Real wall?** No — paradigm, not constraint; but reveals real orienting cost.
**Cross-domain wiring:** Statistics-probability `prior-conditioning`; agentic-reasoning `hint-guided-search`; ml-training `attention-bias`.
**Notes:** Posner (1980) JEP:G; the gold-standard attention measurement paradigm.

### exogenous-orienting (cross-domain alias: `reflexive-orienting`, `peripheral-cue`)
**Domain:** Cognitive Primitives
**Definition:** Involuntary, fast (~100 ms) orienting of attention to a peripheral cue; cannot be fully suppressed even when cue is uninformative.
**Atom or composite:** Atom — reflex `scan` to high-salience transient.
**Cost model:** ~100 ms latency; mandatory.
**Real wall?** Yes — reflex pathway via superior colliculus; not under volitional control.
**Cross-domain wiring:** Decision-logic `interrupt-on-event`; ordo-runtime `hardware-IRQ`; agentic-reasoning `unexpected-token-trigger`.
**Notes:** Jonides (1981); contrasts with slower endogenous orienting.

### endogenous-orienting (cross-domain alias: `voluntary-orienting`, `central-cue`)
**Domain:** Cognitive Primitives
**Definition:** Voluntary, slower (~300 ms) orienting based on symbolic central cues (e.g., an arrow); requires interpretation and goal maintenance.
**Atom or composite:** Composite — `project`(symbol→location) then orient.
**Cost model:** ~300 ms latency; subject to suppression and modulation.
**Real wall?** Yes — interpretation requires working memory; not as fast as reflex.
**Cross-domain wiring:** Agentic-reasoning `instruction-following`; decision-logic `directive-execution`; ml-training `language-conditioned-attention`.
**Notes:** Jonides (1981); Posner et al. (1980); central-cue paradigm.

### orienting-response (cross-domain alias: `orienting-reflex`, `what-is-it-response`)
**Domain:** Cognitive Primitives
**Definition:** Pavlovian whole-organism response to novelty: head turn, pupil dilation, GSR change, increased EEG arousal; gateway to attention and learning.
**Atom or composite:** Atom — multi-channel `scan`-and-engage reflex.
**Cost model:** ~500 ms full body response; habituates with repeated exposure.
**Real wall?** Yes — habituation is mandatory after ~10 repetitions of the same stimulus.
**Cross-domain wiring:** Ml-training `novelty-bonus`; agentic-reasoning `OOD-flag`; decision-logic `anomaly-alert`.
**Notes:** Sokolov (1963); the foundational novelty-attention link.

### alerting-network (cross-domain alias: `phasic-alertness`, `vigilance-network`)
**Domain:** Cognitive Primitives
**Definition:** One of Posner & Petersen's three attention networks: maintains a state of high sensitivity to incoming stimuli; depends on right-frontal and locus coeruleus NE.
**Atom or composite:** Atom — global `scale`-up of sensory gain.
**Cost model:** Sustained alerting expensive; tonic vs phasic forms.
**Real wall?** Yes — vigilance decrement after 20–30 min sustained.
**Cross-domain wiring:** Ordo-runtime `polling-loop-tight`; decision-logic `high-priority-mode`; ml-training `temperature-scaling`.
**Notes:** Posner & Petersen (1990); ANT (Attention Network Test) measures it.

### executive-attention-network-posner-petersen (cross-domain alias: `executive-control-network`, `anterior-attention`)
**Domain:** Cognitive Primitives
**Definition:** Third Posner-Petersen network: resolves conflict, controls focus, anchored in ACC + lateral PFC; dopamine-modulated. Measured via Flanker, Stroop.
**Atom or composite:** Composite — `compare`(competing-responses) + `combine`(suppress losers).
**Cost model:** Conflict trials cost ~80–150 ms; cognitively expensive.
**Real wall?** Yes — limited supervisory capacity; conflict cost is unavoidable.
**Cross-domain wiring:** Decision-logic `arbitration`; agentic-reasoning `tool-conflict-resolution`; ordo-runtime `scheduler-preempt`.
**Notes:** Posner & Petersen (1990); Fan et al. (2002) ANT.

### selective-attention (cross-domain alias: `focused-attention`, `filter-attention`)
**Domain:** Cognitive Primitives
**Definition:** Capacity to focus on one stimulus stream while suppressing others; central to dichotic listening (Cherry 1953) and cocktail-party effect.
**Atom or composite:** Composite — `compare`(streams) + `project`(target-only).
**Cost model:** Suppression imperfect; some unattended info leaks through.
**Real wall?** Yes — capacity limit; cannot fully attend to two streams.
**Cross-domain wiring:** Agentic-reasoning `relevance-filter`; decision-logic `stream-selector`; ml-training `masked-attention`.
**Notes:** Cherry (1953); Broadbent (1958); the founding paradigm of attention research.

### divided-attention (cross-domain alias: `multitasking`, `dual-attention`)
**Domain:** Cognitive Primitives
**Definition:** Capacity to attend to multiple streams or tasks simultaneously; performance falls off sharply with similar tasks sharing modality or resources.
**Atom or composite:** Composite — parallel `combine` with capacity-sharing cost.
**Cost model:** Performance cost scales with task similarity and complexity.
**Real wall?** Yes — Wickens multiple-resource theory; some pairs irreducible.
**Cross-domain wiring:** Decision-logic `parallel-task-budget`; ordo-runtime `time-slicing`; agentic-reasoning `multi-tool-concurrency`.
**Notes:** Wickens (1980, 2002); the basis of cockpit task-load assessment.

### dual-task-interference (cross-domain alias: `DTI`, `task-overlap-cost`)
**Domain:** Cognitive Primitives
**Definition:** Decrement in performance when two tasks are performed concurrently versus sequentially; magnitude reveals shared-resource competition.
**Atom or composite:** Atom — performance `compare`(single vs dual).
**Cost model:** Cost ~10–80% depending on overlap of resources.
**Real wall?** Yes — central bottleneck (Pashler) makes some pairs irreducible.
**Cross-domain wiring:** Ordo-runtime `lock-contention`; decision-logic `concurrent-execution-penalty`; ml-training `negative-transfer`.
**Notes:** Pashler (1994); foundational to driver-distraction research.

### psychological-refractory-period (cross-domain alias: `PRP`, `response-bottleneck`)
**Domain:** Cognitive Primitives
**Definition:** RT to a second task (T2) increases as the SOA from T1 decreases, evidencing a central response-selection bottleneck (Welford 1952; Pashler 1994).
**Atom or composite:** Atom — serialized `compare`-then-respond on shared channel.
**Cost model:** T2 RT = base + (T1-processing-remaining); slope ~1 at short SOAs.
**Real wall?** Yes — response selection is serial; characterizes the central bottleneck.
**Cross-domain wiring:** Ordo-runtime `queue-wait-time`; decision-logic `pipeline-stall`; agentic-reasoning `sequential-tool-call`.
**Notes:** Welford (1952); Pashler (1994); central evidence for the bottleneck model.

### stroop-effect (cross-domain alias: `color-word-interference`, `Stroop-conflict`)
**Domain:** Cognitive Primitives
**Definition:** Slowed naming of ink color when the printed word names a different color; reveals automatic reading interfering with controlled color naming.
**Atom or composite:** Composite — `combine`(automatic-read, controlled-name) → conflict.
**Cost model:** Incongruent trials ~100–200 ms slower than congruent.
**Real wall?** Yes — reading is overlearned and automatic; cannot fully suppress.
**Cross-domain wiring:** Decision-logic `default-vs-override-conflict`; agentic-reasoning `instruction-vs-prior-tension`; ml-training `task-prior-conflict`.
**Notes:** Stroop (1935); MacLeod (1991) review; the canonical executive-attention task.

### attentional-capture (cross-domain alias: `involuntary-capture`, `stimulus-capture`)
**Domain:** Cognitive Primitives
**Definition:** Involuntary shift of attention to a salient or task-relevant feature even when irrelevant to the current goal; abrupt onsets and unique singletons capture strongly.
**Atom or composite:** Atom — reflex `project` to high-salience peripheral event.
**Cost model:** ~100 ms; bypasses voluntary control.
**Real wall?** Yes — bottom-up salience can override top-down set transiently.
**Cross-domain wiring:** Ordo-runtime `interrupt`; decision-logic `priority-preemption`; agentic-reasoning `unexpected-input-handling`.
**Notes:** Yantis & Jonides (1984); Theeuwes (1992).

### change-blindness (cross-domain alias: `change-detection-failure`, `flicker-paradigm`)
**Domain:** Cognitive Primitives
**Definition:** Failure to notice large changes to a scene when the change is masked by a saccade, blink, or flicker; reveals that perception is sparser than felt.
**Atom or composite:** Atom — gap in cross-frame `compare` operation.
**Cost model:** Detection requires attention; otherwise effectively never.
**Real wall?** Yes — without attention, no transient signal to detect change.
**Cross-domain wiring:** Agentic-reasoning `state-diff-without-attention`; decision-logic `unobserved-state-change`; ml-training `frame-prediction-failure`.
**Notes:** Rensink, O'Regan & Clark (1997); Simons & Levin (1998).

### inattentional-blindness (cross-domain alias: `gorilla-experiment`, `unattended-blindness`)
**Domain:** Cognitive Primitives
**Definition:** Failure to notice fully visible but unexpected stimuli when attention is engaged elsewhere; Simons & Chabris (1999) invisible-gorilla demo.
**Atom or composite:** Atom — un-attended region yields null `salience-score`.
**Cost model:** Detection probability falls sharply when attention is loaded.
**Real wall?** Yes — without attention, even high-contrast stimuli are not reported.
**Cross-domain wiring:** Decision-logic `unmonitored-channel`; agentic-reasoning `out-of-context-event-miss`; ordo-runtime `unsubscribed-event`.
**Notes:** Mack & Rock (1998); Simons & Chabris (1999).

### sustained-attention-vigilance (cross-domain alias: `vigilance`, `tonic-alertness`)
**Domain:** Cognitive Primitives
**Definition:** Capacity to maintain focus on a task over long durations, especially with low event rate; performance declines (vigilance decrement) over 20–30 min.
**Atom or composite:** Atom — long-duration `scan` with detection threshold.
**Cost model:** Linear-ish decrement; sensitivity (d′) drops over time.
**Real wall?** Yes — vigilance decrement is robust across modalities.
**Cross-domain wiring:** Ordo-runtime `long-poll`; decision-logic `monitor-drift`; ml-training `sample-fatigue`.
**Notes:** Mackworth (1948) clock test; Warm, Parasuraman & Matthews (2008).

### vigilance-decrement (cross-domain alias: `time-on-task-decrement`, `attention-fatigue`)
**Domain:** Cognitive Primitives
**Definition:** Decline in detection performance over time on a sustained-attention task; emerges within ~10–20 min and worsens with low event rate.
**Atom or composite:** Atom — temporal `scale`-down of vigilance gain.
**Cost model:** Hit-rate drops, RTs lengthen; reversible with break.
**Real wall?** Yes — robust across modalities; underlies shift-work safety risk.
**Cross-domain wiring:** Decision-logic `monitor-degradation`; ml-training `concept-drift-without-retraining`; ordo-runtime `polling-jitter-growth`.
**Notes:** Parasuraman (1979); resource-depletion vs mindlessness debate.

### mind-wandering (cross-domain alias: `task-unrelated-thought`, `default-mode-drift`)
**Domain:** Cognitive Primitives
**Definition:** Spontaneous shift of attention away from current task to self-generated thought; associated with default-mode network activity.
**Atom or composite:** Composite — `project`(internal-content) replaces external `scan`.
**Cost model:** Occupies 30–50% of waking time; cost to performance on attention-demanding tasks.
**Real wall?** Yes — cannot be eliminated; mandatory-attention tasks limit it but do not abolish.
**Cross-domain wiring:** Agentic-reasoning `off-policy-rollout`; decision-logic `background-thread`; ml-training `latent-dreaming`.
**Notes:** Smallwood & Schooler (2006, 2015); Killingsworth & Gilbert (2010).

### task-unrelated-thought (cross-domain alias: `TUT`, `off-task-cognition`)
**Domain:** Cognitive Primitives
**Definition:** Specific instances of internally generated content unrelated to the ongoing task; the operational measure underlying mind-wandering research.
**Atom or composite:** Atom — internal `project` divorced from current task context.
**Cost model:** Measured via experience sampling; counts per probe.
**Real wall?** Yes — sampled frequency is non-zero in nearly all subjects.
**Cross-domain wiring:** Decision-logic `off-goal-execution`; agentic-reasoning `unprompted-generation`; ordo-runtime `idle-task-runs`.
**Notes:** Smallwood & Schooler (2006); experience-sampling methodology.

### multiple-resource-theory-wickens (cross-domain alias: `MRT`, `Wickens-cube`)
**Domain:** Cognitive Primitives
**Definition:** Cognitive resources are multiple, structured along dimensions (stages, codes, modalities); tasks sharing dimensions interfere more than orthogonal pairs.
**Atom or composite:** Composite — `combine`(per-dimension capacity demand).
**Cost model:** Interference predicted by overlap on each dimension of the cube.
**Real wall?** Yes — distinct neural resources cannot be perfectly substituted.
**Cross-domain wiring:** Ordo-runtime `resource-class-quotas`; decision-logic `affinity-scheduling`; ml-training `parameter-sharing-conflict`.
**Notes:** Wickens (1980, 2002, 2008); engineering-psychology workhorse.

### bottleneck-theory-broadbent (cross-domain alias: `Broadbent-filter`, `early-filter-theory`)
**Domain:** Cognitive Primitives
**Definition:** Original filter model: information is selected at an early stage based on physical features before entering a limited-capacity channel for semantic analysis.
**Atom or composite:** Composite — `compare`(features) → `project`(through filter) → bottleneck.
**Cost model:** Filter operates on physical features; rejection saves downstream resources.
**Real wall?** Yes — limited-capacity channel; classic Broadbent (1958) hypothesis.
**Cross-domain wiring:** Ordo-runtime `inbound-filter`; decision-logic `prequalify-then-process`; ml-training `routing-then-experts`.
**Notes:** Broadbent (1958) — Perception and Communication; the founding model of attention.

### late-vs-early-selection (cross-domain alias: `selection-locus-debate`, `filter-locus`)
**Domain:** Cognitive Primitives
**Definition:** Long-running debate on whether attentional selection occurs early (pre-semantic) or late (post-meaning extraction); resolved by load-dependent flexibility.
**Atom or composite:** Atom — `scale` parameter on filter-locus from early to late.
**Cost model:** Late selection costs more per-item processed; early selection more efficient.
**Real wall?** Yes — irrelevant info still indexed but cost depends on load.
**Cross-domain wiring:** Decision-logic `prefilter-vs-postfilter`; agentic-reasoning `retrieval-then-rerank`; ml-training `early-vs-late-fusion`.
**Notes:** Treisman (1960) attenuation; Deutsch & Deutsch (1963) late; Lavie (1995) load theory.

### filter-theory (cross-domain alias: `Broadbent-filter-theory`, `all-or-none-selection`)
**Domain:** Cognitive Primitives
**Definition:** All-or-none version of early selection: unattended channels are fully blocked at the filter stage; later disconfirmed by semantic leakage findings.
**Atom or composite:** Atom — binary `project`(in/out) at filter.
**Cost model:** Computationally cheap; no processing of rejected channel.
**Real wall?** No (as stated); replaced by attenuation theory.
**Cross-domain wiring:** Decision-logic `hard-allowlist`; ordo-runtime `firewall-drop`; agentic-reasoning `hard-context-truncation`.
**Notes:** Broadbent (1958); historically important but empirically superseded.

### attenuation-theory-treisman (cross-domain alias: `Treisman-attenuator`, `graded-filter`)
**Domain:** Cognitive Primitives
**Definition:** Refinement of filter theory: unattended channels are attenuated rather than blocked, allowing semantic content with high signal value to break through.
**Atom or composite:** Atom — `scale`-down of unattended channel rather than gate.
**Cost model:** All channels still processed weakly; high-priority items can break through.
**Real wall?** Yes — empirical: own-name effect in unattended ear (Moray 1959).
**Cross-domain wiring:** Decision-logic `soft-prioritization`; agentic-reasoning `weighted-context-mixing`; ml-training `soft-attention-gating`.
**Notes:** Treisman (1960, 1964); resolves cocktail-party paradox.

### spotlight-gradient (cross-domain alias: `attention-gradient`, `gain-falloff`)
**Domain:** Cognitive Primitives
**Definition:** Attentional enhancement decreases with distance from the focus center; receptive-field gain is graded rather than step-function.
**Atom or composite:** Atom — `scale`(distance) → gain factor (Gaussian-like).
**Cost model:** Quasi-Gaussian falloff; peak gain at center.
**Real wall?** Yes — finite resolution at any aperture setting.
**Cross-domain wiring:** Signal-processing-rf `antenna-beam-pattern`; ml-training `Gaussian-attention-kernel`; agentic-reasoning `decaying-context-weight`.
**Notes:** Downing & Pinker (1985); LaBerge & Brown (1989).

---

## Section 2 — Memory Systems

### sensory-memory (cross-domain alias: `sensory-register`, `pre-categorical-store`)
**Domain:** Cognitive Primitives
**Definition:** Brief, high-capacity buffer holding raw perceptual input for <1 s before fading or being transferred to working memory; modality-specific (iconic, echoic, haptic).
**Atom or composite:** Atom — short-lived `hash` of the sensory surface.
**Cost model:** High capacity (~thousands of items) but decays in 250 ms (iconic) to 4 s (echoic).
**Real wall?** Yes — decay rate is physiological; cannot be voluntarily extended.
**Cross-domain wiring:** Ordo-runtime `ring-buffer`; ml-training `input-cache`; agentic-reasoning `raw-token-buffer`.
**Notes:** Sperling (1960) partial-report; Neisser (1967) coined the term.

### iconic-memory-sperling (cross-domain alias: `visual-sensory-store`, `Sperling-store`)
**Domain:** Cognitive Primitives
**Definition:** Visual sensory store holding a brief (~250 ms) high-resolution snapshot of the visual field; Sperling's partial-report paradigm revealed its capacity.
**Atom or composite:** Atom — full-field `hash` of visual array, briefly preserved.
**Cost model:** Capacity ~9–12 items but decay ~250 ms.
**Real wall?** Yes — decay enforced by retinal/cortical afterimage kinetics.
**Cross-domain wiring:** Ordo-runtime `frame-buffer`; ml-training `visual-feature-cache`; signal-processing-rf `transient-recorder`.
**Notes:** Sperling (1960) Psych Monographs; gold-standard partial-report demonstration.

### echoic-memory (cross-domain alias: `auditory-sensory-store`, `Darwin-Turvey-Crowder`)
**Domain:** Cognitive Primitives
**Definition:** Auditory sensory store holding raw sound for ~3–4 s; longer than iconic to accommodate temporally-extended speech parsing.
**Atom or composite:** Atom — short-window `hash` of acoustic stream.
**Cost model:** Capacity ~5 items, duration 3–4 s.
**Real wall?** Yes — limited duration; speech parsing must beat it.
**Cross-domain wiring:** Signal-processing-rf `audio-ring-buffer`; ml-training `acoustic-context-window`; agentic-reasoning `streaming-audio-cache`.
**Notes:** Darwin, Turvey & Crowder (1972) three-tone analog of Sperling.

### persistence-of-vision (cross-domain alias: `visual-afterimage`, `phosphor-persistence`)
**Domain:** Cognitive Primitives
**Definition:** Phenomenon where an image is retained on the retina/cortex for ~10–250 ms after stimulus offset; underlies motion picture perception.
**Atom or composite:** Atom — `scale`-decaying activation trace.
**Cost model:** Decay ~25–60 ms; longer for brighter stimuli.
**Real wall?** Yes — photoreceptor and cortical decay kinetics.
**Cross-domain wiring:** Ml-training `temporal-smoothing`; signal-processing-rf `integration-time`; ordo-runtime `frame-blend`.
**Notes:** Coltheart (1980) review; substrate for iconic memory.

### working-memory-baddeley (cross-domain alias: `Baddeley-WM`, `multi-component-WM`)
**Domain:** Cognitive Primitives
**Definition:** Multi-component model of active maintenance (Baddeley & Hitch 1974) comprising central executive plus phonological loop, visuospatial sketchpad, and episodic buffer (added 2000).
**Atom or composite:** Composite — `combine`(loop, sketchpad, buffer) under executive control.
**Cost model:** ~4 chunks effective capacity; rehearsal extends loop ~2 s.
**Real wall?** Yes — empirical capacity limit; Miller's 7±2 / Cowan's 4±1.
**Cross-domain wiring:** Agentic-reasoning `scratchpad`; ordo-runtime `register-file`; ml-training `context-window`.
**Notes:** Baddeley & Hitch (1974); Baddeley (2000) episodic buffer addition.

### phonological-loop (cross-domain alias: `articulatory-loop`, `verbal-WM-subsystem`)
**Domain:** Cognitive Primitives
**Definition:** Component of Baddeley's WM specialized for verbal/acoustic information; comprises a passive phonological store and an active articulatory rehearsal process.
**Atom or composite:** Composite — `scan`(store) + active `combine`(rehearsal).
**Cost model:** ~2 s span of speech rehearsable; word-length effect.
**Real wall?** Yes — articulatory rate caps the loop; word-length effect proves it.
**Cross-domain wiring:** Ordo-runtime `streaming-buffer-with-replay`; agentic-reasoning `tokenized-prompt-replay`; ml-training `RNN-hidden-state`.
**Notes:** Baddeley (1986); word-length and phonological-similarity effects.

### visuospatial-sketchpad (cross-domain alias: `VSSP`, `visual-WM-subsystem`)
**Domain:** Cognitive Primitives
**Definition:** Baddeley's WM component for spatial and visual information; thought to comprise a visual cache plus an inner scribe for spatial sequences.
**Atom or composite:** Composite — `combine`(visual-cache, inner-scribe).
**Cost model:** ~3–4 visual chunks; spatial sequences slightly shorter.
**Real wall?** Yes — capacity limit independent from verbal loop (double dissociation).
**Cross-domain wiring:** Ml-training `2D-feature-map-cache`; ordo-runtime `tile-buffer`; agentic-reasoning `image-context-slot`.
**Notes:** Logie (1995); double-dissociation evidence from neuropsychology.

### central-executive (cross-domain alias: `WM-executive`, `attentional-controller`)
**Domain:** Cognitive Primitives
**Definition:** Supervisory component of WM that allocates attention, switches tasks, updates contents, and inhibits irrelevant material; closely tied to prefrontal cortex.
**Atom or composite:** Composite — `combine`(allocate, switch, update, inhibit).
**Cost model:** Severely capacity-limited; bottleneck on complex tasks.
**Real wall?** Yes — single-channel attentional resource; PFC damage impairs.
**Cross-domain wiring:** Ordo-runtime `scheduler`; decision-logic `controller`; agentic-reasoning `planner-orchestrator`.
**Notes:** Baddeley (1986); Miyake et al. (2000) — three EF functions (update, shift, inhibit).

### episodic-buffer-baddeley-2000 (cross-domain alias: `multimodal-WM-buffer`, `4th-WM-component`)
**Domain:** Cognitive Primitives
**Definition:** Multimodal WM store integrating verbal, visual, and LTM information into coherent episodes (added by Baddeley 2000 to plug binding gap).
**Atom or composite:** Composite — `combine`(loop, sketchpad, LTM) via executive control.
**Cost model:** ~4 multimodal chunks; binding is attention-demanding.
**Real wall?** Yes — explicit binding requires attention; reduced under load.
**Cross-domain wiring:** Agentic-reasoning `multimodal-context-slot`; ml-training `cross-modal-fusion`; ordo-runtime `composite-event`.
**Notes:** Baddeley (2000) TICS; the binding-problem patch to the 1974 model.

### cowan-focus-of-attention (cross-domain alias: `embedded-process-model`, `Cowan-WM`)
**Domain:** Cognitive Primitives
**Definition:** WM as an embedded subset of LTM in heightened activation, with a narrower focus of attention holding ~4 chunks (Cowan 2001).
**Atom or composite:** Composite — `project`(activated-LTM, focus-window).
**Cost model:** ~4-chunk focus; activated region larger but not consciously held.
**Real wall?** Yes — 4±1 capacity is meta-analytically robust (Cowan 2001).
**Cross-domain wiring:** Agentic-reasoning `KV-cache-hot-set`; ml-training `top-k-attention`; ordo-runtime `LRU-cache-hot-tier`.
**Notes:** Cowan (2001) BBS; "Magical number 4" revision of Miller.

### n-back (cross-domain alias: `n-back-task`, `Kirchner-task`)
**Domain:** Cognitive Primitives
**Definition:** WM task in which subjects judge whether the current stimulus matches one presented n positions earlier; load increases with n.
**Atom or composite:** Composite — `scan` + `compare`(now, t−n) + update.
**Cost model:** Accuracy falls sharply n>3; widely used neuroimaging probe.
**Real wall?** Yes — capacity caps performance; n=4 near-ceiling for most adults.
**Cross-domain wiring:** Ordo-runtime `lagged-buffer-compare`; agentic-reasoning `recall-from-history`; ml-training `causal-lookback`.
**Notes:** Kirchner (1958); workhorse fMRI WM paradigm.

### complex-span-tasks (cross-domain alias: `reading-span`, `operation-span`)
**Domain:** Cognitive Primitives
**Definition:** WM tasks interleaving memoranda with processing operations (reading sentences, solving equations); measure of WM capacity under dual demand.
**Atom or composite:** Composite — interleaved `combine`(store, process).
**Cost model:** Span typically 3–5 items; predicts higher-order cognition.
**Real wall?** Yes — strong individual-difference predictor of fluid intelligence.
**Cross-domain wiring:** Decision-logic `interleaved-task-test`; ml-training `multi-task-eval`; agentic-reasoning `nested-tool-call-benchmark`.
**Notes:** Daneman & Carpenter (1980); Engle et al. (1999).

### short-term-memory (cross-domain alias: `STM`, `primary-memory`)
**Domain:** Cognitive Primitives
**Definition:** Brief retention of information without active processing; historically distinguished from WM by passivity (though boundary blurs).
**Atom or composite:** Atom — temporary `hash` of recent items.
**Cost model:** ~7±2 items (Miller 1956) or 4±1 chunks (Cowan); 15–30 s without rehearsal.
**Real wall?** Yes — duration and capacity both limited.
**Cross-domain wiring:** Ordo-runtime `tlb-cache`; agentic-reasoning `recent-tokens`; ml-training `recurrent-state`.
**Notes:** Miller (1956); Atkinson & Shiffrin (1968) multistore model.

### long-term-memory (cross-domain alias: `LTM`, `secondary-memory`)
**Domain:** Cognitive Primitives
**Definition:** Effectively unlimited-capacity store of consolidated information over minutes to decades; subdivided into declarative (episodic, semantic) and non-declarative.
**Atom or composite:** Composite — `combine`(declarative, procedural, conditioning, priming).
**Cost model:** Huge capacity; retrieval is cue-dependent and often imperfect.
**Real wall?** No (capacity); yes (retrieval reliability).
**Cross-domain wiring:** Agentic-reasoning `vector-database`; ml-training `model-parameters-as-memory`; ordo-runtime `persistent-store`.
**Notes:** Atkinson & Shiffrin (1968); Squire (1992) taxonomy.

### episodic-memory-tulving (cross-domain alias: `autobiographical-episodes`, `event-memory`)
**Domain:** Cognitive Primitives
**Definition:** Memory for time-and-place-stamped events; supports autonoetic awareness (mental time travel). Tulving (1972) distinguished it from semantic memory.
**Atom or composite:** Composite — `combine`(what, where, when, self).
**Cost model:** High specificity; subject to interference and reconstruction.
**Real wall?** Yes — hippocampus required; H.M. case shows total loss without it.
**Cross-domain wiring:** Agentic-reasoning `episodic-replay`; ml-training `experience-replay-buffer`; ordo-runtime `event-log`.
**Notes:** Tulving (1972, 1983, 2002); foundational distinction in memory taxonomy.

### semantic-memory (cross-domain alias: `world-knowledge`, `conceptual-memory`)
**Domain:** Cognitive Primitives
**Definition:** Memory for general facts, concepts, and meanings divorced from learning context; relies on neocortex (especially anterior temporal lobe).
**Atom or composite:** Composite — `combine`(concept-nodes, relations).
**Cost model:** Vast capacity; activation spreads through associative network.
**Real wall?** Yes — semantic dementia from ATL damage shows specific loss.
**Cross-domain wiring:** Ml-training `LLM-pretrained-weights`; agentic-reasoning `knowledge-base`; ordo-runtime `static-config`.
**Notes:** Tulving (1972); Patterson, Nestor & Rogers (2007) hub-and-spoke.

### procedural-memory (cross-domain alias: `skill-memory`, `motor-memory`)
**Domain:** Cognitive Primitives
**Definition:** Memory for skills and procedures expressed through performance rather than recollection; depends on basal ganglia and cerebellum.
**Atom or composite:** Atom — compiled `project`(input→action).
**Cost model:** Slow to acquire (massed practice); resistant to forgetting once consolidated.
**Real wall?** Yes — explicit retrieval impossible; only enacted.
**Cross-domain wiring:** Ml-training `compiled-policy`; agentic-reasoning `tool-skill-finetune`; ordo-runtime `JIT-compiled-handler`.
**Notes:** Squire (1992); riding-a-bike phenomenology; spared in amnesia.

### declarative-vs-nondeclarative (cross-domain alias: `explicit-vs-implicit-memory`, `Squire-taxonomy`)
**Domain:** Cognitive Primitives
**Definition:** Top-level memory split: declarative (explicit, consciously accessible: episodic + semantic) vs non-declarative (implicit: procedural, priming, conditioning, habits).
**Atom or composite:** Composite — taxonomic `compare` between conscious-accessible classes.
**Cost model:** Different neural substrates; double dissociation in amnesia.
**Real wall?** Yes — H.M. shows declarative loss with intact procedural.
**Cross-domain wiring:** Ml-training `weights-vs-data`; agentic-reasoning `retrieval-vs-finetuning`; decision-logic `policy-vs-history`.
**Notes:** Squire (1992); the canonical LTM taxonomy.

### perceptual-representation-system (cross-domain alias: `PRS`, `pre-semantic-perceptual-memory`)
**Domain:** Cognitive Primitives
**Definition:** Non-declarative store supporting structural/form-based priming; pre-semantic; e.g., word-form system supporting repetition priming.
**Atom or composite:** Atom — form-level `hash` enabling fast recognition.
**Cost model:** Cheap recognition; modality-specific.
**Real wall?** Yes — Tulving & Schacter (1990) dissociated PRS from semantic.
**Cross-domain wiring:** Ml-training `low-level-feature-cache`; ordo-runtime `bloom-filter-hit`; agentic-reasoning `surface-form-cache`.
**Notes:** Tulving & Schacter (1990); explains form-but-not-meaning priming.

### conditioning-memory (cross-domain alias: `classical-conditioning-trace`, `associative-memory-pavlovian`)
**Domain:** Cognitive Primitives
**Definition:** Non-declarative memory for stimulus-stimulus and stimulus-response contingencies; mediated by cerebellum (eyeblink) and amygdala (fear).
**Atom or composite:** Atom — Hebbian `combine`(CS, US) → associative weight.
**Cost model:** Many trials needed; persistent once learned.
**Real wall?** Yes — substrate-specific (cerebellum vs amygdala); not declarative.
**Cross-domain wiring:** Ml-training `associative-weights`; decision-logic `cue-conditioned-response`; agentic-reasoning `trigger-action-mapping`.
**Notes:** Pavlov (1927); Thompson (1986) cerebellar locus.

### priming (cross-domain alias: `repetition-priming`, `prime-target-facilitation`)
**Domain:** Cognitive Primitives
**Definition:** Implicit facilitation of processing for a stimulus by prior exposure to a related stimulus; multiple subtypes (perceptual, semantic, repetition).
**Atom or composite:** Atom — residual activation `scale`-up.
**Cost model:** Effect 20–100 ms; can persist hours-to-days for perceptual priming.
**Real wall?** Yes — empirically robust; spared in amnesia.
**Cross-domain wiring:** Ml-training `KV-cache-hit`; agentic-reasoning `retrieval-recency-boost`; ordo-runtime `hot-path-cache`.
**Notes:** Tulving & Schacter (1990); Schacter (1992) review.

### prospective-memory (cross-domain alias: `intention-memory`, `remember-to-do`)
**Domain:** Cognitive Primitives
**Definition:** Memory for intentions to perform an action at a future time or in response to a cue; distinct from retrospective recall.
**Atom or composite:** Composite — `combine`(intention, cue-monitor, future-context).
**Cost model:** Monitoring cost ongoing; failure rate ~10–30% in daily life.
**Real wall?** Yes — capacity limited; competing tasks crowd out monitoring.
**Cross-domain wiring:** Decision-logic `scheduled-task`; ordo-runtime `cron-job`; agentic-reasoning `deferred-action-queue`.
**Notes:** Einstein & McDaniel (1990); Smith (2003) PAM theory.

### time-based-vs-event-based-prospective (cross-domain alias: `time-based-PM`, `event-based-PM`)
**Domain:** Cognitive Primitives
**Definition:** Time-based PM (act at 3 pm) requires self-initiated monitoring; event-based PM (act when X happens) relies on cue detection.
**Atom or composite:** Composite — alternative `scan`(clock) vs `scan`(events).
**Cost model:** Time-based more demanding; event-based depends on cue salience.
**Real wall?** Yes — without externally salient cues, time-based depends on internal clock.
**Cross-domain wiring:** Ordo-runtime `timer-vs-event-trigger`; decision-logic `polling-vs-interrupt`; agentic-reasoning `time-fence-vs-event-hook`.
**Notes:** Einstein & McDaniel (1990); core PM dichotomy.

### recognition-memory (cross-domain alias: `recognition`, `old-new-judgment`)
**Domain:** Cognitive Primitives
**Definition:** Memory test in which subjects judge whether a stimulus was previously presented; supported by familiarity and recollection components.
**Atom or composite:** Composite — `combine`(familiarity-signal, recollection-signal).
**Cost model:** Faster and easier than recall; sensitivity measured via d′.
**Real wall?** Yes — dual-process theory: familiarity and recollection dissociable.
**Cross-domain wiring:** Ml-training `binary-classifier`; agentic-reasoning `seen-before-flag`; ordo-runtime `cache-hit-vs-miss`.
**Notes:** Yonelinas (2002); Mandler (1980) two-process model.

### recall-vs-recognition (cross-domain alias: `production-vs-selection-memory`, `generate-vs-recognize`)
**Domain:** Cognitive Primitives
**Definition:** Recall requires self-generated production of items; recognition requires only discrimination of old from new. Recognition usually easier.
**Atom or composite:** Composite — `project`(cue→content) (recall) vs `compare`(probe, store) (recognition).
**Cost model:** Recall harder by ~20–50% accuracy; longer RT.
**Real wall?** Yes — recall demands more on retrieval mechanisms.
**Cross-domain wiring:** Ml-training `generation-vs-classification`; agentic-reasoning `open-query-vs-multiple-choice`; decision-logic `synthesize-vs-select`.
**Notes:** Mandler (1980); fundamental memory-testing distinction.

### free-recall (cross-domain alias: `unstructured-recall`, `list-recall`)
**Domain:** Cognitive Primitives
**Definition:** Recall task with no order constraints; subject reports any remembered items. Reveals serial-position curve and clustering.
**Atom or composite:** Composite — repeated `project`(any-cue→item) without order.
**Cost model:** Recall declines from list-middle; primacy and recency saved.
**Real wall?** Yes — capacity limit; ~7 items typical from 20-item lists.
**Cross-domain wiring:** Agentic-reasoning `unordered-listing-task`; ml-training `set-prediction`; decision-logic `unsorted-enumerate`.
**Notes:** Murdock (1962); the classic serial-position paradigm.

### cued-recall (cross-domain alias: `cue-driven-recall`, `paired-associate`)
**Domain:** Cognitive Primitives
**Definition:** Recall in which an external cue (e.g., paired associate) helps retrieve the target; superior to free recall by reducing search.
**Atom or composite:** Composite — `project`(cue→target) via associative link.
**Cost model:** ~30–50% better than free recall depending on cue effectiveness.
**Real wall?** Yes — encoding-specificity bounds cue effectiveness.
**Cross-domain wiring:** Agentic-reasoning `retrieval-with-context`; ml-training `conditional-generation`; ordo-runtime `key-lookup`.
**Notes:** Tulving & Pearlstone (1966); evidence for retrieval-cue dependence.

### serial-recall (cross-domain alias: `ordered-recall`, `digit-span`)
**Domain:** Cognitive Primitives
**Definition:** Recall task requiring reproduction of items in original order; sensitive to order errors and reveals positional uncertainty.
**Atom or composite:** Composite — sequential `order`(items) + report.
**Cost model:** Order encoding extra cost; digit span ~7±2.
**Real wall?** Yes — order capacity less than item capacity.
**Cross-domain wiring:** Ml-training `seq2seq-decoding`; agentic-reasoning `sequence-replay`; ordo-runtime `FIFO-replay`.
**Notes:** Brown (1958); Peterson & Peterson (1959) for STM decay.

### recency-effect (cross-domain alias: `last-items-advantage`, `STM-recency`)
**Domain:** Cognitive Primitives
**Definition:** Superior recall of the last few items in a list, attributed to short-term/working memory holding the tail; eliminated by distractor task.
**Atom or composite:** Atom — STM-contents bonus on tail items.
**Cost model:** Boost ~30–50% on last ~3 items; vanishes with 30 s delay.
**Real wall?** Yes — STM-based component is fragile to interference.
**Cross-domain wiring:** Ml-training `recency-bias`; agentic-reasoning `last-context-priority`; ordo-runtime `LIFO-cache`.
**Notes:** Glanzer & Cunitz (1966); the standard STM/LTM dissociation evidence.

### primacy-effect (cross-domain alias: `first-items-advantage`, `LTM-primacy`)
**Domain:** Cognitive Primitives
**Definition:** Superior recall of the first few items in a list, attributed to greater rehearsal and LTM transfer; survives delay unlike recency.
**Atom or composite:** Atom — LTM-rehearsal bonus on head items.
**Cost model:** Boost ~20% on first ~3 items; survives distractor-filled delay.
**Real wall?** Yes — LTM-based component robust.
**Cross-domain wiring:** Ml-training `early-token-bias`; agentic-reasoning `system-prompt-priority`; ordo-runtime `cold-cache-stickiness`.
**Notes:** Glanzer & Cunitz (1966); Rundus (1971) rehearsal account.

### serial-position-curve (cross-domain alias: `U-shaped-recall-curve`, `primacy-recency-curve`)
**Domain:** Cognitive Primitives
**Definition:** Characteristic U-shaped function relating recall probability to list position: high at start (primacy), low in middle, high at end (recency).
**Atom or composite:** Composite — `combine`(primacy, recency) over position.
**Cost model:** Middle items worst; predictable from dual-store model.
**Real wall?** Yes — dual mechanism; both endpoints empirically robust.
**Cross-domain wiring:** Ml-training `position-encoding-bias-curve`; agentic-reasoning `lost-in-the-middle`; ordo-runtime `cache-edge-locality`.
**Notes:** Murdock (1962); foundational evidence for the multistore model.

### encoding-specificity-tulving-thomson (cross-domain alias: `encoding-specificity-principle`, `ESP`)
**Domain:** Cognitive Primitives
**Definition:** Memory retrieval is best when retrieval cues match encoding context; cue-target effectiveness depends on what was encoded together.
**Atom or composite:** Atom — match `compare`(encoded-context, retrieval-context).
**Cost model:** Up to 2× improvement under matched conditions.
**Real wall?** Yes — empirically replicated repeatedly.
**Cross-domain wiring:** Ml-training `query-key-similarity`; agentic-reasoning `prompt-context-matching`; ordo-runtime `cache-lookup-key-match`.
**Notes:** Tulving & Thomson (1973); foundational retrieval theory.

### state-dependent-memory (cross-domain alias: `state-dependent-retrieval`, `mood-congruent-recall`)
**Domain:** Cognitive Primitives
**Definition:** Recall is best when internal physiological/emotional state at retrieval matches that at encoding (e.g., alcohol-state, mood-state).
**Atom or composite:** Atom — state-vector `compare`(encode, retrieve).
**Cost model:** Effect sizes modest but reliable; mood-congruent ~5–15% boost.
**Real wall?** Yes — state functions as part of the retrieval cue.
**Cross-domain wiring:** Ml-training `domain-shift-degradation`; agentic-reasoning `context-shift-degradation`; ordo-runtime `runtime-mode-affinity`.
**Notes:** Goodwin et al. (1969) alcohol; Bower (1981) mood-state.

### context-dependent-memory (cross-domain alias: `environmental-context-effect`, `Godden-Baddeley`)
**Domain:** Cognitive Primitives
**Definition:** Recall is best when external context at retrieval matches that at encoding; classic underwater-vs-land diving study (Godden & Baddeley 1975).
**Atom or composite:** Atom — context-vector `compare`(encode-env, retrieve-env).
**Cost model:** Effects ~10–30%; eliminated by mental reinstatement.
**Real wall?** Yes — context binds into encoded trace.
**Cross-domain wiring:** Ml-training `train-test-distribution-match`; agentic-reasoning `context-window-shift-penalty`; ordo-runtime `environment-parity`.
**Notes:** Godden & Baddeley (1975); Smith & Vela (2001) meta-analysis.

### source-memory (cross-domain alias: `source-recollection`, `where-when-how-memory`)
**Domain:** Cognitive Primitives
**Definition:** Memory for the origin of information (who said it, where seen, when learned); often dissociated from item recognition.
**Atom or composite:** Composite — `combine`(item-trace, source-tag).
**Cost model:** Source recall worse than item recall; PFC-dependent.
**Real wall?** Yes — frontal lobe damage impairs source while sparing item.
**Cross-domain wiring:** Agentic-reasoning `citation-tracking`; ml-training `provenance-attribution`; ordo-runtime `event-source-id`.
**Notes:** Johnson, Hashtroudi & Lindsay (1993) source-monitoring framework.

### source-monitoring (cross-domain alias: `reality-monitoring`, `source-attribution`)
**Domain:** Cognitive Primitives
**Definition:** Decision process attributing memory contents to internal or external sources; failures produce false memories and confabulation.
**Atom or composite:** Composite — `compare`(internal-features, external-features) → attribute.
**Cost model:** Source confusion ~10–30% on similar sources.
**Real wall?** Yes — features overlap; perfect monitoring impossible.
**Cross-domain wiring:** Agentic-reasoning `hallucination-detection`; ml-training `data-vs-prior-attribution`; decision-logic `provenance-check`.
**Notes:** Johnson et al. (1993); Mitchell & Johnson (2009) update.

### false-memory-drm-paradigm (cross-domain alias: `DRM`, `Deese-Roediger-McDermott`)
**Domain:** Cognitive Primitives
**Definition:** Robust laboratory false-memory paradigm: studying lists of semantic associates (e.g., "bed, rest, awake") produces false recall of the critical lure ("sleep").
**Atom or composite:** Atom — gist-based `combine` produces non-presented item activation.
**Cost model:** False recall ~40–60% for critical lure; high subjective confidence.
**Real wall?** Yes — gist-based memory inherently susceptible.
**Cross-domain wiring:** Ml-training `LLM-confabulation`; agentic-reasoning `hallucinated-detail`; decision-logic `inference-mistaken-for-observation`.
**Notes:** Roediger & McDermott (1995); Deese (1959) original.

### forgetting-curve-ebbinghaus (cross-domain alias: `Ebbinghaus-curve`, `retention-decay`)
**Domain:** Cognitive Primitives
**Definition:** Empirical curve showing exponential-like decay of retention over time without rehearsal; first systematic memory study (Ebbinghaus 1885).
**Atom or composite:** Atom — temporal `scale`-decay of trace strength.
**Cost model:** Steepest in first hour; flattens beyond ~1 day.
**Real wall?** Yes — passive decay (and/or interference) is universal.
**Cross-domain wiring:** Ml-training `catastrophic-forgetting-curve`; ordo-runtime `cache-eviction-curve`; agentic-reasoning `context-window-decay`.
**Notes:** Ebbinghaus (1885); replicated extensively by Murre & Dros (2015).

### retroactive-interference (cross-domain alias: `RI`, `new-overwrites-old`)
**Domain:** Cognitive Primitives
**Definition:** Newly learned material interferes with retrieval of previously learned material; demonstrates that forgetting isn't only decay but also interference.
**Atom or composite:** Atom — `combine`(new-trace) competes with old-trace at retrieval.
**Cost model:** Stronger for similar material; can be massive (e.g., paired-associate AB then AC).
**Real wall?** Yes — empirically robust; not eliminable.
**Cross-domain wiring:** Ml-training `catastrophic-forgetting`; agentic-reasoning `context-overwrite`; ordo-runtime `key-collision`.
**Notes:** McGeoch (1932); foundational evidence against pure-decay theory.

### proactive-interference (cross-domain alias: `PI`, `old-blocks-new`)
**Domain:** Cognitive Primitives
**Definition:** Previously learned material interferes with retrieval of newly learned material; builds up across trials and can be released by category shift.
**Atom or composite:** Atom — prior-trace `combine` competes with new-trace.
**Cost model:** PI build-up reduces recall ~30–50%; release-from-PI restores.
**Real wall?** Yes — empirically robust across paradigms.
**Cross-domain wiring:** Ml-training `prior-bias-overwhelming-data`; agentic-reasoning `stale-context-leakage`; ordo-runtime `cache-staleness-poisoning`.
**Notes:** Wickens (1970) release-from-PI; Underwood (1957) classic.

### decay-theory (cross-domain alias: `passive-forgetting`, `trace-decay`)
**Domain:** Cognitive Primitives
**Definition:** Hypothesis that memory traces fade over time absent rehearsal/reactivation; contrasts with interference theory; both probably operate.
**Atom or composite:** Atom — temporal `scale`-down of trace.
**Cost model:** Pure decay difficult to demonstrate without confound; some evidence in STM.
**Real wall?** Partially — interference accounts for much "decay"; some genuine decay exists.
**Cross-domain wiring:** Ordo-runtime `TTL-expiry`; ml-training `weight-norm-decay`; agentic-reasoning `context-eviction-by-age`.
**Notes:** Brown (1958); contested by McGeoch (1932); modern hybrid view.

### interference-theory (cross-domain alias: `interference-forgetting`, `competition-theory`)
**Domain:** Cognitive Primitives
**Definition:** Forgetting due to competition among traces at retrieval rather than passive decay; accommodates both proactive and retroactive interference.
**Atom or composite:** Composite — `compare`(competing-traces) at retrieval.
**Cost model:** Predicts forgetting as a function of similar-trace count.
**Real wall?** Yes — empirically dominant account of LTM forgetting.
**Cross-domain wiring:** Ml-training `intra-class-confusion`; agentic-reasoning `competing-context-blocking`; decision-logic `arbitration-cost`.
**Notes:** McGeoch (1932); Anderson (1983) ACT-R.

### levels-of-processing-craik-lockhart (cross-domain alias: `LOP`, `depth-of-processing`)
**Domain:** Cognitive Primitives
**Definition:** Memory strength depends on depth of encoding processing (shallow physical features vs deep semantic analysis); semantic processing yields better retention.
**Atom or composite:** Atom — encoding-depth `scale` factor on trace strength.
**Cost model:** Deeper processing slower but ~2× retention.
**Real wall?** Yes — robust empirical effect; semantic > phonological > orthographic.
**Cross-domain wiring:** Ml-training `feature-richness`; agentic-reasoning `semantic-vs-surface-encoding`; decision-logic `enrichment-pass`.
**Notes:** Craik & Lockhart (1972) JVLVB; Craik & Tulving (1975).

### elaborative-encoding (cross-domain alias: `elaboration`, `meaningful-encoding`)
**Domain:** Cognitive Primitives
**Definition:** Encoding strategy linking new information to existing knowledge via examples, analogies, mental imagery; produces durable memory.
**Atom or composite:** Composite — `combine`(new-trace, related-LTM-anchors).
**Cost model:** Encoding slower; retention 2–4× better.
**Real wall?** Yes — depth-of-processing principle empirically supported.
**Cross-domain wiring:** Ml-training `data-augmentation-with-context`; agentic-reasoning `retrieval-augmented-encoding`; decision-logic `enrichment-stage`.
**Notes:** Craik & Tulving (1975); foundational principle for study skills.

### spacing-effect (cross-domain alias: `distributed-practice-effect`, `spaced-repetition`)
**Domain:** Cognitive Primitives
**Definition:** Distributed practice (spaced over time) produces better long-term retention than massed practice (cramming); robust across material and ages.
**Atom or composite:** Atom — `scale` factor by inter-repetition lag.
**Cost model:** 2–3× better long-term retention than massed; optimal lag depends on retention interval.
**Real wall?** Yes — empirically robust meta-analytic effect.
**Cross-domain wiring:** Ml-training `curriculum-spacing`; agentic-reasoning `spaced-rehearsal-schedule`; decision-logic `review-scheduling`.
**Notes:** Ebbinghaus (1885); Cepeda et al. (2006) meta-analysis.

### testing-effect-roediger-karpicke (cross-domain alias: `retrieval-practice`, `test-enhanced-learning`)
**Domain:** Cognitive Primitives
**Definition:** Active retrieval (testing) produces better long-term retention than equivalent time spent restudying; retrieval is itself learning.
**Atom or composite:** Atom — retrieval `project` strengthens trace.
**Cost model:** Retrieval slower than restudy; ~50% better delayed recall.
**Real wall?** Yes — empirically robust; effect grows with delay.
**Cross-domain wiring:** Ml-training `self-distillation`; agentic-reasoning `practice-via-self-query`; decision-logic `active-rehearsal`.
**Notes:** Roediger & Karpicke (2006); huge effect across domains.

### consolidation (cross-domain alias: `memory-consolidation`, `trace-stabilization`)
**Domain:** Cognitive Primitives
**Definition:** Process by which fresh memory traces become stable and resistant to disruption, occurring over minutes (cellular) to years (systems).
**Atom or composite:** Composite — temporal `combine`(synaptic-stabilization, cortical-distribution).
**Cost model:** Hours-to-years; protein-synthesis-dependent.
**Real wall?** Yes — disruption during window erases memory (electroconvulsive shock evidence).
**Cross-domain wiring:** Ml-training `gradient-update-finalization`; ordo-runtime `commit-to-persistent-store`; agentic-reasoning `memory-store-write`.
**Notes:** McGaugh (2000); Squire (1992) systems consolidation.

### reconsolidation (cross-domain alias: `memory-update`, `retrieval-induced-reconsolidation`)
**Domain:** Cognitive Primitives
**Definition:** When a consolidated memory is retrieved, it becomes labile again and must be re-stabilized; opens a window for modification or erasure.
**Atom or composite:** Composite — `project`(retrieve) → labile state → `combine`(update).
**Cost model:** Reconsolidation window ~6 h; protein-synthesis-dependent.
**Real wall?** Yes — retrieved memories are temporarily editable.
**Cross-domain wiring:** Ml-training `online-update`; agentic-reasoning `retrieved-memory-edit`; ordo-runtime `read-modify-write`.
**Notes:** Nader, Schafe & LeDoux (2000); revolutionary finding for memory editing.

### sleep-dependent-consolidation (cross-domain alias: `offline-consolidation`, `sleep-replay`)
**Domain:** Cognitive Primitives
**Definition:** Sleep-dependent strengthening and reorganization of memory traces; hippocampal replay during slow-wave sleep transfers content to cortex.
**Atom or composite:** Composite — `combine`(SWS-replay, REM-integration).
**Cost model:** Hours of sleep; selective for important / emotional content.
**Real wall?** Yes — sleep deprivation impairs consolidation empirically.
**Cross-domain wiring:** Ml-training `offline-replay`; agentic-reasoning `background-consolidation-pass`; ordo-runtime `nightly-batch-job`.
**Notes:** Stickgold (2005); Wilson & McNaughton (1994) place-cell replay.

### schema-bartlett (cross-domain alias: `cognitive-schema`, `frame-knowledge`)
**Domain:** Cognitive Primitives
**Definition:** Organized knowledge structure that guides encoding and recall; produces systematic distortions toward schema-consistent content (Bartlett 1932).
**Atom or composite:** Composite — `combine`(input, schema-slots).
**Cost model:** Schema-consistent items recalled better; inconsistent items distorted.
**Real wall?** Yes — schema-driven distortion is inherent; cannot be eliminated.
**Cross-domain wiring:** Ml-training `prior-knowledge-bias`; agentic-reasoning `template-driven-extraction`; decision-logic `default-frame`.
**Notes:** Bartlett (1932) "War of the Ghosts"; Brewer & Treyens (1981) office study.

### script-schank-abelson (cross-domain alias: `event-script`, `stereotyped-event-sequence`)
**Domain:** Cognitive Primitives
**Definition:** Specialized schema for stereotyped event sequences (e.g., restaurant script); enables inference of unmentioned steps.
**Atom or composite:** Composite — ordered `combine`(scene-frames).
**Cost model:** Scripts auto-fill gaps; produce intrusion errors on script-consistent items.
**Real wall?** Yes — schema-driven inference unavoidable.
**Cross-domain wiring:** Agentic-reasoning `workflow-template`; ml-training `event-prediction-prior`; decision-logic `procedure-template`.
**Notes:** Schank & Abelson (1977); foundational AI-cognitive-science contact point.

### engram-tonegawa (cross-domain alias: `memory-engram`, `engram-cell`)
**Domain:** Cognitive Primitives
**Definition:** Physical/neural substrate of a specific memory; modern optogenetic studies (Tonegawa lab) identify and manipulate ensembles of engram cells.
**Atom or composite:** Atom — sparse ensemble `hash` instantiating one memory.
**Cost model:** ~1–5% of hippocampal neurons per memory; reactivation triggers recall.
**Real wall?** Yes — physical substrate; can be deleted, reactivated, or false-implanted.
**Cross-domain wiring:** Ml-training `sparse-prototype-cell`; ordo-runtime `addressed-memory-cell`; agentic-reasoning `memory-id-handle`.
**Notes:** Tonegawa et al. (2015) Nature review; Semon (1904) original concept.

---

## Section 3 — Metacognition & Self-Awareness

### metacognitive-monitoring (cross-domain alias: `meta-monitoring`, `knowledge-of-knowing`)
**Domain:** Cognitive Primitives
**Definition:** Process of tracking one's own cognitive states (e.g., comprehension, confidence, progress) during ongoing task performance; the "monitoring" half of Nelson-Narens model.
**Atom or composite:** Atom — `project`(object-level state → meta-level signal).
**Cost model:** Continuous lightweight readout; sensitivity limited.
**Real wall?** Yes — finite introspective resolution; calibration imperfect.
**Cross-domain wiring:** Agentic-reasoning `self-evaluation`; decision-logic `confidence-score`; ml-training `predictive-uncertainty`.
**Notes:** Nelson & Narens (1990); foundational metacognition framework.

### metacognitive-control (cross-domain alias: `meta-control`, `strategy-regulation`)
**Domain:** Cognitive Primitives
**Definition:** Use of metacognitive monitoring outputs to regulate cognition (e.g., allocate study time, switch strategy); the "control" half of Nelson-Narens.
**Atom or composite:** Composite — `combine`(monitor-signal, action-policy).
**Cost model:** Modulates downstream cognition; cheap conditional branch.
**Real wall?** Yes — control is only as good as monitoring is.
**Cross-domain wiring:** Decision-logic `policy-override`; agentic-reasoning `replan-on-low-confidence`; ml-training `early-exit`.
**Notes:** Nelson & Narens (1990); empirical work on study-time allocation.

### feeling-of-knowing (cross-domain alias: `FOK`, `meta-recognition-judgment`)
**Domain:** Cognitive Primitives
**Definition:** Prospective judgment that one will be able to recognize or recall an item even when current retrieval fails; predicts subsequent recognition above chance.
**Atom or composite:** Atom — `project`(partial-trace → meta-confidence).
**Cost model:** Cheap; based on cue familiarity and partial retrieval.
**Real wall?** Yes — accurate but imperfect; Koriat (1993) accessibility account.
**Cross-domain wiring:** Ml-training `softmax-margin`; agentic-reasoning `near-miss-confidence`; ordo-runtime `cache-near-miss-signal`.
**Notes:** Hart (1965); Koriat (1993) accessibility model.

### tip-of-the-tongue (cross-domain alias: `TOT`, `near-retrieval-state`)
**Domain:** Cognitive Primitives
**Definition:** Strong feeling that a known item is almost retrievable, often with partial information (first letter, syllable count); a special case of FOK.
**Atom or composite:** Atom — partial-trace activation signaling near-recall.
**Cost model:** Lasts seconds to minutes; resolution often spontaneous.
**Real wall?** Yes — universal phenomenon; demonstrates fractional access.
**Cross-domain wiring:** Ml-training `partial-decoding-stall`; agentic-reasoning `token-completion-stall`; ordo-runtime `partial-key-lookup`.
**Notes:** Brown & McNeill (1966); the classic TOT investigation.

### judgment-of-learning (cross-domain alias: `JOL`, `study-time-prediction`)
**Domain:** Cognitive Primitives
**Definition:** Prediction during/after study that one will recall an item on a future test; often poorly calibrated, susceptible to fluency illusions.
**Atom or composite:** Atom — `project`(study-fluency → future-recall-estimate).
**Cost model:** Cheap; tends to be optimistic.
**Real wall?** Yes — fluency-based cue is partly misleading.
**Cross-domain wiring:** Ml-training `validation-loss-estimate`; agentic-reasoning `self-rated-mastery`; decision-logic `est-time-remaining`.
**Notes:** Nelson & Dunlosky (1991); Koriat (1997) cue-utilization.

### retrospective-confidence (cross-domain alias: `posthoc-confidence`, `after-action-confidence`)
**Domain:** Cognitive Primitives
**Definition:** Confidence rating given after a response; reflects integration of evidence and signal-detection sensitivity; better calibrated than prospective ratings.
**Atom or composite:** Atom — `project`(post-decision-state → confidence-scalar).
**Cost model:** Cheap; aligned with d′ when balanced.
**Real wall?** Yes — confidence is inherently a posterior estimate.
**Cross-domain wiring:** Ml-training `softmax-probability`; agentic-reasoning `answer-confidence-score`; decision-logic `posterior-decision-quality`.
**Notes:** Fleming & Lau (2014); meta-d′ measure.

### prospective-confidence (cross-domain alias: `pre-task-confidence`, `forward-confidence`)
**Domain:** Cognitive Primitives
**Definition:** Confidence rating given before attempting a task; tends to be poorly calibrated and biased toward optimism.
**Atom or composite:** Atom — `project`(self-assessed-ability → prior-confidence).
**Cost model:** Cheap to elicit; less accurate than retrospective.
**Real wall?** Yes — without doing the task, info is limited.
**Cross-domain wiring:** Ml-training `entropy-of-prior`; agentic-reasoning `plan-confidence`; decision-logic `risk-estimate-prior`.
**Notes:** Koriat (2012); calibration generally worse pre-task.

### calibration (cross-domain alias: `metacognitive-calibration`, `confidence-accuracy-match`)
**Domain:** Cognitive Primitives
**Definition:** Match between subjective probability/confidence and objective frequency of correctness; perfect calibration: 80% confidence → 80% correct.
**Atom or composite:** Atom — `compare`(confidence-curve, accuracy-curve).
**Cost model:** Measured across many trials; visualized as reliability diagram.
**Real wall?** Yes — humans systematically miscalibrate (overconfidence).
**Cross-domain wiring:** Ml-training `expected-calibration-error`; agentic-reasoning `trust-but-verify`; statistics-probability `reliability-diagram`.
**Notes:** Lichtenstein, Fischhoff & Phillips (1982); huge literature on miscalibration.

### overconfidence-bias (cross-domain alias: `overconfidence-effect`, `excessive-confidence`)
**Domain:** Cognitive Primitives
**Definition:** Systematic tendency to give higher confidence than accuracy warrants; pervasive across domains; manifests as overprecision, overestimation, and overplacement.
**Atom or composite:** Atom — positive bias on confidence `project`.
**Cost model:** Costly in calibration; not eliminated by feedback.
**Real wall?** Yes — robust across cultures and expertise.
**Cross-domain wiring:** Ml-training `model-overconfidence`; agentic-reasoning `unjustified-certainty`; decision-logic `risk-underestimation`.
**Notes:** Moore & Healy (2008); the three faces of overconfidence.

### hard-easy-effect (cross-domain alias: `difficulty-calibration-bias`, `regressive-calibration`)
**Domain:** Cognitive Primitives
**Definition:** People are overconfident on hard items and underconfident on easy items; calibration curve has shallower slope than ideal.
**Atom or composite:** Atom — regression-to-mean in `project`(true-difficulty → confidence).
**Cost model:** Systematic across calibration studies.
**Real wall?** Yes — replicable in many domains; partly a regression artifact.
**Cross-domain wiring:** Ml-training `temperature-too-low-or-high`; statistics-probability `regression-to-mean`; agentic-reasoning `difficulty-blind-confidence`.
**Notes:** Lichtenstein & Fischhoff (1977); explained partly by Moore & Healy (2008).

### dunning-kruger (cross-domain alias: `DK-effect`, `unskilled-and-unaware`)
**Domain:** Cognitive Primitives
**Definition:** Tendency for low performers to overestimate ability and high performers to slightly underestimate it; partly a statistical artifact of regression.
**Atom or composite:** Atom — quartile-based bias in self-assessment.
**Cost model:** Empirically observed across many domains.
**Real wall?** Partially — replicable, but partly regression-to-mean.
**Cross-domain wiring:** Ml-training `weak-model-overconfidence`; agentic-reasoning `low-skill-high-claim`; decision-logic `expertise-blindness`.
**Notes:** Kruger & Dunning (1999); critiqued by Nuhfer et al. (2017).

### knew-it-all-along (cross-domain alias: `meta-hindsight`, `hindsight-at-meta-level`)
**Domain:** Cognitive Primitives
**Definition:** Meta-level hindsight bias: after learning the answer, people overestimate prior probability they would have given the correct answer.
**Atom or composite:** Atom — retrospective bias in `project`(prior-state).
**Cost model:** Pervasive; difficult to eliminate.
**Real wall?** Yes — robust empirically; mechanisms partly automatic.
**Cross-domain wiring:** Ml-training `post-hoc-rationalization`; agentic-reasoning `confirmation-after-feedback`; decision-logic `hindsight-evaluation-bias`.
**Notes:** Fischhoff (1975); Hawkins & Hastie (1990) review.

### metamemory (cross-domain alias: `memory-about-memory`, `meta-memory-judgments`)
**Domain:** Cognitive Primitives
**Definition:** Subdomain of metacognition: knowledge about and monitoring of one's own memory, including FOK, JOL, source confidence, and capacity estimates.
**Atom or composite:** Composite — `combine`(FOK, JOL, source-conf, span-estimate).
**Cost model:** Multi-component; each subprocess cheap.
**Real wall?** Yes — calibration limited; some monitoring impossible (cannot know what you don't know).
**Cross-domain wiring:** Agentic-reasoning `retrieval-confidence`; ml-training `model-card-uncertainty`; decision-logic `memory-reliability-estimate`.
**Notes:** Flavell (1979); Nelson & Narens (1990) framework.

### theory-of-mind-about-self (cross-domain alias: `meta-self`, `reflective-ToM`)
**Domain:** Cognitive Primitives
**Definition:** Application of theory-of-mind machinery to model one's own beliefs, desires, and intentions; shares neural substrate (medial PFC) with ToM about others.
**Atom or composite:** Composite — `project`(other-modeling-machinery → self-target).
**Cost model:** Same machinery as social ToM; recruits mPFC.
**Real wall?** Yes — limited introspective access; partly confabulatory.
**Cross-domain wiring:** Agentic-reasoning `self-modeling`; ml-training `learned-self-state-estimator`; decision-logic `internal-state-estimate`.
**Notes:** Frith & Frith (2003); Carruthers (2009) on self-as-other.

### introspection (cross-domain alias: `self-observation`, `looking-inward`)
**Domain:** Cognitive Primitives
**Definition:** Direct attention to one's own mental contents; classically a Wundtian method; modern view: limited and partly confabulatory (Nisbett & Wilson 1977).
**Atom or composite:** Atom — `project`(internal-state → reportable-content).
**Cost model:** Cheap; resolution low; report often post-hoc.
**Real wall?** Yes — much cognition is opaque to introspection.
**Cross-domain wiring:** Ml-training `interpretability-limits`; agentic-reasoning `chain-of-thought-confabulation`; decision-logic `audit-trail-incompleteness`.
**Notes:** Nisbett & Wilson (1977) "Telling more than we can know"; foundational critique.

### mental-time-travel (cross-domain alias: `chronesthesia`, `temporal-self-projection`)
**Domain:** Cognitive Primitives
**Definition:** Capacity to mentally project oneself into past (episodic recall) or future (prospection); core feature of autonoetic awareness (Tulving).
**Atom or composite:** Composite — `project`(self → past-or-future).
**Cost model:** Resource-demanding; DMN/hippocampus-dependent.
**Real wall?** Yes — hippocampal damage impairs both past and future thinking.
**Cross-domain wiring:** Agentic-reasoning `rollout-into-future`; ml-training `world-model-rollout`; decision-logic `simulation-of-outcomes`.
**Notes:** Tulving (2002); Schacter, Addis & Buckner (2007) constructive memory.

### autonoetic-awareness-tulving (cross-domain alias: `self-knowing-consciousness`, `chronesthesia`)
**Domain:** Cognitive Primitives
**Definition:** Awareness that a remembered or imagined event is part of one's own subjective time; the experiential signature of episodic memory and mental time travel.
**Atom or composite:** Atom — self+time-tag on retrieved content.
**Cost model:** Phenomenal feature; minimal additional compute.
**Real wall?** Yes — diminished or absent in some amnesias and developmental disorders.
**Cross-domain wiring:** Agentic-reasoning `provenance-of-experience`; ml-training `self-attribution-token`; decision-logic `subjective-source-tag`.
**Notes:** Tulving (1985) "How many memory systems are there?".

### reflective-consciousness (cross-domain alias: `higher-order-awareness`, `meta-awareness`)
**Domain:** Cognitive Primitives
**Definition:** Awareness of one's own conscious states (e.g., awareness of awareness); supports introspective report and metacognitive control.
**Atom or composite:** Composite — `project`(consciousness-state → higher-order-state).
**Cost model:** Adds an extra representational layer; PFC-dependent.
**Real wall?** Yes — capacity-limited; absent in some states (flow, dreams).
**Cross-domain wiring:** Ml-training `meta-token-prediction`; agentic-reasoning `self-monitor-loop`; ordo-runtime `kernel-introspection`.
**Notes:** Rosenthal (1997) higher-order thought; Lau & Rosenthal (2011).

### global-workspace-baars (cross-domain alias: `GW`, `global-broadcasting-theory`)
**Domain:** Cognitive Primitives
**Definition:** Consciousness as a global broadcast across specialized processors; the "workspace" makes selected content available to all subsystems (Baars 1988).
**Atom or composite:** Composite — `combine`(many-modules) via shared broadcast bus.
**Cost model:** Single broadcast at a time; serial bottleneck on conscious access.
**Real wall?** Yes — serial bottleneck is empirically robust (PRP, attentional blink).
**Cross-domain wiring:** Ordo-runtime `event-bus`; agentic-reasoning `shared-scratchpad`; ml-training `cross-module-attention`.
**Notes:** Baars (1988); Dehaene & Naccache (2001) global neuronal workspace.

### higher-order-thought-theory-rosenthal (cross-domain alias: `HOT`, `meta-representation-theory`)
**Domain:** Cognitive Primitives
**Definition:** Theory that a mental state is conscious iff there is a higher-order thought representing it; locates consciousness in meta-representation.
**Atom or composite:** Composite — `project`(first-order-state → second-order-representation).
**Cost model:** Requires extra representational layer.
**Real wall?** Yes (theoretical) — empirical signatures debated.
**Cross-domain wiring:** Agentic-reasoning `meta-token-pointer`; ml-training `self-attention-on-own-output`; ordo-runtime `reflective-handle`.
**Notes:** Rosenthal (1997, 2005); contested but influential consciousness theory.

### error-related-negativity (cross-domain alias: `ERN`, `Ne-component`)
**Domain:** Cognitive Primitives
**Definition:** Negative-going EEG deflection (~100 ms post-error) originating in ACC; reflects rapid, automatic error detection prior to conscious awareness.
**Atom or composite:** Atom — automatic `compare`(intended, actual) → error signal.
**Cost model:** ~100 ms latency; biologically free.
**Real wall?** Yes — ACC-mediated; amplitude modulated by error significance.
**Cross-domain wiring:** Decision-logic `error-detection`; agentic-reasoning `output-validation`; ml-training `prediction-error-signal`.
**Notes:** Falkenstein et al. (1990); Gehring et al. (1993).

### error-detection (cross-domain alias: `error-monitoring`, `mismatch-detection`)
**Domain:** Cognitive Primitives
**Definition:** Process of recognizing that an action or response deviated from the intended outcome; supported by ACC and dopaminergic prediction-error signals.
**Atom or composite:** Atom — `compare`(intended, observed) → mismatch.
**Cost model:** Fast (~100 ms ERN); cheap.
**Real wall?** Yes — empirically robust ACC-mediated process.
**Cross-domain wiring:** Decision-logic `validator`; ordo-runtime `assertion-check`; agentic-reasoning `output-self-check`.
**Notes:** Botvinick et al. (2001); foundational conflict-monitoring paper.

### error-correction (cross-domain alias: `post-error-adjustment`, `correction-response`)
**Domain:** Cognitive Primitives
**Definition:** Behavioral and cognitive adjustments following error detection: slowing, strategy shift, output revision; mediated by control loop from ACC to lateral PFC.
**Atom or composite:** Composite — `combine`(error-signal, control-update).
**Cost model:** RT slowing ~50–100 ms on next trial; cognitive overhead.
**Real wall?** Yes — correction takes time; cannot undo all errors.
**Cross-domain wiring:** Decision-logic `retry-with-adjustment`; ml-training `gradient-based-update`; ordo-runtime `error-handler-with-backoff`.
**Notes:** Rabbitt (1966); post-error slowing literature.

### post-error-slowing (cross-domain alias: `PES`, `error-induced-caution`)
**Domain:** Cognitive Primitives
**Definition:** Lengthened RT on the trial following an error; signature of automatic strategic adjustment toward caution.
**Atom or composite:** Atom — `scale`-up of caution parameter for next trial.
**Cost model:** ~50–100 ms slowing on N+1.
**Real wall?** Yes — robust empirical finding.
**Cross-domain wiring:** Decision-logic `back-off-after-failure`; ordo-runtime `exponential-backoff`; ml-training `learning-rate-reduction`.
**Notes:** Rabbitt (1966); Dutilh et al. (2012) DDM account.

### conflict-monitoring-botvinick-cohen (cross-domain alias: `ACC-conflict-monitoring`, `conflict-detection-theory`)
**Domain:** Cognitive Primitives
**Definition:** Theory that ACC monitors response conflict and signals lateral PFC to increase top-down control on subsequent trials; explains Stroop adaptation.
**Atom or composite:** Composite — `compare`(competing-responses) → conflict → `scale`-up control.
**Cost model:** Conflict-detection cheap; control adjustment costs trials of adaptation.
**Real wall?** Yes — conflict adaptation empirically robust.
**Cross-domain wiring:** Decision-logic `arbitration-with-feedback`; ml-training `confidence-weighted-update`; agentic-reasoning `disagreement-trigger`.
**Notes:** Botvinick, Braver, Barch, Carter & Cohen (2001).

### cognitive-control (cross-domain alias: `executive-control`, `EF-control`)
**Domain:** Cognitive Primitives
**Definition:** Capacity to flexibly configure cognition to achieve internal goals against habit or distraction; encompasses inhibition, updating, and shifting.
**Atom or composite:** Composite — `combine`(inhibit, update, shift).
**Cost model:** Effortful; capacity-limited; PFC-dependent.
**Real wall?** Yes — limited PFC capacity; declines with fatigue.
**Cross-domain wiring:** Decision-logic `policy-controller`; agentic-reasoning `meta-controller`; ordo-runtime `scheduler-with-priority`.
**Notes:** Miller & Cohen (2001); Miyake et al. (2000) three-component model.

### cognitive-load-sweller (cross-domain alias: `mental-load`, `CLT`)
**Domain:** Cognitive Primitives
**Definition:** Total demand on working memory during a task; partitioned into intrinsic (material complexity), extraneous (presentation), and germane (schema-building) load.
**Atom or composite:** Composite — `combine`(intrinsic, extraneous, germane).
**Cost model:** Sum cannot exceed WM capacity; overflow causes performance collapse.
**Real wall?** Yes — empirical capacity limit.
**Cross-domain wiring:** Agentic-reasoning `context-budget-load`; decision-logic `complexity-budget`; ml-training `task-difficulty-metric`.
**Notes:** Sweller (1988, 2010); foundational instructional-design theory.

### intrinsic-load (cross-domain alias: `inherent-complexity`, `material-complexity-load`)
**Domain:** Cognitive Primitives
**Definition:** Load due to the material's inherent complexity (element interactivity); cannot be reduced without changing the task.
**Atom or composite:** Atom — task-side `scale` of WM demand.
**Cost model:** Fixed for a given task and learner; reduced by expertise (chunking).
**Real wall?** Yes — task-inherent; only schema acquisition reduces it.
**Cross-domain wiring:** Ml-training `task-difficulty-index`; agentic-reasoning `irreducible-context-cost`; decision-logic `task-complexity-base`.
**Notes:** Sweller, Ayres & Kalyuga (2011).

### extraneous-load (cross-domain alias: `presentation-load`, `wasteful-load`)
**Domain:** Cognitive Primitives
**Definition:** Load due to suboptimal presentation/instruction that adds no learning value; minimization is a major design goal.
**Atom or composite:** Atom — design-side `scale` of WM waste.
**Cost model:** Reducible via better design (worked examples, integrated diagrams).
**Real wall?** No — eliminable in principle by better design.
**Cross-domain wiring:** Agentic-reasoning `prompt-bloat`; ml-training `inefficient-tokenization`; decision-logic `interface-friction`.
**Notes:** Sweller (1994); foundational to multimedia learning principles.

### germane-load (cross-domain alias: `learning-load`, `schema-construction-load`)
**Domain:** Cognitive Primitives
**Definition:** Load directly devoted to schema construction and automation; "useful" effortful processing distinct from extraneous load.
**Atom or composite:** Atom — productive `combine`(elements → schema).
**Cost model:** Worth incurring; produces lasting learning.
**Real wall?** Soft — limited by remaining WM budget.
**Cross-domain wiring:** Ml-training `useful-gradient-signal`; agentic-reasoning `productive-context-spend`; decision-logic `value-adding-overhead`.
**Notes:** Sweller (1994, 2010 update); controversial in later writings.

### mental-effort (cross-domain alias: `subjective-effort`, `task-load-rating`)
**Domain:** Cognitive Primitives
**Definition:** Subjective intensity of cognitive engagement; correlates with pupil dilation, ACC activity, and NASA-TLX ratings; effortful tasks felt as costly.
**Atom or composite:** Atom — scalar `project`(neural-activation → felt-effort).
**Cost model:** Effort is felt as cost; minimization is a basic principle.
**Real wall?** Yes — effort-based choice (Westbrook & Braver 2015) shows real opportunity cost.
**Cross-domain wiring:** Decision-logic `effort-cost-term`; agentic-reasoning `compute-budget-spend`; ml-training `compute-aware-policy`.
**Notes:** Kahneman (1973); Shenhav et al. (2017) expected value of control.

### mental-fatigue (cross-domain alias: `cognitive-fatigue`, `task-fatigue`)
**Domain:** Cognitive Primitives
**Definition:** Subjective and performance state following prolonged demanding cognition; characterized by reduced engagement, slower RTs, and disengagement.
**Atom or composite:** Atom — temporal `scale`-down of engagement.
**Cost model:** Builds over ~30–120 min; recovers with rest/change of task.
**Real wall?** Yes — robust performance decrements.
**Cross-domain wiring:** Ordo-runtime `worker-burn-in`; decision-logic `cooldown-required`; ml-training `online-learning-rate-decay`.
**Notes:** Kurzban et al. (2013) opportunity-cost model.

### flow-state-csikszentmihalyi (cross-domain alias: `flow`, `optimal-experience`)
**Domain:** Cognitive Primitives
**Definition:** Phenomenal state of complete absorption in a task with skill-challenge balance, loss of self-consciousness, and intrinsic reward (Csikszentmihalyi 1990).
**Atom or composite:** Composite — `combine`(skill-challenge-match, intrinsic-reward).
**Cost model:** Highly productive; reduced metacognitive overhead.
**Real wall?** Yes — requires skill-challenge balance; fragile to interruption.
**Cross-domain wiring:** Decision-logic `auto-pilot-zone`; agentic-reasoning `low-self-monitor-mode`; ml-training `optimal-learning-zone`.
**Notes:** Csikszentmihalyi (1990); positive psychology cornerstone.

### self-regulation-vohs-baumeister (cross-domain alias: `self-control-regulation`, `volitional-regulation`)
**Domain:** Cognitive Primitives
**Definition:** Capacity to override impulses and align behavior with longer-term goals; involves goal-setting, monitoring, and effortful inhibition.
**Atom or composite:** Composite — `combine`(monitor, inhibit, adjust).
**Cost model:** Effortful; resource model contested (ego depletion replications).
**Real wall?** Yes — empirical individual differences predict outcomes.
**Cross-domain wiring:** Decision-logic `policy-override`; agentic-reasoning `self-correction-loop`; ml-training `reward-shaping-against-impulse`.
**Notes:** Baumeister, Vohs & Tice (2007); replication crisis update (Hagger et al. 2016).

### ego-depletion-contested (cross-domain alias: `willpower-depletion`, `self-control-resource`)
**Domain:** Cognitive Primitives
**Definition:** Hypothesis that self-control draws on a limited resource that depletes with use; original effect failed large replication (Hagger et al. 2016).
**Atom or composite:** Atom — `scale`-down of inhibitory capacity over time.
**Cost model:** Original ~d=0.6 effect; multilab replication ~d=0.04.
**Real wall?** Contested — original strong claim not supported by large replications.
**Cross-domain wiring:** Decision-logic `degrading-policy-quality`; agentic-reasoning `budget-depletion-effect`; ml-training `online-fatigue-effect`.
**Notes:** Baumeister et al. (1998); Hagger et al. (2016) failed multilab.

### regulatory-focus (cross-domain alias: `promotion-prevention-focus`, `Higgins-focus`)
**Domain:** Cognitive Primitives
**Definition:** Distinction between promotion focus (pursuit of gains, ideals) and prevention focus (avoidance of losses, oughts); shapes goal-pursuit strategies.
**Atom or composite:** Atom — orientation `project` modulating value function.
**Cost model:** Stable trait + context-driven shifts.
**Real wall?** Yes — empirical individual differences.
**Cross-domain wiring:** Decision-logic `risk-orientation-mode`; ml-training `reward-vs-penalty-objective`; agentic-reasoning `safety-vs-exploration-bias`.
**Notes:** Higgins (1997, 1998); regulatory-focus theory.

### self-monitoring-snyder (cross-domain alias: `Snyder-self-monitoring`, `social-self-presentation`)
**Domain:** Cognitive Primitives
**Definition:** Trait reflecting attention to and regulation of self-presentation in social contexts; high self-monitors adjust behavior to audience cues.
**Atom or composite:** Composite — `combine`(monitor-audience, adjust-self).
**Cost model:** Continuous monitoring cost; trait-stable.
**Real wall?** Yes — individual-difference trait; replicable.
**Cross-domain wiring:** Agentic-reasoning `audience-adaptive-output`; decision-logic `social-context-modulation`; ml-training `RLHF-style-tuning`.
**Notes:** Snyder (1974); foundational personality construct.

---

## Section 4 — Learning & Plasticity

### hebbian-learning (cross-domain alias: `cells-that-fire-together`, `correlative-plasticity`)
**Domain:** Cognitive Primitives
**Definition:** Synaptic strengthening when pre- and post-synaptic neurons fire together; "fire together, wire together" (Hebb 1949). Foundational learning rule of cortex.
**Atom or composite:** Atom — `combine`(pre-activity, post-activity) → Δw.
**Cost model:** Local update O(1) per synapse; biologically free.
**Real wall?** Yes — unconstrained Hebbian leads to runaway; needs normalization.
**Cross-domain wiring:** Ml-training `outer-product-weight-update`; ordo-runtime `coactivation-edge-strengthening`; agentic-reasoning `co-occurrence-association`.
**Notes:** Hebb (1949) — The Organization of Behavior; the founding learning rule.

### stdp (cross-domain alias: `spike-timing-dependent-plasticity`, `temporal-Hebb`)
**Domain:** Cognitive Primitives
**Definition:** Refinement of Hebbian rule in which synaptic change depends on relative spike timing within ~20 ms: pre-before-post strengthens, post-before-pre weakens.
**Atom or composite:** Atom — timing-window `compare`(t_pre, t_post) → Δw.
**Cost model:** Asymmetric time window; per-spike updates.
**Real wall?** Yes — biological asymmetry; respects causality.
**Cross-domain wiring:** Ml-training `temporal-credit-assignment`; signal-processing-rf `pulse-timing-correlation`; ordo-runtime `causal-event-edge`.
**Notes:** Markram et al. (1997); Bi & Poo (1998).

### ltp (cross-domain alias: `long-term-potentiation`, `Bliss-Lomo`)
**Domain:** Cognitive Primitives
**Definition:** Persistent strengthening of synapses following brief high-frequency stimulation; cellular basis of memory; NMDA-receptor dependent at most synapses.
**Atom or composite:** Atom — `scale`-up of synaptic weight, persistent over hours-days.
**Cost model:** Induction expensive (theta-burst); maintenance via protein synthesis.
**Real wall?** Yes — saturation and homeostatic regulation prevent runaway.
**Cross-domain wiring:** Ml-training `gradient-based-weight-update`; ordo-runtime `learned-edge-weight`; agentic-reasoning `strengthened-association`.
**Notes:** Bliss & Lømo (1973); Bear & Malenka (1994) NMDA mechanism.

### ltd (cross-domain alias: `long-term-depression`, `synaptic-weakening`)
**Domain:** Cognitive Primitives
**Definition:** Persistent weakening of synapses following low-frequency stimulation; complement to LTP enabling bidirectional plasticity and renormalization.
**Atom or composite:** Atom — `scale`-down of synaptic weight.
**Cost model:** Induced by sustained low-frequency input.
**Real wall?** Yes — required for net plasticity homeostasis.
**Cross-domain wiring:** Ml-training `weight-decay`; ordo-runtime `edge-pruning`; agentic-reasoning `forgetting-unused-association`.
**Notes:** Dudek & Bear (1992); cerebellar LTD (Ito 1989).

### classical-conditioning-pavlov (cross-domain alias: `Pavlovian-conditioning`, `respondent-conditioning`)
**Domain:** Cognitive Primitives
**Definition:** Learning that a previously neutral stimulus (CS) predicts a biologically significant stimulus (US), eliciting a conditioned response (CR).
**Atom or composite:** Composite — `combine`(CS, US) repeated → CS triggers CR.
**Cost model:** Trials needed; speed depends on biological relevance.
**Real wall?** Yes — requires consistent CS→US contingency; weakens with random pairings.
**Cross-domain wiring:** Ml-training `supervised-association`; agentic-reasoning `cue-action-mapping`; decision-logic `trigger-handler`.
**Notes:** Pavlov (1927); foundational associative-learning paradigm.

### rescorla-wagner-model (cross-domain alias: `RW-model`, `error-driven-conditioning`)
**Domain:** Cognitive Primitives
**Definition:** Quantitative model of classical conditioning: ΔV = αβ(λ − ΣV); learning proportional to prediction error; explains blocking and overshadowing.
**Atom or composite:** Atom — `compare`(predicted, actual) → Δassociation-strength.
**Cost model:** Trial-by-trial scalar update.
**Real wall?** Yes — predictions empirically confirmed; basis for modern RL.
**Cross-domain wiring:** Ml-training `td-learning-precursor`; agentic-reasoning `prediction-error-based-update`; statistics-probability `online-MLE-like`.
**Notes:** Rescorla & Wagner (1972); ancestor of TD-learning.

### blocking-effect (cross-domain alias: `Kamin-blocking`, `prediction-error-blocking`)
**Domain:** Cognitive Primitives
**Definition:** When a CS already predicts a US, a second CS added in compound fails to gain associative strength; predicts learning requires prediction error.
**Atom or composite:** Atom — zero `prediction-error` blocks weight update.
**Cost model:** Trials with compound CS produce no learning for added cue.
**Real wall?** Yes — robust empirical finding driving RW model.
**Cross-domain wiring:** Ml-training `redundant-feature-suppression`; agentic-reasoning `no-new-info-no-update`; decision-logic `information-gain-gating`.
**Notes:** Kamin (1969); foundational for prediction-error theories.

### overshadowing (cross-domain alias: `cue-competition`, `salience-overshadowing`)
**Domain:** Cognitive Primitives
**Definition:** When two CSs are presented in compound, the more salient one gains more associative strength; less salient is "overshadowed."
**Atom or composite:** Atom — `compare`(salience) determines weight share.
**Cost model:** Compound conditioning yields salience-weighted distribution.
**Real wall?** Yes — robust empirical phenomenon.
**Cross-domain wiring:** Ml-training `feature-salience-bias`; agentic-reasoning `dominant-cue-attribution`; statistics-probability `feature-weight-by-variance`.
**Notes:** Pavlov (1927); Mackintosh (1976) attentional account.

### latent-inhibition (cross-domain alias: `pre-exposure-effect`, `non-reinforced-pre-exposure`)
**Domain:** Cognitive Primitives
**Definition:** Pre-exposure to a stimulus without consequences slows later conditioning to it; learner has learned to ignore the irrelevant cue.
**Atom or composite:** Atom — `scale`-down of attention to pre-exposed stimulus.
**Cost model:** Many pre-exposure trials; effect persists.
**Real wall?** Yes — robust effect across species.
**Cross-domain wiring:** Ml-training `informative-feature-filtering`; agentic-reasoning `irrelevant-context-suppression`; decision-logic `noise-pretraining`.
**Notes:** Lubow & Moore (1959); a foundation of attention-learning interaction.

### operant-conditioning-skinner (cross-domain alias: `instrumental-learning`, `Skinnerian-conditioning`)
**Domain:** Cognitive Primitives
**Definition:** Learning that behavior produces consequences; future behavior frequency changes based on reinforcement and punishment contingencies.
**Atom or composite:** Composite — `combine`(behavior, consequence) → behavior-frequency update.
**Cost model:** Trial-by-trial; reinforcement schedule matters.
**Real wall?** Yes — three-term contingency (SD, R, SR) is basic learning building block.
**Cross-domain wiring:** Ml-training `reinforcement-learning`; agentic-reasoning `reward-shaped-policy`; decision-logic `consequence-driven-update`.
**Notes:** Skinner (1938); the founding behaviorist paradigm.

### positive-reinforcement (cross-domain alias: `reward-presentation`, `Sr+`)
**Domain:** Cognitive Primitives
**Definition:** Adding a desirable consequence to increase behavior frequency; the most common reinforcement type.
**Atom or composite:** Atom — `scale`-up of behavior probability via reward.
**Cost model:** Effective with low cost per trial.
**Real wall?** Yes — reward must be biologically/cognitively valued.
**Cross-domain wiring:** Ml-training `positive-reward-signal`; agentic-reasoning `success-reinforced-policy`; decision-logic `goal-attainment-update`.
**Notes:** Skinner (1953); core operant principle.

### negative-reinforcement (cross-domain alias: `aversive-removal`, `Sr−`)
**Domain:** Cognitive Primitives
**Definition:** Removing an aversive stimulus to increase behavior frequency; not punishment — frequency increases.
**Atom or composite:** Atom — `scale`-up of behavior via aversive offset.
**Cost model:** Strong learner; basis of escape/avoidance learning.
**Real wall?** Yes — requires functional aversive present.
**Cross-domain wiring:** Ml-training `negative-cost-removal`; decision-logic `escape-policy`; agentic-reasoning `friction-reduction-reinforced`.
**Notes:** Skinner (1953); often confused with punishment.

### positive-punishment (cross-domain alias: `punishment-presentation`, `Sp+`)
**Domain:** Cognitive Primitives
**Definition:** Adding an aversive consequence to decrease behavior frequency; effective only with consistent immediate application.
**Atom or composite:** Atom — `scale`-down of behavior via aversive consequence.
**Cost model:** Reliable but with collateral effects (avoidance, fear).
**Real wall?** Yes — must be contingent and immediate.
**Cross-domain wiring:** Ml-training `loss-signal`; decision-logic `failure-penalty`; agentic-reasoning `mistake-penalty`.
**Notes:** Skinner (1953); Azrin & Holz (1966) parameters.

### negative-punishment (cross-domain alias: `response-cost`, `Sp−`)
**Domain:** Cognitive Primitives
**Definition:** Removing a desirable consequence to decrease behavior frequency (e.g., time-out, fine); often used in applied behavior analysis.
**Atom or composite:** Atom — `scale`-down of behavior via reward removal.
**Cost model:** Effective when reward is reliable.
**Real wall?** Yes — requires established reward to remove.
**Cross-domain wiring:** Ml-training `reward-withholding`; decision-logic `privilege-revocation`; agentic-reasoning `reward-clawback`.
**Notes:** Skinner (1953); foundational in behavioral therapy.

### schedules-of-reinforcement (cross-domain alias: `FR-VR-FI-VI`, `reinforcement-schedules`)
**Domain:** Cognitive Primitives
**Definition:** Patterns of contingency between behavior and reinforcer: Fixed-Ratio (every N), Variable-Ratio (avg N), Fixed-Interval (every t), Variable-Interval (avg t). Each yields characteristic response patterns.
**Atom or composite:** Atom — sampling rule for reinforcer delivery.
**Cost model:** VR most resistant to extinction; FI shows scalloping.
**Real wall?** Yes — schedule effects on rate/extinction are robust.
**Cross-domain wiring:** Ml-training `reward-schedule-design`; agentic-reasoning `intermittent-reward-tuning`; decision-logic `incentive-schedule`.
**Notes:** Ferster & Skinner (1957); foundational schedule taxonomy.

### matching-law-herrnstein (cross-domain alias: `Herrnstein-matching`, `relative-rate-matching`)
**Domain:** Cognitive Primitives
**Definition:** Animals distribute behavior across alternatives in proportion to relative reinforcement rates: B1/B2 = R1/R2; foundational for behavioral economics.
**Atom or composite:** Atom — `scale`-proportional allocation.
**Cost model:** Behavior tracks reinforcement ratio over thousands of trials.
**Real wall?** Yes — robust quasi-optimal allocation.
**Cross-domain wiring:** Decision-logic `proportional-resource-allocation`; agentic-reasoning `tool-use-probability-by-reward`; ml-training `softmax-exploration-by-Q`.
**Notes:** Herrnstein (1961); generalized matching law (Baum 1974).

### td-learning-sutton-barto (cross-domain alias: `TD-learning`, `temporal-difference`)
**Domain:** Cognitive Primitives
**Definition:** Learning rule that updates value estimates using a TD error δ = r + γV(s') − V(s); bridges Pavlovian and instrumental learning; cornerstone of modern RL.
**Atom or composite:** Atom — `compare`(predicted-V, observed-r+γV') → δ → weight update.
**Cost model:** Trial-by-trial scalar update; online.
**Real wall?** Yes — convergence requires sufficient exploration.
**Cross-domain wiring:** Ml-training `value-iteration`; agentic-reasoning `value-based-policy`; decision-logic `value-update-step`.
**Notes:** Sutton (1988); Sutton & Barto (1998/2018) textbook.

### reward-prediction-error (cross-domain alias: `RPE`, `delta-signal`)
**Domain:** Cognitive Primitives
**Definition:** Difference between actual and predicted reward; signal that drives learning in both biological (dopamine) and computational (TD-error) systems.
**Atom or composite:** Atom — `compare`(predicted-r, actual-r).
**Cost model:** Per-event scalar signal.
**Real wall?** Yes — robust empirical signal in dopaminergic neurons.
**Cross-domain wiring:** Ml-training `td-error`; agentic-reasoning `surprise-driven-update`; statistics-probability `Bayesian-update`.
**Notes:** Rescorla & Wagner (1972); Schultz, Dayan & Montague (1997).

### dopamine-rpe-schultz (cross-domain alias: `dopamine-prediction-error`, `Schultz-RPE`)
**Domain:** Cognitive Primitives
**Definition:** Phasic firing of midbrain dopamine neurons (VTA, SNc) encodes reward prediction error: bursts on positive surprise, dips on negative surprise.
**Atom or composite:** Atom — biological substrate of RPE signal.
**Cost model:** Brief phasic burst; broadcasts via D1/D2 receptors.
**Real wall?** Yes — robust electrophysiological signature.
**Cross-domain wiring:** Ml-training `td-error-broadcast`; ordo-runtime `system-wide-signal`; agentic-reasoning `reward-update-broadcast`.
**Notes:** Schultz, Dayan & Montague (1997) Science; landmark unification of biology and TD.

### eligibility-traces (cross-domain alias: `e-traces`, `trace-decay-credit`)
**Domain:** Cognitive Primitives
**Definition:** Decaying memory of recent state/action visits; allows credit assignment to events that preceded current TD error by multiple steps.
**Atom or composite:** Atom — temporal `scale`-decay of credit tag.
**Cost model:** O(states) per step; decays exponentially.
**Real wall?** Yes — limits effective credit-assignment horizon.
**Cross-domain wiring:** Ml-training `TD-λ`; ordo-runtime `event-trail-with-decay`; agentic-reasoning `recent-action-credit`.
**Notes:** Sutton (1988); foundational in TD(λ).

### eligibility-decay (cross-domain alias: `trace-decay-rate`, `lambda-parameter`)
**Domain:** Cognitive Primitives
**Definition:** Rate at which eligibility-trace credit fades for past states/actions; the λ in TD(λ) controlling the bias-variance tradeoff.
**Atom or composite:** Atom — exponential `scale`-decay factor.
**Cost model:** Fast decay = low-variance/high-bias; slow decay = high-variance/low-bias.
**Real wall?** Yes — tradeoff is fundamental, not eliminable.
**Cross-domain wiring:** Ml-training `lambda-hyperparam`; statistics-probability `bias-variance-tradeoff`; agentic-reasoning `context-decay-rate`.
**Notes:** Sutton & Barto (2018) ch. 12.

### model-based-learning (cross-domain alias: `cognitive-model-based`, `forward-search-RL`)
**Domain:** Cognitive Primitives
**Definition:** Learning that builds an explicit model of environment transitions and rewards, then plans via simulation; flexible but computationally expensive.
**Atom or composite:** Composite — `combine`(transition-model, reward-model, planner).
**Cost model:** Planning O(branching^horizon); model maintenance ongoing.
**Real wall?** Yes — flexibility costs compute; model errors compound.
**Cross-domain wiring:** Ml-training `model-based-RL`; agentic-reasoning `world-model-rollout`; decision-logic `planner-with-model`.
**Notes:** Daw, Niv & Dayan (2005); two-system theory.

### model-free-learning (cross-domain alias: `cached-value-learning`, `policy-gradient-style`)
**Domain:** Cognitive Primitives
**Definition:** Learning that directly updates value/policy estimates from experience without an explicit environment model; cheap at decision time but inflexible.
**Atom or composite:** Atom — cached `project`(state → value-or-action).
**Cost model:** Cheap at decision time; needs many trials to converge.
**Real wall?** Yes — inflexible to model changes (devaluation insensitivity).
**Cross-domain wiring:** Ml-training `Q-learning`; agentic-reasoning `cached-policy`; decision-logic `habit-execution`.
**Notes:** Daw et al. (2005); Dolan & Dayan (2013) habits vs goals.

### dual-system-rl (cross-domain alias: `arbitrated-RL`, `MB-MF-arbiter`)
**Domain:** Cognitive Primitives
**Definition:** Architecture in which model-based and model-free learners coexist; an arbiter weights their contributions based on reliability and computational budget.
**Atom or composite:** Composite — `combine`(MB-output, MF-output) by arbitration.
**Cost model:** Arbitration overhead; sum of both system costs.
**Real wall?** Yes — neither system alone explains behavior; both required.
**Cross-domain wiring:** Decision-logic `arbiter-of-fast-slow`; agentic-reasoning `cached-vs-planned`; ml-training `mixture-of-experts`.
**Notes:** Daw, Gershman, Seymour, Dayan & Dolan (2011).

### habit-vs-goal-directed-control-daw (cross-domain alias: `S-R-vs-S-O-R`, `habit-goal-arbiter`)
**Domain:** Cognitive Primitives
**Definition:** Distinction between stimulus-response habits (model-free, devaluation-insensitive) and goal-directed action (model-based, devaluation-sensitive).
**Atom or composite:** Composite — `compare`(arbitrate by reliability) habit-vs-goal.
**Cost model:** Habit fast/cheap; goal-directed slow/flexible.
**Real wall?** Yes — devaluation paradigm cleanly dissociates.
**Cross-domain wiring:** Ml-training `cached-policy-vs-search`; agentic-reasoning `tool-use-habit-vs-plan`; decision-logic `routine-vs-novel-mode`.
**Notes:** Daw, Niv & Dayan (2005); Dickinson (1985) devaluation.

### generalization-gradient (cross-domain alias: `stimulus-generalization`, `transfer-curve`)
**Domain:** Cognitive Primitives
**Definition:** Tendency for conditioned responses to spread to similar stimuli, with response strength decreasing as a function of stimulus distance from training value.
**Atom or composite:** Atom — similarity-modulated `scale` of response.
**Cost model:** Gradient typically monotonic; sharper after discrimination training.
**Real wall?** Yes — perceptual similarity is the basis; cannot eliminate.
**Cross-domain wiring:** Ml-training `kernel-similarity-prediction`; agentic-reasoning `near-cue-generalization`; statistics-probability `RBF-kernel`.
**Notes:** Guttman & Kalish (1956); Shepard (1987) universal law of generalization.

### discrimination-training (cross-domain alias: `differential-conditioning`, `S+S-training`)
**Domain:** Cognitive Primitives
**Definition:** Training in which one stimulus (S+) is reinforced and a similar stimulus (S−) is not; sharpens the generalization gradient.
**Atom or composite:** Composite — `compare`(S+, S−) trial pairs → narrowed gradient.
**Cost model:** Many trials; produces peak shift.
**Real wall?** Yes — discriminability limited by perceptual resolution.
**Cross-domain wiring:** Ml-training `contrastive-learning`; agentic-reasoning `positive-negative-examples`; decision-logic `boundary-tightening`.
**Notes:** Hanson (1959); foundational discrimination paradigm.

### peak-shift (cross-domain alias: `peak-shift-effect`, `Hanson-peak-shift`)
**Domain:** Cognitive Primitives
**Definition:** After discrimination training, the peak response shifts AWAY from S− beyond S+; demonstrates non-linear interactions in associative learning.
**Atom or composite:** Atom — `combine`(excitatory-gradient, inhibitory-gradient) → shifted peak.
**Cost model:** Emerges after discrimination training.
**Real wall?** Yes — robust phenomenon explained by gradient summation.
**Cross-domain wiring:** Ml-training `decision-boundary-shift`; agentic-reasoning `over-correction-from-near-negative`; decision-logic `boundary-overshoot`.
**Notes:** Hanson (1959); explained by Spence (1937) gradient theory.

### equivalence-classes (cross-domain alias: `stimulus-equivalence`, `equivalence-relation`)
**Domain:** Cognitive Primitives
**Definition:** Set of stimuli that become functionally equivalent following relational training; demonstrates reflexivity, symmetry, transitivity (Sidman 1971).
**Atom or composite:** Composite — `combine`(reflexive, symmetric, transitive) relations.
**Cost model:** Few direct trainings; many derived relations.
**Real wall?** Yes — requires verbal-cognitive capacity; uniquely strong in humans.
**Cross-domain wiring:** Ml-training `relational-generalization`; agentic-reasoning `transitive-inference`; logic-reasoning `equivalence-class`.
**Notes:** Sidman (1971, 1994); core of RFT (Hayes et al. 2001).

### stimulus-equivalence (cross-domain alias: `derived-equivalence`, `Sidman-equivalence`)
**Domain:** Cognitive Primitives
**Definition:** Demonstration that conditional discriminations spontaneously yield derived relations (symmetry, transitivity) without explicit training.
**Atom or composite:** Composite — derived `combine` from minimal direct training.
**Cost model:** Cheap derivation after baseline training.
**Real wall?** Yes — appears mostly in verbally competent organisms.
**Cross-domain wiring:** Ml-training `zero-shot-transfer`; agentic-reasoning `inferred-equivalence`; logic-reasoning `equivalence-derivation`.
**Notes:** Sidman (1971) foundational; basis for derived relational responding.

### one-shot-learning (cross-domain alias: `single-trial-learning`, `rapid-binding`)
**Domain:** Cognitive Primitives
**Definition:** Acquisition of a new association in a single exposure; characteristic of episodic memory, fear conditioning, and certain language phenomena (fast mapping).
**Atom or composite:** Atom — single-trial `combine` produces lasting trace.
**Cost model:** Trial cheap; requires hippocampus or amygdala depending on type.
**Real wall?** Yes — relies on rapid binding mechanism; not all material learnable in one trial.
**Cross-domain wiring:** Ml-training `few-shot-learning`; agentic-reasoning `single-example-update`; ordo-runtime `single-event-commit`.
**Notes:** Kandel (2001) Nobel lecture; Marr (1971) hippocampal theory.

### fast-mapping (cross-domain alias: `quick-word-learning`, `referent-mapping`)
**Domain:** Cognitive Primitives
**Definition:** Children's (and adults') ability to map a novel word to a novel referent after a single exposure; supports rapid vocabulary growth.
**Atom or composite:** Atom — `combine`(novel-form, novel-referent) → mapping.
**Cost model:** One trial sufficient; refinement over later exposures.
**Real wall?** Yes — relies on mutual-exclusivity bias; not absolute.
**Cross-domain wiring:** Ml-training `one-shot-classification`; agentic-reasoning `novel-token-binding`; decision-logic `name-to-referent-mapping`.
**Notes:** Carey & Bartlett (1978); foundational word-learning.

### transfer-learning-cognitive (cross-domain alias: `cognitive-transfer`, `transfer-of-training`)
**Domain:** Cognitive Primitives
**Definition:** Application of learning from one task to another; positive transfer when prior learning facilitates new; negative when it interferes.
**Atom or composite:** Atom — `project`(source-skill → target-skill).
**Cost model:** Depends on shared structure; can be positive, zero, or negative.
**Real wall?** Yes — far transfer is rare and small (Sala & Gobet meta-analyses).
**Cross-domain wiring:** Ml-training `transfer-learning`; agentic-reasoning `skill-reuse-across-domains`; decision-logic `policy-portability`.
**Notes:** Thorndike & Woodworth (1901); Barnett & Ceci (2002) taxonomy.

### far-transfer-vs-near-transfer (cross-domain alias: `transfer-distance`, `near-far-transfer`)
**Domain:** Cognitive Primitives
**Definition:** Near transfer: similar context and content (robust); far transfer: different context/content (rare). Distinction predicts training-program effectiveness.
**Atom or composite:** Atom — `scale`-distance metric on transfer.
**Cost model:** Near transfer reliable; far transfer needs deep abstraction.
**Real wall?** Yes — far transfer notoriously weak across cognitive training research.
**Cross-domain wiring:** Ml-training `OOD-generalization-gap`; agentic-reasoning `cross-domain-generalization`; decision-logic `domain-shift-cost`.
**Notes:** Barnett & Ceci (2002); Sala & Gobet (2017) meta-analyses.

### formal-discipline-doctrine-critique (cross-domain alias: `mental-faculties-doctrine`, `discredited-far-transfer`)
**Domain:** Cognitive Primitives
**Definition:** Historical view that training the mind on Latin/logic strengthens general thinking; experimentally rejected by Thorndike & Woodworth (1901).
**Atom or composite:** N/A — discredited claim about general transfer.
**Cost model:** Historically motivated extensive curricular choices.
**Real wall?** Yes — formal discipline is empirically falsified.
**Cross-domain wiring:** Ml-training `cautionary-tale-against-general-transfer`; agentic-reasoning `task-specific-vs-general-skills`.
**Notes:** Thorndike & Woodworth (1901) — foundational rejection.

### curriculum-learning-cognitive (cross-domain alias: `staged-instruction`, `progressive-difficulty`)
**Domain:** Cognitive Primitives
**Definition:** Instruction sequenced from simple to complex to scaffold understanding; aligns with ZPD (Vygotsky) and reduces cognitive load early.
**Atom or composite:** Composite — ordered `combine`(stages) by difficulty.
**Cost model:** Sequencing matters; poor curricula waste effort.
**Real wall?** Yes — prerequisite ordering limits possible sequences.
**Cross-domain wiring:** Ml-training `curriculum-learning`; agentic-reasoning `staged-tool-introduction`; decision-logic `phased-rollout`.
**Notes:** Bengio et al. (2009) ML adaptation; Vygotsky (1934) original.

### expertise-development-chase-simon (cross-domain alias: `Chase-Simon-expertise`, `chunk-based-expertise`)
**Domain:** Cognitive Primitives
**Definition:** Expertise grows by accumulating ~50,000+ domain-specific chunks; experts perceive structured patterns rather than discrete elements (Chase & Simon 1973 chess).
**Atom or composite:** Composite — large library of `hash`-keyed chunks.
**Cost model:** ~10 years deliberate practice; massive memory base.
**Real wall?** Yes — chunk accumulation takes time; cannot shortcut.
**Cross-domain wiring:** Ml-training `pretrained-feature-library`; agentic-reasoning `domain-specific-priors`; decision-logic `template-library`.
**Notes:** Chase & Simon (1973); Ericsson (1996) deliberate practice.

### deliberate-practice-ericsson (cross-domain alias: `DP`, `effortful-practice`)
**Domain:** Cognitive Primitives
**Definition:** Effortful, focused practice targeting specific weaknesses with feedback; distinct from mere repetition; predicts expert performance.
**Atom or composite:** Composite — `combine`(targeted-task, feedback, repetition).
**Cost model:** Effortful per session; ~4 hrs/day sustainable.
**Real wall?** Yes — limit on daily DP duration; not infinitely scalable.
**Cross-domain wiring:** Ml-training `curriculum-with-feedback`; agentic-reasoning `weakness-targeted-training`; decision-logic `improvement-loop`.
**Notes:** Ericsson, Krampe & Tesch-Römer (1993) Psych Rev.

### perceptual-learning-gibson (cross-domain alias: `differentiation-learning`, `Gibsonian-PL`)
**Domain:** Cognitive Primitives
**Definition:** Long-lasting changes in perception due to experience; learners discriminate features previously invisible (e.g., wine tasters, sonar operators).
**Atom or composite:** Atom — `scale`-up of feature-detector sensitivity.
**Cost model:** Many trials of exposure; lasting cortical changes.
**Real wall?** Yes — biological substrate must be there; some discriminations require early experience.
**Cross-domain wiring:** Ml-training `feature-extractor-fine-tuning`; agentic-reasoning `domain-specific-perceptual-tuning`; signal-processing-rf `matched-filter-learning`.
**Notes:** Gibson & Gibson (1955); Goldstone (1998) review.

### category-learning-prototype-vs-exemplar (cross-domain alias: `prototype-vs-exemplar-model`, `category-representation`)
**Domain:** Cognitive Primitives
**Definition:** Two accounts of category representation: prototype (summary feature average) vs exemplar (stored instances retrieved at test). Evidence supports both depending on task.
**Atom or composite:** Composite — `combine`(prototype-summary, exemplar-instances).
**Cost model:** Prototype cheap; exemplar costs storage but supports variability.
**Real wall?** Yes — different mechanisms for different conditions.
**Cross-domain wiring:** Ml-training `centroid-vs-knn`; agentic-reasoning `summary-vs-store-strategies`; decision-logic `average-vs-lookup`.
**Notes:** Posner & Keele (1968) prototypes; Medin & Schaffer (1978) exemplars.

### perceptual-narrowing (cross-domain alias: `experience-pruning`, `cortical-pruning`)
**Domain:** Cognitive Primitives
**Definition:** Loss of capacity to discriminate distinctions not present in early experience (e.g., infant phoneme discrimination narrows to native language by 1y).
**Atom or composite:** Atom — `scale`-down of unused feature detectors.
**Cost model:** Pruning during sensitive period; difficult to reverse.
**Real wall?** Yes — sensitive-period windows close; later acquisition harder.
**Cross-domain wiring:** Ml-training `pruning-during-training`; agentic-reasoning `language-specific-tokenizer`; decision-logic `feature-set-narrowing`.
**Notes:** Werker & Tees (1984); foundational phoneme-narrowing finding.

### sensitive-periods (cross-domain alias: `sensitive-window`, `learning-window`)
**Domain:** Cognitive Primitives
**Definition:** Periods of heightened plasticity for specific learning (e.g., language, vision); harder but not impossible to acquire after the window closes.
**Atom or composite:** Atom — temporal `scale`-window of elevated plasticity.
**Cost model:** Acquisition during window cheap; after window costly.
**Real wall?** Yes — biological maturation of underlying circuits.
**Cross-domain wiring:** Ml-training `early-training-stage-criticality`; agentic-reasoning `bootstrap-window`; decision-logic `phase-dependent-learning`.
**Notes:** Knudsen (2004); auditory localization in owls.

### critical-periods (cross-domain alias: `critical-window`, `irreversible-learning-period`)
**Domain:** Cognitive Primitives
**Definition:** Strict subset of sensitive periods: experience must occur within window or capacity is permanently lost (e.g., ocular dominance columns in V1).
**Atom or composite:** Atom — biological window with irreversible closure.
**Cost model:** Critical periods short; permanent miss if exposure absent.
**Real wall?** Yes — Hubel & Wiesel (1970) deprived cats demonstrate.
**Cross-domain wiring:** Ml-training `early-training-irreversibility`; agentic-reasoning `bootstrap-required`; decision-logic `one-shot-config-window`.
**Notes:** Hubel & Wiesel (1970); foundational visual cortex work.

### neural-plasticity (cross-domain alias: `brain-plasticity`, `neuroplasticity`)
**Domain:** Cognitive Primitives
**Definition:** Capacity of the nervous system to reorganize structure and function in response to experience; spans synaptic, structural, and systems levels.
**Atom or composite:** Composite — `combine`(synaptic, structural, systems-level changes).
**Cost model:** Slow timescale (hours-years); homeostatic constraints.
**Real wall?** Yes — bounded by metabolic and structural constraints.
**Cross-domain wiring:** Ml-training `model-update-capacity`; ordo-runtime `runtime-reconfig`; agentic-reasoning `online-finetuning`.
**Notes:** Pascual-Leone et al. (2005); review of human plasticity.

### synaptic-plasticity (cross-domain alias: `synapse-level-plasticity`, `weight-update-biology`)
**Domain:** Cognitive Primitives
**Definition:** Use-dependent changes in synaptic strength (LTP, LTD); the cellular substrate of learning at the connection level.
**Atom or composite:** Atom — Δw per synapse.
**Cost model:** Local update; protein synthesis for persistence.
**Real wall?** Yes — synaptic count bounded; weights normalized.
**Cross-domain wiring:** Ml-training `weight-update`; ordo-runtime `edge-weight-modification`; agentic-reasoning `association-strength-update`.
**Notes:** Citri & Malenka (2008) Nat Rev Neurosci.

### structural-plasticity (cross-domain alias: `structural-rewiring`, `dendritic-plasticity`)
**Domain:** Cognitive Primitives
**Definition:** Changes in physical neural structure: spine formation/elimination, axon sprouting, neurogenesis; provides large-scale reorganization capability.
**Atom or composite:** Composite — `combine`(synapse-creation, neurogenesis).
**Cost model:** Slow (days-months); metabolically expensive.
**Real wall?** Yes — bounded by anatomy and energy budget.
**Cross-domain wiring:** Ml-training `architecture-search-NAS`; ordo-runtime `topology-reconfig`; agentic-reasoning `new-tool-acquisition`.
**Notes:** Holtmaat & Svoboda (2009); two-photon spine imaging.

### systems-vs-synaptic-consolidation (cross-domain alias: `systems-consolidation`, `multi-scale-consolidation`)
**Domain:** Cognitive Primitives
**Definition:** Two distinct consolidation processes: synaptic (hours, local) and systems (years, hippocampus→cortex transfer); both required for durable memory.
**Atom or composite:** Composite — temporal `combine`(synaptic-rapid, systems-slow).
**Cost model:** Synaptic: hours; systems: months-years.
**Real wall?** Yes — distinct mechanisms; Ribot's law on retrograde amnesia.
**Cross-domain wiring:** Ml-training `short-vs-long-term-update`; ordo-runtime `cache-to-disk-tiering`; agentic-reasoning `working-to-LTM-transfer`.
**Notes:** Squire (1992); Frankland & Bontempi (2005).

### retrograde-amnesia (cross-domain alias: `RA`, `past-memory-loss`)
**Domain:** Cognitive Primitives
**Definition:** Loss of memories formed before the time of injury or onset; often shows temporal gradient (Ribot's law): recent worse than remote.
**Atom or composite:** Atom — pre-event `scale`-down of accessible content.
**Cost model:** Cannot be relearned without re-exposure.
**Real wall?** Yes — substrate damage; partial recovery sometimes.
**Cross-domain wiring:** Ml-training `weight-corruption`; ordo-runtime `data-loss-event`; agentic-reasoning `prior-state-loss`.
**Notes:** Ribot (1882); H.M. case (Scoville & Milner 1957).

### anterograde-amnesia (cross-domain alias: `AA`, `new-learning-deficit`)
**Domain:** Cognitive Primitives
**Definition:** Inability to form new declarative memories after injury onset, while old memories may be preserved; classically observed in H.M. with hippocampal lesion.
**Atom or composite:** Atom — block on post-event `combine` into LTM.
**Cost model:** Old memories intact; new ones not formed.
**Real wall?** Yes — hippocampus required for declarative encoding.
**Cross-domain wiring:** Ml-training `frozen-weights`; ordo-runtime `read-only-store`; agentic-reasoning `inference-only-mode`.
**Notes:** Scoville & Milner (1957) H.M. case; foundational neuropsychology.

### savings-on-relearning (cross-domain alias: `relearning-savings`, `Ebbinghaus-savings`)
**Domain:** Cognitive Primitives
**Definition:** Reduced trials to criterion when relearning previously learned but apparently forgotten material; demonstrates residual trace beyond explicit recall.
**Atom or composite:** Atom — diff `compare`(initial-trials, relearn-trials).
**Cost model:** Savings often > 50% on apparently forgotten material.
**Real wall?** No (positive resource) — savings exposes hidden retention.
**Cross-domain wiring:** Ml-training `warm-restart-savings`; agentic-reasoning `cached-residual-knowledge`; ordo-runtime `cold-start-vs-warm-restart`.
**Notes:** Ebbinghaus (1885); foundational implicit-memory evidence.

---

## Section 5 — Intuition, Heuristics & System 1

### system-1-kahneman (cross-domain alias: `S1`, `fast-thinking`)
**Domain:** Cognitive Primitives
**Definition:** Fast, automatic, effortless, intuitive mode of cognition contrasted with deliberative System 2; Kahneman's (2011) synthesis of dual-process theory.
**Atom or composite:** Atom — single-pass `project`(input → response).
**Cost model:** Low effort (~100 ms); pattern-matching dominant.
**Real wall?** Yes — coverage limited to overlearned patterns; biased.
**Cross-domain wiring:** Ml-training `forward-pass-only`; agentic-reasoning `fast-direct-answer`; decision-logic `cached-policy`.
**Notes:** Kahneman (2011) — Thinking, Fast and Slow; Stanovich & West (2000) origin.

### fast-thinking (cross-domain alias: `intuitive-mode`, `automatic-cognition`)
**Domain:** Cognitive Primitives
**Definition:** Cognitive mode characterized by low latency, parallel processing, and minimal effort; nearly synonymous with System 1 but emphasizes the speed phenomenology.
**Atom or composite:** Atom — pattern-`hash`-and-respond cycle.
**Cost model:** ~100–500 ms; cheap.
**Real wall?** Yes — speed at cost of accuracy in novel situations.
**Cross-domain wiring:** Ml-training `nearest-neighbor-lookup`; agentic-reasoning `cache-served-response`; ordo-runtime `fast-path`.
**Notes:** Kahneman (2011); Evans (2008) review of dual-process taxonomy.

### dual-process-theory (cross-domain alias: `two-system-theory`, `Type-1-Type-2`)
**Domain:** Cognitive Primitives
**Definition:** Cognition emerges from interaction of two systems: fast/automatic and slow/deliberate. Multiple labels (Type 1/2, System 1/2, intuitive/reflective).
**Atom or composite:** Composite — `combine`(System1-output, System2-override).
**Cost model:** S1 cheap; S2 expensive; arbiter selects.
**Real wall?** Yes — empirical dissociation across tasks (CRT, conflict tasks).
**Cross-domain wiring:** Ml-training `mixture-of-experts`; decision-logic `fast-vs-slow-arbiter`; agentic-reasoning `quick-vs-tool-use`.
**Notes:** Evans & Stanovich (2013); Sloman (1996) earliest two-system formulation.

### heuristic-and-bias-program-kahneman-tversky (cross-domain alias: `H&B-program`, `KT-program`)
**Domain:** Cognitive Primitives
**Definition:** Research program documenting systematic deviations from rational choice attributable to mental shortcuts; foundational behavioral economics tradition.
**Atom or composite:** Composite — catalog of `project`(shortcut → biased-output).
**Cost model:** Heuristics cheap; biases costly when stakes high.
**Real wall?** Yes — biases are systematic, not random; partially correctible.
**Cross-domain wiring:** Decision-logic `bias-aware-policy`; ml-training `bias-mitigation`; agentic-reasoning `error-pattern-library`.
**Notes:** Tversky & Kahneman (1974) Science; the founding manifesto.

### availability-heuristic (cross-domain alias: `availability`, `ease-of-recall-bias`)
**Domain:** Cognitive Primitives
**Definition:** Estimating frequency/probability by ease of bringing examples to mind; produces systematic overestimation of vivid, recent, or memorable events.
**Atom or composite:** Atom — `scale`-up by recall-fluency.
**Cost model:** Cheap; uses retrieval fluency directly.
**Real wall?** Yes — uncorrectable without external data; vividness biases.
**Cross-domain wiring:** Ml-training `retrieval-frequency-bias`; agentic-reasoning `context-recency-overweight`; statistics-probability `MLE-from-biased-sample`.
**Notes:** Tversky & Kahneman (1973); foundational heuristic.

### representativeness-heuristic (cross-domain alias: `similarity-heuristic`, `prototype-match`)
**Domain:** Cognitive Primitives
**Definition:** Estimating probability by similarity to a prototype, ignoring base rates and sample size; source of conjunction fallacy and base-rate neglect.
**Atom or composite:** Atom — `compare`(target, prototype) → probability estimate.
**Cost model:** Cheap; matches feature template.
**Real wall?** Yes — base-rate neglect persists despite training.
**Cross-domain wiring:** Ml-training `prototype-classifier`; agentic-reasoning `stereotyped-classification`; statistics-probability `base-rate-neglect`.
**Notes:** Kahneman & Tversky (1972); Linda problem (Tversky & Kahneman 1983).

### anchoring-and-adjustment (cross-domain alias: `anchoring`, `numerical-anchoring`)
**Domain:** Cognitive Primitives
**Definition:** Estimates anchored on an initial value (even irrelevant) and insufficiently adjusted; produces predictable biases in negotiation, judgment, valuation.
**Atom or composite:** Atom — `project`(anchor + small Δ).
**Cost model:** Adjustment effortful; insufficient amount adjusted.
**Real wall?** Yes — robust across domains and expert populations.
**Cross-domain wiring:** Decision-logic `prior-pinning-bias`; ml-training `init-dependent-convergence`; agentic-reasoning `first-context-overweight`.
**Notes:** Tversky & Kahneman (1974); experts also susceptible (Northcraft & Neale 1987).

### affect-heuristic-slovic (cross-domain alias: `affect-as-information`, `emotional-shortcut`)
**Domain:** Cognitive Primitives
**Definition:** Quick judgment by affective response (good/bad feeling); produces systematic relations between perceived risk and benefit even when independent.
**Atom or composite:** Atom — `project`(emotional-tag → judgment).
**Cost model:** Very fast; embodied response.
**Real wall?** Yes — somatic-marker / affect-based shortcut is automatic.
**Cross-domain wiring:** Ml-training `sentiment-driven-classification`; decision-logic `emotion-tagged-routing`; agentic-reasoning `valence-biased-response`.
**Notes:** Slovic et al. (2007); affect heuristic and benefit-risk relations.

### recognition-heuristic-goldstein-gigerenzer (cross-domain alias: `recognition-heuristic`, `RH`)
**Domain:** Cognitive Primitives
**Definition:** When choosing between two options and recognizing only one, infer the recognized has higher criterion value; surprisingly effective in many domains.
**Atom or composite:** Atom — binary `compare`(recognized, unrecognized) → choose recognized.
**Cost model:** Cheap; one bit of information.
**Real wall?** Soft — works when recognition correlates with criterion (e.g., city size).
**Cross-domain wiring:** Ml-training `name-based-classifier`; agentic-reasoning `known-token-preference`; decision-logic `familiarity-bias`.
**Notes:** Goldstein & Gigerenzer (2002) Psych Rev; canonical fast-and-frugal heuristic.

### take-the-best (cross-domain alias: `TTB`, `one-good-reason-decision`)
**Domain:** Cognitive Primitives
**Definition:** Lexicographic decision rule: examine cues in validity order, stop at the first discriminating cue, decide on that cue alone; ignores all others.
**Atom or composite:** Composite — ordered `scan`(cues) + early-stop `compare`.
**Cost model:** Very low; usually one or two cues consulted.
**Real wall?** Soft — robust under noise and small samples (Gigerenzer & Goldstein 1996).
**Cross-domain wiring:** Decision-logic `lexicographic-rule`; ml-training `decision-stump`; agentic-reasoning `first-discriminating-tool`.
**Notes:** Gigerenzer & Goldstein (1996); often beats weighted-additive under realistic conditions.

### fast-and-frugal-heuristics-abc (cross-domain alias: `F&F-heuristics`, `ABC-heuristics`)
**Domain:** Cognitive Primitives
**Definition:** Class of simple decision strategies (recognition, TTB, tallying) that exploit environmental structure to make accurate decisions with minimal computation.
**Atom or composite:** Composite — `combine`(search, stop, decide) rules.
**Cost model:** Very low; bounded rationality friendly.
**Real wall?** Soft — ecologically rational; less robust outside fit environments.
**Cross-domain wiring:** Decision-logic `simple-bounded-rules`; ml-training `low-parameter-models`; agentic-reasoning `minimal-context-decision`.
**Notes:** Gigerenzer & Todd (1999) — Simple Heuristics That Make Us Smart.

### satisficing-simon (cross-domain alias: `aspiration-level-search`, `good-enough-search`)
**Domain:** Cognitive Primitives
**Definition:** Search that stops at the first option meeting an aspiration level rather than seeking the optimum; foundational bounded-rationality concept.
**Atom or composite:** Composite — `scan` + threshold `compare` → stop.
**Cost model:** Cheaper than optimization; depends on aspiration calibration.
**Real wall?** Soft — required when optimization is intractable.
**Cross-domain wiring:** Decision-logic `threshold-stopping`; agentic-reasoning `good-enough-output`; ml-training `early-stopping-criterion`.
**Notes:** Simon (1956); foundational bounded-rationality work.

### ecological-rationality (cross-domain alias: `ER`, `Gigerenzer-rationality`)
**Domain:** Cognitive Primitives
**Definition:** A heuristic is rational to the extent its structure matches its environment; rejects context-free optimality in favor of strategy-environment fit.
**Atom or composite:** Atom — `compare`(strategy-structure, env-structure) → fitness.
**Cost model:** Per-environment evaluation.
**Real wall?** Yes — no universally optimal strategy across all environments.
**Cross-domain wiring:** Ml-training `inductive-bias-matched-to-data`; decision-logic `domain-specific-policy`; agentic-reasoning `strategy-domain-pairing`.
**Notes:** Gigerenzer et al. (1999); contrasts with logical/classical rationality.

### gut-feeling (cross-domain alias: `intuition-feeling`, `embodied-judgment`)
**Domain:** Cognitive Primitives
**Definition:** Phenomenal hunch about a decision arising before/without explicit reasoning; often expert-acquired pattern recognition surfacing as feeling.
**Atom or composite:** Atom — `project`(pattern-match → felt-confidence).
**Cost model:** Fast; opaque introspection.
**Real wall?** Yes — accurate only in domains of high experience and good feedback.
**Cross-domain wiring:** Ml-training `model-confidence-without-explanation`; agentic-reasoning `confidence-without-trace`; decision-logic `quick-judgment`.
**Notes:** Klein (1998); Kahneman & Klein (2009) intuition debate.

### intuitive-expertise-klein-rpd (cross-domain alias: `RPD-model`, `recognition-primed-decision`)
**Domain:** Cognitive Primitives
**Definition:** Klein's model: experts recognize prototypical situations and mentally simulate one course of action to test; minimal explicit comparison.
**Atom or composite:** Composite — `combine`(recognition, mental-simulation, action).
**Cost model:** Fast; experts converge in seconds.
**Real wall?** Yes — works in high-validity domains; not in low-feedback domains.
**Cross-domain wiring:** Agentic-reasoning `expert-policy-execution`; decision-logic `pattern-based-action`; ml-training `expert-imitation`.
**Notes:** Klein (1998) — Sources of Power; founded NDM.

### recognition-primed-decision (cross-domain alias: `RPD`, `expert-recognize-act`)
**Domain:** Cognitive Primitives
**Definition:** Specific decision pattern: situation recognized as familiar → action retrieved → minimal evaluation needed; experts in time pressure typify this pattern.
**Atom or composite:** Atom — `project`(situation → cached-action).
**Cost model:** ~seconds; bypasses explicit comparison.
**Real wall?** Yes — requires extensive domain experience.
**Cross-domain wiring:** Ml-training `cached-policy`; agentic-reasoning `expert-direct-tool-call`; decision-logic `pattern-action-pairing`.
**Notes:** Klein, Calderwood & Clinton-Cirocco (1986); foundational RPD study.

### naturalistic-decision-making (cross-domain alias: `NDM`, `field-decision-research`)
**Domain:** Cognitive Primitives
**Definition:** Research program studying how experts make decisions under time pressure, uncertainty, and high stakes in real-world settings; complements lab heuristics work.
**Atom or composite:** Composite — `combine`(expertise-pattern, time-pressure, dynamic-context).
**Cost model:** Field-study cost high; ecological validity high.
**Real wall?** N/A — research approach.
**Cross-domain wiring:** Agentic-reasoning `real-world-decision-trace`; decision-logic `field-validated-policy`; ml-training `production-deployment-study`.
**Notes:** Klein & Calderwood (1991); Zsambok & Klein (1997).

### somatic-marker-hypothesis-damasio (cross-domain alias: `SMH`, `body-loop-hypothesis`)
**Domain:** Cognitive Primitives
**Definition:** Hypothesis that bodily states (somatic markers) tag options with anticipated affect, guiding decision-making implicitly; vmPFC-mediated.
**Atom or composite:** Atom — body-state `project`(option → affective-tag).
**Cost model:** Fast embodied signal; bypasses explicit reasoning.
**Real wall?** Yes — vmPFC lesions impair decision-making despite intact reasoning.
**Cross-domain wiring:** Ml-training `learned-value-tag`; agentic-reasoning `option-valence-cache`; decision-logic `embodied-prior`.
**Notes:** Damasio (1994) — Descartes' Error; Bechara et al. (1997) Iowa gambling.

### implicit-cognition (cross-domain alias: `implicit-processes`, `unconscious-cognition`)
**Domain:** Cognitive Primitives
**Definition:** Mental processes influencing behavior without conscious awareness or intent; revealed by indirect measures (priming, IAT).
**Atom or composite:** Atom — `project`(input → behavior) without conscious access.
**Cost model:** Fast; not reportable.
**Real wall?** Yes — much cognition is opaque to introspection.
**Cross-domain wiring:** Ml-training `latent-representation`; agentic-reasoning `internal-state-not-in-output`; decision-logic `unobservable-state`.
**Notes:** Greenwald & Banaji (1995); foundational implicit-social-cognition framework.

### implicit-attitudes-iat (cross-domain alias: `IAT`, `Implicit-Association-Test`)
**Domain:** Cognitive Primitives
**Definition:** Reaction-time-based measure of automatic associations between concepts and evaluations; controversial as predictor of behavior but established as a measure.
**Atom or composite:** Atom — RT-difference `compare`(congruent, incongruent).
**Cost model:** Quick computer test; reliability moderate.
**Real wall?** Partially — measurement valid; predictive validity contested.
**Cross-domain wiring:** Ml-training `bias-measurement-probe`; decision-logic `latent-bias-audit`; agentic-reasoning `association-pattern-audit`.
**Notes:** Greenwald, McGhee & Schwartz (1998); Oswald et al. (2013) meta-analytic critique.

### embodied-cognition (cross-domain alias: `embodied-mind`, `grounded-cognition`)
**Domain:** Cognitive Primitives
**Definition:** Cognition is shaped by bodily action and sensorimotor experience; concepts are grounded in perceptual-motor systems, not abstract symbols alone.
**Atom or composite:** Composite — `combine`(motor-state, perceptual-state, conceptual-state).
**Cost model:** Distributed across body+brain.
**Real wall?** Yes — body shapes possible cognition; replicability of strong forms contested.
**Cross-domain wiring:** Ml-training `multimodal-grounded-models`; agentic-reasoning `tool-use-grounding`; decision-logic `physical-context-aware-policy`.
**Notes:** Barsalou (2008); Clark (1997).

### gist-representation-fuzzy-trace (cross-domain alias: `gist-FTT`, `fuzzy-trace-gist`)
**Domain:** Cognitive Primitives
**Definition:** Reyna & Brainerd's fuzzy-trace theory: memory encodes parallel gist (meaning) and verbatim (surface) traces; gist dominates intuitive reasoning.
**Atom or composite:** Composite — parallel `combine`(gist-trace, verbatim-trace).
**Cost model:** Cheap to encode both; gist more durable.
**Real wall?** Yes — verbatim fades fast; gist drives risk decisions.
**Cross-domain wiring:** Ml-training `multi-scale-representation`; agentic-reasoning `gist-vs-exact-tradeoff`; decision-logic `summary-driven-choice`.
**Notes:** Reyna & Brainerd (1995); Reyna (2012) update.

### priming-cognitive (cross-domain alias: `cognitive-priming`, `semantic-pre-activation`)
**Domain:** Cognitive Primitives
**Definition:** Implicit facilitation of cognitive processes (concept retrieval, decision) by prior exposure to related material; analog to perceptual priming at semantic level.
**Atom or composite:** Atom — pre-activation `scale`-up of associated concepts.
**Cost model:** Cheap; effects ~10–100 ms.
**Real wall?** Yes — replicability of "social priming" effects contested.
**Cross-domain wiring:** Ml-training `KV-cache-warming`; agentic-reasoning `context-warming`; decision-logic `pre-activation-state`.
**Notes:** Meyer & Schvaneveldt (1971); reproducibility critique by Doyen et al. (2012).

### conceptual-priming (cross-domain alias: `meaning-based-priming`, `category-priming`)
**Domain:** Cognitive Primitives
**Definition:** Priming that depends on shared meaning/category rather than surface form; e.g., "doctor" → faster recognition of "nurse."
**Atom or composite:** Atom — semantic-network `scale`-up at related nodes.
**Cost model:** Cheap; persists for hours-days for novel concepts.
**Real wall?** Yes — robust effect, well-replicated.
**Cross-domain wiring:** Ml-training `embedding-neighborhood-activation`; agentic-reasoning `topic-warming`; decision-logic `concept-context-pre-loading`.
**Notes:** Tulving & Schacter (1990); foundational implicit-memory work.

### semantic-priming (cross-domain alias: `meaning-priming`, `word-association-priming`)
**Domain:** Cognitive Primitives
**Definition:** Faster recognition of a word when preceded by a semantically related word; demonstrates spreading activation in semantic memory.
**Atom or composite:** Atom — pre-activation along semantic-network edges.
**Cost model:** Cheap; effects ~30–100 ms.
**Real wall?** Yes — empirically robust foundation of semantic memory research.
**Cross-domain wiring:** Ml-training `embedding-similarity-warmup`; agentic-reasoning `relevant-context-priming`; decision-logic `topic-prep`.
**Notes:** Meyer & Schvaneveldt (1971); Collins & Loftus (1975) spreading activation.

### mental-shortcut (cross-domain alias: `cognitive-shortcut`, `general-heuristic`)
**Domain:** Cognitive Primitives
**Definition:** General term for any simplified cognitive operation that approximates a more demanding computation; umbrella for heuristics.
**Atom or composite:** Atom — coarse `project` replacing exact computation.
**Cost model:** Low compute; calibrated accuracy.
**Real wall?** Yes — accuracy ceiling defined by shortcut quality.
**Cross-domain wiring:** Decision-logic `approximate-policy`; ml-training `surrogate-objective`; agentic-reasoning `cached-shortcut-answer`.
**Notes:** Common across heuristics literature; Gigerenzer popularized "rules of thumb."

### automatic-processing (cross-domain alias: `automaticity`, `non-controlled-processing`)
**Domain:** Cognitive Primitives
**Definition:** Processing that occurs without intent, awareness, or interference with other tasks; achieved through extensive practice or innate hardwiring.
**Atom or composite:** Atom — compiled `project` requiring no attention.
**Cost model:** Effectively free; runs in parallel.
**Real wall?** Yes — once automatic, hard to suppress (Stroop).
**Cross-domain wiring:** Ml-training `compiled-policy`; ordo-runtime `JIT-compiled-handler`; agentic-reasoning `cached-response-path`.
**Notes:** Schneider & Shiffrin (1977); Bargh (1994) four horsemen of automaticity.

### controlled-processing (cross-domain alias: `effortful-processing`, `deliberate-processing`)
**Domain:** Cognitive Primitives
**Definition:** Processing requiring attention, capacity-limited, slow, intentional; complement of automatic processing; can be overridden mid-execution.
**Atom or composite:** Composite — `combine`(attention, working-memory, monitoring).
**Cost model:** Expensive; capacity-limited.
**Real wall?** Yes — limited by attention and WM.
**Cross-domain wiring:** Decision-logic `deliberate-policy`; agentic-reasoning `chain-of-thought-with-monitoring`; ordo-runtime `interpreted-execution`.
**Notes:** Schneider & Shiffrin (1977); foundational automatic-controlled distinction.

### default-mode (cross-domain alias: `DMN-default`, `default-cognition`)
**Domain:** Cognitive Primitives
**Definition:** Baseline cognitive mode when not focused on external task: self-referential, time travel, mind wandering; supported by default-mode network (DMN).
**Atom or composite:** Composite — `combine`(self-reference, prospection, retrospection).
**Cost model:** Always-on; cheap relative to task-positive mode.
**Real wall?** Yes — DMN activity inversely correlated with task focus.
**Cross-domain wiring:** Ordo-runtime `idle-task`; agentic-reasoning `background-thread`; ml-training `unconstrained-generation`.
**Notes:** Raichle et al. (2001); Andrews-Hanna (2012) review.

### ironic-processes-wegner (cross-domain alias: `Wegner-ironic`, `thought-rebound`)
**Domain:** Cognitive Primitives
**Definition:** Attempts to suppress a thought paradoxically increase its accessibility; monitoring for forbidden content provides repeated activation.
**Atom or composite:** Composite — `combine`(suppress-process, monitor-process) → rebound.
**Cost model:** Resource-dependent; worse under load.
**Real wall?** Yes — Wegner (1994) demonstrated robustly.
**Cross-domain wiring:** Ml-training `negative-prompt-side-effect`; agentic-reasoning `suppression-leak`; decision-logic `forbidden-class-leakage`.
**Notes:** Wegner (1994) Psych Rev "Ironic processes of mental control."

### mere-exposure-effect (cross-domain alias: `exposure-liking`, `Zajonc-effect`)
**Domain:** Cognitive Primitives
**Definition:** Repeated exposure to a stimulus increases liking, even without conscious recognition; demonstrated even for subliminal stimuli (Zajonc 1968).
**Atom or composite:** Atom — exposure-count `scale`-up of positive affect.
**Cost model:** Very cheap; no conscious processing needed.
**Real wall?** Yes — empirically robust meta-analytically (Bornstein 1989).
**Cross-domain wiring:** Ml-training `frequency-as-preference`; agentic-reasoning `familiarity-bias-output`; decision-logic `seen-before-up-weight`.
**Notes:** Zajonc (1968); Bornstein (1989) meta-analysis.

### fluency-cue (cross-domain alias: `processing-fluency`, `ease-as-truth-signal`)
**Domain:** Cognitive Primitives
**Definition:** Subjective ease of processing serves as a cue for memory, truth, liking, and confidence; high fluency → endorsement, regardless of actual validity.
**Atom or composite:** Atom — `project`(processing-ease → endorsement).
**Cost model:** Free; piggybacks on processing.
**Real wall?** Yes — fluency drives many JOL/truth errors.
**Cross-domain wiring:** Ml-training `low-loss-as-truth`; agentic-reasoning `easy-generation-as-correct`; decision-logic `effort-low-as-validity`.
**Notes:** Schwarz (2004); Reber et al. (1998) fluency-judgment work.

### familiarity-heuristic (cross-domain alias: `familiarity-bias`, `known-equals-better`)
**Domain:** Cognitive Primitives
**Definition:** Tendency to choose familiar options over unfamiliar ones, even when objectively worse; partial overlap with recognition heuristic and mere-exposure effect.
**Atom or composite:** Atom — `compare`(familiar, unfamiliar) → choose familiar.
**Cost model:** Very cheap; default in low-info situations.
**Real wall?** Yes — robust default; not eliminable without intervention.
**Cross-domain wiring:** Decision-logic `safe-default-bias`; ml-training `in-distribution-preference`; agentic-reasoning `known-tool-preference`.
**Notes:** Tversky & Kahneman (1973); related to status-quo bias.

---

## Section 6 — Imagination, Simulation & System 2

