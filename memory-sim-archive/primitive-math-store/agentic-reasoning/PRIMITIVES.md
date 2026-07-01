# Agentic Reasoning Domain Primitives
> Cross-domain wiring: mental simulation = world model + physics simulation; tool use = function call + API contract;
> causal reasoning = do-calculus + intervention; CoT = step-by-step decomposition; theory of mind = recursive belief attribution

## 1. Reasoning Frameworks

### [PRIM-001] chain-of-thought
- **Atom/Composite:** Composite
- **Definition:** Chain-of-Thought (CoT): generate intermediate reasoning steps before final answer. "Let's think step by step." Implicit vs. explicit CoT; generated vs. retrieved CoT.
- **Cost Model:** Steps = O(k) for k-step reasoning; each step = one LLM forward pass; latency = k × per-step latency.
- **Real Wall:** CoT effectiveness plateaus for >5–7 steps; self-consistency (sample multiple CoTs, vote) improves accuracy at cost O(n·k).
- **Cross-Domain Aliases:** step-decomposition (control-numerical-opt), incremental-deduction (linear-algebra-matrix).
- **Notes:** Wei et al. (2022); CoT works for reasoning tasks (math, logic, code) but not for factual retrieval.

### [PRIM-002] tree-of-thought
- **Atom/Composite:** Composite
- **Definition:** Tree-of-Thought (ToT): explore branching reasoning paths; each node = partial solution; DFS/BFS for search; evaluate branch quality to prune.
- **Cost Model:** Branch factor b, depth d, evaluation cost per node; worst-case O(b^d); pruning reduces to O(b·d) on average.
- **Real Wall:** Evaluation function quality determines pruning accuracy; noisy evaluation → wrong branches kept, correct branches pruned.
- **Cross-Domain Aliases:** search-tree (linear-algebra-matrix), backtracking (control-numerical-opt).
- **Notes:** Yao et al. (2023); combines Monte Carlo Tree Search intuition with LLM self-evaluation.

### [PRIM-003] graph-of-thought
- **Atom/Composite:** Composite
- **Definition:** Graph-of-Thought (GoT): model reasoning as DAG; nodes = reasoning units (thought, fact, conclusion); edges = dependencies; supports non-linear, cyclic, and self-referential thought.
- **Cost Model:** Graph construction O(N edges); evaluation propagation O(N); merging thought nodes reduces redundancy.
- **Real Wall:** Cycle detection and handling; graph consistency maintenance; no established benchmark for GoT quality.
- **Cross-Domain Aliases:** dag-evaluation (linear-algebra-matrix), belief-network (information-theory-coding).
- **Notes:** chi-yang hsu (2403.07144); GoT subsumes CoT (chain = linear graph) and ToT (tree = acyclic graph).

### [PRIM-004] program-aided-language-model
- **Atom/Composite:** Composite
- **Definition:** PAL (Program-Aided Language Model): LLM generates executable code to solve reasoning tasks; code executor computes result; LLM interprets output.
- **Cost Model:** Code generation + execution + interpretation; execution can use external libraries (math, sympy, networkx).
- **Real Wall:** Code generation errors (syntax, logic); execution sandboxing; Python vs. LLM arithmetic (PAL reduces LLM math errors).
- **Cross-Domain Aliases:** tool-use-execution (agentic-reasoning), symbolic-execution (control-numerical-opt).
- **Notes:** Gao et al. (2022); effective for math word problems, date reasoning, table QA.

### [PRIM-005] react-reasoning-acting
- **Atom/Composite:** Composite
- **Definition:** ReAct (Reasoning + Acting): interleave thought steps with tool actions; thought = reason about situation; action = call tool; observation = tool result; repeat until done.
- **Cost Model:** Each iteration = LLM forward pass + tool call + result parsing; converges when action yields final answer.
- **Real Wall:** Tool call errors cascade; loop detection needed (max iterations); action space quality determines success rate.
- **Cross-Domain Aliases:** think-act-observe (agentic-reasoning), guided-exploration (ml-training).
- **Notes:** Yao et al. (2023, ICLR); ReAct + CoT (SCoRe) combines reasoning trace with actions.

### [PRIM-006] scratchpad-reasoning
- **Atom/Composite:** Primitive
- **Definition:** Scratchpad: LLM writes working notes during reasoning; not part of final output; serves as explicit working memory for multi-step problems.
- **Cost Model:** Scratchpad tokens count toward context limit; each step writes to scratchpad; at most O(context) scratchpad size.
- **Real Wall:** LLM may overwrite scratchpad (catastrophic forgetting within generation); token budget limits scratchpad length.
- **Cross-Domain Aliases:** working-memory (agentic-reasoning), intermediate-state (control-numerical-opt).
- **Notes:** Nye et al. (2021); formalizing the scratchpad as a separate "internal monologue."

### [PRIM-007] self-consistency
- **Atom/Composite:** Composite
- **Definition:** Self-consistency: sample multiple reasoning paths (CoT or ReAct), aggregate answers via majority vote. Improves CoT by trading inference cost for accuracy.
- **Cost Model:** O(n) forward passes for n samples; vote = argmax over answer set; cost = n × per-path cost.
- **Real Wall:** Answer aggregation tricky for open-ended outputs (need string matching or LLM-as-judge); marginal gains after n=10–20.
- **Cross-Domain Aliases:** ensemble-vote (ml-training), multi-sample-aggregation (information-theory-coding).
- **Notes:** Wang et al. (2022); effective for math and logical reasoning; less useful for factual QA.

### [PRIM-008] algorithm-of-thoughts
- **Atom/Composite:** Composite
- **Definition:** AoT: embed algorithmic structures (DFS, BFS, tree, graph) into LLM reasoning; LLM serves as state evaluator, not full searcher; uses few-shot examples of algorithmic patterns.
- **Cost Model:** Fewer LLM calls than ToT (algorithm drives search, not brute force); state evaluation per node.
- **Real Wall:** LLM as state evaluator may misjudge search frontier; algorithm must fit problem structure; not general-purpose.
- **Cross-Domain Aliases:** heuristic-search (control-numerical-opt), guided-reasoning (linear-algebra-matrix).
- **Notes:** (Sel et al. 2024); bridges symbolic search with neural evaluation.

