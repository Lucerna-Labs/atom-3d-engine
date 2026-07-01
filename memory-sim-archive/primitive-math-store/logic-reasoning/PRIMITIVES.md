# Logic and Reasoning Primitives

> Domain: Logic and Reasoning — formal systems for proof, decision procedures, automated reasoning, and symbolic inference. Spans propositional/first-order/modal/temporal/description logics, SAT/SMT solving, theorem proving, and non-monotonic reasoning.

## Primitive Catalog

---

## Section 1. Propositional Logic & SAT Solving

### LR.001: Conjunctive Normal Form
**Definition:** Representation of a propositional formula as a conjunction of disjunctions of literals.
**Cost Model:** Tseitin transformation converts arbitrary formula to CNF in O(n) size with auxiliary variables.
**Real Wall:** Naive distribution causes exponential blowup; Tseitin loses structural information solvers might exploit.
**Cross-Domain Aliases:** cnf, clausal_form, product_of_sums
**Notes:** Universal input language for SAT solvers; Plaisted-Greenbaum optimizes by polarity tracking.

---

### LR.002: Disjunctive Normal Form
**Definition:** Representation of a propositional formula as a disjunction of conjunctions of literals.
**Cost Model:** Converting CNF→DNF is worst-case exponential in number of clauses.
**Real Wall:** DNF satisfiability is trivial but tautology checking is coNP-hard; rarely used as solver input.
**Cross-Domain Aliases:** dnf, sum_of_products, minterm_form
**Notes:** Dual of CNF; useful for knowledge compilation and #SAT enumeration.

---

### LR.003: Negation Normal Form
**Definition:** Formula where negations appear only on atomic propositions and connectives are limited to AND/OR.
**Cost Model:** Linear-time conversion via De Morgan's laws and double-negation elimination.
**Cross-Domain Aliases:** nnf, polarity_form
**Real Wall:** Loses implication structure that some proof systems prefer; bipartite circuit explosion still possible.
**Notes:** Intermediate step before CNF/DNF; foundation for d-DNNF knowledge compilation.

---

### LR.004: Tseitin Transformation
**Definition:** Linear-size CNF encoding of arbitrary propositional formula using fresh auxiliary variables per subformula.
**Cost Model:** O(n) variables and clauses for formula of size n; introduces equisatisfiable (not equivalent) form.
**Real Wall:** Auxiliary variables explode the search space; solver heuristics must avoid branching on them poorly.
**Cross-Domain Aliases:** tseitin_encoding, definitional_cnf, structure_preserving_cnf
**Notes:** Tseitin 1968; Plaisted-Greenbaum variant uses one-sided polarity to halve clauses.

---

### LR.005: DPLL Algorithm
**Definition:** Backtracking SAT procedure with unit propagation and pure literal elimination on partial assignments.
**Cost Model:** Worst-case O(2^n); branching factor mitigated by propagation closure at each node.
**Real Wall:** Naive backtracking revisits same conflicts repeatedly; no learning across the search tree.
**Cross-Domain Aliases:** dpll, davis_putnam_logemann_loveland, backtrack_sat
**Notes:** Davis-Putnam-Logemann-Loveland 1962; superseded by CDCL but conceptual ancestor of all modern solvers.

---

### LR.006: CDCL Algorithm
**Definition:** Conflict-Driven Clause Learning SAT procedure that derives new clauses from conflict analysis and non-chronologically backjumps.
**Cost Model:** Polynomial per conflict for analysis; total runtime exponential but practically tractable on structured instances.
**Real Wall:** Memory pressure from learned clause database; aggressive deletion required to stay performant.
**Cross-Domain Aliases:** cdcl, conflict_driven_search, grasp_style
**Notes:** Marques-Silva & Sakallah GRASP 1996; Moskewicz Chaff 2001 added watched literals and VSIDS.

---

### LR.007: Unit Propagation
**Definition:** Iterative assignment of variables forced by unit clauses (clauses with one unassigned literal under current assignment).
**Cost Model:** O(L) per propagation step with watched literals; dominates 80%+ of SAT solver runtime.
**Real Wall:** Cache misses on clause traversal; only two-watched-literal scheme keeps it fast.
**Cross-Domain Aliases:** bcp, boolean_constraint_propagation, unit_resolution
**Notes:** Central inner loop; lazy data structures (Zhang 2001) revolutionized performance.

---

### LR.008: Pure Literal Elimination
**Definition:** Removal of variables that appear only positively or only negatively in remaining clauses, assigned to satisfy them.
**Cost Model:** O(L) per pass; rarely repeated in modern CDCL due to maintenance cost.
**Real Wall:** Watched-literal data structures make detecting purity expensive; mostly applied in preprocessing.
**Cross-Domain Aliases:** pure_literal_rule, monotone_simplification
**Notes:** Classical DPLL component; SatELite preprocessor reintroduces it as bounded variable elimination.

---

### LR.009: Watched Literals
**Definition:** Lazy clause-monitoring scheme that watches exactly two unassigned literals per clause for propagation triggers.
**Cost Model:** O(1) amortized propagation per assignment; no work on backtrack.
**Real Wall:** Random memory access patterns; cache locality is the practical bottleneck on modern CPUs.
**Cross-Domain Aliases:** two_watched_literals, lazy_data_structures
**Notes:** Moskewicz et al. Chaff 2001; the single most important practical SAT engineering breakthrough.

---

### LR.010: VSIDS Heuristic
**Definition:** Variable State Independent Decaying Sum branching heuristic that scores variables by recent involvement in conflicts.
**Cost Model:** O(log n) per decision via priority queue; activity bumps O(1).
**Real Wall:** Decay rate tuning is instance-sensitive; can lock into local search regions.
**Cross-Domain Aliases:** vsids, activity_heuristic, conflict_history_branching
**Notes:** Chaff 2001; exponential VSIDS (EVSIDS) avoids periodic rescaling.

---

### LR.011: Phase Saving
**Definition:** Heuristic that remembers the last assigned value of a variable and tries that polarity first on revisit.
**Cost Model:** O(1) lookup per decision; one bit per variable.
**Real Wall:** Can trap solver in unsatisfiable subspace; periodic phase resetting (rephasing) needed.
**Cross-Domain Aliases:** phase_caching, polarity_saving, progress_saving
**Notes:** Pipatsrisawat & Darwiche 2007; standard in MiniSat and descendants.

---

### LR.012: Restart Policy
**Definition:** Periodic backtrack to decision level zero (keeping learned clauses) to escape unproductive search regions.
**Cost Model:** Geometric, Luby, or glue-based schedules; restart frequency tuned per benchmark family.
**Real Wall:** Too-frequent restarts thrash; too-rare leaves solver stuck. Glucose-style LBD-based restarts won.
**Cross-Domain Aliases:** restart, luby_restart, glucose_restart
**Notes:** Gomes et al. 1998; Luby et al. 1993 optimal universal schedule.

---

### LR.013: Glue Clauses
**Definition:** Learned clauses with low Literal Block Distance, indicating high reasoning value across decision levels.
**Cost Model:** LBD computed in O(|clause|) at learning time; one int per clause.
**Real Wall:** LBD is a proxy; some long-LBD clauses still useful and get incorrectly deleted.
**Cross-Domain Aliases:** glue, lbd_clause, high_quality_lemma
**Notes:** Audemard & Simon Glucose 2009; revolutionized clause database management.

---

### LR.014: Literal Block Distance
**Definition:** Number of distinct decision levels among literals in a learned clause; LBD=2 clauses are 'glue'.
**Cost Model:** O(|clause|) to compute, recomputed on touch in some implementations.
**Real Wall:** Static LBD at learning time becomes stale; dynamic LBD updates add bookkeeping cost.
**Cross-Domain Aliases:** lbd, glue_value, block_distance
**Notes:** Glucose's central metric; replaced clause length as the primary quality measure.

---

### LR.015: Conflict Analysis
**Definition:** Procedure that traces a conflicting assignment back through the implication graph to derive a learned clause.
**Cost Model:** O(|trail|) per conflict; First-UIP scheme is the standard.
**Real Wall:** Larger learned clauses are weaker; UIP heuristics balance clause size vs. propagation power.
**Cross-Domain Aliases:** conflict_analysis, clause_learning, resolution_proof_extraction
**Notes:** Marques-Silva 1996; 1UIP (first unique implication point) became the canonical scheme.

---

### LR.016: First UIP
**Definition:** First Unique Implication Point — the closest dominator to the conflict in the implication graph, used to cut the learned clause.
**Cost Model:** Single backward traversal of the implication graph, O(decision_level).
**Real Wall:** Alternative schemes (last UIP, all UIP) rarely match 1UIP in practice for unclear reasons.
**Cross-Domain Aliases:** 1uip, first_uip, asserting_clause
**Notes:** Zhang et al. 2001; produces an asserting clause that flips exactly one variable on backjump.

---

### LR.017: Non-Chronological Backjumping
**Definition:** Jump backward in the decision stack past irrelevant decisions to the level where the learned clause becomes asserting.
**Cost Model:** O(|trail|) to undo assignments; jumping distance is data-dependent.
**Real Wall:** Aggressive backjumping can lose useful propagation state; restart vs. backjump tradeoff.
**Cross-Domain Aliases:** backjumping, non_chronological_backtrack, intelligent_backtrack
**Notes:** Stallman & Sussman 1977 for general CSP; transferred to SAT by GRASP.

---

### LR.018: Clause Deletion Policy
**Definition:** Strategy for removing learned clauses to bound memory; typically deletes high-LBD, low-activity clauses.
**Cost Model:** O(n log n) periodic sort by quality metric; deletion frequency controlled by reduce-db schedule.
**Real Wall:** Deleting a useful clause forces re-derivation later; conservation vs. memory pressure tradeoff.
**Cross-Domain Aliases:** clause_db_reduction, learnt_clause_cleanup, garbage_collection
**Notes:** MiniSat uses activity-based deletion; Glucose uses LBD-based; CryptoMiniSat hybrid.

---

### LR.019: Subsumption
**Definition:** Removal of a clause C2 when another clause C1 is a subset of C2 (C1 implies C2).
**Cost Model:** O(|C1|·|C2|) per pair; total O(|F|²) without indexing.
**Real Wall:** Quadratic in clause count; signature/feature indexing required for large instances.
**Cross-Domain Aliases:** subsumption, clause_simplification, redundancy_elimination
**Notes:** Both forward (new clause subsumed) and backward (new clause subsumes existing); SatELite key preprocessor.

---

### LR.020: Self-Subsuming Resolution
**Definition:** When clause C1 = {l, X} resolves with C2 = {¬l, X, Y} to produce {X, Y}, strengthening C2.
**Cost Model:** O(|C1|·|C2|) per pair; integrated into preprocessing passes.
**Real Wall:** Order of application matters; can interact poorly with subsumption ordering.
**Cross-Domain Aliases:** self_subsumption, vivification_step
**Notes:** Eén & Biere SatELite 2005; standard inprocessing technique.

---

### LR.021: Bounded Variable Elimination
**Definition:** Elimination of a variable by resolving all clauses containing it pairwise, retained if resulting clause count grows by less than threshold.
**Cost Model:** O(deg(v)²) per variable; threshold typically 0 or small constant.
**Real Wall:** Can quadratically blow up clause count if applied poorly; ordering heuristic critical.
**Cross-Domain Aliases:** bve, variable_elimination, davis_putnam_resolution
**Notes:** Eén & Biere SatELite; original DP-procedure resolution but bounded to control growth.

---

### LR.022: Failed Literal Probing
**Definition:** Tentatively assign a literal and run unit propagation; if conflict, assign the opposite as unit clause.
**Cost Model:** O(|F|) per literal probed; quadratic worst case.
**Real Wall:** Expensive without good candidate selection; tree-based lookahead solvers integrate it deeply.
**Cross-Domain Aliases:** failed_literal, probing, look_ahead
**Notes:** Standard in lookahead solvers (March, Kissat lookahead mode); also called 'unit failure'.

