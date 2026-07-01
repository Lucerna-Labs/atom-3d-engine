### QT.158: Mean Hitting Time (CTMC)
**Definition:** Solve Tᵢ = 1 + Σⱼ PᵢⱼTⱼ for transient states.
**Cost Model:** Mean time to absorption from each starting state.
**Real Wall:** Used for mean time to failure (MTTF) calculations.
**Cross-Domain Aliases:** mean_hitting_time, mttf
**Notes:** MTTF = Σᵢ πᵢTᵢ where π is stationary distribution over non-absorbing states.

---

### QT.159: Reliability Function (Hazard)
**Definition:** R(t) = P(no failure in [0,t]); h(t) = f(t)/R(t) = -R'(t)/R(t).
**Cost Model:** For exponential: R(t) = e^{-λt}, h(t) = λ (constant).
**Real Wall:** Non-constant h(t) indicates wear-out or infant mortality.
**Cross-Domain Aliases:** reliability_function, hazard_rate
**Notes:** Weibull hazard: h(t) = (k/λ)(t/λ)^{k-1}.

---

### QT.160: Availability (Two-State Model)
**Definition:** A = P(system operational at time t) → A∞ = μ/(λ+μ) as t→∞.
**Cost Model:** Instantaneous availability: A(t) = λ/(λ+μ) + μ/(λ+μ)e^{-(λ+μ)t}.
**Real Wall:** Asymptotic availability independent of initial state.
**Cross-Domain Aliases:** availability, uptime_ratio
**Notes:** For repairable systems with exponential up/down times.

---

### QT.161: Point Availability
**Definition:** P(system operational at time t) regardless of past history.
**Cost Model:** A(t) = P(X(t) = UP) for CTMC; depends on initial state.
**Real Wall:** Computed via transient analysis from initial operational state.
**Cross-Domain Aliases:** point_availability, instantaneous_uptime
**Notes:** Approaches steady-state availability as t → ∞.

---

### QT.162: Interval Availability
**Definition:** Fraction of time system operational in interval [0,T].
**Cost Model:** Â(T) = (1/T)∫₀ᵀ 1{X(s)=UP} ds.
**Real Wall:** Time-average availability; converges to steady-state A∞.
**Cross-Domain Aliases:** interval_availability, average_uptime
**Notes:** Use renewal reward: reward rate 1 when UP, 0 when DOWN.

---

### QT.163: Reliability (Non-Repairable)
**Definition:** R(t) = P(T > t) where T is time to failure; no repair considered.
**Cost Model:** For series system: R_s(t) = Πᵢ Rᵢ(t).
**Real Wall:** Exponential components give exponential system reliability.
**Cross-Domain Aliases:** reliability, series_reliability
**Notes:** System fails when any component fails (series model).

---

### QT.164: Redundancy (k-out-of-n)
**Definition:** System operational if at least k of n components operational.
**Cost Model:** For identical exponential: R_kofn = Σᵢ₌ₖⁿ C(n,i) e^{-iλt}(1-e^{-λt})^{n-i}.
**Real Wall:** Common in aerospace: 2-out-of-3 voting.
**Cross-Domain Aliases:** k_of_n, parallel_redundancy
**Notes:** k=n is series; k=1 is fully parallel (active redundancy).

---

### QT.165: Standby Redundancy
**Definition:** Warm/hot standby with standby failures; cold standby has zero failure rate when inactive.
**Cost Model:** Hot standby: identical to active; cold: no failure until activated.
**Real Wall:** Warm: reduced failure rate ρλ when on standby (0 < ρ < 1).
**Cross-Domain Aliases:** standby_reliability, warm_standby
**Notes:** Detect-and-switch time also matters in practice.

---

### QT.166: Imperfect Switching
**Definition:** Switch from failed primary to standby has probability p of working.
**Cost Model:** Effective reliability reduced by factor p per switch failure.
**Real Wall:** Models real-world relay failures, wire breaks.
**Cross-Domain Aliases:** imperfect_switch, p_switch
**Notes:** Can be combined with detection probability < 1.

---

### QT.167: Coverage Factor
**Definition:** Probability that fault handling successfully recovers from fault.
**Cost Model:** System fails if fault not covered; coverage c < 1 increases failure rate.
**Real Wall:** Critical in fault-tolerant computing (avionics).
**Cross-Domain Aliases:** coverage, fault_coverage
**Notes:** Fault Tree Analysis uses coverage for common-cause failures.

---

### QT.168: Fault Tree Analysis (FTA)
**Definition:** Boolean model of system failure in terms of component failures.
**Cost Model:** Compute top event probability from leaf probabilities via AND/OR gates.
**Real Wall:** Static fault trees; dynamic FTA handles sequences.
**Cross-Domain Aliases:** fault_tree, fta
**Notes:** Minimal cut sets identified via Boolean algebra simplification.

---

### QT.169: Reliability Block Diagram (RBD)
**Definition:** Blocks represent components; series/parallel configurations.
**Cost Model:** Series: R = Πᵢ Rᵢ; parallel: R = 1 - Πᵢ (1-Rᵢ).
**Real Wall:** Translates to equivalent reliability for simple configs.
**Cross-Domain Aliases:** rbd, reliability_diagram
**Notes:** RBD is equivalent to fault tree for series-parallel systems.

---

### QT.170: Markov Reliability Models
**Definition:** States with transitions representing failure/repair; absorbing states = failure.
**Cost Model:** Solve for probability of reaching failure states.
**Real Wall:** More general than fault trees; handles sequences and dependencies.
**Cross-Domain Aliases:** markov_reliability, state_based_reliability
**Notes:** Essential for complex fault-tolerant systems (flight control).

---

### QT.171: Semi-Markov Decision Process (SMDP)
**Definition:** MDP with semi-Markov holding times; decision epochs at transitions.
**Cost Model:** Optimal policy via dynamic programming with expected discounted cost.
**Real Wall:** Used for optimal control of queues (admission, routing).
**Cross-Domain Aliases:** smdp, semi_markov_control
**Notes:** Generalizes CTMC control and renewal decision problems.

---

### QT.172: Optimal Hedging Point Policy
**Definition:** Stock level policy: order when inventory ≤ s; order up to S.
**Cost Model:** (s,S) policy optimal for many inventory systems.
**Real Wall:** Simple to implement; near-optimal in practice.
**Cross-Domain Aliases:** sS_policy, inventory_control
**Notes:** Connects queueing theory to supply chain management.

---

### QT.173: Fluid Bandwidth (Effective Bandwidth)
**Definition:** Rate function α(s) = lim_{t→∞} (1/t)log E[e^{sA(t)}].
**Cost Model:** Characterizes traffic envelope; larger for burstier sources.
**Real Wall:** ATM: α(s) = λt(s)/t; effective bandwidth for connection admission.
**Cross-Domain Aliases:** effective_bandwidth_fluid, fluid_envelope
**Notes:** Key for large deviations analysis of buffer overflow.

---

### QT.174: Network Calculus (Arrival Curve)
**Definition:** Arrival curve α(t) bounds cumulative arrivals: A(s,t) ≤ α(t-s).
**Cost Model:** For token bucket: α(t) = min(ρt + σ, rt) where ρ ≤ r.
**Real Wall:** Affine (linear) and token bucket are standard arrival curves.
**Cross-Domain Aliases:** arrival_curve, traffic_envelope
**Notes:** From affine arrivals: A(s,t) ≤ ρ(t-s) + σ.

---

### QT.175: Network Calculus (Service Curve)
**Definition:** Service curve β(t) guarantees output: Y(s,t) ≥ β(t-s).
**Cost Model:** For latency-rate: β(t) = R[t-T]^+ where R = rate, T = latency.
**Real Wall:** Lower bound on service actually provided to a flow.
**Cross-Domain Aliases:** service_curve, guarantee
**Notes:** FIFO server with rate R and latency T provides β(t) = R[t-T]^+.

---

### QT.176: Convolution of Service Curves
**Definition:** Tandem network: end-to-end service curve = convolution of per-node curves.
**Cost Model:** β₁⊗β₂(t) = inf_{0≤s≤t} β₁(t-s) + β₂(s).
**Real Wall:** Computes overall service guarantee through network path.
**Cross-Domain Aliases:** service_convolution, tandem_network
**Notes:** Network calculus equivalent of convolution of transfer functions.

---

### QT.177: Backlog Bound (Network Calculus)
**Definition:** For arrivals with curve α and service β: B ≤ sup_t [α(t) - β(t)].
**Cost Model:** Maximum backlog = vertical deviation between curves.
**Real Wall:** Deterministic bound; holds for every time instant.
**Cross-Domain Aliases:** backlog_bound, buffer_requirement
**Notes:** Key for worst-case dimensioning of buffers.

---

### QT.178: Delay Bound (Network Calculus)
**Definition:** Maximum delay d satisfies: for all t, inf{τ: β(t+τ) ≥ α(t)} ≤ d.
**Cost Model:** Horizontal deviation: largest τ such that β lags α by τ.
**Real Wall:** End-to-end delay bound = sum of per-node delays + propagation.
**Cross-Domain Aliases:** delay_bound, worst_case_delay
**Notes:** Used for real-time networking (avionics, industrial Ethernet).

---

### QT.179: Stochastic Network Calculus (Moment Generators)
**Definition:** Use mgf bounds: E[e^{θA(s,t)}] ≤ e^{θα(θ,s)}; stochastic service curves.
**Cost Model:** Probabilistic bounds on backlog and delay.
**Real Wall:** Less conservative than deterministic; accounts for statistical multiplexing.
**Cross-Domain Aliases:** stochastic_nc, probabilistic_nc
**Notes:** Growing area; combines queueing theory with network calculus.

---

### QT.180: Decoupling Approximation (Queues)
**Definition:** Assume queue lengths at different nodes are independent.
**Cost Model:** Approximate marginals via M/M/1 or M/G/1 formulas.
**Real Wall:** Accurate when traffic is balanced and loads not too high.
**Cross-Domain Aliases:** decoupling, independent_queues
**Notes:** Basis for approximation algorithms for product-form networks.

---

### QT.181: Reduced Load Approximation
**Definition:** Account for blocking probability in network by reducing effective rate.
**Cost Model:** Effective rate λᵢ' = λᵢ(1 - B_j) at node j excluding j.
**Real Wall:** Fixed-point iteration: B depends on λ', λ' depends on B.
**Cross-Domain Aliases:** reduced_load, fixed_point_network
**Notes:** More accurate than independent queue approximation.

---

### QT.182: Knapsack Approximation (Network Design)
**Definition:** Approximate optimal link capacity allocation using fluid relaxation.
**Cost Model:** O(n log n) via greedy or Lagrangian relaxation.
**Real Wall:** Links with capacity constraints; minimize cost given throughput.
**Cross-Domain Aliases:** capacity_allocation, network_design
**Notes:** Related to facility location and bandwidth allocation.

---

### QT.183: Mean Queue Length (M/M/1)
**Cost Model:** L = ρ/(1-ρ) = ρ²/(1-ρ) + ρ; use Little's law: L = λW.
**Real Wall:** Grows as ρ → 1: L ~ 1/(1-ρ).
**Cross-Domain Aliases:** queue_length, mean_qlen
**Notes:** As ρ→1, queue grows without bound (instability in practice).

---

### QT.184: Variance of Queue Length (M/M/1)
**Cost Model:** Var(Q) = ρ/(1-ρ)².
**Real Wall:** Large variance near ρ=1; queue is exponentially distributed.
**Cross-Domain Aliases:** qlen_variance, queue_variance
**Notes:** P(Q=n) = (1-ρ)ρⁿ: geometric distribution.

---