### [PRIM-009] process-supervision-reward-model
- **Atom/Composite:** Composite
- **Definition:** Process Reward Model (PRM): train reward model that scores each reasoning step (not just final answer); use for beam search over reasoning paths.
- **Cost Model:** PRM training requires step-level labels (human or LLM annotators); inference = PRM score per step + search.
- **Real Wall:** Step-level annotation is expensive; PRM can be gamed (plausible steps that lead to wrong answers); credit assignment across steps is hard.
- **Cross-Domain Aliases:** step-reward (ml-training), fine-grained-feedback (control-numerical-opt).
- **Notes:** Lightman et al. (2023) "Let's Verify Step by Step"; vs. outcome reward model (ORM) which scores only final answer.

### [PRIM-010] reasoning-via-ask
- **Atom/Composite:** Composite
- **Definition:** Ask-after-Thinking: reason first (generate CoT), then decide whether to ask (tool call or external query) based on confidence; reduces unnecessary tool calls.
- **Cost Model:** Each round = think + decide (LLM call) + optional action; threshold tuning for decide function.
- **Real Wall:** Over-reliance on internal reasoning vs. external retrieval; threshold too high → missed corrections; threshold too low → wasted tool calls.
- **Cross-Domain Aliases:** confidence-gated-action (agentic-reasoning), selective-retrieval (retrieval-search).

## 2. Causal Reasoning

### [PRIM-011] do-calculus-intervention
- **Atom/Composite:** Primitive
- **Definition:** Pearl's do-operator: do(X = x) = intervene, set variable X to value x; causal effect = P(Y | do(X=x)) vs. observational P(Y | X=x).
- **Cost Model:** Identification via do-calculus rules (insertion/deletion, action/observation, cascade); requires causal DAG.
- **Real Wall:** Causal DAG may be unknown or miss confounders; unmeasured confounders → identification impossible (backdoor criterion).
- **Cross-Domain Aliases:** intervention (biology-bioinformatics), causal-effect-estimation (ml-training).
- **Notes:** Pearl (2009); three-layer causal hierarchy: association (seeing) → intervention (doing) → counterfactual (imagining).

### [PRIM-012] causal-discovery
- **Atom/Composite:** Composite
- **Definition:** Causal discovery: infer causal structure from observational data; methods: PC algorithm (constraint-based), FCI (latent confounders), GES (score-based), NOTEARS (continuous optimization).
- **Cost Model:** PC algorithm: conditional independence tests O(N·d²) per test; score-based methods: greedy search over DAG space.
- **Real Wall:** Faithfulness assumption (no fine-tuned cancellations); curse of dimensionality (d variables → 2^(d²) possible graphs).
- **Cross-Domain Aliases:** structure-learning (ml-training), bayesian-network-inference (information-theory-coding).
- **Notes:** Spirtes et al. (2000); Shimizu et al. (2006) LiNGAM for linear non-Gaussian acyclic models.

### [PRIM-013] counterfactual-reasoning
- **Atom/Composite:** Composite
- **Definition:** Counterfactual: "What if X had been different?" Requires structural causal model; compute: Abduction (update beliefs with evidence) → Action (intervene) → Prediction.
- **Cost Model:** SCM evaluation per counterfactual; requires model of mechanism; not derivable from data alone.
- **Real Wall:** Counterfactuals not empirically testable; requires strong assumptions; humans reason counterfactually naturally.
- **Cross-Domain Aliases:** what-if-analysis (control-numerical-opt), hypothetical-reasoning (agentic-reasoning).
- **Notes:** Pearl (2009) structural causal model; Halpern & Pearl causal model; counterfactual regret in game theory.

### [PRIM-014] instrumental-variable
- **Atom/Composite:** Primitive
- **Definition:** Instrumental variable Z: affects treatment X, has no direct effect on outcome Y except through X; enables causal effect estimation despite unmeasured confounders.
- **Cost Model:** Two-stage least squares (2SLS): regress X on Z, regress Y on predicted X; weak instrument → large variance.
- **Real Wall:** Instrument relevance (must correlate with X); exclusion restriction (Z affects Y only through X); many proposed instruments fail both.
- **Cross-Domain Aliases:** proxy-causal (biology-bioinformatics), exogeneity-assumption (ml-training).
- **Notes:** Angrist et al. (1996) LATE (Local Average Treatment Effect); natural experiments as instruments.

### [PRIM-015] mediation-analysis
- **Atom/Composite:** Composite
- **Definition:** Mediation: decompose total effect into direct effect (not through mediator M) + indirect effect (through M); natural direct/indirect effect; controlled direct effect.
- **Cost Model:** Sequential ignorability assumption for identification; sensitivity analysis for unmeasured confounding.
- **Real Wall:** Mediation requires no confounders of mediator-outcome relationship (except through treatment); Baron-Kenny method outdated.
- **Cross-Domain Aliases:** path-analysis (linear-algebra-matrix), effect-decomposition (control-numerical-opt).
- **Notes:** Imai et al. (2010) causal mediation analysis; sensitivityUnmeasuredMedMed for robustness.

### [PRIM-016] frontdoor-criterion
- **Atom/Composite:** Primitive
- **Definition:** Frontdoor criterion: causal effect identifiable if all paths from X to Y go through mediator M, and no unmeasured confounding at M.
- **Cost Model:** Identified as: P(Y | do(X=x)) = Σ_m P(M=m | do(X=x)) × P(Y | do(M=m), X=x).
- **Real Wall:** Requires no backdoor paths (no unmeasured confounders between X and M); frontdoor often impractical (rare complete mediator).
- **Cross-Domain Aliases:** mediation-identification (agentic-reasoning), indirect-path (linear-algebra-matrix).
- **Notes:** Pearl (2009) frontdoor vs. backdoor adjustment; useful when backdoor paths cannot be blocked.

### [PRIM-017] sensitivity-analysis-causal
- **Atom/Composite:** Primitive
- **Definition:** Causal sensitivity analysis: assess how robust causal estimate is to unmeasured confounding; compute minimum strength of confounding that would nullify result.
- **Cost Model:** E-value computation: R = exp(OR × ln(OR) / 2); bounding factor approach; tipping point analysis.
- **Real Wall:** Assumptions about confounder-outcome relationship; sensitivity parameters not empirically verifiable.
- **Cross-Domain Aliases:** robustness-check (ml-training), assumption-sensitivity (control-numerical-opt).

