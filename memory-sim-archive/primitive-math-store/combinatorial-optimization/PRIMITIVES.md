# Combinatorial Optimization Domain Primitives
> Cross-domain wiring: IP = integer programming + cutting planes; CP = constraint propagation + search;
> matroid = independence system + greedy; flow = network + augmenting path.

## 1. Integer Programming

### [PRIM-001] integer-programming
- **Atom/Composite:** Composite
- **Definition:** IP (Integer Programming): optimization over integer variables with linear constraints. NP-hard in general; branch-and-bound solves practically.
- **Cost Model:** Branch-and-bound: exponential worst-case; LP relaxation provides lower bound; cutting planes improve bound.
- **Real Wall:** Problem formulation (modeling) quality matters more than solver; different formulations have vastly different solving times.
- **Cross-Domain Aliases:** combinatorial-optimization (control-numerical-opt), mixed-integer-linear-programming (type-theory-programming-languages).
- **Notes:** Gomory (1958), Land & Doig (1960); modern MIP solvers (CPLEX, Gurobi, SCIP) handle millions of variables.

### [PRIM-002] linear-relaxation
- **Atom/Composite:** Primitive
- **Definition:** LP relaxation: remove integrality constraints from IP. Optimal LP value ≤ optimal IP value (lower bound for minimization).
- **Cost Model:** Simplex or interior point O(n·m) for n variables, m constraints; solves much faster than branch-and-bound.
- **Real Wall:** LP relaxation gap (IP-LP)/IP = integrality gap; large gap means branch-and-bound explores many nodes.
- **Cross-Domain Aliases:** convex-relaxation (control-numerical-opt), linear-bound (combinatorial-optimization).
- **Notes:** LP relaxation gap measures inherent difficulty of integrality; some problems have integrality gap = 1 (IP = LP).

### [PRIM-003] branch-and-bound
- **Atom/Composite:** Composite
- **Definition:** Branch-and-bound: recursively branch on fractional variable. B&B tree: each node = LP subproblem. Prune by bound or infeasibility.
- **Cost Model:** Worst-case exponential; effective pruning depends on bound quality; modern solvers process 10^6+ nodes/second.
- **Real Wall:** Node selection (best-first, depth-first, best-estimate) and variable selection (strong branching, pseudocost) affect performance.
- **Cross-Domain Aliases:** divide-and-conquer (control-numerical-opt), enumeration-tree (logic-reasoning).
- **Notes:** Land & Doig (1960); B&B is the backbone of modern MIP solvers; modern enhancements include preprocessing and parallelism.

### [PRIM-004] branch-and-cut
- **Atom/Composite:** Composite
- **Definition:** Branch-and-cut: B&B + cutting planes added at nodes. Cuts tighten LP relaxation without removing integer solutions.
- **Cost Model:** Cut generation adds inequalities; separation oracle finds most violated cut; Gomory, MIR, flow cover cuts.
- **Real Wall:** Cut selection: too many cuts = expensive LP solves; too few = weak bound. GMI cuts are most effective.
- **Cross-Domain Aliases:** cutting-plane-bb (control-numerical-opt), lp-strengthening (combinatorial-optimization).
- **Notes:** Padberg & Rinaldi (1987); modern MIP solvers are branch-cut-and-price (BCP) with column generation.

### [PRIM-005] cutting-planes
- **Atom/Composite:** Composite
- **Definition:** Cutting planes: valid inequalities that cut off fractional LP solutions. Gomory mixed integer (GMI), MIR, lift-and-project.
- **Cost Model:** Cut separation: O(n·m) for GMI; separation oracle must find violated inequality; heuristics often sufficient.
- **Real Wall:** Different problem classes have specialized cuts; TSP has subtour elimination; packing has cover inequalities.
- **Cross-Domain Aliases:** valid-inequality (control-numerical-opt), separation-oracle (combinatorial-optimization).
- **Notes:** Gomory (1958); cutting planes can theoretically solve any IP in polynomial time (by equivalence of optimization and separation).

### [PRIM-006] gomory-cuts
- **Atom/Composite:** Primitive
- **Definition:** Gomory mixed integer (GMI) cuts: from optimal LP tableau row with fractional basic variable. Very effective in practice.
- **Cost Model:** GMI from tableau row O(n); can generate multiple cuts per row; Chvátal-Gomory rank > 1 cuts possible.
- **Real Wall:** GMI cuts are universal (valid for all IPs); problem-specific cuts (clique, cover) can be stronger.
- **Cross-Domain Aliases:** mixed-integer-cut (combinatorial-optimization), tableau-cut (control-numerical-opt).
- **Notes:** Gomory (1958), Chvátal (1973); GMI cuts are the workhorse of modern MIP solvers.

### [PRIM-007] lift-and-project
- **Atom/Composite:** Composite
- **Definition:** Lift-and-project (Balas-Perduc): iteratively lift 0-1 variables to higher-dimensional spaces. Add binary constraint x_i + x_j ≥ 1 in projection.
- **Cost Model:** O(2^n) in worst case; sparse version (SOS1) more practical; used for 0-1 IPs.
- **Real Wall:** Lift-and-project is effective for small 0-1 IPs; too expensive for large IPs without preprocessing.
- **Cross-Domain Aliases:** disjunctive-programming (control-numerical-opt), binary-lifting (combinatorial-optimization).
- **Notes:** Balas et al. (1993); lift-and-project generates rank-1 cuts from disjunction of two half-spaces.

### [PRIM-008] disjunctive-programming
- **Atom/Composite:** Composite
- **Definition:** Disjunctive programming: represent feasible region as union of polyhedra. Convex hull = convex hull of union of polyhedra.
- **Cost Model:** Convexification cost is exponential in number of disjuncts; each disjunct = one case in case analysis.
- **Real Wall:** Disjunctive cuts generalize lift-and-project; extended formulation may be smaller than convex hull projection.
- **Cross-Domain Aliases:** union-of-polyhedra (control-numerical-opt), case-analysis (logic-reasoning).
- **Notes:** Balas (1979); disjunctive programming is the theoretical foundation of lift-and-project and B&B.

### [PRIM-009] preprocessing-ip
- **Atom/Composite:** Composite
- **Definition:** IP preprocessing: presolve (bounds tightening, variable elimination, probing). Reduce problem size before branching.
- **Cost Model:** Presolve O(n·m) typically; dramatically reduces problem size for many instances; beneficial in almost all cases.
- **Real Wall:** Aggressive presolve can identify infeasibility early; may change problem structure (SOS, special ordered sets).
- **Cross-Domain Aliases:** model-reduction (control-numerical-opt), presolve (combinatorial-optimization).
- **Notes:** Savelsbergh (1994); modern MIP solvers have extensive presolve; even 10x reduction in variables is common.

### [PRIM-010] strong-branching
- **Atom/Composite:** Composite
- **Definition:** Strong branching: try both branches at fractional variable, pick branch with better bound. Most accurate but expensive.
- **Cost Model:** O(n·LP) for n fractional variables; limits exploration at each node; pseudocost branching approximates cheaply.
- **Real Wall:** Strong branching is most effective near root; too expensive deep in tree; hybrid strategies adapt.
- **Cross-Domain Aliases:** variable-selection (control-numerical-opt), branching-quality (combinatorial-optimization).
- **Notes:** Appleget et al. (2000); strong branching is the gold standard for variable selection; modern solvers use it near root.

### [PRIM-011] pseudocost-branching
- **Atom/Composite:** Primitive
- **Definition:** Pseudocost branching: track historical objective change per unit of fractional variable fixed. Estimate branch quality cheaply.
- **Cost Model:** O(1) to compute pseudocost estimate; accumulates from previous branch decisions; need sufficient exploration first.
- **Real Wall:** Pseudocost branching is fast but unreliable early in search; hybrid with strong branching near root.
- **Cross-Domain Aliases:** history-branching (control-numerical-opt), estimated-quality (combinatorial-optimization).
- **Notes:** Benichou et al. (1971); most practical branching heuristic; combined with reliability (require min observations).

### [PRIM-012] conflict-analysis
- **Atom/Composite:** Composite
- **Definition:** Conflict analysis: analyze infeasible LP node to learn conflict clause. No-good learning for MIP.
- **Cost Model:** Conflict extraction from bound tightening and constraint propagation O(n·m); generates clause for clause database.
- **Real Wall:** Conflict analysis dramatically reduces B&B tree; works especially well for structured constraints.
- **Cross-Domain Aliases:** nogood-learning (combinatorial-optimization), infeasibility-analysis (control-numerical-opt).
- **Notes:** Achterberg et al. (2005); conflict analysis adapted from SAT/CDCL to MIP; key for structured problems.

### [PRIM-013] mip-start
- **Atom/Composite:** Primitive
- **Definition:** MIP start: provide initial feasible solution to warm-start branch-and-bound. Reduces optimality gap immediately.
- **Cost Model:** MIP start evaluation O(n) to check feasibility; if feasible, set incumbent and prune nodes immediately.
- **Real Wall:** Even suboptimal MIP start is valuable; heuristic solutions (greedy, local search) commonly used.
- **Cross-Domain Aliases:** initial-solution (control-numerical-opt), warm-start (combinatorial-optimization).
- **Notes:** Bixby et al. (1992); MIP start can cut solve time by orders of magnitude; heuristics generate initial solutions.

### [PRIM-014] callback-mechanism
- **Atom/Composite:** Composite
- **Definition:** MIP solver callbacks: user-defined hooks for branching decisions, node selection, solution found. Enables problem-specific heuristics.
- **Cost Model:** Callback overhead: each callback invocation adds O(1) to O(n) cost; balance between customization and overhead.
- **Real Wall:** Custom branching can dramatically improve solve time for structured problems; callback-based heuristics (VRP, scheduling) are common.
- **Cross-Domain Aliases:** solver-hook (combinatorial-optimization), user-extension (type-theory-programming-languages).
- **Notes:** Gurobi, CPLEX, SCIP support callbacks; enables integration of problem-specific knowledge.

### [PRIM-015] decomposition-mip
- **Atom/Composite:** Composite
- **Definition:** Dantzig-Wolfe decomposition: reformulate MIP as master problem + pricing subproblems. Column generation.
- **Cost Model:** Master LP: O(n) per iteration; subproblem: O(n) per variable; converge slowly near optimum.
- **Real Wall:** DW decomposition works when structure allows: set covering, vehicle routing, crew scheduling.
- **Cross-Domain Aliases:** column-generation (control-numerical-opt), master-subproblem (combinatorial-optimization).
- **Notes:** Dantzig & Wolfe (1960); DW + branch-and-bound = branch-and-price; used in airline crew scheduling.

### [PRIM-016] benders-decomposition
- **Atom/Composite:** Composite
- **Definition:** Benders decomposition: separate complicating variables (master) from rest (subproblem). Add optimality and feasibility cuts.
- **Cost Model:** Master: LP with generated cuts; subproblem: LP or IP per iteration; converges slowly near optimum.
- **Real Wall:** Effective when subproblem is easy (LP) and complicating variables are few; stochastic programming uses Benders.
- **Cross-Domain Aliases:** lagrangian-decomposition (control-numerical-opt), cut-generation (combinatorial-optimization).
- **Notes:** Benders (1962); modern Benders uses modern cut selection; GBD = Generalized Benders for MIP subproblems.

### [PRIM-017] lagrangian-relaxation
- **Atom/Composite:** Composite
- **Definition:** Lagrangian relaxation: relax complicating constraints into objective with Lagrange multipliers. Subgradient optimization for multipliers.
- **Cost Model:** Subgradient method O(1/ε²) convergence; Lagrangian dual = convex envelope of IP; dual bound.
- **Real Wall:** Lagrangian dual provides tighter bound than LP relaxation for many problems; dual pricing identifies violated inequalities.
- **Cross-Domain Aliases:** dual-relaxation (control-numerical-opt), lagrangian-bound (combinatorial-optimization).
- **Notes:** Held & Karp (1971); Lagrangian relaxation is the basis of many approximation algorithms.

### [PRIM-018] knapsack-problem
- **Atom/Composite:** Composite
- **Definition:** 0-1 Knapsack: maximize value with weight constraint. Classic NP-hard problem; pseudo-polynomial DP, FPTAS.
- **Cost Model:** DP O(n·W) for n items, capacity W; FPTAS: O(n·(1/ε)) for (1-ε) approximation.
- **Real Wall:** Branch-and-bound with greedy bound solves practical knapsack instances quickly; core concept for many other problems.
- **Cross-Domain Aliases:** resource-allocation (combinatorial-optimization), bounded-knapsack (control-numerical-opt).
- **Notes:** Knuth (1978); knapsack is the simplest NP-hard problem; every NP has a pseudopolynomial DP like knapsack.

### [PRIM-019] traveling-salesman-problem
- **Atom/Composite:** Composite
- **Definition:** TSP (Traveling Salesman Problem): find shortest Hamiltonian cycle visiting all cities. NP-hard; metric TSP has 3/2-approximation.
- **Cost Model:** Exact: branch-and-cut (Concorde) solves instances with 10^4+ cities; approximation: Christofides algorithm O(n³).
- **Real Wall:** Metric TSP (triangle inequality) is the practically relevant variant; asymmetric TSP (ATSP) is harder.
- **Cross-Domain Aliases:** hamiltonian-cycle (combinatorial-optimization), metric-tsp (control-numerical-opt).
- **Notes:** Applegate et al. (2006); TSP is the most studied combinatorial optimization problem; Concorde is the world-record solver.

### [PRIM-020] subtour-elimination
- **Atom/Composite:** Composite
- **Definition:** Subtour elimination constraints: for TSP, prevent tours that don't visit all cities. Valid inequalities for TSP polytope.
- **Cost Model:** Separation oracle: find minimum capacity subtour (min-cut) in graph; O(E log V) via min-cut.
- **Real Wall:** Subtour elimination cuts are the main cutting planes for TSP; combined with branch-and-cut.
- **Cross-Domain Aliases:** cycle-constraint (combinatorial-optimization), separation-oracle (control-numerical-opt).
- **Notes:** Dantzig et al. (1954); subtour elimination + LP relaxation + cutting planes = Concorde algorithm.

### [PRIM-021] branch-and-price
- **Atom/Composite:** Composite
- **Definition:** Branch-and-price: branch-and-bound + column generation. Master problem generates columns, subproblem prices out columns.
- **Cost Model:** Column generation per node: O(n) per iteration; convergence slow near optimum; strong branching on pricing variables.
- **Real Wall:** B&P is state-of-the-art for set covering, VRP, bin packing; pricing subproblem determines tractability.
- **Cross-Domain Aliases:** column-generation-bb (control-numerical-opt), master-pricing (combinatorial-optimization).
- **Notes:** Barnhart et al. (1998); B&P is the most powerful method for large-scale vehicle routing and crew scheduling.

### [PRIM-022] matroid-intersection
- **Atom/Composite:** Composite
- **Definition:** Matroid intersection: find maximum cardinality set in intersection of two matroids. Greedy + exchange argument.
- **Cost Model:** O(r·n·EO) for matroid rank r, n elements, EO = matroid oracle query. Polynomial for any two matroids.
- **Real Wall:** Matroid intersection generalizes bipartite matching, spanning trees, and independent set problems.
- **Cross-Domain Aliases:** independence-system (combinatorial-optimization), matroid-oracle (control-numerical-opt).
- **Notes:** Edmonds (1979); matroids are the most general class where greedy algorithm works for optimization.

### [PRIM-023] matroid-intersection-greedy
- **Atom/Composite:** Composite
- **Definition:** Matroid intersection greedy: start with empty set, greedily add elements respecting both matroids. Exchange argument ensures optimality.
- **Cost Model:** Greedy phase O(n·EO) with augmentation; each augmentation increases cardinality; O(n²) total.
- **Real Wall:** Matroid intersection greedy is optimal for two matroids; requires careful exchange operations.
- **Cross-Domain Aliases:** greedy-exchange (control-numerical-opt), independence-greedy (combinatorial-optimization).
- **Notes:** Edmonds (1979); greedy works for each matroid alone; matroid intersection requires exchange between the two.

### [PRIM-024] polymatroid-optimization
- **Atom/Composite:** Composite
- **Definition:** Polymatroid: submodular function constraints. Polymatroid optimization via greedy on sorted elements.
- **Cost Model:** Polymatroid maximization (cardinal) via greedy O(n log n); submodular minimization via Frank-Wolfe O(n²·ε^{-2}).
- **Real Wall:** Polymatroid constraints model knapsack-like resource allocation; submodularity enables approximation guarantees.
- **Cross-Domain Aliases:** submodular-optimization (combinatorial-optimization), polymatroid-constraint (control-numerical-opt).
- **Notes:** Edmonds (1970); polymatroid = extension of matroid to submodular functions.

### [PRIM-025] submodular-optimization
- **Atom/Composite:** Composite
- **Definition:** Submodular optimization: maximize/minimize submodular functions (diminishing returns). NP-hard for maximization; greedy 1-1/e approximation.
- **Cost Model:** Greedy maximization: O(n·f) for submodular function f; lazier-than-lazy greedy improves to O(n log n).
- **Real Wall:** Submodular maximization has strong approximation guarantees; covers sensor placement, viral marketing, document summarization.
- **Cross-Domain Aliases:** diminishing-returns-optimization (control-numerical-opt), submodular-function (combinatorial-optimization).
- **Notes:** Nemhauser et al. (1978); submodular functions model economies of scale; greedy gives 1-1/e for monotone.

### [PRIM-026] submodular-minimization
- **Atom/Composite:** Composite
- **Definition:** Submodular minimization: find set minimizing submodular function. Polynomial time via Fujishige's algorithm O(n⁶·EO).
- **Cost Model:** Fujishige-Wolfe: O(n⁶·EO) in practice; faster combinatorial algorithms O(n³·EO) for specific cases.
- **Real Wall:** Submodular minimization is polynomial; used for graph cuts, equilibrium in games, and convex optimization.
- **Cross-Domain Aliases:** submodular-function-min (control-numerical-opt), convex-submodular (combinatorial-optimization).
- **Notes:** Fujishige (2005); submodular minimization is the continuous counterpart of submodular maximization.

### [PRIM-027] set-cover-approximation
- **Atom/Composite:** Composite
- **Definition:** Set cover: cover all elements with minimum number of sets. Greedy gives ln(n)-approximation; hardness of ln(n).
- **Cost Model:** Greedy: O(n·m) for n elements, m sets; select set covering most uncovered elements each step.
- **Real Wall:** Greedy achieves ln(n) which is optimal under hardness; unweighted set cover has PTAS on planar graphs.
- **Cross-Domain Aliases:** covering-problem (combinatorial-optimization), greedy-approx (control-numerical-opt).
- **Notes:** Feige (1998) proved hardness of ln(n) for set cover; greedy is near-optimal in practice.

### [PRIM-028] set-packing
- **Atom/Composite:** Primitive
- **Definition:** Set packing: find maximum number of pairwise disjoint sets. Generalizes matching; NP-hard; exact via IP.
- **Cost Model:** Set packing IP: exponential constraints; can be solved via branch-and-price with column generation.
- **Real Wall:** Set packing generalizes 3D matching (NP-hard); maximum independent set in hypergraphs.
- **Cross-Domain Aliases:** independent-system (combinatorial-optimization), packing-constraint (control-numerical-opt).
- **Notes:** Karp (1972); set packing is the covering dual; primal-dual algorithms apply to both.

### [PRIM-029] vertex-cover
- **Atom/Composite:** Primitive
- **Definition:** Vertex cover: find minimum vertex set covering all edges. 2-approximation via maximal matching; NP-hard.
- **Cost Model:** 2-approximation: O(E) for maximal matching + 2× cover; SDP relaxation gives 2-approx; integrality gap = 2.
- **Real Wall:** Vertex cover is FPT (2-approx via kernel of size 2k); parameterized by k = solution size.
- **Cross-Domain Aliases:** edge-covering (combinatorial-optimization), minimum-vertex-set (control-numerical-opt).
- **Notes:** Karp (1972); vertex cover is the simplest FPT problem; kernelization to 2k is classic.

### [PRIM-030] maximum-cut
- **Atom/Composite:** Composite
- **Definition:** Max-cut: partition vertices to maximize edges crossing partition. 0.5-approx via random partition; Goemans-Williamson gives 0.878-approx.
- **Cost Model:** GW SDP: O(n^{3.5}) for SDP; random hyperplane rounding gives 0.878-approx; hardness of 0.5+ε.
- **Real Wall:** Max-cut is NP-hard; SDP relaxation is tight (gap = 0 for planar graphs).
- **Cross-Domain Aliases:** graph-partition (combinatorial-optimization), sdp-relaxation (control-numerical-opt).
- **Notes:** Goemans & Williamson (1994); SDP relaxation is the key to the 0.878-approx; unique games conjecture relates to optimality.

### [PRIM-031] steiner-tree
- **Atom/Composite:** Composite
- **Definition:** Steiner tree: connect terminal set with minimum total edge weight. NP-hard; 2-approx via MST; 1.39-approx via Dreyfus-Wagner.
- **Cost Model:** MST heuristic O(E log V); Dreyfus-Wagner DP O(3^T · 2^T) for T terminals; PTAS via Arora.
- **Real Wall:** Steiner tree is APX-hard; in planar graphs, PTAS exists; metric Steiner tree has 1.55-approx.
- **Cross-Domain Aliases:** minimum-spanning-tree (combinatorial-optimization), network-design (control-numerical-opt).
- **Notes:** Dreyfus & Wagner (1971); Steiner tree is fundamental in network design, VLSI routing.

### [PRIM-032] facility-location
- **Atom/Composite:** Composite
- **Definition:** Facility location: open facilities to serve clients at minimum total cost (opening + assignment). Uncapacitated: 1.488-approx via LP.
- **Cost Model:** LP relaxation: O(n²·m) for n facilities, m clients; primal-dual 1.61-approx; LP rounding 1.488-approx.
- **Real Wall:** Metric facility location has 1.488-approx; hardness of 1.463; non-metric is NP-hard.
- **Cross-Domain Aliases:** uncapacitated-facility (combinatorial-optimization), location-optimization (control-numerical-opt).
- **Notes:** Jain et al. (2003); facility location is a classic approximation algorithm problem; LP rounding + primal-dual = main techniques.

### [PRIM-033] k-median
- **Atom/Composite:** Composite
- **Definition:** K-median: select k facilities to minimize total distance to clients. LP relaxation gives k-approx; local search gives 3+ε.
- **Cost Model:** LP-based: O(n²·m·k) to solve; local search: O(n^k) iterations but practical with good initialization.
- **Real Wall:** K-median is NP-hard; LP relaxation gap can be large; primal-dual + clustering gives practical results.
- **Cross-Domain Aliases:** k-center (combinatorial-optimization), facility-k-selection (control-numerical-opt).
- **Notes:** Arya et al. (2004); k-median vs k-means: k-median uses median (L1) distance; k-means uses centroid (L2).

### [PRIM-034] vehicle-routing
- **Atom/Composite:** Composite
- **Definition:** VRP (Vehicle Routing Problem): route vehicles to serve customers. CVRP (capacitated) has 1-1.5% of OPT known for large instances.
- **Cost Model:** Clarke-Wright savings heuristic O(n²); savings + neighborhood search gets within 2-3% of OPT for CVRP.
- **Real Wall:** VRP variants: VRPTW (time windows), VRPPD (pickup-delivery), IRP (inventory routing). All NP-hard.
- **Cross-Domain Aliases:** routing-optimization (combinatorial-optimization), tsp-generalization (control-numerical-opt).
- **Notes:** Dantzig & Ramser (1959); VRP is the practical generalization of TSP; heuristics + IP dominate.

### [PRIM-035] scheduling-theory
- **Atom/Composite:** Composite
- **Definition:** Scheduling: assign jobs to machines over time to optimize objective. Makespan (C_max), tardiness, flow time.
- **Cost Model:** P||C_max (identical parallel machines): LPT gives 4/3-approx; FPTAS for fixed m machines. P|prec|C_max is NP-hard.
- **Real Wall:** Scheduling is one of the most applied areas of combinatorial optimization; many tractable special cases.
- **Cross-Domain Aliases:** job-shop-scheduling (combinatorial-optimization), machine-assignment (control-numerical-opt).
- **Notes:** Graham et al. (1979); three-field notation α|β|γ: α=machine environment, β=job characteristics, γ=objective.

### [PRIM-036] makespan-minimization
- **Atom/Composite:** Composite
- **Definition:** Makespan (C_max): minimize completion time of last job. P||C_max has PTAS; FPTAS for fixed m machines.
- **Cost Model:** LPT (Longest Processing Time first): 4/3-1/(3m) approx; list scheduling: 2-1/m approx.
- **Real Wall:** Scheduling with precedence constraints (P|prec|C_max) is NP-hard; preemptive version (P|pmtn|C_max) is polynomial.
- **Cross-Domain Aliases:** makespan-scheduling (control-numerical-opt), parallel-machines (combinatorial-optimization).
- **Notes:** Graham (1966); makespan is the most studied single-machine scheduling objective.

### [PRIM-037] tardiness-scheduling
- **Atom/Composite:** Composite
- **Definition:** Tardiness scheduling: jobs have due dates; minimize total tardiness Σ max(0, C_j - d_j). 1||ΣT_j is NP-hard.
- **Cost Model:** EDD (Earliest Due Date) minimizes maximum lateness L_max; ΣT_j is more complex (requires DP for small instances).
- **Real Wall:** Tardiness scheduling is common in manufacturing; dispatch rules (SPT, EDD, ATC) are practical heuristics.
- **Cross-Domain Aliases:** due-date-scheduling (control-numerical-opt), weighted-tardiness (combinatorial-optimization).
- **Notes:** Lawler (1977); tardiness is more sensitive than lateness; SPT minimizes total completion time.

### [PRIM-038] job-shop-scheduling
- **Atom/Composite:** Composite
- **Definition:** Job shop: m machines, each job has machine-ordered operations. Disjunctive graph representation.
- **Cost Model:** Job shop with m machines: FPTAS for fixed m; J||C_max is NP-hard for m≥2; branch-and-bound via critical path.
- **Real Wall:** Job shop is the classic scheduling problem; disjunctive graph + local search is the practical approach.
- **Cross-Domain Aliases:** open-shop (combinatorial-optimization), flow-shop (control-numerical-opt).
- **Notes:** Giffler & Thompson (1960); job shop scheduling is one of the most difficult combinatorial optimization problems.

### [PRIM-039] flow-shop-scheduling
- **Atom/Composite:** Composite
- **Definition:** Flow shop: all jobs processed on machines in same order. F||C_max (machines, makespan) has polynomial solution for m=2.
- **Cost Model:** For m=2: Johnson's rule O(n log n); for m≥3: F||C_max is NP-hard; NEH heuristic is near-optimal.
- **Real Wall:** Flow shop is a special case of job shop with regular objective; permutation schedules are optimal for makespan.
- **Cross-Domain Aliases:** permutation-schedule (control-numerical-opt), makespan-flow (combinatorial-optimization).
- **Notes:** Johnson (1954); NEH heuristic (Nawaz, Enscore, Ham) gives 1+ approximation for flow shop.

### [PRIM-040] bin-packing
- **Atom/Composite:** Composite
- **Definition:** Bin packing: pack items of sizes (0,1] into minimum number of unit bins. FF, FFD, NF for online; APTAS for offline.
- **Cost Model:** FFD (First Fit Decreasing): 11/9·OPT + 6/9; APTAS: (1+ε)·OPT + O(1/ε²); NF (Next Fit): 2-1/m.
- **Real Wall:** Bin packing is strongly NP-hard; real-world packing uses shelf algorithms, branch-and-price, constraint programming.
- **Cross-Domain Aliases:** container-loading (combinatorial-optimization), packing-approximation (control-numerical-opt).
- **Notes:** Karmarkar & Karp (1982) APTAS; bin packing is the dual of vertex cover (in some sense); approximation schemes are the best possible.

### [PRIM-041] cutting-stock
- **Atom/Composite:** Composite
- **Definition:** Cutting stock: cut items from standard rolls to meet demand. Integer programming with column generation.
- **Cost Model:** Column generation: generate cutting patterns as columns; cutting patterns are bounded knapsack subproblems.
- **Real Wall:** Cutting stock is the practical version of bin packing; Gilmore-Gomory column generation is classic.
- **Cross-Domain Aliases:** 1d-cutting (combinatorial-optimization), pattern-generation (control-numerical-opt).
- **Notes:** Gilmore & Gomory (1961); cutting stock is the motivating application for column generation.

### [PRIM-042] multi-dimensional-knapsack
- **Atom/Composite:** Composite
- **Definition:** Multidimensional knapsack (MKP): multiple constraints. CPLEX handles instances with 100 constraints, 1000 variables.
- **Cost Model:** LP relaxation gap can be large; core identification + local search improves branch-and-bound.
- **Real Wall:** MKP is harder than 0-1 knapsack; surrogate constraint relaxation is effective for small gap.
- **Cross-Domain Aliases:** multi-constraint-knapsack (combinatorial-optimization), resource-allocation (control-numerical-opt).
- **Notes:** Freville & Hanafi (2005); MKP arises in capital budgeting, project selection, processor allocation.