### QT.185: M/M/1 Busy Period
**Definition:** Duration from when queue becomes positive until it returns to zero.
**Cost Model:** Laplace transform: B̂(s) = (s+λ+μ - √((λ+μ+s)² - 4λμ))/2λ.
**Real Wall:** Mean busy period = 1/(μ-λ) for M/M/1.
**Cross-Domain Aliases:** busy_period, busy_time
**Notes:** Independent of future arrivals; starts at first arrival that finds empty queue.

---

### QT.186: Wald's Equation
**Definition:** If Xᵢ i.i.d. with E[X] finite and N stopping time: E[Σᵢ₌₁ᴺ Xᵢ] = E[N]·E[X].
**Cost Model:** Allows computation of expected sum over random number of terms.
**Real Wall:** Holds for non-negative N (many queueing applications).
**Cross-Domain Aliases:** wald_identity, stopping_time_sum
**Notes:** Key for computing expected work over random horizon.

---

### QT.187: Law of Total Probability (Queueing)
**Definition:** Condition on number present at arrival: W = E[W|N] weighted by arrival probability.
**Cost Model:** P(N=n) = π_n for Poisson arrivals (PASTA).
**Real Wall:** Basis for deriving waiting time distributions.
**Cross-Domain Aliases:** total_probability_queue, arrival_decomposition
**Notes:** For M/G/1: E[W²] derived by conditioning on number in queue.

---

### QT.188: FCFS Multi-Server Wait (Exact)
**Definition:** P(W > 0) = C(c, a); E[W|W > 0] = C(c,a)E[S]/(cμ(1-ρ)).
**Cost Model:** C(c,a) from Erlang C; gives exact wait distribution for M/M/c.
**Real Wall:** Exponential waiting time conditional on waiting: P(W > t|W > 0) = e^{-(cμ-λ)t}.
**Cross-Domain Aliases:** mmc_waiting, exact_multiserver_wait
**Notes:** Standard call center formula.

---

### QT.189: Transform Inversion (Queueing)
**Definition:** Numerically invert Laplace-Stieltjes transform to get CDF.
**Cost Model:** Euler or Talbot algorithm: O(n log n) for n-point approximation.
**Real Wall:** Needed for non-Markovian waiting time distributions.
**Cross-Domain Aliases:** laplace_inversion, transform_inversion
**Notes:** Post-Widder algorithm gives stable numerical results.

---

### QT.190: Embedded Chain Analysis (G/G/c)
**Definition:** State observed at departure epochs; solves equilibrium.
**Cost Model:** k-step transition probabilities; steady-state via spectral radius of P.
**Real Wall:** Complex for c > 1; exact analysis limited to special cases.
**Cross-Domain Aliases:** egc_embedded, departure_embedded_ggc
**Notes:** Approximations needed for general service times.

---

### QT.191: Priority Multi-Server (Non-Preemptive)
**Definition:** c servers serve higher priority before lower; preemptive resume optional.
**Cost Model:** Wᵢ = (ΣⱼρⱼE[Sⱼ²])/(2(1-Σⱼ₌₁ⁱρⱼ)(1-Σⱼ₌₁ⁱ⁻¹ρⱼ)) for non-preemptive.
**Real Wall:** Priority inversion: lower classes wait for all higher class jobs.
**Cross-Domain Aliases:** priority_multiserver, multi_priority
**Notes:** Useful for real-time: hard real-time gets priority over soft.

---

### QT.192: Time-Sharing (Time Slice)
**Definition:** Round-robin with quantum q; preempt when quantum expires.
**Cost Model:** For exponential service: T = E[S] + (n-1)q/2 approximately.
**Real Wall:** Fairness: each job gets q per round.
**Cross-Domain Aliases:** round_robin, quantum_scheduling
**Notes:** Classic CPU scheduling; good for interactive systems.

---

### QT.193: Multiclass Queueing Network
**Definition:** Different job classes have different routing, service requirements, priorities.
**Cost Model:** BCMP theorem gives product-form if service is exponential (FCFS/PS).
**Real Wall:** Mixed priority disciplines break product-form.
**Cross-Domain Aliases:** multiclass_network, class_routing
**Notes:** Each class has separate arrival rate and routing matrix.

---

### QT.194: Flow-Equivalence (Decomposition)
**Definition:** Replace network by equivalent single queue matching throughput.
**Cost Model:** Solve fixed point for effective arrival rate; approximate marginal.
**Real Wall:** Works when network is highly connected (no bottlenecks).
**Cross-Domain Aliases:** flow_equivalence, network_decomposition
**Notes:** Basis for approximate analysis of general networks.

---

### QT.195: Effective Bandwidth (Logarithmic)
**Definition:** α(θ) = lim_{t→∞} (1/t) log E[e^{θA(t)}] for cumulative arrivals.
**Cost Model:** Logarithmic moment generating function of arrival process.
**Real Wall:** Characterizes large deviations of cumulative arrivals.
**Cross-Domain Aliases:** mgf_rate, asymptotic_envelope_rate
**Notes:** For fluid ON/OFF source: α(θ) = r·θ·p·(1-e^{-rθ})/(1-e^{-rθp}).

---

### QT.196: Workload Process (Semimartingale)
**Definition:** Virtual waiting time W(t) as supremum of netput process.
**Cost Model:** W(t) = sup_{0≤s≤t} (A(s,t) - B(s,t)) where A=arrivals, B=service.
**Real Wall:** For G/G/1: W(t) converges in distribution to stationary W.
**Cross-Domain Aliases:** workload, virtual_wait_semimartingale
**Notes:** Strongly connected to reflection principle for Brownian motion.

---

### QT.197: Diffusion Approximation (Queue Length)
**Definition:** Scale: Q^(n)(t) = Q(nt)/n → reflected Brownian motion.
**Cost Model:** RBM with drift μ = λ - μ and variance σ² = λ + μ.
**Real Wall:** Valid when utilization near 1 (heavy traffic).
**Cross-Domain Aliases:** rbm_limit, heavy_traffic_diffusion
**Notes:** Fluid + diffusion: fluid for large scale, diffusion for fluctuations.

---

### QT.198: Functional CLT (Queueing)
**Definition:** FCLT: (Q(n·) - n·(1-ρ))/√n → σ·RBM(μ,1).
**Cost Model:** Functional convergence in D[0,∞); sample path limit.
**Real Wall:** Allows approximation of queue process by Brownian motion.
**Cross-Domain Aliases:** functional_clt, queue_fclt
**Notes:** Refined Heavy Traffic Approximations (RHTAs) build on this.

---

### QT.199: Steady-State Probability (M/M/1)
**Cost Model:** π_n = (1-ρ)ρⁿ; for all n ≥ 0.
**Real Wall:** Exists iff ρ < 1 (system stable).
**Cross-Domain Aliases:** mm1_stationary_dist, geometric_stationary
**Notes:** Geometric with parameter ρ; mean = ρ/(1-ρ).

---

### QT.200: Traffic Intensity (Utilization)
**Definition:** ρ = λ/μ for M/M/1; ρ = λE[S] = A/c for M/M/c.
**Cost Model:** Dimensionless; ρ < 1 required for stability.
**Real Wall:** System becomes unstable and queue grows without bound if ρ ≥ 1.
**Cross-Domain Aliases:** utilization_rho, traffic_intensity
**Notes:** Core parameter for all queueing analysis.

---

### QT.201: Erlang Loss Formula (Recursive)
**Definition:** B(c,a) = (a·B(c-1,a))/(c + a·B(c-1,a)) for c ≥ 1; B(0,a) = 1.
**Cost Model:** O(c) recursion; numerically stable for any a, c.
**Real Wall:** Computes blocking probability in loss system.
**Cross-Domain Aliases:** erlang_b_recursive, loss_recursion
**Notes:** Starting from B(0,a) = 1, iteratively compute up to B(c,a).

---

### QT.202: Erlang C Formula
**Definition:** C(c,a) = (a^c/(c!(1-ρ)))·B(c,a) / [Σ_{k=0}^{c-1} a^k/k! + a^c/(c!(1-ρ))].
**Cost Model:** Compute B(c,a) first; then C = B·(a^c/(c!(1-ρ))) / denominator.
**Real Wall:** Gives probability of delay (waiting > 0) in M/M/c.
**Cross-Domain Aliases:** erlang_c, delay_probability
**Notes:** Used for call center staffing (WFM).

---

### QT.203: Engset Formula (Recursive)
**Definition:** E(N,c,a) = N·a·E(N-1,c,a)/(c + (N-1)·a + N·a·E(N-1,c,a)).
**Cost Model:** O(N) recursion; similar to Erlang B recursion.
**Real Wall:** Blocked probability for finite population with c servers.
**Cross-Domain Aliases:** engset, finite_pop_loss_recursion
**Notes:** Reduces to Erlang B as N → ∞.

---

### QT.204: Safari's Formula (Erlang Generalizations)
**Definition:** Formula relating blocking probabilities across traffic mixes.
**Cost Model:** Approximate for mixed traffic in loss system.
**Real Wall:** Used in network design for non-Poisson traffic.
**Cross-Domain Aliases:** safari_formula, mixed_traffic_approx
**Notes:** Empirical accuracy better than pure Erlang assumptions.

---

### QT.205: Point-to-Point Connection (M/M/c/c+N)
**Definition:** c active channels, N overflow channels; Engset-like.
**Cost Model:** Solve birth-death with state-dependent rates.
**Real Wall:** Overflow is rare when c > offered load + buffer.
**Cross-Domain Aliases:** overflow_system, mmpp_overflow
**Notes:** Used for hierarchical network design.

---

### QT.206: State-Space Explosion
**Definition:** CTMC with k queues each up to N has (N+1)^k states.
**Cost Model:** Exponential growth limits exact analysis to small systems.
**Real Wall:** 10 queues of 10 each → 10¹⁰ states (intractable).
**Cross-Domain Aliases:** state_explosion, curse_of_dim
**Notes:** Justifies approximate and asymptotic methods.

---

### QT.207: Mean Field Approximation
**Definition:** Approximate interaction of N queues by average behavior.
**Cost Model:** Fixed point for fraction of queues in each state.
**Real Wall:** Exact as N → ∞; good for N ≥ 20.
**Cross-Domain Aliases:** mean_field, fluid_limit_network
**Notes:** Used for load balancing, server farms, biological systems.

---

### QT.208: Monotonicity (Stochastic Ordering)
**Definition:** X ≥_{st} Y if P(X > t) ≥ P(Y > t) for all t.
**Cost Model:** Comparison of stochastic systems via coupling.
**Real Wall:** Used to compare queueing policies without full analysis.
**Cross-Domain Aliases:** stochastic_ordering, monotonicity_queue
**Notes:** If FCFS ≥_{st} LCFS in M/G/1, we know FCFS has stochastically larger wait.

---

### QT.209: Sample Path Comparison (Coupling)
**Definition:** Couple two systems on same probability space; compare pathwise.
**Cost Model:** Joint construction to establish stochastic ordering.
**Real Wall:** Requires monotonicity property of service/arrival distributions.
**Cross-Domain Aliases:** coupling, pathwise_comparison
**Notes:** Most powerful method for proving performance ordering.

---

### QT.210: Comparison of Disciplines
**Definition:** For fixed arrival/service: FCFS < PS < SRPT in mean response time.
**Cost Model:** Non-preemptive priority intermediate; depends on job size distribution.
**Real Wall:** SRPT optimal; PS ~ 1/(1-ρ); FCFS much worse for heavy-tailed.
**Cross-Domain Aliases:** discipline_comparison, policy_comparison
**Notes:** Heavy-tailed job size distributions amplify discipline differences.

---