### [PRIM-018] causal-inference-with-text
- **Atom/Composite:** Composite
- **Definition:** Causal inference using text as treatment/control/confounder; text-as-treatment: compare outcomes for documents containing vs. not containing term; LDA topic features as confounders.
- **Cost Model:** Text processing (embedding, tokenization) + causal model; BERT embeddings as high-dimensional confounders.
- **Real Wall:** Text representation bias; unmeasured confounders in text; semantic drift (same term has different meaning in different corpora).
- **Cross-Domain Aliases:** observational-study-from-text (information-theory-coding), text-as-confounder (ml-training).

## 3. World Models / Mental Simulation

### [PRIM-019] mental-simulation
- **Atom/Composite:** Composite
- **Definition:** Mental simulation: humans simulate physical, social, or temporal scenarios; object permanence; counterfactual mental models; mental rotation (Shepard & Metzler).
- **Cost Model:** Cognitive load increases with simulation complexity; limited working memory capacity (~4–7 chunks).
- **Real Wall:** Systematic biases (framing, availability, anchoring); simulation accuracy degrades with complexity; expert intuition vs. novice simulation.
- **Cross-Domain Aliases:** physics-engine-simulation (physics-diffusion), imagination (agentic-reasoning).
- **Notes:** Tenenbaum et al. (2011) "How to grow a mind"; cognitive science evidence for intuitive physics engine.

### [PRIM-020] intuitive-physics-engine
- **Atom/Composite:** Composite
- **Definition:** Intuitive physics engine: probabilistic simulation of physical scenarios (support, collision, gravity, object permanence); similar to physics engine but approximate and fast.
- **Cost Model:** Particle-based approximations; Bayesian mental simulation (pz, probability as mental model).
- **Real Wall:** Failures in high-speed, complex-geometry, or invisible scenarios; humans systematically violate physical laws (Newtonian intuition).
- **Cross-Domain Aliases:** physics-simulation (physics-diffusion), approximate-physics (control-numerical-opt).
- **Notes:** Battaglia et al. (2013) "Simulation as an engine of physical scene understanding."

### [PRIM-021] theory-of-mind
- **Atom/Composite:** Composite
- **Definition:** Theory of Mind (ToM): infer mental states (beliefs, desires, intentions) of others; recursive ("I think she thinks that he thinks…"); false belief tasks.
- **Cost Model:** Recursion depth = number of mental state layers; mental state complexity increases with depth.
- **Real Wall:** ToM deficits in autism spectrum; cultural variation in ToM expression; anthropomorphism in AI attribution.
- **Cross-Domain Aliases:** recursive-belief (agentic-reasoning), social-cognition (cognitive-reasoning).
- **Notes:** Premack & Woodruff (1978); modern LLMs fail some ToM tasks despite impressive performance; recursive ToM is benchmark for social reasoning.

### [PRIM-022] situation-model
- **Atom/Composite:** Primitive
- **Definition:** Situation model: mental representation of text/narrative events; maintains characters, locations, temporal relations; updated as narrative progresses.
- **Cost Model:** Tracking effort increases with entity count; Winograd Schema Challenge tests resolution of ambiguous pronouns via situation model.
- **Real Wall:** Anaphora resolution errors; coreference chains (who did what to whom); event ordering in complex narratives.
- **Cross-Domain Aliases:** entity-tracking (information-theory-coding), narrative-state (agentic-reasoning).
- **Notes:** Zwaan & Radvansky (1998) event-indexing model; grounding language in spatial/temporal/causal relations.

### [PRIM-023] counterfactual-imagination
- **Atom/Composite:** Composite
- **Definition:** Counterfactual imagination: mentally undo a past event ("if I had taken the other road") and simulate alternative outcomes; key to causal reasoning, planning, regret.
- **Cost Model:** Requires model of causal mechanism; similarity assessment (how different is the counterfactual from reality?).
- **Real Wall:** Counterfactuals are not directly observable; human counterfactuals are often implausible; mental simulation fidelity is limited.
- **Cross-Domain Aliases:** counterfactual-reasoning (agentic-reasoning), what-if-simulation (physics-diffusion).

### [PRIM-024] counterfactual-regret-minimization
- **Atom/Composite:** Composite
- **Definition:** Counterfactual Regret Minimization (CFR): iterative algorithm for solving imperfect-information games; regrets = difference between action taken and best possible action.
- **Cost Model:** O(I · A · H) per iteration where I = infosets, A = actions, H = history depth; converges to Nash equilibrium.
- **Real Wall:** Convergence may be slow (10⁴–10⁶ iterations); abstraction needed for large games (Libratus, Pluribus).
- **Cross-Domain Aliases:** regret-minimization (ml-training), game-theoretic-solution (control-numerical-opt).
- **Notes:** Neller & Lanctot (2013); used in no-limit poker (Libratus, 2017); CFR+ = averaging over regret-matched strategies.

### [PRIM-025] mental-rotation
- **Atom/Composite:** Primitive
- **Definition:** Mental rotation: cognitively rotate objects to compare; reaction time proportional to rotation angle; fMRI shows parietal cortex involvement.
- **Cost Model:** Rotation time = k × angle; linear with angle up to 180°; discontinuous at 180° (mirrored).
- **Real Wall:** Complex objects rotate slower; viewpoint invariance requires separate mental rotation; neural correlates specific to object type.
- **Cross-Domain Aliases:** spatial-transformation (linear-algebra-matrix), rotation-matrix (graphics-rendering-lod).
- **Notes:** Shepard & Metzler (1971); classical cognitive science benchmark for spatial reasoning.

## 4. Memory Systems

### [PRIM-026] episodic-memory
- **Atom/Composite:** Composite
- **Definition:** Episodic memory: autobiographical memory of specific events; what-where-when encoding; hippocampus-dependent; vulnerable to interference.
- **Cost Model:** Encoding: O(1) per event; retrieval: search + pattern completion; forgetting curves (Ebbinghaus).
- **Real Wall:** False memory (DRM paradigm); memory reconsolidation; retrograde amnesia (hippocampal damage).
- **Cross-Domain Aliases:** experience-replay (ml-training), trace-consolidation (biology-bioinformatics).
- **Notes:** Tulving (1972); episodic vs. semantic memory distinction; REMem (2026) formalizes episodic recollection for LLM agents.