---

### LR.023: Pure SAT
**Definition:** Decision problem of determining whether a propositional CNF formula has a satisfying assignment.
**Cost Model:** NP-complete; Cook-Levin 1971. Modern solvers handle millions of clauses in industrial cases.
**Real Wall:** Worst-case exponential; structured industrial instances are tractable, random 3-SAT near threshold is hard.
**Cross-Domain Aliases:** sat, propositional_satisfiability, boolean_sat
**Notes:** First proven NP-complete problem; SAT competition annually benchmarks state of the art.

---

### LR.024: Sharp SAT
**Definition:** Counting problem: how many satisfying assignments does a CNF formula have? #P-complete.
**Cost Model:** Even harder than SAT; component caching and knowledge compilation are key techniques.
**Real Wall:** #SAT is #P-complete (Valiant); few practical solvers scale past tens of variables for unstructured inputs.
**Cross-Domain Aliases:** sharp_sat, model_counting, sat_counting
**Notes:** sharpSAT (Thurley 2006); foundation for probabilistic inference reductions.

---

### LR.025: Maximum Satisfiability
**Definition:** Find an assignment maximizing the number (or weight) of satisfied clauses in an unsatisfiable formula.
**Cost Model:** NPO-complete; modern MaxSAT solvers use CDCL with unsat-core-guided refinement.
**Real Wall:** Optimization layer over SAT; many real problems are weighted partial MaxSAT.
**Cross-Domain Aliases:** maxsat, weighted_partial_maxsat, soft_clause_optimization
**Notes:** OLL, RC2, WBO algorithms; central to optimization in formal verification and planning.

---

## Section 2. CDCL Internals & Modern SAT Engineering

### LR.026: Implication Graph
**Definition:** Directed acyclic graph where nodes are assignments and edges trace unit-propagation causes.
**Cost Model:** Implicit; reconstructed during conflict analysis via clause reasons.
**Real Wall:** Storing reasons (which clause caused each propagation) is one pointer per variable.
**Cross-Domain Aliases:** implication_graph, reason_graph, propagation_dag
**Notes:** Conceptual structure; in practice solvers store only the 'reason' clause per assignment.

---

### LR.027: Decision Level
**Definition:** Depth in the search tree of a variable assignment, with 0 reserved for top-level unit clauses.
**Cost Model:** O(1) per variable; one integer per literal on the trail.
**Real Wall:** Backjump targets a specific level; precise tracking essential for correctness.
**Cross-Domain Aliases:** decision_level, dl, branch_depth
**Notes:** Drives 1UIP cut computation and LBD measurement.

---

### LR.028: Assignment Trail
**Definition:** Sequential record of variable assignments in the order made by decisions and propagations.
**Cost Model:** O(n) space; O(1) push and undo via stack discipline.
**Real Wall:** Cache locality of trail iteration is critical for propagation speed.
**Cross-Domain Aliases:** trail, assignment_stack
**Notes:** Central data structure of CDCL; head pointer advances during BCP.

---

### LR.029: Inprocessing
**Definition:** Application of formula simplification techniques (subsumption, BVE, probing) interleaved with CDCL search.
**Cost Model:** Periodic between restarts; budget tuned to avoid dominating runtime.
**Real Wall:** Inprocessing on learned clauses risks correctness; must preserve equisatisfiability.
**Cross-Domain Aliases:** inprocessing, simplification_during_search
**Notes:** Järvisalo, Heule, Biere 2012; modern solvers spend significant time here.

---

### LR.030: Random Polarity
**Definition:** Branching policy that occasionally chooses random truth value instead of saved phase to diversify search.
**Cost Model:** O(1) per decision; random number generation overhead negligible.
**Real Wall:** Pure randomness underperforms phase saving; used as periodic 'rephasing' perturbation.
**Cross-Domain Aliases:** random_phase, rephasing, polarity_diversification
**Notes:** Kissat introduced systematic rephasing schedules.

---

### LR.031: Chronological Backtracking
**Definition:** Reverting to the previous decision level rather than backjumping to the assertion level.
**Cost Model:** O(1) per level undone; less aggressive than non-chronological.
**Real Wall:** Loses opportunity to skip irrelevant assignments; only beneficial in certain hybrid schemes.
**Cross-Domain Aliases:** chronobt, chronological_bt, classical_backtrack
**Notes:** Nadel & Ryvchin 2018 showed chronological BT can outperform non-chronological in some cases.

---

### LR.032: Stabilization Mode
**Definition:** Period of CDCL search with reduced restart frequency to allow deep exploration.
**Cost Model:** Alternates with restart-heavy 'focused' mode; switching schedule heuristic.
**Real Wall:** Detecting when stabilization is productive vs. wasted is open problem.
**Cross-Domain Aliases:** stable_mode, focused_search_alternation
**Notes:** Kissat and CaDiCaL alternate between modes for robustness across benchmark families.

---

### LR.033: Local Search SAT
**Definition:** SAT solving by flipping variables in a complete assignment to reduce unsatisfied clause count.
**Cost Model:** Per-flip O(deg(v)); typically incomplete (only finds SAT, not UNSAT).
**Real Wall:** Stuck in local optima; restarts and noise (WalkSAT) escape but cannot prove unsatisfiability.
**Cross-Domain Aliases:** local_search_sat, walksat, gsat, sls
**Notes:** Selman, Kautz, Cohen WalkSAT 1994; ProbSAT and YalSAT modern variants.

---

### LR.034: Survey Propagation
**Definition:** Message-passing inspired by statistical physics for random k-SAT near the satisfiability threshold.
**Cost Model:** O(M) per iteration on factor graph; converges in O(log n) iterations on tractable inputs.
**Real Wall:** Only excels on random k-SAT; structured industrial instances do not benefit.
**Cross-Domain Aliases:** survey_propagation, sp, cavity_method
**Notes:** Mézard, Parisi, Zecchina 2002; theoretical breakthrough not adopted by industrial solvers.

---

### LR.035: Belief Propagation On SAT
**Definition:** Loopy belief propagation on the formula's factor graph to estimate variable marginals.
**Cost Model:** O(|E|) per round; iterations until convergence (often non-convergent).
**Real Wall:** No convergence guarantees on loopy graphs; survey propagation generalizes it.
**Cross-Domain Aliases:** bp_sat, factor_graph_propagation
**Notes:** Related to message-passing in probabilistic graphical models; conceptual bridge to inference.

---

### LR.036: Cube And Conquer
**Definition:** Divide-and-conquer strategy that splits a SAT instance into 'cubes' (partial assignments) solved in parallel by CDCL.
**Cost Model:** Lookahead phase for cube generation; CDCL on each cube can be embarrassingly parallel.
**Real Wall:** Cube generation cost can dominate; load balancing across cubes critical.
**Cross-Domain Aliases:** cube_and_conquer, lookahead_cdcl
**Notes:** Heule, Kullmann, Biere; used for Schur Number Five and Pythagorean Triples Problem.

---

### LR.037: Portfolio SAT Solver
**Definition:** Parallel execution of multiple SAT solver configurations, returning the first to finish.
**Cost Model:** Linear speedup possible if solvers diverse; no super-linear in worst case.
**Real Wall:** Memory bandwidth contention; clause sharing between portfolio members non-trivial.
**Cross-Domain Aliases:** portfolio_sat, parallel_sat, plingeling_style
**Notes:** Plingeling, ManySAT; clause sharing improves but increases coupling.

---

### LR.038: DRAT Proof
**Definition:** Deletion Resolution Asymmetric Tautology — proof format for refutation of UNSAT instances by clause additions and deletions.
**Cost Model:** Proof generation O(|search|); checking proof O(|proof|·|formula|).
**Real Wall:** Proofs can be terabytes; specialized checkers (drat-trim) and DPR refinements help.
**Cross-Domain Aliases:** drat, drat_proof, unsat_certificate
**Notes:** Wetzler, Heule, Hunt 2014; mandated in SAT competition since 2014.

---

### LR.039: LRAT Proof
**Definition:** Linear RAT proof format with hints for fast checking via formally verified tools.
**Cost Model:** Proof has integer hints making checking linear-time; larger than DRAT.
**Real Wall:** Generating LRAT requires DRAT-trim post-processing; not native output of most solvers.
**Cross-Domain Aliases:** lrat, linear_rat
**Notes:** Used with cake_lpr verified checker; enables formal verification of UNSAT results.

---

### LR.040: Reverse Unit Propagation
**Definition:** Proof step verification by negating clause and checking unit propagation derives empty clause.
**Cost Model:** O(|F|) per step; underlies DRUP/DRAT checking.
**Real Wall:** Quadratic-ish total cost; advanced checkers use hints (LRAT) for linear checking.
**Cross-Domain Aliases:** rup, reverse_unit_prop
**Notes:** Goldberg & Novikov 2003; basis for DRUP proof format.

---

## Section 3. SMT & Theories

### LR.041: Satisfiability Modulo Theories
**Definition:** Decision problem of satisfiability of a first-order formula with respect to a background theory (LRA, LIA, BV, arrays, etc.).
**Cost Model:** Theory-dependent; undecidable in general (e.g., nonlinear integer arithmetic).
**Real Wall:** Theory combinations multiply complexity; performance depends on theory propagation quality.
**Cross-Domain Aliases:** smt, modulo_theory, theory_sat
**Notes:** SMT-LIB standard; Z3, CVC5, MathSAT, Yices are leading solvers.

---

### LR.042: Lazy SMT
**Definition:** Architecture where SAT solver enumerates Boolean models and theory solver checks each for theory consistency.
**Cost Model:** Repeated SAT calls with theory lemmas added; many round-trips between SAT and T.
**Real Wall:** Naive lazy approach has exponential SAT-T communication; modern solvers use online integration.
**Cross-Domain Aliases:** lazy_smt, lemma_on_demand
**Notes:** Original SMT architecture; superseded by DPLL(T) integration.

---

### LR.043: DPLL(T) Architecture
**Definition:** Tightly integrated SMT framework where theory solver participates in unit propagation and conflict analysis.
**Cost Model:** Theory propagation amortized across SAT search; conflict clauses include theory lemmas.
**Real Wall:** Theory solver must be incremental and backtrackable; significant engineering investment.
**Cross-Domain Aliases:** dpll_t, cdcl_t, online_smt
**Notes:** Ganzinger, Hagen, Nieuwenhuis, Oliveras, Tinelli 2004; standard modern SMT architecture.

---

### LR.044: Eager SMT
**Definition:** Translation of theory formula into pure SAT via theory-specific encoding, then standard SAT solving.
**Cost Model:** Encoding size depends on theory; bit-blasting for BV is canonical example.
**Real Wall:** Encoding blowup; lossy for theories without finite encoding (e.g., reals).
**Cross-Domain Aliases:** eager_smt, bit_blasting_approach, sat_encoded_smt
**Notes:** Used for bit-vectors via bit-blasting; competitive with lazy for fixed-width BV.

---

### LR.045: Theory Propagation
**Definition:** Theory solver derives literal assignments forced by current partial assignment and reports to SAT layer.
**Cost Model:** Per-theory; congruence closure O(α(n)) amortized, simplex O(n) per propagation.
**Real Wall:** Eager propagation costly; lazy propagation misses pruning. Balance is solver-specific.
**Cross-Domain Aliases:** theory_propagation, t_propagation
**Notes:** Key to DPLL(T) performance; quality of theory propagation differentiates SMT solvers.

---

### LR.046: Nelson-Oppen Combination
**Definition:** Procedure to combine decision procedures for stably-infinite, signature-disjoint theories via variable equality propagation.
**Cost Model:** Polynomial overhead on top of individual theory solvers; equality propagation step central.
**Real Wall:** Requires stable infiniteness and disjoint signatures; many practical theories violate assumptions.
**Cross-Domain Aliases:** nelson_oppen, no_combination, theory_combination
**Notes:** Nelson & Oppen 1979; foundation for combining multiple theories in single SMT query.