### QT.211: Heavy Traffic Optimality
**Definition:** In heavy traffic, many disciplines become equivalent (diffusion limit).
**Cost Model:** All work-conserving policies have same diffusion-scale performance.
**Real Wall:** Differences emerge at lower loads or in finite buffers.
**Cross-Domain Aliases:** heavy_traffic_optimality, asymptotic_equivalence
**Notes:** Explains why FCFS works well when utilization is high.

---

### QT.212: Power-of-d Dispatching
**Definition:** Query d randomly selected servers; route to least loaded.
**Cost Model:** As d grows: maximum queue converges to log log N / log d.
**Real Wall:** d=2 already gives most benefit; diminishing returns beyond d=4.
**Cross-Domain Aliases:** power_of_d, shortest_of_d
**Notes:** Rediscoverable: sampling 2 beats 1 dramatically; sampling 4 slightly better than 2.

---

### QT.213: Join-the-Shortest-Queue (JSQ)
**Definition:** Join server with fewest waiting jobs; known optimal for identical servers.
**Cost Model:** Minimizes expected waiting time for any traffic.
**Real Wall:** Requires global state knowledge; coordination overhead.
**Cross-Domain Aliases:** jsq_optimal, jsl, shortest_queue_dispatch
**Notes:** Optimal among all dispatching policies for symmetric servers.

---

### QT.214: c-MUQ (c-Most-Loaded Queue)
**Definition:** Check c most loaded servers; reject if all have ≥ threshold.
**Cost Model:** O(c) lookup; useful when global state unavailable.
**Real Wall:** Alternative to JSQ when state observation is costly.
**Cross-Domain Aliases:** c_muq, join_idle_otherwise_load
**Notes:** Adaptive threshold: join shortest if possible; else wait or reject.

---

### QT.215: Task Assignment with Batch Service
**Definition:** Servers process batches of tasks; dispatch batch requests to minimize overhead.
**Cost Model:** Batch dispatch reduces overhead but increases latency.
**Real Wall:** MapReduce shuffle phase benefits from batching.
**Cross-Domain Aliases:** batch_assignment, mapreduce_shuffle
**Notes:** Trade-off between dispatching overhead and batching delay.

---

### QT.216: Threshold Control (Server Provisioning)
**Definition:** Provision additional servers when queue exceeds threshold T.
**Cost Model:** Threshold T minimizes cost of waiting vs cost of servers.
**Real Wall:** Hysteresis avoids thrashing; thresholds at two levels.
**Cross-Domain Aliases:** threshold_provisioning, auto_scaling
**Notes:** Cloud auto-scaling uses similar threshold-plus-hysteresis rules.

---

### QT.217: Proportional Response (Cloud Autoscaling)
**Definition:** Scale servers proportionally to current load: c = ⌈λ/μ·C⌉.
**Cost Model:** Maintains constant utilization C; responsive to load changes.
**Real Wall:** Instant scale-up possible in cloud; scale-down needs cooldown.
**Cross-Domain Aliases:** proportional_scaling, utilization_feedback
**Notes:** Core of many cloud autoscaling controllers.

---

### QT.218: Stability Regions
**Definition:** Set of arrival rates for which system is stable (positive recurrent).
**Cost Model:** For Jackson network: all λᵢ such that λᵢ < μᵢ for each node.
**Real Wall:** Stability region may be non-convex for more complex networks.
**Cross-Domain Aliases:** stability_region, throughput_region
**Notes:** Unstable region: queues grow without bound.

---

### QT.219: Unstable Queue
**Definition:** λ ≥ μ for M/M/1 → queue grows linearly with time.
**Cost Model:** L(t) ≈ (λ-μ)t for large t (drift dominates).
**Real Wall:** Instability causes unbounded delays; system overload.
**Cross-Domain Aliases:** unstable_queue, overload
**Notes:** Small overload can cause huge queues (bad initial conditions).

---

### QT.220: Throughput-Optimal Scheduling
**Definition:** Policy that stabilizes system for all feasible arrival rates.
**Cost Model:** Max-weight scheduling: serve queue with largest weight w·λ.
**Real Wall:** Throughput-optimal iff weights correctly chosen.
**Cross-Domain Aliases:** throughput_optimal, max_weight
**Notes:** Backpressure routing is throughput-optimal for wireless networks.

---

## Section: Classical Queue Extensions

### QT.221: M/M/c Multi-Server Queue
**Definition:** Poisson arrivals, exponential service, c parallel servers, infinite buffer; stationary when ρ=λ/(cμ)<1.
**Cost Model:** Erlang C formula gives P(wait>0); mean wait E[W]=C(c,a)/(cμ−λ).
**Real Wall:** Assumes identical servers and exponential service; real call centers violate both.
**Cross-Domain Aliases:** mmc_queue, erlang_delay_model
**Notes:** Erlang (1917); base model for call-center staffing.

---

### QT.222: M/M/c/K Finite Buffer Queue
**Definition:** M/M/c with capacity K; arrivals finding K customers are lost (blocking).
**Cost Model:** Closed-form via birth-death; blocking probability P_K computed by truncated geometric tail.
**Real Wall:** Finite memory in routers, finite waiting rooms in clinics.
**Cross-Domain Aliases:** mmck_queue, finite_buffer
**Notes:** Generalises both Erlang B (K=c) and M/M/c (K=∞).

---

### QT.223: M/M/∞ Infinite-Server Queue
**Definition:** Each arrival receives its own server immediately; no queueing delay.
**Cost Model:** Stationary L is Poisson with mean λ/μ; insensitive to service distribution (becomes M/G/∞).
**Real Wall:** Models self-service systems, response-time of independent jobs.
**Cross-Domain Aliases:** infinite_server, mg_infinity
**Notes:** Foundational for offered-load and MOL approximations (Eick–Massey–Whitt).

---

### QT.224: M/M/1/N Finite Capacity Single Server
**Definition:** Single-server queue with at most N customers in system; excess arrivals blocked.
**Cost Model:** π_k = ρ^k(1−ρ)/(1−ρ^{N+1}); blocking P_B = π_N.
**Real Wall:** Drop-tail buffers in switches and call-center trunks.
**Cross-Domain Aliases:** mm1n_queue, finite_capacity_queue
**Notes:** Loss-delay hybrid; reduces to Geometric when N→∞.

---

### QT.225: M/M/c/c Erlang B Loss System
**Definition:** c servers, no waiting room; blocked arrivals lost.
**Cost Model:** Erlang B: B(c,a)=(a^c/c!)/Σ_{k=0}^c a^k/k!.
**Real Wall:** Classic trunk-sizing formula; insensitive to service distribution.
**Cross-Domain Aliases:** erlang_b, loss_system
**Notes:** Erlang (1917); insensitivity proved by Sevastyanov.

---

### QT.226: M/D/1 Deterministic Service
**Definition:** Poisson arrivals with constant (deterministic) service time.
**Cost Model:** P–K simplifies: E[W]=ρ/(2μ(1−ρ)); half the M/M/1 wait.
**Real Wall:** Models constant-bitrate packet streams, paced jobs.
**Cross-Domain Aliases:** md1_queue, deterministic_service
**Notes:** Special case of M/G/1 with C_s²=0.

---

### QT.227: D/M/1 Deterministic Arrivals
**Definition:** Constant interarrival times, exponential service, one server.
**Cost Model:** Stationary wait at arrival epochs is geometric with parameter σ satisfying μ(1−σ)=λ ln(1/σ)... root of characteristic equation.
**Real Wall:** Periodic jobs (cron, polled sensors).
**Cross-Domain Aliases:** dm1_queue, periodic_arrivals
**Notes:** Lindley root method; dual of M/D/1.

---

### QT.228: D/D/1 Fully Deterministic Queue
**Definition:** Deterministic arrivals and service; queue is either empty or grows linearly.
**Cost Model:** Trivial: if 1/λ ≥ 1/μ, no queue; else infinite queue.
**Real Wall:** Edge of stability; used as worst-case in real-time scheduling.
**Cross-Domain Aliases:** dd1_queue, deterministic_queue
**Notes:** Foundation of network calculus arrival/service curves.

---

### QT.229: M/E_k/1 Erlang-k Service
**Definition:** Poisson arrivals, Erlang-k (sum of k exponentials) service.
**Cost Model:** P–K with C_s²=1/k; E[W]=(1+1/k)ρ/(2μ(1−ρ)).
**Real Wall:** Models multi-stage processing with similar phase rates.
**Cross-Domain Aliases:** mek1_queue, erlang_service
**Notes:** Cox (1955); k→∞ recovers M/D/1.

---

### QT.230: GI/M/1 General Independent Arrivals
**Definition:** I.i.d. general interarrival times, exponential service, single server.
**Cost Model:** Stationary queue at arrival is geometric with parameter σ, the unique root in (0,1) of A*(μ(1−σ))=σ, where A* is interarrival LST.
**Real Wall:** Common when arrivals deviate from Poisson but service is roughly memoryless.
**Cross-Domain Aliases:** gim1_queue, smith_root_equation
**Notes:** Smith (1953); embedded Markov chain at arrivals.

---

### QT.231: M/H_2/1 Hyperexponential Service
**Definition:** Poisson arrivals with two-phase hyperexponential service (mixture of exponentials).
**Cost Model:** Variance large (C_s²>1); P–K gives E[W]=(1+C_s²)ρ/(2μ(1−ρ)).
**Real Wall:** Models heavy-tailed service via phase-type fit.
**Cross-Domain Aliases:** mh21_queue, hyperexponential_service
**Notes:** Used in PH-fitting of empirical service distributions.

---

## Section: Advanced Queue Analysis

### QT.232: Lindley Recursion
**Definition:** Waiting time of n-th customer: W_{n+1}=(W_n+S_n−A_{n+1})^+ in any GI/GI/1 queue.
**Cost Model:** Distributional recursion; stationary W solves W =_d (W+X)^+ with X=S−A.
**Real Wall:** Closed form requires solving the integral (Wiener–Hopf) equation.
**Cross-Domain Aliases:** lindley_recursion, waiting_time_recursion
**Notes:** Lindley (1952); cornerstone of single-server analysis.

---

### QT.233: Pollaczek-Khinchine Formula
**Definition:** Mean wait in M/G/1: E[W]=λE[S²]/(2(1−ρ)).
**Cost Model:** Closed form requiring only first two moments of S.
**Real Wall:** Assumes Poisson arrivals; variance dominates the wait.
**Cross-Domain Aliases:** pk_formula, pollaczek_khinchine
**Notes:** Pollaczek (1930), Khinchine (1932); workhorse of capacity planning.

---

### QT.234: Pollaczek-Khinchine Transform
**Definition:** Laplace–Stieltjes transform of stationary wait in M/G/1: W*(s)=s(1−ρ)/(s−λ+λB*(s)).
**Cost Model:** Numerical inversion (Abate–Whitt) yields full distribution.
**Real Wall:** Inversion is delicate for heavy tails.
**Cross-Domain Aliases:** pk_transform, lst_wait
**Notes:** Generates all moments and tail.

---

### QT.235: Takács Recurrence
**Definition:** Recurrence for moments of the busy period and waiting time in M/G/1 in terms of service-time moments.
**Cost Model:** k-th moment requires moments up to order k of service time.
**Real Wall:** Numerical stability degrades at high orders.
**Cross-Domain Aliases:** takacs_recurrence, busy_period_moments
**Notes:** Takács (1962).

---

### QT.236: Supplementary Variable Method
**Definition:** Augment Markov state with remaining or elapsed service time to recover Markov property in non-Markovian queues.
**Cost Model:** Reduces M/G/1, GI/M/c to PDE/ODE on densities.
**Real Wall:** PDEs are infinite-dimensional; numerics needed.
**Cross-Domain Aliases:** supplementary_variable, age_process
**Notes:** Cox (1955); basis of GI/G/1 phase-type analysis.

---