### [PRIM-027] semantic-memory
- **Atom/Composite:** Composite
- **Definition:** Semantic memory: structured factual knowledge; not tied to personal experience; prefrontal cortex; slower to update than episodic.
- **Cost Model:** Retrieval: O(1) for known facts; inference over knowledge graph; conflict resolution for contradictory facts.
- **Real Wall:** Hallucination = false semantic memory; cataphora (mention before fact); knowledge conflicts in training data.
- **Cross-Domain Aliases:** knowledge-graph (retrieval-search), structured-facts (information-theory-coding).
- **Notes:** Collins & Quillian (1969) semantic network; spreading activation retrieval; retrieval vs. inference boundary.

### [PRIM-028] working-memory
- **Atom/Composite:** Primitive
- **Definition:** Working memory: active maintenance + manipulation of information; limited capacity (Miller's 7±2 chunks); phonological loop, visuospatial sketchpad, central executive.
- **Cost Model:** Maintenance cost = decay rate; interference from concurrent tasks; chunking reduces slot count.
- **Real Wall:** Cognitive load theory; dual-task interference; working memory capacity correlates with reasoning ability.
- **Cross-Domain Aliases:** context-maintenance (ml-training), attention-state (control-numerical-opt).
- **Notes:** Baddeley (1974); Baddeley-Hitch model; modern view: multiple components, not single store.

### [PRIM-029] procedural-memory
- **Atom/Composite:** Composite
- **Definition:** Procedural memory: skill-based memory (how to ride a bike, type); implicit, cerebellum/basal ganglia; resists explicit recall.
- **Cost Model:** Learning curve: exponential improvement with practice; power law of practice; automaticity threshold.
- **Real Wall:** Skill transfer limited; bad habits hard to overwrite; amnesia spares procedural (good for skill rehabilitation).
- **Cross-Domain Aliases:** weight-consolidation (ml-training), motor-program (biology-bioinformatics).
- **Notes:** Ryle (1949) "knowing how" vs. "knowing that"; habit formation = procedural memory consolidation.

### [PRIM-030] memory-consolidation
- **Atom/Composite:** Composite
- **Definition:** Memory consolidation: hippocampus-dependent short-term → neocortex-dependent long-term; synaptic consolidation (LTP) + systems consolidation (replay during sleep).
- **Cost Model:** Replay frequency (sharp-wave ripples in sleep); consolidation during offline periods; emotional salience enhances.
- **Real Wall:** Sleep disruption impairs consolidation; interference (proactive/retroactive); memory reconsolidation on reactivation.
- **Cross-Domain Aliases:** gradient-accumulation (ml-training), slow-weight-update (ml-training).
- **Notes:** McClelland et al. (1995) complementary learning systems; catastrophic forgetting in neural nets vs. gradual consolidation in brain.

### [PRIM-031] autobiographical-memory
- **Atom/Composite:** Composite
- **Definition:** Autobiographical memory: personal narrative integrating episodic + semantic + self-referential memory; forms life story; self-continuity.
- **Cost Model:** Construction from episodic fragments + semantic knowledge; reconstruction is reconstructive (not reproductive).
- **Real Wall:** Memory is reconstructive (vulnerable to suggestion); reminiscence bump (more memories from ages 10–30);flashbulb memories are not as accurate as they feel.
- **Cross-Domain Aliases:** life-history (agentic-reasoning), self-narrative (cognitive-reasoning).

### [PRIM-032] prospective-memory
- **Atom/Composite:** Primitive
- **Definition:** Prospective memory: remember to perform intended action in future (e.g., take medication at 8am); self-initiated retrieval triggered by cue.
- **Cost Model:** Cue detection vs. target retrieval; focal vs. non-focal prospective memory; cost in ongoing task performance.
- **Real Wall:** Prospective memory failures are common; forgetting after delay; importance/valuing affects remembering.
- **Cross-Domain Aliases:** scheduled-reminder (agentic-reasoning), future-intention (cognitive-reasoning).

## 5. Planning / Scheduling

### [PRIM-033] goal-hierarchy
- **Atom/Composite:** Composite
- **Definition:** Goal hierarchy: decompose high-level goal into subgoals; HTN (Hierarchical Task Network) planning; goal preconditions + effects.
- **Cost Model:** Goal tree depth × branching factor; HTN planning complexity is hard; nonlinear plans (interleaved subgoals).
- **Real Wall:** Goal conflicts; means-ends analysis may get stuck in local minima; goal abandonment vs. plan repair.
- **Cross-Domain Aliases:** task-decomposition (control-numerical-opt), hierarchical-planning (distributed-systems).
- **Notes:** HTN = hierarchical task network; non-hierarchical = linear planning (STRIPS, PDDL).

### [PRIM-034] planning-with-constraints
- **Atom/Composite:** Composite
- **Definition:** Constraint-based planning: CSP (constraint satisfaction) or SAT encoding; planning as satisfiability (SATPlan); scheduling with resource constraints.
- **Cost Model:** Constraint propagation reduces search; variable ordering heuristics; backtracking for CSP; worst-case NP-complete.
- **Real Wall:** Overconstrained problems (no solution); soft constraints (preferences); dynamic constraint addition (plan repair).
- **Cross-Domain Aliases:** constraint-satisfaction (control-numerical-opt), resource-allocation (distributed-systems).

### [PRIM-035] monte-carlo-tree-search
- **Atom/Composite:** Composite
- **Definition:** MCTS: simulation-based search for sequential decision problems; UCT (Upper Confidence Bound for Trees) balances exploration/exploitation; four phases: selection, expansion, simulation, backpropagation.
- **Cost Model:** O(b·d) per simulation; convergence: more simulations → better policy; parallel MCTS for speed.
- **Real Wall:** Cold start (no knowledge → random simulation); deterministic vs. stochastic MCTS; knowledge-guided rollouts.
- **Cross-Domain Aliases:** uct-search (control-numerical-opt), game-tree-search (linear-algebra-matrix).
- **Notes:** AlphaGo (Silver et al., 2016) combined MCTS + value network + policy network.

### [PRIM-036] partial-order-planning
- **Atom/Composite:** Composite
- **Definition:** POP: maintain partial order of actions; don't order unrelated actions; add ordering constraints as needed; least commitment principle.
- **Cost Model:** Plan space search; fewer commitments = more flexibility; plan refinement vs. plan space search.
- **Real Wall:** Multiple solutions (plan equivalence); least commitment may delay detection of unsolvable goals.
- **Cross-Domain Aliases:** partial-order-scheduling (control-numerical-opt), flexible-ordering (distributed-systems).
- **Notes:** Weld (1994) excellent overview; compared to total-order planners (LinearTask, HTN).

### [PRIM-037] planning-via-language-models
- **Atom/Composite:** Composite
- **Definition:** LLM-based planning: generate plan as text sequence; validate via execution or simulation; fix via re-prompting or critic.
- **Cost Model:** Plan generation = LLM forward pass; validation may require external planner or simulation; iterative refinement = multiple passes.
- **Real Wall:** Hallucinated actions (impossible steps); physical implausibility; LLM cannot ground actions in actual state space.
- **Cross-Domain Aliases:** language-planning (agentic-reasoning), llm-as-planner (cognitive-reasoning).
- **Notes:** Huang et al. (2022) "Language to Rewards"; LLMs as reward specifiers for robot control.

### [PRIM-038] least-commitment-planning
- **Atom/Composite:** Primitive
- **Definition:** Least commitment: don't make ordering decisions until necessary; wait to bind variables until needed; supports parallel execution.
- **Cost Model:** Deferred commitment increases search space flexibility; can lead to larger plan space but more executable plans.
- **Real Wall:** Execution requires commitment; late binding of variables may cause runtime errors.
- **Cross-Domain Aliases:** lazy-binding (distributed-systems), deferred-decision (control-numerical-opt).

### [PRIM-039] planning-with-time-windows
- **Atom/Composite:** Composite
- **Definition:** Temporal planning: actions have duration and temporal constraints; Simple Time Network (STN) for consistency checking; Temporal Plan Networks (TPN).
- **Cost Model:** STN consistency: O(N²) to check all constraints; flexible execution windows; deadline-aware scheduling.
- **Real Wall:** Temporal uncertainty (action duration unknown); conditional effects on time; external events disrupting plan.
- **Cross-Domain Aliases:** scheduling-optimization (control-numerical-opt), temporal-networks (linear-algebra-matrix).
- **Notes:** Dechter et al. (1991) temporal constraint networks; PDDL 2.1 added duration and continuous effects.

## 6. Tool Use / Function Calling

### [PRIM-040] tool-description-schema
- **Atom/Composite:** Primitive
- **Definition:** Tool schema: structured description of tool capability, parameters, types, constraints (OpenAI function calling, JSON Schema, or natural language).
- **Cost Model:** Schema parsing overhead; parameter validation; type checking; security: injection of malicious tool descriptions.
- **Real Wall:** Tool descriptions may be inaccurate (hallucinated capability); parameter type mismatches; tool conflicts (two tools with same name).
- **Cross-Domain Aliases:** api-contract (distributed-systems), interface-specification (information-theory-coding).
- **Notes:** Anthropic tool use, OpenAI function calling, ReAct with custom tools; schema quality determines tool use accuracy.

### [PRIM-041] tool-selection
- **Atom/Composite:** Composite
- **Definition:** Tool selection: given task, choose which tool(s) to call; single vs. batch selection; learned (model-based) vs. heuristic (retrieval or rule-based).
- **Cost Model:** Selection policy = LLM call or retrieval; cost per tool varies; wrong tool = failed execution + wasted cycles.
- **Real Wall:** Tool overload (many available tools → selection degrades); tool dependencies (output of tool A = input of tool B).
- **Cross-Domain Aliases:** action-selection (control-numerical-opt), function-router (distributed-systems).

### [PRIM-042] tool-execution
- **Atom/Composite:** Composite
- **Definition:** Tool execution: call tool with parameters; handle result, error, timeout; sandboxed execution for code/tools.
- **Cost Model:** Execution time varies (ms for API, s for code execution, minutes for data processing); timeout handling.
- **Real Wall:** Tool errors (API failures, rate limits, auth expiry); nondeterministic results; tool state may change between calls.
- **Cross-Domain Aliases:** function-call-execution (distributed-systems), api-invocation (networking).
- **Notes:** Tool use reliability: need retry logic, fallback, graceful degradation.

### [PRIM-043] tool-error-handling
- **Atom/Composite:** Composite
- **Definition:** Tool error handling: retry on transient failures (429, 500); circuit breaker pattern; fallback to alternative tool; graceful degradation.
- **Cost Model:** Retry budget (max N retries); exponential backoff (base^attempt × jitter); error categorization (retryable vs. fatal).
- **Real Wall:** Error message parsing (LLM interprets ambiguous errors); poison pill errors (always fail → loop without retry cap).
- **Cross-Domain Aliases:** retry-backoff (distributed-systems), error-recovery (control-numerical-opt).

### [PRIM-044] multi-tool-orchestration
- **Atom/Composite:** Composite
- **Definition:** Multi-tool orchestration: plan sequence of tool calls with dependencies; DAG of tool invocations; parallel tool calls where independent.
- **Cost Model:** Tool graph execution (topological sort); parallel tools reduce wall-clock time; synchronization barriers for dependent tools.
- **Real Wall:** Tool dependencies complex (output format of tool A may not match input of tool B); missing tools break orchestration.
- **Cross-Domain Aliases:** workflow-orchestration (distributed-systems), pipeline-composition (control-numerical-opt).
- **Notes:** ToolChain (Mei et al. 2024); DAG-based tool orchestration; VS. sequential single-tool calls.

### [PRIM-045] tool-result-grounding
- **Atom/Composite:** Primitive
- **Definition:** Tool result grounding: LLM interprets tool output (JSON, text, image, error) and incorporates into reasoning; avoid hallucinating facts from tool results.
- **Cost Model:** Result parsing complexity; schema alignment between tool output and LLM input format.
- **Real Wall:** LLM may ignore tool result ("hallucinated" content from tool output); long tool results truncated by context limit.
- **Cross-Domain Aliases:** result-interpretation (agentic-reasoning), output-grounding (ml-training).

### [PRIM-046] sandboxed-code-execution
- **Atom/Composite:** Composite
- **Definition:** Sandboxed code execution: run Python/JS in restricted environment (e-container, wasmtime, eBPF); timeout + memory limit; no filesystem or network access.
- **Cost Model:** Container spin-up time; execution time (limited CPU); memory limit enforcement; isolation overhead.
- **Real Wall:** Side-channel attacks via timing; resource exhaustion; sandbox escape vulnerabilities.
- **Cross-Domain Aliases:** isolated-execution (distributed-systems), sandboxed-process (cryptography-hashing).
- **Notes:** Used in Codex, Copilot, ChatGPT Code Interpreter; E(0) = trusted computing base for untrusted code.

### [PRIM-047] web-search-as-tool
- **Atom/Composite:** Composite
- **Definition:** Web search tool: query search engine, retrieve snippets; answer synthesis from top results; citation/grounding.
- **Cost Model:** Search API cost (per query); result count; snippet length; LLM processes top-k results.
- **Real Wall:** Search quality varies by query; LLM may prefer confident but wrong snippet; recency bias (recent = authoritative).
- **Cross-Domain Aliases:** retrieval-augmented-generation (retrieval-search), web-grounding (information-theory-coding).
- **Notes:** WebGPT (Microsoft); Bing Chat; retrieval-augmented tool use; tool can be search engine, wiki, or knowledge base.

### [PRIM-048] calculator-as-tool
- **Atom/Composite:** Primitive
- **Definition:** Calculator: precise arithmetic; symbolic math (integration, differentiation); unit conversion; date/time arithmetic.
- **Cost Model:** Computation time; precision (floating-point vs. arbitrary precision); symbolic vs. numeric.
- **Real Wall:** LLM arithmetic errors well-documented; round-off errors; unit conversion errors (temperature, currency).
- **Cross-Domain Aliases:** precise-arithmetic (control-numerical-opt), symbol-manipulation (linear-algebra-matrix).

### [PRIM-049] knowledge-graph-query
- **Atom/Composite:** Composite
- **Definition:** Knowledge graph query: traverse KG (Wikidata, ConceptNet) to retrieve structured facts; SPARQL or LLM-generated graph queries.
- **Cost Model:** Graph traversal depth; entity linking accuracy; subgraph extraction.
- **Real Wall:** KG coverage gaps; entity linking errors; LLM-generated queries may not match KG schema.
- **Cross-Domain Aliases:** graph-query (retrieval-search), structured-retrieval (information-theory-coding).

### [PRIM-050] retrieval-augmented-generation
- **Atom/Composite:** Composite
- **Definition:** RAG: retrieve relevant documents → inject into LLM context → generate answer; vector similarity search (ANN) + reranking.
- **Cost Model:** Retrieval latency; embedding cost; context length; reranking overhead (cross-encoder).
- **Real Wall:** Retrieval quality (recall vs. precision); context stuffing; stale retrieval; hallucination from retrieved content.
- **Cross-Domain Aliases:** context-augmentation (agentic-reasoning), document-grounding (information-theory-coding).
- **Notes:** Lewis et al. (2020); in-context learning via retrieved examples; real-time retrieval vs. static knowledge.

## 7. Metacognition / Reflection

### [PRIM-051] self-reflection
- **Atom/Composite:** Composite
- **Definition:** Self-reflection: agent evaluates its own reasoning/actions for correctness, coherence, completeness; Reflexion (Shinn et al.) stores verbal reflection as episodic memory.
- **Cost Model:** Reflection LLM call per step; reflection storage + retrieval; self-critique vs. external critique.
- **Real Wall:** Self-deception (confident but wrong); reflection quality depends on reasoning capability; may reinforce errors.
- **Cross-Domain Aliases:** self-critique (agentic-reasoning), reflective-learning (ml-training).
- **Notes:** Shinn et al. (2023) Reflexion; self-reflection without parameter updates (verbal reinforcement learning).

### [PRIM-052] confidence-calibration
- **Atom/Composite:** Composite
- **Definition:** Confidence calibration: predicted probability matches empirical frequency; ECE (Expected Calibration Error); temperature scaling, Platt scaling, isotonic regression.
- **Cost Model:** Calibration measurement (ECE); temperature tuning on held-out set; overconfident models degrade downstream decisions.
- **Real Wall:** LLMs are systematically overconfident; calibration doesn't improve accuracy, only reliability of probabilities.
- **Cross-Domain Aliases:** probability-calibration (ml-training), belief-updating (control-numerical-opt).
- **Notes:** Guo et al. (2017) on calibration of neural nets; LLM calibration studied via MMLU, TruthfulQA.

### [PRIM-053] reasoning-monitoring
- **Atom/Composite:** Primitive
- **Definition:** Reasoning monitoring: track reasoning state (what am I doing? what have I done? what's left?); detect reasoning loops, contradictions, dead ends.
- **Cost Model:** O(1) to log current step; O(N) to scan history for loops; monitor cost adds to total reasoning cost.
- **Real Wall:** Reasoning loops: LLM repeats same steps; contradiction detection requires cross-step consistency check.
- **Cross-Domain Aliases:** loop-detection (control-numerical-opt), state-tracking (distributed-systems).

### [PRIM-054] uncertainty-quantification
- **Atom/Composite:** Composite
- **Definition:** UQ: quantify uncertainty in reasoning/output; aleatoric (irreducible) vs. epistemic (reducible with more data); Bayesian neural nets, MC dropout, ensemble.
- **Cost Model:** Ensemble cost O(N) for N models; MC dropout cost O(T) for T samples; test-time augmentation cost.
- **Real Wall:** Overconfidence; spurious certainty; LLM outputs are deterministic (no natural uncertainty); prompting for "I don't know" is unreliable.
- **Cross-Domain Aliases:** bayesian-uncertainty (ml-training), entropy-estimation (information-theory-coding).

### [PRIM-055] instruction-following-verification
- **Atom/Composite:** Composite
- **Definition:** Instruction verification: check if output satisfies all constraints in instruction; constraint satisfaction vs. constraint relaxation; graded verification (all vs. most constraints).
- **Cost Model:** Constraint extraction (LLM); per-constraint check; partial credit for partial satisfaction.
- **Real Wall:** Ambiguous instructions (constraint not clearly stated); constraint conflicts; verifiability (some constraints hard to check).
- **Cross-Domain Aliases:** constraint-check (agentic-reasoning), output-validation (ml-training).
- **Notes:** IFEval (Zhou et al. 2023); instruction-following score; 25 types of verifiable constraints.

### [PRIM-056] constitutional-ai
- **Atom/Composite:** Composite
- **Definition:** Constitutional AI: self-critique against principles (constitution); identify violations → revise; SL+CAI (Supervision + Constitutional) for harmless, helpful responses.
- **Cost Model:** Multiple LLM passes (initial response → critique → revision); constitutional principles as text; iterative revision until all principles satisfied.
- **Real Wall:** Principles may conflict; principle hierarchy; constitutional review doesn't guarantee safety (adversarial principles).
- **Cross-Domain Aliases:** principle-critique (agentic-reasoning), policy-constraint (control-numerical-opt).
- **Notes:** Bai et al. (2022); RLHF vs. CAI (no human labels for harm identification); principles as structured constraints.

### [PRIM-057] chain-of-verification
- **Atom/Composite:** Composite
- **Definition:** CoV (Chain of Verification): generate response → list verification questions → answer independently → revise original if contradictions found.
- **Cost Model:** 2–3× LLM passes (generate, verify, revise); verification questions = explicit fact checks.
- **Real Wall:** Verification questions may miss key facts; LLM may confirm its own errors (confirmation bias); completeness of verification list is key.
- **Cross-Domain Aliases:** fact-checking (agentic-reasoning), verification-loop (ml-training).

### [PRIM-058] self-consistency-verification
- **Atom/Composite:** Composite
- **Definition:** Self-consistency check: generate multiple solutions, compare answers, accept if agreement, probe disagreement otherwise.
- **Cost Model:** O(n) forward passes; majority vote for answer; divergence analysis for disagreements.
- **Real Wall:** Different wrong answers can still majority-vote incorrectly; requires answer extractability; trade-off: accuracy vs. compute.
- **Cross-Domain Aliases:** multi-sample-consistency (agentic-reasoning), answer-agreement (ml-training).

## 8. Compositionality / Generalization

### [PRIM-059] compositional-generalization
- **Atom/Composite:** Composite
- **Definition:** Compositional generalization: combine known primitives in novel ways; SCAN (Simple Compositional Language) benchmark; gSCAN spatial reasoning benchmark.
- **Cost Model:** Generalization test: train on subset of compositions, test on held-out; unseen combinations reveal compositional gaps.
- **Real Wall:** Neural nets memorize compositions (no systematic generalization); systematicity vs. productivity debate in cognitive science.
- **Cross-Domain Aliases:** systematic-generalization (ml-training), novel-composition (cognitive-reasoning).
- **Notes:** Lake & Baroni (2018) SCAN; Winsor et al. (2025) gSCAN; transformer compositional abilities vs. human systematicity.

### [PRIM-060] analogical-reasoning
- **Atom/Composite:** Composite
- **Definition:** Analogical reasoning: map structure from source domain to target; surface vs. structural similarity; proportion analogy (A:B :: C:D).
- **Cost Model:** Source retrieval (memory search) + mapping (structure alignment) + transfer; analogical mapping algorithm (SME).
- **Real Wall:** Surface similarity distracts from structural; analogy quality depends on source domain selection; analogical reasoning is slow.
- **Cross-Domain Aliases:** structure-mapping (linear-algebra-matrix), cross-domain-transfer (ml-training).
- **Notes:** Gentner (1983) structure-mapping theory; Dedre Gentner's work; Hofstadter on analogy as core of cognition.

### [PRIM-061] zero-shot-generalization
- **Atom/Composite:** Composite
- **Definition:** Zero-shot: perform task without task-specific training; prompt engineering + in-context learning; depends on pre-training distribution.
- **Cost Model:** Prompt design cost (iterative); in-context examples (k-shot) vs. zero; task complexity affects zero-shot difficulty.
- **Real Wall:** Out-of-distribution tasks degrade; zero-shot performance on novel tasks is unpredictable; prompting sensitivity.
- **Cross-Domain Aliases:** task-transfer (ml-training), in-context-learning (cognitive-reasoning).
- **Notes:** Brown et al. (2020) GPT-3 in-context learning; zero-shot = k=0 in-context; depends on pre-training data.

### [PRIM-062] few-shot-generalization
- **Atom/Composite:** Composite
- **Definition:** Few-shot: k demonstrations in prompt; in-context learning (ICL); semantic vs. syntactic demonstrations; selection of informative examples.
- **Cost Model:** k demonstrations = k × (example length + inference cost); optimal k varies by task; diminishing returns beyond k=16.
- **Real Wall:** Label noise in demonstrations; example ordering bias; misaligned examples hurt more than help.
- **Cross-Domain Aliases:** in-context-learning (ml-training), demonstration-selection (retrieval-search).

### [PRIM-063] meta-learning
- **Atom/Composite:** Composite
- **Definition:** Meta-learning: learn to learn; MAML (Model-Agnostic Meta-Learning) finds init that adapts quickly; Reptile; transformer-based meta-learners.
- **Cost Model:** Meta-training over tasks; inner loop (adaptation) + outer loop (meta-update); computational cost high.
- **Real Wall:** Meta-overfitting (fits meta-training tasks but not new ones); domain shift between meta-train and meta-test tasks.
- **Cross-Domain Aliases:** fast-adaptation (ml-training), learning-to-learn (cognitive-reasoning).
- **Notes:** Finn et al. (2017) MAML; Schmidhuber (1987) self-referential learning; meta-learning for few-shot adaptation.

### [PRIM-064] continual-learning
- **Atom/Composite:** Composite
- **Definition:** Continual learning: learn tasks sequentially without forgetting; catastrophic forgetting problem; regularization (EWC, SI), replay (ER), architectural (packnet).
- **Cost Model:** Forgetting = performance drop on old tasks; rehearsal buffer size; regularization strength vs. plasticity.
- **Real Wall:** Storage budget for replay; task boundaries may be unclear; temporal drift in data distribution.
- **Cross-Domain Aliases:** incremental-learning (ml-training), knowledge-retention (cognitive-reasoning).
- **Notes:** Parisi et al. (2019) continual learning survey; lifelong learning in biological systems vs. catastrophic forgetting in NNs.

### [PRIM-065] curriculum-learning
- **Atom/Composite:** Composite
- **Definition:** Curriculum learning: train on easy tasks first, then progressively harder; spacing effect; difficulty rating; teacher-student curriculum.
- **Cost Model:** Curriculum design cost (task ordering); automatic curriculum via PEBEL, self-paced learning.
- **Real Wall:** Hard-to-easy (reverse curriculum) sometimes works better; not all curricula help; task difficulty estimation is hard.
- **Cross-Domain Aliases:** progressive-training (ml-training), staged-complexity (control-numerical-opt).
- **Notes:** Elman (1993) first curriculum experiments; Bengio et al. (2009) formal curriculum learning; Zaremba & Sutskever (2014) curriculum for LSTMs.

## 9. Abstraction / Generalization

### [PRIM-066] abstraction-hierarchy
- **Atom/Composite:** Composite
- **Definition:** Abstraction hierarchy: multiple levels of representation (symbolic → subsymbolic → perceptual); levels of abstraction in explanation.
- **Cost Model:** Abstraction cost = information loss vs. generalization gain; too much abstraction loses detail; too little loses generality.
- **Real Wall:** Cross-level reasoning (grounding abstract concepts in concrete instances); abstraction is domain-dependent.
- **Cross-Domain Aliases:** level-of-detail (graphics-rendering-lod), hierarchical-representation (linear-algebra-matrix).

### [PRIM-067] schema-induction
- **Atom/Composite:** Composite
- **Definition:** Schema induction: learn structured templates from examples; scripts (Schank & Abelson) for events; frame schemas; slot-filler structures.
- **Cost Model:** Schema matching + instantiation; slot filling cost; schema conflict resolution.
- **Real Wall:** Schema rigidity (applies even when inappropriate); schema updating vs. retention; cultural variation in schemas.
- **Cross-Domain Aliases:** template-learning (information-theory-coding), slot-filling (retrieval-search).
- **Notes:** Bartlett (1932) schema; Rumelhart (1980) schema theory; modern: language models encode schemas from text.

### [PRIM-068] symbolic-manipulation
- **Atom/Composite:** Primitive
- **Definition:** Symbolic manipulation: formal rule application on symbolic expressions; algebraic simplification; logic inference (forward/backward chaining); term rewriting.
- **Cost Model:** Rule application cost; search space explosion (unification); early termination heuristics.
- **Real Wall:** Symbolic vs. neural trade-off; hybrid (neural-symbolic) systems; grounding symbols to meaning.
- **Cross-Domain Aliases:** rule-application (control-numerical-opt), term-rewriting (information-theory-coding).
- **Notes:** Newell & Simon's Physical Symbol System Hypothesis; neural theorem provers (Leo III, Coqhammer); GPT-4 = neural + symbolic.

### [PRIM-069] concept-induction
- **Atom/Composite:** Composite
- **Definition:** Concept induction: learn conceptual categories from examples; prototype theory (central tendency) vs. exemplar theory (store all examples) vs. theory theory (causal explanation).
- **Cost Model:** Prototype computation (average); exemplar storage (memory cost); theory building (causal model).
- **Real Wall:** Category boundary vagueness; basic level advantage; graded membership; conceptual change (Kuhn).
- **Cross-Domain Aliases:** category-learning (ml-training), prototype-matching (retrieval-search).
- **Notes:** Rosch (1975) prototype theory; natural categories are not classical (Aristotelian necessary and sufficient conditions).

### [PRIM-070] explanation-generation
- **Atom/Composite:** Composite
- **Definition:** Explanation generation: produce causal/mechanistic account of why something happened or why a decision was made; contrastive explanations (why X, not Y?).
- **Cost Model:** LLM-based explanation generation; counterfactual vs. mechanistic; explanation length vs. fidelity trade-off.
- **Real Wall:** Explanations can be post-hoc rationalizations (not true causes); LLM may generate plausible but wrong explanations.
- **Cross-Domain Aliases:** causal-attribution (agentic-reasoning), rationale-generation (cognitive-reasoning).
- **Notes:** Miller (2018) "Explanation in AI"; contrastive (Lipton); generative (why-mechanism); contrastive is preferred.

## 10. Social / Collaborative Reasoning

### [PRIM-071] multi-agent-deliberation
- **Atom/Composite:** Composite
- **Definition:** Multi-agent deliberation: multiple agents discuss, vote, or negotiate to reach consensus; truth-tracking via majority or deliberation dynamics.
- **Cost Model:** Communication rounds × number of agents; consensus reaching cost; lying/manipulation detection.
- **Real Wall:** Social loafing; groupthink; agents may converge on wrong answer (Delhi-iliuzo effect); diverse agents improve deliberation.
- **Cross-Domain Aliases:** consensus-formation (distributed-systems), multi-agent-voting (ml-training).
- **Notes:** Agents of different capabilities (e.g., expert + generalist) improve collective reasoning; simulated debate.

### [PRIM-072] debate-as-verification
- **Atom/Composite:** Composite
- **Definition:** Agent debate: two agents argue for/against a claim; judge evaluates arguments; truth-tracking improves with debate vs. single agent.
- **Cost Model:** Two agents × rounds; judge evaluation; debaters can introduce false claims.
- **Real Wall:** Asymmetric debate (one side stronger); judge susceptibility to rhetoric; computational cost of multiple debaters.
- **Cross-Domain Aliases:** adversarial-verification (ml-training), critique-exchange (agentic-reasoning).

### [PRIM-073] recursive-belonging-attribution
- **Atom/Composite:** Composite
- **Definition:** Recursive belief attribution: "I believe you believe I believe X"; n-order theory of mind; each layer adds uncertainty; practical limit ~3–4 levels.
- **Cost Model:** Exponential branching (2^n for binary beliefs); n-order ToM requires (n-1) lower-order beliefs as preconditions.
- **Real Wall:** Human performance drops at 3+ levels; practical limit on recursion depth; not always well-defined (ambiguous belief content).
- **Cross-Domain Aliases:** nested-theory-of-mind (cognitive-reasoning), recursive-agency (agentic-reasoning).
- **Notes:** Stalnaker (2008) on common knowledge; common knowledge = infinite recursion; rarely achieved in practice.

### [PRIM-074] authority-reasoning
- **Atom/Composite:** Primitive
- **Definition:** Authority reasoning: defer to credible source; credential checking; credential → trust → adoption; reputation systems.
- **Cost Model:** Credential verification cost; reputation aggregation; authority vs. content quality.
- **Real Wall:** Authority bias (defer even when wrong); fake credentials; credential inflation.
- **Cross-Domain Aliases:** source-credibility (information-theory-coding), credential-trust (cognitive-reasoning).

### [PRIM-075] adversarial-robustness-reasoning
- **Atom/Composite:** Composite
- **Definition:** Adversarial reasoning: anticipate adversarial perturbations; worst-case analysis; red-teaming; prompt injection defense.
- **Cost Model:** Red-teaming cost (human + LLM); adversarial training; worst-case robustness evaluation.
- **Real Wall:** Prompt injection (system prompt override via user input); jailbreaks; distribution of adversarial inputs unknown.
- **Cross-Domain Aliases:** adversarial-training (ml-training), worst-case-analysis (control-numerical-opt).

## Appendix: Primitive Count

Total primitives in agentic-reasoning domain: **75**
