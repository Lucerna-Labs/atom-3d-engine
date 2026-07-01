# Formal Verification Domain Primitives
> Cross-domain wiring: model checking = state-space enumeration; theorem proving = symbolic deduction; symbolic execution = program → formula; abstract interpretation = Galois connection between concrete and abstract domains

## 1. Model Checking

### [PRIM-001] state-transition-system
- **Atom/Composite:** Primitive
- **Definition:** Model of a system as states and transitions: TS = (S, S₀, Act, →, AP, L). States S, initial states S₀ ⊆ S, actions Act, transition relation → ⊆ S × Act × S, atomic propositions AP, labeling L: S → 2^AP.
- **Cost Model:** State space size = |S|; explicit model checking enumerates all states; symbolic uses BDDs or SAT solvers.
- **Real Wall:** State explosion: |S| grows exponentially with components; compositional methods mitigate but don't solve.
- **Cross-Domain Aliases:** finite-state-machine (control-numerical-opt), state-graph (linear-algebra-matrix).
- **Notes:** Basis of model checking (Clarke, Emerson, Sifakis — Turing Award 2007).

### [PRIM-002] computation-tree-logic
- **Atom/Composite:** Primitive
- **Definition:** CTL: branching-time temporal logic with path quantifiers (A = all paths, E = exists path) and temporal operators (X = next, F = eventually, G = always, U = until, W = weak until). Model checking: labeling algorithm.
- **Cost Model:** CTL model checking: O(|TS| · |φ|) via fixed-point algorithms; symbolic CTL: O(BDD size · |φ|).
- **Real Wall:** Expressive enough for many properties; CTL cannot express some properties (e.g., "always eventually p holds on all paths" = AG EF p); CTL* (full branching time) is more expressive.
- **Cross-Domain Aliases:** branching-temporal-logic (control-numerical-opt), path-quantifier-logic (information-theory-coding).
- **Notes:** Model checking algorithm by Clarke & Emerson (1981); symbolic CTL by McMillan (1992) using BDDs.

### [PRIM-003] linear-temporal-logic
- **Atom/Composite:** Primitive
- **Definition:** LTL: temporal logic over single execution path; operators: X (next), U (until), R (release), F (finally), G (globally). Encodes ω-regular properties. Safety (G ¬bad), liveness (F good), fairness (GF can).
- **Cost Model:** LTL → ω-automaton (Büchi): exponential blowup; automata-theoretic model checking: product of TS and automaton; complexity O(|TS| · 2^|φ|).
- **Real Wall:** LTL model checking is PSPACE-complete (in |TS|); explicit-state MC has complexity O(|TS| · 2^|φ|); symbolic is PSPACE in practice.
- **Cross-Domain Aliases:** linear-path-logic (control-numerical-opt), omega-regular (information-theory-coding).
- **Notes:** LTL is a subset of CTL*; used in hardware verification (IBM, Intel), protocol verification (SPIN).

### [PRIM-004] bulm-model-checking
- **Atom/Composite:** Primitive
- **Definition:** Bounded Model Checking (BMC): unroll the state transition system for k steps and check if a property violation (counterexample) exists in ≤ k steps. Encode as SAT or SMT formula: ¬φ at step k.
- **Cost Model:** SAT/SMT solving: O(k · |TS|) unrolling; NP-complete per step; incremental SAT (push/pop) for increasing k.
- **Real Wall:** Completeness threshold: k must reach the diameter of the state graph; for liveness, need deeper unrolling; k-induction (inductive invariants for k > diameter).
- **Cross-Domain Aliases:** bounded-unroll-check (control-numerical-opt), sat-model-checking (information-theory-coding).
- **Notes:** CBMC (C bounded model checker); used in software model checking (Java, C); BLAST, LLBMC.

### [PRIM-005] bdd-symbolic-model-checking
- **Atom/Composite:** Composite
- **Definition:** Binary Decision Diagram (BDD): canonical representation of Boolean functions; ROBDD = reduced ordered BDD (unique canonical form). BDD-based symbolic model checking uses BDDs to represent state sets and transition relations.
- **Cost Model:** BDD size varies wildly (exponential in worst case, compact for many practical functions); variable ordering critical (heuristics: window permutation, sifting); BDD operations = O(|BDD|).
- **Real Wall:** BDD explosion for some functions (multipliers, cryptographic functions); variable ordering problem is NP-hard; dynamic variable reordering helps.
- **Cross-Domain Aliases:** boolean-function-canonical (linear-algebra-matrix), symbolic-state-enumeration (control-numerical-opt).