### QT.237: Wiener-Hopf Factorization
**Definition:** Factor random-walk characteristic function into positive and negative ladder parts: 1−φ(z)=(1−φ_+(z))(1−φ_−(z)).
**Cost Model:** Yields distributions of max and min of partial sums.
**Real Wall:** Explicit only for special increment laws.
**Cross-Domain Aliases:** wiener_hopf, ladder_factorization
**Notes:** Underlies Lindley equation solution; Spitzer.

---

### QT.238: Fluctuation Theory
**Definition:** Study of maxima, minima, ladder epochs of random walks and Lévy processes.
**Cost Model:** Provides exact stationary distributions for queues via duality.
**Real Wall:** Heavy machinery; rarely closed form beyond stable laws.
**Cross-Domain Aliases:** fluctuation_theory, random_walk_extrema
**Notes:** Feller; Spitzer; Bertoin.

---

### QT.239: Ladder Variables
**Definition:** First strict ascending ladder epoch τ and height H of a random walk.
**Cost Model:** Workload distribution in GI/GI/1 equals geometric sum of ladder heights.
**Real Wall:** Joint law (τ,H) hard outside special cases.
**Cross-Domain Aliases:** ladder_epochs, ascending_ladder
**Notes:** Foundation of Pollaczek–Spitzer identity.

---

### QT.240: Spitzer's Identity
**Definition:** E[e^{−sM_n}]=exp(Σ_{k=1}^n E[e^{−sS_k^+}]/k), giving the law of M_n=max(S_0,...,S_n).
**Cost Model:** Converts maxima to sums via logarithm.
**Real Wall:** Useful mainly for transient behaviour and asymptotics.
**Cross-Domain Aliases:** spitzer_identity, max_of_walk
**Notes:** Spitzer (1956).

---

### QT.241: Random Walk in Queueing
**Definition:** Workload V_n in GI/GI/1 equals reflected random walk with increments X_n=S_n−A_{n+1}.
**Cost Model:** V_n=max(0,V_{n−1}+X_n); stationary V exists iff E[X]<0.
**Real Wall:** Heavy-tailed increments yield power-law workload tails.
**Cross-Domain Aliases:** reflected_walk, workload_random_walk
**Notes:** Duality with maxima of partial sums.

---

## Section: Heavy-Traffic Limits

### QT.242: Diffusion Approximation
**Definition:** As ρ→1, scaled workload converges to reflected Brownian motion with drift −(1−ρ) and variance λC_a²+μC_s².
**Cost Model:** Continuous-state approximation; mean wait ≈ (C_a²+C_s²)ρ/(2(1−ρ)μ).
**Real Wall:** Approximation breaks far from heavy traffic.
**Cross-Domain Aliases:** diffusion_limit, kingman_heavy_traffic
**Notes:** Kingman (1962), Iglehart–Whitt (1970).

---

### QT.243: Reflected Brownian Motion (RBM)
**Definition:** Brownian motion constrained to nonnegative half-line via Skorokhod reflection.
**Cost Model:** Stationary exponential with rate 2|drift|/variance.
**Real Wall:** One-dimensional clean; multidimensional needs reflection matrix.
**Cross-Domain Aliases:** rbm, regulated_brownian
**Notes:** Heavy-traffic limit of single-server queues.

---

### QT.244: Halfin-Whitt Regime
**Definition:** Many-server limit with c→∞, ρ→1 such that (1−ρ)√c → β; queue and idle both Θ(√c).
**Cost Model:** Limit process is diffusion combining OU below 0 and BM above 0.
**Real Wall:** Quality-and-efficiency-driven (QED); calls staffed near critical load.
**Cross-Domain Aliases:** halfin_whitt, qed_regime
**Notes:** Halfin–Whitt (1981); foundation of square-root staffing.

---

### QT.245: Square-Root Staffing
**Definition:** Staff c = R + β√R, where R=λ/μ is offered load, to achieve QED performance.
**Cost Model:** β parameterises trade-off between cost and quality.
**Real Wall:** Requires accurate forecast of R; β chosen by Erlang-C or abandonment models.
**Cross-Domain Aliases:** sqrt_staffing, beta_staffing
**Notes:** Borst–Mandelbaum–Reiman (2004).

---

### QT.246: Non-Degenerate Slow-Down (NDS)
**Definition:** Heavy-traffic regime where slow-down (sojourn over service time) has nontrivial limit.
**Cost Model:** Characterises performance of size-based scheduling like SRPT.
**Real Wall:** Different scaling than Halfin–Whitt; SRPT's slow-down is O(1).
**Cross-Domain Aliases:** nds_regime, slowdown_limit
**Notes:** Bansal–Harchol-Balter (2001).

---

### QT.247: QED Regime
**Definition:** Quality- and Efficiency-Driven: both probability of delay and utilization stay bounded away from 0 and 1.
**Cost Model:** Mid-range staffing achieving moderate wait with high utilization.
**Real Wall:** Operational sweet spot for call centers.
**Cross-Domain Aliases:** qed, quality_efficiency_driven
**Notes:** Garnett–Mandelbaum–Reiman.

---

### QT.248: Efficiency-Driven (ED) Regime
**Definition:** Utilization → 1 faster than √c; almost all customers wait.
**Cost Model:** Mean wait is Θ(1/μ); fluid limit nontrivial.
**Real Wall:** High-utilization shops, batch processing.
**Cross-Domain Aliases:** ed_regime, efficiency_driven
**Notes:** Whitt (2004).

---

### QT.249: Quality-Driven (QD) Regime
**Definition:** Staff so that probability of delay → 0; almost no customer waits.
**Cost Model:** c grows faster than R+√R; gives near-zero wait.
**Real Wall:** Premium-quality services, emergency response.
**Cross-Domain Aliases:** qd_regime, overstaffing
**Notes:** Companion to ED and QED regimes.

---

### QT.250: Fluid Limit
**Definition:** Functional law of large numbers: scaled queue process X^n(t)/n → x(t) deterministic ODE.
**Cost Model:** Solves fluid ODE dx/dt=λ−μmin(x,c)/c.
**Real Wall:** First-order; misses fluctuations.
**Cross-Domain Aliases:** fluid_limit, lln_limit
**Notes:** Chen–Mandelbaum.

---

### QT.251: Hydrodynamic Limit
**Definition:** Space–time scaling limit yielding PDE for density of customers in many-server or networked systems.
**Cost Model:** Reaction–diffusion or transport PDE.
**Real Wall:** Heavy machinery from interacting particle systems.
**Cross-Domain Aliases:** hydrodynamic_limit, density_pde
**Notes:** Kipnis–Landim.

---

### QT.252: Mean-Field Limit
**Definition:** As N servers go to infinity under symmetric coupling, fraction of servers in each state converges to deterministic measure.
**Cost Model:** McKean–Vlasov ODE; insensitive to higher moments.
**Real Wall:** Assumes exchangeability; breaks under heterogeneity.
**Cross-Domain Aliases:** mean_field, mckean_vlasov
**Notes:** Foundation of supermarket model and power-of-d analyses.

---

### QT.253: V-Model (Many-Server Limit)
**Definition:** Limiting diffusion for M/M/n+M with abandonment in Halfin–Whitt; combines OU dynamics on either side of 0.
**Cost Model:** PDE/ODE for stationary density; closed forms via Gaussian/exponential pieces.
**Real Wall:** Models call centers with abandonment under QED staffing.
**Cross-Domain Aliases:** v_model, mmn_g_diffusion
**Notes:** Garnett–Mandelbaum–Reiman (2002).

---

## Section: Network Queueing

### QT.254: BCMP Networks
**Definition:** Product-form networks supporting four service disciplines (FCFS exponential, PS, LCFS-PR, IS) with any phase-type service.
**Cost Model:** Stationary distribution is product over nodes; insensitive within allowed classes.
**Real Wall:** Restrictive disciplines; FCFS only with exponential.
**Cross-Domain Aliases:** bcmp_network, product_form_network
**Notes:** Baskett–Chandy–Muntz–Palacios (1975).

---

### QT.255: Kelly's Theorem
**Definition:** Reversible Markov processes have product-form stationary distributions; queueing networks satisfying detailed balance retain insensitivity.
**Cost Model:** Reduces multidim analysis to per-node computations.
**Real Wall:** Reversibility is fragile under non-Poisson arrivals.
**Cross-Domain Aliases:** kelly_theorem, reversibility_theorem
**Notes:** Kelly (1979).

---

### QT.256: Reversible Networks
**Definition:** Networks whose time-reversal has identical dynamics, implying product-form stationary distributions.
**Cost Model:** Detailed balance equations λπ=λ'π'.
**Real Wall:** Most realistic networks are not reversible.
**Cross-Domain Aliases:** reversible_network, detailed_balance_network
**Notes:** Kolmogorov criterion for cycle-product equality.

---

### QT.257: Quasi-Reversibility
**Definition:** Generalises reversibility: forward and reverse processes have same arrival and departure structures conditional on state.
**Cost Model:** Sufficient for product form across heterogeneous customer classes.
**Real Wall:** Service discipline still constrained.
**Cross-Domain Aliases:** quasi_reversible, qr_node
**Notes:** Kelly (1976).

---

### QT.258: Whittle Networks
**Definition:** Network class with state-dependent routing yet product-form stationary distribution.
**Cost Model:** Solvable via local balance.
**Real Wall:** Specific routing forms required.
**Cross-Domain Aliases:** whittle_network, state_dependent_product_form
**Notes:** Whittle (1985).

---

### QT.259: Loss Networks
**Definition:** Multi-resource blocking networks where arriving call uses several units; blocked if any resource saturated.
**Cost Model:** Erlang fixed-point approximation for blocking probabilities.
**Real Wall:** Exact solution exponential in number of resources.
**Cross-Domain Aliases:** loss_network, multirate_blocking
**Notes:** Kelly (1991); models broadband admission control.

---

### QT.260: Kelly-Whittle Loss Networks
**Definition:** Loss networks with product-form solution under bandwidth-sharing capacity constraints.
**Cost Model:** Truncation of product-form by capacity polytope.
**Real Wall:** Computing normalizing constant is #P-hard.
**Cross-Domain Aliases:** kelly_whittle, multirate_loss
**Notes:** Used in spectrum and slot allocation.

---

### QT.261: Arrival Theorem
**Definition:** In a closed BCMP product-form network with N customers, an arriving customer sees the time-stationary distribution of a system with N−1 customers.
**Cost Model:** Enables recursive Mean Value Analysis.
**Real Wall:** Holds only for product-form networks.
**Cross-Domain Aliases:** arrival_theorem, sevcik_mitrani
**Notes:** Sevcik–Mitrani (1981).

---

### QT.262: Mean Value Analysis (MVA)
**Definition:** Recursive algorithm computing mean response time, throughput and queue length in closed product-form networks.
**Cost Model:** O(MN) per population growth, M nodes, N customers.
**Real Wall:** Restricted to product form; multiclass increases dimension.
**Cross-Domain Aliases:** mva, exact_mva
**Notes:** Reiser–Lavenberg (1980).

---

### QT.263: Approximate MVA
**Definition:** Iterative MVA-style algorithm relaxing product-form assumptions (e.g., Schweitzer, Bard).
**Cost Model:** Fixed-point iteration; cheap for large N.
**Real Wall:** Accuracy degrades with feedback loops or heavy traffic.
**Cross-Domain Aliases:** approx_mva, schweitzer_mva
**Notes:** Schweitzer (1979), Bard (1979).

---

### QT.264: Convolution Algorithm
**Definition:** Computes normalization constant G(N) of closed product-form network via successive convolutions over nodes.
**Cost Model:** O(MN) time and memory.
**Real Wall:** Numerical overflow for large N without scaling.
**Cross-Domain Aliases:** convolution_algorithm, buzen_algorithm
**Notes:** Buzen (1973).