---

### LR.047: Shostak Combination
**Definition:** Theory combination via canonization and solving operations, alternative to Nelson-Oppen.
**Cost Model:** Theory-specific canonizer; combination via shared canonical forms.
**Real Wall:** Requires Shostak theories (canonizable and solvable); narrower applicability than NO.
**Cross-Domain Aliases:** shostak_combination, canonizer_solver_method
**Notes:** Shostak 1984; PVS and SVC used this method; less common in modern SMT.

---

### LR.048: Congruence Closure
**Definition:** Decision procedure for equality of uninterpreted function terms; closes equalities under congruence axiom.
**Cost Model:** O((n+m)·α(n)) where n is terms and m is equalities, via union-find.
**Real Wall:** Term representation cost; e-graph maintenance dominates incremental cases.
**Cross-Domain Aliases:** congruence_closure, cc, euf_solver
**Notes:** Downey, Sethi, Tarjan 1980; basis for EUF theory and e-graph rewriting.

---

### LR.049: Equality Graph
**Definition:** Data structure (e-graph) representing equivalence classes of terms with shared subterms, used by congruence closure.
**Cost Model:** Operations near-linear with union-find; rebuild during backtrack non-trivial.
**Real Wall:** E-matching for quantifiers requires extensive indexing; size grows with proof effort.
**Cross-Domain Aliases:** e_graph, egraph, equivalence_graph
**Notes:** Standard data structure in Z3, CVC5; also foundation of egg equality saturation library.

---

### LR.050: Linear Real Arithmetic Solver
**Definition:** Decision procedure for linear inequalities over the reals; solved by simplex-style algorithms.
**Cost Model:** Simplex O(n³) practical; exponential worst case.
**Real Wall:** Coefficient blowup in rational arithmetic; bignum overhead significant.
**Cross-Domain Aliases:** lra, lra_solver, linear_real
**Notes:** Dutertre & de Moura 2006 general-form simplex; standard in modern SMT.

---

### LR.051: Linear Integer Arithmetic Solver
**Definition:** Decision procedure for linear integer arithmetic; uses LRA with Gomory cuts or branch-and-bound.
**Cost Model:** Doubly exponential worst case (Presburger arithmetic); practical heuristics critical.
**Real Wall:** Integer feasibility after LRA relaxation is the bottleneck; cuts and branching heuristics matter.
**Cross-Domain Aliases:** lia, lia_solver, linear_integer
**Notes:** Presburger arithmetic decidable; modern SMT uses Cooper's method or branch-cut.

---

### LR.052: Bit-Vector Solver
**Definition:** Decision procedure for fixed-width bit-vector logic with arithmetic, bitwise, and shift operations.
**Cost Model:** Bit-blasting reduces to SAT in O(n·w) bits where w is width; SAT then dominates.
**Real Wall:** Width-128+ multiplications produce massive SAT instances; eager solvers struggle.
**Cross-Domain Aliases:** bv_solver, bitvector_theory, qfbv
**Notes:** Used heavily in software/hardware verification; Boolector, Z3 strong here.

---

### LR.053: Array Theory Solver
**Definition:** Decision procedure for array select/store operations with extensionality axiom.
**Cost Model:** Lazy axiom instantiation; quadratic in number of array operations worst case.
**Real Wall:** Extensionality reasoning is expensive; nested arrays compound the cost.
**Cross-Domain Aliases:** array_theory, ax_solver, theory_of_arrays
**Notes:** McCarthy 1962 read-over-write axiom; Stump et al. lazy instantiation.

---

### LR.054: Theory Of Uninterpreted Functions
**Definition:** First-order equality logic with function symbols having no semantics beyond the congruence axiom.
**Cost Model:** Decided by congruence closure; near-linear time.
**Real Wall:** Quantifier-free is easy; combining with quantifiers requires E-matching or MBQI.
**Cross-Domain Aliases:** euf, uninterpreted_functions, theory_of_uf
**Notes:** Often combined with LIA/LRA via Nelson-Oppen; foundation theory of SMT.

---

### LR.055: Theory Of Strings
**Definition:** Decision procedure for word equations, string length, and regular expression constraints.
**Cost Model:** Undecidable in general; fragments (regular constraints + linear length) decidable.
**Real Wall:** Long strings explode search; symbolic reasoning required for unbounded lengths.
**Cross-Domain Aliases:** string_theory, qfslia, sequence_solver
**Notes:** CVC5 and Z3 have competing approaches; PyEx, S3 specialized solvers.

---

### LR.056: Floating-Point Theory
**Definition:** Decision procedure for IEEE 754 floating-point arithmetic with NaN, infinities, rounding modes.
**Cost Model:** Bit-blasting to SAT; multiplication especially expensive due to mantissa handling.
**Real Wall:** Cost of FP multiplication SAT encoding is enormous; abstract domains help bound work.
**Cross-Domain Aliases:** fp_theory, qffp, ieee_754_solver
**Notes:** Brain et al. SymFPU; Z3 and MathSAT support; essential for safety-critical software verification.

---

### LR.057: Nonlinear Real Arithmetic
**Definition:** Decision procedure for polynomial constraints over the reals; decidable via cylindrical algebraic decomposition.
**Cost Model:** CAD is doubly exponential; modern solvers use partial CAD with model construction.
**Real Wall:** Doubly exponential is real on degree > 4 polynomials; heuristic MCSAT/NLSAT methods dominate.
**Cross-Domain Aliases:** nra, nonlinear_real, polynomial_solver
**Notes:** Jovanović & de Moura MCSAT 2012; Tarski decidability via Collins CAD.

---