### [PRIM-043] prize-collecting-tsp
- **Atom/Composite:** Composite
- **Definition:** Prize-collecting TSP (PCTSP): collect prizes from visited cities, pay travel cost. Minimize travel cost + penalty for uncollected prize.
- **Cost Model:** Nearest neighbor + penalty adjustment; primal-dual gives 2-approx; IP with branch-and-cut.
- **Real Wall:** PCTSP is practical variant of TSP; similar to orienteering (maximize prize with length constraint).
- **Cross-Domain Aliases:** orienteering (combinatorial-optimization), prize-tsp (control-numerical-opt).
- **Notes:** Balas (1989); PCTSP has two equivalent formulations: fixed budget or fixed prize collection.

### [PRIM-044] orienteering-problem
- **Atom/Composite:** Composite
- **Definition:** Orienteering: maximize prize collected within budget/time limit. Max-prize with length constraint. Approximation: 2+ε.
- **Cost Model:** Greedy + local search; IP with time-indexed formulation; dynamic programming for tree metrics.
- **Real Wall:** Orienteering is the root of many outdoor sports; used in routing with profits.
- **Cross-Domain Aliases:** prize-collecting-routing (combinatorial-optimization), max-prize-path (control-numerical-opt).
- **Notes:** Golden et al. (1987); orienteering is NP-hard; PTAS on planar graphs via Arora.

### [PRIM-045] constraint-programming
- **Atom/Composite:** Composite
- **Definition:** CP (Constraint Programming): declarative programming over finite domains. Variables with domains, constraints, search.
- **Cost Model:** Arc consistency (AC-3) O(ed³); constraint propagation is exponential in worst case but very effective in practice.
- **Real Wall:** CP excels at combinatorial problems with complex constraints (scheduling, configuration); not suited to linear optimization.
- **Cross-Domain Aliases:** combinatorial-search (logic-reasoning), domain-reasoning (combinatorial-optimization).
- **Notes:** Montanari (1974), Mackworth (1977); CP is the practical alternative to IP for many scheduling and configuration problems.

### [PRIM-046] arc-consistency
- **Atom/Composite:** Composite
- **Definition:** Arc consistency (AC-3): for each binary constraint (X,Y), remove values from domain of X inconsistent with Y.
- **Cost Model:** AC-3 O(ed³) for e constraints, d domain size; AC-4 O(ed²) worst-case; AC-3 is simple and practical.
- **Real Wall:** AC-3 alone is very effective; stronger consistencies (path consistency, i-consistency) are rarely worth the cost.
- **Cross-Domain Aliases:** constraint-propagation (combinatorial-optimization), domain-filtering (logic-reasoning).
- **Notes:** Mackworth (1977); arc consistency is the most important propagation algorithm; 1B-constraint propagation.

### [PRIM-047] global-constraints
- **Atom/Composite:** Composite
- **Definition:** Global constraints: alldifferent, cumulative, circuit, bin-packing. Capture common combinatorial substructures efficiently.
- **Cost Model:** alldifferent: bipartite matching O(n·d^2.5) for n variables, d domain; specialized algorithms much faster than decomposition.
- **Real Wall:** Global constraints are the main advantage of CP over SAT/IP; enable strong propagation without exponential blowup.
- **Cross-Domain Aliases:** high-level-constraint (combinatorial-optimization), combinatorial-constraint (logic-reasoning).
- **Notes:** Régin (1994); alldifferent constraint uses bipartite matching; other global constraints have specialized filtering algorithms.

### [PRIM-048] alldifferent-constraint
- **Atom/Composite:** Primitive
- **Definition:** alldifferent constraint: all variables must take distinct values. Filtered by maximum matching on bipartite graph.
- **Cost Model:** Bipartite matching O(n·d^2.5) per propagation; incrementally updated O(n·d) per assignment.
- **Real Wall:** alldifferent is surprisingly powerful; detects Hall violators (overconstrained subsets) and prunes domains.
- **Cross-Domain Aliases:** distinctness-constraint (combinatorial-optimization), assignment-constraint (logic-reasoning).
- **Notes:** Régin (1994); alldifferent = permutation constraint; essential for scheduling, assignment, and rostering.

### [PRIM-049] cumulative-constraint
- **Atom/Composite:** Primitive
- **Definition:** cumulative constraint: limit resource usage over time. Each task has start time, duration, resource consumption; sum ≤ capacity.
- **Cost Model:** Time-table filtering O(n·T) for time-based; edge-finding O(n log n) for task-based; disjunctive scheduling O(n²).
- **Real Wall:** cumulative is the core of scheduling CP; edge-finding and overload checking are key filtering algorithms.
- **Cross-Domain Aliases:** resource-constraint (combinatorial-optimization), scheduling-constraint (control-numerical-opt).
- **Notes:** Aggoun & Beldiceanu (1993); cumulative enables resource-constrained project scheduling (RCPSP).

### [PRIM-050] search-heuristics-cp
- **Atom/Composite:** Composite
- **Definition:** CP search heuristics: variable ordering (first-fail, Dom/wdeg), value ordering (random, bound, impact). Dominance detection.
- **Cost Model:** Variable selection O(n) per decision; value selection O(d) per variable; restart strategy O(1) per restart.
- **Real Wall:** First-fail (smallest domain first) is robust heuristic; Dom/wdeg adapts to constraint structure via failure counting.
- **Cross-Domain Aliases:** branching-heuristic (control-numerical-opt), search-guidance (combinatorial-optimization).
- **Notes:** Harvey & Ginsberg (1995); limited discrepancy search (LDS) explores paths with few discrepancies first.

### [PRIM-051] min-cut
- **Atom/Composite:** Composite
- **Definition:** Min-cut: find minimum capacity edge set separating source s from sink t. Max-flow min-cut theorem: max flow = min cut capacity.
- **Cost Model:** Ford-Fulkerson O(E·|f|) worst-case; Edmonds-Karp O(V·E²); Dinic O(V²·E); push-relabel O(V³).
- **Real Wall:** Push-relabel and Dinic are fastest in practice; capacity scaling improves performance on dense graphs.
- **Cross-Domain Aliases:** network-separation (distributed-systems), s-t-cut (combinatorial-optimization).
- **Notes:** Ford & Fulkerson (1956); min-cut is fundamental to image segmentation, clustering, and bipartite matching.

### [PRIM-052] max-flow
- **Atom/Composite:** Composite
- **Definition:** Max flow: push maximum flow from source to sink in directed network. Residual graph + augmenting path.
- **Cost Model:** Push-relabel O(V³) worst-case, O(V²·√E) for unit capacities; Dinic O(min(V^{2/3}, √E)·E).
- **Real Wall:** Boykov-Kolmogorov algorithm uses BFS/DFS alternating and is very fast for computer vision graphs.
- **Cross-Domain Aliases:** flow-augmentation (distributed-systems), network-flow (combinatorial-optimization).
- **Notes:** Edmonds-Karp (1972); max-flow is polynomial; push-relabel is the most widely used algorithm.

### [PRIM-053] min-cost-flow
- **Atom/Composite:** Composite
- **Definition:** Min-cost flow: find flow of given value with minimum total cost. Successive shortest path + capacity scaling.
- **Cost Model:** Successive shortest path O(F·E·log V) for integer flows F; cost scaling O(E·log C·E·log V).
- **Real Wall:** Min-cost flow is the generalization of shortest path, assignment, and transportation problems.
- **Cross-Domain Aliases:** transportation-problem (combinatorial-optimization), optimal-flow (distributed-systems).
- **Notes:** Edmonds-Karp (1972) for uncapacitated; Orlin (1997) for O(ES log V) capacitated.

### [PRIM-054] maximum-matching-bipartite
- **Atom/Composite:** Composite
- **Definition:** Maximum bipartite matching: largest set of edges with no shared vertices. Kuhn-Munkres (Hungarian) for weighted.
- **Cost Model:** Hopcroft-Karp O(√V·E) for unweighted; Hungarian O(V³) for weighted; network flow reduction.
- **Real Wall:** Bipartite matching is polynomial; key subroutine for assignment problems, transportation, scheduling.
- **Cross-Domain Aliases:** hungarian-algorithm (control-numerical-opt), assignment-matching (combinatorial-optimization).
- **Notes:** Kuhn (1955), Munkres (1957); Hungarian algorithm = Kuhn-Munkres for maximum.

### [PRIM-055] hungarian-algorithm
- **Atom/Composite:** Composite
- **Definition:** Hungarian (Kuhn-Munkres) algorithm: O(n³) for assignment problem. Dual variables u_i, v_j; complementary slackness.
- **Cost Model:** O(n³) worst-case; implementations O(n²) on sparse instances; competitive with min-cost flow.
- **Real Wall:** Hungarian is the workhorse for assignment problems; used in track assignment, scheduling, object tracking.
- **Cross-Domain Aliases:** assignment-optimal (combinatorial-optimization), dual-slackness (control-numerical-opt).
- **Notes:** Kuhn (1955); Hungarian algorithm is dual-based; computes optimal dual variables alongside primal matching.

### [PRIM-056] general-matching
- **Atom/Composite:** Composite
- **Definition:** General (non-bipartite) matching: Edmonds' blossom algorithm O(V³). Find maximum cardinality matching in general graph.
- **Cost Model:** Blossom algorithm O(V³) worst-case; faster implementations O(V²·E) practical; Micali-Vazirani O(√V·E).
- **Real Wall:** Blossom algorithm is surprisingly complex; Edmonds' original is O(V⁴); Micali-Vazirani is the asymptotically fastest.
- **Cross-Domain Aliases:** non-bipartite-matching (combinatorial-optimization), blossom-shrinking (control-numerical-opt).
- **Notes:** Edmonds (1965); blossom algorithm is one of the most celebrated combinatorial algorithms.

### [PRIM-057] online-algorithms
- **Atom/Composite:** Composite
- **Definition:** Online algorithms: decisions made without knowledge of future input. Competitive analysis: compare to offline optimal.
- **Cost Model:** Competitive ratio = sup over inputs of online_cost/OPT; lower bound via adversary argument.
- **Real Wall:** Online algorithms are practically relevant for caching, scheduling, routing; randomized algorithms often better.
- **Cross-Domain Aliases:** ski-rental (combinatorial-optimization), paging (networking).
- **Notes:** Sleator & Tarjan (1985); online algorithms formalize the worst-case competitive analysis framework.

### [PRIM-058] paging-algorithm
- **Atom/Composite:** Composite
- **Definition:** Paging (cache replacement): FIFO, LRU, LFU, CLOCK. LRU is O(1) with linked list; competitive ratio = k for LRU (k = cache size).
- **Cost Model:** LRU: O(1) per access with doubly linked list; Belady (optimal): O(n) per access, offline.
- **Real Wall:** LRU is good in practice but not optimal; LFD (Longest Forward Distance) is optimal offline; LRU-2 and LRFU are practical improvements.
- **Cross-Domain Aliases:** cache-replacement (networking), memory-management (combinatorial-optimization).
- **Notes:** Belady (1966); paging is the classic online problem; LRU competitive ratio = k on uniform request sequences.

### [PRIM-059] ski-rental-problem
- **Atom/Composite:** Composite
- **Definition:** Ski rental: rent at $1/day, buy at $N. Optimal strategy: rent for N-1 days, then buy. 2-competitive.
- **Cost Model:** Randomized: rent for geometrically distributed time, then buy; 1.58-competitive (e/(e-1) ≈ 1.58).
- **Real Wall:** Ski rental is the simplest online problem with randomization advantage; many similar problems (car rental, TCP slow start).
- **Cross-Domain Aliases:** online-buy-rent (combinatorial-optimization), lease-purchase (control-numerical-opt).
- **Notes:** Karlin et al. (1988); ski rental has deterministic lower bound of 2 and randomized upper bound of e/(e-1).

### [PRIM-060] metrical-task-systems
- **Atom/Composite:** Composite
- **Definition:** MTS (Metrical Task System): online optimization over metric space with service costs. Competitive ratio = O(log D) for D=diameter.
- **Cost Model:** Deterministic: Double Cover algorithm O(log D)-competitive; randomized: 2O(log log D)-competitive (BK algorithm).
- **Real Wall:** MTS generalizes paging, k-server, and list update; k-server conjecture: competitive ratio = k for deterministic.
- **Cross-Domain Aliases:** online-metric (combinatorial-optimization), task-scheduling (control-numerical-opt).
- **Notes:** Borodin et al. (1992); MTS is the unifying framework for online problems on metric spaces.

### [PRIM-061] k-server-problem
- **Atom/Composite:** Composite
- **Definition:** K-server problem: k mobile servers on metric space; serve requests at minimum total distance. Work function algorithm (WFA).
- **Cost Model:** WFA is k-competitive for any metric; deterministic lower bound = k; no better deterministic algorithm known.
- **Real Wall:** K-server is the generalization of paging (k=1 is paging); RANDOMIZED is O(log k log D)-competitive.
- **Cross-Domain Aliases:** mobile-server (networking), multi-server-online (combinatorial-optimization).
- **Notes:** Manasse et al. (1990); k-server conjecture: competitive ratio = k for all metrics, unresolved.

### [PRIM-062] list-update
- **Atom/Composite:** Composite
- **Definition:** List update: online list of items; access item at cost = position. Transpose and MTF heuristics; competitive ratio = 2 for MTF.
- **Cost Model:** MTF (Move-to-Front): 2-competitive; COMB algorithm: 1.75-competitive; randomized lower bound = 1.75.
- **Real Wall:** List update is simpler than paging; serves as a testbed for online algorithm analysis.
- **Cross-Domain Aliases:** self-organizing-list (combinatorial-optimization), online-list (control-numerical-opt).
- **Notes:** Sleater & Tarjan (1985); list update is one of the first online problems with provable guarantees.

### [PRIM-063] matroid-greedy
- **Atom/Composite:** Primitive
- **Definition:** Matroid greedy algorithm: sort elements by weight descending, add if independent. Optimal for any matroid + weight.
- **Cost Model:** O(n log n + n·EO) where EO = oracle query for independence; greedy is optimal by exchange argument.
- **Real Wall:** Matroid greedy generalizes: minimum spanning tree (graphic matroid), linear matroid (linear independence), matching matroid.
- **Cross-Domain Aliases:** independent-set-greedy (control-numerical-opt), matroid-optimization (combinatorial-optimization).
- **Notes:** Edmonds (1979); matroids are exactly the structures where greedy is optimal.

### [PRIM-064] polymatroid-greedy
- **Atom/Composite:** Primitive
- **Definition:** Polymatroid greedy: for weighted polymatroid maximization, sort by weight and add if still in polymatroid.
- **Cost Model:** O(n log n + n·EO) for EO = submodular oracle query; greedy is optimal for polymatroids.
- **Real Wall:** Polymatroid greedy generalizes knapsack with submodular constraint; used in budget allocation.
- **Cross-Domain Aliases:** submodular-greedy (control-numerical-opt), polymatroid-constraint (combinatorial-optimization).
- **Notes:** Edmonds (1970); polymatroid = polymatroid constraint; polymatroid greedy = continuous knapsack with diminishing returns.

### [PRIM-065] submodular-max-greedy
- **Atom/Composite:** Composite
- **Definition:** Greedy for submodular maximization (cardinality constraint): select elements maximizing marginal gain. 1-1/e approx.
- **Cost Model:** O(n·f) for submodular function f; lazy evaluation with priority queue reduces to O(n log n) calls.
- **Real Wall:** 1-1/e is optimal for general submodular maximization (unless NP-hard); lazy greedy makes it practical.
- **Cross-Domain Aliases:** marginal-gain-greedy (control-numerical-opt), sensor-placement (combinatorial-optimization).
- **Notes:** Nemhauser et al. (1978); 1-1/e approx is optimal; lazy greedy (Minoux 1978) is practical.

### [PRIM-066] multilinear-relaxation
- **Atom/Composite:** Composite
- **Definition:** Multilinear relaxation: continuous extension of submodular function to [0,1]^n. Evaluated at fractional x via multilinear extension.
- **Cost Model:** Continuous greedy algorithm: O(n·f·ε^{-1}) to get (1-1/e - ε) approximation for monotone submodular.
- **Real Wall:** Multilinear extension enables continuous optimization; sampled rounding gives approximation guarantees.
- **Cross-Domain Aliases:** continuous-extension (control-numerical-opt), submodular-relaxation (combinatorial-optimization).
- **Notes:** Chekuri et al. (2014); continuous greedy + pipage rounding = (1-1/e - ε) for monotone submodular maximization.

### [PRIM-067] pipage-rounding
- **Atom/Composite:** Composite
- **Definition:** Pipage rounding: round fractional solution of matroid intersection to integral solution with no loss in objective.
- **Cost Model:** O(n·d) per rounding step; converges in O(n) steps; each step reduces fractional variables.
- **Real Wall:** Pipage rounding is used with continuous greedy; gives approximation for submodular maximization.
- **Cross-Domain Aliases:** fractional-rounding (control-numerical-opt), matroid-rounding (combinatorial-optimization).
- **Notes:** Ageev & Sviridenko (2004); pipage rounding is the standard rounding for matroid polytopes.

### [PRIM-068] primal-dual-approximation
- **Atom/Composite:** Composite
- **Definition:** Primal-dual approximation: maintain feasible primal and dual; increase dual when primal violates constraints. 2-approx for set cover, vertex cover.
- **Cost Model:** O(n·m) for n elements, m sets; primal-dual is faster than LP rounding and often equally good.
- **Real Wall:** Primal-dual is the classic approximation technique; works for covering and packing problems.
- **Cross-Domain Aliases:** dual-fitting (control-numerical-opt), primal-dual-ratio (combinatorial-optimization).
- **Notes:** Goemans & Williamson (1994); primal-dual ratio = dual objective / primal objective ≤ approximation factor.

### [PRIM-069] local-ratio-approximation
- **Atom/Composite:** Composite
- **Definition:** Local ratio: decompose weight function into sum of simple weight functions; apply recursively. Generalizes primal-dual.
- **Cost Model:** O(n·m) per reduction step; typically same approximation as primal-dual; simpler analysis.
- **Real Wall:** Local ratio is intuitive: reduce problem by subtracting a weight function that can be optimally covered.
- **Cross-Domain Aliases:** weight-reduction (control-numerical-opt), recursive-approx (combinatorial-optimization).
- **Notes:** Bar-Yehuda & Kehat (2001); local ratio gives same 2-approx for vertex cover and set cover, with simpler analysis.

### [PRIM-070]ptas-planar-graphs
- **Atom/Composite:** Composite
- **Definition:** PTAS for planar graphs: Arora's PTAS for TSP, Steiner tree, facility location. Polynomial in n, (1+ε) in approximation.
- **Cost Model:** Arora's PTAS for planar TSP: O(n·(log n)^{O(1/ε)}) via planar separator and dynamic programming.
- **Real Wall:** Planar graph structure (planar separator theorem) enables efficient PTAS for many problems.
- **Cross-Domain Aliases:** planar-approx (combinatorial-optimization), separator-theorem (control-numerical-opt).
- **Notes:** Arora (1998); PTAS on planar graphs uses planar separators and dynamic programming; extends to bounded-genus graphs.

### [PRIM-071] fptas-knapsack
- **Atom/Composite:** Composite
- **Definition:** FPTAS for knapsack: scaling + DP. Scale values, round to integers, run pseudo-polynomial DP. O(n·(1/ε)) time.
- **Cost Model:** O(n·(1/ε)) for FPTAS; trade-off between n and ε; for large n, FPTAS is near-linear.
- **Real Wall:** FPTAS is the best possible for knapsack (unless P=NP); FPTAS for knapsack is practical.
- **Cross-Domain Aliases:** pseudo-polynomial-dp (control-numerical-opt), scaling-dp (combinatorial-optimization).
- **Notes:** Ibarra & Kim (1975); FPTAS means (1-ε)·OPT in time polynomial in n and 1/ε.

### [PRIM-072] fptParameterized-complexity
- **Atom/Composite:** Composite
- **Definition:** FPT (Fixed-Parameter Tractable): algorithm O(f(k)·n^c) for parameter k. Kernelization reduces to O(k^c) instance.
- **Cost Model:** FPT: O(f(k)·n^c) where f(k) is exponential in k but not in n; kernelization O(n + k^c).
- **Real Wall:** FPT algorithms solve practical instances even for large k if f(k) grows slowly; kernelization is crucial.
- **Cross-Domain Aliases:** fixed-parameter (combinatorial-optimization), kernelization (control-numerical-opt).
- **Notes:** Downey & Fellows (1999); FPT is the main framework for exact algorithms; kernelization is preprocessing to small instance.

### [PRIM-073] kernelization-vertex-cover
- **Atom/Composite:** Composite
- **Definition:** Vertex cover kernelization: if |E| > k·n, return NO; else kernel of size O(k²). Simple and effective.
- **Cost Model:** Kernel extraction O(n·m); if max degree > k, include vertex in cover; reduces to kernel of size O(k²).
- **Real Wall:** Kernelization gives efficient preprocessing; vertex cover kernel is one of the simplest and most effective.
- **Cross-Domain Aliases:** reduction-rules (control-numerical-opt), preprocessing-fpt (combinatorial-optimization).
- **Notes:** Buss & Goldsmith (1993); vertex cover kernel is standard textbook example; 2k kernel exists via crown decomposition.

### [PRIM-074] bounded-treewidth-dp
- **Atom/Composite:** Composite
- **Definition:** DP over treewidth: O(w·2^w·n) for treewidth w. Nice tree decomposition, DP over bags.
- **Cost Model:** Treewidth computation O(n·2^n) exact; heuristics (minimum fill, maximum cardinality) give practical treewidth.
- **Real Wall:** Many problems have small treewidth on real-world instances; Courcelle's theorem: MSO on trees is linear.
- **Cross-Domain Aliases:** tree-decomposition (combinatorial-optimization), nice-tree-dp (control-numerical-opt).
- **Notes:** Bodlaender (1993); treewidth is the key parameter for efficient DP; exact treewidth is NP-hard but heuristics work.

### [PRIM-075] branchwidth-network-flow
- **Atom/Composite:** Primitive
- **Definition:** Branchwidth: related to treewidth (branchwidth = treewidth for graphs). Connected to network flow (min-cut = branchwidth for planar graphs).
- **Cost Model:** Branchwidth computation is as hard as treewidth; for planar graphs, branchwidth = min-cut capacity.
- **Real Wall:** Planar branchwidth relates to minimum cut; helps for parameterized algorithms on planar graphs.
- **Cross-Domain Aliases:** treewidth-relation (combinatorial-optimization), planar-graph (computational-geometry).
- **Notes:** Robertson & Seymour (1991); branchwidth and treewidth are related by a constant factor; both measure graph connectivity.