---

## Section: G/G/1 Analysis

### QT.265: Kingman's Formula
**Definition:** Heavy-traffic approximation for GI/GI/1 wait: E[W] ≈ (ρ/(1−ρ))·((C_a²+C_s²)/2)·(1/μ).
**Cost Model:** Closed-form requiring only first two moments of interarrival and service.
**Real Wall:** Accurate near saturation; underestimates for moderate ρ.
**Cross-Domain Aliases:** kingman_formula, gi_gi_1_approx
**Notes:** Kingman (1961).

---

### QT.266: Marshall's Inequality
**Definition:** Upper bound E[W]≤(λ(σ_A²+σ_S²)/(2(1−ρ)))+ρ/(μ) on stationary wait in GI/GI/1.
**Cost Model:** Bound only; tight near heavy traffic.
**Real Wall:** Slack at low loads.
**Cross-Domain Aliases:** marshall_bound, gi_gi_1_bound
**Notes:** Marshall (1968).

---

### QT.267: GI/GI/1 Lower Bound
**Definition:** E[W] ≥ (ρ²+λ²σ_A²−ρ)/(2λ(1−ρ)) (Kingman lower bound on stationary wait).
**Cost Model:** Two-moment closed-form bound.
**Real Wall:** Conservative for light traffic.
**Cross-Domain Aliases:** kingman_lower_bound, gi_gi_1_lower
**Notes:** Kingman (1962).

---

### QT.268: Lindley Integral Equation
**Definition:** Stationary wait W in GI/GI/1 satisfies F_W(x)=∫_{-∞}^x F_W(x−u)dF_X(u), with X=S−A.
**Cost Model:** Solved via Wiener–Hopf or Pollaczek transform.
**Real Wall:** Closed form rare; numerical inversion required.
**Cross-Domain Aliases:** lindley_equation, integral_equation_wait
**Notes:** Lindley (1952).

---

### QT.269: Busy Period
**Definition:** Time from arrival to empty system until next empty epoch.
**Cost Model:** In M/G/1, LST B*(s) satisfies B*(s)=B^o*(s+λ−λB*(s)).
**Real Wall:** Heavy-tailed service yields heavy-tailed busy periods.
**Cross-Domain Aliases:** busy_period, takacs_equation
**Notes:** Takács (1955).

---

### QT.270: Virtual Waiting Time
**Definition:** Time a virtual customer arriving at time t would wait; equals remaining workload V(t).
**Cost Model:** Same distribution as actual wait at Poisson arrivals (PASTA).
**Real Wall:** Differs from waiting time under non-Poisson arrivals.
**Cross-Domain Aliases:** virtual_wait, workload_process
**Notes:** Reich (1958).

---

### QT.271: Marchal's Approximation
**Definition:** Two-moment formula improving Kingman by including third moments for GI/G/1 wait.
**Cost Model:** Closed-form correction term.
**Real Wall:** Accuracy varies with distribution shape.
**Cross-Domain Aliases:** marchal_approx, two_moment_approx
**Notes:** Marchal (1976).

---

### QT.272: Whitt's Approximations
**Definition:** Family of GI/G/1 and GI/G/m approximations using squared coefficient of variation and refined moment matching.
**Cost Model:** Closed form; tuned for QNA.
**Real Wall:** Empirical; error bounds heuristic.
**Cross-Domain Aliases:** whitt_approx, qna_approx
**Notes:** Whitt (1983).

---

## Section: Priority Queueing

### QT.273: Non-Preemptive Priority (Cobham)
**Definition:** M/G/1 with K priority classes; service in progress is never interrupted.
**Cost Model:** E[W_k]=λΣ E[S_i²]/2 / ((1−Σ_{i≤k}ρ_i)(1−Σ_{i<k}ρ_i)).
**Real Wall:** Class k blocked by class >k once a low job starts.
**Cross-Domain Aliases:** cobham_priority, non_preemptive_priority
**Notes:** Cobham (1954).

---

### QT.274: Preemptive Resume Priority
**Definition:** Higher-priority arrival interrupts lower; preempted job resumes from where left off.
**Cost Model:** E[W_k]=Σ_{i≤k}λ_iE[S_i²]/(2(1−Σ_{i≤k}ρ_i)) + ρ_kE[S_k]/(1−Σ_{i≤k−1}ρ_i).
**Real Wall:** Requires checkpointing; ignores switching cost.
**Cross-Domain Aliases:** preemptive_resume, prr_priority
**Notes:** Standard in OS scheduling models.

---

### QT.275: Preemptive Repeat Priority
**Definition:** Preempted job restarts from scratch upon resumption.
**Cost Model:** Effective service distribution changes; analysed via residual life.
**Real Wall:** Wastes work; rarely optimal.
**Cross-Domain Aliases:** preemptive_repeat, prp_priority
**Notes:** Models real-time tasks with no checkpointing.

---

### QT.276: M/G/1 with Priorities
**Definition:** Combines M/G/1 P–K with priority indexing across classes; conservation law links all classes.
**Cost Model:** Σ ρ_k E[W_k] = ρλE[S²]/(2(1−ρ)) (Kleinrock conservation).
**Real Wall:** Strict priorities can starve low classes.
**Cross-Domain Aliases:** mg1_priority, kleinrock_conservation
**Notes:** Kleinrock (1975).

---

### QT.277: Klimov Index
**Definition:** Index policy maximizing weighted reward in multiclass M/G/1: serve class with largest c_kμ_k/(1−ρ_k+) extension.
**Cost Model:** Computed via Gittins-style recursion.
**Real Wall:** Optimal only under specific reward structures.
**Cross-Domain Aliases:** klimov_index, multiclass_index
**Notes:** Klimov (1974).

---

### QT.278: c-mu Rule
**Definition:** Serve class with largest c_kμ_k (holding cost × service rate) to minimize expected holding cost.
**Cost Model:** Optimal for multi-class M/M/1 with linear holding costs.
**Real Wall:** Suboptimal under abandonments or heavy-tailed service.
**Cross-Domain Aliases:** cmu_rule, smith_rule
**Notes:** Cox–Smith (1961).

---

### QT.279: Generalized c-mu Rule (Gcμ)
**Definition:** Asymptotically optimal scheduling for many-server queues with convex holding cost: serve class with largest C'_k(Q_k)μ_k.
**Cost Model:** Optimal in heavy traffic.
**Real Wall:** Requires convex holding costs and known queue lengths.
**Cross-Domain Aliases:** gcmu_rule, generalized_cmu
**Notes:** Mandelbaum–Stolyar (2004).

---

## Section: Scheduling Policies

### QT.280: FCFS (First-Come-First-Served)
**Definition:** Serve in arrival order; nonpreemptive.
**Cost Model:** Mean wait given by Kingman/PK; fair by arrival time.
**Real Wall:** Sensitive to large jobs (head-of-line blocking).
**Cross-Domain Aliases:** fcfs, fifo_service
**Notes:** Default discipline in most analyses.

---

### QT.281: LCFS (Last-Come-First-Served)
**Definition:** Newest arrival is served next.
**Cost Model:** Same mean wait as FCFS under M/G/1 but higher variance.
**Real Wall:** Starves old arrivals; useful for staleness-sensitive queues.
**Cross-Domain Aliases:** lcfs, lifo_service
**Notes:** Variant: LCFS-PR (preemptive resume) — product form in BCMP.

---

### QT.282: SRPT (Shortest Remaining Processing Time)
**Definition:** Always serve the job with smallest remaining size.
**Cost Model:** Optimal mean response time for any arrival sequence.
**Real Wall:** Requires knowing remaining size; starves big jobs.
**Cross-Domain Aliases:** srpt, smith_rule
**Notes:** Schrage (1968); Bansal–Harchol-Balter (2001) for tail bounds.

---

### QT.283: SJF (Shortest Job First)
**Definition:** Non-preemptive: dispatch smallest waiting job next.
**Cost Model:** Mean wait minimised among non-preemptive policies (Smith).
**Real Wall:** Requires size estimates.
**Cross-Domain Aliases:** sjf, smith_static_rule
**Notes:** Smith (1956).

---

### QT.284: Processor Sharing (PS)
**Definition:** Server splits capacity equally among all present jobs.
**Cost Model:** Insensitive: mean response time E[S]/(1−ρ); distribution independent of service law beyond mean.
**Real Wall:** Idealised; real OS round-robin has quantum overhead.
**Cross-Domain Aliases:** ps_discipline, round_robin_limit
**Notes:** Kleinrock (1967).

---

### QT.285: Discriminatory PS (DPS)
**Definition:** PS with class-dependent weights w_k controlling share.
**Cost Model:** Mean delays solve linear system in class loads.
**Real Wall:** Weights must be assigned; complex multi-class analysis.
**Cross-Domain Aliases:** dps_discipline, weighted_ps
**Notes:** Fayolle–Mitrani–Iasnogorodski (1980).

---

### QT.286: Generalized Processor Sharing (GPS)
**Definition:** Idealised fluid model: each backlogged class receives bandwidth proportional to weight φ_k.
**Cost Model:** Class throughput min(λ_k, φ_kC/Σ_{j∈backlogged}φ_j).
**Real Wall:** Pure fluid; packets need WFQ approximation.
**Cross-Domain Aliases:** gps_discipline, fluid_share
**Notes:** Parekh–Gallager (1993).

---

### QT.287: Weighted Fair Queueing (WFQ)
**Definition:** Packet-by-packet approximation of GPS based on virtual finish times.
**Cost Model:** Per-packet O(log N) work; delay bound within one max-size packet of GPS.
**Real Wall:** Computing system virtual time is expensive at high speed.
**Cross-Domain Aliases:** wfq, pgps
**Notes:** Parekh–Gallager (1993).

---

### QT.288: Foreground-Background (FB)
**Definition:** Serve job with smallest attained service first.
**Cost Model:** Optimal under DHR (decreasing-hazard-rate) service times.
**Real Wall:** Needs per-job age tracking; many context switches.
**Cross-Domain Aliases:** fb_discipline, mlfq_limit
**Notes:** Kleinrock (1976).

---

### QT.289: LAS (Least Attained Service)
**Definition:** Synonym of FB: jobs with least cumulative service get priority.
**Cost Model:** Same closed form as FB; favors short jobs without explicit knowledge.
**Real Wall:** Heavy bookkeeping; sensitive to size oracle errors.
**Cross-Domain Aliases:** las_discipline, least_attained
**Notes:** Used in TCP-friendly scheduling.

---

### QT.290: LAS-PS Hybrid
**Definition:** LAS among classes with PS within class; combines size-awareness and fairness.
**Cost Model:** Class-level mean delays computed via DPS-style analysis.
**Real Wall:** Approximate; closed forms only for two classes.
**Cross-Domain Aliases:** las_ps, hybrid_scheduling
**Notes:** Aalto–Ayesta (2007).

---

## Section: Stability and Lyapunov Methods

### QT.291: Foster-Lyapunov Criterion
**Definition:** Markov chain positive-recurrent if there exists V(x)≥0 with E[V(X_{n+1})−V(X_n)|X_n=x]≤−ε outside compact set.
**Cost Model:** Choice of V dictates difficulty; quadratic V common.
**Real Wall:** Sufficient, not necessary; designing V is creative.
**Cross-Domain Aliases:** foster_lyapunov, drift_criterion
**Notes:** Foster (1953).

---

### QT.292: Throughput Stability
**Definition:** Queue stable iff time-average departure rate equals arrival rate.
**Cost Model:** lim_{t→∞} D(t)/t = λ a.s.
**Real Wall:** Long simulations needed to verify.
**Cross-Domain Aliases:** rate_stability, ergodic_throughput
**Notes:** Equivalent to positive recurrence under mild conditions.