### [PRIM-006] counterexample-guided-abstraction-refinement
- **Atom/Composite:** Composite
- **Definition:** CEGAR: iteratively (1) abstract model → (2) check abstract model → (3) if abstract counterexample is spurious, refine abstraction using Craig interpolation → repeat. Used for infinite-state or large finite-state systems.
- **Cost Model:** Each iteration: abstract MC + spurious path analysis + interpolation; converges in O(n) iterations where n = number of relevant predicates.
- **Real Wall:** Spurious counterexamples must be traceable to predicates; predicate discovery is the hard part; too many predicates → abstraction too concrete (BDD explosion).
- **Cross-Domain Aliases:** abstraction-refinement-loop (control-numerical-opt), iterative-abstract-check (information-theory-coding).
- **Notes:** McMillan (2002) applied CEGAR to infinite-state systems; Impact on software model checking (CPAChecker, SLAM/Blast).

### [PRIM-007] partial-order-reduction
- **Atom/Composite:** Primitive
- **Definition:** POR: reduce state space by exploring only one representative execution per equivalence class of concurrent events. Ample set, persistent set, sleep set algorithms. Independence → commutativity → reduced exploration.
- **Cost Model:** O(|S|) → O(|S|/k) where k = concurrency factor; static POR analysis (independence of actions) or dynamic; stubborn set reduction.
- **Real Wall:** POR must preserve properties being checked (safety vs. liveness require different reductions); deadlocks may be missed without care; not all reductions are interchangeable.
- **Cross-Domain Aliases:** state-space-reduction (distributed-systems), commutativity-exploit (control-numerical-opt).
- **Notes:** Peled (1993) partial order reduction; used in SPIN model checker; combined with BDDs for scalable hardware MC.

### [PRIM-008] symmetry-reduction
- **Atom/Composite:** Primitive
- **Definition:** Exploit symmetries in the state space (e.g., interchangeable processes, identical components) to reduce the explored state graph. Quotient system = original / symmetry group.
- **Cost Model:** Symmetry detection: group-theoretic analysis; quotient construction: orbits of states; canonical representative per orbit.
- **Real Wall:** Symmetric systems are a special case; detecting automorphism group of labeled transition system is expensive; orbit bisimulation.
- **Cross-Domain Aliases:** group-symmetry-reduce (linear-algebra-matrix), isomorphic-state-collapse (distributed-systems).
- **Notes:** Clarke & Jha (1993) symmetry reduction for CCS processes; used in Murphi protocol verifier.