### LR.058: Nonlinear Integer Arithmetic
**Definition:** Decision problem for polynomial constraints over integers; undecidable (Hilbert's 10th problem).
**Cost Model:** Undecidable; practical solvers use bit-blasting with bounded ranges.
**Real Wall:** No complete procedure; even quadratic Diophantine constraints can stall solvers.
**Cross-Domain Aliases:** nia, nonlinear_integer
**Notes:** Matiyasevich 1970 undecidability; SMT handles via bounded reasoning or NRA relaxation.

---

### LR.059: Difference Logic
**Definition:** Fragment of linear arithmetic restricted to constraints of form x - y ≤ c; decidable in polynomial time.
**Cost Model:** Negative-cycle detection in constraint graph O(VE); used as efficient sub-procedure.
**Real Wall:** Many real problems exceed difference logic; falls back to full LIA/LRA.
**Cross-Domain Aliases:** diff_logic, dl, ufrd
**Notes:** Cotton & Maler; used in scheduling and timed automata verification.

---

### LR.060: Bit-Blasting
**Definition:** Eager encoding of bit-vector operations into propositional logic via circuit-level expansion.
**Cost Model:** O(w·n) clauses where w is bit-width; multiplication produces O(w²) clauses.
**Real Wall:** Multiplication and division generate huge CNF; modular reduction algorithms help.
**Cross-Domain Aliases:** bit_blasting, bitblast, eager_bv_encoding
**Notes:** Underlies QF_BV solvers like Boolector; lossy for unbounded BV but ideal for fixed widths.

---

### LR.061: Ackermannization
**Definition:** Elimination of uninterpreted functions by introducing fresh variables for each application and equality constraints.
**Cost Model:** O(k²) added equalities for k applications of a function.
**Real Wall:** Quadratic blowup makes it impractical at scale; congruence closure preferred.
**Cross-Domain Aliases:** ackermannization, function_application_lemma
**Notes:** Ackermann 1954; used to reduce EUF to equality logic; in SMT mostly historical.

---

### LR.062: CDCL(T)
**Definition:** Refinement of DPLL(T) using full CDCL machinery (1UIP analysis, watched literals) in the Boolean engine.
**Cost Model:** Standard CDCL costs plus theory propagation overhead.
**Real Wall:** Coordinating clause learning with theory solver state requires careful design.
**Cross-Domain Aliases:** cdcl_t, modern_smt_architecture
**Notes:** Synonym for DPLL(T) in modern usage; emphasizes CDCL not DPLL underneath.

---

### LR.063: Model-Based Theory Combination
**Definition:** Theory combination via model construction and disagreement resolution, avoiding Nelson-Oppen restrictions.
**Cost Model:** Per-theory model production; combined model verification.
**Real Wall:** Some theories cannot produce explicit models efficiently; partial models a research area.
**Cross-Domain Aliases:** mbtc, model_combination
**Notes:** de Moura & Bjørner 2008; relaxes stable infiniteness requirement of NO.

---

### LR.064: MCSAT Architecture
**Definition:** Model-Constructing SAT: alternative SMT architecture that constructs theory models incrementally during search.
**Cost Model:** Theory-specific extensions of CDCL; richer than DPLL(T) lemma exchange.
**Real Wall:** Theory plugin engineering more demanding; not all theories yet supported.
**Cross-Domain Aliases:** mcsat, ncdcl, natural_cdcl
**Notes:** Jovanović & de Moura; basis for nonlinear arithmetic in Yices2 and Z3.

---

### LR.065: Simplex For LRA
**Definition:** General-form simplex algorithm adapted for incremental SMT solving with explanations.
**Cost Model:** Pivot operations dominate; O(mn) per pivot, exponential worst case.
**Real Wall:** Coefficient growth in rational arithmetic; numerical issues if using floating point.
**Cross-Domain Aliases:** simplex_lra, dual_simplex
**Notes:** Dutertre & de Moura 2006; uses bound propagation extensively.

---

### LR.066: Gomory Cut
**Definition:** Cutting plane derived from fractional simplex solution to tighten integer relaxation.
**Cost Model:** O(n) to generate; effectiveness depends on cut strength.
**Real Wall:** Cut accumulation degrades simplex tableau; selective cuts essential.
**Cross-Domain Aliases:** gomory_cut, integer_cut
**Notes:** Gomory 1958; foundation of cutting plane method for ILP; used in LIA SMT.

---

## Section 4. First-Order Logic & Resolution

### LR.067: First-Order Logic
**Definition:** Logic with variables, quantifiers (∀, ∃), predicates, functions, and equality; semi-decidable.
**Cost Model:** Validity is semi-decidable (Church 1936); enumeration of proofs via complete proof system.
**Real Wall:** Quantifier reasoning expensive; combinatorial blowup in instantiation choices.
**Cross-Domain Aliases:** fol, predicate_logic, first_order_predicate_calculus
**Notes:** Frege 1879 foundation; Gödel 1930 completeness theorem.

---

### LR.068: Resolution
**Definition:** Inference rule for clausal logic: from {A ∨ C} and {¬A ∨ D}, derive {C ∨ D}.
**Cost Model:** Per-step O(|C|+|D|·|θ|) with unification θ; saturation requires fair strategy.
**Real Wall:** Search space explodes; ordered/unit/hyperresolution restrictions essential.
**Cross-Domain Aliases:** resolution, robinson_resolution
**Notes:** Robinson 1965; foundation of automated theorem proving for FOL.

---

### LR.069: Unification
**Definition:** Computation of most general substitution making two terms syntactically equal.
**Cost Model:** Linear-time algorithms (Paterson-Wegman, Martelli-Montanari) with sharing.
**Real Wall:** Occurs-check expensive; many Prolog implementations skip it for speed at correctness risk.
**Cross-Domain Aliases:** unification, mgu, most_general_unifier
**Notes:** Robinson 1965 original; Martelli-Montanari 1982 efficient algorithm.

---

### LR.070: Skolemization
**Definition:** Removal of existential quantifiers by replacing with fresh function symbols depending on enclosing universals.
**Cost Model:** Linear preprocessing; introduces Skolem functions whose arities depend on quantifier scope.
**Real Wall:** Inner Skolemization is more efficient than outer but harder to reason about; choice matters.
**Cross-Domain Aliases:** skolemization, existential_elimination
**Notes:** Skolem 1928; preserves satisfiability not equivalence.

---

### LR.071: Prenex Normal Form
**Definition:** First-order formula with all quantifiers at the front, matrix quantifier-free.
**Cost Model:** Linear conversion via rewrite rules; may shuffle quantifiers losing local scoping.
**Real Wall:** Mixing scopes can produce harder Skolem functions; antiprenex sometimes better.
**Cross-Domain Aliases:** pnf, prenex_form
**Notes:** Standard step before Skolemization and CNF for first-order theorem proving.

---

### LR.072: Herbrand Universe
**Definition:** Set of all ground terms constructible from constants and function symbols of a first-order signature.
**Cost Model:** Infinite in general; enumeration via term depth bound.
**Real Wall:** Herbrand instantiation only complete in the limit; depth bounds give heuristic enumeration.
**Cross-Domain Aliases:** herbrand_universe, ground_term_universe
**Notes:** Herbrand 1930; basis for ground instantiation methods.

---

### LR.073: Herbrand Theorem
**Definition:** Theorem stating a universal FOL formula is unsatisfiable iff some finite set of ground instances is propositionally unsatisfiable.
**Cost Model:** Conceptually reduces FOL to SAT but with infinite ground space.
**Real Wall:** Determining which instances to enumerate is the hard part; iterative deepening common.
**Cross-Domain Aliases:** herbrand_theorem, herbrand_completeness
**Notes:** Reduces validity to compactness over Herbrand base; foundational for instance-based methods.

---

### LR.074: Semantic Tableaux
**Definition:** Refutation calculus working backward from formula structure, splitting on disjunctions and instantiating quantifiers.
**Cost Model:** Branch length bounded by formula complexity; quantifier rules cause infinite branches.
**Real Wall:** Free-variable tableaux require unification at closure; γ-rule enumeration unbounded.
**Cross-Domain Aliases:** tableaux, smullyan_tableaux, beth_tableaux
**Notes:** Smullyan 1968; Fitting; basis for many automated proof systems and DL solvers.

---

### LR.075: Sequent Calculus
**Definition:** Proof system with sequents Γ ⊢ Δ and rules introducing connectives on left/right sides.
**Cost Model:** Proof search dual to natural deduction; cut elimination key meta-theorem.
**Real Wall:** Proof search has many redundant choices; focusing reduces nondeterminism.
**Cross-Domain Aliases:** sequent_calculus, gentzen_lk
**Notes:** Gentzen 1934 LK and LJ; cut-elimination theorem foundational result.

---

### LR.076: Natural Deduction
**Definition:** Proof system with introduction and elimination rules for each connective, modeling informal reasoning.
**Cost Model:** Proof search via backward chaining; less direct than sequent calculus.
**Real Wall:** Many proofs require lemmas not derivable by pure introduction/elimination chains.
**Cross-Domain Aliases:** natural_deduction, gentzen_nd
**Notes:** Gentzen 1934; basis for proof assistants like Coq's tactic language.

---

### LR.077: Hilbert System
**Definition:** Axiomatic proof system with few rules (modus ponens, generalization) and many axiom schemas.
**Cost Model:** Proof construction unintuitive; verification linear in proof length.
**Real Wall:** Manual proof construction painful; replaced by natural deduction/sequent for human use.
**Cross-Domain Aliases:** hilbert_calculus, axiomatic_proof_system
**Notes:** Frege/Hilbert/Russell tradition; theoretical importance for completeness proofs.

---

### LR.078: Superposition Calculus
**Definition:** Saturation-based proof calculus combining resolution with ordered paramodulation for equational reasoning.
**Cost Model:** Per-inference O(|C|·|D|·indexing); orderings restrict applicable inferences.
**Real Wall:** Combinatorial explosion of inferences; given-clause selection critical.
**Cross-Domain Aliases:** superposition, ordered_paramodulation
**Notes:** Bachmair & Ganzinger; powers Vampire, E, Spass; modern FOL prover backbone.

---

### LR.079: Paramodulation
**Definition:** Inference rule for equality: substitute equals in a clause given an equation in another.
**Cost Model:** Per-step O(|C|·|D|·|term|) with term indexing.
**Real Wall:** Combinatorial explosion without ordering restrictions; modern provers use ordered paramodulation only.
**Cross-Domain Aliases:** paramodulation, equality_replacement
**Notes:** Robinson & Wos 1969; ordered restriction in superposition.

---

### LR.080: Term Ordering
**Definition:** Well-founded ordering on terms used to restrict applicability of resolution/paramodulation inferences.
**Cost Model:** Comparison per pair O(|s|·|t|) typical; KBO uses precedence and weights.
**Real Wall:** Choice of ordering massively affects proof search; KBO vs LPO is application-specific.
**Cross-Domain Aliases:** term_order, simplification_ordering
**Notes:** LPO (Kamin-Lévy), KBO (Knuth-Bendix); essential to superposition completeness.

---

### LR.081: Lexicographic Path Ordering
**Definition:** Term ordering based on precedence of root symbols with lexicographic comparison of arguments.
**Cost Model:** O(|s|·|t|) comparison; precedence must be supplied.
**Real Wall:** Precedence choice non-obvious; affects which equations orient left-to-right vs right-to-left.
**Cross-Domain Aliases:** lpo, lex_path_ordering
**Notes:** Kamin & Lévy 1980; popular in equational reasoning despite KBO often outperforming.

---

### LR.082: Knuth-Bendix Ordering
**Definition:** Weighted term ordering with symbol weights and precedence; total reduction ordering on ground terms.
**Cost Model:** Linear comparison via weight computation and precedence check.
**Real Wall:** Weight assignment optimization is heuristic; performance varies wildly across choices.
**Cross-Domain Aliases:** kbo, knuth_bendix_order
**Notes:** Knuth & Bendix 1970; primary ordering in Vampire and E theorem provers.

---

### LR.083: Saturation
**Definition:** Closure of a clause set under all applicable inference rules until no new clauses produced or contradiction found.
**Cost Model:** Unbounded in general; given-clause algorithm interleaves selection and inference.
**Real Wall:** Saturation rarely terminates on hard problems; given-clause selection drives effectiveness.
**Cross-Domain Aliases:** saturation, clause_set_closure
**Notes:** Theoretical foundation of completeness for resolution/superposition.

---

### LR.084: Given-Clause Algorithm
**Definition:** Saturation loop selecting a 'given clause' and computing all inferences with previously processed clauses.
**Cost Model:** Per-iteration O(|processed|·|inferences|); selection heuristics dominate practical performance.
**Real Wall:** Selection heuristics ad-hoc; clause weight + age combination standard but not optimal.
**Cross-Domain Aliases:** given_clause_loop, otter_loop, discount_loop
**Notes:** Otter loop (McCune) and DISCOUNT loop (Avenhaus) main variants.

---

### LR.085: Demodulation
**Definition:** Simplification of a clause by rewriting using an oriented unit equation.
**Cost Model:** Per-step O(|target|·|equation|) with term indexing.
**Real Wall:** Index maintenance expensive; demodulator selection affects effectiveness.
**Cross-Domain Aliases:** demodulation, rewrite_simplification
**Notes:** Wos & Robinson; combined with subsumption in modern provers.

---

### LR.086: Hyperresolution
**Definition:** Resolution variant combining one positive nucleus clause with multiple electron clauses to derive a positive clause.
**Cost Model:** Per-step O(product of clause sizes); restricts proof shape to reduce search.
**Real Wall:** Limited to certain proof structures; less common in modern provers than ordered resolution.
**Cross-Domain Aliases:** hyperresolution, hr, positive_hyperresolution
**Notes:** Robinson 1965; refinement of resolution useful for Horn-like fragments.

---

### LR.087: Set Of Support Strategy
**Definition:** Restriction requiring every resolution inference to involve at least one clause from a designated set (typically the negated goal).
**Cost Model:** Reduces inference candidates; preserves completeness when SoS is unsatisfiable in axioms.
**Real Wall:** Choice of initial SoS impacts proof discovery; too narrow loses completeness.
**Cross-Domain Aliases:** set_of_support, sos_strategy
**Notes:** Wos, Robinson, Carson 1965; classical resolution refinement.

---

### LR.088: Connection Method
**Definition:** Proof method using matrix representation of formulas and seeking spanning sets of connections (complementary literal pairs).
**Cost Model:** Connection enumeration similar to model elimination; depth-bounded search.
**Real Wall:** Spanning set search NP-hard; heuristic exploration needed.
**Cross-Domain Aliases:** connection_method, matrix_method, bibel_method
**Notes:** Bibel 1981, Andrews 1981; alternative to resolution.

---

### LR.089: Model Elimination
**Definition:** Proof procedure based on chains of literals connected by complementary unification, ancestor of Prolog.
**Cost Model:** Depth-first proof search; backtracking similar to Prolog SLD resolution.
**Real Wall:** Backtracking redundancy without learning; modern variants add lemmaization.
**Cross-Domain Aliases:** model_elimination, me, leancop_style
**Notes:** Loveland 1968; leanCoP and connection tableau implementations.

---

### LR.090: Equality Reasoning
**Definition:** Handling of equality predicate in FOL via axioms (reflexivity, symmetry, transitivity, congruence) or built-in inference.
**Cost Model:** Axioms blow up search; built-in via paramodulation more efficient.
**Real Wall:** Equality is the hardest 'theory' in pure FOL; superposition specifically designed for it.
**Cross-Domain Aliases:** equality_reasoning, equational_proof
**Notes:** Brand's transformation eliminates equality via flat terms; superposition handles natively.

---

## Section 5. Modal & Temporal Logics

### LR.091: Modal Logic K
**Definition:** Minimal normal modal logic with K axiom □(p→q) → (□p → □q) and necessitation rule.
**Cost Model:** PSPACE-complete satisfiability; tableau decision procedures standard.
**Real Wall:** Even minimal modal logic is PSPACE-complete; practical instances need clever heuristics.
**Cross-Domain Aliases:** modal_k, minimal_modal, system_k
**Notes:** Kripke 1963 semantics; foundation for all normal modal logics.

---

### LR.092: Modal Logic T
**Definition:** Extension of K with axiom T: □p → p (reflexivity of accessibility relation).
**Cost Model:** PSPACE-complete; tableau with reflexivity rule.
**Real Wall:** Same complexity class as K but slightly different proof structure.
**Cross-Domain Aliases:** modal_t, reflexive_modal, system_t
**Notes:** Corresponds to reflexive Kripke frames; weakest modal logic with □p → p.

---

### LR.093: Modal Logic S4
**Definition:** Modal logic T extended with axiom 4: □p → □□p (transitivity).
**Cost Model:** PSPACE-complete; tableau may need to detect cycles.
**Real Wall:** Loop detection in tableau search; preorder semantics.
**Cross-Domain Aliases:** s4, lewis_s4, transitive_reflexive_modal
**Notes:** Corresponds to preorders; intuitionistic logic embeddable via Gödel translation.

---

### LR.094: Modal Logic S5
**Definition:** Modal logic with equivalence-relation accessibility; □p means p true in all worlds.
**Cost Model:** NP-complete satisfiability (collapse to one cluster of worlds).
**Real Wall:** Easier than S4 due to flat structure; reduces to classical reasoning over worlds.
**Cross-Domain Aliases:** s5, equivalence_modal, partition_modal
**Notes:** Standard logic for knowledge (epistemic interpretation).

---

### LR.095: Modal Logic B
**Definition:** Modal logic T with symmetry axiom: p → □◇p.
**Cost Model:** PSPACE-complete; tableau with reflexivity and symmetry rules.
**Real Wall:** Less commonly used than S4/S5; specialized symmetric reasoning.
**Cross-Domain Aliases:** brouwer_modal, b_logic
**Notes:** Named after Brouwerian logic connection; symmetric Kripke frames.

---

### LR.096: Modal Logic KD45
**Definition:** Modal logic with seriality (D), transitivity (4), and Euclideanness (5); used for belief.
**Cost Model:** PSPACE-complete; doxastic reasoning system.
**Real Wall:** Captures consistent belief but not knowledge; subtleties in iterated modalities.
**Cross-Domain Aliases:** kd45, doxastic_logic, belief_logic
**Notes:** Standard logic for belief in epistemic AI; weaker than S5 (knowledge).

---

### LR.097: Kripke Semantics
**Definition:** Possible-worlds semantics with accessibility relation R; □p true at w iff p true at all worlds R-accessible from w.
**Cost Model:** Model size potentially exponential in formula size; bisimulation reduces.
**Real Wall:** Model exploration is the central cost; bisimulation quotient yields canonical models.
**Cross-Domain Aliases:** kripke_semantics, possible_worlds, relational_semantics
**Notes:** Kripke 1959, 1963; unified treatment of modal, temporal, intuitionistic logics.

---

### LR.098: Accessibility Relation
**Definition:** Binary relation on possible worlds determining which worlds a given world can 'see' for modal evaluation.
**Cost Model:** Frame property checking O(W²) for binary properties.
**Real Wall:** Choice of frame conditions characterizes logic; expressing 'all S5 frames' requires meta-reasoning.
**Cross-Domain Aliases:** accessibility, reachability_relation, frame_relation
**Notes:** Properties (reflexive, transitive, etc.) determine which modal axioms valid.

---

### LR.099: Modal Correspondence Theory
**Definition:** Study of relationships between modal axioms and properties of accessibility relations on Kripke frames.
**Cost Model:** Correspondence checks via second-order translation (Sahlqvist for first-order corresponding).
**Real Wall:** Not all modal formulas have first-order frame correspondents (Goldblatt-Thomason).
**Cross-Domain Aliases:** correspondence_theory, sahlqvist_correspondence
**Notes:** Sahlqvist 1975; van Benthem; foundation of frame-theoretic modal logic.

---

### LR.100: Modal Tableau
**Definition:** Tableau proof system for modal logics with rules for □ and ◇ creating new tableau worlds.
**Cost Model:** PSPACE upper bound for K, T, K4, S4 via clever world reuse; exponential without it.
**Real Wall:** Generated worlds proliferate; blocking conditions essential for termination.
**Cross-Domain Aliases:** modal_tableau, prefixed_tableau
**Notes:** Fitting 1972; Massacci, Hudelmaier optimizations.

---

### LR.101: Linear Temporal Logic
**Definition:** Modal logic over linear time with operators X (next), U (until), F (eventually), G (always).
**Cost Model:** PSPACE-complete satisfiability; model checking PSPACE-complete in formula, linear in model.
**Real Wall:** State explosion in model checking; symbolic representations help but not for all properties.
**Cross-Domain Aliases:** ltl, linear_time_logic, pnueli_logic
**Notes:** Pnueli 1977; standard specification language for hardware/software verification.

---

### LR.102: Computation Tree Logic
**Definition:** Branching-time temporal logic with path quantifiers A (all paths) and E (some path) before temporal operators.
**Cost Model:** Polynomial-time model checking O(|M|·|φ|); satisfiability EXPTIME-complete.
**Real Wall:** Branching expressiveness less useful in practice than LTL; counterexamples are trees not paths.
**Cross-Domain Aliases:** ctl, branching_time_logic, emerson_clarke
**Notes:** Clarke & Emerson 1981; basis of model checking field.

---

### LR.103: CTL Star
**Definition:** Combination of LTL and CTL; allows arbitrary path formulas inside path quantifiers.
**Cost Model:** Model checking PSPACE-complete; satisfiability 2EXPTIME-complete.
**Real Wall:** More expressive but harder than LTL or CTL alone; rarely used in tools.
**Cross-Domain Aliases:** ctl_star, ctlstar, full_branching_logic
**Notes:** Emerson & Halpern 1986; subsumes LTL and CTL but rarely implemented.

---

### LR.104: Mu-Calculus
**Definition:** Modal logic with least and greatest fixpoint operators μ and ν; expressively complete for bisimulation-invariant MSO.
**Cost Model:** Model checking in NP ∩ coNP (parity game); polynomial conjectured but not proven.
**Real Wall:** Practical model checking via parity games; complexity open question.
**Cross-Domain Aliases:** mu_calculus, modal_mu, kozen_logic
**Notes:** Kozen 1983; expressively captures CTL, CTL*, LTL.

---

### LR.105: Past LTL
**Definition:** LTL extended with past operators (Y for previous, S for since, P for once, H for historically).
**Cost Model:** Same complexity as LTL (PSPACE) for satisfiability; past adds expressiveness without complexity.
**Real Wall:** Past operators awkward in monitoring tools; many implementations omit them.
**Cross-Domain Aliases:** past_ltl, ltl_past, ptl
**Notes:** Lichtenstein, Pnueli, Zuck 1985; equivalent expressiveness to pure-future LTL.

---

### LR.106: Buchi Automaton
**Definition:** Nondeterministic ω-automaton accepting infinite words via infinite visits to accepting states.
**Cost Model:** Construction from LTL formula exponential 2^O(|φ|); emptiness check linear.
**Real Wall:** Determinization (Safra) doubly exponential; preferred to avoid for translation.
**Cross-Domain Aliases:** buchi_automaton, omega_automaton, nondeterministic_buchi
**Notes:** Büchi 1962; foundation of automata-theoretic model checking.

---

### LR.107: Omega Automaton
**Definition:** Finite automaton with acceptance condition over infinite words (Büchi, Rabin, Streett, parity, Muller).
**Cost Model:** Acceptance conditions trade succinctness for complexity of operations.
**Real Wall:** Determinization complexity varies by acceptance condition; parity vs Rabin trade-offs.
**Cross-Domain Aliases:** omega_automaton, infinite_word_automaton
**Notes:** Foundational for verification of reactive systems; Vardi's framework.

---

### LR.108: Rabin Automaton
**Definition:** Deterministic ω-automaton with acceptance condition as set of pairs (L_i, U_i) of state sets.
**Cost Model:** Determinizable from Büchi via Safra construction; exponential.
**Real Wall:** Safra construction implementation notoriously hard; modern tools use it sparingly.
**Cross-Domain Aliases:** rabin_automaton, rabin_condition
**Notes:** Rabin 1969; used for synthesis where determinism essential.

---

### LR.109: Parity Game
**Definition:** Infinite-duration game on graph where priorities determine winner via parity of max priority occurring infinitely often.
**Cost Model:** Polynomial in vertices and edges per level of priority hierarchy; total complexity open.
**Real Wall:** Quasi-polynomial algorithms exist (Calude et al. 2017); polynomial complexity famous open problem.
**Cross-Domain Aliases:** parity_game, max_parity_game
**Notes:** Foundation of mu-calculus model checking; tied to synthesis.

---

### LR.110: LTL Synthesis
**Definition:** Construction of a finite-state controller realizing an LTL specification against an adversarial environment.
**Cost Model:** 2EXPTIME-complete in formula size; Safra construction central.
**Real Wall:** Doubly exponential makes industrial-scale synthesis infeasible; restricted fragments (GR(1)) used.
**Cross-Domain Aliases:** ltl_synthesis, reactive_synthesis
**Notes:** Pnueli & Rosner 1989; SyntComp annual synthesis competition.

---

### LR.111: GR(1) Synthesis
**Definition:** Generalized Reactivity rank 1: tractable LTL synthesis fragment with conjunctions of safety and Boolean fairness.
**Cost Model:** Cubic in state space; polynomial in formula for fixed structure.
**Real Wall:** Restriction excludes many natural specifications; but covers most reactive controllers.
**Cross-Domain Aliases:** gr1, gr_one, piterman_pnueli_synthesis
**Notes:** Piterman, Pnueli, Sa'ar 2006; basis of practical reactive synthesis tools.

---

### LR.112: Fairness Constraint
**Definition:** Constraint requiring certain transitions or states to be visited infinitely often during execution.
**Cost Model:** Adds Büchi acceptance conditions; integrated into model checking algorithms.
**Real Wall:** Fairness modeling fragile; over/under-specification yields wrong verification outcomes.
**Cross-Domain Aliases:** fairness, weak_fairness, strong_fairness
**Notes:** Lamport's fairness; distinguishes weak (always enabled → eventually taken) from strong.

---

### LR.113: Symbolic Model Checking
**Definition:** Model checking using symbolic representations (typically BDDs) of state sets and transition relations.
**Cost Model:** Exponential worst case but practical for many circuits with ~100 state bits.
**Real Wall:** BDD size sensitive to variable ordering; can blow up unpredictably.
**Cross-Domain Aliases:** symbolic_mc, smv, nusmv
**Notes:** McMillan 1992; SMV/NuSMV implementation; pioneered industrial model checking.

---

### LR.114: Bounded Model Checking
**Definition:** Search for counterexamples up to a fixed unrolling depth via reduction to SAT.
**Cost Model:** SAT solver on unrolled instance; depth k yields O(k·|M|) sized formula.
**Real Wall:** Incomplete for safety properties without completeness threshold; k-induction helps.
**Cross-Domain Aliases:** bmc, bounded_mc
**Notes:** Biere et al. 1999; killer app of SAT solvers in hardware verification.

---

### LR.115: K-Induction
**Definition:** Inductive proof of safety property: base case for k steps, induction over k consecutive steps.
**Cost Model:** k SAT calls; completeness threshold may be large.
**Real Wall:** Determining sufficient k often requires invariant strengthening.
**Cross-Domain Aliases:** k_induction, induction_proof_safety
**Notes:** Sheeran, Singh, Stålmarck 2000; foundation of safety property proving.

---

## Section 6. Description Logics & Ontologies

### LR.116: Description Logic ALC
**Definition:** Attributive Concept Language with Complement: minimal expressive DL with conjunction, disjunction, negation, existential and universal restrictions.
**Cost Model:** PSPACE-complete satisfiability; tableau decision procedure standard.
**Real Wall:** Even ALC blowup possible; modern reasoners optimize with caching and absorption.
**Cross-Domain Aliases:** alc, attributive_language_complement
**Notes:** Schmidt-Schauß & Smolka 1991; foundation of expressive DLs.

---

### LR.117: ALCN
**Definition:** ALC extended with unqualified number restrictions (≥n R, ≤n R).
**Cost Model:** PSPACE-complete; number reasoning via choose rule.
**Real Wall:** Cardinality reasoning interacts subtly with role hierarchies.
**Cross-Domain Aliases:** alcn, alc_with_number_restrictions
**Notes:** Number restrictions limit role fillers without specifying their type.

---

### LR.118: SHOIQ
**Definition:** Very expressive DL with role hierarchies, transitive roles, nominals, inverse roles, and qualified number restrictions.
**Cost Model:** NEXPTIME-complete; tableau with sophisticated blocking.
**Real Wall:** Nominals and inverse roles together cause exponential blow-up; pre-completion graphs needed.
**Cross-Domain Aliases:** shoiq, owl_dl_logic
**Notes:** Logical basis of OWL DL; Horrocks et al.

---

### LR.119: SROIQ
**Definition:** SHOIQ extended with complex role inclusion axioms, self-restrictions, and reflexive/irreflexive roles.
**Cost Model:** N2EXPTIME-complete; doubly exponential.
**Real Wall:** Implementation complexity high; few full SROIQ reasoners exist.
**Cross-Domain Aliases:** sroiq, owl2_dl_logic
**Notes:** Horrocks, Kutz, Sattler 2006; logical basis of OWL 2 DL.

---

### LR.120: DL-Lite
**Definition:** Family of lightweight description logics designed for tractable ontology-based data access.
**Cost Model:** Polynomial-time reasoning; first-order rewritable.
**Real Wall:** Limited expressiveness; cannot express many natural axioms.
**Cross-Domain Aliases:** dl_lite, lightweight_dl
**Notes:** Calvanese et al.; basis of OWL 2 QL profile.

---

### LR.121: Description Logic EL
**Definition:** DL with conjunction, existential restrictions, and top; admits polynomial classification.
**Cost Model:** Polynomial-time subsumption via completion algorithm.
**Real Wall:** Very limited expressiveness; cannot express disjunction or universal restrictions.
**Cross-Domain Aliases:** el, el_plus_plus
**Notes:** Baader et al.; basis of OWL 2 EL profile; used for SNOMED CT medical ontology.

---

### LR.122: OWL DL
**Definition:** Web Ontology Language with computability via mapping to SHOIN(D); decidable description logic profile.
**Cost Model:** NEXPTIME-complete for reasoning.
**Real Wall:** Decidable but impractical for large ontologies; profiles (EL, QL, RL) used in practice.
**Cross-Domain Aliases:** owl_dl, owl_description_logic
**Notes:** W3C 2004; basis for Semantic Web ontology reasoning.

---

### LR.123: OWL 2 EL Profile
**Definition:** Polynomial-time reasoning profile of OWL 2 based on EL++; used for large bio-medical ontologies.
**Cost Model:** Polynomial in size of ontology; suitable for million-axiom ontologies.
**Real Wall:** Limited expressiveness; not all conceptual modeling possible.
**Cross-Domain Aliases:** owl2_el, el_profile
**Notes:** ELK reasoner; classifies SNOMED CT in seconds.

---

### LR.124: OWL 2 QL Profile
**Definition:** OWL 2 profile based on DL-Lite_R; designed for ontology-mediated query answering.
**Cost Model:** Reasoning via SQL rewriting; AC0 data complexity.
**Real Wall:** Very weak expressiveness; primarily for accessing databases via ontologies.
**Cross-Domain Aliases:** owl2_ql, ql_profile
**Notes:** Maps to relational database queries; basis of OBDA systems.

---

### LR.125: OWL 2 RL Profile
**Definition:** OWL 2 profile implementable in rule-based systems via Datalog.
**Cost Model:** Polynomial reasoning via Datalog evaluation.
**Real Wall:** Cannot capture all OWL DL; lossy translation.
**Cross-Domain Aliases:** owl2_rl, rl_profile
**Notes:** Compatible with rule-engine implementations; bridges DL and rules.

---

### LR.126: DL Tableau Algorithm
**Definition:** Tableau decision procedure for description logics with completion, blocking, and dependency-directed backtracking.
**Cost Model:** Worst case exponential or worse; optimizations (caching, absorption) crucial.
**Real Wall:** Blocking conditions correctness subtle; reasoner bugs historically common.
**Cross-Domain Aliases:** dl_tableau, kuxxx_tableau
**Notes:** Horrocks; basis of FaCT++, Pellet, HermiT reasoners.

---

### LR.127: Concept Subsumption
**Definition:** Determining whether one concept is a sub-concept of another in a description logic ontology.
**Cost Model:** Reduces to satisfiability of conjunction with negation.
**Real Wall:** Core reasoning task; classification computes all subsumptions.
**Cross-Domain Aliases:** subsumption, concept_inclusion
**Notes:** Central inference task in DL knowledge bases.

---

### LR.128: Role Hierarchy
**Definition:** Partial order on roles in DL, with axioms R ⊑ S meaning R is a sub-role of S.
**Cost Model:** Precomputed transitive closure of role hierarchy.
**Real Wall:** Complex role inclusions (R∘S ⊑ T) can cause undecidability; SROIQ restricts them.
**Cross-Domain Aliases:** role_hierarchy, role_subsumption
**Notes:** Distinguishes simple from complex roles in SROIQ.

---

### LR.129: Nominals In Description Logic
**Definition:** Singleton concepts {o} consisting of exactly one named individual.
**Cost Model:** Increase reasoning complexity; tableau requires individual-level reasoning.
**Real Wall:** Nominals + inverse roles + number restrictions = NEXPTIME.
**Cross-Domain Aliases:** nominals, dl_individuals_as_concepts
**Notes:** The 'O' in SHOIQ; allow naming specific individuals in TBox axioms.

---

### LR.130: ABox Reasoning
**Definition:** Reasoning about assertions on individuals (instance checking, conjunctive query answering).
**Cost Model:** Typically harder than TBox reasoning; data complexity often coNP.
**Real Wall:** Large ABox scaling; combined approach with materialization or query rewriting.
**Cross-Domain Aliases:** abox, instance_reasoning
**Notes:** ABox vs TBox separation; OBDA focuses on ABox via QL profile.

---

## Section 7. ASP & Logic Programming

### LR.131: Answer Set Programming
**Definition:** Declarative programming paradigm with logic programs interpreted under stable model semantics.
**Cost Model:** Σ_2^P-complete for disjunctive programs; ground program evaluation via SAT-like search.
**Real Wall:** Grounding step can produce huge intermediate programs; smart grounders essential.
**Cross-Domain Aliases:** asp, stable_model_programming
**Notes:** Gelfond & Lifschitz 1988; clingo (Potassco) state of the art.

---

### LR.132: Stable Model Semantics
**Definition:** Semantics for logic programs via fixpoint: M is a stable model if M is minimal model of program reduct P^M.
**Cost Model:** Existence is NP-complete (normal programs); coNP for verification.
**Real Wall:** Non-monotonicity makes incremental reasoning hard; minor changes can drastically alter answer sets.
**Cross-Domain Aliases:** stable_model, gelfond_lifschitz_semantics
**Notes:** Foundation of ASP; equivalent to well-founded semantics for stratified programs.

---

### LR.133: Well-Founded Semantics
**Definition:** Three-valued semantics for logic programs with negation as failure; computes unique minimal model.
**Cost Model:** Polynomial time for ground programs.
**Real Wall:** Three-valued (true/false/undefined) less expressive than stable models for some uses.
**Cross-Domain Aliases:** wfs, well_founded_model
**Notes:** Van Gelder, Ross, Schlipf 1991; basis of XSB Prolog.

---

### LR.134: ASP Grounder
**Definition:** Program transformation eliminating variables by enumerating all valid ground instances.
**Cost Model:** Worst-case exponential in arity and domain size; gringo uses semi-naive bottom-up.
**Real Wall:** Grounding bottleneck for large domains; intelligent grounding crucial.
**Cross-Domain Aliases:** grounder, gringo, asp_grounding
**Notes:** gringo (Potassco); lazy grounding research ongoing.

---

### LR.135: ASP Solver
**Definition:** Search procedure for stable models of ground ASP programs using CDCL-like algorithm.
**Cost Model:** CDCL adapted with unfounded set detection; SAT solver heart plus loop handling.
**Real Wall:** Loop nogood maintenance; conflict analysis must respect minimality semantics.
**Cross-Domain Aliases:** asp_solver, clasp, claspq
**Notes:** clasp solver in Potassco suite; integrated in clingo.

---

### LR.136: Disjunctive Logic Program
**Definition:** Logic program allowing disjunction in heads of rules; expressive power Σ_2^P.
**Cost Model:** Stable model existence Σ_2^P-complete.
**Real Wall:** Higher complexity than normal programs; harder to ground efficiently.
**Cross-Domain Aliases:** disjunctive_lp, disjunctive_asp
**Notes:** DLV solver specialty; clingo also supports.

---

### LR.137: Choice Rule
**Definition:** ASP construct {p(X) : domain(X)} = n meaning choose exactly n atoms p(X) where X is in domain.
**Cost Model:** Compiled to multiple normal rules with cardinality constraints.
**Real Wall:** Cardinality bounds can blow up grounding; aggregate handling tricky.
**Cross-Domain Aliases:** choice_rule, cardinality_rule
**Notes:** Niemelä extension; pervasive in modern ASP encodings.

---

### LR.138: Aggregate In ASP
**Definition:** Constructs like #sum, #count, #min, #max over sets of literals; allow concise constraints.
**Cost Model:** Aggregate-to-cardinality translation potentially blows up.
**Real Wall:** Aggregate semantics under recursion subtle (Ferraris semantics most general).
**Cross-Domain Aliases:** aggregate, asp_aggregate
**Notes:** clingo supports; semantics standardized but variant interpretations exist.

---

### LR.139: SLD Resolution
**Definition:** Selective Linear Definite resolution: depth-first goal-directed resolution for Horn clauses.
**Cost Model:** Depth-first backtracking; potentially infinite branches without tabling.
**Real Wall:** Loops on left-recursive programs; cycle detection via tabling.
**Cross-Domain Aliases:** sld, sld_resolution, prolog_resolution
**Notes:** Robinson + Kowalski refinement; Prolog computation rule.

---

### LR.140: Tabling In Logic Programming
**Definition:** Memoization technique storing solved subgoals to avoid recomputation and ensure termination.
**Cost Model:** Polynomial for Datalog programs via tabling; exponential without.
**Real Wall:** Memory overhead of table maintenance; subsumption checking expensive.
**Cross-Domain Aliases:** tabling, slg_resolution, well_founded_evaluation
**Notes:** XSB Prolog and B-Prolog support; foundation of efficient Datalog evaluation.

---

### LR.141: Datalog
**Definition:** Logic programming language restricted to function-free Horn clauses; polynomial-time query answering.
**Cost Model:** EXPTIME-complete combined complexity; PTIME data complexity.
**Real Wall:** No function symbols means cannot express many natural recursive structures.
**Cross-Domain Aliases:** datalog, function_free_prolog
**Notes:** Database community foundation; LogicBlox, Soufflé, RDFox engines.

---

### LR.142: Semi-Naive Evaluation
**Definition:** Bottom-up Datalog evaluation strategy avoiding rederivation by only using new tuples.
**Cost Model:** Fixed-point computation; each iteration time linear in delta size.
**Real Wall:** Memory for storing intermediate relations; differential approach for incremental.
**Cross-Domain Aliases:** semi_naive, bottom_up_evaluation
**Notes:** Bancilhon 1986; foundation of modern Datalog engines.

---

### LR.143: Magic Sets Transformation
**Definition:** Rewriting Datalog program to simulate top-down evaluation bottom-up, focusing computation.
**Cost Model:** Adds 'magic' predicates filtering relevant tuples.
**Real Wall:** Transformation produces larger program; benefit depends on query selectivity.
**Cross-Domain Aliases:** magic_sets, supplementary_magic
**Notes:** Bancilhon, Maier, Sagiv, Ullman 1986; combines top-down focus with bottom-up evaluation.

---

### LR.144: Prolog Cut
**Definition:** Control construct '!' that prevents backtracking past it within a rule.
**Cost Model:** O(1) per cut; pragmatic control over search.
**Real Wall:** Cut destroys logical purity; semantically equivalent transformations break.
**Cross-Domain Aliases:** prolog_cut, control_cut
**Notes:** Necessary evil in classical Prolog; modern declarative variants avoid via constraints.

---

### LR.145: Constraint Logic Programming
**Definition:** Logic programming integrated with constraint solvers (CLP(R), CLP(FD), CLP(B)).
**Cost Model:** Underlying constraint solver dictates; constraint propagation interleaved with unification.
**Real Wall:** Constraint solver integration adds complexity; debugging non-trivial.
**Cross-Domain Aliases:** clp, constraint_lp
**Notes:** Jaffar & Maher 1987; SICStus, ECLiPSe, SWI-Prolog implementations.

---

## Section 8. Inductive Logic Programming

### LR.146: Inductive Logic Programming
**Definition:** Machine learning paradigm learning logic programs from examples and background knowledge.
**Cost Model:** Search through hypothesis space; refinement operators expand candidates.
**Real Wall:** Search space exponential in vocabulary; mode declarations and language bias prune.
**Cross-Domain Aliases:** ilp, inductive_logic_programming
**Notes:** Muggleton 1991; FOIL, PROGOL, ALEPH, Metagol systems.

---

### LR.147: FOIL Algorithm
**Definition:** First-Order Inductive Learner: top-down greedy learning of definite clauses via information-gain heuristic.
**Cost Model:** Greedy clause construction; per-clause polynomial in examples and predicates.
**Real Wall:** Greedy choice locks in suboptimal literals; cannot recover from poor early decisions.
**Cross-Domain Aliases:** foil, first_order_inductive_learner
**Notes:** Quinlan 1990; inspired by ID3 decision tree learner adapted to FOL.

---

### LR.148: PROGOL Algorithm
**Definition:** ILP system using inverse entailment to construct most specific clause covering an example, then generalizing.
**Cost Model:** Per-example bottom clause construction; generalization via theta-subsumption.
**Real Wall:** Bottom clauses can be huge; mode declarations crucial to bound them.
**Cross-Domain Aliases:** progol, inverse_entailment
**Notes:** Muggleton 1995; foundation of mode-directed ILP.

---

### LR.149: Mode Declaration
**Definition:** ILP language bias specifying input/output types and modes of predicate arguments.
**Cost Model:** Pruning factor on hypothesis search.
**Real Wall:** Mode declarations require domain expertise; poor modes blow up search.
**Cross-Domain Aliases:** mode_decl, mode_directed_ilp
**Notes:** Critical for practical ILP; ALEPH and Metagol use them extensively.

---

### LR.150: Predicate Invention
**Definition:** ILP technique creating new predicate symbols to capture latent concepts not in vocabulary.
**Cost Model:** Greatly expands hypothesis space; Metagol uses metarules to control.
**Real Wall:** Search becomes intractable without strong bias; metarules a research area.
**Cross-Domain Aliases:** predicate_invention, concept_invention
**Notes:** Metagol (Muggleton et al.); key to learning expressive programs.

---

### LR.151: Theta Subsumption
**Definition:** Generalization ordering on clauses: C θ-subsumes D iff there exists substitution θ with Cθ ⊆ D.
**Cost Model:** NP-complete to decide in general.
**Real Wall:** Subsumption checks dominate ILP runtime; subsumption indexing essential.
**Cross-Domain Aliases:** theta_subsumption, plotkin_subsumption
**Notes:** Plotkin 1970; basis for refinement operators.

---

### LR.152: Refinement Operator
**Definition:** Function mapping a clause to its specializations (or generalizations) for ILP hypothesis search.
**Cost Model:** Search step per refinement; cost depends on operator's branching factor.
**Real Wall:** Operator design tradeoffs: completeness vs. efficiency.
**Cross-Domain Aliases:** refinement_op, downward_refinement
**Notes:** van der Laag & Nienhuys-Cheng; foundational ILP theory.

---

### LR.153: Least General Generalization
**Definition:** Operation finding the most specific clause that θ-subsumes two given clauses.
**Cost Model:** Exponential in number of clauses; LGG of n clauses can blow up.
**Real Wall:** Practical LGG limited to small clause sets; greedy approaches used.
**Cross-Domain Aliases:** lgg, plotkin_lgg
**Notes:** Plotkin 1970; basis of bottom-up ILP approaches.

---

### LR.154: Metagol
**Definition:** ILP system using meta-interpreters and metarules to learn programs with predicate invention.
**Cost Model:** Search over metarule instantiations; abductive reasoning over metarules.
**Real Wall:** Metarule design is a meta-level engineering problem; transfer learning approaches help.
**Cross-Domain Aliases:** metagol, meta_interpretive_learning
**Notes:** Muggleton, Lin, Tamaddoni-Nezhad 2015; learns higher-order programs.

---

### LR.155: Probabilistic Logic Programming
**Definition:** Extension of logic programming with probability distributions over models or proofs.
**Cost Model:** #P-hard for general inference; bounded approximations via knowledge compilation.
**Real Wall:** Combining logic and probability is computationally explosive; ProbLog uses SDDs.
**Cross-Domain Aliases:** plp, problog
**Notes:** Sato distribution semantics; ProbLog, PRISM systems.

---

## Section 9. Non-Monotonic Reasoning

### LR.156: Default Logic
**Definition:** Non-monotonic logic with default rules of form 'if A, and B is consistent, then conclude C'.
**Cost Model:** Extension existence Σ_2^P-complete for propositional defaults.
**Real Wall:** Extensions may not exist or be multiple; default ordering controversial.
**Cross-Domain Aliases:** default_logic, reiter_defaults
**Notes:** Reiter 1980; foundational non-monotonic formalism.

---

### LR.157: Circumscription
**Definition:** Minimization semantics: prefer models where extensions of certain predicates are minimal.
**Cost Model:** Π_2^P-complete for propositional circumscription.
**Real Wall:** Computational complexity high; not widely implemented as standalone.
**Cross-Domain Aliases:** circumscription, mccarthy_circumscription
**Notes:** McCarthy 1980; formalizes closed-world assumptions.

---

### LR.158: Autoepistemic Logic
**Definition:** Modal logic for reasoning about an agent's own beliefs; uses operator L for 'believes'.
**Cost Model:** Σ_2^P-complete for stable expansion existence.
**Real Wall:** Subtleties in iterated belief; multiple stable expansions per theory.
**Cross-Domain Aliases:** auto_epistemic, moore_logic
**Notes:** Moore 1985; related to ASP and stable model semantics.

---

### LR.159: Closed World Assumption
**Definition:** Heuristic: any ground atom not provable from the knowledge base is assumed false.
**Cost Model:** Trivially polynomial given decision procedure for positive facts.
**Real Wall:** Fails when knowledge incomplete; database semantics implicitly assumes it.
**Cross-Domain Aliases:** cwa, closed_world
**Notes:** Reiter 1978; standard in databases and logic programming.

---

### LR.160: Negation As Failure
**Definition:** Prolog/ASP convention treating goals that fail to prove as false.
**Cost Model:** Failure semantics: not P succeeds iff P fails finitely.
**Real Wall:** Non-monotonic: adding facts can change conclusions; floundering on free variables.
**Cross-Domain Aliases:** naf, negation_as_failure
**Notes:** Clark 1978; basis for default reasoning in logic programs.

---

### LR.161: Predicate Completion
**Definition:** Procedure converting if-rules into iff-definitions, making negation classical.
**Cost Model:** Linear transformation per predicate.
**Real Wall:** Loses non-monotonicity; fixed-point semantics differs from completion.
**Cross-Domain Aliases:** clark_completion, predicate_completion
**Notes:** Clark 1978; relates negation as failure to classical negation.

---

### LR.162: Frame Problem
**Definition:** Difficulty of representing in logic which facts persist when actions are performed.
**Cost Model:** Naive frame axioms quadratic in fluents and actions.
**Real Wall:** Combinatorial explosion of frame axioms; situation calculus and event calculus address.
**Cross-Domain Aliases:** frame_problem, mccarthy_hayes_frame
**Notes:** McCarthy & Hayes 1969; central problem in AI knowledge representation.

---

### LR.163: Situation Calculus
**Definition:** First-order formalism for reasoning about actions and change, with situation as first-class entity.
**Cost Model:** Reasoning is undecidable in general; restricted fragments tractable.
**Real Wall:** Successor state axioms tame frame problem but still expensive to reason about.
**Cross-Domain Aliases:** situation_calculus, reiter_sitcalc
**Notes:** McCarthy 1963; Reiter 1991 modern formulation.

---

### LR.164: Event Calculus
**Definition:** First-order action formalism using events, fluents, and time points as primitives.
**Cost Model:** Subset of first-order logic; restrictable to decidable fragments.
**Real Wall:** Combining with planning is computationally hard; SAT-based and ASP encodings used.
**Cross-Domain Aliases:** event_calculus, ec_kowalski
**Notes:** Kowalski & Sergot 1986; alternative to situation calculus.

---

### LR.165: Belief Revision
**Definition:** Formal study of how rational agents change their beliefs in response to new information.
**Cost Model:** AGM postulates characterize rational revision; implementation via partial meet contraction.
**Real Wall:** Multiple consistent revisions exist; tie-breaking philosophical.
**Cross-Domain Aliases:** belief_revision, agm_revision
**Notes:** Alchourrón, Gärdenfors, Makinson 1985; foundational AGM framework.

---

### LR.166: Truth Maintenance System
**Definition:** Subsystem of reasoning system tracking dependencies among beliefs for retraction.
**Cost Model:** Polynomial in beliefs and justifications; assumption-based variants more expressive.
**Real Wall:** ATMS exponential in number of assumptions; trades memory for query speed.
**Cross-Domain Aliases:** tms, atms, jtms
**Notes:** Doyle 1979 JTMS; de Kleer 1986 ATMS.

---

### LR.167: Argumentation Framework
**Definition:** Directed graph of arguments with attack relations; semantics determine acceptability sets.
**Cost Model:** Verification polynomial; extension existence NP-complete or harder.
**Real Wall:** Multiple semantics (grounded, preferred, stable) for same framework.
**Cross-Domain Aliases:** dung_framework, argumentation
**Notes:** Dung 1995; central in AI ethics and legal reasoning.

---

### LR.168: Inheritance Network
**Definition:** Graph representing class hierarchy with default and strict inheritance links.
**Cost Model:** Path-based inheritance NP-hard; multiple inheritance with exceptions tricky.
**Real Wall:** Conflicting paths require preference ordering; Nixon Diamond classical example.
**Cross-Domain Aliases:** inheritance_network, default_inheritance
**Notes:** Touretzky 1986; precursor to description logic class hierarchies.

---

### LR.169: Reiter Default
**Definition:** Default rule of form (A : B) / C meaning: if A, B consistent with theory, then conclude C.
**Cost Model:** Decision via fixpoint over justifications.
**Real Wall:** Extensions may not exist or be multiple; ordered defaults a remedy.
**Cross-Domain Aliases:** reiter_default, normal_default
**Notes:** Normal defaults (where B = C) are well-behaved subclass.

---

### LR.170: Skeptical Vs Credulous Reasoning
**Definition:** Skeptical: conclusion is consequence of every extension; Credulous: conclusion in some extension.
**Cost Model:** Skeptical Π-level harder than credulous Σ-level.
**Real Wall:** Application determines choice; tools should support both.
**Cross-Domain Aliases:** skeptical_inference, credulous_inference, brave_cautious
**Notes:** Standard distinction in non-monotonic reasoning theory.

---

## Section 10. Verification & Hoare-Style Reasoning

### LR.171: Hoare Logic
**Definition:** Axiomatic semantics with triples {P} S {Q}: if precondition P holds before statement S, postcondition Q holds after.
**Cost Model:** Proof rules per statement type; loop invariant supply required.
**Real Wall:** Loop invariant discovery is the central undecidable bottleneck; user must provide.
**Cross-Domain Aliases:** hoare_logic, axiomatic_semantics
**Notes:** Hoare 1969; foundation of program verification.

---

### LR.172: Weakest Precondition
**Definition:** Predicate transformer wp(S, Q) yielding the weakest predicate ensuring Q holds after executing S.
**Cost Model:** Backward symbolic execution; exponential in branching for full path enumeration.
**Real Wall:** Quantifier elimination required; non-arithmetic theories cause undecidability.
**Cross-Domain Aliases:** wp, weakest_pre, dijkstra_wp
**Notes:** Dijkstra 1976; foundation of guarded command language semantics.

---

### LR.173: Strongest Postcondition
**Definition:** Predicate transformer sp(P, S) yielding the strongest predicate guaranteed to hold after S given P.
**Cost Model:** Forward symbolic execution; exponential in branching.
**Real Wall:** Strongest postcondition typically uglier than weakest precondition; introduces existentials.
**Cross-Domain Aliases:** sp, strongest_post
**Notes:** Dual of weakest precondition; preferred direction depends on tool.

---

### LR.174: Separation Logic
**Definition:** Extension of Hoare logic with separating conjunction (P * Q) and points-to (x ↦ v) for heap reasoning.
**Cost Model:** Frame rule enables local reasoning; entailment between assertions still undecidable in general.
**Real Wall:** Quantifier reasoning in symbolic heap; bi-abduction central but expensive.
**Cross-Domain Aliases:** separation_logic, sl
**Notes:** Reynolds 2002; basis of Infer, VeriFast, Iris.

---

### LR.175: Frame Rule
**Definition:** Inference rule in separation logic: if {P} C {Q}, then {P*R} C {Q*R} for non-modified R.
**Cost Model:** Allows local reasoning; framing inference per call.
**Real Wall:** Determining what to frame is the bi-abduction problem.
**Cross-Domain Aliases:** frame_rule, sl_frame
**Notes:** Enables compositional reasoning about heap-manipulating programs.

---

### LR.176: Refinement Type
**Definition:** Type system extension with predicates: e.g., {x : Int | x > 0} is the type of positive integers.
**Cost Model:** Subtyping checks reduce to SMT queries.
**Real Wall:** SMT query complexity dominates; expressive refinements cause undecidability.
**Cross-Domain Aliases:** refinement_type, liquid_type
**Notes:** LiquidHaskell, Refinement Types in F*, Stainless.

---

### LR.177: Dependent Type
**Definition:** Type that depends on values; e.g., Vec n A is a vector of length n containing values of type A.
**Cost Model:** Type checking can be arbitrarily complex; full dependent types undecidable.
**Real Wall:** Definitional equality decidable; propositional equality requires proofs.
**Cross-Domain Aliases:** dependent_type, pi_type
**Notes:** Per Martin-Löf 1971; basis of Coq, Agda, Lean, Idris.

---

### LR.178: Symbolic Execution
**Definition:** Program analysis interpreting code on symbolic inputs, accumulating path conditions.
**Cost Model:** Path explosion exponential in branches; SMT queries dominate.
**Real Wall:** Path explosion in loops and recursion; merging vs. forking strategies.
**Cross-Domain Aliases:** symbolic_execution, sym_exec, klee_style
**Notes:** King 1976; KLEE, angr, S2E modern tools.

---

### LR.179: Abstract Interpretation
**Definition:** Static analysis framework computing sound over-approximations of program semantics via abstract domains.
**Cost Model:** Per-statement transfer functions; widening for fixpoint convergence.
**Real Wall:** Choice of abstract domain trades precision for cost; widening loses precision.
**Cross-Domain Aliases:** ai, abstract_interpretation, cousot_framework
**Notes:** Cousot & Cousot 1977; Astrée, IKOS, Apron library.

---

### LR.180: Model Checking
**Definition:** Algorithmic verification of finite-state system against temporal specification.
**Cost Model:** Polynomial in model size for CTL; PSPACE in formula for LTL.
**Real Wall:** State space explosion; symbolic and partial-order reduction techniques essential.
**Cross-Domain Aliases:** model_checking, mc
**Notes:** Clarke, Emerson, Sifakis Turing Award 2007.

---

### LR.181: CEGAR
**Definition:** Counterexample-Guided Abstraction Refinement: iteratively refine abstraction using spurious counterexamples.
**Cost Model:** Loop of abstract model checking + concrete simulation + refinement.
**Real Wall:** Refinement may not converge; predicate discovery central problem.
**Cross-Domain Aliases:** cegar, counterexample_refinement
**Notes:** Clarke et al. 2000; basis of SLAM, BLAST verifiers.

---

### LR.182: Predicate Abstraction
**Definition:** Construction of finite abstract model using truth values of given predicates as abstract states.
**Cost Model:** Abstract model exponential in number of predicates.
**Real Wall:** Choosing right predicates is the central skill; Craig interpolation helps.
**Cross-Domain Aliases:** predicate_abstraction, pa
**Notes:** Graf & Saïdi 1997; CEGAR engine in modern verifiers.

---

### LR.183: Craig Interpolation
**Definition:** Given unsatisfiable A ∧ B, find formula I in shared vocabulary such that A ⇒ I and I ⇒ ¬B.
**Cost Model:** Polynomial in unsatisfiability proof for propositional/LRA fragments.
**Real Wall:** Quality of interpolant matters; small interpolants better but harder to extract.
**Cross-Domain Aliases:** interpolation, craig_interpolant
**Notes:** Craig 1957; McMillan 2003 algorithmic; central to IC3/PDR.

---

### LR.184: IC3 / PDR
**Definition:** Property Directed Reachability: incremental inductive verification via frames and counterexample blocking.
**Cost Model:** Per-frame SAT calls; frame number bounded by completeness threshold.
**Real Wall:** Inductive invariant generation; generalization of blocking lemmas crucial.
**Cross-Domain Aliases:** ic3, pdr, property_directed_reachability
**Notes:** Bradley 2011; standard in hardware verification (ABC).

---

### LR.185: Auto-Active Verification
**Definition:** Verification approach where user supplies annotations (contracts, invariants) and tool checks automatically.
**Cost Model:** SMT query per verification condition; user effort in annotation.
**Real Wall:** Annotation burden; loop invariant inference research helps.
**Cross-Domain Aliases:** auto_active, contract_based_verification
**Notes:** Dafny, Why3, Frama-C; middle ground between fully automatic and interactive.

---

## Section 11. Decision Diagrams & Quantifier Reasoning

### LR.186: Reduced Ordered BDD
**Definition:** Canonical representation of Boolean function as DAG with fixed variable order, reduced (no isomorphic subgraphs or redundant nodes).
**Cost Model:** Operations Apply, Restrict in O(|f|·|g|); canonical form enables equality check in O(1).
**Real Wall:** Size exponentially sensitive to variable ordering; finding optimal NP-hard.
**Cross-Domain Aliases:** robdd, bryant_bdd, ordered_bdd
**Notes:** Bryant 1986; revolutionary for hardware verification.

---

### LR.187: Variable Ordering For BDDs
**Definition:** Permutation of variables that determines BDD layer structure; critical to BDD size.
**Cost Model:** Static heuristics (FORCE, fan-in) plus dynamic sifting.
**Real Wall:** No general algorithm guarantees compact ordering; exponential variation possible.
**Cross-Domain Aliases:** bdd_variable_ordering, sifting
**Notes:** Rudell sifting 1993; large literature on heuristics.

---

### LR.188: Zero-Suppressed BDD
**Definition:** BDD variant suppressing nodes with zero high-child; compact for sparse Boolean functions.
**Cost Model:** Same operations as BDD but different reduction rule.
**Real Wall:** Better for combinatorial enumeration; less for general logic.
**Cross-Domain Aliases:** zdd, zero_suppressed_bdd
**Notes:** Minato 1993; used for enumeration problems and combinatorial sets.

---

### LR.189: Algebraic Decision Diagram
**Definition:** BDD generalization with leaves in arbitrary algebraic domain (reals, finite ring).
**Cost Model:** Operations parameterized by domain; matrix-style multiplication via Shannon expansion.
**Real Wall:** Many ADD leaves with distinct values reduce compaction benefit.
**Cross-Domain Aliases:** add, mtbdd, multi_terminal_bdd
**Notes:** Bahar et al. 1993; used in probabilistic model checking.

---

### LR.190: Sentential Decision Diagram
**Definition:** Knowledge compilation target with structured decomposition based on vtree.
**Cost Model:** Tractable conjunction with respect to compatible vtrees.
**Real Wall:** Vtree learning is a research area; sensitive to data structure.
**Cross-Domain Aliases:** sdd, sentential_dd
**Notes:** Darwiche 2011; supports model counting and probabilistic queries.

---

### LR.191: E-Matching
**Definition:** Quantifier instantiation technique matching trigger terms against ground terms in the e-graph.
**Cost Model:** Pattern matching against e-graph; index-based to be fast.
**Real Wall:** Trigger quality dictates effectiveness; matching loops and trigger explosion common.
**Cross-Domain Aliases:** e_matching, ematching, trigger_matching
**Notes:** Standard quantifier handling in Z3, CVC5; basis of SMT quantifier reasoning.

---

### LR.192: Model-Based Quantifier Instantiation
**Definition:** Quantifier handling that builds candidate model and instantiates quantifiers refuted by model.
**Cost Model:** Model construction per round; refinement loop.
**Real Wall:** Model representation must be finite/decidable; works best for definitional quantifiers.
**Cross-Domain Aliases:** mbqi, model_based_qi
**Notes:** Ge & de Moura 2009; complements E-matching in modern SMT.

---

### LR.193: Trigger
**Definition:** Pattern subterm used to guide quantifier instantiation in SMT solvers.
**Cost Model:** Selection per quantifier; manual or auto-generated.
**Real Wall:** Bad triggers cause matching loops or incompleteness; tuning quantifier-heavy specs hard.
**Cross-Domain Aliases:** trigger, pattern, smt_trigger
**Notes:** Z3 supports user-supplied triggers; Dafny verification highly sensitive to them.

---

### LR.194: Knowledge Compilation
**Definition:** Compilation of propositional knowledge base into target language supporting tractable queries.
**Cost Model:** Compilation potentially exponential; queries polynomial.
**Real Wall:** Compilation cost amortized over many queries; useful only when queries repeated.
**Cross-Domain Aliases:** knowledge_compilation, kc
**Notes:** Darwiche & Marquis 2002; map of compilation languages.

---

### LR.195: D-DNNF
**Definition:** Deterministic Decomposable Negation Normal Form; tractable knowledge compilation target.
**Cost Model:** Linear-time model counting; some queries polynomial in d-DNNF size.
**Real Wall:** Compilation to d-DNNF often exponential; only worth it for repeated query workloads.
**Cross-Domain Aliases:** d_dnnf, dnnf, deterministic_decomposable
**Notes:** Darwiche 2001; underpins c2d compiler.

---

## Section 12. Substructural, Many-Valued & Higher-Order Logics

### LR.196: Intuitionistic Logic
**Definition:** Constructive logic rejecting law of excluded middle; proofs must constructively produce witnesses.
**Cost Model:** PSPACE-complete for propositional intuitionistic logic.
**Real Wall:** No classical reasoning shortcuts; proof terms via Curry-Howard.
**Cross-Domain Aliases:** intuitionistic, constructive_logic, heyting_logic
**Notes:** Brouwer, Heyting; foundation of type theory and proof assistants.

---

### LR.197: Linear Logic
**Definition:** Substructural logic where premises must be used exactly once; resource-aware reasoning.
**Cost Model:** Propositional linear logic decidable but high complexity; multiplicative-additive fragment PSPACE.
**Real Wall:** Full linear logic with exponentials undecidable; subexponential restrictions explored.
**Cross-Domain Aliases:** linear_logic, ll, girard_linear
**Notes:** Girard 1987; resource semantics for concurrency, separation logic.

---

### LR.198: Higher-Order Logic
**Definition:** Logic with quantification over functions and predicates, not just individuals.
**Cost Model:** Semi-decidable at best; many useful systems Π_1^1 in arithmetic hierarchy.
**Real Wall:** Higher-order unification undecidable (Goldfarb); pattern unification decidable subset.
**Cross-Domain Aliases:** hol, higher_order_logic, simple_type_theory
**Notes:** Church 1940; basis of HOL, Isabelle/HOL.

---

### LR.199: Second-Order Logic
**Definition:** Extension of FOL allowing quantification over predicates and relations.
**Cost Model:** Validity Π_1^1; not complete with respect to standard semantics.
**Real Wall:** Lacks completeness theorem under standard semantics; Henkin semantics restores it.
**Cross-Domain Aliases:** sol, second_order, monadic_sol
**Notes:** Frege; MSO over trees is decidable (Rabin); SO captures NP via Fagin.

---

### LR.200: Many-Valued Logic
**Definition:** Logic with more than two truth values; Łukasiewicz, Gödel, product logics common families.
**Cost Model:** Decidability varies by family; product logic over [0,1] decidable.
**Real Wall:** Choice of t-norm affects expressiveness; combining with quantifiers tricky.
**Cross-Domain Aliases:** many_valued_logic, mvl, fuzzy_logic
**Notes:** Łukasiewicz 1920; basis of fuzzy logic and substructural reasoning.

---