---

### QT.293: Max-Weight Scheduling
**Definition:** Schedule the configuration maximizing Σ Q_iR_i (queue × rate).
**Cost Model:** Throughput-optimal in switched networks.
**Real Wall:** Computing max-weight may be NP-hard per slot.
**Cross-Domain Aliases:** maxweight, mwm_scheduling
**Notes:** Tassiulas–Ephremides (1992).

---

### QT.294: Max-Pressure / BackPressure Routing
**Definition:** Route across each link the queue-difference-maximizing commodity.
**Cost Model:** Throughput-optimal without knowing arrival rates.
**Real Wall:** Requires per-commodity queues at each node.
**Cross-Domain Aliases:** backpressure, max_pressure
**Notes:** Tassiulas–Ephremides; Neely (2010).

---

### QT.295: Lyapunov Drift Analysis
**Definition:** Bound E[V(X_{n+1})−V(X_n)|X_n] to deduce stability and performance bounds.
**Cost Model:** Drift+penalty technique gives O(1/V, V) trade-off.
**Real Wall:** Drift bound only as good as candidate V.
**Cross-Domain Aliases:** drift_analysis, lyapunov_drift
**Notes:** Neely's drift-plus-penalty framework.

---

### QT.296: Fluid Stability
**Definition:** A multiclass network is stable iff every fluid limit is eventually empty.
**Cost Model:** Replaces stochastic analysis with ODE/LP.
**Real Wall:** Counter-examples (Bramson, Rybko–Stolyar) show necessity issues.
**Cross-Domain Aliases:** fluid_stability, dai_criterion
**Notes:** Dai (1995).

---

## Section: Abandonment and Time-Varying

### QT.297: Erlang A Queue
**Definition:** M/M/c with i.i.d. exponential patience times; customers leave if not served by patience deadline.
**Cost Model:** Closed form for stationary distribution via continued fractions.
**Real Wall:** Patience often non-exponential in practice.
**Cross-Domain Aliases:** erlang_a, mmcm_abandonment
**Notes:** Palm (1937); Garnett–Mandelbaum–Reiman (2002).

---

### QT.298: Customer Impatience
**Definition:** Generic abandonment behaviour: random patience time governs abandonment probability.
**Cost Model:** Mean abandonment rate ≈ λ·P(W>θ) for patience θ.
**Real Wall:** Patience distribution must be measured.
**Cross-Domain Aliases:** impatience, reneging
**Notes:** Affects throughput and quality of service.

---

### QT.299: Time-Varying Abandonment
**Definition:** Patience hazard depends on time of day or workload.
**Cost Model:** Inhomogeneous Markov chain; numerical integration.
**Real Wall:** Hard to estimate from telephone data without censoring corrections.
**Cross-Domain Aliases:** time_varying_abandonment, dynamic_patience
**Notes:** Mandelbaum–Zeltyn (2007).

---

### QT.300: M/M/n+G Queue
**Definition:** M/M/n with general patience distribution G.
**Cost Model:** Approximated via offered-load formula and Erlang-A fit.
**Real Wall:** Patience tails dictate behaviour.
**Cross-Domain Aliases:** mmng, abandonment_general
**Notes:** Whitt (2006).

---

### QT.301: Time-Varying Arrival Rates
**Definition:** Non-homogeneous Poisson arrivals with rate λ(t).
**Cost Model:** Numerical ODE for mean queue; offered load R(t)=∫λ(s)P(S>t−s)ds.
**Real Wall:** Common in call centers; staffing changes with t.
**Cross-Domain Aliases:** nhpp_queue, time_varying_arrivals
**Notes:** Eick–Massey–Whitt (1993).

---

### QT.302: Modified Offered Load (MOL)
**Definition:** Approximate time-varying M/M/c blocking by Erlang B with offered load R(t).
**Cost Model:** Closed-form Erlang B at each t.
**Real Wall:** Underestimates short-term saturation.
**Cross-Domain Aliases:** mol_approx, offered_load_approx
**Notes:** Jagerman (1975).

---

### QT.303: Pointwise Stationary Approximation (PSA)
**Definition:** At each t treat system as stationary M/M/c with current rate λ(t).
**Cost Model:** Cheap; ignores lag.
**Real Wall:** Accurate only when service much faster than rate changes.
**Cross-Domain Aliases:** psa, pointwise_stationary
**Notes:** Green–Kolesar (1991).

---

### QT.304: SIPP (Stationary Independent Period by Period)
**Definition:** Divide horizon into intervals, solve stationary queue in each independently.
**Cost Model:** Stationary Erlang per interval.
**Real Wall:** Misses spillover across intervals.
**Cross-Domain Aliases:** sipp, slot_by_slot
**Notes:** Common heuristic in call-center staffing.

---

## Section: Bulk and Retrial Queues

### QT.305: Bulk Arrivals (M^X/M/1)
**Definition:** Arrivals occur in random-size batches X with distribution B.
**Cost Model:** PGF of queue length: P(z)=(1−ρ)(1−z)μ/(μ(1−z)−λ(1−B(z))).
**Real Wall:** Burstiness amplifies queue; variance of X dominates.
**Cross-Domain Aliases:** batch_arrivals, mx_m_1
**Notes:** Chaudhry–Templeton (1983).

---

### QT.306: Bulk Service (M/M^Y/1)
**Definition:** Server serves up to Y customers simultaneously; batch size Y random.
**Cost Model:** Generating-function analysis; gain when Y > 1.
**Real Wall:** Idle if batch incomplete (threshold rules).
**Cross-Domain Aliases:** batch_service, m_my_1
**Notes:** Used in elevator/transit scheduling.

---

### QT.307: M^X/G/1 Queue
**Definition:** Compound-Poisson batch arrivals with general service.
**Cost Model:** Generalised PK with batch-arrival correction λE[X²]E[S²]/2.
**Real Wall:** Batch-size moments often unknown.
**Cross-Domain Aliases:** compound_poisson_queue, batch_pk
**Notes:** Takagi (1991).

---

### QT.308: M/G^Y/1 Queue
**Definition:** Poisson arrivals, general service of batches of size Y.
**Cost Model:** Embedded Markov chain at batch completions.
**Real Wall:** Threshold and full-batch policies create discontinuities.
**Cross-Domain Aliases:** bulk_service_gen, mgy_1
**Notes:** Used in transport scheduling.

---

### QT.309: M/M/c with Retrials
**Definition:** Blocked customers join an orbit and retry after exp(θ).
**Cost Model:** Approximate steady state via fixed-point or matrix-analytic.
**Real Wall:** No exact closed form for c≥2.
**Cross-Domain Aliases:** retrial_queue, orbit_queue
**Notes:** Falin–Templeton (1997).

---

### QT.310: Persistence and Retrial Rates
**Definition:** Customers retry with persistence parameter; high persistence → near M/M/c, low → loss system.
**Cost Model:** Loss probability and orbit size depend on persistence and θ.
**Real Wall:** Persistence varies across customers.
**Cross-Domain Aliases:** persistence, retrial_persistence
**Notes:** Artalejo–Gomez-Corral (2008).

---

### QT.311: Orbit Dynamics
**Definition:** State variable counting customers retrying outside primary system.
**Cost Model:** Bivariate Markov chain (server state, orbit size).
**Real Wall:** State space infinite; truncation needed for numerics.
**Cross-Domain Aliases:** orbit_state, retrial_state
**Notes:** Falin (1990).

---

## Section: Vacation and Polling Queues

### QT.312: M/G/1 with Multiple Vacations
**Definition:** Server takes i.i.d. vacations of length V whenever empty; continues vacations until customer arrives.
**Cost Model:** Wait LST = M/G/1 LST × (1−V*(s))/(sE[V]).
**Real Wall:** Idle scheduling penalises customers via residual vacation.
**Cross-Domain Aliases:** multiple_vacations, takagi_vacation
**Notes:** Takagi (1991).

---

### QT.313: M/G/1 with Single Vacation
**Definition:** Server takes one vacation upon emptying, then awaits arrivals.
**Cost Model:** Similar decomposition; vacation contribution differs from multiple.
**Real Wall:** Hard to enforce single-vacation discipline.
**Cross-Domain Aliases:** single_vacation, vacation_queue
**Notes:** Doshi (1986).

---

### QT.314: Working Vacations
**Definition:** Server serves at reduced rate during vacation rather than stopping.
**Cost Model:** Two-rate Markov chain; explicit solutions via matrix-analytic.
**Real Wall:** Energy-saving server policies.
**Cross-Domain Aliases:** working_vacation, partial_vacation
**Notes:** Servi–Finn (2002).

---

### QT.315: Threshold Vacations
**Definition:** Server takes vacation when queue drops below threshold N.
**Cost Model:** Embedded chain with threshold renewal points.
**Real Wall:** Used in energy-aware datacenters.
**Cross-Domain Aliases:** n_policy, threshold_vacation
**Notes:** Heyman (1968).

---

### QT.316: Cyclic Polling Systems
**Definition:** Single server cycles through K queues, applying a service discipline at each.
**Cost Model:** Mean cycle time C = Σ E[V_i]/(1−ρ); pseudo-conservation links mean waits.
**Real Wall:** Many practical settings: token rings, factory robot tours.
**Cross-Domain Aliases:** polling_system, cyclic_polling
**Notes:** Takagi (1986).

---

### QT.317: Gated Polling
**Definition:** At each visit, server serves only customers present at arrival to that queue.
**Cost Model:** Polynomial-time mean wait via descendant-set or buffer occupancy techniques.
**Real Wall:** Limits work per visit; predictable cycle length.
**Cross-Domain Aliases:** gated_polling, gated_service
**Notes:** Eisenberg (1972).

---

### QT.318: Exhaustive Polling
**Definition:** Server serves queue i until empty before moving on.
**Cost Model:** Reduces to branching-type recursion (Resing 1993).
**Real Wall:** Long cycles when one queue is heavy.
**Cross-Domain Aliases:** exhaustive_polling, busy_period_polling
**Notes:** Polynomial expectation via branching theorem.

---

### QT.319: k-Limited Polling
**Definition:** Up to k customers served per visit at each queue.
**Cost Model:** No exact closed form; approximations via pseudo-conservation.
**Real Wall:** Practical fairness mechanism.
**Cross-Domain Aliases:** k_limited_polling, limited_service
**Notes:** Resing (1993).

---

### QT.320: Branching Theorem for Polling
**Definition:** Mean queue lengths solve linear equations derived from multitype branching process underlying gated and exhaustive disciplines.
**Cost Model:** Solve K×K linear system.
**Real Wall:** Disciplines must satisfy branching property.
**Cross-Domain Aliases:** branching_polling, resing_theorem
**Notes:** Resing (1993).

---

### QT.321: Kühn Pseudo-Conservation Law
**Definition:** Weighted sum of mean waits in polling system equals expression depending only on cycle time and loads.
**Cost Model:** Σ ρ_iE[W_i] = (ρλE[S²]/(2(1−ρ))) + (ρE[V]/2)(σ_V²/E[V]²+ ...)
**Real Wall:** Doesn't give individual waits, only aggregate.
**Cross-Domain Aliases:** pseudo_conservation, polling_conservation
**Notes:** Boxma–Groenendijk (1987).

---

## Section: Markov Chain Numerics

### QT.322: Rate Matrix (Q-Matrix)
**Definition:** Generator of continuous-time Markov chain; off-diagonal q_{ij}≥0, row sums zero.
**Cost Model:** Stationary π satisfies πQ=0, Σπ=1.
**Real Wall:** Solving large sparse linear systems.
**Cross-Domain Aliases:** generator_matrix, q_matrix
**Notes:** Foundational for CTMCs.

---