### [PRIM-076] iterative-deepening-a-star
- **Atom/Composite:** Composite
- **Definition:** IDA* (Iterative Deepening A*): A* search with incrementally increasing f-bound. Eliminates overhead of priority queue.
- **Cost Model:** O(b^d) where b = branching factor, d = depth; less memory than A*; same optimality guarantees.
- **Real Wall:** IDA* is the classic optimal heuristic search; used in puzzles (15-puzzle, Rubik's cube).
- **Cross-Domain Aliases:** heuristic-search (control-numerical-opt), optimal-search (combinatorial-optimization).
- **Notes:** Korf (1985); IDA* is optimal and memory-efficient; combines A*'s heuristic guidance with depth-first memory efficiency.

### [PRIM-077] conflict-directed-search
- **Atom/Composite:** Composite
- **Definition:** Conflict-directed search: detect and avoid conflicts during search. SAT solvers use conflict clauses; CP uses nogoods.
- **Cost Model:** Conflict analysis O(n) per conflict; generates nogood clause that prunes future search.
- **Real Wall:** Conflict-directed search is the key technique making SAT/CDCL solvers practical.
- **Cross-Domain Aliases:** nogood-avoidance (logic-reasoning), conflict-learning (combinatorial-optimization).
- **Notes:** Silva & Sakallah (1996); conflict-directed search generalizes beyond SAT to CP and MIP.

### [PRIM-078] local-search-tsp
- **Atom/Composite:** Composite
- **Definition:** Local search for TSP: 2-opt, 3-opt, Lin-Kernighan. Swap edges to improve tour; escape local optima via restarts.
- **Cost Model:** 2-opt: O(n²) per improvement; Lin-Kernighan: variable k-opt, O(n^{2.5}) practical; LKH is state-of-the-art.
- **Real Wall:** LKH (Lin-Kernighan heuristic) gets within 1% of OPT for TSP instances up to 10^6 cities.
- **Cross-Domain Aliases:** tsp-improvement (combinatorial-optimization), neighborhood-search (control-numerical-opt).
- **Notes:** Lin & Kernighan (1973); LKH is Helsgaun's improved implementation; dominates all other local search methods.

### [PRIM-079] vns-metaheuristic
- **Atom/Composite:** Composite
- **Definition:** VNS (Variable Neighborhood Search): systematically change neighborhood structure to escape local optima. Shake + local search + move.
- **Cost Model:** O(n) per neighborhood evaluation; neighborhood change is the key parameter; VND (VNS with deterministic change).
- **Real Wall:** VNS is robust and applies to many problems; can be combined with path relinking, tabu search.
- **Cross-Domain Aliases:** neighborhood-change (control-numerical-opt), metaheuristic-search (combinatorial-optimization).
- **Notes:** Mladenović & Hansen (1997); VNS is one of the most versatile metaheuristics; simple to implement.

### [PRIM-080] simulated-annealing
- **Atom/Composite:** Composite
- **Definition:** Simulated annealing: probabilistic local search accepting worse moves with probability exp(-Δ/T). Geometric cooling schedule.
- **Cost Model:** O(n) per evaluation; cooling schedule determines convergence; acceptance probability e^{-Δ/T} enables escape.
- **Real Wall:** SA converges to global optimum in limit if cooling is slow enough; practical schedules are fast but suboptimal.
- **Cross-Domain Aliases:** probabilistic-descent (control-numerical-opt), thermal-search (combinatorial-optimization).
- **Notes:** Kirkpatrick et al. (1983); SA is the first major metaheuristic; inspired by metallurgy annealing.

### [PRIM-081] tabu-search
- **Atom/Composite:** Composite
- **Definition:** Tabu search: local search with short-term memory (tabu list) to avoid revisiting recent solutions. Aspiration criterion overrides tabu.
- **Cost Model:** O(n) per move; tabu list size = neighborhood; long-term memory (frequency, strategic oscillation) improves.
- **Real Wall:** Tabu search is very effective for routing (VRP), scheduling, and quadratic assignment; flexible heuristic.
- **Cross-Domain Aliases:** memory-search (control-numerical-opt), iterative-improvement (combinatorial-optimization).
- **Notes:** Glover (1986); tabu search is one of the most applied metaheuristics; memory differentiates it from local search.

### [PRIM-082] grasp-metaheuristic
- **Atom/Composite:** Composite
- **Definition:** GRASP (Greedy Randomized Adaptive Search Procedure): construct greedy-randomized solution, then local search. Repeat.
- **Cost Model:** GRASP iteration O(n) for construction + O(n) for local search; repeat for R iterations, pick best.
- **Real Wall:** GRASP is easy to implement and applies to many problems; reactive GRASP adapts R based on solution quality.
- **Cross-Domain Aliases:** randomized-greedy (control-numerical-opt), construction-heuristic (combinatorial-optimization).
- **Notes:** Feo & Resende (1989); GRASP combines random construction with deterministic improvement; effective for large instances.

### [PRIM-083] evolutionary-algorithm
- **Atom/Composite:** Composite
- **Definition:** Evolutionary algorithm (EA): population of solutions, selection, crossover, mutation. GA, EP, ES.
- **Cost Model:** O(n·pop) per generation; crossover combines solutions; mutation introduces diversity.
- **Real Wall:** EAs are effective for black-box optimization; hyper-heuristics use EA to select heuristics.
- **Cross-Domain Aliases:** genetic-algorithm (control-numerical-opt), population-search (combinatorial-optimization).
- **Notes:** Holland (1975); genetic algorithms use binary encoding; evolutionary strategies use real-valued encoding.

### [PRIM-084] ant-colony-optimization
- **Atom/Composite:** Composite
- **Definition:** ACO (Ant Colony Optimization): simulated pheromone trail guides search. Pheromone update + probabilistic construction.
- **Cost Model:** O(n·m·ants) per iteration for n cities, m ants; pheromone evaporation prevents stagnation.
- **Real Wall:** ACO is excellent for routing problems (VRP, TSP); pheromone trail provides search memory.
- **Cross-Domain Aliases:** pheromone-search (control-numerical-opt), swarm-optimization (combinatorial-optimization).
- **Notes:** Dorigo et al. (1996); ACO is the swarm intelligence algorithm for combinatorial optimization.

### [PRIM-085] column-generation
- **Atom/Composite:** Composite
- **Definition:** Column generation: solve LP with many variables by pricing out columns. Master LP + pricing subproblem.
- **Cost Model:** Master LP: O(n·m) per iteration; subproblem: O(n) to price columns; convergence is slow near optimum.
- **Real Wall:** Column generation is essential for large-scale set covering, cutting stock, VRP; requires effective pricing subproblem.
- **Cross-Domain Aliases:** master-pricing (control-numerical-opt), decomposition-lp (combinatorial-optimization).
- **Notes:** Gilmore & Gomory (1961); column generation is the basis of Dantzig-Wolfe decomposition; converges slowly near optimum.

### [PRIM-086] benders-decomposition-master
- **Atom/Composite:** Composite
- **Definition:** Benders master problem: iteratively adds feasibility and optimality cuts from subproblem. L-shaped method for stochastic programming.
- **Cost Model:** Master LP: O(n) per cut added; subproblem: O(n) per iteration; convergence slow near optimum.
- **Real Wall:** Benders is effective when subproblem is easy (LP or independent across scenarios); stochastic programming is the main application.
- **Cross-Domain Aliases:** l-shaped-method (control-numerical-opt), scenario-decomposition (combinatorial-optimization).
- **Notes:** Van Slyke & Wets (1969); L-shaped method for two-stage stochastic programming with recourse.

### [PRIM-087] robust-optimization
- **Atom/Composite:** Composite
- **Definition:** Robust optimization: optimize against worst-case within uncertainty set. Uncertainty set: box, ellipsoidal, cardinality constrained.
- **Cost Model:** Ellipsoidal uncertainty: SOCP reformulation; cardinality uncertainty: MILP reformulation O(n·k).
- **Real Wall:** Robust optimization hedges against uncertainty; tractable reformulations for many problem classes.
- **Cross-Domain Aliases:** worst-case-optimization (control-numerical-opt), uncertainty-set (combinatorial-optimization).
- **Notes:** Ben-Tal & Nemirovski (1998); robust optimization produces solutions robust to data perturbations.

### [PRIM-088] multiobjective-optimization
- **Atom/Composite:** Composite
- **Definition:** Multi-objective optimization: Pareto frontier of nondominated solutions. ε-constraint, weighted sum, scalarization.
- **Cost Model:** Weighted sum: solve LP/IP for each weight vector; Pareto frontier enumeration O(m·solve) for m weight vectors.
- **Real Wall:** Multi-objective is common in practice (cost vs. quality vs. time); Pareto frontier is the gold standard.
- **Cross-Domain Aliases:** pareto-optimization (control-numerical-opt), multi-criteria (combinatorial-optimization).
- **Notes:** Ehrgott (2005); Pareto optimal = no objective can be improved without worsening another; scalarization converts to single-objective.

### [PRIM-089] approximation-scheme-fptas
- **Atom/Composite:** Composite
- **Definition:** FPTAS: (1-ε)-approximation in time polynomial in n and 1/ε. Knapsack is the canonical FPTAS example.
- **Cost Model:** Knapsack FPTAS: O(n·(1/ε)) via value scaling + DP; optimal trade-off between n and ε.
- **Real Wall:** FPTAS is the strongest approximation scheme; most problems do not have FPTAS (unless P=NP).
- **Cross-Domain Aliases:** fully-polynomial-scheme (control-numerical-opt), scaling-approx (combinatorial-optimization).
- **Notes:** Ibarra & Kim (1975); FPTAS implies FPT (set cover has PTAS but not FPTAS).

### [PRIM-090] inapproximability
- **Atom/Composite:** Composite
- **Definition:** Inapproximability: hardness of approximation results. PCP theorem: NP ⊆ PCP(log n, 1). Gap amplification.
- **Cost Model:** PCP query complexity = O(1) for gap amplification; hardness factors from PCPs, unique games, label cover.
- **Real Wall:** Approximation lower bounds require hardness assumptions (P ≠ NP, UGC); many results are conditional.
- **Cross-Domain Aliases:** hardness-gap (control-numerical-opt), approximation-lower-bound (combinatorial-optimization).
- **Notes:** Arora & Safra (1998); PCP theorem is the cornerstone of hardness of approximation; many results from PCP + Raz approximator.

### [PRIM-091] unique-games-conjecture
- **Atom/Composite:** Primitive
- **Definition:** UGC (Unique Games Conjecture): hardness of approximating labeling problems. If true, many approximation ratios are optimal.
- **Cost Model:** UGC-hardness: hardness of (0.5+ε)-approx for Max-Cut, 1/2+ε for Min-Uncut; Khot et al. (2007).
- **Real Wall:** UGC implies optimality of Goemans-Williamson (0.878-approx) for Max-Cut; still unproven but strong evidence.
- **Cross-Domain Aliases:** ugc-hardness (control-numerical-opt), label-cover (combinatorial-optimization).
- **Notes:** Khot et al. (2002); UGC is one of the most important open problems in theoretical computer science.

### [PRIM-092] lp-duality-approximation
- **Atom/Composite:** Composite
- **Definition:** LP duality in approximation: dual = relaxation + complementary slackness. Weak duality gives bound; strong duality for IP = LP.
- **Cost Model:** LP dual O(n·m); dual solution gives lower bound; primal feasible + dual feasible = approximation ratio.
- **Real Wall:** Dual fitting is the analysis method for primal-dual algorithms; dual objective lower bounds primal.
- **Cross-Domain Aliases:** dual-bound (control-numerical-opt), lp-relaxation-ratio (combinatorial-optimization).
- **Notes:** V Vazirani (2001); LP duality is the key analysis tool for approximation algorithms.

### [PRIM-093] primal-dual-set-cover
- **Atom/Composite:** Primitive
- **Definition:** Primal-dual set cover: dual constraint y_j for each element; increase y_j until set is paid for; add set to cover.
- **Cost Model:** O(n·m) for n elements, m sets; dual increases by minimum of remaining requirement of each element.
- **Real Wall:** Primal-dual gives 2-approx for set cover; matching upper bound of ln(n) (approximation ratio).
- **Cross-Domain Aliases:** covering-dual (control-numerical-opt), primal-dual-ratio (combinatorial-optimization).
- **Notes:** Bar-Yehuda & Ben-Sasson (2001); primal-dual is the simplest 2-approx for set cover; logarithmic improvement via LTF.

### [PRIM-094] integrality-gap
- **Atom/Composite:** Composite
- **Definition:** Integrality gap: ratio of IP optimum to LP relaxation optimum. Determines best possible LP-based approximation.
- **Cost Model:** Lower bound: specific instance; upper bound: approximation algorithm ratio; gap analysis guides algorithm design.
- **Real Wall:** Large integrality gap means LP relaxation is weak; need stronger formulations or different relaxations.
- **Cross-Domain Aliases:** lp-relaxation-gap (control-numerical-opt), relaxation-quality (combinatorial-optimization).
- **Notes:** Chekuri et al. (2004); integrality gap of 1 = LP can solve IP exactly; gap = f = approximation hardness lower bound.

### [PRIM-095] extended-formulation
- **Atom/Composite:** Composite
- **Definition:** Extended formulation: represent polyhedron as projection of higher-dimensional polyhedron. Yannakakis (1991) lower bounds.
- **Cost Model:** Yannakakis: extension size lower bound = nonnegative rank of slack matrix; factorization gives upper bound.
- **Real Wall:** TSP has no subexponential extended formulation (unless P=NP); matching polytope has exponential extension.
- **Cross-Domain Aliases:** projection-formulation (control-numerical-opt), higher-dim-poly (combinatorial-optimization).
- **Notes:** Yannakakis (1991); extended formulations can be exponentially smaller than natural formulations.

### [PRIM-096] scheduling-on-unrelated-machines
- **Atom/Composite:** Composite
- **Definition:** R||C_max (unrelated machines): job j takes p_ij time on machine i. Makespan minimization is NP-hard; 2-approx via LP.
- **Cost Model:** LP rounding: O(n·m·ε^{-1}) for (2-ε)-approx; lower bound = max{max p_ij, max sum p_ij/m}; FPTAS for fixed m.
- **Real Wall:** Unrelated machines generalize identical (P) and uniform (Q) machines; job-machine assignment is the core decision.
- **Cross-Domain Aliases:** unrelated-machine-scheduling (control-numerical-opt), makespan-ip (combinatorial-optimization).
- **Notes:** Lenstra et al. (1990); R||C_max has 1.5-approx by LP rounding; best known is 1.92-approx.

### [PRIM-097] precedence-constrained-scheduling
- **Atom/Composite:** Composite
- **Definition:** P|prec|C_max (parallel machines with precedence): 2-approx via list scheduling (Graham 1966). FPTAS for fixed m.
- **Cost Model:** List scheduling: O(n·log m) with priority queue; competitive ratio = 2 - 1/m; optimal = P|prec|C_max.
- **Real Wall:** Precedence constraints make scheduling harder but also more constrained; CP scheduling with precedence is standard.
- **Cross-Domain Aliases:** precedence-graph (control-numerical-opt), dag-scheduling (combinatorial-optimization).
- **Notes:** Graham (1966); list scheduling is simple, robust, and near-optimal; more sophisticated algorithms improve slightly.

### [PRIM-098] polynomial-time-approximation-scheme
- **Atom/Composite:** Composite
- **Definition:** PTAS: (1+ε)-approximation in time polynomial in n (exponential in 1/ε). FPTAS: polynomial in n and 1/ε.
- **Cost Model:** PTAS runtime: O(n^{O(1/ε)}) or O(n·log(1/ε)·(1/ε)^{O(1)}); PTAS for many problems on planar graphs.
- **Real Wall:** PTAS is near-optimal for practical ε values (10%); PTAS for planar graphs via Baker's technique.
- **Cross-Domain Aliases:** epsilon-approximation (control-numerical-opt), approximation-scheme (combinatorial-optimization).
- **Notes:** Baker (1983); Baker's technique: dynamic programming on treewidth for planar graphs; gives PTAS for many problems.

### [PRIM-099] approximation-lower-bound
- **Atom/Composite:** Composite
- **Definition:** Approximation lower bound: hardness of approximation ratio. PCP, LTF, UG-hardness.
- **Cost Model:** Lower bound construction: PCP reduction, gap amplification, hardness of label cover.
- **Real Wall:** Lower bounds guide algorithm design; matching upper and lower bounds = optimal approximation.
- **Cross-Domain Aliases:** inapprox-hardness (control-numerical-opt), gap-hardness (combinatorial-optimization).
- **Notes:** Hastad (2001); Max-3SAT(5) has integrality gap of 1/2 + ε but 7/8-approx is NP-hard.

### [PRIM-100] lagrangian-multiplier-saving
- **Atom/Composite:** Primitive
- **Definition:** Lagrangian multiplier saving: when subgradient iteration increases Lagrangian dual, save current multipliers. Improves convergence.
- **Cost Model:** O(n) per iteration; saves best dual found; doesn't guarantee monotonicity but practically helps.
- **Real Wall:** Subgradient methods for Lagrangian relaxation converge slowly (O(1/√T)); multiplier saving helps in practice.
- **Cross-Domain Aliases:** subgradient-method (control-numerical-opt), dual-convergence (combinatorial-optimization).
- **Notes:** Poljak (1969); multiplier saving is a heuristic that maintains best-known dual; not theoretically justified.

### [PRIM-101] simulated-annealing-cooling
- **Atom/Composite:** Composite
- **Definition:** SA cooling schedule: logarithmic T_k = c/log(k), geometric T_k = α·T_{k-1}. Logarithmic: theoretically convergent; geometric: practical.
- **Cost Model:** Logarithmic: O(n·log n) iterations to reach T=0; geometric: O(n·log(1/δ)) iterations for T < δ.
- **Real Wall:** Logarithmic cooling is too slow in practice; geometric cooling with adaptive reheating is practical.
- **Cross-Domain Aliases:** temperature-schedule (control-numerical-opt), thermal-annealing (combinatorial-optimization).
- **Notes:** Geman & Geman (1984); SA with logarithmic cooling converges to global optimum; impractical for large instances.

### [PRIM-102] great-deluge
- **Atom/Composite:** Primitive
- **Definition:** Great deluge: local search with rising acceptance threshold (water level). Accept moves above water level; level rises slowly.
- **Cost Model:** O(n) per move; water level rises linearly; different from SA's temperature parameter.
- **Real Wall:** Great deluge converges faster than SA; level rise speed determines exploration vs. exploitation.
- **Cross-Domain Aliases:** level-acceptance (control-numerical-opt), threshold-search (combinatorial-optimization).
- **Notes:** Dueck (1993); great deluge is deterministic analog of SA; simple and effective for many problems.

### [PRIM-103] ils-metaheuristic
- **Atom/Composite:** Composite
- **Definition:** ILS (Iterated Local Search): perturb best solution, apply local search, accept if improved. Escape local optima via perturbation.
- **Cost Model:** O(n) per perturbation + O(n) per local search; perturbation size = key parameter; adaptive perturbation.
- **Real Wall:** ILS is very effective for TSP, VRP, scheduling; combining with path relinking improves further.
- **Cross-Domain Aliases:** iterated-improvement (control-numerical-opt), perturbation-search (combinatorial-optimization).
- **Notes:** Lourenço et al. (2003); ILS is simple and effective; perturbation must be strong enough to escape but not too strong.

### [PRIM-104] matheuristic
- **Atom/Composite:** Composite
- **Definition:** Matheuristic: combine exact (IP/CP) with heuristic (local search). Feasibility pump, feasibility search, matheuristics.
- **Cost Model:** Feasibility pump: alternating IP solve and rounding; O(n·iterations) per pump; restarts break stagnation.
- **Real Wall:** Matheuristics combine strengths of exact and heuristic methods; particularly effective for MIP with structure.
- **Cross-Domain Aliases:** hybrid-ip-heuristic (control-numerical-opt), exact-heuristic (combinatorial-optimization).
- **Notes:** Fischetti & Salvagnin (2009); feasibility pump is the classic matheuristic for MIP; many hybrid variants.

### [PRIM-105] feasibility-pump
- **Atom/Composite:** Composite
- **Definition:** Feasibility pump: alternate between LP solve (round solution) and finding nearest integer solution. For MIP feasibility.
- **Cost Model:** O(n·m·iterations) where iterations typically < 1000; fast to find feasible solutions.
- **Real Wall:** Feasibility pump finds feasible solutions quickly; diving + rounding + IP solve is common pattern.
- **Cross-Domain Aliases:** solution-pump (control-numerical-opt), pump-heuristic (combinatorial-optimization).
- **Notes:** Fischetti et al. (2005); feasibility pump is the workhorse for MIP feasibility; many variants improve convergence.

### [PRIM-106] rounding-heuristic-ip
- **Atom/Composite:** Composite
- **Definition:** Rounding heuristic: round LP solution to nearest integer; repair infeasibility via local search. Fast feasible solutions.
- **Cost Model:** O(n) for rounding + O(n·iterations) for repair; effective when LP relaxation is tight.
- **Real Wall:** Rounding is the simplest MIP heuristic; randomization (random round + repair) improves coverage.
- **Cross-Domain Aliases:** lp-rounding-heuristic (control-numerical-opt), rounding-repair (combinatorial-optimization).
- **Notes:** Bertsimas et al. (2006); randomized rounding + discrepancy theory gives strong theoretical guarantees.

### [PRIM-107] core-based-approximation
- **Atom/Composite:** Composite
- **Definition:** Core concept: identify hard subproblem (core) after LP relaxation; apply intensive method to core. Scales to large instances.
- **Cost Model:** Core identification O(n) by LP residual; core size = O(k·log n) for k = solution size; apply B&B to core.
- **Real Wall:** Core-based methods solve large combinatorial optimization instances by focusing on hard part.
- **Cross-Domain Aliases:** core-identification (control-numerical-opt), problem-core (combinatorial-optimization).
- **Notes:** Borndörfer et al. (1998); core-based methods are key for large-scale vehicle routing and crew scheduling.

### [PRIM-108] cutting-plane-method
- **Atom/Composite:** Composite
- **Definition:** Cutting plane method: iteratively add violated inequalities. Outer approximation for convex functions.
- **Cost Model:** O(n·m) per cut generation; separation oracle finds most violated inequality; converges slowly near optimum.
- **Real Wall:** Outer approximation (OA) solves MILP with convex nonlinear constraints; combined with B&B = spatial branch-and-bound.
- **Cross-Domain Aliases:** outer-approximation (control-numerical-opt), convex-cutting (combinatorial-optimization).
- **Notes:** Kelley (1960); cutting plane method converges slowly (Chvatal-Gomory rank); bundle methods stabilize.

### [PRIM-109] spatial-branch-bound
- **Atom/Composite:** Composite
- **Definition:** Spatial branch-and-bound: branch on nonlinear variables (convex/concave). Lower bound = convex relaxation; upper bound = feasible solution.
- **Cost Model:** O(n) per node; convergence depends on convexity; many nodes required for non-convex.
- **Real Wall:** spatial branch-and-bound solves non-convex MILP (global optimization); Baron, ANTIGONE, GloMIQO are global solvers.
- **Cross-Domain Aliases:** global-optimization (control-numerical-opt), non-convex-bb (combinatorial-optimization).
- **Notes:** Tawarmalani & Sahinidis (2002); spatial B&B is the method for global optimization of non-convex MILP.

### [PRIM-110] dispatch-rules
- **Atom/Composite:** Composite
- **Definition:** Dispatch rules: simple heuristics for real-time scheduling. SPT (Shortest Processing Time), EDD (Earliest Due Date), CR (Critical Ratio).
- **Cost Model:** O(1) per decision; evaluate all jobs, pick best; rule-based systems for real-time manufacturing.
- **Real Wall:** Dispatch rules are industry standard for real-time scheduling; SPT minimizes average completion time; EDD minimizes maximum lateness.
- **Cross-Domain Aliases:** real-time-scheduling (control-numerical-opt), rule-based-heuristic (combinatorial-optimization).
- **Notes:** Blackstone et al. (1982); dispatch rules are simple, interpretable, and practical; ATCR (Apparent Tardiness Cost) is near-optimal.

### [PRIM-111] scheduling-rule-atcr
- **Atom/Composite:** Primitive
- **Definition:** ATCR (Apparent Tardiness Cost rule): ATC = max(1/σ, (d_j - t - p_j)/σ). Hybrid of SPT and EDD.
- **Cost Model:** O(1) per decision; σ = lookahead parameter; empirical σ ≈ 0.1·average processing time works well.
- **Real Wall:** ATCR dominates dispatch rules for tardiness scheduling; simple to implement and near-optimal.
- **Cross-Domain Aliases:** apparent-tardiness (control-numerical-opt), weighted-tardiness-rule (combinatorial-optimization).
- **Notes:** Vepsalainen & Morton (1987); ATC rule is the best simple dispatch rule for weighted tardiness.

### [PRIM-112] scheduling-shifting-bottleneck
- **Atom/Composite:** Composite
- **Definition:** Shifting Bottleneck Procedure (SBP): identify bottleneck machine, solve single-machine subproblem, repeat. For job shop.
- **Cost Model:** Bottleneck identification O(n·m) per iteration; single-machine scheduling O(n²·log n) per bottleneck.
- **Real Wall:** SBP is the best known heuristic for job shop scheduling; competitive with B&B for large instances.
- **Cross-Domain Aliases:** bottleneck-machine (control-numerical-opt), job-shop-heuristic (combinatorial-optimization).
- **Notes:** Adams et al. (1988); SBP decomposes job shop into single-machine problems; converges in O(m) iterations.

### [PRIM-113] routing-with-time-windows
- **Atom/Composite:** Composite
- **Definition:** VRPTW (Vehicle Routing with Time Windows): routes with time window constraints. Solomon's benchmark instances.
- **Cost Model:** Solomon's I1 heuristic (insertion-based): O(n²) per insertion; local search improves; IP for small instances.
- **Real Wall:** VRPTW is the practical version of VRP; time windows add complexity but are essential in real-world routing.
- **Cross-Domain Aliases:** vrp-tw (combinatorial-optimization), time-constrained-routing (control-numerical-opt).
- **Notes:** Solomon (1987); VRPTW is benchmarked extensively; genetic algorithms and local search dominate.

### [PRIM-114] pickup-delivery-problem
- **Atom/Composite:** Composite
- **Definition:** PDP (Pickup-Delivery Problem): route vehicles to transport goods from pickup to delivery locations. Pairing and segmenting approaches.
- **Cost Model:** PDP with capacity: insert pickup and delivery as pair; precedence constraint; LKH-PDSP for large instances.
- **Real Wall:** PDP is the practical routing problem; requires precedence (pickup before delivery) and capacity constraints.
- **Cross-Domain Aliases:** pdp-routing (combinatorial-optimization), paired-routing (control-numerical-opt).
- **Notes:** Psaraftis (1980); PDP is more complex than VRP; min-cost flow formulation with pairing constraints.

### [PRIM-115] dynamic-vehicle-routing
- **Atom/Composite:** Composite
- **Definition:** Dynamic VRP: orders arrive during execution. Reoptimization and waiting strategies; competitive ratio analysis.
- **Cost Model:** Online algorithms: competitive ratio depends on request rate; insert new requests into current route.
- **Real Wall:** Dynamic VRP is the real-world scenario; static reoptimization is practical when requests slow.
- **Cross-Domain Aliases:** online-vrp (combinatorial-optimization), real-time-routing (control-numerical-opt).
- **Notes:** Psaraftis et al. (2015); dynamic VRP is much harder; anticipatory routing uses forecasts.

### [PRIM-116] quadratic-assignment
- **Atom/Composite:** Composite
- **Definition:** QAP (Quadratic Assignment Problem): assign facilities to locations minimizing sum of distances × flows. NP-hard; heuristic approaches.
- **Cost Model:** Lower bound: linearization (e.g., Gilmore-Lawler), FAC (fire algorithm); upper bound: local search (2-opt, LIS).
- **Real Wall:** QAP is one of the hardest combinatorial optimization problems; instances of size n=30 are very hard.
- **Cross-Domain Aliases:** facility-layout (combinatorial-optimization), quadratic-ip (control-numerical-opt).
- **Notes:** Koopmans & Beckmann (1957); QAP models facility layout, keyboard design; heuristics (GRASP, tabu) are standard.

### [PRIM-117] linear-approximation-sdp
- **Atom/Composite:** Composite
- **Definition:** SDP relaxation: replace quadratic constraint (X = xx^T) with linear LMI (X ≽ 0). Goemans-Williamson rounding for Max-Cut.
- **Cost Model:** SDP solver: O(n^{3.5}) for n variables via interior point; dual-scaling algorithms faster for large n.
- **Real Wall:** SDP relaxation is very powerful; tighter than LP for many problems (Max-Cut, MIS, clustering).
- **Cross-Domain Aliases:** semidefinite-relaxation (control-numerical-opt), rounding-approx (combinatorial-optimization).
- **Notes:** Goemans & Williamson (1994); SDP relaxation + random hyperplane rounding gives 0.878-approx for Max-Cut.

### [PRIM-118] goemans-williamson-rounding
- **Atom/Composite:** Composite
- **Definition:** GW rounding: project SDP solution onto random hyperplane; sign gives cut. Expected value ≥ 0.878·OPT.
- **Cost Model:** O(n²) for hyperplane projection; random hyperplane: O(n) per rounding; expectation over many trials.
- **Real Wall:** GW rounding is the canonical SDP rounding; analysis uses Tracy-Singh partition of sphere.
- **Cross-Domain Aliases:** hyperplane-rounding (control-numerical-opt), sdp-rounding (combinatorial-optimization).
- **Notes:** Goemans & Williamson (1994); GW rounding achieves 0.878 which is optimal assuming UGC.

### [PRIM-119] christofides-algorithm
- **Atom/Composite:** Composite
- **Definition:** Christofides algorithm: MST + minimum-weight perfect matching on odd-degree vertices. 1.5-approx for metric TSP.
- **Cost Model:** MST O(E log V); matching O(V³); total O(V³). 1.5-approx is best known deterministic for metric TSP.
- **Real Wall:** Christofides is the classic 1.5-approx for metric TSP; Random-MST (substituting random for matching) improves in practice.
- **Cross-Domain Aliases:** tsp-approx (combinatorial-optimization), spanning-tree-based (control-numerical-opt).
- **Notes:** Christofides (1976); worst-case is 1.5, average case much better; 1.5-approx is the theoretical guarantee.

### [PRIM-120] arora-ptas-tsp
- **Atom/Composite:** Composite
- **Definition:** Arora's PTAS for planar TSP: recursive quadrisection, dynamic programming on quadtree. O(n·(log n)^{O(1/ε)}) time.
- **Cost Model:** Planar separator: O(n) decomposition; DP: exponential in 4^{O(1/ε)} per cell; overall PTAS runtime.
- **Real Wall:** Arora's PTAS is theoretical but impractical; planar structure (road networks) enables fast PTAS.
- **Cross-Domain Aliases:** planar-tsp (computational-geometry), separator-dp (combinatorial-optimization).
- **Notes:** Arora (1998); PTAS uses planar separators + dynamic programming on guillotine partitions.

### [PRIM-121] scheduling-load-balancing
- **Atom/Composite:** Composite
- **Definition:** Load balancing (makespan minimization on identical machines): greedy LPT = 4/3-1/(3m)-approx; RLFY = 1.0206-approx.
- **Cost Model:** LPT: O(n log n) sort + O(n·m) assignment; RLFY: more complex but achieves 1.0206.
- **Real Wall:** LPT is the practical standard; difference between LPT and optimal is small in practice.
- **Cross-Domain Aliases:** load-balance-scheduling (control-numerical-opt), makespan-greedy (combinatorial-optimization).
- **Notes:** Graham (1966) LPT; RLFY (Rao, flat) is the best known heuristic; FPTAS exists for fixed m.

### [PRIM-122] multiobjective-dp
- **Atom/Composite:** Composite
- **Definition:** Multi-objective DP: Pareto DP table of nondominated states. Dimensions = number of objectives × capacity.
- **Cost Model:** DP table size O(n·k^d) for d objectives, k capacity; pruning reduces to Pareto frontier per state.
- **Real Wall:** Multi-objective DP is exponential in number of objectives; only tractable for small d (< 3).
- **Cross-Domain Aliases:** pareto-dp (control-numerical-opt), multi-criteria-dp (combinatorial-optimization).
- **Notes:** Ehrgott (2005); multi-objective DP enumerates all Pareto optimal solutions; efficient pruning by domination.

### [PRIM-123] stochastic-optimization
- **Atom/Composite:** Composite
- **Definition:** Stochastic optimization: optimize expected value under uncertainty. Chance constraints, risk measures, sample average approximation.
- **Cost Model:** SAA (Sample Average Approximation): solve with n scenarios; convergence rate O(1/√n) by CLT.
- **Real Wall:** Stochastic optimization requires risk modeling; CVaR (Conditional Value at Risk) is more robust than VaR.
- **Cross-Domain Aliases:** stochastic-programming (control-numerical-opt), expected-optimization (combinatorial-optimization).
- **Notes:** Shapiro et al. (2009); two-stage stochastic programming with recourse; sample complexity of SAA.

### [PRIM-124] chance-constrained-optimization
- **Atom/Composite:** Composite
- **Definition:** Chance-constrained optimization: P(constraints satisfied) ≥ 1-ε. Typically requires convex formulation or scenario approximation.
- **Cost Model:** Scenario approach: 1-(1-ε)^{1/m} ≈ ε/m samples; Bernstein bound; requires convexity for exact reformulation.
- **Real Wall:** Chance constraints are more interpretable than risk measures; require safety stock in operations.
- **Cross-Domain Aliases:** safety-stock-optimization (control-numerical-opt), probabilistic-constraint (combinatorial-optimization).
- **Notes:** Charnes & Cooper (1959); chance constraints are generally non-convex; CVaR approximation is more tractable.

### [PRIM-125] robust-knapsack
- **Atom/Composite:** Composite
- **Definition:** Robust knapsack: item weights uncertain within intervals. Minimax regret solution; budget of uncertainty Γ.
- **Cost Model:** Γ-robust knapsack: O(n·Γ) DP; cardinality uncertainty is combinatorial; ellipsoidal uncertainty is SOCP.
- **Real Wall:** Robust optimization hedges against worst-case; Γ controls conservatism; Γ=0 is nominal, Γ=n is worst-case.
- **Cross-Domain Aliases:** robust-ip (control-numerical-opt), interval-uncertainty (combinatorial-optimization).
- **Notes:** Bertsimas & Sim (2004); Γ-robust knapsack has polynomial-time DP; budget uncertainty framework generalizes.

### [PRIM-126] markov-decision-process
- **Atom/Composite:** Composite
- **Definition:** MDP (Markov Decision Process): sequential decision under uncertainty. Value iteration, policy iteration, linear programming.
- **Cost Model:** Value iteration O(S²·A·T) for S states, A actions, T horizon; policy iteration O(S²·A·iterations).
- **Real Wall:** MDP is the foundation of RL; finite-horizon MDP has exact solution; infinite requires discount factor or average cost.
- **Cross-Domain Aliases:** dynamic-programming-under-uncertainty (control-numerical-opt), sequential-decision (combinatorial-optimization).
- **Notes:** Puterman (1994); MDP = Markov reward process + actions; Bellman optimality equation V*(s) = max_a (R(s,a) + γ·Σ_s' P(s'|s,a)·V*(s')).

### [PRIM-127] combinatorial-auctions
- **Atom/Composite:** Composite
- **Definition:** Combinatorial auction: bidders submit bundles of items; winner determination = MAXRES problem (NP-hard).
- **Cost Model:** MAXRES = NP-hard; CPLEX for small instances; greedy + local search for large; OR bidding language simplifies.
- **Real Wall:** Combinatorial auctions enable efficient allocation when items are complements; FCC spectrum auctions use CA mechanisms.
- **Cross-Domain Aliases:** winner-determination (combinatorial-optimization), bundle-auction (control-numerical-opt).
- **Notes:** Cramton et al. (2006); CA winner determination is the hardest subproblem; proxy bidding reduces complexity.

### [PRIM-128] maximum-clique
- **Atom/Composite:** Composite
- **Definition:** Maximum clique: find largest complete subgraph. NP-hard; branch-and-bound (Bron-Kerbosch) + coloring bound.
- **Cost Model:** Bron-Kerbosch with pivot: exponential worst-case; coloring bound (Tomita) improves pruning dramatically.
- **Real Wall:** Maximum clique is a classic NP-hard problem; coloring-based ordering is key to practical algorithms.
- **Cross-Domain Aliases:** independent-set (combinatorial-optimization), branch-and-bound-maximum (control-numerical-opt).
- **Notes:** Tomita et al. (2003); BBMC (Branch and Bound with Max Clique) is the state-of-the-art exact algorithm.

### [PRIM-129] maximum-independent-set
- **Atom/Composite:** Primitive
- **Definition:** Maximum independent set: find largest set of non-adjacent vertices. Complement of maximum clique; MIS on chordal graphs is polynomial.
- **Cost Model:** General graphs: NP-hard; chordal graphs: greedy works; perfect graphs: ellipsoid method solves polynomial.
- **Real Wall:** MIS on planar graphs has PTAS; maximum matching dual; independent set = vertex cover complement.
- **Cross-Domain Aliases:** vertex-independence (combinatorial-optimization), maximum-stable-set (control-numerical-opt).
- **Notes:** MIS is the complement of vertex cover: α(G) + τ(G) = |V|; MIS on bipartite graphs = max matching.

### [PRIM-130] graph-coloring
- **Atom/Composite:** Composite
- **Definition:** Graph coloring: assign minimum colors so adjacent vertices have different colors. NP-hard; Welsh-Powell heuristic gives k·Δ/2 approx.
- **Cost Model:** Welsh-Powell O(n²) greedy coloring; DSATUR (branch on highest saturation vertex) is best exact.
- **Real Wall:** Graph coloring is NP-hard; register allocation (Chaitin) uses graph coloring; heuristic is practical.
- **Cross-Domain Aliases:** chromatic-number (combinatorial-optimization), register-allocation (type-theory-programming-languages).
- **Notes:** Welsh & Powell (1967); graph coloring is one of the most fundamental NP-hard problems; DSATUR = best exact algorithm.

### [PRIM-131] edge-coloring
- **Atom/Composite:** Composite
- **Definition:** Edge coloring: assign minimum colors to edges so adjacent edges have different colors. Vizing's theorem: Δ+1 colors suffice.
- **Cost Model:** Class 1 graphs (edge-chromatic = Δ+1): polynomial; class 2 (NP-hard): (Δ+1) approx via Vizing's algorithm.
- **Real Wall:** Edge coloring of bipartite graphs is polynomial (König's theorem); general graphs use Vizing's algorithm.
- **Cross-Domain Aliases:** timetabling (combinatorial-optimization), schedule-edges (control-numerical-opt).
- **Notes:** Vizing (1964); edge coloring models timetabling and frequency assignment; bipartite graphs have Δ-coloring.

### [PRIM-132] matching-in-general-graphs
- **Atom/Composite:** Composite
- **Definition:** General matching: Edmonds' blossom algorithm finds maximum cardinality matching in general graph.
- **Cost Model:** Blossom algorithm O(V³) worst-case; Micali-Vazirani O(E·√V); Blossom V is the practical implementation.
- **Real Wall:** Blossom algorithm is complex but fast in practice; used for bipartite and general matching.
- **Cross-Domain Aliases:** non-bipartite-matching (combinatorial-optimization), blossom-algorithm (control-numerical-opt).
- **Notes:** Micali & Vazirani (1980); O(E·√V) is asymptotically optimal; Edmonds' blossom algorithm is simpler but O(V³).

### [PRIM-133] stable-marriage-problem
- **Atom/Composite:** Composite
- **Definition:** Stable marriage: match men and women with complete preferences; no blocking pairs (man and woman who prefer each other to current partners).
- **Cost Model:** Gale-Shapley algorithm O(n²) for n men and n women; deferred acceptance finds stable matching.
- **Real Wall:** Stable matching is always guaranteed; men-optimal Gale-Shapley gives male-optimal, female-pessimal stable matching.
- **Cross-Domain Aliases:** stable-matching (combinatorial-optimization), deferred-acceptance (control-numerical-opt).
- **Notes:** Gale & Shapley (1962); hospital/resident matching uses stable matching; strategy-proof for one side.

### [PRIM-134] assignment-problem
- **Atom/Composite:** Primitive
- **Definition:** Assignment problem: match n tasks to n agents minimizing total cost. Hungarian algorithm O(n³); min-cost flow O(n²·log n).
- **Cost Model:** Hungarian O(n³) worst-case; Jonker-Volgenant implementation O(n²) for moderate n.
- **Real Wall:** Assignment is polynomial; used for matching, transport, scheduling; the relaxation of many routing problems.
- **Cross-Domain Aliases:** bipartite-matching-mincost (combinatorial-optimization), hungarian-solver (control-numerical-opt).
- **Notes:** Kuhn (1955); assignment is the simplest perfect matching optimization; linear programming dual gives strong bounds.

### [PRIM-135] minimum-spanning-tree
- **Atom/Composite:** Primitive
- **Definition:** MST (Minimum Spanning Tree): connect all vertices with minimum total edge weight. Kruskal O(E log E), Prim O(E + V log V).
- **Cost Model:** Kruskal: sort edges O(E log E), union-find O(E·α(V)); Prim: Fibonacci heap O(E + V log V).
- **Real Wall:** MST is polynomial and fast; used as subroutine for Steiner tree, approximation algorithms.
- **Cross-Domain Aliases:** mst-algorithm (combinatorial-optimization), spanning-tree-min (control-numerical-opt).
- **Notes:** Prim (1957), Kruskal (1956); MST is one of the most practical combinatorial optimization problems.

### [PRIM-136] shortest-path
- **Atom/Composite:** Composite
- **Definition:** Shortest path: minimum cost path from s to t. Dijkstra O(E + V log V); Bellman-Ford O(VE); A* with heuristic.
- **Cost Model:** Dijkstra with Fibonacci heap O(E + V log V); with binary heap O(E log V); A* reduces search space.
- **Real Wall:** Shortest path is polynomial; A* with admissible heuristic = optimal; heuristic is key to speed.
- **Cross-Domain Aliases:** shortest-path-dijkstra (combinatorial-optimization), a-star (control-numerical-opt).
- **Notes:** Dijkstra (1959); A* = Dijkstra + heuristic; heuristic = estimate of distance to goal; admissible = never overestimates.

### [PRIM-137] parametric-shortest-path
- **Atom/Composite:** Composite
- **Definition:** Parametric shortest path: edge weights are linear functions of parameter λ. Find minimum for all λ. Piecewise linear function.
- **Cost Model:** Solve at breakpoints; number of breakpoints can be O(VE); parametric programming approach is more efficient.
- **Real Wall:** Parametric shortest path appears in robust optimization (interval weights), min-cost flow (concave costs).
- **Cross-Domain Aliases:** lambda-shortest-path (combinatorial-optimization), parametric-optimization (control-numerical-opt).
- **Notes:** Karp & Orlin (1981); parametric shortest path finds shortest path for all λ in [0,1]; useful for bicriteria optimization.

### [PRIM-138] prize-collecting-steiner-tree
- **Atom/Composite:** Composite
- **Definition:** Prize-collecting Steiner tree: maximize prize of connected vertices minus edge costs. 2-approx via primal-dual.
- **Cost Model:** Primal-dual: O(E log V) per iteration; 2-approx; LP rounding gives 1.96-approx; exact via IP.
- **Real Wall:** Prize-collecting Steiner tree is more flexible than standard Steiner tree; natural for network design.
- **Cross-Domain Aliases:** steiner-tree-prize (combinatorial-optimization), generalized-steiner (control-numerical-opt).
- **Notes:** Bienstock et al. (1996); prize-collecting Steiner tree generalizes standard Steiner tree and facility location.

### [PRIM-139] survivable-network-design
- **Atom/Composite:** Composite
- **Definition:** SNDP (Survivable Network Design): design minimum-cost network with k-connectivity requirements between pairs. Primal-dual 2-approx.
- **Cost Model:** Primal-dual O(E·log V) per requirement; 2-approx for edge-connectivity; 2·(2k-1) approx for vertex-connectivity.
- **Real Wall:** SNDP models reliable network design; edge-disjoint vs vertex-disjoint paths have different complexity.
- **Cross-Domain Aliases:** network-design-k-connectivity (combinatorial-optimization), resilient-network (control-numerical-opt).
- **Notes:** Jain (1998); SNDP with edge-connectivity has 2-approx via primal-dual; vertex-connectivity is harder.

### [PRIM-140] online-bipartite-matching
- **Atom/Composite:** Composite
- **Definition:** Online bipartite matching: match arriving requests to fixed servers. RANKING algorithm 1-1/e approx; RANDOMIZED lower bound 1-1/e.
- **Cost Model:** RANKING: O(n·log n) with priority queue; 1-1/e competitive ratio is optimal for online matching.
- **Real Wall:** Online matching models ad allocation, ride sharing; RANKING (Karp et al. 1990) is the canonical online algorithm.
- **Cross-Domain Aliases:** online-assignment (combinatorial-optimization), ad-matching (control-numerical-opt).
- **Notes:** Karp et al. (1990); online matching with adversarial requests and known servers; 1-1/e is optimal by lower bound.

### [PRIM-141] fractional-matching
- **Atom/Composite:** Primitive
- **Definition:** Fractional matching: relax integer constraint x_e ∈ {0,1} to x_e ∈ [0,1]. Bipartite: integer optimal; general: half-integral.
- **Cost Model:** Fractional matching = LP; bipartite: solvable in O(E·V^{1/2}) via bipartite flow; general: Bloss SOMETHING.
- **Real Wall:** Fractional matching provides lower bound for integer matching; half-integrality of general matching polytope.
- **Cross-Domain Aliases:** fractional-assignment (combinatorial-optimization), matching-relaxation (control-numerical-opt).
- **Notes:** Edmonds (1965) showed general matching polytope is half-integral; integral if graph is bipartite.

### [PRIM-142] arborescence
- **Atom/Composite:** Composite
- **Definition:** Minimum cost arborescence (directed spanning tree): directed analog of MST from root. Edmonds' algorithm O(E·V).
- **Cost Model:** Edmonds' algorithm: contract strongly connected components, find minimum incoming edges, contract, repeat. O(E·V).
- **Real Wall:** Arborescence models directed network design, rooted Steiner tree in directed graphs.
- **Cross-Domain Aliases:** directed-mst (combinatorial-optimization), arborescence-algorithm (control-numerical-opt).
- **Notes:** Edmonds (1967); minimum cost arborescence is the directed analog of minimum spanning tree.

### [PRIM-143] chinese-postman
- **Atom/Composite:** Composite
- **Definition:** Chinese postman: traverse all edges of graph at minimum cost. Undirected: add shortest paths between odd-degree vertices.
- **Cost Model:** Undirected: MST on odd-degree subgraph O(V³) for matching; directed: min-cost flow O(E·V·log V).
- **Real Wall:** Chinese postman = CPP (Chinese Postman Problem); used in mail delivery, street sweeping routing.
- **Cross-Domain Aliases:** postman-problem (combinatorial-optimization), route-cover (control-numerical-opt).
- **Notes:** Guan (1962); CPP is polynomial for undirected, directed, and mixed graphs.

### [PRIM-144] windy-postman
- **Atom/Composite:** Composite
- **Definition:** Windy postman: edge costs differ by direction. NP-hard even on undirected graphs; requires solving rural postman as subproblem.
- **Cost Model:** Rural postman: find minimum set of required edges to traverse; then solve windy CPP on augmented graph.
- **Real Wall:** Windy CPP models real-world routing where traversal cost differs by direction (hills, one-way streets).
- **Cross-Domain Aliases:** asymmetric-postman (combinatorial-optimization), directed-route (control-numerical-opt).
- **Notes:** Minieka (1979); windy CPP is NP-hard even for simple graphs; rural postman (required edges subset) is also NP-hard.

### [PRIM-145] rural-postman
- **Atom/Composite:** Composite
- **Definition:** Rural postman: traverse subset of required edges at minimum cost. NP-hard; reduced to Steiner tour problem.
- **Cost Model:** Reduced to Steiner traveling salesman: O(2^r · poly(n)) for r required edges; practical heuristics.
- **Real Wall:** Rural postman = CPP with required edges; arises in snow plowing, street sweeping.
- **Cross-Domain Aliases:** required-edge-route (combinatorial-optimization), subset-postman (control-numerical-opt).
- **Notes:** Eiselt et al. (1995); rural CPP has required edges (must be traversed) vs. optional edges (may be traversed).

### [PRIM-146] network-design-approx
- **Atom/Composite:** Composite
- **Definition:** Network design approximation: primal-dual algorithms for connected subgraph with minimum cost. 2-approx for survivable design.
- **Cost Model:** Primal-dual: O(E·log V) per requirement; 2-approx for edge-connectivity SNDP; bicriteria approximation also studied.
- **Real Wall:** Network design is central to telecommunications; primal-dual is the practical approximation method.
- **Cross-Domain Aliases:** connected-subgraph (combinatorial-optimization), network-budget (control-numerical-opt).
- **Notes:** Goemans & Williamson (1995); primal-dual for network design; 2-approx is the standard result.

### [PRIM-147] covering-ip
- **Atom/Composite:** Primitive
- **Definition:** Set cover IP: minimize Σ_c x_c subject to Σ_{e∈c} x_c ≥ 1 for each element. LP relaxation gives ln(n) bound.
- **Cost Model:** LP relaxation: O(n·m) for n elements, m sets; primal-dual gives 2-approx; hardness of ln(n).
- **Real Wall:** Set cover is the canonical covering problem; LP relaxation + rounding is the textbook approximation approach.
- **Cross-Domain Aliases:** set-cover-relaxation (control-numerical-opt), covering-ip (combinatorial-optimization).
- **Notes:** Feige (1998); set cover has integrality gap = ln(n) and hardness of (1-α)·ln(n) for any α.

### [PRIM-148] packing-ip
- **Atom/Composite:** Primitive
- **Definition:** Packing IP: maximize Σ_v x_v subject to Σ_{e∈v} x_v ≤ b_e. LP relaxation gives 1 bound (integer polyhedron for bipartite).
- **Cost Model:** Packing = covering dual; bipartite packing is totally unimodular → LP solves integer.
- **Real Wall:** Packing problems are generally harder than covering; bipartite packing = assignment generalization.
- **Cross-Domain Aliases:** independent-set-ip (control-numerical-opt), packing-relaxation (combinatorial-optimization).
- **Notes:** Packing and covering are dual problems; LP dual of set cover is maximum weight set packing with unit weights.

### [PRIM-149] zero-one-matrix
- **Atom/Composite:** Primitive
- **Definition:** Packing of 0-1 matrices: consecutive-ones property. PQ-trees for interval graphs; consecutive-ones test is polynomial.
- **Cost Model:** PQ-tree: O(n·m) for n rows, m columns; used for planar graph embedding, physical mapping.
- **Real Wall:** Consecutive-ones property appears in physical mapping (DNA fragments), scheduling (interval orders).
- **Cross-Domain Aliases:** consecutive-ones (combinatorial-optimization), interval-graph (computational-geometry).
- **Notes:** Booth & Lueker (1976); PQ-tree encodes all permutations making 1s consecutive; linear-time algorithm.

### [PRIM-150] multicut
- **Atom/Composite:** Composite
- **Definition:** Multicut: separate terminal pairs by cutting edges. Minimum multicut in trees is polynomial; in general graphs NP-hard.
- **Cost Model:** Trees: O(n) via greedy; graphs: O(V·E·log V) for fixed k pairs; FPT in k = multicut parameterized by number of pairs.
- **Real Wall:** Multicut models network separation; parameterized by number of pairs = FPT.
- **Cross-Domain Aliases:** terminal-separation (combinatorial-optimization), multicut-ip (control-numerical-opt).
- **Notes:** Garg et al. (1996); multicut in trees is polynomial; multicut with k terminal pairs is FPT.

### [PRIM-151] treewidth-approximation
- **Atom/Composite:** Composite
- **Definition:** Treewidth approximation: find upper bound via fill-in, lower bound via minors. Minor-min-width, branchwidth.
- **Cost Model:** Heuristic upper bound: minimum fill O(n³), maximum cardinality search O(n+m); lower bound: grid minor width.
- **Real Wall:** Treewidth approximation guides algorithm selection; graphs of small treewidth are tractable for many problems.
- **Cross-Domain Aliases:** treewidth-heuristic (control-numerical-opt), graph-decomposition (combinatorial-optimization).
- **Notes:** Bodlaender (1993); treewidth computation is NP-hard but heuristic upper bound is practical.

### [PRIM-152] branch-and-price-vrp
- **Atom/Composite:** Composite
- **Definition:** Branch-and-price for VRP: column generation for pricing subproblem (shortest path with resource constraints).
- **Cost Model:** SPPRC (Shortest Path Problem with Resource Constraints): O(n·states) via DP; state-space expansion is the bottleneck.
- **Real Wall:** B&P is the state-of-the-art for large VRP instances; pricing subproblem complexity determines tractability.
- **Cross-Domain Aliases:** vrp-column-gen (control-numerical-opt), routing-master (combinatorial-optimization).
- **Notes:** Desrochers et al. (1992); SPPRC pricing with time windows + capacity; solves CVRP instances with 100+ customers.

### [PRIM-153] iterative-rounding
- **Atom/Composite:** Composite
- **Definition:** Iterative rounding: round LP solution, fix variables, re-solve. Used for matroid intersection, network design.
- **Cost Model:** O(log n) iterations typically; each iteration solves LP; integral solution reached by round-fixing.
- **Real Wall:** Iterative rounding gives 1.5-approx for network design; simpler and more practical than primal-dual.
- **Cross-Domain Aliases:** round-and-fix (control-numerical-opt), lp-iterative (combinatorial-optimization).
- **Notes:** Jain (2001); iterative rounding is a powerful technique; generalizes matroid intersection and network design.

### [PRIM-154] randomized-rounding
- **Atom/Composite:** Composite
- **Definition:** Randomized rounding: round LP solution x̂ to integer x with P(x_e=1) = x̂_e. Chernoff bound gives concentration.
- **Cost Model:** O(1) per variable to round; Chernoff: P(|X - E[X]| ≥ ε·E[X]) ≤ 2·exp(-ε²·E[X]/3).
- **Real Wall:** Randomized rounding works for covering/packing when variables are independent enough; correlation rounding improves.
- **Cross-Domain Aliases:** probabilistic-rounding (control-numerical-opt), chernoff-bound (combinatorial-optimization).
- **Notes:** Raghavan & Thompson (1987); randomized rounding with Chernoff bounds gives approximation for set cover.

### [PRIM-155] discrepancy-theory
- **Atom/Composite:** Composite
- **Definition:** Discrepancy theory: color elements of a set system to minimize imbalance. Beck-Fiala O(t·log n) discrepancy.
- **Cost Model:** Beck-Fiala: O(n) to color with max t overlaps; partial coloring + iterative method.
- **Real Wall:** Discrepancy theory applied to rounding, sampling, derandomization; geometric discrepancy has applications.
- **Cross-Domain Aliases:** coloring-bounds (combinatorial-optimization), partial-coloring (control-numerical-opt).
- **Notes:** Beck & Fiala (1981); discrepancy theory gives rounding guarantees; partial coloring is the key technique.

### [PRIM-156] network-flow-decomposition
- **Atom/Composite:** Composite
- **Definition:** Flow decomposition: decompose flow into paths and cycles. At most E paths in O(E·V) time.
- **Cost Model:** Decomposition O(E·V); cycles cancel each other out; paths are the essential flow units.
- **Real Wall:** Flow decomposition is key for network flow algorithms; enables path-based algorithms (shortest path routing).
- **Cross-Domain Aliases:** path-decomposition (distributed-systems), flow-paths (combinatorial-optimization).
- **Notes:** Edmonds & Karp (1972); flow decomposition shows any feasible flow = path flow + cycle flow.

### [PRIM-157] cost-scaling-push-relabel
- **Atom/Composite:** Composite
- **Definition:** Cost scaling (Cancel-and-Tighten): min-cost flow with capacity scaling. O(E·log(C·U)) for integral capacities.
- **Cost Model:** Cost scaling O(E·log(C·U)) = strongly polynomial O(E·log V·log U); faster in practice than successive shortest path.
- **Real Wall:** Cost scaling is the fastest min-cost flow algorithm in practice; Orlin's O(E·log V) is asymptotically optimal.
- **Cross-Domain Aliases:** mincost-scaling (control-numerical-opt), scaling-algorithm (combinatorial-optimization).
- **Notes:** Goldberg & Tarjan (1988); cost scaling outperforms other min-cost flow algorithms on practical instances.

### [PRIM-158] capacity-scaling
- **Atom/Composite:** Primitive
- **Definition:** Capacity scaling: scale capacities by powers of 2, refine in each phase. Reduces max-flow complexity.
- **Cost Model:** Capacity scaling O(E·log U·min(V^{2/3}, √E)) for max flow; log U phases × O(E·min(...)) per phase.
- **Real Wall:** Capacity scaling is simple and practical; faster than Edmonds-Karp for dense graphs.
- **Cross-Domain Aliases:** scaling-maxflow (control-numerical-opt), flow-scaling (combinatorial-optimization).
- **Notes:** Edmonds-Karp is capacity scaling with delta = 1; general delta scaling improves to O(E·min(V^{2/3}, √E)·log U).

### [PRIM-159] scheduling-open-shop
- **Atom/Composite:** Composite
- **Definition:** Open shop: jobs need each machine for some duration, order is irrelevant. O|n| |C_max polynomial (Gonzalez & Sahni).
- **Cost Model:** Open shop has polynomial algorithm; each job visits each machine once in arbitrary order.
- **Real Wall:** Open shop is easier than job shop (no precedence on machines); makespan minimization is polynomial.
- **Cross-Domain Aliases:** permutation-open-shop (control-numerical-opt), flexible-shop (combinatorial-optimization).
- **Notes:** Gonzalez & Sahni (1976); open shop is the easiest shop problem; flow shop (fixed order) is harder.

### [PRIM-160] scheduling-with-setups
- **Atom/Composite:** Composite
- **Definition:** Scheduling with setups: sequence-dependent setup times. Makespan is NP-hard even on single machine.
- **Cost Model:** TSP reduction: machine = traveling salesman, jobs = cities, setup = travel time. Approximation via TSP heuristics.
- **Real Wall:** Setup times are common in manufacturing (tool changes, cleaning); TSP reduction enables heuristics.
- **Cross-Domain Aliases:** sequence-dependent-setup (control-numerical-opt), setup-matrix (combinatorial-optimization).
- **Notes:** Allahverdi et al. (2008); setup scheduling is a major research area; heuristics and FPTAS exist for special cases.

### [PRIM-161] multiprocessor scheduling-theory
- **Atom/Composite:** Composite
- **Definition:** Multiprocessor scheduling: P|prec|C_max (parallel machines, precedence, makespan). Fully polynomial time approximation scheme (FPTAS).
- **Cost Model:** FPTAS: O(n·(1/ε)^{1/ε}) for P|prec|C_max; pseudopolynomial in n; practical PTAS also available.
- **Real Wall:** P|prec|C_max is NP-hard but has FPTAS; precedence constraints add complexity.
- **Cross-Domain Aliases:** parallel-machine-approx (control-numerical-opt), precedence-scheduling (combinatorial-optimization).
- **Notes:** Lawler (1978); FPTAS for P|prec|C_max uses DP with state pruning; approximation scheme is best possible.

### [PRIM-162] approximation-preserving-reduction
- **Atom/Composite:** Composite
- **Definition:** AP-reduction: reduction preserving approximation ratio. L-reductions preserve approximation factors.
- **Cost Model:** AP-reduction: transform instance, solve, map back; preserves constant-factor approximation.
- **Real Wall:** AP-reductions enable approximation-preserving transformations between problems; show hardness of approximation.
- **Cross-Domain Aliases:** approx-reduction (control-numerical-opt), hardness-mapping (combinatorial-optimization).
- **Notes:** Crescenzi et al. (1995); AP-reduction is the standard reduction for approximation-preserving mappings.

### [PRIM-163] label-setting-mdp
- **Atom/Composite:** Composite
- **Definition:** Label setting: solve shortest path in DAG by topological order. O(E) for acyclic graphs; dynamic programming.
- **Cost Model:** Topological sort O(V+E); relax edges in order O(E); no cycles to handle.
- **Real Wall:** Label setting is simpler than Dijkstra; only applicable when graph is acyclic (DAG).
- **Cross-Domain Aliases:** dag-shortest-path (combinatorial-optimization), topological-relaxation (control-numerical-opt).
- **Notes:** DAG shortest path is the simplest shortest path problem; used as subroutine in many algorithms.

### [PRIM-164] dijkstra-with-attributes
- **Atom/Composite:** Composite
- **Definition:** Dijkstra with state (label) expansion: label = (node, time, load, ...). Resource-constrained shortest path.
- **Cost Model:** State space size O(V·D·R) for D time steps, R resource levels; dominates labeling algorithms.
- **Real Wall:** Resource-constrained shortest path is the pricing subproblem for VRP with time windows and capacity.
- **Cross-Domain Aliases:** rcspp (combinatorial-optimization), constrained-shortest-path (control-numerical-opt).
- **Notes:** Irnich & Desaulniers (2005); RCSPP = shortest path with resource constraints; used in branch-and-price.

### [PRIM-165] label-setting-algorithms
- **Atom/Composite:** Composite
- **Definition:** Label setting algorithms: multi-criteria shortest path, resource-constrained shortest path. Pareto-optimal labels.
- **Cost Model:** Label dominance: prune dominated labels; number of Pareto-optimal labels can be exponential.
- **Real Wall:** Label setting is the most general shortest path framework; used for multi-criteria routing, stochastic shortest path.
- **Cross-Domain Aliases:** multi-criteria-path (combinatorial-optimization), label-dominance (control-numerical-opt).
- **Notes:** Dial et al. (1980); SWISS method for label-setting with monotone cost functions.

### [PRIM-166] column-generation-pricing
- **Atom/Composite:** Composite
- **Definition:** Pricing subproblem: find column with negative reduced cost. Must be solvable efficiently for column generation to work.
- **Cost Model:** Pricing complexity determines column generation tractability; must be polynomial or pseudo-polynomial.
- **Real Wall:** Effective pricing subproblem is key to successful column generation; heuristic pricing works in practice.
- **Cross-Domain Aliases:** negative-reduced-cost (control-numerical-opt), column-pricing (combinatorial-optimization).
- **Notes:** Desrosiers & Lübbecke (2005); pricing subproblem is the bottleneck of column generation; dual variables guide search.

### [PRIM-167] dual-ascent
- **Atom/Composite:** Composite
- **Definition:** Dual ascent: increase dual variables to improve dual bound. Used in primal-dual algorithms.
- **Cost Model:** Dual ascent O(n·m) per iteration; terminates when no improving direction exists.
- **Real Wall:** Dual ascent is simpler than subgradient for some problems; provides feasible dual solution.
- **Cross-Domain Aliases:** dual-improvement (control-numerical-opt), ascent-method (combinatorial-optimization).
- **Notes:** Balakrishnan et al. (1989); dual ascent is the continuous version of the primal-dual framework.

### [PRIM-168] matroid-union
- **Atom/Composite:** Composite
- **Definition:** Matroid union: find maximum cardinality set in union of k matroids. Edmonds' matroid union theorem.
- **Cost Model:** O(r·k·EO) for matroid rank r, k matroids, EO = oracle query; polynomial for any matroids.
- **Real Wall:** Matroid union generalizes many problems: matroid intersection (k=2), spanning forest (k=2 graphic matroids).
- **Cross-Domain Aliases:** union-matroid (combinatorial-optimization), matroid-combination (control-numerical-opt).
- **Notes:** Edmonds (1979); matroid union is polynomial; cardinality = sum of ranks - overlap; exchange argument proves optimality.

### [PRIM-169] matroid-intersection-algorithm
- **Atom/Composite:** Composite
- **Definition:** Matroid intersection algorithm: augment along alternating paths in exchange graph. O(r·n·EO) for n elements.
- **Cost Model:** Exchange graph construction O(n·EO); augment along augmenting path O(n); O(r·n·EO) total.
- **Real Wall:** Matroid intersection is the most general polynomial combinatorial optimization (given matroid oracles).
- **Cross-Domain Aliases:** alternating-path (control-numerical-opt), intersection-matroid (combinatorial-optimization).
- **Notes:** Edmonds (1979); matroid intersection algorithm is the prototype for many combinatorial algorithms.

### [PRIM-170] parity-constraints
- **Atom/Composite:** Primitive
- **Definition:** Parity constraints: x modulo 2 = 0 or 1. Parity IPs are NP-hard; parity polytope has exponential description.
- **Cost Model:** Parity constraint x ≡ 0 (mod 2) → add x_i variables; can be encoded as x_i + y_i = 1 and parity constraints.
- **Real Wall:** Parity constraints arise in tournament scheduling, graph orientation; NP-hard to optimize.
- **Cross-Domain Aliases:** modulo-constraint (combinatorial-optimization), parity-polytope (control-numerical-opt).
- **Notes:** Seymour (1979); parity constraints require exponential linear description; separation is NP-hard.

### [PRIM-171] multiobjective-combinatorial
- **Atom/Composite:** Composite
- **Definition:** Multi-objective combinatorial optimization: find all Pareto optimal solutions. Scalarization + Pareto filtering.
- **Cost Model:** Scalarization for m weight vectors: O(m·solve); Pareto frontier can be exponential in size.
- **Real Wall:** Pareto frontier size can be exponential; decision maker chooses among Pareto solutions.
- **Cross-Domain Aliases:** pareto-optimal-set (combinatorial-optimization), multi-objective-search (control-numerical-opt).
- **Notes:** Ehrgott (2005); multi-objective optimization is the realistic setting; Pareto frontier = compromise-free solutions.

### [PRIM-172] minmax-regret
- **Atom/Composite:** Composite
- **Definition:** Minmax regret: minimize worst-case regret under uncertainty. Regret = cost(decision) - optimal cost.
- **Cost Model:** Minmax regret is harder than nominal optimization; often NP-hard even when nominal problem is polynomial.
- **Real Wall:** Minmax regret is more robust than minmax; requires solving nominal problems for each scenario.
- **Cross-Domain Aliases:** robust-decision (control-numerical-opt), regret-minimization (combinatorial-optimization).
- **Notes:** Kouvelis & Yu (2013); minmax regret is the robust optimization criterion for decision-making under uncertainty.

### [PRIM-173] scheduling-without-waits
- **Atom/Composite:** Composite
- **Definition:** No-wait scheduling: jobs must be processed consecutively without waiting. Reduces to TSP in flow shop.
- **Cost Model:** No-wait flow shop = TSP: makespan = max over arcs of cumulative time; transform to asymmetric TSP.
- **Real Wall:** No-wait constraint occurs in chemical processing (temperature-sensitive jobs); reduces to TSP which is NP-hard.
- **Cross-Domain Aliases:** continuous-flow (control-numerical-opt), no-wait-shop (combinatorial-optimization).
- **Notes:** Reddi & Wilhelm (1972); no-wait flow shop reduces to TSP; allows efficient heuristic transfer.

### [PRIM-174] berge-lemma
- **Atom/Composite:** Primitive
- **Definition:** Berge's lemma: set of edges M is maximum matching iff no augmenting path exists.
- **Cost Model:** Augmenting path search O(E) per iteration; maximum cardinality matching algorithm uses Berge's lemma.
- **Real Wall:** Berge's lemma is the fundamental characterization; applies to bipartite and general matching.
- **Cross-Domain Aliases:** augmenting-path (combinatorial-optimization), matching-characterization (control-numerical-opt).
- **Notes:** Berge (1957); Berge's lemma is the combinatorial characterization of maximum matching.

### [PRIM-175] menger-theorem
- **Atom/Composite:** Primitive
- **Definition:** Menger's theorem: maximum number of vertex-disjoint s-t paths = minimum s-t vertex cut. Edge version: edge-disjoint paths = edge cut.
- **Cost Model:** Menger's theorem = max-flow min-cut for vertex/edge connectivity; s-t cut in directed graph.
- **Real Wall:** Menger's theorem provides dual characterization; used in network reliability, connectivity augmentation.
- **Cross-Domain Aliases:** connectivity-theorem (combinatorial-optimization), disjoint-paths (control-numerical-opt).
- **Notes:** Menger (1927); vertex-connectivity and edge-connectivity are fundamental graph parameters.

### [PRIM-176] menger-max-flow
- **Atom/Composite:** Composite
- **Definition:** Menger via max-flow: transform vertex connectivity to edge connectivity via vertex splitting. Split each vertex v into v_in, v_out.
- **Cost Model:** Split graph: O(2V) vertices, O(E + 2V) edges; max flow gives vertex-disjoint paths.
- **Real Wall:** Vertex splitting is the standard reduction for Menger's theorem; applies to both directed and undirected graphs.
- **Cross-Domain Aliases:** vertex-splitting (combinatorial-optimization), connectivity-flow (control-numerical-opt).
- **Notes:** Menger's theorem is equivalent to max-flow min-cut; the vertex splitting trick makes it computable.

### [PRIM-177] ear-decomposition
- **Atom/Composite:** Composite
- **Definition:** Ear decomposition: decompose 2-edge-connected graph into cycles and paths (ears). Used for traveling salesman reduction.
- **Cost Model:** Ear decomposition O(E·V) construction; ear = simple path from vertex to vertex with endpoints already in decomposition.
- **Real Wall:** Ear decomposition shows 2-edge-connected graph = cycle + ears; used in TSP analysis and algorithms.
- **Cross-Domain Aliases:** ear-decomposition (combinatorial-optimization), ear-cycle (control-numerical-opt).
- **Notes:** Whitney (1932); ear decomposition is a fundamental structural tool; shows every 2-edge-connected graph has ear decomposition.

### [PRIM-178] tsptw
- **Atom/Composite:** Composite
- **Definition:** TSP with Time Windows (TSPTW): TSP + time window constraints on visits. CP + branch-and-bound dominates.
- **Cost Model:** Time windows add feasibility constraints; constraint propagation (time consistency) reduces search.
- **Real Wall:** TSPTW is the practical routing problem with delivery windows; highly constrained in practice.
- **Cross-Domain Aliases:** tsp-with-windows (combinatorial-optimization), time-constrained-tsp (control-numerical-opt).
- **Notes:** Dumas et al. (1995); TSPTW is NP-hard; local search + CP is the practical approach.

### [PRIM-179] asymmetric-tsp
- **Atom/Composite:** Composite
- **Definition:** ATSP (Asymmetric TSP): directed graph with asymmetric distances. NP-hard; reduction to ATSP from standard TSP.
- **Cost Model:** ATSP reduction to ATSP with triangle inequality (ATSP) O(n³) transformation; general ATSP is harder.
- **Real Wall:** ATSP arises in routing problems with different costs for direction (one-way streets, toll roads).
- **Cross-Domain Aliases:** directed-tsp (combinatorial-optimization), asymmetric-cost (control-numerical-opt).
- **Notes:** ATSP is APX-hard (no constant approx unless P=NP); metric ATSP has 0.55-approx (Kale, 2000).

### [PRIM-180] ptas-for-bins
- **Atom/Composite:** Composite
- **Definition:** PTAS for bin packing: asymptotic PTAS (APTAS) by Karmarkar & Karp. Partition into small and large items, round sizes.
- **Cost Model:** APTAS: O(n·(1/ε)^{O(1)}) time; produces solution with OPT + O(1) bins.
- **Real Wall:** APTAS is the best theoretical result for bin packing; practical FFD/BFD are within 11/9·OPT.
- **Cross-Domain Aliases:** asymptotic-ptas (control-numerical-opt), bin-approx (combinatorial-optimization).
- **Notes:** Karmarkar & Karp (1982); APTAS uses linear programming relaxation + rounding; integrality gap of bin packing LP is bounded.

## 2. Graph Cuts and Connectivity

### [PRIM-181] stoer-wagner-mincut
- **Atom/Composite:** Composite
- **Definition:** Stoer-Wagner global minimum cut algorithm: iteratively merge most-tightly-connected vertex pair, record cut-of-the-phase.
- **Cost Model:** O(V·E + V²·log V) using Fibonacci heaps; deterministic, no source/sink needed.
- **Real Wall:** Slower than Karger's randomized in expectation, but deterministic and simple; dominated by maximum adjacency ordering.
- **Cross-Domain Aliases:** global-min-cut (combinatorial-optimization), minimum-cut-phase (networking).
- **Notes:** Stoer & Wagner (1997); avoids max-flow entirely; works for undirected weighted graphs with non-negative weights.

### [PRIM-182] karger-randomized-mincut
- **Atom/Composite:** Composite
- **Definition:** Karger's algorithm: repeatedly contract a uniformly random edge until two vertices remain; cut between them is candidate min-cut.
- **Cost Model:** Single run O(V²); success probability ≥ 2/(V(V-1)); O(V² log V) repetitions for high probability.
- **Real Wall:** Las Vegas algorithm: must repeat to amplify success; Karger-Stein recursive variant runs in O(V² log³ V).
- **Cross-Domain Aliases:** edge-contraction (graphics-rendering-lod), random-contraction (statistics-probability).
- **Notes:** Karger (1993), Karger-Stein (1996); first algorithm to break O(V·E) for min-cut; uses contraction primitive.

### [PRIM-183] gomory-hu-tree
- **Atom/Composite:** Composite
- **Definition:** Gomory-Hu tree: tree on V vertices where minimum s-t cut in graph equals minimum edge weight on s-t path in tree, for all pairs.
- **Cost Model:** V-1 max-flow computations; O(V · MaxFlow); encodes all (V choose 2) pairwise min-cuts in O(V) space.
- **Real Wall:** Requires V-1 max-flow calls; Gusfield's simplification avoids contractions but still needs V-1 flows.
- **Cross-Domain Aliases:** all-pairs-mincut (networking), cut-tree (distributed-systems).
- **Notes:** Gomory & Hu (1961); fundamental data structure for network reliability and clustering.

### [PRIM-184] boykov-kolmogorov-maxflow
- **Atom/Composite:** Composite
- **Definition:** Boykov-Kolmogorov max-flow: augmenting-path algorithm tailored for vision graphs (grid-structured, low connectivity).
- **Cost Model:** Empirically O(V·E) on vision graphs; maintains source/sink search trees to reuse path information.
- **Real Wall:** Worst-case poor but dominates on computer vision applications (image segmentation, stereo); reused trees amortize.
- **Cross-Domain Aliases:** vision-mincut (graphics-rendering-lod), image-graph-cut (signal-processing).
- **Notes:** Boykov & Kolmogorov (2004); standard for energy minimization in computer vision; reuses tree across reparametrizations.

### [PRIM-185] push-relabel-flow
- **Atom/Composite:** Composite
- **Definition:** Push-relabel max-flow: maintain preflow, push excess along admissible edges, relabel heights when stuck.
- **Cost Model:** Generic O(V²·E); FIFO selection O(V³); highest-label O(V²·√E); fastest in practice for dense graphs.
- **Real Wall:** Implementation-sensitive; gap relabeling and global relabeling heuristics critical for performance.
- **Cross-Domain Aliases:** preflow-push (control-numerical-opt), excess-pushing (networking).
- **Notes:** Goldberg & Tarjan (1988); dominates Dinic for dense graphs; basis for HIPR reference implementation.

### [PRIM-186] dinic-blocking-flow
- **Atom/Composite:** Composite
- **Definition:** Dinic's algorithm: build level graph via BFS, find blocking flow in level graph via DFS, repeat until no s-t path.
- **Cost Model:** O(V²·E) general; O(E·√V) for unit-capacity / bipartite matching; O(E·V^{2/3}) for unit graphs.
- **Real Wall:** Blocking flow phase dominates; link-cut trees give O(V·E·log V); link-cut overhead rarely pays off.
- **Cross-Domain Aliases:** level-graph-flow (combinatorial-optimization), layered-augmentation (networking).
- **Notes:** Dinic (1970); independently discovered by Edmonds-Karp variant; fastest classical strongly-polynomial max-flow.

### [PRIM-187] isap-flow
- **Atom/Composite:** Composite
- **Definition:** ISAP (Improved Shortest Augmenting Path): max-flow via shortest augmenting path with gap relabeling and distance labels.
- **Cost Model:** O(V²·E) worst case; often fastest in practice on competitive-programming benchmarks.
- **Real Wall:** Performance depends heavily on graph structure; sensitive to relabeling strategy and gap heuristic.
- **Cross-Domain Aliases:** shortest-augmenting-path (combinatorial-optimization), distance-labeled-flow (networking).
- **Notes:** Ahuja, Magnanti & Orlin (1989); refinement of Edmonds-Karp using vertex distance labels.

### [PRIM-188] parametric-max-flow
- **Atom/Composite:** Composite
- **Definition:** Parametric max-flow: source/sink capacities are functions of parameter λ; trace optimal cut as λ varies.
- **Cost Model:** Gallo-Grigoriadis-Tarjan O(V·E·log(V²/E)): same complexity as a single max-flow for all λ values.
- **Real Wall:** Requires monotone capacity changes; non-monotone needs full recomputation per breakpoint.
- **Cross-Domain Aliases:** parametric-flow (control-numerical-opt), homotopy-flow (statistics-probability).
- **Notes:** Gallo, Grigoriadis & Tarjan (1989); used for selection / minimum-ratio problems, density-based clustering.

### [PRIM-189] multiway-cut
- **Atom/Composite:** Composite
- **Definition:** Multiway cut: remove edges to disconnect k terminal vertices from each other; NP-hard for k ≥ 3.
- **Cost Model:** Isolating cuts give 2(1-1/k)-approximation; Călinescu-Karloff-Rabani LP rounding gives 1.5-1/k.
- **Real Wall:** Polynomial only for k=2 (s-t min-cut); APX-hard for k ≥ 3; best known approx ratio 1.2965 (Sharma-Vondrák).
- **Cross-Domain Aliases:** terminal-separation (graphics-rendering-lod), k-terminal-cut (networking).
- **Notes:** Dahlhaus et al. (1994); Călinescu-Karloff-Rabani (2000); arises in computer vision multi-label problems.

### [PRIM-190] k-cut-problem
- **Atom/Composite:** Composite
- **Definition:** Minimum k-cut: partition V into k non-empty parts minimizing total inter-part edge weight; no specified terminals.
- **Cost Model:** Saran-Vazirani 2(1-1/k)-approximation; Karger-Stein extends to O(V^{2k-2}) exact; FPT in k via tree-cut.
- **Real Wall:** Exact algorithm is polynomial for fixed k but exponential in k; W[1]-hard parametrized by k under standard assumptions.
- **Cross-Domain Aliases:** k-partition (distributed-systems), multi-cut (combinatorial-optimization).
- **Notes:** Saran & Vazirani (1995); Goldschmidt-Hochbaum O(V^{k²/2}) exact; thorpe LP rounding 2-approx.

### [PRIM-191] minimum-multicut
- **Atom/Composite:** Composite
- **Definition:** Minimum multicut: given k source-sink pairs, remove minimum edge set so each pair is separated; LP relaxation = multicommodity flow dual.
- **Cost Model:** O(log k)-approximation via region growing on LP solution; APX-hard.
- **Real Wall:** Multicut and max multicommodity flow gap is Ω(log k); region growing tight up to constants.
- **Cross-Domain Aliases:** pair-separation (networking), multicommodity-cut (combinatorial-optimization).
- **Notes:** Garg, Vazirani & Yannakakis (1996); region-growing LP rounding is the canonical technique.

### [PRIM-192] max-cut-sdp
- **Atom/Composite:** Composite
- **Definition:** Goemans-Williamson max-cut: SDP relaxation places vertices on unit sphere, random hyperplane separates.
- **Cost Model:** SDP solve O(V^{3.5}) interior point; randomized rounding gives 0.878·OPT approximation in expectation.
- **Real Wall:** Khot's UGC implies 0.878 is optimal; SDP is bottleneck; first-order methods (Burer-Monteiro) scale better.
- **Cross-Domain Aliases:** semidefinite-rounding (linear-algebra-matrix), hyperplane-rounding (statistics-probability).
- **Notes:** Goemans & Williamson (1995); landmark result for SDP-based approximation; α_GW ≈ 0.87856.

### [PRIM-193] k-edge-connectivity
- **Atom/Composite:** Composite
- **Definition:** Minimum k edges whose removal disconnects graph; computed via V-1 max-flow calls (Gomory-Hu) or matoid partition.
- **Cost Model:** Gabow O(V·E + V²·log V); Karger randomized Õ(E) for global edge connectivity.
- **Real Wall:** k-edge-connectivity ≤ minimum degree; computing exact value of k expensive; useful for network reliability.
- **Cross-Domain Aliases:** edge-connectivity (distributed-systems), graph-robustness (networking).
- **Notes:** Gabow (1991); Karger Õ(E) randomized; foundation for survivable network design.

### [PRIM-194] k-vertex-connectivity
- **Atom/Composite:** Composite
- **Definition:** Minimum vertices whose removal disconnects graph; computed by vertex splitting + edge connectivity per pair.
- **Cost Model:** Henzinger-Rao-Gabow O(min(k³+V, kV)·E); harder than edge connectivity in general.
- **Real Wall:** No known Õ(E) algorithm; vertex-disjoint paths via Menger's theorem; bottleneck is pairwise max-flow.
- **Cross-Domain Aliases:** node-connectivity (networking), vertex-cut-number (combinatorial-optimization).
- **Notes:** Henzinger, Rao & Gabow (2000); k-vertex-connected graphs are k-resistant to node failures.

### [PRIM-195] cut-tree-gusfield
- **Atom/Composite:** Composite
- **Definition:** Gusfield's simplified Gomory-Hu construction: V-1 max-flow calls without graph contractions.
- **Cost Model:** O(V · MaxFlow); avoids the contraction step of original Gomory-Hu; same asymptotic complexity.
- **Real Wall:** Same V-1 flow calls bottleneck; conceptually simpler than Gomory-Hu original.
- **Cross-Domain Aliases:** all-pairs-cut-tree (combinatorial-optimization), simplified-gh (networking).
- **Notes:** Gusfield (1990); standard implementation of all-pairs min-cut; uses non-crossing s-t pair selection.

## 3. Network Flow Refinements

### [PRIM-196] ford-fulkerson
- **Atom/Composite:** Primitive
- **Definition:** Ford-Fulkerson method: while augmenting path exists in residual graph, push flow along it; family of max-flow algorithms.
- **Cost Model:** O(E·|f*|) with arbitrary path selection; pseudo-polynomial in flow value; not strongly polynomial.
- **Real Wall:** With irrational capacities may not terminate; integer capacities terminate but slowly with bad path choice.
- **Cross-Domain Aliases:** augmenting-path-method (control-numerical-opt), max-flow-template (networking).
- **Notes:** Ford & Fulkerson (1956); foundational framework; specific implementations (Edmonds-Karp, Dinic) give polynomial bounds.

### [PRIM-197] edmonds-karp
- **Atom/Composite:** Composite
- **Definition:** Edmonds-Karp: Ford-Fulkerson with BFS augmenting paths (shortest paths in residual graph by edges).
- **Cost Model:** O(V·E²) strongly polynomial; each augmenting path found in O(E) via BFS; O(V·E) augmentations.
- **Real Wall:** Slower than Dinic in practice; pedagogically clean; first strongly polynomial max-flow.
- **Cross-Domain Aliases:** bfs-augmentation (combinatorial-optimization), shortest-augmenting-path (networking).
- **Notes:** Edmonds & Karp (1972); proved strongly polynomial bound via BFS path length monotonicity.

### [PRIM-198] cycle-canceling-mincost-flow
- **Atom/Composite:** Composite
- **Definition:** Cycle canceling: start with feasible flow, find negative-cost cycle in residual graph, cancel until none exist.
- **Cost Model:** O(V·E²·CU) with minimum-mean cycle; Goldberg-Tarjan O(V·E²·log(V·C)) for integer costs.
- **Real Wall:** Negative cycle detection (Bellman-Ford) per iteration; minimum-mean cycle gives strongly polynomial bound.
- **Cross-Domain Aliases:** negative-cycle-elimination (control-numerical-opt), residual-cycle-cancel (networking).
- **Notes:** Klein (1967); Goldberg-Tarjan (1989) minimum-mean cycle canceling; conceptually clean min-cost flow.

### [PRIM-199] successive-shortest-path
- **Atom/Composite:** Composite
- **Definition:** SSP min-cost flow: repeatedly augment along shortest (cost) path in residual graph; uses potentials to handle negative costs.
- **Cost Model:** O(F·SSSP) for total flow F; with Dijkstra+potentials O(F·(E + V·log V)).
- **Real Wall:** Pseudo-polynomial in F; scaling variants give strongly polynomial bounds (Orlin O(E·log V·(E+V·log V))).
- **Cross-Domain Aliases:** dijkstra-flow (combinatorial-optimization), shortest-cost-path-flow (control-numerical-opt).
- **Notes:** Jewell (1958), Busacker-Gowen (1961); foundation for capacity scaling variants.

### [PRIM-200] network-simplex
- **Atom/Composite:** Composite
- **Definition:** Network simplex: specialized simplex method for min-cost flow exploiting basis = spanning tree structure.
- **Cost Model:** O(V·E·log V·C) Orlin's strongly polynomial; in practice extremely fast (CPLEX, LEMON).
- **Real Wall:** Cycling under degeneracy; anti-cycling rules (Cunningham, Bland) ensure termination.
- **Cross-Domain Aliases:** specialized-simplex (control-numerical-opt), tree-pivot-simplex (combinatorial-optimization).
- **Notes:** Dantzig (1963); Orlin (1997) strongly polynomial; dominates other min-cost flow algorithms in practice.

### [PRIM-201] capacity-scaling-flow
- **Atom/Composite:** Composite
- **Definition:** Capacity scaling: consider only edges with capacity ≥ Δ, halve Δ each phase; works for max-flow and min-cost flow.
- **Cost Model:** O(E²·log U) for max-flow with scaling; O((E·log U)·SSSP) for min-cost.
- **Real Wall:** Practical when capacities span orders of magnitude; reduces work on small-capacity edges.
- **Cross-Domain Aliases:** scaling-augmentation (combinatorial-optimization), capacity-phase (networking).
- **Notes:** Gabow (1985), Edmonds-Karp original; used in Orlin's strongly polynomial min-cost flow.

### [PRIM-202] multicommodity-flow
- **Atom/Composite:** Composite
- **Definition:** Multi-commodity flow: route k commodities through shared network with capacity constraints; LP-feasible but NP-hard integral.
- **Cost Model:** LP O((V·E)^{O(1)}); approximate via FPTAS (Garg-Könemann) in Õ(E²·ε⁻²).
- **Real Wall:** Integer multicommodity flow is NP-hard even with 2 commodities; LP relaxation has Ω(log V) integrality gap.
- **Cross-Domain Aliases:** k-commodity-routing (networking), shared-flow (distributed-systems).
- **Notes:** Ford-Fulkerson (1958); Garg-Könemann FPTAS (1998); foundational in network design and traffic engineering.

### [PRIM-203] generalized-flow
- **Atom/Composite:** Composite
- **Definition:** Generalized flow: each edge has gain factor γ_e; flow at head = γ_e · flow at tail; models lossy/gain networks.
- **Cost Model:** Polynomial: Truemper O(V²·E·log V·log(VB)); Wayne O(E²·V·log²(V)) combinatorial.
- **Real Wall:** Lossy flows (γ < 1) common in financial / currency arbitrage; algorithms more complex than standard flow.
- **Cross-Domain Aliases:** lossy-flow (networking), currency-arbitrage (control-numerical-opt).
- **Notes:** Truemper (1977); Wayne (2002); used in models with exchange rates, leakage, gain.

### [PRIM-204] flow-decomposition
- **Atom/Composite:** Primitive
- **Definition:** Flow decomposition: any feasible flow f can be expressed as sum of at most E paths and cycles.
- **Cost Model:** O(V·E) greedy decomposition; each path/cycle extraction O(V+E).
- **Real Wall:** Decomposition not unique; useful for analysis (routing interpretation) and post-processing.
- **Cross-Domain Aliases:** path-decomposition (combinatorial-optimization), flow-path-extraction (networking).
- **Notes:** Ford-Fulkerson (1956); foundational structural theorem for network flows.

## 4. Matching Algorithms

### [PRIM-205] hungarian-algorithm
- **Atom/Composite:** Composite
- **Definition:** Hungarian / Kuhn-Munkres: weighted bipartite matching via dual updates and augmenting paths.
- **Cost Model:** O(V³) classical; O(V·E + V²·log V) with Fibonacci heaps for sparse weighted bipartite.
- **Real Wall:** Dense matrices give O(V³); JV variant (Jonker-Volgenant) outperforms in practice by constant factors.
- **Cross-Domain Aliases:** assignment-algorithm (control-numerical-opt), kuhn-munkres (linear-algebra-matrix).
- **Notes:** Kuhn (1955), Munkres (1957); named after König and Egerváry; standard for the assignment problem.

### [PRIM-206] hopcroft-karp
- **Atom/Composite:** Composite
- **Definition:** Hopcroft-Karp: maximum cardinality bipartite matching via multiple vertex-disjoint augmenting paths per phase.
- **Cost Model:** O(E·√V); √V phases each O(E); fastest classical bipartite matching algorithm.
- **Real Wall:** Phase-based; BFS to compute layer graph, then DFS to find disjoint augmenting paths.
- **Cross-Domain Aliases:** bipartite-matching (combinatorial-optimization), disjoint-augmenting (networking).
- **Notes:** Hopcroft & Karp (1973); matches Dinic on unit-capacity bipartite; uses augmenting-path-set primitive.

### [PRIM-207] edmonds-blossom
- **Atom/Composite:** Composite
- **Definition:** Edmonds' blossom algorithm: maximum matching in general (non-bipartite) graphs via blossom contraction.
- **Cost Model:** O(V·E·α(V)) Gabow; O(E·√V) Micali-Vazirani for unweighted; conceptually intricate.
- **Real Wall:** Blossom = odd cycle in alternating walk; contraction restores augmenting path structure.
- **Cross-Domain Aliases:** general-matching (combinatorial-optimization), blossom-contraction (graphics-rendering-lod).
- **Notes:** Edmonds (1965); landmark "Paths, Trees, and Flowers" paper; defined polynomial-time as tractability.

### [PRIM-208] weighted-matching-blossom
- **Atom/Composite:** Composite
- **Definition:** Edmonds' weighted matching: extends blossom algorithm with dual variables; O(V³) for weighted general matching.
- **Cost Model:** O(V·E·log V) Galil-Micali-Gabow; O(V·(E + V·log V)) Gabow's variant.
- **Real Wall:** Practical implementations (LEMON, NetworkX) use the O(V³) variant for simplicity.
- **Cross-Domain Aliases:** weighted-blossom (combinatorial-optimization), max-weight-matching (control-numerical-opt).
- **Notes:** Edmonds (1965); Gabow (1990); foundational for assignment under structural constraints.

### [PRIM-209] micali-vazirani
- **Atom/Composite:** Composite
- **Definition:** Micali-Vazirani: maximum unweighted matching in general graphs in O(E·√V).
- **Cost Model:** O(E·√V); tight in matching-shaped problems; conceptually involves double-DFS and bridges.
- **Real Wall:** Notoriously hard to implement correctly; correctness proof took decades to clarify (Vazirani 2014).
- **Cross-Domain Aliases:** general-graph-matching (combinatorial-optimization), sqrt-v-matching (networking).
- **Notes:** Micali & Vazirani (1980); Vazirani (2012) detailed proof; theoretical best for unweighted general matching.

### [PRIM-210] online-bipartite-matching
- **Atom/Composite:** Composite
- **Definition:** Online bipartite matching: offline side known, online side arrives one-by-one and must be matched immediately.
- **Cost Model:** RANKING algorithm achieves (1-1/e) competitive ratio; tight under adversarial arrivals.
- **Real Wall:** Greedy gives only 1/2; RANKING permutes offline side and matches highest-rank free neighbor.
- **Cross-Domain Aliases:** online-matching (distributed-systems), ad-allocation (retrieval-search).
- **Notes:** Karp, Vazirani & Vazirani (1990); KVV is foundational for online ad allocation, AdWords.

### [PRIM-211] adwords-problem
- **Atom/Composite:** Composite
- **Definition:** AdWords / budgeted matching: bidders with budgets, queries arrive online; match query to bidder, deduct bid from budget.
- **Cost Model:** Mehta-Saberi-Vazirani-Vazirani (1-1/e) algorithm; tight under small-bid assumption.
- **Real Wall:** Without small-bid assumption, no better than 1/2; competitive ratio degrades with bid/budget ratio.
- **Cross-Domain Aliases:** budgeted-allocation (retrieval-search), online-ad-matching (distributed-systems).
- **Notes:** Mehta et al. (2007); foundational for sponsored search; trade-off function ψ(x) = 1 - e^{x-1}.

### [PRIM-212] stochastic-online-matching
- **Atom/Composite:** Composite
- **Definition:** Online matching under stochastic arrivals (i.i.d. or known distribution); beats adversarial bounds.
- **Cost Model:** Feldman-Mehta-Mirrokni-Muthukrishnan 0.67-competitive; later improved to 0.706 (Manshadi et al.).
- **Real Wall:** Real online matching often has predictable distribution; algorithms exploit prior knowledge.
- **Cross-Domain Aliases:** stochastic-arrival-matching (statistics-probability), iid-matching (combinatorial-optimization).
- **Notes:** Feldman et al. (2009); learning + matching hybrid algorithms for ad allocation.

### [PRIM-213] hospital-resident-matching
- **Atom/Composite:** Composite
- **Definition:** Hospital-resident / many-to-one stable matching: each hospital has quota; standard deferred-acceptance extended.
- **Cost Model:** O(n·m) deferred acceptance with capacities; preserves stability and strategy-proofness for residents.
- **Real Wall:** NRMP runs annually for U.S. medical residency; capacity violations and couples constraints add complexity.
- **Cross-Domain Aliases:** college-admission (control-numerical-opt), many-to-one-matching (combinatorial-optimization).
- **Notes:** Gale & Shapley (1962); Roth (1984) game-theoretic analysis; deployed by NRMP since 1952.

### [PRIM-214] kidney-exchange
- **Atom/Composite:** Composite
- **Definition:** Kidney exchange: find vertex-disjoint cycles and chains in directed compatibility graph maximizing matched pairs.
- **Cost Model:** NP-hard for cycle length ≥ 3; IP formulation with column generation scales to thousands of pairs.
- **Real Wall:** UNOS national kidney exchange uses branch-and-price; chains from non-directed donors extend matching.
- **Cross-Domain Aliases:** barter-exchange (control-numerical-opt), cycle-cover-matching (combinatorial-optimization).
- **Notes:** Roth, Sönmez & Ünver (2004); deployed at UNOS, NKR; barter exchange in economics.

### [PRIM-215] gallai-edmonds-decomposition
- **Atom/Composite:** Composite
- **Definition:** Gallai-Edmonds structure theorem: decompose graph into D (vertices missed by some max matching), A (neighbors of D), C (rest).
- **Cost Model:** O(V·E) via Edmonds' blossom; structural decomposition useful for matching combinatorics.
- **Real Wall:** Reveals all maximum matchings' structure; foundation for many derived algorithms.
- **Cross-Domain Aliases:** matching-structure (combinatorial-optimization), gallai-decomposition (control-numerical-opt).
- **Notes:** Gallai (1963), Edmonds (1965); D-A-C decomposition is canonical; D = essential, A = inessential, C = matched.

## 5. Integer Programming Refinements

### [PRIM-216] branch-and-cut
- **Atom/Composite:** Composite
- **Definition:** Branch-and-cut: branch-and-bound with cutting planes added at nodes; tightens LP relaxation before branching.
- **Cost Model:** Per-node: LP + cut generation (Gomory, MIR, cover, clique); exponential nodes worst case.
- **Real Wall:** Cut management critical: too many cuts slow LP, too few weaken bound; modern solvers prune aggressively.
- **Cross-Domain Aliases:** cut-and-branch (control-numerical-opt), branch-cut-search (combinatorial-optimization).
- **Notes:** Padberg & Rinaldi (1991); standard in CPLEX, Gurobi, SCIP; TSP record-breaking technique.

### [PRIM-217] branch-and-price
- **Atom/Composite:** Composite
- **Definition:** Branch-and-price: branch-and-bound with column generation at each node; for IPs with exponentially many variables.
- **Cost Model:** Per-node: master LP + pricing subproblem (often shortest path or knapsack); pricing can be NP-hard.
- **Real Wall:** Branching rules must preserve pricing structure; Ryan-Foster, follow-on branching common.
- **Cross-Domain Aliases:** column-branch (combinatorial-optimization), price-and-branch (control-numerical-opt).
- **Notes:** Barnhart et al. (1998); used in crew scheduling, vehicle routing, cutting stock.

### [PRIM-218] column-generation
- **Atom/Composite:** Composite
- **Definition:** Column generation: solve LP with restricted variables, generate new variable (column) with negative reduced cost via pricing problem.
- **Cost Model:** Iterate master LP + pricing; pricing subproblem solves Bellman/knapsack/shortest-path.
- **Real Wall:** Tail-off: last few columns improve little; stabilization (Wentges, du Merle) accelerates convergence.
- **Cross-Domain Aliases:** dantzig-wolfe-decomposition (control-numerical-opt), pricing-loop (combinatorial-optimization).
- **Notes:** Dantzig-Wolfe (1960); Gilmore-Gomory (1961) cutting stock; foundational for large-scale IP.

### [PRIM-219] dantzig-wolfe-decomposition
- **Atom/Composite:** Composite
- **Definition:** Dantzig-Wolfe: reformulate LP with block-diagonal structure via convex combinations of subproblem solutions.
- **Cost Model:** Master LP O(m_master) + subproblem solves; convergence in finite pivots.
- **Real Wall:** Subproblem must be efficiently solvable (often LP, network); coupling constraints become master.
- **Cross-Domain Aliases:** block-decomposition (control-numerical-opt), master-subproblem (combinatorial-optimization).
- **Notes:** Dantzig & Wolfe (1960); precursor to column generation; foundational decomposition method.

### [PRIM-220] benders-decomposition
- **Atom/Composite:** Composite
- **Definition:** Benders decomposition: project out subset of variables, replace with cuts derived from subproblem duals.
- **Cost Model:** Master IP + subproblem LP; subproblem dual produces optimality/feasibility cuts; iterate until tight.
- **Real Wall:** Slow tailing-off; modern variants (combinatorial Benders, logic-based Benders) accelerate.
- **Cross-Domain Aliases:** projection-decomposition (control-numerical-opt), cutting-plane-benders (combinatorial-optimization).
- **Notes:** Benders (1962); revisited for stochastic programming (L-shaped method); used in facility location.

### [PRIM-221] lagrangian-relaxation
- **Atom/Composite:** Composite
- **Definition:** Lagrangian relaxation: dualize hard constraints with multipliers, leaving tractable subproblem; bound via dual.
- **Cost Model:** Subgradient or bundle method updates multipliers; O(iterations·subproblem-solve).
- **Real Wall:** Lagrangian bound ≥ LP bound when relaxed constraints don't have integrality property; gap closes via subgradient.
- **Cross-Domain Aliases:** dual-relaxation (control-numerical-opt), penalty-decomposition (statistics-probability).
- **Notes:** Held & Karp (1970, 1971) TSP; foundational for combinatorial dual bounds.

### [PRIM-222] gomory-cuts
- **Atom/Composite:** Composite
- **Definition:** Gomory fractional cuts: from LP tableau row, derive integer-valid cut violating current fractional solution.
- **Cost Model:** O(LP-row-extraction) per cut; cut may be weak; modern solvers use Gomory mixed-integer (GMI) cuts.
- **Real Wall:** Theoretical finite convergence (Gomory 1958); pure Gomory unstable, mixed-integer Gomory standard in practice.
- **Cross-Domain Aliases:** fractional-cut (combinatorial-optimization), simplex-cut (control-numerical-opt).
- **Notes:** Gomory (1958); first finite cutting plane algorithm for IP; revived in 1990s with MIR cuts.

### [PRIM-223] mir-cuts
- **Atom/Composite:** Composite
- **Definition:** Mixed-Integer Rounding (MIR) cuts: generalize Gomory cuts using rounding inequalities on mixed-integer rows.
- **Cost Model:** O(constraint-row) per cut; MIR strictly stronger than Gomory in general.
- **Real Wall:** MIR closure has bounded rank but exponential separation; aggressive use can over-tilt LP.
- **Cross-Domain Aliases:** mixed-integer-rounding (control-numerical-opt), rounded-cut (combinatorial-optimization).
- **Notes:** Nemhauser & Wolsey (1990); MIR + cover cuts dominate modern MIP solver cut pools.

### [PRIM-224] chvatal-gomory-rank
- **Atom/Composite:** Primitive
- **Definition:** Chvátal-Gomory rank: minimum number of CG rounds to derive a valid inequality from LP relaxation.
- **Cost Model:** Rank can be exponential in input size for some IPs; bounded for TSP, knapsack, vertex cover.
- **Real Wall:** Some IPs have unbounded CG rank; structural results give rank bounds for problem classes.
- **Cross-Domain Aliases:** cg-rank (combinatorial-optimization), cut-rank (control-numerical-opt).
- **Notes:** Chvátal (1973), Schrijver (1980); rank measures combinatorial complexity of IP polytope.

### [PRIM-225] lift-and-project-cuts
- **Atom/Composite:** Composite
- **Definition:** Lift-and-project: lift LP to higher-dimension space (multiplying constraints by 0-1 variables), project back to obtain stronger cuts.
- **Cost Model:** Lovász-Schrijver / Sherali-Adams hierarchies: nth level O(n^k) for k-level lift.
- **Real Wall:** Higher levels exponentially expensive; first level (N+) often used in practice.
- **Cross-Domain Aliases:** sherali-adams (control-numerical-opt), lift-project-hierarchy (combinatorial-optimization).
- **Notes:** Lovász & Schrijver (1991); Sherali & Adams (1990); SDP variants (Lovász theta function).

### [PRIM-226] cover-inequalities
- **Atom/Composite:** Composite
- **Definition:** Cover inequalities for knapsack constraint: subset S of items with sum > capacity → sum of x_i ≤ |S|-1.
- **Cost Model:** Lifting and minimal cover separation; lifted cover inequalities strengthen via sequential lifting.
- **Real Wall:** Cover separation NP-hard in general; heuristics (greedy lift) effective in practice.
- **Cross-Domain Aliases:** knapsack-cover (combinatorial-optimization), cover-cut (control-numerical-opt).
- **Notes:** Balas (1975), Hammer-Johnson-Peled (1975); standard for 0-1 knapsack and BIP.

### [PRIM-227] knapsack-cover-cuts
- **Atom/Composite:** Composite
- **Definition:** Knapsack cover inequalities: for covering knapsack, dual of cover; aggregate residual capacity as constraint.
- **Cost Model:** Carr-Fleischer-Leung-Phillips O(log n)-approximation for capacitated covering IPs via KC inequalities.
- **Real Wall:** Stronger than basic LP for covering problems with bounded variables.
- **Cross-Domain Aliases:** kc-inequality (combinatorial-optimization), covering-cut (control-numerical-opt).
- **Notes:** Carr et al. (2000); resolves integrality gap of LP for capacitated covering.

### [PRIM-228] strong-branching
- **Atom/Composite:** Composite
- **Definition:** Strong branching: at each node, tentatively branch on candidate variables, evaluate LP, choose variable with best progress.
- **Cost Model:** O(candidates · LP-resolve); expensive but reduces tree size dramatically; reliability branching = cached strong.
- **Real Wall:** Full strong branching too slow; reliability branching uses pseudocosts after initial strong evaluations.
- **Cross-Domain Aliases:** look-ahead-branching (combinatorial-optimization), tentative-branch (control-numerical-opt).
- **Notes:** Applegate-Bixby-Chvátal-Cook (1995); essential for hard MIPs; bottleneck for parallel B&B.

### [PRIM-229] pseudocost-branching
- **Atom/Composite:** Composite
- **Definition:** Pseudocost branching: estimate variable impact from historical branching outcomes; cheap surrogate for strong branching.
- **Cost Model:** O(1) per branch decision after initialization; pseudocosts updated as tree explored.
- **Real Wall:** Cold-start problem at root; reliability branching combines pseudocost with initial strong evaluations.
- **Cross-Domain Aliases:** historical-branch (control-numerical-opt), cost-estimate-branch (combinatorial-optimization).
- **Notes:** Bénichou et al. (1971); reliability branching (Achterberg, 2005) is the modern standard.

### [PRIM-230] sos-branching
- **Atom/Composite:** Primitive
- **Definition:** SOS branching (Special Ordered Sets): SOS1 = at most one nonzero, SOS2 = at most two consecutive nonzero; branch on set partition.
- **Cost Model:** Branching on SOS sets reduces tree depth vs. variable branching; built into Gurobi, CPLEX.
- **Real Wall:** Effective for piecewise-linear modeling and discrete choice; requires modeler to declare SOS sets.
- **Cross-Domain Aliases:** sos-constraint (control-numerical-opt), set-branch (combinatorial-optimization).
- **Notes:** Beale & Tomlin (1970); SOS-friendly formulations crucial for piecewise-linear approximations.

### [PRIM-231] gub-branching
- **Atom/Composite:** Primitive
- **Definition:** Generalized Upper Bound (GUB) branching: branch on partition of a GUB (sum_x = 1) constraint to balance tree.
- **Cost Model:** Reduces tree depth vs. variable branching when GUB constraint has many variables.
- **Real Wall:** Requires identification of GUB structure (often implicit); modern solvers detect automatically.
- **Cross-Domain Aliases:** partition-branch (combinatorial-optimization), gub-partition (control-numerical-opt).
- **Notes:** Beale & Tomlin (1970); precursor to SOS; effective for set-partitioning IPs.

### [PRIM-232] no-good-cut
- **Atom/Composite:** Primitive
- **Definition:** No-good cut: exclude specific infeasible/dominated assignment x = x* via sum_{i: x*_i=1} (1-x_i) + sum_{i: x*_i=0} x_i ≥ 1.
- **Cost Model:** O(n) per cut; exponentially many possible but generated on demand.
- **Real Wall:** No-good cuts weak in LP space but useful in combinatorial Benders, CP-IP hybrids.
- **Cross-Domain Aliases:** combinatorial-benders-cut (combinatorial-optimization), exclusion-constraint (control-numerical-opt).
- **Notes:** Codato & Fischetti (2006); used in MIP solvers for solution refinement; symmetry-breaking variants exist.

## 6. Constraint Programming Internals

### [PRIM-233] ac3-arc-consistency
- **Atom/Composite:** Composite
- **Definition:** AC-3: enforce arc consistency by revising each constraint's arc until fixpoint; remove unsupported domain values.
- **Cost Model:** O(e·d³) for e constraints, domain size d; queue-based propagation.
- **Real Wall:** Worst-case bound rarely achieved; AC-3 the practical baseline despite suboptimality.
- **Cross-Domain Aliases:** arc-revision (control-numerical-opt), constraint-propagation (combinatorial-optimization).
- **Notes:** Mackworth (1977); basic constraint propagator; foundation for AC-4, AC-6, AC-2001.

### [PRIM-234] ac4-optimal
- **Atom/Composite:** Composite
- **Definition:** AC-4: optimal worst-case arc consistency via support counters and per-value support sets.
- **Cost Model:** O(e·d²); optimal for binary CSPs in worst case; high memory overhead.
- **Real Wall:** Memory cost of supports often outweighs time savings; AC-6 / AC-2001 are practical refinements.
- **Cross-Domain Aliases:** support-counter-ac (combinatorial-optimization), optimal-arc (control-numerical-opt).
- **Notes:** Mohr & Henderson (1986); first optimal AC; foundation for AC-6 (Bessière, 1994).

### [PRIM-235] gac-generalized-arc
- **Atom/Composite:** Composite
- **Definition:** Generalized Arc Consistency: extend AC to n-ary constraints; remove value v from domain if no tuple supports v.
- **Cost Model:** O(e·t·d) per constraint with t allowed tuples; structured constraints have faster GAC.
- **Real Wall:** GAC on table constraints can be expensive; STR/MDD compact representations accelerate.
- **Cross-Domain Aliases:** n-ary-propagation (combinatorial-optimization), domain-consistency (control-numerical-opt).
- **Notes:** Mackworth (1977); GAC is the standard consistency level for CP; called "hyperarc consistency" too.

### [PRIM-236] bounds-consistency
- **Atom/Composite:** Composite
- **Definition:** Bounds consistency: propagate only on domain bounds (min/max), ignore interior; cheaper than GAC.
- **Cost Model:** O(e·d) typical; bounds(Z) = real interval, bounds(D) = integer endpoint consistency.
- **Real Wall:** Weaker than GAC; preferred for arithmetic and large-domain numeric constraints.
- **Cross-Domain Aliases:** interval-propagation (control-numerical-opt), bound-pruning (combinatorial-optimization).
- **Notes:** Lhomme (1993); standard for sum, linear, alldifferent constraints in numeric CP.

### [PRIM-237] mac-maintain-ac
- **Atom/Composite:** Composite
- **Definition:** MAC (Maintain Arc Consistency): enforce AC after every branching decision in CSP search; default in modern CP solvers.
- **Cost Model:** O(AC) per decision; pays off via heavy pruning vs. plain backtracking.
- **Real Wall:** Trade-off between propagation strength and depth; restart strategies (luby) handle pathological cases.
- **Cross-Domain Aliases:** mac-search (combinatorial-optimization), propagate-and-branch (control-numerical-opt).
- **Notes:** Sabin & Freuder (1994); foundation of modern CP search; combined with conflict learning in lazy clause generation.

### [PRIM-238] conflict-directed-backjumping
- **Atom/Composite:** Composite
- **Definition:** CBJ: on failure, jump back to the deepest variable involved in conflict, not just immediately previous variable.
- **Cost Model:** Maintains conflict set per variable; O(n) overhead per assignment.
- **Real Wall:** Effective with weak propagation; less important when MAC is strong; nogood recording (lazy clause generation) subsumes.
- **Cross-Domain Aliases:** intelligent-backtracking (combinatorial-optimization), conflict-jump (control-numerical-opt).
- **Notes:** Prosser (1993); foundation for CDCL-style learning in CP solvers (lazy clause generation).

### [PRIM-239] alldifferent-constraint
- **Atom/Composite:** Composite
- **Definition:** alldifferent({x_1, ..., x_n}): all variables take distinct values; bipartite matching gives GAC.
- **Cost Model:** Régin's GAC algorithm O(n·√(n·d)); bounds consistency O(n·log n) via interval graph.
- **Real Wall:** Régin's full GAC uses matching theory; bounds(Z) consistency sufficient for many problems.
- **Cross-Domain Aliases:** distinct-constraint (combinatorial-optimization), permutation-constraint (control-numerical-opt).
- **Notes:** Régin (1994); landmark CP global constraint; reduces propagation effort dramatically vs. n² ≠ constraints.

### [PRIM-240] global-cardinality
- **Atom/Composite:** Composite
- **Definition:** gcc(X, V, low, high): each value v ∈ V occurs between low_v and high_v times in X; generalizes alldifferent.
- **Cost Model:** Régin O(n²·d) GAC via network flow; bounds consistency O(n·log n).
- **Real Wall:** Used in rostering, timetabling; computed via min-cost flow for weighted variant.
- **Cross-Domain Aliases:** count-constraint (combinatorial-optimization), cardinality-bound (control-numerical-opt).
- **Notes:** Régin (1996); core for scheduling problems with personnel constraints.

### [PRIM-241] cumulative-constraint
- **Atom/Composite:** Composite
- **Definition:** cumulative(tasks, resource): at every time t, sum of demands of active tasks ≤ resource capacity.
- **Cost Model:** Energetic reasoning O(n²); edge-finding O(n·log n); time-table propagation O(n).
- **Real Wall:** Strong propagation expensive; combinations (TT + edge-finding + EF) standard in CP-RCPSP solvers.
- **Cross-Domain Aliases:** resource-constraint (control-numerical-opt), capacity-task-constraint (combinatorial-optimization).
- **Notes:** Aggoun & Beldiceanu (1993); cornerstone for resource-constrained project scheduling.

### [PRIM-242] regular-constraint
- **Atom/Composite:** Composite
- **Definition:** regular(X, A): sequence X is accepted by deterministic finite automaton A; encodes regular language constraints.
- **Cost Model:** GAC O(n·|Q|·|Σ|); via unfolded automaton DAG.
- **Real Wall:** Encodes scheduling patterns (e.g., max 5 consecutive nights); cleaner than ad hoc constraints.
- **Cross-Domain Aliases:** automaton-constraint (type-theory-programming-languages), regular-language-cp (combinatorial-optimization).
- **Notes:** Pesant (2004); foundational for sequence constraints; precursor to MDD constraints.

### [PRIM-243] table-constraint
- **Atom/Composite:** Composite
- **Definition:** table(X, T): X must equal one of the tuples in T; explicit listing of allowed combinations.
- **Cost Model:** GAC: STR2/STR3 O(t·n) per propagation; MDD-based compresses tuple set.
- **Real Wall:** Tuple set can be huge; compression (MDD, multivalued decision diagrams) essential.
- **Cross-Domain Aliases:** extensional-constraint (control-numerical-opt), tuple-constraint (combinatorial-optimization).
- **Notes:** Lecoutre (2011); STR algorithms dominate dense table propagation.

### [PRIM-244] circuit-constraint
- **Atom/Composite:** Composite
- **Definition:** circuit(X): variables X_i define a Hamiltonian circuit (each value 1..n appears once, no subtours).
- **Cost Model:** Subtour elimination via strongly connected component check; O(n+m) per propagation.
- **Real Wall:** Used to model TSP and routing in CP; subtour propagation weaker than IP cuts.
- **Cross-Domain Aliases:** hamiltonian-constraint (combinatorial-optimization), tour-constraint (control-numerical-opt).
- **Notes:** Lauriere (1978); foundational TSP constraint in CP; combined with bounds propagation on weights.

### [PRIM-245] sequence-constraint
- **Atom/Composite:** Composite
- **Definition:** sequence(X, q, ℓ, u): for every window of q consecutive variables, count of values in S is between ℓ and u.
- **Cost Model:** GAC O(n·q) per propagation; Régin & Puget filtering.
- **Real Wall:** Common in rostering (max nights in a row, min rest); replaces messy conjunctions of cardinality.
- **Cross-Domain Aliases:** sliding-window-constraint (combinatorial-optimization), pattern-constraint (control-numerical-opt).
- **Notes:** Régin & Puget (1997); used in shift scheduling, frequency assignment.

### [PRIM-246] lazy-clause-generation
- **Atom/Composite:** Composite
- **Definition:** LCG: CP propagators explain their inferences as SAT clauses; SAT solver learns from conflicts.
- **Cost Model:** Per-propagator explanation O(propagator-state); SAT solver applies CDCL on learned clauses.
- **Real Wall:** Explanations must be efficient; full GAC explanation can be expensive (e.g., alldifferent).
- **Cross-Domain Aliases:** sat-cp-hybrid (type-theory-programming-languages), conflict-learning-cp (combinatorial-optimization).
- **Notes:** Ohrimenko-Stuckey-Codish (2009); transforms CP solver into SAT-strength CDCL; Chuffed solver.

### [PRIM-247] luby-restart
- **Atom/Composite:** Composite
- **Definition:** Luby restart sequence: 1,1,2,1,1,2,4,1,1,2,1,1,2,4,8,...; provably optimal universal restart strategy.
- **Cost Model:** O(log) overhead per restart; learned clauses/nogoods retained across restarts.
- **Real Wall:** Heavy-tailed search distributions justified by Luby/Sinclair/Zuckerman analysis.
- **Cross-Domain Aliases:** geometric-restart (statistics-probability), universal-restart (combinatorial-optimization).
- **Notes:** Luby-Sinclair-Zuckerman (1993); proven optimal up to constant factor; used in MiniSAT, Chuffed.

## 7. Vehicle Routing and TSP Variants

### [PRIM-248] christofides-algorithm
- **Atom/Composite:** Composite
- **Definition:** Christofides 1.5-approximation for metric TSP: MST + minimum weight perfect matching on odd-degree vertices + Eulerian tour shortcut.
- **Cost Model:** O(V³) dominated by matching; for metric instances; tight up to (50+ε)/41 (Karlin-Klein-Gharan, 2021).
- **Real Wall:** First polynomial constant-factor approximation for metric TSP; standing record 1976-2020.
- **Cross-Domain Aliases:** christofides-serdyukov (combinatorial-optimization), metric-tsp-approx (control-numerical-opt).
- **Notes:** Christofides (1976), Serdyukov (1978 independently); landmark approximation algorithm.

### [PRIM-249] held-karp-dp
- **Atom/Composite:** Composite
- **Definition:** Held-Karp dynamic programming for TSP: dp[S][v] = min cost to visit subset S ending at v.
- **Cost Model:** O(2^n · n²) time, O(2^n · n) space; tightest exact algorithm for general TSP.
- **Real Wall:** Memory bottleneck at n ≈ 25; only competitive for small to medium instances.
- **Cross-Domain Aliases:** bitmask-dp (combinatorial-optimization), subset-dp (control-numerical-opt).
- **Notes:** Bellman (1962), Held & Karp (1962); fundamental DP technique; basis for TSP lower bound (HK bound).

### [PRIM-250] lin-kernighan
- **Atom/Composite:** Composite
- **Definition:** Lin-Kernighan: variable-depth local search for TSP via sequential edge exchanges (k-opt with adaptive k).
- **Cost Model:** Per move heuristic O(n²) average; LKH implementation finds optimal/near-optimal on 100K-city instances.
- **Real Wall:** Most powerful TSP heuristic; LKH (Helsgaun) uses α-nearness and richer moves.
- **Cross-Domain Aliases:** variable-depth-search (combinatorial-optimization), k-opt-local-search (control-numerical-opt).
- **Notes:** Lin & Kernighan (1973); Helsgaun (2000, 2017) LKH-2/3 dominates large-scale TSP.

### [PRIM-251] concorde-tsp-solver
- **Atom/Composite:** Composite
- **Definition:** Concorde: state-of-the-art exact TSP solver using branch-and-cut with subtour/comb/clique-tree inequalities + Lin-Kernighan.
- **Cost Model:** Solved 85,900-city TSPLIB instance; exponential worst case but exceptional practice.
- **Real Wall:** Concorde combines decades of cutting-plane research; freely available academic.
- **Cross-Domain Aliases:** exact-tsp (combinatorial-optimization), tsp-record (control-numerical-opt).
- **Notes:** Applegate, Bixby, Chvátal, Cook (2006); definitive exact TSP solver.

### [PRIM-252] tsp-with-neighborhoods
- **Atom/Composite:** Composite
- **Definition:** TSPN: visit at least one point of each given region (neighborhood) once, in shortest tour.
- **Cost Model:** APX-hard; constant-factor approximation for disjoint connected regions (Mata & Mitchell).
- **Real Wall:** Continuous embedding of TSP; geometric structure exploited (disks, polygons).
- **Cross-Domain Aliases:** region-tsp (graphics-rendering-lod), watcher-tsp (combinatorial-optimization).
- **Notes:** Arkin & Hassin (1994); applied in drone routing, sensor coverage planning.

### [PRIM-253] prize-collecting-tsp
- **Atom/Composite:** Composite
- **Definition:** PC-TSP: visit subset of cities; each city has prize, missed cities incur penalty; minimize travel + penalty.
- **Cost Model:** Bienstock-Goemans-Simchi-Levi-Williamson 2.5-approximation via LP rounding.
- **Real Wall:** Models routing with optional stops, courier services; generalizes TSP.
- **Cross-Domain Aliases:** orienteering-tsp (combinatorial-optimization), selective-tsp (control-numerical-opt).
- **Notes:** Balas (1989); Bienstock et al. (1993); foundational for selective routing.

### [PRIM-254] generalized-tsp
- **Atom/Composite:** Composite
- **Definition:** GTSP: cities partitioned into clusters; visit exactly one city per cluster in shortest tour.
- **Cost Model:** Reduction to standard TSP via Noon-Bean transformation; tractable up to ~500 clusters.
- **Real Wall:** Applications in last-mile delivery, drilling sequence; reduction blows up TSP size.
- **Cross-Domain Aliases:** cluster-tsp (combinatorial-optimization), set-tsp (control-numerical-opt).
- **Notes:** Noon & Bean (1991); standard reformulation for solver application; Lien-Bee-Tang DP for small clusters.

### [PRIM-255] chinese-postman
- **Atom/Composite:** Composite
- **Definition:** Chinese Postman Problem (CPP): traverse every edge of graph at least once, minimum total distance; closed tour.
- **Cost Model:** Polynomial via T-join / minimum weight perfect matching on odd-degree vertices, O(V³).
- **Real Wall:** Solvable polynomially; mixed and rural variants are NP-hard.
- **Cross-Domain Aliases:** route-inspection (control-numerical-opt), edge-tour (combinatorial-optimization).
- **Notes:** Guan (1962); Edmonds-Johnson (1973) polynomial algorithm via matching.

### [PRIM-256] rural-postman
- **Atom/Composite:** Composite
- **Definition:** RPP: traverse a required subset of edges in shortest closed tour; NP-hard generally.
- **Cost Model:** NP-hard; heuristics: connectivity repair + odd-degree matching; LP-based exact for moderate size.
- **Real Wall:** Models street sweeping, snow plowing on subset of streets; subtour avoidance critical.
- **Cross-Domain Aliases:** edge-subset-tour (combinatorial-optimization), partial-cpp (control-numerical-opt).
- **Notes:** Lenstra & Rinnooy Kan (1976); standard arc routing problem variant.

### [PRIM-257] cvrp
- **Atom/Composite:** Composite
- **Definition:** Capacitated VRP: route fleet from depot to customers, each vehicle has capacity Q, minimize total distance.
- **Cost Model:** Branch-cut-and-price scales to ~300 customers exact; metaheuristics (HGS, FILO, LKH-3) for larger.
- **Real Wall:** Classical CVRP benchmark; CVRPLib instances from 1980s still actively used.
- **Cross-Domain Aliases:** vehicle-routing (control-numerical-opt), capacitated-routing (combinatorial-optimization).
- **Notes:** Dantzig & Ramser (1959); first formulated VRP; cornerstone of operations research.

### [PRIM-258] vrptw
- **Atom/Composite:** Composite
- **Definition:** VRPTW: VRP with time windows at each customer; vehicles must arrive within [a_i, b_i].
- **Cost Model:** Branch-cut-and-price up to ~100 customers (Solomon benchmarks); ALNS for larger.
- **Real Wall:** Time windows often dominate cost; tight windows reduce feasibility space exponentially.
- **Cross-Domain Aliases:** time-windowed-routing (combinatorial-optimization), tw-vehicle-routing (control-numerical-opt).
- **Notes:** Solomon (1987); Solomon benchmark instances are the de facto standard.

### [PRIM-259] pdptw
- **Atom/Composite:** Composite
- **Definition:** Pickup and Delivery Problem with Time Windows: pair pickup and delivery requests, precedence + same vehicle.
- **Cost Model:** Coupling constraints (pickup before delivery, same vehicle); branch-cut-and-price effective.
- **Real Wall:** Models courier services, ride-sharing; structurally harder than VRPTW.
- **Cross-Domain Aliases:** dial-a-ride (control-numerical-opt), pickup-delivery (combinatorial-optimization).
- **Notes:** Savelsbergh & Sol (1995); Li-Lim benchmarks; Lyft/Uber dispatch algorithms.

### [PRIM-260] dial-a-ride
- **Atom/Composite:** Composite
- **Definition:** DARP: PDPTW + user inconvenience constraints (max ride time, max waiting); typical for paratransit.
- **Cost Model:** Multi-objective: minimize cost + user inconvenience; LNS/ALNS dominant.
- **Real Wall:** Service quality vs. cost trade-off; on-demand services blur DARP and VRP boundaries.
- **Cross-Domain Aliases:** paratransit-routing (control-numerical-opt), shared-ride (combinatorial-optimization).
- **Notes:** Psaraftis (1980); active research area driven by paratransit and ride-sharing.

### [PRIM-261] savings-algorithm
- **Atom/Composite:** Composite
- **Definition:** Clarke-Wright savings: start with depot-customer-depot routes, merge pairs with highest s_ij = d_0i + d_0j - d_ij savings.
- **Cost Model:** O(n² log n) sort + greedy merging; baseline heuristic for CVRP.
- **Real Wall:** Quality limited by greedy choice; LKH-3, HGS, FILO dominate modern benchmarks.
- **Cross-Domain Aliases:** clarke-wright (combinatorial-optimization), savings-merge (control-numerical-opt).
- **Notes:** Clarke & Wright (1964); first practical VRP heuristic; still in commercial software.

### [PRIM-262] sweep-algorithm
- **Atom/Composite:** Composite
- **Definition:** Sweep heuristic for VRP: sort customers by polar angle around depot, partition into routes by capacity.
- **Cost Model:** O(n log n) sort; cluster-first, route-second strategy.
- **Real Wall:** Geometric heuristic; ignores asymmetric costs; baseline only.
- **Cross-Domain Aliases:** angular-clustering (combinatorial-optimization), polar-sweep (graphics-rendering-lod).
- **Notes:** Gillett & Miller (1974); two-phase decomposition; widely taught for intuition.

### [PRIM-263] alns
- **Atom/Composite:** Composite
- **Definition:** Adaptive Large Neighborhood Search: destroy-and-repair operators with weights adapted by success history.
- **Cost Model:** Iteration ~O(destroy + repair); operators (random, related, worst removal; greedy, regret insertion).
- **Real Wall:** State-of-the-art for many VRP variants; tuning operator weights is heuristic.
- **Cross-Domain Aliases:** adaptive-lns (control-numerical-opt), destroy-repair (combinatorial-optimization).
- **Notes:** Ropke & Pisinger (2006); dominant metaheuristic for PDPTW and complex VRP variants.

### [PRIM-264] hgs-vrp
- **Atom/Composite:** Composite
- **Definition:** Hybrid Genetic Search: population-based GA + local search (Lin-Kernighan-style) for CVRP, VRPTW.
- **Cost Model:** Generations × local-search; HGS-CVRP outperforms previous benchmarks.
- **Real Wall:** State-of-the-art for CVRP since 2012; combines diversity (GA) and intensification (LS).
- **Cross-Domain Aliases:** memetic-vrp (control-numerical-opt), hybrid-ga-vrp (combinatorial-optimization).
- **Notes:** Vidal et al. (2012); HGS-CVRP repository sets modern benchmarks.

## 8. Scheduling

### [PRIM-265] johnson-rule
- **Atom/Composite:** Composite
- **Definition:** Johnson's rule: two-machine flow shop F2||Cmax; schedule jobs in order min(p_1j, p_2j), partition by which is smaller.
- **Cost Model:** O(n log n) sort + partition; optimal for two-machine flow shop.
- **Real Wall:** Generalization to ≥3 machines is NP-hard; Johnson's rule heuristic for larger m.
- **Cross-Domain Aliases:** two-machine-flowshop (combinatorial-optimization), johnson-schedule (control-numerical-opt).
- **Notes:** Johnson (1954); foundational scheduling result; three-machine special case (Johnson) also polynomial.

### [PRIM-266] makespan-minimization
- **Atom/Composite:** Composite
- **Definition:** P||Cmax: schedule n jobs on m parallel identical machines to minimize maximum completion time.
- **Cost Model:** NP-hard for m ≥ 2; LPT 4/3-approximation; PTAS exists (Hochbaum-Shmoys).
- **Real Wall:** Classical bin-packing flavor; LPT (Longest Processing Time first) is the canonical heuristic.
- **Cross-Domain Aliases:** load-balancing (distributed-systems), parallel-machine-scheduling (combinatorial-optimization).
- **Notes:** Graham (1969); LPT 4/3-approximation; PTAS by Hochbaum-Shmoys (1987).

### [PRIM-267] rcpsp
- **Atom/Composite:** Composite
- **Definition:** Resource-Constrained Project Scheduling: tasks with precedence and resource demands, minimize project duration.
- **Cost Model:** NP-hard; branch-and-bound (Brucker, Demeulemeester), CP with cumulative constraints dominant.
- **Real Wall:** PSPLib benchmark; CP and SAT-based scheduling state-of-the-art for j120 instances.
- **Cross-Domain Aliases:** project-scheduling (control-numerical-opt), constrained-pert (combinatorial-optimization).
- **Notes:** Pritsker (1969); PSPLib instances (Kolisch & Sprecher, 1996) are benchmarks.

### [PRIM-268] weighted-tardiness
- **Atom/Composite:** Composite
- **Definition:** 1||Σ w_j T_j: minimize sum of weighted tardiness on single machine; NP-hard.
- **Cost Model:** Pseudo-polynomial DP O(n · sum w_j); branch-and-bound + DP for moderate n.
- **Real Wall:** Standard objective in scheduling; arises in customer service-level commitments.
- **Cross-Domain Aliases:** tardy-job-cost (control-numerical-opt), weighted-lateness (combinatorial-optimization).
- **Notes:** Lawler (1977); NP-hard reduction from partition; foundation for due-date scheduling.

### [PRIM-269] earliness-tardiness
- **Atom/Composite:** Composite
- **Definition:** Just-in-time scheduling: minimize Σ (α_j E_j + β_j T_j); penalizes early and late completions.
- **Cost Model:** NP-hard; preemptive case polynomial via LP.
- **Real Wall:** Models JIT manufacturing penalties; common-due-date special case admits polynomial algorithms.
- **Cross-Domain Aliases:** jit-scheduling (control-numerical-opt), bidirectional-tardiness (combinatorial-optimization).
- **Notes:** Baker & Scudder (1990); JIT philosophy driver; common in lean manufacturing.

### [PRIM-270] job-shop-scheduling
- **Atom/Composite:** Composite
- **Definition:** J||Cmax: each job has machine-order routing through m machines; minimize makespan.
- **Cost Model:** Strongly NP-hard; CP with disjunctive constraints + edge finding state-of-the-art.
- **Real Wall:** OR-bench instances (Lawrence, Taillard) drive research; 10×10 (FT10) takes hours classically.
- **Cross-Domain Aliases:** disjunctive-scheduling (combinatorial-optimization), shop-makespan (control-numerical-opt).
- **Notes:** Roy & Sussmann (1964) disjunctive graph; Carlier-Pinson (1989) branch-and-bound; OR-tools, CP-SAT dominate.

### [PRIM-271] open-shop-scheduling
- **Atom/Composite:** Composite
- **Definition:** O||Cmax: m machines, each job processed once on each machine in any order; minimize makespan.
- **Cost Model:** Polynomial for m=2 (Gonzalez-Sahni O(n)); NP-hard for m ≥ 3.
- **Real Wall:** Cleaner structure than job-shop; arises in maintenance, testing scheduling.
- **Cross-Domain Aliases:** unordered-shop (combinatorial-optimization), open-flow (control-numerical-opt).
- **Notes:** Gonzalez & Sahni (1976); two-machine open shop O(n); m ≥ 3 strongly NP-hard.

### [PRIM-272] flow-shop-scheduling
- **Atom/Composite:** Composite
- **Definition:** F||Cmax: same machine routing for all jobs; classic permutation flow shop.
- **Cost Model:** NEH heuristic O(n²m); branch-and-bound (Carlier) exact for moderate n.
- **Real Wall:** NEH (Nawaz-Enscore-Ham) is the standard heuristic baseline; Taillard benchmarks.
- **Cross-Domain Aliases:** permutation-flowshop (combinatorial-optimization), ordered-flow (control-numerical-opt).
- **Notes:** Nawaz, Enscore & Ham (1983) NEH; Taillard (1993) benchmark instances; still active research.

### [PRIM-273] scheduling-with-rejection
- **Atom/Composite:** Composite
- **Definition:** Scheduling with rejection: each job has rejection penalty; choose subset to schedule, minimize makespan + penalties.
- **Cost Model:** PTAS for single machine; for parallel machines, FPTAS via DP.
- **Real Wall:** Models overcommitment and SLA penalties; arises in cloud scheduling.
- **Cross-Domain Aliases:** selective-scheduling (control-numerical-opt), rejection-cost-schedule (combinatorial-optimization).
- **Notes:** Bartal et al. (1996); generalizes classical scheduling by allowing opt-out.

### [PRIM-274] preemptive-scheduling
- **Atom/Composite:** Composite
- **Definition:** Preemption allowed: jobs can be interrupted and resumed on same/different machine; often polynomial.
- **Cost Model:** P|pmtn|Cmax polynomial via McNaughton's rule O(n); 1|pmtn,r_j|Cmax via EDD O(n log n).
- **Real Wall:** Polynomial in many cases where non-preemptive is NP-hard; preemption assumption strong.
- **Cross-Domain Aliases:** interruptible-scheduling (control-numerical-opt), pmtn-schedule (combinatorial-optimization).
- **Notes:** McNaughton (1959); EDF (earliest deadline first) for real-time systems is preemptive scheduling.

### [PRIM-275] alpha-beta-gamma-notation
- **Atom/Composite:** Primitive
- **Definition:** Graham-Lawler-Lenstra-Rinnooy Kan α|β|γ notation: α=machines, β=job characteristics, γ=objective.
- **Cost Model:** Notational, not algorithmic; standardizes scheduling problem specification.
- **Real Wall:** ~4500 scheduling problems classified; complexity status known for most.
- **Cross-Domain Aliases:** graham-notation (combinatorial-optimization), scheduling-taxonomy (control-numerical-opt).
- **Notes:** Graham et al. (1979); foundational classification; e.g., 1|prec|Σw_j C_j is NP-hard.

### [PRIM-276] online-scheduling
- **Atom/Composite:** Composite
- **Definition:** Online scheduling: jobs arrive over time; decisions made without knowledge of future jobs.
- **Cost Model:** Competitive ratios analyzed; LIST scheduling (2 - 1/m)-competitive for makespan.
- **Real Wall:** Random-order vs. adversarial arrivals give different bounds; clairvoyant vs. non-clairvoyant.
- **Cross-Domain Aliases:** competitive-scheduling (combinatorial-optimization), arrival-based-scheduling (control-numerical-opt).
- **Notes:** Graham (1966) LIST; online scheduling drives modern cloud / cluster schedulers.

## 9. Online Algorithms

### [PRIM-277] ski-rental
- **Atom/Composite:** Composite
- **Definition:** Ski rental problem: rent (cost 1/day) or buy (cost B) without knowing future; optimal deterministic ratio 2 - 1/B.
- **Cost Model:** Deterministic 2-competitive: rent for B-1 days, buy on day B; randomized e/(e-1) ≈ 1.58.
- **Real Wall:** Canonical online rent-or-buy template; arises in cache prefetching, TCP congestion.
- **Cross-Domain Aliases:** rent-or-buy (combinatorial-optimization), online-investment (statistics-probability).
- **Notes:** Karlin et al. (1988); foundational online algorithm; tight randomized via Yao's principle.

### [PRIM-278] lru-paging
- **Atom/Composite:** Primitive
- **Definition:** LRU (Least Recently Used) paging: on miss, evict the page used least recently; k-competitive vs. OPT_k.
- **Cost Model:** LRU is k-competitive for k-page cache; matches lower bound for deterministic algorithms.
- **Real Wall:** Universal in OS / DB buffer pools; matches LFU and FIFO competitively in worst case.
- **Cross-Domain Aliases:** lru-cache (distributed-systems), cache-replacement (retrieval-search).
- **Notes:** Sleator & Tarjan (1985); k = cache size; randomized MARKER algorithm Θ(log k)-competitive.

### [PRIM-279] k-server-problem
- **Atom/Composite:** Composite
- **Definition:** k-server problem: serve sequence of requests in metric space using k servers, moving server costs distance.
- **Cost Model:** Work-function algorithm (2k-1)-competitive (Koutsoupias-Papadimitriou); randomized polylog-competitive.
- **Real Wall:** k-server conjecture: k-competitive deterministic algorithm exists; open for general metrics.
- **Cross-Domain Aliases:** server-relocation (distributed-systems), online-task-assignment (combinatorial-optimization).
- **Notes:** Manasse-McGeoch-Sleator (1990); deep open problem connecting metric embeddings and online algorithms.

### [PRIM-280] list-update
- **Atom/Composite:** Composite
- **Definition:** List update: maintain linked list under access requests; minimize total access cost via reordering.
- **Cost Model:** Move-to-Front (MTF) is 2-competitive; Frequency Count, Transpose worse.
- **Real Wall:** Foundational online algorithm; applications in data compression (Burrows-Wheeler precursor).
- **Cross-Domain Aliases:** move-to-front (distributed-systems), self-adjusting-list (retrieval-search).
- **Notes:** Sleator & Tarjan (1985); MTF 2-competitive proved by potential function argument.

### [PRIM-281] secretary-problem
- **Atom/Composite:** Composite
- **Definition:** Secretary problem: n candidates arrive in random order; accept/reject irrevocably; optimal stopping rule = 1/e threshold.
- **Cost Model:** Optimal probability of selecting best ≈ 1/e ≈ 0.368; threshold after n/e samples.
- **Real Wall:** Foundation for online matroid secretary problem; matroid generalization open in some cases.
- **Cross-Domain Aliases:** optimal-stopping (statistics-probability), online-selection (combinatorial-optimization).
- **Notes:** Dynkin (1963), Lindley (1961); generalizes to k secretaries, matroid secretary, prophet inequalities.

### [PRIM-282] prophet-inequalities
- **Atom/Composite:** Composite
- **Definition:** Prophet inequality: gambler observes random variables sequentially, optimal stopping ≥ 1/2 expected max.
- **Cost Model:** Median-based threshold achieves 1/2-approximation; tight for adversarial order.
- **Real Wall:** Foundational in posted-price mechanisms; tight via Samuel-Cahn threshold.
- **Cross-Domain Aliases:** posted-price (control-numerical-opt), stopping-inequality (statistics-probability).
- **Notes:** Krengel-Sucheston (1977); Samuel-Cahn (1984) median threshold; deep connections to mechanism design.

### [PRIM-283] competitive-ratio
- **Atom/Composite:** Primitive
- **Definition:** Competitive ratio c: online algorithm A is c-competitive if A(σ) ≤ c·OPT(σ) + α for all input sequences σ.
- **Cost Model:** Analytical concept; lower bounds via adversarial sequences (Yao's principle for randomized).
- **Real Wall:** Worst-case adversarial measure; smoothed / stochastic analysis gives tighter real-world bounds.
- **Cross-Domain Aliases:** worst-case-ratio (control-numerical-opt), online-approximation (combinatorial-optimization).
- **Notes:** Sleator & Tarjan (1985); foundational definition; resource augmentation refines worst-case.

### [PRIM-284] yao-principle
- **Atom/Composite:** Primitive
- **Definition:** Yao's minimax principle: expected cost of best randomized algorithm on worst-case input = expected cost of best deterministic algorithm on worst input distribution.
- **Cost Model:** Lower-bound technique; reduces randomized lower bounds to distributional deterministic bounds.
- **Real Wall:** Powerful for lower bounds (e.g., randomized paging, sorting); requires constructing hard distribution.
- **Cross-Domain Aliases:** minimax-duality (statistics-probability), randomized-lower-bound (combinatorial-optimization).
- **Notes:** Yao (1977); fundamental tool for randomized algorithm lower bounds.

## 10. Submodular Optimization

### [PRIM-285] submodular-set-function
- **Atom/Composite:** Primitive
- **Definition:** Submodular function f: 2^V → R satisfies f(A ∪ {v}) - f(A) ≥ f(B ∪ {v}) - f(B) for A ⊆ B; diminishing returns.
- **Cost Model:** Value-oracle model: f-evaluation O(1) assumed; analysis counts oracle queries.
- **Real Wall:** Submodularity is "discrete convexity"; ubiquitous in coverage, entropy, influence functions.
- **Cross-Domain Aliases:** diminishing-returns (statistics-probability), discrete-convex (control-numerical-opt).
- **Notes:** Edmonds (1970); foundational; covers entropy, matroid rank, coverage, cut function.

### [PRIM-286] greedy-monotone-submodular
- **Atom/Composite:** Composite
- **Definition:** Greedy algorithm for monotone submodular maximization s.t. cardinality constraint k: pick element with max marginal gain k times.
- **Cost Model:** O(k·n·query-time); (1 - 1/e)-approximation; tight unless P=NP.
- **Real Wall:** Optimal in oracle model under cardinality; many applications (sensor placement, summarization).
- **Cross-Domain Aliases:** greedy-coverage (retrieval-search), submodular-greedy (combinatorial-optimization).
- **Notes:** Nemhauser, Wolsey & Fisher (1978); landmark approximation result.

### [PRIM-287] continuous-greedy
- **Atom/Composite:** Composite
- **Definition:** Continuous greedy: maximize multilinear extension F(x) of submodular f via gradient ascent over matroid polytope.
- **Cost Model:** O(n²·k) function evaluations; (1 - 1/e)-approximation for matroid-constrained submodular max.
- **Real Wall:** Achieves 1-1/e under any matroid (extends greedy beyond cardinality); foundational LP relaxation.
- **Cross-Domain Aliases:** multilinear-relaxation (control-numerical-opt), continuous-submodular (combinatorial-optimization).
- **Notes:** Călinescu-Chekuri-Pál-Vondrák (2011); extension of greedy to matroid constraints.

### [PRIM-288] double-greedy-nonmonotone
- **Atom/Composite:** Composite
- **Definition:** Double greedy for unconstrained non-monotone submodular maximization: simultaneous greedy include/exclude per element.
- **Cost Model:** O(n) function evaluations; 1/2-approximation; tight.
- **Real Wall:** Non-monotone submodular harder than monotone; double greedy is the canonical 1/2-approximation.
- **Cross-Domain Aliases:** symmetric-greedy (combinatorial-optimization), buchbinder-greedy (control-numerical-opt).
- **Notes:** Buchbinder-Feldman-Naor-Schwartz (2012); elegant 1/2-approximation via randomized linear-time algorithm.

### [PRIM-289] lovasz-extension
- **Atom/Composite:** Composite
- **Definition:** Lovász extension f̂(x) of submodular f: convex when f is submodular; f̂ minimizable in polynomial time.
- **Cost Model:** Subgradient construction O(n log n); reduces submodular minimization to convex optimization.
- **Real Wall:** Continuous relaxation enables polynomial-time submodular minimization; piecewise-linear function.
- **Cross-Domain Aliases:** convex-extension (control-numerical-opt), submodular-convexification (combinatorial-optimization).
- **Notes:** Lovász (1983); key tool for polynomial-time submodular minimization (Iwata-Fleischer-Fujishige, Schrijver).

### [PRIM-290] submodular-minimization
- **Atom/Composite:** Composite
- **Definition:** Minimize submodular function over 2^V; polynomial-time solvable (combinatorial / continuous algorithms).
- **Cost Model:** Iwata-Fleischer-Fujishige O(n⁵·EO + n⁶); ellipsoid via Lovász extension; recent Õ(n²·EO) (Lee-Sidford-Wong).
- **Real Wall:** Polynomial but high-degree; practical instances solved via min-cut for graph submodular.
- **Cross-Domain Aliases:** submodular-min (control-numerical-opt), min-cut-submodular (combinatorial-optimization).
- **Notes:** Grötschel-Lovász-Schrijver (1981) ellipsoid; combinatorial algorithms (IFF, Schrijver) followed.

### [PRIM-291] submodular-flow
- **Atom/Composite:** Composite
- **Definition:** Submodular flow: flow on directed graph satisfying submodular conservation constraints at vertex partitions.
- **Cost Model:** Polynomial via combinatorial algorithms (Fujishige, Iwata); generalizes min-cost flow and matroid intersection.
- **Real Wall:** Most general polynomial-time combinatorial optimization framework; unifies many problems.
- **Cross-Domain Aliases:** generalized-flow (networking), supermodular-flow (combinatorial-optimization).
- **Notes:** Edmonds & Giles (1977); foundational unifying framework; uses submodular polyhedra.

### [PRIM-292] influence-maximization
- **Atom/Composite:** Composite
- **Definition:** Influence maximization: pick k seeds in graph to maximize expected influence under Independent Cascade or Linear Threshold.
- **Cost Model:** Monotone submodular under IC/LT models; greedy (1 - 1/e)-approximation; reverse influence sampling (RIS).
- **Real Wall:** Function evaluation expensive (Monte Carlo); CELF / RIS accelerate substantially.
- **Cross-Domain Aliases:** viral-marketing (control-numerical-opt), seed-selection (retrieval-search).
- **Notes:** Kempe-Kleinberg-Tardos (2003); landmark for submodular maximization in social networks.

## 11. Approximation Algorithm Toolkit

### [PRIM-293] dependent-rounding
- **Atom/Composite:** Composite
- **Definition:** Dependent rounding: round fractional LP solution to integral while preserving expectation and bounding correlations.
- **Cost Model:** Pipage rounding O(n²); randomized rounding with negative correlation.
- **Real Wall:** Negative correlation crucial for concentration bounds; pipage round handles cardinality constraints.
- **Cross-Domain Aliases:** pipage-rounding (combinatorial-optimization), correlated-rounding (control-numerical-opt).
- **Notes:** Ageev & Sviridenko (2004); foundational for facility location, scheduling approximations.

### [PRIM-294] randomized-rounding
- **Atom/Composite:** Composite
- **Definition:** Randomized rounding: solve LP, set x_i = 1 with probability equal to LP value; gives expected approximation.
- **Cost Model:** O(LP-solve + O(n) rounding); concentration via Chernoff bounds.
- **Real Wall:** Foundational LP-based approximation; multi-coverage and packing problems standard application.
- **Cross-Domain Aliases:** lp-rounding (combinatorial-optimization), probabilistic-rounding (statistics-probability).
- **Notes:** Raghavan & Thompson (1987); landmark technique; Õ(log n) approx for many problems.

### [PRIM-295] primal-dual-schema
- **Atom/Composite:** Composite
- **Definition:** Primal-dual schema: maintain feasible primal-dual pair; iteratively raise dual variables to tighten primal constraints.
- **Cost Model:** Often O(n³) or O(LP-solve); gives combinatorial approximation algorithms avoiding LP.
- **Real Wall:** Tighter analysis than rounding; e.g., 2-approximation for vertex cover via primal-dual.
- **Cross-Domain Aliases:** lp-duality-method (combinatorial-optimization), primal-dual-pair (control-numerical-opt).
- **Notes:** Goemans & Williamson (1995); used in Steiner tree, facility location, network design.

### [PRIM-296] local-ratio-method
- **Atom/Composite:** Composite
- **Definition:** Local ratio: decompose problem weight into smaller weight functions; combine ratios via local ratio theorem.
- **Cost Model:** O(n) per decomposition; equivalent to primal-dual but recursive analysis.
- **Real Wall:** Conceptually clean; works for problems where LP doesn't expose structure.
- **Cross-Domain Aliases:** weight-decomposition (combinatorial-optimization), recursive-ratio (control-numerical-opt).
- **Notes:** Bar-Yehuda & Even (1985); 2-approx for vertex cover; foundation for scheduling approximations.

### [PRIM-297] sdp-relaxation
- **Atom/Composite:** Composite
- **Definition:** SDP relaxation: relax variables to vectors on unit sphere; constraints become inner-product inequalities.
- **Cost Model:** SDP solve O(n^{3.5}) interior point; provides tighter bounds than LP for cut/clustering problems.
- **Real Wall:** SDP solve scales poorly; first-order Burer-Monteiro methods reduce cost.
- **Cross-Domain Aliases:** semidefinite-relaxation (linear-algebra-matrix), psd-relaxation (control-numerical-opt).
- **Notes:** Lovász (1979) theta function; Goemans-Williamson (1995) max-cut; bedrock for tight approximations.

### [PRIM-298] sum-of-squares-hierarchy
- **Atom/Composite:** Composite
- **Definition:** Sum-of-squares (SoS) hierarchy: nested SDP relaxations of polynomial optimization; tighter at higher levels.
- **Cost Model:** Level-d SoS has n^O(d) variables; solves in n^O(d) time.
- **Real Wall:** Higher levels expensive; level-4 SoS solves quasi-NP problems; UGC-resistant bounds.
- **Cross-Domain Aliases:** lasserre-hierarchy (control-numerical-opt), sos-relaxation (combinatorial-optimization).
- **Notes:** Lasserre (2001), Parrilo (2000); hierarchy generalizes Sherali-Adams; tight for many CSP problems.

### [PRIM-299] iterative-rounding
- **Atom/Composite:** Composite
- **Definition:** Iterative rounding: solve LP, fix variables with extreme values (0 or 1), simplify remaining LP, iterate.
- **Cost Model:** O(n) LP solves; relies on extreme point structure of LP polytope.
- **Real Wall:** Used in Jain's 2-approximation for survivable network design; powerful when extreme points have structure.
- **Cross-Domain Aliases:** lp-iterative (combinatorial-optimization), extreme-point-rounding (control-numerical-opt).
- **Notes:** Jain (2001); 2-approximation for survivable network design; foundation for degree-constrained MST.

### [PRIM-300] resource-augmentation
- **Atom/Composite:** Primitive
- **Definition:** Resource augmentation analysis: online algorithm gets extra resources (speed, capacity); compare to weaker optimal.
- **Cost Model:** Analytical refinement of competitive analysis; often closes gap between worst-case and practice.
- **Real Wall:** Justifies practical scheduling algorithms (SRPT with speed augmentation = O(1)-competitive).
- **Cross-Domain Aliases:** speed-augmentation (distributed-systems), beyond-worst-case (combinatorial-optimization).
- **Notes:** Kalyanasundaram & Pruhs (2000); reconciles online algorithm theory with practice.

## 12. Polyhedral Combinatorics

### [PRIM-301] tsp-polytope
- **Atom/Composite:** Composite
- **Definition:** TSP polytope: convex hull of incidence vectors of Hamiltonian tours; exponential facets, NP-hard separation.
- **Cost Model:** Subtour elimination constraints (V choose 2 facets); comb inequalities, clique-tree inequalities.
- **Real Wall:** Full description unknown; Concorde uses ~30 inequality classes from polyhedral research.
- **Cross-Domain Aliases:** tsp-hull (combinatorial-optimization), tour-polytope (control-numerical-opt).
- **Notes:** Dantzig-Fulkerson-Johnson (1954); foundational polyhedral combinatorics object.

### [PRIM-302] matching-polytope
- **Atom/Composite:** Composite
- **Definition:** Matching polytope: convex hull of incidence vectors of matchings; Edmonds showed half-integral inequalities suffice.
- **Cost Model:** Odd-set inequalities for general matching; bipartite matching polytope is integral.
- **Real Wall:** Edmonds gave the complete polyhedral description; separation polynomial via matching algorithm.
- **Cross-Domain Aliases:** edmonds-polytope (combinatorial-optimization), matching-hull (control-numerical-opt).
- **Notes:** Edmonds (1965); landmark polytope; foundation for polynomial matching algorithms.

### [PRIM-303] stable-set-polytope
- **Atom/Composite:** Composite
- **Definition:** Stable set polytope: convex hull of incidence vectors of independent sets; complete description known for perfect graphs (Lovász theta).
- **Cost Model:** Clique inequalities, odd hole inequalities, odd antihole inequalities; complete characterization elusive.
- **Real Wall:** Stable set is NP-hard; polytope characterization tied to graph theory (perfect graph theorem).
- **Cross-Domain Aliases:** independent-set-hull (combinatorial-optimization), antichain-polytope (control-numerical-opt).
- **Notes:** Padberg (1973), Lovász (1972); rank inequalities; theta function bounds.

### [PRIM-304] spanning-tree-polytope
- **Atom/Composite:** Composite
- **Definition:** Spanning tree polytope: convex hull of incidence vectors of spanning trees; described by Edmonds' rank inequalities.
- **Cost Model:** O(V·E) separation via matroid rank computation; integral by total dual integrality.
- **Real Wall:** Beautifully structured; Edmonds-Giles theorem gives integral solutions of LP.
- **Cross-Domain Aliases:** mst-polytope (combinatorial-optimization), tree-hull (linear-algebra-matrix).
- **Notes:** Edmonds (1971); rank inequalities x(E[S]) ≤ |S|-1; greedy achieves integral optimum.

### [PRIM-305] comparability-graph
- **Atom/Composite:** Primitive
- **Definition:** Comparability graph: edges represent comparable pairs in some partial order; perfect graph class.
- **Cost Model:** Recognition O(V+E); coloring/clique polynomial (vs. NP-hard general); transitive orientation.
- **Real Wall:** Subclass of perfect graphs; useful when poset structure is present in instances.
- **Cross-Domain Aliases:** poset-graph (combinatorial-optimization), transitive-orientation (control-numerical-opt).
- **Notes:** Gallai (1967); recognition and decomposition foundational; dual = interval graph.

### [PRIM-306] perfect-graph-theorem
- **Atom/Composite:** Composite
- **Definition:** Strong perfect graph theorem: graph is perfect iff it has no odd hole or odd antihole; chromatic = clique number on all subgraphs.
- **Cost Model:** Recognition O(V⁹) (Chudnovsky et al.); coloring/clique polynomial via Grötschel-Lovász-Schrijver.
- **Real Wall:** Theoretically clean; recognition expensive; relevant when graph structure is known to be perfect.
- **Cross-Domain Aliases:** strong-perfect-graph (combinatorial-optimization), berge-graph (control-numerical-opt).
- **Notes:** Chudnovsky-Robertson-Seymour-Thomas (2006); resolved Berge's 1961 conjecture.

### [PRIM-307] chordal-graph
- **Atom/Composite:** Composite
- **Definition:** Chordal graph: every cycle of length ≥ 4 has a chord; perfect elimination ordering exists.
- **Cost Model:** Recognition / coloring / clique O(V+E) via lexicographic BFS or MCS.
- **Real Wall:** Treewidth-1 ≈ chordal closure; clique tree decomposition exact and small.
- **Cross-Domain Aliases:** triangulated-graph (graphics-rendering-lod), perfect-elimination-graph (combinatorial-optimization).
- **Notes:** Dirac (1961); fundamental for treewidth, sparse matrix elimination.

### [PRIM-308] integrality-gap
- **Atom/Composite:** Primitive
- **Definition:** Integrality gap: supremum over instances of OPT_IP / OPT_LP; measures LP relaxation quality.
- **Cost Model:** Theoretical limit on LP-based approximation; computed analytically or via worst-case instances.
- **Real Wall:** Gap = lower bound on approximation ratio of LP-based algorithms; e.g., vertex cover LP gap = 2.
- **Cross-Domain Aliases:** lp-gap (combinatorial-optimization), relaxation-gap (control-numerical-opt).
- **Notes:** Lovász (1975); central concept in approximation theory; basis for hierarchy strength analysis.

## 13. Set Cover, Steiner, Facility Location

### [PRIM-309] weighted-set-cover
- **Atom/Composite:** Composite
- **Definition:** Weighted set cover: cover all elements with minimum weight set collection; greedy ln n + 1-approximation.
- **Cost Model:** Greedy: pick set with minimum cost/coverage ratio; O((m+n) log n) per iteration.
- **Real Wall:** ln n approximation is tight (Feige); no better unless P=NP.
- **Cross-Domain Aliases:** weighted-cover (combinatorial-optimization), min-cost-cover (control-numerical-opt).
- **Notes:** Chvátal (1979); Feige (1998) tightness; foundation of approximation textbooks.

### [PRIM-310] k-set-cover
- **Atom/Composite:** Composite
- **Definition:** k-set cover: each set has at most k elements; H_k ≈ ln k-approximation via greedy.
- **Cost Model:** O(m·n) greedy; better than ln n for small k; tight up to constant.
- **Real Wall:** Restricted set sizes give improved approximations; geometric set cover even better.
- **Cross-Domain Aliases:** bounded-set-cover (combinatorial-optimization), k-cover (retrieval-search).
- **Notes:** Hochbaum (1982); k = 2 is vertex cover (2-approx).

### [PRIM-311] geometric-set-cover
- **Atom/Composite:** Composite
- **Definition:** Geometric set cover: points and ranges (disks, half-planes); admits PTAS for many range families.
- **Cost Model:** ε-net-based PTAS O(n^{O(1/ε)}); local search PTAS for half-planes.
- **Real Wall:** Geometric structure enables PTAS where general set cover is hard.
- **Cross-Domain Aliases:** range-cover (graphics-rendering-lod), shape-cover (combinatorial-optimization).
- **Notes:** Mustafa & Ray (2010) local-search PTAS; arises in wireless sensor coverage.

### [PRIM-312] multi-cover
- **Atom/Composite:** Composite
- **Definition:** Multi-cover: each element e needs coverage r_e; generalizes set cover (r_e = 1).
- **Cost Model:** Greedy: ln(max r_e · max set size)-approximation; LP rounding tighter.
- **Real Wall:** Models redundancy requirements in networks, fault-tolerance.
- **Cross-Domain Aliases:** redundant-cover (networking), multiple-cover (combinatorial-optimization).
- **Notes:** Dobson (1982); generalization of set cover with multiplicity.

### [PRIM-313] set-packing
- **Atom/Composite:** Composite
- **Definition:** Set packing: maximum disjoint subset collection from given family; LP relaxation, NP-hard.
- **Cost Model:** Greedy 1/k-approximation for k-uniform; LP rounding gives O(√m).
- **Real Wall:** Dual to set cover; maximum matching in hypergraphs.
- **Cross-Domain Aliases:** disjoint-set-selection (combinatorial-optimization), independent-hyperedges (control-numerical-opt).
- **Notes:** Hurkens & Schrijver (1989) local search; 3-set packing → 3-dim matching.

### [PRIM-314] steiner-tree-in-graphs
- **Atom/Composite:** Composite
- **Definition:** Steiner tree problem in graphs: minimum-weight subtree connecting given terminal set; may use non-terminal Steiner points.
- **Cost Model:** NP-hard; 1.39-approximation via LP-based randomized rounding (Byrka et al.).
- **Real Wall:** Foundational connectivity problem; SteinLib benchmarks.
- **Cross-Domain Aliases:** steiner-min-tree (combinatorial-optimization), terminal-connection (networking).
- **Notes:** Karp (1972) NP-hardness; Byrka-Grandoni-Rothvoss-Sanità (2010) 1.39-approx.

### [PRIM-315] prize-collecting-steiner
- **Atom/Composite:** Composite
- **Definition:** Prize-collecting Steiner: each terminal has prize, skip terminals at penalty; minimize tree cost + penalties.
- **Cost Model:** Goemans-Williamson 2-approximation via primal-dual; LP-based 1.91 (Archer et al.).
- **Real Wall:** Models telecommunications network design with optional sites.
- **Cross-Domain Aliases:** pcs-tree (combinatorial-optimization), penalty-steiner (control-numerical-opt).
- **Notes:** Bienstock et al. (1993); foundational network design with rejection.

### [PRIM-316] group-steiner-tree
- **Atom/Composite:** Composite
- **Definition:** Group Steiner tree: connect at least one vertex from each terminal group; harder than standard Steiner.
- **Cost Model:** O(log² n · log k)-approximation via embedding into tree metrics (Garg-Konjevod-Ravi).
- **Real Wall:** Quasi-polynomial gap; arises in keyword search on graphs.
- **Cross-Domain Aliases:** group-connectivity (retrieval-search), set-steiner (combinatorial-optimization).
- **Notes:** Reich & Widmayer (1990); approximation via tree embeddings.

### [PRIM-317] uncapacitated-facility-location
- **Atom/Composite:** Composite
- **Definition:** UFL: open facilities (cost f_i), assign clients to facilities (cost c_ij); minimize total cost.
- **Cost Model:** Best approx 1.488 (Li, 2011); primal-dual 1.61 (Jain-Vazirani); LP rounding 1.5.
- **Real Wall:** Classical OR problem; CPLEX/Gurobi solve thousands of facilities exactly.
- **Cross-Domain Aliases:** plant-location (control-numerical-opt), warehouse-placement (combinatorial-optimization).
- **Notes:** Shmoys-Tardos-Aardal (1997); Jain-Vazirani (2001); Li (2011) best approx.

### [PRIM-318] capacitated-facility-location
- **Atom/Composite:** Composite
- **Definition:** Capacitated FL: each facility has capacity u_i; harder LP relaxation than UFL.
- **Cost Model:** Local search 5-approximation; LP has unbounded integrality gap without rounding tricks.
- **Real Wall:** Capacity constraints make LP weak; combinatorial methods dominate.
- **Cross-Domain Aliases:** capacity-constrained-fl (control-numerical-opt), cap-fl (combinatorial-optimization).
- **Notes:** Pál-Tardos-Wexler (2001) local search 8-approx; subsequent improvements.

### [PRIM-319] k-median-problem
- **Atom/Composite:** Composite
- **Definition:** k-median: choose k facilities to minimize total client-to-nearest-facility distance.
- **Cost Model:** Best approx (2.675 + ε)-approximation (Byrka et al.); local search 5-approx.
- **Real Wall:** Closely related to k-means; metric k-median has PTAS in geometric settings.
- **Cross-Domain Aliases:** medoid-clustering (statistics-probability), k-facility (combinatorial-optimization).
- **Notes:** Charikar-Guha-Tardos-Shmoys (2002); LP-rounding approximations.

### [PRIM-320] k-center-problem
- **Atom/Composite:** Composite
- **Definition:** k-center: minimize maximum distance from client to nearest of k chosen centers.
- **Cost Model:** Gonzalez 2-approximation greedy (farthest-first traversal); tight.
- **Real Wall:** Worst-case (minmax) objective; simple greedy is optimal up to factor 2.
- **Cross-Domain Aliases:** minmax-clustering (statistics-probability), bottleneck-facility (combinatorial-optimization).
- **Notes:** Gonzalez (1985); foundational result; HoSh 2-approx for asymmetric variant.

### [PRIM-321] capacitated-k-median
- **Atom/Composite:** Composite
- **Definition:** k-median with capacities: each facility serves at most u clients; harder than uncapacitated.
- **Cost Model:** LP has unbounded gap; constant-factor (O(1/ε)) bicriteria approximations exist.
- **Real Wall:** Resists LP-based approximation; recent breakthroughs by Li (2017) and others.
- **Cross-Domain Aliases:** cap-k-median (control-numerical-opt), bounded-cluster-size (combinatorial-optimization).
- **Notes:** Aardal et al. (2015); long-standing open: constant approximation without bicriteria violation.

## 14. Knapsack and Packing Variants

### [PRIM-322] quadratic-knapsack
- **Atom/Composite:** Composite
- **Definition:** Quadratic knapsack: maximize Σ p_i x_i + Σ p_ij x_i x_j subject to capacity; NP-hard.
- **Cost Model:** Branch-and-bound with quadratic Lagrangian bounds; LP linearization (McCormick).
- **Real Wall:** Models synergy between selected items; harder than linear knapsack.
- **Cross-Domain Aliases:** qkp (control-numerical-opt), pairwise-profit-knapsack (combinatorial-optimization).
- **Notes:** Gallo, Hammer, Simeone (1980); QKP benchmark instances.

### [PRIM-323] multidim-knapsack
- **Atom/Composite:** Composite
- **Definition:** Multi-dimensional knapsack: m resources, each with capacity; maximize profit subject to all m constraints.
- **Cost Model:** PTAS for fixed m via Frieze-Clarke rounding; FPTAS open for m ≥ 2.
- **Real Wall:** Standard 0-1 multidim knapsack arises in portfolio, capital budgeting; harder than 1D.
- **Cross-Domain Aliases:** d-dim-knapsack (control-numerical-opt), multiconstraint-knapsack (combinatorial-optimization).
- **Notes:** Frieze & Clarke (1984); PTAS via LP rounding; FPTAS conjectured impossible for m ≥ 2.

### [PRIM-324] multiple-knapsack
- **Atom/Composite:** Composite
- **Definition:** Multiple knapsack: m knapsacks with capacities; each item assignable to at most one knapsack; maximize total profit.
- **Cost Model:** PTAS via Chekuri-Khanna; FPTAS via Kellerer for two knapsacks.
- **Real Wall:** Differs from multidim knapsack: items go to one of m bins each with its own capacity.
- **Cross-Domain Aliases:** generalized-assignment-mkp (control-numerical-opt), bin-knapsack (combinatorial-optimization).
- **Notes:** Chekuri & Khanna (2005); PTAS based on LP rounding.

### [PRIM-325] bounded-knapsack
- **Atom/Composite:** Composite
- **Definition:** Bounded knapsack: each item has upper bound b_i on quantity; multiple copies allowed up to bound.
- **Cost Model:** O(n·W) DP after binary expansion; O(n·W·log b) with item replication.
- **Real Wall:** Practical for resource allocation with bounded inventory; binary representation standard.
- **Cross-Domain Aliases:** bounded-multiplicity-knapsack (combinatorial-optimization), inventory-knapsack (control-numerical-opt).
- **Notes:** Standard variant; bound expansion is the trick to reduce to 0-1.

### [PRIM-326] unbounded-knapsack
- **Atom/Composite:** Composite
- **Definition:** Unbounded knapsack: each item type can be selected any number of times; minimize cost or maximize value.
- **Cost Model:** O(n·W) DP forward iteration; coin-change uses same recurrence.
- **Real Wall:** Cutting stock / coin change canonical applications; smaller state space than 0-1.
- **Cross-Domain Aliases:** unbounded-coin-change (control-numerical-opt), repetition-knapsack (combinatorial-optimization).
- **Notes:** Classical DP problem; equivalent to shortest path in weighted DAG.

### [PRIM-327] fractional-knapsack
- **Atom/Composite:** Composite
- **Definition:** Fractional knapsack: items divisible; sort by value/weight ratio, take greedily until full.
- **Cost Model:** O(n log n) sort + O(n) greedy; O(n) via weighted median; optimal greedy.
- **Real Wall:** Polynomial in continuous case (LP relaxation of 0-1 knapsack); upper bound for 0-1 knapsack.
- **Cross-Domain Aliases:** continuous-knapsack (control-numerical-opt), greedy-knapsack (combinatorial-optimization).
- **Notes:** Dantzig (1957); textbook example of greedy optimality on a matroid (transversal).

## 15. Graph Partitioning and Coloring

### [PRIM-328] balanced-partitioning
- **Atom/Composite:** Composite
- **Definition:** Balanced k-partitioning: partition V into k parts of equal size, minimize cut weight; NP-hard with hardness Ω(n^{ε}).
- **Cost Model:** Spectral partitioning O(V·E + V²); recursive bisection O(V·E log k).
- **Real Wall:** No constant-factor approximation; O(√(log n · log k))-approximation via metric embedding.
- **Cross-Domain Aliases:** k-partition (distributed-systems), graph-bisection (combinatorial-optimization).
- **Notes:** Andreev & Räcke (2006) hardness; KaHIP, METIS practical solvers.

### [PRIM-329] metis-partitioning
- **Atom/Composite:** Composite
- **Definition:** METIS multilevel partitioning: coarsen graph by edge contraction, partition coarsest level, refine via Kernighan-Lin.
- **Cost Model:** O(V + E) coarsening + O(E·log V) refinement per level.
- **Real Wall:** Standard practical graph partitioning library; used in parallel sparse linear algebra, mesh partitioning.
- **Cross-Domain Aliases:** multilevel-partition (graphics-rendering-lod), karypis-metis (linear-algebra-matrix).
- **Notes:** Karypis & Kumar (1998); foundation for parallel sparse matrix decomposition.

### [PRIM-330] normalized-cut
- **Atom/Composite:** Composite
- **Definition:** Normalized cut: minimize cut(A, B) · (1/vol(A) + 1/vol(B)); NP-hard, spectral relaxation tractable.
- **Cost Model:** Solve generalized eigenvalue problem O(V³); spectral cut via Fiedler vector + threshold.
- **Real Wall:** Used in image segmentation; spectral relaxation gap O(√Φ) (Cheeger).
- **Cross-Domain Aliases:** spectral-clustering (graphics-rendering-lod), n-cut (statistics-probability).
- **Notes:** Shi & Malik (2000); influential in computer vision; connected to spectral graph theory.

### [PRIM-331] cheeger-inequality
- **Atom/Composite:** Primitive
- **Definition:** Cheeger inequality: λ_2 / 2 ≤ Φ(G) ≤ √(2·λ_2), where Φ is conductance, λ_2 second eigenvalue of normalized Laplacian.
- **Cost Model:** Bounds quality of spectral cut: spectral cut is √(2·OPT_conductance)-approximation.
- **Real Wall:** Tight in both directions; foundation of spectral clustering quality analysis.
- **Cross-Domain Aliases:** conductance-bound (statistics-probability), spectral-conductance (linear-algebra-matrix).
- **Notes:** Cheeger (1970) manifold version; Alon-Milman (1985) graph version; bedrock for spectral methods.

### [PRIM-332] graph-coloring
- **Atom/Composite:** Composite
- **Definition:** Graph coloring: assign colors to vertices so adjacent vertices differ; minimize colors used (chromatic number χ).
- **Cost Model:** NP-hard to approximate within n^{1-ε}; greedy uses Δ+1 colors.
- **Real Wall:** Inapproximability is severe; DSATUR, RLF heuristics practical for small to medium graphs.
- **Cross-Domain Aliases:** vertex-coloring (combinatorial-optimization), proper-coloring (graphics-rendering-lod).
- **Notes:** Karp (1972) NP-hard; Feige-Kilian (1996) inapproximability; register allocation key application.

### [PRIM-333] edge-coloring-vizing
- **Atom/Composite:** Composite
- **Definition:** Vizing's theorem: edge chromatic number χ'(G) ∈ {Δ, Δ+1}; constructive Δ+1-coloring exists in O(V·E).
- **Cost Model:** O(V·E) constructive proof; deciding χ'(G) = Δ is NP-hard.
- **Real Wall:** χ' constrained to two values, yet deciding which is hard; practical Δ+1 coloring uses recursive recoloring.
- **Cross-Domain Aliases:** chromatic-index (combinatorial-optimization), edge-color (graphics-rendering-lod).
- **Notes:** Vizing (1964); foundational; Δ+1 algorithm classic in combinatorial optimization.

### [PRIM-334] list-coloring
- **Atom/Composite:** Composite
- **Definition:** List coloring: each vertex has list L(v) of allowed colors; find proper coloring drawing from lists.
- **Cost Model:** NP-hard; choice number ch(G) ≥ χ(G); equal for some graph classes (chordal, line graphs).
- **Real Wall:** Generalizes chromatic number; arises in frequency assignment.
- **Cross-Domain Aliases:** choosability (combinatorial-optimization), color-with-lists (graphics-rendering-lod).
- **Notes:** Vizing (1976), Erdős-Rubin-Taylor (1979); ch(G) can be much larger than χ(G).

### [PRIM-335] fractional-coloring
- **Atom/Composite:** Composite
- **Definition:** Fractional chromatic number χ_f(G): minimum k/d such that k colors and each vertex gets d colors, adjacent disjoint.
- **Cost Model:** LP relaxation of coloring; computable polynomially for perfect graphs.
- **Real Wall:** Bridges combinatorial coloring and LP; useful for tight bounds.
- **Cross-Domain Aliases:** lp-coloring (combinatorial-optimization), fractional-chromatic (control-numerical-opt).
- **Notes:** Lovász (1975); χ_f(G) = lim_{n→∞} χ(G[n])/n; computable for perfect graphs via theta function.

### [PRIM-336] defective-coloring
- **Atom/Composite:** Composite
- **Definition:** k-defective coloring: color with d-defects allowed (each vertex has at most d same-colored neighbors).
- **Cost Model:** Tractable for planar graphs (4 colors with 1 defect via Cowen et al.); NP-hard in general.
- **Real Wall:** Relaxes proper coloring; used in radio frequency assignment with interference tolerance.
- **Cross-Domain Aliases:** improper-coloring (combinatorial-optimization), tolerance-coloring (graphics-rendering-lod).
- **Notes:** Cowen, Cowen & Woodall (1986); applied to fault-tolerant scheduling.

### [PRIM-337] equitable-coloring
- **Atom/Composite:** Composite
- **Definition:** Equitable k-coloring: proper coloring where color class sizes differ by at most 1.
- **Cost Model:** NP-hard; Hajnal-Szemerédi theorem: equitable Δ+1-coloring always exists.
- **Real Wall:** Models load-balanced timetabling; class size balance critical.
- **Cross-Domain Aliases:** balanced-coloring (combinatorial-optimization), workload-coloring (distributed-systems).
- **Notes:** Hajnal & Szemerédi (1970); polynomial constructive proof recently.

### [PRIM-338] graph-class-recognition
- **Atom/Composite:** Primitive
- **Definition:** Recognize special graph classes (interval, bipartite, chordal, planar) to apply specialized polynomial algorithms.
- **Cost Model:** Many recognizable in O(V+E); planar O(V+E) (Boyer-Myrvold), chordal O(V+E) (LBFS).
- **Real Wall:** Identifying graph structure enables faster algorithms; worth checking before applying general methods.
- **Cross-Domain Aliases:** structural-recognition (combinatorial-optimization), graph-class (graphics-rendering-lod).
- **Notes:** Algorithmic graph theory toolbox; Brandstädt-Le-Spinrad reference encyclopedia.

## 16. Independent Set, Clique, Other Specialized

### [PRIM-339] maximum-independent-set
- **Atom/Composite:** Composite
- **Definition:** Maximum independent set (MIS): largest set of pairwise non-adjacent vertices; NP-hard.
- **Cost Model:** Exact O(1.1996^n) (Xiao-Nagamochi); inapproximable within n^{1-ε}.
- **Real Wall:** Complement of max clique; arises in scheduling conflict graphs, code assignment.
- **Cross-Domain Aliases:** max-stable-set (combinatorial-optimization), anticlique (graphics-rendering-lod).
- **Notes:** Karp (1972) NP-hard; Xiao-Nagamochi (2017) best exact; foundational hard problem.

### [PRIM-340] maximum-clique
- **Atom/Composite:** Composite
- **Definition:** Maximum clique: largest complete subgraph; NP-hard, inapproximable within n^{1-ε}.
- **Cost Model:** Exact O(1.2002^n) Robson; Bron-Kerbosch enumerates all maximal cliques.
- **Real Wall:** Drug discovery, social network communities; DIMACS benchmark instances.
- **Cross-Domain Aliases:** max-complete-subgraph (combinatorial-optimization), clique-detection (graphics-rendering-lod).
- **Notes:** Karp (1972) NP-hard; Bron-Kerbosch (1973) enumeration; Carraghan-Pardalos branch-and-bound.

### [PRIM-341] edmonds-arborescence
- **Atom/Composite:** Composite
- **Definition:** Minimum cost arborescence (rooted directed spanning tree): for each non-root, find min-cost in-edge; resolve cycles by contraction.
- **Cost Model:** Tarjan O(E·log V); Gabow O(E + V·log V); fastest known.
- **Real Wall:** Directed analog of MST; harder than undirected MST due to cycle handling.
- **Cross-Domain Aliases:** chu-liu-edmonds (combinatorial-optimization), directed-mst (networking).
- **Notes:** Chu-Liu (1965), Edmonds (1967), Bock (1971); independent discoveries.

### [PRIM-342] boruvka-mst
- **Atom/Composite:** Composite
- **Definition:** Borůvka's algorithm for MST: in each phase, every component selects cheapest outgoing edge; contract; repeat.
- **Cost Model:** O(E·log V) total; log V phases each O(E); parallelizable.
- **Real Wall:** Oldest MST algorithm; basis for parallel and distributed MST (linear-work algorithms).
- **Cross-Domain Aliases:** parallel-mst (distributed-systems), boruvka-step (combinatorial-optimization).
- **Notes:** Borůvka (1926); Karger-Klein-Tarjan (1995) randomized O(E) MST builds on Borůvka steps.

### [PRIM-343] karger-klein-tarjan-mst
- **Atom/Composite:** Composite
- **Definition:** Karger-Klein-Tarjan: randomized linear-time MST via recursive sampling, Borůvka steps, verification.
- **Cost Model:** Expected O(E) randomized; deterministic linear-time MST is open problem.
- **Real Wall:** First (and only known) expected linear-time MST; relies on linear-time MST verification (King 1995).
- **Cross-Domain Aliases:** randomized-mst (combinatorial-optimization), linear-time-mst (networking).
- **Notes:** Karger, Klein & Tarjan (1995); deterministic linear MST remains open.

### [PRIM-344] bottleneck-spanning-tree
- **Atom/Composite:** Composite
- **Definition:** Bottleneck spanning tree: minimize maximum edge weight; min-max version of MST.
- **Cost Model:** O(V + E) via Camerini's algorithm: median-based partition.
- **Real Wall:** Computable in linear time; MST is bottleneck-optimal but not unique.
- **Cross-Domain Aliases:** minmax-tree (combinatorial-optimization), bottleneck-mst (control-numerical-opt).
- **Notes:** Camerini (1978); MST is a bottleneck tree but bottleneck trees need not be MST.

## 17. Stochastic and Robust Combinatorial Optimization

### [PRIM-345] two-stage-stochastic-ip
- **Atom/Composite:** Composite
- **Definition:** Two-stage stochastic IP: first-stage decisions before uncertainty, second-stage after; minimize expected total cost.
- **Cost Model:** L-shaped method (Benders for stochastic LP); sample average approximation for scenario set.
- **Real Wall:** Scenario count grows exponentially with uncertainty dimension; SAA controls via sample size.
- **Cross-Domain Aliases:** two-stage-recourse (control-numerical-opt), scenario-ip (statistics-probability).
- **Notes:** Birge & Louveaux (1997); foundational stochastic programming text.

### [PRIM-346] sample-average-approximation
- **Atom/Composite:** Composite
- **Definition:** SAA: approximate stochastic optimization expectation with sample mean over N scenarios; solve deterministic equivalent.
- **Cost Model:** O(N · deterministic-solve); statistical bounds on optimality gap via concentration.
- **Real Wall:** Sample size for ε-accurate solution: N = O(d/ε²); requires unbiased scenario sampling.
- **Cross-Domain Aliases:** monte-carlo-optimization (statistics-probability), saa-method (combinatorial-optimization).
- **Notes:** Shapiro (2003); fundamental method for stochastic programming.

### [PRIM-347] robust-combinatorial
- **Atom/Composite:** Composite
- **Definition:** Robust combinatorial optimization: minimize worst-case cost over uncertainty set; conservative immunization.
- **Cost Model:** Polynomial for special uncertainty sets (Γ-robustness, ellipsoidal); NP-hard in general.
- **Real Wall:** Choice of uncertainty set critical; price of robustness vs. solution quality trade-off.
- **Cross-Domain Aliases:** worst-case-optimization (control-numerical-opt), bertsimas-sim (combinatorial-optimization).
- **Notes:** Bertsimas & Sim (2003); Γ-robustness gives tractable robust IP for many classical problems.

### [PRIM-348] distributionally-robust
- **Atom/Composite:** Composite
- **Definition:** DRO: minimize worst-case expected cost over ambiguity set of distributions; bridges robust and stochastic.
- **Cost Model:** Wasserstein-ball DRO becomes tractable convex program; moment-based DRO via SDP.
- **Real Wall:** Choice of ambiguity set is the modeling challenge; Wasserstein DRO has nice statistical properties.
- **Cross-Domain Aliases:** dro (statistics-probability), worst-case-distribution (control-numerical-opt).
- **Notes:** Esfahani & Kuhn (2018); Wasserstein DRO; growing area at ML/OR interface.

### [PRIM-349] surrogate-relaxation
- **Atom/Composite:** Composite
- **Definition:** Surrogate relaxation: aggregate multiple constraints into single weighted sum; relaxed problem has same variables.
- **Cost Model:** Maximize over surrogate multipliers for tightest bound; subgradient over surrogate multipliers.
- **Real Wall:** Bound at least as tight as LP relaxation; computing best surrogate multipliers can be hard.
- **Cross-Domain Aliases:** aggregated-relaxation (control-numerical-opt), constraint-aggregation (combinatorial-optimization).
- **Notes:** Glover (1968); surrogate constraint analysis; combined with Lagrangian for stronger bounds.

## 18. Auctions, Voting, Game-Theoretic Combinatorial

### [PRIM-350] vcg-mechanism
- **Atom/Composite:** Composite
- **Definition:** VCG (Vickrey-Clarke-Groves) mechanism: allocate to maximize total value, charge each agent externality on others.
- **Cost Model:** Solve allocation problem once + once per agent (n+1 total); polynomial when allocation is polynomial.
- **Real Wall:** Truthful by construction; combinatorial auctions VCG implementation requires NP-hard allocation.
- **Cross-Domain Aliases:** vickrey-clarke-groves (control-numerical-opt), truthful-mechanism (combinatorial-optimization).
- **Notes:** Vickrey (1961), Clarke (1971), Groves (1973); foundational mechanism for combinatorial auctions.
- **Notes:** Karmarkar & Karp (1982); APTAS is the current best for bin packing; shows OPT + O(log²(OPT)) possible.