### [PRIM-009] inductive-invariant
- **Atom/Composite:** Composite
- **Definition:** Inductive invariant: property P is an inductive invariant of TS if (1) P holds initially (S₀ ⊆ P) and (2) P is preserved by all transitions (P ∧ ¬P' is unreachable). Proving P by induction over the state graph.
- **Cost Model:** Checking inductiveness: O(|TS|) per invariant candidate; finding inductive invariants: fixed-point computation (EG → EU → EX operators); invariant synthesis from templates.
- **Real Wall:** Finding strong enough invariants (without false negatives) is hard; over-approximations can be too weak (miss violations); under-approximations miss real bugs.
- **Cross-Domain Aliases:** invariant-checking (control-numerical-opt), fixed-point-logic (information-theory-coding).

### [PRIM-010] k-induction
- **Atom/Composite:** Composite
- **Definition:** k-induction: prove safety by showing (1) base case: no violation in first k steps, (2) inductive step: from any state at step i, no violation at i+k. Unrolling + induction over path length.
- **Cost Model:** Base case: bounded model check for k steps; inductive step: unrolled transition for k steps + SAT/SMT solving.
- **Real Wall:** k must be ≥ the longest counterexample length (diameter) for completeness; too small k = not inductive (fails); too large k = expensive.
- **Cross-Domain Aliases:** bounded-inductive-proof (control-numerical-opt), incremental-unroll (information-theory-coding).
- **Notes:** Used in CBMC, K-Inductor; combined with invariant generation (IC3/PDR) for more power.

### [PRIM-011] interpolation-model-checking
- **Atom/Composite:** Composite
- **Definition:** Craig interpolation: given A ∧ B = unsatisfiable, there exists an interpolant I such that A → I and I ∧ B = unsatisfiable. Used in BMC counterexample refinement (Craig → predicate abstraction).
- **Cost Model:** Interpolant computation depends on SAT solver internals (resolution proof); MIP-based or theory-specific interpolation; complexity O(proof size).
- **Real Wall:** Interpolant strength (how much itgeneralizes from refutation) varies by proof system; theory-specific interpolants are stronger but require theory solvers.
- **Cross-Domain Aliases:** craig-interpolation (information-theory-coding), formula-refinement (control-numerical-opt).
- **Notes:** McMillan (2003) applied interpolation to software model checking; lazy abstraction with interpolants = Impact algorithm.

### [PRIM-012] fairness-constraint
- **Atom/Composite:** Primitive
- **Definition:** Fairness constraint: restrict attention to fair execution paths where certain actions occur infinitely often (or sufficiently often). Weak fairness (continuously enabled → eventually taken), strong fairness (infinitely often enabled → infinitely often taken).
- **Cost Model:** Fair model checking adds fairness automata (Büchi) to product TS; LTL with fairness: G F φ (globally infinitely often φ); complexity increases by factor of fairness automaton size.
- **Real Wall:** Specifying fairness correctly is subtle; weak vs. strong fairness has different implications for liveness properties; over-fairness can make bugs unreachable.
- **Cross-Domain Aliases:** fairness-abstraction (control-numerical-opt), infinite-path-quantifier (information-theory-coding).

### [PRIM-013] parametric-model-checking
- **Atom/Composite:** Composite
- **Definition:** Model checking where system parameters (timing constants, probabilities) are symbolic; determine for which parameter values the system satisfies a property. Parameter synthesis problem.
- **Cost Model:** Synthesis region in parameter space; symbolic representation as polyhedra (real parameters) or finite set (discrete parameters); complexity is coNP-hard in general.
- **Real Wall:** Parameter space may be continuous; abstraction needed; most systems are parameterizable in bounded ranges.
- **Cross-Domain Aliases:** parameter-synthesis (control-numerical-opt), symbolic-parameter-space (linear-algebra-matrix).

### [PRIM-014] real-time-model-checking
- **Atom/Composite:** Composite
- **Definition:** Model checking timed systems: extensions of LTS with clocks (Timed Automata, UPPAAL). TCTL (Timed CTL) adds clock constraints. Zone graph = symbolic representation of clock regions.
- **Cost Model:** Zone graph: finite abstraction of infinite state space; state space size = O(|clocks| · C_max); UPPAAL uses difference bound matrices (DBMs).
- **Real Wall:** Zone explosion (many clocks, large constants); extrapolation (abstraction) needed for large constants; real-time model checking is PSPACE-complete.
- **Cross-Domain Aliases:** timed-automata (control-numerical-opt), clock-region-abstraction (linear-algebra-matrix).

### [PRIM-015] probabilistic-model-checking
- **Atom/Composite:** Composite
- **Definition:** Model checking Markov chains/decision processes for probabilistic properties: PCTL (probabilistic CTL), MDP (Markov Decision Process). Compute probability of reaching a state set, expected reward until absorption.
- **Cost Model:** DTMC model checking: O(|S|³) for solving linear equations (reachability); MDP: linear programming or value iteration O(|S| · |Act| · |T|); DTMC = discrete time, CTMC = continuous time.
- **Real Wall:** State space explosion (product of components); numerical solution of linear systems (precision vs. performance); MDP synthesis (optimal strategy existence).
- **Cross-Domain Aliases:** probabilistic-reachability (ml-training), markov-chain-check (control-numerical-opt).
- **Notes:** PRISM model checker (Hinton et al.); used for reliability analysis, network protocol correctness, robotics.

### [PRIM-016] spin-promela-model-checking
- **Atom/Composite:** Composite
- **Definition:** SPIN (Simple Promela Interpreter): explicit-state model checker for distributed software/protocols. Input: Promela specification (processes, channels, assertions). LTL model checking via Büchi automata.
- **Cost Model:** State space: explicit enumeration; compression via minimisation; on-the-fly MC (don't build full state space); worst-case O(|S|) memory and time.
- **Real Wall:** State explosion limits scalability; partial order reduction (por) is critical for concurrent protocols; compression helps but doesn't solve explosion.
- **Cross-Domain Aliases:** protocol-verification (distributed-systems), promela-verifier (control-numerical-opt).
- **Notes:** Holtzmann (1997) SPIN; used in bug-finding for aerospace (Plan 9, Cadar et al.), network protocols; saved the Mars Pathfinder (priority inversion bug).

## 2. Theorem Proving

### [PRIM-017] higher-order-logic
- **Atom/Composite:** Primitive
- **Definition:** Higher-order logic (HOL): logic with quantification over functions and predicates; more expressive than first-order logic. Type theory foundation (simple type theory: →, ×, →, ∀, λ-abstraction).
- **Cost Model:** Proof search is undecidable; interactive (human-guided) theorem proving; automation (SMT solver integration, decision procedures, hammer tools).
- **Real Wall:** Expressiveness vs. automation trade-off; higher-order unification is complex; type checking prevents most logical inconsistencies.
- **Cross-Domain Aliases:** higher-order-deduction (information-theory-coding), type-system-logic (programming-language-theory).
- **Notes:** HOL4, Isabelle/HOL, Coq (CIC with inductive types), Lean; used in hardware (浮点 proving), security protocols, compilers.

### [PRIM-018] constructive-type-theory
- **Atom/Composite:** Composite
- **Definition:** Calculus of Inductive Constructions (CIC, Coq) or Calculus of Constructions with universes; types depend on terms (dependent types); propositions as types (Curry-Howard correspondence); proof terms are programs.
- **Cost Model:** Type checking: O(size of term); universe consistency checking (Cumulative universes); definitional equality vs. propositional equality.
- **Real Wall:** Universe inconsistency (set混入) is real; computational content of proofs can be extracted (proof extraction); decidability of type checking but not of proof search.
- **Cross-Domain Aliases:** curry-howard (programming-language-theory), dependent-type-proof (information-theory-coding).
- **Notes:** Four color theorem (Coq), odd order theorem (Coq, 2012); Lean 4 for mathematics (liquid tensor experiment, perfectoid spaces).

### [PRIM-019] sequent-calculus
- **Atom/Composite:** Primitive
- **Definition:** Gentzen sequent calculus: judgments of the form Γ ⊢ Δ (from Γ derive at least one formula from Δ). Inference rules: left/right rules for each connective; cut elimination (Gentzen's Hauptsatz): cut is admissible.
- **Cost Model:** Proof search via back-chaining (from goal to subgoals); proof complexity = tree depth; cut elimination removes cuts (more analytic proofs).
- **Real Wall:** Cut-free sequent proofs can be exponentially larger than proofs with cuts; proof search complexity is PSPACE in general.
- **Cross-Domain Aliases:** sequent-proof (information-theory-coding), backward-deduction (control-numerical-opt).

### [PRIM-020] natural-deduction
- **Atom/Composite:** Primitive
- **Definition:** Natural deduction (Gentzen, Prawitz): proof rules for each connective that mirror intuitive reasoning; introduction rules (how to prove) and elimination rules (how to use). Normalization = proof simplification (no detours).
- **Cost Model:** Normalization algorithm (β-reduction, η-expansion); proof terms as λ-terms; type inference via bidirectional typing.
- **Real Wall:** Normalization ensures no "proof by contradiction" detours; dependent types require type annotations; universe management in CIC.
- **Cross-Domain Aliases:** intro-elim-rules (programming-language-theory), lambda-proof (linear-algebra-matrix).

### [PRIM-021] smt-solving-integration
- **Atom/Composite:** Composite
- **Definition:** SMT solver (Z3, CVC5, Princess): SAT solver + theory solvers for arithmetic (LIA, NRA), arrays, bitvectors, strings. Decides satisfiability of formulas in decidable theories.
- **Cost Model:** DPLL(T): SAT solver manages Boolean structure, theory solver handles theory atoms; Nelson-Oppen: theory combination (agree on shared variables); O(n^2) for n theories in naive combination.
- **Real Wall:** Theory explosion; some theories are undecidable (real closed fields with exponentiation); interpolation support varies by solver.
- **Cross-Domain Aliases:** smt-theory-solving (control-numerical-opt), boolean-theory-solver (information-theory-coding).
- **Notes:** SMT-LIB standard format; used in bounded model checking, symbolic execution, program verification, test generation.

### [PRIM-022] coinductive-proof
- **Atom/Composite:** Primitive
- **Definition:** Coinduction: proof principle for potentially infinite structures (streams, processes, lazy data). Greatest fixed point (μ) as opposed to inductive least fixed point (ν). Coinductive hypothesis = assumption of the property to be proved.
- **Cost Model:** Coinductive proof search = greatest fixed point computation; regular tree structures; bisimulation as coinductive equality.
- **Real Wall:** Guardedness condition (ensures productivity); opportune use of "corecursion" for infinite objects; coinductive types in proof assistants (Coq's coinductive).
- **Cross-Domain Aliases:** greatest-fixedpoint (control-numerical-opt), bisimulation-equivalence (distributed-systems).
- **Notes:** Coinductive definitions: infinite streams (0,1,0,1,…); proof of stream bisimulation; used for protocol equivalence, behavioral types.

### [PRIM-023] tla-specification
- **Atom/Composite:** Composite
- **Definition:** TLA+ (Temporal Logic of Actions): high-level specification language for concurrent and distributed systems; PlusCal = algorithmic language that compiles to TLA+; model checking via TLC (explicit) or TLAPS (proof).
- **Cost Model:** PlusCal → TLA+ compilation (simple); TLC model checker: explicit state enumeration; TLAPS: proof system for TLA+ theorems ( Isabelle/HOL backend).
- **Real Wall:** State explosion limits model checking (use TLC for bounded instances); PlusCal algorithm must be finite-state for model checking; infinite-state needs proof (TLAPS).
- **Cross-Domain Aliases:** temporal-specification (distributed-systems), algorithm-model-check (control-numerical-opt).
- **Notes:** Lamport (Turing Award 2013); AWS uses TLA+ for DynamoDB, S3; critical bugs found (Azure Cosmos DB, etc.).

### [PRIM-024] induction-principle
- **Atom/Composite:** Primitive
- **Definition:** Mathematical induction: prove P(0) ∧ (P(n) → P(n+1)) → P(n) for all n. Structural induction on data types; well-founded induction on measure. Core to all theorem provers.
- **Cost Model:** Induction schema application; recursive function definitions → induction theorems (primrec); mutual induction for mutually recursive definitions.
- **Real Wall:** Induction is powerful but requires finding the right induction scheme; non-terminating recursion breaks induction; induction over measure is key for termination.
- **Cross-Domain Aliases:** structural-induction (programming-language-theory), recursion-proof (control-numerical-opt).

### [PRIM-025] rewriting-and-termination
- **Atom/Composite:** Composite
- **Definition:** Term rewriting: normalize expressions by applying directed equations. Confluence (Church-Rosser): no matter the order, same normal form. Termination: no infinite rewrite sequences.
- **Cost Model:** Confluence checking: critical pairs (overlapping LHS); Knuth-Bendix completion; termination provers use recursive path order (RPO), polynomial interpretations.
- **Real Wall:** Termination of arbitrary rewriting is undecidable; must restrict to decidable classes; higher-order rewriting (HOλ→) adds complexity.
- **Cross-Domain Aliases:** term-rewriting (programming-language-theory), confluent-normalization (linear-algebra-matrix).
- **Notes:** Maude system (algebraic specification); used in protocol verification, functional programming (Haskell's rewrite system).

### [PRIM-026] decision-procedure
- **Atom/Composite:** Composite
- **Definition:** Decision procedure: algorithm that decides truth of formulas in a specific theory. Theories: Presburger arithmetic (N, +, ≤, constants), real closed fields (Tarski: QE), arrays, bitvectors.
- **Cost Model:** Presburger arithmetic: O(n^6) (Ω(n^5) lower bound); QE for real closed fields: doubly exponential (CAD); arrays: extensionality + quantifier elimination.
- **Real Wall:** Combination of theories (Nelson-Oppen) requires stably infinite theories; non-stably infinite theories (bitvectors) need different combination methods.
- **Cross-Domain Aliases:** theory-decider (control-numerical-opt), quantifier-elimination (linear-algebra-matrix).

## 3. Symbolic Execution

### [PRIM-027] symbolic-execution-engine
- **Atom/Composite:** Composite
- **Definition:** Symbolic execution: execute a program with symbolic values (variables) instead of concrete values; path condition = conjunction of constraints on inputs. Explores all feasible paths; path explosion is the bottleneck.
- **Cost Model:** Path explosion: each branch doubles paths; constraint solving per path (path condition → SMT); lazy solving (solve incrementally) reduces overhead.
- **Real Wall:** Path explosion limits scalability; environment modeling (system calls, libraries) is critical; floating-point symbolic values are hard.
- **Cross-Domain Aliases:** symbolic-program-analysis (control-numerical-opt), path-condition-solving (information-theory-coding).
- **Notes:** KLEE (LLVM-based symbolic execution for C); Angr (binary analysis); Manticore (EVM binary analysis).

### [PRIM-028] concolic-execution
- **Atom/Composite:** Composite
- **Definition:** Concolic (concrete + symbolic): run program concretely, collect path conditions symbolically; flip one branch condition to generate new concrete input that explores a different path. DART (2005), CUTE, CREST, KLEE-DSE.
- **Cost Model:** Alternating concrete + symbolic execution; each run: concrete execution + symbolic path condition collection + constraint solving for new input; systematic coverage with depth-first or greedy search.
- **Real Wall:** Path explosion; constraint solving bottleneck for complex path conditions; floating-point paths; search heuristics determine bug-finding effectiveness.
- **Cross-Domain Aliases:** dynamic-symbolic-execution (control-numerical-opt), concrete-symbolic-alternation (ml-training).

### [PRIM-029] symbolic-heap-separation-logic
- **Atom/Composite:** Composite
- **Definition:** Separation logic (SL): assertion language for heap-manipulating programs; spatial conjunction (P ∗ Q = heap can be split); symbolic heap: describes shape of memory (linked lists, trees). Used in shape analysis.
- **Cost Model:** Decision procedure for SL (inherently undecidable in general); decidable fragments: symbolic heaps with separation; entailment checking = subtyping of shape descriptors.
- **Real Wall:** Expressiveness vs. decidability: cannot handle all data structures; bi-abductive inference (infer preconditions) is key for scalable analysis.
- **Cross-Domain Aliases:** heap-shape-analysis (biology-bioinformatics), memory-logic (programming-language-theory).
- **Notes:** Infer static analyzer (Facebook/Meta); SpaceInvader, Sleek; used in bug finding for C/linux kernel.

### [PRIM-030] abstract-interpretation
- **Atom/Composite:** Primitive
- **Definition:** Abstract interpretation: sound overapproximation of program semantics in an abstract domain. Galois connection: (Concrete ⊆, α, γ ⊆) Abstract. Fixpoint computation over abstract domain gives sound program analysis.
- **Cost Model:** Abstract domain determines cost/precision trade-off; widening (force convergence of ascending chains) is key algorithm; narrow refines abstract values using concrete operations.
- **Real Wall:** Widening can cause imprecision (lose precision at loop headers); finding the right abstract domain for a given property is the main design decision.
- **Cross-Domain Aliases:** sound-overapproximation (control-numerical-opt), galois-connection (linear-algebra-matrix).
- **Notes:** Cousot & Cousot (1977); numerical domains: intervals, octagons, polyhedra; shape domains: separation logic; used in Astrée (aircraft control software), Infer (Meta).

### [PRIM-031] interval-analysis
- **Atom/Composite:** Primitive
- **Definition:** Interval domain: [l, u] for each variable; abstract semantics of assignments and guards; fixed-point via widening at loop headers. Computes range of each variable at each program point.
- **Cost Model:** O(n · |program|) where n = number of variables; widening at each loop header guarantees termination; precision determined by loop invariants.
- **Real Wall:** Very coarse (ignores relations between variables); polyhedra domain captures all affine relations; octagon captures relations of form ±x ± y ≤ c.
- **Cross-Domain Aliases:** range-analysis (control-numerical-opt), variable-bound-computation (linear-algebra-matrix).

### [PRIM-032] weakest-precondition
- **Atom/Composite:** Composite
- **Definition:** Weakest preconditions (Dijkstra): for program S and postcondition Q, WP(S, Q) = weakest condition on initial state that guarantees Q holds after S. wp calculus: sequential composition, conditionals, loops (via fixpoint).
- **Cost Model:** WP of loops = greatest fixed point (specification via loop invariants); recursive definition for while loops; complexity O(|S| · |Q|).
- **Real Wall:** Computing WP for loops requires invariants; without invariants, WP is not computable; strongest postconditions also used.
- **Cross-Domain Aliases:** program-verification-calculus (control-numerical-opt), backward-hoare (programming-language-theory).

### [PRIM-033] relational-abstract-domain
- **Atom/Composite:** Composite
- **Definition:** Relational abstract domains capture relations between variables (not just individual ranges). Octagons (±x ± y ≤ c), polyhedra (affine constraints), zones, template constraint matrices.
- **Cost Model:** Polyhedra: O(n^4) per polyhedron operation; octagons: O(n^3); relational domains scale poorly with dimension; non-relational domains (intervals) are cheap.
- **Real Wall:** Polyhedra blow up fast (exponential in dimension); convex approximation needed; disjunctive completion (powerset domain) increases precision at exponential cost.
- **Cross-Domain Aliases:** relational-analysis (control-numerical-opt), constraint-propagation (linear-algebra-matrix).
- **Notes:** Apron library (Jeannet, Miné); used in numerical stability analysis, buffer overflow detection, WCET analysis.

### [PRIM-034] invariant-synthesis
- **Atom/Composite:** Composite
- **Definition:** Automatic synthesis of loop invariants: inference from program traces (ICE algorithm), template-based (assume polynomial form → solve constraints), interpolation-based (Craig interpolants from BMC).
- **Cost Model:** Template: solve linear constraints; ICE: enumerate candidate predicates; interpolation: extract predicates from proof of unsatisfiability.
- **Real Wall:** Rich invariant spaces are expensive; polynomial invariants of high degree; not all programs have simple algebraic invariants.
- **Cross-Domain Aliases:** loop-invariant-inference (control-numerical-opt), predicate-abstraction (information-theory-coding).
- **Notes:** Houdini (FLABOT algorithm); InvGen; ICE (Gulwani et al.); used in Counterexample-Guided Invariant Synthesis (CEGIS).

### [PRIM-035] assume-guarantee-reasoning
- **Atom/Composite:** Composite
- **Definition:** Assume-guarantee: to verify component C: assume environment satisfies A, prove C satisfies G under A. Circular assume-guarantee (CAG): A ∧ C ⊨ G and A ⊨ pre(C) are checked simultaneously.
- **Cost Model:** Circular CAG requires fixed-point computation (alternating assumptions); decomposing into non-circular rules is simpler; compositional verification reduces state space.
- **Real Wall:** Circular reasoning is subtle (circularity must be well-founded); non-circular assume-guarantee requires environment behavior to be accurately modeled.
- **Cross-Domain Aliases:** compositional-verification (distributed-systems), contract-based-verification (programming-language-theory).

### [PRIM-036] software-model-checking
- **Atom/Composite:** Composite
- **Definition:** Software model checking: model checking applied to program source code (C, Java, LLVM bytecode). Combines symbolic execution, abstract interpretation, interpolation, CEGAR. Tools: CBMC, BLAST, CPAchecker, SeaHorn.
- **Cost Model:** Complexity: undecidable (Turing-complete language) vs. decidable (bounded model checking); tools use decidable fragments or approximations.
- **Real Wall:** Pointers, heap allocation, recursive data structures, dynamic dispatch; environment modeling (library calls, system calls) is critical and hard.
- **Cross-Domain Aliases:** program-analysis-model-check (control-numerical-opt), source-verification (distributed-systems).

## Appendix: Primitive Count

Total primitives in formal-verification domain: **36**