### QT.323: CTMC Stationary Distribution
**Definition:** π such that πQ=0; gives long-run fraction of time in each state.
**Cost Model:** GTH algorithm O(n³) or sparse Krylov iteratives.
**Real Wall:** Curse of dimensionality on networked state spaces.
**Cross-Domain Aliases:** ctmc_stationary, equilibrium_distribution
**Notes:** Gross–Harris (1998).

---

### QT.324: Matrix Exponential e^{Qt}
**Definition:** Transient distribution at t given by π(t)=π(0)e^{Qt}.
**Cost Model:** Scaling-and-squaring or Padé; O(n³) per call.
**Real Wall:** Numerical instability for stiff Q.
**Cross-Domain Aliases:** matrix_exp, transient_ctmc
**Notes:** Moler–Van Loan (2003).

---

### QT.325: Uniformization (Jensen's Method)
**Definition:** Sample CTMC via Poisson(qt) jumps on subordinated discrete chain P=I+Q/q.
**Cost Model:** O(qt·n²) per transient evaluation; numerically stable.
**Real Wall:** q must dominate diagonal entries.
**Cross-Domain Aliases:** uniformization, jensen_method
**Notes:** Jensen (1953); Grassmann (1977).

---

### QT.326: Krylov Methods for e^{Qt}
**Definition:** Build Krylov subspace span{v,Qv,...,Q^{k}v} and approximate e^{Qt}v in projection.
**Cost Model:** O(kn) per step; converges geometrically for normal Q.
**Real Wall:** Loss of orthogonality; reorthogonalization required.
**Cross-Domain Aliases:** krylov_exp, arnoldi_exp
**Notes:** Saad (1992); Expokit.

---

## Section: Phase-Type and Matrix-Analytic

### QT.327: Exponential Phase-Type
**Definition:** Distribution of absorption time in CTMC with single transient state.
**Cost Model:** F(t)=1−e^{−μt}; the simplest phase-type.
**Real Wall:** Memoryless—often misfits real data.
**Cross-Domain Aliases:** exp_phase, single_phase
**Notes:** Base building block.

---

### QT.328: Erlang Distribution
**Definition:** Sum of k iid exponentials; CV² = 1/k.
**Cost Model:** Closed-form PDF, MGF, LST.
**Real Wall:** Underdispersed (CV²<1); limited fit range.
**Cross-Domain Aliases:** erlang_k, gamma_int_shape
**Notes:** Erlang (1909).

---

### QT.329: Hyperexponential Distribution
**Definition:** Mixture of n exponentials with weights p_i and rates μ_i; CV²≥1.
**Cost Model:** F(t)=1−Σp_i e^{−μ_it}.
**Real Wall:** Tail at most exponential; can't model heavy tails.
**Cross-Domain Aliases:** hyperexponential, hk_distribution
**Notes:** Cox (1955).

---

### QT.330: Coxian Distribution
**Definition:** Series of exponential phases with possible exit after each phase.
**Cost Model:** n parameters fit any CV² > 1/n.
**Real Wall:** Acyclic phase-type; not all PHs are Coxian.
**Cross-Domain Aliases:** coxian_distribution, ph_canonical
**Notes:** Cumani's canonical form.

---

### QT.331: HyperErlang Distribution
**Definition:** Mixture of Erlang distributions with possibly different shape parameters.
**Cost Model:** Dense subclass of phase-type; closed under sums and mixtures.
**Real Wall:** Fitting nonconvex.
**Cross-Domain Aliases:** hyper_erlang, mixture_erlang
**Notes:** Used in EM fitting (Asmussen–Nerman–Olsson).

---

### QT.332: General Phase-Type Distribution
**Definition:** Distribution of absorption time in finite-state CTMC; F(t)=1−αe^{Tt}1.
**Cost Model:** O(n²) per density evaluation.
**Real Wall:** Identifiability ambiguous; many representations.
**Cross-Domain Aliases:** phase_type, neuts_ph
**Notes:** Neuts (1981).

---

### QT.333: PH/PH/1 Queue
**Definition:** Single-server queue with both interarrival and service times phase-type.
**Cost Model:** Solved by matrix-analytic methods; QBD structure.
**Real Wall:** State explosion as phases grow.
**Cross-Domain Aliases:** phph1_queue, mam_queue
**Notes:** Neuts (1981).

---

### QT.334: Matrix-Geometric Distribution (Neuts)
**Definition:** Stationary distribution of QBD has form π_k = π_0 R^k.
**Cost Model:** Compute rate matrix R via iterative solution of R²+R B + A = 0.
**Real Wall:** Cyclic reduction or logarithmic reduction needed for speed.
**Cross-Domain Aliases:** matrix_geometric, neuts_r_matrix
**Notes:** Neuts (1981).

---

### QT.335: QBD (Quasi-Birth-Death) Process
**Definition:** Markov chain whose transitions depend on phase and level, with level changes by ±1.
**Cost Model:** Block tridiagonal generator; R-matrix algorithms.
**Real Wall:** Boundary conditions delicate.
**Cross-Domain Aliases:** qbd_process, level_chain
**Notes:** Latouche–Ramaswami (1999).

---

### QT.336: GI/M/1-Type Markov Chain
**Definition:** Markov chain with block-Hessenberg generator (level can decrease by multiple but only increase by 1).
**Cost Model:** Solved via matrix R from Σ A_k R^k = R.
**Real Wall:** Computation of R requires iterative scheme.
**Cross-Domain Aliases:** gim1_type, neuts_gm1
**Notes:** Neuts (1981).

---

### QT.337: M/G/1-Type Markov Chain
**Definition:** Block-Hessenberg generator with single-step level decreases and unbounded increases.
**Cost Model:** Solved via G-matrix; cyclic reduction.
**Real Wall:** G computed iteratively; convergence depends on traffic.
**Cross-Domain Aliases:** mg1_type, neuts_mg1
**Notes:** Neuts (1989).

---

### QT.338: R-Matrix
**Definition:** Minimal nonnegative solution to R²A_2+RA_1+A_0=0 in QBD analysis.
**Cost Model:** Spectral radius bounded by 1 under stability.
**Real Wall:** Cyclic reduction yields quadratic convergence.
**Cross-Domain Aliases:** r_matrix, qbd_r
**Notes:** Latouche–Ramaswami (1993).

---

### QT.339: G-Matrix
**Definition:** Minimal nonnegative solution to G=Σ A_k G^k in M/G/1-type chains.
**Cost Model:** Cyclic-reduction algorithm O(K³log(1/ε)).
**Real Wall:** Sensitive to overflow at high orders.
**Cross-Domain Aliases:** g_matrix, mg1_g
**Notes:** Ramaswami's algorithm.

---

### QT.340: MAM Solvers
**Definition:** Software/libraries implementing matrix-analytic methods (e.g., SMCSolver, BuTools).
**Cost Model:** O(n³ log(1/ε)) for QBD; specialized algorithms exist for sparse blocks.
**Real Wall:** Limited by RAM for large phase spaces.
**Cross-Domain Aliases:** mam_solver, smcsolver
**Notes:** Bini–Latouche–Meini (2005).

---

## Section: Markov-Modulated Arrivals

### QT.341: Markov-Modulated Poisson Process (MMPP)
**Definition:** Arrival rate depends on state of background CTMC; arrivals are Poisson conditional on state.
**Cost Model:** Two-state MMPP parameterised by 4 numbers; closed-form moments.
**Real Wall:** Captures burstiness but not long-range dependence.
**Cross-Domain Aliases:** mmpp, switched_poisson
**Notes:** Fischer–Meier-Hellstern (1993).

---

### QT.342: Markovian Arrival Process (MAP)
**Definition:** General class of Markov-modulated counting processes with rate matrices D_0,D_1.
**Cost Model:** Moments and Laplace transform expressible via (D_0,D_1).
**Real Wall:** Fitting from data requires EM.
**Cross-Domain Aliases:** map_process, neuts_map
**Notes:** Neuts (1979); Lucantoni (1991).

---

### QT.343: Batch MAP (BMAP)
**Definition:** MAP extended so that each transition triggers a batch of arrivals.
**Cost Model:** Rate matrices D_k for batches of size k.
**Real Wall:** Used to model bursty packet aggregations.
**Cross-Domain Aliases:** bmap, neuts_bmap
**Notes:** Lucantoni (1991).

---

### QT.344: MAP/PH/1 Queue
**Definition:** MAP arrivals with phase-type service.
**Cost Model:** Solved via matrix-analytic; closed-form distributions.
**Real Wall:** Block sizes explode with phase counts.
**Cross-Domain Aliases:** map_ph_1, structured_queue
**Notes:** Lucantoni (1991).

---

## Section: Renewal Theory

### QT.345: Renewal Equation
**Definition:** Z(t)=z(t)+∫_0^t Z(t−u)dF(u) describes renewal-reward processes.
**Cost Model:** Solved via convolution and Laplace transform.
**Real Wall:** Long-time behaviour from key renewal theorem.
**Cross-Domain Aliases:** renewal_equation, smith_equation
**Notes:** Feller (1971).

---

### QT.346: Smith's Key Renewal Theorem
**Definition:** lim_{t→∞}∫_0^t z(t−u)dm(u)=(1/μ)∫_0^∞z(u)du for directly Riemann integrable z.
**Cost Model:** Provides asymptotic constants of renewal-reward sums.
**Real Wall:** Requires d.R.i. condition.
**Cross-Domain Aliases:** key_renewal, smith_theorem
**Notes:** Smith (1958).

---

### QT.347: Age and Excess Life
**Definition:** A(t) and Y(t): time since last and time to next renewal, respectively.
**Cost Model:** Stationary joint density f_X(a+y)/μ.
**Real Wall:** Inspection paradox: stationary lifetime is size-biased.
**Cross-Domain Aliases:** age_excess, inspection_paradox
**Notes:** Cox (1962).

---

### QT.348: Equilibrium Distribution
**Definition:** F_e(x)=∫_0^x (1−F(u))du/μ; distribution of stationary residual life.
**Cost Model:** Equilibrium has higher CV than original under DFR.
**Real Wall:** Knowledge of full F required.
**Cross-Domain Aliases:** equilibrium_distribution, residual_life
**Notes:** Cox (1962).

---

### QT.349: Blackwell's Renewal Theorem
**Definition:** For non-arithmetic F, m(t+h)−m(t)→h/μ as t→∞.
**Cost Model:** Provides linear growth rate of renewal function.
**Real Wall:** Requires non-arithmetic; arithmetic case needs lattice version.
**Cross-Domain Aliases:** blackwell_theorem, renewal_growth
**Notes:** Blackwell (1948).

---

## Section: Branching and Population Processes

### QT.350: Galton-Watson Branching
**Definition:** Each individual produces i.i.d. random number of offspring; population evolves over generations.
**Cost Model:** Extinction probability is smallest fixed point of pgf.
**Real Wall:** Foundation of epidemic and offspring models.
**Cross-Domain Aliases:** galton_watson, branching_chain
**Notes:** Watson–Galton (1874); Athreya–Ney (1972).

---

### QT.351: Multitype Branching
**Definition:** Each type i has offspring distribution; mean matrix M dictates growth.
**Cost Model:** Extinction if spectral radius ρ(M)≤1.
**Real Wall:** Eigen-analysis grows with type count.
**Cross-Domain Aliases:** multitype_bp, harris_branching
**Notes:** Harris (1963).

---

### QT.352: Age-Dependent Branching
**Definition:** Individuals reproduce after random lifetimes; offspring count depends on age.
**Cost Model:** Bellman–Harris equation for population size.
**Real Wall:** Lifetimes typically non-exponential.
**Cross-Domain Aliases:** bellman_harris, age_dependent_bp
**Notes:** Bellman–Harris (1948).

---

### QT.353: Branching with Immigration
**Definition:** Branching process augmented by i.i.d. immigration each generation.
**Cost Model:** Stationary if subcritical (m<1); explicit pgf relations.
**Real Wall:** Models queueing busy periods with re-entry.
**Cross-Domain Aliases:** branching_immigration, foster_branching
**Notes:** Foster (1971).

---

## Section: Stochastic Processes Foundations

### QT.354: Brownian Motion / Wiener Process
**Definition:** Continuous Gaussian process B(t) with independent increments B(t)−B(s)∼N(0,t−s).
**Cost Model:** Sample paths Hölder-continuous of order <1/2; nowhere differentiable.
**Real Wall:** Idealisation; real noise rarely strictly Brownian.
**Cross-Domain Aliases:** brownian_motion, wiener_process
**Notes:** Foundation of diffusion approximations.

---

### QT.355: Brownian Bridge
**Definition:** Brownian motion conditioned to return to 0 at time 1: B^br(t)=B(t)−tB(1).
**Cost Model:** Mean 0, Cov(s,t)=min(s,t)−st.
**Real Wall:** Appears in goodness-of-fit and bridged simulations.
**Cross-Domain Aliases:** brownian_bridge, conditioned_brownian
**Notes:** Doob (1949).

---

### QT.356: Ornstein-Uhlenbeck Process
**Definition:** Stationary Gaussian solution to dX=−θ(X−μ)dt+σdW.
**Cost Model:** Stationary normal with variance σ²/(2θ).
**Real Wall:** Mean-reverting; arises in QED limits.
**Cross-Domain Aliases:** ou_process, mean_reverting
**Notes:** Uhlenbeck–Ornstein (1930).

---

### QT.357: Geometric Brownian Motion
**Definition:** dS=μSdt+σSdW; log S is Brownian with drift.
**Cost Model:** Closed-form moments E[S(t)]=S_0 e^{μt}.
**Real Wall:** Used in finance; tail not heavy enough for crashes.
**Cross-Domain Aliases:** gbm, log_brownian
**Notes:** Samuelson (1965).

---

### QT.358: Itô Calculus
**Definition:** Stochastic integration: ∫f(B_s)dB_s defined as L² limit of step-function approximations.
**Cost Model:** Itô's lemma: df(B)=f'(B)dB+(1/2)f''(B)dt.
**Real Wall:** Non-anticipating integrands required.
**Cross-Domain Aliases:** ito_calculus, stochastic_integration
**Notes:** Itô (1944).

---

### QT.359: Stochastic Differential Equations
**Definition:** dX=μ(X,t)dt+σ(X,t)dW; existence/uniqueness under Lipschitz drift and diffusion.
**Cost Model:** Numerics: Euler-Maruyama O(√Δt), Milstein O(Δt).
**Real Wall:** Strong vs weak convergence subtleties.
**Cross-Domain Aliases:** sde, ito_diffusion
**Notes:** Kloeden–Platen (1992).

---

## Section: Lévy Processes

### QT.360: Compound Poisson Process
**Definition:** N(t) Poisson with rate λ, with each jump distributed iid like Y.
**Cost Model:** Characteristic function exp(λt(E[e^{iuY}]−1)).
**Real Wall:** Models insurance claims, batch arrivals.
**Cross-Domain Aliases:** compound_poisson, cpp
**Notes:** Lundberg (1903).

---

### QT.361: Lévy-Khintchine Representation
**Definition:** Every Lévy process has characteristic exponent ψ(u)=iau−σ²u²/2+∫(e^{iux}−1−iux1_{|x|<1})ν(dx).
**Cost Model:** Three components: drift, diffusion, jumps.
**Real Wall:** Lévy measure ν may be hard to estimate.
**Cross-Domain Aliases:** levy_khintchine, levy_triplet
**Notes:** Khintchine (1937).

---

### QT.362: Jump-Diffusion
**Definition:** SDE dX=μdt+σdW+dJ with Poisson jumps J.
**Cost Model:** Numerics combine SDE step with jump count.
**Real Wall:** Merton (1976) used in option pricing.
**Cross-Domain Aliases:** jump_diffusion, merton_model
**Notes:** Used in financial mathematics and queueing with shocks.

---

### QT.363: Subordinators
**Definition:** Nondecreasing Lévy processes used as random clocks.
**Cost Model:** Laplace exponent φ(λ)=bλ+∫(1−e^{−λx})ν(dx).
**Real Wall:** Time-changed Lévy processes capture nonstationary clocks.
**Cross-Domain Aliases:** subordinator, levy_clock
**Notes:** Bertoin (1996).

---

### QT.364: α-Stable Processes
**Definition:** Self-similar Lévy processes with index α∈(0,2]; α=2 gives Brownian motion.
**Cost Model:** No second moment when α<2.
**Real Wall:** Models heavy-tailed traffic and finance.
**Cross-Domain Aliases:** stable_process, alpha_stable
**Notes:** Samorodnitsky–Taqqu (1994).

---

## Section: Large Deviations and Effective Bandwidth

### QT.365: Cramér's Theorem
**Definition:** For i.i.d. sums S_n/n, P(S_n/n≥x) decays like e^{−nI(x)} with I=Λ*.
**Cost Model:** Convex Λ*(x)=sup_λ(λx−Λ(λ)).
**Real Wall:** Needs MGF Λ to exist near 0.
**Cross-Domain Aliases:** cramer_theorem, sample_mean_ldp
**Notes:** Cramér (1938).

---

### QT.366: Gärtner-Ellis Theorem
**Definition:** Generalisation of Cramér using limiting log-MGF Λ(λ)=lim_n (1/n)log E[e^{nλY_n}].
**Cost Model:** Yields rate function for dependent sequences.
**Real Wall:** Requires Λ differentiable and steep.
**Cross-Domain Aliases:** gartner_ellis, ldp_general
**Notes:** Ellis (1984).

---

### QT.367: Sanov's Theorem
**Definition:** Empirical distribution of i.i.d. samples concentrates on true law with rate KL divergence.
**Cost Model:** Rate function I(μ)=KL(μ∥ν).
**Real Wall:** Discrete alphabets clean; continuous needs care.
**Cross-Domain Aliases:** sanov_theorem, empirical_ldp
**Notes:** Sanov (1957).

---

### QT.368: Queueing Rate Function
**Definition:** Tail of stationary queue: P(Q>x) ≈ e^{−ηx} with η rate from large-deviation principle on arrival process.
**Cost Model:** η solves Λ(η)=ημ for arrival cumulant.
**Real Wall:** Heavy tails preclude exponential decay.
**Cross-Domain Aliases:** rate_function_queue, decay_exponent
**Notes:** Glynn–Whitt (1994).

---

### QT.369: Sample-Path Large Deviations
**Definition:** LDP at level of trajectories of scaled queue or workload process.
**Cost Model:** Rate functional minimised over absolutely continuous paths.
**Real Wall:** Mogulskii's theorem; variational calculation needed.
**Cross-Domain Aliases:** sample_path_ldp, mogulskii
**Notes:** Dembo–Zeitouni (2010).

---

### QT.370: Effective Bandwidth
**Definition:** α(s,t)=(1/(st))log E[e^{sX(0,t)}] where X(0,t) is workload over interval.
**Cost Model:** Used to derive admission control bounds in shared links.
**Real Wall:** Assumes stationary source statistics.
**Cross-Domain Aliases:** effective_bandwidth, kelly_eb
**Notes:** Kelly (1996).

---

### QT.371: Deterministic Network Calculus
**Definition:** Worst-case analysis using arrival curves α(t) and service curves β(t).
**Cost Model:** Backlog ≤ sup_s α(s)−β(s); delay ≤ sup_s inf{τ:α(s)≤β(s+τ)}.
**Real Wall:** Bounds can be loose; assumes deterministic envelopes.
**Cross-Domain Aliases:** network_calculus, dnc
**Notes:** Le Boudec–Thiran (2001).

---

### QT.372: Stochastic Network Calculus
**Definition:** Probabilistic envelopes on traffic and service; bounds on tail of backlog and delay.
**Cost Model:** Combine Chernoff-type bounds with min-plus algebra.
**Real Wall:** Tightness depends on choice of moment generating bound.
**Cross-Domain Aliases:** snc, fidler_snc
**Notes:** Jiang–Liu (2008).

---

### QT.373: Min-Plus Convolution
**Definition:** (α⊗β)(t)=inf_{0≤s≤t}{α(s)+β(t−s)}; algebraic structure underlying network calculus.
**Cost Model:** Convex hull of arrival/service curves.
**Real Wall:** Numerical efficiency depends on piecewise representations.
**Cross-Domain Aliases:** min_plus, max_plus_algebra
**Notes:** Le Boudec–Thiran (2001).

---

### QT.374: Leaky Bucket / Token Bucket
**Definition:** Traffic regulator allowing rate r with burst b: arrival curve α(t)=rt+b.
**Cost Model:** Worst-case delay ≤ b/μ for service of rate μ ≥ r.
**Real Wall:** Real shapers have finite memory.
**Cross-Domain Aliases:** token_bucket, leaky_bucket
**Notes:** Turner (1986).

---

## Section: Game Theory and Strategic Queueing

### QT.375: Naor's Threshold
**Definition:** Self-interested customers in observable M/M/1 join iff queue length ≤ ⌊(Rμ−C)/C⌋.
**Cost Model:** Individual threshold differs from social optimum.
**Real Wall:** Customers assumed rational, fully informed.
**Cross-Domain Aliases:** naor_threshold, observable_queue_game
**Notes:** Naor (1969).

---

### QT.376: Observable vs Unobservable Queue Games
**Definition:** In unobservable case, equilibrium join probability balances waiting cost and reward.
**Cost Model:** Equilibrium joining probability solves (R−CW(λq))q=0.
**Real Wall:** Information asymmetry alters equilibria.
**Cross-Domain Aliases:** queue_game_info, edelson_hildebrand
**Notes:** Edelson–Hildebrand (1975).

---

### QT.377: Individually vs Socially Optimal Policies
**Definition:** Selfish equilibrium throughput exceeds social optimum; admission price restores efficiency.
**Cost Model:** Pigovian toll = externality marginal cost.
**Real Wall:** Requires policy enforcement; price design depends on demand curve.
**Cross-Domain Aliases:** social_vs_individual_optim, pigou_toll
**Notes:** Hassin–Haviv (2003).

---

## Section: Service-Time Distributions

### QT.378: Lognormal Service Times
**Definition:** Service ~ exp(N(μ,σ²)); right-skewed and used to fit empirical call-handling durations.
**Cost Model:** CV² = e^{σ²}−1.
**Real Wall:** Not phase-type; numerical methods rely on moment matching.
**Cross-Domain Aliases:** lognormal_service, call_center_service
**Notes:** Brown et al. (2005) call-center data.

---

### QT.379: Pareto / Heavy-Tailed Service
**Definition:** Service tail P(S>x)~x^{−α}; finite mean iff α>1, finite variance iff α>2.
**Cost Model:** Queue tails inherit power-law decay under FCFS.
**Real Wall:** Drives long-range dependence in file transfers.
**Cross-Domain Aliases:** pareto_service, heavy_tailed_service
**Notes:** Crovella–Bestavros (1996).

---

### QT.380: Queueing Network Analyzer (QNA)
**Definition:** Whitt's parametric decomposition approximating GI/GI/1 nodes in open networks using two-moment matching.
**Cost Model:** Per-node SCV propagation: c_d² = ρ²c_s² + (1−ρ²)c_a²; splitting and merging formulas.
**Real Wall:** Heuristic; errors compound with feedback.
**Cross-Domain Aliases:** qna, whitt_qna
**Notes:** Whitt (1983).

---
